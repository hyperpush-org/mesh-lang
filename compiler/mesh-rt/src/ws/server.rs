//! WebSocket server runtime: actor-per-connection with reader thread bridge.
//!
//! Integrates the Phase 59 WebSocket protocol layer (frame codec, handshake,
//! close) with Mesh's actor system. Each accepted WebSocket connection spawns
//! a dedicated actor with crash isolation via `catch_unwind`.
//!
//! The **reader thread bridge** is the novel component: a dedicated OS thread
//! per connection reads WebSocket frames from the TCP stream and pushes them
//! into the actor's mailbox via reserved type tags, without blocking the M:N
//! scheduler's worker threads.
//!
//! ## Architecture
//!
//! ```text
//! TcpListener (accept loop on calling thread)
//!     |
//!     v  spawn actor per connection
//! ws_connection_entry (actor coroutine on scheduler worker)
//!     |
//!     +-- perform_upgrade (HTTP -> WebSocket)
//!     +-- call on_connect (accept/reject)
//!     +-- wrap stream in Arc<Mutex<WsStream>> (unified plain/TLS)
//!     +-- spawn reader thread (shared mutex for read/write)
//!     +-- actor_message_loop (receive -> dispatch)
//!     +-- cleanup (close frame, shutdown reader thread)
//! ```

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use parking_lot::Mutex;
use rustls::{ServerConfig, ServerConnection, StreamOwned};

use super::close::{process_frame, send_close, validate_text_payload, WsCloseCode};
use super::frame::{read_frame, write_frame, WsFrame, WsOpcode};
use super::handshake::perform_upgrade;
use crate::actor::process::Process;
use crate::actor::stack;
use crate::actor::{global_scheduler, Message, MessageBuffer, ProcessId, ProcessState};
use crate::string::MeshString;

// ---------------------------------------------------------------------------
// Stream abstraction for plain TCP and TLS WebSocket connections
// ---------------------------------------------------------------------------

/// Stream abstraction for plain TCP and TLS WebSocket connections.
/// Mirrors HttpStream in http/server.rs. Both variants implement Read + Write.
pub(crate) enum WsStream {
    Plain(TcpStream),
    Tls(StreamOwned<ServerConnection, TcpStream>),
}

impl WsStream {
    /// Set the read timeout on the underlying TcpStream.
    /// Works for both Plain and Tls variants since TLS uses the underlying
    /// TCP socket's timeout.
    fn set_read_timeout(&self, dur: Option<Duration>) -> std::io::Result<()> {
        match self {
            WsStream::Plain(s) => s.set_read_timeout(dur),
            WsStream::Tls(s) => s.get_ref().set_read_timeout(dur),
        }
    }
}

impl Read for WsStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            WsStream::Plain(s) => s.read(buf),
            WsStream::Tls(s) => s.read(buf),
        }
    }
}

impl Write for WsStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            WsStream::Plain(s) => s.write(buf),
            WsStream::Tls(s) => s.write(buf),
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            WsStream::Plain(s) => s.flush(),
            WsStream::Tls(s) => s.flush(),
        }
    }
}

// ---------------------------------------------------------------------------
// Heartbeat state (BEAT-01 through BEAT-05)
// ---------------------------------------------------------------------------

/// Tracks ping/pong heartbeat state for dead connection detection.
///
/// The reader thread sends periodic Ping frames with random 4-byte payloads
/// and validates that the Pong response echoes the payload. If no valid Pong
/// is received within `pong_timeout` after the last Ping, the connection is
/// considered dead and closed with code 1001 (Going Away).
struct HeartbeatState {
    last_ping_sent: Instant,
    last_pong_received: Instant,
    ping_interval: Duration, // default 30s
    pong_timeout: Duration,  // default 10s
    pending_ping_payload: Option<[u8; 4]>,
}

impl HeartbeatState {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            last_ping_sent: now,
            last_pong_received: now,
            ping_interval: Duration::from_secs(30),
            pong_timeout: Duration::from_secs(10),
            pending_ping_payload: None,
        }
    }

    fn should_send_ping(&self) -> bool {
        self.last_ping_sent.elapsed() >= self.ping_interval
    }

    fn is_pong_overdue(&self) -> bool {
        if self.pending_ping_payload.is_some() {
            self.last_ping_sent.elapsed() >= self.pong_timeout
        } else {
            false
        }
    }
}

// ---------------------------------------------------------------------------
// Fragment reassembly state (FRAG-01 through FRAG-03)
// ---------------------------------------------------------------------------

/// Tracks fragment reassembly state for continuation frames.
///
/// RFC 6455 Section 5.4: fragmented messages consist of a first fragment
/// (FIN=0, data opcode), zero or more continuation fragments (FIN=0,
/// opcode 0x0), and a final fragment (FIN=1, opcode 0x0). Control frames
/// may be interleaved between fragments.
struct FragmentState {
    /// The opcode of the first fragment (Text or Binary). None = not in a fragment sequence.
    initial_opcode: Option<WsOpcode>,
    /// Accumulated payload bytes from all fragments so far.
    buffer: Vec<u8>,
    /// Maximum total message size (16 MiB).
    max_message_size: usize,
}

impl FragmentState {
    fn new() -> Self {
        Self {
            initial_opcode: None,
            buffer: Vec::new(),
            max_message_size: 16 * 1024 * 1024,
        }
    }

    fn is_assembling(&self) -> bool {
        self.initial_opcode.is_some()
    }
}

