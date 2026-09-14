#!/usr/bin/env bash
# Entry point for the beauty card pipeline (design kit -> judged live cards).
# Setup for both branches: lab/core/REPRODUCE.md.
#   tools/beauty-pipeline.sh --kit atro-native-all --stages beauty
#   tools/beauty-pipeline.sh --kit <name> --stages splash-makepad
#   tools/beauty-pipeline.sh --kit <name> --stages audit
#   tools/beauty-pipeline.sh --kit <name> --stages report
set -euo pipefail
if [[ "${1:-}" == "--image-to-appcard-flow" ]]; then
    shift
    ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
    PYTHON="${BEAUTY_PYTHON:-$ROOT/lab/image-to-appcard/.venv/bin/python}"
    if [[ ! -x "$PYTHON" ]]; then PYTHON=python3; fi
    exec "$PYTHON" "$ROOT/lab/image-to-appcard-flow/flow.py" "$@"
fi
if [[ "${1:-}" == "--repair" ]]; then
    shift
    ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
    PYTHON="${BEAUTY_PYTHON:-$ROOT/lab/sketch-to-appcard/.venv/bin/python}"
    if [[ ! -x "$PYTHON" ]]; then PYTHON=python3; fi
    exec "$PYTHON" "$ROOT/lab/core/repair.py" "$@"
fi
if [[ "${1:-}" == "--ux-image" ]]; then
    shift
    ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
    PYTHON="${BEAUTY_PYTHON:-$ROOT/lab/image-to-appcard/.venv/bin/python}"
    if [[ ! -x "$PYTHON" ]]; then PYTHON=python3; fi
    exec "$PYTHON" "$ROOT/lab/image-to-appcard/run.py" "$@"
fi
cd "$(dirname "${BASH_SOURCE[0]}")/../lab/sketch-to-appcard"
PYTHON="${BEAUTY_PYTHON:-$PWD/.venv/bin/python}"
if [[ ! -x "$PYTHON" ]]; then PYTHON=python3; fi
exec "$PYTHON" run_kit.py "$@"
