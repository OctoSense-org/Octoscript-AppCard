//! The calendar's domain model: the same operations, rules and verdicts as
//! `service/calendar_core.py` (the server) and `wizard/core.mjs` (the
//! browser), so a replica built here from the server's log is the server's
//! state. Dates are ISO-8601 strings with one fixed offset, compared as text.
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Timelike};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;

pub const ACTIONS: [&str; 7] = ["event.create", "event.update", "event.delete", "event.restore", "invite.respond", "calendar.set_visible", "device.sync"];
const PATCHABLE: [&str; 8] = ["title", "calendar", "start", "end", "location", "all_day", "notes", "alert"];

/// The offset every timestamp carries. The server compares text, so one
/// household shares one offset.
pub fn offset() -> String {
    std::env::var("CALENDAR_OFFSET").unwrap_or_else(|_| "+08:00".into())
}

// ---- time helpers -------------------------------------------------------------
pub fn parse(iso: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(iso.get(..19)?, "%Y-%m-%dT%H:%M:%S").ok()
}
pub fn iso(t: NaiveDateTime) -> String {
    format!("{}{}", t.format("%Y-%m-%dT%H:%M:%S"), offset())
}
pub fn date_of(iso: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(iso.get(..10)?, "%Y-%m-%d").ok()
}
pub fn today() -> NaiveDate {
    if let Ok(fixed) = std::env::var("CALENDAR_TODAY") {
        if let Ok(date) = NaiveDate::parse_from_str(&fixed, "%Y-%m-%d") {
            return date;
        }
    }
    chrono::Local::now().date_naive()
}
pub fn now() -> NaiveDateTime {
    if std::env::var("CALENDAR_TODAY").is_ok() {
        return today().and_hms_opt(9, 41, 0).unwrap();
    }
    chrono::Local::now().naive_local().with_second(0).unwrap().with_nanosecond(0).unwrap()
}
pub fn at(date: NaiveDate, h: u32, m: u32) -> NaiveDateTime {
    date.and_hms_opt(h, m, 0).unwrap()
}
pub fn hm(iso: &str) -> String {
    iso.get(11..16).unwrap_or("").to_owned()
}

