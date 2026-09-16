"""USB companion for the native Mail module. Keep mail credentials on the Mac."""
from __future__ import annotations

import argparse
from datetime import datetime, timezone
from http.server import BaseHTTPRequestHandler, HTTPServer
import json
import os
from pathlib import Path
import secrets
import signal
import shutil


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--port', type=int, default=8187)
    parser.add_argument('--sample', action='store_true')
    parser.add_argument('--fixture', type=Path, help='Fictional mailbox JSON; requires --sample')
    parser.add_argument('--cache', type=Path, help='Copy an existing live mailbox into this isolated session')
    args = parser.parse_args()
    if args.fixture and not args.sample:
        parser.error('--fixture requires --sample')
    if args.cache and args.sample:
        parser.error('--cache cannot be used with --sample')
    root = Path(__file__).resolve().parents[1]
    session = root / 'runtime/android' / datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%SZ')
    session.mkdir(parents=True, mode=0o700)
    os.umask(0o077)
    os.environ['OCTOS_MAIL_RUNTIME'] = str(session)
    os.environ['OCTOS_MAIL_PRIVATE_DIR'] = str(session / 'private')
    if args.cache:
        (session / 'private').mkdir(mode=0o700)
        shutil.copyfile(args.cache, session / 'private/mailbox.json')
    if args.sample:
        os.environ['OCTOS_MAIL_CREDENTIALS'] = str(session / 'account.json')
        for key in ('GMAIL_ADDRESS', 'GMAIL_APP_PASSWORD'):
            os.environ.pop(key, None)
    import run as mail
    from account import defaults, save_account
    from mailbox import atomic_json
    if args.sample:
        save_account({**defaults('preview@example.com'), 'password': '', 'sync_enabled': False})
    app = mail.MailSession(live=not args.sample)
    if args.fixture:
        app.state['mailbox'] = json.loads(args.fixture.read_text())
    app.build = 'android-' + secrets.token_hex(12)
    app.mount()
    token = secrets.token_urlsafe(32)
    endpoint = f'http://127.0.0.1:{args.port}/{token}'
    state = {'nonce': '', 'sequence': 0}

    def frame():
        request = app.meta['request']
        directory = Path(request['card']).parent
        data = json.loads(Path(request['data']).read_text())
        for place in data['$kit']['placements'].values():
            layout = place['layout']
            if layout.get('src', '').startswith('http://127.0.0.1:8170/'):
                layout['src'] = endpoint + '/assets/' + layout['src'].rsplit('/', 1)[-1]
        return {'nonce': request['nonce'], 'card': Path(request['card']).read_text(), 'data': data,
                'pack': json.loads((directory / 'kit/native/light/kit.json').read_text()),
                'scroll_watch': request.get('scroll_watch', []),
                'scroll_restore': request.get('scroll_restore', []),
                'preserve_input_selection': request.get('preserve_input_selection', False)}

    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *_):
            pass  # URLs contain the session capability; never log them.

        def reply(self, value, status=200, content_type='application/json'):
            content = value if isinstance(value, bytes) else json.dumps(value).encode()
            self.send_response(status)
            self.send_header('Content-Type', content_type)
            self.send_header('Content-Length', str(len(content)))
            self.send_header('Cache-Control', 'no-store')
            self.end_headers()
            self.wfile.write(content)

        def do_GET(self):
            prefix = '/' + token + '/assets/'
            if not self.path.startswith(prefix):
                self.reply({'error': 'not found'}, 404); return
            name = self.path[len(prefix):]
            if '/' in name or not name.endswith('.svg'):
                self.reply({'error': 'not found'}, 404); return
            path = Path(app.meta['request']['card']).parent / 'assets' / name
            if not path.is_file():
                self.reply({'error': 'not found'}, 404); return
            self.reply(path.read_bytes(), content_type='image/svg+xml')

        def do_POST(self):
            if self.path != '/' + token + '/exchange':
                self.reply({'error': 'not found'}, 404); return
            try:
                length = int(self.headers.get('Content-Length', '0'))
                if not 0 < length <= 262144:
                    raise ValueError('invalid length')
                request = json.loads(self.rfile.read(length))
                native = app.meta['request']
                if request.get('nonce') == native['nonce']:
                    if state['nonce'] != native['nonce']:
                        state.update(nonce=native['nonce'], sequence=0)
                        atomic_json(Path(native['actions']), [])
                    atomic_json(Path(native['result']), {'request': native})
                    atomic_json(Path(native['layout']), {'nonce': native['nonce'], 'elements': request.get('layout', [])})
                    events = json.loads(Path(native['actions']).read_text())
                    for action in request.get('actions', []):
                        if action['sequence'] > state['sequence']:
                            events.append(action)
                            state['sequence'] = action['sequence']
                    atomic_json(Path(native['actions']), events)
                    if 'scroll_state' in native:
                        atomic_json(Path(native['scroll_state']), {'nonce': native['nonce'], 'elements': request.get('scroll', [])})
                    app.poll()
                response = {'ack': request.get('sequence', 0)}
                if request.get('nonce') != app.meta['request']['nonce']:
                    response['frame'] = frame()
                self.reply(response)
            except (ValueError, KeyError, TypeError):
                self.reply({'error': 'invalid request'}, 400)

    server = HTTPServer(('127.0.0.1', args.port), Handler)
    server.timeout = .05
    receipt = {'pid': os.getpid(), 'endpoint': endpoint, 'sample': args.sample, 'session': str(session)}
    atomic_json(session / 'companion.json', receipt)
    atomic_json(root / 'runtime/latest-android.json', receipt)
    print(f'Mail companion ready; receipt: {session / "companion.json"}', flush=True)
    def stop(*_):
        raise KeyboardInterrupt
    signal.signal(signal.SIGTERM, stop)
    try:
        while True:
            server.handle_request()
            app.poll()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()


if __name__ == '__main__':
    main()
