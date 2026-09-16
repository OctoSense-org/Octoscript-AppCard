"""The calendar service's domain model: a pure reducer over an ordered log of
operations. The sync server, the native runtime and the browser wizard all
apply the same operations in the same order and reach the same state; the
browser twin is wizard/core.mjs and service/fixtures/ops.json pins both.

An operation is {id, actor, action, payload}. `id` is the client's unique
operation id (replaying the identical operation is a no-op; reusing the id
for different content is refused), `actor` is the device that made the
change, `action` one of ACTIONS. Every rule is explicit: overlapping events
are rejected (one owner, one timeline), undo names the operation it reverses, and an accepted
invitation adds the event — nothing else changes another calendar.
"""
import copy
import hashlib
import json

SCHEMA_VERSION = 1
ACTIONS = ('event.create', 'event.update', 'event.delete', 'event.restore',
           'invite.respond', 'calendar.set_visible', 'device.sync')
INVITE_STATUSES = ('accepted', 'declined', 'maybe')

FIXTURE = {
    'today': '2026-09-24',
    'devices': {'alex-phone': {'name': {'cn': 'Alex · 手机', 'en': 'Alex · Phone'}, 'last_sync': '2026-09-24T09:41:00+08:00'},
                'sam-desktop': {'name': {'cn': 'Sam · 桌面', 'en': 'Sam · Desktop'}, 'last_sync': '2026-09-24T09:40:00+08:00'}},
    'calendars': {'family': {'name': {'cn': '家庭', 'en': 'Family'}, 'color': 'ff3b30', 'visible': True, 'shared_with': ['sam']},
                  'work': {'name': {'cn': '工作', 'en': 'Work'}, 'color': '0a84ff', 'visible': True, 'shared_with': []},
                  'personal': {'name': {'cn': '个人', 'en': 'Personal'}, 'color': '34c759', 'visible': True, 'shared_with': []},
                  'birthdays': {'name': {'cn': '生日', 'en': 'Birthdays'}, 'color': 'af52de', 'visible': False, 'shared_with': []}},
    'events': {
        'dentist': {'id': 'dentist', 'title': {'cn': '牙医', 'en': 'Dentist'}, 'calendar': 'personal', 'start': '2026-09-24T10:30:00+08:00', 'end': '2026-09-24T11:15:00+08:00',
                    'location': {'cn': '日出牙科', 'en': 'Sunrise Dental'}, 'created_by': 'alex-phone', 'updated_by': 'alex-phone', 'deleted': False},
        'team-sync': {'id': 'team-sync', 'title': {'cn': '团队同步', 'en': 'Team sync'}, 'calendar': 'work', 'start': '2026-09-24T14:00:00+08:00', 'end': '2026-09-24T14:45:00+08:00',
                      'location': {'cn': '视频会议', 'en': 'Video call'}, 'created_by': 'alex-phone', 'updated_by': 'alex-phone', 'deleted': False},
        'dinner': {'id': 'dinner', 'title': {'cn': '与 Sam 晚餐', 'en': 'Dinner with Sam'}, 'calendar': 'family', 'start': '2026-09-24T19:00:00+08:00', 'end': '2026-09-24T20:30:00+08:00',
                   'location': {'cn': '莲花厨房', 'en': 'Lotus Kitchen'}, 'created_by': 'sam-desktop', 'updated_by': 'sam-desktop', 'deleted': False},
    },
    'invitations': {
        'hike': {'id': 'hike', 'from': 'sam', 'title': {'cn': '周末远足', 'en': 'Weekend hike'}, 'calendar': 'family', 'start': '2026-09-26T09:00:00+08:00', 'end': '2026-09-26T12:00:00+08:00',
                 'location': {'cn': '西山步道', 'en': 'West Hill trail'}, 'status': 'pending', 'event_id': None},
    },
}

PIANO = {'id': 'piano', 'title': {'cn': '钢琴课', 'en': 'Piano lesson'}, 'calendar': 'family', 'start': '2026-09-25T16:00:00+08:00', 'end': '2026-09-25T17:00:00+08:00',
         'location': {'cn': '', 'en': ''}}


def initial_state():
    return {'schema_version': SCHEMA_VERSION, 'seq': 0, 'applied': {}, 'devices': copy.deepcopy(FIXTURE['devices']),
            'calendars': copy.deepcopy(FIXTURE['calendars']), 'events': copy.deepcopy(FIXTURE['events']),
            'invitations': copy.deepcopy(FIXTURE['invitations']), 'history': []}


def canonical(value):
    return json.dumps(value, sort_keys=True, ensure_ascii=False, separators=(',', ':'))


def digest(value):
    return hashlib.sha256(canonical(value).encode('utf-8')).hexdigest()


class Rejected(Exception):
    def __init__(self, reason):
        super().__init__(reason)
        self.reason = reason


def _check_shape(op):
    if not isinstance(op, dict):
        raise Rejected('invalid_operation')
    for key in ('id', 'actor', 'action'):
        if not isinstance(op.get(key), str) or not op[key].strip():
            raise Rejected('invalid_operation')
    if op['action'] not in ACTIONS:
        raise Rejected('unknown_action')
    if 'payload' in op and not isinstance(op['payload'], dict):
        raise Rejected('invalid_operation')


def _event_shape(event):
    if not isinstance(event, dict):
        raise Rejected('invalid_event')
    for key in ('id', 'calendar', 'start', 'end'):
        if not isinstance(event.get(key), str) or not event[key]:
            raise Rejected('invalid_event')
    title = event.get('title')
    if not isinstance(title, dict) or not (title.get('cn') or title.get('en')):
        raise Rejected('invalid_event')
    if event['end'] <= event['start']:
        raise Rejected('invalid_event')


