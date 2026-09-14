# Reproducible Makepad service-card WASM host

This module packages the working browser host as a portable pipeline step. It mounts L0 source and native kits in memory, producing real Makepad `View`, `Label`, `Button`, `Svg` and `Image` widgets. It does not reproduce the design with a DOM overlay. Service events and navigation remain the enclosing flow controller's responsibility.

## Build interface

Requirements: Python 3.9+, Git, Cargo/Rustup with the nightly toolchain and its `rust-src` component, and the project inputs below. Node 18+ and Playwright are required only for the browser smoke. The compiler uses the pinned Makepad custom WASM target, release optimization, no LTO and no atomics; SharedArrayBuffer and COOP/COEP headers are unnecessary. The exact installed Rust toolchain is recorded, not claimed to be interchangeable across versions.

```sh
python3 lab/image-to-appcard-flow/wasm/build.py \
  --workspace "$PIPELINE_ROOT" \
  --project "$SERVICE_PROJECT" \
  --output "$BUILD_ARTIFACTS/wasm-dist" \
  --build-dir "$BUILD_CACHE/service-card-wasm"
```

`--workspace` is an Octoscript-AppCard checkout containing the `makepad`, `splash` and `splash-makepad` dependency checkouts. `--project` is an external service project. `--output` must be new; the tool refuses to overwrite an existing package. `--build-dir` is a reusable scratch directory, separate from output and source submodules.

For an existing publication directory, `--replace` builds and verifies a complete staged package before an OS atomic directory exchange (macOS `renamex_np` / Linux `renameat2`). The output path never disappears during exchange. The previous package is then archived beside it as `.<name>.previous-<build-id>`; interruption after exchange leaves the old package intact at the recorded staging path. Platforms without atomic exchange fail while retaining the previous output. Failed compilation or packaging leaves the published directory untouched. The flow orchestrator uses this option when refreshing its configured `outputs.wasm` location.

The default `--dependency-mode isolated` clones the recorded revisions into the scratch directory and applies the bundled patches there. Existing source submodules are neither checked out nor patched. Local Git objects are reused when available; otherwise the recorded public origins are cloned. A dependency checkout with additional edits fails validation rather than being reset. To reuse an already compatible working checkout, `--dependency-mode existing` verifies exact revisions, tracked patch hashes and modified source hashes before and after building.

`--cargo-makepad /path/to/cargo-makepad` reuses an existing compiler executable and records its SHA. Otherwise the tool builds `cargo-makepad` into scratch if no executable is present. `--target-dir /path/to/cargo-cache` symlinks the generated host's `target` directory to a caller-owned cache; do not share an active cache between concurrent builds. `--prepare-only` stages and verifies the dependency snapshot and generated Cargo manifest without compiling.

The automatic compiler build uses the bundled `toolchain/cargo-makepad.lock` with `--locked`, retains that lock and compiler log in the run evidence, and only writes to an isolated dependency checkout. Existing-dependency mode requires an existing compiler executable so it does not generate a lock inside the source submodule.

The project supplies:

- `fonts/NotoSansSC-{Regular,Medium,Bold}.ttf` and `fonts/OFL.txt`
- `cards/**/contract.json` and/or `service-cards/**/contract.json`
- Each exported card's `page.card`, `page.data.json`, `mapping.json`, native kit and `assets/` as produced by the image-to-AppCard pipeline

Contract IDs determine output `card-assets/<id>/assets/<file>` paths. IDs and flat artwork filenames must contain only letters, numbers, underscore, hyphen or dot. Conflicting duplicate IDs and assets outside the project are rejected. This initial host embeds the complete Noto Sans SC Regular font and retains four known fallback faces; other font sets require an explicit template/package change and renewed rendering checks.

Discovery stops descending once a card directory has `contract.json`, so earlier `rounds/` snapshots and nested evidence are preserved as history and are never accidentally shipped as current artwork.

## Dependency snapshot

`dependencies.lock.json` fixes the known working revisions and SHA-256 hashes of the complete tracked patch sets. `patches/makepad.patch` restores checked VM evaluation, native state/geometry APIs, text tracking in logical pixels, Label allocation measurement, and the no-atomics SDF path. `patches/splash-makepad.patch` carries the corresponding VM/draw API adaptations and browser asset policy. The clean Splash revision has an empty patch. Native-only compatibility hunks are retained to reproduce the known source snapshot; this is not a claim that each hunk is necessary for every browser card.

