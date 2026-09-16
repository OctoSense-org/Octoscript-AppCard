//! On-device POP3 and SMTP. Credentials are never included in protocol errors.
use base64::{engine::general_purpose::STANDARD, Engine};
use mailparse::{MailHeaderMap, ParsedMail};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::HashSet,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpStream, ToSocketAddrs},
    sync::Arc,
    time::Duration,
};

pub fn text<'a>(v: &'a Value, key: &str) -> &'a str {
    v[key].as_str().unwrap_or("")
}
pub fn hash(value: &str) -> String {
    format!("{:x}", Sha256::digest(value.as_bytes()))
}
pub fn identity(account: &Value) -> String {
    hash(&format!(
        "{}\n{}",
        text(account, "host"),
        text(account, "username")
    ))[..24]
        .into()
}

pub fn defaults() -> Value {
    json!({"address":"","username":"","host":"pop.gmail.com","port":"995","security":"tls","recent":true,
        "smtp_host":"smtp.gmail.com","smtp_port":"465","smtp_security":"tls","password":"","sync_enabled":false})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mime_decodes_encoded_subject_and_multipart_body() {
        let raw=b"From: Alex <alex@example.com>\r\nSubject: =?UTF-8?B?SGVsbG8g5LiW55WM?=\r\nMIME-Version: 1.0\r\nContent-Type: multipart/alternative; boundary=demo\r\n\r\n--demo\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Transfer-Encoding: base64\r\n\r\nRnVsbCBwbGFpbiBjb250ZW50\r\n--demo\r\nContent-Type: text/html; charset=utf-8\r\n\r\n<p>Full <strong>HTML</strong> content</p>\r\n--demo--\r\n";
        let message = decode(raw, "stable-uid").unwrap();
        assert_eq!(message["subject"], "Hello 世界");
        assert_eq!(message["body"], "Full plain content");
        assert!(text(&message, "html").contains("<strong>HTML</strong>"));
        assert_eq!(message["id"], decode(raw, "stable-uid").unwrap()["id"]);
    }
    #[test]
    fn account_rejects_protocol_injection_and_plaintext_modes() {
        let mut account = defaults();
        account["address"] = json!("reader@example.com");
        account["username"] = json!("reader@example.com");
        account["password"] = json!("fixture-secret");
        assert!(validate(&account).is_ok());
        account["username"] = json!("reader\r\nDELE 1");
        assert!(validate(&account).is_err());
        account["username"] = json!("reader@example.com");
        account["security"] = json!("plain");
        assert!(validate(&account).is_err());
    }
}
pub fn validate(account: &Value) -> Result<(), String> {
    for key in ["address", "username", "host", "smtp_host", "password"] {
        let value = text(account, key);
        if value.is_empty() || value.contains(['\r', '\n', '\0']) {
            return Err(format!("Enter a valid {key}."));
        }
    }
    if !text(account, "address").contains('@') {
        return Err("Enter an email address.".into());
    }
    for key in ["port", "smtp_port"] {
        if text(account, key)
            .parse::<u16>()
            .ok()
            .filter(|n| *n > 0)
            .is_none()
        {
            return Err("Port must be between 1 and 65535.".into());
        }
    }
    for key in ["security", "smtp_security"] {
        if !matches!(text(account, key), "tls" | "starttls") {
            return Err("Choose TLS or STARTTLS.".into());
        }
    }
    Ok(())
}

