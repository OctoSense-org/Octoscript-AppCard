/** Browser preview controller: the real session (../service.mjs) and sync client
 * (../sync.mjs) driving an HTML rendering of the storyboard. Query parameters:
 *   device=alex-phone|sam-desktop   which device this tab is
 *   server=http://127.0.0.1:8190    the sync server (omit for the offline demo)
 *   token=…                         the server's bearer token
 *   locale=cn|en */
import {createSession, getView, activateControl, nextUpdate, goBack, restart, errorMessage} from '../service.mjs';
import {createSyncClient, SessionSync} from '../sync.mjs';

const params = new URLSearchParams(location.search);
const device = params.get('device') || 'alex-phone';
const locale = params.get('locale') || 'cn';
const root = document.querySelector('[data-calendar-preview]');
const $ = selector => root.querySelector(selector);
const scenes = await (await fetch(new URL('./scenes.json', import.meta.url))).json();
const log = [];
const note = line => { log.unshift(`${new Date().toISOString().slice(11, 19)} ${line}`); $('[data-log]').textContent = log.slice(0, 40).join('\n'); };

let eventCounter = 0;
const eventId = () => `${device}-${Date.now().toString(36)}-${++eventCounter}`;
let session = createSession({locale, device});
let sync = null;
if (params.get('server')) {
  const client = createSyncClient({baseUrl: params.get('server'), token: params.get('token') || '', device});
  sync = new SessionSync(session, client, {onChange: s => { session = s; render(); }});
}

function argbToCss(value) { const n = Number(value) >>> 0; return `rgba(${(n >> 16) & 255}, ${(n >> 8) & 255}, ${n & 255}, ${((n >>> 24) & 255) / 255})`; }

function render() {
  const view = getView(session);
  const scene = scenes.scenes[String(view.frameId)];
  const screen = $('[data-screen]');
  if (screen.dataset.sceneFrame !== String(view.frameId)) { screen.innerHTML = scene.html; screen.dataset.sceneFrame = String(view.frameId); }
  $('[data-phone]').classList.toggle('desktop', scene.surface === 'desktop');
  for (const [id, text] of Object.entries(view.nativeText)) { const el = screen.querySelector(`[data-text="${id}"]`); if (el) el.textContent = text; }
  for (const el of screen.querySelectorAll('[data-node]')) el.style.removeProperty('--restyled');
  for (const [id, style] of Object.entries(view.nativeStyles)) {
    const el = screen.querySelector(`[data-node="${id}"]`);
    if (!el) continue;
    if (style.bg !== undefined) el.style.background = argbToCss(style.bg);
    if (style.color !== undefined) el.style.color = argbToCss(style.color);
  }
  const primary = view.guidance.primary?.sourceId;
  for (const el of screen.querySelectorAll('[data-control]')) {
    const id = el.dataset.control;
    const enabled = view.nativeEnabled[id] !== false;
    el.classList.toggle('disabled', !enabled);
    el.classList.toggle('primary', id === primary);
    el.title = view.nativeControls.find(c => c.sourceId === id)?.label || id;
    el.onclick = () => act(id);
  }
  $('[data-guide-title]').textContent = view.guidance.title;
  $('[data-guide-text]').textContent = view.guidance.description;
  const next = $('[data-next-update]');
  next.hidden = !view.nextUpdate; if (view.nextUpdate) next.textContent = view.nextUpdate.label;
  $('[data-back]').disabled = !view.canGoBack;
  $('[data-seq]').textContent = view.seq; $('[data-pending]').textContent = view.pending; $('[data-frame]').textContent = view.frameId;
  $('[data-device-label]').textContent = view.state.devices[device].name[view.locale];
  const dot = $('[data-conn-dot]'); dot.className = 'dot ' + (sync ? (view.connected ? 'on' : 'off') : '');
  $('[data-conn-text]').textContent = sync ? (view.connected ? `synced with ${params.get('server')}` : 'connecting…') : 'offline demo (no server)';
  $('[data-notices]').innerHTML = view.notices.slice(-3).map(n => `<span class="notice">${n.action} refused: ${n.reason}</span>`).join('');
  document.documentElement.lang = view.locale === 'en' ? 'en' : 'zh';
  root.dataset.currentFrame = view.frameId; root.dataset.renderId = view.renderId;
}

function act(sourceId) {
  const view = getView(session);
  try { session = activateControl(session, sourceId, eventId(), view.renderId); note(`tap ${sourceId} → frame ${session.ui.frame}${session.pending.length ? ` (${session.pending.length} pending)` : ''}`); }
  catch (error) { note(`✗ ${sourceId}: ${errorMessage(error, view.locale)}`); return; }
  afterChange();
}
function afterChange() {
  if (sync) { sync.update(session); sync.sync().catch(error => note('sync failed: ' + error.message)); }
  render();
}
$('[data-next-update]').onclick = () => { try { session = nextUpdate(session, eventId()); note('showing the synced change'); } catch (error) { note(errorMessage(error, session.locale)); } afterChange(); };
$('[data-back]').onclick = () => { try { session = goBack(session); } catch (error) { note(errorMessage(error, session.locale)); } afterChange(); };
$('[data-restart]').onclick = () => { session = restart(session); note('restart'); afterChange(); };
for (const button of root.querySelectorAll('[data-locale]')) button.onclick = () => { const url = new URL(location.href); url.searchParams.set('locale', button.dataset.locale); location.href = url; };

render();
if (sync) {
  try { await sync.connect(); note(`connected · log seq ${session.remote.seq}`); render(); sync.follow({wait: 25}).catch(error => note('follow stopped: ' + error.message)); }
  catch (error) { note('cannot connect: ' + error.message); }
}
window.__calendarPreview = {get session() { return session; }, getView: () => getView(session), act, sync};
