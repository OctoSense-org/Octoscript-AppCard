//! Scene trees and their L0 compilation. A scene is the same node shape the
//! design pipeline authors (`stack`, `text`, `button`, `svg`, `input`), built
//! here at runtime from the calendar state; `compile` turns it into the L0
//! ledger + kit pack + placements that `octoscript_ui_l0::realize` and
//! `kit_pack::lower` accept — the same lowering the reviewed storyboard cards
//! went through, so the native widgets are the same Kit components.
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, HashMap};

pub const WIDTH: f64 = 406.0;
pub const HEIGHT: f64 = 716.0;
pub const REGULAR: &str = "self:resources/service/NotoSansSC-Regular.ttf";
pub const BOLD: &str = "self:resources/service/NotoSansSC-Bold.ttf";

// iOS-like palette (rrggbb)
pub const INK: &str = "1c1c1e";
pub const GRAY: &str = "8e8e93";
pub const GRAY2: &str = "c7c7cc";
pub const LINE: &str = "e5e5ea";
pub const PAGE: &str = "ffffff";
pub const GROUP: &str = "f2f2f7";
pub const RED: &str = "ff3b30";
pub const BLUE: &str = "0a84ff";
pub const GREEN: &str = "34c759";
pub const WHITE: &str = "ffffff";

/// `rrggbb` or `rrggbbaa` → ARGB u32.
pub fn col(v: &str) -> u32 {
    let v = v.trim_start_matches('#');
    let (rgb, alpha) = if v.len() == 8 { (&v[..6], &v[6..8]) } else { (v, "ff") };
    u32::from_str_radix(&format!("{alpha}{rgb}"), 16).unwrap_or(0xff000000)
}

#[derive(Clone, Copy, PartialEq)]
pub enum Align { Left, Center, Right }

pub struct Scene {
    pub nodes: Vec<Value>,
    parents: Vec<String>,
    /// Page-absolute origin every child of a node is placed against: children
    /// of a scrolling container are authored relative to it, but the renderer
    /// keeps source frames window-local, so the container's origin is added.
    origins: HashMap<String, (f64, f64)>,
    pub controls: Map<String, Value>,
    pub assets: HashMap<String, String>,
    asset_base: String,
}

