"""Open standalone on-device Mail, or opt into the legacy USB companion."""
import argparse
import json
from pathlib import Path
import shlex
import shutil
import subprocess


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--adb', default=shutil.which('adb'), help='Existing Android platform-tools/adb')
    parser.add_argument('--serial', help='Required when multiple devices are attached')
    parser.add_argument('--apk', type=Path, help='Install a built preview APK before opening it')
    parser.add_argument('--capture', action='store_true', help='Enable the app-owned GPU capture hook')
    parser.add_argument('--companion', action='store_true', help='Use the legacy Mac USB companion')
    parser.add_argument('--demo', action='store_true', help='Use an isolated on-device fictional mailbox')
    parser.add_argument('--bootstrap', type=Path, help='Consume a local account JSON into private Android storage')
    parser.add_argument('--probe', action='store_true', help='Export bounds and counts for owned native tests')
    parser.add_argument('--record', action='store_true', help='Capture app-owned GPU frames; requires --demo')
    args = parser.parse_args()
    if args.record and not args.demo:
        parser.error('--record requires --demo')
    if args.demo and (args.companion or args.bootstrap):
        parser.error('--demo cannot import an account or use a companion')
    if not args.adb:
        parser.error('adb is not on PATH; pass --adb /path/to/platform-tools/adb')
    devices = subprocess.check_output([args.adb, 'devices'], text=True)
    attached = [row.split()[0] for row in devices.splitlines()[1:] if len(row.split()) >= 2 and row.split()[1] == 'device']
    if not args.serial:
        if len(attached) != 1:
            parser.error('connect one authorised device or specify --serial')
        args.serial = attached[0]
    if args.serial not in attached:
        parser.error('selected device is not connected and authorised')
    adb = [args.adb, '-s', args.serial]
    package = 'dev.makepad.octosense.mailpreview'
    running = subprocess.run(adb + ['shell', 'pidof', package], capture_output=True, text=True)
    if running.stdout.strip():
        parser.error('close the existing OctoSense Mail preview before starting a new session')
    root = Path(__file__).resolve().parents[1]
    if args.apk:
        subprocess.run(adb + ['install', '--no-incremental', '-r', str(args.apk.resolve())], check=True)
    config = {'test_actions': ['launch-mail'], 'mail_demo': args.demo}
    if args.companion:
        receipt = json.loads((root / 'runtime/latest-android.json').read_text())
        endpoint = receipt['endpoint']
        from urllib.parse import urlsplit
        from urllib.request import Request, urlopen
        address = urlsplit(endpoint)
        if address.hostname != '127.0.0.1' or address.scheme != 'http' or not address.port:
            parser.error('the companion receipt must name its USB loopback service')
        with urlopen(Request(endpoint + '/exchange', b'{"nonce":"","sequence":0,"actions":[]}', {'Content-Type':'application/json'}), timeout=5) as response:
            if response.status != 200:
                parser.error('Mail companion is unavailable')
        subprocess.run(adb + ['reverse', f'tcp:{address.port}', f'tcp:{address.port}'], check=True)
        config['mail_endpoint'] = endpoint
    folder = f'/sdcard/Android/data/{package}/files'
    if args.capture or args.probe or args.record or args.bootstrap:
        subprocess.run(adb + ['shell', 'mkdir', '-p', folder], check=True)
    if args.bootstrap:
        destination = folder + '/mail-bootstrap.json'
        subprocess.run(adb + ['push', str(args.bootstrap.resolve()), destination], check=True, stdout=subprocess.DEVNULL)
        subprocess.run(adb + ['shell', 'chmod', '600', destination], check=True)
        config['mail_bootstrap'] = destination
    if args.probe:
        config['mail_probe'] = folder + '/mail-probe.json'
    if args.capture:
        config['test_actions'].append(f'capture:{folder}/mail-capture.png')
    if args.record:
        config['test_actions'].append(f'record:{folder}/mail-recording')
    command = ['am', 'start', '-n', package + '/.MakepadApp', '--es', 'makepad.APP_CONFIG', json.dumps(config)]
    subprocess.run(adb + ['shell', shlex.join(command)], check=True)
    print('Mail opened: ' + ('Mac USB companion.' if args.companion else 'independent on-device services; no Mac companion required.'))


if __name__ == '__main__':
    main()
