// Browser smoke for the preview: two Chromium pages (phone and desktop) against
// one live Python server. The phone creates the piano lesson; the desktop is
// offered the change and shows the synced card; a conflicting slot stays
// disabled on both. Writes screenshots to wizard/preview/smoke-evidence/<stamp>/.
//   node wizard/preview/smoke.mjs      (uses $BEAUTY_PYTHON or python3, and the website's Playwright)
import {createRequire} from 'node:module';
import {spawn} from 'node:child_process';
import {mkdirSync, mkdtempSync, rmSync, writeFileSync, readFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import http from 'node:http';

const here = path.dirname(fileURLToPath(import.meta.url));
const app = path.resolve(here, '../..');
const website = path.resolve(app, '../..', '..');
const {chromium} = createRequire(path.join(website, 'package.json'))('playwright');
const python = process.env.BEAUTY_PYTHON || 'python3';
const TOKEN = 'preview-smoke-token';
const stamp = new Date().toISOString().replace(/[:.]/g, '-');
const evidence = path.join(here, 'smoke-evidence', stamp);
mkdirSync(evidence, {recursive: true});

function serveStatic(rootDir) {
  const types = {'.html': 'text/html', '.mjs': 'text/javascript', '.js': 'text/javascript', '.json': 'application/json', '.css': 'text/css', '.ttf': 'font/ttf'};
  const server = http.createServer((req, res) => {
    let file = path.join(rootDir, decodeURIComponent(new URL(req.url, 'http://x').pathname));
    if (file.endsWith('/')) file += 'index.html';
    try { const body = readFileSync(file); res.writeHead(200, {'content-type': types[path.extname(file)] || 'application/octet-stream'}); res.end(body); }
    catch { res.writeHead(404); res.end('not found'); }
  });
  return new Promise(resolve => server.listen(0, '127.0.0.1', () => resolve({server, port: server.address().port})));
}

const dir = mkdtempSync(path.join(tmpdir(), 'calendar-preview-'));
spawn(python, [path.join(app, 'scripts/export_preview.py')], {stdio: 'inherit'}).on('exit', () => {});
await new Promise(resolve => setTimeout(resolve, 1500));
const {server: staticServer, port: staticPort} = await serveStatic(app);
const origin = `http://127.0.0.1:${staticPort}`;
const child = spawn(python, [path.join(app, 'server/calendar_server.py'), '--db', path.join(dir, 'calendar.sqlite'), '--port', '0', '--token', TOKEN, '--allow-origin', origin], {stdio: ['ignore', 'pipe', 'pipe']});
const serverUrl = await new Promise((resolve, reject) => { let out = ''; child.stdout.on('data', c => { out += c; const m = out.match(/listening on (http:\/\/[^ ]+)/); if (m) resolve(m[1]); }); child.stderr.on('data', c => { out += c; }); setTimeout(() => reject(new Error('server: ' + out)), 15000); });

const browser = await chromium.launch();
const report = {stamp, serverUrl, steps: []};
const step = (name, ok, detail) => { report.steps.push({name, ok, detail}); console.log((ok ? 'ok  ' : 'FAIL') + ' ' + name + (detail ? ' — ' + detail : '')); if (!ok) process.exitCode = 1; };
try {
  const open = async (device, locale) => {
    const page = await browser.newPage({viewport: {width: 900, height: 820}, deviceScaleFactor: 1});
    page.on('pageerror', error => step(`${device} page error`, false, error.message));
    await page.goto(`${origin}/wizard/preview/?device=${device}&server=${serverUrl}&token=${TOKEN}&locale=${locale}`);
    await page.waitForFunction(() => window.__calendarPreview?.getView().connected, null, {timeout: 15000});
    return page;
  };
  const phone = await open('alex-phone', 'cn'), desktop = await open('sam-desktop', 'en');
  const view = page => page.evaluate(() => { const v = window.__calendarPreview.getView(); return {frame: v.frameId, seq: v.seq, pending: v.pending, next: v.nextUpdate?.label || null, enabled: v.nativeEnabled, text: v.nativeText, piano: v.state.events.piano ? {start: v.state.events.piano.start, deleted: v.state.events.piano.deleted} : null}; });
  const tap = async (page, id) => { await page.click(`[data-control="${id}"]`); await page.waitForTimeout(150); };
  await phone.screenshot({path: path.join(evidence, 'phone-01-month.png')});
  await tap(phone, 'add_event'); await tap(phone, 'pick_time');
  let v = await view(phone);
  step('phone: 14:00 slot is disabled by the Team sync conflict', v.frame === 7 && v.enabled.slot_14 === false, JSON.stringify({frame: v.frame, slot_14: v.enabled.slot_14}));
  await phone.screenshot({path: path.join(evidence, 'phone-02-picker.png')});
  await tap(phone, 'slot_16'); await tap(phone, 'save_event');
  await phone.waitForFunction(() => window.__calendarPreview.getView().pending === 0 && window.__calendarPreview.getView().seq >= 1, null, {timeout: 10000});
  v = await view(phone);
  step('phone: piano lesson accepted by the server', v.frame === 5 && v.seq === 1 && v.piano && !v.piano.deleted, JSON.stringify({seq: v.seq, piano: v.piano}));
  await phone.screenshot({path: path.join(evidence, 'phone-03-synced-card.png')});
  await desktop.waitForFunction(() => window.__calendarPreview.getView().seq >= 1, null, {timeout: 10000});
  v = await view(desktop);
  step('desktop: replica has the event and is offered the change without moving', v.frame === 1 && v.piano && v.next === 'Show 1 change from Alex · Phone', JSON.stringify({frame: v.frame, next: v.next}));
  await desktop.click('[data-next-update]'); await desktop.waitForTimeout(150);
  v = await view(desktop);
  step('desktop: synced card shows the phone’s event', v.frame === 5 && v.text.card_title === 'Piano lesson' && v.text.card_source.endsWith('from Alex · Phone'), JSON.stringify({title: v.text.card_title, source: v.text.card_source}));
  await desktop.screenshot({path: path.join(evidence, 'desktop-01-synced-card.png')});
  await tap(desktop, 'undo_event');
  await desktop.waitForFunction(() => window.__calendarPreview.getView().pending === 0 && window.__calendarPreview.getView().seq >= 2, null, {timeout: 10000});
  await phone.waitForFunction(() => window.__calendarPreview.getView().seq >= 2, null, {timeout: 10000});
  v = await view(phone);
  step('phone: the desktop’s undo deleted the event in the phone replica', v.piano?.deleted === true && v.next === '查看 1 项来自 Sam · 桌面 的更改', JSON.stringify({piano: v.piano, next: v.next}));
  await desktop.screenshot({path: path.join(evidence, 'desktop-02-removed-card.png')});
  await tap(desktop, 'undo_delete');
  await phone.waitForFunction(() => window.__calendarPreview.getView().seq >= 3 && !window.__calendarPreview.getView().state.events.piano.deleted, null, {timeout: 10000});
  v = await view(phone);
  step('phone: restore from the desktop arrived', v.piano?.deleted === false && v.seq === 3, JSON.stringify(v.piano));
  const digests = await Promise.all([phone, desktop].map(p => p.evaluate(async () => { const m = await import('../core.mjs'); const {stateOf} = await import('../service.mjs'); return m.stateDigest(stateOf(window.__calendarPreview.session)); })));
  const snapshot = await (await fetch(serverUrl + '/v1/state', {headers: {authorization: 'Bearer ' + TOKEN}})).json();
  step('both replicas equal the server digest', digests[0] === snapshot.digest && digests[1] === snapshot.digest, snapshot.digest.slice(0, 16));
} finally {
  await browser.close(); child.kill(); staticServer.close(); rmSync(dir, {recursive: true, force: true});
  writeFileSync(path.join(evidence, 'report.json'), JSON.stringify(report, null, 2) + '\n');
  console.log('evidence:', evidence);
}
