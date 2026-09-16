"""A device's view of the calendar service: submit operations, follow the log,
keep a local replica by replaying accepted records through the shared reducer.
Used by the server tests and by native runtimes that host the calendar cards."""
import json
import sys
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import calendar_core as core  # noqa: E402


class SyncError(Exception):
    pass


class SyncClient:
    def __init__(self, base_url, token, device, timeout=40):
        self.base_url = base_url.rstrip('/')
        self.token = token
        self.device = device
        self.timeout = timeout
        self.state = core.initial_state()
        self.seq = 0
        self.records = []
        self._counter = 0

    def _request(self, method, path, body=None, timeout=None):
        request = urllib.request.Request(self.base_url + path, method=method, headers={'Authorization': 'Bearer ' + self.token})
        data = None
        if body is not None:
            data = json.dumps(body, ensure_ascii=False).encode('utf-8')
            request.add_header('Content-Type', 'application/json')
        try:
            with urllib.request.urlopen(request, data, timeout=timeout or self.timeout) as response:
                return response.status, json.loads(response.read().decode('utf-8'))
        except urllib.error.HTTPError as error:
            return error.code, json.loads(error.read().decode('utf-8') or '{}')
        except (urllib.error.URLError, TimeoutError, OSError) as error:
            raise SyncError(str(error)) from error

    def next_id(self):
        self._counter += 1
        return f'{self.device}-{self.seq}-{self._counter}'

    def bootstrap(self):
        """Adopt the server's snapshot; the local replica starts from there."""
        status, body = self._request('GET', '/v1/state')
        if status != 200:
            raise SyncError(f'state: {status} {body}')
        self.state, self.seq = body['state'], body['seq']
        if core.state_digest(self.state) != body['digest']:
            raise SyncError('snapshot digest mismatch')
        return body

    def ingest(self, records):
        for record in records:
            if record['seq'] <= self.seq:
                continue
            if record['accepted']:
                self.state, _ = core.apply(self.state, record['op'])
            self.seq = record['seq']
            self.records.append(record)
        return records

    def pull(self, wait=0):
        status, body = self._request('GET', f'/v1/ops?since={self.seq}&wait={wait}', timeout=self.timeout + wait)
        if status != 200:
            raise SyncError(f'ops: {status} {body}')
        return self.ingest(body['records'])

    def submit(self, action, payload=None, op_id=None):
        """Send one operation and fold the server's verdict into the replica.
        Returns the record; check record['accepted'] and record['reason']."""
        op = {'id': op_id or self.next_id(), 'actor': self.device, 'action': action, 'payload': payload or {}}
        status, record = self._request('POST', '/v1/ops', op)
        if status != 200:
            raise SyncError(f'submit: {status} {record}')
        # Anything the server logged before ours (another device) arrives with it.
        self.pull()
        return record

    def digest(self):
        return core.state_digest(self.state)

    def follow(self, on_record, stop=None, wait=25):
        """Long-poll forever (or until stop() is true), handing each new record to on_record."""
        while not (stop and stop()):
            for record in self.pull(wait=wait):
                on_record(record)
