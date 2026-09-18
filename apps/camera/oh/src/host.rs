//! The app driver on OpenHarmony: owns the session, mounts its scenes as
//! ArkUI nodes under a persistent stack that also holds the camera surface,
//! routes clicks and raw touches back into the session and drives the camera.
//! ArkTS calls `tick` every 16 ms (the motion clock) and performs the
//! requests it pulls afterwards (permission prompts, gallery saves).
use crate::camera::{Camera, Capture, CaptureResult, Control};
use crate::motion::{BarDrag, Spring, CHIP_PITCH, MODE_PITCH};
use crate::mount::{self, Mounted};
use octoscript_oh_arkui::arkui::{self, attr, event, ty, Node, NodeContentHandle, NodeHandle, TOUCH_DOWN, TOUCH_MOVE};
use octosense_camera_logic::session::{self, Session};
use std::cell::RefCell;
use std::collections::HashMap;
use std::os::raw::c_int;

extern "C" {
    fn octoscript_add_child(parent: NodeHandle, child: NodeHandle) -> c_int;
    fn octoscript_remove_child(parent: NodeHandle, child: NodeHandle) -> c_int;
    fn octoscript_set_event_handler(h: extern "C" fn(i32, i32));
}

thread_local! {
    pub static HOST: RefCell<Option<Host>> = const { RefCell::new(None) };
    /// Status-bar height (vp), reported by the ability possibly before the tree is mounted.
    pub static TOP_INSET: std::cell::Cell<f32> = const { std::cell::Cell::new(0.0) };
}

/// The touch registration on the persistent page stack.
const TOUCH_TARGET: i32 = 0x7000;
/// How long the zoom dial stays after the finger leaves it.
const DIAL_HIDE_AFTER: f64 = 0.6;
/// A finger must slide this far (vp) before it turns the dial; a tap's jitter is not a turn.
const DIAL_SLIDE: f64 = 4.0;

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
enum PreviewState { #[default] Idle, Running }

pub struct Host {
    session: Session,
    files_dir: String,
    /// The persistent page: [camera surface, chrome].
    outer: Node,
    surface: Node,
    surface_rect: (f64, f64, f64, f64),
    chrome: Option<Mounted>,
    mounted_revision: Option<u64>,
    camera: Camera,
    permission: Option<bool>,
    mic: Option<bool>,
    mic_asked: bool,
    preview: PreviewState,
    preview_front: bool,
    preview_aspect: f64,
    /// Things ArkTS must do for us ("kind|value"), pulled with `next_request`.
    requests: std::collections::VecDeque<String>,
    photos: HashMap<String, String>,
    /// Icon SVGs already written under `{files_dir}/icons` (ArkUI reads them by path).
    icons_written: std::cell::RefCell<std::collections::HashSet<String>>,
    recording_path: Option<String>,
    zoom_ratio: f32,
    sent_zoom: Option<f32>,
    sent_flash: Option<session::Flash>,
    sent_ev: Option<f32>,
    touch_seen: bool,
    finger_start: Option<(f64, f64)>,
    finger_last: Option<(f64, f64)>,
    /// A click that follows a drag or a long press is not a tap.
    suppress_clicks_until: f64,
    // motion: the mode strip and the zoom chip slide with a spring; a finger can drag the bar
    bar: Spring,
    chip: Spring,
    bar_drag: Option<BarDrag>,
    pinch: Option<(f64, f32)>,
    // a finger dragging the focus box: when the real focus point was last sent
    focus_drag: bool,
    focus_sent_at: f64,
    // the roulette zoom dial: a press on the quick-zoom bar, the finger's slide, the fling after release
    pill_press: Option<(f64, f64, f64)>,
    dial_drag: Option<(f64, f64, f64)>,
    dial_log2: f64,
    dial_vel: f64,
    dial_hide_at: Option<f64>,
    dial_sent_at: f64,
}

fn now() -> f64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0)
}

