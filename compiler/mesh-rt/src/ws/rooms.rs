//! WebSocket room registry for pub/sub broadcast messaging.
//!
//! Provides a global concurrent registry mapping room names to sets of
//! connection handles. Connections can join named rooms, leave rooms, and
//! broadcast text frames to all members of a room.
//!
//! ## Architecture
//!
//! Modeled on `ProcessRegistry` (`crates/mesh-rt/src/actor/registry.rs`):
//! - `rooms: RwLock<FxHashMap<String, HashSet<usize>>>` -- room to connections
//! - `conn_rooms: RwLock<FxHashMap<usize, HashSet<String>>>` -- reverse index
//!
//! Lock ordering: always acquire `rooms` first, then `conn_rooms` (nested,
//! consistent order to prevent deadlock).
//!
//! ## Runtime Functions
//!
//! - `mesh_ws_join(conn, room)` -- subscribe connection to room
//! - `mesh_ws_leave(conn, room)` -- unsubscribe connection from room
//! - `mesh_ws_broadcast(room, msg)` -- send text frame to all in room
//! - `mesh_ws_broadcast_except(room, msg, except)` -- send to all except one

use parking_lot::RwLock;
use rustc_hash::FxHashMap;
use std::collections::HashSet;
use std::sync::atomic::Ordering;
use std::sync::OnceLock;

use super::server::WsConnection;
use super::{write_frame, WsOpcode};
use crate::string::MeshString;

// ---------------------------------------------------------------------------
// RoomRegistry
// ---------------------------------------------------------------------------

/// Global room registry for WebSocket pub/sub.
///
/// Maps room names to sets of connection handles (WsConnection pointer as
/// usize), with a reverse index for O(rooms_per_conn) cleanup on disconnect.
pub struct RoomRegistry {
    /// room_name -> set of connection handles
    rooms: RwLock<FxHashMap<String, HashSet<usize>>>,
    /// connection_handle -> set of room names (reverse index for cleanup)
    conn_rooms: RwLock<FxHashMap<usize, HashSet<String>>>,
}

impl RoomRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        RoomRegistry {
            rooms: RwLock::new(FxHashMap::default()),
            conn_rooms: RwLock::new(FxHashMap::default()),
        }
    }

    /// Subscribe a connection to a named room.
    ///
    /// Lock ordering: rooms write, then conn_rooms write.
    pub fn join(&self, conn: usize, room: String) {
        let mut rooms = self.rooms.write();
        let mut conn_rooms = self.conn_rooms.write();
        rooms.entry(room.clone()).or_default().insert(conn);
        conn_rooms.entry(conn).or_default().insert(room);
    }

    /// Unsubscribe a connection from a room.
    ///
    /// Removes the connection from the room's member set, and removes the
    /// room from the connection's room set. Empty entries are cleaned up
    /// to prevent memory leaks.
    ///
    /// Lock ordering: rooms write, then conn_rooms write.
    pub fn leave(&self, conn: usize, room: &str) {
        let mut rooms = self.rooms.write();
        let mut conn_rooms = self.conn_rooms.write();

        if let Some(members) = rooms.get_mut(room) {
            members.remove(&conn);
            if members.is_empty() {
                rooms.remove(room);
            }
        }

        if let Some(room_set) = conn_rooms.get_mut(&conn) {
            room_set.remove(room);
            if room_set.is_empty() {
                conn_rooms.remove(&conn);
            }
        }
    }

    /// Remove a connection from all rooms. Called on disconnect.
    ///
    /// Removes the connection from the reverse index, then removes it from
    /// each room's member set. Empty rooms are cleaned up.
    ///
    /// Lock ordering: rooms write, then conn_rooms write.
    pub fn cleanup_connection(&self, conn: usize) {
        let mut rooms = self.rooms.write();
        let mut conn_rooms = self.conn_rooms.write();

        if let Some(room_names) = conn_rooms.remove(&conn) {
            for room_name in room_names {
                if let Some(members) = rooms.get_mut(&room_name) {
                    members.remove(&conn);
                    if members.is_empty() {
                        rooms.remove(&room_name);
                    }
                }
            }
        }
    }

    /// Get a snapshot of all connection handles in a room.
    ///
    /// Acquires only the rooms read lock, released before the caller iterates.
    pub fn members(&self, room: &str) -> Vec<usize> {
        self.rooms
            .read()
            .get(room)
            .map(|s| s.iter().copied().collect())
            .unwrap_or_default()
    }
}

