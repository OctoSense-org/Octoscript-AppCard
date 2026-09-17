//! `OctosUiAgent` — Makepad `Agent` implementation that bridges the UI to
//! `octos-app-transport`.
//!
//! Owns a Tokio runtime + the `(cmd_tx, evt_rx)` pair returned by
//! `octos_app_transport::ws::spawn`. UI calls translate to `OutboundCommand`s;
//! transport notifications drained on `handle_event` translate to `AgentEvent`s
//! the chat surface already understands.
//!
//! Crate boundary: `OctosUiAgent` is the *only* place inside `app/` that
//! talks to `octos-app-transport`. UI code goes through the `Agent` trait.

use std::collections::{HashMap, HashSet};

use makepad_ai::{Agent, AgentEvent, PromptId, SessionConfig, SessionId, StopReason};
use makepad_widgets::*;
use octos_app_store::state::{reduce as store_reduce, ConnectionEvent, Event as StoreEvent};
use octos_app_store::toasts::{Toast, ToastKind};
use octos_app_transport::{
    stdio, ws, Capabilities, ConnectionState, LifecycleResult, OutboundCommand, TransportConfig,
    TransportEvent,
};
use octos_core::app_ui::{
    AppUiBackendEvent as UiNotification, AppUiInputItem as InputItem,
    AppUiInterruptTurn as TurnInterruptParams, AppUiOpenSession as SessionOpenParams,
    AppUiSubmitPrompt as TurnStartParams,
};
use octos_core::ui_protocol::{
    ApprovalDecision, ApprovalId, ApprovalRespondParams, PayloadV2, ReasoningEffortLevel,
    TaskOutputReadParams, UiCursor,
};
use octos_core::{ui_protocol::TurnId, SessionKey};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::app::sessions::APP_STATE;

/// Posted from the transport drain when a `session/hydrate` reply lands.
/// `App::handle_actions` folds it into `CHAT_DATA` if `session_id` still
/// matches the session the user resumed (guards against a stale reply after
/// the user has already switched again).
#[derive(Debug)]
pub struct SessionResumeHydrated {
    pub session_id: SessionId,
    /// `(role, content)` rows in seq order, roles as the wire sends them
    /// ("user" / "assistant" / other — the App filters).
    pub messages: Vec<(String, String)>,
}

/// `Agent` implementation backed by the Octos UI Protocol over WebSocket.
pub struct OctosUiAgent {
    /// Owned Tokio runtime — required because `ws::spawn` calls
    /// `tokio::spawn` internally and `app/` has no global runtime. Held for
    /// the agent's lifetime; the WS task lives inside it.
    _runtime: Runtime,
    /// Outbound side of the transport channel. Cloneable, lock-free
    /// `try_send` from the main thread.
    cmd_tx: Sender<OutboundCommand>,
    /// Inbound side; drained each tick on `handle_event`.
    evt_rx: Receiver<TransportEvent>,
    /// Makepad SessionId → octos-core SessionKey.
    session_keys: HashMap<SessionId, SessionKey>,
    /// octos-core SessionKey → Makepad SessionId (reverse lookup for
    /// notifications arriving from the wire).
    session_ids: HashMap<SessionKey, SessionId>,
    /// Sessions for which the server has answered `session/open`.
    ready_sessions: std::collections::HashSet<SessionId>,
    /// Makepad PromptId → octos-core TurnId.
    turn_ids: HashMap<PromptId, TurnId>,
    /// octos-core TurnId → Makepad PromptId (reverse lookup).
    prompt_ids: HashMap<TurnId, PromptId>,
    /// W08: Makepad PromptId → the SessionKey that owns it, so `cancel_prompt`
    /// (and future per-prompt ops) target the RIGHT session in a multi-session
    /// client instead of guessing the "first" one.
    prompt_sessions: HashMap<PromptId, SessionKey>,
    /// Per-turn timings contain no prompt text, credentials or request bodies.
    generation_started: HashMap<TurnId, (std::time::Instant, bool)>,
    app_prompt_cache: super::app_prompt_cache::AppPromptCache,
    /// Most recent connection state — also mirrored into
    /// `APP_STATE.connection` (via `fold_connection_into_store`) for the
    /// top-bar status indicator and toast queue. Kept locally so we can
    /// detect transitions (Reconnecting → Live, Live → Failed, …) without
    /// re-reading the store under a write lock.
    connection_state: ConnectionState,
    /// Server-negotiated capability set. W05 reads this when deciding which
    /// approval / pane affordances to show. Stored as soon as
    /// `CapabilityNegotiated` arrives.
    #[allow(dead_code)]
    capabilities: Option<Capabilities>,
    /// Workspace cwd requested for every `session/open`, when configured.
    workspace_cwd: Option<String>,
    /// Profile id from the transport config — fallback owner for sidebar
    /// rows the server returns without a `profile_id` (session/list).
    fallback_profile: String,
    /// "Thinking" composer toggle state. When on, every turn requests a
    /// per-turn reasoning-effort override (thinking-capable models: DeepSeek
    /// V4, OpenAI reasoning models, Grok-4). Set by `Agent::set_thinking`;
    /// `None` (off) falls back to the gateway/profile default.
    thinking: bool,
    /// True when the stdio transport is in use (spawned `octos serve --stdio`)
    /// rather than WebSocket. Stdio carries no `X-Profile-Id` header, so
    /// `session/open` must name the profile in its params instead.
    stdio_transport: bool,
    /// Turns whose durable `assistant_persisted` row has already been bridged
    /// as `TextAuthoritative`. A terminal for one of these can finalize at
    /// once; a terminal for any other turn must wait (see `finish_turn`).
    persisted_seen: HashSet<TurnId>,
    /// Turns that have reached their terminal but whose authoritative text
    /// has not arrived. Each waits for the durable row on the live lane, or
    /// for the `session/hydrate` reply requested on its behalf, or for
    /// [`AUTHORITATIVE_WAIT`] to expire — whichever comes first.
    pending_completion: HashMap<TurnId, PendingCompletion>,
    /// `session/hydrate` requests issued on behalf of held turns, per session,
    /// not yet answered. Their replies are the hold's business even when the
    /// live row has resolved the hold first — a reply that fell through to
    /// the resume flow would replace the chat with the session's history
    /// mid-turn.
    hold_hydrates: HashMap<SessionKey, u32>,
}

/// A turn held open at its terminal until the kernel's durable assistant row
/// is in hand — or the wait expires.
///
/// WHY THE WAIT EXISTS. `message/delta` is ephemeral: the kernel drops it on
/// the floor when its stdio writer backs up (a 1024-frame queue; a fast model
/// such as DeepSeek V4 Flash at ~200 deltas/s fills it on a phone), and the
/// live ledger forwarder that carries the durable `assistant_persisted` row
/// lags behind the DIRECT-sent legacy `turn/completed`. Measured on the
/// OnePlus: 897 of 1840 text deltas arrived, the persisted row had not, and
/// the terminal came anyway — so the turn finalized on 3251 of 7194 chars,
/// an unclosed ```runl0 fence, and the card drew as a code block. The kernel
/// keeps the whole reply in its ledger the entire time; `session/hydrate`
/// reads it back. Holding the terminal until that text is here is what makes
/// the card the model wrote the card that renders.
#[derive(Debug)]
struct PendingCompletion {
    session_id: SessionKey,
    since: std::time::Instant,
}

/// Upper bound on how long a completed turn waits for its durable text.
/// A kernel that never persists a row (an empty reply; a build without the
/// v2 lane) must not leave the spinner running forever, so after this the
/// turn completes on the streamed text with a warning in the log.
const AUTHORITATIVE_WAIT: std::time::Duration = std::time::Duration::from_secs(8);