impl Host {
    pub fn create(slot: NodeContentHandle, files_dir: String) -> Option<()> {
        let outer = Node::new(ty::stack())?.width(406.0).height(776.0).bg(0xff000000)
            .f32v_attr(attr::position(), &[0.0, TOP_INSET.get()])
            .on_event(event::touch(), TOUCH_TARGET);
        let surface = crate::xcomp::surface(406.0, 541.0)?.f32v_attr(attr::position(), &[0.0, 39.4]);
        unsafe { octoscript_add_child(outer.raw(), surface.raw()) };
        let outer = unsafe { outer.mount_keep(slot).ok()? };
        let session = Session::new("cn");
        let zoom_ratio = session.zoom_ratio();
        let host = Host {
            session, files_dir, outer, surface, surface_rect: (0.0, 39.4, 406.0, 541.0), chrome: None, mounted_revision: None,
            camera: Camera::new(), permission: None, mic: None, mic_asked: false, preview: PreviewState::Idle, preview_front: false, preview_aspect: 0.0,
            requests: std::collections::VecDeque::from(vec!["permission|camera".to_owned()]), photos: HashMap::new(), icons_written: Default::default(), recording_path: None,
            zoom_ratio, sent_zoom: None, sent_flash: None, sent_ev: None,
            touch_seen: false, finger_start: None, finger_last: None, suppress_clicks_until: 0.0,
            bar: Spring::default(), chip: Spring::default(), bar_drag: None, pinch: None,
            focus_drag: false, focus_sent_at: 0.0,
            pill_press: None, dial_drag: None, dial_log2: 0.0, dial_vel: 0.0, dial_hide_at: None, dial_sent_at: 0.0,
        };
        HOST.with(|h| *h.borrow_mut() = Some(host));
        unsafe { octoscript_set_event_handler(on_event) };
        arkui::set_touch_handler(Some(on_touch));
        HOST.with(|h| { if let Some(host) = h.borrow_mut().as_mut() { host.remount(); } });
        info!("camera-oh: host created");
        Some(())
    }

    /// Render the session into a fresh chrome tree and swap it under the surface.
    pub fn remount(&mut self) {
        let started = std::time::Instant::now();
        self.session.now = now();
        let scene = self.session.render("oh");
        let image_src = |name: &str| {
            if let Some(p) = self.photos.get(name) { return Some(format!("file://{p}")); }
            let svg = scene.assets.get(name)?;
            let path = format!("{}/icons/{name}", self.files_dir);
            if !self.icons_written.borrow().contains(name) {
                let _ = std::fs::create_dir_all(format!("{}/icons", self.files_dir));
                if let Err(e) = std::fs::write(&path, svg.as_bytes()) { error!("camera-oh: icon {name}: {e}"); return None; }
                self.icons_written.borrow_mut().insert(name.to_owned());
            }
            Some(format!("file://{path}"))
        };
        let Some(mounted) = mount::build(&scene, &image_src) else { error!("camera-oh: scene did not build"); return };
        if let Some(old) = self.chrome.take() { unsafe { octoscript_remove_child(self.outer.raw(), old.root.raw()) }; drop(old); }
        unsafe { octoscript_add_child(self.outer.raw(), mounted.root.raw()) };
        self.chrome = Some(mounted);
        self.mounted_revision = Some(self.session.revision);
        // the viewfinder box follows the mode (4:3 photo, 16:9 video)
        let (vx, vy, vw, vh) = self.session.viewfinder();
        if (vx, vy, vw, vh) != self.surface_rect {
            self.surface_rect = (vx, vy, vw, vh);
            unsafe { mount::set_position(self.surface.raw(), vx as f32, vy as f32); Node::set_f32_attr_raw(self.surface.raw(), attr::width(), vw as f32); Node::set_f32_attr_raw(self.surface.raw(), attr::height(), vh as f32); }
        }
        let hidden = matches!(self.session.overlay, session::Overlay::Settings(_) | session::Overlay::Gallery);
        unsafe { mount::set_visible(self.surface.raw(), !hidden) };
        // a spring in flight carries on from the fresh layout
        self.apply_motion();
        info!("camera-oh: mount took {:.1} ms ({} nodes)", started.elapsed().as_secs_f64() * 1e3, scene.nodes.len());
    }

