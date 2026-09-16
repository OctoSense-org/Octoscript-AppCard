#!/usr/bin/env python3
"""The calendar sync server: one SQLite database, one ordered log, one reducer.

Every client (phone app, desktop cards, browser wizard, tests) submits
operations here; the server applies them with service/calendar_core.py in
arrival order, records the verdict — accepted or rejected, with the reason —
and hands the log back to whoever asks for `since=<seq>`. Clients replay the
accepted records through the same reducer, so every device converges on the
same state without trusting each other.

    python3 server/calendar_server.py --db runtime/calendar.sqlite --port 8190

HTTP (JSON; every request except /v1/health needs `Authorization: Bearer <token>`):
    GET  /v1/health                → {ok, seq}
    GET  /v1/state                 → {seq, state, digest}
    GET  /v1/ops?since=N[&wait=S]  → {seq, records}; blocks up to S seconds for news
    GET  /v1/stream?since=N        → text/event-stream of the same records
    POST /v1/ops  {id, actor, action, payload} → the record with its verdict

Standard library only: sqlite3 (WAL), ThreadingHTTPServer, a Condition for
long-polls. The token comes from --token, $CALENDAR_TOKEN, or a token file the
server creates next to the database on first start (owner-only permissions).
"""
import argparse
import json
import os
import secrets
import sqlite3
import sys
import threading
import time
from datetime import datetime, timezone
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import parse_qs, urlsplit

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'service'))
import calendar_core as core  # noqa: E402

SCHEMA = """
CREATE TABLE IF NOT EXISTS log (
  seq INTEGER PRIMARY KEY AUTOINCREMENT,
  op_id TEXT NOT NULL, actor TEXT NOT NULL, action TEXT NOT NULL,
  op_json TEXT NOT NULL, accepted INTEGER NOT NULL, reason TEXT, replayed INTEGER NOT NULL DEFAULT 0,
  received_at TEXT NOT NULL);
CREATE INDEX IF NOT EXISTS log_op_id ON log(op_id);
CREATE TABLE IF NOT EXISTS snapshot (id INTEGER PRIMARY KEY CHECK (id = 1), seq INTEGER NOT NULL, state_json TEXT NOT NULL);
"""


def now():
    return datetime.now(timezone.utc).isoformat(timespec='milliseconds')


class Store:
    """The database: the log is the truth, the snapshot is a cache of replaying it."""

    def __init__(self, path):
        self.path = str(path)
        self.lock = threading.RLock()
        self.changed = threading.Condition(self.lock)
        self.db = sqlite3.connect(self.path, check_same_thread=False, isolation_level=None)
        self.db.execute('PRAGMA journal_mode=WAL')
        self.db.execute('PRAGMA synchronous=NORMAL')
        self.db.executescript(SCHEMA)
        self.state, self.seq = self._load()

    def _load(self):
        row = self.db.execute('SELECT seq, state_json FROM snapshot WHERE id = 1').fetchone()
        state = json.loads(row[1]) if row else core.initial_state()
        seq = row[0] if row else 0
        # Anything logged after the snapshot (an interrupted write) is replayed.
        for log_seq, op_json, accepted in self.db.execute('SELECT seq, op_json, accepted FROM log WHERE seq > ? ORDER BY seq', (seq,)):
            if accepted:
                state, _ = core.apply(state, json.loads(op_json))
            seq = log_seq
        return state, seq

    def submit(self, op):
        with self.lock:
            received = now()
            try:
                state, result = core.apply(self.state, op)
                accepted, reason, replayed = True, None, result['replayed']
            except core.Rejected as error:
                state, accepted, reason, replayed = self.state, False, error.reason, False
            self.db.execute('BEGIN IMMEDIATE')
            try:
                cur = self.db.execute('INSERT INTO log (op_id, actor, action, op_json, accepted, reason, replayed, received_at) VALUES (?,?,?,?,?,?,?,?)',
                                      (op.get('id') if isinstance(op, dict) else None, str(op.get('actor', '')) if isinstance(op, dict) else '',
                                       str(op.get('action', '')) if isinstance(op, dict) else '', core.canonical(op), int(accepted), reason, int(replayed), received))
                seq = cur.lastrowid
                self.db.execute('INSERT INTO snapshot (id, seq, state_json) VALUES (1, ?, ?) ON CONFLICT(id) DO UPDATE SET seq = excluded.seq, state_json = excluded.state_json',
                                (seq, core.canonical(state)))
                self.db.execute('COMMIT')
            except Exception:
                self.db.execute('ROLLBACK')
                raise
            self.state, self.seq = state, seq
            self.changed.notify_all()
            return {'seq': seq, 'op': op, 'accepted': accepted, 'reason': reason, 'replayed': replayed, 'received_at': received}

    def records(self, since):
        with self.lock:
            rows = self.db.execute('SELECT seq, op_json, accepted, reason, replayed, received_at FROM log WHERE seq > ? ORDER BY seq', (since,)).fetchall()
        return [{'seq': r[0], 'op': json.loads(r[1]), 'accepted': bool(r[2]), 'reason': r[3], 'replayed': bool(r[4]), 'received_at': r[5]} for r in rows]

    def wait(self, since, timeout):
        """Block until the log grows past `since` or `timeout` seconds pass."""
        deadline = time.monotonic() + timeout
        with self.lock:
            while self.seq <= since:
                remaining = deadline - time.monotonic()
                if remaining <= 0:
                    return False
                self.changed.wait(remaining)
            return True

    def snapshot(self):
        with self.lock:
            return {'seq': self.seq, 'state': json.loads(core.canonical(self.state)), 'digest': core.state_digest(self.state)}

    def close(self):
        with self.lock:
            self.db.close()


