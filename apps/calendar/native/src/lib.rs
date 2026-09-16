//! The Calendar as an in-process OctoSense AppModule. Every screen is a scene
//! built from the calendar state (session.rs), compiled to L0 (scene.rs) and
//! mounted as real Makepad Kit widgets — the same lowering the reviewed
//! storyboard cards use. The module keeps a replica of the calendar service's
//! log (model.rs) and, when a server is configured, syncs it (sync.rs).
use makepad_app_module::{
    makepad_ai_services::wire::{ServiceCall, ServiceManifest, ToolResult},
    AppModule, ExecOutcome, InstanceHandles, InstanceParts, OpenArgKind, OpenSchema, ServiceExecutor, ValidatedOpen,
};
pub use makepad_widgets;
use makepad_widgets::*;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::io::{Read, Write as _};
use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub mod icons;
pub mod model;
pub mod scene;
pub mod session;
pub mod sync;

script_mod! {
    use mod.prelude.widgets.*
    mod.service_regular = crate_resource("self:resources/service/NotoSansSC-Regular.ttf")
    mod.service_bold = crate_resource("self:resources/service/NotoSansSC-Bold.ttf")
    mod.widgets.CalendarView = set_type_default() do #(CalendarView::register_widget(vm)) {
        width: Fill height: Fill flow: Overlay
        show_bg: true draw_bg.color: #fff
        host := Splash {
            width: Fill height: Fill
            padding: 24
            Label { width: Fill draw_text.wrap: Words text: "Opening Calendar…" }
        }
    }
}

/// Where this instance finds its service and who it is to the service.
#[derive(Clone, Default)]
pub struct Config { pub server: Option<String>, pub token: String, pub device: String, pub locale: String }