def overlaps(a, b):
    """ISO-8601 strings with the same offset compare lexically."""
    return a['start'] < b['end'] and b['start'] < a['end']


def conflict(state, candidate, ignore=None):
    """One owner, one timeline: an overlap on any calendar is a conflict."""
    for other in state['events'].values():
        if other['deleted'] or other['id'] == ignore:
            continue
        if overlaps(other, candidate):
            return other['id']
    return None


def apply(state, op):
    """Return (next_state, result). result = {'accepted': bool, 'reason': str|None, 'replayed': bool}.
    Rejections never change the state; the caller records them in its own log."""
    _check_shape(op)
    if op['id'] in state['applied']:
        if state['applied'][op['id']] != canonical(op):
            raise Rejected('operation_id_reused')
        return state, {'accepted': True, 'reason': None, 'replayed': True}
    if op['actor'] not in state['devices']:
        raise Rejected('unknown_device')
    payload = op.get('payload') or {}
    next_state = copy.deepcopy(state)
    action = op['action']
    if action == 'event.create':
        event = payload.get('event')
        _event_shape(event)
        if event['calendar'] not in next_state['calendars']:
            raise Rejected('unknown_calendar')
        existing = next_state['events'].get(event['id'])
        if existing and not existing['deleted']:
            raise Rejected('duplicate_event')
        other = conflict(next_state, event)
        if other:
            raise Rejected('conflict:' + other)
        next_state['events'][event['id']] = {**copy.deepcopy(event), 'location': event.get('location') or {'cn': '', 'en': ''},
                                             'created_by': op['actor'], 'updated_by': op['actor'], 'deleted': False}
    elif action == 'event.update':
        event = next_state['events'].get(payload.get('id'))
        if not event or event['deleted']:
            raise Rejected('missing_event')
        patch = payload.get('patch')
        if not isinstance(patch, dict) or not patch or set(patch) - {'title', 'calendar', 'start', 'end', 'location'}:
            raise Rejected('invalid_patch')
        candidate = {**event, **patch}
        _event_shape(candidate)
        if candidate['calendar'] not in next_state['calendars']:
            raise Rejected('unknown_calendar')
        other = conflict(next_state, candidate, ignore=event['id'])
        if other:
            raise Rejected('conflict:' + other)
        event.update(copy.deepcopy(patch))
        event['updated_by'] = op['actor']
    elif action == 'event.delete':
        event = next_state['events'].get(payload.get('id'))
        if not event or event['deleted']:
            raise Rejected('missing_event')
        event['deleted'] = True
        event['updated_by'] = op['actor']
    elif action == 'event.restore':
        event = next_state['events'].get(payload.get('id'))
        if not event or not event['deleted']:
            raise Rejected('missing_event')
        other = conflict(next_state, event, ignore=event['id'])
        if other:
            raise Rejected('conflict:' + other)
        event['deleted'] = False
        event['updated_by'] = op['actor']
    elif action == 'invite.respond':
        invite = next_state['invitations'].get(payload.get('id'))
        status = payload.get('status')
        if not invite:
            raise Rejected('missing_invitation')
        if status not in INVITE_STATUSES:
            raise Rejected('invalid_status')
        if invite['status'] == 'accepted' and status != 'accepted' and invite['event_id']:
            # Withdrawing an acceptance removes only our copy; Sam's event is Sam's.
            next_state['events'][invite['event_id']]['deleted'] = True
            invite['event_id'] = None
        if status == 'accepted' and invite['status'] != 'accepted':
            event = {'id': 'invite-' + invite['id'], 'title': invite['title'], 'calendar': invite['calendar'], 'start': invite['start'], 'end': invite['end'], 'location': invite['location']}
            other = conflict(next_state, event)
            if other:
                raise Rejected('conflict:' + other)
            next_state['events'][event['id']] = {**copy.deepcopy(event), 'created_by': op['actor'], 'updated_by': op['actor'], 'deleted': False, 'invitation': invite['id']}
            invite['event_id'] = event['id']
        invite['status'] = status
        invite['answered_by'] = op['actor']
    elif action == 'calendar.set_visible':
        calendar = next_state['calendars'].get(payload.get('id'))
        if not calendar or not isinstance(payload.get('visible'), bool):
            raise Rejected('invalid_calendar')
        calendar['visible'] = payload['visible']
    elif action == 'device.sync':
        device = next_state['devices'].get(op['actor'])
        at = payload.get('at')
        if not isinstance(at, str) or not at:
            raise Rejected('invalid_sync')
        device['last_sync'] = at
        device['sync_count'] = device.get('sync_count', 0) + 1
    next_state['seq'] = state['seq'] + 1
    next_state['applied'][op['id']] = canonical(op)
    next_state['history'].append({'seq': next_state['seq'], 'id': op['id'], 'actor': op['actor'], 'action': action})
    return next_state, {'accepted': True, 'reason': None, 'replayed': False}


def replay(ops, state=None):
    """Apply a log of {op, accepted} records; rejected records are skipped, as
    every client sees the same server verdicts."""
    state = state or initial_state()
    for record in ops:
        if record.get('accepted', True):
            state, _ = apply(state, record['op'] if 'op' in record else record)
    return state


def state_digest(state):
    """Identity of the synced data, independent of local bookkeeping."""
    return digest({k: state[k] for k in ('seq', 'calendars', 'events', 'invitations', 'devices')})


def visible_events(state, day=None):
    events = [e for e in state['events'].values() if not e['deleted'] and state['calendars'][e['calendar']]['visible']]
    if day:
        events = [e for e in events if e['start'][:10] == day]
    return sorted(events, key=lambda e: (e['start'], e['id']))