// ---------------------------------------------------------------------------
// Global instance
// ---------------------------------------------------------------------------

/// The global room registry, lazily initialized.
static GLOBAL_ROOM_REGISTRY: OnceLock<RoomRegistry> = OnceLock::new();

/// Get a reference to the global room registry.
pub fn global_room_registry() -> &'static RoomRegistry {
    GLOBAL_ROOM_REGISTRY.get_or_init(RoomRegistry::new)
}

// ---------------------------------------------------------------------------
// Cluster-aware broadcast helpers (pub(crate) for use from dist::node reader)
// ---------------------------------------------------------------------------

/// Broadcast a text frame to local room members only.
///
/// Extracts the local delivery logic from `mesh_ws_broadcast` into a reusable
/// helper. Called by the reader loop when receiving `DIST_ROOM_BROADCAST` from
/// a remote node (local-only delivery, no re-forwarding to prevent storms).
///
/// Returns the number of write failures (0 = all succeeded).
pub(crate) fn local_room_broadcast(room: &str, msg: &str) -> i64 {
    let payload = msg.as_bytes();

    // Snapshot member list (read lock, released immediately)
    let members = global_room_registry().members(room);

    let mut failures = 0i64;
    for conn_usize in members {
        let conn = unsafe { &*(conn_usize as *const WsConnection) };
        // Check shutdown flag to avoid writing to closing connections
        if conn.shutdown.load(Ordering::SeqCst) {
            continue;
        }
        let mut stream = conn.write_stream.lock();
        if write_frame(&mut *stream, WsOpcode::Text, payload, true).is_err() {
            failures += 1;
        }
    }
    failures
}

/// Forward a room broadcast to all connected cluster nodes.
///
/// Follows the collect-then-iterate pattern from `global.rs::broadcast_global_register`:
/// acquire sessions read lock, collect `Arc<NodeSession>` references, drop lock,
/// then iterate and write to each session's stream.
///
/// Returns immediately (no-op) if distribution is not started (`node_state()` is None).
pub(crate) fn broadcast_room_to_cluster(room: &str, msg: &str) {
    let state = match crate::dist::node::node_state() {
        Some(s) => s,
        None => return,
    };

    // Build payload: [tag 0x1E][u16 room_name_len][room_name][u32 msg_len][msg]
    let room_bytes = room.as_bytes();
    let msg_bytes = msg.as_bytes();
    let mut payload = Vec::with_capacity(1 + 2 + room_bytes.len() + 4 + msg_bytes.len());
    payload.push(crate::dist::node::DIST_ROOM_BROADCAST);
    payload.extend_from_slice(&(room_bytes.len() as u16).to_le_bytes());
    payload.extend_from_slice(room_bytes);
    payload.extend_from_slice(&(msg_bytes.len() as u32).to_le_bytes());
    payload.extend_from_slice(msg_bytes);

    // Collect session references, then drop sessions lock before writing.
    let sessions: Vec<std::sync::Arc<crate::dist::node::NodeSession>> = {
        let map = state.sessions.read();
        map.values().map(|s| std::sync::Arc::clone(s)).collect()
    };

    for session in &sessions {
        let mut stream = session.stream.lock().unwrap();
        let _ = crate::dist::node::write_msg(&mut *stream, &payload);
    }
}

// ---------------------------------------------------------------------------
// Runtime functions (extern "C" for Mesh codegen)
// ---------------------------------------------------------------------------

/// Subscribe a WebSocket connection to a named room.
///
/// `conn` is a pointer to a `WsConnection`. `room_name` is a pointer to a
/// `MeshString` containing the room name.
///
/// Returns 0 on success, -1 on null arguments.
#[no_mangle]
pub extern "C" fn mesh_ws_join(conn: *mut u8, room_name: *const MeshString) -> i64 {
    if conn.is_null() || room_name.is_null() {
        return -1;
    }
    // Extract room name as owned String to prevent GC dangling reference (Pitfall 4)
    let room = unsafe { (*room_name).as_str().to_string() };
    global_room_registry().join(conn as usize, room);
    0
}

