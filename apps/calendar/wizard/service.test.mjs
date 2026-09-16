import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import {createSession, getView, activateControl, applyRecords, adoptSnapshot, nextUpdate, goBack, restart, errorMessage, stateOf, scenario} from './service.mjs';
import * as core from './core.mjs';
import {frames} from './copy.mjs';
import {buildNativePayload} from '../../shared/render.mjs';

const routes = JSON.parse(fs.readFileSync(new URL('../route_test.json', import.meta.url))).routes;
const bundle = JSON.parse(fs.readFileSync(new URL('./card-bundle/cards.bundle.json', import.meta.url)));
const path = (obj, name) => name.split('.').reduce((v, k) => v?.[k], obj);
let event = 0;
const click = (s, id) => activateControl(s, id, `test-${++event}`, getView(s).renderId);

function verify(s) {
  const view = getView(s), state = view.state;
  for (const e of Object.values(state.events)) if (!e.deleted) assert.equal(core.conflict(state, e, e.id), null, 'no two live events overlap');
  assert.equal(Object.values(state.events).filter(e => e.invitation === 'hike' && !e.deleted).length, state.invitations.hike.status === 'accepted' ? 1 : 0);
  const textIds = Object.keys(frames[view.frameId].text).sort();
  assert.deepEqual(Object.keys(view.nativeText).sort(), textIds);
  for (const control of view.nativeControls) {
    assert.ok(Object.hasOwn(frames[view.frameId].controls, control.sourceId));
    assert.ok(control.label);
    for (const id of control.textIds) assert.ok(Object.hasOwn(view.nativeText, id));
  }
  const rendered = buildNativePayload(bundle, view, {assetBase: 'http://127.0.0.1:8170/ux-images/', id: view.renderId});
  assert.equal(rendered.frameId, view.frameId);
  return view;
}

for (const locale of ['cn', 'en']) for (const route of routes) test(`${locale}: ${route.name}`, () => {
  let s = createSession({locale}); verify(s);
  for (const step of route.steps) {
    if (step.disabled) { const control = getView(s).nativeControls.find(v => v.sourceId === step.control); assert.equal(control?.enabled, false, step.control); assert.throws(() => click(s, step.control), {code: 'disabled_control'}); }
    else if (step.control) s = click(s, step.control);
    else if (step.update) s = nextUpdate(s, `test-${++event}`);
    else if (step.back) s = goBack(s);
    else if (step.restart) s = restart(s);
    const v = verify(s);
    assert.equal(v.frameId, step.frame, `${route.name}: ${JSON.stringify(step)}`);
    for (const [key, value] of Object.entries(step.assert || {})) assert.deepEqual(path(v, key), value, `${route.name}: ${key}`);
  }
});

test('every scene renders in both locales from the initial session, and the bundle covers all ten', () => {
  assert.equal(Object.keys(bundle.scenes).length, 10);
  assert.equal(scenario.id, 'calendar');
  for (const locale of ['cn', 'en']) for (const frame of Object.keys(frames)) {
    const s = createSession({locale});
    const forced = {...structuredClone(s), ui: {...structuredClone(s.ui), frame: Number(frame), surface: [5, 6, 9, 10].includes(Number(frame)) ? 'desktop' : 'app', synced: {key: 'k', eventId: 'dinner', from: 'sam-desktop', action: 'event.update'}, removed: 'dinner', draft: structuredClone(core.PIANO)}};
    Object.freeze(forced);
    const view = getView(forced);
    assert.equal(view.frameId, Number(frame));
    const rendered = buildNativePayload(bundle, view, {assetBase: 'http://127.0.0.1:8170/ux-images/', id: view.renderId});
    assert.ok(rendered.card.includes('view root'));
  }
});