enum Wire {
    Plain(BufReader<TcpStream>),
    Tls(BufReader<rustls::StreamOwned<rustls::ClientConnection, TcpStream>>),
}
impl Wire {
    fn connect(host: &str, port: u16) -> Result<Self, String> {
        let addresses = (host, port)
            .to_socket_addrs()
            .map_err(|_| "Cannot resolve the mail server. Check Wi-Fi.")?;
        let mut socket = None;
        for address in addresses.take(4) {
            if let Ok(stream) = TcpStream::connect_timeout(&address, Duration::from_secs(12)) {
                socket = Some(stream);
                break;
            }
        }
        let socket = socket.ok_or("Cannot connect to the mail server. Check Wi-Fi.")?;
        socket
            .set_read_timeout(Some(Duration::from_secs(20)))
            .map_err(|_| "Cannot configure mail connection.")?;
        socket
            .set_write_timeout(Some(Duration::from_secs(20)))
            .map_err(|_| "Cannot configure mail connection.")?;
        Ok(Self::Plain(BufReader::new(socket)))
    }
    fn tls(self, host: &str) -> Result<Self, String> {
        let Self::Plain(reader) = self else {
            return Err("Connection is already encrypted.".into());
        };
        if !reader.buffer().is_empty() {
            return Err("Unexpected bytes before TLS.".into());
        }
        let roots = rustls::RootCertStore {
            roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
        };
        let config = rustls::ClientConfig::builder_with_provider(Arc::new(
            rustls::crypto::ring::default_provider(),
        ))
        .with_safe_default_protocol_versions()
        .map_err(|_| "TLS configuration failed.")?
        .with_root_certificates(roots)
        .with_no_client_auth();
        let name = rustls::pki_types::ServerName::try_from(host.to_owned())
            .map_err(|_| "Invalid TLS server name.")?;
        let mut client = rustls::ClientConnection::new(Arc::new(config), name)
            .map_err(|_| "TLS setup failed.")?;
        let mut socket = reader.into_inner();
        while client.is_handshaking() {
            client
                .complete_io(&mut socket)
                .map_err(|_| "TLS verification failed. Check server name and device date.")?;
        }
        Ok(Self::Tls(BufReader::new(rustls::StreamOwned::new(
            client, socket,
        ))))
    }
    fn write(&mut self, bytes: &[u8]) -> Result<(), String> {
        let writer: &mut dyn Write = match self {
            Self::Plain(r) => r.get_mut(),
            Self::Tls(r) => r.get_mut(),
        };
        writer
            .write_all(bytes)
            .and_then(|_| writer.flush())
            .map_err(|_| "Connection interrupted while writing mail data.".into())
    }
    fn line(&mut self) -> Result<Vec<u8>, String> {
        let reader: &mut dyn BufRead = match self {
            Self::Plain(r) => r,
            Self::Tls(r) => r,
        };
        let mut out = Vec::new();
        // A bounded read also covers malformed servers with no newline.
        reader
            .take(2_100_000)
            .read_until(b'\n', &mut out)
            .map_err(|_| "Mail connection timed out or closed.")?;
        if !out.ends_with(b"\n") {
            return Err("Invalid or oversized mail response.".into());
        }
        Ok(out)
    }
    fn pop(&mut self, command: &str) -> Result<Vec<u8>, String> {
        self.write(format!("{command}\r\n").as_bytes())?;
        let line = self.line()?;
        if !line.starts_with(b"+OK") {
            return Err(
                "Mail server rejected the request. Check account settings and app password.".into(),
            );
        }
        Ok(line)
    }
    fn multiline(&mut self, limit: usize) -> Result<Vec<Vec<u8>>, String> {
        let mut rows = Vec::new();
        let mut size = 0;
        loop {
            let mut line = self.line()?;
            if line == b".\r\n" || line == b".\n" {
                break;
            }
            if line.starts_with(b"..") {
                line.remove(0);
            }
            size += line.len();
            if size > limit {
                return Err("Message exceeds the download limit.".into());
            }
            rows.push(line);
        }
        Ok(rows)
    }
    fn smtp_response(&mut self, expected: &[u16]) -> Result<(), String> {
        for _ in 0..100 {
            let line = self.line()?;
            let code = std::str::from_utf8(line.get(..3).unwrap_or_default())
                .ok()
                .and_then(|v| v.parse::<u16>().ok())
                .unwrap_or(0);
            if !expected.contains(&code) {
                return Err(
                    "Outgoing server rejected the request. Check its settings and app password."
                        .into(),
                );
            }
            if line.get(3) != Some(&b'-') {
                return Ok(());
            }
        }
        Err("Invalid outgoing server response.".into())
    }
    fn smtp(&mut self, command: &str, expected: &[u16]) -> Result<(), String> {
        self.write(format!("{command}\r\n").as_bytes())?;
        self.smtp_response(expected)
    }
}