impl OctosUiAgent {
    /// Construct a new agent. Spawns the WebSocket task immediately so
    /// `create_session` can ship `session/open` on the first call.
    ///
    /// On a missing / invalid env (no bearer, unreachable URL) the transport
    /// task drops to `ConnectionState::Failed` after the budget expires; the
    /// agent stays usable, sends fail silently into a closed channel, and
    /// `is_session_ready` stays `false` forever — matching M1's "boots even
    /// without a server" requirement.
    pub fn new(config: TransportConfig) -> Self {
        #[cfg(target_env = "ohos")]
        let config = TransportConfig {
            profile_id: octos_app_transport::ProfileId::new("_main"),
            ..config
        };
        let workspace_cwd = config.workspace_cwd.clone();
        let fallback_profile = config.profile_id.0.clone();
        let stdio_transport = config.stdio.is_some() || cfg!(target_env = "ohos");
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .expect("octos-ui-agent: tokio runtime build");
        let (cmd_tx, evt_rx) = {
            let _guard = runtime.enter();
            // Wake the makepad UI thread whenever a transport event queues —
            // otherwise replies that land while the app is idle (no touch,
            // no animation) sit undrained until the next unrelated event.
            let waker = Some(std::sync::Arc::new(|| {
                makepad_widgets::SignalToUI::set_ui_signal();
            }) as std::sync::Arc<dyn Fn() + Send + Sync>);
            #[cfg(target_env = "ohos")]
            {
                octos_app_transport::embedded::spawn_with_waker(config, waker,
                    std::sync::Arc::new(|message| log::info!("{message}")))
            }
            #[cfg(not(target_env = "ohos"))]
            // stdio spawns `octos serve --stdio` as a child; ws dials a socket.
            if stdio_transport {
                stdio::spawn_with_waker(config, waker)
            } else {
                ws::spawn_with_waker(config, waker)
            }
        };
        Self {
            _runtime: runtime,
            cmd_tx,
            evt_rx,
            session_keys: HashMap::new(),
            session_ids: HashMap::new(),
            ready_sessions: std::collections::HashSet::new(),
            turn_ids: HashMap::new(),
            prompt_ids: HashMap::new(),
            prompt_sessions: HashMap::new(),
            generation_started: HashMap::new(),
            app_prompt_cache: Default::default(),
            connection_state: ConnectionState::Idle,
            capabilities: None,
            workspace_cwd,
            fallback_profile,
            thinking: false,
            stdio_transport,
            persisted_seen: HashSet::new(),
            pending_completion: HashMap::new(),
            hold_hydrates: HashMap::new(),
        }
    }

