# Shared beauty loop

This loop supports [the lab's LLM composition goal](../LLM-COMPOSITION.md):
make styles, native component functions and app-card layouts discoverable and
composable from user preferences and other context. Source-design parity is
one validation stage; runtime export and context-based composition require
their own interfaces and evidence.

Start with [the generic reproduction guide](REPRODUCE.md) for installation,
local Studio setup, new-input examples and commands for both branches. It
contains no credentials or personal machine configuration. The
[reviewer contract](MODEL-REVIEW.md) explains image reading: local OCR and
measurement are separate from explicit semantic/visual review, with no
Astra-specific service dependency.
See [the reproduction update validation](PORTABILITY.md) for tested coverage
and its limits.

Both source paths use the same acceptance sequence:

1. Preserve the original design and prompt/source document. Review element
   roles, fonts, layout, numerical regions and artwork ownership.
2. Compose native text, controls, surfaces and numerical charts. Keep an exact
   source-to-widget mapping. Use isolated source artwork only for its declared
   image region, with decoded-pixel provenance for crops.
3. Compile and run the release app through Studio RunItem. Collect
   WidgetTreeDump, WidgetQuery, WidgetSnapshot, layout, data and interaction
   observations tied to the request nonce and build.
4. Gate hierarchy, text, state, dimensions, alignment, spacing, visibility and
   clipping. Host-only, stale and incomplete observations fail. Numerical
   data must match actual native widget arrays, not just a copied hash.
5. Inspect the source/native screenshots for typography, colors, imagery and
   effects. A structural pass never grants visual acceptance.
6. Turn findings into a concrete repair plan, apply it, compile and recapture.
   Repeat the gates. Acceptance remains limited to the tested artboard.

`review.py prepare-source` creates an unresolved Sketch source-review template.
`review.py prepare` freezes a source/native screenshot pair for either branch;
`review.py submit` records a human or vision-reviewer's explicit decision after
checking image hashes and required criteria. These commands never judge images.
The native Sketch loop defaults to `visual_review: "external"`; its legacy
provider CLI is opt-in. Image generation is an external PNG intake step.

## Studio setup and recovery

Use a release Studio and its matching cargo-makepad client. Run the Studio
process with enough backing texture space for the largest device-pixel
capture. Merely sending RunViewResize does not guarantee that allocation.

```sh
bash tools/beauty-studio.sh /path/to/target/release/makepad-studio \
  --remote --mounts=splashref:/absolute/path/to/Octoscript-Makepad \
  --bind=127.0.0.1:8002

lab/sketch-to-appcard/.venv/bin/python lab/core/studio_bridge.py \
  --binary /path/to/target/release/cargo-makepad \
  --studio 127.0.0.1:8002 --port 8169 --log /tmp/beauty-studio.jsonl

BEAUTY_BRIDGE=http://127.0.0.1:8169 lab/sketch-to-appcard/.venv/bin/python \
  lab/sketch-to-appcard/capture_loop.py --kit taskplan-l0-all
```

The launcher defaults to 2048 × 4096 backing pixels. Override
`BEAUTY_CAPTURE_WIDTH_PX` and `BEAUTY_CAPTURE_HEIGHT_PX` for larger captures.
The bridge persists across builds. Set `BEAUTY_BRIDGE` for `run_kit.py` as well;
its doctor checks the persistent bridge instead of requiring a second client.
Serialize Sketch captures: the current host uses one shared request file, and
replacing its Studio RunItem also interrupts an earlier capture.

Sketch capture retries only recognized blank/readback transport failures, at
most three attempts. Each restart uses Studio RunItem and reuses only complete
hash-verified checkpoints. Structural or semantic failures require repairs.
Inspect `capture-recovery.json` and `capture-attempts/` after interruption.
Derived gate reports are excluded from immutable capture hashes, so repeated
audits cannot invalidate their own evidence.

The L0 capture preflight compares the compiled card tree against its current
native source tree and stops before Studio when they disagree.
If source designs changed after L0 promotion, regenerate the cards before
capturing: `promote_l0.py --kit <source-native-kit>`. The normal `beauty` stage
includes promotion. The `audit` stage only verifies saved evidence.

## Reviewable repair plans

Plans are structured JSON, not executable scripts. They bind `inputs` and
immutable `evidence` using SHA-256, list concrete findings, and specify each
operation's category, element and reason. Supported operations are `set`
(JSON pointer with before/after), `document` (replace a JSON document), and
`asset` (copy an explicitly reviewed source asset). Categories cover layout,
typography, color, asset, semantic, data, hierarchy and state.

```sh
bash tools/beauty-pipeline.sh --repair --root lab/image-to-appcard/weather-11 \
  --plan lab/image-to-appcard/weather-11/repair-001-chart.json --preview

bash tools/beauty-pipeline.sh --ux-image --design weather-11 \
  --repair-plan lab/image-to-appcard/weather-11/repair-001-chart.json \
  --stages repair,font,compile,capture,gate,gallery
```

`weather-11/repair-001.json` and `repair-001-chart.json` are real reviewed
examples, with font and numerical-curve changes validated in Studio.
Completed plans replay without applying their changes twice. A write-ahead
journal permits recovery between file writes and refuses conflicting edits
or changed dependencies. Applying a repair never approves its output.

Sketch plans can update a semantic manifest or reviewed input JSON with this
same engine, followed by import, L0 promotion where applicable, capture and
audit. Source screenshots and prior gate evidence are never repair outputs.

## Explicit limits

The shared policy is `policy.py`, loading `../image-to-appcard/mapping-rules.json`.
Semantic classification and visual judgments still require review. Every
Sketch artboard requires a `source_review` receipt in its semantic manifest,
bound to the source hierarchy and reference hashes, even when no named chart
is detected. The reviewer must inspect anonymous data regions too. A containing
card's reviewed role cannot conceal an unreviewed chart child. Name/OCR
heuristics nominate regions; they cannot prove a chart is decorative. Unknown
roles and unsupported adapters fail until resolved. Reviewed Sketch adapters
cover line/area curves, bars and intervals, styled donut arcs, radar axes,
bubbles, waveforms and progress tracks. Source transforms, masks, gradients,
numeric domains and units must be explicit and validated. The image adapter
supports a reviewed single-series region. Other chart forms need additional
adapters and numerical validation. See the [old-kit migration](OLD-KIT-MIGRATION.md)
for the Taskplan, Atro and Camo mappings and their evidence.

The pipeline does not claim live market/weather data, arbitrary responsive
layouts, full application navigation, accessibility or Mate 70 parity from a
desktop screenshot. Those need separate functional and device evidence.

## OpenHarmony instrument

Makepad's Studio inspection only sees widgets makepad draws itself, so a card
Octoscript-OH renders as native ArkUI needs the OS's own instrument.
`ohos_instrument.py` captures the same artifacts a Studio round has —
`tree.json` (WidgetTreeDump shape), `snapshot.json`, `queries.json` and
`native.png` — from `uitest dumpLayout` and `uitest screenCap` over hdc, in
logical pixels (`--density`, 3.25 on the Mate 70 Air). Ids come from the ArkUI
component id, which Octoscript-OH sets from the card node's `id`.

```sh
python3 lab/core/ohos_instrument.py --device 5ZGYD25B13020968 \
  --bundle com.example.myapplication --out lab/core/work/<kit>/ohos/001
```
