/** The calendar domain reducer — the browser twin of service/calendar_core.py.
 * Same operations, same rejections, same state; service/fixtures/ops.json is
 * replayed by both test suites and the resulting state digests must match. */
export const SCHEMA_VERSION = 1;
export const ACTIONS = ['event.create', 'event.update', 'event.delete', 'event.restore', 'invite.respond', 'calendar.set_visible', 'device.sync'];
const INVITE_STATUSES = ['accepted', 'declined', 'maybe'];

export const FIXTURE = {
  today: '2026-09-24',
  devices: {'alex-phone': {name: {cn: 'Alex · 手机', en: 'Alex · Phone'}, last_sync: '2026-09-24T09:41:00+08:00'},
            'sam-desktop': {name: {cn: 'Sam · 桌面', en: 'Sam · Desktop'}, last_sync: '2026-09-24T09:40:00+08:00'}},
  calendars: {family: {name: {cn: '家庭', en: 'Family'}, color: 'ff3b30', visible: true, shared_with: ['sam']},
              work: {name: {cn: '工作', en: 'Work'}, color: '0a84ff', visible: true, shared_with: []},
              personal: {name: {cn: '个人', en: 'Personal'}, color: '34c759', visible: true, shared_with: []},
              birthdays: {name: {cn: '生日', en: 'Birthdays'}, color: 'af52de', visible: false, shared_with: []}},
  events: {
    dentist: {id: 'dentist', title: {cn: '牙医', en: 'Dentist'}, calendar: 'personal', start: '2026-09-24T10:30:00+08:00', end: '2026-09-24T11:15:00+08:00', location: {cn: '日出牙科', en: 'Sunrise Dental'}, created_by: 'alex-phone', updated_by: 'alex-phone', deleted: false},
    'team-sync': {id: 'team-sync', title: {cn: '团队同步', en: 'Team sync'}, calendar: 'work', start: '2026-09-24T14:00:00+08:00', end: '2026-09-24T14:45:00+08:00', location: {cn: '视频会议', en: 'Video call'}, created_by: 'alex-phone', updated_by: 'alex-phone', deleted: false},
    dinner: {id: 'dinner', title: {cn: '与 Sam 晚餐', en: 'Dinner with Sam'}, calendar: 'family', start: '2026-09-24T19:00:00+08:00', end: '2026-09-24T20:30:00+08:00', location: {cn: '莲花厨房', en: 'Lotus Kitchen'}, created_by: 'sam-desktop', updated_by: 'sam-desktop', deleted: false},
  },
  invitations: {
    hike: {id: 'hike', from: 'sam', title: {cn: '周末远足', en: 'Weekend hike'}, calendar: 'family', start: '2026-09-26T09:00:00+08:00', end: '2026-09-26T12:00:00+08:00', location: {cn: '西山步道', en: 'West Hill trail'}, status: 'pending', event_id: null},
  },
};
export const PIANO = {id: 'piano', title: {cn: '钢琴课', en: 'Piano lesson'}, calendar: 'family', start: '2026-09-25T16:00:00+08:00', end: '2026-09-25T17:00:00+08:00', location: {cn: '', en: ''}};

const clone = value => structuredClone(value);
export function initialState() {
  return {schema_version: SCHEMA_VERSION, seq: 0, applied: {}, devices: clone(FIXTURE.devices), calendars: clone(FIXTURE.calendars), events: clone(FIXTURE.events), invitations: clone(FIXTURE.invitations), history: []};
}

/** JSON with sorted keys, no spaces — byte-identical to Python's canonical(). */
export function canonical(value) {
  if (Array.isArray(value)) return '[' + value.map(canonical).join(',') + ']';
  if (value && typeof value === 'object') return '{' + Object.keys(value).sort().map(k => JSON.stringify(k) + ':' + canonical(value[k])).join(',') + '}';
  return JSON.stringify(value);
}
async function sha256(text) {
  const bytes = new TextEncoder().encode(text);
  const subtle = globalThis.crypto?.subtle;
  if (!subtle) throw new Error('WebCrypto is required');
  const hash = await subtle.digest('SHA-256', bytes);
  return [...new Uint8Array(hash)].map(b => b.toString(16).padStart(2, '0')).join('');
}
/** Only the state digest hashes (asynchronously, WebCrypto); the reducer itself is synchronous. */
export const digest = value => sha256(canonical(value));

export class Rejected extends Error { constructor(reason) { super(reason); this.reason = reason; } }
const reject = reason => { throw new Rejected(reason); };