    /// Synthesise a fresh `SessionKey` from a Makepad `SessionId`. The
    /// LiveId-as-u64 → hex string round-trip is stable for the agent's
    /// lifetime; the server treats the value as an opaque identifier.
    ///
    /// The key embeds a per-process boot nonce: `SessionId`s are sequential
    /// (1, 2, …) so without it every app launch would mint the same
    /// "octos-app:…0001" key and silently re-attach to (and grow) the
    /// previous launch's server session. Old sessions stay reachable via
    /// `session/list` + `resume_session`.
    fn make_session_key(&self, session_id: SessionId) -> SessionKey {
        static BOOT_NONCE: std::sync::LazyLock<u64> = std::sync::LazyLock::new(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0)
        });
        // stdio: the kernel resolves the turn's ProfileRuntime from the
        // SESSION KEY's `{profile}:{channel}:{chat_id}` shape
        // (`SessionKey::profile_id()` — the `session/open` param alone does
        // not bind it; see octos `run_native_*_turn`'s
        // `session_id.profile_id().or(routed_profile_id)`). Without the
        // prefix every turn/start fails with "No ProfileRuntime registered
        // for profile '<unset>'". `api` is a registry channel name
        // (`is_channel_name`), which the profile-prefix parse requires;
        // the WS transport keeps the legacy opaque key (profile rides the
        // `X-Profile-Id` header / bearer instead).
        if self.stdio_transport {
            SessionKey(format!(
                "{}:api:{:08x}-{:08x}",
                self.fallback_profile,
                *BOOT_NONCE,
                session_id.0.0 as u32
            ))
        } else {
            SessionKey(format!(
                "octos-app:{:08x}-{:08x}",
                *BOOT_NONCE,
                session_id.0.0 as u32
            ))
        }
    }

    /// Profile to name in `session/open` params. The stdio transport carries
    /// no `X-Profile-Id` header, so the profile must ride in the params; the
    /// WebSocket transport uses the header and leaves this `None` (unchanged
    /// behaviour).
    fn open_profile_id(&self) -> Option<String> {
        self.stdio_transport.then(|| self.fallback_profile.clone())
    }

    /// Best-effort post to the transport task. Logs (and drops) on a closed
    /// or full channel — the caller can't usefully recover here.
    fn post(&self, cmd: OutboundCommand) {
        match self.cmd_tx.try_send(cmd) {
            Ok(()) => {}
            Err(tokio::sync::mpsc::error::TrySendError::Full(_)) => {
                log::warn!("octos-ui-agent: command channel full; dropping");
            }
            Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => {
                log::warn!("octos-ui-agent: transport task gone; command dropped");
            }
        }
    }

    /// Forget every routing entry for `turn`. The prompt it mapped to, if
    /// it was tracked.
    fn retire_turn(&mut self, turn: &TurnId) -> Option<PromptId> {
        let pid = self.prompt_ids.remove(turn)?;
        self.turn_ids.remove(&pid);
        self.prompt_sessions.remove(&pid);
        self.generation_started.remove(turn);
        self.persisted_seen.remove(turn);
        self.pending_completion.remove(turn);
        Some(pid)
    }

    /// A turn reached a successful terminal on `lane`. It completes now if
    /// its durable assistant row has already been bridged; otherwise it is
    /// held and the row is fetched with `session/hydrate`, and the turn
    /// completes when either that reply or the live row lands — see
    /// [`PendingCompletion`] for why a terminal alone is not enough.
    fn finish_turn(&mut self, turn: &TurnId, lane: &str) -> Vec<AgentEvent> {
        let Some(&pid) = self.prompt_ids.get(turn) else {
            return Vec::new();
        };
        if self.persisted_seen.contains(turn) {
            self.retire_turn(turn);
            return vec![AgentEvent::TurnComplete {
                prompt_id: pid,
                stop_reason: StopReason::EndTurn,
            }];
        }
        if self.pending_completion.contains_key(turn) {
            // The other lane's terminal for a turn already held.
            return Vec::new();
        }
        let Some(session_id) = self.prompt_sessions.get(&pid).cloned() else {
            self.retire_turn(turn);
            return vec![AgentEvent::TurnComplete {
                prompt_id: pid,
                stop_reason: StopReason::EndTurn,
            }];
        };
        log::warn!(
            "octos-ui-agent: {lane} for turn {} arrived before its persisted row; \
             holding completion and hydrating {}",
            turn.0,
            session_id.0
        );
        self.pending_completion.insert(
            turn.clone(),
            PendingCompletion {
                session_id: session_id.clone(),
                since: std::time::Instant::now(),
            },
        );
        *self.hold_hydrates.entry(session_id.clone()).or_insert(0) += 1;
        self.post(OutboundCommand::HydrateSession {
            session_id: session_id.0,
        });
        Vec::new()
    }

    /// The durable text for `turn` is in hand. Bridged as authoritative; a
    /// turn held at its terminal completes right behind it.
    fn authoritative_arrived(&mut self, turn: &TurnId, text: String) -> Vec<AgentEvent> {
        let Some(&pid) = self.prompt_ids.get(turn) else {
            return Vec::new();
        };
        self.persisted_seen.insert(turn.clone());
        let mut out = vec![AgentEvent::TextAuthoritative {
            prompt_id: pid,
            text,
        }];
        if self.pending_completion.contains_key(turn) {
            self.retire_turn(turn);
            out.push(AgentEvent::TurnComplete {
                prompt_id: pid,
                stop_reason: StopReason::EndTurn,
            });
        }
        out
    }

    /// The `session/hydrate` reply requested for a held `turn`. Its
    /// assistant row for that turn — or, failing a turn-tagged row, the
    /// newest assistant row — is the text the turn completes on. A reply
    /// with no assistant text (an empty reply; a decode failure) completes
    /// the turn on what streamed, so it never hangs on a bad reply.
    fn complete_from_hydrate(&mut self, turn: &TurnId, result: serde_json::Value) -> Vec<AgentEvent> {
        let text = match serde_json::from_value::<octos_core::ui_protocol::SessionHydrateResult>(result) {
            Ok(r) => {
                let rows = r.messages.unwrap_or_default();
                let tagged = rows
                    .iter()
                    .rev()
                    .find(|row| row.role == "assistant" && row.turn_id.as_ref() == Some(turn));
                let row = tagged.or_else(|| {
                    log::warn!(
                        "octos-ui-agent: hydrate carries no assistant row tagged for turn {}; \
                         taking the newest",
                        turn.0
                    );
                    rows.iter().rev().find(|row| row.role == "assistant")
                });
                row.map(|row| row.content.clone()).filter(|text| !text.trim().is_empty())
            }
            Err(e) => {
                log::warn!("octos-ui-agent: decode session/hydrate for held turn: {e}");
                None
            }
        };
        match text {
            Some(text) => {
                log::info!(
                    "octos-ui-agent: held turn {} completes on its hydrated row ({} chars)",
                    turn.0,
                    text.chars().count()
                );
                self.authoritative_arrived(turn, text)
            }
            None => self
                .retire_turn(turn)
                .map(|pid| {
                    log::warn!(
                        "octos-ui-agent: no assistant text hydrated for turn {}; completing on the streamed text",
                        turn.0
                    );
                    vec![AgentEvent::TurnComplete {
                        prompt_id: pid,
                        stop_reason: StopReason::EndTurn,
                    }]
                })
                .unwrap_or_default(),
        }
    }

    /// Complete every held turn whose [`AUTHORITATIVE_WAIT`] has run out, on
    /// whatever text streamed.
    fn expire_pending_completions(&mut self) -> Vec<AgentEvent> {
        let expired: Vec<TurnId> = self
            .pending_completion
            .iter()
            .filter(|(_, p)| p.since.elapsed() >= AUTHORITATIVE_WAIT)
            .map(|(turn, _)| turn.clone())
            .collect();
        let mut out = Vec::new();
        for turn in expired {
            if let Some(pid) = self.retire_turn(&turn) {
                log::warn!(
                    "octos-ui-agent: no persisted row for turn {} within {:?}; completing on the streamed text",
                    turn.0,
                    AUTHORITATIVE_WAIT
                );
                out.push(AgentEvent::TurnComplete {
                    prompt_id: pid,
                    stop_reason: StopReason::EndTurn,
                });
            }
        }
        out
    }

    /// W05 — extract a cheap, send-safe handle for issuing
    /// `approval/respond` commands without cloning the whole agent. The
    /// handle wraps a `Sender<OutboundCommand>` plus a runtime handle, so
    /// `App::handle_actions` can fire `approval/respond` even though the
    /// agent itself is held behind `Box<dyn Agent>`.
    pub fn approval_handle(&self) -> ApprovalHandle {
        ApprovalHandle {
            cmd_tx: self.cmd_tx.clone(),
            runtime: self._runtime.handle().clone(),
        }
    }

    /// Cheap handle for one-shot `task/output/read` requests from the coding
    /// workspace. Mirrors `approval_handle` so UI code does not downcast the
    /// boxed `Agent`.
    pub fn task_output_handle(&self) -> TaskOutputHandle {
        TaskOutputHandle {
            cmd_tx: self.cmd_tx.clone(),
            runtime: self._runtime.handle().clone(),
        }
    }

    /// Translate one `TransportEvent` into zero-or-more `AgentEvent`s.
    /// Updates internal id maps and connection bookkeeping as a side effect.
    fn translate(&mut self, event: TransportEvent) -> Vec<AgentEvent> {
        match event {
            TransportEvent::ConnectionState(state) => {
                // An automatic first prompt can already be queued while the
                // initial connection opens. Preserve its pending reference;
                // reconnects still discard all previously trusted state.
                let initial_connection = matches!(self.connection_state,
                    ConnectionState::Idle | ConnectionState::Dialing | ConnectionState::Handshaking)
                    && matches!(state,
                        ConnectionState::Idle | ConnectionState::Dialing | ConnectionState::Handshaking);
                if state != ConnectionState::Live && !initial_connection {
                    self.app_prompt_cache.clear();
                }
                let prev = std::mem::replace(&mut self.connection_state, state.clone());
                self.fold_connection_into_store(&prev, &self.connection_state.clone());
                Vec::new()
            }
            TransportEvent::CapabilityNegotiated(caps) => {
                // W05 — mirror the negotiated capability flags into the
                // process-wide ApprovalsPane state. The widget reads
                // `APPROVAL_CAPS.read()` in `populate_card` to decide
                // whether to render typed sub-views and the scope dropdown.
                if let Ok(mut g) = crate::app::approvals::APPROVAL_CAPS.write() {
                    g.typed_approvals = caps.typed_approvals;
                    g.pane_snapshots = caps.pane_snapshots;
                }
                self.capabilities = Some(caps);
                // M12 D-5 — `GET /api/sessions` is retired; hydrate the
                // sidebar over the wire once the connection is live.
                self.post(OutboundCommand::ListSessions);
                Vec::new()
            }
            TransportEvent::RpcResult(LifecycleResult::SessionOpen(open)) => {
                let key = open.opened.session_id.clone();
                if let Some(sid) = self.session_ids.get(&key).copied() {
                    self.ready_sessions.insert(sid);
                    return vec![AgentEvent::SessionReady { session_id: sid }];
                }
                Vec::new()
            }
            TransportEvent::RpcResult(_) => Vec::new(),
            TransportEvent::RpcError {
                method, error, ..
            } => {
                let msg = format!("{method}: {} ({})", error.message, error.code);
                if method == "session/open" {
                    if let Some(&sid) = self.session_keys.keys().next() {
                        return vec![AgentEvent::SessionError {
                            session_id: sid,
                            error: msg,
                        }];
                    }
                }
                // W08 best-effort: an RPC error with no pending-request context
                // can't be attributed to a specific session/prompt, so it lands
                // on an arbitrary in-flight prompt. Precise routing would need
                // the transport to carry session/turn on `RpcError` (follow-up).
                if let Some(&pid) = self.prompt_ids.values().next() {
                    return vec![AgentEvent::PromptError {
                        prompt_id: pid,
                        error: msg,
                    }];
                }
                log::warn!("octos-ui-agent: rpc error with no handler: {msg}");
                Vec::new()
            }
            TransportEvent::DurableNotification { payload, cursor } => {
                self.fold_into_store(payload.clone(), cursor);
                self.translate_notification(payload)
            }
            TransportEvent::EphemeralNotification { payload } => {
                // Ephemerals (`message/delta`) carry no cursor — pass `None`
                // so `state::reduce` skips the cursor advance per
                // `octos-app-store/src/state.rs:148-152`.
                self.fold_into_store(payload.clone(), None);
                self.translate_notification(payload)
            }
            TransportEvent::SessionsListed { sessions } => {
                // Same downstream path as the old REST hydrate: project and
                // post `SessionListAction::Hydrated` for `handle_actions`.
                let fallback =
                    octos_app_store::auth::ProfileId::from(self.fallback_profile.clone());
                crate::app::sessions::hydrate_from_ws_value(sessions, &fallback);
                Vec::new()
            }
            TransportEvent::SessionHydrated { session_id, result } => {
                let key = SessionKey(session_id);
                // A turn held at its terminal asked for this reply: the
                // durable assistant row it carries is the text the turn
                // finalizes on. Consumed here — it must not replay the whole
                // session over a chat that is mid-turn — and consumed even
                // when the live row already resolved the hold.
                if let Some(outstanding) = self.hold_hydrates.get_mut(&key) {
                    *outstanding -= 1;
                    if *outstanding == 0 {
                        self.hold_hydrates.remove(&key);
                    }
                    let held = self
                        .pending_completion
                        .iter()
                        .find(|(_, p)| p.session_id == key)
                        .map(|(turn, _)| turn.clone());
                    return match held {
                        Some(turn) => self.complete_from_hydrate(&turn, result),
                        None => Vec::new(),
                    };
                }
                // Resume flow: decode the chat rows and hand them to the App
                // via a posted action (same pattern as `SessionListAction`).
                let Some(&sid) = self.session_ids.get(&key) else {
                    log::warn!(
                        "octos-ui-agent: session/hydrate reply for unmapped {}",
                        key.0
                    );
                    return Vec::new();
                };
                match serde_json::from_value::<
                    octos_core::ui_protocol::SessionHydrateResult,
                >(result)
                {
                    Ok(r) => {
                        let mut messages: Vec<(String, String)> = Vec::new();
                        if let Some(rows) = r.messages {
                            for row in rows {
                                messages.push((row.role, row.content));
                            }
                        }
                        log::info!(
                            "octos-ui-agent: hydrated {} rows for {}",
                            messages.len(),
                            key.0
                        );
                        Cx::post_action(SessionResumeHydrated {
                            session_id: sid,
                            messages,
                        });
                    }
                    Err(e) => {
                        log::warn!("octos-ui-agent: decode session/hydrate: {e}")
                    }
                }
                Vec::new()
            }
        }
    }

    /// Fold a `tool/*` / `task/*` / `turn/*` notification into the global
    /// `APP_STATE`. Replaces the W04-todo "buffer for now" behaviour the
    /// previous translate path used. The store's `apply_protocol`
    /// (`octos-app-store/src/state.rs:148`) already handles every
    /// `UiNotification` variant; we just hand off here.
    ///
    /// Read-write lock contention is bounded — the lock is held only for
    /// the duration of `reduce`, which is a small in-memory mutation. If a
    /// reader (the `TaskDock` widget, the `SessionList` widget) is mid-draw,
    /// we wait for it. This mirrors the `APP_STATE.write()` pattern used by
    /// `App::handle_actions` at `app/src/main.rs:2587-2599` when applying
    /// optimistic session deletes.
    fn fold_into_store(&self, n: UiNotification, cursor: Option<UiCursor>) {
        let event = StoreEvent::Protocol { cursor, notification: n };
        match APP_STATE.write() {
            Ok(mut state) => store_reduce(&mut state, event),
            Err(e) => log::warn!("octos-ui-agent: APP_STATE poisoned: {e}"),
        }
    }

    /// W04 follow-up #3: mirror connection-state transitions into the store
    /// so the top bar can render a coloured dot (Live = green, Reconnecting
    /// = amber, Failed/Idle = red) and push a transient toast on edges.
    /// Toasts are deduped by transition, not state, so a flap that lands on
    /// the same state still emits one toast (e.g. Live → Reconnecting →
    /// Live shows both reconnecting + reconnect-success). `prev == next`
    /// is a no-op: the transport may resend the current state on internal
    /// re-entrancy.
    fn fold_connection_into_store(&self, prev: &ConnectionState, next: &ConnectionState) {
        if prev == next {
            return;
        }
        let store_event = match next {
            ConnectionState::Live | ConnectionState::ReplayApplying => {
                Some(ConnectionEvent::Connected)
            }
            ConnectionState::Reconnecting { .. } => Some(ConnectionEvent::Reconnecting),
            ConnectionState::Idle
            | ConnectionState::Dialing
            | ConnectionState::Handshaking
            | ConnectionState::Failed => Some(ConnectionEvent::Offline),
        };
        let toast = match (prev, next) {
            // Reconnect arc: prev was a degraded state, we're back to Live.
            (
                ConnectionState::Reconnecting { .. } | ConnectionState::ReplayApplying,
                ConnectionState::Live,
            ) => Some(Toast::new(ToastKind::ReconnectSuccess, "Reconnected")),
            // Falling into Reconnecting from anywhere — show backoff toast.
            (_, ConnectionState::Reconnecting { attempt }) => Some(Toast::new(
                ToastKind::Reconnecting,
                format!("Reconnecting (attempt {attempt})"),
            )),
            // Cumulative budget exhausted — terminal failure toast.
            (_, ConnectionState::Failed) => Some(Toast::new(
                ToastKind::Error,
                "Connection failed; restart to retry",
            )),
            // First Live (Dialing/Handshaking → Live) — confirm online.
            (ConnectionState::Dialing | ConnectionState::Handshaking, ConnectionState::Live) => {
                Some(Toast::new(ToastKind::ReconnectSuccess, "Connected"))
            }
            _ => None,
        };
        match APP_STATE.write() {
            Ok(mut state) => {
                if let Some(ev) = store_event {
                    store_reduce(&mut state, StoreEvent::Connection(ev));
                }
                if let Some(t) = toast {
                    store_reduce(&mut state, StoreEvent::Toast(t));
                }
            }
            Err(e) => log::warn!("octos-ui-agent: APP_STATE poisoned: {e}"),
        }
    }

    fn translate_notification(&mut self, n: UiNotification) -> Vec<AgentEvent> {
        // Current native OUP connections project terminal lifecycle events into
        // v2. Finalizing only legacy turn/completed left the spinner running
        // after a valid card had already arrived. Retire the turn once, using
        // its explicit ID; a later duplicate legacy terminal then does nothing.
        if let UiNotification::EnvelopeV2(ev) = &n {
            if let PayloadV2::TurnTerminal { outcome, error, token_usage } = &ev.envelope.payload {
                let turn = self.prompt_ids.keys()
                    .find(|turn| turn.0.to_string() == ev.envelope.turn_id).cloned();
                if let Some(turn) = turn {
                    let success = matches!(outcome, octos_core::ui_protocol::TurnTerminalOutcome::Completed);
                    // The legacy `turn/completed` may already have reached
                    // this turn and be holding it for its durable text; its
                    // metrics were recorded then. Record them once.
                    if !self.pending_completion.contains_key(&turn) {
                        let elapsed = self.generation_started.remove(&turn)
                            .map(|(started, _)| started.elapsed().as_millis());
                        self.app_prompt_cache.complete(&turn, success,
                            token_usage.as_ref().map(|usage| usage.output_tokens as usize));
                        log::info!("generation-metric {}", serde_json::json!({
                            "event": if success { "completed" } else { "failed" },
                            "turn_id": turn, "elapsed_ms": elapsed,
                            "outcome": outcome, "token_usage": token_usage
                        }));
                    }
                    return if success {
                        self.finish_turn(&turn, "v2 terminal")
                    } else {
                        let pid = self.retire_turn(&turn).expect("turn was tracked above");
                        vec![AgentEvent::PromptError { prompt_id: pid,
                            error: error.as_ref().map(|e| e.message.clone())
                                .unwrap_or_else(|| format!("Generation ended: {outcome:?}")) }]
                    };
                }
            }
        }
        match &n {
            UiNotification::ContextNormalizationReported(ev) => {
                self.app_prompt_cache.context(&ev.session_id,
                    ev.context_state.token_estimate.max(ev.normalization.token_estimate),
                    ev.normalization.dropped_count == 0 && ev.normalization.truncated_count == 0);
                log::info!("generation-metric {}", serde_json::json!({
                    "event": "context_normalized", "session_id": ev.session_id,
                    "context_tokens": ev.context_state.token_estimate,
                    "prompt_tokens_estimate": ev.normalization.token_estimate,
                    "dropped": ev.normalization.dropped_count,
                    "truncated": ev.normalization.truncated_count
                }));
            }
            UiNotification::ContextCompactionStarted(ev) => {
                self.app_prompt_cache.compacting(&ev.session_id, ev.threshold_tokens);
            }
            UiNotification::ContextCompactionCompleted(ev) => {
                self.app_prompt_cache.invalidate(&ev.session_id);
            }
            // Tool loops can add context beyond a single card response.
            UiNotification::ToolStarted(ev) => {
                self.app_prompt_cache.forget(&ev.session_id);
            }
            UiNotification::ProgressUpdated(ev) => {
                if let Some(usage) = &ev.metadata.token_cost {
                    if let Some(window) = usage.context_window {
                        self.app_prompt_cache.window(&ev.session_id, window as usize);
                    }
                    log::info!("generation-metric {}", serde_json::json!({
                        "event": "provider_usage", "session_id": ev.session_id,
                        "turn_id": ev.turn_id, "token_usage": usage
                    }));
                }
            }
            UiNotification::MessageDelta(ev) if !ev.text.is_empty() => {
                if let Some((started, first_seen)) = self.generation_started.get_mut(&ev.turn_id) {
                    if !*first_seen {
                        *first_seen = true;
                        log::info!("generation-metric {}", serde_json::json!({
                            "event": "first_text", "turn_id": ev.turn_id,
                            "elapsed_ms": started.elapsed().as_millis()
                        }));
                    }
                }
            }
            // A turn the v2 terminal already reached (and is holding for its
            // durable text) has had its metrics recorded; do not record twice.
            UiNotification::TurnCompleted(ev) if !self.pending_completion.contains_key(&ev.turn_id) => {
                self.app_prompt_cache.complete(&ev.turn_id, true, ev.tokens_out.map(|tokens| tokens as usize));
                if let Some((started, _)) = self.generation_started.remove(&ev.turn_id) {
                    log::info!("generation-metric {}", serde_json::json!({
                        "event": "completed", "turn_id": ev.turn_id,
                        "elapsed_ms": started.elapsed().as_millis(),
                        "tokens_in": ev.tokens_in, "tokens_out": ev.tokens_out
                    }));
                }
            }
            UiNotification::TurnError(ev) => {
                self.app_prompt_cache.complete(&ev.turn_id, false, None);
                self.generation_started.remove(&ev.turn_id);
                log::info!("generation-metric {}", serde_json::json!({
                    "event": "failed", "turn_id": ev.turn_id, "code": ev.code
                }));
            }
            UiNotification::EnvelopeV2(ev) => {
                if let PayloadV2::TurnTerminal { token_usage: Some(token_usage), .. } = &ev.envelope.payload {
                    log::info!("generation-metric {}", serde_json::json!({
                        "event": "usage", "session_id": ev.session_id,
                        "turn_id": ev.envelope.turn_id,
                        "token_usage": token_usage
                    }));
                }
            }
            _ => {}
        }
        match n {
            UiNotification::MessageDelta(ev) => self
                .prompt_ids
                .get(&ev.turn_id)
                .copied()
                .map(|pid| {
                    vec![AgentEvent::TextDelta {
                        prompt_id: pid,
                        text: ev.text,
                    }]
                })
                .unwrap_or_default(),
            // 2026-07 protocol catch-up: server-side reasoning stream maps
            // onto the chat surface's thinking strip.
            UiNotification::ReasoningDelta(ev) => self
                .prompt_ids
                .get(&ev.turn_id)
                .copied()
                .map(|pid| {
                    vec![AgentEvent::ThinkingDelta {
                        prompt_id: pid,
                        text: ev.text,
                    }]
                })
                .unwrap_or_default(),
            UiNotification::ToolStarted(ev) => self
                .prompt_ids
                .get(&ev.turn_id)
                .copied()
                .map(|pid| {
                    let input = ev
                        .arguments
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "{}".to_owned());
                    vec![AgentEvent::ToolRequest {
                        prompt_id: pid,
                        tool_use_id: ev.tool_call_id,
                        tool_name: ev.tool_name,
                        tool_input: input,
                    }]
                })
                .unwrap_or_default(),
            // The durably-stored row. Its text is what the kernel actually
            // saved, so it is authoritative over our own `MessageDelta`
            // accumulation — a delta lost in transit leaves that accumulation
            // short by one chunk, spliced together mid-token, and an app-card
            // DSL mangled that way fails to parse for reasons the model never
            // wrote. Bridged so the consumer can prefer it at turn end.
            // octos #1746 removed the `message/persisted` notification; the same
            // durable row now arrives as a canonical projection envelope, and v2
            // delivery is UNCONDITIONAL (no capability to request). Only
            // `AssistantPersisted` is bridged — the deltas, tool lifecycle and
            // turn terminal already have their own notifications above, so
            // bridging them here would double every one.
            UiNotification::EnvelopeV2(ev) => match ev.envelope.payload {
                PayloadV2::AssistantPersisted { text, .. } if !text.trim().is_empty() => {
                    // Route by the envelope's turn when it names one we track
                    // (the wire carries it as a bare String against our
                    // `TurnId(Uuid)` keys, so compare the rendering), else by
                    // SESSION: `prompt_sessions` holds exactly the prompts
                    // still in flight, so the session's entry is the turn this
                    // row belongs to — the routing the old lane used.
                    let turn = self
                        .prompt_ids
                        .keys()
                        .find(|turn| turn.0.to_string() == ev.envelope.turn_id)
                        .cloned()
                        .or_else(|| {
                            self.prompt_sessions
                                .iter()
                                .find(|(_, sess)| **sess == ev.session_id)
                                .and_then(|(pid, _)| self.turn_ids.get(pid).cloned())
                        });
                    turn.map(|turn| self.authoritative_arrived(&turn, text))
                        .unwrap_or_default()
                }
                _ => Vec::new(),
            },
            // The legacy terminal is DIRECT-sent and can overtake the ledger
            // lane carrying the durable row (the kernel says so itself, in
            // `send_notification_lifecycle`); `finish_turn` holds it until
            // that row — or its `session/hydrate` stand-in — has landed.
            UiNotification::TurnCompleted(ev) => self.finish_turn(&ev.turn_id, "legacy turn/completed"),
            UiNotification::TurnError(ev) => self
                .retire_turn(&ev.turn_id)
                .map(|pid| {
                    vec![AgentEvent::PromptError {
                        prompt_id: pid,
                        error: format!("{}: {}", ev.code, ev.message),
                    }]
                })
                .unwrap_or_default(),
            // Drained into APP_STATE by `fold_into_store` above. The
            // TaskDock widget (`app/src/app/task_dock.rs`) reads them back
            // via `APP_STATE.tool_calls` / `APP_STATE.tasks` on each redraw,
            // so we don't bridge them through `AgentEvent` — there's no
            // round-trip required. ApprovalRequested lands on the
            // `ApprovalsSlice` (W05 surface). Warning toasts already land in
            // `state.toasts` via the store reducer.
            // Drained into APP_STATE via `fold_into_store`. Listed
            // explicitly so a new `UiNotification` variant tickles a compile
            // error here and forces a deliberate decision (forward-compat
            // per spec § 4.1).
            UiNotification::TurnStarted(_)
            | UiNotification::ToolProgress(_)
            | UiNotification::ToolCompleted(_)
            | UiNotification::TaskUpdated(_)
            | UiNotification::TaskOutputDelta(_)
            | UiNotification::ApprovalRequested(_)
            | UiNotification::ApprovalAutoResolved(_)
            | UiNotification::ApprovalDecided(_)
            | UiNotification::ApprovalCancelled(_)
            | UiNotification::ProgressUpdated(_)
            | UiNotification::ReplayLossy(_)
            | UiNotification::SessionOpened(_)
            | UiNotification::Warning(_)
            // 2026-07 protocol catch-up — folded into APP_STATE (or
            // deliberately unsurfaced) by the store reducer; nothing to
            // bridge through AgentEvent. Kept explicit per the note above.
            | UiNotification::UserQuestionRequested(_)
            | UiNotification::VisualGenerating(_)
            | UiNotification::VisualSucceeded(_)
            | UiNotification::VisualFailed(_)
            | UiNotification::VoiceExit(_)
            | UiNotification::TurnSpawnComplete(_)
            | UiNotification::FileAttached(_)
            | UiNotification::SessionEventBridged(_)
            | UiNotification::RouterStatus(_)
            | UiNotification::RouterFailover(_)
            | UiNotification::QueueState(_)
            | UiNotification::AgentUpdated(_)
            | UiNotification::AgentOutputDelta(_)
            | UiNotification::AgentArtifactUpdated(_)
            | UiNotification::SessionGoalUpdated(_)
            | UiNotification::SessionGoalCleared(_)
            | UiNotification::LoopUpdated(_)
            | UiNotification::LoopFired(_)
            | UiNotification::LoopCompleted(_)
            // MonitorRuntime (octos main, 2026-08) — kernel-side watchers;
            // no chat-surface projection here.
            | UiNotification::MonitorUpdated(_)
            | UiNotification::MonitorFired(_)
            | UiNotification::MonitorExpired(_)
            | UiNotification::ContextCompactionCompleted(_)
            | UiNotification::ContextCompactionStarted(_)
            | UiNotification::TurnSteerDropped(_)
            | UiNotification::BackgroundActivity(_)
            | UiNotification::ContextNormalizationReported(_)
            | UiNotification::SessionOrchestration(_)
            // 2026-07 protocol catch-up: no plan pane / voice surface here.
            | UiNotification::PlanUpdated(_)
            | UiNotification::VoiceAudioChunk(_)
            | UiNotification::Envelope(_)
            // 2026-08 catch-up: background skill jobs and peer staging have no
            // surface in this app. (EnvelopeV2 is bridged above, not ignored.)
            | UiNotification::SkillActionJobUpdated(_)
            | UiNotification::PeerStaged(_)
            | UiNotification::PeerClosed(_) => Vec::new(),
        }
    }
}