impl Scene {
    pub fn new(asset_base: &str, bg: &str) -> Self {
        let mut scene = Scene { nodes: Vec::new(), parents: Vec::new(), origins: HashMap::new(), controls: Map::new(), assets: HashMap::new(), asset_base: asset_base.into() };
        scene.nodes.push(json!({"t": "stack", "id": "page", "x": 0, "y": 0, "w": WIDTH, "h": HEIGHT, "variant": "surface", "bg": col(bg), "radius": 0, "c": []}));
        scene.parents.push("page".into());
        scene.origins.insert("page".into(), (0.0, 0.0));
        scene
    }
    fn push(&mut self, mut node: Value, parent: &str) {
        let (ox, oy) = self.origins.get(parent).copied().unwrap_or((0.0, 0.0));
        let x = node["x"].as_f64().unwrap_or(0.0) + ox;
        let y = node["y"].as_f64().unwrap_or(0.0) + oy;
        node["x"] = json!(x);
        node["y"] = json!(y);
        let scrolls = node["variant"] == "scroll_y";
        let id = node["id"].as_str().unwrap_or("").to_owned();
        self.origins.insert(id, if scrolls { (x, y) } else { (ox, oy) });
        self.nodes.push(node);
        self.parents.push(parent.into());
    }
    pub fn stack(&mut self, id: &str, parent: &str, x: f64, y: f64, w: f64, h: f64, bg: Option<&str>, radius: f64, border: Option<&str>) {
        let mut node = json!({"t": "stack", "id": id, "x": x, "y": y, "w": w, "h": h, "c": []});
        if let Some(bg) = bg { node["variant"] = json!("surface"); node["bg"] = json!(col(bg)); node["radius"] = json!(radius); }
        if let Some(border) = border { node["border"] = json!(0.7); node["bordercolor"] = json!(col(border)); }
        self.push(node, parent);
    }
    /// A vertically scrolling container; children use coordinates relative to its top-left.
    pub fn scroll(&mut self, id: &str, parent: &str, x: f64, y: f64, w: f64, h: f64) {
        self.push(json!({"t": "stack", "id": id, "x": x, "y": y, "w": w, "h": h, "variant": "scroll_y", "c": []}), parent);
    }
    pub fn text(&mut self, id: &str, parent: &str, text: &str, x: f64, y: f64, w: f64, h: f64, size: f64, bold: bool, color: &str, align: Align) {
        let alignx = match align { Align::Left => 0.0, Align::Center => 0.5, Align::Right => 1.0 };
        self.push(json!({"t": "text", "id": id, "text": text, "x": x, "y": y, "w": w, "h": h, "size": size, "line_height": h, "weight": if bold { 700 } else { 400 },
                         "color": col(color), "font_src": if bold { BOLD } else { REGULAR }, "variant": "single_line", "alignx": alignx}), parent);
    }
    /// A text field named `id`: a Kit form field wrapping the native input, so edits arrive as KitAction::Changed on `id`.
    pub fn input(&mut self, id: &str, parent: &str, value: &str, placeholder: &str, x: f64, y: f64, w: f64, h: f64, size: f64, focused: bool) {
        self.stack(id, parent, x, y, w, h, None, 0.0, None);
        self.push(json!({"t": "input", "id": format!("{id}_input"), "text": value, "placeholder": placeholder, "x": x, "y": y, "w": w, "h": h, "size": size, "line_height": h, "weight": 400,
                         "color": col(INK), "font_src": REGULAR, "variant": "single_line", "alignx": 0, "focused": if focused { 1 } else { 0 }}), id);
        self.controls.insert(id.into(), json!({"event": id, "input": true, "enabled": true}));
    }
    pub fn icon(&mut self, id: &str, parent: &str, name: &str, x: f64, y: f64, w: f64, h: f64, color: &str) {
        let file = format!("{name}-{color}.svg");
        if let Some(svg) = crate::icons::svg(name, color) { self.assets.insert(file.clone(), svg); }
        self.push(json!({"t": "svg", "id": id, "x": x, "y": y, "w": w, "h": h, "src": format!("{}/assets/{file}", self.asset_base)}), parent);
    }
    /// A tappable area named `id`; whatever is drawn inside it (added with `parent = id`) is part of the button.
    pub fn button(&mut self, id: &str, parent: &str, x: f64, y: f64, w: f64, h: f64, enabled: bool) {
        self.stack(id, parent, x, y, w, h, None, 0.0, None);
        self.push(json!({"t": "button", "id": format!("{id}_control"), "x": x, "y": y, "w": w, "h": h, "enabled": enabled}), id);
        self.controls.insert(id.into(), json!({"event": id, "enabled": enabled}));
    }
    /// A labelled button in one of the storyboard styles.
    pub fn labelled(&mut self, id: &str, parent: &str, label: &str, x: f64, y: f64, w: f64, h: f64, style: &str, size: f64, enabled: bool) {
        self.button(id, parent, x, y, w, h, enabled);
        let (bg, color, bold, border) = match style {
            "filled" => (Some(RED), WHITE, true, None),
            "blue" => (Some(BLUE), WHITE, true, None),
            "green" => (Some(GREEN), WHITE, true, None),
            "outline" => (Some(GROUP), INK, false, Some(GRAY2)),
            "danger" => (Some(GROUP), RED, false, Some(GRAY2)),
            _ => (None, RED, false, None),
        };
        let (bg, color, border) = if enabled { (bg, color, border) } else { (bg.map(|_| "e5e5ea"), GRAY2, None) };
        self.stack(&format!("{id}_surface"), id, x, y, w, h, bg, if bg.is_some() { 10.0 } else { 0.0 }, border);
        self.text(&format!("{id}_label"), id, label, x, y, w, h, size, bold, color, if style == "text" { Align::Left } else { Align::Center });
    }
    /// Assemble the flat node list into the page tree.
    pub fn tree(&self) -> Value {
        let mut children: BTreeMap<String, Vec<Value>> = BTreeMap::new();
        fn build(id: &str, nodes: &[Value], parents: &[String], children: &mut BTreeMap<String, Vec<Value>>, controls: &Map<String, Value>) -> Value {
            let index = nodes.iter().position(|n| n["id"] == id).unwrap();
            let mut node = nodes[index].clone();
            if node.get("c").is_some() {
                let kids: Vec<usize> = (0..nodes.len()).filter(|i| parents[*i] == id && *i != index).collect();
                // Native labels above surfaces: the same ordering the pipeline uses.
                let mut ordered: Vec<Value> = kids.iter().map(|i| build(nodes[*i]["id"].as_str().unwrap(), nodes, parents, children, controls)).collect();
                // Labels above surfaces (the pipeline's ordering), the hit area on top of everything.
                ordered.sort_by_key(|n| if n["t"] == "button" { 2 } else if n["t"] == "text" { 1 } else { 0 });
                // A control's stack is the Kit component: the wrapper that turns the
                // native button's click or the input's edit into a KitAction on this id.
                if controls.contains_key(id) {
                    if let Some(button) = ordered.iter().position(|n| n["t"] == "button") {
                        let mut bindings = json!({"control": [button]});
                        if let Some(label) = ordered.iter().position(|n| n["t"] == "text") { bindings["label"] = json!([label]); }
                        node["kit"] = json!(json!({"widget": "KitButton", "bindings": bindings}).to_string());
                    } else if let Some(input) = ordered.iter().position(|n| n["t"] == "input") {
                        node["kit"] = json!(json!({"widget": "KitFormField", "bindings": {"input": [input]}}).to_string());
                    }
                }
                node["c"] = Value::Array(ordered);
            }
            node
        }
        build("page", &self.nodes, &self.parents, &mut children, &self.controls)
    }
}

