import copy
from contextlib import redirect_stdout
import importlib.util
from io import StringIO
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

from PIL import Image

SPEC = importlib.util.spec_from_file_location('image_to_appcard_flow_atlas', Path(__file__).with_name('atlas.py'))
atlas = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(atlas)


class AtlasIntakeTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.project = self.root / 'project'
        self.project.mkdir()
        self.source = self.project / 'atlas.png'
        image = Image.new('RGB', (44, 46))
        image.putdata([(x * 5 % 256, y * 5 % 256, (x + y) * 3 % 256)
                       for y in range(46) for x in range(44)])
        image.save(self.source)
        self.prompt = self.project / 'prompt.md'
        self.prompt.write_bytes('一次生成全部八个页面\r\n保持原生应用与桌面卡片分开\n'.encode())
        self.manifest = {'schema_version': 1, 'id': 'fresh-service-flow', 'artboard': [40, 76],
                         'generation': {'atlas': 'atlas.png', 'prompt': 'prompt.md',
                                        'provider': 'Caller-supplied fixture', 'model': 'example-image-model',
                                        'requested_size': [80, 80], 'quality': 'high'},
                         'scenes': [{'id': str(i + 1), 'design_id': f'example-{i + 1:02d}',
                                     'crop': [(i % 4) * 11, (i // 4) * 23, 10, 22],
                                     'surface': 'app' if i < 2 else 'desktop'} for i in range(8)]}
        self.manifest_path = self.project / 'flow.json'
        self.output = self.root / 'intake'
        self.save_manifest()

    def tearDown(self):
        self.temp.cleanup()

    def save_manifest(self):
        self.manifest_path.write_text(json.dumps(self.manifest, ensure_ascii=False, indent=2) + '\n')

    def run_intake(self):
        return atlas.intake(self.manifest_path, self.project, self.output)

    def snapshot(self, path):
        return {p.relative_to(path).as_posix(): (atlas.digest(p.read_bytes()), p.stat().st_mtime_ns)
                for p in path.rglob('*') if p.is_file()}

    def test_exact_crop_and_original_bytes_preserved_and_requested_actual_separate(self):
        result = self.run_intake()
        self.assertEqual(result['generation']['actual_size'], [44, 46])
        self.assertEqual(result['generation']['requested_size'], [80, 80])
        self.assertFalse(result['generation']['requested_size_matches_actual'])
        self.assertFalse(result['generation']['performed_by_intake'])
        self.assertEqual((self.output / result['originals']['atlas']).read_bytes(), self.source.read_bytes())
        self.assertEqual((self.output / result['originals']['prompt']).read_bytes(), self.prompt.read_bytes())
        self.assertEqual((self.output / result['originals']['manifest']).read_bytes(), self.manifest_path.read_bytes())
        original = Image.open(self.source)
        for scene in result['scenes']:
            x, y, w, h = scene['crop']
            crop = Image.open(self.output / scene['crop_image'])
            self.assertEqual(crop.size, (w, h))
            self.assertEqual(crop.tobytes(), original.crop((x, y, x + w, y + h)).tobytes())
            self.assertFalse(scene['crop_resampled'])
            self.assertEqual(scene['source_review']['status'], 'not_reviewed')
            self.assertEqual(scene['native_mapping']['status'], 'not_started')

    def test_uniform_fit_at_two_x_has_reversible_pixel_and_logical_transforms(self):
        self.manifest['artboard'] = [406, 776]
        self.save_manifest()
        result = self.run_intake()
        scene = result['scenes'][3]
        transform = scene['transform']
        with Image.open(self.output / scene['reference_image']) as reference:
            self.assertEqual(reference.size, (812, 1552))
        self.assertEqual(transform['crop_to_reference_pixels'][0], transform['crop_to_reference_pixels'][4])
        self.assertFalse(transform['stretched'])
        def point(matrix, x, y):
            a, b, c, d, e, f = matrix
            return a * x + b * y + c, d * x + e * y + f
        for x, y in [(33, 0), (38.25, 14.5), (43, 22)]:
            rendered = point(transform['atlas_to_reference_pixels'], x, y)
            restored = point(transform['reference_pixels_to_atlas'], *rendered)
            logical = point(transform['atlas_to_logical'], x, y)
            restored_logical = point(transform['logical_to_atlas'], *logical)
            self.assertAlmostEqual(restored[0], x)
            self.assertAlmostEqual(restored[1], y)
            self.assertAlmostEqual(restored_logical[0], x)
            self.assertAlmostEqual(restored_logical[1], y)

    def test_idempotent_replay_does_not_touch_files_or_reviewed_additions(self):
        self.run_intake()
        reviewed = self.output / 'reviewed-mapped.json'
        reviewed.write_text('{"preserve":"reviewed work"}')
        before = self.snapshot(self.output)
        result = self.run_intake()
        self.assertTrue(result['replayed'])
        self.assertEqual(self.snapshot(self.output), before)

    def test_changed_source_prompt_or_manifest_requires_new_output(self):
        self.run_intake()
        before = self.snapshot(self.output)
        for path in [self.prompt, self.source, self.manifest_path]:
            with self.subTest(path=path.name):
                original = path.read_bytes()
                if path == self.source:
                    Image.new('RGB', (44, 46), 'red').save(path)
                elif path == self.prompt:
                    path.write_bytes(original + b'Another instruction\n')
                else:
                    path.write_bytes(original + b'\n')
                with self.assertRaisesRegex(ValueError, 'input changed'):
                    self.run_intake()
                self.assertEqual(self.snapshot(self.output), before)
                path.write_bytes(original)

    def test_existing_reviewed_directory_is_not_overwritten(self):
        self.output.mkdir()
        (self.output / 'mapped.json').write_text('{"approved":true}')
        (self.output / 'native.png').write_bytes(b'Historical evidence')
        before = self.snapshot(self.output)
        with self.assertRaisesRegex(ValueError, 'already exists'):
            self.run_intake()
        self.assertEqual(self.snapshot(self.output), before)

    def test_tampered_crop_or_receipt_is_rejected_without_overwrite(self):
        result = self.run_intake()
        crop = self.output / result['scenes'][0]['crop_image']
        original = crop.read_bytes()
        crop.write_bytes(b'modified')
        before = self.snapshot(self.output)
        with self.assertRaisesRegex(ValueError, 'artifact was modified'):
            self.run_intake()
        self.assertEqual(self.snapshot(self.output), before)
        crop.write_bytes(original)
        (self.output / 'intake.json').write_bytes(b'{}')
        before = self.snapshot(self.output)
        with self.assertRaisesRegex(ValueError, 'receipt was modified'):
            self.run_intake()
        self.assertEqual(self.snapshot(self.output), before)

    def test_invalid_bounds_are_checked_before_any_output_mutation(self):
        self.run_intake()
        before = self.snapshot(self.output)
        original = copy.deepcopy(self.manifest)
        for rect in [[-1, 0, 10, 22], [0, 0, 0, 22], [40, 0, 10, 22], [0, 40, 10, 22],
                     [0.0, 0, 10, 22], [True, 0, 10, 22], [0, 0, 10]]:
            with self.subTest(rect=rect):
                self.manifest = copy.deepcopy(original)
                self.manifest['scenes'][0]['crop'] = rect
                self.save_manifest()
                with self.assertRaisesRegex(ValueError, 'crop'):
                    self.run_intake()
                self.assertEqual(self.snapshot(self.output), before)

    def test_overlapping_crops_are_rejected_and_touching_edges_are_allowed(self):
        self.manifest['scenes'][1]['crop'] = [9, 0, 10, 22]
        self.save_manifest()
        with self.assertRaisesRegex(ValueError, 'overlaps'):
            self.run_intake()
        self.assertFalse(self.output.exists())
        self.manifest['scenes'][1]['crop'] = [10, 0, 10, 22]
        self.save_manifest()
        self.assertEqual(len(self.run_intake()['scenes']), 8)

    def test_scene_and_design_ids_are_unique_and_path_safe(self):
        original = copy.deepcopy(self.manifest)
        for key, value in [('id', '1'), ('design_id', 'example-01'), ('design_id', '../outside')]:
            self.manifest = copy.deepcopy(original)
            self.manifest['scenes'][1][key] = value
            self.save_manifest()
            with self.assertRaises(ValueError):
                self.run_intake()
            self.assertFalse(self.output.exists())

    def test_eight_to_twelve_scenes_are_required(self):
        original = copy.deepcopy(self.manifest)
        for count in [7, 13]:
            self.manifest = copy.deepcopy(original)
            self.manifest['scenes'] = (original['scenes'] * 2)[:count]
            self.save_manifest()
            with self.assertRaisesRegex(ValueError, '8–12'):
                self.run_intake()
        self.assertFalse(self.output.exists())

    def test_twelve_scenes_in_one_atlas_are_accepted(self):
        Image.new('RGB', (44, 69), 'white').save(self.source)
        self.manifest['scenes'] = [
            {'id': str(i + 1), 'design_id': f'example-{i + 1:02d}',
             'crop': [(i % 4) * 11, (i // 4) * 23, 10, 22], 'surface': 'desktop'}
            for i in range(12)]
        self.save_manifest()
        self.assertEqual(len(self.run_intake()['scenes']), 12)

    def test_multi_frame_image_is_not_accepted_as_one_static_atlas(self):
        first = Image.new('RGB', (44, 46), 'white')
        first.save(self.source, format='GIF', save_all=True,
                   append_images=[Image.new('RGB', (44, 46), 'black')], duration=100)
        with self.assertRaisesRegex(ValueError, 'one static atlas'):
            self.run_intake()
        self.assertFalse(self.output.exists())

    def test_integer_image_that_png_would_clip_is_rejected(self):
        Image.new('I', (44, 46), 100000).save(self.source, format='TIFF')
        with self.assertRaisesRegex(ValueError, 'mode cannot be preserved'):
            self.run_intake()
        self.assertFalse(self.output.exists())

    def test_cli_reports_requested_actual_dimensions_and_review_status(self):
        stdout = StringIO()
        with redirect_stdout(stdout):
            code = atlas.main(['--manifest', str(self.manifest_path), '--project', str(self.project),
                               '--output', str(self.output)])
        result = json.loads(stdout.getvalue())
        self.assertEqual(code, 0)
        self.assertEqual(result['actual_size'], [44, 46])
        self.assertEqual(result['requested_size'], [80, 80])
        self.assertEqual(result['review_status'], 'not_reviewed')
        self.assertEqual(result['native_mapping_status'], 'not_started')

    def test_project_paths_cannot_escape_by_absolute_traversal_or_symlink(self):
        escaped = self.root / 'outside.png'
        escaped.write_bytes(self.source.read_bytes())
        (self.project / 'escape.png').symlink_to(escaped)
        for relative in [str(escaped), '../outside.png', 'escape.png']:
            self.manifest['generation']['atlas'] = relative
            self.save_manifest()
            with self.assertRaisesRegex(ValueError, 'within the project'):
                self.run_intake()
        self.assertFalse(self.output.exists())

    def test_render_failure_does_not_publish_partial_output(self):
        with patch.object(atlas, 'fit_reference', side_effect=RuntimeError('renderer fixture failure')):
            with self.assertRaisesRegex(RuntimeError, 'renderer fixture failure'):
                self.run_intake()
        self.assertFalse(self.output.exists())
        self.assertFalse(list(self.output.parent.glob('.intake.intake-*')))

    def test_rgba_crops_keep_alpha_and_reference_uses_declared_matte(self):
        Image.new('RGBA', (44, 46), (80, 120, 160, 0)).save(self.source)
        result = self.run_intake()
        scene = result['scenes'][0]
        crop = Image.open(self.output / scene['crop_image'])
        reference = Image.open(self.output / scene['reference_image'])
        self.assertEqual(crop.mode, 'RGBA')
        self.assertEqual(crop.getpixel((3, 4)), (80, 120, 160, 0))
        self.assertEqual(reference.mode, 'RGB')
        self.assertEqual(reference.getpixel((40, 70)), atlas.MATTE)

    def test_quality_and_dimension_types_do_not_silently_change_requests(self):
        original = copy.deepcopy(self.manifest)
        cases = [('quality', 'low'), ('requested_size', [80.5, 80]), ('requested_size', [True, 80])]
        for key, value in cases:
            self.manifest = copy.deepcopy(original)
            self.manifest['generation'][key] = value
            self.save_manifest()
            with self.assertRaises(ValueError):
                self.run_intake()
        self.assertFalse(self.output.exists())


if __name__ == '__main__':
    unittest.main()