impl Agent for OctosUiAgent {
    fn create_session(&mut self, _cx: &mut Cx, config: SessionConfig) -> SessionId {
        let session_id = SessionId::new();
        let key = self.make_session_key(session_id);
        log::info!("octos-ui-agent: create_session → session/open {}", key.0);
        self.session_keys.insert(session_id, key.clone());
        self.session_ids.insert(key.clone(), session_id);
        let profile_id = self.open_profile_id();
        self.post(OutboundCommand::OpenSession(SessionOpenParams {
            session_id: key,
            // 2026-07 protocol catch-up: server-side session topic label and
            // per-session sandbox override — both server-defaulted when None.
            topic: None,
            profile_id,
            // Per-session cwd override first (the AMA composer session hints
            // its workspace INTO the app-cards memory tree so its file tools
            // can author new app specs — `session.workspace_cwd.v1` is
            // default-enabled on the stdio transport), else the agent-wide
            // workspace.
            cwd: config.cwd.or_else(|| self.workspace_cwd.clone()),
            sandbox: None,
            after: None,
        }));
        session_id
    }

    /// Re-attach to an existing server session (sidebar resume): map a fresh
    /// local `SessionId` to the given key, re-open it (octos sessions are
    /// stateful — `session/open` on an existing key attaches), then request
    /// its chat history via `session/hydrate`. The history lands as a posted
    /// `SessionResumeHydrated` action.
    fn resume_session(&mut self, _cx: &mut Cx, backend_key: &str) -> Option<SessionId> {
        let key = SessionKey(backend_key.to_owned());
        // Invalidate at the start of a resume, before any new prompt can be
        // queued. Its later open receipt must not erase that new submission.
        self.app_prompt_cache.forget(&key);
        // Re-use the existing mapping if the user re-taps the same session.
        let session_id = if let Some(&sid) = self.session_ids.get(&key) {
            sid
        } else {
            let sid = SessionId::new();
            self.session_keys.insert(sid, key.clone());
            self.session_ids.insert(key.clone(), sid);
            sid
        };
        log::info!("octos-ui-agent: resume_session → {}", key.0);
        let profile_id = self.open_profile_id();
        // Fresh open (no cursor bracket): the connection cursor belongs to
        // the session we're switching AWAY from.
        self.post(OutboundCommand::OpenSessionFresh(SessionOpenParams {
            session_id: key.clone(),
            topic: None,
            profile_id,
            cwd: self.workspace_cwd.clone(),
            sandbox: None,
            after: None,
        }));
        self.post(OutboundCommand::HydrateSession { session_id: key.0 });
        Some(session_id)
    }

