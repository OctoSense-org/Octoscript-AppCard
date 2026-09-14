import ctypes
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

from publish import publish_package


class PublicationTests(unittest.TestCase):
    def fixture(self, root):
        staged, target, previous = (Path(root) / name for name in ("staged", "published", "previous"))
        for directory, value in ((staged, "new"), (target, "old")):
            directory.mkdir()
            (directory / "payload").write_text(value)
        return staged, target, previous

    def test_exchange_failure_preserves_published_and_staged_packages(self):
        with tempfile.TemporaryDirectory() as root:
            staged, target, previous = self.fixture(root)
            with patch("publish.exchange_directories", side_effect=OSError("exchange rejected")):
                with self.assertRaises(OSError):
                    publish_package(staged, target, previous)
            self.assertEqual((target / "payload").read_text(), "old")
            self.assertEqual((staged / "payload").read_text(), "new")
            self.assertFalse(previous.exists())

    def test_real_exchange_retains_previous_package(self):
        libc = ctypes.CDLL(None)
        available = (sys.platform == "darwin" and hasattr(libc, "renamex_np")) or (
            sys.platform.startswith("linux") and hasattr(libc, "renameat2"))
        if not available:
            self.skipTest("OS does not expose atomic directory exchange")
        with tempfile.TemporaryDirectory() as root:
            staged, target, previous = self.fixture(root)
            publish_package(staged, target, previous)
            self.assertEqual((target / "payload").read_text(), "new")
            self.assertEqual((previous / "payload").read_text(), "old")
            self.assertFalse(staged.exists())

    def test_symlink_output_does_not_replace_another_directory(self):
        with tempfile.TemporaryDirectory() as root:
            staged, target, previous = self.fixture(root)
            link = Path(root) / "link"
            link.symlink_to(target, target_is_directory=True)
            with self.assertRaises(RuntimeError):
                publish_package(staged, link, previous)
            self.assertEqual((target / "payload").read_text(), "old")


if __name__ == "__main__":
    unittest.main()
