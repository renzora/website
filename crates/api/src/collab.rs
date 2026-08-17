//! Collaborative editing relay.
//!
//! Two people running the Renzora editor want to work on one project together.
//! The editor can already do that over a direct TCP connection, which is fine on
//! a LAN and useless between two ordinary home connections — both sides are
//! behind NAT and neither can accept an inbound connection. This is the piece
//! that fixes it: both editors connect *outward* to renzora.com, and the server
//! forwards bytes between them.
//!
//! ## What the relay does and does not understand
//!
//! It does not parse the editor's protocol at all. The payloads are opaque
//! binary frames — scene snapshots, file chunks, camera positions — and this
//! module's entire job is deciding *which socket* each one goes to. That is
//! deliberate: the editor's protocol changes with the editor, and a relay that
//! understood it would have to be redeployed in lockstep with a desktop app that
//! users upgrade whenever they feel like it.
//!
//! ## The envelope
//!
//! A direct session gives the host one socket per guest, so "send to guest 3" is
//! just "write to socket 3". Through a relay the host has a single socket
//! carrying everyone's traffic, so each message needs to say who it belongs to.
//! Every binary message is therefore `[peer: u32 LE][payload…]`:
//!
//! | Direction | `peer` means |
//! |---|---|
//! | host → relay | send to this guest; [`BROADCAST`] means all of them |
//! | relay → host | this guest sent it |
//! | guest → relay | ignored — a guest can only talk to the host |
//! | relay → guest | always [`HOST_PEER`] |
//!
//! Relay *control* messages (a guest joined, the host vanished) go as WebSocket
//! **text** frames carrying JSON, so they can never be confused with payload.
//! That split is what keeps the relay from needing a message type of its own
//! inside the editor's protocol.
//!
//! ## Rooms live in memory, and that is on purpose
//!
//! A session exists only while its host is connected. Persisting rooms to
//! Postgres would mean rows that outlive the socket they describe, a cleanup job
//! to delete them, and a window where the database claims a session is live and
//! nothing is listening. The room *is* the connection, so it is stored next to
//! the connection. The cost is that a server restart ends every session — which
//! is what a restart does anyway, since it drops the sockets.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension, Path, Query, State,
    },
    response::Response,
    routing::{delete, get, post},
    Json, Router,
};

use rand::Rng;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{error::ApiError, jwt, middleware, middleware::AuthUser, notify, AppState};

/// The peer id the host answers to. Guests only ever talk to the host, so this
/// is the only value they will ever see in an inbound envelope.
pub const HOST_PEER: u32 = 0;

/// A host→relay target meaning "every guest in the room".
pub const BROADCAST: u32 = u32::MAX;

/// How long a room waits for its host to actually connect before being swept.
/// Creating a session and never opening the socket should not hold a code
/// forever.
const HOST_GRACE: Duration = Duration::from_secs(300);

/// Longest a single session may run. Generous — an editing session is measured
/// in hours — but not unbounded, so a forgotten editor left running overnight
/// eventually releases its slot.
const MAX_SESSION: Duration = Duration::from_secs(12 * 3600);

/// Largest relayed message. The editor caps its own frames well below this; the
/// limit is here so that one participant cannot make the server buffer an
/// arbitrary amount on its say-so.
const MAX_MESSAGE: usize = 32 * 1024 * 1024;

/// Queue depth per connection before the slow side is disconnected.
///
/// Bounded, and a full queue is fatal to that connection rather than something
/// to drop messages from. Dropping is not available to us: the relay cannot tell
/// a discardable camera update from a scene snapshot, and silently losing the
/// latter desynchronises two people's projects with no error anywhere. Cutting
/// a peer that cannot keep up at least fails loudly, on the side that is failing.
const QUEUE_DEPTH: usize = 512;

/// Concurrent sessions one account may host.
const MAX_ROOMS_PER_HOST: usize = 3;

// ── Rooms ───────────────────────────────────────────────────────────────────

type Tx = mpsc::Sender<Message>;

struct Participant {
    /// Shown to whoever is looking at the room. The account behind it is not
    /// kept: the host learns a guest's `user_id` from the `peer_joined` control
    /// message, and holding a second copy here would be a copy to keep correct
    /// for no reader.
    username: String,
    tx: Tx,
}

