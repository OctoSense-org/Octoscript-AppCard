#!/usr/bin/env python3
"""Author the Calendar flow from the storyboard spec: manifest, atlas intake, frozen
references, native contracts, reviewed semantic maps, service actions and the
compiled L0 scenes. The atlas is a deterministic Chromium render of the same
spec (scripts/design.py), so measured crops and authored geometry agree; intake
still validates and freezes the actual PNG bytes."""
from pathlib import Path
import copy, hashlib, json, sys
ROOT = Path(__file__).resolve().parents[1]
LAB = ROOT.parents[1] / 'lab'
sys.path[:0] = [str(ROOT / 'scripts'), str(LAB / 'image-to-appcard-flow'), str(LAB / 'image-to-appcard')]
import design
from design import storyboard, atlas_layout, metrics, FONT_SRC, SVG
from atlas import intake
from prepare import prepare
from catalogue import walk
from semantics import propose, write_brief, preflight
import compile as compiler
compiler.GALLERY = ROOT / 'artwork'

PROVIDER = 'Authored HTML storyboard rendered by headless Chromium through Playwright; no generative image model'
MODEL = 'chromium 153.0.8010.12 (playwright)'

def sha(p): return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def save(p, v): Path(p).parent.mkdir(parents=True, exist_ok=True); Path(p).write_text(json.dumps(v, ensure_ascii=False, indent=2) + '\n')
def col(v):
    """'rrggbb' or 'rrggbbaa' → ARGB u32, the contract's colour encoding."""
    v = v.lstrip('#'); alpha = v[6:8] if len(v) == 8 else 'ff'
    return int(alpha + v[:6], 16)

CARDS = [('calendar-synced-05', '5', 'calendar_card', 'calendar'), ('calendar-invite-06', '6', 'invite_card', 'calendar'),
         ('calendar-sync-status-09', '9', 'sync_card', 'sync'), ('calendar-removed-10', '10', 'calendar_card', 'calendar')]

def manifest(scenes):
    crops, size = atlas_layout(scenes)
    return dict(schema_version=1, id='calendar', artboard=[406, 776], locales=['en', 'cn'],
        generation=dict(atlas='source/atlas.png', prompt='source/prompt.txt', provider=PROVIDER, model=MODEL, requested_size=size, quality='high'),
        scenes=[dict(id=str(s.n), design_id=s.id, directory=f'cards/{s.id}', crop=crop, surface=s.surface, title=s.title['cn'], title_en=s.title['en']) for s, crop in zip(scenes, crops)],
        cards=[dict(id=id, scene=scene, root=root, owner=owner) for id, scene, root, owner in CARDS],
        artwork=dict(source_prefix='http://127.0.0.1:8170/ux-images/', root='artwork'),
        outputs=dict(intake='pipeline-output/intake', cards='pipeline-output/service-cards', bundle='wizard/card-bundle', runs='pipeline-output/runs'),
        browser_modules=['wizard/service.mjs', 'wizard/core.mjs', 'wizard/sync.mjs', 'wizard/copy.mjs'],
        checks={'service-test': [
            {'argv': ['{python}', '-m', 'unittest', 'discover', '-s', 'service', '-p', 'test_*.py'], 'cwd': '.'},
            {'argv': ['{python}', '-m', 'unittest', 'discover', '-s', 'server', '-p', 'test_*.py'], 'cwd': '.'},
            {'argv': ['{node}', '--test', 'wizard/service.test.mjs', 'wizard/core.test.mjs'], 'cwd': '.'},
            {'argv': ['{node}', '--test', 'wizard/sync.test.mjs'], 'cwd': '.'}]})