/// Result of feeding a data frame to the fragment reassembly state machine.
enum ReassembleResult {
    /// A complete message is ready (unfragmented or final fragment assembled).
    Complete(WsFrame),
    /// Still accumulating fragments.
    Accumulating,
    /// Total message size exceeded the limit (FRAG-03).
    TooLarge,
    /// Protocol error (e.g., new message during fragmented sequence, unexpected continuation).
    ProtocolError(&'static str),
}

/// Feed a data frame through the fragment reassembly state machine.
///
/// Control frames (Ping/Pong/Close) must be handled BEFORE calling this
/// function -- they are not part of the fragment sequence (FRAG-02).
fn reassemble(frag: &mut FragmentState, frame: WsFrame) -> ReassembleResult {
    match frame.opcode {
        // Unfragmented message: FIN=1, data opcode, not currently assembling
        WsOpcode::Text | WsOpcode::Binary if frame.fin && !frag.is_assembling() => {
            ReassembleResult::Complete(frame)
        }
        // First fragment: FIN=0, data opcode, not currently assembling
        WsOpcode::Text | WsOpcode::Binary if !frame.fin && !frag.is_assembling() => {
            frag.initial_opcode = Some(frame.opcode);
            frag.buffer = frame.payload;
            if frag.buffer.len() > frag.max_message_size {
                frag.initial_opcode = None;
                frag.buffer.clear();
                return ReassembleResult::TooLarge;
            }
            ReassembleResult::Accumulating
        }
        // Protocol error: new message started while assembling
        WsOpcode::Text | WsOpcode::Binary if frag.is_assembling() => {
            frag.initial_opcode = None;
            frag.buffer.clear();
            ReassembleResult::ProtocolError("new message during fragmented sequence")
        }
        // Continuation fragment: FIN=0, currently assembling
        WsOpcode::Continuation if !frame.fin && frag.is_assembling() => {
            if frag.buffer.len() + frame.payload.len() > frag.max_message_size {
                frag.initial_opcode = None;
                frag.buffer.clear();
                return ReassembleResult::TooLarge;
            }
            frag.buffer.extend_from_slice(&frame.payload);
            ReassembleResult::Accumulating
        }
        // Final fragment: FIN=1, continuation, currently assembling
        WsOpcode::Continuation if frame.fin && frag.is_assembling() => {
            if frag.buffer.len() + frame.payload.len() > frag.max_message_size {
                frag.initial_opcode = None;
                frag.buffer.clear();
                return ReassembleResult::TooLarge;
            }
            frag.buffer.extend_from_slice(&frame.payload);
            let opcode = frag.initial_opcode.take().unwrap();
            let payload = std::mem::take(&mut frag.buffer);
            ReassembleResult::Complete(WsFrame {
                fin: true,
                opcode,
                payload,
            })
        }
        // Protocol error: continuation without preceding first fragment
        WsOpcode::Continuation if !frag.is_assembling() => {
            ReassembleResult::ProtocolError("unexpected continuation frame")
        }
        // Control frames should never reach reassemble (handled before this call)
        _ => ReassembleResult::ProtocolError("unexpected opcode in reassembly"),
    }
}

// ---------------------------------------------------------------------------
// Reserved type tags for WebSocket mailbox messages
// ---------------------------------------------------------------------------

/// Reserved type tag for WebSocket text frames.
pub const WS_TEXT_TAG: u64 = u64::MAX - 1;

/// Reserved type tag for WebSocket binary frames.
pub const WS_BINARY_TAG: u64 = u64::MAX - 2;

/// Reserved type tag for WebSocket disconnect (close/error from client).
pub const WS_DISCONNECT_TAG: u64 = u64::MAX - 3;

/// Reserved type tag for WebSocket connect notification.
pub const WS_CONNECT_TAG: u64 = u64::MAX - 4;

/// WebSocket close code 1008 (Policy Violation) for on_connect rejection.
const WS_POLICY_VIOLATION: u16 = 1008;

// ---------------------------------------------------------------------------
// Handler and connection structs
// ---------------------------------------------------------------------------

/// WebSocket handler containing three Mesh closure pairs (on_connect,
/// on_message, on_close). Each closure is a `{fn_ptr, env_ptr}` pair.
///
/// Passed from the Mesh-compiled program to `mesh_ws_serve`. The struct
/// is `#[repr(C)]` so Mesh's codegen can construct it directly.
#[repr(C)]
struct WsHandler {
    on_connect_fn: *mut u8,
    on_connect_env: *mut u8,
    on_message_fn: *mut u8,
    on_message_env: *mut u8,
    on_close_fn: *mut u8,
    on_close_env: *mut u8,
}

// WsHandler contains raw function pointers transferred between threads.
// The pointers are to compiled Mesh functions which are valid for the
// lifetime of the program.
unsafe impl Send for WsHandler {}

/// Connection handle for `Ws.send` -- stored on the Rust heap (not GC heap)
/// because it contains an `Arc<Mutex<WsStream>>`.
pub(crate) struct WsConnection {
    pub(crate) write_stream: Arc<Mutex<WsStream>>,
    pub(crate) shutdown: Arc<AtomicBool>,
}

/// Arguments passed to the spawned WebSocket actor, following the HTTP
/// server's `ConnectionArgs` pattern.
#[repr(C)]
struct WsConnectionArgs {
    handler: WsHandler,
    stream_ptr: usize,
}

// WsConnectionArgs contains raw pointers but is only used for transfer
// to the actor entry function.
unsafe impl Send for WsConnectionArgs {}

// ---------------------------------------------------------------------------
// Public API: mesh_ws_serve, mesh_ws_send, mesh_ws_send_binary
// ---------------------------------------------------------------------------

/// Start a WebSocket server on the given port, blocking the calling thread.
///
/// Binds a TCP listener and spawns one actor per accepted WebSocket
/// connection. Each connection actor runs the upgrade handshake, calls
/// lifecycle callbacks (on_connect, on_message, on_close), and uses a
/// reader thread bridge for non-blocking frame delivery.
///
/// # Arguments
///
/// Six function/env pointer pairs for the three callbacks, plus the port:
/// - `on_connect_fn/env`: Called after handshake with (conn, path, headers)
/// - `on_message_fn/env`: Called for each text/binary frame with (conn, msg)
/// - `on_close_fn/env`: Called when connection ends with (conn, code, reason)
/// - `port`: TCP port to listen on
#[no_mangle]
pub extern "C" fn mesh_ws_serve(
    on_connect_fn: *mut u8,
    on_connect_env: *mut u8,
    on_message_fn: *mut u8,
    on_message_env: *mut u8,
    on_close_fn: *mut u8,
    on_close_env: *mut u8,
    port: i64,
) {
    // Ensure the actor scheduler is initialized (idempotent).
    crate::actor::mesh_rt_init_actor(0);

    let addr = format!("0.0.0.0:{}", port);
    let listener = match TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!(
                "[mesh-rt] Failed to start WebSocket server on {}: {}",
                addr, e
            );
            return;
        }
    };

    eprintln!("[mesh-rt] WebSocket server listening on {}", addr);

    // Wrap raw pointers for Send (function pointers are valid for program lifetime).
    let handler = SendableHandler {
        on_connect_fn,
        on_connect_env,
        on_message_fn,
        on_message_env,
        on_close_fn,
        on_close_env,
    };

    // Spawn an OS thread for the accept loop so Ws.serve returns immediately.
    // This allows calling Ws.serve before HTTP.serve in the same function
    // without blocking (both are blocking accept loops).
    std::thread::Builder::new()
        .name(format!("ws-accept-{}", port))
        .spawn(move || {
            ws_accept_loop(listener, handler);
        })
        .expect("Failed to spawn WebSocket accept thread");
}

