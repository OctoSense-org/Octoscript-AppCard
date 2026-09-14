#!/usr/bin/env python3
"""Immutable intake of 8–12 scenes from one caller-supplied image atlas.

This module crops and normalizes an existing image. It never generates an
image, classifies a UI, produces Makepad widgets, or grants visual approval.
Manifest atlas/prompt paths are relative to --project, not the manifest file.
"""

import argparse
import copy
from datetime import datetime, timezone
import hashlib
from io import BytesIO
import json
import os
from pathlib import Path
import re
import shutil
import sys
import tempfile

from PIL import Image

VERSION = 1
PIXEL_RATIO = 2
MATTE = (240, 245, 242)
IDENTIFIER = re.compile(r'[A-Za-z0-9][A-Za-z0-9_-]*\Z')


def digest(data):
    return hashlib.sha256(data).hexdigest()


def encoded(document):
    return (json.dumps(document, ensure_ascii=False, indent=2) + '\n').encode('utf-8')


def integer_pair(value, label):
    if not isinstance(value, list) or len(value) != 2 or any(type(v) is not int or v <= 0 for v in value):
        raise ValueError(label + ' must contain two positive integers')
    return value


def identifier(value, label):
    if not isinstance(value, str) or not IDENTIFIER.fullmatch(value):
        raise ValueError(label + ' must be a nonempty ASCII identifier using letters, numbers, _ or -')
    return value


def source_path(project, relative, label):
    if not isinstance(relative, str) or not relative or Path(relative).is_absolute() or '..' in Path(relative).parts:
        raise ValueError(label + ' must be a relative path within the project')
    path = (project / relative).resolve(strict=True)
    if not path.is_relative_to(project) or not path.is_file():
        raise ValueError(label + ' must resolve to a file within the project')
    return path