struct Room {
    code: String,
    host_user_id: Uuid,
    host_username: String,
    project: String,
    created_at: Instant,
    /// `None` until the host's socket arrives, and again if it drops.
    host: Option<Participant>,
    guests: HashMap<u32, Participant>,
    next_peer: u32,
}

impl Room {
    fn is_expired(&self) -> bool {
        let age = self.created_at.elapsed();
        if age > MAX_SESSION {
            return true;
        }
        // Created but never claimed by its host.
        self.host.is_none() && age > HOST_GRACE
    }
}

/// Every live session on this server.
#[derive(Clone, Default)]
pub struct CollabRooms(Arc<Mutex<HashMap<String, Room>>>);

impl CollabRooms {
    pub fn new() -> Self {
        Self::default()
    }

    /// Drop rooms nobody is using any more.
    ///
    /// Called from the request paths rather than a background task: rooms only
    /// become stale in ways that matter when someone is trying to create or join
    /// one, and a timer would be a second thing to reason about for no gain at
    /// this scale.
    fn sweep(&self) {
        if let Ok(mut rooms) = self.0.lock() {
            rooms.retain(|_, room| !room.is_expired());
        }
    }
}

/// A generated room code.
///
/// Alphabet excludes `0/O` and `1/I/L`, because these get read aloud and typed
/// by hand. 8 characters of it is ~41 bits — far past guessing, given that a
/// wrong code simply 404s and there is nothing to enumerate.
fn new_code() -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZ23456789";
    let mut rng = rand::thread_rng();
    (0..8).map(|_| ALPHABET[rng.gen_range(0..ALPHABET.len())] as char).collect()
}

// ── REST ────────────────────────────────────────────────────────────────────

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/sessions", post(create_session))
        .route("/sessions/:code", get(get_session))
        .route("/sessions/:code", delete(end_session))
        .route("/sessions/:code/invite", post(invite))
        .layer(axum::middleware::from_fn(middleware::require_auth))
}

/// The relay socket. Separate router because it authenticates from a query
/// parameter rather than a header — a browser/WebSocket client cannot set
/// headers on the handshake, the same reason `/ws/live` does it this way.
pub fn ws_router() -> Router<AppState> {
    Router::new().route("/collab/:code", get(relay_handler))
}

#[derive(Deserialize)]
struct CreateSession {
    /// The project's folder name. Shown to a joiner so they can tell they are
    /// about to join the session they were expecting.
    #[serde(default)]
    project: String,
}

#[derive(Serialize)]
struct SessionCreated {
    code: String,
    /// Where to point the editor. Returned rather than assembled client-side so
    /// a staging deployment does not need the desktop app to know about it.
    ws_url: String,
    expires_in_secs: u64,
}

async fn create_session(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreateSession>,
) -> Result<Json<SessionCreated>, ApiError> {
    state.collab_rooms.sweep();

    let username: Option<(String,)> = sqlx::query_as("SELECT username FROM users WHERE id = $1")
        .bind(auth.user_id)
        .fetch_optional(&state.db)
        .await?;
    let Some((username,)) = username else {
        return Err(ApiError::Unauthorized);
    };

    let project = body.project.trim().chars().take(128).collect::<String>();

    let code = {
        let mut rooms = state
            .collab_rooms
            .0
            .lock()
            .map_err(|_| ApiError::Internal("room registry poisoned".into()))?;

        let mine = rooms.values().filter(|r| r.host_user_id == auth.user_id).count();
        if mine >= MAX_ROOMS_PER_HOST {
            return Err(ApiError::Validation(format!(
                "You already have {mine} sessions open. End one before starting another."
            )));
        }

        // Retry rather than trusting one draw. Collisions are vanishingly
        // unlikely, and a collision here would hand two hosts the same room.
        let code = (0..8)
            .map(|_| new_code())
            .find(|candidate| !rooms.contains_key(candidate))
            .ok_or_else(|| ApiError::Internal("could not allocate a room code".into()))?;

        rooms.insert(
            code.clone(),
            Room {
                code: code.clone(),
                host_user_id: auth.user_id,
                host_username: username,
                project: project.clone(),
                created_at: Instant::now(),
                host: None,
                guests: HashMap::new(),
                next_peer: 1,
            },
        );
        code
    };

    Ok(Json(SessionCreated {
        ws_url: ws_url(&state.site_url, &code),
        code,
        expires_in_secs: MAX_SESSION.as_secs(),
    }))
}