def author(scene, record):
    d = ROOT / record['directory']
    if (d / 'latest.json').exists() or (d / 'rounds').exists():
        raise ValueError('Retain captured native mappings; authoring cannot overwrite capture history')
    (d / 'assets').mkdir(exist_ok=True)
    refhash = sha(d / 'reference.png')
    tr = json.loads((d / 'atlas-provenance.json').read_text())['transform']
    # The reference is the 2× crop of a 2× render: the transform is the identity in logical units.
    scale = tr['uniform_scale'] / 2; ox, oy = [v / 2 for v in tr['offset_pixels']]
    def rect(x, y, w, h): return [ox + x * scale, oy + y * scale, w * scale, h * scale]
    root = dict(t='stack', id='page', x=0, y=0, w=406, h=776, variant='surface', bg=col(design.DESK if scene.surface == 'desktop' else design.PAGE), c=[])
    nodes = {'page': root}; assets = {}; textcopy = {}; measure = []
    def add(node, parent):
        nodes[parent].setdefault('c', []).append(node); nodes[node['id']] = node; return node
    for n in scene.nodes:
        kind = n['kind']; parent = n['parent']
        if kind == 'stack':
            node = dict(t='stack', id=n['id'], **dict(zip('xywh', rect(n['x'], n['y'], n['w'], n['h']))), c=[])
            if n['bg']: node.update(variant='surface', bg=col(n['bg']), radius=n['radius'] * scale)
            if n['border']: node.update(border=.7, bordercolor=col(n['border']))
            add(node, parent)
        elif kind == 'control':
            add(dict(t='button', id=n['id'], **dict(zip('xywh', rect(n['x'], n['y'], n['w'], n['h']))), enabled=int(bool(n['enabled']))), parent)
        elif kind == 'icon':
            id = n['id']; path = d / 'assets' / f'{id}.svg'; viewbox = '0 0 56 28' if n['icon'] == 'status' else '0 0 28 28'
            path.write_text(f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="{viewbox}" fill="none" stroke="#{n["color"][:6]}" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">{SVG[n["icon"]]}</svg>')
            add(dict(t='svg', id=id, **dict(zip('xywh', rect(n['x'], n['y'], n['w'], n['h']))), src=''), parent)
            assets[id] = dict(path=f'assets/{path.name}', sha256=sha(path), method='reference_svg', reference_sha256=refhash, fit='stretch', clip=True,
                              notes='Storyboard icon: the same vector paths the reference was rendered from; no text or raster')
        elif kind == 'text':
            id = n['id']; s = n['size']; cn = n['cn']; textcopy[id] = dict(cn=cn, en=n['en'])
            (x0, y0, x1, y1), adv, asc, desc = metrics(n['weight'], cn)
            # Where Chromium put the ink: the baseline centres the hhea line box in the CSS line box.
            baseline = n['y'] + (n['h'] - (asc - desc) * s) / 2 + asc * s
            start = n['x'] + {'left': 0, 'center': (n['w'] - adv * s) / 2, 'right': n['w'] - adv * s}[n['align']]
            ink = [start + x0 * s, baseline - y1 * s, (x1 - x0) * s, (y1 - y0) * s]
            ix, iy, iw, ih = rect(*ink); size = s * scale; inkh = (y1 - y0) * size
            logical_w = max(adv * size + 4, iw + 3)
            node = dict(t='text', id=id, text=cn, x=ix - x0 * size, y=iy + (ih - inkh) / 2 - 2, w=logical_w, h=inkh + 4, size=size, weight=n['weight'], tracking=0,
                        line_height=inkh + 4, alignx=0, variant='single_line', font_src=FONT_SRC[n['weight']], color=col(n['color']),
                        font_asc=y1 + 2 / size - asc, font_desc=(y1 * size + 2 - (inkh + 4)) / size - desc)
            add(node, parent)
            measure.append(dict(id=id, css_box=[n['x'], n['y'], n['w'], n['h']], ink_bounds=ink, copy=textcopy[id], method='Authored CSS box; ink from bundled Noto glyph metrics at the rendered size'))
    # Native label layers above surfaces, then bind actual child indexes for each control.
    for node in walk(root):
        if 'c' in node: node['c'].sort(key=lambda v: v['t'] == 'text')
    controls = {}
    for id, control in scene.controls.items():
        children = nodes[id]['c']
        bindings = {'control': [next(i for i, v in enumerate(children) if v['t'] == 'button')]}
        labels = [i for i, v in enumerate(children) if v['t'] == 'text' and v['id'] in control['text_ids']]
        if labels: bindings['label'] = [labels[0]]
        nodes[id]['kit'] = json.dumps(dict(widget='KitButton', bindings=bindings), separators=(',', ':'))
        controls[id] = dict(event=control['event'], text_ids=list(control['text_ids']), ocr_ids=[], source_bounds=control['source_bounds'], enabled=bool(control['enabled']))
    contract = dict(schema_version=1, id=scene.id, app='service', number=scene.n, title=scene.title['cn'], artboard=[406, 776], font_family='Noto Sans SC',
                    structure='Authored iOS-style calendar storyboard; app screens and desktop service cards', content_source='Fictional two-device household fixture (Alex · Phone, Sam · Desktop); every change is an explicit user action synced through the calendar service',
                    palette=dict(name='iOS', page='#ffffff', panel='#f2f2f7', ink='#1c1c1e', accent='#ff3b30'), graphics={}, tree=root)
    save(d / 'contract.json', contract)
    save(d / 'mapped.json', dict(schema_version=1, reference_sha256=refhash, tree=copy.deepcopy(root),
         changes=[dict(source='Storyboard spec rendered at 2× by Chromium; contract geometry is the spec, ink boxes from glyph metrics')],
         limits=['The reference is a deterministic render of the same spec, not an independent design source', 'No native capture or visual approval yet']))
    if (d / 'semantic-map.json').exists(): (d / 'semantic-map.json').unlink()
    semantic = propose(d); semantic.update(contract_sha256=sha(d / 'contract.json'), reference_sha256=refhash)
    for entry in semantic['elements']:
        if entry['id'] in assets:
            entry.update(role='icon', decision='reviewed', confidence=1, basis='Storyboard vector icon; native SVG paths without raster or text', asset=assets[entry['id']])
        elif entry['id'] in scene.cards:
            entry.update(role='card', decision='reviewed', basis='Explicit independently owned service surface, native Label/Button children')
    save(d / 'semantic-map.json', semantic); write_brief(d, semantic)
    save(d / 'service-actions.json', dict(frame_id=scene.n, source=scene.id, controls=controls))
    save(d / 'source-measurements.json', dict(atlas_sha256=sha(ROOT / 'source/atlas.png'), reference_sha256=refhash, source_crop=record['crop'], transform=tr,
         reviewed_source=True, text=measure, limits=['The storyboard is authored, so measurement is a consistency check of the render against the spec, not OCR of an unknown image', 'No native visual acceptance']))
    report = preflight(d)
    if report.get('errors') or report.get('status') == 'fail': raise ValueError(report)
    result = compiler.compile_page(d)
    print(json.dumps(dict(scene=scene.n, labels=len(textcopy), controls=len(controls), compiled=result), ensure_ascii=False), flush=True)
    return dict(text=textcopy, controls=controls, surface=scene.surface, title=scene.title)

if __name__ == '__main__':
    scenes = storyboard(); doc = manifest(scenes); path = ROOT / 'image-to-appcard-flow.json'
    if path.exists() and json.loads(path.read_text()) != doc: raise ValueError('Manifest changed; preserve the old intake and choose a new run')
    save(path, doc)
    intake(path, ROOT, ROOT / 'pipeline-output/intake')
    if not (ROOT / 'cards/calendar-01').exists(): prepare(path, ROOT, ROOT / 'pipeline-output/intake')
    bundle = {str(s.n): author(s, rec) for s, rec in zip(scenes, doc['scenes'])}
    (ROOT / 'wizard').mkdir(exist_ok=True)
    (ROOT / 'wizard/copy.mjs').write_text('// Generated by scripts/author_calendar.py from scripts/design.py — do not edit.\nexport const frames = ' + json.dumps(bundle, ensure_ascii=False, separators=(',', ':')) + ';\n')
    save(ROOT / 'source/native-measurement-summary.json', dict(frames=len(scenes), atlas_dimensions=doc['generation']['requested_size'], native_only=True,
         labels=sum(len(v['text']) for v in bundle.values()), controls=sum(len(v['controls']) for v in bundle.values()), capture='not run', visual_gate='not run'))