def validate(manifest_path, project):
    """Read all inputs and validate every scene before touching the output."""
    project = Path(project).resolve(strict=True)
    if not project.is_dir():
        raise ValueError('project must be a directory')
    manifest_path = Path(manifest_path).resolve(strict=True)
    raw_manifest = manifest_path.read_bytes()
    try:
        manifest = json.loads(raw_manifest)
    except (ValueError, UnicodeError) as error:
        raise ValueError('manifest must be valid UTF-8 JSON') from error
    if not isinstance(manifest, dict) or type(manifest.get('schema_version')) is not int or manifest['schema_version'] != VERSION:
        raise ValueError('manifest requires schema_version 1')
    identifier(manifest.get('id'), 'manifest id')
    integer_pair(manifest.get('artboard'), 'artboard')
    generation = manifest.get('generation')
    if not isinstance(generation, dict):
        raise ValueError('generation must describe one existing atlas and its exact prompt')
    integer_pair(generation.get('requested_size'), 'generation.requested_size')
    if generation.get('quality') != 'high':
        raise ValueError('generation.quality must be high')
    if not isinstance(generation.get('provider'), str) or not generation['provider'].strip():
        raise ValueError('generation.provider is required as caller-declared provenance')
    if 'model' in generation and (not isinstance(generation['model'], str) or not generation['model'].strip()):
        raise ValueError('generation.model must be a nonempty string when supplied')
    atlas_path = source_path(project, generation.get('atlas'), 'generation.atlas')
    prompt_path = source_path(project, generation.get('prompt'), 'generation.prompt')
    atlas_bytes, prompt_bytes = atlas_path.read_bytes(), prompt_path.read_bytes()
    try:
        if not prompt_bytes.decode('utf-8-sig').strip():
            raise ValueError('generation.prompt must not be empty')
    except UnicodeError as error:
        raise ValueError('generation.prompt must be UTF-8 text') from error
    try:
        atlas = Image.open(BytesIO(atlas_bytes))
        if getattr(atlas, 'n_frames', 1) != 1:
            raise ValueError('the input must be one static atlas, not an animated or multi-page image')
        atlas.load()
    except (OSError, Image.DecompressionBombError) as error:
        raise ValueError('generation.atlas is not a readable static raster image') from error
    if atlas.mode not in {'RGB', 'RGBA', 'L', 'LA', 'P', 'I;16'}:
        raise ValueError('atlas mode cannot be preserved in PNG crops: ' + atlas.mode)
    # A rotated EXIF presentation must not silently change the crop coordinate
    # system. Coordinates always address the stored pixel matrix.
    orientation = atlas.getexif().get(274, 1)
    scenes = manifest.get('scenes')
    if not isinstance(scenes, list) or not 8 <= len(scenes) <= 12:
        raise ValueError('one atlas intake requires 8–12 scenes')
    ids, designs, rectangles = set(), set(), []
    width, height = atlas.size
    for index, scene in enumerate(scenes):
        if not isinstance(scene, dict):
            raise ValueError(f'scenes[{index}] must be an object')
        ident = identifier(scene.get('id'), f'scenes[{index}].id')
        design_id = identifier(scene.get('design_id'), f'scenes[{index}].design_id')
        if ident in ids or design_id in designs:
            raise ValueError('scene ids and design_ids must each be unique')
        ids.add(ident)
        designs.add(design_id)
        if scene.get('surface') not in {'app', 'desktop'}:
            raise ValueError(f'{design_id}: surface must be app or desktop')
        rect = scene.get('crop')
        if not isinstance(rect, list) or len(rect) != 4 or any(type(v) is not int for v in rect):
            raise ValueError(f'{design_id}: crop must contain four integer pixel coordinates')
        x, y, w, h = rect
        if min(x, y) < 0 or min(w, h) <= 0 or x + w > width or y + h > height:
            raise ValueError(f'{design_id}: crop is outside the actual {width}×{height} atlas or has no area')
        for other_id, (a, b, c, d) in rectangles:
            if max(x, a) < min(x + w, a + c) and max(y, b) < min(y + h, b + d):
                raise ValueError(f'{design_id}: crop overlaps {other_id}')
        rectangles.append((design_id, rect))
    hashes = {'manifest_sha256': digest(raw_manifest), 'atlas_sha256': digest(atlas_bytes),
              'prompt_sha256': digest(prompt_bytes)}
    fingerprint = digest(json.dumps({'version': VERSION, **hashes}, sort_keys=True).encode())
    return {'project': project, 'manifest_path': manifest_path, 'manifest': manifest,
            'manifest_bytes': raw_manifest, 'atlas_bytes': atlas_bytes, 'prompt_bytes': prompt_bytes,
            'atlas': atlas, 'atlas_format': atlas.format, 'atlas_mode': atlas.mode,
            'exif_orientation': orientation, 'hashes': hashes, 'fingerprint': fingerprint}


def transformation(crop, artboard):
    x, y, width, height = crop
    rw, rh = [v * PIXEL_RATIO for v in artboard]
    scale = min(rw / width, rh / height)
    ox, oy = (rw - width * scale) / 2, (rh - height * scale) / 2
    inverse = 1 / scale
    return {
        'coordinate_convention': 'continuous top-left pixel-boundary coordinates; affine arrays are [a,b,c,d,e,f]',
        'method': 'uniform affine contain', 'resampler': 'Pillow BICUBIC',
        'source_crop_pixels': crop, 'source_crop_size': [width, height],
        'artboard': artboard, 'pixel_ratio': PIXEL_RATIO, 'reference_size': [rw, rh],
        'uniform_scale': scale, 'offset_pixels': [ox, oy],
        'content_bounds_pixels': [ox, oy, width * scale, height * scale],
        'crop_to_reference_pixels': [scale, 0, ox, 0, scale, oy],
        'reference_pixels_to_crop': [inverse, 0, -ox * inverse, 0, inverse, -oy * inverse],
        'atlas_to_reference_pixels': [scale, 0, ox - x * scale, 0, scale, oy - y * scale],
        'reference_pixels_to_atlas': [inverse, 0, x - ox * inverse, 0, inverse, y - oy * inverse],
        'atlas_to_logical': [scale / PIXEL_RATIO, 0, (ox - x * scale) / PIXEL_RATIO,
                             0, scale / PIXEL_RATIO, (oy - y * scale) / PIXEL_RATIO],
        'logical_to_atlas': [PIXEL_RATIO * inverse, 0, x - ox * inverse,
                             0, PIXEL_RATIO * inverse, y - oy * inverse],
        'matte_rgb': list(MATTE), 'stretched': False, 'reference_resampled': True,
    }