/// Unsubscribe a WebSocket connection from a named room.
///
/// `conn` is a pointer to a `WsConnection`. `room_name` is a pointer to a
/// `MeshString` containing the room name.
///
/// Returns 0 on success, -1 on null arguments.
#[no_mangle]
pub extern "C" fn mesh_ws_leave(conn: *mut u8, room_name: *const MeshString) -> i64 {
    if conn.is_null() || room_name.is_null() {
        return -1;
    }
    let room = unsafe { (*room_name).as_str() };
    global_room_registry().leave(conn as usize, room);
    0
}

/// Broadcast a text frame to all connections in a named room, cluster-wide.
///
/// `room_name` and `msg` are pointers to `MeshString` values. Performs local
/// delivery first (snapshot members, write frames), then forwards the message
/// to all connected cluster nodes via `DIST_ROOM_BROADCAST`.
///
/// Returns the number of local write failures (0 = all succeeded), or -1 on
/// null arguments.
#[no_mangle]
pub extern "C" fn mesh_ws_broadcast(room_name: *const MeshString, msg: *const MeshString) -> i64 {
    if room_name.is_null() || msg.is_null() {
        return -1;
    }
    let room = unsafe { (*room_name).as_str() };
    let text = unsafe { (*msg).as_str() };

    // Step 1: Local delivery to this node's room members
    let failures = local_room_broadcast(room, text);

    // Step 2: Forward to all connected cluster nodes
    broadcast_room_to_cluster(room, text);

    failures
}

