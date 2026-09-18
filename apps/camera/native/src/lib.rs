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
use makepad_widgets::makepad_platform::video::{CameraCaptureEvent, CameraCaptureRequest, CameraCaptureResult, CameraControl, CameraFlashMode, VideoFormatId, VideoInputId, VideoInputsEvent, VideoPixelFormat};
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

/// content type + bytes, by asset file name
type Assets = HashMap<String, (&'static str, Vec<u8>)>;

struct AssetServer { endpoint: String, assets: Arc<Mutex<Assets>>, stop: Arc<AtomicBool> }

impl AssetServer {
    /// The renderer only loads vector art from loopback URLs; serve the scene's icons from memory.
    fn start() -> Result<Self, String> {
        let listener = TcpListener::bind("127.0.0.1:0").map_err(|e| format!("asset server: {e}"))?;
        listener.set_nonblocking(true).map_err(|e| format!("asset server: {e}"))?;
        let token = format!("{:x}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0) ^ (std::process::id() as u128) << 64);
        let endpoint = format!("http://127.0.0.1:{}/{token}", listener.local_addr().map_err(|e| e.to_string())?.port());
        let assets: Arc<Mutex<Assets>> = Arc::default();
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
                        match body {
                            Some((content_type, bytes)) => {
                                let head = format!("HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nCache-Control: no-store\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", bytes.len());
                                let _ = socket.write_all(head.as_bytes()).and_then(|_| socket.write_all(&bytes));
                            }
                            None => { let _ = socket.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"); }
                        }
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
struct CameraChoice { input_id: VideoInputId, name: String, front: bool, formats: Vec<makepad_widgets::makepad_platform::video::VideoFormat> }

impl CameraChoice {
    /// The best preview profile for a viewfinder aspect (width/height of the
    /// sensor frame: 4:3 for the photo modes, 16:9 for video), largest first
    /// within 1080p.
    fn format_for(&self, aspect: f64) -> Option<(VideoFormatId, usize, usize)> {
        let usable = |f: &&makepad_widgets::makepad_platform::video::VideoFormat| matches!(f.pixel_format, VideoPixelFormat::NV12 | VideoPixelFormat::YUY2 | VideoPixelFormat::YUV420) && f.width <= 1920 && f.height <= 1080 && f.height > 0;
        let pixel = |f: &makepad_widgets::makepad_platform::video::VideoFormat| match f.pixel_format { VideoPixelFormat::NV12 => 3, VideoPixelFormat::YUY2 => 2, VideoPixelFormat::YUV420 => 1, _ => 0 };
        let best = self.formats.iter().filter(usable).min_by(|a, b| {
            let da = ((a.width as f64 / a.height as f64) - aspect).abs(); let db = ((b.width as f64 / b.height as f64) - aspect).abs();
            // nearest aspect (within 2 %), then the most pixels, then the better pixel format
            let key = |d: f64, f: &makepad_widgets::makepad_platform::video::VideoFormat| ((d * 50.0).round() as i64, -((f.width * f.height) as i64), -pixel(f));
            key(da, a).cmp(&key(db, b))
        })?;
        Some((best.format_id, best.width, best.height))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
enum PreviewState { #[default] Idle, Starting, Running, Stopping }

/// A critically-damped-ish spring in artboard units (HarmonyOS-style
/// `springMotion`: ~0.42 s response, 0.9 damping fraction), stepped per frame.
#[derive(Default, Clone, Copy)]
struct Spring { pos: f64, vel: f64, last_t: Option<f64> }
impl Spring {
    fn active(&self) -> bool { self.pos.abs() > 0.05 || self.vel.abs() > 1.0 }
    /// Start from `from` and settle at 0.
    fn kick(&mut self, from: f64) { self.pos = from; self.last_t = None; }
    fn hold(&mut self, at: f64) { self.pos = at; self.vel = 0.0; self.last_t = None; }
    fn step(&mut self, t: f64) {
        let dt = self.last_t.map(|l| (t - l).clamp(0.0, 0.05)).unwrap_or(0.0);
        self.last_t = Some(t);
        let w = std::f64::consts::TAU / 0.42; let z = 0.9; let (k, c) = (w * w, 2.0 * z * w);
        let h = dt / 4.0;
        for _ in 0..4 { let a = -k * self.pos - c * self.vel; self.vel += a * h; self.pos += self.vel * h; }
        if !self.active() { self.pos = 0.0; self.vel = 0.0; }
    }
}

/// A finger sliding the mode bar: where it started, the bar index at that time and the last velocity sample.
#[derive(Clone, Copy)]
struct BarDrag { start_x: f64, index0: usize, last_x: f64, last_t: f64, velocity: f64, moved: bool }

const MODE_PITCH: f64 = 52.3;
const CHIP_PITCH: f64 = 40.0;

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
    #[rust] preview_aspect: f64,
    #[rust] preview_seen: HashSet<String>,
    #[rust] started_at: f64,
    // motion: the mode strip and the zoom chip slide with a spring; a finger can drag the bar
    #[rust] next_frame: NextFrame,
    #[rust] bar: Spring,
    #[rust] chip: Spring,
    #[rust] bar_drag: Option<BarDrag>,
    #[rust] strips: HashMap<String, (f64, f64, f64, f64)>,
    #[rust] pinch: Option<(f64, f32)>,
    #[rust] finger_last: Option<DVec2>,
    // the real camera: the open input and what was last sent to it
    #[rust] camera_input: Option<VideoInputId>,
    #[rust] zoom_ratio: f32,
    #[rust] sent_zoom: Option<f32>,
    #[rust] sent_flash: Option<session::Flash>,
    #[rust] sent_ev: Option<f32>,
    #[rust] mic: Option<PermissionStatus>,
    #[rust] mic_asked: bool,
    #[rust] recording_path: Option<String>,
    // a finger dragging the focus box: when the real focus point was last sent
    #[rust] focus_drag: bool,
    #[rust] focus_sent_at: f64,
}

fn retire(cx: &mut Cx, widget: &WidgetRef) {
    octoscript_widgets::kit::retire_overlay(cx, widget);
    let mut children = Vec::new();
    widget.children(&mut |_, child| children.push(child));
    for child in children { retire(cx, &child); }
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
        let (want_front, want_aspect) = { let s = self.session(); (s.front, if s.mode.wide() { 16.0 / 9.0 } else { 4.0 / 3.0 }) };
        let video = self.view.video(cx, ids!(preview));
        match self.preview {
            // Another camera or another viewfinder shape (4:3 photo vs 16:9 video): reopen with the matching profile.
            PreviewState::Running if want_front != self.preview_front || (want_aspect - self.preview_aspect).abs() > 0.05 => {
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
                let Some((format_id, w, h)) = choice.format_for(want_aspect) else { return };
                video.set_camera_preview_mode(cx, VideoCameraPreviewMode::Texture);
                video.set_source_camera(cx, choice.input_id, format_id);
                video.begin_playback(cx);
                self.preview = PreviewState::Starting;
                self.preview_front = choice.front;
                self.preview_aspect = want_aspect;
                log!("camera: preview profile {w}x{h} for a {:.2} viewfinder", want_aspect);
                self.camera_input = Some(choice.input_id);
                self.sent_zoom = None; self.sent_flash = None; self.sent_ev = None;
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
        if let Some(server) = &self.assets { if let Ok(mut map) = server.assets.lock() { map.extend(frame.assets.iter().map(|(k, v)| (k.clone(), ("image/svg+xml", v.clone().into_bytes())))); } }
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
        self.hit_rects = scene.button_rects();
        self.strips = scene.nodes.iter().filter(|n| n["variant"] == "scroll_y").filter_map(|n| Some((n["id"].as_str()?.to_owned(), (n["x"].as_f64()?, n["y"].as_f64()?, n["w"].as_f64()?, n["h"].as_f64()?)))).collect();
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
        let mut ui = octoscript_makepad::design::to_makepad_ui(&tree)?;
        // The sliding strips are scroll nodes only so their children are laid out
        // relative to them; as widgets they are plain clipped overlays that the
        // module moves as a whole (a scrolling view would stack the labels).
        for strip in ["mode_strip", "zoom_chip_strip"] {
            if let Some(native) = frame.mapping.get(strip) {
                ui = ui.replace(&format!("{native} := ScrollYView {{"), &format!("{native} := View {{\nflow: Overlay clip_x: true clip_y: true"));
            }
        }
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
        self.apply_motion(cx);
        self.view.redraw(cx);
        Ok(())
    }

    /// Put the sliding strips where the springs (or the finger) say, in place, without a remount.
    fn apply_motion(&mut self, cx: &mut Cx) {
        for (strip, offset) in [("mode_strip", self.bar.pos), ("zoom_chip_strip", self.chip.pos)] {
            let (Some(&(x, y, w, h)), Some(native)) = (self.strips.get(strip), self.mapping.get(strip).cloned()) else { continue };
            let pos = dvec2(self.art_origin.x + (x + offset) * self.scale, self.art_origin.y + y * self.scale);
            self.view.view(cx, &[LiveId::from_str(&native)]).set_walk(cx, Walk { abs_pos: Some(pos), width: Size::Fixed(w * self.scale), height: Size::Fixed(h * self.scale), ..Default::default() });
        }
        if self.bar.active() || self.chip.active() || self.bar_drag.is_some() { self.next_frame = cx.new_next_frame(); }
        self.view.redraw(cx);
    }

    /// The session changed: animate the bar/chip from where they were and push the new lens, flash and EV to the camera.
    fn after_change(&mut self, cx: &mut Cx, before: (Option<usize>, usize, f64)) {
        let (old_bar, old_zoom, drag_offset) = before;
        let (new_bar, new_zoom) = { let s = self.session(); (s.mode_bar_index(), s.zoom_index()) };
        match (old_bar, new_bar) {
            (Some(a), Some(b)) if a != b => self.bar.kick((b as f64 - a as f64) * MODE_PITCH + drag_offset),
            (Some(_), Some(_)) if drag_offset != 0.0 => self.bar.kick(drag_offset),
            _ => self.bar.hold(0.0),
        }
        if old_bar == new_bar && old_zoom != new_zoom { self.chip.kick((old_zoom as f64 - new_zoom as f64) * CHIP_PITCH); } else if old_bar != new_bar { self.chip.hold(0.0); }
        self.zoom_ratio = self.session().zoom_ratio();
        self.sync_camera(cx);
        if self.session().mode.is_video() && !self.mic_asked { self.mic_asked = true; cx.request_permission(Permission::AudioInput); }
    }

    /// The shutter in a video mode: the session already flipped its recording state; drive the platform recorder to match.
    fn toggle_recording(&mut self, cx: &mut Cx) {
        let Some(input) = self.camera_input else { return };
        if self.preview != PreviewState::Running { return; }
        if self.session().recording != session::Recording::Off {
            let Some(dir) = cx.get_data_dir() else { return };
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
            let path = format!("{dir}/DCIM/VID_{now}.mp4");
            self.recording_path = Some(path.clone());
            let audio = matches!(self.mic, Some(PermissionStatus::Granted));
            cx.camera_capture(input, CameraCaptureRequest::StartVideo { path, audio, library: true });
        } else if self.recording_path.take().is_some() {
            cx.camera_capture(input, CameraCaptureRequest::StopVideo);
        }
    }

    /// Send what the UI state asks of the real camera, only when it changed.
    fn sync_camera(&mut self, cx: &mut Cx) {
        let Some(input) = self.camera_input else { return };
        if self.preview != PreviewState::Running { return; }
        let (flash, ev) = { let s = self.session(); (s.flash, s.ev_bias()) };
        if self.sent_zoom != Some(self.zoom_ratio) { self.sent_zoom = Some(self.zoom_ratio); cx.camera_control(input, CameraControl::ZoomRatio(self.zoom_ratio)); }
        if self.sent_flash != Some(flash) {
            self.sent_flash = Some(flash);
            let mode = match flash { session::Flash::Off => CameraFlashMode::Off, session::Flash::On => CameraFlashMode::On, session::Flash::Auto => CameraFlashMode::Auto, session::Flash::Torch => CameraFlashMode::Torch };
            cx.camera_control(input, CameraControl::Flash(mode));
        }
        if self.sent_ev != Some(ev) { self.sent_ev = Some(ev); cx.camera_control(input, CameraControl::ExposureBias(ev)); }
    }

    /// The shutter in a still mode: ask the platform camera for a JPEG next to the app's data and hand it to the gallery.
    fn take_photo(&mut self, cx: &mut Cx) {
        let Some(input) = self.camera_input else { log!("camera: shutter without an open camera (mock capture)"); return };
        if self.preview != PreviewState::Running { return; }
        let Some(dir) = cx.get_data_dir() else { log!("camera: no data directory for captures"); return };
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        let path = format!("{dir}/DCIM/IMG_{now}.jpg");
        cx.camera_capture(input, CameraCaptureRequest::Photo { path, library: true });
    }

    /// A capture finished (or failed) on the platform camera.
    fn on_capture(&mut self, cx: &mut Cx, result: &CameraCaptureResult) {
        match result {
            CameraCaptureResult::Photo { path, width, height } => {
                log!("camera: photo {path} ({width}x{height})");
                match std::fs::read(path) {
                    Ok(bytes) => {
                        let name = format!("thumb_{}.jpg", self.session().shots);
                        if let Some(server) = &self.assets { if let Ok(mut map) = server.assets.lock() { map.retain(|k, _| !k.starts_with("thumb_")); map.insert(name.clone(), ("image/jpeg", bytes)); } }
                        self.session().last_photo = Some(name);
                        self.session().revision += 1;
                        self.remount_now(cx);
                    }
                    Err(e) => log!("camera: cannot read {path}: {e}"),
                }
            }
            CameraCaptureResult::SavedToLibrary { path, uri } => log!("camera: gallery {} {path}", if uri.is_some() { "took" } else { "refused" }),
            CameraCaptureResult::VideoStarted { path } => log!("camera: recording {path}"),
            CameraCaptureResult::VideoStopped { path } => log!("camera: recording saved {path}"),
            CameraCaptureResult::Failed { what, error } => {
                log!("camera: {what} failed: {error}");
                if what == "video" && self.session().recording != session::Recording::Off { self.session().recording = session::Recording::Off; self.recording_path = None; }
                let text = { let en = self.config.locale == "en"; if en { format!("Capture failed: {error}") } else { format!("拍摄失败：{error}") } };
                self.session().toast = Some((text, self.now() + 2.5)); self.session().revision += 1; self.remount_now(cx);
            }
            other => log!("camera: capture {other:?}"),
        }
    }

    /// Slide the focus reticle under the finger and re-aim the real camera (throttled).
    fn drag_focus(&mut self, cx: &mut Cx, x: f64, y: f64, done: bool) {
        if !self.session().move_focus(x, y) { return; }
        let (bx, by) = self.session().focus_box(x, y);
        if let (Some(&(_, _, w, h)), Some(native)) = (self.strips.get("focus_strip"), self.mapping.get("focus_strip").cloned()) {
            let pos = dvec2(self.art_origin.x + (bx - 32.0) * self.scale, self.art_origin.y + by * self.scale);
            self.view.view(cx, &[LiveId::from_str(&native)]).set_walk(cx, Walk { abs_pos: Some(pos), width: Size::Fixed(w * self.scale), height: Size::Fixed(h * self.scale), ..Default::default() });
            self.view.redraw(cx);
        }
        let now = self.now();
        if done || now - self.focus_sent_at > 0.08 {
            self.focus_sent_at = now;
            let (_, vy, vw, vh) = self.session().viewfinder();
            if let Some(input) = self.camera_input { cx.camera_control(input, CameraControl::FocusPoint { x: (x / vw).clamp(0.0, 1.0), y: ((y - vy) / vh).clamp(0.0, 1.0) }); }
        }
    }

    fn on_mode_bar(&mut self, x: f64, y: f64) -> bool {
        let s = self.session();
        (576.6..=632.6).contains(&y) && (0.0..=406.0).contains(&x) && s.overlay == session::Overlay::None && s.mode_bar_index().is_some() && s.recording == session::Recording::Off
    }

    /// The finger left the mode bar: pick the entry it points at (with a little fling) and let the spring settle.
    fn end_bar_drag(&mut self, cx: &mut Cx, drag: BarDrag, x: f64) {
        let offset = self.bar.pos;
        if !drag.moved { self.bar.hold(0.0); return; }
        let _ = x;
        let projected = offset + drag.velocity * 0.12;
        let target = (drag.index0 as f64 - projected / MODE_PITCH).round().clamp(0.0, (session::Mode::BAR.len() - 1) as f64) as usize;
        let before = (Some(drag.index0), self.session().zoom_index(), offset);
        let now = self.now(); self.session().now = now;
        self.suppress_activation = true;
        let selected = self.session().select_mode_index(target);
        self.after_change(cx, before);
        if selected { self.remount_now(cx); } else { self.apply_motion(cx); }
    }

    /// Mount the new scene right away (instead of at the next 0.1 s tick) so a
    /// spring starts from the new layout rather than sliding the old one first.
    fn remount_now(&mut self, cx: &mut Cx) {
        self.mounted_revision = None;
        if self.viewport.x > 0. {
            if let Err(error) = self.mount(cx) { log!("camera: cannot mount: {error}"); self.mounted_revision = Some(self.session().revision); }
        }
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
    /// Like `on_control`, ignoring the viewfinder-wide scrims some overlays put under their chrome.
    fn on_small_control(&self, x: f64, y: f64) -> bool { self.hit_rects.iter().any(|(rx, ry, rw, rh)| *rw < 300.0 && x >= *rx && x <= rx + rw && y >= *ry && y <= ry + rh) }

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
                    let before = { let s = self.session(); (s.mode_bar_index(), s.zoom_index(), 0.0) };
                    let (was_video, was_exposing) = { let s = self.session(); (s.mode.is_video(), matches!(s.overlay, session::Overlay::Exposing { .. })) };
                    let changed = self.session().activate(&id);
                    self.after_change(cx, before);
                    if id == "shutter" && !was_video && !was_exposing { self.take_photo(cx); }
                    if id == "shutter" && was_video { self.toggle_recording(cx); }
                    if id == "rec_pause" && was_video {
                        let paused = matches!(self.session().recording, session::Recording::Paused { .. });
                        if let Some(input) = self.camera_input { cx.camera_capture(input, if paused { CameraCaptureRequest::PauseVideo } else { CameraCaptureRequest::ResumeVideo }); }
                    }
                    if changed { self.remount_now(cx); }
                }
                for action in actions {
                    let Some(capture) = action.downcast_ref::<CameraCaptureEvent>() else { continue };
                    self.on_capture(cx, &capture.result);
                }
            }
            Event::PermissionResult(result) if result.permission == Permission::AudioInput => {
                log!("camera: microphone permission {:?}", result.status);
                self.mic = Some(result.status);
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
                    let choice = CameraChoice { input_id: d.input_id, name: d.name.clone(), front, formats: d.formats.clone() };
                    choice.format_for(4.0 / 3.0)?;
                    Some(choice)
                }).collect();
                if self.cameras.is_empty() { log!("camera: no usable camera; the viewfinder stays a placeholder"); }
                self.drive_preview(cx);
            }
            Event::VideoPlaybackPrepared(e) => {
                log!("camera: preview prepared {}x{}", e.video_width, e.video_height);
                if self.preview == PreviewState::Starting { self.preview = PreviewState::Running; }
                self.zoom_ratio = self.session().zoom_ratio();
                self.sync_camera(cx);
            }
            Event::VideoTextureUpdated(_) => { if self.session().placeholder { self.session().placeholder = false; self.session().revision += 1; self.mounted_revision = None; } }
            // The Video widget sees this event after us and only then leaves CleaningUp, so the
            // next preview is opened from the timer tick rather than here (front/rear switch on OHOS).
            Event::VideoPlaybackResourcesReleased(_) => { self.preview = PreviewState::Idle; self.session().placeholder = true; self.session().revision += 1; self.mounted_revision = None; }
            Event::VideoDecodingError(e) => { log!("camera: preview error: {}", e.error); self.preview = PreviewState::Idle; }
            Event::KeyDown(KeyEvent { key_code: KeyCode::Escape, .. }) | Event::KeyDown(KeyEvent { key_code: KeyCode::Back, .. }) => {
                if self.session().back() { self.session().revision += 1; self.mounted_revision = None; }
            }
            _ => {}
        }
        if let Some(frame) = self.next_frame.is_event(event) {
            if self.bar_drag.is_none() { self.bar.step(frame.time); }
            self.chip.step(frame.time);
            self.apply_motion(cx);
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
        let mut moved: Option<DVec2> = None;
        match event {
            Event::MouseDown(e) => { if std::env::var("CAMERA_TRACE").is_ok() { log!("camera: pointer down at {:?}", e.abs); } self.finger_start = Some(e.abs); self.finger_last = Some(e.abs); self.suppress_activation = false; }
            Event::MouseMove(e) if self.finger_start.is_some() => moved = Some(e.abs),
            Event::MouseUp(e) => ended = Some(e.abs),
            Event::TouchUpdate(t) => {
                // Two fingers on the viewfinder: pinch to zoom the real lens; the selected chip shows the live value.
                if t.touches.len() >= 2 && self.preview == PreviewState::Running {
                    let (a, b) = (t.touches[0].abs, t.touches[1].abs);
                    let dist = (a - b).length().max(1.0);
                    if t.touches.iter().any(|p| p.state == TouchState::Stop) { self.pinch = None; }
                    else {
                        let (d0, r0) = *self.pinch.get_or_insert((dist, self.zoom_ratio));
                        let ratio = (r0 * (dist / d0) as f32).clamp(0.5, 10.0);
                        if (ratio - self.zoom_ratio).abs() > 0.02 {
                            self.zoom_ratio = ratio;
                            if let Some(input) = self.camera_input { cx.camera_control(input, CameraControl::ZoomRatio(ratio)); self.sent_zoom = Some(ratio); }
                            let items = { let s = self.session(); s.mode.zooms(s.front) };
                            let sel = self.session().zoom_index().min(items.len().saturating_sub(1));
                            if let Some(label) = items.get(sel) {
                                if let Some(native) = self.mapping.get(&format!("zoom_{}_label", label.trim_end_matches('x').to_lowercase())).cloned() {
                                    self.view.widget(cx, &[LiveId::from_str(&native)]).set_text(cx, &format!("{ratio:.1}x"));
                                    self.view.redraw(cx);
                                }
                            }
                        }
                    }
                    self.finger_start = None; self.bar_drag = None;
                } else {
                    for touch in &t.touches {
                        match touch.state {
                            TouchState::Start => { self.finger_start = Some(touch.abs); self.finger_last = Some(touch.abs); self.suppress_activation = false; }
                            TouchState::Move => moved = Some(touch.abs),
                            TouchState::Stop => { ended = Some(touch.abs); self.pinch = None; }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
        // A finger on the mode bar drags the strip; it snaps to an entry on release.
        if let (Some(p), Some(start)) = (moved, self.finger_start) {
            let (x, y) = self.to_artboard(p);
            let (x0, y0) = self.to_artboard(start);
            let t = self.now();
            // With the focus box up, the finger drags it (and the camera's focus point) instead of swiping modes.
            let focus_up = matches!(self.session().overlay, session::Overlay::Focus(..));
            if self.bar_drag.is_none() && focus_up && !self.on_small_control(x0, y0) && (self.focus_drag || (x - x0).hypot(y - y0) > 6.0) {
                self.focus_drag = true;
                self.drag_focus(cx, x, y, false);
            }
            if self.bar_drag.is_none() && !self.focus_drag && self.on_mode_bar(x0, y0) && (x - x0).abs() > 4.0 && (x - x0).abs() > (y - y0).abs() {
                let index0 = self.session().mode_bar_index().unwrap_or(3);
                self.bar_drag = Some(BarDrag { start_x: x0, index0, last_x: x, last_t: t, velocity: 0.0, moved: false });
            }
            if let Some(mut drag) = self.bar_drag {
                let raw = x - drag.start_x;
                let (lo, hi) = ((drag.index0 as f64 - (session::Mode::BAR.len() - 1) as f64) * MODE_PITCH, drag.index0 as f64 * MODE_PITCH);
                let offset = if raw < lo { lo + (raw - lo) * 0.3 } else if raw > hi { hi + (raw - hi) * 0.3 } else { raw };
                let dt = (t - drag.last_t).max(1e-3);
                drag.velocity = 0.6 * drag.velocity + 0.4 * (x - drag.last_x) / dt;
                drag.last_x = x; drag.last_t = t; drag.moved = drag.moved || raw.abs() > 8.0;
                if drag.moved { self.suppress_activation = true; }
                self.bar_drag = Some(drag);
                self.bar.hold(offset);
                self.apply_motion(cx);
            }
            self.finger_last = Some(p);
        }
        if ended.is_some() && std::env::var("CAMERA_TRACE").is_ok() { log!("camera: pointer up at {:?} start {:?}", ended, self.finger_start); }
        if let Some(end) = ended {
            if self.focus_drag {
                self.focus_drag = false;
                let (x, y) = self.to_artboard(end);
                self.drag_focus(cx, x, y, true);
                self.finger_start = None;
            }
            // (only take the drag on a release: evaluating it for every event would drop it mid-drag)
            if let Some(drag) = self.bar_drag.take() {
                let (x, _) = self.to_artboard(end);
                self.finger_start = None;
                self.end_bar_drag(cx, drag, x);
            }
        }
        if let (Some(end), Some(start)) = (ended, ended.and_then(|_| self.finger_start.take())) {
            let (x0, y0) = self.to_artboard(start);
            let (x1, y1) = self.to_artboard(end);
            let (dx, dy) = (x1 - x0, y1 - y0);
            if std::env::var("CAMERA_TRACE").is_ok() { let (overlay, mode) = { let s = self.session(); (s.overlay.clone(), s.mode) }; log!("camera: gesture dx={dx:.0} dy={dy:.0} from ({x0:.0},{y0:.0}) overlay={overlay:?} mode={mode:?}"); }
            let now = self.now();
            self.session().now = now;
            let before = { let s = self.session(); (s.mode_bar_index(), s.zoom_index(), 0.0) };
            if dx.abs() > 40.0 || dy.abs() > 40.0 {
                let from_bottom = y0 > 600.0;
                if self.session().swipe(dx, dy, from_bottom) { self.mounted_revision = None; }
                self.after_change(cx, before);
            } else if !self.on_control(x1, y1) && self.session().focus_tap(x1, y1) {
                self.mounted_revision = None;
                let (_, vy, vw, vh) = self.session().viewfinder();
                if let Some(input) = self.camera_input { cx.camera_control(input, CameraControl::FocusPoint { x: (x1 / vw).clamp(0.0, 1.0), y: ((y1 - vy) / vh).clamp(0.0, 1.0) }); }
            }
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