def load_token(explicit, db_path):
    if explicit:
        return explicit
    if os.environ.get('CALENDAR_TOKEN'):
        return os.environ['CALENDAR_TOKEN']
    token_file = Path(db_path).with_suffix('.token')
    if not token_file.exists():
        token_file.parent.mkdir(parents=True, exist_ok=True)
        token_file.write_text(secrets.token_urlsafe(32))
        os.chmod(token_file, 0o600)
    return token_file.read_text().strip()


class Handler(BaseHTTPRequestHandler):
    server_version = 'octosense-calendar/1'
    protocol_version = 'HTTP/1.1'

    def log_message(self, fmt, *args):
        if self.server.verbose:
            sys.stderr.write('[calendar-server] ' + fmt % args + '\n')

    # -- plumbing -----------------------------------------------------------
    def cors(self):
        origin = self.headers.get('Origin')
        if origin and origin in self.server.allow_origins:
            self.send_header('Access-Control-Allow-Origin', origin)
            self.send_header('Vary', 'Origin')
            self.send_header('Access-Control-Allow-Headers', 'authorization, content-type')
            self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')

    def reply(self, status, body, content_type='application/json'):
        data = (json.dumps(body, ensure_ascii=False) if content_type == 'application/json' else body).encode('utf-8')
        self.send_response(status)
        self.send_header('Content-Type', content_type + '; charset=utf-8')
        self.send_header('Content-Length', str(len(data)))
        self.send_header('Cache-Control', 'no-store')
        self.cors()
        self.end_headers()
        self.wfile.write(data)

    def authorized(self):
        header = self.headers.get('Authorization', '')
        if header.startswith('Bearer ') and secrets.compare_digest(header[7:].strip(), self.server.token):
            return True
        self.reply(401, {'error': 'unauthorized'})
        return False

    def query(self):
        return {k: v[-1] for k, v in parse_qs(urlsplit(self.path).query).items()}

    def since(self, q):
        try:
            return max(0, int(q.get('since', '0')))
        except ValueError:
            self.reply(400, {'error': 'since must be an integer'})
            return None

    # -- routes -------------------------------------------------------------
    def do_OPTIONS(self):
        self.send_response(204)
        self.cors()
        self.send_header('Content-Length', '0')
        self.end_headers()

    def do_GET(self):
        path = urlsplit(self.path).path
        store = self.server.store
        if path == '/v1/health':
            return self.reply(200, {'ok': True, 'seq': store.seq})
        if not self.authorized():
            return
        if path == '/v1/state':
            return self.reply(200, store.snapshot())
        if path == '/v1/ops':
            q = self.query()
            since = self.since(q)
            if since is None:
                return
            wait = min(float(q.get('wait', '0') or 0), self.server.max_wait)
            if wait > 0:
                store.wait(since, wait)
            return self.reply(200, {'seq': store.seq, 'records': store.records(since)})
        if path == '/v1/stream':
            return self.stream()
        self.reply(404, {'error': 'not found'})

    def stream(self):
        store = self.server.store
        since = self.since(self.query())
        if since is None:
            return
        self.send_response(200)
        self.send_header('Content-Type', 'text/event-stream; charset=utf-8')
        self.send_header('Cache-Control', 'no-store')
        self.send_header('Connection', 'close')
        self.cors()
        self.end_headers()
        try:
            while not self.server.stopping:
                records = store.records(since)
                for record in records:
                    self.wfile.write(f'id: {record["seq"]}\nevent: record\ndata: {json.dumps(record, ensure_ascii=False)}\n\n'.encode('utf-8'))
                    since = record['seq']
                self.wfile.flush()
                if not store.wait(since, self.server.keepalive):
                    self.wfile.write(b': keepalive\n\n')
                    self.wfile.flush()
        except (BrokenPipeError, ConnectionResetError):
            pass

    def do_POST(self):
        path = urlsplit(self.path).path
        if not self.authorized():
            return
        if path != '/v1/ops':
            return self.reply(404, {'error': 'not found'})
        try:
            length = int(self.headers.get('Content-Length', '0'))
            if length > self.server.max_body:
                return self.reply(413, {'error': 'operation too large'})
            op = json.loads(self.rfile.read(length).decode('utf-8'))
        except (ValueError, UnicodeDecodeError):
            return self.reply(400, {'error': 'body must be a JSON operation'})
        if not isinstance(op, dict):
            return self.reply(400, {'error': 'body must be a JSON operation'})
        record = self.server.store.submit(op)
        self.reply(200, record)