    /// The octos `SessionKey` string mapped to `session_id`. Layer 3: the
    /// multi-app switcher created these sessions via `create_session` (which
    /// returns only the local `SessionId`); this hands back the backend key so
    /// the switcher can `resume_session` → hydrate on foreground switch.
    fn backend_key(&self, session_id: SessionId) -> Option<String> {
        self.session_keys.get(&session_id).map(|k| k.0.clone())
    }

    fn send_prompt(&mut self, _cx: &mut Cx, session_id: SessionId, text: &str) -> PromptId {
        let prompt_id = PromptId::new();
        let turn_id = TurnId::new();
        self.turn_ids.insert(prompt_id, turn_id.clone());
        self.prompt_ids.insert(turn_id.clone(), prompt_id);
        let Some(key) = self.session_keys.get(&session_id).cloned() else {
            log::warn!("octos-ui-agent: send_prompt for unknown session");
            return prompt_id;
        };
        // W08: remember which session owns this prompt, so cancel/routing can
        // target it without guessing.
        self.prompt_sessions.insert(prompt_id, key.clone());
        // Reference reuse depends on local backend lifecycle telemetry and the
        // default compaction policy. Remote/overridden policies keep full input.
        let cache_enabled = (self.stdio_transport || cfg!(target_env = "ohos"))
            && self.capabilities.as_ref().is_some_and(|caps| caps.context_lifecycle)
            && std::env::var_os("OCTOS_CONTEXT_COMPACT_THRESHOLD_TOKENS").is_none()
            && std::env::var_os("OCTOS_A2APP_DISABLE_REFERENCE_CACHE").is_none();
        let (sent_text, reference_hit) = self.app_prompt_cache.prepare(&key, &turn_id, text, cache_enabled);
        self.generation_started.insert(turn_id.clone(), (std::time::Instant::now(), false));
        log::info!("generation-metric {}", serde_json::json!({
            "event": "submitted", "turn_id": turn_id, "session_id": key,
            "prompt_bytes": sent_text.len(), "original_prompt_bytes": text.len(),
            "reference_cache_hit": reference_hit,
            "transport": if cfg!(target_env = "ohos") { "embedded" } else if self.stdio_transport { "stdio" } else { "websocket" }
        }));
        self.post(OutboundCommand::StartTurn(TurnStartParams {
            session_id: key,
            turn_id,
            input: vec![InputItem::Text {
                text: sent_text,
            }],
            // 2026-07 protocol catch-up: attachments, topic routing, prompt
            // rewrite, per-turn reasoning effort, and live-video capture are
            // all opt-in; text-only turns send the neutral defaults.
            media: Vec::new(),
            topic: None,
            rewrite_for: None,
            // Driven by the composer's "Thinking" toggle (`set_thinking`).
            // `High` when on; `None` defers to the gateway/profile default.
            reasoning_effort: self.thinking.then_some(ReasoningEffortLevel::High),
            // Context-scoped tools stay unadvertised: an app agent's whole job
            // is to emit a card, and every extra tool it could reach for is a
            // way to answer with something other than one.
            tool_context: None,
            live_video: false,
        }));
        prompt_id
    }

