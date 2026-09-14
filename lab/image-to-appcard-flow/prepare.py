#!/usr/bin/env python3
"""Install frozen atlas references for new designs; never overwrite reviewed models."""
import argparse
import json
from pathlib import Path
import shutil
import tempfile
from flow import local, read_manifest, sha
from atlas import intake as validate_intake


def prepare(manifest, project, intake):
    doc = read_manifest(manifest, project)
    # Same-input atlas replay validates every frozen file and does not rewrite it.
    receipt = validate_intake(manifest, project, intake)
    records = {scene['design_id']: scene for scene in receipt['scenes']}
    for scene in doc['scenes']:
        target = local(project, scene['directory'])
        if target.exists():
            raise FileExistsError('Design already exists; retain its mapping/reference: ' + scene['directory'])
        if target.is_relative_to(intake) or intake.is_relative_to(target):
            raise ValueError('Design directory overlaps immutable atlas intake')
    prepared = []
    for scene in doc['scenes']:
        target = local(project, scene['directory'])
        target.parent.mkdir(parents=True, exist_ok=True)
        stage = Path(tempfile.mkdtemp(prefix='.prepare-', dir=target.parent))
        record = records[scene['design_id']]
        try:
            shutil.copy2(local(intake, record['reference_image']), stage / 'reference.png')
            shutil.copy2(local(intake, receipt['originals']['prompt']), stage / 'image-prompt.md')
            if (sha(stage / 'reference.png') != record['reference_sha256'] or
                    sha(stage / 'image-prompt.md') != receipt['inputs']['prompt_sha256']):
                raise ValueError('Frozen intake changed during reference preparation')
            generation = {**receipt['generation'], 'image_sha256': record['reference_sha256'],
                          'atlas_sha256': receipt['inputs']['atlas_sha256'],
                          'prompt_sha256': receipt['inputs']['prompt_sha256'],
                          'source_manifest_sha256': receipt['inputs']['manifest_sha256'],
                          'source_crop': record['crop'], 'normalization': record['transform'],
                          'note': 'Derived reference from one atlas; no independent screen generation'}
            (stage / 'generation.json').write_text(json.dumps(generation, indent=2) + '\n')
            (stage / 'atlas-provenance.json').write_text(json.dumps(record, indent=2) + '\n')
            (stage / 'AUTHORING.md').write_text(
                '# Native mapping pending\n\nAuthor contract.json with this design ID and [406,776] artboard,\n'
                'explicit text, fonts, hierarchy, data and controls. Run observe,measure,map,\n'
                'then review the semantic map and add service-actions.json before compilation.\n'
                'These references do not establish visual or native acceptance.\n')
            if target.exists():
                raise FileExistsError('Design appeared during preparation: ' + str(target))
            stage.rename(target)
            prepared.append(str(target))
        finally:
            if stage.exists():
                shutil.rmtree(stage)
    return {'prepared': prepared, 'mapping_status': 'requires_authored_contract_and_semantic_review'}


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('manifest', 'project', 'intake'):
        parser.add_argument('--' + name, required=True, type=Path)
    args = parser.parse_args()
    print(json.dumps(prepare(args.manifest.resolve(), args.project.resolve(), args.intake.resolve())))