    // ---- node helpers -------------------------------------------------------

    fn handle(&self, id: &str) -> Option<NodeHandle> { self.chrome.as_ref()?.handles.get(id).copied() }
    fn rel(&self, id: &str) -> Option<(f64, f64)> { self.chrome.as_ref()?.rel.get(id).copied() }
    fn set_text(&self, id: &str, text: &str) { if let Some(h) = self.handle(id) { unsafe { mount::set_text(h, text) } } }
    /// Move a strip to its laid-out place plus an offset (parent-relative, artboard units).
    fn move_strip(&self, id: &str, dx: f64, dy: f64) {
        if let (Some(h), Some((x, y))) = (self.handle(id), self.rel(id)) { unsafe { mount::set_position(h, (x + dx) as f32, (y + dy) as f32) } }
    }
    fn control_rects(&self) -> impl Iterator<Item = (f64, f64, f64, f64)> + '_ {
        self.chrome.iter().flat_map(|m| m.controls.values().filter_map(move |id| m.rects.get(id).copied()))
    }
    fn on_control(&self, x: f64, y: f64) -> bool { self.control_rects().any(|(rx, ry, rw, rh)| x >= rx && x <= rx + rw && y >= ry && y <= ry + rh) }
    /// Like `on_control`, ignoring the viewfinder-wide scrims some overlays put under their chrome.
    fn on_small_control(&self, x: f64, y: f64) -> bool { self.control_rects().any(|(rx, ry, rw, rh)| rw < 300.0 && x >= rx && x <= rx + rw && y >= ry && y <= ry + rh) }

    /// Put the sliding strips where the springs (or the finger) say, in place, without a remount.
    fn apply_motion(&mut self) {
        self.move_strip("mode_strip", self.bar.pos, 0.0);
        self.move_strip("zoom_chip_strip", self.chip.pos, 0.0);
    }

    /// The session changed: animate the bar/chip from where they were and push the new lens, flash and EV to the camera.
    fn after_change(&mut self, before: (Option<usize>, usize, f64)) {
        let (old_bar, old_zoom, drag_offset) = before;
        let (new_bar, new_zoom) = (self.session.mode_bar_index(), self.session.zoom_index());
        match (old_bar, new_bar) {
            (Some(a), Some(b)) if a != b => self.bar.kick((b as f64 - a as f64) * MODE_PITCH + drag_offset),
            (Some(_), Some(_)) if drag_offset != 0.0 => self.bar.kick(drag_offset),
            _ => self.bar.hold(0.0),
        }
        if old_bar == new_bar && old_zoom != new_zoom { self.chip.kick((old_zoom as f64 - new_zoom as f64) * CHIP_PITCH); } else if old_bar != new_bar { self.chip.hold(0.0); }
        self.zoom_ratio = self.session.zoom_ratio();
        self.sync_camera();
        if self.session.mode.is_video() && !self.mic_asked { self.mic_asked = true; self.requests.push_back("permission|microphone".into()); }
    }

    // ---- clicks -------------------------------------------------------------

    fn on_click(&mut self, target: i32) {
        let t = now();
        if t < self.suppress_clicks_until { return; }
        let Some(control) = self.chrome.as_ref().and_then(|m| m.controls.get(&target).cloned()) else { return };
        // a tap on any other control while the dial is up takes the dial down first, as on the phone
        if self.session.overlay == session::Overlay::ZoomDial { self.close_dial(); }
        let (was_video, was_exposing) = (self.session.mode.is_video(), matches!(self.session.overlay, session::Overlay::Exposing { .. }));
        self.session.now = t;
        let before = (self.session.mode_bar_index(), self.session.zoom_index(), 0.0);
        let changed = self.session.activate(&control);
        self.after_change(before);
        if control == "shutter" && !was_video && !was_exposing { self.take_photo(); }
        if control == "shutter" && was_video { self.toggle_recording(); }
        if control == "rec_pause" && was_video {
            let paused = matches!(self.session.recording, session::Recording::Paused { .. });
            self.camera.capture(if paused { Capture::PauseVideo } else { Capture::ResumeVideo });
        }
        if changed { self.remount(); }
    }

    // ---- touches ------------------------------------------------------------

    /// ArkUI reports touch coordinates in vp relative to the page stack, which is the artboard.
    fn to_artboard(&mut self, x: f32, y: f32) -> (f64, f64) {
        if !self.touch_seen { self.touch_seen = true; info!("camera-oh: first touch at {x:.0},{y:.0} vp"); }
        (x as f64, y as f64)
    }

    /// One raw touch on the page: pinch, mode-bar drag, focus drag, the zoom dial, swipes and focus taps.
    pub fn on_touch(&mut self, action: i32, x: f32, y: f32, fingers: i32, x2: f32, y2: f32) {
        let p = self.to_artboard(x, y);
        let t = now();
        // Two fingers on the viewfinder: pinch to zoom the real lens; the selected chip shows the live value.
        if fingers >= 2 && action == TOUCH_MOVE && self.preview == PreviewState::Running && self.session.overlay == session::Overlay::None {
            let q = self.to_artboard(x2, y2);
            let dist = (p.0 - q.0).hypot(p.1 - q.1).max(1.0);
            let (d0, r0) = *self.pinch.get_or_insert((dist, self.zoom_ratio));
            let (lo, hi) = self.session.zoom_range();
            let ratio = (r0 * (dist / d0) as f32).clamp(lo, hi);
            if (ratio - self.zoom_ratio).abs() > 0.02 {
                self.zoom_ratio = ratio;
                self.camera.control(Control::Zoom(ratio));
                self.sent_zoom = Some(ratio);
                self.session.zoom_live = Some(ratio);
                let items = self.session.mode.zooms(self.session.front);
                let sel = self.session.zoom_index().min(items.len().saturating_sub(1));
                if let Some(label) = items.get(sel) { self.set_text(&format!("zoom_{}_label", label.trim_end_matches('x').to_lowercase()), &format!("{ratio:.1}x")); }
            }
            self.finger_start = None; self.bar_drag = None; self.pill_press = None;
            self.suppress_clicks_until = t + 0.3;
            return;
        }
        let mut moved = None;
        let mut ended = None;
        match action {
            TOUCH_DOWN => {
                self.finger_start = Some(p); self.finger_last = Some(p); self.pinch = None;
                if self.on_zoom_pill(p.0, p.1) { self.pill_press = Some((p.0, p.1, t)); }
                if self.session.overlay == session::Overlay::ZoomDial { self.dial_hide_at = None; self.dial_vel = 0.0; }
            }
            TOUCH_MOVE => moved = Some(p),
            _ => { ended = Some(p); self.pinch = None; }
        }
        // A finger on the mode bar drags the strip; it snaps to an entry on release.
        if let (Some((x, y)), Some((x0, y0))) = (moved, self.finger_start) {
            // A sideways slide on the quick-zoom bar opens the roulette dial; with the dial up any slide turns it.
            if self.dial_drag.is_none() && self.pill_press.is_some() && (x - x0).abs() > 6.0 && self.session.overlay == session::Overlay::None { self.open_dial(); }
            if self.session.overlay == session::Overlay::ZoomDial {
                // a still finger (a tap on the shutter, say) leaves the dial alone
                if self.dial_drag.is_none() && (x - x0).abs() < DIAL_SLIDE { self.finger_last = Some((x, y)); return; }
                let (last_x, last_t, vel) = self.dial_drag.unwrap_or((x, t, 0.0));
                let dx = x - last_x;
                // the ring turns with the finger: an arc length of dx at radius R, 17° per octave
                let dlog2 = -(dx / session::DIAL_R).to_degrees() / session::DEG_PER_OCTAVE as f64;
                let dt = (t - last_t).max(1e-3);
                self.dial_drag = Some((x, t, 0.5 * vel + 0.5 * dlog2 / dt));
                self.dial_log2 += dlog2;
                self.suppress_clicks_until = t + 0.3;
                self.apply_dial(false);
                self.finger_last = Some((x, y));
                return;
            }
            // With the focus box up, the finger drags it (and the camera's focus point) instead of swiping modes.
            let focus_up = matches!(self.session.overlay, session::Overlay::Focus(..));
            if self.bar_drag.is_none() && focus_up && !self.on_small_control(x0, y0) && (self.focus_drag || (x - x0).hypot(y - y0) > 6.0) {
                self.focus_drag = true;
                self.drag_focus(x, y, false);
            }
            if self.bar_drag.is_none() && !self.focus_drag && self.on_mode_bar(x0, y0) && (x - x0).abs() > 4.0 && (x - x0).abs() > (y - y0).abs() {
                let index0 = self.session.mode_bar_index().unwrap_or(3);
                self.bar_drag = Some(BarDrag { start_x: x0, index0, last_x: x, last_t: t, velocity: 0.0, moved: false });
            }
            if let Some(mut drag) = self.bar_drag {
                let raw = x - drag.start_x;
                let (lo, hi) = ((drag.index0 as f64 - (session::Mode::BAR.len() - 1) as f64) * MODE_PITCH, drag.index0 as f64 * MODE_PITCH);
                let offset = if raw < lo { lo + (raw - lo) * 0.3 } else if raw > hi { hi + (raw - hi) * 0.3 } else { raw };
                let dt = (t - drag.last_t).max(1e-3);
                drag.velocity = 0.6 * drag.velocity + 0.4 * (x - drag.last_x) / dt;
                drag.last_x = x; drag.last_t = t; drag.moved = drag.moved || raw.abs() > 8.0;
                if drag.moved { self.suppress_clicks_until = t + 0.3; }
                self.bar_drag = Some(drag);
                self.bar.hold(offset);
                self.apply_motion();
            }
            self.finger_last = Some((x, y));
        }
        let Some((x1, y1)) = ended else { return };
        self.pill_press = None;
        if let Some((_, _, vel)) = self.dial_drag.take() {
            self.dial_vel = vel.clamp(-12.0, 12.0);
            self.dial_hide_at = Some(t + DIAL_HIDE_AFTER);
            if self.dial_vel.abs() <= 0.02 { self.apply_dial(true); }
            self.finger_start = None;
            self.suppress_clicks_until = t + 0.3;
        } else if self.session.overlay == session::Overlay::ZoomDial {
            self.dial_hide_at = Some(t + DIAL_HIDE_AFTER);
            self.finger_start = None;
        }
        if self.focus_drag {
            self.focus_drag = false;
            self.drag_focus(x1, y1, true);
            self.finger_start = None;
            self.suppress_clicks_until = t + 0.3;
        }
        if let Some(drag) = self.bar_drag.take() {
            self.finger_start = None;
            self.end_bar_drag(drag);
        }
        let Some((x0, y0)) = self.finger_start.take() else { return };
        let (dx, dy) = (x1 - x0, y1 - y0);
        self.session.now = t;
        let before = (self.session.mode_bar_index(), self.session.zoom_index(), 0.0);
        if dx.abs() > 40.0 || dy.abs() > 40.0 {
            let from_bottom = y0 > 600.0;
            let changed = self.session.swipe(dx, dy, from_bottom);
            self.after_change(before);
            if changed { self.remount(); }
        } else if !self.on_control(x1, y1) && self.session.focus_tap(x1, y1) {
            self.remount();
            let (_, vy, vw, vh) = self.session.viewfinder();
            self.camera.control(Control::FocusPoint { x: (x1 / vw).clamp(0.0, 1.0), y: ((y1 - vy) / vh).clamp(0.0, 1.0) });
        }
    }

    fn on_mode_bar(&self, x: f64, y: f64) -> bool {
        let s = &self.session;
        (576.6..=632.6).contains(&y) && (0.0..=406.0).contains(&x) && s.overlay == session::Overlay::None && s.mode_bar_index().is_some() && s.recording == session::Recording::Off
    }

    /// The finger left the mode bar: pick the entry it points at (with a little fling) and let the spring settle.
    fn end_bar_drag(&mut self, drag: BarDrag) {
        let offset = self.bar.pos;
        if !drag.moved { self.bar.hold(0.0); return; }
        let projected = offset + drag.velocity * 0.12;
        let target = (drag.index0 as f64 - projected / MODE_PITCH).round().clamp(0.0, (session::Mode::BAR.len() - 1) as f64) as usize;
        let before = (Some(drag.index0), self.session.zoom_index(), offset);
        self.session.now = now();
        let selected = self.session.select_mode_index(target);
        self.after_change(before);
        if selected { self.remount(); } else { self.apply_motion(); }
    }

    /// Slide the focus reticle under the finger and re-aim the real camera (throttled).
    fn drag_focus(&mut self, x: f64, y: f64, done: bool) {
        if !self.session.move_focus(x, y) { return; }
        let (bx, by) = self.session.focus_box(x, y);
        if let Some(h) = self.handle("focus_strip") { unsafe { mount::set_position(h, (bx - 32.0) as f32, by as f32) } }
        let t = now();
        if done || t - self.focus_sent_at > 0.08 {
            self.focus_sent_at = t;
            let (_, vy, vw, vh) = self.session.viewfinder();
            self.camera.control(Control::FocusPoint { x: (x / vw).clamp(0.0, 1.0), y: ((y - vy) / vh).clamp(0.0, 1.0) });
        }
    }

    /// The quick-zoom pill's rectangle in artboard units, when a mode shows one.
    fn zoom_pill_rect(&self) -> Option<(f64, f64, f64, f64)> {
        let s = &self.session;
        let n = s.mode.zooms(s.front).len();
        if n == 0 || s.overlay != session::Overlay::None || s.recording != session::Recording::Off { return None; }
        let w = 40.0 * n as f64;
        Some((203.0 - w / 2.0, if s.mode == session::Mode::Pro { 441.5 } else { 517.5 }, w, 40.0))
    }
    fn on_zoom_pill(&self, x: f64, y: f64) -> bool {
        self.zoom_pill_rect().map_or(false, |(px, py, pw, ph)| x >= px && x <= px + pw && y >= py && y <= py + ph)
    }

    /// Show the roulette dial at the current zoom.
    fn open_dial(&mut self) {
        if self.session.overlay == session::Overlay::ZoomDial { return; }
        self.dial_log2 = (self.session.zoom_ratio() as f64).log2();
        self.dial_vel = 0.0;
        self.session.overlay = session::Overlay::ZoomDial;
        self.session.revision += 1;
        self.suppress_clicks_until = now() + 0.3;
        self.remount();
        self.apply_dial(true);
    }

    /// Turn the ring so the current zoom sits under the dot, rewrite the readouts, aim the lens.
    fn apply_dial(&mut self, force: bool) {
        let (lo, hi) = self.session.zoom_range();
        self.dial_log2 = self.dial_log2.clamp((lo as f64).log2(), (hi as f64).log2());
        let zoom = 2f64.powf(self.dial_log2) as f32;
        if let Some(h) = self.handle("zoom_dial") {
            let angle = -(self.dial_log2 * session::DEG_PER_OCTAVE as f64) as f32;
            unsafe { Node::set_f32v_raw(h, attr::rotate(), &[0.0, 0.0, 1.0, angle, 0.0]) };
        }
        let text = format!("{zoom:.1}x");
        self.set_text("dial_value", &text);
        self.set_text("dial_big", &text);
        // the stop labels ride along the ring
        let current = self.dial_log2 as f32;
        for (label, stop, _) in self.session.zoom_stops() {
            let id = format!("dial_stop_{}", label.trim_end_matches('x').to_lowercase());
            let Some(h) = self.handle(&id) else { continue };
            let octaves = stop.log2() - current;
            let (x, y) = session::dial_stop_pos(octaves);
            // a stop right under the readout gives way to it, as on the phone; one past the ring's ends is off
            let shown = octaves.abs() > 0.22 && y < self.session.preview_bottom() - 24.0;
            unsafe { mount::set_position(h, (x - 30.0) as f32, y as f32); mount::set_visible(h, shown); }
        }
        self.session.zoom_live = Some(zoom);
        self.zoom_ratio = zoom;
        let t = now();
        if force || t - self.dial_sent_at > 0.06 {
            self.dial_sent_at = t;
            self.camera.control(Control::Zoom(zoom));
            self.sent_zoom = Some(zoom);
        }
    }

    /// The dial goes away: the chips come back with the live value on the selected one.
    fn close_dial(&mut self) {
        if self.session.overlay != session::Overlay::ZoomDial { return; }
        self.dial_hide_at = None;
        self.dial_vel = 0.0;
        self.session.overlay = session::Overlay::None;
        self.session.snap_zoom_to_chip();
        self.remount();
        let zoom = 2f64.powf(self.dial_log2) as f32;
        self.camera.control(Control::Zoom(zoom));
        self.sent_zoom = Some(zoom);
    }

    /// The recording timer and the long-exposure countdown tick without a remount.
    fn refresh_timer(&mut self) {
        if let Some(text) = self.session.exposing_text() { self.set_text("exposing_count", &text); return; }
        if self.session.recording == session::Recording::Off { return; }
        let text = self.session.timer_text();
        self.set_text("rec_timer", &text);
    }

    // ---- the ability -------------------------------------------------------

    /// The window is edge to edge; the artboard starts under the system status bar.
    pub fn set_top_inset(&mut self, top_vp: f32) {
        TOP_INSET.set(top_vp);
        unsafe { mount::set_position(self.outer.raw(), 0.0, top_vp) };
    }

    pub fn back(&mut self) -> bool {
        self.session.now = now();
        let handled = self.session.back();
        if handled { self.session.revision += 1; self.remount(); }
        handled
    }

    pub fn permission_result(&mut self, name: &str, granted: bool) {
        match name {
            "camera" => { self.permission = Some(granted); info!("camera-oh: camera permission {granted}"); }
            "microphone" => { self.mic = Some(granted); }
            _ => {}
        }
    }

    pub fn capture_saved(&mut self, path: &str, ok: bool) {
        info!("camera-oh: gallery {} {path}", if ok { "took" } else { "refused" });
    }

    // ---- the camera --------------------------------------------------------

    fn drive_preview(&mut self) {
        if self.permission != Some(true) { return; }
        let surface = crate::xcomp::SURFACE_ID.load(std::sync::atomic::Ordering::SeqCst);
        if surface == 0 { return; }
        let want_front = self.session.front;
        let want_aspect = if self.session.mode.wide() { 16.0 / 9.0 } else { 4.0 / 3.0 };
        if self.preview == PreviewState::Running && want_front == self.preview_front && (want_aspect - self.preview_aspect).abs() < 0.05 { return; }
        if self.preview == PreviewState::Running { self.camera.stop_preview(); self.preview = PreviewState::Idle; }
        match self.camera.start_preview(want_front, want_aspect, surface) {
            Ok((w, h)) => {
                info!("camera-oh: preview {w}x{h} on surface {surface}");
                self.preview = PreviewState::Running; self.preview_front = want_front; self.preview_aspect = want_aspect;
                self.sent_zoom = None; self.sent_flash = None; self.sent_ev = None;
                self.zoom_ratio = self.session.zoom_ratio();
                if self.session.placeholder { self.session.placeholder = false; self.session.revision += 1; }
                self.sync_camera();
            }
            Err(e) => { error!("camera-oh: preview: {e}"); self.permission = None; self.requests.push_back("permission|camera".into()); }
        }
    }

    /// Send what the UI state asks of the real camera, only when it changed.
    fn sync_camera(&mut self) {
        if self.preview != PreviewState::Running { return; }
        if self.sent_zoom != Some(self.zoom_ratio) { self.sent_zoom = Some(self.zoom_ratio); self.camera.control(Control::Zoom(self.zoom_ratio)); }
        let flash = self.session.flash;
        if self.sent_flash != Some(flash) {
            self.sent_flash = Some(flash);
            self.camera.control(Control::Flash(match flash { session::Flash::Off => 0, session::Flash::On => 1, session::Flash::Auto => 2, session::Flash::Torch => 3 }));
        }
        let ev = self.session.ev_bias();
        if self.sent_ev != Some(ev) { self.sent_ev = Some(ev); self.camera.control(Control::ExposureBias(ev)); }
    }

    fn take_photo(&mut self) {
        if self.preview != PreviewState::Running { return; }
        let path = format!("{}/DCIM/IMG_{}.jpg", self.files_dir, now() as u64);
        self.camera.capture(Capture::Photo { path });
    }

    fn toggle_recording(&mut self) {
        if self.preview != PreviewState::Running { return; }
        if self.session.recording != session::Recording::Off {
            let path = format!("{}/DCIM/VID_{}.mp4", self.files_dir, now() as u64);
            self.recording_path = Some(path.clone());
            self.camera.capture(Capture::StartVideo { path, audio: self.mic == Some(true) });
        } else if self.recording_path.take().is_some() {
            self.camera.capture(Capture::StopVideo);
        }
    }

    pub fn next_request(&mut self) -> String { self.requests.pop_front().unwrap_or_default() }

    /// Every 16 ms from ArkTS: motion steps, time passes, results arrive.
    pub fn tick(&mut self) {
        let t = now();
        // springs and the coasting dial
        if self.bar_drag.is_none() { self.bar.step(t); }
        self.chip.step(t);
        if self.dial_drag.is_none() && self.dial_vel.abs() > 0.02 && self.session.overlay == session::Overlay::ZoomDial {
            // the ring coasts after a flick and slows exponentially (about a quarter second)
            let dt = 1.0 / 60.0;
            self.dial_log2 += self.dial_vel * dt;
            self.dial_vel *= (-dt / 0.22f64).exp();
            let (lo, hi) = self.session.zoom_range();
            if self.dial_log2 <= (lo as f64).log2() || self.dial_log2 >= (hi as f64).log2() { self.dial_vel = 0.0; }
            self.apply_dial(false);
            self.dial_hide_at = Some(t + DIAL_HIDE_AFTER);
        }
        if self.bar.active() || self.chip.active() || self.bar_drag.is_some() { self.apply_motion(); }
        if let Some((_, _, t0)) = self.pill_press { if t - t0 > 0.35 && self.session.overlay == session::Overlay::None { self.open_dial(); } }
        if let Some(at) = self.dial_hide_at { if t > at && self.dial_drag.is_none() { self.close_dial(); } }
        // the session's own clock
        if self.session.tick(t) { self.remount(); }
        if self.mounted_revision != Some(self.session.revision) { self.remount(); } else { self.refresh_timer(); }
        self.drive_preview();
        for result in crate::camera::take_results() {
            match result {
                CaptureResult::Photo { path, width, height } => {
                    info!("camera-oh: photo {path} ({width}x{height})");
                    let name = format!("thumb_{}.jpg", self.session.shots);
                    self.photos.insert(name.clone(), path.clone());
                    self.session.last_photo = Some(name);
                    self.session.revision += 1;
                    self.requests.push_back(format!("save|{path}"));
                    self.remount();
                }
                CaptureResult::VideoStopped { path } => { info!("camera-oh: video saved {path}"); self.requests.push_back(format!("save|{path}")); }
                CaptureResult::Failed { what, error } => {
                    error!("camera-oh: {what} failed: {error}");
                    if what == "video" && self.session.recording != session::Recording::Off { self.session.recording = session::Recording::Off; self.recording_path = None; }
                    self.session.toast = Some((format!("拍摄失败：{error}"), t + 2.5)); self.session.revision += 1; self.remount();
                }
                other => info!("camera-oh: {other:?}"),
            }
        }
    }
}

extern "C" fn on_event(target: i32, _event_type: i32) {
    HOST.with(|h| { if let Some(host) = h.borrow_mut().as_mut() { host.on_click(target); } });
}

extern "C" fn on_touch(target: i32, action: i32, x: f32, y: f32, fingers: i32, x2: f32, y2: f32) {
    if target != TOUCH_TARGET { return; }
    HOST.with(|h| { if let Some(host) = h.borrow_mut().as_mut() { host.on_touch(action, x, y, fingers, x2, y2); } });
}
