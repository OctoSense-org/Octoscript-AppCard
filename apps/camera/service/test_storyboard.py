"""The storyboard is the measured Mate 70 Air layout: every scene keeps the
device's fixed geometry (docs/ux-map.md), ids are unique, and every control
sits inside the artboard."""
import sys, unittest
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'scripts'))
import design


class Storyboard(unittest.TestCase):
    def setUp(self):
        self.scenes = design.storyboard()

    def test_twelve_scenes_with_unique_ids(self):
        self.assertEqual(len(self.scenes), 12)
        for s in self.scenes:
            ids = [n['id'] for n in s.nodes]
            self.assertEqual(len(ids), len(set(ids)), s.id)

    def test_controls_inside_the_artboard(self):
        for s in self.scenes:
            for id, c in s.controls.items():
                x, y, w, h = c['source_bounds']
                self.assertGreaterEqual(x, 0, (s.id, id)); self.assertGreaterEqual(y, 0, (s.id, id))
                self.assertLessEqual(x + w, 406.01, (s.id, id)); self.assertLessEqual(y + h, 776.01, (s.id, id))

    def test_device_geometry(self):
        """The measured anchors: shutter 64 ⌀ centred at (203, 691.7); mode bar labels 52.3 apart, selected at x 203; 4:3 viewfinder 406×541 at y 39.4."""
        photo = self.scenes[0]
        shutter = photo.node('shutter'); self.assertEqual((shutter['x'] + shutter['w'] / 2, shutter['y'] + shutter['h'] / 2, shutter['w']), (203, 691.7, 64))
        view = photo.node('viewfinder'); self.assertEqual((view['y'], view['w'], view['h']), (39.4, 406, 541))
        sel = photo.node('mode_photo'); self.assertAlmostEqual(sel['x'] + sel['w'] / 2, 203)
        nxt = photo.node('mode_video'); self.assertAlmostEqual(nxt['x'] - sel['x'], 52.3)
        video = self.scenes[4]; self.assertEqual(video.node('viewfinder')['h'], 721)

    def test_every_capture_scene_has_a_shutter_and_a_handle(self):
        for s in self.scenes:
            self.assertIn('shutter', s.controls, s.id)
            if s.n != 6: self.assertIn('treasure_handle', s.controls, s.id)


if __name__ == '__main__':
    unittest.main()
