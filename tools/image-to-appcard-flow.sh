#!/usr/bin/env bash
# One generated atlas → reviewed native components → service flow → WASM.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
exec bash "$ROOT/tools/beauty-pipeline.sh" --image-to-appcard-flow "$@"
