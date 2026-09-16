"""Gmail POP3 transport and MIME cache. Never deletes server messages."""
from __future__ import annotations
from datetime import datetime, timezone
from email import policy
from email.parser import BytesParser
from email.utils import parseaddr, parsedate_to_datetime
from html.parser import HTMLParser
from pathlib import Path
import hashlib
import base64
import json
import os
import poplib
import re
import ssl
from account import load_account, candidate, account_id

ROOT = Path(__file__).resolve().parents[1]
PRIVATE = Path(os.environ.get('OCTOS_MAIL_PRIVATE_DIR', ROOT / 'private')).resolve()
MAX_MESSAGE = 2_000_000


def atomic_json(path, data):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(path.suffix + '.new')
    fd = os.open(tmp, os.O_CREAT | os.O_WRONLY | os.O_TRUNC, 0o600)
    os.fchmod(fd,0o600)
    with os.fdopen(fd, 'w') as stream:
        json.dump(data, stream, ensure_ascii=False, indent=2)
    tmp.replace(path)


def credentials():
    data=load_account()
    address,password=data['address'],data.get('password','')
    if not address or not password:
        raise ValueError('Set GMAIL_ADDRESS and GMAIL_APP_PASSWORD in your local environment.')
    return address, password


def connect(account):
    context=ssl.create_default_context()
    if account['security']=='tls':
        client=poplib.POP3_SSL(account['host'],int(account['port']),timeout=20,context=context)
    else:
        client=poplib.POP3(account['host'],int(account['port']),timeout=20)
    try:
        if account['security']=='starttls':client.stls(context=context)
        username=account['username']
        if account['recent'] and not username.startswith('recent:'):username='recent:'+username
        client.user(username);client.pass_(account['password'])
        return client
    except Exception:
        client.close()
        raise


def disconnect(client):
    try:client.quit()
    except (OSError,poplib.error_proto):client.close()


def test_connection(account):
    client=connect(account)
    try:
        count,_=client.stat()
        return count
    finally:disconnect(client)