impl Config {
    pub fn from_env() -> Self {
        let device = std::env::var("CALENDAR_DEVICE").unwrap_or_else(|_| if cfg!(any(target_os = "android", target_os = "ios")) { "alex-phone".into() } else { "sam-desktop".into() });
        Config { server: std::env::var("CALENDAR_SERVER").ok().filter(|s| !s.is_empty()), token: std::env::var("CALENDAR_TOKEN").unwrap_or_default(), device, locale: std::env::var("CALENDAR_LOCALE").unwrap_or_else(|_| "en".into()) }
    }
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
        std::thread::Builder::new().name("calendar-assets".into()).spawn(move || {
            while !flag.load(Ordering::Acquire) {
                match listener.accept() {
                    Ok((mut socket, _)) => {
                        socket.set_read_timeout(Some(Duration::from_secs(2))).ok();
                        // Read until the request head is complete; the first packet may hold only part of it.
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

#[derive(Script, ScriptHook, Widget)]
pub struct CalendarView {
    #[deref] view: View,
    #[rust] config: Config,
    #[rust] session: Option<session::Session>,
    #[rust] timer: Timer,
    #[rust] started: bool,
    #[rust] assets: Option<AssetServer>,
    #[rust] link: Option<(Sender<sync::Command>, Receiver<sync::Reply>)>,
    #[rust] sent: HashSet<String>,
    #[rust] elements: Vec<Value>,
    #[rust] actions: HashMap<String, Value>,
    #[rust] mapping: HashMap<String, String>,
    #[rust] viewport: DVec2,
    #[rust] origin: DVec2,
    #[rust] retired: Option<View>,
    #[rust] mounted_revision: Option<u64>,
    #[rust] suppress_activation: bool,
}

fn retire(cx: &mut Cx, widget: &WidgetRef) {
    octoscript_widgets::kit::retire_overlay(cx, widget);
    let mut children = Vec::new();
    widget.children(&mut |_, child| children.push(child));
    for child in children { retire(cx, &child); }
}

impl CalendarView {
    fn session(&mut self) -> &mut session::Session {
        if self.session.is_none() { self.session = Some(session::Session::new(&self.config.device, &self.config.locale)); }
        self.session.as_mut().unwrap()
    }

    fn start(&mut self, cx: &mut Cx) {
        self.started = true;
        self.timer = cx.start_interval(0.1);
        match AssetServer::start() { Ok(server) => self.assets = Some(server), Err(error) => log!("calendar: {error}") }
        let config = self.config.clone();
        let session = self.session();
        session.server = config.server.clone();
        if let Some(url) = &config.server {
            match sync::parse_endpoint(url, &config.token) {
                Some(endpoint) => {
                    let (commands, command_rx) = channel();
                    let (reply_tx, replies) = channel();
                    if std::thread::Builder::new().name("calendar-sync".into()).spawn(move || sync::run(endpoint, command_rx, reply_tx)).is_ok() {
                        self.link = Some((commands, replies));
                    }
                }
                None => session.last_error = Some(format!("bad CALENDAR_SERVER: {url}")),
            }
        }
    }

    fn mount(&mut self, cx: &mut Cx) -> Result<(), String> {
        let base = self.assets.as_ref().map(|a| a.endpoint.clone()).unwrap_or_else(|| "http://127.0.0.1:1/none".into());
        let scene = self.session().render(&base);
        let frame = scene::compile(&scene, "calendar-native");
        if let Some(server) = &self.assets { if let Ok(mut map) = server.assets.lock() { map.extend(frame.assets.iter().map(|(k, v)| (k.clone(), v.clone()))); } }
        let report = octoscript_ui_l0::realize(&frame.card, &frame.data, Default::default());
        let root = report.complete_root()?;
        let source = octoscript_ui_l0::kit_pack::lower(root, &frame.pack, &frame.data)?;
        let mut tree = octoscript_makepad::design::prepare(&source)?;
        self.elements = octoscript_makepad::l0::inspectable(&mut tree);
        // Fit the 406×716 logical page to the tile the launcher gave us: one
        // uniform scale (text keeps its proportions), centred horizontally.
        let scale = (self.viewport.x / scene::WIDTH).min(self.viewport.y / scene::HEIGHT).clamp(0.5, 3.0);
        let origin = dvec2(self.origin.x + (self.viewport.x - scene::WIDTH * scale) / 2.0, self.origin.y);
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
        let view = cx.with_vm(|vm| vm.eval_checked(sm, 2_000_000).map(|value| View::script_from_value(vm, value)).ok_or_else(|| "Calendar widget tree rejected".to_owned()))?;
        let host = self.view.widget(cx, ids!(host));
        let mut host = host.borrow_mut::<Splash>().ok_or("Calendar host missing")?;
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
        self.view.redraw(cx);
        Ok(())
    }

    /// Editor text changed: keep the caret, refresh only what depends on the draft.
    fn refresh_editor(&mut self, cx: &mut Cx) {
        let (valid, hint) = {
            let session = self.session();
            let Some(draft) = session.draft.clone() else { return };
            let conflict = session.state().conflict(&model::iso(draft.start), &model::iso(draft.end), if draft.editing { Some(&draft.id) } else { None }).map(|e| e.title.get(&session.locale).to_owned());
            let valid = !draft.title.trim().is_empty() && conflict.is_none() && (draft.all_day || draft.end > draft.start);
            let locale = session.locale.clone();
            let hint = match conflict { Some(other) => if locale == "en" { format!("Overlaps “{other}” — choose another time") } else { format!("与「{other}」重叠，请更换时间") },
                None if draft.title.trim().is_empty() => if locale == "en" { "Add a title".into() } else { "请输入标题".into() },
                None => if session.connected { if locale == "en" { "Saving syncs this to the other device".into() } else { "保存后会同步到其他设备".into() } } else if locale == "en" { "Not connected: saved on this device".into() } else { "未连接服务器：保存后记录在本机".into() } };
            (valid, hint)
        };
        if let Some(native) = self.mapping.get("save_event_control").cloned() {
            self.view.button(cx, &[LiveId::from_str(&native)]).set_enabled(cx, valid);
            self.view.widget(cx, &[LiveId::from_str(&native)]).set_disabled(cx, !valid);
        }
        if let Some(native) = self.mapping.get("hint_text").cloned() { self.view.widget(cx, &[LiveId::from_str(&native)]).set_text(cx, &hint); }
        self.view.redraw(cx);
    }

    fn pump_sync(&mut self) {
        let Some((commands, replies)) = self.link.as_ref() else { return };
        let mut received = Vec::new();
        while let Ok(reply) = replies.try_recv() { received.push(reply); }
        let unsent: Vec<Value> = self.session.as_ref().map(|s| s.pending.iter().filter(|op| !self.sent.contains(&op.id)).map(|op| serde_json::to_value(op).unwrap()).collect()).unwrap_or_default();
        for op in unsent {
            if let Some(id) = op["id"].as_str() { self.sent.insert(id.to_owned()); }
            let _ = commands.send(sync::Command::Submit(op));
        }
        let session = self.session();
        for reply in received {
            match reply {
                sync::Reply::Snapshot { seq, state } => match serde_json::from_value::<model::State>(state) {
                    Ok(state) => session.adopt_snapshot(seq, state),
                    Err(e) => session.last_error = Some(format!("snapshot: {e}")),
                },
                sync::Reply::Records(records) => session.apply_records(&records),
                sync::Reply::Submitted(record) => session.apply_records(&[record]),
                sync::Reply::Error(error) => { if session.last_error.as_deref() != Some(&error) { log!("calendar: sync: {error}"); } session.last_error = Some(error); session.revision += 1; }
            }
        }
    }

    fn shutdown(&mut self, cx: &mut Cx) {
        cx.stop_timer(self.timer);
        if let Some(server) = self.assets.take() { server.stop.store(true, Ordering::Release); }
        if let Some((commands, _)) = self.link.take() { let _ = commands.send(sync::Command::Stop); }
        for (_, child) in &self.view.children { retire(cx, child); }
    }
}

impl Widget for CalendarView {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let result = self.view.draw_walk(cx, scope, walk);
        let rect = self.view.area().rect(cx);
        self.origin = rect.pos;
        if rect.size.x > 100. && rect.size.y > 100. { self.viewport = rect.size; }
        result
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.started { self.start(cx); }
        if let Event::Actions(actions) = event {
            for action in actions {
                let Some(action) = action.downcast_ref::<WidgetAction>() else { continue };
                let kit = action.cast::<octoscript_widgets::kit::KitAction>();
                if std::env::var("CALENDAR_TRACE").is_ok() { log!("calendar: widget action {:?} kit={:?} suppressed={}", action.widget_uid, kit, self.suppress_activation); }
                let kind = match kit {
                    octoscript_widgets::kit::KitAction::Activated if !self.suppress_activation => None,
                    octoscript_widgets::kit::KitAction::Changed(value) => Some(value),
                    _ => continue,
                };
                let Some(native) = self.elements.iter().filter_map(|e| e["id"].as_str()).find(|id| self.view.widget(cx, &[LiveId::from_str(id)]).widget_uid() == action.widget_uid).map(str::to_owned) else { log!("calendar: action from an unmapped widget"); continue };
                let Some(control) = self.actions.get(&native).cloned() else { log!("calendar: no control for {native}"); continue };
                let Some(id) = control["event"].as_str().map(str::to_owned) else { continue };
                match kind {
                    Some(value) => { if control["input"] == true { self.session().field_changed(&id, &value); self.refresh_editor(cx); } }
                    None => { if self.session().activate(&id) { self.mounted_revision = None; } }
                }
            }
        }
        if self.timer.is_event(event).is_some() {
            self.suppress_activation = false;
            self.pump_sync();
            let revision = self.session().revision;
            if self.mounted_revision != Some(revision) && self.viewport.x > 0. {
                if let Err(error) = self.mount(cx) { log!("calendar: cannot mount: {error}"); self.mounted_revision = Some(revision); }
            }
        }
        self.view.handle_event(cx, event, scope);
    }
}

pub struct CalendarModule;
pub static CALENDAR_MODULE: CalendarModule = CalendarModule;
impl AppModule for CalendarModule {
    fn id(&self) -> &'static str { "calendar" }
    fn label(&self) -> &'static str { "Calendar" }
    fn capabilities(&self) -> &'static [&'static str] { &["net"] }
    fn open_schema(&self) -> OpenSchema {
        OpenSchema::new(1).arg("server", OpenArgKind::Text, false).arg("token", OpenArgKind::Text, false).arg("device", OpenArgKind::Text, false).arg("locale", OpenArgKind::Text, false)
    }
    fn register(&self, vm: &mut ScriptVm) {
        octoscript_widgets::design::script_mod(vm);
        octoscript_widgets::kit::script_mod(vm);
        script_mod(vm);
    }
    fn create(&self, vm: &mut ScriptVm, open: ValidatedOpen, handles: InstanceHandles) -> InstanceParts {
        let value = script_eval!(vm, { use mod.widgets.* CalendarView {} });
        let root = WidgetRef::script_from_value(vm, value);
        if let Some(mut calendar) = root.borrow_mut::<CalendarView>() {
            let mut config = Config::from_env();
            if let Some(server) = open.text("server").filter(|s| s.starts_with("http://")) { config.server = Some(server.trim_end_matches('/').to_owned()); }
            if let Some(token) = open.text("token") { config.token = token.to_owned(); }
            if let Some(device) = open.text("device") { config.device = device.to_owned(); }
            if let Some(locale) = open.text("locale") { config.locale = locale.to_owned(); }
            calendar.config = config;
            calendar.viewport = handles.viewport.size;
        }
        let cleanup = root.clone();
        InstanceParts { root, executor: Box::new(CalendarExecutor), shutdown: Box::new(move |vm| { if let Some(mut calendar) = cleanup.borrow_mut::<CalendarView>() { calendar.shutdown(vm.cx_mut()); } }) }
    }
}
struct CalendarExecutor;
impl ServiceExecutor for CalendarExecutor {
    fn manifest(&self) -> ServiceManifest { ServiceManifest::new("calendar", "Calendar", "The household calendar, synced through the calendar service.") }
    fn execute(&mut self, _cx: &mut Cx, call: &ServiceCall) -> ExecOutcome { ExecOutcome::Done(ToolResult::unavailable(&call.call_id, "Use the Calendar interface")) }
}
