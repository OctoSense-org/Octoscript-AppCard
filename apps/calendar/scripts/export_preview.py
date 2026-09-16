#!/usr/bin/env python3
"""Export the storyboard scenes as tagged HTML fragments for the browser preview
(wizard/preview). The preview is an HTML rendering of the same storyboard the
atlas came from — it exercises the real session, sync client and server, but
it is not the native Makepad renderer and is no substitute for the WASM host."""
from pathlib import Path
import json, sys
ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'scripts'))
import design

scenes = design.storyboard()
out = {'schema_version': 1, 'artboard': list(design.ARTBOARD), 'note': 'HTML preview of the storyboard; not the native renderer',
       'scenes': {str(s.n): {'id': s.id, 'surface': s.surface, 'title': s.title, 'html': design.scene_html(s, interactive=True)} for s in scenes}}
target = ROOT / 'wizard/preview/scenes.json'
target.parent.mkdir(parents=True, exist_ok=True)
target.write_text(json.dumps(out, ensure_ascii=False) + '\n')
print(json.dumps({'scenes': len(scenes), 'bytes': target.stat().st_size, 'output': str(target)}))
