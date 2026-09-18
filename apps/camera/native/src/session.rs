//! The camera's state and screens: what the Mate 70 Air camera shows and what
//! each tap does (docs/ux-map.md). `render` draws the current state as a scene
//! over the live preview, `activate` handles a control. Everything the device
//! cannot do here (real capture, XMAGE processing, …) is mocked with the same
//! UX: the state changes exactly as on the phone.
use crate::scene::{Align, Scene, BLACK, BLUE, BOX, BOX_CIRCLE, CHIP, CHIP_TEXT, DIM, DIVIDER, DOT, GREY, INK_DIM, MENU, PANEL, PANEL_SEG, PILL, PILL2, PRO_BAR, REC, RED, RESPILL, ROUND, TOAST, WHITE, ZOOM_PILL};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mode { Night, Snapshot, Portrait, Photo, Video, Pro, More, SuperMacro, HighRes, SlowMo, Aperture, TimeLapse, LightPainting, Panorama }

impl Mode {
    pub const BAR: [Mode; 7] = [Mode::Night, Mode::Snapshot, Mode::Portrait, Mode::Photo, Mode::Video, Mode::Pro, Mode::More];
    pub const MORE: [Mode; 7] = [Mode::SuperMacro, Mode::HighRes, Mode::SlowMo, Mode::Aperture, Mode::TimeLapse, Mode::LightPainting, Mode::Panorama];
    pub fn id(self) -> &'static str {
        match self { Mode::Night => "night", Mode::Snapshot => "snapshot", Mode::Portrait => "portrait", Mode::Photo => "photo", Mode::Video => "video", Mode::Pro => "pro", Mode::More => "more",
            Mode::SuperMacro => "supermacro", Mode::HighRes => "highres", Mode::SlowMo => "slowmo", Mode::Aperture => "aperture", Mode::TimeLapse => "timelapse", Mode::LightPainting => "lightpaint", Mode::Panorama => "panorama" }
    }
    pub fn label(self, locale: &str) -> &'static str {
        let en = locale == "en";
        match self {
            Mode::Night => if en { "Night" } else { "夜景" }, Mode::Snapshot => if en { "Snapshot" } else { "闪拍" }, Mode::Portrait => if en { "Portrait" } else { "人像" },
            Mode::Photo => if en { "Photo" } else { "拍照" }, Mode::Video => if en { "Video" } else { "录像" }, Mode::Pro => if en { "Pro" } else { "专业" }, Mode::More => if en { "More" } else { "更多" },
            Mode::SuperMacro => if en { "Super macro" } else { "超级微距" }, Mode::HighRes => if en { "High-res" } else { "高像素" }, Mode::SlowMo => if en { "Slow-mo" } else { "慢动作" },
            Mode::Aperture => if en { "Aperture" } else { "大光圈" }, Mode::TimeLapse => if en { "Time-lapse" } else { "延时摄影" }, Mode::LightPainting => if en { "Light painting" } else { "流光快门" },
            Mode::Panorama => if en { "Panorama" } else { "超清全景" },
        }
    }
    pub fn icon(self) -> &'static str {
        match self { Mode::SuperMacro => "macro", Mode::HighRes => "highres", Mode::SlowMo => "slowmo", Mode::Aperture => "aperture", Mode::TimeLapse => "timelapse", Mode::LightPainting => "lightpaint", Mode::Panorama => "panorama", _ => "camera" }
    }
    pub fn is_video(self) -> bool { matches!(self, Mode::Video | Mode::SlowMo | Mode::TimeLapse) }
    pub fn wide(self) -> bool { self.is_video() }
    pub fn is_sub(self) -> bool { Mode::MORE.contains(&self) }
    /// The quick-zoom options of a mode (rear camera).
    pub fn zooms(self, front: bool) -> &'static [&'static str] {
        if front { return &["W", "1x"]; }
        match self { Mode::Portrait | Mode::Aperture => &["1x", "2x", "3x"], Mode::SlowMo => &["1x", "2x"], Mode::Panorama => &["1x", "2x", "4x"], Mode::HighRes | Mode::LightPainting | Mode::More => &[], _ => &["W", "1x", "2x", "3x", "5x"] }
    }
    pub fn intro(self, locale: &str) -> Option<(&'static str, &'static str)> {
        let en = locale == "en";
        Some(match self {
            Mode::Pro => ("专业", if en { "Adjust metering (M), ISO, shutter (S), exposure (EV), focus (AF) and white balance (WB) freely; long-press the dotted EV·, AF·, WB· labels to lock them." } else { "使用专业模式，您可以在拍摄时自由调节测光方式 (M)、感光度 (ISO)、快门 (S)、曝光补偿 (EV)、对焦方式 (AF)、白平衡 (WB) 及设置中的高级功能，满足多样化、个性化拍摄需求，长按 EV·、AF·、WB· 带点图标可锁定参数" }),
            Mode::TimeLapse => ("延时摄影", if en { "Condenses a long recording into a short clip that replays how a scene changes." } else { "可将长时间录制的影像合成为短视频，在短时间内再现景物变化的过程。" }),
            Mode::Panorama => ("超清全景", if en { "Turn the phone steadily along the guide, keep the centre circle on the target; the frames are stitched into one wide picture." } else { "沿提示引导方向平稳转动手机，避免抖动，将拍摄的中心圆圈对准推荐点位，拍摄完成后相机自动将捕获的画面合成一张更大画幅的全景照片。适用于壮丽河山、城市俯瞰等场景的拍摄。" }),
            Mode::Portrait => ("人像", if en { "Blurs the background around people and applies skin smoothing; adjust the aperture and beauty level below the preview." } else { "人像模式突出人物主体，虚化背景；可在预览下方调节光圈与美肤。" }),
            _ => return None,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flash { Auto, Off, On, Torch }
impl Flash {
    pub fn icon(self) -> &'static str { match self { Flash::Auto => "flash_auto", Flash::Off => "flash_off", Flash::On => "flash_on", Flash::Torch => "flash_torch" } }
    pub fn label(self, locale: &str) -> &'static str { let en = locale == "en"; match self { Flash::Auto => if en { "Auto" } else { "自动" }, Flash::Off => if en { "Off" } else { "关闭" }, Flash::On => if en { "On" } else { "打开" }, Flash::Torch => if en { "Torch" } else { "常亮" } } }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Overlay {
    None,
    FlashPanel,
    FilterMenu,
    ResPanel,
    /// The treasure box, page 0 or 1.
    Box(usize),
    /// The More grid; `true` while editing the layout.
    More(bool),
    Intro(Mode),
    /// A ruler for the named parameter (beauty, aperture, night shutter, …).
    Ruler(Ruler),
    Focus(f64, f64),
    Settings(usize),
    BeautyDialog(bool),
    Gallery,
    ProParam(usize),
    /// A long exposure in progress: chrome hidden, countdown, cancel on the shutter.
    Exposing { until: f64 },
    /// The 小艺视觉 scanner page.
    Vision,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ruler { Beauty, Aperture, NightShutter, SlowRate, TimeLapse, MacroFocus }

#[derive(Clone, Debug, PartialEq)]
pub enum Recording { Off, Running { started: f64, paused_for: f64 }, Paused { elapsed: f64, since: f64 } }

#[derive(Clone, Debug)]
enum Action {
    SetMode(Mode), SwipeMode(i32), OpenFlash, SetFlash(Flash), ToggleLive, ToggleAi, ToggleStab, OpenFilter, SetFilter(usize), Zoom(usize),
    ToggleBox, BoxPage(usize), BoxSettings, BoxVision, BoxWatermark, BoxRatio, BoxExposure, BoxGrid, MoreEdit, MoreEditDone, MoreEditReset, Intro(Mode), CloseIntro, IntroMore,
    Shutter, Switch, Thumbnail, Back, OpenRuler(Ruler), RulerSet(Ruler, i32), Info, OpenRes, SetRes(usize), SetFps(usize),
    ProParam(usize), ProLock(usize), ProValue(usize, i32), ProFormat, ProVideo, NightSub, PanoDir, LightPaint(usize), SubModeClose, RecPause, RecStill,
    SettingsScroll(i32), SettingsToggle(usize), ExposureEffect, BeautyDialog(bool), BeautyDialogOk, ClosePanel,
}

pub struct Session {
    pub locale: String,
    pub mode: Mode,
    pub front: bool,
    pub flash: Flash,
    pub live_photo: bool,
    pub ai_compose: bool,
    pub stabilise: bool,
    pub grid: bool,
    pub watermark: bool,
    pub ratio: usize,
    pub filter: usize,
    pub zoom: HashMap<Mode, usize>,
    pub beauty: i32,
    pub beauty_on: bool,
    pub aperture: usize,
    pub night_shutter: usize,
    pub night_sub: usize,
    pub slow_rate: usize,
    pub timelapse: usize,
    pub macro_focus: usize,
    pub video_res: usize,
    pub video_fps: usize,
    pub pro: [usize; 6],
    pub pro_locked: [bool; 6],
    pub pro_raw: bool,
    pub pro_video: bool,
    pub pano_vertical: bool,
    pub light_paint: usize,
    pub overlay: Overlay,
    pub toast: Option<(String, f64)>,
    pub recording: Recording,
    pub intro_seen: HashSet<Mode>,
    pub shots: u32,
    pub settings: [bool; 8],
    pub low_light: bool,
    /// No live frames yet: paint the viewfinder placeholder in the scene.
    pub placeholder: bool,
    pub now: f64,
    pub panel_until: Option<f64>,
    controls: HashMap<String, Action>,
    pub revision: u64,
}

pub const FILTERS: [(&str, &str, &str); 8] = [("原色", "Original", "film"), ("鲜艳", "Vivid", "film_solid"), ("明快", "Bright", "film_solid"), ("黑白", "Mono", "film"), ("自然", "Natural", "petal"), ("胶片", "Film", "petal"), ("电影", "Cinema", "petal"), ("动漫", "Anime", "petal")];
pub const APERTURES: [&str; 10] = ["F0.95", "F1.2", "F1.4", "F2.0", "F2.4", "F2.8", "F4.0", "F5.6", "F8.0", "F16"];
pub const NIGHT_SHUTTERS: [&str; 7] = ["自动", "1s", "2s", "4s", "8s", "15s", "30s"];
pub const SLOW_RATES: [&str; 3] = ["4x", "8x", "32x"];
pub const RES: [(&str, &str, &str); 4] = [("4K", "16:9", "4K"), ("1080p", "全屏", "1080p"), ("1080p", "16:9", "1080p"), ("720p", "16:9", "720p")];
pub const FPS: [&str; 2] = ["30 fps", "60 fps"];
pub const PRO_LABELS: [&str; 6] = ["M", "ISO", "S", "EV•", "AF•", "WB•"];
pub const PRO_VALUES: [&[&str]; 6] = [&["matrix", "centre", "spot"], &["50", "100", "200", "400", "800", "1600", "3200", "6400"], &["1/4000", "1/1000", "1/500", "1/250", "1/125", "1/60", "1/30", "1/15", "1/8", "1/4", "1/2", "1\"", "4\"", "8\"", "30\""], &["-4", "-3", "-2", "-1", "0", "+1", "+2", "+3", "+4"], &["AF-S", "AF-C", "MF"], &["AWB", "2800K", "4000K", "5000K", "6500K", "8000K"]];
pub const LIGHT_PAINT: [(&str, &str); 4] = [("车水马龙", "Traffic trails"), ("光绘涂鸦", "Light graffiti"), ("丝绢流水", "Silky water"), ("绚丽星轨", "Star trails")];
pub const SETTINGS: [(&str, &str); 8] = [("相机智控", "Smart control"), ("拍照静音", "Mute shutter"), ("水平仪", "Level"), ("音量键功能", "Volume key"), ("笑脸抓拍", "Smile capture"), ("手势拍照", "Gesture capture"), ("地理位置", "Location tag"), ("高效视频格式", "Efficient video")];

fn t<'a>(locale: &str, cn: &'a str, en: &'a str) -> &'a str { if locale == "en" { en } else { cn } }

impl Session {
    pub fn new(locale: &str) -> Self {
        let mut zoom = HashMap::new();
        for m in Mode::BAR.iter().chain(Mode::MORE.iter()) { zoom.insert(*m, if m.zooms(false).first() == Some(&"W") { 1 } else { 0 }); }
        Session { locale: locale.into(), mode: Mode::Photo, front: false, flash: Flash::Off, live_photo: false, ai_compose: false, stabilise: false, grid: false, watermark: false, ratio: 0, filter: 0, zoom,
            beauty: 5, beauty_on: true, aperture: 4, night_shutter: 0, night_sub: 0, slow_rate: 1, timelapse: 0, macro_focus: 0, video_res: 2, video_fps: 0, pro: [0, 0, 4, 4, 1, 0], pro_locked: [false; 6], pro_raw: false, pro_video: false,
            pano_vertical: false, light_paint: 0, overlay: Overlay::None, toast: None, recording: Recording::Off, intro_seen: HashSet::new(), shots: 0, settings: [false, false, false, true, false, false, true, true], low_light: false, placeholder: true, now: 0.0, panel_until: None,
            controls: HashMap::new(), revision: 0 }
    }
    pub fn zoom_index(&self) -> usize { *self.zoom.get(&self.mode).unwrap_or(&0) }
    /// The selected quick-zoom chip as a lens ratio (`W` is the ultra-wide, 0.5×).
    pub fn zoom_ratio(&self) -> f32 {
        let items = self.mode.zooms(self.front);
        match items.get(self.zoom_index().min(items.len().saturating_sub(1))) {
            Some(&"W") => 0.5,
            Some(label) => label.trim_end_matches('x').parse().unwrap_or(1.0),
            None => 1.0,
        }
    }
    /// Exposure compensation the Pro strip asks for (EV steps), 0 elsewhere.
    pub fn ev_bias(&self) -> f32 {
        if self.mode != Mode::Pro { return 0.0; }
        PRO_VALUES[3].get(self.pro[3]).and_then(|v| v.trim_start_matches('+').parse().ok()).unwrap_or(0.0)
    }
    /// Where the current mode sits in the mode bar (None for the More sub-modes).
    pub fn mode_bar_index(&self) -> Option<usize> { Mode::BAR.iter().position(|m| *m == self.mode) }
    /// Select a mode-bar entry by index (a finger drag on the bar landed there).
    pub fn select_mode_index(&mut self, i: usize) -> bool {
        let Some(&m) = Mode::BAR.get(i) else { return false };
        if m == self.mode || self.overlay != Overlay::None { return false; }
        self.go(m); self.revision += 1; true
    }
    fn set_toast(&mut self, text: String) { self.toast = Some((text, self.now + 1.8)); }
    /// Time passes: toasts expire, the recording timer runs. Returns true when the screen changed.
    pub fn tick(&mut self, now: f64) -> bool {
        self.now = now;
        let mut changed = false;
        if let Some((_, until)) = &self.toast { if now > *until { self.toast = None; changed = true; } }
        if let Overlay::FlashPanel = self.overlay { if let Some(until) = self.panel_until { if now > until { self.overlay = Overlay::None; self.panel_until = None; changed = true; } } }
        if let Overlay::Exposing { until } = self.overlay { if now > until { self.overlay = Overlay::None; self.shots += 1; changed = true; } }
        if changed { self.revision += 1; }
        changed
    }
    pub fn elapsed(&self) -> f64 {
        match &self.recording { Recording::Off => 0.0, Recording::Running { started, paused_for } => self.now - started - paused_for, Recording::Paused { elapsed, .. } => *elapsed }
    }
    pub fn timer_text(&self) -> String { let s = self.elapsed().max(0.0) as u64; format!("{:02}:{:02}", s / 60, s % 60) }
    /// The live countdown of a long exposure, if one is running.
    pub fn exposing_text(&self) -> Option<String> { if let Overlay::Exposing { until } = self.overlay { Some(format!("{:.1}s", (until - self.now).max(0.0))) } else { None } }
    fn ctl(&mut self, id: &str, action: Action) -> String { self.controls.insert(id.into(), action); id.into() }
    fn go(&mut self, mode: Mode) {
        if self.recording != Recording::Off { self.recording = Recording::Off; }
        self.mode = mode;
        self.overlay = if mode == Mode::More { Overlay::More(false) } else if mode.intro(&self.locale).is_some() && !self.intro_seen.contains(&mode) { Overlay::Intro(mode) } else { Overlay::None };
        if mode == Mode::HighRes { self.set_toast(t(&self.locale, "AI 超清适合拍摄光照充足的静物和风景，拍摄时请持稳手机", "AI high-res suits well-lit still subjects and landscapes; hold the phone steady").into()); }
    }

    /// A tap on a control. Returns true when the screen needs re-rendering.
    pub fn activate(&mut self, control: &str) -> bool {
        let Some(action) = self.controls.get(control).cloned() else { return false };
        let locale = self.locale.clone();
        match action {
            Action::SetMode(m) => self.go(m),
            Action::SwipeMode(d) => {
                if !self.mode.is_sub() && self.overlay == Overlay::None {
                    let i = Mode::BAR.iter().position(|m| *m == self.mode).unwrap_or(3) as i32 + d;
                    if (0..Mode::BAR.len() as i32).contains(&i) { self.go(Mode::BAR[i as usize]); }
                }
            }
            Action::OpenFlash => { self.overlay = Overlay::FlashPanel; self.panel_until = Some(self.now + 3.0); }
            Action::SetFlash(f) => { self.flash = f; self.overlay = Overlay::None; self.panel_until = None; }
            Action::ToggleLive => { self.live_photo = !self.live_photo; if self.live_photo { self.set_toast(t(&locale, "动态照片已开启", "Live photo on").into()); } }
            Action::ToggleAi => { self.ai_compose = !self.ai_compose; if self.ai_compose { self.set_toast(t(&locale, "AI 辅助构图已开启，可在 1x、2x 焦段中生效，风光、建筑、人像类场景推荐构图概率更高", "AI composition on: works at 1x and 2x; landscapes, buildings and people get suggestions most often").into()); } }
            Action::ToggleStab => { self.stabilise = !self.stabilise; self.set_toast(t(&locale, if self.stabilise { "视频防抖已开启" } else { "视频防抖已关闭" }, if self.stabilise { "Stabilisation on" } else { "Stabilisation off" }).into()); }
            Action::OpenFilter => self.overlay = Overlay::FilterMenu,
            Action::SetFilter(i) => { self.filter = i; self.overlay = Overlay::None; }
            Action::Zoom(i) => { self.zoom.insert(self.mode, i); }
            Action::ToggleBox => self.overlay = if matches!(self.overlay, Overlay::Box(_)) { Overlay::None } else { Overlay::Box(0) },
            Action::BoxPage(p) => self.overlay = Overlay::Box(p),
            Action::BoxSettings => self.overlay = Overlay::Settings(0),
            Action::BoxVision => self.overlay = Overlay::Vision,
            Action::BoxWatermark => { self.watermark = !self.watermark; }
            Action::BoxRatio => self.overlay = Overlay::Box(2),
            Action::BoxExposure => self.overlay = Overlay::Box(3),
            Action::BoxGrid => { self.grid = !self.grid; }
            Action::MoreEdit => self.overlay = Overlay::More(true),
            Action::MoreEditDone | Action::MoreEditReset => self.overlay = Overlay::More(false),
            Action::Intro(m) => self.overlay = Overlay::Intro(m),
            Action::CloseIntro => { if let Overlay::Intro(m) = self.overlay { self.intro_seen.insert(m); } self.overlay = Overlay::None; }
            Action::IntroMore => self.set_toast(t(&locale, "更多介绍：请查看相机帮助（模拟）", "More: see the camera help (mock)").into()),
            Action::Shutter => self.shutter(),
            Action::Switch => { self.front = !self.front; if self.front && self.mode == Mode::Portrait { self.beauty_on = true; } }
            Action::Thumbnail => self.overlay = Overlay::Gallery,
            Action::Back => { self.back(); }
            Action::OpenRuler(r) => self.overlay = if self.overlay == Overlay::Ruler(r) { Overlay::None } else { Overlay::Ruler(r) },
            Action::RulerSet(r, v) => match r {
                Ruler::Beauty => self.beauty = v.clamp(0, 10),
                Ruler::Aperture => self.aperture = (v.max(0) as usize).min(APERTURES.len() - 1),
                Ruler::NightShutter => self.night_shutter = (v.max(0) as usize).min(NIGHT_SHUTTERS.len() - 1),
                Ruler::SlowRate => { self.slow_rate = (v.max(0) as usize).min(SLOW_RATES.len() - 1); self.overlay = Overlay::None; }
                Ruler::TimeLapse => { self.timelapse = (v.max(0) as usize).min(4); self.overlay = Overlay::None; }
                Ruler::MacroFocus if v < 0 => { self.ratio = (-v - 1) as usize; }
                Ruler::MacroFocus => { self.macro_focus = (v.max(0) as usize).min(1); self.overlay = Overlay::None; }
            },
            Action::Info => self.overlay = Overlay::Intro(self.mode),
            Action::OpenRes => self.overlay = Overlay::ResPanel,
            Action::SetRes(i) => { self.video_res = i; self.overlay = Overlay::None; }
            Action::SetFps(i) => { self.video_fps = i; self.overlay = Overlay::None; }
            Action::ProParam(i) => self.overlay = if self.overlay == Overlay::ProParam(i) { Overlay::None } else { Overlay::ProParam(i) },
            Action::ProLock(i) => { if i >= 3 { self.pro_locked[i] = !self.pro_locked[i]; } }
            Action::ProValue(i, v) => { self.pro[i] = (v.max(0) as usize).min(PRO_VALUES[i].len() - 1); }
            Action::ProFormat => { self.pro_raw = !self.pro_raw; self.set_toast(t(&locale, if self.pro_raw { "已切换为 RAW+JPG" } else { "已切换为 JPG" }, if self.pro_raw { "RAW+JPG" } else { "JPG" }).into()); }
            Action::ProVideo => { self.pro_video = !self.pro_video; }
            Action::NightSub => { self.night_sub = (self.night_sub + 1) % 2; self.set_toast(t(&locale, if self.night_sub == 0 { "超级夜景" } else { "夜景增强" }, if self.night_sub == 0 { "Super night" } else { "Night enhance" }).into()); }
            Action::PanoDir => { self.pano_vertical = !self.pano_vertical; }
            Action::LightPaint(i) => { self.light_paint = i; }
            Action::SubModeClose => self.go(Mode::Photo),
            Action::RecPause => match self.recording.clone() {
                Recording::Running { started, paused_for } => self.recording = Recording::Paused { elapsed: self.now - started - paused_for, since: self.now },
                Recording::Paused { elapsed, since } => self.recording = Recording::Running { started: self.now - elapsed - (self.now - since), paused_for: self.now - since },
                Recording::Off => {}
            },
            Action::RecStill => { self.shots += 1; self.set_toast(t(&locale, "已保存照片", "Photo saved").into()); }
            Action::SettingsScroll(d) => { if let Overlay::Settings(p) = self.overlay { self.overlay = Overlay::Settings((p as i32 + d).clamp(0, 1) as usize); } }
            Action::SettingsToggle(i) => { self.settings[i] = !self.settings[i]; }
            Action::ExposureEffect => self.set_toast(t(&locale, "曝光效果预览（模拟）", "Exposure preview (mock)").into()),
            Action::BeautyDialog(on) => self.overlay = Overlay::BeautyDialog(on),
            Action::BeautyDialogOk => { if let Overlay::BeautyDialog(on) = self.overlay { self.beauty_on = on; } self.overlay = Overlay::None; }
            Action::ClosePanel => { self.overlay = Overlay::None; self.panel_until = None; }
        }
        self.revision += 1;
        true
    }
    fn shutter(&mut self) {
        if self.mode.is_video() {
            match self.recording {
                Recording::Off => { self.recording = Recording::Running { started: self.now, paused_for: 0.0 }; self.overlay = Overlay::None; }
                _ => { self.recording = Recording::Off; self.shots += 1; self.set_toast(t(&self.locale, "视频已保存", "Video saved").into()); }
            }
        } else if let Overlay::Exposing { .. } = self.overlay {
            self.overlay = Overlay::None; // cancel
        } else if self.mode == Mode::Night || (self.mode == Mode::Photo && self.low_light) {
            self.overlay = Overlay::Exposing { until: self.now + 4.1 };
        } else {
            self.shots += 1;
            self.overlay = Overlay::None;
            if self.mode == Mode::Panorama { self.set_toast(t(&self.locale, "请沿箭头方向平稳转动手机", "Turn the phone steadily along the arrow").into()); }
        }
    }
    /// Back: closes the top-most overlay, then leaves a More sub-mode, else nothing (the host exits).
    pub fn back(&mut self) -> bool {
        match self.overlay.clone() {
            Overlay::None => { if self.mode.is_sub() { self.go(Mode::Photo); true } else if self.mode == Mode::More { self.go(Mode::Photo); true } else { false } }
            Overlay::Intro(m) => { self.intro_seen.insert(m); self.overlay = Overlay::None; true }
            Overlay::More(true) => { self.overlay = Overlay::More(false); true }
            Overlay::More(false) => { self.go(Mode::Photo); true }
            Overlay::Box(p) if p >= 2 => { self.overlay = Overlay::Box(0); true }
            Overlay::Settings(p) if p > 0 => { self.overlay = Overlay::Settings(0); true }
            Overlay::Settings(_) | Overlay::Vision => { self.overlay = Overlay::Box(0); true }
            _ => { self.overlay = Overlay::None; self.panel_until = None; true }
        }
    }
    /// A finger swipe on the preview: horizontal → mode, vertical from the bottom → treasure box.
    pub fn swipe(&mut self, dx: f64, dy: f64, from_bottom: bool) -> bool {
        if dx.abs() > 40.0 && dx.abs() > dy.abs() * 1.5 {
            if let Overlay::Box(p) = self.overlay { self.overlay = Overlay::Box(if dx < 0.0 { 1 } else { 0 }); let _ = p; self.revision += 1; return true; }
            if self.overlay == Overlay::None && !self.mode.is_sub() {
                let i = Mode::BAR.iter().position(|m| *m == self.mode).unwrap_or(3) as i32 + if dx < 0.0 { 1 } else { -1 };
                if (0..Mode::BAR.len() as i32).contains(&i) { self.go(Mode::BAR[i as usize]); self.revision += 1; return true; }
            }
        } else if dy < -40.0 && from_bottom && self.overlay == Overlay::None { self.overlay = Overlay::Box(0); self.revision += 1; return true; }
        else if dy > 40.0 && matches!(self.overlay, Overlay::Box(_)) { self.overlay = Overlay::None; self.revision += 1; return true; }
        false
    }
    /// A tap on the preview that no control took: focus + exposure UI.
    pub fn focus_tap(&mut self, x: f64, y: f64) -> bool {
        if self.overlay != Overlay::None || y < 39.4 || y > self.preview_bottom() { return false; }
        self.overlay = Overlay::Focus(x, y); self.revision += 1; true
    }
    pub fn preview_bottom(&self) -> f64 { 39.4 + if self.mode.wide() { 721.0 } else { 541.0 } }
    /// The viewfinder rectangle in artboard units.
    pub fn viewfinder(&self) -> (f64, f64, f64, f64) { (0.0, 39.4, 406.0, if self.mode.wide() { 721.0 } else { 541.0 }) }

    // ---- rendering ----------------------------------------------------------------------
    pub fn render(&mut self, asset_base: &str) -> Scene {
        self.controls.clear();
        let mut s = Scene::new(asset_base, None);
        let locale = self.locale.clone();
        // Black chrome above and below the viewfinder (the preview itself is the live texture underneath).
        let (_, vy, _, vh) = self.viewfinder();
        if self.placeholder && !matches!(self.overlay, Overlay::Settings(_) | Overlay::Gallery) { s.stack("viewfinder", "page", 0.0, vy, 406.0, vh, Some(if self.mode == Mode::Night { "2c2c2e" } else { "3a3a3c" }), 0.0, None); }
        s.stack("chrome_top", "page", 0.0, 0.0, 406.0, vy, Some(BLACK), 0.0, None);
        s.stack("chrome_bottom", "page", 0.0, vy + vh, 406.0, 776.0 - vy - vh, Some(BLACK), 0.0, None);
        if self.grid && !self.mode.wide() { for k in 1..3 { s.stack(&format!("grid_v{k}"), "page", 406.0 * k as f64 / 3.0, vy, 0.7, vh, Some("ffffff80"), 0.0, None); s.stack(&format!("grid_h{k}"), "page", 0.0, vy + vh * k as f64 / 3.0, 406.0, 0.7, Some("ffffff80"), 0.0, None); } }
        if self.mode == Mode::Panorama { self.render_pano_guide(&mut s); }
        let box_open = matches!(self.overlay, Overlay::Box(_));
        if self.overlay == Overlay::Vision { self.render_vision(&mut s); return s; }
        if let Overlay::Exposing { until } = self.overlay {
            let id = self.ctl("shutter", Action::Shutter);
            s.button(&id, "page", 171.0, 659.7, 64.0, 64.0, true);
            for k in 0..24 { let a = std::f64::consts::TAU * k as f64 / 24.0; s.stack(&format!("shutter_dot{k}"), &id, 203.0 + 30.5 * a.cos() - 1.5, 691.7 + 30.5 * a.sin() - 1.5, 3.0, 3.0, Some(WHITE), 1.5, None); }
            s.stack("shutter_cancel", &id, 190.0, 678.7, 26.0, 26.0, Some(INK_DIM), 6.0, None);
            self.render_exposing(&mut s, until);
            return s;
        }
        self.render_top_bar(&mut s);
        if !box_open && self.recording == Recording::Off && !matches!(self.overlay, Overlay::Ruler(_) | Overlay::ProParam(_)) { self.render_params(&mut s); }
        if !box_open && !matches!(self.overlay, Overlay::Ruler(_) | Overlay::ProParam(_) | Overlay::More(_)) { self.render_zoom(&mut s); }
        if self.mode == Mode::Pro && !box_open && self.recording == Recording::Off { self.render_pro_bar(&mut s); }
        if !box_open && self.recording == Recording::Off { self.render_mode_bar(&mut s); }
        self.render_foot(&mut s, box_open);
        if !box_open { self.render_handle(&mut s, true, 724.6); }
        // Overlays, top-most last
        match self.overlay.clone() {
            Overlay::None => {}
            Overlay::FlashPanel => self.render_flash_panel(&mut s),
            Overlay::FilterMenu => self.render_filter_menu(&mut s),
            Overlay::ResPanel => self.render_res_panel(&mut s),
            Overlay::Box(page) => self.render_box(&mut s, page),
            Overlay::More(edit) => self.render_more(&mut s, edit),
            Overlay::Intro(m) => self.render_intro(&mut s, m),
            Overlay::Ruler(r) => self.render_ruler(&mut s, r),
            Overlay::Focus(x, y) => self.render_focus(&mut s, x, y),
            Overlay::Settings(page) => self.render_settings(&mut s, page),
            Overlay::BeautyDialog(on) => self.render_beauty_dialog(&mut s, on),
            Overlay::Gallery => self.render_gallery(&mut s),
            Overlay::ProParam(i) => self.render_pro_param(&mut s, i),
            Overlay::Exposing { .. } | Overlay::Vision => {}
        }
        if let Some((text, _)) = self.toast.clone() {
            let lines = if text.chars().count() > 22 { 2 } else { 1 };
            let w = if lines == 2 { 300.0 } else { (text.chars().count() as f64 * 12.0 + 32.0).min(300.0) };
            let h = if lines == 2 { 44.0 } else { 28.0 };
            s.stack("toast", "page", 203.0 - w / 2.0, 53.5, w, h, Some(TOAST), 12.0, None);
            if lines == 2 {
                let mid = text.chars().count() / 2; let a: String = text.chars().take(mid).collect(); let b: String = text.chars().skip(mid).collect();
                s.text("toast_text", "page", &a, 203.0 - w / 2.0, 55.5, w, 20.0, 12.0, false, WHITE, Align::Center);
                s.text("toast_text2", "page", &b, 203.0 - w / 2.0, 75.5, w, 20.0, 12.0, false, WHITE, Align::Center);
            } else { s.text("toast_text", "page", &text, 203.0 - w / 2.0, 53.5, w, h, 12.0, false, WHITE, Align::Center); }
        }
        let _ = locale;
        s
    }
    fn round_button(&mut self, s: &mut Scene, id: &str, action: Action, icon: &str, cx: f64, cy: f64, d: f64, bg: &str, size: f64) {
        let id = self.ctl(id, action);
        s.button(&id, "page", cx - d / 2.0, cy - d / 2.0, d, d, true);
        s.stack(&format!("{id}_bg"), &id, cx - d / 2.0, cy - d / 2.0, d, d, Some(bg), d / 2.0, None);
        s.icon(&format!("{id}_icon"), &id, icon, cx - size / 2.0, cy - size / 2.0, size, size, WHITE);
    }
    fn render_top_bar(&mut self, s: &mut Scene) {
        let locale = self.locale.clone();
        let mode = self.mode;
        if self.recording != Recording::Off {
            s.stack("rec_dot", "page", 165.0, 13.0, 10.0, 10.0, Some(if matches!(self.recording, Recording::Paused { .. }) { GREY } else { REC }), 5.0, None);
            s.text_w("rec_timer", "page", &self.timer_text(), 177.8, 3.1, 64.0, 30.2, 20.0, 500, WHITE, Align::Left);
            let items: Vec<(&str, &str, Action)> = vec![("flash", self.flash.icon(), Action::OpenFlash)];
            self.top_pill(s, &items, None);
            return;
        }
        // left
        match mode {
            Mode::Photo | Mode::Night | Mode::Portrait => self.round_button(s, "ai_compose", Action::ToggleAi, if self.ai_compose { "ai_compose_on" } else { "ai_compose" }, 34.0, 18.0, 36.0, ROUND, 20.0),
            Mode::Video | Mode::TimeLapse | Mode::SlowMo => self.res_pill(s),
            Mode::Pro => {
                let id = self.ctl("pro_format", Action::ProFormat);
                s.button(&id, "page", 12.0, 0.0, 52.0, 36.0, true);
                s.stack("pro_format_bg", &id, 12.0, 0.0, 52.0, 36.0, Some(ROUND), 18.0, None);
                s.text("pro_format_label", &id, if self.pro_raw { "RAW" } else { "JPG" }, 12.0, 0.0, 52.0, 36.0, 13.0, true, WHITE, Align::Center);
            }
            _ => {}
        }
        // right pill
        let flash = ("flash", self.flash.icon(), Action::OpenFlash);
        let live = ("live_photo", if self.live_photo { "live_on" } else { "live_off" }, Action::ToggleLive);
        let film = ("filter", "film", Action::OpenFilter);
        let stab = ("stabilise", "stabilise", Action::ToggleStab);
        let chip = Some(("filter", FILTERS[self.filter].0));
        match mode {
            Mode::Photo | Mode::Snapshot | Mode::SuperMacro | Mode::HighRes => if self.front || mode != Mode::Photo { self.top_pill(s, &[flash, live, film], None) } else { self.top_pill(s, &[flash, live], chip) },
            Mode::Portrait => if self.front { self.top_pill(s, &[flash, live], chip) } else { self.top_pill(s, &[], chip) },
            Mode::Video => if self.front { self.top_pill(s, &[flash], None) } else { self.top_pill(s, &[stab, flash, film], None) },
            Mode::Pro | Mode::SlowMo => self.top_pill(s, &[flash, film], None),
            Mode::Night | Mode::Aperture | Mode::TimeLapse | Mode::LightPainting => self.top_pill(s, &[film], None),
            Mode::More | Mode::Panorama => {}
        }
        if matches!(mode, Mode::Portrait | Mode::Pro | Mode::Night | Mode::TimeLapse | Mode::Panorama) {
            let id = self.ctl("info", Action::Info);
            s.button(&id, "page", 352.0, 55.5, 40.0, 40.0, true);
            s.icon("info_icon", &id, "info", 362.0, 65.5, 20.0, 20.0, WHITE);
        }
        if matches!(mode, Mode::Pro | Mode::TimeLapse) {
            let id = self.ctl("pro_exposure_effect", Action::ExposureEffect);
            s.button(&id, "page", 352.0, 99.2, 40.0, 40.0, true);
            s.icon("pro_exposure_effect_icon", &id, "sun", 362.0, 109.2, 20.0, 20.0, WHITE);
        }
        if mode == Mode::Photo && self.low_light {
            let id = self.ctl("night_enhance", Action::NightSub);
            s.button(&id, "page", 346.0, 457.0, 48.0, 48.0, true);
            s.icon("night_enhance_icon", &id, "night_enhance", 359.0, 461.0, 22.0, 22.0, WHITE);
            s.text_w("night_enhance_label", &id, t(&locale, "夜景增强", "Night enhance"), 340.0, 487.0, 60.0, 16.0, 11.0, 500, WHITE, Align::Center);
        }
    }
    fn top_pill(&mut self, s: &mut Scene, items: &[(&str, &str, Action)], chip: Option<(&str, &str)>) {
        if items.is_empty() && chip.is_none() { return; }
        let w = 52.0 * items.len() as f64 + if chip.is_some() { 78.0 } else { 0.0 } + 20.0;
        let x0 = 394.0 - w;
        s.stack("top_pill", "page", x0, 0.0, w, 36.0, Some(PILL), 18.0, None);
        let mut x = x0 + 10.0;
        for (id, icon, action) in items {
            let id = self.ctl(id, action.clone());
            s.button(&id, "page", x, 0.0, 52.0, 36.0, true);
            s.icon(&format!("{id}_icon"), &id, icon, x + 16.0, 8.0, 20.0, 20.0, WHITE);
            x += 52.0;
        }
        if let Some((id, label)) = chip {
            let id = self.ctl(id, Action::OpenFilter);
            s.button(&id, "page", x, 0.0, 68.0, 36.0, true);
            s.stack(&format!("{id}_frame"), &id, x + 4.0, 5.0, 60.0, 26.0, None, 13.0, Some(WHITE));
            s.icon(&format!("{id}_icon"), &id, "film", x + 10.0, 8.0, 20.0, 20.0, WHITE);
            s.text(&format!("{id}_label"), &id, label, x + 31.0, 5.0, 30.0, 26.0, 10.5, true, WHITE, Align::Left);
        }
    }
    fn res_pill(&mut self, s: &mut Scene) {
        let res = RES[self.video_res].0; let fps = if self.mode == Mode::SlowMo { "240 fps" } else { FPS[self.video_fps] };
        s.stack("res_pill", "page", 12.0, 0.0, 104.0, 36.0, Some(RESPILL), 18.0, None);
        let a = self.ctl("video_res", Action::OpenRes);
        s.button(&a, "page", 12.0, 0.0, 56.0, 36.0, true);
        s.text_w("video_res_label", &a, res, 24.0, 0.0, 42.0, 36.0, 13.0, 500, WHITE, Align::Left);
        s.stack("res_sep", "page", 68.0, 10.0, 1.0, 16.0, Some(GREY), 0.0, None);
        let b = self.ctl("video_fps", if self.mode == Mode::SlowMo { Action::OpenRuler(Ruler::SlowRate) } else { Action::OpenRes });
        s.button(&b, "page", 70.0, 0.0, 46.0, 36.0, true);
        s.text_w("video_fps_label", &b, fps, 76.0, 0.0, 40.0, 36.0, 13.0, 500, WHITE, Align::Left);
    }
    fn render_zoom(&mut self, s: &mut Scene) {
        let items = self.mode.zooms(self.front);
        if items.is_empty() { return; }
        let y = if self.mode == Mode::Pro { 441.5 } else { 517.5 };
        let w = 40.0 * items.len() as f64; let x0 = 203.0 - w / 2.0;
        let sel = self.zoom_index().min(items.len() - 1);
        s.stack("zoom_pill", "page", x0, y, w, 40.0, Some(ZOOM_PILL), 20.0, None);
        // The selected chip lives in its own strip so the module can slide it
        // between labels (the phone's selection glides with a spring).
        s.scroll("zoom_chip_strip", "page", x0, y, w, 40.0);
        s.stack("zoom_chip", "zoom_chip_strip", sel as f64 * 40.0, 0.0, 40.0, 40.0, Some(CHIP), 20.0, None);
        for (i, label) in items.iter().enumerate() {
            let id = self.ctl(&format!("zoom_{}", label.trim_end_matches('x').to_lowercase()), Action::Zoom(i));
            let x = x0 + i as f64 * 40.0;
            s.button(&id, "page", x, y, 40.0, 40.0, true);
            s.text(&format!("{id}_label"), &id, label, x, y, 40.0, 40.0, 13.0, true, if i == sel { CHIP_TEXT } else { WHITE }, Align::Center);
        }
    }
    fn param(&mut self, s: &mut Scene, id: &str, action: Action, icon: &str, label: &str, right: bool, x_override: Option<f64>) {
        let x = x_override.unwrap_or(if right { 340.0 } else { 0.0 });
        let id = self.ctl(id, action);
        s.button(&id, "page", x, 513.5, 60.0, 48.0, true);
        s.icon(&format!("{id}_icon"), &id, icon, x + 20.0, 519.5, 20.0, 20.0, WHITE);
        s.text_w(&format!("{id}_label"), &id, label, x, 541.5, 60.0, 20.0, 12.0, 500, WHITE, Align::Center);
    }
    fn render_params(&mut self, s: &mut Scene) {
        let locale = self.locale.clone();
        match self.mode {
            Mode::Portrait => {
                if self.front {
                    let label = if self.beauty_on { t(&locale, "美颜 开", "Beauty on") } else { t(&locale, "美颜 关", "Beauty off") };
                    self.param(s, "beauty", Action::BeautyDialog(self.beauty_on), "beauty", label, false, None);
                    self.param(s, "aperture", Action::OpenRuler(Ruler::Aperture), "aperture_solid", "F16", true, None);
                } else {
                    let label = format!("{} {}", t(&locale, "美肤", "Skin"), self.beauty);
                    self.param(s, "beauty", Action::OpenRuler(Ruler::Beauty), "beauty", &label, false, None);
                    self.param(s, "aperture", Action::OpenRuler(Ruler::Aperture), "aperture_solid", APERTURES[self.aperture], true, None);
                }
            }
            Mode::Video => {
                let label = if self.front { t(&locale, if self.beauty_on { "美颜 开" } else { "美颜 关" }, if self.beauty_on { "Beauty on" } else { "Beauty off" }).to_owned() } else { format!("{} {}", t(&locale, "美肤", "Skin"), 0) };
                self.param(s, "beauty", if self.front { Action::BeautyDialog(self.beauty_on) } else { Action::OpenRuler(Ruler::Beauty) }, "beauty", &label, false, None);
            }
            Mode::Night => {
                self.param(s, "night_shutter", Action::OpenRuler(Ruler::NightShutter), "shutter_s", t(&locale, NIGHT_SHUTTERS[self.night_shutter], if self.night_shutter == 0 { "Auto" } else { NIGHT_SHUTTERS[self.night_shutter] }), false, Some(6.2));
                self.param(s, "night_sub_mode", Action::NightSub, "moon", t(&locale, if self.night_sub == 0 { "超级夜景" } else { "夜景增强" }, if self.night_sub == 0 { "Super night" } else { "Enhance" }), true, None);
            }
            Mode::SuperMacro => self.param(s, "macro_af", Action::OpenRuler(Ruler::MacroFocus), "sliders", t(&locale, if self.macro_focus == 0 { "自动" } else { "手动" }, if self.macro_focus == 0 { "Auto" } else { "Manual" }), false, None),
            Mode::SlowMo => self.param(s, "slow_rate", Action::OpenRuler(Ruler::SlowRate), "slowmo", SLOW_RATES[self.slow_rate], true, None),
            Mode::Aperture => self.param(s, "aperture", Action::OpenRuler(Ruler::Aperture), "aperture_solid", APERTURES[self.aperture], true, None),
            Mode::TimeLapse => self.param(s, "timelapse_auto", Action::OpenRuler(Ruler::TimeLapse), "sliders", t(&locale, ["自动", "0.5s", "1s", "3s", "10s"][self.timelapse], ["Auto", "0.5s", "1s", "3s", "10s"][self.timelapse]), true, None),
            Mode::Panorama => self.param(s, "pano_dir", Action::PanoDir, "sun", t(&locale, if self.pano_vertical { "纵向" } else { "横向" }, if self.pano_vertical { "Vertical" } else { "Horizontal" }), true, None),
            Mode::LightPainting => {
                for (i, (cn, en)) in LIGHT_PAINT.iter().enumerate() {
                    let x = 27.7 + i as f64 * 88.6;
                    let id = self.ctl(&format!("lightpaint_{i}"), Action::LightPaint(i));
                    s.button(&id, "page", x, 498.5, 84.6, 84.6, true);
                    s.stack(&format!("{id}_tile"), &id, x, 498.5, 84.6, 84.6, Some(["5a4a3a", "6a4020", "3a5a6a", "2a3a6a"][i]), 4.0, if i == self.light_paint { Some(WHITE) } else { None });
                    s.text_w(&format!("{id}_label"), &id, t(&self.locale, cn, en), x + 4.0, 498.5 + 56.0, 76.6, 24.0, 12.0, 500, WHITE, Align::Left);
                }
            }
            _ => {}
        }
    }
    fn render_pro_bar(&mut self, s: &mut Scene) {
        s.stack("pro_bar", "page", 0.0, 501.8, 406.0, 60.0, Some(PRO_BAR), 0.0, None);
        for i in 0..6 {
            let x = 12.0 + i as f64 * 66.7;
            let id = self.ctl(&format!("pro_{i}"), Action::ProParam(i));
            s.button(&id, "page", x, 506.8, 48.0, 50.2, true);
            let active = self.overlay == Overlay::ProParam(i);
            s.text(&format!("{id}_label"), &id, PRO_LABELS[i], x, 508.8, 48.0, 20.0, 15.0, true, if active { "ffd54a" } else { WHITE }, Align::Center);
            if i == 0 { s.icon(&format!("{id}_icon"), &id, "metering", x + 16.0, 530.8, 16.0, 16.0, WHITE); }
            else { s.text_w(&format!("{id}_value"), &id, PRO_VALUES[i][self.pro[i]], x, 529.8, 48.0, 16.0, 12.0, 500, WHITE, Align::Center); }
            if i == 5 { s.stack(&format!("{id}_underline"), &id, x + 10.0, 546.8, 28.0, 1.5, Some(WHITE), 0.0, None); }
            if i >= 3 && self.pro_locked[i] { s.stack(&format!("{id}_lock"), &id, x + 40.0, 512.9, 8.0, 8.0, Some("ffd54a"), 4.0, None); }
        }
    }
    fn render_mode_bar(&mut self, s: &mut Scene) {
        let locale = self.locale.clone();
        if self.mode.is_sub() {
            let label = self.mode.label(&locale); let four = label.chars().count() >= 4;
            let w = if four { 104.0 } else { 90.0 }; let x = 203.0 - w / 2.0;
            let id = self.ctl("sub_mode_close", Action::SubModeClose);
            s.button(&id, "page", x, 589.8, w, 30.0, true);
            s.stack("sub_mode_bg", &id, x, 589.8, w, 30.0, Some(PILL2), 15.0, Some("ffffff66"));
            s.text("sub_mode_label", &id, label, x + 13.0, 596.6, w - 45.0, 16.3, 14.0, true, WHITE, Align::Left);
            s.icon("sub_mode_x", &id, "close", x + w - 29.0, 596.9, 16.0, 16.0, WHITE);
            return;
        }
        let i0 = Mode::BAR.iter().position(|m| *m == self.mode).unwrap_or(3) as i32;
        // All seven labels sit in one strip that the module slides (finger
        // tracking and the spring settle of the phone's mode bar); the ones
        // beyond the edges are clipped by the strip.
        // (coordinates inside the strip are relative to its top-left corner)
        s.scroll("mode_strip", "page", 0.0, 576.6, 406.0, 56.0);
        for (i, m) in Mode::BAR.iter().enumerate() {
            let cx = 203.0 + (i as i32 - i0) as f64 * 52.3;
            let x = cx - 26.0;
            let id = self.ctl(&format!("mode_{}", m.id()), Action::SetMode(*m));
            s.button(&id, "mode_strip", x, 0.0, 52.0, 56.0, true);
            let dist = (i as i32 - i0).abs();
            let color = if dist <= 2 { WHITE } else if dist == 3 { GREY } else { "3c3c3c" };
            s.text_w(&format!("{id}_label"), &id, m.label(&locale), x, 12.0, 52.0, 24.0, 16.0, if dist == 0 { 700 } else { 500 }, color, Align::Center);
        }
        s.stack("mode_dot", "page", 199.0, 618.5, 8.0, 8.0, Some(RED), 4.0, None);
    }
    fn render_shutter(&mut self, s: &mut Scene, cx: f64, cy: f64, d: f64) {
        let kind = match (self.mode, &self.recording) {
            (_, Recording::Running { .. }) | (_, Recording::Paused { .. }) => "recording",
            (Mode::Video | Mode::SlowMo, _) => "video",
            (Mode::TimeLapse, _) => "timelapse",
            (Mode::Night | Mode::HighRes | Mode::LightPainting, _) => "dotted",
            (Mode::Pro, _) if self.pro_video => "video",
            _ => "photo",
        };
        let r = d / 2.0;
        let id = self.ctl("shutter", Action::Shutter);
        s.button(&id, "page", cx - r, cy - r, d, d, true);
        if kind == "dotted" || kind == "timelapse" {
            for k in 0..24 { let a = std::f64::consts::TAU * k as f64 / 24.0; s.stack(&format!("shutter_dot{k}"), &id, cx + (r - 1.5) * a.cos() - 1.5, cy + (r - 1.5) * a.sin() - 1.5, 3.0, 3.0, Some(WHITE), 1.5, None); }
        } else {
            s.stack("shutter_ring", &id, cx - r, cy - r, d, d, Some(WHITE), r, None);
            s.stack("shutter_gap", &id, cx - r + 3.0, cy - r + 3.0, d - 6.0, d - 6.0, Some(BLACK), r - 3.0, None);
        }
        if kind == "recording" { s.stack("shutter_stop", &id, cx - 13.0, cy - 13.0, 26.0, 26.0, Some(REC), 6.0, None); return; }
        let inner = d - 16.0;
        s.stack("shutter_disc", &id, cx - inner / 2.0, cy - inner / 2.0, inner, inner, Some(WHITE), inner / 2.0, None);
        if kind == "video" || kind == "timelapse" { s.stack("shutter_rec", &id, cx - 13.0, cy - 13.0, 26.0, 26.0, Some(REC), 13.0, None); }
    }
    fn render_foot(&mut self, s: &mut Scene, box_open: bool) {
        let (tx, ty, td, scx, scy, sd, rx, ry, rd) = if box_open { (77.0, 533.0, 40.0, 203.0, 552.8, 52.0, 309.0, 553.0, 40.0) } else { (53.2, 669.8, 44.0, 203.0, 691.7, 64.0, 330.8, 691.8, 44.0) };
        let recording = self.recording != Recording::Off;
        if recording {
            let id = self.ctl("rec_pause", Action::RecPause);
            s.button(&id, "page", tx, ty, td, td, true);
            s.stack("rec_pause_bg", &id, tx, ty, td, td, Some("131313"), td / 2.0, Some(WHITE));
            s.icon("rec_pause_icon", &id, if matches!(self.recording, Recording::Paused { .. }) { "video_camera" } else { "pause" }, tx + 10.0, ty + 10.0, 24.0, 24.0, WHITE);
        } else {
            let id = self.ctl("thumbnail", Action::Thumbnail);
            s.button(&id, "page", tx, ty, td, td, true);
            s.stack("thumbnail_ring", &id, tx, ty, td, td, Some(WHITE), td / 2.0, None);
            let tint = ["b9b7b1", "8fa3b8", "b89f8f", "9fb88f"][(self.shots % 4) as usize];
            s.stack("thumbnail_img", &id, tx + 1.5, ty + 1.5, td - 3.0, td - 3.0, Some(tint), td / 2.0 - 1.5, None);
            s.stack("thumbnail_line", &id, tx + 3.0, ty + td / 2.0 - 1.0, td - 6.0, 2.0, Some("2a2a2a"), 0.0, None);
        }
        self.render_shutter(s, scx, scy, sd);
        if recording {
            let id = self.ctl("rec_still", Action::RecStill);
            s.button(&id, "page", rx - rd / 2.0, ry - rd / 2.0, rd, rd, true);
            s.stack("rec_still_ring", &id, rx - rd / 2.0, ry - rd / 2.0, rd, rd, Some(WHITE), rd / 2.0, None);
            s.stack("rec_still_gap", &id, rx - rd / 2.0 + 2.0, ry - rd / 2.0 + 2.0, rd - 4.0, rd - 4.0, Some(BLACK), rd / 2.0 - 2.0, None);
            s.stack("rec_still_disc", &id, rx - rd / 2.0 + 5.0, ry - rd / 2.0 + 5.0, rd - 10.0, rd - 10.0, Some(WHITE), rd / 2.0 - 5.0, None);
            return;
        }
        let right: Option<(&str, &str, Action)> = {
            match self.mode {
                Mode::HighRes | Mode::LightPainting | Mode::Panorama => None,
                Mode::Pro => Some(if self.pro_video { ("pro_video", "camera", Action::ProVideo) } else { ("pro_video", "video_camera", Action::ProVideo) }),
                Mode::SuperMacro | Mode::Aperture => Some(("pro_video", "video_camera", Action::ProVideo)),
                _ => Some(("switch_camera", "switch_camera", Action::Switch)),
            }
        };
        if let Some((id, icon, action)) = right {
            let id = self.ctl(id, action);
            s.button(&id, "page", rx - rd / 2.0, ry - rd / 2.0, rd, rd, true);
            s.stack(&format!("{id}_bg"), &id, rx - rd / 2.0, ry - rd / 2.0, rd, rd, Some("131313"), rd / 2.0, Some(WHITE));
            s.icon(&format!("{id}_icon"), &id, icon, rx - 12.0, ry - 12.0, 24.0, 24.0, WHITE);
        }
    }
    fn render_handle(&mut self, s: &mut Scene, up: bool, y: f64) {
        let id = self.ctl("treasure_handle", Action::ToggleBox);
        s.button(&id, "page", 172.9, y, 60.0, 32.0, true);
        s.icon("treasure_handle_icon", &id, if up { "chevron_up" } else { "chevron_down" }, 191.0, y + 4.0, 24.0, 24.0, INK_DIM);
    }
    fn segment(&mut self, s: &mut Scene, id: &str, x: f64, y: f64, w: f64, h: f64, options: &[(&str, Option<&str>, String, Action)], selected: usize, two_line: bool) {
        s.stack(&format!("{id}_track"), "page", x, y, w, h, Some(PANEL_SEG), h / 2.0, None);
        let ow = w / options.len() as f64;
        for (i, (oid, icon, label, action)) in options.iter().enumerate() {
            let ox = x + i as f64 * ow;
            let id = self.ctl(oid, action.clone());
            s.button(&id, "page", ox, y, ow, h, true);
            let sel = i == selected;
            if sel { s.stack(&format!("{id}_chip"), &id, ox + 2.0, y + 2.0, ow - 4.0, h - 4.0, Some(WHITE), (h - 4.0) / 2.0, None); }
            let color = if sel { BLACK } else { WHITE };
            if let Some(icon) = icon {
                s.icon(&format!("{id}_icon"), &id, icon, ox + ow / 2.0 - 10.0, y + 6.0, 20.0, 20.0, color);
                s.text_w(&format!("{id}_label"), &id, label, ox, y + 29.0, ow, 16.0, 13.0, 500, color, Align::Center);
            } else if two_line {
                let mut parts = label.splitn(2, '\n');
                let a = parts.next().unwrap_or(""); let b = parts.next().unwrap_or("");
                s.text_w(&format!("{id}_label"), &id, a, ox, y + 8.0, ow, 18.0, 13.0, 500, color, Align::Center);
                s.text_w(&format!("{id}_label2"), &id, b, ox, y + 26.0, ow, 18.0, 13.0, 500, color, Align::Center);
            } else { s.text_w(&format!("{id}_label"), &id, label, ox, y, ow, h, 14.0, 500, color, Align::Center); }
        }
    }
    fn render_flash_panel(&mut self, s: &mut Scene) {
        let locale = self.locale.clone();
        s.stack("flash_panel", "page", 16.0, 52.0, 374.0, 104.0, Some(PANEL), 24.0, None);
        s.text_w("flash_title", "page", t(&locale, "闪光灯", "Flash"), 28.0, 66.0, 200.0, 24.0, 16.0, 500, WHITE, Align::Left);
        let options: Vec<(&str, Option<&str>, String, Action)> = [Flash::Auto, Flash::Off, Flash::On, Flash::Torch].iter().map(|f| (match f { Flash::Auto => "flash_auto", Flash::Off => "flash_off_opt", Flash::On => "flash_on", Flash::Torch => "flash_torch" }, Some(f.icon()), f.label(&locale).to_owned(), Action::SetFlash(*f))).collect();
        let selected = [Flash::Auto, Flash::Off, Flash::On, Flash::Torch].iter().position(|f| *f == self.flash).unwrap_or(1);
        self.segment(s, "flash_seg", 28.0, 94.5, 350.0, 50.0, &options, selected, false);
    }
    fn render_filter_menu(&mut self, s: &mut Scene) {
        let scrim = self.ctl("scrim", Action::ClosePanel);
        s.button(&scrim, "page", 0.0, 36.0, 406.0, 540.0, true);
        s.stack("filter_menu", "page", 186.0, 44.0, 204.0, 416.0, Some(MENU), 16.0, None);
        for (i, (cn, en, icon)) in FILTERS.iter().enumerate() {
            let y = 48.0 + i as f64 * 50.5;
            let id = self.ctl(&format!("style_{i}"), Action::SetFilter(i));
            s.button(&id, "page", 190.0, y, 196.0, 50.5, true);
            s.icon(&format!("{id}_icon"), &id, icon, 202.0, y + 15.4, 20.0, 20.0, WHITE);
            s.text(&format!("{id}_label"), &id, t(&self.locale, cn, en), 230.0, y + 14.0, 112.0, 22.0, 16.0, false, WHITE, Align::Left);
            if i == self.filter { s.icon(&format!("{id}_check"), &id, "check", 350.0, y + 13.2, 24.0, 24.0, WHITE); }
            if i < 7 { s.stack(&format!("{id}_line"), "page", 202.0, y + 50.0, 176.0, 1.0, Some(DIVIDER), 0.0, None); }
        }
    }
    fn render_res_panel(&mut self, s: &mut Scene) {
        let locale = self.locale.clone();
        let scrim = self.ctl("scrim", Action::ClosePanel);
        s.button(&scrim, "page", 0.0, 36.0, 406.0, 540.0, true);
        s.stack("res_panel", "page", 16.0, 52.0, 374.0, 190.0, Some(PANEL), 24.0, None);
        s.text_w("res_title", "page", t(&locale, "视频分辨率", "Video resolution"), 28.0, 66.0, 200.0, 24.0, 16.0, 500, WHITE, Align::Left);
        let options: Vec<(&str, Option<&str>, String, Action)> = RES.iter().enumerate().map(|(i, (a, b, _))| (["res_4k", "res_1080_full", "res_1080", "res_720"][i], None, format!("{a}\n{b}"), Action::SetRes(i))).collect();
        self.segment(s, "res_seg", 28.0, 96.0, 350.0, 50.0, &options, self.video_res, true);
        s.text_w("fps_title", "page", t(&locale, "视频帧率", "Frame rate"), 28.0, 152.0, 200.0, 24.0, 16.0, 500, WHITE, Align::Left);
        let options: Vec<(&str, Option<&str>, String, Action)> = FPS.iter().enumerate().map(|(i, f)| (["fps_30", "fps_60"][i], None, f.to_string(), Action::SetFps(i))).collect();
        self.segment(s, "fps_seg", 28.0, 180.0, 350.0, 50.0, &options, self.video_fps, false);
    }
    fn render_box(&mut self, s: &mut Scene, page: usize) {
        let locale = self.locale.clone();
        s.stack("box", "page", 16.0, 586.5, 374.0, 204.0, Some(BOX), 24.0, None);
        if page >= 2 {
            // second level: ‹ title ×
            let back = self.ctl("box_back", Action::BoxPage(0)); s.button(&back, "page", 20.0, 590.5, 40.0, 40.0, true); s.icon("box_back_icon", &back, "back", 28.0, 598.5, 24.0, 24.0, WHITE);
            let close = self.ctl("box_close", Action::ClosePanel); s.button(&close, "page", 346.0, 590.5, 40.0, 40.0, true); s.icon("box_close_icon", &close, "close", 354.0, 598.5, 24.0, 24.0, WHITE);
            if page == 2 {
                s.text("box_title", "page", t(&locale, "照片比例", "Aspect ratio"), 60.0, 598.5, 286.0, 24.0, 16.0, true, WHITE, Align::Center);
                for (i, (cn, en)) in [("4:3", "4:3"), ("1:1", "1:1"), ("全屏", "Full")].iter().enumerate() {
                    let x = 74.7 + i as f64 * 85.5;
                    let id = self.ctl(&format!("ratio_{i}"), Action::RulerSet(Ruler::MacroFocus, -1 - i as i32));
                    s.button(&id, "page", x, 664.6, 85.5, 72.0, true);
                    s.stack(&format!("{id}_circle"), &id, x + 22.75, 668.6, 40.0, 40.0, Some(if self.ratio == i { WHITE } else { BOX_CIRCLE }), 20.0, None);
                    s.icon(&format!("{id}_icon"), &id, "ratio", x + 32.75, 678.6, 20.0, 20.0, if self.ratio == i { BLACK } else { WHITE });
                    s.text(&format!("{id}_label"), &id, t(&locale, cn, en), x, 712.6, 85.5, 16.0, 11.0, false, WHITE, Align::Center);
                }
            } else {
                s.text("box_title", "page", t(&locale, "曝光", "Exposure"), 60.0, 598.5, 286.0, 24.0, 16.0, true, WHITE, Align::Center);
                for (k, label) in ["-4", "-2", "0", "+2", "+4"].iter().enumerate() { s.text(&format!("ev_l{k}"), "page", label, 37.0 + k as f64 * 83.0 - 20.0, 660.0, 40.0, 16.0, 12.0, true, WHITE, Align::Center); }
                for k in 0..17 {
                    let x = 37.0 + k as f64 * (332.0 / 16.0); let tall = k % 2 == 0;
                    let id = self.ctl(&format!("ev_tick{k}"), Action::ProValue(3, (k / 2) as i32));
                    s.button(&id, "page", x - 10.0, 670.0, 20.0, 40.0, true);
                    if k as usize == self.pro[3] * 2 { s.stack("ev_marker", &id, x - 1.5, 684.0, 3.0, 20.0, Some(REC), 1.5, None); }
                    else { s.stack(&format!("{id}_mark"), &id, x - 0.75, if tall { 688.0 } else { 692.0 }, 1.5, if tall { 14.0 } else { 8.0 }, Some(if tall { WHITE } else { INK_DIM }), 0.75, None); }
                }
            }
            return;
        }
        self.render_handle(s, false, 586.5);
        let page0: Vec<(&str, &str, &str, &str, Action, bool)> = vec![
            ("box_settings", "gear", "设置", "Settings", Action::BoxSettings, false), ("box_vision", "vision", "小艺视觉", "Celia vision", Action::BoxVision, false), ("box_xmage", "film", "XMAGE 风格", "XMAGE style", Action::OpenFilter, false), ("box_exposure", "exposure", "曝光", "Exposure", Action::BoxExposure, false),
            ("box_ai", if self.ai_compose { "ai_compose_on" } else { "ai_compose" }, "AI 辅助构图", "AI composition", Action::ToggleAi, self.ai_compose), ("box_watermark", "watermark", "水印", "Watermark", Action::BoxWatermark, self.watermark), ("box_ratio", "ratio", "照片比例", "Aspect ratio", Action::BoxRatio, false), ("box_flash", self.flash.icon(), "闪光灯", "Flash", Action::OpenFlash, false)];
        let page1: Vec<(&str, &str, &str, &str, Action, bool)> = vec![("box_live", if self.live_photo { "live_on" } else { "live_off" }, "动态照片", "Live photo", Action::ToggleLive, self.live_photo), ("box_grid", "grid", "参考线", "Grid", Action::BoxGrid, self.grid)];
        let items = if page == 0 { page0 } else { page1 };
        for (i, (id, icon, cn, en, action, on)) in items.into_iter().enumerate() {
            let (r, c) = (i / 4, i % 4); let x = 32.0 + c as f64 * 85.5; let y = 622.5 + r as f64 * 72.0;
            let id = self.ctl(id, action);
            s.button(&id, "page", x, y, 85.5, 72.0, true);
            s.stack(&format!("{id}_circle"), &id, x + 22.75, y + 4.0, 40.0, 40.0, Some(if on { "ffffff" } else { BOX_CIRCLE }), 20.0, None);
            s.icon(&format!("{id}_icon"), &id, icon, x + 32.75, y + 14.0, 20.0, 20.0, if on { BLACK } else { WHITE });
            if id == "box_ratio" { s.text(&format!("{id}_glyph"), &id, ["4:3", "1:1", "全屏"][self.ratio], x + 32.75, y + 16.0, 20.0, 16.0, 7.0, true, WHITE, Align::Center); }
            s.text(&format!("{id}_label"), &id, t(&locale, cn, en), x - 6.0, y + 48.0, 97.5, 16.0, 11.0, false, WHITE, Align::Center);
        }
        let a = self.ctl("box_page_0", Action::BoxPage(0)); s.button(&a, "page", 175.0, 754.0, 28.0, 20.0, true);
        s.stack("box_page_0_dot", &a, 183.0, 761.5, if page == 0 { 20.0 } else { 6.0 }, 6.0, Some(if page == 0 { WHITE } else { DOT }), 3.0, None);
        let b = self.ctl("box_page_1", Action::BoxPage(1)); s.button(&b, "page", 203.0, 754.0, 28.0, 20.0, true);
        s.stack("box_page_1_dot", &b, if page == 0 { 213.0 } else { 199.0 }, 761.5, if page == 1 { 20.0 } else { 6.0 }, 6.0, Some(if page == 1 { WHITE } else { DOT }), 3.0, None);
    }
    fn render_more(&mut self, s: &mut Scene, edit: bool) {
        let locale = self.locale.clone();
        s.stack("more_panel", "page", 16.0, 236.0, 374.0, 334.0, Some("000000a0"), 24.0, None);
        if edit {
            let x = self.ctl("more_cancel", Action::MoreEditDone); s.button(&x, "page", 28.9, 248.9, 32.0, 32.0, true); s.icon("more_cancel_icon", &x, "close", 32.9, 252.9, 24.0, 24.0, WHITE);
            s.text_w("more_hint", "page", t(&locale, "拖动图标可调整布局", "Drag icons to rearrange"), 101.2, 256.9, 204.0, 16.0, 12.0, 500, WHITE, Align::Center);
            let r = self.ctl("more_reset", Action::MoreEditReset); s.button(&r, "page", 293.1, 248.9, 32.0, 32.0, true); s.icon("more_reset_icon", &r, "reset", 297.1, 252.9, 24.0, 24.0, WHITE);
            let d = self.ctl("more_done", Action::MoreEditDone); s.button(&d, "page", 345.1, 248.9, 32.0, 32.0, true); s.icon("more_done_icon", &d, "check", 349.1, 252.9, 24.0, 24.0, WHITE);
        } else {
            let e = self.ctl("more_edit", Action::MoreEdit); s.button(&e, "page", 28.9, 248.9, 32.0, 32.0, true); s.icon("more_edit_icon", &e, "pencil", 32.9, 252.9, 24.0, 24.0, WHITE);
            let i = self.ctl("more_info", Action::Intro(Mode::Pro)); s.button(&i, "page", 345.1, 248.9, 32.0, 32.0, true); s.icon("more_info_icon", &i, "info", 349.1, 252.9, 24.0, 24.0, WHITE);
        }
        for (i, m) in Mode::MORE.iter().enumerate() {
            let (r, c) = (i / 3, i % 3); let x = 30.4 + c as f64 * 119.0; let y = 292.9 + r as f64 * 89.0;
            let id = self.ctl(&format!("more_{}", m.id()), if edit { Action::MoreEdit } else { Action::SetMode(*m) });
            s.button(&id, "page", x, y, 107.0, 77.0, true);
            if edit { s.stack(&format!("{id}_bg"), &id, x, y, 107.0, 77.0, Some("ffffff33"), 16.0, None); }
            s.icon(&format!("{id}_icon"), &id, m.icon(), x + 41.5, y + 15.7, 24.0, 24.0, WHITE);
            s.text_w(&format!("{id}_label"), &id, m.label(&locale), x, y + 45.6, 107.0, 18.0, 14.0, 500, WHITE, Align::Center);
        }
        if edit {
            for (i, m) in [Mode::Snapshot, Mode::Portrait, Mode::Photo, Mode::Video, Mode::Pro].iter().enumerate() {
                let x = 26.8 + i as f64 * 73.8;
                let id = self.ctl(&format!("edit_mode_{}", m.id()), Action::MoreEditDone);
                s.button(&id, "page", x, 582.5, 62.0, 28.0, true);
                s.stack(&format!("{id}_bg"), &id, x, 582.5, 62.0, 28.0, Some(PILL2), 14.0, Some("ffffff33"));
                s.text_w(&format!("{id}_label"), &id, m.label(&locale), x, 582.5, 62.0, 28.0, 14.0, 500, if matches!(m, Mode::Snapshot | Mode::Pro) { WHITE } else { GREY }, Align::Center);
            }
        }
    }
    fn render_intro(&mut self, s: &mut Scene, m: Mode) {
        let locale = self.locale.clone();
        let Some((title, body)) = m.intro(&locale) else { self.overlay = Overlay::None; return };
        s.stack("intro_scrim", "page", 0.0, 0.0, 406.0, 776.0, Some(DIM), 0.0, None);
        s.stack("intro_card", "page", 39.1, 111.1, 327.9, 542.2, Some(["3b5f8a", "5a7a4a", "8a6a4a", "4a4a6a"][(m as usize) % 4]), 24.0, None);
        s.stack("intro_caption", "page", 39.1, 560.9, 327.9, 92.3, Some("00000066"), 0.0, None);
        s.text("intro_title", "page", t(&locale, title, m.label("en")), 55.1, 572.9, 223.6, 32.0, 24.0, true, WHITE, Align::Left);
        let more = self.ctl("intro_more", Action::IntroMore);
        s.labelled(&more, "page", t(&locale, "更多介绍", "More"), 286.7, 576.0, 64.3, 25.8, "pill", 12.0, true);
        let chars: Vec<char> = body.chars().collect(); let n = chars.len().min(2 * 26);
        let line1: String = chars.iter().take(26).collect(); let line2: String = chars.iter().skip(26).take(26).collect();
        s.text("intro_body", "page", &line1, 55.1, 608.9, 295.9, 14.0, 12.0, false, WHITE, Align::Left);
        if n > 26 { s.text("intro_body2", "page", &line2, 55.1, 623.9, 295.9, 14.0, 12.0, false, WHITE, Align::Left); }
        let close = self.ctl("intro_close", Action::CloseIntro);
        s.button(&close, "page", 331.0, 119.1, 28.0, 28.0, true);
        s.stack("intro_close_bg", &close, 331.0, 119.1, 28.0, 28.0, Some("00000080"), 14.0, None);
        s.icon("intro_close_icon", &close, "close", 334.3, 122.5, 21.2, 21.2, WHITE);
    }
    fn render_ruler(&mut self, s: &mut Scene, r: Ruler) {
        let locale = self.locale.clone();
        let (title, min, max, ticks, value): (&str, String, String, usize, i32) = match r {
            Ruler::Beauty => (t(&locale, "美肤", "Skin"), "0".into(), "10".into(), 25, self.beauty * 24 / 10),
            Ruler::Aperture => (t(&locale, "虚拟光圈", "Virtual aperture"), APERTURES[0].into(), APERTURES[9].into(), 10, self.aperture as i32),
            Ruler::NightShutter => (t(&locale, "快门速度", "Shutter speed"), "1/4".into(), "32".into(), 7, self.night_shutter as i32),
            Ruler::SlowRate => (t(&locale, "速率", "Rate"), SLOW_RATES[0].into(), SLOW_RATES[2].into(), 3, self.slow_rate as i32),
            Ruler::TimeLapse => (t(&locale, "速率", "Rate"), "0.5s".into(), "10s".into(), 5, self.timelapse as i32),
            Ruler::MacroFocus => (t(&locale, "对焦", "Focus"), t(&locale, "近", "Near").into(), t(&locale, "远", "Far").into(), 2, self.macro_focus as i32),
        };
        let auto = matches!(r, Ruler::NightShutter | Ruler::TimeLapse | Ruler::MacroFocus);
        let (x0, width) = if auto { (93.2, 270.0) } else { (40.0, 326.0) };
        s.stack("ruler_bar", "page", 0.0, 497.5, 406.0, 80.0, Some(PRO_BAR), 0.0, None);
        if r == Ruler::SlowRate {
            s.text("ruler_big", "page", SLOW_RATES[self.slow_rate], 160.9, 415.7, 84.9, 52.0, 40.0, true, WHITE, Align::Center);
            s.text_w("ruler_fps", "page", t(&locale, "240 帧/秒", "240 fps"), 152.0, 467.7, 102.0, 16.0, 12.0, 500, WHITE, Align::Center);
        }
        s.text("ruler_title", "page", title, 153.0, 498.0, 100.0, 20.0, 14.0, true, WHITE, Align::Center);
        if auto {
            let id = self.ctl("ruler_auto", Action::RulerSet(r, 0));
            s.button(&id, "page", 44.0, 525.7, 44.0, 44.0, true);
            s.stack("ruler_auto_bg", &id, 50.1, 531.7, 32.0, 32.0, Some(if value == 0 { REC } else { "5a5a5e" }), 16.0, None);
            s.text("ruler_auto_label", &id, "A", 50.1, 531.7, 32.0, 32.0, 16.0, true, WHITE, Align::Center);
        }
        s.text("ruler_min", "page", &min, x0 - 20.0, 517.0, 60.0, 16.0, 12.0, true, WHITE, Align::Center);
        s.text("ruler_max", "page", &max, x0 + width - 40.0, 517.0, 60.0, 16.0, 12.0, true, WHITE, Align::Center);
        if r == Ruler::Beauty { s.text("ruler_mid", "page", "5", 193.0, 517.0, 20.0, 16.0, 12.0, true, WHITE, Align::Center); }
        if r == Ruler::Aperture { for (id, label, x) in [("ruler_l1", "F2.4", 100.0), ("ruler_l2", "F4", 183.0), ("ruler_l3", "F7.1", 264.0)] { s.text(id, "page", label, x, 517.0, 40.0, 16.0, 12.0, true, WHITE, Align::Center); } }
        let step = width / (ticks - 1) as f64;
        for k in 0..ticks {
            let x = x0 + k as f64 * step; let tall = k % 3 == 0 || ticks <= 10;
            let id = self.ctl(&format!("ruler_tick{k}"), Action::RulerSet(r, if r == Ruler::Beauty { (k as i32 * 10 + 12) / 24 } else { k as i32 }));
            s.button(&id, "page", x - step / 2.0, 530.0, step, 46.0, true);
            if k as i32 == value { s.stack("ruler_marker", &id, x - 1.5, 538.0, 3.0, 20.0, Some(REC), 1.5, None); }
            else { s.stack(&format!("{id}_mark"), &id, x - 0.75, if tall { 545.0 } else { 549.0 }, 1.5, if tall { 14.0 } else { 8.0 }, Some(if tall { WHITE } else { INK_DIM }), 0.75, None); }
        }
    }
    fn render_pro_param(&mut self, s: &mut Scene, i: usize) {
        let values = PRO_VALUES[i];
        s.stack("pro_values_bar", "page", 0.0, 413.5, 406.0, 88.0, Some(PRO_BAR), 0.0, None);
        if i == 1 || i == 2 {
            let auto = self.ctl("pro_auto", Action::ProValue(i, if i == 1 { 0 } else { 4 }));
            s.button(&auto, "page", 44.0, 452.0, 42.0, 42.0, true);
            s.stack("pro_auto_bg", &auto, 50.1, 458.2, 29.0, 29.0, Some(REC), 14.5, None);
            s.text("pro_auto_label", &auto, "A", 50.1, 458.2, 29.0, 29.0, 15.0, true, WHITE, Align::Center);
        }
        s.stack("pro_active_dot", "page", 12.0 + i as f64 * 66.7 + 22.0, 553.0, 6.0, 6.0, Some(REC), 3.0, None);
        let n = values.len(); let w = (300.0 / n as f64).min(60.0); let x0 = 230.0 - w * n as f64 / 2.0;
        for (k, v) in values.iter().enumerate() {
            let x = x0 + k as f64 * w;
            let id = self.ctl(&format!("pro_value_{k}"), Action::ProValue(i, k as i32));
            s.button(&id, "page", x, 430.0, w, 60.0, true);
            let sel = self.pro[i] == k;
            s.text_w(&format!("{id}_label"), &id, v, x - 10.0, 430.0, w + 20.0, 20.0, 11.0, 700, if sel { WHITE } else { INK_DIM }, Align::Center);
            s.stack(&format!("{id}_tick"), &id, x + w / 2.0 - (if sel { 1.5 } else { 0.75 }), if sel { 458.0 } else { 462.0 }, if sel { 3.0 } else { 1.5 }, if sel { 20.0 } else { 12.0 }, Some(if sel { REC } else { WHITE }), 1.0, None);
        }
        if i >= 3 {
            let lock = self.ctl("pro_lock", Action::ProLock(i));
            s.labelled(&lock, "page", t(&self.locale, if self.pro_locked[i] { "已锁定" } else { "锁定" }, if self.pro_locked[i] { "Locked" } else { "Lock" }), 326.0, 405.0, 64.0, 28.0, "pill", 12.0, true);
        }
    }
    fn render_focus(&mut self, s: &mut Scene, x: f64, y: f64) {
        let scrim = self.ctl("scrim", Action::ClosePanel);
        s.button(&scrim, "page", 0.0, 36.0, 406.0, 540.0, true);
        let (bx, by) = ((x - 40.0).clamp(4.0, 362.0), (y - 40.0).clamp(43.0, self.preview_bottom() - 84.0));
        for (k, (dx, dy, w, h)) in [(0.0, 0.0, 20.0, 1.5), (0.0, 0.0, 1.5, 20.0), (60.0, 0.0, 20.0, 1.5), (78.5, 0.0, 1.5, 20.0), (0.0, 78.5, 20.0, 1.5), (0.0, 60.0, 1.5, 20.0), (60.0, 78.5, 20.0, 1.5), (78.5, 60.0, 1.5, 20.0)].iter().enumerate() {
            s.stack(&format!("focus_{k}"), "page", bx + dx, by + dy, *w, *h, Some(WHITE), 0.0, None);
        }
        s.icon("focus_sun", "page", "sun", (bx - 30.0).max(4.0), by + 30.0, 20.0, 20.0, WHITE);
        s.stack("ev_track", "page", 398.0, by - 20.0, 2.0, 120.0, Some("ffffff99"), 1.0, None);
        s.stack("smart_tip", "page", 225.5, 257.8, 160.0, 62.0, Some(TOAST), 12.0, None);
        s.text("smart_tip_text", "page", t(&self.locale, "开启相机智控，可通过", "Turn on smart control to"), 233.5, 261.8, 150.0, 20.0, 12.0, false, WHITE, Align::Left);
        s.text("smart_tip_text2", "page", t(&self.locale, "长触保持曝光和对焦", "long-press to lock AE/AF"), 233.5, 279.8, 150.0, 20.0, 12.0, false, WHITE, Align::Left);
        let go = self.ctl("smart_go", Action::BoxSettings);
        s.button(&go, "page", 233.5, 297.8, 60.0, 20.0, true);
        s.text("smart_go_label", &go, t(&self.locale, "去开启", "Turn on"), 233.5, 297.8, 60.0, 20.0, 12.0, false, BLUE, Align::Left);
    }
    fn render_settings(&mut self, s: &mut Scene, page: usize) {
        let locale = self.locale.clone();
        s.stack("settings_page", "page", 0.0, 0.0, 406.0, 776.0, Some(BLACK), 0.0, None);
        let back = self.ctl("settings_back", Action::Back);
        s.button(&back, "page", 8.0, 8.0, 44.0, 40.0, true);
        s.icon("settings_back_icon", &back, "back", 18.0, 16.0, 24.0, 24.0, WHITE);
        s.text("settings_title", "page", t(&locale, "设置", "Settings"), 60.0, 8.0, 200.0, 40.0, 20.0, true, WHITE, Align::Left);
        let per = 6;
        for (k, (cn, en)) in SETTINGS.iter().enumerate().skip(page * per).take(per) {
            let row = k - page * per; let y = 72.0 + row as f64 * 64.0;
            let id = self.ctl(&format!("setting_{k}"), Action::SettingsToggle(k));
            s.button(&id, "page", 16.0, y, 374.0, 56.0, true);
            s.stack(&format!("{id}_bg"), &id, 16.0, y, 374.0, 56.0, Some(BOX), 16.0, None);
            s.text(&format!("{id}_label"), &id, t(&locale, cn, en), 32.0, y, 240.0, 56.0, 16.0, false, WHITE, Align::Left);
            let on = self.settings[k];
            s.stack(&format!("{id}_switch"), &id, 328.0, y + 14.0, 46.0, 28.0, Some(if on { BLUE } else { "5a5a5e" }), 14.0, None);
            s.stack(&format!("{id}_knob"), &id, if on { 348.0 } else { 330.0 }, y + 16.0, 24.0, 24.0, Some(WHITE), 12.0, None);
        }
        let prev = self.ctl("settings_prev", Action::SettingsScroll(-1)); s.labelled(&prev, "page", t(&locale, "上一页", "Previous"), 60.0, 700.0, 120.0, 40.0, "outline", 14.0, page > 0);
        let next = self.ctl("settings_next", Action::SettingsScroll(1)); s.labelled(&next, "page", t(&locale, "下一页", "Next"), 226.0, 700.0, 120.0, 40.0, "outline", 14.0, (page + 1) * per < SETTINGS.len());
    }
    fn render_beauty_dialog(&mut self, s: &mut Scene, on: bool) {
        let locale = self.locale.clone();
        s.stack("dialog_scrim", "page", 0.0, 0.0, 406.0, 776.0, Some(DIM), 0.0, None);
        s.stack("beauty_dialog", "page", 16.0, 230.0, 374.0, 318.0, Some("2a2a2c"), 32.0, None);
        s.text("beauty_dialog_title", "page", t(&locale, "美颜状态", "Beauty"), 16.0, 244.0, 374.0, 40.0, 20.0, true, WHITE, Align::Center);
        for (i, (cn, en, is_on)) in [("关", "Off", false), ("开", "On", true)].iter().enumerate() {
            let x = 40.0 + i as f64 * 166.0;
            let id = self.ctl(if *is_on { "beauty_on" } else { "beauty_off" }, Action::BeautyDialog(*is_on));
            s.button(&id, "page", x, 296.0, 160.0, 180.0, true);
            s.stack(&format!("{id}_sample"), &id, x, 296.0, 160.0, 140.0, Some(if *is_on { "7a5a3a" } else { "6a5a4a" }), 12.0, None);
            s.stack(&format!("{id}_radio"), &id, x + 6.0, 448.0, 22.0, 22.0, None, 11.0, Some(WHITE));
            if on == *is_on { s.stack(&format!("{id}_radio_dot"), &id, x + 11.0, 453.0, 12.0, 12.0, Some(BLUE), 6.0, None); }
            s.text(&format!("{id}_label"), &id, t(&locale, cn, en), x + 40.0, 444.0, 80.0, 30.0, 16.0, false, WHITE, Align::Left);
        }
        let ok = self.ctl("beauty_dialog_ok", Action::BeautyDialogOk);
        s.labelled(&ok, "page", t(&locale, "确定", "OK"), 143.0, 496.0, 120.0, 40.0, "text", 16.0, true);
        s.node_color("beauty_dialog_ok_label", BLUE);
    }
    fn render_exposing(&mut self, s: &mut Scene, until: f64) {
        let locale = self.locale.clone();
        s.text("exposing_hint", "page", t(&locale, "正在改善拍摄画质，请持稳您的设备", "Improving image quality, hold the device steady"), 0.0, 53.5, 406.0, 30.0, 14.0, false, WHITE, Align::Center);
        s.stack("exposing_toast", "page", 92.0, 118.0, 222.0, 40.0, Some(TOAST), 20.0, None);
        s.text("exposing_toast_text", "page", t(&locale, "请移动手机对准拍摄点", "Move the phone onto the target"), 92.0, 118.0, 222.0, 40.0, 12.0, false, INK_DIM, Align::Center);
        s.text("exposing_count", "page", &format!("{:.1}s", (until - self.now).max(0.0)), 103.0, 300.0, 200.0, 80.0, 48.0, false, WHITE, Align::Center);
    }
    fn render_vision(&mut self, s: &mut Scene) {
        let locale = self.locale.clone();
        s.stack("vision_top", "page", 0.0, 0.0, 406.0, 39.4, Some(BLACK), 0.0, None);
        s.stack("vision_bottom", "page", 0.0, 580.3, 406.0, 195.7, Some(BLACK), 0.0, None);
        let close = self.ctl("vision_close", Action::Back); s.button(&close, "page", 16.0, 1.5, 40.0, 40.0, true); s.icon("vision_close_icon", &close, "close", 24.0, 9.5, 24.0, 24.0, WHITE);
        let more = self.ctl("vision_more", Action::BoxSettings); s.button(&more, "page", 350.0, 1.5, 40.0, 40.0, true); s.text("vision_more_label", &more, "⋯", 350.0, 1.5, 40.0, 40.0, 20.0, true, WHITE, Align::Center);
        s.text("vision_tip", "page", t(&locale, "扫一扫", "Scan"), 0.0, 57.5, 406.0, 21.2, 16.0, true, WHITE, Align::Center);
        s.text("vision_tip2", "page", t(&locale, "支持识别二维码/动植物/商品", "QR codes, plants, animals and products"), 0.0, 86.8, 406.0, 16.3, 12.0, false, INK_DIM, Align::Center);
        s.stack("vision_frame", "page", 63.0, 169.8, 280.0, 280.0, None, 16.0, Some("ffffff99"));
        let torch = self.ctl("vision_torch", Action::SetFlash(Flash::Torch)); s.button(&torch, "page", 170.1, 526.5, 66.1, 56.0, true);
        s.icon("vision_torch_icon", &torch, "flash_torch", 191.3, 531.4, 24.0, 24.0, WHITE); s.text("vision_torch_label", &torch, t(&locale, "轻触照亮", "Tap to light"), 170.1, 555.4, 66.1, 22.2, 11.0, false, WHITE, Align::Center);
        let cap = self.ctl("vision_capture", Action::RecStill); s.button(&cap, "page", 171.0, 661.5, 64.0, 64.0, true);
        s.stack("vision_capture_ring", &cap, 171.0, 661.5, 64.0, 64.0, Some(WHITE), 32.0, None); s.stack("vision_capture_gap", &cap, 174.0, 664.5, 58.0, 58.0, Some(BLACK), 29.0, None); s.stack("vision_capture_disc", &cap, 179.0, 669.5, 48.0, 48.0, Some(WHITE), 24.0, None);
        let gal = self.ctl("vision_gallery", Action::Thumbnail); s.button(&gal, "page", 31.4, 669.5, 48.0, 48.0, true); s.stack("vision_gallery_bg", &gal, 31.4, 669.5, 48.0, 48.0, Some(BOX_CIRCLE), 24.0, None); s.icon("vision_gallery_icon", &gal, "panorama", 43.4, 681.5, 24.0, 24.0, WHITE);
        for (i, (cn, en, x, w)) in [("识文", "Text", 77.8, 28.3), ("翻译", "Translate", 130.1, 28.3), ("扫一扫", "Scan", 182.4, 42.1), ("扫描", "Document", 248.5, 28.3)].iter().enumerate() {
            let id = self.ctl(&format!("vision_tab_{i}"), Action::BoxVision);
            s.button(&id, "page", *x - 8.0, 740.6, *w + 16.0, 26.2, true);
            s.text_w(&format!("{id}_label"), &id, t(&locale, cn, en), *x, 740.6, *w, 26.2, 13.0, 500, if i == 2 { WHITE } else { GREY }, Align::Center);
        }
    }
    fn render_gallery(&mut self, s: &mut Scene) {
        s.stack("gallery_page", "page", 0.0, 0.0, 406.0, 776.0, Some(BLACK), 0.0, None);
        let tint = ["b9b7b1", "8fa3b8", "b89f8f", "9fb88f"][(self.shots % 4) as usize];
        s.stack("gallery_photo", "page", 0.0, 112.0, 406.0, 541.0, Some(tint), 0.0, None);
        s.stack("gallery_line", "page", 0.0, 380.0, 406.0, 6.0, Some("2a2a2a"), 0.0, None);
        let back = self.ctl("gallery_back", Action::Back);
        s.button(&back, "page", 0.0, 0.0, 406.0, 776.0, true);
        s.text("gallery_hint", &back, t(&self.locale, "轻触返回相机", "Tap to return to the camera"), 0.0, 700.0, 406.0, 24.0, 12.0, false, GREY, Align::Center);
    }
    fn render_pano_guide(&mut self, s: &mut Scene) {
        if self.pano_vertical {
            s.stack("pano_band", "page", 24.3, 63.4, 60.0, 358.2, Some("ffffff26"), 0.0, None);
            s.stack("pano_frame", "page", 24.3, 182.0, 60.0, 120.0, None, 0.0, Some(WHITE));
        } else {
            s.stack("pano_band", "page", 40.0, 63.4, 328.0, 80.0, Some("ffffff26"), 0.0, None);
            s.stack("pano_frame", "page", 142.0, 63.4, 122.0, 80.0, None, 0.0, Some(WHITE));
            s.icon("pano_arrow_l", "page", "arrow_left", 92.0, 89.4, 28.0, 28.0, WHITE);
            s.icon("pano_arrow_r", "page", "arrow_right", 288.0, 89.4, 28.0, 28.0, WHITE);
            s.stack("pano_line_l", "page", 60.0, 103.0, 32.0, 1.5, Some(WHITE), 0.0, None); s.stack("pano_line_r", "page", 316.0, 103.0, 32.0, 1.5, Some(WHITE), 0.0, None);
        }
        s.stack("pano_target", "page", 185.0, 292.0, 36.0, 36.0, None, 18.0, Some(WHITE));
        s.stack("pano_target_in", "page", 194.0, 301.0, 18.0, 18.0, Some(WHITE), 9.0, None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn render_ok(session: &mut Session) -> crate::scene::Scene {
        let scene = session.render("http://127.0.0.1:1/t");
        let frame = crate::scene::compile(&scene, "camera-test");
        let report = octoscript_ui_l0::realize(&frame.card, &frame.data, Default::default());
        let root = report.complete_root().expect("complete root");
        let source = octoscript_ui_l0::kit_pack::lower(root, &frame.pack, &frame.data).expect("lower");
        let mut tree = octoscript_makepad::design::prepare(&source).expect("prepare");
        octoscript_makepad::l0::inspectable(&mut tree);
        assert!(!octoscript_makepad::design::to_makepad_ui(&tree).unwrap().is_empty());
        scene
    }

    #[test]
    fn every_mode_and_overlay_renders_and_lowers() {
        let mut s = Session::new("cn");
        for m in Mode::BAR.iter().chain(Mode::MORE.iter()) {
            s.go(*m); render_ok(&mut s);
            while s.overlay != Overlay::None { assert!(s.back()); render_ok(&mut s); }
        }
        s.go(Mode::Photo);
        for overlay in [Overlay::FlashPanel, Overlay::FilterMenu, Overlay::Box(0), Overlay::Box(1), Overlay::Box(2), Overlay::Box(3), Overlay::Focus(200.0, 300.0), Overlay::Settings(0), Overlay::Gallery, Overlay::Ruler(Ruler::Beauty), Overlay::Exposing { until: 4.0 }, Overlay::Vision] {
            s.overlay = overlay; render_ok(&mut s);
        }
        s.go(Mode::Video); s.overlay = Overlay::ResPanel; render_ok(&mut s);
        s.go(Mode::Pro); s.intro_seen.insert(Mode::Pro); s.overlay = Overlay::ProParam(1); render_ok(&mut s);
        s.front = true; s.go(Mode::Portrait); s.overlay = Overlay::BeautyDialog(true); render_ok(&mut s);
    }

    #[test]
    fn the_mode_bar_and_more_grid_follow_the_phone() {
        let mut s = Session::new("cn");
        render_ok(&mut s);
        assert!(s.activate("mode_video")); assert_eq!(s.mode, Mode::Video);
        let scene = render_ok(&mut s);
        assert!(scene.controls.contains_key("video_res"), "video shows the resolution pill");
        assert!(s.activate("mode_more")); assert_eq!(s.overlay, Overlay::More(false));
        render_ok(&mut s);
        assert!(s.activate("more_slowmo")); assert_eq!(s.mode, Mode::SlowMo);
        let scene = render_ok(&mut s);
        assert!(scene.controls.contains_key("sub_mode_close"), "a More sub-mode shows the pill instead of the bar");
        assert!(s.activate("sub_mode_close")); assert_eq!(s.mode, Mode::Photo);
        render_ok(&mut s);
        assert!(s.activate("mode_pro")); assert_eq!(s.overlay, Overlay::Intro(Mode::Pro), "first visit shows the intro card");
        render_ok(&mut s);
        assert!(s.activate("intro_close")); assert_eq!(s.overlay, Overlay::None);
        render_ok(&mut s);
        assert!(s.activate("mode_photo")); render_ok(&mut s); assert!(s.activate("mode_pro")); assert_eq!(s.overlay, Overlay::None, "seen once");
    }

    #[test]
    fn flash_filter_zoom_and_capture_change_state_like_the_phone() {
        let mut s = Session::new("cn");
        render_ok(&mut s);
        assert!(s.activate("flash")); assert_eq!(s.overlay, Overlay::FlashPanel);
        render_ok(&mut s);
        assert!(s.activate("flash_torch")); assert_eq!(s.flash, Flash::Torch); assert_eq!(s.overlay, Overlay::None);
        render_ok(&mut s);
        assert!(s.activate("filter")); render_ok(&mut s);
        assert!(s.activate("style_3")); assert_eq!(s.filter, 3);
        render_ok(&mut s);
        assert!(s.activate("zoom_3")); assert_eq!(s.zoom_index(), 3);
        assert!(s.activate("live_photo")); assert!(s.live_photo); assert!(s.toast.is_some());
        assert!(s.activate("shutter")); assert_eq!(s.shots, 1);
        s.now = 10.0; s.go(Mode::Video); render_ok(&mut s);
        assert!(s.activate("shutter")); assert!(matches!(s.recording, Recording::Running { .. }));
        s.now = 17.0; assert_eq!(s.timer_text(), "00:07");
        render_ok(&mut s);
        assert!(s.activate("rec_pause")); assert!(matches!(s.recording, Recording::Paused { .. }));
        s.now = 20.0; assert_eq!(s.timer_text(), "00:07");
        assert!(s.activate("rec_pause")); s.now = 22.0; assert_eq!(s.timer_text(), "00:09");
        assert!(s.activate("shutter")); assert_eq!(s.recording, Recording::Off); assert_eq!(s.shots, 2);
    }

    #[test]
    fn treasure_box_settings_and_back() {
        let mut s = Session::new("cn");
        render_ok(&mut s);
        assert!(s.activate("treasure_handle")); assert_eq!(s.overlay, Overlay::Box(0));
        let scene = render_ok(&mut s);
        assert!(!scene.controls.contains_key("mode_photo"), "the mode bar hides under the box");
        assert!(s.swipe(-120.0, 0.0, false)); assert_eq!(s.overlay, Overlay::Box(1));
        render_ok(&mut s);
        assert!(s.activate("box_grid")); assert!(s.grid);
        assert!(s.activate("box_page_0")); render_ok(&mut s);
        assert!(s.activate("box_settings")); assert_eq!(s.overlay, Overlay::Settings(0));
        render_ok(&mut s);
        assert!(s.activate("setting_0")); assert!(s.settings[0]);
        assert!(s.back()); assert_eq!(s.overlay, Overlay::Box(0));
        assert!(s.back()); assert_eq!(s.overlay, Overlay::None);
        assert!(!s.back(), "nothing left to close: the host exits");
        assert!(s.swipe(-100.0, 5.0, false)); assert_eq!(s.mode, Mode::Video);
        assert!(s.swipe(0.0, -80.0, true)); assert_eq!(s.overlay, Overlay::Box(0));
    }
}

#[cfg(test)]
mod strips {
    use super::*;
    /// The sliding strips are scroll containers whose children are stored
    /// relative to them; after lowering they must land on the bar (the
    /// lowering adds the container origin once) and the module's hit
    /// rectangles must be absolute again.
    #[test]
    fn mode_strip_children_land_on_the_bar() {
        let mut s = Session::new("cn");
        let scene = s.render("http://127.0.0.1:1/t");
        let frame = crate::scene::compile(&scene, "camera-test");
        let report = octoscript_ui_l0::realize(&frame.card, &frame.data, Default::default());
        let root = report.complete_root().expect("complete root");
        let source = octoscript_ui_l0::kit_pack::lower(root, &frame.pack, &frame.data).expect("lower");
        let mut tree = octoscript_makepad::design::prepare(&source).expect("prepare");
        octoscript_makepad::l0::inspectable(&mut tree);
        fn find<'a>(n: &'a octoscript_node::UiNode, id: &str) -> Option<&'a octoscript_node::UiNode> {
            if n.attrs.id.as_deref() == Some(id) { return Some(n); }
            n.children.iter().find_map(|c| find(c, id))
        }
        let strip = find(&tree, frame.mapping.get("mode_strip").unwrap()).expect("mode strip lowered");
        assert_eq!(strip.children.len(), Mode::BAR.len());
        for child in &strip.children { assert_eq!(child.attrs.y, Some(576.6)); }
        let photo = find(&tree, frame.mapping.get("mode_photo").unwrap()).unwrap();
        assert!((photo.attrs.x.unwrap() - 177.0).abs() < 0.01, "the selected mode is centred");
        let chip = find(&tree, frame.mapping.get("zoom_chip").unwrap()).unwrap();
        assert_eq!((chip.attrs.x, chip.attrs.y), (Some(143.0), Some(517.5)));
        assert!(scene.button_rects().iter().any(|r| (r.0 - 177.0).abs() < 0.01 && (r.1 - 576.6).abs() < 0.01), "hit rectangles are absolute");
    }
}
