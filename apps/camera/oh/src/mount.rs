//! Scene → ArkUI. The scene is the same absolute-coordinate node list the
//! Makepad host lowers through L0; here every node becomes an ArkUI node
//! positioned inside its parent (`NODE_POSITION` is parent-relative), so the
//! tree is built as-is and the strips the app slides are ordinary clipped
//! stacks whose position can be moved later.
use octoscript_oh_arkui::arkui::{attr, event, ty, Node, NodeHandle};
use octosense_camera_logic::scene::Scene;
use serde_json::Value;
use std::collections::HashMap;

// ArkUI enum values (node_attributes/*.h)
pub const FONT_W400: i32 = 3;
pub const FONT_W500: i32 = 4;
pub const FONT_W700: i32 = 6;
pub const TEXT_ALIGN_START: i32 = 0;
pub const TEXT_ALIGN_CENTER: i32 = 1;
pub const TEXT_ALIGN_END: i32 = 2;
pub const ALIGNMENT_CENTER: i32 = 4;
pub const HIT_TEST_TRANSPARENT: i32 = 2;
pub const HIT_TEST_NONE: i32 = 3;
pub const OBJECT_FIT_CONTAIN: i32 = 0;
pub const VISIBILITY_VISIBLE: i32 = 0;
pub const VISIBILITY_HIDDEN: i32 = 1;

/// A mounted scene: the node tree plus what the host needs to talk to it.
pub struct Mounted {
    pub root: Node,
    /// scene id → node handle (valid while `root` lives)
    pub handles: HashMap<String, NodeHandle>,
    /// scene id → absolute rectangle in artboard units
    pub rects: HashMap<String, (f64, f64, f64, f64)>,
    /// scene id → position relative to its parent node (what `set_position` moves from)
    pub rel: HashMap<String, (f64, f64)>,
    /// click target id → control id (`Session::activate` takes the latter)
    pub controls: HashMap<i32, String>,
}

/// Where an image asset (icon SVG, photo thumbnail) lives on this host, as an Image `src`.
pub type ImageSource<'a> = dyn Fn(&str) -> Option<String> + 'a;

pub fn build(scene: &Scene, image_src: &ImageSource<'_>) -> Option<Mounted> {
    let tree = scene.tree();
    let mut m = Mounted { root: Node::new(ty::stack())?, handles: HashMap::new(), rects: HashMap::new(), rel: HashMap::new(), controls: HashMap::new() };
    let mut next_target = 1i32;
    // the page node itself is the root stack
    let (w, h) = (tree["w"].as_f64().unwrap_or(406.0), tree["h"].as_f64().unwrap_or(776.0));
    let root = std::mem::replace(&mut m.root, Node::new(ty::stack())?);
    let mut root = root.width(w as f32).height(h as f32).bg(0x00000000).i32_attr(attr::hit_test(), HIT_TEST_TRANSPARENT);
    m.handles.insert("page".into(), root.raw());
    for child in tree["c"].as_array().into_iter().flatten() {
        if let Some(node) = emit(child, (0.0, 0.0), scene, image_src, &mut m, &mut next_target) { root = root.child(node); }
    }
    m.root = root;
    Some(m)
}