/// The relay URL for a code, derived from the configured site URL so that
/// staging and local deployments answer with their own address.
fn ws_url(site_url: &str, code: &str) -> String {
    let base = site_url.trim_end_matches('/');
    let ws = if let Some(rest) = base.strip_prefix("https://") {
        format!("wss://{rest}")
    } else if let Some(rest) = base.strip_prefix("http://") {
        format!("ws://{rest}")
    } else {
        format!("wss://{base}")
    };
    format!("{ws}/api/ws/collab/{code}")
}

#[derive(Serialize)]
struct GuestInfo {
    peer: u32,
    username: String,
}

#[derive(Serialize)]
struct SessionInfo {
    code: String,
    host_username: String,
    project: String,
    /// Who is already in there. A joiner should be able to see that before
    /// deciding, and the host's panel uses it to list the room.
    guests: Vec<GuestInfo>,
    /// False while the host has created the session but not yet connected.
    host_online: bool,
    ws_url: String,
}

/// What a joiner sees before committing. Deliberately readable by any signed-in
/// user holding the code — the code *is* the invitation, and a joiner needs to
/// see whose session it is before handing their editor over to it.
async fn get_session(
    State(state): State<AppState>,
    Extension(_auth): Extension<AuthUser>,
    Path(code): Path<String>,
) -> Result<Json<SessionInfo>, ApiError> {
    state.collab_rooms.sweep();
    let rooms = state
        .collab_rooms
        .0
        .lock()
        .map_err(|_| ApiError::Internal("room registry poisoned".into()))?;
    let room = rooms.get(&normalize(&code)).ok_or(ApiError::NotFound)?;
    Ok(Json(SessionInfo {
        code: room.code.clone(),
        host_username: room.host_username.clone(),
        project: room.project.clone(),
        guests: room
            .guests
            .iter()
            .map(|(&peer, p)| GuestInfo { peer, username: p.username.clone() })
            .collect(),
        host_online: room.host.is_some(),
        ws_url: ws_url(&state.site_url, &room.code),
    }))
}

async fn end_session(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(code): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let room = {
        let mut rooms = state
            .collab_rooms
            .0
            .lock()
            .map_err(|_| ApiError::Internal("room registry poisoned".into()))?;
        let code = normalize(&code);
        match rooms.get(&code) {
            Some(room) if room.host_user_id == auth.user_id => rooms.remove(&code),
            Some(_) => return Err(ApiError::Unauthorized),
            None => return Err(ApiError::NotFound),
        }
    };
    // Outside the lock: telling everyone the room is gone is a send per socket,
    // and the registry should not be held while that happens.
    if let Some(room) = room {
        close_room(room, "the host ended the session");
    }
    Ok(Json(serde_json::json!({ "ended": true })))
}

#[derive(Deserialize)]
struct InviteRequest {
    user_id: Uuid,
}

/// Invite someone by pushing the code to them as a notification.
///
/// The notification is the delivery mechanism, not the permission: anyone
/// holding the code can join. Sending it this way just saves reading eight
/// characters down a phone line, and gives the invitee something clickable.
async fn invite(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(code): Path<String>,
    Json(body): Json<InviteRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if body.user_id == auth.user_id {
        return Err(ApiError::Validation("You are already in this session".into()));
    }

    let (host_username, project) = {
        let rooms = state
            .collab_rooms
            .0
            .lock()
            .map_err(|_| ApiError::Internal("room registry poisoned".into()))?;
        let room = rooms.get(&normalize(&code)).ok_or(ApiError::NotFound)?;
        if room.host_user_id != auth.user_id {
            return Err(ApiError::Unauthorized);
        }
        (room.host_username.clone(), room.project.clone())
    };

    // Invitations are for people you know. Without this the endpoint is a way to
    // push a notification to any account on the site by id.
    let are_friends: Option<(i64,)> = sqlx::query_as(
        "SELECT 1 FROM friends WHERE user_id = $1 AND friend_id = $2 AND status = 'accepted'",
    )
    .bind(auth.user_id)
    .bind(body.user_id)
    .fetch_optional(&state.db)
    .await?;
    if are_friends.is_none() {
        return Err(ApiError::Validation(
            "You can only invite friends to a session".into(),
        ));
    }

    let code = normalize(&code);
    let title = format!("{host_username} invited you to edit together");
    let body_text = if project.is_empty() {
        format!("Join their session with code {code}")
    } else {
        format!("Join “{project}” with code {code}")
    };
    notify::notify(
        &state,
        body.user_id,
        "collab_invite",
        &title,
        &body_text,
        Some(&format!("renzora://collab/{code}")),
        None,
    )
    .await?;

    // Also push it as its own live event, so an editor that is running can offer
    // a one-click join instead of making the user read the code out of a
    // notification and type it back in.
    state.ws_broadcast.send_to_user(
        body.user_id,
        "collab_invite",
        serde_json::json!({
            "code": code,
            "host_username": host_username,
            "project": project,
            "ws_url": ws_url(&state.site_url, &code),
        }),
    );

    Ok(Json(serde_json::json!({ "invited": true })))
}

