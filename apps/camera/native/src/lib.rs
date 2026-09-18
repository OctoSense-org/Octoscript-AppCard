//! The Mate 70 Air camera as an in-process OctoSense AppModule. Every screen
//! is a scene built from the camera state (session.rs), compiled to L0
//! (scene.rs) and mounted as real Makepad Kit widgets over a live preview:
//! Makepad's video input (the device camera on macOS/Android; a placeholder
//! where the platform has no camera path). The UX — modes, chrome, panels,
//! gestures — follows docs/ux-map.md; capture and processing are mocked.
use makepad_app_module::{
    makepad_ai_services::wire::{ServiceCall, ServiceManifest, ToolResult},
    AppModule, ExecOutcome, InstanceHandles, InstanceParts, OpenArgKind, OpenSchema, ServiceExecutor, ValidatedOpen,
};
pub use makepad_widgets;
use makepad_widgets::makepad_platform::permission::{Permission, PermissionStatus};
use makepad_widgets::makepad_platform::event::TouchState;
use makepad_widgets::makepad_platform::video::{VideoFormatId, VideoInputId, VideoInputsEvent, VideoPixelFormat};
use makepad_widgets::*;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::io::{Read, Write as _};
use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub mod icons;
pub mod scene;
pub mod session;

script_mod! {
    use mod.prelude.widgets.*
    mod.service_regular = crate_resource("self:resources/service/NotoSansSC-Regular.ttf")
    mod.service_medium = crate_resource("self:resources/service/NotoSansSC-Medium.ttf")
    mod.service_bold = crate_resource("self:resources/service/NotoSansSC-Bold.ttf")
    mod.widgets.CameraView = set_type_default() do #(CameraView::register_widget(vm)) {
        width: Fill height: Fill flow: Overlay
        show_bg: true draw_bg +: { color: #000 }
        preview_host := View {
            width: 100 height: 100 flow: Overlay
            show_bg: true draw_bg +: { color: #3a3a3c }
            preview := Video { width: Fill height: Fill autoplay: false show_controls: false }
        }
        host := Splash {
            width: Fill height: Fill
            padding: 24
            Label { width: Fill draw_text.wrap: Words draw_text.color: #fff text: "Opening Camera…" }
        }
    }
}

/// Who this instance is.
#[derive(Clone, Default)]
pub struct Config { pub locale: String }
impl Config {
    pub fn from_env() -> Self { Config { locale: std::env::var("CAMERA_LOCALE").unwrap_or_else(|_| "cn".into()) } }
}

struct AssetServer { endpoint: String, assets: Arc<Mutex<HashMap<String, String>>>, stop: Arc<AtomicBool> }

impl AssetServer {
    /// The renderer only loads vector art from loopback URLs; serve the scene's icons from memory.
    fn start() -> Result<Self, String> {
        let listener = TcpListener::bind("127.0.0.1:0").map_err(|e| format!("asset server: {e}"))?;
        listener.set_nonblocking(true).map_err(|e| format!("asset server: {e}"))?;
        let token = format!("{:x}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0) ^ (std::process::id() as u128) << 64);
        let endpoint = format!("http://127.0.0.1:{}/{token}", listener.local_addr().map_err(|e| e.to_string())?.port());
        let assets: Arc<Mutex<HashMap<String, String>>> = Arc::default();
        let stop = Arc::new(AtomicBool::new(false));
        let (shared, flag) = (assets.clone(), stop.clone());
        std::thread::Builder::new().name("camera-assets".into()).spawn(move || {
            while !flag.load(Ordering::Acquire) {
                match listener.accept() {
                    Ok((mut socket, _)) => {
                        socket.set_nonblocking(false).ok();
                        socket.set_read_timeout(Some(Duration::from_secs(2))).ok();
                        let mut data = Vec::new();
                        let mut buffer = [0u8; 4096];
                        while !data.windows(4).any(|w| w == b"\r\n\r\n") && data.len() < 32768 {
                            match socket.read(&mut buffer) { Ok(0) | Err(_) => break, Ok(n) => data.extend_from_slice(&buffer[..n]) }
                        }
                        let head = String::from_utf8_lossy(&data);
                        let route = head.lines().next().unwrap_or("").split_whitespace().nth(1).unwrap_or("").to_owned();
                        let body = route.strip_prefix(&format!("/{token}/assets/")).and_then(|name| shared.lock().ok()?.get(name).cloned());
                        let response = match body {
                            Some(svg) => format!("HTTP/1.1 200 OK\r\nContent-Type: image/svg+xml\r\nCache-Control: no-store\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{svg}", svg.len()),
                            None => "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_owned(),
                        };
                        let _ = socket.write_all(response.as_bytes());
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => std::thread::sleep(Duration::from_millis(8)),
                    Err(_) => break,
                }
            }
        }).map_err(|e| e.to_string())?;
        Ok(AssetServer { endpoint, assets, stop })
    }
}

/// One camera the platform offers, with the format we would open it in.
#[derive(Clone, Debug)]
struct CameraChoice { input_id: VideoInputId, format_id: VideoFormatId, name: String, front: bool }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
enum PreviewState { #[default] Idle, Starting, Running, Stopping }

#[derive(Script, ScriptHook, Widget)]
pub struct CameraView {
    #[deref] view: View,
    #[rust] config: Config,
    #[rust] session: Option<session::Session>,
    #[rust] timer: Timer,
    #[rust] started: bool,
    #[rust] assets: Option<AssetServer>,
    #[rust] elements: Vec<Value>,
    #[rust] actions: HashMap<String, Value>,
    #[rust] mapping: HashMap<String, String>,
    #[rust] hit_rects: Vec<(f64, f64, f64, f64)>,
    #[rust] viewport: DVec2,
    #[rust] origin: DVec2,
    #[rust] scale: f64,
    #[rust] art_origin: DVec2,
    #[rust] mounted_viewport: DVec2,
    #[rust] retired: Option<View>,
    #[rust] mounted_revision: Option<u64>,
    #[rust] suppress_activation: bool,
    #[rust] finger_start: Option<DVec2>,
    #[rust] cameras: Vec<CameraChoice>,
    #[rust] permission: Option<PermissionStatus>,
    #[rust] preview: PreviewState,
    #[rust] preview_front: bool,
    #[rust] preview_seen: HashSet<String>,
    #[rust] started_at: f64,
}

fn retire(cx: &mut Cx, widget: &WidgetRef) {
    octoscript_widgets::kit::retire_overlay(cx, widget);
    let mut children = Vec::new();
    widget.children(&mut |_, child| children.push(child));
    for child in children { retire(cx, &child); }
}

fn pick_format(desc: &makepad_widgets::makepad_platform::video::VideoInputDesc) -> Option<VideoFormatId> {
    let rank = |f: &makepad_widgets::makepad_platform::video::VideoFormat| {
        let pixel = match f.pixel_format { VideoPixelFormat::NV12 => 3, VideoPixelFormat::YUY2 => 2, VideoPixelFormat::YUV420 => 1, _ => 0 };
        let size = if f.width > 1920 || f.height > 1080 { 0 } else { f.width * f.height };
        (pixel, size, (f.frame_rate.unwrap_or(0.0) * 100.0) as usize)
    };
    desc.formats.iter().filter(|f| matches!(f.pixel_format, VideoPixelFormat::NV12 | VideoPixelFormat::YUY2 | VideoPixelFormat::YUV420)).max_by_key(|f| rank(f)).map(|f| f.format_id)
}

impl CameraView {
    fn session(&mut self) -> &mut session::Session {
        if self.session.is_none() { self.session = Some(session::Session::new(&self.config.locale)); }
        self.session.as_mut().unwrap()
    }
    fn now(&self) -> f64 { std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0) }

    fn start(&mut self, cx: &mut Cx) {
        self.started = true;
        self.timer = cx.start_interval(0.1);
        match AssetServer::start() { Ok(server) => self.assets = Some(server), Err(error) => log!("camera: {error}") }
        let now = self.now();
        self.started_at = now;
        self.session().now = now;
        if std::env::var("CAMERA_NO_PREVIEW").is_err() {
            cx.request_permission(Permission::Camera);
            cx.video_input(0, |_frame| {});
        }
    }

    /// Open the camera the session wants (rear/front) in the Video widget; one step per call, driven by the platform's events.
    fn drive_preview(&mut self, cx: &mut Cx) {
        let want_front = self.session().front;
        let video = self.view.video(cx, ids!(preview));
        match self.preview {
            PreviewState::Running if want_front != self.preview_front => {
                if self.cameras.iter().any(|c| c.front == want_front) { self.preview = PreviewState::Stopping; video.stop_and_cleanup_resources(cx); }
            }
            PreviewState::Idle => {
                // The capture backend asks for access itself; a missing prompt result (bare binaries
                // on macOS never get the completion delivered) must not keep the viewfinder dark.
                let waited = self.now() - self.started_at;
                match self.permission {
                    Some(PermissionStatus::Granted) => {}
                    Some(_) => return,
                    None if waited < 2.0 => return,
                    None => { log!("camera: no permission result after {waited:.1}s; opening the camera anyway"); self.permission = Some(PermissionStatus::Granted); }
                }
                let Some(choice) = self.cameras.iter().find(|c| c.front == want_front).or(self.cameras.first()).cloned() else { return };
                video.set_camera_preview_mode(cx, VideoCameraPreviewMode::Texture);
                video.set_source_camera(cx, choice.input_id, choice.format_id);
                video.begin_playback(cx);
                self.preview = PreviewState::Starting;
                self.preview_front = choice.front;
                if self.preview_seen.insert(choice.name.clone()) { log!("camera: opening {} ({})", choice.name, if choice.front { "front" } else { "rear" }); }
            }
            _ => {}
        }
    }

    fn place_preview(&mut self, cx: &mut Cx) {
        let (x, y, w, h) = self.session().viewfinder();
        let scale = self.scale;
        let origin = self.art_origin;
        let rect = Rect { pos: dvec2(origin.x + x * scale, origin.y + y * scale), size: dvec2(w * scale, h * scale) };
        let host = self.view.view(cx, ids!(preview_host));
        host.set_walk(cx, Walk { abs_pos: Some(rect.pos), width: Size::Fixed(rect.size.x), height: Size::Fixed(rect.size.y), ..Default::default() });
        host.set_visible(cx, !matches!(self.session().overlay, session::Overlay::Settings(_) | session::Overlay::Gallery));
    }

    fn mount(&mut self, cx: &mut Cx) -> Result<(), String> {
        let base = self.assets.as_ref().map(|a| a.endpoint.clone()).unwrap_or_else(|| "http://127.0.0.1:1/none".into());
        let scene = self.session().render(&base);
        let frame = scene::compile(&scene, "camera-native");
        if let Some(server) = &self.assets { if let Ok(mut map) = server.assets.lock() { map.extend(frame.assets.iter().map(|(k, v)| (k.clone(), v.clone()))); } }
        let report = octoscript_ui_l0::realize(&frame.card, &frame.data, Default::default());
        let root = report.complete_root()?;
        let source = octoscript_ui_l0::kit_pack::lower(root, &frame.pack, &frame.data)?;
        let mut tree = octoscript_makepad::design::prepare(&source)?;
        self.elements = octoscript_makepad::l0::inspectable(&mut tree);
        // Fit the 406×776 artboard to the tile: one uniform scale, centred horizontally, top-aligned (black bars fill the rest, as on the phone).
        let scale = (self.viewport.x / scene::WIDTH).min(self.viewport.y / scene::HEIGHT).clamp(0.3, 4.0);
        let origin = dvec2(self.origin.x + (self.viewport.x - scene::WIDTH * scale) / 2.0, self.origin.y + (self.viewport.y - scene::HEIGHT * scale) / 2.0);
        self.scale = scale;
        self.art_origin = origin;
        self.mounted_viewport = self.viewport;
        self.hit_rects = scene.nodes.iter().filter(|n| n["t"] == "button").filter_map(|n| Some((n["x"].as_f64()?, n["y"].as_f64()?, n["w"].as_f64()?, n["h"].as_f64()?))).collect();
        fn fit(node: &mut octoscript_node::UiNode, scale: f64, origin: DVec2) {
            let a = &mut node.attrs;
            if let Some(v) = &mut a.x { *v = *v * scale + origin.x; }
            if let Some(v) = &mut a.y { *v = *v * scale + origin.y; }
            for value in [&mut a.w, &mut a.h, &mut a.size, &mut a.line_height, &mut a.radius] { if let Some(v) = value { *v *= scale as f32; } }
            for child in &mut node.children { fit(child, scale, origin); }
        }
        fit(&mut tree, scale, origin);
        tree.attrs.x = Some(self.origin.x);
        tree.attrs.y = Some(self.origin.y);
        tree.attrs.w = Some(self.viewport.x as f32);
        tree.attrs.h = Some(self.viewport.y as f32);
        let ui = octoscript_makepad::design::to_makepad_ui(&tree)?;
        let sm = ScriptMod {
            cargo_manifest_path: env!("CARGO_MANIFEST_DIR").into(), module_path: module_path!().into(), file: file!().into(), line: 1, column: 0, values: Vec::new(),
            code: format!("use mod.prelude.widgets.*\nreturn View{{width:Fill height:Fill flow:Overlay {ui}}}"),
        };
        cx.set_key_focus(Area::Empty);
        let view = cx.with_vm(|vm| vm.eval_checked(sm, 2_000_000).map(|value| View::script_from_value(vm, value)).ok_or_else(|| "Camera widget tree rejected".to_owned()))?;
        let host = self.view.widget(cx, ids!(host));
        let mut host = host.borrow_mut::<Splash>().ok_or("Camera host missing")?;
        self.retired = Some(std::mem::replace(&mut host.view, view));
        if let Some(old) = &self.retired { for (_, child) in &old.children { retire(cx, child); } }
        let uid = host.widget_uid();
        let mut children = Vec::new();
        host.children(&mut |id, child| children.push((id, child)));
        drop(host);
        for (id, child) in children { cx.widget_tree_insert_child_deep(uid, id, child); }
        cx.widget_tree_mark_dirty(uid);
        for element in &self.elements {
            if let (Some(id), Some(enabled)) = (element["id"].as_str(), element["enabled"].as_i64()) {
                self.view.widget(cx, &[LiveId::from_str(id)]).set_disabled(cx, enabled == 0);
                self.view.button(cx, &[LiveId::from_str(id)]).set_enabled(cx, enabled != 0);
            }
        }
        self.actions = frame.actions;
        self.mapping = frame.mapping;
        self.mounted_revision = Some(self.session().revision);
        self.suppress_activation = true;
        self.place_preview(cx);
        self.view.redraw(cx);
        Ok(())
    }

    /// The recording timer ticks without remounting the scene.
    fn refresh_timer(&mut self, cx: &mut Cx) {
        if let Some(text) = self.session().exposing_text() {
            if let Some(native) = self.mapping.get("exposing_count").cloned() { self.view.widget(cx, &[LiveId::from_str(&native)]).set_text(cx, &text); self.view.redraw(cx); }
            return;
        }
        if self.session().recording == session::Recording::Off { return; }
        let text = self.session().timer_text();
        if let Some(native) = self.mapping.get("rec_timer").cloned() { self.view.widget(cx, &[LiveId::from_str(&native)]).set_text(cx, &text); self.view.redraw(cx); }
    }

    /// Window → artboard coordinates.
    fn to_artboard(&self, p: DVec2) -> (f64, f64) {
        let scale = if self.scale > 0.0 { self.scale } else { 1.0 };
        ((p.x - self.art_origin.x) / scale, (p.y - self.art_origin.y) / scale)
    }
    fn on_control(&self, x: f64, y: f64) -> bool { self.hit_rects.iter().any(|(rx, ry, rw, rh)| x >= *rx && x <= rx + rw && y >= *ry && y <= ry + rh) }

    fn shutdown(&mut self, cx: &mut Cx) {
        cx.stop_timer(self.timer);
        if let Some(server) = self.assets.take() { server.stop.store(true, Ordering::Release); }
        let video = self.view.video(cx, ids!(preview));
        if !video.is_unprepared() && !video.is_cleaning_up() { video.stop_and_cleanup_resources(cx); }
        for (_, child) in &self.view.children { retire(cx, child); }
    }
}

impl Widget for CameraView {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let result = self.view.draw_walk(cx, scope, walk);
        let rect = self.view.area().rect(cx);
        self.origin = rect.pos;
        if rect.size.x > 100. && rect.size.y > 100. {
            self.viewport = rect.size;
            // The tile grew or shrank since the last mount: refit on the next tick.
            if (self.viewport - self.mounted_viewport).length() > 1.0 && self.mounted_revision.is_some() { self.mounted_revision = None; }
        }
        result
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.started { self.start(cx); }
        match event {
            Event::Actions(actions) => {
                for action in actions {
                    let Some(action) = action.downcast_ref::<WidgetAction>() else { continue };
                    let kit = action.cast::<octoscript_widgets::kit::KitAction>();
                    if std::env::var("CAMERA_TRACE").is_ok() { log!("camera: widget action {:?} kit={:?} suppressed={}", action.widget_uid, kit, self.suppress_activation); }
                    if !matches!(kit, octoscript_widgets::kit::KitAction::Activated) || self.suppress_activation { continue; }
                    let Some(native) = self.elements.iter().filter_map(|e| e["id"].as_str()).find(|id| self.view.widget(cx, &[LiveId::from_str(id)]).widget_uid() == action.widget_uid).map(str::to_owned) else { log!("camera: action from an unmapped widget"); continue };
                    let Some(control) = self.actions.get(&native).cloned() else { log!("camera: no control for {native}"); continue };
                    let Some(id) = control["event"].as_str().map(str::to_owned) else { continue };
                    let now = self.now();
                    self.session().now = now;
                    if self.session().activate(&id) { self.mounted_revision = None; }
                }
            }
            Event::PermissionResult(result) if result.permission == Permission::Camera => {
                log!("camera: permission result {:?}", result.status);
                self.permission = Some(result.status);
                if !matches!(result.status, PermissionStatus::Granted) { log!("camera: permission {:?}", result.status); }
                self.drive_preview(cx);
            }
            Event::VideoInputs(VideoInputsEvent { descs }) => {
                log!("camera: {} video input(s): {:?}", descs.len(), descs.iter().map(|d| d.name.clone()).collect::<Vec<_>>());
                self.cameras = descs.iter().enumerate().filter_map(|(i, d)| {
                    let name = d.name.to_lowercase();
                    let front = name.contains("front") || name.contains("facetime") || name.contains("user") || (descs.len() > 1 && i == 1 && !name.contains("back"));
                    Some(CameraChoice { input_id: d.input_id, format_id: pick_format(d)?, name: d.name.clone(), front })
                }).collect();
                if self.cameras.is_empty() { log!("camera: no usable camera; the viewfinder stays a placeholder"); }
                self.drive_preview(cx);
            }
            Event::VideoPlaybackPrepared(e) => { log!("camera: preview prepared {}x{}", e.video_width, e.video_height); if self.preview == PreviewState::Starting { self.preview = PreviewState::Running; } }
            Event::VideoTextureUpdated(_) => { if self.session().placeholder { self.session().placeholder = false; self.session().revision += 1; self.mounted_revision = None; } }
            Event::VideoPlaybackResourcesReleased(_) => { self.preview = PreviewState::Idle; self.session().placeholder = true; self.session().revision += 1; self.mounted_revision = None; self.drive_preview(cx); }
            Event::VideoDecodingError(e) => { log!("camera: preview error: {}", e.error); self.preview = PreviewState::Idle; }
            Event::KeyDown(KeyEvent { key_code: KeyCode::Escape, .. }) | Event::KeyDown(KeyEvent { key_code: KeyCode::Back, .. }) => {
                if self.session().back() { self.session().revision += 1; self.mounted_revision = None; }
            }
            _ => {}
        }
        if self.timer.is_event(event).is_some() {
            self.suppress_activation = false;
            let now = self.now();
            if self.session().tick(now) { self.mounted_revision = None; }
            let revision = self.session().revision;
            if self.mounted_revision != Some(revision) && self.viewport.x > 0. {
                if let Err(error) = self.mount(cx) { log!("camera: cannot mount: {error}"); self.mounted_revision = Some(revision); }
            } else { self.refresh_timer(cx); }
            self.drive_preview(cx);
        }
        self.view.handle_event(cx, event, scope);
        // Preview gestures come from the raw pointer events (a child — the video
        // widget or a Kit button — may have captured the digit): a swipe changes
        // mode / opens the toolbox, a tap that hits no control focuses.
        let mut ended: Option<DVec2> = None;
        match event {
            Event::MouseDown(e) => { self.finger_start = Some(e.abs); self.suppress_activation = false; }
            Event::MouseUp(e) => ended = Some(e.abs),
            Event::TouchUpdate(t) => {
                for touch in &t.touches {
                    match touch.state {
                        TouchState::Start => { self.finger_start = Some(touch.abs); self.suppress_activation = false; }
                        TouchState::Stop => ended = Some(touch.abs),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        if let (Some(end), Some(start)) = (ended, self.finger_start.take()) {
            let (x0, y0) = self.to_artboard(start);
            let (x1, y1) = self.to_artboard(end);
            let (dx, dy) = (x1 - x0, y1 - y0);
            let now = self.now();
            self.session().now = now;
            if dx.abs() > 40.0 || dy.abs() > 40.0 {
                let from_bottom = y0 > 600.0;
                if self.session().swipe(dx, dy, from_bottom) { self.mounted_revision = None; }
            } else if !self.on_control(x1, y1) && self.session().focus_tap(x1, y1) { self.mounted_revision = None; }
        }
    }
}

pub struct CameraModule;
pub static CAMERA_MODULE: CameraModule = CameraModule;
impl AppModule for CameraModule {
    fn id(&self) -> &'static str { "camera" }
    fn label(&self) -> &'static str { "Camera" }
    fn capabilities(&self) -> &'static [&'static str] { &["camera"] }
    fn open_schema(&self) -> OpenSchema { OpenSchema::new(1).arg("locale", OpenArgKind::Text, false) }
    fn register(&self, vm: &mut ScriptVm) {
        octoscript_widgets::design::script_mod(vm);
        octoscript_widgets::kit::script_mod(vm);
        script_mod(vm);
    }
    fn create(&self, vm: &mut ScriptVm, open: ValidatedOpen, handles: InstanceHandles) -> InstanceParts {
        let value = script_eval!(vm, { use mod.widgets.* CameraView {} });
        let root = WidgetRef::script_from_value(vm, value);
        if let Some(mut camera) = root.borrow_mut::<CameraView>() {
            let mut config = Config::from_env();
            if let Some(locale) = open.text("locale") { config.locale = locale.to_owned(); }
            camera.config = config;
            camera.viewport = handles.viewport.size;
        }
        let cleanup = root.clone();
        InstanceParts { root, executor: Box::new(CameraExecutor), shutdown: Box::new(move |vm| { if let Some(mut camera) = cleanup.borrow_mut::<CameraView>() { camera.shutdown(vm.cx_mut()); } }) }
    }
}
struct CameraExecutor;
impl ServiceExecutor for CameraExecutor {
    fn manifest(&self) -> ServiceManifest { ServiceManifest::new("camera", "Camera", "The camera: modes, capture and the live viewfinder.") }
    fn execute(&mut self, _cx: &mut Cx, call: &ServiceCall) -> ExecOutcome { ExecOutcome::Done(ToolResult::unavailable(&call.call_id, "Use the Camera interface")) }
}
