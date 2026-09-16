/** The Calendar session: which screen is showing, what the user is drafting, and
 * the device's replica of the synced calendar (wizard/core.mjs).
 *
 * Pure and local, like the other wizards: every call returns a new frozen
 * session. Network is someone else's job — a user action that changes the
 * calendar produces an operation in `session.pending`; wizard/sync.mjs sends
 * pending operations to the server and folds the server's verdicts (and the
 * other device's operations) back in with `applyRecords`. Until a verdict
 * arrives the screen shows the optimistic state; a rejection rolls it back
 * and leaves a notice. A remote change never moves the screen by itself: it
 * is offered as `view.nextUpdate` and shown when the user asks. */
import {frames} from './copy.mjs';
import * as core from './core.mjs';

const freeze = value => { if (value && typeof value === 'object' && !Object.isFrozen(value)) { Object.freeze(value); for (const child of Object.values(value)) freeze(child); } return value; };
const clone = value => structuredClone(value);
const t = (locale, cn, en) => locale === 'en' ? en : cn;
const appFrames = new Set([1, 2, 3, 4, 7, 8]);
const ROWS = ['open_dentist', 'open_team_sync', 'open_dinner'];
const SLOTS = {slot_13: ['13:00', '14:00'], slot_14: ['14:00', '15:00'], slot_15: ['15:00', '16:00'], slot_16: ['16:00', '17:00'], slot_17: ['17:00', '18:00']};
const TOGGLES = {toggle_family: 'family', toggle_work: 'work', toggle_personal: 'personal', toggle_birthdays: 'birthdays'};
const CLOCK = '2026-09-24T09:41:00+08:00';
const SAM_MOVES_DINNER = {id: 'sam-desktop:moves-dinner', actor: 'sam-desktop', action: 'event.update', payload: {id: 'dinner', patch: {start: '2026-09-24T19:30:00+08:00', end: '2026-09-24T21:00:00+08:00'}}};

export const scenario = freeze({id: 'calendar', title: {cn: '一本日历，两台设备', en: 'One calendar, two devices'},
  description: {cn: '手机上的每一次更改都经日历服务记录并同步到 Sam 的桌面；冲突会被拒绝，撤销只针对指定的那一步', en: 'Every change on the phone is recorded by the calendar service and synced to Sam’s desktop; conflicts are refused, undo names the step it reverses'},
  phases: {cn: ['浏览日历', '新建日程', '桌面卡片', '同步状态'], en: ['Browse', 'Create', 'Desktop cards', 'Sync']}});
export const FIXTURE = core.FIXTURE;

const guides = {
  1: ['九月', '今天有三项日程。点一行查看详情，或用 + 新建；更改会同步到 Sam 的桌面', 'September', 'Three events today. Open a row for details or tap + to create; changes sync to Sam’s desktop'],
  2: ['一天的安排', '晚餐是 Sam 在桌面上加的，已同步到手机。点开任一项查看', 'The day', 'Dinner was added by Sam on the desktop and synced here. Open any event'],
  3: ['日程详情', '删除只移除这一项，并同步到 Sam；撤销可以恢复同一条日程', 'Event details', 'Delete removes this one event and syncs to Sam; undo restores the same event'],
  4: ['新建日程', '选择时间时冲突的时段不可选。添加后日程发送到日历服务，Sam 的桌面会收到卡片', 'New event', 'Conflicting slots are disabled in the time picker. Add sends the event to the service; Sam’s desktop gets a card'],
  5: ['已同步', '这张卡片报告另一台设备已经完成的更改。知道了只表示已读；撤销只针对这一项', 'Synced', 'This card reports a change already made on the other device. Got it acknowledges; Undo affects only this item'],
  6: ['来自 Sam 的邀请', '接受会把远足加入你的日历并回复 Sam；拒绝不会删除 Sam 的日程', 'Invitation from Sam', 'Accept adds the hike to your calendar and answers Sam; Decline never deletes Sam’s event'],
  7: ['选择时间', '与现有日程重叠的时段保持禁用，服务端也会拒绝重叠', 'Choose a time', 'Slots overlapping an existing event stay disabled; the service refuses overlaps too'],
  8: ['日历', '显示或隐藏日历只改变可见性，不会删除任何日程', 'Calendars', 'Showing or hiding a calendar changes visibility only; nothing is deleted'],
  9: ['同步状态', '两台设备读取同一份日志。立即同步只记录一次同步，不会代你做任何更改', 'Sync status', 'Both devices read the same log. Sync now records a sync; it never changes the calendar for you'],
  10: ['已删除', '这一项在手机上删除并已同步。撤销删除会恢复同一条日程', 'Removed', 'This event was deleted on the phone and synced. Undo delete restores the same event'],
};

