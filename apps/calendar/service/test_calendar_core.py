"""Reducer invariants and the shared two-device fixture (also replayed by wizard/core.test.mjs)."""
import copy
import json
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import calendar_core as core

FIXTURE = json.loads((Path(__file__).resolve().parent / 'fixtures/ops.json').read_text())


def op(id, action, payload=None, actor='alex-phone'):
    return {'id': id, 'actor': actor, 'action': action, 'payload': payload or {}}


class FixtureReplay(unittest.TestCase):
    def test_every_record_gets_the_pinned_verdict(self):
        state = core.initial_state()
        for record in FIXTURE['records']:
            before = copy.deepcopy(state)
            try:
                state, result = core.apply(state, record['op'])
                self.assertTrue(record['accepted'], record['op']['id'])
                self.assertEqual(result['replayed'], record.get('replayed', False), record['op']['id'])
            except core.Rejected as error:
                self.assertFalse(record['accepted'], record['op']['id'])
                self.assertEqual(error.reason, record['reason'], record['op']['id'])
                self.assertEqual(state, before, 'a rejection must not change the state')
        expect = FIXTURE['expect']
        self.assertEqual(state['seq'], expect['seq'])
        self.assertEqual([e['id'] for e in core.visible_events(state, core.FIXTURE['today'])], expect['events_visible_today'])
        self.assertEqual([e['id'] for e in core.visible_events(state, '2026-09-26')], expect['events_visible_2026-09-26'])
        self.assertEqual(state['events']['dinner']['start'], expect['dinner_start'])
        self.assertEqual(state['invitations']['hike']['status'], expect['hike_status'])
        self.assertEqual(state['calendars']['birthdays']['visible'], expect['birthdays_visible'])
        self.assertEqual(state['devices']['sam-desktop']['sync_count'], expect['sam_sync_count'])
        self.assertEqual(core.state_digest(state), expect['state_digest'], 'the browser twin pins the same digest')

    def test_replay_skips_rejected_records_and_is_idempotent(self):
        once = core.replay(FIXTURE['records'])
        twice = core.replay(FIXTURE['records'], once)
        self.assertEqual(core.state_digest(once), core.state_digest(twice))
        self.assertEqual(twice['seq'], once['seq'])


class Invariants(unittest.TestCase):
    def setUp(self):
        self.state = core.initial_state()

    def test_overlap_on_any_calendar_is_rejected(self):
        clash = dict(core.PIANO, id='clash', start='2026-09-24T14:00:00+08:00', end='2026-09-24T15:00:00+08:00', calendar='work')
        for calendar in ('work', 'family'):
            with self.assertRaises(core.Rejected) as caught:
                core.apply(self.state, op('a', 'event.create', {'event': dict(clash, calendar=calendar)}))
            self.assertEqual(caught.exception.reason, 'conflict:team-sync')

    def test_touching_slots_do_not_conflict(self):
        touching = dict(core.PIANO, id='after', start='2026-09-24T14:45:00+08:00', end='2026-09-24T15:30:00+08:00', calendar='work')
        state, _ = core.apply(self.state, op('a', 'event.create', {'event': touching}))
        self.assertIn('after', state['events'])

    def test_delete_then_restore_returns_the_same_event_and_conflicts_block_restore(self):
        state, _ = core.apply(self.state, op('d', 'event.delete', {'id': 'dinner'}))
        self.assertTrue(state['events']['dinner']['deleted'])
        blocker = dict(core.PIANO, id='blocker', start='2026-09-24T19:00:00+08:00', end='2026-09-24T20:00:00+08:00')
        state, _ = core.apply(state, op('c', 'event.create', {'event': blocker}))
        with self.assertRaises(core.Rejected) as caught:
            core.apply(state, op('r', 'event.restore', {'id': 'dinner'}))
        self.assertEqual(caught.exception.reason, 'conflict:blocker')
        state, _ = core.apply(state, op('d2', 'event.delete', {'id': 'blocker'}))
        state, _ = core.apply(state, op('r2', 'event.restore', {'id': 'dinner'}))
        self.assertEqual(state['events']['dinner']['title'], core.FIXTURE['events']['dinner']['title'])

    def test_accepting_an_invitation_adds_one_event_and_declining_removes_only_our_copy(self):
        state, _ = core.apply(self.state, op('a', 'invite.respond', {'id': 'hike', 'status': 'accepted'}))
        self.assertEqual(state['invitations']['hike']['event_id'], 'invite-hike')
        state, _ = core.apply(state, op('b', 'invite.respond', {'id': 'hike', 'status': 'accepted', }, actor='sam-desktop'))
        self.assertEqual(sum(1 for e in state['events'].values() if e.get('invitation') == 'hike'), 1, 'acknowledging again never duplicates')
        state, _ = core.apply(state, op('c', 'invite.respond', {'id': 'hike', 'status': 'declined'}))
        self.assertTrue(state['events']['invite-hike']['deleted'])
        self.assertIsNone(state['invitations']['hike']['event_id'])
        self.assertEqual(state['invitations']['hike']['title'], core.FIXTURE['invitations']['hike']['title'], "Sam's invitation itself is untouched")

    def test_replaying_an_id_with_different_content_is_refused(self):
        state, _ = core.apply(self.state, op('x', 'event.delete', {'id': 'dinner'}))
        with self.assertRaises(core.Rejected) as caught:
            core.apply(state, op('x', 'event.delete', {'id': 'dentist'}))
        self.assertEqual(caught.exception.reason, 'operation_id_reused')
        again, result = core.apply(state, op('x', 'event.delete', {'id': 'dinner'}))
        self.assertTrue(result['replayed'])
        self.assertEqual(again['seq'], state['seq'])

    def test_unknown_actor_and_malformed_operations(self):
        for bad, reason in [(op('a', 'event.delete', {'id': 'dinner'}, actor='nobody'), 'unknown_device'),
                            ({'id': 'a', 'actor': 'alex-phone', 'action': 'event.explode'}, 'unknown_action'),
                            ({'id': '', 'actor': 'alex-phone', 'action': 'event.delete'}, 'invalid_operation'),
                            (op('a', 'event.create', {'event': dict(core.PIANO, end=core.PIANO['start'])}), 'invalid_event'),
                            (op('a', 'event.create', {'event': dict(core.PIANO, calendar='moon')}), 'unknown_calendar')]:
            with self.assertRaises(core.Rejected) as caught:
                core.apply(self.state, bad)
            self.assertEqual(caught.exception.reason, reason)

    def test_visibility_hides_events_without_deleting_them(self):
        state, _ = core.apply(self.state, op('v', 'calendar.set_visible', {'id': 'work', 'visible': False}))
        self.assertEqual([e['id'] for e in core.visible_events(state, '2026-09-24')], ['dentist', 'dinner'])
        self.assertFalse(state['events']['team-sync']['deleted'])


if __name__ == '__main__':
    unittest.main()