def fit_reference(crop, transform):
    """Use one affine scale on both axes; avoid integer-resize aspect drift."""
    source = crop.convert('RGBA')
    fitted = source.transform(tuple(transform['reference_size']), Image.Transform.AFFINE,
                              tuple(transform['reference_pixels_to_crop']),
                              resample=Image.Resampling.BICUBIC, fillcolor=(*MATTE, 255))
    matte = Image.new('RGBA', fitted.size, (*MATTE, 255))
    return Image.alpha_composite(matte, fitted).convert('RGB')


def existing_intake(output, fingerprint):
    """Verify immutable output before returning a no-write idempotent replay."""
    receipt = output / 'intake.json'
    receipt_hash = output / 'intake.sha256'
    if not receipt.is_file() or not receipt_hash.is_file():
        raise ValueError('output already exists without an intact intake; use a new output directory')
    raw = receipt.read_bytes()
    if digest(raw) != receipt_hash.read_text().strip():
        raise ValueError('existing intake receipt was modified; preserve it and use a new output directory')
    document = json.loads(raw)
    if document.get('input_fingerprint') != fingerprint:
        raise ValueError('input changed; preserve the previous intake and use a new output directory')
    for relative, expected in document.get('files', {}).items():
        path = output / relative
        resolved = path.resolve()
        if not resolved.is_relative_to(output.resolve()) or path.is_symlink() or not path.is_file() or digest(path.read_bytes()) != expected:
            raise ValueError('existing intake artifact was modified or is missing: ' + relative)
    return {**document, 'replayed': True}