export function createSession({locale = 'cn', device = 'alex-phone'} = {}) {
  if (!['en', 'cn'].includes(locale)) throw new TypeError('Unsupported locale');
  if (!Object.hasOwn(core.FIXTURE.devices, device)) throw new TypeError('Unknown device');
  return freeze({schemaVersion: 1, locale, device, epoch: 0, revision: 0, connected: false,
    ui: {frame: 1, surface: 'app', returnTo: 1, selected: 'dinner', draft: null, editing: false, synced: null, removed: null, acknowledged: [], unseen: [], demoDelivered: false},
    remote: {seq: 0, state: core.initialState()}, pending: [], notices: [], history: [], events: []});
}

function renderId(s) { return `calendar:${s.epoch}:${s.revision}:${s.ui.frame}`; }
function failure(code) { const error = new Error(code); error.code = code; return error; }
function setFrame(ui, frame) { ui.frame = frame; ui.surface = appFrames.has(frame) ? 'app' : 'desktop'; }
function commit(s, ui, pending, eventId, extra = {}) {
  return freeze({...s, ...extra, revision: s.revision + 1, ui, pending, history: [...s.history, {ui: clone(s.ui), pending: clone(s.pending)}], events: eventId ? [...s.events, eventId] : s.events});
}

/** The replica as this device sees it: confirmed state plus its own unconfirmed operations. */
export function stateOf(session) {
  let state = session.remote.state;
  for (const op of session.pending) { try { ({state} = core.apply(state, op)); } catch (error) { if (!(error instanceof core.Rejected)) throw error; } }
  return state;
}

// -- formatting ---------------------------------------------------------------
const WD_CN = '日一二三四五六', WD_EN = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'], MON_EN = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
function weekday(iso) { const [y, m, d] = iso.slice(0, 10).split('-').map(Number); return new Date(Date.UTC(y, m - 1, d)).getUTCDay(); }
export function fmtDate(iso, locale) { const m = Number(iso.slice(5, 7)), d = Number(iso.slice(8, 10)), wd = weekday(iso); return locale === 'en' ? `${WD_EN[wd]}, ${MON_EN[m - 1]} ${d}` : `${m}月${d}日 周${WD_CN[wd]}`; }
export const fmtTime = iso => iso.slice(11, 16);
const range = (e, sep = ' – ') => fmtTime(e.start) + sep + fmtTime(e.end);
const name = (pair, locale) => pair?.[locale] || pair?.cn || pair?.en || '';
const deviceName = (state, id, locale) => name(state.devices[id]?.name, locale) || id;
const calName = (state, id, locale) => name(state.calendars[id]?.name, locale);
const today = core.FIXTURE.today;
const dayEvents = state => core.visibleEvents(state, today);
function slotEvent(ui, slot) { const [a, b] = SLOTS[slot]; return {...(ui.draft || core.PIANO), start: `${today}T${a}:00+08:00`, end: `${today}T${b}:00+08:00`}; }
function draftValid(state, ui) { const d = ui.draft; if (!d) return false; try { core.apply(state, {id: 'probe', actor: 'alex-phone', action: ui.editing ? 'event.update' : 'event.create', payload: ui.editing ? {id: d.id, patch: pick(d)} : {event: d}}); return true; } catch { return false; } }
const pick = d => ({title: d.title, calendar: d.calendar, start: d.start, end: d.end, location: d.location || {cn: '', en: ''}});

