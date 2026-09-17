//! Native HarmonyOS transport. Framing/reduction is shared with stdio and WS.
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc;
use crate::proto::{build_outbound, handle_inbound_text, try_emit, Outbound, SharedState, CHANNEL_BUFFER};
use crate::{ConnectionState, OutboundCommand, TransportConfig, TransportEvent};

pub const CORE_VERSION: &str = octos_cli::embedded::VERSION;

struct CoreLog(Arc<dyn Fn(&str) + Send + Sync>);
impl std::io::Write for CoreLog {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        (self.0)(&String::from_utf8_lossy(bytes));
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}

pub fn spawn_with_waker(
    config: TransportConfig,
    wake: Option<Arc<dyn Fn() + Send + Sync>>,
    diagnostic: Arc<dyn Fn(&str) + Send + Sync>,
) -> (mpsc::Sender<OutboundCommand>, mpsc::Receiver<TransportEvent>) {
    let (commands, rx) = mpsc::channel(CHANNEL_BUFFER);
    let (events, mut inner) = mpsc::channel(CHANNEL_BUFFER);
    let (out, result) = mpsc::channel(CHANNEL_BUFFER);
    tokio::spawn(async move {
        while let Some(event) = inner.recv().await {
            if out.send(event).await.is_err() { break; }
            if let Some(wake) = &wake { wake(); }
        }
    });
    tokio::spawn(run(config, rx, events, diagnostic));
    (commands, result)
}

async fn run(
    config: TransportConfig,
    mut commands: mpsc::Receiver<OutboundCommand>,
    events: mpsc::Sender<TransportEvent>,
    diagnostic: Arc<dyn Fn(&str) + Send + Sync>,
) {
    let log_sink = diagnostic.clone();
    let _ = tracing_subscriber::fmt()
        .with_env_filter("info,reqwest=warn,hyper=warn,html5ever=warn,octos.prompt_cache=trace")
        .with_ansi(false)
        .with_writer(move || CoreLog(log_sink.clone()))
        .try_init();
    try_emit(&events, TransportEvent::ConnectionState(ConnectionState::Dialing));
    let home = std::path::PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join("octos-home");
    let (client, server) = tokio::io::duplex(1024 * 1024);
    let (reader, mut writer) = tokio::io::split(client);
    let (server_reader, server_writer) = tokio::io::split(server);
    let mut core = tokio::spawn(async move {
        octos_cli::embedded::serve_io(&home, server_reader, server_writer).await
    });
    let mut lines = BufReader::new(reader).lines();
    let persist = config.cursor_file.clone().map(|p| {
        Arc::new(crate::cursor::FileCursorPersist::new(p)) as Arc<dyn crate::cursor::CursorPersist>
    });
    let mut shared = SharedState::new(config.cursor.clone(), persist);
    let mut state = ConnectionState::Handshaking;
    try_emit(&events, TransportEvent::ConnectionState(state.clone()));
    diagnostic(&format!("embedded core {CORE_VERSION}: starting canonical OUP"));
    let mut failed = true;
    loop {
        tokio::select! {
            biased;
            result = &mut core => {
                diagnostic(&format!("embedded core stopped: {result:?}"));
                break;
            }
            command = commands.recv() => {
                let Some(command) = command else { failed = false; break; };
                match build_outbound(command, &mut shared) {
                    Outbound::Disconnect => { failed = false; break; }
                    Outbound::Skip => {}
                    Outbound::Send { id, frame, pending } => {
                        if writer.write_all(format!("{frame}\n").as_bytes()).await.is_err() { break; }
                        if let Some(pending) = pending { shared.pending.insert(id, pending); }
                    }
                }
            }
            line = lines.next_line() => {
                match line {
                    Ok(Some(text)) if !text.trim().is_empty() => {
                        if let Some(next) = handle_inbound_text(&text, &mut shared, &events, &mut state).await {
                            try_emit(&events, TransportEvent::ConnectionState(next));
                        }
                    }
                    Ok(Some(_)) => {}
                    _ => break,
                }
            }
        }
    }
    shared.registry.cancel_all();
    shared.pending.clear();
    drop(writer);
    // EOF lets the canonical dispatcher drain owned turns before shutdown.
    if !core.is_finished() && tokio::time::timeout(std::time::Duration::from_secs(12), &mut core).await.is_err() {
        core.abort();
    }
    if failed { try_emit(&events, TransportEvent::ConnectionState(ConnectionState::Failed)); }
}