/// Wrapper for raw callback pointers to satisfy Send requirement.
/// Safe because these are function pointers (or null) that remain valid
/// for the entire program lifetime.
struct SendableHandler {
    on_connect_fn: *mut u8,
    on_connect_env: *mut u8,
    on_message_fn: *mut u8,
    on_message_env: *mut u8,
    on_close_fn: *mut u8,
    on_close_env: *mut u8,
}
unsafe impl Send for SendableHandler {}

/// Accept loop for WebSocket connections. Runs on a dedicated OS thread,
/// dispatching each accepted connection to an actor on the Mesh scheduler.
fn ws_accept_loop(listener: TcpListener, h: SendableHandler) {
    for tcp_stream in listener.incoming() {
        let tcp_stream = match tcp_stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[mesh-rt] accept error: {}", e);
                continue;
            }
        };

        // Set read timeout before wrapping in WsStream.
        tcp_stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .ok();

        let ws_stream = WsStream::Plain(tcp_stream);

        // Pack handler (copy the 6 pointers) and stream into args.
        let handler = WsHandler {
            on_connect_fn: h.on_connect_fn,
            on_connect_env: h.on_connect_env,
            on_message_fn: h.on_message_fn,
            on_message_env: h.on_message_env,
            on_close_fn: h.on_close_fn,
            on_close_env: h.on_close_env,
        };
        let stream_ptr = Box::into_raw(Box::new(ws_stream)) as usize;
        let args = WsConnectionArgs {
            handler,
            stream_ptr,
        };
        let args_ptr = Box::into_raw(Box::new(args)) as *const u8;
        let args_size = std::mem::size_of::<WsConnectionArgs>() as u64;

        let sched = global_scheduler();
        sched.spawn(
            ws_connection_entry as *const u8,
            args_ptr,
            args_size,
            1, // Normal priority
        );
    }
}

/// Start a WebSocket TLS server on the given port, blocking the calling thread.
///
/// Same as `mesh_ws_serve` but wraps each connection in TLS via rustls.
/// Certificate and private key are loaded from PEM files at the given paths.
#[no_mangle]
pub extern "C" fn mesh_ws_serve_tls(
    on_connect_fn: *mut u8,
    on_connect_env: *mut u8,
    on_message_fn: *mut u8,
    on_message_env: *mut u8,
    on_close_fn: *mut u8,
    on_close_env: *mut u8,
    port: i64,
    cert_path: *const MeshString,
    key_path: *const MeshString,
) {
    crate::actor::mesh_rt_init_actor(0);

    let cert_str = unsafe { (*cert_path).as_str() };
    let key_str = unsafe { (*key_path).as_str() };

    let tls_config = match crate::http::server::build_server_config(cert_str, key_str) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[mesh-rt] Failed to load TLS certificates: {}", e);
            return;
        }
    };

    let addr = format!("0.0.0.0:{}", port);
    let listener = match TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!(
                "[mesh-rt] Failed to start WebSocket TLS server on {}: {}",
                addr, e
            );
            return;
        }
    };

    eprintln!("[mesh-rt] WebSocket TLS server listening on {}", addr);

    let config_ptr = Arc::into_raw(tls_config) as usize;

    for tcp_stream in listener.incoming() {
        let tcp_stream = match tcp_stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[mesh-rt] accept error: {}", e);
                continue;
            }
        };

        // Set read timeout BEFORE TLS wrapping (Pitfall 1 from research)
        tcp_stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .ok();

        let tls_config = unsafe { Arc::from_raw(config_ptr as *const ServerConfig) };
        let conn = match ServerConnection::new(Arc::clone(&tls_config)) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[mesh-rt] TLS connection setup failed: {}", e);
                std::mem::forget(tls_config);
                continue;
            }
        };
        std::mem::forget(tls_config);

        let tls_stream = StreamOwned::new(conn, tcp_stream);
        let ws_stream = WsStream::Tls(tls_stream);

        let handler = WsHandler {
            on_connect_fn,
            on_connect_env,
            on_message_fn,
            on_message_env,
            on_close_fn,
            on_close_env,
        };
        let stream_ptr = Box::into_raw(Box::new(ws_stream)) as usize;
        let args = WsConnectionArgs {
            handler,
            stream_ptr,
        };
        let args_ptr = Box::into_raw(Box::new(args)) as *const u8;
        let args_size = std::mem::size_of::<WsConnectionArgs>() as u64;

        let sched = global_scheduler();
        sched.spawn(ws_connection_entry as *const u8, args_ptr, args_size, 1);
    }
}