function enabled(session, state, id) {
  const ui = session.ui, frame = ui.frame;
  if (id === 'back_year' || id === 'dock_mail' || id === 'dock_settings') return false;
  if (ROWS.includes(id)) return Boolean(dayEvents(state)[ROWS.indexOf(id)]);
  if (id === 'edit_event' || id === 'delete_event') return Boolean(state.events[ui.selected] && !state.events[ui.selected].deleted);
  if (id === 'save_event') return draftValid(state, ui);
  if (id in SLOTS) return !core.conflict(state, slotEvent(ui, id), ui.editing ? ui.draft?.id : null);
  if (id === 'ack_sync') return Boolean(ui.synced) && !ui.acknowledged.includes(ui.synced.key);
  if (id === 'undo_event') return Boolean(ui.synced?.eventId && state.events[ui.synced.eventId] && !state.events[ui.synced.eventId].deleted && ui.synced.action !== 'event.delete');
  if (id === 'accept_invite') return state.invitations.hike.status !== 'accepted';
  if (id === 'maybe_invite') return state.invitations.hike.status !== 'maybe';
  if (id === 'decline_invite') return state.invitations.hike.status !== 'declined';
  if (id === 'undo_delete') return Boolean(ui.removed && state.events[ui.removed]?.deleted);
  if (frame === 1 && id === 'tab_today') return false;
  if (frame === 8 && id === 'tab_calendars') return false;
  return true;
}