test('immutability, idempotent event ids, stale renders, unknown and disabled controls', () => {
  const s = createSession();
  const view = getView(s);
  assert.throws(() => { s.ui.frame = 2; }, TypeError);
  const once = activateControl(s, 'open_today', 'e1', view.renderId);
  assert.equal(activateControl(once, 'open_today', 'e1', 'anything'), once, 'a replayed event id returns the same session');
  assert.throws(() => activateControl(s, 'open_today', 'e2', 'calendar:9:9:9'), {code: 'stale_render'});
  assert.throws(() => activateControl(s, 'pay_now', 'e3', view.renderId), {code: 'unknown_control'});
  assert.throws(() => activateControl(s, 'back_year', 'e4', view.renderId), {code: 'disabled_control'});
  assert.equal(errorMessage({code: 'disabled_control'}, 'en'), 'This action is currently unavailable');
  assert.equal(errorMessage({code: 'rejected:conflict:team-sync'}, 'en'), 'Overlaps an existing event; the service refused it');
});

test('a server rejection rolls the optimistic change back and leaves a notice; the other device’s change stays unseen until asked for', () => {
  let s = createSession();
  s = adoptSnapshot(s, {seq: 0, state: core.initialState(), digest: 'x'});
  assert.equal(s.connected, true);
  s = click(s, 'add_event'); s = click(s, 'save_event');
  assert.equal(stateOf(s).events.piano.deleted, false);
  const op = s.pending[0];
  s = applyRecords(s, [
    {seq: 1, op: {id: 'sam-desktop:first', actor: 'sam-desktop', action: 'event.create', payload: {event: {...core.PIANO, id: 'sam-piano', title: {cn: 'Sam 的钢琴课', en: 'Sam’s piano'}}}}, accepted: true, replayed: false},
    {seq: 2, op, accepted: false, reason: 'conflict:sam-piano', replayed: false},
  ]);
  assert.equal(s.pending.length, 0);
  assert.equal(stateOf(s).events.piano, undefined, 'the rejected create is gone from the replica');
  assert.deepEqual(s.notices.map(n => n.reason), ['conflict:sam-piano']);
  assert.equal(s.ui.frame, 5, 'the screen did not move by itself');
  assert.equal(s.ui.unseen.length, 1);
  const view = getView(s);
  assert.ok(view.nextUpdate, 'the remote change is offered, not forced');
  assert.equal(view.seq, 2);
  s = nextUpdate(s, 'show');
  assert.equal(s.ui.frame, 5);
  assert.equal(s.ui.synced.eventId, 'sam-piano');
  assert.equal(getView(s).nativeText.card_source, '家庭日历 · 来自 Sam · 桌面');
  assert.throws(() => nextUpdate(s, 'again'), {code: 'no_update'});
  assert.equal(applyRecords(s, [{seq: 1, op, accepted: true}]), s, 'records at or below the cursor are ignored');
});

test('connected sessions do not simulate the desktop; offline ones do it once', () => {
  const offline = createSession();
  assert.ok(getView(offline).nextUpdate);
  const shown = nextUpdate(offline, 'demo');
  assert.equal(shown.remote.seq, 1);
  assert.equal(getView(click(shown, 'open_calendar')).nextUpdate, null, 'the demo update is offered once');
  const online = adoptSnapshot(createSession(), {seq: 7, state: core.initialState(), digest: 'x'});
  assert.equal(getView(online).nextUpdate, null);
  assert.equal(online.remote.seq, 7);
});

test('history restores the screen and the unsent operations; restart keeps the server replica', () => {
  let s = createSession();
  s = click(s, 'add_event'); s = click(s, 'save_event');
  assert.equal(s.pending.length, 1);
  const back = goBack(s);
  assert.equal(back.ui.frame, 4);
  assert.equal(back.pending.length, 0, 'the unsent create is withdrawn with its screen');
  assert.throws(() => goBack(createSession()), {code: 'no_history'});
  const synced = applyRecords(s, [{seq: 1, op: s.pending[0], accepted: true}]);
  const fresh = restart(synced);
  assert.equal(fresh.epoch, 1);
  assert.equal(fresh.remote.seq, 1);
  assert.equal(stateOf(fresh).events.piano.deleted, false);
  assert.equal(fresh.ui.frame, 1);
});
