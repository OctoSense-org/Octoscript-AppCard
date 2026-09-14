#!/usr/bin/env python3
"""Run a reviewed image-to-service-card project without hiding unrun gates."""
import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import uuid

HERE = Path(__file__).resolve().parent
WORKSPACE = HERE.parents[1]
STAGES = ('intake', 'prepare', 'observe', 'measure', 'map', 'semantic', 'compile', 'capture', 'gate', 'extract', 'bundle',
          'service-test', 'wasm', 'integrate', 'web-test', 'hosted-test')
DEFAULT_STAGES = 'intake,semantic,compile,bundle,service-test'


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def local(root, name):
    if not isinstance(name, str) or not name or Path(name).is_absolute():
        raise ValueError('Project paths must be nonempty relative paths')
    result = (root / name).resolve()
    if not result.is_relative_to(root.resolve()):
        raise ValueError('Path escapes project: ' + name)
    return result


def read_manifest(path, project):
    doc = json.loads(path.read_text())
    if doc.get('schema_version') != 1:
        raise ValueError('Expected schema_version: 1')
    if not re.fullmatch(r'[a-z0-9]+(?:-[a-z0-9]+)*', doc.get('id', '')):
        raise ValueError('Invalid flow id')
    if doc.get('artboard') != [406, 776]:
        raise ValueError('Current native image adapter supports artboard [406, 776]')
    scenes = doc.get('scenes', [])
    if not 8 <= len(scenes) <= 12:
        raise ValueError('Author 8–12 scenes from one atlas')
    for key in ('id', 'design_id', 'directory'):
        values = [s.get(key) for s in scenes]
        if any(not isinstance(v, str) or not v for v in values) or len(set(values)) != len(values):
            raise ValueError('Scenes require unique string ' + key)
    for scene in scenes:
        if not re.fullmatch(r'[a-z0-9]+(?:-[a-z0-9]+)*', scene['design_id']):
            raise ValueError('Invalid design_id')
        if scene.get('surface') not in ('app', 'desktop'):
            raise ValueError('Scene surface must be app or desktop')
        local(project, scene['directory'])
        crop = scene.get('crop')
        if not isinstance(crop, list) or len(crop) != 4 or any(type(v) is not int for v in crop):
            raise ValueError('Each scene requires an integer [x,y,w,h] atlas crop')
    generation = doc['generation']
    for key in ('atlas', 'prompt'):
        local(project, generation[key])
    if not generation.get('provider', '').strip():
        raise ValueError('Declare the actual generator; model may be omitted if unknown')
    if doc.get('locales') != ['en', 'cn']:
        raise ValueError('Declare locales [en, cn] and validate both')
    for name in doc.get('outputs', {}).values():
        local(project, name)
    for name in doc.get('browser_modules', []):
        local(project, name)
    local(project, doc['artwork']['root'])
    return doc


def fingerprint(doc, manifest, project):
    paths = [manifest, local(project, doc['generation']['atlas']), local(project, doc['generation']['prompt'])]
    for scene in doc['scenes']:
        directory = local(project, scene['directory'])
        paths += [directory / name for name in ('contract.json', 'mapped.json', 'semantic-map.json', 'service-actions.json')]
    paths += [local(project, name) for name in doc.get('browser_modules', [])]
    files = {str(p.relative_to(project)) if p.is_relative_to(project) else str(p): sha(p) if p.is_file() else None for p in paths}
    return {'sha256': hashlib.sha256(json.dumps(files, sort_keys=True).encode()).hexdigest(), 'files': files}


def paths_for(doc, project):
    defaults = {'intake': 'pipeline-output/intake', 'bundle': 'wizard/card-bundle',
                'wasm': 'wizard/wasm-dist', 'cards': 'pipeline-output/service-cards', 'runs': 'pipeline-output/runs'}
    return {k: local(project, doc.get('outputs', {}).get(k, v)) for k, v in defaults.items()}


def configured_commands(doc, stage, project, values):
    entries = doc.get('checks', {}).get(stage)
    if not isinstance(entries, list) or not entries:
        raise ValueError(f'{stage} needs explicit checks.{stage} command arrays in the manifest')
    result = []
    for entry in entries:
        argv = entry.get('argv')
        if not isinstance(argv, list) or not argv or any(not isinstance(v, str) or not v for v in argv):
            raise ValueError('Check argv must be a nonempty array of strings; shell strings are unsupported')
        # Only these complete argument placeholders are substituted. No shell is used.
        args = [values.get(v, v) for v in argv]
        cwd = entry.get('cwd', '.')
        if cwd == '{website}':
            if not values.get('{website}'):
                raise ValueError(stage + ' requires --website')
            cwd = values['{website}']
        else:
            cwd = str(local(project, cwd))
        if any(v in ('{website}', '{project}', '{workspace}', '{node}', '{python}') and not values.get(v) for v in argv):
            raise ValueError('Missing command placeholder value')
        result.append({'argv': args, 'cwd': cwd})
    return result