class CalendarServer(ThreadingHTTPServer):
    daemon_threads = True
    allow_reuse_address = True

    def __init__(self, address, store, token, allow_origins=(), verbose=False):
        super().__init__(address, Handler)
        self.store, self.token, self.allow_origins, self.verbose = store, token, set(allow_origins), verbose
        self.max_wait, self.keepalive, self.max_body, self.stopping = 30.0, 15.0, 64 * 1024, False

    def shutdown(self):
        self.stopping = True
        with self.store.lock:
            self.store.changed.notify_all()
        super().shutdown()


def serve(db, port=8190, bind='127.0.0.1', token=None, allow_origins=(), verbose=False, announce=print):
    store = Store(db)
    server = CalendarServer((bind, port), store, load_token(token, db), allow_origins, verbose)
    announce(f'[calendar-server] listening on http://{bind}:{server.server_address[1]} db={db} seq={store.seq}')
    return server


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.split('\n\n')[0])
    parser.add_argument('--db', default=str(Path(__file__).resolve().parents[1] / 'runtime/calendar.sqlite'))
    parser.add_argument('--port', type=int, default=8190)
    parser.add_argument('--bind', default='127.0.0.1')
    parser.add_argument('--token', help='shared bearer token (default: $CALENDAR_TOKEN or a generated <db>.token file)')
    parser.add_argument('--allow-origin', action='append', default=[], help='browser origin allowed to call the API (repeatable)')
    parser.add_argument('--verbose', action='store_true')
    args = parser.parse_args(argv)
    Path(args.db).parent.mkdir(parents=True, exist_ok=True)
    server = serve(args.db, args.port, args.bind, args.token, args.allow_origin, args.verbose, announce=lambda line: print(line, flush=True))
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.shutdown()
        server.store.close()


if __name__ == '__main__':
    main()