fn pop_connect(account: &Value) -> Result<Wire, String> {
    validate(account)?;
    let host = text(account, "host");
    let mut wire = Wire::connect(host, text(account, "port").parse().unwrap())?;
    if text(account, "security") == "tls" {
        wire = wire.tls(host)?;
    }
    if !wire.line()?.starts_with(b"+OK") {
        return Err("Invalid POP3 greeting.".into());
    }
    if text(account, "security") == "starttls" {
        wire.pop("STLS")?;
        wire = wire.tls(host)?;
    }
    let username = text(account, "username");
    let username =
        if host == "pop.gmail.com" && account["recent"] == true && !username.starts_with("recent:")
        {
            format!("recent:{username}")
        } else {
            username.into()
        };
    wire.pop(&format!("USER {username}"))?;
    wire.pop(&format!("PASS {}", text(account, "password")))?;
    Ok(wire)
}
pub fn test(account: &Value) -> Result<Value, String> {
    let mut wire = pop_connect(account)?;
    let count = String::from_utf8_lossy(&wire.pop("STAT")?)
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok())
        .ok_or("Invalid mailbox count.")?;
    let _ = wire.pop("QUIT");
    Ok(json!({"available":count,"tls_verified":true,"transport":"device-pop3"}))
}
pub fn fetch(account: &Value, seen: &HashSet<String>) -> Result<Value, String> {
    let mut wire = pop_connect(account)?;
    let available = String::from_utf8_lossy(&wire.pop("STAT")?)
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok())
        .ok_or("Invalid mailbox count.")?;
    wire.pop("UIDL")?;
    let rows = wire.multiline(20_000_000)?;
    wire.pop("LIST")?;
    let sizes: std::collections::HashMap<usize, usize> = wire
        .multiline(20_000_000)?
        .iter()
        .filter_map(|line| {
            let line = std::str::from_utf8(line).ok()?;
            let mut parts = line.split_whitespace();
            Some((parts.next()?.parse().ok()?, parts.next()?.parse().ok()?))
        })
        .collect();
    let remaining: Vec<(usize, String)> = rows
        .iter()
        .filter_map(|line| {
            let mut parts = std::str::from_utf8(line).ok()?.split_whitespace();
            let number = parts.next()?.parse().ok()?;
            let uid = parts.next()?.to_owned();
            (!seen.contains(&uid)).then_some((number, uid))
        })
        .collect();
    let mut messages = Vec::new();
    let mut skipped = Vec::new();
    for (number, uid) in remaining.iter().rev().take(25) {
        if sizes.get(number).copied().unwrap_or(usize::MAX) > 2_000_000 {
            skipped.push(uid.clone());
            continue;
        }
        wire.pop(&format!("RETR {number}"))?;
        let raw = wire.multiline(2_100_000)?.concat();
        messages.push(decode(&raw, uid)?);
    }
    let _ = wire.pop("QUIT");
    Ok(
        json!({"address":account["address"],"account_id":identity(account),"messages":messages,"available":available,
        "skipped_uids":skipped,"has_more":remaining.len()>25,"synced_at":chrono::Utc::now().to_rfc3339(),"tls_verified":true,"host":account["host"],"transport":"device-pop3"}),
    )
}

fn collect_parts(
    part: &ParsedMail<'_>,
    plain: &mut String,
    html: &mut String,
    images: &mut Value,
    attachments: &mut Vec<Value>,
) {
    let mime = part.ctype.mimetype.as_str();
    let disposition = part.get_content_disposition();
    let cid = part
        .headers
        .get_first_value("Content-ID")
        .unwrap_or_default()
        .trim_matches(['<', '>'])
        .to_owned();
    if !cid.is_empty()
        && matches!(
            mime,
            "image/png" | "image/jpeg" | "image/gif" | "image/webp"
        )
    {
        if let Ok(bytes) = part.get_body_raw() {
            images[cid] = format!("data:{mime};base64,{}", STANDARD.encode(bytes)).into();
        }
    } else if disposition.disposition == mailparse::DispositionType::Attachment
        || disposition.params.contains_key("filename")
    {
        if let Ok(bytes) = part.get_body_raw() {
            attachments.push(json!({"filename":disposition.params.get("filename").or_else(||part.ctype.params.get("name")).map(String::as_str).unwrap_or("attachment"),
                "mime":mime,"size":bytes.len(),"data":STANDARD.encode(&bytes),"sha256":format!("{:x}",Sha256::digest(&bytes))}));
        }
    } else if part.subparts.is_empty() {
        if mime == "text/plain" {
            if let Ok(value) = part.get_body() {
                if !plain.is_empty() {
                    plain.push('\n');
                }
                plain.push_str(&value);
            }
        }
        if mime == "text/html" {
            if let Ok(value) = part.get_body() {
                html.push_str(&value);
            }
        }
    }
    for child in &part.subparts {
        collect_parts(child, plain, html, images, attachments);
    }
}
pub fn decode(raw: &[u8], uid: &str) -> Result<Value, String> {
    let parsed = mailparse::parse_mail(raw).map_err(|_| "Cannot decode this email.")?;
    let header = |name: &str| parsed.headers.get_first_value(name).unwrap_or_default();
    let from = header("From");
    let addresses = mailparse::addrparse(&from).ok();
    let first = addresses
        .as_ref()
        .and_then(|a| a.first())
        .and_then(|a| match a {
            mailparse::MailAddr::Single(v) => Some(v),
            _ => None,
        });
    let address = first.map(|a| a.addr.clone()).unwrap_or_default();
    let sender = first
        .and_then(|a| a.display_name.clone())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| {
            if address.is_empty() {
                "Unknown sender".into()
            } else {
                address.clone()
            }
        });
    let mut plain = String::new();
    let mut html = String::new();
    let mut images = json!({});
    let mut attachments = Vec::new();
    collect_parts(
        &parsed,
        &mut plain,
        &mut html,
        &mut images,
        &mut attachments,
    );
    if plain.trim().is_empty() {
        plain = scraper::Html::parse_fragment(&html)
            .root_element()
            .text()
            .collect::<Vec<_>>()
            .join(" ");
    }
    if plain.trim().is_empty() {
        plain = "(No readable message body)".into();
    }
    let date = mailparse::dateparse(&header("Date"))
        .ok()
        .and_then(|n| chrono::DateTime::from_timestamp(n, 0))
        .unwrap_or_else(chrono::Utc::now);
    let subject = header("Subject");
    let subject = if subject.trim().is_empty() {
        "(No subject)".into()
    } else {
        subject
    };
    Ok(
        json!({"id":&hash(uid)[..24],"uid":uid,"sender":sender,"address":address,"subject":subject,
        "message_id":header("Message-ID"),"reply_to":header("Reply-To"),"references":header("References"),
        "body":plain,"preview":plain.split_whitespace().collect::<Vec<_>>().join(" ").chars().take(180).collect::<String>(),
        "html":html,"inline_images":images,"attachment_items":attachments,"attachments":attachments.len(),
        "date":date.to_rfc3339(),"time":date.format("%b %d").to_string(),"unread":true,"flagged":false,"archived":false,"source":"gmail"}),
    )
}

