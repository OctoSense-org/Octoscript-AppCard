# Lab structure and cleanup policy

Organize the lab around [LLM-driven style and app-card composition](LLM-COMPOSITION.md):
ingest designs, extract reusable capabilities, publish a complete registry,
select from context and validate the resulting native composition. Directory
cleanup supports that product goal. The linked contract distinguishes current
runtime capabilities from the exposure and context work still required.

The lab keeps two source adapters, Sketch → AppCard and image → AppCard, and one shared
`core`: `core` owns policy, review, repair, the Studio transport, composition and gates and the
native renderer; `sketch-to-appcard` owns document import and kit promotion; `image-to-appcard`
owns image observation and mapping. `image-to-appcard-flow` extends the image
adapter into multi-screen service state, independent cards and WASM/web delivery;
it reuses core gates and does not define a third source adapter. Keep shared
rules in the core and source-specific conversion in its adapter. New
experiments belong in a named study directory with a README stating purpose,
inputs, entry point and status. Remove obsolete studies after checking their
callers and recording the cleanup decision.

## Current directory boundaries

```text
lab/
  README.md                 supported entry points and status
  STRUCTURE.md              ownership and retention rules
  maintain.py               inventory and narrowly scoped cache cleanup
  tests/                    maintenance safety tests
  core/                     shared by both adapters: mapping policy, review packets,
                            repair engine, Studio bridge, composition and gates,
                            the native renderer, kit configuration
    kits/                   kit configuration
    work/<kit>/             source copies, final assets and capture evidence (ignored)
    examples/               generic, redistributable input templates
    qa-work/                local verification evidence (ignored)
  sketch-to-appcard/        Sketch adapter: import, native composition, kit promotion;
                            `run_kit.py`, its regression tests, bundled fonts
    .venv/                  local environment (ignored)
  image-to-appcard/         generated-image adapter: intake, measurement, mapping;
                            `run.py`, its regression tests
    <design-id>/            prompt, reference, contract and reviewed mapping
      rounds/<round>/       immutable capture evidence (ignored)
    .venv/                  local environment (ignored)
  image-to-appcard-flow/    whole-flow orchestration, atlas intake, native subtree
                            export, provenance-aware bundle and WASM templates;
                            application sources/evidence stay in external projects
```

Code, durable input descriptions and generic examples are versionable. New
scratch outputs go under an adapter's `work/` or `qa-work/`; they must not add
more `out2`, `frozen3` or `shots_final_final` siblings to its source directory.
Preserve explicit source IDs, run IDs and hash-bound review records.

## Findings and safe cleanup

The initial inventory on 2026-09-08 measured about 51 GiB allocated under lab,
including 48 GiB in Sketch `work/`, 2.1 GiB in `image-to-appcard`, and 875 MiB in
`gates`. The initial Git inventory contained 6,268 lab files, approximately
1.03 GiB of file content;
thousands are historical images and generated artifacts. These are different
problems: deleting local caches saves workspace space, while changing Git
tracking/history requires a dataset and retention migration.

The retired beauty pipeline and unused RICO dataset were initially archived,
then deleted at the user's request. Neither had a current code caller. Their
archive directories and the empty archive index have also been removed.

`maintain.py clean` selects only Python bytecode directories and Finder
metadata. `--exports` also selects Sketch `graphic-export-cache` directories
under the adapter's work tree, provided original/resolved documents and a
source receipt remain. These are intermediate CLI exports copied into final
assets by `sketch_assets.py`; a cache miss triggers re-export. They are outside
the native capture fingerprint. Six export caches occupied approximately
9.78 GiB before cleanup. The next import that needs those exports can take
longer and requires the original compatible Sketch tool.

Cleanup previews by default, validates every candidate before deletion,
refuses tracked files and symlinks, and skips installed environments. Stop
import jobs first; this tool does not coordinate with an active importer.
There is no broad `purge` or automatic removal of failed review rounds.

## Files that are old but still required

| Path | Verified consumer / reason to retain |
|---|---|
| Sketch `cards2/`, `frozen2/` and related fixtures | Used by legacy capture/regression scripts; not interchangeable with current native cards |
| `image-to-appcard/weather-v1/` | Original image pilot and its provenance, referenced by the adapter README |
| `image-to-appcard/.deps/` | Still used by some measurement/repair scripts; the portable guide uses dedicated environments |
| Captures, repair rounds, import receipts, source trees and manifests | Acceptance, replay, migration provenance and regression evidence |

## Further structural migration

The desirable next layout groups beauty work by `ingest`, `library`, `export`,
`context` and `validation`, with shared infrastructure, research, application
fixtures and an explicit artifact root. The registry/export layer exposes
styles, component functions and app-card recipes to the LLM. This needs a
compatibility migration, rather than filesystem moves alone:

1. Introduce configuration for code, design, dataset and artifact roots;
   replace `HERE.parent`, absolute paths and runtime resource URLs at their
   actual consumers. Keep existing CLI paths working during the transition.
2. Move app-consumed baselines and corpora with all imports and `include_str!`
   references. Split reusable gates from one-off study runners, and adapters
   from their tests and data. Retire legacy scripts only after their callers
   and regression inputs are migrated.
3. Export large historical datasets/evidence with manifests, checksums,
   restoration commands and a stated retention policy. Keep current source,
   accepted runs and referenced repair history available. Git LFS or artifact
   storage is a storage choice, not permission to discard evidence.
4. Revalidate affected captures and browser links after changing paths.
   Sketch capture fingerprints currently include absolute input paths; image
   proof also names source paths. Moving an active tree can invalidate saved
   evidence even when file bytes are unchanged. Never edit proof hashes to
   make a moved tree appear current.

This cleanup leaves those active roots in place. It establishes clear
ownership, removes obsolete studies, prevents new transient artifacts entering Git,
and removes rebuildable caches without changing the accepted designs.

## Cleanup verification

The completed cleanup removed 79,227 cache/metadata files representing 9.786
GiB of allocated storage. That run left about 41.56 GiB before the subsequent
archive deletion. All 255 tests passed: 187 Sketch, 41 image, 22 shared and five
maintenance safety tests. Shared native capture fingerprints are unchanged
for Taskplan, Atro and Camo (475 configured screens); no recapture was needed.
Modified documentation links and whitespace checks pass.

The local [validation record](core/qa-work/structure/validation.json)
records that initial cleanup. The subsequent
[archive deletion record](core/qa-work/structure/archive-removal.json)
records removal of both archived directories, including approximately 645 MiB
of file content. Active pipeline sources and validation evidence remain in place.