function nativeCopy(session, state) {
  const ui = session.ui, frame = ui.frame, locale = session.locale;
  const out = Object.fromEntries(Object.entries(frames[frame].text).map(([id, pair]) => [id, pair[locale]]));
  const set = (id, cn, en = cn) => { if (Object.hasOwn(out, id)) out[id] = t(locale, cn, en); };
  const evs = dayEvents(state);
  if (frame === 1 || frame === 2) {
    set('list_title', `${fmtDate(CLOCK, 'cn')} · 今天`, `${fmtDate(CLOCK, 'en')} · Today`);
    set('day_sub', `今天 · ${evs.length} 项日程`, `Today · ${evs.length} event${evs.length === 1 ? '' : 's'}`);
    ROWS.forEach((row, i) => {
      const e = evs[i];
      if (!e) { for (const suffix of ['_time', '_label', '_sub', '_start', '_end', '_loc', '_range']) set(row + suffix, i === evs.length ? (suffix === '_label' ? '没有更多日程' : '') : '', i === evs.length ? (suffix === '_label' ? 'No more events' : '') : ''); return; }
      const from = e.created_by !== session.device ? t(locale, ` · 来自 ${deviceName(state, e.created_by, 'cn')}`, ` · from ${deviceName(state, e.created_by, 'en')}`) : '';
      set(row + '_time', fmtTime(e.start)); set(row + '_start', fmtTime(e.start)); set(row + '_end', fmtTime(e.end));
      set(row + '_label', name(e.title, 'cn'), name(e.title, 'en'));
      set(row + '_sub', calName(state, e.calendar, 'cn') + from, calName(state, e.calendar, 'en') + from);
      set(row + '_loc', name(e.location, 'cn') + from, name(e.location, 'en') + from);
      set(row + '_range', range(e));
    });
  }
  if (frame === 3) {
    const e = state.events[ui.selected];
    if (e) {
      set('event_title', name(e.title, 'cn'), name(e.title, 'en'));
      const shared = state.calendars[e.calendar]?.shared_with?.length;
      set('event_calendar', `${calName(state, e.calendar, 'cn')}日历${shared ? ' · 与 Sam 共享' : ''}`, `${calName(state, e.calendar, 'en')} calendar${shared ? ' · shared with Sam' : ''}`);
      set('detail_when', `2026年${fmtDate(e.start, 'cn')}`, `${fmtDate(e.start, 'en')}, 2026`); set('detail_when_sub', range(e));
      set('detail_where', name(e.location, 'cn') || '未设置地点', name(e.location, 'en') || 'No location'); set('detail_where_sub', e.location?.cn ? '南京西路 88 号' : '', e.location?.en ? '88 West Nanjing Rd' : '');
      const by = e.updated_by === session.device ? t(locale, '在本机更改', 'changed on this device') : t(locale, `由 ${deviceName(state, e.updated_by, 'cn')} 更改 · 已同步到本机`, `changed by ${deviceName(state, e.updated_by, 'en')} · synced here`);
      set('sync_note', by, by);
      if (e.deleted) set('delete_event_label', '已删除', 'Deleted');
    }
  }
  if (frame === 4) {
    const d = ui.draft;
    if (d) {
      set('nav_title', ui.editing ? '编辑日程' : '新建日程', ui.editing ? 'Edit Event' : 'New Event');
      set('save_event_label', ui.editing ? '完成' : '添加', ui.editing ? 'Done' : 'Add');
      set('title_field', name(d.title, 'cn'), name(d.title, 'en'));
      set('location_field', name(d.location, 'cn') || '地点或视频通话', name(d.location, 'en') || 'Location or Video Call');
      set('start_value', `${fmtDate(d.start, 'cn')}  ${fmtTime(d.start)}`, `${fmtDate(d.start, 'en')}   ${fmtTime(d.start)}`);
      set('end_value', fmtTime(d.end));
      set('calendar_value', calName(state, d.calendar, 'cn'), calName(state, d.calendar, 'en'));
      const clash = core.conflict(state, d, ui.editing ? d.id : null);
      if (clash) set('sync_hint_text', `与「${name(state.events[clash].title, 'cn')}」重叠，请更换时间`, `Overlaps “${name(state.events[clash].title, 'en')}” — choose another time`);
      else set('sync_hint_text', session.connected ? '保存后会同步到 Sam 的桌面' : '保存后会同步到 Sam 的桌面（演示：本地记录）', session.connected ? 'Saving syncs this to Sam · Desktop' : 'Saving syncs this to Sam · Desktop (demo: recorded locally)');
    }
  }
  if (frame === 5 && ui.synced) {
    const e = state.events[ui.synced.eventId];
    const kind = {'event.create': ['已同步', 'Synced'], 'event.update': ['已更新', 'Updated'], 'event.delete': ['已删除', 'Removed'], 'event.restore': ['已恢复', 'Restored'], 'invite.respond': ['已回复邀请', 'Invitation answered']}[ui.synced.action] || ['已同步', 'Synced'];
    set('calendar_card_kicker', `日历 · ${kind[0]}`, `Calendar · ${kind[1]}`);
    if (e) { set('card_title', name(e.title, 'cn'), name(e.title, 'en')); set('card_when', `${fmtDate(e.start, 'cn')} · ${range(e, '–')}`, `${fmtDate(e.start, 'en')} · ${range(e, '–')}`); }
    set('card_source', `${calName(state, e?.calendar || 'family', 'cn')}日历 · 来自 ${deviceName(state, ui.synced.from, 'cn')}`, `${calName(state, e?.calendar || 'family', 'en')} calendar · from ${deviceName(state, ui.synced.from, 'en')}`);
    if (ui.acknowledged.includes(ui.synced.key)) set('ack_sync_label', '已确认', 'Acknowledged');
  }
  if (frame === 6) {
    const inv = state.invitations.hike, status = inv.status;
    const label = {pending: ['邀请 · 来自 Sam', 'Invitation · from Sam'], accepted: ['已接受 · 已加入日历', 'Accepted · in your calendar'], maybe: ['已回复 · 待定', 'Answered · Maybe'], declined: ['已拒绝', 'Declined']}[status];
    set('invite_card_kicker', label[0], label[1]);
    if (status !== 'pending') set('invite_note', status === 'accepted' ? 'Sam 已收到你的回复；远足已在你的家庭日历' : 'Sam 已收到你的回复；Sam 的日程不受影响', status === 'accepted' ? 'Sam has your answer; the hike is in your Family calendar' : 'Sam has your answer; Sam’s event is unchanged');
  }
  if (frame === 7) {
    set('picker_sub', `${fmtDate(CLOCK, 'cn')} · ${calName(state, ui.draft?.calendar || 'family', 'cn')}日历`, `${fmtDate(CLOCK, 'en')} · ${calName(state, ui.draft?.calendar || 'family', 'en')} calendar`);
    for (const slot of Object.keys(SLOTS)) {
      const clash = core.conflict(state, slotEvent(ui, slot), ui.editing ? ui.draft?.id : null);
      set(slot + '_why', clash ? `与「${name(state.events[clash].title, 'cn')}」冲突` : '', clash ? `Conflicts with “${name(state.events[clash].title, 'en')}”` : '');
    }
  }
  if (frame === 8) {
    for (const [id, cal] of Object.entries(TOGGLES)) {
      const c = state.calendars[cal];
      set(id + '_sub', c.visible ? (c.shared_with.length ? '与 Sam 共享' : '') : '已隐藏', c.visible ? (c.shared_with.length ? 'Shared with Sam' : '') : 'Hidden');
    }
    const p = state.devices['alex-phone'];
    set('sync_status', `已同步 · ${fmtTime(p.last_sync)}${session.pending.length ? ` · ${session.pending.length} 项待发送` : ''}`, `Synced · ${fmtTime(p.last_sync)}${session.pending.length ? ` · ${session.pending.length} pending` : ''}`);
  }
  if (frame === 9) {
    const p = state.devices['alex-phone'], d = state.devices['sam-desktop'];
    set('device_phone', `${deviceName(state, 'alex-phone', 'cn')} · ${fmtTime(p.last_sync)}`, `${deviceName(state, 'alex-phone', 'en')} · ${fmtTime(p.last_sync)}`);
    set('device_desktop', `${deviceName(state, 'sam-desktop', 'cn')} · ${fmtTime(d.last_sync)}`, `${deviceName(state, 'sam-desktop', 'en')} · ${fmtTime(d.last_sync)}`);
    const n = session.pending.length, syncs = (p.sync_count || 0) + 12;
    set('pending_count', `${n} 项待同步更改 · 第 ${syncs} 次同步`, `${n} pending change${n === 1 ? '' : 's'} · sync #${syncs}`);
    set('sync_card_kicker', n ? '同步 · 有待发送更改' : '同步 · 最新', n ? 'Sync · Changes pending' : 'Sync · Up to date');
    set('sync_title', n ? '本机有未发送的更改' : '两台设备一致', n ? 'This device has unsent changes' : 'Both devices agree');
  }
  if (frame === 10 && ui.removed) {
    const e = state.events[ui.removed];
    if (e) {
      set('removed_title', name(e.title, 'cn'), name(e.title, 'en'));
      set('removed_when', `${fmtDate(e.start, 'cn')} · ${range(e, '–')}`, `${fmtDate(e.start, 'en')} · ${range(e, '–')}`);
      if (!e.deleted) { set('calendar_card_kicker', '日历 · 已恢复', 'Calendar · Restored'); set('removed_note', '已恢复同一条日程，并同步到 Sam', 'The same event is back, and synced to Sam'); set('undo_delete_label', '已恢复', 'Restored'); }
    }
  }
  return out;
}

