"""Launch the standalone release email app with Makepad's built-in instrument."""
from __future__ import annotations
import argparse
import fcntl
import json
import os
from pathlib import Path
import queue
import re
import signal
import socket
import subprocess
import sys
import threading
import time
import uuid
from common import ROOT, PIPELINE, IMAGE, NATIVE_ROOT, RUNTIME
from mailbox import atomic_json, fetch_gmail, save_live, merge_mailbox, fixture, PRIVATE, test_connection
from account import load_account, public_account, candidate, save_account, account_id, identity
from model import initial, reduce, visible, has_more
from render import render

from instrument import Instrument
from mail_actions import MailActions
from copy import deepcopy
import gmail_sync

SESSION=RUNTIME/'session'
CURRENT=RUNTIME/'current-request.json'
SCENES={'inbox':1,'mailboxes':2,'read':3,'search':4,'compose':5,'settings':7,'card':8,'drafts':2,'attachments':3,'folders':2,'services':7}


def listening(port):
    try:
        with socket.create_connection(('127.0.0.1',port),timeout=.2):return True
    except OSError:return False


def start_process(args,log,env=None,cwd=ROOT):
    with Path(log).open('a') as stream:
        p=subprocess.Popen([str(a) for a in args],cwd=cwd,stdout=stream,stderr=subprocess.STDOUT,env=env,start_new_session=True)
    return p.pid


def launch_native(app,hidden=False):
    from core.native_runtime import verify, runtime_tool
    verify(NATIVE_ROOT)
    logs=ROOT/'runtime/logs';logs.mkdir(parents=True,exist_ok=True)
    env=os.environ.copy();env.update(RUSTFLAGS='',CARGO_PROFILE_RELEASE_LTO='false',BEAUTY_REQUEST=str(CURRENT))
    for key in ('STUDIO_HOST','STUDIO_BUILD','STUDIO_CRATE','MAKEPAD_REMOTE','MAKEPAD_FOCUS'):
        env.pop(key,None)
    if hidden:env['MAKEPAD_HIDE_WINDOWS']='1'
    else:env.pop('MAKEPAD_HIDE_WINDOWS',None)
    workspace=NATIVE_ROOT/'octoscript-makepad'
    with (logs/'build.log').open('a') as log:
        subprocess.run(['cargo','build','--release','-p','kit-host','--bin','beauty-host'],cwd=workspace,env=env,stdout=log,stderr=log,check=True)
    runtime_tool(NATIVE_ROOT).verify_cargo(NATIVE_ROOT, workspace/'Cargo.toml')
    if not listening(8170):
        start_process([sys.executable,'-m','http.server','8170','--bind','127.0.0.1','--directory',PIPELINE/'docs/reviews/theme-phone-evidence'],logs/'artwork.log',env)
    app.build='native-'+uuid.uuid4().hex
    app.mount()
    logfile=logs/(app.build+'.log')
    with logfile.open('w') as log:
        os.chmod(logfile,0o600)
        process=subprocess.Popen([str(workspace/'target/release/beauty-host'),'--remote'],cwd=workspace,env=env,stdout=log,stderr=subprocess.STDOUT,start_new_session=True)
    deadline=time.monotonic()+30
    while time.monotonic()<deadline:
        match=re.search(r'\[makepad-remote\] listening on (127\.0\.0\.1:\d+)',logfile.read_text())
        if match:
            remote={'pid':process.pid,'endpoint':'http://'+match.group(1),'build_id':app.build,'hidden':hidden,'log':str(logfile)}
            atomic_json(SESSION/'remote.json',remote)
            return process,Instrument(remote['endpoint'])
        if process.poll() is not None:raise RuntimeError('Native mail app exited during startup; see '+str(logfile))
        time.sleep(.1)
    process.terminate()
    raise RuntimeError('Native instrument startup timed out')