/// Broadcast a text frame to all connections in a room except one, cluster-wide.
///
/// Same as `mesh_ws_broadcast` but skips the connection at `except_conn` for
/// local delivery. The excluded connection only applies on this node (it is a
/// local pointer); remote nodes deliver to ALL their local members, which is
/// correct since the excluded connection is never on those nodes.
///
/// `except_conn` can be null (treated as no exclusion).
///
/// Returns the number of local write failures (0 = all succeeded), or -1 on
/// null room_name or msg.
#[no_mangle]
pub extern "C" fn mesh_ws_broadcast_except(
    room_name: *const MeshString,
    msg: *const MeshString,
    except_conn: *mut u8,
) -> i64 {
    if room_name.is_null() || msg.is_null() {
        return -1;
    }
    let room = unsafe { (*room_name).as_str() };
    let text = unsafe { (*msg).as_str() };
    let payload = text.as_bytes();
    let except = except_conn as usize;

    // Step 1: Local delivery with exclusion (except_conn only meaningful locally)
    let members = global_room_registry().members(room);

    let mut failures = 0i64;
    for conn_usize in members {
        if conn_usize == except {
            continue; // skip excluded connection
        }
        let conn = unsafe { &*(conn_usize as *const WsConnection) };
        // Check shutdown flag to avoid writing to closing connections
        if conn.shutdown.load(Ordering::SeqCst) {
            continue;
        }
        let mut stream = conn.write_stream.lock();
        if write_frame(&mut *stream, WsOpcode::Text, payload, true).is_err() {
            failures += 1;
        }
    }

    // Step 2: Forward full message to all connected cluster nodes
    // Remote nodes deliver to ALL their local members (no exclusion needed --
    // the excluded connection is local to this node by definition).
    broadcast_room_to_cluster(room, text);

    failures
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Create a fresh registry for testing (avoids global state interference).
    fn fresh_registry() -> RoomRegistry {
        RoomRegistry::new()
    }

    #[test]
    fn test_join_and_members() {
        let reg = fresh_registry();
        reg.join(100, "lobby".to_string());
        reg.join(200, "lobby".to_string());
        reg.join(100, "chat".to_string());

        let mut members = reg.members("lobby");
        members.sort();
        assert_eq!(members, vec![100, 200]);

        let members = reg.members("chat");
        assert_eq!(members, vec![100]);

        assert!(reg.members("nonexistent").is_empty());
    }

    #[test]
    fn test_leave() {
        let reg = fresh_registry();
        reg.join(100, "lobby".to_string());
        reg.join(200, "lobby".to_string());

        reg.leave(100, "lobby");

        let members = reg.members("lobby");
        assert_eq!(members, vec![200]);
    }

    #[test]
    fn test_leave_removes_empty_room() {
        let reg = fresh_registry();
        reg.join(100, "temp".to_string());
        reg.leave(100, "temp");

        // Room should be removed entirely
        assert!(reg.members("temp").is_empty());
        assert!(reg.rooms.read().get("temp").is_none());
    }

    #[test]
    fn test_cleanup_connection() {
        let reg = fresh_registry();
        reg.join(100, "lobby".to_string());
        reg.join(100, "chat".to_string());
        reg.join(100, "game".to_string());
        reg.join(200, "lobby".to_string());

        reg.cleanup_connection(100);

        // 100 should be gone from all rooms
        assert_eq!(reg.members("lobby"), vec![200]);
        assert!(reg.members("chat").is_empty());
        assert!(reg.members("game").is_empty());

        // Empty rooms should be cleaned up
        assert!(reg.rooms.read().get("chat").is_none());
        assert!(reg.rooms.read().get("game").is_none());

        // conn_rooms reverse index should be cleaned up
        assert!(reg.conn_rooms.read().get(&100).is_none());
    }

    #[test]
    fn test_cleanup_nonexistent_connection_is_noop() {
        let reg = fresh_registry();
        // Should not panic
        reg.cleanup_connection(999);
    }

    #[test]
    fn test_join_same_room_twice_is_idempotent() {
        let reg = fresh_registry();
        reg.join(100, "lobby".to_string());
        reg.join(100, "lobby".to_string());

        let members = reg.members("lobby");
        assert_eq!(members, vec![100]);
    }

    #[test]
    fn test_leave_nonexistent_room_is_noop() {
        let reg = fresh_registry();
        reg.join(100, "lobby".to_string());
        // Should not panic
        reg.leave(100, "nonexistent");
        // Original membership unchanged
        assert_eq!(reg.members("lobby"), vec![100]);
    }

    #[test]
    fn test_concurrent_join_leave() {
        use std::sync::Arc;

        let reg = Arc::new(fresh_registry());
        let num_threads = 8;

        let handles: Vec<_> = (0..num_threads)
            .map(|t| {
                let reg = Arc::clone(&reg);
                std::thread::spawn(move || {
                    let conn = (t + 1) * 100;
                    reg.join(conn, "shared".to_string());
                    // Verify own membership
                    let members = reg.members("shared");
                    assert!(members.contains(&conn));
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        let members = reg.members("shared");
        assert_eq!(members.len(), num_threads);
    }

    #[test]
    fn test_null_args_return_negative_one() {
        assert_eq!(mesh_ws_join(std::ptr::null_mut(), std::ptr::null()), -1);
        assert_eq!(mesh_ws_leave(std::ptr::null_mut(), std::ptr::null()), -1);
        assert_eq!(mesh_ws_broadcast(std::ptr::null(), std::ptr::null()), -1);
        assert_eq!(
            mesh_ws_broadcast_except(std::ptr::null(), std::ptr::null(), std::ptr::null_mut()),
            -1
        );
    }

    // -----------------------------------------------------------------------
    // Cluster broadcast tests (Phase 69)
    // -----------------------------------------------------------------------

    #[test]
    fn test_local_room_broadcast_empty_room() {
        // Calling local_room_broadcast on a non-existent room should return 0
        // failures and not panic. Verifies graceful handling of empty rooms.
        let failures = local_room_broadcast("empty_room_that_does_not_exist", "hello");
        assert_eq!(failures, 0);
    }

    #[test]
    fn test_broadcast_room_to_cluster_no_distribution() {
        // When node distribution is not started (node_state() returns None),
        // broadcast_room_to_cluster should return immediately without panic.
        // In test context, NODE_STATE is typically not initialized (unless
        // test_mesh_node_start_binds_listener ran first), so this tests the
        // early-return guard.
        broadcast_room_to_cluster("lobby", "hello");
        // No panic = success. The function returns () so no value to check,
        // but reaching this point means the early-return guard worked.
    }

    // -----------------------------------------------------------------------
    // DIST_ROOM_BROADCAST wire format tests (Phase 69)
    // -----------------------------------------------------------------------
    //
    // Follow the in-memory payload byte verification pattern from global.rs
    // wire format tests. No network I/O, no NODE_STATE dependency.

    #[test]
    fn test_dist_room_broadcast_wire_format() {
        use crate::dist::node::DIST_ROOM_BROADCAST;

        let room = "lobby";
        let msg = "hello world";

        // Encode: [tag 0x1E][u16 room_name_len][room_name][u32 msg_len][msg]
        let room_bytes = room.as_bytes();
        let msg_bytes = msg.as_bytes();
        let mut payload = Vec::new();
        payload.push(DIST_ROOM_BROADCAST);
        payload.extend_from_slice(&(room_bytes.len() as u16).to_le_bytes());
        payload.extend_from_slice(room_bytes);
        payload.extend_from_slice(&(msg_bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(msg_bytes);

        // Decode using the same logic as the reader loop handler.
        assert_eq!(payload[0], DIST_ROOM_BROADCAST);
        assert_eq!(payload[0], 0x1E);

        let decoded_room_len = u16::from_le_bytes(payload[1..3].try_into().unwrap()) as usize;
        assert_eq!(decoded_room_len, room.len());

        let decoded_room = std::str::from_utf8(&payload[3..3 + decoded_room_len]).unwrap();
        assert_eq!(decoded_room, room);

        let decoded_msg_len = u32::from_le_bytes(
            payload[3 + decoded_room_len..7 + decoded_room_len]
                .try_into()
                .unwrap(),
        ) as usize;
        assert_eq!(decoded_msg_len, msg.len());

        let decoded_msg = std::str::from_utf8(
            &payload[7 + decoded_room_len..7 + decoded_room_len + decoded_msg_len],
        )
        .unwrap();
        assert_eq!(decoded_msg, msg);

        // Verify total payload length matches expected.
        assert_eq!(payload.len(), 1 + 2 + room.len() + 4 + msg.len());
    }

    #[test]
    fn test_dist_room_broadcast_wire_roundtrip() {
        use crate::dist::node::DIST_ROOM_BROADCAST;

        // Test with various inputs: empty message, ASCII room, multi-byte UTF-8 room.
        let test_cases: Vec<(&str, &str)> = vec![
            ("lobby", ""),                        // empty message
            ("chat_room_42", "hello world"),      // ASCII room + message
            ("\u{1F680}rocket", "blast off!"),    // multi-byte UTF-8 room name (rocket emoji)
            ("room", "\u{00E9}\u{00E8}\u{00EA}"), // multi-byte UTF-8 message (accented chars)
        ];

        for (room, msg) in &test_cases {
            let room_bytes = room.as_bytes();
            let msg_bytes = msg.as_bytes();

            // Encode using broadcast_room_to_cluster's logic
            let mut payload = Vec::with_capacity(1 + 2 + room_bytes.len() + 4 + msg_bytes.len());
            payload.push(DIST_ROOM_BROADCAST);
            payload.extend_from_slice(&(room_bytes.len() as u16).to_le_bytes());
            payload.extend_from_slice(room_bytes);
            payload.extend_from_slice(&(msg_bytes.len() as u32).to_le_bytes());
            payload.extend_from_slice(msg_bytes);

            // Decode using reader loop logic
            assert_eq!(payload[0], DIST_ROOM_BROADCAST);

            let decoded_room_len = u16::from_le_bytes(payload[1..3].try_into().unwrap()) as usize;
            assert_eq!(decoded_room_len, room_bytes.len());

            if payload.len() >= 3 + decoded_room_len + 4 {
                let decoded_room = std::str::from_utf8(&payload[3..3 + decoded_room_len]).unwrap();
                assert_eq!(decoded_room, *room);

                let decoded_msg_len = u32::from_le_bytes(
                    payload[3 + decoded_room_len..7 + decoded_room_len]
                        .try_into()
                        .unwrap(),
                ) as usize;
                assert_eq!(decoded_msg_len, msg_bytes.len());

                if payload.len() >= 7 + decoded_room_len + decoded_msg_len {
                    let decoded_msg = std::str::from_utf8(
                        &payload[7 + decoded_room_len..7 + decoded_room_len + decoded_msg_len],
                    )
                    .unwrap();
                    assert_eq!(decoded_msg, *msg);
                } else {
                    panic!("payload too short for message body");
                }
            } else {
                panic!("payload too short for room name + msg_len header");
            }

            // Verify exact payload length.
            assert_eq!(
                payload.len(),
                1 + 2 + room_bytes.len() + 4 + msg_bytes.len(),
                "payload length mismatch for room={:?}, msg={:?}",
                room,
                msg,
            );
        }
    }
}