function checkShape(op) {
  if (!op || typeof op !== 'object') reject('invalid_operation');
  for (const key of ['id', 'actor', 'action']) if (typeof op[key] !== 'string' || !op[key].trim()) reject('invalid_operation');
  if (!ACTIONS.includes(op.action)) reject('unknown_action');
  if ('payload' in op && (op.payload === null || typeof op.payload !== 'object' || Array.isArray(op.payload))) reject('invalid_operation');
}
function eventShape(event) {
  if (!event || typeof event !== 'object') reject('invalid_event');
  for (const key of ['id', 'calendar', 'start', 'end']) if (typeof event[key] !== 'string' || !event[key]) reject('invalid_event');
  if (!event.title || typeof event.title !== 'object' || !(event.title.cn || event.title.en)) reject('invalid_event');
  if (event.end <= event.start) reject('invalid_event');
}
export const overlaps = (a, b) => a.start < b.end && b.start < a.end;
export function conflict(state, candidate, ignore = null) {
  for (const other of Object.values(state.events)) {
    if (other.deleted || other.id === ignore) continue; // one owner, one timeline
    if (overlaps(other, candidate)) return other.id;
  }
  return null;
}

/** Apply one operation. Returns {state, result}; a rejection throws Rejected and leaves the state untouched. */
export function apply(state, op) {
  checkShape(op);
  if (Object.hasOwn(state.applied, op.id)) {
    if (state.applied[op.id] !== canonical(op)) reject('operation_id_reused');
    return {state, result: {accepted: true, reason: null, replayed: true}};
  }
  if (!Object.hasOwn(state.devices, op.actor)) reject('unknown_device');
  const payload = op.payload || {};
  const next = clone(state);
  switch (op.action) {
    case 'event.create': {
      const event = payload.event; eventShape(event);
      if (!Object.hasOwn(next.calendars, event.calendar)) reject('unknown_calendar');
      const existing = next.events[event.id];
      if (existing && !existing.deleted) reject('duplicate_event');
      const other = conflict(next, event); if (other) reject('conflict:' + other);
      next.events[event.id] = {...clone(event), location: event.location || {cn: '', en: ''}, created_by: op.actor, updated_by: op.actor, deleted: false};
      break;
    }
    case 'event.update': {
      const event = next.events[payload.id];
      if (!event || event.deleted) reject('missing_event');
      const patch = payload.patch;
      if (!patch || typeof patch !== 'object' || !Object.keys(patch).length || Object.keys(patch).some(k => !['title', 'calendar', 'start', 'end', 'location'].includes(k))) reject('invalid_patch');
      const candidate = {...event, ...patch}; eventShape(candidate);
      if (!Object.hasOwn(next.calendars, candidate.calendar)) reject('unknown_calendar');
      const other = conflict(next, candidate, event.id); if (other) reject('conflict:' + other);
      Object.assign(event, clone(patch)); event.updated_by = op.actor;
      break;
    }
    case 'event.delete': {
      const event = next.events[payload.id];
      if (!event || event.deleted) reject('missing_event');
      event.deleted = true; event.updated_by = op.actor;
      break;
    }
    case 'event.restore': {
      const event = next.events[payload.id];
      if (!event || !event.deleted) reject('missing_event');
      const other = conflict(next, event, event.id); if (other) reject('conflict:' + other);
      event.deleted = false; event.updated_by = op.actor;
      break;
    }
    case 'invite.respond': {
      const invite = next.invitations[payload.id]; const status = payload.status;
      if (!invite) reject('missing_invitation');
      if (!INVITE_STATUSES.includes(status)) reject('invalid_status');
      if (invite.status === 'accepted' && status !== 'accepted' && invite.event_id) { next.events[invite.event_id].deleted = true; invite.event_id = null; }
      if (status === 'accepted' && invite.status !== 'accepted') {
        const event = {id: 'invite-' + invite.id, title: invite.title, calendar: invite.calendar, start: invite.start, end: invite.end, location: invite.location};
        const other = conflict(next, event); if (other) reject('conflict:' + other);
        next.events[event.id] = {...clone(event), created_by: op.actor, updated_by: op.actor, deleted: false, invitation: invite.id};
        invite.event_id = event.id;
      }
      invite.status = status; invite.answered_by = op.actor;
      break;
    }
    case 'calendar.set_visible': {
      const calendar = next.calendars[payload.id];
      if (!calendar || typeof payload.visible !== 'boolean') reject('invalid_calendar');
      calendar.visible = payload.visible;
      break;
    }
    case 'device.sync': {
      const device = next.devices[op.actor];
      if (typeof payload.at !== 'string' || !payload.at) reject('invalid_sync');
      device.last_sync = payload.at; device.sync_count = (device.sync_count || 0) + 1;
      break;
    }
  }
  next.seq = state.seq + 1;
  next.applied[op.id] = canonical(op);
  next.history.push({seq: next.seq, id: op.id, actor: op.actor, action: op.action});
  return {state: next, result: {accepted: true, reason: null, replayed: false}};
}

export function replay(records, state = initialState()) {
  for (const record of records) {
    if (record.accepted === false) continue;
    ({state} = apply(state, 'op' in record ? record.op : record));
  }
  return state;
}
export const stateDigest = state => digest({seq: state.seq, calendars: state.calendars, events: state.events, invitations: state.invitations, devices: state.devices});
export function visibleEvents(state, day = null) {
  let events = Object.values(state.events).filter(e => !e.deleted && state.calendars[e.calendar].visible);
  if (day) events = events.filter(e => e.start.slice(0, 10) === day);
  return events.sort((a, b) => (a.start + a.id).localeCompare(b.start + b.id));
}