function primaryFor(session, state) {
  const ui = session.ui;
  return {1: 'open_dinner', 2: 'open_dinner', 3: 'delete_event', 4: 'save_event', 5: ui.acknowledged.includes(ui.synced?.key) ? 'open_calendar' : 'ack_sync', 6: 'accept_invite', 7: 'slot_16', 8: 'done_calendars', 9: 'sync_now', 10: state.events[ui.removed]?.deleted ? 'undo_delete' : 'open_calendar'}[ui.frame];
}

export function getView(session) {
  const state = stateOf(session), ui = clone(session.ui), frameId = ui.frame, locale = session.locale, nativeText = nativeCopy(session, state);
  const nativeControls = Object.entries(frames[frameId].controls).map(([sourceId, control]) => ({sourceId, textIds: [...control.text_ids], enabled: enabled(session, state, sourceId),
    label: control.text_ids.length ? control.text_ids.map(id => nativeText[id]).filter(Boolean).join(' · ') || sourceId : sourceId}));
  const nativeEnabled = Object.fromEntries(nativeControls.map(c => [c.sourceId, c.enabled]));
  const nativeStyles = {};
  if (frameId === 5 && ui.acknowledged.includes(ui.synced?.key)) { nativeStyles.ack_sync_surface = {bg: 0xffe5e5ea}; nativeStyles.ack_sync_label = {color: 0xff8e8e93}; }
  if (frameId === 8) for (const [id, cal] of Object.entries(TOGGLES)) nativeStyles[id + '_label'] = {color: state.calendars[cal].visible ? 0xff1c1c1e : 0xff8e8e93};
  if (frameId === 6) for (const [id, status] of [['accept_invite', 'accepted'], ['maybe_invite', 'maybe'], ['decline_invite', 'declined']]) if (state.invitations.hike.status === status) nativeStyles[id + '_surface'] = {bg: 0xffe5e5ea};
  const guide = guides[frameId], primaryId = primaryFor(session, state), control = nativeControls.find(c => c.sourceId === primaryId && c.enabled);
  const unseen = ui.unseen.length > 0, demo = !session.connected && !ui.demoDelivered && [1, 2, 3].includes(frameId);
  const nextUpdate = unseen ? {label: t(locale, `查看 ${ui.unseen.length} 项来自 ${deviceName(state, ui.unseen[0].from, 'cn')} 的更改`, `Show ${ui.unseen.length} change${ui.unseen.length === 1 ? '' : 's'} from ${deviceName(state, ui.unseen[0].from, 'en')}`)}
    : demo ? {label: t(locale, '模拟 Sam 在桌面上更改晚餐时间', 'Simulate Sam moving dinner on the desktop')} : null;
  const completed = Boolean(state.events.piano && !state.events.piano.deleted && state.invitations.hike.status !== 'pending' && ui.acknowledged.length);
  return freeze({schemaVersion: 1, renderId: renderId(session), frameId, locale, phase: frameId <= 3 ? 0 : frameId === 4 || frameId === 7 ? 1 : frameId === 8 ? 3 : 2,
    state, ui, seq: session.remote.seq, connected: session.connected, pending: session.pending.length, notices: clone(session.notices),
    nativeControls, nativeText, nativeEnabled, nativeStyles, nativeLayout: {},
    guidance: {title: guide[locale === 'en' ? 2 : 0], description: guide[locale === 'en' ? 3 : 1], primary: control ? {kind: 'native', sourceId: control.sourceId, label: control.label} : null},
    nextUpdate, canGoBack: session.history.length > 0, completed});
}