/// One mountable frame: the L0 ledger, its data and pack, and how native ids map back.
pub struct Frame {
    pub card: String,
    pub data: Value,
    pub pack: Value,
    /// native path (`beauty_0_3_1`) → control declaration, inherited by a control's children.
    pub actions: HashMap<String, Value>,
    /// source id → native path
    pub mapping: HashMap<String, String>,
    pub assets: HashMap<String, String>,
}

fn hash(text: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(text.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn compile(scene: &Scene, ledger: &str) -> Frame {
    let tree = scene.tree();
    let mut pack = json!({"schema_version": 1, "theme": "light", "tokens": {}, "components": {}});
    let mut placements = json!({});
    let mut declarations = String::new();
    let mut definitions = BTreeMap::new();
    let mut actions = HashMap::new();
    let mut mapping = HashMap::new();
    struct Ctx<'a> { controls: &'a Map<String, Value>, pack: &'a mut Value, placements: &'a mut Value, declarations: &'a mut String, definitions: &'a mut BTreeMap<String, String>, actions: &'a mut HashMap<String, Value>, mapping: &'a mut HashMap<String, String> }
    fn emit(node: &Value, depth: usize, path: &str, parent_control: Option<&Value>, ctx: &mut Ctx) -> String {
        let id = node["id"].as_str().unwrap_or("");
        ctx.mapping.insert(id.into(), path.into());
        let control = ctx.controls.get(id).or(parent_control);
        if let Some(control) = control { ctx.actions.insert(path.into(), control.clone()); }
        let mut style = node.clone();
        let obj = style.as_object_mut().unwrap();
        for key in ["id", "c", "x", "y", "w", "h", "src", "text", "placeholder", "enabled", "selected"] { obj.remove(key); }
        let mut props = json!({});
        let mut params = vec!["instance: text".to_owned()];
        let mut args = vec!["instance: instance".to_owned()];
        let mut call = vec![format!("instance: {}", json!(id))];
        for key in ["text", "placeholder", "enabled", "selected"] {
            if let Some(value) = node.get(key) {
                props[key] = json!(key);
                let boolean = matches!(key, "enabled" | "selected");
                params.push(format!("{key}: {}", if boolean { "bool" } else { "text" }));
                args.push(format!("{key}: {key}"));
                let name = format!("{id}_{key}");
                if boolean {
                    ctx.declarations.push_str(&format!("state {name} {{ shape: bool, initial: {} }}\n", *value == json!(true) || *value == json!(1)));
                    call.push(format!("{key}: {name}"));
                } else {
                    ctx.declarations.push_str(&format!("copy {name} {{ class: user-copy, en: {value} }}\n"));
                    call.push(format!("{key}: copy.{name}"));
                }
            }
        }
        let slot = node.get("c").is_some();
        let component = format!("Cal{}", &hash(&format!("{style}{props}{slot}"))[..14]);
        ctx.pack["components"][&component] = json!({"style": style, "props": props, "slot": slot});
        let mut layout = json!({});
        for key in ["x", "y", "w", "h", "src"] { if let Some(v) = node.get(key) { layout[key] = v.clone(); } }
        ctx.placements[id] = json!({"component": component, "layout": layout});
        ctx.definitions.insert(component.clone(), format!("component {component}({}) {{\n view Kit(component: {}, {}){}\n}}\n", params.join(", "), json!(component), args.join(", "), if slot { " { slot }" } else { "" }));
        let mut out = format!("{}{component}({})", "  ".repeat(depth), call.join(", "));
        if let Some(children) = node["c"].as_array() {
            out.push_str(" {\n");
            for (i, child) in children.iter().enumerate() {
                out.push_str(&emit(child, depth + 1, &format!("{path}_{i}"), control, ctx));
                out.push('\n');
            }
            out.push_str(&format!("{}}}", "  ".repeat(depth)));
        }
        out
    }
    let body = {
        let mut ctx = Ctx { controls: &scene.controls, pack: &mut pack, placements: &mut placements, declarations: &mut declarations, definitions: &mut definitions, actions: &mut actions, mapping: &mut mapping };
        emit(&tree, 0, "beauty_0", None, &mut ctx)
    };
    let card = format!("# ledger {ledger}@1.0.0\n# level: L0\n# profile: ui/l0\ntheme light\n{declarations}{}\nview root {body}\n", definitions.values().cloned().collect::<String>());
    Frame { card, data: json!({"$kit": {"theme": "light", "placements": placements}}), pack, actions, mapping, assets: scene.assets.clone() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_scene_lowers_through_the_shared_l0_pipeline() {
        let mut s = Scene::new("http://127.0.0.1:1/t", PAGE);
        s.text("title", "page", "九月 September", 20.0, 40.0, 300.0, 44.0, 34.0, true, RED, Align::Left);
        s.labelled("add", "page", "Add", 300.0, 40.0, 80.0, 36.0, "filled", 15.0, true);
        s.labelled("off", "page", "Off", 200.0, 40.0, 80.0, 36.0, "outline", 15.0, false);
        s.input("title_field", "page", "", "Title", 20.0, 100.0, 360.0, 40.0, 17.0, false);
        s.icon("i", "page", "calendar", 20.0, 160.0, 24.0, 24.0, RED);
        s.scroll("list", "page", 0.0, 200.0, 406.0, 400.0);
        s.text("row", "list", "quoted \"text\" { stays data }", 20.0, 10.0, 300.0, 24.0, 15.0, false, INK, Align::Left);
        let frame = compile(&s, "calendar-test");
        let report = octoscript_ui_l0::realize(&frame.card, &frame.data, Default::default());
        let root = report.complete_root().expect("complete root");
        let source = octoscript_ui_l0::kit_pack::lower(root, &frame.pack, &frame.data).expect("lower");
        let mut tree = octoscript_makepad::design::prepare(&source).expect("prepare");
        let elements = octoscript_makepad::l0::inspectable(&mut tree);
        assert!(!octoscript_makepad::design::to_makepad_ui(&tree).unwrap().is_empty());
        assert!(elements.len() > 8);
        assert_eq!(frame.actions.get(&frame.mapping["add_control"]).unwrap()["event"], "add");
        assert_eq!(frame.actions.get(&frame.mapping["add_label"]).unwrap()["event"], "add", "a label inside a button inherits its control");
        assert!(frame.assets.contains_key("calendar-ff3b30.svg"));
    }
}