fn emit(n: &Value, origin: (f64, f64), scene: &Scene, image_src: &ImageSource<'_>, m: &mut Mounted, next_target: &mut i32) -> Option<Node> {
    let kind = n["t"].as_str()?;
    let id = n["id"].as_str().unwrap_or("").to_owned();
    let (x, y, w, h) = (n["x"].as_f64()?, n["y"].as_f64()?, n["w"].as_f64()?, n["h"].as_f64()?);
    let place = |node: Node| node.f32v_attr(attr::position(), &[(x - origin.0) as f32, (y - origin.1) as f32]).width(w as f32).height(h as f32);
    let mut node = match kind {
        "button" => return None, // the wrapper stack carries the click
        "stack" => {
            let mut s = place(Node::new(ty::stack())?);
            if let Some(bg) = n["bg"].as_u64() { s = s.bg(bg as u32); }
            if let Some(r) = n["radius"].as_f64() { s = s.radius(r as f32); }
            if let (Some(b), Some(c)) = (n["border"].as_f64(), n["bordercolor"].as_u64()) { s = s.f32_attr(attr::border_width(), b as f32).u32_attr(attr::border_color(), c as u32); }
            if n["variant"] == "scroll_y" { s = s.i32_attr(attr::clip(), 1); }
            s
        }
        "text" | "input" => {
            let weight = n["weight"].as_u64().unwrap_or(400);
            let align = match n["alignx"].as_f64().unwrap_or(0.0) { a if a >= 0.75 => TEXT_ALIGN_END, a if a >= 0.25 => TEXT_ALIGN_CENTER, _ => TEXT_ALIGN_START };
            place(Node::new(ty::text())?)
                .text(n["text"].as_str().unwrap_or(""))
                .font_size(n["size"].as_f64().unwrap_or(14.0) as f32)
                .font_color(n["color"].as_u64().unwrap_or(0xffffffff) as u32)
                .font_weight(if weight >= 700 { FONT_W700 } else if weight >= 500 { FONT_W500 } else { FONT_W400 })
                .i32_attr(attr::text_align(), align)
                .i32_attr(attr::alignment(), ALIGNMENT_CENTER)
                .i32_attr(attr::hit_test(), HIT_TEST_NONE)
        }
        "svg" => {
            // ArkUI's Image takes no data: URI, so the host hands out a file path for the asset.
            let file = n["src"].as_str().and_then(|s| s.rsplit("/assets/").next()).unwrap_or("");
            let Some(src) = image_src(file) else { return None };
            place(Node::new(ty::image())?).string_attr(attr::image_src(), &src).i32_attr(attr::image_fit(), OBJECT_FIT_CONTAIN).i32_attr(attr::hit_test(), HIT_TEST_NONE)
        }
        "image" => {
            let file = n["src"].as_str().and_then(|s| s.rsplit("/assets/").next()).unwrap_or("");
            let Some(src) = image_src(file) else { return None };
            place(Node::new(ty::image())?).string_attr(attr::image_src(), &src).i32_attr(attr::image_fit(), 1).radius((w / 2.0) as f32).i32_attr(attr::hit_test(), HIT_TEST_NONE)
        }
        _ => return None,
    };
    m.rects.insert(id.clone(), (x, y, w, h));
    m.rel.insert(id.clone(), (x - origin.0, y - origin.1));
    // A control: its wrapper stack takes the click (children do not consume it).
    let is_control = n["c"].as_array().map_or(false, |c| c.iter().any(|k| k["t"] == "button" && k["enabled"] != 0));
    if is_control && !id.is_empty() {
        let target = *next_target;
        *next_target += 1;
        node = node.on_event(event::click(), target);
        m.controls.insert(target, id.clone());
    }
    for child in n["c"].as_array().into_iter().flatten() {
        if let Some(c) = emit(child, (x, y), scene, image_src, m, next_target) { node = node.child(c); }
    }
    if !id.is_empty() { m.handles.insert(id, node.raw()); }
    Some(node)
}

/// Move a mounted node (a strip, the surface) to a new artboard position relative to its parent origin.
pub unsafe fn set_position(handle: NodeHandle, x: f32, y: f32) {
    Node::set_f32v_raw(handle, attr::position(), &[x, y]);
}
pub unsafe fn set_text(handle: NodeHandle, text: &str) {
    Node::set_string_raw(handle, attr::text_content(), text);
}
pub unsafe fn set_visible(handle: NodeHandle, visible: bool) {
    Node::set_i32_raw(handle, attr::visibility(), if visible { VISIBILITY_VISIBLE } else { VISIBILITY_HIDDEN });
}