    fn set_thinking(&mut self, on: bool) {
        self.thinking = on;
    }

    fn send_tool_result(
        &mut self,
        _cx: &mut Cx,
        _session_id: SessionId,
        _tool_use_id: &str,
        _result: &str,
        _is_error: bool,
    ) {
        log::warn!("octos-ui-agent: ignoring tool result; AppUI has no contract command for it");
    }

    fn cancel_prompt(&mut self, _cx: &mut Cx, prompt_id: PromptId) {
        let Some(turn_id) = self.turn_ids.get(&prompt_id).cloned() else {
            return;
        };
        // W08: cancel the turn on the SESSION that owns this prompt (recorded
        // in `send_prompt`) — not a guessed "first" session.
        let Some(key) = self.prompt_sessions.get(&prompt_id).cloned() else {
            return;
        };
        self.app_prompt_cache.complete(&turn_id, false, None);
        self.post(OutboundCommand::InterruptTurn(TurnInterruptParams {
            session_id: key,
            turn_id,
        }));
    }

    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event) -> Vec<AgentEvent> {
        let mut out = Vec::new();
        loop {
            match self.evt_rx.try_recv() {
                Ok(evt) => out.extend(self.translate(evt)),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    log::warn!("octos-ui-agent: transport channel disconnected");
                    break;
                }
            }
        }
        // A held terminal whose durable text never came completes on what
        // streamed. Swept here because this runs on every UI event — the
        // stream tick keeps them coming while a turn is open.
        out.extend(self.expire_pending_completions());
        out
    }

    fn is_session_ready(&self, session_id: SessionId) -> bool {
        self.ready_sessions.contains(&session_id)
    }

    fn is_stateless(&self) -> bool {
        // Octos sessions are stateful server-side.
        false
    }
}

/// Handle exposed by `OctosUiAgent::task_output_handle` for one-shot
/// `task/output/read` RPCs from the coding task drill-down.
#[derive(Clone)]
pub struct TaskOutputHandle {
    cmd_tx: Sender<OutboundCommand>,
    runtime: tokio::runtime::Handle,
}

impl TaskOutputHandle {
    pub fn read(&self, params: TaskOutputReadParams) {
        use tokio::sync::oneshot;

        let task_id = params.task_id.clone();
        let session_id = params.session_id.clone();
        let (tx, rx) = oneshot::channel();
        let cmd = OutboundCommand::RequestTaskOutput { params, reply: tx };
        match self.cmd_tx.try_send(cmd) {
            Ok(()) => {}
            Err(tokio::sync::mpsc::error::TrySendError::Full(_))
            | Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => {
                Cx::post_action(crate::app::coding::TaskOutputAction {
                    task_id,
                    session_id,
                    outcome: crate::app::coding::TaskOutputOutcome::Failed(
                        "transport unavailable".to_owned(),
                    ),
                });
                return;
            }
        }
        self.runtime.spawn(async move {
            let outcome = match rx.await {
                Ok(Ok(result)) => crate::app::coding::TaskOutputOutcome::Loaded(result),
                Ok(Err(err)) => crate::app::coding::TaskOutputOutcome::Failed(format!(
                    "{} ({})",
                    err.message, err.code
                )),
                Err(_) => crate::app::coding::TaskOutputOutcome::Failed(
                    "transport dropped the reply channel".to_owned(),
                ),
            };
            Cx::post_action(crate::app::coding::TaskOutputAction {
                task_id,
                session_id,
                outcome,
            });
        });
    }
}

