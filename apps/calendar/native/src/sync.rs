//! The device's link to the calendar service (server/calendar_server.py): a
//! worker thread speaking plain HTTP/1.1 over a TCP socket (the service is
//! loopback or USB-forwarded), long-polling the log and submitting this
//! device's operations. Results come back over a channel the view drains.
use serde_json::Value;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

pub enum Command { Submit(Value), Poll(i64), Stop }
pub enum Reply { Snapshot { seq: i64, state: Value }, Records(Vec<Value>), Submitted(Value), Error(String) }

#[derive(Clone)]
pub struct Endpoint { pub host: String, pub port: u16, pub path: String, pub token: String }

pub fn parse_endpoint(url: &str, token: &str) -> Option<Endpoint> {
    let rest = url.trim().strip_prefix("http://")?;
    let (authority, path) = rest.split_once('/').map(|(a, p)| (a, format!("/{p}"))).unwrap_or((rest, String::new()));
    let (host, port) = authority.split_once(':').map(|(h, p)| (h.to_owned(), p.parse().ok())).unwrap_or((authority.to_owned(), Some(80)));
    Some(Endpoint { host, port: port?, path: path.trim_end_matches('/').to_owned(), token: token.into() })
}

fn request(endpoint: &Endpoint, method: &str, path: &str, body: Option<&str>, timeout: Duration) -> Result<(u16, String), String> {
    let mut socket = TcpStream::connect_timeout(&format!("{}:{}", endpoint.host, endpoint.port).parse().map_err(|_| "bad server address".to_string())?, Duration::from_secs(5))
        .map_err(|e| format!("cannot reach {}:{} ({e})", endpoint.host, endpoint.port))?;
    socket.set_read_timeout(Some(timeout)).ok();
    socket.set_write_timeout(Some(Duration::from_secs(5))).ok();
    let body = body.unwrap_or("");
    let head = format!("{method} {}{path} HTTP/1.1\r\nHost: {}\r\nAuthorization: Bearer {}\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n", endpoint.path, endpoint.host, endpoint.token, body.len());
    socket.write_all(head.as_bytes()).and_then(|_| socket.write_all(body.as_bytes())).map_err(|e| format!("send failed: {e}"))?;
    let mut data = Vec::new();
    socket.read_to_end(&mut data).map_err(|e| format!("read failed: {e}"))?;
    let text = String::from_utf8_lossy(&data);
    let (head, body) = text.split_once("\r\n\r\n").ok_or("malformed response")?;
    let status: u16 = head.split_whitespace().nth(1).and_then(|s| s.parse().ok()).ok_or("malformed status")?;
    Ok((status, body.to_owned()))
}

/// Run the link until `Stop`: one thread submits this device's operations,
/// another long-polls the log, so a submission never waits behind a poll.
pub fn run(endpoint: Endpoint, commands: Receiver<Command>, replies: Sender<Reply>) {
    let poller = replies.clone();
    let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let poll_stop = stop.clone();
    let poll_endpoint = endpoint.clone();
    let _ = std::thread::Builder::new().name("calendar-poll".into()).spawn(move || poll(poll_endpoint, poller, poll_stop));
    loop {
        match commands.recv() {
            Ok(Command::Stop) | Err(_) => { stop.store(true, std::sync::atomic::Ordering::Release); return; }
            Ok(Command::Poll(_)) => {}
            Ok(Command::Submit(op)) => match request(&endpoint, "POST", "/v1/ops", Some(&op.to_string()), Duration::from_secs(10)) {
                Ok((200, body)) => { if let Ok(record) = serde_json::from_str::<Value>(&body) { let _ = replies.send(Reply::Submitted(record)); } }
                Ok((401, _)) => { let _ = replies.send(Reply::Error("unauthorized: check CALENDAR_TOKEN".into())); }
                Ok((status, _)) => { let _ = replies.send(Reply::Error(format!("submit refused: {status}"))); }
                Err(e) => { let _ = replies.send(Reply::Error(e)); }
            },
        }
    }
}

fn poll(endpoint: Endpoint, replies: Sender<Reply>, stop: std::sync::Arc<std::sync::atomic::AtomicBool>) {
    let mut since: Option<i64> = None;
    let mut backoff = Duration::from_millis(500);
    while !stop.load(std::sync::atomic::Ordering::Acquire) {
        if since.is_none() {
            match request(&endpoint, "GET", "/v1/state", None, Duration::from_secs(10)) {
                Ok((200, body)) => match serde_json::from_str::<Value>(&body) {
                    Ok(snapshot) => { since = snapshot["seq"].as_i64(); let _ = replies.send(Reply::Snapshot { seq: since.unwrap_or(0), state: snapshot["state"].clone() }); backoff = Duration::from_millis(500); }
                    Err(e) => { let _ = replies.send(Reply::Error(format!("bad snapshot: {e}"))); }
                },
                Ok((401, _)) => { let _ = replies.send(Reply::Error("unauthorized: check CALENDAR_TOKEN".into())); }
                Ok((status, _)) => { let _ = replies.send(Reply::Error(format!("server answered {status}"))); }
                Err(e) => { let _ = replies.send(Reply::Error(e)); }
            }
            if since.is_none() { std::thread::sleep(backoff); backoff = (backoff * 2).min(Duration::from_secs(15)); continue; }
        }
        let cursor = since.unwrap_or(0);
        match request(&endpoint, "GET", &format!("/v1/ops?since={cursor}&wait=20"), None, Duration::from_secs(30)) {
            Ok((200, body)) => match serde_json::from_str::<Value>(&body) {
                Ok(page) => {
                    let records = page["records"].as_array().cloned().unwrap_or_default();
                    if let Some(last) = records.last() { since = last["seq"].as_i64(); }
                    if !records.is_empty() { let _ = replies.send(Reply::Records(records)); }
                }
                Err(e) => { let _ = replies.send(Reply::Error(format!("bad records: {e}"))); }
            },
            Ok((status, _)) => { let _ = replies.send(Reply::Error(format!("poll answered {status}"))); since = None; }
            Err(e) => { let _ = replies.send(Reply::Error(e)); since = None; std::thread::sleep(Duration::from_secs(1)); }
        }
    }
}