def write_intake(staging, validated):
    manifest, atlas = validated['manifest'], validated['atlas']
    originals = staging / 'original'
    originals.mkdir()
    suffix = {'JPEG': 'jpg', 'TIFF': 'tiff'}.get(validated['atlas_format'], (validated['atlas_format'] or 'image').lower())
    atlas_name = 'original/atlas.' + suffix
    (staging / atlas_name).write_bytes(validated['atlas_bytes'])
    (originals / 'prompt.txt').write_bytes(validated['prompt_bytes'])
    (originals / 'manifest.json').write_bytes(validated['manifest_bytes'])
    actual_size = list(atlas.size)
    generation = {key: copy.deepcopy(value) for key, value in manifest['generation'].items()
                  if key in {'provider', 'model', 'requested_size', 'quality'}}
    generation.update(actual_size=actual_size,
                      requested_size_matches_actual=generation['requested_size'] == actual_size,
                      atlas_count=1, performed_by_intake=False,
                      provenance='Provider/model/requested settings are caller declarations; dimensions and hashes are measured from the supplied bytes')
    rows = []
    for scene in manifest['scenes']:
        directory = staging / 'scenes' / scene['design_id']
        directory.mkdir(parents=True)
        x, y, w, h = scene['crop']
        cropped = atlas.crop((x, y, x + w, y + h))
        # PNG is lossless; retain source pixel mode and values. Reference
        # compositing/normalization is a separate, explicitly derived image.
        cropped.save(directory / 'crop.png', format='PNG')
        transform = transformation(scene['crop'], manifest['artboard'])
        fit_reference(cropped, transform).save(directory / 'reference.png', format='PNG')
        row = {'id': scene['id'], 'design_id': scene['design_id'], 'surface': scene['surface'],
               'declared_scene': copy.deepcopy(scene), 'crop': scene['crop'],
               'crop_image': (directory / 'crop.png').relative_to(staging).as_posix(),
               'reference_image': (directory / 'reference.png').relative_to(staging).as_posix(),
               'provenance': (directory / 'provenance.json').relative_to(staging).as_posix(),
               'crop_sha256': digest((directory / 'crop.png').read_bytes()),
               'reference_sha256': digest((directory / 'reference.png').read_bytes()),
               'source_atlas_sha256': validated['hashes']['atlas_sha256'],
               'source_prompt_sha256': validated['hashes']['prompt_sha256'],
               'crop_resampled': False, 'crop_mode': cropped.mode, 'transform': transform,
               'source_review': {'status': 'not_reviewed', 'reviewer': None},
               'native_mapping': {'status': 'not_started', 'automatic': False}}
        (directory / 'provenance.json').write_bytes(encoded(row))
        rows.append(row)
    document = {'schema_version': VERSION, 'id': manifest['id'],
                'kind': 'single-atlas-multi-scene-intake',
                'created_at': datetime.now(timezone.utc).isoformat(),
                'input_fingerprint': validated['fingerprint'], 'inputs': validated['hashes'],
                'originals': {'atlas': atlas_name, 'prompt': 'original/prompt.txt', 'manifest': 'original/manifest.json'},
                'generation': generation, 'artboard': manifest['artboard'], 'pixel_ratio': PIXEL_RATIO,
                'source_pixel_matrix': {'format': validated['atlas_format'], 'mode': validated['atlas_mode'],
                                        'size': actual_size, 'exif_orientation': validated['exif_orientation'],
                                        'orientation_applied': False},
                'intake_tool_sha256': digest(Path(__file__).read_bytes()), 'scenes': rows,
                'review_status': 'not_reviewed', 'native_mapping_status': 'not_started',
                'limitations': ['No image generation executed', 'No manual or automatic visual review claimed',
                                'No semantic mapping or native Makepad components generated',
                                'Crop coordinates address the stored atlas pixel matrix without EXIF rotation'],
                'files': {p.relative_to(staging).as_posix(): digest(p.read_bytes())
                          for p in sorted(staging.rglob('*')) if p.is_file()}}
    raw = encoded(document)
    (staging / 'intake.json').write_bytes(raw)
    (staging / 'intake.sha256').write_text(digest(raw) + '\n')
    return document


def intake(manifest_path, project, output):
    """Validate and atomically create a fresh intake, or replay it unchanged."""
    validated = validate(manifest_path, project)
    output = Path(output).absolute()
    if output.is_symlink():
        raise ValueError('output must not be a symlink')
    if output.exists():
        if not output.is_dir():
            raise ValueError('output exists and is not a directory')
        if any(output.iterdir()):
            return existing_intake(output, validated['fingerprint'])
    output.parent.mkdir(parents=True, exist_ok=True)
    staging = Path(tempfile.mkdtemp(prefix='.' + output.name + '.intake-', dir=output.parent))
    try:
        document = write_intake(staging, validated)
        try:
            os.rename(staging, output)
        except OSError:
            # A parallel identical intake may have won the atomic rename.
            # Never replace the winner or any reviewed files created there.
            if output.is_dir() and any(output.iterdir()):
                return existing_intake(output, validated['fingerprint'])
            raise
        return {**document, 'replayed': False}
    finally:
        if staging.exists():
            shutil.rmtree(staging)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--manifest', required=True, type=Path)
    parser.add_argument('--project', required=True, type=Path)
    parser.add_argument('--output', required=True, type=Path)
    args = parser.parse_args(argv)
    try:
        result = intake(args.manifest, args.project, args.output)
    except (OSError, ValueError, Image.DecompressionBombError) as error:
        parser.exit(2, str(error) + '\n')
    print(json.dumps({'id': result['id'], 'output': str(args.output.absolute()),
                      'receipt': str(args.output.absolute() / 'intake.json'),
                      'scenes': len(result['scenes']), 'actual_size': result['generation']['actual_size'],
                      'requested_size': result['generation']['requested_size'], 'replayed': result['replayed'],
                      'review_status': result['review_status'], 'native_mapping_status': result['native_mapping_status']}, ensure_ascii=False))
    return 0


if __name__ == '__main__':
    sys.exit(main())
