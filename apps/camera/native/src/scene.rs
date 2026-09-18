//! Scene trees and their L0 compilation. A scene is the same node shape the
//! design pipeline authors (`stack`, `text`, `button`, `svg`, `input`), built
//! here at runtime from the camera state; `compile` turns it into the L0
//! ledger + kit pack + placements that `octoscript_ui_l0::realize` and
//! `kit_pack::lower` accept — the same lowering the reviewed storyboard cards
//! went through, so the native widgets are the same Kit components.
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, HashMap};

pub const WIDTH: f64 = 406.0;
pub const HEIGHT: f64 = 776.0;
pub const REGULAR: &str = "self:resources/service/NotoSansSC-Regular.ttf";
pub const MEDIUM: &str = "self:resources/service/NotoSansSC-Medium.ttf";
pub const BOLD: &str = "self:resources/service/NotoSansSC-Bold.ttf";

// The Mate camera palette (rrggbb / rrggbbaa), sampled on the device (docs/ux-map.md)
pub const BLACK: &str = "000000";
pub const WHITE: &str = "ffffff";
pub const PILL: &str = "111111";
pub const PILL2: &str = "1f1f1f";
pub const ROUND: &str = "151515";
pub const RESPILL: &str = "272727";
pub const CHIP: &str = "dcdcdc";
pub const CHIP_TEXT: &str = "4f4f4f";
pub const ZOOM_PILL: &str = "0000004d";
pub const GREY: &str = "727272";
pub const RED: &str = "e2362c";
pub const REC: &str = "ea3323";
pub const BOX: &str = "202020";
pub const BOX_CIRCLE: &str = "333333";
pub const PANEL: &str = "00000073";
pub const PANEL_SEG: &str = "ffffff26";
pub const MENU: &str = "2f3034";
pub const DIVIDER: &str = "46474b";
pub const PRO_BAR: &str = "00000080";
pub const INK_DIM: &str = "c8c8c8";
pub const DOT: &str = "808080";
pub const TOAST: &str = "00000080";
pub const BLUE: &str = "4a90ff";
pub const DIM: &str = "00000099";

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
    /// `bg = None` leaves the page transparent so the live preview underneath shows through.
    pub fn new(asset_base: &str, bg: Option<&str>) -> Self {
        let mut scene = Scene { nodes: Vec::new(), parents: Vec::new(), origins: HashMap::new(), controls: Map::new(), assets: HashMap::new(), asset_base: asset_base.into() };
        let mut page = json!({"t": "stack", "id": "page", "x": 0, "y": 0, "w": WIDTH, "h": HEIGHT, "c": []});
        if let Some(bg) = bg { page["variant"] = json!("surface"); page["bg"] = json!(col(bg)); page["radius"] = json!(0); }
        scene.nodes.push(page);
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
        else if border.is_some() { node["variant"] = json!("surface"); node["bg"] = json!(col("00000000")); node["radius"] = json!(radius); }
        if let Some(border) = border { node["border"] = json!(0.7); node["bordercolor"] = json!(col(border)); }
        self.push(node, parent);
    }
    /// A vertically scrolling container; children use coordinates relative to its top-left.
    pub fn scroll(&mut self, id: &str, parent: &str, x: f64, y: f64, w: f64, h: f64) {
        self.push(json!({"t": "stack", "id": id, "x": x, "y": y, "w": w, "h": h, "variant": "scroll_y", "c": []}), parent);
    }
    /// One line of text centred in the box `y..y+h`. The label keeps the
    /// font's natural line height (Noto Sans SC: 1.45×) and is offset to the
    /// box's middle — a taller line box seats the glyphs at its top.
    pub fn text(&mut self, id: &str, parent: &str, text: &str, x: f64, y: f64, w: f64, h: f64, size: f64, bold: bool, color: &str, align: Align) {
        self.text_w(id, parent, text, x, y, w, h, size, if bold { 700 } else { 400 }, color, align)
    }
    /// One line of text with an explicit weight (400 / 500 / 700).
    pub fn text_w(&mut self, id: &str, parent: &str, text: &str, x: f64, y: f64, w: f64, h: f64, size: f64, weight: u32, color: &str, align: Align) {
        let alignx = match align { Align::Left => 0.0, Align::Center => 0.5, Align::Right => 1.0 };
        let line = ((size * 1.45).min(h) * 100.0).round() / 100.0;
        let top = ((y + (h - line) / 2.0) * 100.0).round() / 100.0;
        let font = match weight { 700 => BOLD, 500 => MEDIUM, _ => REGULAR };
        self.push(json!({"t": "text", "id": id, "text": text, "x": x, "y": top, "w": w, "h": line, "size": size, "line_height": line, "weight": weight,
                         "color": col(color), "font_src": font, "variant": "single_line", "alignx": alignx}), parent);
    }
    /// A text field named `id`: a Kit form field wrapping the native input, so edits arrive as KitAction::Changed on `id`.
    pub fn input(&mut self, id: &str, parent: &str, value: &str, placeholder: &str, x: f64, y: f64, w: f64, h: f64, size: f64, focused: bool) {
        self.stack(id, parent, x, y, w, h, None, 0.0, None);
        self.push(json!({"t": "input", "id": format!("{id}_input"), "text": value, "placeholder": placeholder, "x": x, "y": y, "w": w, "h": h, "size": size, "line_height": h, "weight": 400,
                         "color": col(WHITE), "font_src": REGULAR, "variant": "single_line", "alignx": 0, "focused": if focused { 1 } else { 0 }}), id);
        self.controls.insert(id.into(), json!({"event": id, "input": true, "enabled": true}));
    }
    /// A raster image served from the loopback asset server (`name` is the asset file).
    pub fn image(&mut self, id: &str, parent: &str, name: &str, x: f64, y: f64, w: f64, h: f64) {
        self.push(json!({"t": "image", "id": id, "x": x, "y": y, "w": w, "h": h, "src": format!("{}/assets/{name}", self.asset_base)}), parent);
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
            "chip" => (Some(WHITE), BLACK, true, None),
            "pill" => (Some(PILL2), WHITE, true, Some("ffffff66")),
            "outline" => (None, WHITE, false, Some(WHITE)),
            _ => (None, WHITE, false, None),
        };
        let (bg, color, border) = if enabled { (bg, color, border) } else { (bg, GREY, border) };
        self.stack(&format!("{id}_surface"), id, x, y, w, h, bg, if bg.is_some() { 10.0 } else { 0.0 }, border);
        self.text(&format!("{id}_label"), id, label, x, y, w, h, size, bold, color, if style == "text" { Align::Left } else { Align::Center });
    }
    /// Recolour an already added text node.
    pub fn node_color(&mut self, id: &str, color: &str) {
        if let Some(n) = self.nodes.iter_mut().find(|n| n["id"] == id) { n["color"] = json!(col(color)); }
    }
    /// Assemble the flat node list into the page tree.
    /// Every button's rectangle in artboard coordinates (`push` already
    /// resolved scroll-relative children to absolute positions).
    pub fn button_rects(&self) -> Vec<(f64, f64, f64, f64)> {
        self.nodes.iter().filter(|n| n["t"] == "button").filter_map(|n| Some((n["x"].as_f64()?, n["y"].as_f64()?, n["w"].as_f64()?, n["h"].as_f64()?))).collect()
    }
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
        let component = format!("Cam{}", &hash(&format!("{style}{props}{slot}"))[..14]);
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
        let mut s = Scene::new("http://127.0.0.1:1/t", None);
        s.text("title", "page", "拍照 Photo", 20.0, 40.0, 300.0, 44.0, 34.0, true, RED, Align::Left);
        s.labelled("add", "page", "Add", 300.0, 40.0, 80.0, 36.0, "chip", 15.0, true);
        s.labelled("off", "page", "Off", 200.0, 40.0, 80.0, 36.0, "outline", 15.0, false);
        s.input("title_field", "page", "", "Title", 20.0, 100.0, 360.0, 40.0, 17.0, false);
        s.icon("i", "page", "flash_off", 20.0, 160.0, 24.0, 24.0, RED);
        s.scroll("list", "page", 0.0, 200.0, 406.0, 400.0);
        s.text("row", "list", "quoted \"text\" { stays data }", 20.0, 10.0, 300.0, 24.0, 15.0, false, WHITE, Align::Left);
        let frame = compile(&s, "camera-test");
        let report = octoscript_ui_l0::realize(&frame.card, &frame.data, Default::default());
        let root = report.complete_root().expect("complete root");
        let source = octoscript_ui_l0::kit_pack::lower(root, &frame.pack, &frame.data).expect("lower");
        let mut tree = octoscript_makepad::design::prepare(&source).expect("prepare");
        let elements = octoscript_makepad::l0::inspectable(&mut tree);
        assert!(!octoscript_makepad::design::to_makepad_ui(&tree).unwrap().is_empty());
        assert!(elements.len() > 8);
        assert_eq!(frame.actions.get(&frame.mapping["add_control"]).unwrap()["event"], "add");
        assert_eq!(frame.actions.get(&frame.mapping["add_label"]).unwrap()["event"], "add", "a label inside a button inherits its control");
        assert!(frame.assets.contains_key("flash_off-e2362c.svg"));
    }
}
