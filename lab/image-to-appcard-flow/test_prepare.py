import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
from PIL import Image
import prepare


class PreparationTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.manifest = self.root / 'flow.json'
        Image.new('RGB', (80, 20), 'white').save(self.root / 'atlas.png')
        (self.root / 'prompt.txt').write_bytes(b'exact submitted prompt\r\n')
        doc = {'schema_version': 1, 'id': 'test-flow', 'artboard': [406, 776], 'locales': ['en', 'cn'],
               'generation': {'atlas': 'atlas.png', 'prompt': 'prompt.txt', 'provider': 'fixture', 'requested_size': [80, 20], 'quality': 'high'},
               'artwork': {'root': 'artwork'},
               'scenes': [{'id': str(i+1), 'design_id': f'fixture-{i+1}', 'directory': f'cards/fixture-{i+1}',
                           'surface': 'app' if i == 0 else 'desktop', 'crop': [i*10, 0, 10, 20]} for i in range(8)]}
        self.manifest.write_text(json.dumps(doc))

    def test_installs_reference_and_provenance_without_inventing_mapping(self):
        result = prepare.prepare(self.manifest, self.root, self.root / 'intake')
        self.assertEqual(len(result['prepared']), 8)
        folder = self.root / 'cards/fixture-1'
        self.assertEqual((folder / 'image-prompt.md').read_bytes(), b'exact submitted prompt\r\n')
        generation = json.loads((folder / 'generation.json').read_text())
        self.assertEqual(generation['image_sha256'], prepare.sha(folder / 'reference.png'))
        self.assertEqual(generation['source_crop'], [0, 0, 10, 20])
        self.assertFalse((folder / 'mapped.json').exists())
        self.assertFalse((folder / 'contract.json').exists())

    def test_existing_later_scene_blocks_entire_batch_before_first_copy(self):
        folder = self.root / 'cards/fixture-8'
        folder.mkdir(parents=True)
        (folder / 'mapped.json').write_text('reviewed')
        with self.assertRaises(FileExistsError):
            prepare.prepare(self.manifest, self.root, self.root / 'intake')
        self.assertFalse((self.root / 'cards/fixture-1').exists())
        self.assertEqual((folder / 'mapped.json').read_text(), 'reviewed')

    def test_live_source_changes_after_intake_do_not_taint_frozen_provenance(self):
        actual_intake = prepare.validate_intake
        original_atlas_hash = prepare.sha(self.root / 'atlas.png')
        original_prompt_hash = prepare.sha(self.root / 'prompt.txt')

        def mutate_live_sources(*args):
            receipt = actual_intake(*args)
            (self.root / 'prompt.txt').write_text('Changed after frozen intake validation')
            Image.new('RGB', (80, 20), 'black').save(self.root / 'atlas.png')
            return receipt

        with patch.object(prepare, 'validate_intake', side_effect=mutate_live_sources):
            prepare.prepare(self.manifest, self.root, self.root / 'intake')
        folder = self.root / 'cards/fixture-1'
        self.assertEqual((folder / 'image-prompt.md').read_bytes(), b'exact submitted prompt\r\n')
        generation = json.loads((folder / 'generation.json').read_text())
        self.assertEqual(generation['atlas_sha256'], original_atlas_hash)
        self.assertEqual(generation['prompt_sha256'], original_prompt_hash)
        self.assertEqual(generation['actual_size'], [80, 20])
        self.assertNotEqual(generation['atlas_sha256'], prepare.sha(self.root / 'atlas.png'))
        self.assertEqual(generation['image_sha256'], prepare.sha(folder / 'reference.png'))


if __name__ == '__main__':
    unittest.main()