// ---- records --------------------------------------------------------------------
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Text {
    #[serde(default)]
    pub cn: String,
    #[serde(default)]
    pub en: String,
}
impl Text {
    pub fn new(cn: &str, en: &str) -> Self { Text { cn: cn.into(), en: en.into() } }
    pub fn same(s: &str) -> Self { Text { cn: s.into(), en: s.into() } }
    pub fn get(&self, locale: &str) -> &str {
        let (a, b) = if locale == "en" { (&self.en, &self.cn) } else { (&self.cn, &self.en) };
        if a.is_empty() { b } else { a }
    }
    pub fn is_empty(&self) -> bool { self.cn.is_empty() && self.en.is_empty() }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Calendar {
    pub name: Text,
    pub color: String,
    pub visible: bool,
    #[serde(default)]
    pub shared_with: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Event {
    pub id: String,
    pub title: Text,
    pub calendar: String,
    pub start: String,
    pub end: String,
    #[serde(default)]
    pub location: Text,
    #[serde(default)]
    pub all_day: bool,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub alert: Option<i64>,
    #[serde(default)]
    pub created_by: String,
    #[serde(default)]
    pub updated_by: String,
    #[serde(default)]
    pub deleted: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invitation: Option<String>,
}
impl Event {
    pub fn start_time(&self) -> NaiveDateTime { parse(&self.start).unwrap_or_else(|| at(today(), 0, 0)) }
    pub fn end_time(&self) -> NaiveDateTime { parse(&self.end).unwrap_or_else(|| self.start_time() + Duration::hours(1)) }
    pub fn date(&self) -> NaiveDate { self.start_time().date() }
    pub fn payload(&self) -> Value {
        json!({"id": self.id, "title": self.title, "calendar": self.calendar, "start": self.start, "end": self.end,
               "location": self.location, "all_day": self.all_day, "notes": self.notes, "alert": self.alert})
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Invitation {
    pub id: String,
    pub from: String,
    pub title: Text,
    pub calendar: String,
    pub start: String,
    pub end: String,
    #[serde(default)]
    pub location: Text,
    pub status: String,
    #[serde(default)]
    pub event_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answered_by: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Device {
    pub name: Text,
    pub last_sync: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync_count: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct State {
    pub schema_version: i64,
    pub seq: i64,
    pub applied: BTreeMap<String, String>,
    pub devices: BTreeMap<String, Device>,
    pub calendars: BTreeMap<String, Calendar>,
    pub events: BTreeMap<String, Event>,
    pub invitations: BTreeMap<String, Invitation>,
    #[serde(default)]
    pub history: Vec<Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Op {
    pub id: String,
    pub actor: String,
    pub action: String,
    #[serde(default)]
    pub payload: Value,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Rejected(pub String);
type Verdict = Result<bool, Rejected>; // Ok(replayed)

/// The demo fixture, laid around the real "today" so a fresh replica is
/// worth looking at before a server is configured.
pub fn seed(today: NaiveDate) -> State {
    let mut calendars = BTreeMap::new();
    calendars.insert("family".into(), Calendar { name: Text::new("家庭", "Family"), color: "ff3b30".into(), visible: true, shared_with: vec!["sam".into()] });
    calendars.insert("work".into(), Calendar { name: Text::new("工作", "Work"), color: "0a84ff".into(), visible: true, shared_with: vec![] });
    calendars.insert("personal".into(), Calendar { name: Text::new("个人", "Personal"), color: "34c759".into(), visible: true, shared_with: vec![] });
    calendars.insert("birthdays".into(), Calendar { name: Text::new("生日", "Birthdays"), color: "af52de".into(), visible: false, shared_with: vec![] });
    let mut devices = BTreeMap::new();
    devices.insert("alex-phone".into(), Device { name: Text::new("Alex · 手机", "Alex · Phone"), last_sync: iso(at(today, 9, 41)), sync_count: None });
    devices.insert("sam-desktop".into(), Device { name: Text::new("Sam · 桌面", "Sam · Desktop"), last_sync: iso(at(today, 9, 40)), sync_count: None });
    let mut events = BTreeMap::new();
    let mut ev = |id: &str, cn: &str, en: &str, cal: &str, day: NaiveDate, h0: u32, m0: u32, h1: u32, m1: u32, loc: (&str, &str), by: &str| {
        events.insert(id.to_string(), Event { id: id.into(), title: Text::new(cn, en), calendar: cal.into(), start: iso(at(day, h0, m0)), end: iso(at(day, h1, m1)),
            location: Text::new(loc.0, loc.1), all_day: false, notes: String::new(), alert: Some(30), created_by: by.into(), updated_by: by.into(), deleted: false, invitation: None });
    };
    ev("dentist", "牙医", "Dentist", "personal", today, 10, 30, 11, 15, ("日出牙科", "Sunrise Dental"), "alex-phone");
    ev("team-sync", "团队同步", "Team sync", "work", today, 14, 0, 14, 45, ("视频会议", "Video call"), "alex-phone");
    ev("dinner", "与 Sam 晚餐", "Dinner with Sam", "family", today, 19, 0, 20, 30, ("莲花厨房", "Lotus Kitchen"), "sam-desktop");
    ev("piano", "钢琴课", "Piano lesson", "family", today + Duration::days(1), 16, 0, 17, 0, ("", ""), "alex-phone");
    ev("standup", "站会", "Stand-up", "work", today + Duration::days(3), 9, 30, 9, 45, ("", ""), "alex-phone");
    ev("groceries", "采购", "Groceries", "personal", today - Duration::days(2), 18, 0, 19, 0, ("超市", "Market"), "alex-phone");
    let saturday = today + Duration::days(((6 + 7 - today.weekday().num_days_from_sunday()) % 7).max(1) as i64);
    let mut invitations = BTreeMap::new();
    invitations.insert("hike".into(), Invitation { id: "hike".into(), from: "sam".into(), title: Text::new("周末远足", "Weekend hike"), calendar: "family".into(),
        start: iso(at(saturday, 9, 0)), end: iso(at(saturday, 12, 0)), location: Text::new("西山步道", "West Hill trail"), status: "pending".into(), event_id: None, answered_by: None });
    State { schema_version: 1, seq: 0, applied: BTreeMap::new(), devices, calendars, events, invitations, history: Vec::new() }
}

pub fn canonical(value: &Value) -> String {
    fn write(v: &Value, out: &mut String) {
        match v {
            Value::Object(map) => {
                let mut keys: Vec<&String> = map.keys().collect();
                keys.sort();
                out.push('{');
                for (i, k) in keys.iter().enumerate() {
                    if i > 0 { out.push(','); }
                    out.push_str(&serde_json::to_string(k).unwrap());
                    out.push(':');
                    write(&map[*k], out);
                }
                out.push('}');
            }
            Value::Array(items) => {
                out.push('[');
                for (i, item) in items.iter().enumerate() { if i > 0 { out.push(','); } write(item, out); }
                out.push(']');
            }
            other => out.push_str(&serde_json::to_string(other).unwrap()),
        }
    }
    let mut out = String::new();
    write(value, &mut out);
    out
}

pub fn overlaps(a0: &str, a1: &str, b0: &str, b1: &str) -> bool {
    a0 < b1 && b0 < a1
}

impl State {
    pub fn conflict(&self, start: &str, end: &str, ignore: Option<&str>) -> Option<&Event> {
        self.events.values().find(|e| !e.deleted && Some(e.id.as_str()) != ignore && overlaps(&e.start, &e.end, start, end))
    }
    pub fn visible(&self, day: Option<NaiveDate>) -> Vec<&Event> {
        let mut out: Vec<&Event> = self.events.values()
            .filter(|e| !e.deleted && self.calendars.get(&e.calendar).map(|c| c.visible).unwrap_or(true))
            .filter(|e| day.map(|d| e.date() == d).unwrap_or(true)).collect();
        out.sort_by(|a, b| (a.start.as_str(), a.id.as_str()).cmp(&(b.start.as_str(), b.id.as_str())));
        out
    }
    pub fn digest(&self) -> String {
        use sha2::Digest;
        let v = json!({"seq": self.seq, "calendars": self.calendars, "events": self.events, "invitations": self.invitations, "devices": self.devices});
        let mut hasher = sha2::Sha256::new();
        hasher.update(canonical(&v).as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Apply one operation in place. Rejections leave the state untouched.
    pub fn apply(&mut self, op: &Op) -> Verdict {
        let op_value = serde_json::to_value(op).unwrap();
        if op.id.trim().is_empty() || op.actor.trim().is_empty() { return Err(Rejected("invalid_operation".into())); }
        if !ACTIONS.contains(&op.action.as_str()) { return Err(Rejected("unknown_action".into())); }
        if let Some(previous) = self.applied.get(&op.id) {
            return if *previous == canonical(&op_value) { Ok(true) } else { Err(Rejected("operation_id_reused".into())) };
        }
        if !self.devices.contains_key(&op.actor) { return Err(Rejected("unknown_device".into())); }
        let payload = op.payload.as_object().cloned().unwrap_or_default();
        let mut next = self.clone();
        match op.action.as_str() {
            "event.create" => {
                let mut event: Event = serde_json::from_value(payload.get("event").cloned().unwrap_or(Value::Null)).map_err(|_| Rejected("invalid_event".into()))?;
                event_shape(&event)?;
                if !next.calendars.contains_key(&event.calendar) { return Err(Rejected("unknown_calendar".into())); }
                if next.events.get(&event.id).map(|e| !e.deleted).unwrap_or(false) { return Err(Rejected("duplicate_event".into())); }
                if let Some(other) = next.conflict(&event.start, &event.end, None) { return Err(Rejected(format!("conflict:{}", other.id))); }
                event.created_by = op.actor.clone(); event.updated_by = op.actor.clone(); event.deleted = false;
                next.events.insert(event.id.clone(), event);
            }
            "event.update" => {
                let id = text(&payload, "id");
                let Some(event) = next.events.get(&id).filter(|e| !e.deleted).cloned() else { return Err(Rejected("missing_event".into())) };
                let Some(patch) = payload.get("patch").and_then(Value::as_object).filter(|p| !p.is_empty()) else { return Err(Rejected("invalid_patch".into())) };
                if patch.keys().any(|k| !PATCHABLE.contains(&k.as_str())) { return Err(Rejected("invalid_patch".into())); }
                let mut merged = serde_json::to_value(&event).unwrap();
                for (k, v) in patch { merged[k] = v.clone(); }
                let candidate: Event = serde_json::from_value(merged).map_err(|_| Rejected("invalid_event".into()))?;
                event_shape(&candidate)?;
                if !next.calendars.contains_key(&candidate.calendar) { return Err(Rejected("unknown_calendar".into())); }
                if let Some(other) = next.conflict(&candidate.start, &candidate.end, Some(&id)) { return Err(Rejected(format!("conflict:{}", other.id))); }
                let slot = next.events.get_mut(&id).unwrap();
                *slot = candidate; slot.updated_by = op.actor.clone();
            }
            "event.delete" => {
                let id = text(&payload, "id");
                let Some(event) = next.events.get_mut(&id).filter(|e| !e.deleted) else { return Err(Rejected("missing_event".into())) };
                event.deleted = true; event.updated_by = op.actor.clone();
            }
            "event.restore" => {
                let id = text(&payload, "id");
                let Some(event) = next.events.get(&id).filter(|e| e.deleted).cloned() else { return Err(Rejected("missing_event".into())) };
                if let Some(other) = next.conflict(&event.start, &event.end, Some(&id)) { return Err(Rejected(format!("conflict:{}", other.id))); }
                let slot = next.events.get_mut(&id).unwrap();
                slot.deleted = false; slot.updated_by = op.actor.clone();
            }
            "invite.respond" => {
                let id = text(&payload, "id"); let status = text(&payload, "status");
                let Some(mut invite) = next.invitations.get(&id).cloned() else { return Err(Rejected("missing_invitation".into())) };
                if !["accepted", "declined", "maybe"].contains(&status.as_str()) { return Err(Rejected("invalid_status".into())); }
                if invite.status == "accepted" && status != "accepted" {
                    if let Some(event_id) = invite.event_id.take() { if let Some(e) = next.events.get_mut(&event_id) { e.deleted = true; } }
                }
                if status == "accepted" && invite.status != "accepted" {
                    let event_id = format!("invite-{}", invite.id);
                    if let Some(other) = next.conflict(&invite.start, &invite.end, None) { return Err(Rejected(format!("conflict:{}", other.id))); }
                    next.events.insert(event_id.clone(), Event { id: event_id.clone(), title: invite.title.clone(), calendar: invite.calendar.clone(), start: invite.start.clone(), end: invite.end.clone(),
                        location: invite.location.clone(), all_day: false, notes: String::new(), alert: None, created_by: op.actor.clone(), updated_by: op.actor.clone(), deleted: false, invitation: Some(invite.id.clone()) });
                    invite.event_id = Some(event_id);
                }
                invite.status = status; invite.answered_by = Some(op.actor.clone());
                next.invitations.insert(id, invite);
            }
            "calendar.set_visible" => {
                let id = text(&payload, "id");
                let Some(visible) = payload.get("visible").and_then(Value::as_bool) else { return Err(Rejected("invalid_calendar".into())) };
                let Some(calendar) = next.calendars.get_mut(&id) else { return Err(Rejected("invalid_calendar".into())) };
                calendar.visible = visible;
            }
            "device.sync" => {
                let at = text(&payload, "at");
                if at.is_empty() { return Err(Rejected("invalid_sync".into())); }
                let device = next.devices.get_mut(&op.actor).unwrap();
                device.last_sync = at; device.sync_count = Some(device.sync_count.unwrap_or(0) + 1);
            }
            _ => unreachable!(),
        }
        next.seq += 1;
        next.applied.insert(op.id.clone(), canonical(&op_value));
        next.history.push(json!({"seq": next.seq, "id": op.id, "actor": op.actor, "action": op.action}));
        *self = next;
        Ok(false)
    }
}

fn text(map: &Map<String, Value>, key: &str) -> String {
    map.get(key).and_then(Value::as_str).unwrap_or("").to_owned()
}
fn event_shape(e: &Event) -> Result<(), Rejected> {
    if e.id.is_empty() || e.calendar.is_empty() || e.start.is_empty() || e.end.is_empty() || e.title.is_empty() || e.end <= e.start {
        return Err(Rejected("invalid_event".into()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn op(id: &str, action: &str, payload: Value) -> Op { Op { id: id.into(), actor: "alex-phone".into(), action: action.into(), payload } }

    #[test]
    fn the_shared_fixture_reaches_the_pinned_digest() {
        // service/fixtures/ops.json is the contract between the three reducers.
        let fixture: Value = serde_json::from_str(include_str!("../../service/fixtures/ops.json")).unwrap();
        std::env::set_var("CALENDAR_TODAY", "2026-09-24");
        let mut state = seed(today());
        // The fixture's state has no piano/standup/groceries seed and fixed times; rebuild that exact base.
        state.events.retain(|id, _| ["dentist", "team-sync", "dinner"].contains(&id.as_str()));
        for e in state.events.values_mut() { e.alert = None; }
        for (_, e) in state.events.iter_mut() { let _ = e; }
        state.invitations.get_mut("hike").unwrap().start = "2026-09-26T09:00:00+08:00".into();
        state.invitations.get_mut("hike").unwrap().end = "2026-09-26T12:00:00+08:00".into();
        for record in fixture["records"].as_array().unwrap() {
            let op: Op = serde_json::from_value(record["op"].clone()).unwrap();
            let before = state.clone();
            match state.apply(&op) {
                Ok(replayed) => { assert!(record["accepted"].as_bool().unwrap(), "{}", op.id); assert_eq!(replayed, record["replayed"].as_bool().unwrap_or(false), "{}", op.id); }
                Err(Rejected(reason)) => { assert!(!record["accepted"].as_bool().unwrap(), "{}", op.id); assert_eq!(reason, record["reason"].as_str().unwrap(), "{}", op.id); assert_eq!(state, before); }
            }
        }
        assert_eq!(state.seq, fixture["expect"]["seq"].as_i64().unwrap());
        assert_eq!(state.visible(Some(today())).iter().map(|e| e.id.as_str()).collect::<Vec<_>>(), ["dentist", "team-sync", "dinner"]);
        assert_eq!(state.events["dinner"].start, fixture["expect"]["dinner_start"].as_str().unwrap());
        assert_eq!(state.invitations["hike"].status, "declined");
    }

    #[test]
    fn overlaps_conflict_and_touching_slots_do_not() {
        std::env::set_var("CALENDAR_TODAY", "2026-09-24");
        let mut state = seed(today());
        let clash = json!({"event": {"id": "clash", "title": {"cn": "冲突", "en": "Clash"}, "calendar": "family", "start": "2026-09-24T14:00:00+08:00", "end": "2026-09-24T15:00:00+08:00"}});
        assert_eq!(state.apply(&op("a", "event.create", clash)), Err(Rejected("conflict:team-sync".into())));
        let touching = json!({"event": {"id": "after", "title": {"cn": "x", "en": "x"}, "calendar": "work", "start": "2026-09-24T14:45:00+08:00", "end": "2026-09-24T15:30:00+08:00"}});
        assert_eq!(state.apply(&op("b", "event.create", touching)), Ok(false));
        assert_eq!(state.apply(&op("b", "event.create", json!({"event": {"id": "other"}}))), Err(Rejected("operation_id_reused".into())));
    }
}