/// W05 — handle exposed by `OctosUiAgent::approval_handle`. Carries a
/// cheap clone of the transport sender plus a runtime handle so the
/// approvals widget can post `approval/respond` and forward the wire
/// reply back to the UI thread without holding the agent itself. Cloning
/// is `Arc`-shaped under the hood (mpsc + tokio runtime handles).
#[derive(Clone)]
pub struct ApprovalHandle {
    cmd_tx: Sender<OutboundCommand>,
    runtime: tokio::runtime::Handle,
}

impl ApprovalHandle {
    /// Issue `approval/respond` and forward the wire reply to the UI
    /// thread as an `ApprovalAsyncAction`. Idempotent — the server
    /// enforces single-decision semantics; we just surface the outcome.
    /// See `workstreams/W05-approvals-diff.md` § "Approval response flow".
    pub fn respond(
        &self,
        session_id: SessionKey,
        approval_id: ApprovalId,
        decision: ApprovalDecision,
        scope: Option<String>,
    ) {
        use tokio::sync::oneshot;
        // `ApprovalDecision` is no longer `Copy` (FIX-01); clone it once for
        // the wire params and keep the original for the failure branch +
        // async reply.
        let mut params =
            ApprovalRespondParams::new(session_id, approval_id.clone(), decision.clone());
        params.approval_scope = scope;
        let (tx, rx) = oneshot::channel();
        let cmd = OutboundCommand::SendApprovalResponse { params, reply: tx };
        match self.cmd_tx.try_send(cmd) {
            Ok(()) => {}
            Err(tokio::sync::mpsc::error::TrySendError::Full(_))
            | Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => {
                Cx::post_action(crate::app::approvals::ApprovalAsyncAction {
                    approval_id,
                    decision,
                    outcome: crate::app::approvals::ApprovalAsyncOutcome::Failed {
                        message: "transport unavailable".to_owned(),
                        code: 0,
                        data: None,
                    },
                });
                return;
            }
        }
        let approval_id_for_task = approval_id.clone();
        self.runtime.spawn(async move {
            let outcome = match rx.await {
                Ok(Ok(res)) => crate::app::approvals::ApprovalAsyncOutcome::Accepted {
                    runtime_resumed: res.runtime_resumed,
                },
                // Forward the structured RpcError so the UI can detect
                // `-32011 APPROVAL_NOT_PENDING` and recover the decision
                // from `data.recorded_decision`. See
                // `octos-cli/src/api/ui_protocol_approvals.rs:198-215`.
                Ok(Err(err)) => crate::app::approvals::ApprovalAsyncOutcome::Failed {
                    message: err.message.clone(),
                    code: err.code,
                    data: err.data.clone(),
                },
                Err(_) => crate::app::approvals::ApprovalAsyncOutcome::Failed {
                    message: "transport dropped the reply channel".to_owned(),
                    code: 0,
                    data: None,
                },
            };
            Cx::post_action(crate::app::approvals::ApprovalAsyncAction {
                approval_id: approval_id_for_task,
                decision,
                outcome,
            });
        });
    }
}

#[cfg(test)]
mod generation_terminal_tests {
    use super::*;
    use octos_core::ui_protocol::{EnvelopeV2, EnvelopeV2Notification, TurnTerminalOutcome, TurnCompletedEvent};

    fn agent() -> OctosUiAgent {
        let (cmd_tx, _) = tokio::sync::mpsc::channel(1);
        let (_, evt_rx) = tokio::sync::mpsc::channel(1);
        OctosUiAgent {
            _runtime: tokio::runtime::Builder::new_current_thread().build().unwrap(),
            cmd_tx, evt_rx, session_keys: HashMap::new(), session_ids: HashMap::new(),
            ready_sessions: Default::default(), turn_ids: HashMap::new(),
            prompt_ids: HashMap::new(), prompt_sessions: HashMap::new(),
            generation_started: HashMap::new(), connection_state: ConnectionState::Idle,
            app_prompt_cache: Default::default(),
            capabilities: None, workspace_cwd: None, fallback_profile: "_main".into(),
            thinking: false, stdio_transport: true,
            persisted_seen: HashSet::new(), pending_completion: HashMap::new(),
            hold_hydrates: HashMap::new(),
        }
    }

    fn terminal(turn: &TurnId, outcome: TurnTerminalOutcome) -> UiNotification {
        UiNotification::EnvelopeV2(EnvelopeV2Notification {
            session_id: SessionKey("_main:test".into()), topic: None,
            envelope: EnvelopeV2 {
                thread_id: "shared-thread".into(), seq: 1, cursor: None,
                turn_id: turn.0.to_string(), client_message_id: None,
                payload: PayloadV2::TurnTerminal { outcome, error: None, token_usage: None },
            },
        })
    }

    /// The durable assistant row, as the ledger forwarder delivers it.
    fn persisted(turn: &TurnId, text: &str) -> UiNotification {
        UiNotification::EnvelopeV2(EnvelopeV2Notification {
            session_id: SessionKey("_main:test".into()), topic: None,
            envelope: EnvelopeV2 {
                thread_id: "shared-thread".into(), seq: 2, cursor: None,
                turn_id: turn.0.to_string(), client_message_id: None,
                payload: PayloadV2::AssistantPersisted {
                    text: text.into(),
                    assistant_segment_id: format!("{}:assistant:iteration:1", turn.0),
                    meta: octos_core::ui_protocol::MessageMeta {
                        message_id: "row-1".into(),
                        persisted_at: chrono::Utc::now(),
                        media: Vec::new(),
                    },
                },
            },
        })
    }

    /// The direct-sent legacy terminal — the one that overtakes the ledger lane.
    fn legacy_completed(turn: &TurnId) -> UiNotification {
        UiNotification::TurnCompleted(TurnCompletedEvent {
            session_id: SessionKey("_main:test".into()), topic: None, turn_id: turn.clone(),
            cursor: None, tokens_in: None, tokens_out: None, session_result: None,
        })
    }

    /// A `session/hydrate` reply carrying the turn's assistant row.
    fn hydrated(turn: &TurnId, text: &str) -> TransportEvent {
        TransportEvent::SessionHydrated {
            session_id: "_main:test".into(),
            result: serde_json::json!({
                "session_id": "_main:test",
                "cursor": {"stream": "_main:test", "seq": 3},
                "messages": [
                    {"seq": 1, "role": "user", "content": "tokyo weather",
                     "turn_id": turn, "persisted_at": "2026-09-14T09:28:19Z"},
                    {"seq": 2, "role": "assistant", "content": text,
                     "turn_id": turn, "persisted_at": "2026-09-14T09:28:36Z"}
                ]
            }),
        }
    }

    const CARD: &str = "```runl0\n# level: L0\n# model: weather\nview root Col {}\n```";

    /// The failure as measured: 897 of 1840 deltas, then the legacy
    /// terminal, then (late) the persisted row. The terminal alone must not
    /// finalize; the row does, as authoritative text.
    #[test]
    fn legacy_terminal_waits_for_the_persisted_row() {
        let mut agent = agent();
        let (turn, prompt) = track(&mut agent);
        assert!(agent.translate_notification(legacy_completed(&turn)).is_empty(),
            "a terminal without the durable row must hold the turn");
        assert!(agent.prompt_ids.contains_key(&turn), "a held turn keeps routing late frames");
        assert!(agent.pending_completion.contains_key(&turn));
        let events = agent.translate_notification(persisted(&turn, CARD));
        assert!(matches!(events.as_slice(),
            [AgentEvent::TextAuthoritative { prompt_id: p1, text }, AgentEvent::TurnComplete { prompt_id: p2, .. }]
            if *p1 == prompt && *p2 == prompt && text == CARD), "{events:?}");
        assert!(agent.prompt_ids.is_empty() && agent.pending_completion.is_empty()
            && agent.persisted_seen.is_empty());
        assert!(agent.translate_notification(terminal(&turn, TurnTerminalOutcome::Completed)).is_empty(),
            "the ledger lane's own terminal, arriving after, does nothing");
    }

