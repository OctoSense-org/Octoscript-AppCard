# Native App Card testing with Makepad's built-in instrument

Use the app's built-in HTTP instrument for new native UI inspection and interaction tests. Build a release executable and launch it directly with `--remote`. Automated tests can hide its native window with `MAKEPAD_HIDE_WINDOWS=1`. Studio, RunItem, and the Studio bridge are not required for this workflow.

This is the default native testing workflow for image-to-appcard projects. The image intake, semantic mapping, L0 compilation, service reducer, bundle, and visual-review requirements remain in place. Older Studio capture commands have a compatibility boundary described below; documenting the direct instrument does not change their implementation.

## Build and launch an owned instance

Follow [Makepad's agent runbook](../../makepad/AGENTS.md) and [app remote reference](../../makepad/docs/agents/app-remote.md). Read the running binary's `GET /` response for its actual protocol. Use a fresh release build after runtime changes, and record the source revisions and local adapter patches.

From the pipeline root, for the existing `beauty-host` L0 adapter:

```sh
APP_PIPELINE_ROOT="$PWD"
APP_NATIVE_REQUEST="/absolute/path/to/app/runtime/current-request.json"
APP_NATIVE_LOG="$(mktemp -t appcard-native)"
cd "$OCTOSENSE_WORKSPACE/octoscript-makepad"
RUSTFLAGS='' CARGO_PROFILE_RELEASE_LTO=false cargo build --release -p kit-host --bin beauty-host
MAKEPAD_HIDE_WINDOWS=1 BEAUTY_REQUEST="$APP_NATIVE_REQUEST" \
  ./target/release/beauty-host --remote > "$APP_NATIVE_LOG" 2>&1 &
APP_NATIVE_PID=$!
```

The request file must already be authored by the app/controller. It identifies the compiled `.card`, data JSON, kit directory, dimensions, unique request `nonce`, output paths for native/layout/action evidence, and a unique launch identity. The current host also accepts that standalone identity in its historical `build_id` field; it is not a Studio build ID. Mount one request at a time. Do not let two controllers share a request or action file.

Run the executable from its owning workspace so resources resolve correctly. Start a loopback artwork server only when compiled assets require one. Set the app's own credential/configuration path separately; never place credentials in the request or launch arguments.

The startup log contains the instrument endpoint and owned process:

```text
[makepad-remote] listening on 127.0.0.1:PORT pid=PID app=beauty-host grabs=...
```

Retain that PID and endpoint. Do not discover or take over an unrelated running app. For a visible handoff, omit `MAKEPAD_HIDE_WINDOWS`; do not activate the window unless the user asks.

**Headless here means a hidden native window.** The tested macOS route retains Metal, native controls, and platform WebViews. It still needs a native macOS session. It does not claim that the separate `cfg(headless)` software renderer, a display-free Linux runner, iOS, or WebAssembly has been verified.

## Inspect and inject real input

Set `APP_NATIVE_ENDPOINT` to the endpoint printed by that launch:

```sh
curl --fail --silent --show-error "$APP_NATIVE_ENDPOINT/"
curl --fail --silent --show-error "$APP_NATIVE_ENDPOINT/s"
curl --fail --silent --show-error --get "$APP_NATIVE_ENDPOINT/snap" \
  --data-urlencode 'q=TextInput' --data 'all=1'
```

`/snap` returns `s` entries with native widget ID `i`, type `ty`, window `w`, and rectangle `r: [x,y,width,height]`; text/value fields include `t` and `val`. Resolve source IDs through the compiled `mapping.json`, then select the exact native ID. A substring query may also match descendants. `all=1` includes otherwise omitted widgets; it does not prove they are visible or clickable.

**Rectangles are window-local logical points.** Do not add the OS window position or multiply by DPI. Fetch fresh bounds after a mount, scroll, or resize. Click within the current visible rectangle:

```sh
# X and Y come from the latest native /snap rectangle.
curl --fail --silent --show-error --get "$APP_NATIVE_ENDPOINT/click" \
  --data "x=$APP_CLICK_X" --data "y=$APP_CLICK_Y" --data 'wait=1'
curl --fail --silent --show-error --get "$APP_NATIVE_ENDPOINT/k" \
  --data 'k=press' --data 'c=KeyA' --data 'cmd=1' --data 'wait=1'
curl --fail --silent --show-error --get "$APP_NATIVE_ENDPOINT/t" \
  --data-urlencode 't=pop.example.com' --data 'wait=1'
curl --fail --silent --show-error --get "$APP_NATIVE_ENDPOINT/m" \
  --data 'k=scroll' --data 'x=200' --data 'y=450' --data 'dy=600' --data 'wait=1'
```

`wait=1` waits for the resulting native frame. Also wait for the app's controller/reducer and request nonce to settle before asserting an asynchronous change. Acceptance tests must observe real native actions and resulting state; directly calling a reducer is a unit test, not an input test. Cover disabled controls, validation, retry, stale responses, selection/focus across redraws, and scroll restoration where relevant.

Use `/d` for the native tree and `/log?n=50` for app logs. Assert geometry and clipping from the host's measured layout where `/snap` reports only the clipped rectangle. Do not substitute rendered text or an entire screenshot for a native control.

## Capture, provenance, and cleanup

Use `/g` to capture the app's own drawable. It returns a `png` path; `/g?raw=1` returns PNG bytes. A screenshot is evidence for visual inspection, not automatic visual approval. Platform overlays such as WKWebView may require the app's existing WebKit inspection/snapshot hook; do not claim a Metal capture includes them. Never use an OS/window/display screenshot as a fallback.

Record at least:

- Native executable and relevant source/artifact hashes, launch identity, PID, hidden/visible mode, and request nonce.
- The instrument protocol response, native snapshots/tree, observed action/result evidence, and app-owned captures needed by the change.
- Fixture versus real-service checks, passed/failed assertions, and unrun visual, browser, or platform checks.
- Cleanup result for each temporary test process.

Keep personal mail, credentials, and real-account captures under private ignored storage. Publish fictional fixtures and counts-only live-service receipts. A masked TextInput can still expose its underlying value through inspection or action logging. Use a secure platform credential dialog with a private controller pipe, or an explicitly protected credential channel; do not type real passwords through `/t`, put them in URLs, or copy them into card data, snapshots, prompts, or logs.

Finish every temporary test instance with:

```sh
curl --fail --silent --show-error "$APP_NATIVE_ENDPOINT/gq"
wait "$APP_NATIVE_PID"
```

`/gq` returns `png` as an **array of paths** and `quit: 1`; `/g` returns one path. Copy any required evidence and confirm the process exited. If capture is unavailable, use `/quit`. If `/gq` already closed the process, a failed second request is not a reason to relaunch. Preserve an instance only when handing the app to the user as requested. A human-closed window is not a crash to auto-restart.

## Working example: native mail app and server settings

The [Mail app](../../apps/mail/README.md) in `apps/mail/` implements this workflow without Studio:

| Project file | Responsibility |
|---|---|
| `scripts/run.py` | Build release, launch `beauty-host --remote`, optionally hide its window, discover and record the owned endpoint/PID, run the service controller, close on exit. |
| `scripts/instrument.py` | Direct HTTP requests, app-owned capture, and nonce-bound readiness checks. |
| `scripts/verify_settings.py` | Hidden native fixture: edit fields, validate ports, test success/failure, save/reload, handle a private password result, switch account caches, discard unsaved edits, then `/gq`. |
| `scripts/verify_completion.py` | Hidden native Send/Reply, attachment, Gmail folder/flag, retry and outgoing-settings flows. Uses a local TLS SMTP server and an IMAP protocol fixture; confirms `/gq` exit. |
| `scripts/verify_server_connection.py` | Explicit live-account save/connection check using native controls, publishing counts/status only. |
| `service/account.py`, `service/mailbox.py` | Saved POP3 configuration, TLS/STARTTLS before authentication, credential identity, bounded retrieval, and separate account mail caches. |
| `service/sending.py`, `service/gmail_sync.py`, `service/attachments.py` | SMTP delivery journal, Gmail metadata/labels, and verified private attachment extraction; secrets stay in the service. |
| `scripts/mail_actions.py` | Async controller, per-account pending changes, stale-response checks, delivery recovery, folder pagination and attachment actions. |
| `service/password_prompt.swift` | AppKit secure password entry through a private process pipe; no password in L0 or instrument messages. |

The settings surface includes email, login name, host, port, SSL/TLS or STARTTLS, Gmail recent mode, secure password change, Save, Test Connection, and Check for Mail. Test Connection uses edited settings and performs authentication/STAT without retrieving mail. Save persists settings; Check for Mail uses the saved account. Changing host/login requires a password for that identity and keeps messages/drafts separate.

The outgoing-settings subpage edits SMTP host/port and SSL/TLS or STARTTLS, and enables Gmail IMAP synchronization. Its connection test authenticates SMTP with NOOP and reads IMAP folders; it does not send mail. Compose/Reply have separate Send and Save Draft controls. Delivery journaling blocks automatic resends after uncertain SMTP acceptance. Attachment Open validates downloaded bytes before dispatching the platform viewer. Gmail mutations use stable message identity and an idempotent, persistent queue; failed operations remain visible for retry.

The 2026-09-15 completion fixture passed through the built-in instrument with hidden windows. It delivered three fictional messages to a **local TLS SMTP server**, tested rejection and disconnect recovery, and exercised folder/flag mutations against an **IMAP protocol fixture**. The viewer launch was intercepted after verifying extracted file bytes. Separate read-only Gmail checks authenticated SMTP/IMAP, matched 87 cached messages, listed 29 folders, and downloaded a real Sent message and attachment. No external test message or live flag/label mutation was performed. The companion app keeps counts-only receipts and fictional screenshots in `evidence/`; personal content stays private. Thirty service/controller tests cover protocol behavior, delivery recovery, queue races, identity and pagination. Native AppKit password-dialog compilation was verified; the dialog's controller result uses a fixture subprocess. These results do not claim image parity or other-platform acceptance.

Keep app source and curated fixture evidence in its dedicated `apps/mail/` directory; generated sessions and personal mail remain ignored. Do not make generic pipeline documentation depend on a developer-specific absolute home directory, copy private email into this repository, or regenerate the original atlas for a code-only settings change.

## Android launcher modules

The pinned Makepad release compiles its HTTP `--remote` instrument out on
Android. Do not report desktop HTTP probes or Studio captures as Android passes.
Mail exports the reviewed renderer's scene templates and applies the same L0
realization, native kit lowering and renderer inside OctoSense-mobile. Its
standalone Rust workers run mail services on the phone; the internal loopback
transport carries only the module's local frames/actions/assets. A Mac companion
is now an explicit legacy mode. See [Mail's Android instructions](../../apps/mail/README.md#standalone-android-mail).

Build the locked runtime's release APK with an existing Android SDK and install
an explicitly owned preview package. With `open_android.py --demo --probe`,
observe native widget bounds and inject actual device touch input using
`device_instrument.py`; convert logical coordinates with the current density.
Verify resulting state and nonce. Exercise subject search, scrolling across the
60-row window, full-length plain/HTML reading, compose/drafts and settings.
A controller-only action is not a touch test. Real network validation must remove
the mail USB reverse and stop the owned Mac companion before exercising Refresh.
Record only counts, verified transport and cache persistence for a real account.

Use the launcher's opt-in GPU texture readback hook for app-owned captures.
`--record --demo` writes timestamped native PNG frames in the preview's external
files directory. Android WebView is a separate platform surface: capture its own
page through its inspection endpoint. To make a feature video, align the host
and device clocks, record actual touch interactions, and composite WebView frames
at the measured reader bounds relative to the app drawable. Encode with an
already-installed encoder (the macOS AVFoundation APIs also suffice). Mark any
chapter captions as presentation additions. Neither raw layer captures the other.
Use fictional mail for every video frame. Do not use `adb screencap`,
`adb screenrecord`, MediaProjection or other OS capture methods.

Record device/backend, exact source pins, APK hash, checks and limitations.
Stop the owned capture process, remove its inspection forwarding and close the
fixture preview after recording. Retain a real-account session only when the
user requests an open app. A standalone POP3 pass does not imply Android IMAP
sync, attachment previews or verified live SMTP delivery. Keep recordings and
bulk frames local; include only scoped, credential-free evidence in reviews.

## Existing capture/gate compatibility

The current `lab/image-to-appcard/studio.py` and the flow runner's `capture --launch` stage are still Studio-backed legacy capture adapters. They do **not** switch to direct HTTP because this guide exists. Their saved-round gates expect their existing evidence schema. Preserve prior evidence; do not invent Studio IDs or relabel a direct snapshot as a legacy gate pass.

For new direct-instrument app work, run the shared authoring/compile/service stages explicitly, then the project's owned native verifier:

```sh
bash tools/image-to-appcard-flow.sh run \
  --project "$FLOW_PROJECT" --manifest "$FLOW_PROJECT/image-to-appcard-flow.json" \
  --stages semantic,compile,bundle,service-test
```

Do not select legacy `capture --launch` for a request to test without Studio. Keep direct-instrument results alongside the project's evidence with explicit provenance. Adapting the shared capture/gate implementation to consume these results is separate work; until then, report those legacy stages as unrun. Semantic correctness, native interaction, source-image fidelity, visual review, and browser/platform acceptance remain separate decisions.


## Shared runtime and HTML reader checks

All applications select the root `native-runtime.lock.json`; its
Octoscript-Makepad release owns the underlying `runtime.json`. Prepare the
sibling sources with `python3 tools/setup-native.py` and verify the dependency
graph with `--check --cargo-manifest app/Cargo.toml`. Mail and WASM do not apply
private framework patches. Preserve existing edits before updating a checkout.

`apps/mail/scripts/verify_runtime.py` launches an owned hidden native Metal
window and uses the built-in HTTP instrument. `/event?data=<JSON>` dispatches
app-defined probes. The `beauty-host` supports `webview_inspect` (native widget
ID, result path, optional snapshot path and scroll position) and
`webview_lifecycle` (owned browser count). The fixture checks formatted HTML,
script blocking, full reader scrolling, WebView disposal, subject search and a
150-row inbox with bounded native widgets. It captures only the app drawable or
its own WKWebView, finishes through `/gq`, and waits for process exit. It does not
use Studio or a software GPU.