pub fn send(account: &Value, draft: &Value) -> Result<Value, String> {
    validate(account)?;
    let to = text(draft, "to").trim();
    if !to.contains('@') || to.contains(['\r', '\n', '<', '>', ' ', ',', ';']) {
        return Err("Enter one recipient email address.".into());
    }
    let subject = text(draft, "subject");
    if subject.contains(['\r', '\n']) {
        return Err("Subject cannot contain line breaks.".into());
    }
    let host = text(account, "smtp_host");
    let mut wire = Wire::connect(host, text(account, "smtp_port").parse().unwrap())?;
    if text(account, "smtp_security") == "tls" {
        wire = wire.tls(host)?;
    }
    wire.smtp_response(&[220])?;
    wire.smtp("EHLO octosense.local", &[250])?;
    if text(account, "smtp_security") == "starttls" {
        wire.smtp("STARTTLS", &[220])?;
        wire = wire.tls(host)?;
        wire.smtp("EHLO octosense.local", &[250])?;
    }
    let auth = STANDARD.encode(format!(
        "\0{}\0{}",
        text(account, "username"),
        text(account, "password")
    ));
    wire.smtp(&format!("AUTH PLAIN {auth}"), &[235])?;
    wire.smtp(&format!("MAIL FROM:<{}>", text(account, "address")), &[250])?;
    wire.smtp(&format!("RCPT TO:<{to}>"), &[250, 251])?;
    wire.smtp("DATA", &[354])?;
    let id = text(draft, "message_id");
    let encoded = STANDARD.encode(text(draft, "body"));
    let lines = encoded
        .as_bytes()
        .chunks(76)
        .map(|s| std::str::from_utf8(s).unwrap())
        .collect::<Vec<_>>()
        .join("\r\n");
    let mut message=format!("From: {}\r\nTo: {to}\r\nSubject: =?UTF-8?B?{}?=\r\nDate: {}\r\nMessage-ID: {id}\r\nMIME-Version: 1.0\r\nContent-Type: text/plain; charset=UTF-8\r\nContent-Transfer-Encoding: base64\r\n",text(account,"address"),STANDARD.encode(subject),chrono::Utc::now().to_rfc2822());
    for (key, name) in [("in_reply_to", "In-Reply-To"), ("references", "References")] {
        let v = text(draft, key);
        if !v.is_empty() && !v.contains(['\r', '\n']) {
            message.push_str(&format!("{name}: {v}\r\n"));
        }
    }
    message.push_str(&format!("\r\n{lines}\r\n.\r\n"));
    // After DATA begins, connection loss must never trigger an automatic retry.
    wire.write(message.as_bytes())
        .and_then(|_| wire.smtp_response(&[250]))
        .map_err(|_| "Delivery uncertain. Check Gmail Sent before sending again.")?;
    let _ = wire.smtp("QUIT", &[221]);
    Ok(json!({"message_id":id,"accepted":true}))
}