class PlainHTML(HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.parts, self.hidden = [], 0

    def handle_starttag(self, tag, attrs):
        if tag in ('script', 'style', 'head'):
            self.hidden += 1
        elif not self.hidden and tag in ('p', 'br', 'div', 'li', 'tr'):
            self.parts.append('\n')

    def handle_endtag(self, tag):
        if tag in ('script', 'style', 'head'):
            self.hidden = max(0, self.hidden - 1)

    def handle_data(self, data):
        if not self.hidden:
            self.parts.append(data)


def decode_message(raw, uid):
    message = BytesParser(policy=policy.default).parsebytes(raw)
    name, address = parseaddr(str(message.get('From', '')))
    part = message.get_body(preferencelist=('plain', 'html')) if message.is_multipart() else message
    try:
        body = part.get_content() if part else ''
    except (LookupError, UnicodeError):
        body = (part.get_payload(decode=True) or b'').decode('utf-8', errors='replace') if part else ''
    if not isinstance(body, str):
        body = ''
    if part and part.get_content_type() == 'text/html':
        parser = PlainHTML()
        parser.feed(body)
        body = ''.join(parser.parts)
    html_part=message.get_body(preferencelist=('html',))
    html=''
    if html_part and html_part.get_content_type()=='text/html':
        try:html=html_part.get_content()
        except (LookupError,UnicodeError):html=(html_part.get_payload(decode=True) or b'').decode('utf-8',errors='replace')
        if not isinstance(html,str):html=''
    inline_images={}
    if html:
        for image in message.walk():
            cid=str(image.get('Content-ID','')).strip('<>')
            mime=image.get_content_type()
            if cid and mime in ('image/png','image/jpeg','image/gif','image/webp','image/svg+xml'):
                payload=image.get_payload(decode=True)
                if payload:inline_images[cid]='data:'+mime+';base64,'+base64.b64encode(payload).decode('ascii')
    body = re.sub(r'\n[ \t]*\n(?:[ \t]*\n)+', '\n\n', body).strip()
    try:
        date = parsedate_to_datetime(str(message.get('Date', ''))).astimezone(timezone.utc)
    except (ValueError, TypeError, OverflowError):
        date = datetime.now(timezone.utc)
    attachment_items=[]
    for i,item in enumerate(message.iter_attachments()):
        if item.get_content_disposition()=='inline' and item.get('Content-ID'):continue
        payload=item.get_payload(decode=True)
        if payload is None and item.get_content_type()=='message/rfc822':
            nested=item.get_payload();payload=b'\r\n'.join(m.as_bytes() for m in nested) if isinstance(nested,list) else b''
        payload=payload or b''
        attachment_items.append({'part':i,'filename':item.get_filename() or f'attachment-{i+1}',
            'mime':item.get_content_type(),'size':len(payload),'sha256':hashlib.sha256(payload).hexdigest(),
            'data':base64.b64encode(payload).decode('ascii')})
    return {'id': hashlib.sha256(uid.encode()).hexdigest()[:24], 'uid': uid,
            'message_id':str(message.get('Message-ID','')).strip(),'reply_to':parseaddr(str(message.get('Reply-To',message.get('From',''))))[1],
            'references':str(message.get('References','')).replace('\r','').replace('\n',' '),'attachment_items':attachment_items,
            'sender': name or address or 'Unknown sender', 'address': address,
            'subject': str(message.get('Subject', '(No subject)')) or '(No subject)',
            'body': body or '(No readable message body)', 'preview': ' '.join(body.split())[:180],
            'html':html, 'inline_images':inline_images,
            'date': date.isoformat(), 'time': date.astimezone().strftime('%b %d'),
            'attachments': len(attachment_items), 'unread': True,
            'flagged': False, 'archived': False, 'source': 'gmail'}


def fetch_gmail(limit=25, *, uids=None, exclude_uids=None, account=None, max_message=MAX_MESSAGE):
    """Use recent mode, verified TLS, stable UIDL identity and a bounded download."""
    if account is None:
        account=load_account()
    account=candidate(account,account.get('password',''))
    limit = max(1, min(int(limit), 100))
    client = connect(account)
    messages, skipped, skipped_uids = [], 0, []
    try:
        count, total_bytes = client.stat()
        uid_rows = client.uidl()[1]
        if uids is not None:uid_rows=[row for row in uid_rows if row.decode('ascii').split(maxsplit=1)[1] in uids]
        if exclude_uids is not None:uid_rows=[row for row in uid_rows if row.decode('ascii').split(maxsplit=1)[1] not in exclude_uids]
        sizes = {int(row.split()[0]): int(row.split()[1]) for row in client.list()[1]}
        for row in reversed(uid_rows[-limit:]):
            number, uid = row.decode('ascii').split(maxsplit=1)
            number = int(number)
            if sizes.get(number, max_message + 1) > max_message:
                skipped += 1
                skipped_uids.append(uid)
                continue
            raw = b'\r\n'.join(client.retr(number)[1])
            if len(raw) <= max_message:
                messages.append(decode_message(raw, uid))
            else:
                skipped += 1
                skipped_uids.append(uid)
        return {'address': account['address'], 'account_id':account_id(account), 'messages': messages, 'available': count,
                'total_bytes': total_bytes, 'skipped_large': skipped,
                'skipped_uids':skipped_uids,
                **({'has_more':len(uid_rows)>limit} if exclude_uids is not None else {}),
                'synced_at': datetime.now(timezone.utc).isoformat(), 'mode': 'recent' if account['recent'] else 'standard',
                'host': account['host'], 'port': int(account['port']), 'security':account['security'], 'tls_verified': True}
    finally:
        disconnect(client)


def save_live(result):
    PRIVATE.mkdir(parents=True, exist_ok=True, mode=0o700)
    path = PRIVATE / 'mailbox.json'
    old = json.loads(path.read_text()) if path.exists() else {'messages': []}
    result=merge_mailbox(old,result)
    atomic_json(path,result)
    return result


def merge_mailbox(old,result):
    if (old.get('account_id') and result.get('account_id') and old['account_id']!=result['account_id']) or (old.get('address')=='you@gmail.com' and result.get('address') not in (None,'you@gmail.com')):
        old={'messages':[]}
    explicit_more=result.get('has_more')
    by_id = {m['id']: m for m in old['messages']}
    def channel(m):
        uid=str(m.get('uid',''))
        return 'imap' if uid.startswith('imap:') else 'sent' if uid.startswith('sent:') else 'pop'
    by_rfc={}
    by_gmail={m['gmail_id']:m for m in old['messages'] if m.get('gmail_id')}
    for m in old['messages']:
        if m.get('message_id'):by_rfc.setdefault(m['message_id'],[]).append(m)
    for message in result['messages']:
        previous=by_id.get(message['id'])
        if previous is None and message.get('gmail_id'):previous=by_gmail.get(message['gmail_id'])
        if previous is None:
            matches=by_rfc.get(message.get('message_id'),[])
            if len(matches)==1 and channel(matches[0])!=channel(message):
                match=matches[0]
                if not (match.get('gmail_id') and message.get('gmail_id') and match['gmail_id']!=message['gmail_id']):previous=match
        if previous is not None:
            uid=message.get('uid') if channel(message)=='pop' else previous.get('uid',message.get('uid'))
            message={**message,'id':previous['id'],'uid':uid}
            if not message.get('server_state'):message.update({k: previous.get(k, False) for k in ('unread', 'flagged', 'archived')})
            message={**previous,**message}
        by_id[message['id']] = message
        if message.get('gmail_id'):by_gmail[message['gmail_id']]=message
        if message.get('message_id'):
            entries=by_rfc.setdefault(message['message_id'],[])
            entries[:]=[m for m in entries if m['id']!=message['id']]+[message]
    result = {**old, **result, 'skipped_uids':sorted(set(old.get('skipped_uids',[]))|set(result.get('skipped_uids',[]))),
              'messages': sorted(by_id.values(), key=lambda m: m['date'], reverse=True)}
    if explicit_more is None:
        known=sum(1 for m in result['messages'] if not str(m.get('uid','')).startswith(('imap:','sent:')))
        result['has_more']=result.get('available',0)>known+len(result['skipped_uids'])
    return result


def fixture():
    rows = [
        ('Alex Morgan', 'Weekend plans', "Hi there,\n\nLet's meet by the water on Saturday. I found a quiet spot with great coffee and a view of the boats.\n\nHow does 10:30 sound?\n\nSee you soon,\nAlex", '9:24 AM'),
        ('Studio Team', 'September design review', 'The updated screens are ready for a look.\n\nPlease review the inbox and compose flows before our next design review.\n\nThanks,\nThe team', '8:40 AM'),
        ('Jamie Lee', 'A little inspiration', 'I thought you would enjoy this collection.\n\nA few ideas for our next project.\n\nJamie', 'Yesterday'),
        ('Travel Notes', 'Your next chapter', 'A few places worth exploring.\n\nTake the scenic route this weekend.', 'Yesterday'),
        ('Casey Park', 'Coffee next week?', 'Are you free on Tuesday morning?\n\nThere is a new cafe around the corner.\n\nCasey', 'Monday')]
    return {'address': 'you@gmail.com', 'available': 5, 'synced_at': None,
            'messages': [{'id': f'demo-{i}', 'sender': name, 'address': f'person{i}@example.com',
                'subject': subject, 'body': body, 'preview': ' '.join(body.split()), 'time': time,
                'date': f'2026-09-{15-i:02d}T09:24:00+00:00', 'unread': i < 2,
                'flagged': i == 0, 'archived': False, 'attachments': 0, 'source': 'demo'}
                for i, (name, subject, body, time) in enumerate(rows)]}