/// Codes are shown uppercase and typed however the user likes.
fn normalize(code: &str) -> String {
    code.trim().to_ascii_uppercase()
}

// ── The relay socket ────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct RelayQuery {
    token: String,
}

async fn relay_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(code): Path<String>,
    Query(params): Query<RelayQuery>,
) -> Response {
    let user_id = match jwt::validate_token(&params.token, &state.jwt_secret) {
        Ok(claims) if claims.token_type == "access" => claims.sub,
        _ => {
            return Response::builder().status(401).body("Unauthorized".into()).unwrap();
        }
    };

    let code = normalize(&code);
    state.collab_rooms.sweep();

    // Resolve the username once here, where we can still return an HTTP status.
    // After the upgrade the only way to refuse is to close the socket, which
    // tells the client far less.
    let username: Option<(String,)> = sqlx::query_as("SELECT username FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(&state.db)
        .await
        .ok()
        .flatten();
    let Some((username,)) = username else {
        return Response::builder().status(401).body("Unknown account".into()).unwrap();
    };

    {
        let rooms = match state.collab_rooms.0.lock() {
            Ok(rooms) => rooms,
            Err(_) => {
                return Response::builder().status(500).body("Registry unavailable".into()).unwrap()
            }
        };
        let Some(room) = rooms.get(&code) else {
            return Response::builder().status(404).body("No such session".into()).unwrap();
        };
        // A second host socket for the same room would leave the first one
        // connected but unreachable, so refuse it outright rather than
        // silently replacing it.
        if room.host_user_id == user_id && room.host.is_some() {
            return Response::builder()
                .status(409)
                .body("This session already has a host connected".into())
                .unwrap();
        }
    }

    ws.on_upgrade(move |socket| run_relay(socket, state, code, user_id, username))
}

async fn run_relay(
    mut socket: WebSocket,
    state: AppState,
    code: String,
    user_id: Uuid,
    username: String,
) {
    // One channel per participant, and only this task ever touches the socket.
    // Anything wanting to reach this participant — the other side of the room,
    // the relay itself — pushes into the channel, so there is never a second
    // writer to race with.
    let (tx, mut rx) = mpsc::channel::<Message>(QUEUE_DEPTH);

    // Claim a slot.
    let role = {
        let mut rooms = match state.collab_rooms.0.lock() {
            Ok(rooms) => rooms,
            Err(_) => return,
        };
        let Some(room) = rooms.get_mut(&code) else {
            return;
        };
        let participant = Participant { username: username.clone(), tx: tx.clone() };
        if room.host_user_id == user_id && room.host.is_none() {
            room.host = Some(participant);
            Role::Host
        } else {
            let peer = room.next_peer;
            room.next_peer += 1;
            room.guests.insert(peer, participant);
            // Tell the host who just arrived. A relayed session has no
            // per-guest socket for the host to notice opening, so this text
            // frame is the only announcement it gets.
            if let Some(host) = &room.host {
                let _ = host.tx.try_send(control(serde_json::json!({
                    "event": "peer_joined",
                    "peer": peer,
                    "username": username,
                    "user_id": user_id,
                })));
            }
            Role::Guest(peer)
        }
    };

    let me = match role {
        Role::Host => HOST_PEER,
        Role::Guest(peer) => peer,
    };
    let _ = tx
        .send(control(serde_json::json!({
            "event": "ready",
            "role": if matches!(role, Role::Host) { "host" } else { "guest" },
            "peer": me,
        })))
        .await;

    // Pump until this side goes away. One task, both directions — the same
    // shape `/ws/live` uses, and the reason no `futures` split is needed.
    loop {
        tokio::select! {
            outbound = rx.recv() => {
                let Some(msg) = outbound else { break };
                let closing = matches!(msg, Message::Close(_));
                if socket.send(msg).await.is_err() || closing {
                    break;
                }
            }
            inbound = socket.recv() => {
                match inbound {
                    Some(Ok(Message::Binary(data))) => {
                        // Under four bytes there is no envelope to read, so the
                        // sender is not speaking this protocol.
                        if data.len() > MAX_MESSAGE || data.len() < 4 {
                            break;
                        }
                        if !forward(&state, &code, role, data) {
                            break;
                        }
                    }
                    // Text frames on this socket are relay control, and clients
                    // have nothing to say on that channel. Ignored rather than
                    // refused, so a control message from a newer client cannot
                    // break an older server.
                    Some(Ok(Message::Text(_))) | Some(Ok(Message::Pong(_))) => {}
                    Some(Ok(Message::Ping(payload))) => {
                        if socket.send(Message::Pong(payload)).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(Message::Close(_))) | None | Some(Err(_)) => break,
                }
            }
        }
    }

    depart(&state, &code, role).await;
}

#[derive(Clone, Copy)]
enum Role {
    Host,
    Guest(u32),
}

fn control(value: serde_json::Value) -> Message {
    Message::Text(value.to_string())
}

/// Route one payload. Returns false if the sender should be disconnected.
fn forward(state: &AppState, code: &str, from: Role, data: Vec<u8>) -> bool {
    let target = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    let Ok(mut rooms) = state.collab_rooms.0.lock() else {
        return false;
    };
    let Some(room) = rooms.get_mut(code) else {
        return false;
    };

    match from {
        Role::Guest(peer) => {
            // A guest's envelope is rewritten, never trusted: the target it
            // names is ignored and replaced with its own id addressed to the
            // host. Otherwise one guest could forge traffic as another, and the
            // host has no way to tell them apart.
            let Some(host) = &room.host else {
                return false; // nobody to talk to
            };
            let mut framed = peer.to_le_bytes().to_vec();
            framed.extend_from_slice(&data[4..]);
            host.tx.try_send(Message::Binary(framed)).is_ok()
        }
        Role::Host => {
            let payload = &data[4..];
            if target == BROADCAST {
                // A guest that cannot keep up is dropped from the send list but
                // does not fail the broadcast — the others are keeping up fine,
                // and its own socket task will notice and leave.
                let mut framed = HOST_PEER.to_le_bytes().to_vec();
                framed.extend_from_slice(payload);
                for guest in room.guests.values() {
                    let _ = guest.tx.try_send(Message::Binary(framed.clone()));
                }
                true
            } else {
                let mut framed = HOST_PEER.to_le_bytes().to_vec();
                framed.extend_from_slice(payload);
                if let Some(guest) = room.guests.get(&target) {
                    let _ = guest.tx.try_send(Message::Binary(framed));
                }
                // A message for a guest who has already left is not the host's
                // fault and must not cost it its connection.
                true
            }
        }
    }
}

/// Remove a participant, and tear the room down if it was the host.
async fn depart(state: &AppState, code: &str, role: Role) {
    let closing = {
        let Ok(mut rooms) = state.collab_rooms.0.lock() else {
            return;
        };
        match role {
            Role::Guest(peer) => {
                if let Some(room) = rooms.get_mut(code) {
                    room.guests.remove(&peer);
                    if let Some(host) = &room.host {
                        let _ = host.tx.try_send(control(serde_json::json!({
                            "event": "peer_left",
                            "peer": peer,
                        })));
                    }
                }
                None
            }
            // The host leaving ends the session. Guests are not left connected
            // to a room with nothing in it: the host owns the document, so
            // without it there is nothing for anyone to be editing.
            Role::Host => rooms.remove(code),
        }
    };
    if let Some(room) = closing {
        close_room(room, "the host disconnected");
    }
}

fn close_room(room: Room, reason: &str) {
    let notice = control(serde_json::json!({ "event": "host_gone", "reason": reason }));
    for guest in room.guests.values() {
        let _ = guest.tx.try_send(notice.clone());
        let _ = guest.tx.try_send(Message::Close(None));
    }
    if let Some(host) = &room.host {
        let _ = host.tx.try_send(Message::Close(None));
    }
    tracing::info!(code = %room.code, "collab session ended: {reason}");
}
