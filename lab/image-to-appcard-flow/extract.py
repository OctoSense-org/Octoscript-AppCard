#!/usr/bin/env python3
"""Extract explicitly owned service surfaces as independently mountable native trees."""
import argparse
import copy
import json
from pathlib import Path
import shutil
import sys
import tempfile
from PIL import Image
from flow import local, read_manifest, sha

IMAGE = Path(__file__).resolve().parents[1] / 'image-to-appcard'
sys.path.insert(0, str(IMAGE))
from catalogue import walk
from compile import compile_page


def write(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n')


def extract(manifest, project, output):
    doc = read_manifest(manifest, project)
    if output.exists():
        raise FileExistsError('Retain reviewed standalone cards; use a new output directory')
    cards = doc.get('cards', [])
    if not cards:
        raise ValueError('Declare cards with id, scene, root, and owner; ownership is never guessed from text')
    import re
    for field in ('id', 'owner'):
        if any(not re.fullmatch(r'[a-z0-9]+(?:-[a-z0-9]+)*', c.get(field, '')) for c in cards):
            raise ValueError('Invalid standalone ' + field)
    if len({c['id'] for c in cards}) != len(cards):
        raise ValueError('Duplicate standalone card id')
    scenes = {s['id']: s for s in doc['scenes']}
    output.parent.mkdir(parents=True, exist_ok=True)
    stage = Path(tempfile.mkdtemp(prefix='.extract-', dir=output.parent))
    try:
        catalogue = []
        for card in cards:
            scene = scenes[card['scene']]
            src = local(project, scene['directory'])
            model = json.loads((src / 'mapped.json').read_text())
            contract = json.loads((src / 'contract.json').read_text())
            semantic = json.loads((src / 'semantic-map.json').read_text())
            nodes = {n['id']: n for n in walk(model['tree'])}
            tree = copy.deepcopy(nodes[card['root']])
            if tree['t'] != 'stack':
                raise ValueError('A service card must be a native composition root')
            x, y, w, h = [tree[k] for k in 'xywh']
            with Image.open(src / 'reference.png') as reference:
                sx, sy = reference.width / contract['artboard'][0], reference.height / contract['artboard'][1]
                px, py, right, bottom = round(x * sx), round(y * sy), round((x + w) * sx), round((y + h) * sy)
                if min(px, py) < 0 or right > reference.width or bottom > reference.height or min(w, h) <= 0:
                    raise ValueError('Standalone source region is outside the reference')
                dest = stage / card['owner'] / card['id']
                dest.mkdir(parents=True)
                reference.crop((px, py, right, bottom)).save(dest / 'reference.png')
            for node in walk(tree):
                node['x'] -= x
                node['y'] -= y
            ids = {n['id'] for n in walk(tree)}
            derived = {**contract, 'id': card['id'], 'artboard': [w, h], 'tree': tree, 'standalone_service_card': True}
            write(dest / 'contract.json', derived)
            write(dest / 'mapped.json', {'schema_version': 1, 'reference_sha256': sha(dest / 'reference.png'), 'tree': tree,
                  'changes': [{'source': scene['directory'] + '/mapped.json', 'source_sha256': sha(src / 'mapped.json'),
                               'operation': 'Extract native subtree and translate origin', 'source_root': card['root'], 'translation': [-x, -y]}]})
            semantic = {**semantic, 'contract_sha256': sha(dest / 'contract.json'), 'reference_sha256': sha(dest / 'reference.png'),
                        'elements': [copy.deepcopy(e) for e in semantic['elements'] if e['id'] in ids]}
            for entry in semantic['elements']:
                data = entry.get('data')
                if data:
                    source = local(src, data['path'])
                    if sha(source) != data['sha256']:
                        raise ValueError('Stale declared numeric data: ' + entry['id'])
                    target = local(dest, data['path'])
                    target.parent.mkdir(parents=True, exist_ok=True)
                    shutil.copy2(source, target)
                asset = entry.get('asset')
                if not asset:
                    continue
                source = local(src, asset['path'])
                if sha(source) != asset['sha256']:
                    raise ValueError('Stale declared artwork: ' + entry['id'])
                target = local(dest, asset['path'])
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(source, target)
                asset['reference_sha256'] = semantic['reference_sha256']
                if asset['method'] == 'source_crop':
                    asset['crop_pixels'][0] -= px
                    asset['crop_pixels'][1] -= py
            write(dest / 'semantic-map.json', semantic)
            shutil.copy2(src / 'image-prompt.md', dest / 'image-prompt.md')
            generation = json.loads((src / 'generation.json').read_text())
            write(dest / 'generation.json', {'provider': generation['provider'], 'model': generation.get('model', 'not recorded'),
                  'derived_from': scene['directory'] + '/reference.png', 'derived_from_sha256': sha(src / 'reference.png'),
                  'source_region_pixels': [px, py, right-px, bottom-py], 'image_sha256': sha(dest / 'reference.png'),
                  'original_prompt_sha256': sha(dest / 'image-prompt.md'), 'new_image_generation': False})
            actions = json.loads((src / 'service-actions.json').read_text())
            actions['controls'] = {k: v for k, v in actions['controls'].items() if k in ids}
            actions['owner'] = card['owner']
            write(dest / 'service-actions.json', actions)
            result = compile_page(dest)
            catalogue.append({**card, 'folder': str(dest.relative_to(stage)), 'artboard': [w, h], **result})
        write(stage / 'catalogue.json', {'schema_version': 1, 'kind': 'native-service-appcards', 'cards': catalogue,
              'validation': 'Compiled native subtrees; independent Studio input/layout and visual review remain required'})
        stage.rename(output)
        return {'cards': len(catalogue), 'output': str(output)}
    finally:
        if stage.exists():
            shutil.rmtree(stage)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('project', 'manifest', 'output'):
        parser.add_argument('--' + name, required=True, type=Path)
    args = parser.parse_args()
    print(json.dumps(extract(args.manifest.resolve(), args.project.resolve(), args.output.resolve())))
