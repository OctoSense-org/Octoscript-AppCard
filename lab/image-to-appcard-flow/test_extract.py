import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
from PIL import Image
import extract


class ExtractionTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.manifest = self.root / 'flow.json'
        self.doc = json.loads((Path(__file__).parent / 'examples/aircon.flow.json').read_text())
        self.doc['cards'] = [{'id': 'calendar-updated', 'scene': '1', 'root': 'calendar', 'owner': 'calendar'}]
        source = self.root / self.doc['scenes'][0]['directory']
        source.mkdir(parents=True)
        self.source = source
        tree = {'id': 'page', 't': 'stack', 'x': 0, 'y': 0, 'w': 406, 'h': 776, 'c': [
            {'id': 'calendar', 't': 'stack', 'x': 20, 'y': 100, 'w': 300, 'h': 200, 'c': [
                {'id': 'confirm', 't': 'button', 'x': 40, 'y': 220, 'w': 100, 'h': 40}]}]}
        self.write(source / 'mapped.json', {'tree': tree})
        self.write(source / 'contract.json', {'id': 'aircon-01', 'artboard': [406, 776], 'tree': tree})
        self.write(source / 'semantic-map.json', {'elements': [{'id': n, 'role': 'layout' if n != 'confirm' else 'button'} for n in ['page', 'calendar', 'confirm']]})
        self.write(source / 'service-actions.json', {'controls': {'confirm': {'event': 'calendar.acknowledge'}, 'outside': {'event': 'payment.pay'}}})
        self.write(source / 'generation.json', {'provider': 'fixture-generator', 'model': 'fixture-model'})
        (source / 'image-prompt.md').write_text('Exact submitted fixture prompt')
        Image.new('RGB', (812, 1552), 'white').save(source / 'reference.png')
        self.output = self.root / 'exports'

    def write(self, path, value):
        path.write_text(json.dumps(value))

    def run_export(self):
        self.write(self.manifest, self.doc)
        # Compilation itself belongs to the existing adapter's tests and native evidence.
        with patch('extract.compile_page', return_value={'nodes': 2}) as compiler:
            result = extract.extract(self.manifest, self.root, self.output)
        return result, compiler

    def test_extracts_native_tree_and_owned_actions_with_reversible_origin(self):
        result, compiler = self.run_export()
        self.assertEqual(result['cards'], 1)
        compiler.assert_called_once()
        card = self.output / 'calendar/calendar-updated'
        model = json.loads((card / 'mapped.json').read_text())
        self.assertEqual((model['tree']['x'], model['tree']['y']), (0, 0))
        self.assertEqual((model['tree']['c'][0]['x'], model['tree']['c'][0]['y']), (20, 120))
        self.assertEqual(model['changes'][0]['translation'], [-20, -100])
        actions = json.loads((card / 'service-actions.json').read_text())
        self.assertEqual(list(actions['controls']), ['confirm'])
        self.assertEqual(actions['owner'], 'calendar')
        with Image.open(card / 'reference.png') as reference:
            self.assertEqual(reference.size, (600, 400))
        self.assertEqual(json.loads((self.source / 'mapped.json').read_text())['tree']['c'][0]['x'], 20)

    def test_failed_later_card_does_not_publish_partial_batch(self):
        self.doc['cards'].append({'id': 'missing', 'scene': '1', 'root': 'missing', 'owner': 'payment'})
        with self.assertRaises(KeyError):
            self.run_export()
        self.assertFalse(self.output.exists())
        self.assertFalse(list(self.root.glob('.extract-*')))

    def test_existing_reviewed_export_is_preserved(self):
        self.output.mkdir()
        (self.output / 'review.json').write_text('reviewed')
        with self.assertRaises(FileExistsError):
            self.run_export()
        self.assertEqual((self.output / 'review.json').read_text(), 'reviewed')

    def add_numeric_chart(self):
        data_path = self.source / 'data/temperature.json'
        data_path.parent.mkdir()
        self.write(data_path, [{'hour': 0, 'temperature': 19}, {'hour': 1, 'temperature': 22}])
        model = json.loads((self.source / 'mapped.json').read_text())
        model['tree']['c'][0]['c'].append({'id': 'temperature', 't': 'stockplot', 'x': 40, 'y': 140, 'w': 180, 'h': 60})
        self.write(self.source / 'mapped.json', model)
        entry = {'id': 'temperature', 'role': 'chart.line',
                 'data': {'path': 'data/temperature.json', 'sha256': extract.sha(data_path), 'origin': 'fixture',
                          'x_key': 'hour', 'y_keys': ['temperature'], 'units': {'x': 'hour', 'y': 'degrees C'},
                          'domain': {'x': [0, 1], 'y': [10, 30]}},
                 'behavior': {'selected_series': 'temperature'}}
        semantic = json.loads((self.source / 'semantic-map.json').read_text())
        semantic['elements'].append(entry)
        self.write(self.source / 'semantic-map.json', semantic)
        return data_path, entry

    def test_data_bound_card_carries_exact_numeric_file_and_metadata(self):
        path, original = self.add_numeric_chart()
        self.run_export()
        card = self.output / 'calendar/calendar-updated'
        self.assertEqual((card / 'data/temperature.json').read_bytes(), path.read_bytes())
        semantic = json.loads((card / 'semantic-map.json').read_text())
        extracted = next(entry for entry in semantic['elements'] if entry['id'] == 'temperature')
        self.assertEqual(extracted, original)
        model = json.loads((card / 'mapped.json').read_text())
        chart = next(node for node in model['tree']['c'] if node['id'] == 'temperature')
        self.assertEqual((chart['x'], chart['y']), (20, 40))

    def test_stale_numeric_data_fails_without_publishing_output(self):
        path, _ = self.add_numeric_chart()
        self.write(path, [{'hour': 0, 'temperature': 100}])
        with self.assertRaisesRegex(ValueError, 'Stale declared numeric data'):
            self.run_export()
        self.assertFalse(self.output.exists())
        self.assertFalse(list(self.root.glob('.extract-*')))

    def test_numeric_data_path_cannot_escape_source_directory(self):
        path, _ = self.add_numeric_chart()
        escaped = self.root / 'outside.json'
        escaped.write_bytes(path.read_bytes())
        (self.source / 'escape.json').symlink_to(escaped)
        original = json.loads((self.source / 'semantic-map.json').read_text())
        for relative in ['../../outside.json', str(escaped), 'escape.json']:
            with self.subTest(path=relative):
                original['elements'][-1]['data']['path'] = relative
                self.write(self.source / 'semantic-map.json', original)
                with self.assertRaises(ValueError):
                    self.run_export()
                self.assertFalse(self.output.exists())


if __name__ == '__main__':
    unittest.main()
