"""Publish complete directories with no gap in the visible output path."""
from __future__ import annotations

import ctypes
import os
from pathlib import Path
import sys


def exchange_directories(staged: Path, target: Path) -> None:
    libc = ctypes.CDLL(None, use_errno=True)
    if sys.platform == "darwin" and hasattr(libc, "renamex_np"):
        call = libc.renamex_np
        call.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_uint]
        result = call(os.fsencode(staged), os.fsencode(target), 2)  # RENAME_SWAP
    elif sys.platform.startswith("linux") and hasattr(libc, "renameat2"):
        call = libc.renameat2
        call.argtypes = [ctypes.c_int, ctypes.c_char_p, ctypes.c_int, ctypes.c_char_p, ctypes.c_uint]
        result = call(-100, os.fsencode(staged), -100, os.fsencode(target), 2)  # RENAME_EXCHANGE
    else:
        raise RuntimeError("Atomic directory exchange is unavailable; previous output retained")
    if result:
        code = ctypes.get_errno()
        raise OSError(code, os.strerror(code), str(target))


def publish_package(staged: Path, target: Path, previous: Path | None) -> None:
    if staged.parent != target.parent or (previous and previous.parent != target.parent):
        raise RuntimeError("Atomic package publication requires sibling directories")
    if target.is_symlink() or (target.exists() and not target.is_dir()):
        raise RuntimeError("Output must be a directory, not a symlink or regular file")
    if not target.exists():
        os.replace(staged, target)
        return
    if previous is None or previous.exists():
        raise RuntimeError("An unused previous-package path is required for replacement")
    exchange_directories(staged, target)
    # Output now contains the verified new package; the former output remains
    # complete at staged even if the following archival rename is interrupted.
    os.replace(staged, previous)
