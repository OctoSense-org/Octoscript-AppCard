"""Measured native bounds + actual Android input for an owned Mail preview."""
import json
import subprocess
import shlex
import time


class DeviceInstrument:
    package = 'dev.makepad.octosense.mailpreview'
    files = '/sdcard/Android/data/' + package + '/files'

    def __init__(self, adb, serial):
        self.adb = [str(adb), '-s', serial]

    def shell(self, *args):
        return subprocess.check_output(self.adb + ['shell', shlex.join(list(map(str, args)))], text=True)

    def probe(self):
        return json.loads(subprocess.check_output(self.adb + ['exec-out', 'cat', self.files + '/mail-probe.json']))

    def wait(self, predicate, timeout=15):
        end = time.monotonic() + timeout
        while time.monotonic() < end:
            try:
                value = self.probe()
                if predicate(value):
                    return value
            except (ValueError, KeyError, subprocess.CalledProcessError):
                pass
            time.sleep(.15)
        raise TimeoutError('Mail native state did not settle')

    def foreground(self):
        activity = self.shell('dumpsys', 'activity', 'activities')
        assert self.package + '/.MakepadApp' in activity.split('topResumedActivity=')[1].split('\n')[0], 'Owned Mail preview is not foreground'

    def bounds(self, source_id):
        def ready(p):
            native = p['mapping'].get(source_id)
            return native and any(row['id'] == native for row in p['layout'])
        p = self.wait(ready)
        native = p['mapping'][source_id]
        return next(row['bounds'] for row in p['layout'] if row['id'] == native)

    def tap(self, source_id):
        self.foreground()
        x, y, w, h = self.bounds(source_id)
        density = self.shell('wm', 'density').strip().splitlines()[-1].split(':')[-1].strip()
        scale = int(density) / 160
        self.shell('input', 'tap', round((x + w / 2) * scale), round((y + h / 2) * scale))

    def type(self, value):
        self.foreground()
        self.shell('input', 'text', value.replace(' ', '%s'))

    def swipe(self, x1, y1, x2, y2, milliseconds=650):
        self.foreground()
        self.shell('input', 'swipe', x1, y1, x2, y2, milliseconds)