function checkEvent(session, eventId) { if (typeof eventId !== 'string' || !eventId.trim()) throw failure('invalid_event'); return session.events.includes(eventId); }
const opFor = (session, eventId, action, payload) => ({id: `${session.device}:${eventId}`, actor: session.device, action, payload});

export function activateControl(session, sourceId, eventId, expectedRenderId) {
  if (checkEvent(session, eventId)) return session;
  if (expectedRenderId !== renderId(session)) throw failure('stale_render');
  const control = getView(session).nativeControls.find(c => c.sourceId === sourceId);
  if (!control) throw failure('unknown_control');
  if (!control.enabled) throw failure('disabled_control');
  const state = stateOf(session), ui = clone(session.ui), pending = clone(session.pending);
  let op = null;
  const go = frame => setFrame(ui, frame);
  const openEditor = (draft, editing) => { ui.draft = draft; ui.editing = editing; ui.returnTo = ui.frame; go(4); };
  switch (sourceId) {
    case 'open_today': go(2); break;
    case 'back_month': case 'tab_today': case 'dock_calendar': case 'open_calendar': case 'done_calendars': go(1); break;
    case 'tab_calendars': go(8); break;
    case 'tab_inbox': go(6); break;
    case 'add_event': openEditor(clone(core.PIANO), false); break;
    case 'open_dentist': case 'open_team_sync': case 'open_dinner': ui.selected = dayEvents(state)[ROWS.indexOf(sourceId)].id; go(3); break;
    case 'back_day': go(2); break;
    case 'edit_event': openEditor({...clone(state.events[ui.selected]), location: state.events[ui.selected].location || {cn: '', en: ''}}, true); break;
    case 'delete_event': op = opFor(session, eventId, 'event.delete', {id: ui.selected}); ui.removed = ui.selected; go(10); break;
    case 'cancel_editor': ui.draft = null; ui.editing = false; go(ui.returnTo); break;
    case 'pick_time': ui.returnTo = 4; go(7); break;
    case 'back_editor': go(4); break;
    case 'pick_calendar': { const order = ['family', 'work', 'personal']; ui.draft.calendar = order[(order.indexOf(ui.draft.calendar) + 1) % order.length]; break; }
    case 'save_event': {
      const d = ui.draft;
      op = ui.editing ? opFor(session, eventId, 'event.update', {id: d.id, patch: pick(d)}) : opFor(session, eventId, 'event.create', {event: {id: d.id, ...pick(d)}});
      ui.synced = {key: op.id, eventId: d.id, from: session.device, action: op.action}; ui.selected = d.id; ui.draft = null; ui.editing = false; go(5); break;
    }
    case 'ack_sync': ui.acknowledged = [...ui.acknowledged, ui.synced.key]; break;
    case 'undo_event': op = opFor(session, eventId, 'event.delete', {id: ui.synced.eventId}); ui.removed = ui.synced.eventId; go(10); break;
    case 'undo_delete': op = opFor(session, eventId, 'event.restore', {id: ui.removed}); ui.selected = ui.removed; break;
    case 'accept_invite': case 'maybe_invite': case 'decline_invite': {
      const status = {accept_invite: 'accepted', maybe_invite: 'maybe', decline_invite: 'declined'}[sourceId];
      op = opFor(session, eventId, 'invite.respond', {id: 'hike', status});
      if (status === 'accepted') { ui.synced = {key: op.id, eventId: 'invite-hike', from: session.device, action: 'invite.respond'}; go(5); }
      break;
    }
    case 'sync_now': op = opFor(session, eventId, 'device.sync', {at: CLOCK}); break;
    default:
      if (sourceId in SLOTS) { ui.draft = slotEvent(ui, sourceId); go(4); break; }
      if (sourceId in TOGGLES) { op = opFor(session, eventId, 'calendar.set_visible', {id: TOGGLES[sourceId], visible: !state.calendars[TOGGLES[sourceId]].visible}); break; }
      throw failure('unknown_control');
  }
  if (op) {
    try { core.apply(state, op); } catch (error) { if (error instanceof core.Rejected) throw failure('rejected:' + error.reason); throw error; }
    pending.push(op);
  }
  return commit(session, ui, pending, eventId);
}

