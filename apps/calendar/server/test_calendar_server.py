"""Two devices against one live server: convergence, verdicts, persistence, auth, streaming."""
import http.client
import json
import sys
import tempfile
import threading
import time
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path[:0] = [str(HERE), str(HERE.parent / 'service')]
import calendar_core as core  # noqa: E402
import calendar_server  # noqa: E402
from sync_client import SyncClient, SyncError  # noqa: E402

PIANO_OP = {'event': core.PIANO}


class LiveServer:
    def __init__(self, db, token='test-token'):
        self.server = calendar_server.serve(db, port=0, token=token, allow_origins=['http://127.0.0.1:4321'], announce=lambda line: None)
        self.thread = threading.Thread(target=self.server.serve_forever, daemon=True)
        self.thread.start()
        self.url = f'http://127.0.0.1:{self.server.server_address[1]}'
        self.token = token

    def client(self, device):
        client = SyncClient(self.url, self.token, device)
        client.bootstrap()
        return client

    def stop(self):
        self.server.shutdown()
        self.server.server_close()
        self.server.store.close()


class ServerTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.db = Path(self.tmp.name) / 'calendar.sqlite'
        self.live = LiveServer(self.db)

    def tearDown(self):
        self.live.stop()
        self.tmp.cleanup()

    def test_two_devices_converge_and_see_each_others_verdicts(self):
        phone, desktop = self.live.client('alex-phone'), self.live.client('sam-desktop')
        record = phone.submit('event.create', PIANO_OP)
        self.assertTrue(record['accepted'])
        self.assertEqual(record['seq'], 1)
        self.assertEqual([r['seq'] for r in desktop.pull()], [1])
        self.assertIn('piano', desktop.state['events'])
        clash = desktop.submit('event.create', {'event': dict(core.PIANO, id='clash', start='2026-09-25T16:30:00+08:00', end='2026-09-25T17:30:00+08:00')})
        self.assertFalse(clash['accepted'])
        self.assertEqual(clash['reason'], 'conflict:piano')
        phone.pull()
        self.assertEqual(phone.records[-1]['reason'], 'conflict:piano', 'the other device sees the rejection in the log too')
        self.assertEqual(phone.digest(), desktop.digest())
        self.assertEqual(phone.digest(), self.live.server.store.snapshot()['digest'])
        self.assertEqual(phone.state['seq'], 1, 'rejected records do not advance the state sequence')
        self.assertEqual(phone.seq, 2, 'but they do advance the log cursor')

    def test_replaying_an_identical_operation_is_a_no_op_and_a_reused_id_is_refused(self):
        phone = self.live.client('alex-phone')
        first = phone.submit('event.delete', {'id': 'dinner'}, op_id='phone-op-1')
        again = phone.submit('event.delete', {'id': 'dinner'}, op_id='phone-op-1')
        self.assertTrue(again['accepted'] and again['replayed'])
        self.assertEqual(phone.state['seq'], 1)
        reused = phone.submit('event.delete', {'id': 'dentist'}, op_id='phone-op-1')
        self.assertEqual(reused['reason'], 'operation_id_reused')
        self.assertFalse(phone.state['events']['dentist']['deleted'])
        self.assertEqual(first['seq'], 1)

    def test_long_poll_wakes_when_the_other_device_writes(self):
        phone, desktop = self.live.client('alex-phone'), self.live.client('sam-desktop')
        arrived = []
        waiter = threading.Thread(target=lambda: arrived.extend(desktop.pull(wait=10)))
        started = time.monotonic()
        waiter.start()
        time.sleep(0.2)
        phone.submit('invite.respond', {'id': 'hike', 'status': 'accepted'})
        waiter.join(10)
        self.assertLess(time.monotonic() - started, 5, 'the poll returned as soon as the write landed, not at the timeout')
        self.assertEqual([r['op']['action'] for r in arrived], ['invite.respond'])
        self.assertEqual(desktop.state['invitations']['hike']['status'], 'accepted')

    def test_state_survives_a_restart_and_equals_a_replay_of_the_log(self):
        phone = self.live.client('alex-phone')
        phone.submit('event.create', PIANO_OP)
        phone.submit('calendar.set_visible', {'id': 'birthdays', 'visible': True})
        phone.submit('event.create', {'event': dict(core.PIANO, id='dup')})  # rejected: conflict
        before = self.live.server.store.snapshot()
        self.live.stop()
        self.live = LiveServer(self.db)
        after = self.live.server.store.snapshot()
        self.assertEqual(after, before)
        self.assertEqual(after['seq'], 3)
        replayed = core.replay(self.live.server.store.records(0))
        self.assertEqual(core.state_digest(replayed), after['digest'])
        fresh = self.live.client('sam-desktop')
        self.assertTrue(fresh.state['calendars']['birthdays']['visible'])

    def test_snapshot_is_rebuilt_from_the_log_when_it_lags(self):
        phone = self.live.client('alex-phone')
        phone.submit('event.delete', {'id': 'dinner'})
        self.live.stop()
        import sqlite3
        db = sqlite3.connect(self.db)
        db.execute('DELETE FROM snapshot')
        db.commit()
        db.close()
        self.live = LiveServer(self.db)
        self.assertTrue(self.live.server.store.state['events']['dinner']['deleted'])
        self.assertEqual(self.live.server.store.seq, 1)

    def test_auth_cors_and_malformed_bodies(self):
        conn = http.client.HTTPConnection('127.0.0.1', self.live.server.server_address[1], timeout=5)
        def call(method, path, body=None, headers=None):
            conn.request(method, path, body=body, headers=headers or {})
            response = conn.getresponse()
            return response.status, dict(response.getheaders()), response.read()
        self.assertEqual(call('GET', '/v1/health')[0], 200)
        self.assertEqual(call('GET', '/v1/state')[0], 401)
        self.assertEqual(call('GET', '/v1/state', headers={'Authorization': 'Bearer wrong'})[0], 401)
        self.assertEqual(call('POST', '/v1/ops', b'not json', {'Authorization': 'Bearer test-token'})[0], 400)
        status, _, body = call('POST', '/v1/ops', b'{"id":"x","actor":"alex-phone","action":"event.fly"}', {'Authorization': 'Bearer test-token'})
        self.assertEqual(status, 200)
        self.assertEqual(json.loads(body)['reason'], 'unknown_action')
        status, headers, _ = call('OPTIONS', '/v1/ops', headers={'Origin': 'http://127.0.0.1:4321', 'Access-Control-Request-Method': 'POST'})
        self.assertEqual(status, 204)
        self.assertEqual(headers.get('Access-Control-Allow-Origin'), 'http://127.0.0.1:4321')
        self.assertIsNone(call('OPTIONS', '/v1/ops', headers={'Origin': 'https://evil.example'})[1].get('Access-Control-Allow-Origin'))
        with self.assertRaises(SyncError):
            SyncClient(self.live.url, 'wrong', 'alex-phone').bootstrap()

    def test_event_stream_delivers_records_as_they_land(self):
        phone = self.live.client('alex-phone')
        conn = http.client.HTTPConnection('127.0.0.1', self.live.server.server_address[1], timeout=10)
        conn.request('GET', '/v1/stream?since=0', headers={'Authorization': 'Bearer test-token'})
        response = conn.getresponse()
        self.assertEqual(response.getheader('Content-Type'), 'text/event-stream; charset=utf-8')
        phone.submit('device.sync', {'at': '2026-09-24T09:50:00+08:00'})
        lines = []
        while not any(line.startswith(b'data: ') for line in lines):
            lines.append(response.fp.readline())
        record = json.loads(next(line for line in lines if line.startswith(b'data: '))[6:])
        self.assertEqual(record['seq'], 1)
        self.assertEqual(record['op']['action'], 'device.sync')
        conn.close()


if __name__ == '__main__':
    unittest.main()