Generated `makepad_platform/web.js` receives three exact-match packaging patches: iframe blur does not recapture focus, initial focus prevents parent scrolling, and later keyboard focus prevents scrolling. Original dependency JavaScript remains unchanged. The index-to-bridge and bridge-to-WASM URLs are stamped with the WASM SHA to avoid an older cached module after publication.

`patches/image-to-appcard.patch` separately preserves the parent lab's Chinese OCR, disabled-control activation checks, viewport cropping and capture lifecycle fixes, with base revision and source hashes in `lab_compatibility` in the lock. These changes are required to reproduce the corresponding native Studio inspection workflow; the in-memory WASM build does not call those lab adapters. On the recorded clean pipeline revision, `git apply --check lab/image-to-appcard-flow/wasm/patches/image-to-appcard.patch` verifies applicability before applying that patch. If the lab fixes have already been committed upstream, compare the recorded source hashes instead of applying twice. The WASM builder never changes those source files or initializes Studio.

## Browser transport and artwork policy

The iframe accepts messages only from its actual same-origin parent. Send `{type:'octosense:render', id, generation, card, data, kit, mapping, width, height, updates}`. `updates` contains `{id,text?,enabled?}` entries keyed by source or native widget IDs; Button IDs usually end with `_control`. Width and height are the requested card's logical artboard, not a fixed screenshot size.

After checked L0/kit lowering and a real draw, `octosense:rendered` echoes the request ID/generation and actual widget areas, text layouts, image readiness and enabled state. It waits for the mapped font and image/vector resources. `octosense:inspect` returns the corresponding `octosense:snapshot`. Actual `KitAction::Activated` events produce `octosense:action` with the same request ID/generation and source control ID. The parent must reject stale generations and own the service reducer; the host does not perform payment or other external service actions.

Asset URLs must stay inside the iframe's same-origin sibling `card-assets/` directory. The bridge rejects traversal, foreign origins, query/fragment components and noncanonical URL forms. The locked WASM renderer permits HTTP `127.0.0.1`/`localhost` development addresses and exactly these HTTPS bases:

- `https://octosense.org/wasm/service-cards/card-assets/`
- `https://octosense-org.github.io/wasm/service-cards/card-assets/`
- `https://octosense-org.github.io/Octosense-website/wasm/service-cards/card-assets/`

Other domains or deployment prefixes require a reviewed WASM policy patch and rebuild. Native/lab rendering retains its original loopback-only policy. This restriction is intentionally not an arbitrary remote-resource bypass.

## Evidence and browser smoke

Every run gets a new `runs/<build-id>/receipt.json` and compiler log in scratch. Failed builds retain their actual failure and any incomplete package. Successful `build.json` records the WASM SHA, compiler/toolchain, exact dependency revisions plus dirty patch/source hashes, pipeline/template sources, generated Cargo/native sources, project font/card/artwork hashes, generated JavaScript patches and every shipped file's hash except `build.json` itself. The run receipt also hashes the final package receipt. Pipeline hashes live in `pipeline_sources`; `sources` contains only paths relative to `--project`, preserving the site publisher's input-verification contract.

```sh
node lab/image-to-appcard-flow/wasm/smoke.cjs \
  --dist "$BUILD_ARTIFACTS/wasm-dist" \
  --card "$SERVICE_PROJECT/cards/product" \
  --playwright "$PLAYWRIGHT_MODULE" \
  --origin https://octosense.org \
  --activate buy_control --expected-action buy \
  --output "$BUILD_ARTIFACTS/browser-smoke"
```

The smoke intercepts the chosen origin and serves the local package under `/wasm/service-cards/`; it does not deploy or contact live services. `--base-path` can select another permitted deployment prefix, and `--asset-prefix` sets the source lab artwork URL prefix to rewrite. `--disabled` optionally names a disabled native control to click. `--browser` selects an explicit Chromium executable. The test uses the selected contract's artboard, saves only the canvas drawable, checks actual widget bounds and native actions, and rejects foreign/traversal artwork. Its report binds the binary and fixture hashes. This is a native mount/action smoke, not a full image-parity gate or a complete service-flow test.

Chromium 1243 intermittently stalled when cold-mounting a text-heavy appointment card directly in the original host. A hosted-origin run passed on 1243, a localhost comparison passed on 1234, and complete integrated flows beginning with the product scene passed on 1243. Failed attempts were retained. Do not infer universal browser compatibility from the build, or silently replace a failed runtime test with a screenshot. Retest the actual deployment, browser and intended start sequence.
