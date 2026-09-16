//! The app's screens and what each tap does. The session owns the replica
//! (confirmed state + this device's unconfirmed operations) and the UI state;
//! `render` draws the current screen as a scene, `activate` handles a control.
//! A remote change never moves the screen: it is listed in the inbox.
use crate::model::{self, at, iso, today, Event, Op, Rejected, State, Text};
use crate::scene::{Align, Scene, BLUE, GRAY, GRAY2, GREEN, GROUP, INK, LINE, PAGE, RED, WHITE};
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Timelike, Weekday};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Screen {
    Month,
    Day(NaiveDate),
    Detail(String),
    Editor,
    Picker(PickField),
    CalendarChooser,
    Calendars,
    Inbox,
    Sync,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PickField { Start, End }

#[derive(Clone, Debug)]
pub struct Draft {
    pub id: String,
    pub editing: bool,
    pub title: String,
    pub location: String,
    pub notes: String,
    pub calendar: String,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub all_day: bool,
    pub alert: Option<i64>,
    pub picker_day: NaiveDate,
}

#[derive(Clone, Debug)]
pub struct Change { pub key: String, pub event_id: Option<String>, pub from: String, pub action: String, pub seq: i64 }
#[derive(Clone, Debug)]
pub struct Notice { pub op_id: String, pub action: String, pub reason: String }

#[derive(Clone, Debug)]
enum Action {
    MonthPrev, MonthNext, Today, SelectDay(NaiveDate), OpenDay(NaiveDate), OpenEvent(String), Add, Tab(Screen), Back,
    DayPrev, DayNext, Edit, Delete, Restore, Cancel, Save, Pick(PickField), Slot(u32, u32), PickerDay(i64), ChooseCalendar, SetCalendar(String),
    AlertCycle, AllDayToggle, ToggleCalendar(String), Invite(String, String), SyncNow, ShowChange(usize), DismissNotice(usize), ToggleLocale, Reconnect,
}

pub struct Session {
    pub device: String,
    pub locale: String,
    pub connected: bool,
    pub server: Option<String>,
    pub last_error: Option<String>,
    pub remote: State,
    pub remote_seq: i64,
    pub pending: Vec<Op>,
    pub screen: Screen,
    pub stack: Vec<Screen>,
    pub month: (i32, u32),
    pub selected: NaiveDate,
    pub draft: Option<Draft>,
    pub focus: Option<String>,
    pub unseen: Vec<Change>,
    pub notices: Vec<Notice>,
    pub removed: Option<String>,
    counter: u64,
    controls: HashMap<String, Action>,
    pub revision: u64,
}

fn t<'a>(locale: &str, cn: &'a str, en: &'a str) -> &'a str { if locale == "en" { en } else { cn } }
fn ident(s: &str) -> String { s.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect() }
const WD_CN: [&str; 7] = ["日", "一", "二", "三", "四", "五", "六"];
const WD_EN: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
const MON_EN: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
const MON_SHORT: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
pub fn fmt_date(d: NaiveDate, locale: &str) -> String {
    let wd = d.weekday().num_days_from_sunday() as usize;
    if locale == "en" { format!("{}, {} {}", WD_EN[wd], MON_SHORT[d.month0() as usize], d.day()) } else { format!("{}月{}日 周{}", d.month(), d.day(), WD_CN[wd]) }
}
fn fmt_hm(t: NaiveDateTime) -> String { format!("{:02}:{:02}", t.hour(), t.minute()) }
fn month_title(y: i32, m: u32, locale: &str) -> String { if locale == "en" { MON_EN[m as usize - 1].to_string() } else { format!("{m}月") } }
fn alert_label(alert: Option<i64>, locale: &str) -> String {
    match alert {
        None => t(locale, "无", "None").into(),
        Some(0) => t(locale, "日程开始时", "At time of event").into(),
        Some(m) if m < 60 => if locale == "en" { format!("{m} min before") } else { format!("提前 {m} 分钟") },
        Some(m) if m % 1440 == 0 => if locale == "en" { format!("{} day before", m / 1440) } else { format!("提前 {} 天", m / 1440) },
        Some(m) => if locale == "en" { format!("{} h before", m / 60) } else { format!("提前 {} 小时", m / 60) },
    }
}
const ALERTS: [Option<i64>; 7] = [None, Some(0), Some(5), Some(15), Some(30), Some(60), Some(1440)];
/// Calendars in the order people expect (shared first, hidden last), not the map's alphabetical one.
fn ordered_calendars(state: &State) -> Vec<(&String, &model::Calendar)> {
    let rank = |id: &str| ["family", "work", "personal", "birthdays"].iter().position(|k| *k == id).unwrap_or(9);
    let mut out: Vec<_> = state.calendars.iter().collect();
    out.sort_by_key(|(id, c)| (!c.visible, rank(id), (*id).clone()));
    out
}

impl Session {
    pub fn new(device: &str, locale: &str) -> Self {
        let today = today();
        Session { device: device.into(), locale: locale.into(), connected: false, server: None, last_error: None, remote: model::seed(today), remote_seq: 0, pending: Vec::new(),
            screen: Screen::Month, stack: Vec::new(), month: (today.year(), today.month()), selected: today, draft: None, focus: None, unseen: Vec::new(), notices: Vec::new(),
            removed: None, counter: 0, controls: HashMap::new(), revision: 0 }
    }

    /// The replica as this device sees it: confirmed state plus its own unconfirmed operations.
    pub fn state(&self) -> State {
        let mut state = self.remote.clone();
        for op in &self.pending { let _ = state.apply(op); }
        state
    }
    fn next_op(&mut self, action: &str, payload: Value) -> Op {
        self.counter += 1;
        Op { id: format!("{}:{}:{}", self.device, self.remote_seq, self.counter), actor: self.device.clone(), action: action.into(), payload }
    }
    /// Try an operation on the replica; keep it pending for the server, or explain the rejection.
    fn submit(&mut self, action: &str, payload: Value) -> Result<(), String> {
        let op = self.next_op(action, payload);
        let mut probe = self.state();
        match probe.apply(&op) {
            Ok(_) => { self.pending.push(op); self.last_error = None; Ok(()) }
            Err(Rejected(reason)) => { self.last_error = Some(self.explain(&reason)); Err(reason) }
        }
    }
    pub fn explain(&self, reason: &str) -> String {
        let locale = &self.locale;
        if let Some(other) = reason.strip_prefix("conflict:") {
            let title = self.state().events.get(other).map(|e| e.title.get(locale).to_owned()).unwrap_or_else(|| other.into());
            return if locale == "en" { format!("Overlaps “{title}”") } else { format!("与「{title}」重叠") };
        }
        match reason {
            "missing_event" => t(locale, "这条日程已不存在", "That event no longer exists").into(),
            "invalid_event" => t(locale, "请填写标题，结束时间要晚于开始", "Add a title; the end must be after the start").into(),
            other => if locale == "en" { format!("Refused: {other}") } else { format!("已拒绝：{other}") },
        }
    }

    // ---- server records ------------------------------------------------------------
    pub fn adopt_snapshot(&mut self, seq: i64, state: State) {
        self.remote = state; self.remote_seq = seq; self.connected = true; self.last_error = None; self.revision += 1;
    }
    /// Fold server records (log order): confirm or roll back ours, adopt the other device's.
    pub fn apply_records(&mut self, records: &[Value]) {
        for record in records {
            let seq = record["seq"].as_i64().unwrap_or(0);
            if seq <= self.remote_seq { continue; }
            self.remote_seq = seq;
            let Ok(op) = serde_json::from_value::<Op>(record["op"].clone()) else { continue };
            let mine = op.actor == self.device;
            if record["accepted"].as_bool().unwrap_or(false) {
                let _ = self.remote.apply(&op);
                if !mine && !record["replayed"].as_bool().unwrap_or(false) && op.action.starts_with("event.") || (!mine && op.action == "invite.respond") {
                    let event_id = op.payload["event"]["id"].as_str().or(op.payload["id"].as_str()).map(|s| if op.action == "invite.respond" { format!("invite-{s}") } else { s.to_owned() });
                    self.unseen.push(Change { key: op.id.clone(), event_id, from: op.actor.clone(), action: op.action.clone(), seq });
                }
            } else if mine && self.pending.iter().any(|p| p.id == op.id) {
                let reason = record["reason"].as_str().unwrap_or("rejected").to_owned();
                self.notices.push(Notice { op_id: op.id.clone(), action: op.action.clone(), reason: self.explain(&reason) });
            }
            self.pending.retain(|p| p.id != op.id);
            self.revision += 1;
        }
    }

    // ---- navigation ------------------------------------------------------------------
    fn go(&mut self, screen: Screen) { let previous = std::mem::replace(&mut self.screen, screen); self.stack.push(previous); self.focus = None; }
    fn back(&mut self) { if let Some(previous) = self.stack.pop() { self.screen = previous; } else { self.screen = Screen::Month; } self.focus = None; }
    fn tab(&mut self, screen: Screen) { self.screen = screen; self.stack.clear(); self.focus = None; }
    fn new_draft(&self, day: NaiveDate) -> Draft {
        // The next free hour: a new event never starts inside an existing one.
        let mut start = if day == today() { let n = model::now(); at(day, (n.hour() + 1).min(23), 0) } else { at(day, 9, 0) };
        let state = self.state();
        for _ in 0..48 {
            if state.conflict(&iso(start), &iso(start + Duration::hours(1)), None).is_none() || start.hour() >= 23 { break; }
            start += Duration::minutes(30);
        }
        Draft { id: format!("ev-{}-{}", self.device, chrono::Local::now().timestamp_millis()), editing: false, title: String::new(), location: String::new(), notes: String::new(),
                calendar: "family".into(), start, end: start + Duration::hours(1), all_day: false, alert: Some(30), picker_day: day }
    }
    fn draft_of(event: &Event) -> Draft {
        Draft { id: event.id.clone(), editing: true, title: event.title.cn.clone(), location: event.location.cn.clone(), notes: event.notes.clone(), calendar: event.calendar.clone(),
                start: event.start_time(), end: event.end_time(), all_day: event.all_day, alert: event.alert, picker_day: event.date() }
    }
    fn draft_payload(d: &Draft) -> Value {
        let (start, end) = if d.all_day { (at(d.start.date(), 0, 0), at(d.start.date(), 23, 59)) } else { (d.start, d.end) };
        json!({"id": d.id, "title": Text::same(d.title.trim()), "calendar": d.calendar, "start": iso(start), "end": iso(end), "location": Text::same(d.location.trim()),
               "all_day": d.all_day, "notes": d.notes.trim(), "alert": d.alert})
    }
    fn draft_conflict(&self, d: &Draft) -> Option<String> {
        let state = self.state();
        let (start, end) = if d.all_day { (iso(at(d.start.date(), 0, 0)), iso(at(d.start.date(), 23, 59))) } else { (iso(d.start), iso(d.end)) };
        state.conflict(&start, &end, if d.editing { Some(&d.id) } else { None }).map(|e| e.title.get(&self.locale).to_owned())
    }

    /// A text field changed (KitAction::Changed on an input).
    pub fn field_changed(&mut self, control: &str, value: &str) {
        if let Some(draft) = self.draft.as_mut() {
            match control { "title_field" => draft.title = value.into(), "location_field" => draft.location = value.into(), "notes_field" => draft.notes = value.into(), _ => {} }
            self.focus = Some(control.into());
        }
    }

    /// A tap on a control. Returns true when the screen needs re-rendering.
    pub fn activate(&mut self, control: &str) -> bool {
        let Some(action) = self.controls.get(control).cloned() else { return false };
        let locale = self.locale.clone();
        match action {
            Action::MonthPrev => { let (y, m) = self.month; self.month = if m == 1 { (y - 1, 12) } else { (y, m - 1) }; }
            Action::MonthNext => { let (y, m) = self.month; self.month = if m == 12 { (y + 1, 1) } else { (y, m + 1) }; }
            Action::Today => { let d = today(); self.selected = d; self.month = (d.year(), d.month()); if let Screen::Day(_) = self.screen { self.screen = Screen::Day(d); } }
            Action::SelectDay(d) => { if self.selected == d { self.go(Screen::Day(d)); } else { self.selected = d; self.month = (d.year(), d.month()); } }
            Action::OpenDay(d) => { self.selected = d; self.go(Screen::Day(d)); }
            Action::DayPrev => if let Screen::Day(d) = self.screen { let n = d - Duration::days(1); self.screen = Screen::Day(n); self.selected = n; },
            Action::DayNext => if let Screen::Day(d) = self.screen { let n = d + Duration::days(1); self.screen = Screen::Day(n); self.selected = n; },
            Action::OpenEvent(id) => { self.removed = None; self.go(Screen::Detail(id)); }
            Action::Add => { self.draft = Some(self.new_draft(self.selected)); self.go(Screen::Editor); }
            Action::Tab(screen) => self.tab(screen),
            Action::Back => self.back(),
            Action::Edit => if let Screen::Detail(id) = &self.screen { if let Some(e) = self.state().events.get(id) { self.draft = Some(Self::draft_of(e)); self.go(Screen::Editor); } },
            Action::Delete => if let Screen::Detail(id) = self.screen.clone() { if self.submit("event.delete", json!({"id": id})).is_ok() { self.removed = Some(id); } },
            Action::Restore => if let Screen::Detail(id) = self.screen.clone() { if self.submit("event.restore", json!({"id": id})).is_ok() { self.removed = None; } },
            Action::Cancel => { self.draft = None; self.back(); }
            Action::Save => if let Some(d) = self.draft.clone() {
                let payload = Self::draft_payload(&d);
                let result = if d.editing {
                    let mut patch = payload.clone(); patch.as_object_mut().unwrap().remove("id");
                    self.submit("event.update", json!({"id": d.id, "patch": patch}))
                } else { self.submit("event.create", json!({"event": payload})) };
                if result.is_ok() { self.draft = None; self.selected = d.start.date(); self.month = (self.selected.year(), self.selected.month()); self.back(); if let Screen::Detail(_) = self.screen { } else { self.screen = Screen::Detail(d.id.clone()); self.stack = vec![Screen::Month]; } }
            },
            Action::Pick(field) => { if let Some(d) = self.draft.as_mut() { d.picker_day = if field == PickField::Start { d.start.date() } else { d.end.date() }; } self.go(Screen::Picker(field)); }
            Action::PickerDay(delta) => if let Some(d) = self.draft.as_mut() { d.picker_day += Duration::days(delta); },
            Action::Slot(h, m) => if let (Screen::Picker(field), Some(d)) = (self.screen.clone(), self.draft.as_mut()) {
                let chosen = at(d.picker_day, h, m);
                match field {
                    PickField::Start => { let duration = d.end - d.start; d.start = chosen; d.end = chosen + duration.max(Duration::minutes(15)); }
                    PickField::End => { if chosen > d.start { d.end = chosen; } else { d.end = d.start + Duration::minutes(30); } }
                }
                self.back();
            },
            Action::ChooseCalendar => self.go(Screen::CalendarChooser),
            Action::SetCalendar(id) => { if let Some(d) = self.draft.as_mut() { d.calendar = id; } self.back(); }
            Action::AlertCycle => if let Some(d) = self.draft.as_mut() { let i = ALERTS.iter().position(|a| *a == d.alert).unwrap_or(0); d.alert = ALERTS[(i + 1) % ALERTS.len()]; },
            Action::AllDayToggle => if let Some(d) = self.draft.as_mut() { d.all_day = !d.all_day; },
            Action::ToggleCalendar(id) => { let visible = self.state().calendars.get(&id).map(|c| c.visible).unwrap_or(true); let _ = self.submit("calendar.set_visible", json!({"id": id, "visible": !visible})); }
            Action::Invite(id, status) => { let _ = self.submit("invite.respond", json!({"id": id, "status": status})); }
            Action::SyncNow => { let _ = self.submit("device.sync", json!({"at": iso(model::now())})); }
            Action::ShowChange(i) => if i < self.unseen.len() { let change = self.unseen.remove(i); if let Some(id) = change.event_id { if self.state().events.contains_key(&id) { self.go(Screen::Detail(id)); } } },
            Action::DismissNotice(i) => if i < self.notices.len() { self.notices.remove(i); },
            Action::ToggleLocale => self.locale = if locale == "en" { "cn".into() } else { "en".into() },
            Action::Reconnect => { self.last_error = None; }
        }
        self.revision += 1;
        true
    }

    // ---- rendering ----------------------------------------------------------------------
    pub fn render(&mut self, asset_base: &str) -> Scene {
        self.controls.clear();
        let state = self.state();
        let desktop = matches!(self.screen, Screen::Inbox | Screen::Sync);
        let mut s = Scene::new(asset_base, if desktop { "f2f2f7" } else { PAGE });
        match self.screen.clone() {
            Screen::Month => self.render_month(&mut s, &state),
            Screen::Day(d) => self.render_day(&mut s, &state, d),
            Screen::Detail(id) => self.render_detail(&mut s, &state, &id),
            Screen::Editor => self.render_editor(&mut s, &state),
            Screen::Picker(field) => self.render_picker(&mut s, &state, field),
            Screen::CalendarChooser => self.render_chooser(&mut s, &state),
            Screen::Calendars => self.render_calendars(&mut s, &state),
            Screen::Inbox => self.render_inbox(&mut s, &state),
            Screen::Sync => self.render_sync(&mut s, &state),
        }
        s
    }
    fn ctl(&mut self, id: &str, action: Action) -> String { self.controls.insert(id.into(), action); id.into() }
    fn nav_button(&mut self, s: &mut Scene, id: &str, action: Action, icon: Option<&str>, label: &str, x: f64, w: f64, align: Align) {
        let id = self.ctl(id, action);
        s.button(&id, "page", x, 6.0, w, 36.0, true);
        let mut tx = x;
        if let Some(icon) = icon { s.icon(&format!("{id}_icon"), &id, icon, x + 2.0, 13.0, 22.0, 22.0, RED); tx += 24.0; }
        if !label.is_empty() { s.text(&format!("{id}_label"), &id, label, tx, 6.0, w - (tx - x), 36.0, 17.0, false, RED, align); }
    }
    fn tab_bar(&mut self, s: &mut Scene, active: &str) {
        let locale = self.locale.clone();
        s.stack("tab_bar", "page", 0.0, 660.0, 406.0, 56.0, Some("f9f9f9"), 0.0, None);
        s.stack("tab_line", "page", 0.0, 660.0, 406.0, 1.0, Some(LINE), 0.0, None);
        let tabs = [("tab_today", Screen::Month, "calendar", "今天", "Today"), ("tab_calendars", Screen::Calendars, "list", "日历", "Calendars"), ("tab_inbox", Screen::Inbox, "tray", "收件箱", "Inbox"), ("tab_sync", Screen::Sync, "sync", "同步", "Sync")];
        let badge = self.unseen.len() + self.state().invitations.values().filter(|i| i.status == "pending").count();
        for (i, (id, screen, icon, cn, en)) in tabs.iter().enumerate() {
            let x = 8.0 + i as f64 * 97.5;
            let color = if *id == active { RED } else { GRAY };
            let id = self.ctl(id, Action::Tab(screen.clone()));
            s.button(&id, "tab_bar", x, 662.0, 97.0, 54.0, id.as_str() != active);
            s.icon(&format!("{id}_icon"), &id, icon, x + 37.0, 666.0, 24.0, 24.0, color);
            s.text(&format!("{id}_label"), &id, t(&locale, cn, en), x, 693.0, 97.0, 16.0, 11.0, false, color, Align::Center);
            if id == "tab_inbox" && badge > 0 {
                s.stack("inbox_badge", &id, x + 58.0, 662.0, 18.0, 18.0, Some(RED), 9.0, None);
                s.text("inbox_badge_count", &id, &badge.to_string(), x + 58.0, 662.0, 18.0, 18.0, 11.0, true, WHITE, Align::Center);
            }
        }
    }
    fn render_month(&mut self, s: &mut Scene, state: &State) {
        let locale = self.locale.clone();
        let (y, m) = self.month;
        let first = NaiveDate::from_ymd_opt(y, m, 1).unwrap();
        let days = if m == 12 { NaiveDate::from_ymd_opt(y + 1, 1, 1) } else { NaiveDate::from_ymd_opt(y, m + 1, 1) }.unwrap().signed_duration_since(first).num_days() as u32;
        self.nav_button(s, "month_prev", Action::MonthPrev, Some("back"), &y.to_string(), 12.0, 90.0, Align::Left);
        self.nav_button(s, "month_next", Action::MonthNext, Some("forward"), "", 232.0, 40.0, Align::Left);
        self.nav_button(s, "go_today", Action::Today, None, t(&locale, "今天", "Today"), 276.0, 66.0, Align::Center);
        self.nav_button(s, "add_event", Action::Add, Some("plus"), "", 350.0, 44.0, Align::Left);
        s.text("month_title", "page", &month_title(y, m, &locale), 20.0, 48.0, 300.0, 44.0, 34.0, true, RED, Align::Left);
        let year_note = if locale == "en" { y.to_string() } else { format!("{y}年") };
        s.text("month_year", "page", &year_note, 200.0, 66.0, 186.0, 24.0, 15.0, false, GRAY, Align::Right);
        for i in 0..7 { s.text(&format!("weekday_{i}"), "page", if locale == "en" { &WD_EN[i][..1] } else { WD_CN[i] }, 20.0 + i as f64 * 52.0, 100.0, 52.0, 16.0, 12.0, false, GRAY, Align::Center); }
        s.stack("grid_line", "page", 20.0, 120.0, 366.0, 1.0, Some(LINE), 0.0, None);
        let offset = first.weekday().num_days_from_sunday();
        let rows = ((offset + days + 6) / 7).max(5);
        let cell_h = if rows > 5 { 44.0 } else { 52.0 };
        let today = today();
        for day in 1..=days {
            let date = NaiveDate::from_ymd_opt(y, m, day).unwrap();
            let index = offset + day - 1;
            let (r, c) = (index / 7, index % 7);
            let (x, yy) = (20.0 + c as f64 * 52.0, 126.0 + r as f64 * cell_h);
            let id = self.ctl(&format!("day_{}", date.format("%Y_%m_%d")), Action::SelectDay(date));
            s.button(&id, "page", x, yy, 52.0, cell_h, true);
            let ring_y = yy + 2.0;
            if date == today { s.stack(&format!("{id}_ring"), &id, x + 8.0, ring_y, 36.0, 36.0, Some(RED), 18.0, None); }
            else if date == self.selected { s.stack(&format!("{id}_ring"), &id, x + 8.0, ring_y, 36.0, 36.0, Some(GROUP), 18.0, Some(GRAY2)); }
            let color = if date == today { WHITE } else if date.weekday() == Weekday::Sat || date.weekday() == Weekday::Sun { GRAY } else { INK };
            s.text(&format!("{id}_num"), &id, &day.to_string(), x, ring_y, 52.0, 36.0, 18.0, date == today, color, Align::Center);
            let colors: Vec<String> = state.visible(Some(date)).iter().take(3).map(|e| state.calendars.get(&e.calendar).map(|c| c.color.clone()).unwrap_or_else(|| RED.into())).collect();
            for (k, color) in colors.iter().enumerate() {
                s.stack(&format!("{id}_dot{k}"), &id, x + 26.0 - 4.0 * colors.len() as f64 + k as f64 * 8.0 - 1.0, yy + cell_h - 10.0, 6.0, 6.0, Some(color), 3.0, None);
            }
        }
        let list_top = 126.0 + rows as f64 * cell_h + 6.0;
        s.stack("list_line", "page", 20.0, list_top, 366.0, 1.0, Some(LINE), 0.0, None);
        let selected = self.selected;
        let events = state.visible(Some(selected));
        let title = format!("{}{}", fmt_date(selected, &locale), if selected == today { t(&locale, " · 今天", " · Today") } else { "" });
        s.text("list_title", "page", &title, 20.0, list_top + 8.0, 250.0, 20.0, 13.0, true, GRAY, Align::Left);
        let open = self.ctl("open_day", Action::OpenDay(selected));
        s.button(&open, "page", 276.0, list_top + 4.0, 110.0, 28.0, true);
        s.text("open_day_label", &open, t(&locale, "查看当天 ›", "Open day ›"), 276.0, list_top + 4.0, 110.0, 28.0, 13.0, false, RED, Align::Right);
        let list_h = 660.0 - (list_top + 34.0);
        s.scroll("list", "page", 0.0, list_top + 34.0, 406.0, list_h);
        if events.is_empty() { s.text("empty", "list", t(&locale, "这一天没有日程", "No events"), 20.0, 12.0, 366.0, 24.0, 15.0, false, GRAY2, Align::Left); }
        for (i, e) in events.iter().enumerate() {
            let yy = i as f64 * 56.0;
            let id = self.ctl(&format!("ev_{}", ident(&e.id)), Action::OpenEvent(e.id.clone()));
            s.button(&id, "list", 20.0, yy, 366.0, 52.0, true);
            let color = state.calendars.get(&e.calendar).map(|c| c.color.clone()).unwrap_or_else(|| RED.into());
            s.stack(&format!("{id}_bar"), &id, 20.0, yy + 8.0, 4.0, 36.0, Some(&color), 2.0, None);
            s.text(&format!("{id}_time"), &id, &if e.all_day { t(&locale, "全天", "All day").into() } else { fmt_hm(e.start_time()) }, 36.0, yy + 4.0, 62.0, 24.0, 15.0, true, INK, Align::Left);
            s.text(&format!("{id}_title"), &id, e.title.get(&locale), 100.0, yy + 4.0, 250.0, 24.0, 15.0, false, INK, Align::Left);
            let from = if e.created_by != self.device { state.devices.get(&e.created_by).map(|d| format!(" · {}", d.name.get(&locale))).unwrap_or_default() } else { String::new() };
            let sub = format!("{}{}", state.calendars.get(&e.calendar).map(|c| c.name.get(&locale).to_owned()).unwrap_or_default(), from);
            s.text(&format!("{id}_sub"), &id, &sub, 100.0, yy + 28.0, 250.0, 18.0, 12.0, false, GRAY, Align::Left);
            s.icon(&format!("{id}_chevron"), &id, "chevron", 366.0, yy + 18.0, 16.0, 16.0, GRAY2);
        }
        s.stack("list_end", "list", 0.0, events.len() as f64 * 56.0 + 8.0, 1.0, 1.0, None, 0.0, None);
        self.tab_bar(s, "tab_today");
    }
    fn render_day(&mut self, s: &mut Scene, state: &State, day: NaiveDate) {
        let locale = self.locale.clone();
        self.nav_button(s, "back_month", Action::Back, Some("back"), &month_title(day.year(), day.month(), &locale), 12.0, 140.0, Align::Left);
        self.nav_button(s, "day_prev", Action::DayPrev, Some("back"), "", 262.0, 40.0, Align::Left);
        self.nav_button(s, "day_next", Action::DayNext, Some("forward"), "", 302.0, 40.0, Align::Left);
        self.nav_button(s, "add_event", Action::Add, Some("plus"), "", 350.0, 44.0, Align::Left);
        let title = format!("{}{}", fmt_date(day, &locale), if day == today() { t(&locale, " · 今天", " · Today") } else { "" });
        s.text("day_title", "page", &title, 20.0, 50.0, 366.0, 32.0, 24.0, true, INK, Align::Left);
        let events = state.visible(Some(day));
        let n = events.len();
        s.text("day_sub", "page", &if locale == "en" { format!("{n} event{}", if n == 1 { "" } else { "s" }) } else { format!("{n} 项日程") }, 20.0, 84.0, 366.0, 18.0, 13.0, false, GRAY, Align::Left);
        let mut top = 108.0;
        for (i, e) in events.iter().filter(|e| e.all_day).enumerate() {
            let id = self.ctl(&format!("allday_{}", ident(&e.id)), Action::OpenEvent(e.id.clone()));
            let color = state.calendars.get(&e.calendar).map(|c| c.color.clone()).unwrap_or_else(|| RED.into());
            s.button(&id, "page", 20.0, top + i as f64 * 34.0, 366.0, 30.0, true);
            s.stack(&format!("{id}_bg"), &id, 20.0, top + i as f64 * 34.0, 366.0, 30.0, Some(&format!("{color}22")), 8.0, None);
            s.text(&format!("{id}_title"), &id, &format!("{} · {}", t(&locale, "全天", "All day"), e.title.get(&locale)), 32.0, top + i as f64 * 34.0, 340.0, 30.0, 14.0, true, INK, Align::Left);
        }
        top += events.iter().filter(|e| e.all_day).count() as f64 * 34.0;
        let hour_h = 48.0;
        // Start the timeline an hour before the first event (or at 07:00), so the day is in view without scrolling.
        let first_hour = events.iter().filter(|e| !e.all_day).map(|e| e.start_time().hour()).min().map(|h| h.saturating_sub(1)).unwrap_or(7).min(7);
        s.scroll("timeline", "page", 0.0, top, 406.0, 660.0 - top);
        let head = 12.0; // room for the first hour label, which sits centred on its line
        for h in first_hour..24u32 {
            let yy = head + (h - first_hour) as f64 * hour_h;
            s.text(&format!("hour_{h}"), "timeline", &format!("{h:02}:00"), 12.0, yy - 8.0, 44.0, 16.0, 11.0, false, GRAY, Align::Right);
            s.stack(&format!("hour_line_{h}"), "timeline", 64.0, yy, 330.0, 1.0, Some(LINE), 0.0, None);
        }
        s.stack("timeline_end", "timeline", 0.0, head + (24 - first_hour) as f64 * hour_h + 4.0, 1.0, 1.0, None, 0.0, None);
        if day == today() { let n = model::now(); let yy = head + (n.hour() as f64 + n.minute() as f64 / 60.0 - first_hour as f64) * hour_h; s.stack("now_dot", "timeline", 58.0, yy - 4.0, 8.0, 8.0, Some(RED), 4.0, None); s.stack("now_line", "timeline", 64.0, yy - 1.0, 330.0, 2.0, Some(RED), 0.0, None); }
        // Overlapping timed events share the width.
        let timed: Vec<&&Event> = events.iter().filter(|e| !e.all_day).collect();
        for (i, e) in timed.iter().enumerate() {
            let start = e.start_time(); let end = e.end_time();
            let minutes = (end - start).num_minutes().max(20) as f64;
            let yy = head + (start.hour() as f64 + start.minute() as f64 / 60.0 - first_hour as f64) * hour_h;
            let hh = minutes / 60.0 * hour_h - 2.0;
            let lane_count = timed.iter().filter(|o| model::overlaps(&o.start, &o.end, &e.start, &e.end)).count().max(1) as f64;
            let lane = timed[..i].iter().filter(|o| model::overlaps(&o.start, &o.end, &e.start, &e.end)).count() as f64;
            let w = (322.0 - (lane_count - 1.0) * 4.0) / lane_count;
            let x = 68.0 + lane * (w + 4.0);
            let id = self.ctl(&format!("ev_{}", ident(&e.id)), Action::OpenEvent(e.id.clone()));
            let color = state.calendars.get(&e.calendar).map(|c| c.color.clone()).unwrap_or_else(|| RED.into());
            s.button(&id, "timeline", x, yy + 1.0, w, hh, true);
            s.stack(&format!("{id}_bg"), &id, x, yy + 1.0, w, hh, Some(&format!("{color}22")), 8.0, None);
            s.stack(&format!("{id}_bar"), &id, x, yy + 1.0, 4.0, hh, Some(&color), 2.0, None);
            s.text(&format!("{id}_title"), &id, e.title.get(&locale), x + 12.0, yy + 3.0, w - 16.0, 20.0, 14.0, true, INK, Align::Left);
            if hh > 36.0 { s.text(&format!("{id}_time"), &id, &format!("{} – {}{}", fmt_hm(start), fmt_hm(end), if e.location.is_empty() { String::new() } else { format!(" · {}", e.location.get(&locale)) }), x + 12.0, yy + 23.0, w - 16.0, 16.0, 12.0, false, &color, Align::Left); }
        }
        self.tab_bar(s, "tab_today");
    }
    fn render_detail(&mut self, s: &mut Scene, state: &State, id: &str) {
        let locale = self.locale.clone();
        let Some(e) = state.events.get(id).cloned() else { self.nav_button(s, "back", Action::Back, Some("back"), t(&locale, "返回", "Back"), 12.0, 120.0, Align::Left); s.text("gone", "page", t(&locale, "这条日程已不存在", "This event no longer exists"), 20.0, 120.0, 366.0, 24.0, 15.0, false, GRAY, Align::Left); return; };
        self.nav_button(s, "back", Action::Back, Some("back"), &fmt_date(e.date(), &locale), 12.0, 180.0, Align::Left);
        if !e.deleted { self.nav_button(s, "edit_event", Action::Edit, None, t(&locale, "编辑", "Edit"), 306.0, 88.0, Align::Right); }
        let color = state.calendars.get(&e.calendar).map(|c| c.color.clone()).unwrap_or_else(|| RED.into());
        s.stack("detail_head", "page", 20.0, 56.0, 366.0, 92.0, Some(&format!("{color}1a")), 14.0, None);
        s.stack("detail_bar", "page", 20.0, 56.0, 5.0, 92.0, Some(&color), 3.0, None);
        s.text("event_title", "page", e.title.get(&locale), 40.0, 66.0, 330.0, 32.0, 24.0, true, INK, Align::Left);
        let cal = state.calendars.get(&e.calendar);
        let shared = cal.map(|c| !c.shared_with.is_empty()).unwrap_or(false);
        let cal_line = format!("{}{}{}", cal.map(|c| c.name.get(&locale).to_owned()).unwrap_or_default(), t(&locale, "日历", " calendar"), if shared { t(&locale, " · 与 Sam 共享", " · shared with Sam") } else { "" });
        s.text("event_calendar", "page", &cal_line, 40.0, 102.0, 330.0, 18.0, 13.0, false, GRAY, Align::Left);
        if e.deleted { s.text("event_deleted", "page", t(&locale, "已删除 · 可撤销", "Deleted · can be restored"), 40.0, 122.0, 330.0, 18.0, 13.0, true, RED, Align::Left); }
        let when = if locale == "en" { format!("{}, {}", fmt_date(e.date(), "en"), e.date().year()) } else { format!("{}年{}", e.date().year(), fmt_date(e.date(), "cn")) };
        let range = if e.all_day { t(&locale, "全天", "All day").to_owned() } else { format!("{} – {}", fmt_hm(e.start_time()), fmt_hm(e.end_time())) };
        let by = if e.updated_by == self.device { t(&locale, "在本机更改", "changed on this device").to_owned() } else { let d = state.devices.get(&e.updated_by).map(|d| d.name.get(&locale).to_owned()).unwrap_or_else(|| e.updated_by.clone()); if locale == "en" { format!("changed by {d} · synced here") } else { format!("由 {d} 更改 · 已同步到本机") } };
        let rows: Vec<(&str, &str, String, String)> = vec![
            ("clock", "detail_when", when, range),
            ("pin", "detail_where", if e.location.is_empty() { t(&locale, "未设置地点", "No location").into() } else { e.location.get(&locale).to_owned() }, String::new()),
            ("bell", "detail_alert", t(&locale, "提醒", "Alert").into(), alert_label(e.alert, &locale)),
            ("note", "detail_notes", t(&locale, "备注", "Notes").into(), if e.notes.is_empty() { t(&locale, "无", "None").into() } else { e.notes.clone() }),
            ("sync", "detail_sync", t(&locale, "同步", "Sync").into(), by),
        ];
        s.stack("detail_group", "page", 20.0, 166.0, 366.0, rows.len() as f64 * 52.0 + 10.0, Some(GROUP), 14.0, None);
        for (i, (icon, id, main, sub)) in rows.iter().enumerate() {
            let yy = 176.0 + i as f64 * 52.0;
            s.icon(&format!("{id}_icon"), "page", icon, 36.0, yy + 6.0, 22.0, 22.0, GRAY);
            s.text(id, "page", main, 72.0, yy, 300.0, 22.0, 15.0, false, INK, Align::Left);
            s.text(&format!("{id}_sub"), "page", sub, 72.0, yy + 22.0, 300.0, 18.0, 13.0, false, GRAY, Align::Left);
            if i + 1 < rows.len() { s.stack(&format!("{id}_line"), "page", 72.0, yy + 46.0, 300.0, 1.0, Some(LINE), 0.0, None); }
        }
        let y0 = 166.0 + rows.len() as f64 * 52.0 + 30.0;
        if e.deleted { let id = self.ctl("restore_event", Action::Restore); s.labelled(&id, "page", t(&locale, "撤销删除", "Undo delete"), 20.0, y0, 366.0, 50.0, "filled", 17.0, true); }
        else { let id = self.ctl("delete_event", Action::Delete); s.labelled(&id, "page", t(&locale, "删除日程", "Delete Event"), 20.0, y0, 366.0, 50.0, "danger", 17.0, true); }
        if let Some(error) = self.last_error.clone() { s.text("detail_error", "page", &error, 20.0, y0 + 60.0, 366.0, 20.0, 13.0, false, RED, Align::Center); }
        self.tab_bar(s, "");
    }
    fn render_editor(&mut self, s: &mut Scene, state: &State) {
        let locale = self.locale.clone();
        let Some(d) = self.draft.clone() else { self.screen = Screen::Month; return self.render_month(s, state); };
        let cancel = self.ctl("cancel_editor", Action::Cancel);
        s.labelled(&cancel, "page", t(&locale, "取消", "Cancel"), 12.0, 8.0, 80.0, 36.0, "text", 17.0, true);
        s.text("nav_title", "page", t(&locale, if d.editing { "编辑日程" } else { "新建日程" }, if d.editing { "Edit Event" } else { "New Event" }), 100.0, 8.0, 206.0, 36.0, 17.0, true, INK, Align::Center);
        let conflict = self.draft_conflict(&d);
        let valid = !d.title.trim().is_empty() && conflict.is_none() && (d.all_day || d.end > d.start);
        let save = self.ctl("save_event", Action::Save);
        s.labelled(&save, "page", t(&locale, if d.editing { "完成" } else { "添加" }, if d.editing { "Done" } else { "Add" }), 318.0, 12.0, 72.0, 30.0, "filled", 15.0, valid);
        s.stack("title_group", "page", 20.0, 58.0, 366.0, 100.0, Some(GROUP), 14.0, None);
        s.input("title_field", "page", &d.title, t(&locale, "标题", "Title"), 36.0, 66.0, 334.0, 40.0, 17.0, self.focus.as_deref() == Some("title_field"));
        s.stack("title_line", "page", 36.0, 108.0, 334.0, 1.0, Some(LINE), 0.0, None);
        s.input("location_field", "page", &d.location, t(&locale, "地点或视频通话", "Location or Video Call"), 36.0, 116.0, 334.0, 36.0, 17.0, self.focus.as_deref() == Some("location_field"));
        s.stack("time_group", "page", 20.0, 172.0, 366.0, 150.0, Some(GROUP), 14.0, None);
        s.text("allday_label", "page", t(&locale, "全天", "All-day"), 36.0, 184.0, 200.0, 26.0, 17.0, false, INK, Align::Left);
        let allday = self.ctl("allday_toggle", Action::AllDayToggle);
        s.button(&allday, "page", 320.0, 180.0, 62.0, 34.0, true);
        s.stack("allday_switch", &allday, 330.0, 184.0, 51.0, 31.0, Some(if d.all_day { GREEN } else { "d1d1d6" }), 16.0, None);
        s.stack("allday_knob", &allday, if d.all_day { 352.0 } else { 332.0 }, 186.0, 27.0, 27.0, Some(WHITE), 14.0, None);
        s.stack("allday_line", "page", 36.0, 222.0, 334.0, 1.0, Some(LINE), 0.0, None);
        let pick_start = self.ctl("pick_start", Action::Pick(PickField::Start));
        s.button(&pick_start, "page", 20.0, 226.0, 366.0, 46.0, true);
        s.text("pick_start_label", &pick_start, t(&locale, "开始", "Starts"), 36.0, 236.0, 100.0, 26.0, 17.0, false, INK, Align::Left);
        let start_text = if d.all_day { fmt_date(d.start.date(), &locale) } else { format!("{}  {}", fmt_date(d.start.date(), &locale), fmt_hm(d.start)) };
        s.stack("start_chip", &pick_start, 176.0, 234.0, 194.0, 30.0, Some("e3e3e8"), 8.0, None);
        s.text("start_value", &pick_start, &start_text, 176.0, 234.0, 194.0, 30.0, 14.0, false, INK, Align::Center);
        s.stack("start_line", "page", 36.0, 274.0, 334.0, 1.0, Some(LINE), 0.0, None);
        let pick_end = self.ctl("pick_end", Action::Pick(PickField::End));
        s.button(&pick_end, "page", 20.0, 278.0, 366.0, 42.0, !d.all_day);
        s.text("pick_end_label", &pick_end, t(&locale, "结束", "Ends"), 36.0, 284.0, 100.0, 26.0, 17.0, false, if d.all_day { GRAY2 } else { INK }, Align::Left);
        let end_text = if d.all_day { t(&locale, "全天", "All day").to_owned() } else if d.end.date() == d.start.date() { fmt_hm(d.end) } else { format!("{}  {}", fmt_date(d.end.date(), &locale), fmt_hm(d.end)) };
        s.stack("end_chip", &pick_end, 176.0, 284.0, 194.0, 30.0, Some("e3e3e8"), 8.0, None);
        s.text("end_value", &pick_end, &end_text, 176.0, 284.0, 194.0, 30.0, 14.0, false, if d.all_day { GRAY2 } else { INK }, Align::Center);
        s.stack("meta_group", "page", 20.0, 336.0, 366.0, 100.0, Some(GROUP), 14.0, None);
        let choose = self.ctl("pick_calendar", Action::ChooseCalendar);
        s.button(&choose, "page", 20.0, 340.0, 366.0, 46.0, true);
        s.text("pick_calendar_label", &choose, t(&locale, "日历", "Calendar"), 36.0, 350.0, 100.0, 26.0, 17.0, false, INK, Align::Left);
        let cal = state.calendars.get(&d.calendar);
        s.stack("calendar_dot", &choose, 262.0, 358.0, 10.0, 10.0, Some(&cal.map(|c| c.color.clone()).unwrap_or_else(|| RED.into())), 5.0, None);
        s.text("calendar_value", &choose, cal.map(|c| c.name.get(&locale)).unwrap_or(""), 278.0, 350.0, 80.0, 26.0, 17.0, false, GRAY, Align::Right);
        s.icon("calendar_chevron", &choose, "chevron", 362.0, 355.0, 16.0, 16.0, GRAY2);
        s.stack("calendar_line", "page", 36.0, 388.0, 334.0, 1.0, Some(LINE), 0.0, None);
        let alert = self.ctl("alert_cycle", Action::AlertCycle);
        s.button(&alert, "page", 20.0, 392.0, 366.0, 42.0, true);
        s.text("alert_label", &alert, t(&locale, "提醒", "Alert"), 36.0, 398.0, 100.0, 26.0, 17.0, false, INK, Align::Left);
        s.text("alert_value", &alert, &alert_label(d.alert, &locale), 180.0, 398.0, 178.0, 26.0, 17.0, false, GRAY, Align::Right);
        s.icon("alert_chevron", &alert, "chevron", 362.0, 403.0, 16.0, 16.0, GRAY2);
        s.stack("notes_group", "page", 20.0, 450.0, 366.0, 60.0, Some(GROUP), 14.0, None);
        s.input("notes_field", "page", &d.notes, t(&locale, "备注", "Notes"), 36.0, 460.0, 334.0, 40.0, 16.0, self.focus.as_deref() == Some("notes_field"));
        let hint = if let Some(other) = conflict.clone() { if locale == "en" { format!("Overlaps “{other}” — choose another time") } else { format!("与「{other}」重叠，请更换时间") } }
                   else if d.title.trim().is_empty() { t(&locale, "请输入标题", "Add a title").into() }
                   else if self.connected { t(&locale, "保存后会同步到其他设备", "Saving syncs this to the other device").into() } else { t(&locale, "未连接服务器：保存后记录在本机", "Not connected: saved on this device").into() };
        s.stack("hint", "page", 20.0, 526.0, 366.0, 40.0, Some(if conflict.is_some() { "fef3f2" } else { "eef7ff" }), 12.0, None);
        s.text("hint_text", "page", &hint, 20.0, 526.0, 366.0, 40.0, 13.0, false, if conflict.is_some() { RED } else { BLUE }, Align::Center);
        if d.editing { let del = self.ctl("delete_from_editor", Action::Delete); let _ = del; }
        if let Some(error) = self.last_error.clone() { s.text("editor_error", "page", &error, 20.0, 576.0, 366.0, 20.0, 13.0, false, RED, Align::Center); }
    }
    fn render_picker(&mut self, s: &mut Scene, state: &State, field: PickField) {
        let locale = self.locale.clone();
        let Some(d) = self.draft.clone() else { self.screen = Screen::Month; return self.render_month(s, state); };
        self.nav_button(s, "back_editor", Action::Back, Some("back"), t(&locale, "日程", "Event"), 12.0, 120.0, Align::Left);
        s.text("picker_title", "page", t(&locale, if field == PickField::Start { "开始时间" } else { "结束时间" }, if field == PickField::Start { "Starts" } else { "Ends" }), 20.0, 50.0, 300.0, 32.0, 24.0, true, INK, Align::Left);
        let prev = self.ctl("picker_prev", Action::PickerDay(-1));
        s.labelled(&prev, "page", "‹", 20.0, 90.0, 44.0, 36.0, "outline", 20.0, true);
        let next = self.ctl("picker_next", Action::PickerDay(1));
        s.labelled(&next, "page", "›", 342.0, 90.0, 44.0, 36.0, "outline", 20.0, true);
        let day_text = if locale == "en" { format!("{}, {}", fmt_date(d.picker_day, "en"), d.picker_day.year()) } else { format!("{}年{}", d.picker_day.year(), fmt_date(d.picker_day, "cn")) };
        s.text("picker_day", "page", &day_text, 70.0, 90.0, 266.0, 36.0, 17.0, true, INK, Align::Center);
        s.text("picker_hint", "page", t(&locale, "与现有日程重叠的时段不可选", "Slots overlapping an existing event are disabled"), 20.0, 132.0, 366.0, 18.0, 12.0, false, GRAY, Align::Center);
        s.scroll("slots", "page", 0.0, 156.0, 406.0, 504.0);
        let duration = (d.end - d.start).max(Duration::minutes(15));
        let mut n = 0;
        for half in 0..48u32 {
            let (h, m) = (half / 2, (half % 2) * 30);
            let chosen = at(d.picker_day, h, m);
            let (start, end) = match field { PickField::Start => (chosen, chosen + duration), PickField::End => (d.start, chosen) };
            let conflict = if field == PickField::End && chosen <= d.start { None } else { state.conflict(&iso(start), &iso(end), if d.editing { Some(&d.id) } else { None }).map(|e| e.title.get(&locale).to_owned()) };
            let enabled = conflict.is_none() && !(field == PickField::End && chosen <= d.start);
            let current = match field { PickField::Start => d.start == chosen, PickField::End => d.end == chosen };
            let (col_i, row) = (half % 2, half / 2);
            let (x, yy) = (20.0 + col_i as f64 * 186.0, row as f64 * 50.0);
            let id = self.ctl(&format!("slot_{h:02}{m:02}"), Action::Slot(h, m));
            s.button(&id, "slots", x, yy, 180.0, 44.0, enabled);
            s.stack(&format!("{id}_surface"), &id, x, yy, 180.0, 44.0, Some(if !enabled { "f7f7f9" } else if current { "ffe5e3" } else { GROUP }), 12.0, if current { Some(RED) } else { None });
            s.text(&format!("{id}_label"), &id, &format!("{h:02}:{m:02}"), x + 14.0, yy + (if conflict.is_some() { 4.0 } else { 10.0 }), 100.0, 24.0, 16.0, false, if enabled { INK } else { GRAY2 }, Align::Left);
            if let Some(other) = conflict { s.text(&format!("{id}_why"), &id, &other, x + 14.0, yy + 24.0, 160.0, 16.0, 11.0, false, GRAY, Align::Left); }
            if current { s.icon(&format!("{id}_check"), &id, "check", x + 150.0, yy + 12.0, 20.0, 20.0, RED); }
            n += 1;
        }
        s.stack("slots_end", "slots", 0.0, (n as f64 / 2.0).ceil() * 50.0 + 8.0, 1.0, 1.0, None, 0.0, None);
    }
    fn render_chooser(&mut self, s: &mut Scene, state: &State) {
        let locale = self.locale.clone();
        self.nav_button(s, "back_editor", Action::Back, Some("back"), t(&locale, "日程", "Event"), 12.0, 120.0, Align::Left);
        s.text("chooser_title", "page", t(&locale, "日历", "Calendar"), 20.0, 50.0, 300.0, 32.0, 24.0, true, INK, Align::Left);
        let current = self.draft.as_ref().map(|d| d.calendar.clone()).unwrap_or_default();
        s.stack("chooser_group", "page", 20.0, 96.0, 366.0, state.calendars.len() as f64 * 52.0 + 4.0, Some(GROUP), 14.0, None);
        for (i, (id, c)) in ordered_calendars(state).into_iter().enumerate() {
            let yy = 100.0 + i as f64 * 52.0;
            let ctl = self.ctl(&format!("choose_{}", ident(id)), Action::SetCalendar(id.clone()));
            s.button(&ctl, "page", 20.0, yy, 366.0, 50.0, true);
            s.stack(&format!("{ctl}_dot"), &ctl, 36.0, yy + 19.0, 12.0, 12.0, Some(&c.color), 6.0, None);
            s.text(&format!("{ctl}_label"), &ctl, c.name.get(&locale), 60.0, yy + 12.0, 240.0, 26.0, 17.0, false, INK, Align::Left);
            if *id == current { s.icon(&format!("{ctl}_check"), &ctl, "check", 350.0, yy + 14.0, 22.0, 22.0, RED); }
            if i + 1 < state.calendars.len() { s.stack(&format!("{ctl}_line"), "page", 60.0, yy + 50.0, 316.0, 1.0, Some(LINE), 0.0, None); }
        }
    }
    fn render_calendars(&mut self, s: &mut Scene, state: &State) {
        let locale = self.locale.clone();
        s.text("nav_title", "page", t(&locale, "日历", "Calendars"), 100.0, 8.0, 206.0, 36.0, 17.0, true, INK, Align::Center);
        let lang = self.ctl("toggle_locale", Action::ToggleLocale);
        s.labelled(&lang, "page", t(&locale, "English", "中文"), 296.0, 8.0, 98.0, 36.0, "text", 15.0, true);
        s.text("section_icloud", "page", t(&locale, "本机与 Sam 共享", "On this device · shared with Sam"), 36.0, 58.0, 300.0, 18.0, 13.0, false, GRAY, Align::Left);
        s.stack("calendars_group", "page", 20.0, 80.0, 366.0, state.calendars.len() as f64 * 52.0 + 4.0, Some(GROUP), 14.0, None);
        for (i, (id, c)) in ordered_calendars(state).into_iter().enumerate() {
            let yy = 84.0 + i as f64 * 52.0;
            let ctl = self.ctl(&format!("toggle_{}", ident(id)), Action::ToggleCalendar(id.clone()));
            s.button(&ctl, "page", 20.0, yy, 366.0, 50.0, true);
            s.icon(&format!("{ctl}_check"), &ctl, "check", 32.0, yy + 14.0, 22.0, 22.0, if c.visible { &c.color } else { "e5e5ea" });
            s.stack(&format!("{ctl}_dot"), &ctl, 66.0, yy + 19.0, 12.0, 12.0, Some(&c.color), 6.0, None);
            let sub = if !c.visible { t(&locale, "已隐藏", "Hidden").to_owned() } else if !c.shared_with.is_empty() { t(&locale, "与 Sam 共享", "Shared with Sam").to_owned() } else { String::new() };
            s.text(&format!("{ctl}_label"), &ctl, c.name.get(&locale), 88.0, yy + if sub.is_empty() { 12.0 } else { 4.0 }, 240.0, 26.0, 17.0, false, if c.visible { INK } else { GRAY }, Align::Left);
            if !sub.is_empty() { s.text(&format!("{ctl}_sub"), &ctl, &sub, 88.0, yy + 28.0, 240.0, 18.0, 12.0, false, GRAY, Align::Left); }
            let count = state.events.values().filter(|e| !e.deleted && e.calendar == *id).count();
            s.text(&format!("{ctl}_count"), &ctl, &count.to_string(), 300.0, yy + 12.0, 70.0, 26.0, 15.0, false, GRAY2, Align::Right);
            if i + 1 < state.calendars.len() { s.stack(&format!("{ctl}_line"), "page", 66.0, yy + 50.0, 310.0, 1.0, Some(LINE), 0.0, None); }
        }
        let y0 = 84.0 + state.calendars.len() as f64 * 52.0 + 24.0;
        s.stack("sync_group", "page", 20.0, y0, 366.0, 64.0, Some(GROUP), 14.0, None);
        s.icon("sync_icon", "page", "sync", 36.0, y0 + 21.0, 22.0, 22.0, if self.connected { GREEN } else { GRAY });
        let me = state.devices.get(&self.device);
        let status = if self.connected { format!("{} · {}", t(&locale, "已同步", "Synced"), me.map(|d| model::hm(&d.last_sync)).unwrap_or_default()) } else { t(&locale, "未连接同步服务", "Not connected to the sync service").to_owned() };
        s.text("sync_status", "page", &status, 72.0, y0 + 8.0, 300.0, 24.0, 15.0, false, INK, Align::Left);
        let names: Vec<String> = state.devices.values().map(|d| d.name.get(&locale).to_owned()).collect();
        s.text("sync_devices", "page", &format!("{} {}: {}", state.devices.len(), t(&locale, "台设备", "devices"), names.join(", ")), 72.0, y0 + 34.0, 300.0, 18.0, 12.0, false, GRAY, Align::Left);
        self.tab_bar(s, "tab_calendars");
    }
    fn render_inbox(&mut self, s: &mut Scene, state: &State) {
        let locale = self.locale.clone();
        s.text("nav_title", "page", t(&locale, "收件箱", "Inbox"), 100.0, 8.0, 206.0, 36.0, 17.0, true, INK, Align::Center);
        s.scroll("inbox", "page", 0.0, 52.0, 406.0, 608.0);
        let mut yy = 8.0;
        let invitations: Vec<_> = state.invitations.values().cloned().collect();
        if invitations.is_empty() && self.unseen.is_empty() { s.text("inbox_empty", "inbox", t(&locale, "没有新的邀请或更改", "No invitations or changes"), 20.0, 20.0, 366.0, 24.0, 15.0, false, GRAY2, Align::Center); }
        for inv in &invitations {
            s.stack(&format!("card_{}", ident(&inv.id)), "inbox", 20.0, yy, 366.0, 190.0, Some(WHITE), 16.0, Some(LINE));
            s.stack(&format!("tile_{}", ident(&inv.id)), "inbox", 40.0, yy + 18.0, 36.0, 36.0, Some(BLUE), 10.0, None);
            s.icon(&format!("icon_{}", ident(&inv.id)), "inbox", "people", 47.0, yy + 25.0, 22.0, 22.0, WHITE);
            let kicker = match inv.status.as_str() { "accepted" => t(&locale, "已接受 · 已加入日历", "Accepted · in your calendar"), "maybe" => t(&locale, "已回复 · 待定", "Answered · Maybe"), "declined" => t(&locale, "已拒绝", "Declined"), _ => t(&locale, "邀请 · 来自 Sam", "Invitation · from Sam") };
            s.text(&format!("kicker_{}", ident(&inv.id)), "inbox", kicker, 88.0, yy + 27.0, 280.0, 18.0, 13.0, false, GRAY, Align::Left);
            s.text(&format!("title_{}", ident(&inv.id)), "inbox", inv.title.get(&locale), 40.0, yy + 66.0, 320.0, 30.0, 22.0, true, INK, Align::Left);
            let when = format!("{} · {}–{}", fmt_date(model::date_of(&inv.start).unwrap_or(today()), &locale), model::hm(&inv.start), model::hm(&inv.end));
            s.text(&format!("when_{}", ident(&inv.id)), "inbox", &when, 40.0, yy + 98.0, 320.0, 20.0, 14.0, false, INK, Align::Left);
            s.text(&format!("where_{}", ident(&inv.id)), "inbox", &format!("{} · {}", inv.location.get(&locale), state.calendars.get(&inv.calendar).map(|c| c.name.get(&locale).to_owned()).unwrap_or_default()), 40.0, yy + 120.0, 320.0, 18.0, 13.0, false, GRAY, Align::Left);
            for (k, (status, cn, en, style)) in [("accepted", "接受", "Accept", "blue"), ("maybe", "待定", "Maybe", "outline"), ("declined", "拒绝", "Decline", "outline")].iter().enumerate() {
                let id = self.ctl(&format!("invite_{}_{status}", ident(&inv.id)), Action::Invite(inv.id.clone(), status.to_string()));
                s.labelled(&id, "inbox", t(&locale, cn, en), 40.0 + k as f64 * 110.0, yy + 146.0, 100.0, 36.0, style, 15.0, inv.status != *status);
            }
            yy += 202.0;
        }
        let unseen = self.unseen.clone();
        for (i, change) in unseen.iter().enumerate() {
            let id = self.ctl(&format!("change_{i}"), Action::ShowChange(i));
            s.button(&id, "inbox", 20.0, yy, 366.0, 78.0, true);
            s.stack(&format!("{id}_card"), &id, 20.0, yy, 366.0, 78.0, Some(WHITE), 16.0, Some(LINE));
            s.stack(&format!("{id}_tile"), &id, 40.0, yy + 20.0, 36.0, 36.0, Some(RED), 10.0, None);
            s.icon(&format!("{id}_icon"), &id, "calendar", 47.0, yy + 27.0, 22.0, 22.0, WHITE);
            let from = state.devices.get(&change.from).map(|d| d.name.get(&locale).to_owned()).unwrap_or_else(|| change.from.clone());
            let kind = match change.action.as_str() { "event.create" => t(&locale, "新增了日程", "added an event"), "event.update" => t(&locale, "更改了日程", "changed an event"), "event.delete" => t(&locale, "删除了日程", "removed an event"), "event.restore" => t(&locale, "恢复了日程", "restored an event"), _ => t(&locale, "回复了邀请", "answered an invitation") };
            let title = change.event_id.as_ref().and_then(|e| state.events.get(e)).map(|e| e.title.get(&locale).to_owned()).unwrap_or_default();
            s.text(&format!("{id}_kicker"), &id, &format!("{from} {kind}"), 88.0, yy + 16.0, 280.0, 20.0, 13.0, false, GRAY, Align::Left);
            s.text(&format!("{id}_title"), &id, &title, 88.0, yy + 38.0, 260.0, 24.0, 17.0, true, INK, Align::Left);
            s.icon(&format!("{id}_chevron"), &id, "chevron", 362.0, yy + 30.0, 16.0, 16.0, GRAY2);
            yy += 88.0;
        }
        s.stack("inbox_end", "inbox", 0.0, yy + 8.0, 1.0, 1.0, None, 0.0, None);
        self.tab_bar(s, "tab_inbox");
    }
    fn render_sync(&mut self, s: &mut Scene, state: &State) {
        let locale = self.locale.clone();
        s.text("nav_title", "page", t(&locale, "同步", "Sync"), 100.0, 8.0, 206.0, 36.0, 17.0, true, INK, Align::Center);
        s.stack("sync_card", "page", 20.0, 60.0, 366.0, 300.0, Some(WHITE), 16.0, Some(LINE));
        let ok = self.connected && self.last_error.is_none();
        s.stack("sync_tile", "page", 40.0, 78.0, 36.0, 36.0, Some(if ok { GREEN } else if self.connected { "ff9500" } else { GRAY }), 10.0, None);
        s.icon("sync_tile_icon", "page", "sync", 47.0, 85.0, 22.0, 22.0, WHITE);
        let kicker = if !self.connected { t(&locale, "同步 · 未连接", "Sync · Not connected") } else if !self.pending.is_empty() { t(&locale, "同步 · 有待发送更改", "Sync · Changes pending") } else if self.last_error.is_some() { t(&locale, "同步 · 出错", "Sync · Error") } else { t(&locale, "同步 · 最新", "Sync · Up to date") };
        s.text("sync_kicker", "page", kicker, 88.0, 87.0, 280.0, 18.0, 13.0, false, GRAY, Align::Left);
        s.text("sync_title", "page", &if self.connected { t(&locale, "两台设备共用一份日志", "One log, both devices").to_owned() } else { t(&locale, "本机模式", "On-device only").to_owned() }, 40.0, 126.0, 320.0, 30.0, 22.0, true, INK, Align::Left);
        let mut yy = 162.0;
        for (id, d) in &state.devices {
            let me = if *id == self.device { t(&locale, "（本机）", " (this device)") } else { "" };
            s.text(&format!("device_{}", ident(id)), "page", &format!("{}{} · {}{}", d.name.get(&locale), me, model::hm(&d.last_sync), d.sync_count.map(|n| format!(" · ×{n}")).unwrap_or_default()), 40.0, yy, 320.0, 20.0, 14.0, false, INK, Align::Left);
            yy += 22.0;
        }
        let server = self.server.clone().unwrap_or_else(|| t(&locale, "未配置服务器（CALENDAR_SERVER）", "No server configured (CALENDAR_SERVER)").into());
        s.text("sync_server", "page", &server, 40.0, yy + 4.0, 320.0, 18.0, 12.0, false, GRAY, Align::Left);
        s.text("sync_counts", "page", &format!("{} {} · {} {}", t(&locale, "日志序号", "log seq"), self.remote_seq, self.pending.len(), t(&locale, "项待发送", "pending")), 40.0, yy + 24.0, 320.0, 18.0, 13.0, false, GRAY, Align::Left);
        let now = self.ctl("sync_now", Action::SyncNow);
        s.labelled(&now, "page", t(&locale, "立即同步", "Sync now"), 40.0, 292.0, 150.0, 40.0, "green", 15.0, true);
        let re = self.ctl("reconnect", Action::Reconnect);
        s.labelled(&re, "page", t(&locale, "重试连接", "Retry"), 204.0, 292.0, 150.0, 40.0, "outline", 15.0, !self.connected || self.last_error.is_some());
        if let Some(error) = self.last_error.clone() { s.text("sync_error", "page", &error, 40.0, 336.0, 320.0, 18.0, 12.0, false, RED, Align::Left); }
        let notices = self.notices.clone();
        for (i, n) in notices.iter().enumerate() {
            let yy = 380.0 + i as f64 * 64.0;
            if yy > 600.0 { break; }
            let id = self.ctl(&format!("notice_{i}"), Action::DismissNotice(i));
            s.button(&id, "page", 20.0, yy, 366.0, 56.0, true);
            s.stack(&format!("{id}_card"), &id, 20.0, yy, 366.0, 56.0, Some("fef3f2"), 12.0, Some("fecdca"));
            s.text(&format!("{id}_title"), &id, &format!("{} {}", n.action, t(&locale, "被服务端拒绝", "was refused by the service")), 36.0, yy + 6.0, 320.0, 22.0, 14.0, true, "b42318", Align::Left);
            s.text(&format!("{id}_reason"), &id, &format!("{} · {}", n.reason, t(&locale, "点按关闭", "tap to dismiss")), 36.0, yy + 28.0, 320.0, 18.0, 12.0, false, "b42318", Align::Left);
        }
        self.tab_bar(s, "tab_sync");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn session() -> Session { std::env::set_var("CALENDAR_TODAY", "2026-09-24"); Session::new("alex-phone", "en") }
    fn render_ok(session: &mut Session) -> crate::scene::Scene {
        let scene = session.render("http://127.0.0.1:1/t");
        let frame = crate::scene::compile(&scene, "calendar-test");
        let report = octoscript_ui_l0::realize(&frame.card, &frame.data, Default::default());
        let root = report.complete_root().expect("complete root");
        let source = octoscript_ui_l0::kit_pack::lower(root, &frame.pack, &frame.data).expect("lower");
        let mut tree = octoscript_makepad::design::prepare(&source).expect("prepare");
        octoscript_makepad::l0::inspectable(&mut tree);
        assert!(!octoscript_makepad::design::to_makepad_ui(&tree).unwrap().is_empty());
        scene
    }

    #[test]
    fn every_screen_renders_and_lowers() {
        let mut s = session();
        render_ok(&mut s);
        assert!(s.activate("day_2026_09_24"));
        assert!(s.activate("day_2026_09_24"), "a second tap opens the day");
        assert_eq!(s.screen, Screen::Day(today()));
        render_ok(&mut s);
        assert!(s.activate("ev_dinner"));
        assert_eq!(s.screen, Screen::Detail("dinner".into()));
        render_ok(&mut s);
        assert!(s.activate("edit_event"));
        render_ok(&mut s);
        assert!(s.activate("pick_start"));
        render_ok(&mut s);
        assert!(s.activate("pick_calendar") == false, "not on the picker");
        assert!(s.activate("slot_2000"));
        assert_eq!(s.screen, Screen::Editor);
        render_ok(&mut s);
        assert!(s.activate("pick_calendar"));
        render_ok(&mut s);
        assert!(s.activate("choose_work"));
        render_ok(&mut s);
        assert!(s.activate("save_event"));
        let state = s.state();
        assert_eq!(state.events["dinner"].calendar, "work");
        assert_eq!(state.events["dinner"].start, "2026-09-24T20:00:00+08:00");
        assert_eq!(state.events["dinner"].end, "2026-09-24T21:30:00+08:00", "the duration is kept when the start moves");
        assert_eq!(s.pending.len(), 1);
        for tab in ["tab_calendars", "tab_inbox", "tab_sync"] { render_ok(&mut s); assert!(s.activate(tab)); render_ok(&mut s); }
    }

    #[test]
    fn creating_an_event_needs_a_title_and_refuses_overlaps() {
        let mut s = session();
        render_ok(&mut s);
        assert!(s.activate("add_event"));
        let scene = render_ok(&mut s);
        assert_eq!(scene.controls["save_event"]["enabled"], false, "no title yet");
        s.field_changed("title_field", "Coffee");
        let scene = render_ok(&mut s);
        assert_eq!(scene.controls["save_event"]["enabled"], true);
        // Move the start onto the team sync: the picker disables that slot and refuses it.
        assert!(s.activate("pick_start"));
        let scene = render_ok(&mut s);
        assert_eq!(scene.controls["slot_1400"]["enabled"], false);
        assert_eq!(scene.controls["slot_1500"]["enabled"], true);
        assert!(s.activate("slot_1500"));
        render_ok(&mut s);
        assert!(s.activate("save_event"));
        assert_eq!(s.pending[0].action, "event.create");
        assert_eq!(s.pending[0].payload["event"]["start"], "2026-09-24T15:00:00+08:00");
        assert!(matches!(s.screen, Screen::Detail(_)));
    }

    #[test]
    fn server_records_confirm_ours_roll_back_rejections_and_list_theirs() {
        let mut s = session();
        s.adopt_snapshot(0, model::seed(today()));
        render_ok(&mut s);
        assert!(s.activate("tab_calendars"));
        render_ok(&mut s);
        assert!(s.activate("toggle_birthdays"));
        assert!(s.state().calendars["birthdays"].visible);
        let mine = serde_json::to_value(&s.pending[0]).unwrap();
        let theirs = json!({"id": "sam-desktop:1", "actor": "sam-desktop", "action": "event.delete", "payload": {"id": "dentist"}});
        s.apply_records(&[json!({"seq": 1, "op": theirs, "accepted": true}), json!({"seq": 2, "op": mine, "accepted": false, "reason": "invalid_calendar"})]);
        assert!(s.pending.is_empty());
        assert!(!s.state().calendars["birthdays"].visible, "rolled back");
        assert!(s.state().events["dentist"].deleted);
        assert_eq!(s.unseen.len(), 1);
        assert_eq!(s.notices.len(), 1);
        assert_eq!(s.screen, Screen::Calendars, "the screen did not move");
        assert!(s.activate("tab_inbox"));
        render_ok(&mut s);
        assert!(s.activate("invite_hike_accepted"));
        assert_eq!(s.state().invitations["hike"].status, "accepted");
        assert!(s.state().events.contains_key("invite-hike"));
    }
}