    /// When even the live row is lost (the ledger forwarder lagged), the
    /// `session/hydrate` reply the hold requested completes the turn.
    #[test]
    fn held_turn_completes_on_the_hydrated_row() {
        let mut agent = agent();
        let (turn, prompt) = track(&mut agent);
        assert!(agent.translate_notification(legacy_completed(&turn)).is_empty());
        let events = agent.translate(hydrated(&turn, CARD));
        assert!(matches!(events.as_slice(),
            [AgentEvent::TextAuthoritative { text, .. }, AgentEvent::TurnComplete { prompt_id, .. }]
            if *prompt_id == prompt && text == CARD), "{events:?}");
        assert!(agent.pending_completion.is_empty());
    }

    /// The hydrate reply for a held turn is consumed by it and never replays
    /// the session over the chat (no `SessionResumeHydrated`); a reply whose
    /// assistant text is empty still completes the turn.
    #[test]
    fn empty_hydrated_row_completes_on_the_streamed_text() {
        let mut agent = agent();
        let (turn, prompt) = track(&mut agent);
        assert!(agent.translate_notification(legacy_completed(&turn)).is_empty());
        let events = agent.translate(hydrated(&turn, "   "));
        assert!(matches!(events.as_slice(), [AgentEvent::TurnComplete { prompt_id, .. }] if *prompt_id == prompt),
            "{events:?}");
    }

    /// The hydrate a hold requested still belongs to the hold when the live
    /// row resolved it first: its reply is swallowed, never replayed over the
    /// chat as a session resume.
    #[test]
    fn a_late_hold_hydrate_reply_is_swallowed() {
        let mut agent = agent();
        let (turn, _) = track(&mut agent);
        assert!(agent.translate_notification(legacy_completed(&turn)).is_empty());
        assert_eq!(agent.hold_hydrates.get(&SessionKey("_main:test".into())), Some(&1));
        assert!(matches!(agent.translate_notification(persisted(&turn, CARD)).as_slice(),
            [AgentEvent::TextAuthoritative { .. }, AgentEvent::TurnComplete { .. }]));
        assert!(agent.translate(hydrated(&turn, CARD)).is_empty());
        assert!(agent.hold_hydrates.is_empty());
    }

    /// The ordered lane: row, then terminal — completes at the terminal.
    #[test]
    fn persisted_row_first_completes_at_the_terminal() {
        let mut agent = agent();
        let (turn, prompt) = track(&mut agent);
        let events = agent.translate_notification(persisted(&turn, CARD));
        assert!(matches!(events.as_slice(), [AgentEvent::TextAuthoritative { .. }]));
        let events = agent.translate_notification(legacy_completed(&turn));
        assert!(matches!(events.as_slice(), [AgentEvent::TurnComplete { prompt_id, .. }] if *prompt_id == prompt));
        assert!(agent.prompt_ids.is_empty());
    }

    /// A kernel that never persists a row must not hang the turn.
    #[test]
    fn held_turn_expires_onto_the_streamed_text() {
        let mut agent = agent();
        let (turn, prompt) = track(&mut agent);
        assert!(agent.translate_notification(legacy_completed(&turn)).is_empty());
        assert!(agent.expire_pending_completions().is_empty(), "not yet");
        agent.pending_completion.get_mut(&turn).unwrap().since =
            std::time::Instant::now() - AUTHORITATIVE_WAIT;
        let events = agent.expire_pending_completions();
        assert!(matches!(events.as_slice(), [AgentEvent::TurnComplete { prompt_id, .. }] if *prompt_id == prompt));
        assert!(agent.prompt_ids.is_empty() && agent.pending_completion.is_empty());
    }

    #[test]
    fn l0_migration_first_reference_survives_startup_but_not_reconnect_or_resume() {
        let mut agent = agent();
        let key = SessionKey("_main:test".into());
        let session = SessionId::new();
        agent.session_ids.insert(key.clone(), session);
        agent.session_keys.insert(session, key.clone());
        let prompt = crate::l0_prompt_all("Convert 20 degrees Celsius to Fahrenheit.");
        let first = TurnId::new();
        assert!(!agent.app_prompt_cache.prepare(&key, &first, &prompt, false).1);
        agent.translate(TransportEvent::ConnectionState(ConnectionState::Dialing));
        agent.translate(TransportEvent::ConnectionState(ConnectionState::Handshaking));
        let opened = serde_json::from_value(serde_json::json!({"opened":{"session_id":key}})).unwrap();
        agent.translate(TransportEvent::RpcResult(LifecycleResult::SessionOpen(opened)));
        agent.translate(TransportEvent::ConnectionState(ConnectionState::Live));
        agent.app_prompt_cache.context(&key, 1, true);
        agent.app_prompt_cache.window(&key, 262_144);
        agent.app_prompt_cache.complete(&first, true, Some(694));
        let next = crate::l0_prompt_all("Compare my saved cities.");
        let (sent, hit) = agent.app_prompt_cache.prepare(&key, &TurnId::new(), &next, true);
        assert!(hit, "the first successfully accepted reference must be reusable");
        assert!(sent.len() < 1_000);

        let mut cx = Cx::new(Box::new(|_, _| {}));
        agent.resume_session(&mut cx, &key.0);
        let resumed = TurnId::new();
        assert!(!agent.app_prompt_cache.prepare(&key, &resumed, &next, true).1);
        agent.app_prompt_cache.context(&key, 1, true);
        agent.app_prompt_cache.window(&key, 262_144);
        agent.app_prompt_cache.complete(&resumed, true, Some(400));
        assert!(agent.app_prompt_cache.prepare(&key, &TurnId::new(), &next, true).1);
        agent.translate(TransportEvent::ConnectionState(ConnectionState::Reconnecting { attempt: 1 }));
        assert!(!agent.app_prompt_cache.prepare(&key, &TurnId::new(), &next, true).1);
    }

    fn track(agent: &mut OctosUiAgent) -> (TurnId, PromptId) {
        let turn = TurnId::new();
        let prompt = PromptId::new();
        agent.prompt_ids.insert(turn.clone(), prompt);
        agent.turn_ids.insert(prompt, turn.clone());
        agent.prompt_sessions.insert(prompt, SessionKey("_main:test".into()));
        agent.generation_started.insert(turn.clone(), (std::time::Instant::now(), false));
        (turn, prompt)
    }

    #[test]
    fn v2_completes_exact_turn_once_and_ignores_background_terminal() {
        let mut agent = agent();
        let (turn, prompt) = track(&mut agent);
        assert!(agent.translate_notification(terminal(&TurnId::new(), TurnTerminalOutcome::Completed)).is_empty());
        assert!(agent.prompt_ids.contains_key(&turn));
        // The ledger lane is ordered: the durable row precedes its terminal.
        assert!(matches!(agent.translate_notification(persisted(&turn, CARD)).as_slice(),
            [AgentEvent::TextAuthoritative { .. }]));
        let events = agent.translate_notification(terminal(&turn, TurnTerminalOutcome::Completed));
        assert!(matches!(events.as_slice(), [AgentEvent::TurnComplete { prompt_id, .. }] if *prompt_id == prompt));
        assert!(agent.prompt_ids.is_empty() && agent.turn_ids.is_empty()
            && agent.prompt_sessions.is_empty() && agent.generation_started.is_empty());
        assert!(agent.translate_notification(terminal(&turn, TurnTerminalOutcome::Completed)).is_empty());
        assert!(agent.translate_notification(UiNotification::TurnCompleted(TurnCompletedEvent {
            session_id: SessionKey("_main:test".into()), topic: None, turn_id: turn,
            cursor: None, tokens_in: None, tokens_out: None, session_result: None,
        })).is_empty());
    }

    #[test]
    fn v2_failure_ends_loading_as_an_error() {
        let mut agent = agent();
        let (turn, prompt) = track(&mut agent);
        let events = agent.translate_notification(terminal(&turn, TurnTerminalOutcome::Errored));
        assert!(matches!(events.as_slice(), [AgentEvent::PromptError { prompt_id, .. }] if *prompt_id == prompt));
        assert!(agent.prompt_ids.is_empty() && agent.generation_started.is_empty());
    }
}
