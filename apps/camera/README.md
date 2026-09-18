# Camera — the Mate 70 Air camera, replicated in Makepad and in native ArkUI

The HUAWEI Mate 70 Air camera app (HarmonyOS 6.1, app 6.1.6.152) rebuilt with
the [image-to-appcard flow](../../lab/image-to-appcard-flow/README.md) and
Makepad's built-in instrumentation. Every page of the real app was captured on
the device over `hdc` (screenshot + `uitest` layout dump), measured into the
406 × 776 artboard, and written up as an interaction map; the storyboard, the
pipeline contracts and the native module are all derived from those numbers.

The app logic (`logic/`) is one Rust crate with two hosts: `native/` draws it
with Makepad (macOS, Android, OpenHarmony) and `oh/` draws it with the phone's
own ArkUI widgets through Octoscript-OH's C-API binding. Both run on the Mate.

## What is here

| Part | Files | What it does |
| --- | --- | --- |
| UX map | `docs/ux-map.md` | The measured anatomy of every screen (geometry in vp, colours, fonts), the mode table, every top-chrome control, preview gestures, panels, the intro card, mode-specific overlays, capture states and the state model. |
| Storyboard | `scripts/design.py`, `source/storyboard.html`, `source/atlas.png`, `source/prompt.txt` | 12 scenes at the phone's own geometry: 拍照 · 闪光灯 panel · 色彩风格 menu · 人像 · 录像 · 录像中 · 专业 · 夜景 · 更多 · 百宝箱 · 美肤 ruler · 超清全景. Rendered by headless Chromium at 2× (`source/render_atlas.mjs`); no image model. |
| Pipeline | `image-to-appcard-flow.json`, `scripts/author_camera.py`, `scripts/build_camera.py`, `cards/camera-01..12`, `artwork/`, `wizard/card-bundle/` | intake → prepare → contracts + reviewed semantic maps → compile (L0 + kit) → extract two cards (`camera-flash-02`, `camera-toolbox-10`) → bundle. Receipts under `pipeline-output/runs/`. |
| Icons | `scripts/gen_icons.py` → `logic/src/icons.rs` | One SVG dictionary feeds both the atlas and the runtime. |
| Tests | `service/test_storyboard.py` (`checks.service-test`), `logic` unit tests | Storyboard geometry anchors (shutter 64 ⌀ at (203, 691.7), 52.3 vp mode pitch, 4:3 / 16:9 viewfinders); every mode and overlay renders and lowers through the shared L0 pipeline; mode bar, More grid, flash/filter/zoom, recording timer and pause, toolbox pages, settings and Back behave like the phone. |
| **App logic** | `logic/` (crate `octosense-camera-logic`) | Host-independent. `session.rs` is the camera's state machine and draws each state as a scene (absolute artboard coordinates, controls, SVG assets); `scene.rs` is that scene format; `icons.rs` the glyphs. Both hosts below consume it unchanged. |
| **Makepad host** | `native/` (crate `octosense-camera`) | An OctoSense `AppModule`: lowers each scene through L0 to Kit widgets, mounts it over a live preview (Makepad video input) and handles swipes, taps, springs, the dial and Back; real capture on OpenHarmony through Makepad's `camera_capture`. |
| **ArkUI host** | `oh/` (crate `octosense-camera-oh`, `oh/deveco/` shell, `oh/build.sh`) | The same scenes as native ArkUI nodes (Stack / Text / Image / XComponent) built from Rust through the ArkUI C API (Octoscript-OH's `octoscript-oh-arkui`); the preview streams straight into an XComponent surface; capture, zoom, focus, flash and EV go to the camera NDK from the same crate; ArkTS is only the ability shell, the permission prompts and the gallery hand-off. |

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

The ArkUI host on the Mate 70 Air (DevEco Studio 26 with its OpenHarmony SDK
and `aarch64-unknown-linux-ohos` Rust target; the device on `hdc`):

```sh
cd apps/camera/oh
sh build.sh              # cargo → libcamera_oh.so → hvigor HAP → install → launch → hilog
sh build.sh --build-only
```

`oh/Cargo.toml` expects sibling checkouts of `Octoscript-OH` (branch
`feat/raw-touch-rotate`: the ArkUI shim's raw-touch stream and `NODE_ROTATE`),
`octoscript` and `makepad`; `build.sh` signs with the DevEco auto-signing
profile of `~/DevEcoStudioProjects/MyApplication` (bundle
`com.example.myapplication`, with the `WRITE_IMAGEVIDEO` ACL for silent
gallery saves) and replaces whichever host is installed under that bundle.

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
  Makepad's video input (macOS, Android, OpenHarmony). On macOS a bare binary
  may never receive the camera permission prompt; the module then opens the
  device after 2 s anyway and keeps the placeholder if no frames arrive.
  On OpenHarmony the shutter takes a real 4096×3072 JPEG and 录像 records
  1080p30 MP4 clips with audio (pause/resume included) through Makepad's
  `camera_capture`; each file lands in the app's `DCIM` folder, the gallery
  corner shows the still, and the system "允许保存" dialog hands it to the
  phone's gallery (`docs/evidence/mate-capture-*.jpeg`). XMAGE styles,
  小艺视觉 and the in-app gallery view remain mock-ups.
- OpenHarmony (on the Mate 70 Air itself): Makepad's OHOS backend now drives
  the camera NDK (preview into an ImageReceiver, frames read back and
  uploaded as Y/U/V planes), asks for `ohos.permission.CAMERA` through the
  ArkTS bridge and serves the icons over loopback HTTP, so the module runs on
  the phone with the real rear/front viewfinder. Build with cargo-makepad's
  `ohos` backend (`deveco -p octosense --release --features app-camera`,
  DevEco Studio 26, API 24 device) and sign with a DevEco profile.
  `docs/evidence/mate-on-device-tour.jpg` is the replica driven through 24
  states on the phone via the remote instrument (`hdc fport tcp:7891`);
  `mate-on-device-vs-real.jpg` puts the phone's camera app and the replica side
  by side on the same scene. Front-preview mirroring is not done yet.
- Motion: the mode bar and the quick-zoom selection are sliding strips the
  module moves in place every frame with a critically damped spring
  (0.42 s response, 0.9 damping): taps glide, a finger drags the bar and it
  snaps to the nearest entry with a little fling, and the zoom chip slides
  between labels (`docs/evidence/mate-motion-zoom-chip.png`, four frames
  90 ms apart on the phone). Measured through the instrument on the Mate:
  a mode tap settles in about 0.4 s without overshoot.
- Real camera controls on OpenHarmony: tap-to-focus sets the focus and
  metering point and the box follows a dragging finger, the quick-zoom chips
  and a two-finger pinch set the lens ratio, the flash menu sets the
  flash/torch mode and the Pro EV value sets exposure compensation, all
  through Makepad's `camera_control`.
- The roulette zoom dial (`docs/evidence/mate-zoom-dial.jpeg`): a press-and-hold
  or a sideways slide on the quick-zoom bar opens the phone's ring (radius 270
  centred below the viewfinder, ticks every 0.1 octave at 17° per octave,
  labelled stops with focal lengths, red readout under the dot and the large
  readout in the viewfinder). The ring is one SVG the module rotates in place
  as the finger slides, the lens follows continuously, a flick coasts with
  exponential decay, and after 1.2 s the chips return with the live value on
  the nearest stop.

- The ArkUI host (`docs/evidence/mate-oh-native-tour.jpg`, twelve states on
  the phone): the whole chrome is native nodes (a mount is 4 ms for 78 nodes
  against 11–34 ms for the Makepad remount), text is ArkUI Text in the system
  font, icons are the same SVGs served as files, the viewfinder is an
  XComponent the camera NDK renders into with no frame readback, and the
  window is edge to edge under a transparent status bar. Taps come back as
  node click events; every continuous gesture (mode-bar drag and fling,
  focus-box drag, pinch, the dial's slide and coast, the chip and bar springs)
  runs on the raw touch stream Octoscript-OH's shim now forwards, with the
  same spring and dial numbers as the Makepad host, stepped from a 16 ms
  ArkTS timer. Photo and video capture, pause/resume, the front camera and
  the silent gallery save are all verified on the device. Not there: the
  Makepad remote instrument (this host was driven with `uitest` and Hypium
  instead), the intro-card artwork is a flat colour as on the Makepad host,
  and the mode strip is not clipped at the artboard edge.

## Evidence

`docs/evidence/` holds the phone-vs-replica sheets produced by driving the
module through the remote instrument (left: the Mate 70 Air capture cropped to
the artboard; right: the Makepad replica at the same state), and
`mate-oh-native-tour.jpg` for the ArkUI host.
