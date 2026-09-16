"""Repository-relative pipeline and isolated native dependency locations."""
from pathlib import Path
import os
import sys

ROOT = Path(__file__).resolve().parents[1]
RUNTIME = Path(os.environ.get('OCTOS_MAIL_RUNTIME', ROOT / 'runtime')).resolve()
PIPELINE = Path(os.environ.get('OCTOS_APPCARD_PIPELINE', ROOT.parents[1])).resolve()
IMAGE = PIPELINE / 'lab/image-to-appcard'
sys.path.insert(0, str(PIPELINE / 'lab'))
from core.native_paths import WORKSPACE
NATIVE_ROOT = Path(os.environ.get('OCTOS_MAIL_NATIVE_ROOT', WORKSPACE)).resolve()
os.environ.setdefault('OCTOS_APPCARD_NATIVE_ROOT', str(NATIVE_ROOT))
sys.path.insert(0, str(IMAGE))
sys.path.insert(0, str(ROOT / 'service'))
