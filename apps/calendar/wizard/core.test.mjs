import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import * as core from './core.mjs';

const fixture = JSON.parse(fs.readFileSync(new URL('../service/fixtures/ops.json', import.meta.url)));
const op = (id, action, payload = {}, actor = 'alex-phone') => ({id, actor, action, payload});
function rejects(state, operation, reason) {
  assert.throws(() => core.apply(state, operation), error => error instanceof core.Rejected && error.reason === reason);
}

test('the shared fixture replays with the pinned verdicts and the same digest as Python', async () => {
  let state = core.initialState();
  for (const record of fixture.records) {
    const before = structuredClone(state);
    try {
      const applied = core.apply(state, record.op);
      assert.ok(record.accepted, record.op.id);
      assert.equal(applied.result.replayed, Boolean(record.replayed), record.op.id);
      state = applied.state;
    } catch (error) {
      assert.ok(error instanceof core.Rejected, String(error));
      assert.equal(record.accepted, false, record.op.id);
      assert.equal(error.reason, record.reason, record.op.id);
      assert.deepEqual(state, before, 'a rejection must not change the state');
    }
  }
  const expect = fixture.expect;
  assert.equal(state.seq, expect.seq);
  assert.deepEqual(core.visibleEvents(state, core.FIXTURE.today).map(e => e.id), expect.events_visible_today);
  assert.deepEqual(core.visibleEvents(state, '2026-09-26').map(e => e.id), expect['events_visible_2026-09-26']);
  assert.equal(state.events.dinner.start, expect.dinner_start);
  assert.equal(state.invitations.hike.status, expect.hike_status);
  assert.equal(state.calendars.birthdays.visible, expect.birthdays_visible);
  assert.equal(state.devices['sam-desktop'].sync_count, expect.sam_sync_count);
  assert.equal(await core.stateDigest(state), expect.state_digest);
});

test('canonical JSON matches Python json.dumps(sort_keys, compact, ensure_ascii=False)', () => {
  assert.equal(core.canonical({b: [1, 'é', {z: null, a: true}], a: '中文"\\'}), '{"a":"中文\\"\\\\","b":[1,"é",{"a":true,"z":null}]}');
});

test('an overlap on any calendar is rejected; touching slots are fine', async () => {
  const state = core.initialState();
  const clash = {...core.PIANO, id: 'clash', start: '2026-09-24T14:00:00+08:00', end: '2026-09-24T15:00:00+08:00', calendar: 'work'};
  rejects(state, op('a', 'event.create', {event: clash}), 'conflict:team-sync');
  rejects(state, op('b', 'event.create', {event: {...clash, calendar: 'family'}}), 'conflict:team-sync');
  const touching = core.apply(state, op('c', 'event.create', {event: {...clash, start: '2026-09-24T14:45:00+08:00', end: '2026-09-24T15:30:00+08:00'}}));
  assert.ok(touching.state.events.clash);
});

test('replaying an id with different content is refused, identical content is a no-op', async () => {
  const {state} = core.apply(core.initialState(), op('x', 'event.delete', {id: 'dinner'}));
  rejects(state, op('x', 'event.delete', {id: 'dentist'}), 'operation_id_reused');
  const again = core.apply(state, op('x', 'event.delete', {id: 'dinner'}));
  assert.equal(again.result.replayed, true);
  assert.equal(again.state.seq, state.seq);
});

test('invitations: accept adds one event, a second accept never duplicates, decline removes only our copy', async () => {
  let {state} = core.apply(core.initialState(), op('a', 'invite.respond', {id: 'hike', status: 'accepted'}));
  assert.equal(state.invitations.hike.event_id, 'invite-hike');
  ({state} = core.apply(state, op('b', 'invite.respond', {id: 'hike', status: 'accepted'}, 'sam-desktop')));
  assert.equal(Object.values(state.events).filter(e => e.invitation === 'hike').length, 1);
  ({state} = core.apply(state, op('c', 'invite.respond', {id: 'hike', status: 'declined'})));
  assert.equal(state.events['invite-hike'].deleted, true);
  assert.equal(state.invitations.hike.event_id, null);
});
