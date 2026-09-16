"""Run Mail in the OctoSense-mobile desktop phone shell using its Cargo app host.

This is a macOS launcher preview. Android uses linked modules and cannot run
this Python mail controller or the desktop process host.
"""
from __future__ import annotations

import argparse
from datetime import datetime, timezone
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import sys
import time
import uuid


def main():
    source = Path(__file__).resolve().parents[1]
    organization = source.parents[2]
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--launcher-root', type=Path, default=organization / 'OctoSense-mobile')
    parser.add_argument('--runtime-root', type=Path, default=organization)
    parser.add_argument('--sample', action='store_true', help='Use isolated fictional mail and account settings')
    parser.add_argument('--headless', action='store_true', help='Hide the native Metal launcher window')
    parser.add_argument('--open-mail', action='store_true', help='Open Mail immediately after launcher startup')
    args = parser.parse_args()
    launcher = args.launcher_root.resolve()
    runtime_root = args.runtime_root.resolve()
    session = source / 'runtime/launcher' / (datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%SZ') + '-' + uuid.uuid4().hex[:6])
    session.mkdir(parents=True, mode=0o700)
    os.environ['OCTOS_MAIL_RUNTIME'] = str(session)
    os.environ['OCTOS_MAIL_NATIVE_ROOT'] = str(runtime_root)
    if args.sample:
        os.environ['OCTOS_MAIL_PRIVATE_DIR'] = str(session / 'private')
        os.environ['OCTOS_MAIL_CREDENTIALS'] = str(session / 'sample-account.json')
        # A sample session must never inherit a real Gmail login.
        for key in ('GMAIL_ADDRESS', 'GMAIL_APP_PASSWORD'):
            os.environ.pop(key, None)

    import run as mail
    from account import defaults, save_account
    from mailbox import atomic_json
    from instrument import Instrument
    from core.native_runtime import verify

    verify(runtime_root)
    subprocess.run([sys.executable, 'tools/setup-native.py', '--check'], cwd=launcher, check=True,
                   stdout=subprocess.DEVNULL)
    if args.sample:
        save_account({**defaults('preview@example.com'), 'password': '', 'sync_enabled': False})
    app = mail.MailSession(live=not args.sample)
    app.build = 'launcher-' + uuid.uuid4().hex
    app.mount()
    catalog = session / 'apps.json'
    atomic_json(catalog, [{
        'id': 'mail', 'label': 'Mail', 'policy': 'focus',
        'manifest': str(runtime_root / 'octoscript-makepad/Cargo.toml'),
        'package': 'kit-host', 'bin': 'beauty-host',
    }])
    env = os.environ.copy()
    env.update(BEAUTY_REQUEST=str(mail.CURRENT), OCTOSENSE_HOME=str(session / 'launcher-home'),
               CARGO_PROFILE_RELEASE_LTO='false', RUSTFLAGS='')
    for key in ('STUDIO_HOST', 'STUDIO_BUILD', 'STUDIO_CRATE', 'MAKEPAD_REMOTE', 'MAKEPAD_FOCUS', 'CARGO_TARGET_DIR'):
        env.pop(key, None)
    if args.headless:
        env['MAKEPAD_HIDE_WINDOWS'] = '1'
    else:
        env.pop('MAKEPAD_HIDE_WINDOWS', None)

    with (session / 'launcher-build.log').open('w') as build:
        subprocess.run(['cargo', 'build', '--release', '-p', 'octosense', '--features', 'mobile-only'],
                       cwd=launcher, env=env, stdout=build, stderr=subprocess.STDOUT, check=True)
    artwork = None
    process = None
    instrument = None
    def terminate(*_):
        raise SystemExit(0)
    signal.signal(signal.SIGTERM, terminate)
    signal.signal(signal.SIGINT, terminate)
    try:
        if not mail.listening(8170):
            with (session / 'artwork.log').open('w') as log:
                artwork = subprocess.Popen([sys.executable, '-m', 'http.server', '8170', '--bind', '127.0.0.1',
                    '--directory', str(mail.PIPELINE / 'docs/reviews/theme-phone-evidence')], stdout=log, stderr=log)
        command = [str(launcher / 'target/release/octosense'), '--remote', '--apps', str(catalog)]
        if args.open_mail:
            command += ['--test-action', 'launch-mail']
        logfile = session / 'launcher.log'
        with logfile.open('w') as log:
            os.chmod(logfile, 0o600)
            process = subprocess.Popen(command, cwd=launcher, env=env, stdout=log, stderr=subprocess.STDOUT,
                                       start_new_session=True)
        deadline = time.monotonic() + 30
        while time.monotonic() < deadline:
            match = re.search(r'\[makepad-remote\] listening on (127\.0\.0\.1:\d+)', logfile.read_text())
            if match:
                instrument = Instrument('http://' + match.group(1))
                break
            if process.poll() is not None:
                raise RuntimeError('Launcher exited during startup; see ' + str(logfile))
            time.sleep(.1)
        if instrument is None:
            raise RuntimeError('Launcher instrument startup timed out')
        receipt = {'pid': process.pid, 'controller_pid': os.getpid(), 'endpoint': instrument.endpoint,
                   'session': str(session), 'sample': args.sample, 'hidden': args.headless,
                   'hosting': 'OctoSense-mobile desktop Cargo process host', 'runtime': verify(runtime_root)}
        atomic_json(session / 'remote.json', receipt)
        atomic_json(source / 'runtime/latest-launcher.json', receipt)
        print(json.dumps(receipt), flush=True)
        while process.poll() is None:
            app.poll()
            time.sleep(.06)
    finally:
        if process is not None and process.poll() is None:
            if instrument:
                try:
                    instrument.get('/quit')
                except Exception:
                    pass
            try:
                process.wait(timeout=8)
            except subprocess.TimeoutExpired:
                process.terminate()
                process.wait(timeout=5)
        if artwork is not None:
            artwork.terminate()
            artwork.wait(timeout=5)


if __name__ == '__main__':
    main()