class MailSession(MailActions):
    def __init__(self,live=True):
        mailbox=json.loads((PRIVATE/'mailbox.json').read_text()) if live and (PRIVATE/'mailbox.json').exists() else fixture()
        self.state=initial(mailbox)
        self.account=load_account()
        if mailbox['address']!='you@gmail.com' and not mailbox.get('account_id'):
            mailbox['account_id']=account_id(self.account)
        self.settings_password=None;self.settings_generation=0;self.pending_settings=None
        self.reset_settings()
        saved=PRIVATE/'local-state.json'
        if saved.exists():
            data=json.loads(saved.read_text())
            self.state['pending_sync']=data.get('pending_sync',{})
            self.state['drafts']=data.get('drafts',[])
            self.state['draft']=data.get('draft',self.state['draft'])
        self.build=None;self.meta=None;self.cursor=0;self.pending_search=None
        self.mailbox_epoch=0
        self.updates=queue.Queue()
        self.restore_send_state()

    def reset_settings(self):
        self.settings_generation+=1;self.settings_password=None;self.pending_settings=None
        self.state.update(account_form=public_account(self.account),password_saved=bool(self.account.get('password')),
            password_pending=False,account_busy=False,account_error='',account_status='Changes apply when you tap Save.',settings_focus=None)

    def settings_changed(self):
        self.settings_generation+=1
        self.state['account_busy']=False
        self.state['password_saved']=bool(self.account.get('password')) and identity(self.state['account_form'])==identity(self.account)

    def start_account_test(self):
        if self.state['account_busy']:return
        try:config=candidate(self.state['account_form'],self.settings_password)
        except ValueError as e:
            self.state.update(account_error=str(e),settings_focus=None);self.mount();return
        token=self.settings_generation
        self.state.update(account_busy=True,account_error='',account_status='Connecting securely…',settings_focus=None)
        self.mount()
        def work():
            try:self.updates.put(('account_test_ok',test_connection(config),token))
            except Exception:self.updates.put(('account_test_error','Connection failed. Check server, login and password.',token))
        threading.Thread(target=work,daemon=True).start()

    def start_password(self):
        if self.state['account_busy']:return
        token=self.settings_generation
        self.state.update(account_busy=True,account_error='',account_status='Enter your password in the secure dialog.',settings_focus=None)
        self.mount()
        def work():
            try:
                source=ROOT/'service/password_prompt.swift';binary=ROOT/'runtime/mail-password-prompt'
                if not binary.exists() or binary.stat().st_mtime<source.stat().st_mtime:
                    subprocess.run(['swiftc',str(source),'-o',str(binary)],check=True,capture_output=True)
                result=subprocess.run([str(binary)],capture_output=True,text=True)
                if result.returncode==2:self.updates.put(('account_password_cancel',None,token))
                elif result.returncode==0:self.updates.put(('account_password_ok',json.loads(result.stdout)['password'],token))
                else:raise RuntimeError('Password dialog failed')
            except Exception:self.updates.put(('account_test_error','Could not open the secure password dialog.',token))
        threading.Thread(target=work,daemon=True).start()

    def save_settings(self):
        if self.state['account_busy']:return
        if self.state['send_busy'] or self.state['remote_busy']:
            self.state['account_error']='Wait for the current mail operation before saving settings.';self.mount();return
        try:
            config=candidate(self.state['account_form'],self.settings_password)
            changed=account_id(config)!=account_id(self.account)
            if changed:
                old_dir=PRIVATE/'accounts'/account_id(self.account)
                for name in ('mailbox.json','local-state.json'):
                    path=PRIVATE/name
                    if path.exists():atomic_json(old_dir/name,json.loads(path.read_text()))
            save_account(config)
            self.account=config
            self.mailbox_epoch+=1
            self.state.update(syncing=False,loading_more=False,error='',load_error='',remote_busy=False,send_busy=False,attachment_busy=False,remote_error='')
            if changed:
                directory=PRIVATE/'accounts'/account_id(config);path=directory/'mailbox.json'
                box=json.loads(path.read_text()) if path.exists() else {'address':config['address'],'account_id':account_id(config),'host':config['host'],'messages':[],'available':0,'has_more':False}
                local=directory/'local-state.json'
                drafts=json.loads(local.read_text()) if local.exists() else {'drafts':[],'draft':{'to':'','subject':'','body':''},'pending_sync':{}}
                drafts.setdefault('pending_sync',{})
                self.state.update(mailbox=box,folder='inbox',query='',list_scroll=0,list_start=0,selected=None,folders=[],folder_pages={},**drafts)
            elif self.state['mailbox']['address']!='you@gmail.com':
                self.state['mailbox'].update(address=config['address'],account_id=account_id(config))
            self.restore_send_state()
            self.reset_settings()
            self.state['account_status']='Settings saved. Check for Mail to refresh.'
        except ValueError as e:self.state.update(account_error=str(e),settings_focus=None)
        except OSError:self.state.update(account_error='Could not save settings on this Mac.',settings_focus=None)
        self.mount()

    def persist(self):
        atomic_json(SESSION/'state.json',self.state)
        if self.state['mailbox']['address']!='you@gmail.com':
            atomic_json(PRIVATE/'mailbox.json',self.state['mailbox'])
            atomic_json(PRIVATE/'local-state.json',{k:self.state[k] for k in ('drafts','draft','pending_sync')})

    def mount(self):
        nonce='mail-'+uuid.uuid4().hex
        out=SESSION/'mounts'/nonce
        frame=SCENES[self.state['screen']]
        render(self.state,out,'ios-mail-runtime',ROOT/'cards'/f'ios-mail-{frame:02d}'/'reference.png')
        r={'format':'l0-kit','card':str(out/'page.card'),'data':str(out/'page.data.json'),'kit_dir':str(out/'kit'),
           'width':406,'height':776,'title':'Mail · Octoscript','nonce':nonce,'build_id':self.build,
           'result':str(out/'native.json'),'layout':str(out/'layout.json'),'actions':str(out/'actions.json'),
           'semantic':str(out/'semantic-map.json'),'semantic_result':str(out/'semantic-state.json'),'semantic_probe':str(out/'semantic-probe.json')}
        if self.state['screen'] in ('inbox','search'):
            mapping=json.loads((out/'mapping.json').read_text())['elements']
            ids={n['source_id']:n['native_id'] for n in mapping}
            r.update(scroll_state=str(out/'scroll.json'),scroll_watch=[{'id':ids['list_scroll'],'content':ids['list_content']}],
                     scroll_restore=[{'id':ids['list_scroll'],'y':self.state['list_scroll']}],preserve_input_selection=True)
        if self.state['screen'] in ('settings','services'):r['preserve_input_selection']=True
        self.meta={'request':r,'mapping':str(out/'mapping.json'),'service_actions':str(out/'service-actions.json'),'screen':self.state['screen'],'revision':self.state['revision']}
        self.cursor=0
        atomic_json(SESSION/'session.json',self.meta)
        atomic_json(CURRENT,r)
        self.persist()

    def start_sync(self):
        if self.state['syncing'] or self.state['loading_more'] or self.state['remote_busy']:return
        self.state.update(syncing=True,error='')
        self.mount()
        epoch=self.mailbox_epoch
        config=dict(self.account)
        previous=deepcopy(self.state['mailbox'])
        def work():
            try:
                result=fetch_gmail(25,account=config)
                if config.get('sync_enabled'):
                    box=merge_mailbox(previous,result)
                    try:
                        gmail_sync.headers(config,box['messages'])
                        result['messages'],result['_folders']=gmail_sync.pull(config,box['messages'])
                    except Exception:result['_remote_error']='Mail received. Gmail sync failed; tap Check for Mail to retry.'
                self.updates.put(('ok',result,epoch))
            except Exception as e:
                # Do not log the credential or raw POP response in UI evidence.
                reason='Mail server could not be reached. Check settings and try again.'
                if 'AUTH' in str(e):reason='Login failed. Check your server settings and password.'
                self.updates.put(('error',reason,epoch))
        threading.Thread(target=work,daemon=True).start()

    def start_more(self):
        if self.state['loading_more'] or self.state['syncing'] or self.state['remote_busy'] or not has_more(self.state):return
        if self.state['folder'].startswith('remote:'):
            self.start_folder(self.state['folder'][7:],more=True);return
        box=self.state['mailbox']
        seen={m['uid'] for m in box['messages'] if m.get('uid')}|set(box.get('skipped_uids',[]))
        epoch=self.mailbox_epoch
        config=dict(self.account)
        self.state.update(loading_more=True,load_error='')
        self.mount()
        def work():
            try:
                result=fetch_gmail(25,exclude_uids=seen,account=config)
                if config.get('sync_enabled'):
                    try:result['messages'],_=gmail_sync.pull(config,result['messages'])
                    except Exception:result['_remote_error']='Mail received. Gmail sync failed; tap Check for Mail to retry.'
                self.updates.put(('more_ok',result,epoch))
            except Exception:self.updates.put(('more_error','Could not load older messages. Tap to retry.',epoch))
        threading.Thread(target=work,daemon=True).start()

    def scroll_position(self):
        if self.state['screen'] not in ('inbox','search') or self.pending_search:return None
        try:
            data=json.loads(Path(self.meta['request']['scroll_state']).read_text())
            if data['nonce']!=self.meta['request']['nonce'] or not data['elements']:return None
            position=data['elements'][0]
            self.state['list_scroll']=min(position['max_y'],max(0,position['y']))
            return position
        except (KeyError,OSError,ValueError):return None

    def update_list(self,position):
        if position is None:return
        s=self.state;rows=visible(s);first=int(s['list_scroll']//88);start=s['list_start']
        if (first<start+4 and start>0) or (first+6>start+56 and start+60<len(rows)):
            s['list_start']=min(max(0,first-12),max(0,len(rows)-60))
            if s['list_start']!=start:self.mount();return
        if (s['list_scroll']>0 and position['max_y']-s['list_scroll']<300 and not s['query']
                and (s['folder']=='inbox' or s['folder'].startswith('remote:')) and not s['load_error']):self.start_more()

    def action(self,event,payload,event_id):
        if self.mail_action(event,payload):return True
        if event in ('draft_field','compose','reply','open_draft') and self.state['send_busy']:return False
        if event=='draft_field' and self.state['send_uncertain']:return False
        if event in ('account_save','account_test','account_password'):
            self.pending_settings=None
            {'account_save':self.save_settings,'account_test':self.start_account_test,'account_password':self.start_password}[event]()
            return True
        if event in ('account_field','account_security','account_recent','smtp_security','account_sync'):
            old_identity=identity(self.state['account_form'])
            self.state=reduce(self.state,event,payload,event_id)
            if identity(self.state['account_form'])!=old_identity:
                self.settings_password=None;self.state['password_pending']=False
            self.settings_changed()
            if event=='account_field':self.pending_settings=time.monotonic()+.4;self.persist();return False
            self.pending_settings=None;self.mount();return True
        if event=='navigate' and payload.get('screen')=='settings' and self.state['screen'] not in ('settings','services'):self.reset_settings()
        if event=='sync':self.start_sync();return True
        if event=='load_more':self.start_more();return True
        if event in ('sample','live'):
            self.mailbox_epoch+=1
            self.pending_search=None
            path=PRIVATE/'mailbox.json'
            if event=='live' and not path.exists():self.start_sync();return True
            mailbox=fixture() if event=='sample' else json.loads(path.read_text())
            self.state=initial(mailbox)
            self.reset_settings()
            if event=='sample':self.state['drafts']=[{'to':'','subject':'Coffee next week?','body':'Hi Jamie,\n\nWould you like to grab coffee next week?\n\nBest,'}]
            else:
                saved=PRIVATE/'local-state.json'
                if saved.exists():self.state.update(json.loads(saved.read_text()))
            self.restore_send_state()
            self.start_remote()
            self.mount();return True
        before=deepcopy(self.state['mailbox']['messages']) if event in ('open','archive','flag','unread') else None
        self.state=reduce(self.state,event,payload,event_id)
        if before is not None:self.queue_changes(before)
        if event=='open_draft':self.restore_send_state()
        if event=='draft_field':self.persist();return False
        if event=='search':self.pending_search=time.monotonic()+.4;self.persist();return False
        self.pending_search=None
        self.mount();return True

    def poll(self):
        position=self.scroll_position()
        try:
            kind,data,epoch=self.updates.get_nowait()
            if self.mail_update(kind,data,epoch):return
            if kind.startswith('account_'):
                if epoch==self.settings_generation and self.state['screen'] in ('settings','services'):
                    self.state.update(account_busy=False,settings_focus=None)
                    if kind=='account_test_ok':self.state.update(account_status=f'Connected securely · {data:,} messages available.',account_error='')
                    elif kind=='account_password_ok':
                        self.settings_password=data
                        self.state.update(password_pending=True,account_status='New password ready. Tap Save to apply it.',account_error='')
                    elif kind=='account_password_cancel':self.state['account_status']='Password unchanged.'
                    else:self.state['account_error']=data
                    self.mount()
                return
            if epoch==self.mailbox_epoch:
                self.state['loading_more' if kind.startswith('more_') else 'syncing']=False
                if kind in ('ok','more_ok'):
                    rows=visible({**self.state,'screen':'inbox'});index=int(self.state['list_scroll']//88)
                    anchor=rows[index]['id'] if index<len(rows) else None
                    offset=self.state['list_scroll']%88
                    if '_folders' in data:self.state['folders']=data.pop('_folders')
                    self.state['remote_error']=data.pop('_remote_error','')
                    merged=merge_mailbox(self.state['mailbox'],data)
                    self.state.update(mailbox=merged,notice='Mail updated',error='',load_error='')
                    self.overlay_pending();self.start_remote()
                    if anchor:
                        rows=visible({**self.state,'screen':'inbox'})
                        index=next((i for i,m in enumerate(rows) if m['id']==anchor),0)
                        self.state.update(list_scroll=index*88+offset,list_start=min(max(0,index-12),max(0,len(rows)-60)))
                else:self.state['load_error' if kind=='more_error' else 'error']=data
                if kind.startswith('more_') and self.state['screen'] not in ('inbox','search'):self.persist()
                else:self.mount()
                return
        except queue.Empty:pass
        r=self.meta['request']
        if json.loads(CURRENT.read_text())['nonce']!=r['nonce']:raise RuntimeError('Native request ownership changed')
        try:
            native=json.loads(Path(r['result']).read_text())
            if native.get('request',{}).get('nonce')!=r['nonce'] or native.get('request',{}).get('build_id')!=self.build:return
            actions=json.loads(Path(r['actions']).read_text())
        except (OSError,ValueError):return
        mapping=json.loads(Path(self.meta['mapping']).read_text())['elements']
        by_native={m['native_id']:m for m in mapping};by_source={m['source_id']:m for m in mapping}
        controls=json.loads(Path(self.meta['service_actions']).read_text())['controls']
        while self.cursor<len(actions):
            index=self.cursor;self.cursor+=1
            item=actions[index];action=item.get('action',{});kind=action.get('kind')
            if kind not in ('activated','changed'):continue
            node=by_native.get(item.get('id'))
            if not node:continue
            while node['source_id'] not in controls and node.get('parent'):node=by_source[node['parent']]
            control=controls.get(node['source_id'])
            if not control or not control['enabled']:continue
            if bool(control.get('input'))!=(kind=='changed'):continue
            payload={**control.get('payload',{})}
            if kind=='changed':payload['value']=action['value']
            if self.action(control['event'],payload,r['nonce']+':'+str(index)):return
        if self.pending_search and time.monotonic()>=self.pending_search:
            self.pending_search=None;self.mount()
            return
        if self.pending_settings and time.monotonic()>=self.pending_settings:
            self.pending_settings=None;self.mount();return
        if self.pending_search is None:self.update_list(position)


def serve(live,hidden=False):
    SESSION.mkdir(parents=True,exist_ok=True,mode=0o700)
    with (SESSION/'watcher.lock').open('w') as lock:
        fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
        app=MailSession(live)
        process,instrument=launch_native(app,hidden)
        app.start_remote()
        print(json.dumps({'build_id':app.build,'native':'Makepad built-in HTTP instrument','pid':process.pid,'endpoint':instrument.endpoint,'hidden':hidden}),flush=True)
        def terminate(*_):raise SystemExit(0)
        signal.signal(signal.SIGTERM,terminate)
        try:
            while process.poll() is None:app.poll();time.sleep(.06)
        finally:
            if process.poll() is None:
                try:instrument.get('/quit')
                except Exception:pass
                try:process.wait(timeout=5)
                except subprocess.TimeoutExpired:process.terminate()


def main():
    p=argparse.ArgumentParser();p.add_argument('--sample',action='store_true');p.add_argument('--background',action='store_true');p.add_argument('--headless',action='store_true',help='Run the native renderer with its window hidden');a=p.parse_args()
    if a.background:
        SESSION.mkdir(parents=True,exist_ok=True,mode=0o700)
        with (SESSION/'watcher.lock').open('a') as lock:
            try:fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
            except BlockingIOError:
                print('Mail is already running in its native window.')
                return
        (ROOT/'runtime/logs').mkdir(parents=True,exist_ok=True)
        cmd=[sys.executable,Path(__file__).resolve()]+(['--sample'] if a.sample else [])+(['--headless'] if a.headless else [])
        pid=start_process(cmd,ROOT/'runtime/logs/app.log')
        atomic_json(ROOT/'runtime/app-process.json',{'pid':pid})
        print(f'Mail is starting with the built-in Makepad instrument (process {pid}).')
    else:serve(not a.sample,a.headless)

if __name__=='__main__':main()