/// Send a text frame to a WebSocket client.
///
/// `conn` is a pointer to a `WsConnection` (obtained from the on_connect
/// callback). `msg` is a pointer to a `MeshString` containing the text.
///
/// Returns 0 on success, -1 on error.
#[no_mangle]
pub extern "C" fn mesh_ws_send(conn: *mut u8, msg: *const MeshString) -> i64 {
    if conn.is_null() || msg.is_null() {
        return -1;
    }
    let conn = unsafe { &*(conn as *const WsConnection) };
    let text = unsafe { (*msg).as_str() };
    let mut stream = conn.write_stream.lock();
    match write_frame(&mut *stream, WsOpcode::Text, text.as_bytes(), true) {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

/// Send a binary frame to a WebSocket client.
///
/// `conn` is a pointer to a `WsConnection`. `data` and `len` specify the
/// raw bytes to send.
///
/// Returns 0 on success, -1 on error.
#[no_mangle]
pub extern "C" fn mesh_ws_send_binary(conn: *mut u8, data: *const u8, len: i64) -> i64 {
    if conn.is_null() || data.is_null() {
        return -1;
    }
    let conn = unsafe { &*(conn as *const WsConnection) };
    let bytes = unsafe { std::slice::from_raw_parts(data, len as usize) };
    let mut stream = conn.write_stream.lock();
    match write_frame(&mut *stream, WsOpcode::Binary, bytes, true) {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

// ---------------------------------------------------------------------------
// Actor entry point
// ---------------------------------------------------------------------------

/// Actor entry function for a single WebSocket connection.
///
/// Performs the upgrade handshake, splits the stream, spawns the reader
/// thread, runs the message loop, and handles cleanup on exit or crash.
extern "C" fn ws_connection_entry(args: *const u8) {
    if args.is_null() {
        return;
    }

    let args = unsafe { Box::from_raw(args as *mut WsConnectionArgs) };
    let handler = args.handler;
    let mut stream = unsafe { *Box::from_raw(args.stream_ptr as *mut WsStream) };

    // 1. Perform WebSocket upgrade handshake
    let (path, headers) = match perform_upgrade(&mut stream) {
        Ok(ph) => ph,
        Err(e) => {
            eprintln!("[mesh-rt] WS upgrade failed: {}", e);
            return;
        }
    };

    // 2. Wrap stream in Arc<Mutex<WsStream>> for shared reader/writer access.
    //    Unlike TcpStream::try_clone(), StreamOwned (TLS) cannot be cloned.
    //    Using a single Arc<Mutex<WsStream>> unifies both plain and TLS paths.
    let stream = Arc::new(Mutex::new(stream));
    let shutdown = Arc::new(AtomicBool::new(false));

    // 3. Create WsConnection handle (Rust heap, not GC heap)
    let conn = Box::into_raw(Box::new(WsConnection {
        write_stream: stream.clone(),
        shutdown: shutdown.clone(),
    }));
    let conn_ptr = conn as *mut u8;

    // 4. Get current actor's PID and process Arc for reader thread
    let my_pid = stack::get_current_pid().expect("ws_connection_entry: no PID");
    let sched = global_scheduler();
    let proc_arc = sched
        .get_process(my_pid)
        .expect("ws_connection_entry: no process");

    // 5. Call on_connect callback (LIFE-01, LIFE-02)
    let accepted = call_on_connect(&handler, conn_ptr, &path, &headers);
    if !accepted {
        // on_connect rejected -- send close 1008 (Policy Violation)
        let _ = send_close(&mut *stream.lock(), WS_POLICY_VIOLATION, "rejected");
        shutdown.store(true, Ordering::SeqCst);
        // Clean up connection handle
        unsafe {
            drop(Box::from_raw(conn));
        }
        return;
    }

    // 6. Spawn reader thread (read timeout already set before WsStream wrapping)
    let reader_shutdown = shutdown.clone();
    let reader_stream = stream.clone();
    let reader_proc = proc_arc.clone();
    let reader_pid = my_pid;
    std::thread::spawn(move || {
        reader_thread_loop(reader_stream, reader_proc, reader_pid, reader_shutdown);
    });

    // 8. Actor message loop with catch_unwind (ACTOR-01, ACTOR-05)
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        actor_message_loop(&handler, conn_ptr);
    }));

    // 9. Cleanup
    // ROOM-05: remove from all rooms before signaling shutdown
    crate::ws::rooms::global_room_registry().cleanup_connection(conn as usize);
    shutdown.store(true, Ordering::SeqCst);

    if result.is_err() {
        // Actor crashed (ACTOR-05): send close 1011
        let _ = send_close(
            &mut *stream.lock(),
            WsCloseCode::INTERNAL_ERROR,
            "internal error",
        );
        // Call on_close for crash case (best-effort -- GC may not be safe)
        call_on_close(
            &handler,
            conn_ptr,
            WsCloseCode::INTERNAL_ERROR,
            "internal error",
        );
    } else {
        // Normal exit (disconnect): call on_close (LIFE-04)
        call_on_close(&handler, conn_ptr, WsCloseCode::NORMAL, "");
    }

    // Clean up connection handle
    unsafe {
        drop(Box::from_raw(conn));
    }
}

// ---------------------------------------------------------------------------
// Reader thread
// ---------------------------------------------------------------------------

/// Reader thread loop: reads WebSocket frames and pushes them into the
/// actor's mailbox via reserved type tags.
///
/// Runs on a dedicated OS thread to avoid blocking the M:N scheduler's worker
/// threads. Integrates three concerns:
///
/// 1. **Frame dispatch**: control frames (ping/pong/close) handled inline,
///    data frames (text/binary/continuation) routed through fragment reassembly.
/// 2. **Heartbeat** (BEAT-01..05): periodic Ping with random payload, validates
///    Pong response, closes dead connections after pong timeout.
/// 3. **Fragmentation** (FRAG-01..03): reassembles continuation frames into
///    complete messages, enforces 16 MiB size limit, handles interleaved
///    control frames without corrupting fragment state.
///
/// Uses `Arc<Mutex<WsStream>>` for unified plain TCP / TLS access. The mutex
/// is locked briefly for each `read_frame` call (100ms timeout) and released
/// between frames, allowing `Ws.send()` from the actor to acquire the lock.
fn reader_thread_loop(
    stream: Arc<Mutex<WsStream>>,
    proc_arc: Arc<Mutex<Process>>,
    actor_pid: ProcessId,
    shutdown: Arc<AtomicBool>,
) {
    // Set read timeout for the reader thread. Controls:
    // - Maximum contention window for Ws.send() from the actor side
    // - Granularity for shutdown and heartbeat checks
    // 100ms keeps contention low while allowing responsive shutdown.
    {
        let s = stream.lock();
        s.set_read_timeout(Some(Duration::from_millis(100))).ok();
    }

    let mut heartbeat = HeartbeatState::new();
    let mut frag = FragmentState::new();

    loop {
        if shutdown.load(Ordering::SeqCst) {
            break;
        }

        // HEARTBEAT: Check pong timeout (BEAT-04)
        if heartbeat.is_pong_overdue() {
            let mut s = stream.lock();
            let _ = send_close(&mut *s, WsCloseCode::GOING_AWAY, "pong timeout");
            drop(s);
            push_disconnect(&proc_arc, actor_pid);
            break;
        }

        // HEARTBEAT: Send periodic ping (BEAT-01)
        if heartbeat.should_send_ping() {
            let payload: [u8; 4] = rand::random();
            let mut s = stream.lock();
            let _ = write_frame(&mut *s, WsOpcode::Ping, &payload, true);
            drop(s);
            heartbeat.last_ping_sent = Instant::now();
            heartbeat.pending_ping_payload = Some(payload);
        }

        // Lock stream, read one frame, release lock.
        // The 100ms read timeout keeps the contention window small.
        let frame_result = {
            let mut s = stream.lock();
            read_frame(&mut *s)
        };

        match frame_result {
            Ok(frame) => {
                // HEARTBEAT: Handle Pong BEFORE process_frame (BEAT-02, BEAT-03)
                // The existing process_frame silently ignores Pong; we need
                // to inspect the payload to validate it matches our Ping.
                if frame.opcode == WsOpcode::Pong {
                    if let Some(expected) = heartbeat.pending_ping_payload {
                        if frame.payload == expected {
                            heartbeat.last_pong_received = Instant::now();
                            heartbeat.pending_ping_payload = None;
                        }
                    }
                    continue; // Pong handled, skip to next frame
                }

                // Handle control frames inline (FRAG-02): Ping and Close are
                // processed regardless of fragment state, without corrupting
                // the fragment buffer.
                if matches!(frame.opcode, WsOpcode::Ping | WsOpcode::Close) {
                    let mut s = stream.lock();
                    match process_frame(&mut *s, frame) {
                        Ok(None) => { /* Ping -> Pong sent */ }
                        Err(_) => {
                            drop(s);
                            // Close frame -> push disconnect
                            push_disconnect(&proc_arc, actor_pid);
                            break;
                        }
                        _ => {}
                    }
                    continue;
                }

                // FRAGMENTATION: Feed data frames through reassembly (FRAG-01, FRAG-03)
                match reassemble(&mut frag, frame) {
                    ReassembleResult::Complete(msg) => {
                        // UTF-8 validation for text (Pitfall 6: validate on
                        // the fully reassembled payload, not individual fragments)
                        if msg.opcode == WsOpcode::Text {
                            if validate_text_payload(&msg.payload).is_err() {
                                let mut s = stream.lock();
                                let _ =
                                    send_close(&mut *s, WsCloseCode::INVALID_DATA, "invalid UTF-8");
                                drop(s);
                                push_disconnect(&proc_arc, actor_pid);
                                break;
                            }
                        }
                        let tag = match msg.opcode {
                            WsOpcode::Text => WS_TEXT_TAG,
                            WsOpcode::Binary => WS_BINARY_TAG,
                            _ => WS_TEXT_TAG,
                        };
                        let buffer = MessageBuffer::new(msg.payload, tag);
                        let message = Message { buffer };

                        // Push to mailbox and wake actor if Waiting (Pitfall 7)
                        {
                            let mut proc = proc_arc.lock();
                            proc.mailbox.push(message);
                            if matches!(proc.state, ProcessState::Waiting) {
                                proc.state = ProcessState::Ready;
                                drop(proc);
                                let sched = global_scheduler();
                                sched.wake_process(actor_pid);
                            }
                        }
                    }
                    ReassembleResult::Accumulating => { /* waiting for more fragments */ }
                    ReassembleResult::TooLarge => {
                        let mut s = stream.lock();
                        let _ =
                            send_close(&mut *s, WsCloseCode::MESSAGE_TOO_BIG, "message too big");
                        drop(s);
                        push_disconnect(&proc_arc, actor_pid);
                        break;
                    }
                    ReassembleResult::ProtocolError(reason) => {
                        let mut s = stream.lock();
                        let _ = send_close(&mut *s, WsCloseCode::PROTOCOL_ERROR, reason);
                        drop(s);
                        push_disconnect(&proc_arc, actor_pid);
                        break;
                    }
                }
            }
            Err(e) => {
                if shutdown.load(Ordering::SeqCst) {
                    break;
                }
                // Check if it's a timeout (not a real error).
                // macOS returns "Resource temporarily unavailable" (EAGAIN) for
                // short timeouts and "timed out" (ETIMEDOUT) for longer ones.
                if e.contains("timed out")
                    || e.contains("WouldBlock")
                    || e.contains("temporarily unavailable")
                {
                    continue; // Just a read timeout, check shutdown and loop
                }
                // Real I/O error -- push disconnect
                push_disconnect(&proc_arc, actor_pid);
                break;
            }
        }
    }
}

/// Push a WS_DISCONNECT_TAG message to the actor's mailbox and wake it.
fn push_disconnect(proc_arc: &Arc<Mutex<Process>>, actor_pid: ProcessId) {
    let buffer = MessageBuffer::new(Vec::new(), WS_DISCONNECT_TAG);
    let msg = Message { buffer };
    let mut proc = proc_arc.lock();
    proc.mailbox.push(msg);
    if matches!(proc.state, ProcessState::Waiting) {
        proc.state = ProcessState::Ready;
        drop(proc);
        let sched = global_scheduler();
        sched.wake_process(actor_pid);
    }
}

// ---------------------------------------------------------------------------
// Actor message loop
// ---------------------------------------------------------------------------

/// Main message loop for the WebSocket actor.
///
/// Blocks on `mesh_actor_receive(-1)` to get messages from the mailbox.
/// Dispatches based on the type tag:
/// - `WS_TEXT_TAG` / `WS_BINARY_TAG`: call on_message callback
/// - `WS_DISCONNECT_TAG`: client disconnected, exit loop
/// - `EXIT_SIGNAL_TAG`: exit signal from linked actor, exit loop
/// - Other: regular actor-to-actor message (ignored for now)
fn actor_message_loop(handler: &WsHandler, conn_ptr: *mut u8) {
    use crate::actor::mesh_actor_receive;

    loop {
        let msg_ptr = mesh_actor_receive(-1);
        if msg_ptr.is_null() {
            break;
        }

        // Read type_tag from heap layout: [u64 type_tag, u64 data_len, u8... data]
        let type_tag = unsafe {
            let mut tag_bytes = [0u8; 8];
            std::ptr::copy_nonoverlapping(msg_ptr, tag_bytes.as_mut_ptr(), 8);
            u64::from_le_bytes(tag_bytes)
        };

        match type_tag {
            WS_TEXT_TAG | WS_BINARY_TAG => {
                // Read data_len and data pointer
                let (data_len, data_ptr) = unsafe {
                    let mut len_bytes = [0u8; 8];
                    std::ptr::copy_nonoverlapping(msg_ptr.add(8), len_bytes.as_mut_ptr(), 8);
                    let len = u64::from_le_bytes(len_bytes) as usize;
                    (len, msg_ptr.add(16))
                };
                // Call on_message (LIFE-03)
                call_on_message(
                    handler,
                    conn_ptr,
                    data_ptr,
                    data_len,
                    type_tag == WS_TEXT_TAG,
                );
            }
            WS_DISCONNECT_TAG => {
                // Client disconnected (ACTOR-06)
                break;
            }
            tag if tag == crate::actor::EXIT_SIGNAL_TAG => {
                // Exit signal from linked actor
                break;
            }
            _ => {
                // Regular actor-to-actor message -- ignore for now
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Callback invocation helpers
// ---------------------------------------------------------------------------

/// Call the on_connect callback.
///
/// Builds Mesh-level path string and headers map, invokes the callback.
/// Returns `true` if the connection is accepted, `false` if rejected.
///
/// If no on_connect callback is set (null fn pointer), accepts by default.
fn call_on_connect(
    handler: &WsHandler,
    conn_ptr: *mut u8,
    path: &str,
    headers: &[(String, String)],
) -> bool {
    if handler.on_connect_fn.is_null() {
        return true; // No callback = accept
    }

    unsafe {
        // Build Mesh-level path string
        let path_mesh = crate::string::mesh_string_new(path.as_ptr(), path.len() as u64) as *mut u8;

        // Build headers map
        let mut headers_map = crate::collections::map::mesh_map_new_typed(1);
        for (name, value) in headers {
            let key = crate::string::mesh_string_new(name.as_ptr(), name.len() as u64);
            let val = crate::string::mesh_string_new(value.as_ptr(), value.len() as u64);
            headers_map =
                crate::collections::map::mesh_map_put(headers_map, key as u64, val as u64);
        }

        // Call the closure: if env is null, bare function; if non-null, closure
        let result = if handler.on_connect_env.is_null() {
            let f: fn(*mut u8, *mut u8, *mut u8) -> *mut u8 =
                std::mem::transmute(handler.on_connect_fn);
            f(conn_ptr, path_mesh, headers_map)
        } else {
            let f: fn(*mut u8, *mut u8, *mut u8, *mut u8) -> *mut u8 =
                std::mem::transmute(handler.on_connect_fn);
            f(handler.on_connect_env, conn_ptr, path_mesh, headers_map)
        };

        // Convention: non-null = accepted, null = rejected
        !result.is_null()
    }
}

/// Call the on_message callback.
///
/// Converts the frame payload to a MeshString and invokes the callback.
fn call_on_message(
    handler: &WsHandler,
    conn_ptr: *mut u8,
    data_ptr: *const u8,
    data_len: usize,
    _is_text: bool,
) {
    if handler.on_message_fn.is_null() {
        return;
    }

    unsafe {
        // Build a Mesh string from the frame payload
        let msg_mesh = crate::string::mesh_string_new(data_ptr, data_len as u64) as *mut u8;

        if handler.on_message_env.is_null() {
            let f: fn(*mut u8, *mut u8) -> *mut u8 = std::mem::transmute(handler.on_message_fn);
            f(conn_ptr, msg_mesh);
        } else {
            let f: fn(*mut u8, *mut u8, *mut u8) -> *mut u8 =
                std::mem::transmute(handler.on_message_fn);
            f(handler.on_message_env, conn_ptr, msg_mesh);
        }
    }
}

/// Call the on_close callback.
///
/// Invoked when the connection ends (normal disconnect or crash).
fn call_on_close(handler: &WsHandler, conn_ptr: *mut u8, code: u16, reason: &str) {
    if handler.on_close_fn.is_null() {
        return;
    }

    unsafe {
        let code_i64 = code as i64;
        let reason_mesh =
            crate::string::mesh_string_new(reason.as_ptr(), reason.len() as u64) as *mut u8;

        if handler.on_close_env.is_null() {
            let f: fn(*mut u8, i64, *mut u8) -> *mut u8 = std::mem::transmute(handler.on_close_fn);
            f(conn_ptr, code_i64, reason_mesh);
        } else {
            let f: fn(*mut u8, *mut u8, i64, *mut u8) -> *mut u8 =
                std::mem::transmute(handler.on_close_fn);
            f(handler.on_close_env, conn_ptr, code_i64, reason_mesh);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ws::close::parse_close_payload;
    use crate::ws::frame::{apply_mask, read_frame, WsOpcode};
    use std::io::{Read, Write};
    use std::sync::atomic::{AtomicU64, Ordering};

    // ── Callback functions ───────────────────────────────────────────

    /// on_connect with per-test counter via env pointer. Returns non-null (accept).
    extern "C" fn counting_on_connect(
        env: *mut u8,
        _conn: *mut u8,
        _path: *mut u8,
        _headers: *mut u8,
    ) -> *mut u8 {
        if !env.is_null() {
            unsafe {
                (*(env as *const AtomicU64)).fetch_add(1, Ordering::SeqCst);
            }
        }
        1 as *mut u8
    }

    /// on_connect: accept without counting (env=null calling convention).
    extern "C" fn accept_on_connect(_conn: *mut u8, _path: *mut u8, _headers: *mut u8) -> *mut u8 {
        1 as *mut u8
    }

    /// on_message: echo the message back to the client (env=null).
    extern "C" fn echo_on_message(conn: *mut u8, msg: *mut u8) -> *mut u8 {
        mesh_ws_send(conn, msg as *const MeshString);
        std::ptr::null_mut()
    }

    /// on_message: always panic to test crash isolation (env=null).
    /// NOT extern "C" -- Rust ABI allows panic to unwind through catch_unwind.
    /// (extern "C" panics abort the process since Rust 1.71.)
    fn crash_on_message(_conn: *mut u8, _msg: *mut u8) -> *mut u8 {
        panic!("intentional test crash");
    }

    /// on_close with per-test counter via env pointer.
    extern "C" fn counting_on_close(
        env: *mut u8,
        _conn: *mut u8,
        _code: i64,
        _reason: *mut u8,
    ) -> *mut u8 {
        if !env.is_null() {
            unsafe {
                (*(env as *const AtomicU64)).fetch_add(1, Ordering::SeqCst);
            }
        }
        std::ptr::null_mut()
    }

    /// on_close: no-op (env=null calling convention).
    extern "C" fn noop_on_close(_conn: *mut u8, _code: i64, _reason: *mut u8) -> *mut u8 {
        std::ptr::null_mut()
    }

    // ── Helpers ──────────────────────────────────────────────────────

    /// Get a free port by binding to port 0 and releasing.
    fn free_port() -> u16 {
        let l = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        l.local_addr().unwrap().port()
    }

    /// Start a WS server that echoes messages back (no per-test counters).
    fn start_echo_server(port: u16) {
        std::thread::spawn(move || {
            mesh_ws_serve(
                accept_on_connect as *mut u8,
                std::ptr::null_mut(),
                echo_on_message as *mut u8,
                std::ptr::null_mut(),
                noop_on_close as *mut u8,
                std::ptr::null_mut(),
                port as i64,
            );
        });
        std::thread::sleep(Duration::from_millis(200));
    }

    /// Start a WS server with per-test connect/close counters.
    fn start_counting_server(
        port: u16,
        connect_ctr: &'static AtomicU64,
        close_ctr: &'static AtomicU64,
    ) {
        // Cast to usize to cross thread boundary (*mut u8 is !Send).
        let connect_env = connect_ctr as *const AtomicU64 as usize;
        let close_env = close_ctr as *const AtomicU64 as usize;
        std::thread::spawn(move || {
            mesh_ws_serve(
                counting_on_connect as *mut u8,
                connect_env as *mut u8,
                echo_on_message as *mut u8,
                std::ptr::null_mut(),
                counting_on_close as *mut u8,
                close_env as *mut u8,
                port as i64,
            );
        });
        std::thread::sleep(Duration::from_millis(200));
    }

    /// Start a WS server where on_message always panics.
    fn start_crash_server(port: u16) {
        std::thread::spawn(move || {
            mesh_ws_serve(
                accept_on_connect as *mut u8,
                std::ptr::null_mut(),
                crash_on_message as *mut u8,
                std::ptr::null_mut(),
                noop_on_close as *mut u8,
                std::ptr::null_mut(),
                port as i64,
            );
        });
        std::thread::sleep(Duration::from_millis(200));
    }

    /// Connect to a WS server and complete the HTTP upgrade handshake.
    /// Reads the HTTP response byte-by-byte to avoid consuming frame data.
    fn ws_connect(port: u16) -> TcpStream {
        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();

        let key = "dGhlIHNhbXBsZSBub25jZQ==";
        write!(
            stream,
            "GET /ws HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: {key}\r\nSec-WebSocket-Version: 13\r\n\r\n"
        ).unwrap();
        stream.flush().unwrap();

        // Read HTTP response byte-by-byte until \r\n\r\n to avoid
        // consuming any WebSocket frame bytes that follow.
        let mut resp = Vec::new();
        let mut byte = [0u8; 1];
        loop {
            stream.read_exact(&mut byte).unwrap();
            resp.push(byte[0]);
            if resp.ends_with(b"\r\n\r\n") {
                break;
            }
        }
        let resp_str = String::from_utf8_lossy(&resp);
        assert!(
            resp_str.contains("101"),
            "Expected 101 Switching Protocols, got: {}",
            resp_str
        );
        stream
    }

    /// Send a masked text frame (client-to-server must be masked per RFC 6455).
    fn ws_send_text(stream: &mut TcpStream, text: &str) {
        let mask_key = [0x12, 0x34, 0x56, 0x78];
        let mut payload = text.as_bytes().to_vec();
        apply_mask(&mut payload, &mask_key);

        let len = text.len();
        let mut frame = vec![0x81u8]; // FIN=1, opcode=Text
        if len <= 125 {
            frame.push(0x80 | len as u8); // MASK=1
        } else {
            frame.push(0xFE); // MASK=1, 126
            frame.extend_from_slice(&(len as u16).to_be_bytes());
        }
        frame.extend_from_slice(&mask_key);
        frame.extend_from_slice(&payload);

        stream.write_all(&frame).unwrap();
        stream.flush().unwrap();
    }

    /// Send a masked close frame with the given status code.
    fn ws_send_close(stream: &mut TcpStream, code: u16) {
        let mask_key = [0xAA, 0xBB, 0xCC, 0xDD];
        let mut payload = code.to_be_bytes().to_vec();
        apply_mask(&mut payload, &mask_key);

        let mut frame = vec![0x88u8, 0x82u8]; // FIN=1, Close, MASK=1, len=2
        frame.extend_from_slice(&mask_key);
        frame.extend_from_slice(&payload);

        stream.write_all(&frame).unwrap();
        stream.flush().unwrap();
    }

    // ── Tests ────────────────────────────────────────────────────────

    /// End-to-end: connect, send text, get echo, close cleanly.
    #[test]
    fn test_ws_server_end_to_end_echo() {
        let port = free_port();
        start_echo_server(port);

        let mut stream = ws_connect(port);

        // Send text, expect echo back
        ws_send_text(&mut stream, "Hello WebSocket");
        let frame = read_frame(&mut stream).unwrap();
        assert_eq!(frame.opcode, WsOpcode::Text);
        assert_eq!(String::from_utf8_lossy(&frame.payload), "Hello WebSocket");

        // Clean close handshake
        ws_send_close(&mut stream, 1000);
        let close = read_frame(&mut stream).unwrap();
        assert_eq!(close.opcode, WsOpcode::Close);
        let (code, _) = parse_close_payload(&close.payload);
        assert_eq!(code, 1000);
    }

    /// Lifecycle: on_connect fires on handshake, on_close fires on close.
    #[test]
    fn test_ws_server_lifecycle_callbacks() {
        let port = free_port();
        let connect_ctr: &'static AtomicU64 = Box::leak(Box::new(AtomicU64::new(0)));
        let close_ctr: &'static AtomicU64 = Box::leak(Box::new(AtomicU64::new(0)));
        start_counting_server(port, connect_ctr, close_ctr);

        // Before connect
        assert_eq!(connect_ctr.load(Ordering::SeqCst), 0);
        assert_eq!(close_ctr.load(Ordering::SeqCst), 0);

        let mut stream = ws_connect(port);
        for _ in 0..50 {
            if connect_ctr.load(Ordering::SeqCst) >= 1 {
                break;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        assert_eq!(
            connect_ctr.load(Ordering::SeqCst),
            1,
            "on_connect should fire"
        );

        // Send close -> on_close should fire
        ws_send_close(&mut stream, 1000);
        let _ = read_frame(&mut stream); // consume close echo
        for _ in 0..100 {
            if close_ctr.load(Ordering::SeqCst) >= 1 {
                break;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        assert_eq!(close_ctr.load(Ordering::SeqCst), 1, "on_close should fire");
    }

    /// Crash isolation: actor panic sends close 1011, server keeps running.
    #[test]
    fn test_ws_server_crash_sends_1011() {
        let port = free_port();
        start_crash_server(port);

        // First connection: any message triggers panic
        let mut stream = ws_connect(port);
        ws_send_text(&mut stream, "trigger crash");

        let frame = read_frame(&mut stream).unwrap();
        assert_eq!(frame.opcode, WsOpcode::Close);
        let (code, _) = parse_close_payload(&frame.payload);
        assert_eq!(code, 1011, "actor crash should send close code 1011");

        // Second connection: server should still be accepting
        std::thread::sleep(Duration::from_millis(200));
        let _stream2 = ws_connect(port); // panics if server is dead
    }

    /// Reader thread delivers multiple rapid messages in FIFO order.
    #[test]
    fn test_ws_server_reader_thread_delivers_messages() {
        let port = free_port();
        start_echo_server(port);

        let mut stream = ws_connect(port);

        // Send 5 messages rapidly
        for i in 0..5 {
            ws_send_text(&mut stream, &format!("msg-{}", i));
        }

        // All should be echoed back in FIFO order
        for i in 0..5 {
            let frame = read_frame(&mut stream).unwrap();
            assert_eq!(frame.opcode, WsOpcode::Text);
            assert_eq!(
                String::from_utf8_lossy(&frame.payload),
                format!("msg-{}", i),
                "messages should be delivered in FIFO order"
            );
        }
    }

    /// Client disconnect (TCP drop) triggers on_close and server keeps running.
    #[test]
    fn test_ws_server_client_disconnect_cleanup() {
        let port = free_port();
        let connect_ctr: &'static AtomicU64 = Box::leak(Box::new(AtomicU64::new(0)));
        let close_ctr: &'static AtomicU64 = Box::leak(Box::new(AtomicU64::new(0)));
        start_counting_server(port, connect_ctr, close_ctr);

        {
            let mut stream = ws_connect(port);
            ws_send_text(&mut stream, "hello");
            let _ = read_frame(&mut stream).unwrap(); // consume echo
                                                      // stream dropped -> TCP FIN, simulating client disconnect
        }

        // Wait for reader thread to detect disconnect and on_close to fire
        std::thread::sleep(Duration::from_secs(2));
        assert!(
            close_ctr.load(Ordering::SeqCst) >= 1,
            "on_close should fire on client disconnect"
        );

        // Server should still accept new connections
        let _stream2 = ws_connect(port);
    }
}