/** Fold server records (in log order) into the replica: confirm or roll back
 * this device's operations, adopt the other device's, never move the screen. */
export function applyRecords(session, records) {
  let remote = clone(session.remote), pending = clone(session.pending), notices = clone(session.notices);
  const ui = clone(session.ui); let changed = false;
  for (const record of [...records].sort((a, b) => a.seq - b.seq)) {
    if (record.seq <= remote.seq) continue;
    changed = true; remote.seq = record.seq;
    const op = record.op, mine = op.actor === session.device;
    if (record.accepted) {
      try { ({state: remote.state} = core.apply(remote.state, op)); } catch (error) { if (!(error instanceof core.Rejected)) throw error; }
      if (!mine && !record.replayed) {
        const eventId = op.payload?.event?.id || op.payload?.id || (op.action === 'invite.respond' ? 'invite-' + op.payload?.id : null);
        if (['event.create', 'event.update', 'event.delete', 'event.restore', 'invite.respond'].includes(op.action)) ui.unseen.push({key: op.id, eventId, from: op.actor, action: op.action, seq: record.seq});
      }
    } else if (mine && pending.some(p => p.id === op.id)) {
      notices.push({opId: op.id, action: op.action, reason: record.reason, seq: record.seq});
    }
    pending = pending.filter(p => p.id !== op.id);
  }
  if (!changed) return session;
  return freeze({...session, revision: session.revision + 1, remote, pending, notices, ui});
}

