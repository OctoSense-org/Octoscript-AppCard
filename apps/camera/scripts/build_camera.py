#!/usr/bin/env python3
"""Compile, extract and export the authored camera flow; capture is a separate explicit gate."""
from pathlib import Path
import json, sys
ROOT = Path(__file__).resolve().parents[1]; LAB = ROOT.parents[1] / 'lab'
sys.path[:0] = [str(LAB / 'image-to-appcard-flow'), str(LAB / 'image-to-appcard')]
import compile as compiler
compiler.GALLERY = ROOT / 'artwork'
from semantics import preflight
from extract import extract
from bundle import export_bundle
manifest = ROOT / 'image-to-appcard-flow.json'
for scene in json.loads(manifest.read_text())['scenes']:
    d = ROOT / scene['directory']; preflight(d); compiler.compile_page(d)
output = ROOT / 'pipeline-output/service-cards'
if not output.exists(): print(json.dumps(extract(manifest, ROOT, output)))
print(json.dumps(export_bundle(ROOT, manifest, ROOT / 'wizard/card-bundle')))
