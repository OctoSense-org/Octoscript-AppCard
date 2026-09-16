/** Two browser sessions against the real Python server: the phone creates, the
 * desktop sees it; a conflicting desktop change is refused; both replicas match
 * the server's digest. Needs a Python 3 interpreter: $BEAUTY_PYTHON, or python3. */
import test from 'node:test';
import assert from 'node:assert/strict';
import {spawn} from 'node:child_process';
import {mkdtempSync, rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {createSession, getView, activateControl, stateOf} from './service.mjs';
import * as core from './core.mjs';
import {createSyncClient, SessionSync, SyncError} from './sync.mjs';

const here = path.dirname(fileURLToPath(import.meta.url));
const python = process.env.BEAUTY_PYTHON || process.env.SERVICE_PYTHON || 'python3';
const TOKEN = 'sync-test-token';
let event = 0;
const click = (s, id) => activateControl(s, id, `sync-${++event}`, getView(s).renderId);

async function startServer() {
  const dir = mkdtempSync(path.join(tmpdir(), 'calendar-sync-'));
  const child = spawn(python, [path.join(here, '../server/calendar_server.py'), '--db', path.join(dir, 'calendar.sqlite'), '--port', '0', '--token', TOKEN], {stdio: ['ignore', 'pipe', 'pipe']});
  const url = await new Promise((resolve, reject) => {
    let out = '';
    child.stdout.on('data', chunk => { out += chunk; const m = out.match(/listening on (http:\/\/[^ ]+)/); if (m) resolve(m[1]); });
    child.stderr.on('data', chunk => { out += chunk; });
    child.on('exit', code => reject(new Error(`server exited ${code}: ${out}`)));
    setTimeout(() => reject(new Error('server did not start: ' + out)), 15000);
  });
  return {url, stop() { child.kill(); rmSync(dir, {recursive: true, force: true}); }};
}

test('two devices converge through the server', async t => {
  const server = await startServer();
  t.after(() => server.stop());
  const phone = new SessionSync(createSession({device: 'alex-phone'}), createSyncClient({baseUrl: server.url, token: TOKEN, device: 'alex-phone'}));
  const desktop = new SessionSync(createSession({device: 'sam-desktop', locale: 'en'}), createSyncClient({baseUrl: server.url, token: TOKEN, device: 'sam-desktop'}));
  await phone.connect(); await desktop.connect();
  assert.equal(phone.current.connected, true);
  assert.equal(getView(phone.current).nextUpdate, null, 'connected: no simulated desktop');

  // Phone creates the piano lesson and pushes it.
  phone.update(click(click(phone.current, 'add_event'), 'save_event'));
  assert.equal(phone.current.pending.length, 1);
  await phone.sync();
  assert.equal(phone.current.pending.length, 0, 'confirmed by the server');
  assert.equal(phone.current.remote.seq, 1);
  assert.equal(stateOf(phone.current).events.piano.created_by, 'alex-phone');

  // Desktop pulls (long-poll returns at once because the record is there) and is offered the change.
  await desktop.pull(5);
  assert.equal(desktop.current.remote.seq, 1);
  assert.ok(stateOf(desktop.current).events.piano);
  const offered = getView(desktop.current).nextUpdate;
  assert.equal(offered.label, 'Show 1 change from Alex · Phone');
  assert.equal(desktop.current.ui.frame, 1, 'the desktop screen did not move by itself');

  // Desktop tries an overlapping event on another calendar: refused by the server, rolled back locally.
  let d = click(desktop.current, 'add_event');
  d = {...d, ui: {...d.ui, draft: {...core.PIANO, id: 'sam-overlap', calendar: 'work', start: '2026-09-25T16:30:00+08:00', end: '2026-09-25T17:30:00+08:00'}}};
  Object.freeze(d); Object.freeze(d.ui);
  assert.equal(getView(d).nativeEnabled.save_event, false, 'the phone’s event is already in the desktop replica, so the editor refuses the overlap');
  // Force the submission anyway to exercise the server verdict path.
  const op = {id: 'sam-desktop:forced', actor: 'sam-desktop', action: 'event.create', payload: {event: d.ui.draft}};
  const record = await desktop.client.submit(op);
  assert.equal(record.accepted, false);
  assert.equal(record.reason, 'conflict:piano');

  // Desktop answers the invitation; the phone sees it and both digests equal the server's.
  desktop.update(click(click(desktop.current, 'tab_inbox'), 'maybe_invite'));
  await desktop.sync();
  await phone.pull(5);
  assert.equal(stateOf(phone.current).invitations.hike.status, 'maybe');
  assert.equal(stateOf(phone.current).invitations.hike.answered_by, 'sam-desktop');
  const snapshot = await phone.client.bootstrap();
  assert.equal(await core.stateDigest(stateOf(phone.current)), snapshot.digest);
  assert.equal(await core.stateDigest(stateOf(desktop.current)), snapshot.digest);
  assert.equal(snapshot.seq, 3, 'two accepted records and one rejection are all in the log');
  assert.equal(phone.current.remote.seq, 3);

  // A wrong token is refused before anything is read.
  const stranger = createSyncClient({baseUrl: server.url, token: 'nope', device: 'alex-phone'});
  await assert.rejects(stranger.bootstrap(), error => error instanceof SyncError && error.status === 401);
});

test('follow() delivers the other device’s change while waiting', async t => {
  const server = await startServer();
  t.after(() => server.stop());
  const phone = new SessionSync(createSession({device: 'alex-phone'}), createSyncClient({baseUrl: server.url, token: TOKEN, device: 'alex-phone'}));
  const changes = [];
  const desktop = new SessionSync(createSession({device: 'sam-desktop'}), createSyncClient({baseUrl: server.url, token: TOKEN, device: 'sam-desktop'}), {onChange: s => changes.push(s.remote.seq)});
  await phone.connect(); await desktop.connect();
  const controller = new AbortController();
  const following = desktop.follow({signal: controller.signal, wait: 5});
  await new Promise(resolve => setTimeout(resolve, 200));
  phone.update(click(phone.current, 'tab_calendars'));
  phone.update(click(phone.current, 'toggle_birthdays'));
  await phone.sync();
  const started = Date.now();
  while (!stateOf(desktop.current).calendars.birthdays.visible && Date.now() - started < 5000) await new Promise(resolve => setTimeout(resolve, 50));
  assert.equal(stateOf(desktop.current).calendars.birthdays.visible, true);
  assert.ok(changes.includes(1));
  controller.abort();
  await following;
});
