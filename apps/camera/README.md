# Camera — the Mate 70 Air camera, replicated in Makepad

The HUAWEI Mate 70 Air camera app (HarmonyOS 6.1, app 6.1.6.152) rebuilt with
the [image-to-appcard flow](../../lab/image-to-appcard-flow/README.md) and
Makepad's built-in instrumentation. Every page of the real app was captured on
the device over `hdc` (screenshot + `uitest` layout dump), measured into the
406 × 776 artboard, and written up as an interaction map; the storyboard, the
pipeline contracts and the native module are all derived from those numbers.

## What is here

| Part | Files | What it does |
| --- | --- | --- |
| UX map | `docs/ux-map.md` | The measured anatomy of every screen (geometry in vp, colours, fonts), the mode table, every top-chrome control, preview gestures, panels, the intro card, mode-specific overlays, capture states and the state model. |
| Storyboard | `scripts/design.py`, `source/storyboard.html`, `source/atlas.png`, `source/prompt.txt` | 12 scenes at the phone's own geometry: 拍照 · 闪光灯 panel · 色彩风格 menu · 人像 · 录像 · 录像中 · 专业 · 夜景 · 更多 · 百宝箱 · 美肤 ruler · 超清全景. Rendered by headless Chromium at 2× (`source/render_atlas.mjs`); no image model. |
| Pipeline | `image-to-appcard-flow.json`, `scripts/author_camera.py`, `scripts/build_camera.py`, `cards/camera-01..12`, `artwork/`, `wizard/card-bundle/` | intake → prepare → contracts + reviewed semantic maps → compile (L0 + kit) → extract two cards (`camera-flash-02`, `camera-toolbox-10`) → bundle. Receipts under `pipeline-output/runs/`. |
| Icons | `scripts/gen_icons.py` → `native/src/icons.rs` | One SVG dictionary feeds both the atlas and the runtime. |
| Tests | `service/test_storyboard.py` (`checks.service-test`), `native` unit tests | Storyboard geometry anchors (shutter 64 ⌀ at (203, 691.7), 52.3 vp mode pitch, 4:3 / 16:9 viewfinders); every mode and overlay renders and lowers through the shared L0 pipeline; mode bar, More grid, flash/filter/zoom, recording timer and pause, toolbox pages, settings and Back behave like the phone. |
| **Native app** | `native/` (crate `octosense-camera`) | An OctoSense `AppModule`. `session.rs` is the camera's state machine and draws each state as a scene; `scene.rs` compiles it to L0 and Kit widgets; `lib.rs` mounts it over a live preview (Makepad video input: the MacBook / Android camera) and handles swipes, taps and Back. Capture, XMAGE, AI composition etc. are mocked with the phone's exact UX. |

## Run it

```sh
export BEAUTY_PYTHON="$PWD/lab/image-to-appcard/.venv/bin/python"    # repository root
bash tools/image-to-appcard-flow.sh run \
  --project "$PWD/apps/camera" --manifest "$PWD/apps/camera/image-to-appcard-flow.json" \
  --stages intake,semantic,compile,bundle,service-test --node /path/to/node
```

Rebuild from the spec (new atlas → new intake; delete `cards/`, `artwork/`,
`pipeline-output/`, `wizard/`, `image-to-appcard-flow.json` first):

```sh
cd apps/camera
"$BEAUTY_PYTHON" scripts/design.py
node source/render_atlas.mjs source/storyboard.html source/atlas.png 3488 4950
"$BEAUTY_PYTHON" scripts/author_camera.py && "$BEAUTY_PYTHON" scripts/build_camera.py
"$BEAUTY_PYTHON" scripts/gen_icons.py
```

The native module in the OctoSense shell (desktop dev run, driven through the
Makepad remote instrument):

```sh
cd ../OctoSense-mobile && cargo build --release --features app-camera
target/release/octosense --remote --apps ../Octoscript-AppCard/apps/camera/native/desktop-apps.json \
  --module camera --test-action launch-camera
# then /snap?q=录像, /click?x&y&wait=1, /g … against the printed 127.0.0.1 port
cargo test --release --manifest-path ../Octoscript-AppCard/apps/camera/native/Cargo.toml
```

`CAMERA_LOCALE=en` switches the copy; `CAMERA_NO_PREVIEW=1` skips the camera.

## Status and limits

- Captured and mapped: all seven mode-bar modes, the seven More sub-modes,
  flash / live photo / filter / AI composition, video resolution & frame rate,
  recording (timer, pause, still), Pro parameters, night shutter, portrait
  beauty and aperture rulers, toolbox pages and second-level panels, settings,
  intro cards, focus/exposure UI, front camera, gallery hand-off.
- The replica implements all of those states; it has been driven through them
  on macOS with the remote instrument and compared side by side with the phone
  captures (chrome geometry matches; glyphs are hand-drawn approximations of
  HarmonyOS icons; the font is Noto Sans SC in place of HarmonyOS Sans).
- Real capability: the live viewfinder uses the platform camera through
  Makepad's video input (macOS, Android). On macOS a bare binary may never
  receive the camera permission prompt; the module then opens the device after
  2 s anyway and keeps the placeholder if no frames arrive. Photo/video
  capture, zoom lenses, XMAGE styles, 小艺视觉 and the gallery are mock-ups.
- OpenHarmony: Makepad's OHOS backend has no video input yet; the Octoscript-OH
  camera bridge (`libohcamera.so` preview into an XComponent) is the path to
  port. Packaging a HAP for the Mate needs DevEco Studio and a signing profile.
