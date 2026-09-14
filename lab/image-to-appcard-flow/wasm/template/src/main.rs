//! Real Makepad/Splash widgets with an in-memory browser transport.
pub use makepad_widgets;
use makepad_widgets::*;
use makepad_widgets::script::res::CxScriptResourceData;
use serde_json::{json, Value};
use splash_widgets::design::*;
use std::cell::RefCell;
use std::rc::Rc;

const SERVICE_FONT: &[u8] = include_bytes!("../resources/service/NotoSansSC-Regular.ttf");

app_main!(App, font_assets: [
    "octosense_wizard/resources/service/NotoSansSC-Regular.ttf",
    "octosense_wizard/resources/service/NotoSansSC-Medium.ttf",
    "octosense_wizard/resources/service/NotoSansSC-Bold.ttf",
    "makepad_widgets/resources/NotoColorEmoji.ttf",
]);

thread_local! {
    static COMMANDS: RefCell<Vec<Value>> = const { RefCell::new(Vec::new()) };
    static EVENTS: RefCell<Vec<Value>> = const { RefCell::new(Vec::new()) };
    static EVENT_BYTES: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

fn emit(event: Value) { EVENTS.with(|events| events.borrow_mut().push(event)); }

// Only the same-origin bridge calls this ABI. Allocation is transferred exactly once.
#[no_mangle]
pub extern "C" fn octosense_alloc(length: usize) -> *mut u8 {
    let data = vec![0u8; length].into_boxed_slice();
    Box::into_raw(data) as *mut u8
}

#[no_mangle]
pub unsafe extern "C" fn octosense_command(pointer: *mut u8, length: usize) -> u32 {
    let data = Box::from_raw(std::ptr::slice_from_raw_parts_mut(pointer, length));
    match serde_json::from_slice::<Value>(&data) {
        Ok(command) => {
            COMMANDS.with(|commands| commands.borrow_mut().push(command));
            1
        }
        Err(error) => { emit(json!({"type":"octosense:error","error":error.to_string()})); 0 }
    }
}

#[no_mangle]
pub extern "C" fn octosense_events() -> *const u8 {
    let values = EVENTS.with(|events| std::mem::take(&mut *events.borrow_mut()));
    EVENT_BYTES.with(|bytes| {
        *bytes.borrow_mut() = serde_json::to_vec(&values).unwrap();
        bytes.borrow().as_ptr()
    })
}

#[no_mangle]
pub extern "C" fn octosense_events_len() -> usize {
    EVENT_BYTES.with(|bytes| bytes.borrow().len())
}

script_mod! {
    use mod.prelude.widgets.*
    // Register dynamically used fonts before the browser's resource-loading phase.
    mod.service_regular = crate_resource("self:resources/service/NotoSansSC-Regular.ttf")
    mod.service_medium = crate_resource("self:resources/service/NotoSansSC-Medium.ttf")
    mod.service_bold = crate_resource("self:resources/service/NotoSansSC-Bold.ttf")
    mod.service_emoji = crate_resource("makepad_widgets:resources/NotoColorEmoji.ttf")
    startup() do #(App::script_component(vm)) {
        ui: Root {
            main_window := Window {
                show_caption_bar: false
                window.inner_size: vec2(406, 776)
                body +: {
                    flow: Overlay
                    host := Splash { width: Fill height: Fill }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live] ui: WidgetRef,
    #[rust] timer: Timer,
    #[rust] retired_view: Option<View>,
    #[rust] elements: Vec<Value>,
    #[rust] mapping: Vec<Value>,
    #[rust] required_fonts: Vec<String>,
    #[rust] request_id: Value,
    #[rust] generation: Value,
    #[rust] awaiting_draw: bool,
    #[rust] awaiting_ticks: u32,
    #[rust] width: f64,
    #[rust] height: f64,
}

impl App {
    fn native_id<'a>(&'a self, id: &'a str) -> &'a str {
        self.mapping.iter().find(|row| row["source_id"].as_str() == Some(id))
            .and_then(|row| row["native_id"].as_str()).unwrap_or(id)
    }

    fn mount(&mut self, cx: &mut Cx, request: &Value) -> Result<(), String> {
        let card = request["card"].as_str().ok_or("missing card")?;
        let data = &request["data"];
        let pack = &request["kit"];
        let report = splash_ui_l0::realize(card, data, Default::default());
        let root = report.complete_root()?;
        let theme = splash_ui_l0::card_theme(card).unwrap_or_else(|| "dark".into());
        if pack["theme"] != theme { return Err("kit/ledger theme mismatch".into()); }
        let source = splash_ui_l0::kit_pack::lower(root, pack, data)?;
        let mut tree = splash_makepad::design::prepare(&source)?;
        let elements = splash_makepad::l0::inspectable(&mut tree);
        let ui = splash_makepad::design::to_makepad_ui(&tree)?;
        let code = format!("use mod.prelude.widgets.*\nreturn View{{width:Fill height:Fill flow:Overlay {ui}}}");
        let module = ScriptMod {
            cargo_manifest_path: env!("CARGO_MANIFEST_DIR").into(),
            module_path: module_path!().into(), file: file!().into(), line: 1, column: 0,
            code, values: Vec::new(),
        };
        let view = cx.with_vm(|vm| {
            let value = vm.eval_checked(module, 2_000_000)
                .ok_or_else(|| format!("Makepad checked evaluation rejected the card: {:?}", vm.bx.captured_errors))?;
            Ok::<_, String>(View::script_from_value(vm, value))
        })?;
        self.width = request["width"].as_f64().ok_or("missing width")?;
        self.height = request["height"].as_f64().ok_or("missing height")?;
        if self.width <= 0.0 || self.height <= 0.0 { return Err("invalid artboard".into()); }
        self.request_id = request["id"].clone();
        self.generation = request["generation"].clone();
        self.mapping = request["mapping"].as_array()
            .or_else(|| request["mapping"]["elements"].as_array()).ok_or("missing mapping")?.clone();
        self.required_fonts = self.mapping.iter().filter_map(|row| row["font"].as_str())
            .map(|font| font.strip_prefix("self:").unwrap_or(font).to_owned()).collect();
        self.required_fonts.sort();
        self.required_fonts.dedup();
        self.elements = elements;
        let host = self.ui.widget(cx, ids!(host));
        let mut host = host.borrow_mut::<Splash>().ok_or("missing Splash host")?;
        cx.set_key_focus(Area::Empty);
        self.retired_view = Some(std::mem::replace(&mut host.view, view));
        fn retire(cx: &mut Cx, widget: WidgetRef) {
            splash_widgets::kit::retire_overlay(cx, &widget);
            let list = widget.borrow::<DesignOverlay>().and_then(|v| v.draw_list.as_ref().map(|l| l.id()))
                .or_else(|| widget.borrow::<DesignGlassSvg>().and_then(|v| v.draw_list.as_ref().map(|l| l.id())));
            if let Some(id) = list {
                let recording = cx.next_uniform_gen();
                let uniforms = cx.next_uniform_gen();
                cx.draw_lists[id].clear_draw_items(cx.redraw_id, recording, uniforms);
            }
            let mut children = Vec::new();
            widget.children(&mut |_, child| children.push(child));
            for child in children { retire(cx, child); }
        }
        if let Some(retired) = &self.retired_view {
            for (_, child) in &retired.children { retire(cx, child.clone()); }
        }
        let uid = host.widget_uid();
        let mut children = Vec::new();
        host.children(&mut |id, child| children.push((id, child)));
        drop(host);
        for (id, child) in children { cx.widget_tree_insert_child_deep(uid, id, child); }
        cx.widget_tree_mark_dirty(uid);
        for element in &self.elements {
            if let (Some(id), Some(enabled)) = (element["id"].as_str(), element["enabled"].as_i64()) {
                self.ui.widget(cx, &[LiveId::from_str(id)]).set_disabled(cx, enabled == 0);
            }
        }
        if let Some(updates) = request["updates"].as_array() {
            for update in updates {
                let id = self.native_id(update["id"].as_str().ok_or("update missing id")?);
                if !self.elements.iter().any(|element| element["id"] == id) {
                    return Err(format!("Unknown native update: {id}"));
                }
                let widget = self.ui.widget(cx, &[LiveId::from_str(id)]);
                if let Some(text) = update["text"].as_str() { widget.set_text(cx, text); }
                if let Some(enabled) = update["enabled"].as_bool() {
                    // Button hit handling uses enabled; disabled() is its animator state.
                    // Update both through the native APIs, then inspect and click them.
                    self.ui.button(cx, &[LiveId::from_str(id)]).set_enabled(cx, enabled);
                    widget.set_disabled(cx, !enabled);
                }
            }
        }
        self.awaiting_draw = true;
        self.awaiting_ticks = 0;
        cx.redraw_all();
        Ok(())
    }

    fn inspect(&self, cx: &mut Cx, kind: &str) -> bool {
        if self.elements.is_empty() { return false; }
        // A partial fallback family can lay out tofu before the CJK font arrives.
        // A rendered receipt requires every font requested by the mapped card.
        {
            let resources = cx.script_data.resources.resources.borrow();
            if self.required_fonts.iter().any(|font| !resources.iter()
                .any(|resource| resource.abs_path.ends_with(font) && resource.loaded_len() > 0)) {
                return false;
            }
        }
        let mut rows = Vec::new();
        for element in &self.elements {
            let Some(id) = element["id"].as_str() else { continue; };
            let widget = self.ui.widget(cx, &[LiveId::from_str(id)]);
            let area = widget.area();
            if !area.is_valid(cx) { return false; }
            let rect = area.rect(cx);
            let clip = area.clipped_rect(cx);
            let vector_ready = widget.borrow::<Svg>().map(|svg|
                svg.draw_svg.svg_doc.is_some() && !svg.draw_svg.cached_indices.is_empty());
            let image_ready = widget.borrow::<Image>().map(|image| image.size_in_pixels(cx).is_some());
            if vector_ready == Some(false) || image_ready == Some(false) { return false; }
            let text_layout = widget.borrow::<Label>().map(|label| {
                let r = label.text_layout_rect;
                [r.pos.x, r.pos.y, r.size.x, r.size.y]
            });
            if !widget.text().is_empty() && text_layout.is_some_and(|r| r[2] <= 0.0 || r[3] <= 0.0) {
                return false;
            }
            let mapping = self.mapping.iter().find(|row| row["native_id"] == id);
            let hit_enabled = widget.borrow::<Button>().map(|button| button.enabled());
            rows.push(json!({"nativeId":id,"sourceId":mapping.map(|row|row["source_id"].clone()),
                "kind":element["kind"],"bounds":[rect.pos.x,rect.pos.y,rect.size.x,rect.size.y],
                "clippedBounds":[clip.pos.x,clip.pos.y,clip.size.x,clip.size.y],
                "enabled":hit_enabled.unwrap_or(!widget.disabled(cx)),"hitEnabled":hit_enabled,
                "visible":widget.visible(),"text":widget.text(),
                "textLayout":text_layout,"vectorReady":vector_ready,"imageReady":image_ready}));
        }
        emit(json!({"type":kind,"id":self.request_id,"generation":self.generation,
                    "width":self.width,"height":self.height,"widgets":rows,"renderer":"Makepad/WASM"}));
        true
    }
}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        crate::makepad_widgets::theme_mod(vm);
        splash_widgets::widgets_mod(vm);
        splash_widgets::design::script_mod(vm);
        splash_widgets::kit::script_mod(vm);
        splash_widgets::progress::script_mod(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if matches!(event, Event::Startup) {
            // Keep the service's exact CJK face resident before the first mount.
            // This also makes card text independent of deferred font HTTP loading.
            for resource in cx.script_data.resources.resources.borrow_mut().iter_mut() {
                if resource.abs_path.ends_with("resources/service/NotoSansSC-Regular.ttf") {
                    resource.data = CxScriptResourceData::Loaded(Rc::new(SERVICE_FONT.to_vec()));
                }
            }
            self.timer = cx.start_interval(0.04);
            emit(json!({"type":"octosense:ready","renderer":"Makepad/WASM","protocol":1}));
        }
        if let Event::Actions(actions) = event {
            for action in actions {
                let Some(action) = action.downcast_ref::<WidgetAction>() else { continue; };
                if !matches!(action.cast::<splash_widgets::kit::KitAction>(), splash_widgets::kit::KitAction::Activated) { continue; }
                let Some(native) = self.elements.iter().filter_map(|e| e["id"].as_str())
                    .find(|id| self.ui.widget(cx, &[LiveId::from_str(id)]).widget_uid() == action.widget_uid) else { continue; };
                let source = self.mapping.iter().find(|row| row["native_id"] == native)
                    .and_then(|row| row["source_id"].as_str());
                emit(json!({"type":"octosense:action","id":self.request_id,"generation":self.generation,
                            "control":source,"sourceId":source,"nativeId":native,"action":"activated"}));
            }
        }
        if self.timer.is_event(event).is_some() {
            let commands = COMMANDS.with(|commands| std::mem::take(&mut *commands.borrow_mut()));
            for command in commands {
                match command["type"].as_str() {
                    Some("octosense:render") | Some("mount") => {
                        if let Err(error) = self.mount(cx, &command) {
                            emit(json!({"type":"octosense:error","id":command["id"],"generation":command["generation"],"error":error}));
                        }
                    }
                    Some("octosense:inspect") => { self.inspect(cx, "octosense:snapshot"); }
                    _ => emit(json!({"type":"octosense:error","error":"Unknown command"})),
                }
            }
        }
        self.ui.handle_event(cx, event, &mut Scope::empty());
        if self.timer.is_event(event).is_some() && self.awaiting_draw && self.inspect(cx, "octosense:rendered") {
            self.awaiting_draw = false;
        }
        if self.timer.is_event(event).is_some() && self.awaiting_draw {
            self.awaiting_ticks += 1;
            if self.awaiting_ticks == 50 {
                let resources: Vec<_> = cx.script_data.resources.resources.borrow().iter()
                    .map(|r| json!({"path":r.abs_path,"dependency":r.dependency_path,"bytes":r.loaded_len(),"error":r.is_error()})).collect();
                emit(json!({"type":"octosense:diagnostics","id":self.request_id,"generation":self.generation,"resources":resources}));
            }
            cx.redraw_all();
        }
    }
}
