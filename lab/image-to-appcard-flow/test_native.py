from contextlib import redirect_stdout
from io import StringIO
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

from PIL import Image

import native
from flow import sha
from semantics import POLICY


class CurrentSemanticStageTests(unittest.TestCase):
    def test_old_latest_capture_cannot_block_valid_current_preflight(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            adapter = root / 'adapter'
            adapter.mkdir()
            scenes = []
            for number in range(1, 9):
                ident = f'fixture-{number}'
                directory = root / 'cards' / ident
                directory.mkdir(parents=True)
                tree = {'id': 'page', 't': 'stack', 'x': 0, 'y': 0, 'w': 406, 'h': 776, 'c': []}
                contract = {'id': ident, 'artboard': [406, 776], 'tree': tree}
                (directory / 'contract.json').write_text(json.dumps(contract))
                (directory / 'mapped.json').write_text(json.dumps({'tree': tree}))
                (directory / 'service-actions.json').write_text('{"controls":{}}')
                Image.new('RGB', (812, 1552), 'white').save(directory / 'reference.png')
                semantic = {'schema_version': 1, 'policy_version': POLICY['version'],
                            'contract_sha256': sha(directory / 'contract.json'),
                            'reference_sha256': sha(directory / 'reference.png'),
                            'elements': [{'id': 'page', 'role': 'layout', 'basis': 'Current reviewed native composition',
                                          'confidence': 1, 'decision': 'reviewed'}]}
                (directory / 'semantic-map.json').write_text(json.dumps(semantic))
                # An old capture can remain present while its authoring model
                # is repaired. Current semantic preflight must not inspect it.
                (directory / 'latest.json').write_text('{"round":"old-capture"}')
                scenes.append({'id': str(number), 'design_id': ident, 'directory': 'cards/' + ident,
                               'surface': 'desktop', 'crop': [(number - 1) * 10, 0, 10, 20]})
            manifest = root / 'flow.json'
            manifest.write_text(json.dumps({'schema_version': 1, 'id': 'fixture-flow', 'artboard': [406, 776],
                                           'locales': ['en', 'cn'], 'generation': {'atlas': 'atlas.png', 'prompt': 'prompt.txt', 'provider': 'fixture'},
                                           'scenes': scenes, 'artwork': {'root': 'artwork'}}))
            argv = ['native.py', '--manifest', str(manifest), '--project', str(root), '--stage', 'semantic']
            with patch.object(sys, 'argv', argv), patch.object(native, 'IMAGE', adapter), \
                    patch('semantics.audit', side_effect=AssertionError('must not audit the previous capture')), redirect_stdout(StringIO()):
                native.main()
            for scene in scenes:
                result = json.loads((root / scene['directory'] / 'semantic-preflight.json').read_text())
                self.assertTrue(result['pass'])
                self.assertEqual(result['phase'], 'preflight')
                self.assertFalse((root / scene['directory'] / 'semantic-audit.json').exists())


if __name__ == '__main__':
    unittest.main()