def commands_for(stage, doc, manifest, project, website, node, python, launch=False):
    out = paths_for(doc, project)
    common = ['--project', str(project), '--manifest', str(manifest)]
    values = {'{node}': node, '{python}': python, '{project}': str(project),
              '{workspace}': str(WORKSPACE), '{website}': str(website) if website else ''}
    if stage in ('service-test', 'web-test', 'hosted-test'):
        return configured_commands(doc, stage, project, values)
    if stage == 'prepare':
        argv = [python, str(HERE / 'prepare.py'), *common, '--intake', str(out['intake'])]
    elif stage in ('observe', 'measure', 'map', 'semantic', 'compile', 'capture', 'gate'):
        argv = [python, str(HERE / 'native.py'), *common, '--stage', stage]
        if launch and stage == 'capture':
            argv.append('--launch')
    elif stage in ('intake', 'bundle', 'extract'):
        script = {'intake': 'atlas.py', 'bundle': 'bundle.py', 'extract': 'extract.py'}[stage]
        argv = [python, str(HERE / script), *common, '--output', str(out['cards' if stage == 'extract' else stage])]
    elif stage == 'wasm':
        argv = [python, str(HERE / 'wasm/build.py'), '--workspace', str(WORKSPACE),
                '--project', str(project), '--output', str(out['wasm']), '--replace']
    elif stage == 'integrate':
        if website is None:
            raise ValueError('integrate requires --website pointing to the OctoSense Astro checkout')
        script = website / 'scripts/sync-service-wizard.mjs'
        source = out['bundle'].parent
        if out['bundle'].name != 'card-bundle' or out['wasm'] != source / 'wasm-dist':
            raise ValueError('Website sync expects sibling card-bundle and wasm-dist folders')
        argv = [node, str(script), str(source)]
    else:
        raise ValueError('Unknown stage: ' + stage)
    return [{'argv': argv, 'cwd': str(project)}]


def run(stages, doc, manifest, project, website, node, python, launch=False):
    # Resolve every command before any mutation; missing checks cannot create a partial plan.
    plan = {s: commands_for(s, doc, manifest, project, website, node, python, launch) for s in stages}
    directory = paths_for(doc, project)['runs'] / (datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%SZ-') + uuid.uuid4().hex[:8])
    directory.mkdir(parents=True)
    receipt = {'schema_version': 1, 'flow': doc['id'], 'status': 'running',
               'scope': 'Only explicitly executed stages; no automatic visual or production acceptance',
               'inputs': fingerprint(doc, manifest, project), 'stages': [],
               'not_run': [s for s in STAGES if s not in stages]}
    record = directory / 'run.json'
    def save():
        record.write_text(json.dumps(receipt, indent=2) + '\n')
    save()
    try:
        for stage in stages:
            entry = {'stage': stage, 'status': 'running', 'commands': plan[stage]}
            receipt['stages'].append(entry)
            save()
            for index, command in enumerate(plan[stage]):
                log = directory / f'{stage}-{index}.log'
                with log.open('w') as stream:
                    result = subprocess.run(command['argv'], cwd=command['cwd'], stdout=stream, stderr=subprocess.STDOUT)
                entry.setdefault('logs', []).append({'path': log.name, 'sha256': sha(log), 'exit_code': result.returncode})
                print(json.dumps({'stage': stage, 'exit_code': result.returncode, 'log': str(log)}), flush=True)
                if result.returncode:
                    entry['status'] = 'failed'
                    raise RuntimeError(f'{stage} failed; see {log}')
            entry['status'] = 'passed'
            save()
        receipt['status'] = 'completed'
    except (Exception, KeyboardInterrupt) as error:
        receipt['status'] = 'interrupted' if isinstance(error, KeyboardInterrupt) else 'failed'
        receipt['error'] = str(error)
        if receipt['stages'] and receipt['stages'][-1]['status'] == 'running':
            receipt['stages'][-1]['status'] = receipt['status']
        raise
    finally:
        receipt['not_run'] = [s for s in STAGES if s not in [e['stage'] for e in receipt['stages']]]
        receipt['finished_at'] = datetime.now(timezone.utc).isoformat()
        save()
        print(json.dumps({'receipt': str(record), 'status': receipt['status']}), flush=True)
    return receipt


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('command', choices=('plan', 'run', 'status'))
    parser.add_argument('--manifest', required=True, type=Path)
    parser.add_argument('--project', required=True, type=Path)
    parser.add_argument('--website', type=Path)
    parser.add_argument('--stages', default=DEFAULT_STAGES)
    parser.add_argument('--node', default=os.environ.get('SERVICE_NODE', shutil.which('node') or 'node'))
    parser.add_argument('--python', default=sys.executable)
    parser.add_argument('--launch', action='store_true', help='Start a fresh Studio RunItem for capture')
    args = parser.parse_args()
    project, manifest = args.project.resolve(), args.manifest.resolve()
    website = args.website.resolve() if args.website else None
    try:
        doc = read_manifest(manifest, project)
        stages = args.stages.split(',')
        if len(set(stages)) != len(stages) or any(s not in STAGES for s in stages):
            raise ValueError('Choose unique stages from: ' + ','.join(STAGES))
        if args.command == 'plan':
            print(json.dumps({s: commands_for(s, doc, manifest, project, website, args.node, args.python, args.launch) for s in stages}, indent=2))
        elif args.command == 'run':
            run(stages, doc, manifest, project, website, args.node, args.python, args.launch)
        else:
            runs = sorted(paths_for(doc, project)['runs'].glob('*/run.json'), key=lambda p: p.stat().st_mtime_ns)
            latest = json.loads(runs[-1].read_text()) if runs else None
            print(json.dumps({'latest': str(runs[-1]) if runs else None, 'receipt': latest,
                              'current_inputs_match': latest['inputs'] == fingerprint(doc, manifest, project) if latest else False}, indent=2))
    except (OSError, ValueError, KeyError, RuntimeError) as error:
        parser.exit(1, str(error) + '\n')


if __name__ == '__main__':
    main()
