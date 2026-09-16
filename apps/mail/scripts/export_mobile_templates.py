"""Export the reviewed Mail scenes for the on-device Rust controller."""
import json
from copy import deepcopy
from pathlib import Path

import run  # establishes the shared authoring/service import paths
from mailbox import fixture
from model import initial
from render import build, Scene, WHITE


def main():
    box = fixture()
    box['messages'] = box['messages'][:1]
    message = box['messages'][0]
    message.update(id='demo-0', subject='Welcome to Mail', body='Read your mail on this device.',
                   html='', attachment_items=[])
    state = initial(box)
    state['selected'] = message['id']
    scenes = {}
    for page in ('inbox', 'mailboxes', 'settings', 'services', 'compose', 'drafts', 'attachments', 'card'):
        current = deepcopy(state)
        current['screen'] = page
        scene = build(current)
        scenes[page] = {'tree': scene.stack('page', 0, 0, 406, 776, scene.background, children=scene.nodes),
                        'controls': scene.controls, 'assets': scene.assets}
    state['screen'] = 'read'
    state['mailbox']['messages'][0]['html'] = '<p>Welcome to Mail.</p>'
    scene = build(state)
    scenes['read'] = {'tree': scene.stack('page', 0, 0, 406, 776, WHITE, children=scene.nodes),
                      'controls': scene.controls, 'assets': scene.assets}
    # Password input starts empty; saved secrets never enter a card or frame.
    field = Scene().field('password', '', 'New app password', 136, 375, 237, 38,
                          'password', size=15)
    field['c'][0]['password'] = 1
    scenes['password_field'] = field
    root = Path(__file__).resolve().parents[1]
    target = root / 'native/resources/mobile-templates.json'
    target.write_text(json.dumps(scenes, ensure_ascii=False, separators=(',', ':')) + '\n')
    print('Exported reviewed Mail scene templates:', target)


if __name__ == '__main__':
    main()