/** Adopt the server's snapshot (on connect). Pending operations are kept; ones the server already holds are dropped by the next pull. */
export function adoptSnapshot(session, snapshot) {
  if (!snapshot || typeof snapshot.seq !== 'number' || !snapshot.state) throw failure('invalid_snapshot');
  return freeze({...session, revision: session.revision + 1, connected: true, remote: {seq: snapshot.seq, state: clone(snapshot.state)}});
}

/** Show the next remote change on the synced card; offline, simulate Sam's desktop once. */
export function nextUpdate(session, eventId) {
  if (checkEvent(session, eventId)) return session;
  const view = getView(session);
  if (!view.nextUpdate) throw failure('no_update');
  const ui = clone(session.ui);
  let next = session;
  if (ui.unseen.length) { const change = ui.unseen.shift(); ui.synced = change; ui.selected = change.eventId || ui.selected; }
  else {
    next = applyRecords(session, [{seq: session.remote.seq + 1, op: SAM_MOVES_DINNER, accepted: true, replayed: false, simulated: true}]);
    const ui2 = clone(next.ui); const change = ui2.unseen.shift(); ui2.synced = change; ui2.selected = 'dinner'; ui2.demoDelivered = true;
    Object.assign(ui, ui2);
  }
  setFrame(ui, 5);
  return commit(next, ui, clone(next.pending), eventId);
}

export function goBack(session) {
  if (!session.history.length) throw failure('no_history');
  const last = session.history.at(-1);
  return freeze({...session, revision: session.revision + 1, ui: clone(last.ui), pending: clone(last.pending), history: session.history.slice(0, -1)});
}
export function restart(session) { const clean = createSession({locale: session.locale, device: session.device}); return freeze({...clean, epoch: session.epoch + 1, connected: session.connected, remote: clone(session.remote)}); }
export function errorMessage(error, locale = 'cn') {
  const messages = {invalid_event: ['操作标识无效，请重试', 'Invalid action identifier. Please retry'], stale_render: ['卡片已更新，请使用当前操作', 'This card changed. Use the current action'],
    unknown_control: ['当前界面没有这个操作', 'This action is not on the current screen'], disabled_control: ['这个操作当前不可用', 'This action is currently unavailable'],
    no_update: ['目前没有新的同步更改', 'There is no new synced change'], no_history: ['已经是第一步', 'You are at the first step'], invalid_snapshot: ['服务端快照无效', 'The server snapshot is invalid']};
  const code = error?.code || '';
  if (code.startsWith('rejected:')) { const reason = code.slice(9); return reason.startsWith('conflict:') ? t(locale, '与现有日程重叠，服务端已拒绝', 'Overlaps an existing event; the service refused it') : t(locale, `服务端拒绝：${reason}`, `Refused by the service: ${reason}`); }
  const pair = messages[code] || ['操作未完成，请重试', 'The action could not finish. Please retry'];
  return pair[locale === 'en' ? 1 : 0];
}
