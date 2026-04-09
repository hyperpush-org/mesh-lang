use std::collections::VecDeque;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{mpsc, Arc, OnceLock};
use std::time::Duration;

use parking_lot::RwLock;

use super::continuity::{
    continuity_registry, decode_record_payload, encode_record_payload, ContinuityAuthorityStatus,
    ContinuityRecord, ContinuityRegistry,
};
use super::node::{
    execute_transient_operator_query, node_state, write_msg, NodeSession, DIST_OPERATOR_QUERY,
    DIST_OPERATOR_REPLY,
};

pub const DEFAULT_OPERATOR_QUERY_TIMEOUT: Duration = Duration::from_secs(5);
const DEFAULT_DIAGNOSTIC_CAPACITY: usize = 128;
const QUERY_KIND_STATUS: u8 = 0;
const QUERY_KIND_CONTINUITY_LOOKUP: u8 = 1;
const QUERY_KIND_CONTINUITY_LIST: u8 = 2;
const QUERY_KIND_DIAGNOSTICS: u8 = 3;
const REPLY_STATUS_OK: u8 = 0;
const REPLY_STATUS_ERR: u8 = 1;

static OPERATOR_QUERY_REQUEST_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperatorMembershipSnapshot {
    pub local_node: String,
    pub peer_nodes: Vec<String>,
    pub nodes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperatorAuthoritySnapshot {
    pub cluster_role: String,
    pub promotion_epoch: u64,
    pub replication_health: String,
}

impl From<ContinuityAuthorityStatus> for OperatorAuthoritySnapshot {
    fn from(value: ContinuityAuthorityStatus) -> Self {
        Self {
            cluster_role: value.cluster_role.as_str().to_string(),
            promotion_epoch: value.promotion_epoch,
            replication_health: value.replication_health.as_str().to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperatorStatusSnapshot {
    pub membership: OperatorMembershipSnapshot,
    pub authority: OperatorAuthoritySnapshot,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperatorContinuityList {
    pub records: Vec<ContinuityRecord>,
    pub total_records: usize,
    pub truncated: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OperatorDiagnosticRecord {
    pub transition: String,
    pub request_key: Option<String>,
    pub attempt_id: Option<String>,
    pub owner_node: Option<String>,
    pub replica_node: Option<String>,
    pub execution_node: Option<String>,
    pub cluster_role: Option<String>,
    pub promotion_epoch: Option<u64>,
    pub replication_health: Option<String>,
    pub replica_status: Option<String>,
    pub reason: Option<String>,
    pub metadata: Vec<(String, String)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperatorDiagnosticEntry {
    pub sequence: u64,
    pub transition: String,
    pub request_key: Option<String>,
    pub attempt_id: Option<String>,
    pub owner_node: Option<String>,
    pub replica_node: Option<String>,
    pub execution_node: Option<String>,
    pub cluster_role: Option<String>,
    pub promotion_epoch: Option<u64>,
    pub replication_health: Option<String>,
    pub replica_status: Option<String>,
    pub reason: Option<String>,
    pub metadata: Vec<(String, String)>,
}

impl OperatorDiagnosticRecord {
    fn into_entry(self, sequence: u64) -> OperatorDiagnosticEntry {
        OperatorDiagnosticEntry {
            sequence,
            transition: self.transition,
            request_key: self.request_key,
            attempt_id: self.attempt_id,
            owner_node: self.owner_node,
            replica_node: self.replica_node,
            execution_node: self.execution_node,
            cluster_role: self.cluster_role,
            promotion_epoch: self.promotion_epoch,
            replication_health: self.replication_health,
            replica_status: self.replica_status,
            reason: self.reason,
            metadata: self.metadata,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperatorDiagnosticsSnapshot {
    pub entries: Vec<OperatorDiagnosticEntry>,
    pub total_entries: usize,
    pub dropped_entries: u64,
    pub buffer_capacity: usize,
    pub truncated: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatorQueryKind {
    Status,
    ContinuityLookup,
    ContinuityList,
    Diagnostics,
}

impl OperatorQueryKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Status => "status",
            Self::ContinuityLookup => "continuity_lookup",
            Self::ContinuityList => "continuity_list",
            Self::Diagnostics => "diagnostics",
        }
    }

    fn to_wire(self) -> u8 {
        match self {
            Self::Status => QUERY_KIND_STATUS,
            Self::ContinuityLookup => QUERY_KIND_CONTINUITY_LOOKUP,
            Self::ContinuityList => QUERY_KIND_CONTINUITY_LIST,
            Self::Diagnostics => QUERY_KIND_DIAGNOSTICS,
        }
    }

    fn from_wire(value: u8) -> Result<Self, String> {
        match value {
            QUERY_KIND_STATUS => Ok(Self::Status),
            QUERY_KIND_CONTINUITY_LOOKUP => Ok(Self::ContinuityLookup),
            QUERY_KIND_CONTINUITY_LIST => Ok(Self::ContinuityList),
            QUERY_KIND_DIAGNOSTICS => Ok(Self::Diagnostics),
            other => Err(format!("invalid operator query kind {other}")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OperatorQueryError {
    InvalidRequest {
        query: OperatorQueryKind,
        reason: String,
    },
    LocalRejected {
        query: OperatorQueryKind,
        reason: String,
    },
    TargetUnavailable {
        target: String,
        query: OperatorQueryKind,
        reason: String,
    },
    Timeout {
        target: String,
        query: OperatorQueryKind,
        timeout: Duration,
    },
    RemoteRejected {
        target: String,
        query: OperatorQueryKind,
        reason: String,
    },
    Decode {
        target: String,
        query: OperatorQueryKind,
        reason: String,
    },
}

impl fmt::Display for OperatorQueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequest { query, reason } => {
                write!(f, "operator query {} invalid: {}", query.as_str(), reason)
            }
            Self::LocalRejected { query, reason } => {
                write!(
                    f,
                    "local operator query {} rejected: {}",
                    query.as_str(),
                    reason
                )
            }
            Self::TargetUnavailable {
                target,
                query,
                reason,
            } => write!(
                f,
                "operator query {} target {} unavailable: {}",
                query.as_str(),
                target,
                reason
            ),
            Self::Timeout {
                target,
                query,
                timeout,
            } => write!(
                f,
                "operator query {} target {} timed out after {}ms",
                query.as_str(),
                target,
                timeout.as_millis()
            ),
            Self::RemoteRejected {
                target,
                query,
                reason,
            } => write!(
                f,
                "operator query {} target {} rejected: {}",
                query.as_str(),
                target,
                reason
            ),
            Self::Decode {
                target,
                query,
                reason,
            } => write!(
                f,
                "operator query {} target {} decode failed: {}",
                query.as_str(),
                target,
                reason
            ),
        }
    }
}

impl std::error::Error for OperatorQueryError {}

#[derive(Default)]
struct OperatorDiagnosticsInner {
    next_sequence: u64,
    dropped_entries: u64,
    entries: VecDeque<OperatorDiagnosticEntry>,
}

pub struct OperatorDiagnosticsBuffer {
    capacity: usize,
    inner: RwLock<OperatorDiagnosticsInner>,
}

impl OperatorDiagnosticsBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            inner: RwLock::new(OperatorDiagnosticsInner::default()),
        }
    }

    pub fn record(&self, record: OperatorDiagnosticRecord) {
        let mut inner = self.inner.write();
        inner.next_sequence = inner.next_sequence.saturating_add(1);
        let sequence = inner.next_sequence;
        if inner.entries.len() == self.capacity {
            inner.entries.pop_front();
            inner.dropped_entries = inner.dropped_entries.saturating_add(1);
        }
        inner.entries.push_back(record.into_entry(sequence));
    }

    pub fn snapshot(&self, limit: Option<usize>) -> OperatorDiagnosticsSnapshot {
        let inner = self.inner.read();
        let total_entries = inner.entries.len();
        let take = limit.unwrap_or(total_entries).min(total_entries);
        let start = total_entries.saturating_sub(take);
        let entries = inner.entries.iter().skip(start).cloned().collect();
        OperatorDiagnosticsSnapshot {
            entries,
            total_entries,
            dropped_entries: inner.dropped_entries,
            buffer_capacity: self.capacity,
            truncated: inner.dropped_entries > 0 || take < total_entries,
        }
    }

    #[cfg(test)]
    fn clear(&self) {
        *self.inner.write() = OperatorDiagnosticsInner::default();
    }
}

static OPERATOR_DIAGNOSTICS: OnceLock<OperatorDiagnosticsBuffer> = OnceLock::new();

fn diagnostics_buffer() -> &'static OperatorDiagnosticsBuffer {
    OPERATOR_DIAGNOSTICS.get_or_init(|| OperatorDiagnosticsBuffer::new(DEFAULT_DIAGNOSTIC_CAPACITY))
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum OperatorQuery {
    Status,
    ContinuityLookup { request_key: String },
    ContinuityList { limit: Option<usize> },
    Diagnostics { limit: Option<usize> },
}

impl OperatorQuery {
    fn kind(&self) -> OperatorQueryKind {
        match self {
            Self::Status => OperatorQueryKind::Status,
            Self::ContinuityLookup { .. } => OperatorQueryKind::ContinuityLookup,
            Self::ContinuityList { .. } => OperatorQueryKind::ContinuityList,
            Self::Diagnostics { .. } => OperatorQueryKind::Diagnostics,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum OperatorReply {
    Status(OperatorStatusSnapshot),
    ContinuityRecord(ContinuityRecord),
    ContinuityList(OperatorContinuityList),
    Diagnostics(OperatorDiagnosticsSnapshot),
}

pub(crate) fn record_diagnostic(record: OperatorDiagnosticRecord) {
    diagnostics_buffer().record(record);
}

pub fn operator_recent_diagnostics(limit: Option<usize>) -> OperatorDiagnosticsSnapshot {
    diagnostics_buffer().snapshot(limit)
}

pub fn operator_continuity_list(limit: Option<usize>) -> OperatorContinuityList {
    continuity_list_from_registry(continuity_registry(), limit)
}

pub fn operator_continuity_status(
    request_key: &str,
) -> Result<ContinuityRecord, OperatorQueryError> {
    if request_key.is_empty() {
        return Err(OperatorQueryError::InvalidRequest {
            query: OperatorQueryKind::ContinuityLookup,
            reason: "request_key_missing".to_string(),
        });
    }

    continuity_registry()
        .record(request_key)
        .ok_or_else(|| OperatorQueryError::LocalRejected {
            query: OperatorQueryKind::ContinuityLookup,
            reason: "request_key_not_found".to_string(),
        })
}

pub fn operator_status() -> Result<OperatorStatusSnapshot, OperatorQueryError> {
    let state = node_state().ok_or_else(|| OperatorQueryError::TargetUnavailable {
        target: "<local>".to_string(),
        query: OperatorQueryKind::Status,
        reason: "node_not_started".to_string(),
    })?;
    let peer_nodes = peer_names(state);
    Ok(status_snapshot_from_parts(
        &state.name,
        &peer_nodes,
        continuity_registry().authority_status(),
    ))
}

fn execute_transient_query(
    target: &str,
    cookie: &str,
    query: OperatorQuery,
    timeout: Duration,
) -> Result<OperatorReply, OperatorQueryError> {
    let query_kind = query.kind();
    let payload = encode_query_frame(
        OPERATOR_QUERY_REQUEST_ID.fetch_add(1, Ordering::Relaxed),
        &query,
    )
    .map_err(|reason| OperatorQueryError::InvalidRequest {
        query: query_kind,
        reason,
    })?;
    let reply =
        execute_transient_operator_query(target, cookie, &payload, timeout).map_err(|reason| {
            OperatorQueryError::TargetUnavailable {
                target: target.to_string(),
                query: query_kind,
                reason,
            }
        })?;
    let (_request_id, result) =
        decode_query_reply_frame(&reply).map_err(|reason| OperatorQueryError::Decode {
            target: target.to_string(),
            query: query_kind,
            reason,
        })?;
    match result {
        Ok(bytes) => decode_query_reply_payload(query_kind, &bytes).map_err(|reason| {
            OperatorQueryError::Decode {
                target: target.to_string(),
                query: query_kind,
                reason,
            }
        }),
        Err(reason) => Err(OperatorQueryError::RemoteRejected {
            target: target.to_string(),
            query: query_kind,
            reason,
        }),
    }
}

pub fn query_operator_status_remote(
    target: &str,
    cookie: &str,
    timeout: Duration,
) -> Result<OperatorStatusSnapshot, OperatorQueryError> {
    match execute_transient_query(target, cookie, OperatorQuery::Status, timeout)? {
        OperatorReply::Status(snapshot) => Ok(snapshot),
        _ => Err(OperatorQueryError::Decode {
            target: target.to_string(),
            query: OperatorQueryKind::Status,
            reason: "operator reply kind mismatch".to_string(),
        }),
    }
}

pub fn query_operator_continuity_status_remote(
    target: &str,
    cookie: &str,
    request_key: &str,
    timeout: Duration,
) -> Result<ContinuityRecord, OperatorQueryError> {
    if request_key.is_empty() {
        return Err(OperatorQueryError::InvalidRequest {
            query: OperatorQueryKind::ContinuityLookup,
            reason: "request_key_missing".to_string(),
        });
    }
    match execute_transient_query(
        target,
        cookie,
        OperatorQuery::ContinuityLookup {
            request_key: request_key.to_string(),
        },
        timeout,
    )? {
        OperatorReply::ContinuityRecord(record) => Ok(record),
        _ => Err(OperatorQueryError::Decode {
            target: target.to_string(),
            query: OperatorQueryKind::ContinuityLookup,
            reason: "operator reply kind mismatch".to_string(),
        }),
    }
}

pub fn query_operator_continuity_list_remote(
    target: &str,
    cookie: &str,
    limit: Option<usize>,
    timeout: Duration,
) -> Result<OperatorContinuityList, OperatorQueryError> {
    match execute_transient_query(
        target,
        cookie,
        OperatorQuery::ContinuityList { limit },
        timeout,
    )? {
        OperatorReply::ContinuityList(list) => Ok(list),
        _ => Err(OperatorQueryError::Decode {
            target: target.to_string(),
            query: OperatorQueryKind::ContinuityList,
            reason: "operator reply kind mismatch".to_string(),
        }),
    }
}

pub fn query_operator_diagnostics_remote(
    target: &str,
    cookie: &str,
    limit: Option<usize>,
    timeout: Duration,
) -> Result<OperatorDiagnosticsSnapshot, OperatorQueryError> {
    match execute_transient_query(
        target,
        cookie,
        OperatorQuery::Diagnostics { limit },
        timeout,
    )? {
        OperatorReply::Diagnostics(snapshot) => Ok(snapshot),
        _ => Err(OperatorQueryError::Decode {
            target: target.to_string(),
            query: OperatorQueryKind::Diagnostics,
            reason: "operator reply kind mismatch".to_string(),
        }),
    }
}

pub fn query_operator_status(
    target: &str,
    timeout: Duration,
) -> Result<OperatorStatusSnapshot, OperatorQueryError> {
    match execute_query(target, OperatorQuery::Status, timeout)? {
        OperatorReply::Status(snapshot) => Ok(snapshot),
        _ => Err(OperatorQueryError::Decode {
            target: target.to_string(),
            query: OperatorQueryKind::Status,
            reason: "operator reply kind mismatch".to_string(),
        }),
    }
}

pub fn query_operator_continuity_status(
    target: &str,
    request_key: &str,
    timeout: Duration,
) -> Result<ContinuityRecord, OperatorQueryError> {
    if request_key.is_empty() {
        return Err(OperatorQueryError::InvalidRequest {
            query: OperatorQueryKind::ContinuityLookup,
            reason: "request_key_missing".to_string(),
        });
    }
    match execute_query(
        target,
        OperatorQuery::ContinuityLookup {
            request_key: request_key.to_string(),
        },
        timeout,
    )? {
        OperatorReply::ContinuityRecord(record) => Ok(record),
        _ => Err(OperatorQueryError::Decode {
            target: target.to_string(),
            query: OperatorQueryKind::ContinuityLookup,
            reason: "operator reply kind mismatch".to_string(),
        }),
    }
}

pub fn query_operator_continuity_list(
    target: &str,
    limit: Option<usize>,
    timeout: Duration,
) -> Result<OperatorContinuityList, OperatorQueryError> {
    match execute_query(target, OperatorQuery::ContinuityList { limit }, timeout)? {
        OperatorReply::ContinuityList(list) => Ok(list),
        _ => Err(OperatorQueryError::Decode {
            target: target.to_string(),
            query: OperatorQueryKind::ContinuityList,
            reason: "operator reply kind mismatch".to_string(),
        }),
    }
}

pub fn query_operator_diagnostics(
    target: &str,
    limit: Option<usize>,
    timeout: Duration,
) -> Result<OperatorDiagnosticsSnapshot, OperatorQueryError> {
    match execute_query(target, OperatorQuery::Diagnostics { limit }, timeout)? {
        OperatorReply::Diagnostics(snapshot) => Ok(snapshot),
        _ => Err(OperatorQueryError::Decode {
            target: target.to_string(),
            query: OperatorQueryKind::Diagnostics,
            reason: "operator reply kind mismatch".to_string(),
        }),
    }
}

pub(crate) fn handle_operator_query_message(session: &Arc<NodeSession>, msg: &[u8]) {
    match build_query_reply_frame(msg, continuity_registry(), diagnostics_buffer()) {
        Ok(reply) => {
            let mut stream = session.stream.lock().unwrap();
            if let Err(error) = write_msg(&mut *stream, &reply) {
                eprintln!(
                    "mesh operator query: remote={} error=reply_write_failed:{}",
                    session.remote_name, error
                );
            }
        }
        Err(error) => {
            eprintln!(
                "mesh operator query: remote={} error={}",
                session.remote_name, error
            );
        }
    }
}

pub(crate) fn handle_operator_reply_message(session: &Arc<NodeSession>, msg: &[u8]) {
    match decode_query_reply_frame(msg) {
        Ok((request_id, result)) => {
            if let Some(sender) = session
                .pending_operator_queries
                .lock()
                .unwrap()
                .remove(&request_id)
            {
                let _ = sender.send(result);
            }
        }
        Err(error) => {
            eprintln!(
                "mesh operator query: remote={} error=reply_malformed:{}",
                session.remote_name, error
            );
        }
    }
}

fn execute_query(
    target: &str,
    query: OperatorQuery,
    timeout: Duration,
) -> Result<OperatorReply, OperatorQueryError> {
    let query_kind = query.kind();
    let state = node_state().ok_or_else(|| OperatorQueryError::TargetUnavailable {
        target: target.to_string(),
        query: query_kind,
        reason: "node_not_started".to_string(),
    })?;

    if state.name == target {
        return execute_local_query(
            Some(&state.name),
            &peer_names(state),
            continuity_registry(),
            diagnostics_buffer(),
            query,
        )
        .map_err(|reason| OperatorQueryError::LocalRejected {
            query: query_kind,
            reason,
        });
    }

    let session = {
        let sessions = state.sessions.read();
        sessions.get(target).cloned()
    }
    .ok_or_else(|| OperatorQueryError::TargetUnavailable {
        target: target.to_string(),
        query: query_kind,
        reason: "target_not_connected".to_string(),
    })?;

    let request_id = OPERATOR_QUERY_REQUEST_ID.fetch_add(1, Ordering::Relaxed);
    let payload = encode_query_frame(request_id, &query).map_err(|reason| {
        OperatorQueryError::InvalidRequest {
            query: query_kind,
            reason,
        }
    })?;
    let (tx, rx) = mpsc::channel();
    session
        .pending_operator_queries
        .lock()
        .unwrap()
        .insert(request_id, tx);

    {
        let mut stream = session.stream.lock().unwrap();
        if let Err(error) = write_msg(&mut *stream, &payload) {
            session
                .pending_operator_queries
                .lock()
                .unwrap()
                .remove(&request_id);
            return Err(OperatorQueryError::TargetUnavailable {
                target: target.to_string(),
                query: query_kind,
                reason: format!("query_write_failed:{error}"),
            });
        }
    }

    match rx.recv_timeout(timeout) {
        Ok(Ok(bytes)) => decode_query_reply_payload(query_kind, &bytes).map_err(|reason| {
            OperatorQueryError::Decode {
                target: target.to_string(),
                query: query_kind,
                reason,
            }
        }),
        Ok(Err(reason)) => Err(OperatorQueryError::RemoteRejected {
            target: target.to_string(),
            query: query_kind,
            reason,
        }),
        Err(mpsc::RecvTimeoutError::Timeout) => {
            session
                .pending_operator_queries
                .lock()
                .unwrap()
                .remove(&request_id);
            Err(OperatorQueryError::Timeout {
                target: target.to_string(),
                query: query_kind,
                timeout,
            })
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            session
                .pending_operator_queries
                .lock()
                .unwrap()
                .remove(&request_id);
            Err(OperatorQueryError::TargetUnavailable {
                target: target.to_string(),
                query: query_kind,
                reason: "query_disconnected".to_string(),
            })
        }
    }
}

fn peer_names(state: &super::node::NodeState) -> Vec<String> {
    state.sessions.read().keys().cloned().collect()
}

fn normalized_membership(local_node: &str, peer_nodes: &[String]) -> OperatorMembershipSnapshot {
    let mut peer_nodes: Vec<String> = peer_nodes
        .iter()
        .filter(|peer| peer.as_str() != local_node)
        .cloned()
        .collect();
    peer_nodes.sort();
    peer_nodes.dedup();

    let mut nodes = Vec::with_capacity(peer_nodes.len() + 1);
    nodes.push(local_node.to_string());
    nodes.extend(peer_nodes.iter().cloned());
    nodes.sort();
    nodes.dedup();

    OperatorMembershipSnapshot {
        local_node: local_node.to_string(),
        peer_nodes,
        nodes,
    }
}

fn status_snapshot_from_parts(
    local_node: &str,
    peer_nodes: &[String],
    authority: ContinuityAuthorityStatus,
) -> OperatorStatusSnapshot {
    OperatorStatusSnapshot {
        membership: normalized_membership(local_node, peer_nodes),
        authority: authority.into(),
    }
}

fn continuity_list_from_registry(
    registry: &ContinuityRegistry,
    limit: Option<usize>,
) -> OperatorContinuityList {
    let mut records = registry.snapshot().records;
    records.sort_by(|left, right| {
        left.request_key
            .cmp(&right.request_key)
            .then_with(|| left.attempt_id.cmp(&right.attempt_id))
    });
    let total_records = records.len();
    let take = limit.unwrap_or(total_records).min(total_records);
    let truncated = take < total_records;
    records.truncate(take);
    OperatorContinuityList {
        records,
        total_records,
        truncated,
    }
}

fn execute_local_query(
    local_node: Option<&str>,
    peer_nodes: &[String],
    registry: &ContinuityRegistry,
    diagnostics: &OperatorDiagnosticsBuffer,
    query: OperatorQuery,
) -> Result<OperatorReply, String> {
    match query {
        OperatorQuery::Status => {
            let local_node = local_node.ok_or_else(|| "node_not_started".to_string())?;
            Ok(OperatorReply::Status(status_snapshot_from_parts(
                local_node,
                peer_nodes,
                registry.authority_status(),
            )))
        }
        OperatorQuery::ContinuityLookup { request_key } => {
            if request_key.is_empty() {
                return Err("request_key_missing".to_string());
            }
            registry
                .record(&request_key)
                .map(OperatorReply::ContinuityRecord)
                .ok_or_else(|| "request_key_not_found".to_string())
        }
        OperatorQuery::ContinuityList { limit } => Ok(OperatorReply::ContinuityList(
            continuity_list_from_registry(registry, limit),
        )),
        OperatorQuery::Diagnostics { limit } => {
            Ok(OperatorReply::Diagnostics(diagnostics.snapshot(limit)))
        }
    }
}

fn encode_query_frame(request_id: u64, query: &OperatorQuery) -> Result<Vec<u8>, String> {
    let mut payload = Vec::new();
    match query {
        OperatorQuery::Status => {}
        OperatorQuery::ContinuityLookup { request_key } => {
            encode_string(&mut payload, request_key)?;
        }
        OperatorQuery::ContinuityList { limit } | OperatorQuery::Diagnostics { limit } => {
            encode_optional_limit(&mut payload, *limit)?;
        }
    }

    let mut frame = Vec::with_capacity(1 + 8 + 1 + 4 + payload.len());
    frame.push(DIST_OPERATOR_QUERY);
    frame.extend_from_slice(&request_id.to_le_bytes());
    frame.push(query.kind().to_wire());
    frame.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    frame.extend_from_slice(&payload);
    Ok(frame)
}

fn decode_query_header(data: &[u8]) -> Result<(u64, u8, &[u8]), String> {
    if data.len() < 14 {
        return Err("operator query payload too short".to_string());
    }
    if data[0] != DIST_OPERATOR_QUERY {
        return Err(format!("operator query tag mismatch {}", data[0]));
    }
    let request_id = u64::from_le_bytes(data[1..9].try_into().unwrap());
    let kind = data[9];
    let payload_len = u32::from_le_bytes(data[10..14].try_into().unwrap()) as usize;
    if data.len() != 14 + payload_len {
        return Err("operator query payload length mismatch".to_string());
    }
    Ok((request_id, kind, &data[14..]))
}

fn decode_query(kind: OperatorQueryKind, payload: &[u8]) -> Result<OperatorQuery, String> {
    let mut pos = 0;
    let query = match kind {
        OperatorQueryKind::Status => OperatorQuery::Status,
        OperatorQueryKind::ContinuityLookup => OperatorQuery::ContinuityLookup {
            request_key: decode_string(payload, &mut pos)?,
        },
        OperatorQueryKind::ContinuityList => OperatorQuery::ContinuityList {
            limit: decode_optional_limit(payload, &mut pos)?,
        },
        OperatorQueryKind::Diagnostics => OperatorQuery::Diagnostics {
            limit: decode_optional_limit(payload, &mut pos)?,
        },
    };
    if pos != payload.len() {
        return Err("operator query payload trailing bytes".to_string());
    }
    Ok(query)
}

fn encode_query_reply_frame(
    request_id: u64,
    result: Result<Vec<u8>, String>,
) -> Result<Vec<u8>, String> {
    let (status, payload) = match result {
        Ok(payload) => (REPLY_STATUS_OK, payload),
        Err(reason) => {
            let mut payload = Vec::new();
            encode_string(&mut payload, &reason)?;
            (REPLY_STATUS_ERR, payload)
        }
    };

    let mut frame = Vec::with_capacity(1 + 8 + 1 + 4 + payload.len());
    frame.push(DIST_OPERATOR_REPLY);
    frame.extend_from_slice(&request_id.to_le_bytes());
    frame.push(status);
    frame.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    frame.extend_from_slice(&payload);
    Ok(frame)
}

fn decode_query_reply_frame(data: &[u8]) -> Result<(u64, Result<Vec<u8>, String>), String> {
    if data.len() < 14 {
        return Err("operator reply payload too short".to_string());
    }
    if data[0] != DIST_OPERATOR_REPLY {
        return Err(format!("operator reply tag mismatch {}", data[0]));
    }
    let request_id = u64::from_le_bytes(data[1..9].try_into().unwrap());
    let status = data[9];
    let payload_len = u32::from_le_bytes(data[10..14].try_into().unwrap()) as usize;
    if data.len() != 14 + payload_len {
        return Err("operator reply payload length mismatch".to_string());
    }
    let payload = &data[14..];
    match status {
        REPLY_STATUS_OK => Ok((request_id, Ok(payload.to_vec()))),
        REPLY_STATUS_ERR => {
            let mut pos = 0;
            let reason = decode_string(payload, &mut pos)?;
            if pos != payload.len() {
                return Err("operator reply error payload trailing bytes".to_string());
            }
            Ok((request_id, Err(reason)))
        }
        other => Err(format!("invalid operator reply status {other}")),
    }
}

fn build_query_reply_frame(
    msg: &[u8],
    registry: &ContinuityRegistry,
    diagnostics: &OperatorDiagnosticsBuffer,
) -> Result<Vec<u8>, String> {
    let (request_id, kind, payload) = decode_query_header(msg)?;
    let kind = match OperatorQueryKind::from_wire(kind) {
        Ok(kind) => kind,
        Err(reason) => {
            return encode_query_reply_frame(request_id, Err(reason));
        }
    };
    let query = match decode_query(kind, payload) {
        Ok(query) => query,
        Err(reason) => {
            return encode_query_reply_frame(request_id, Err(reason));
        }
    };

    let local_node = node_state().map(|state| state.name.clone());
    let peer_nodes = node_state().map(peer_names).unwrap_or_default();
    let result = execute_local_query(
        local_node.as_deref(),
        &peer_nodes,
        registry,
        diagnostics,
        query,
    )
    .and_then(|reply| encode_query_reply_payload(&reply));

    encode_query_reply_frame(request_id, result)
}

fn encode_query_reply_payload(reply: &OperatorReply) -> Result<Vec<u8>, String> {
    match reply {
        OperatorReply::Status(snapshot) => encode_status_snapshot(snapshot),
        OperatorReply::ContinuityRecord(record) => encode_record_payload(record),
        OperatorReply::ContinuityList(list) => encode_continuity_list(list),
        OperatorReply::Diagnostics(snapshot) => encode_diagnostics_snapshot(snapshot),
    }
}

fn decode_query_reply_payload(
    kind: OperatorQueryKind,
    payload: &[u8],
) -> Result<OperatorReply, String> {
    match kind {
        OperatorQueryKind::Status => decode_status_snapshot(payload).map(OperatorReply::Status),
        OperatorQueryKind::ContinuityLookup => {
            decode_record_payload(payload).map(OperatorReply::ContinuityRecord)
        }
        OperatorQueryKind::ContinuityList => {
            decode_continuity_list(payload).map(OperatorReply::ContinuityList)
        }
        OperatorQueryKind::Diagnostics => {
            decode_diagnostics_snapshot(payload).map(OperatorReply::Diagnostics)
        }
    }
}

fn encode_status_snapshot(snapshot: &OperatorStatusSnapshot) -> Result<Vec<u8>, String> {
    let mut payload = Vec::new();
    encode_string(&mut payload, &snapshot.membership.local_node)?;
    encode_string_list(&mut payload, &snapshot.membership.peer_nodes)?;
    encode_string(&mut payload, &snapshot.authority.cluster_role)?;
    payload.extend_from_slice(&snapshot.authority.promotion_epoch.to_le_bytes());
    encode_string(&mut payload, &snapshot.authority.replication_health)?;
    Ok(payload)
}

fn decode_status_snapshot(payload: &[u8]) -> Result<OperatorStatusSnapshot, String> {
    let mut pos = 0;
    let local_node = decode_string(payload, &mut pos)?;
    let peer_nodes = decode_string_list(payload, &mut pos)?;
    let cluster_role = decode_string(payload, &mut pos)?;
    let promotion_epoch = decode_u64(payload, &mut pos)?;
    let replication_health = decode_string(payload, &mut pos)?;
    if pos != payload.len() {
        return Err("operator status payload trailing bytes".to_string());
    }
    Ok(OperatorStatusSnapshot {
        membership: normalized_membership(&local_node, &peer_nodes),
        authority: OperatorAuthoritySnapshot {
            cluster_role,
            promotion_epoch,
            replication_health,
        },
    })
}

fn encode_continuity_list(list: &OperatorContinuityList) -> Result<Vec<u8>, String> {
    let mut payload = Vec::new();
    payload.extend_from_slice(
        &u32_from_usize(list.total_records, "operator continuity total records")?.to_le_bytes(),
    );
    payload.push(list.truncated as u8);
    payload.extend_from_slice(
        &u32_from_usize(list.records.len(), "operator continuity record count")?.to_le_bytes(),
    );
    for record in &list.records {
        let encoded = encode_record_payload(record)?;
        payload.extend_from_slice(
            &u32_from_usize(encoded.len(), "operator continuity record payload")?.to_le_bytes(),
        );
        payload.extend_from_slice(&encoded);
    }
    Ok(payload)
}

fn decode_continuity_list(payload: &[u8]) -> Result<OperatorContinuityList, String> {
    let mut pos = 0;
    let total_records = decode_u32(payload, &mut pos)? as usize;
    let truncated = decode_bool(payload, &mut pos)?;
    let count = decode_u32(payload, &mut pos)? as usize;
    let mut records = Vec::with_capacity(count);
    for _ in 0..count {
        let record_len = decode_u32(payload, &mut pos)? as usize;
        if pos + record_len > payload.len() {
            return Err("operator continuity record payload truncated".to_string());
        }
        let record = decode_record_payload(&payload[pos..pos + record_len])?;
        pos += record_len;
        records.push(record);
    }
    if pos != payload.len() {
        return Err("operator continuity payload trailing bytes".to_string());
    }
    Ok(OperatorContinuityList {
        records,
        total_records,
        truncated,
    })
}

fn encode_diagnostics_snapshot(snapshot: &OperatorDiagnosticsSnapshot) -> Result<Vec<u8>, String> {
    let mut payload = Vec::new();
    payload.extend_from_slice(
        &u32_from_usize(snapshot.total_entries, "operator diagnostics total entries")?
            .to_le_bytes(),
    );
    payload.extend_from_slice(&snapshot.dropped_entries.to_le_bytes());
    payload.extend_from_slice(
        &u32_from_usize(
            snapshot.buffer_capacity,
            "operator diagnostics buffer capacity",
        )?
        .to_le_bytes(),
    );
    payload.push(snapshot.truncated as u8);
    payload.extend_from_slice(
        &u32_from_usize(snapshot.entries.len(), "operator diagnostics entry count")?.to_le_bytes(),
    );
    for entry in &snapshot.entries {
        encode_diagnostic_entry(&mut payload, entry)?;
    }
    Ok(payload)
}

fn decode_diagnostics_snapshot(payload: &[u8]) -> Result<OperatorDiagnosticsSnapshot, String> {
    let mut pos = 0;
    let total_entries = decode_u32(payload, &mut pos)? as usize;
    let dropped_entries = decode_u64(payload, &mut pos)?;
    let buffer_capacity = decode_u32(payload, &mut pos)? as usize;
    let truncated = decode_bool(payload, &mut pos)?;
    let count = decode_u32(payload, &mut pos)? as usize;
    let mut entries = Vec::with_capacity(count);
    for _ in 0..count {
        entries.push(decode_diagnostic_entry(payload, &mut pos)?);
    }
    if pos != payload.len() {
        return Err("operator diagnostics payload trailing bytes".to_string());
    }
    Ok(OperatorDiagnosticsSnapshot {
        entries,
        total_entries,
        dropped_entries,
        buffer_capacity,
        truncated,
    })
}

fn encode_diagnostic_entry(
    payload: &mut Vec<u8>,
    entry: &OperatorDiagnosticEntry,
) -> Result<(), String> {
    payload.extend_from_slice(&entry.sequence.to_le_bytes());
    encode_string(payload, &entry.transition)?;
    encode_optional_string(payload, entry.request_key.as_deref())?;
    encode_optional_string(payload, entry.attempt_id.as_deref())?;
    encode_optional_string(payload, entry.owner_node.as_deref())?;
    encode_optional_string(payload, entry.replica_node.as_deref())?;
    encode_optional_string(payload, entry.execution_node.as_deref())?;
    encode_optional_string(payload, entry.cluster_role.as_deref())?;
    encode_optional_u64(payload, entry.promotion_epoch)?;
    encode_optional_string(payload, entry.replication_health.as_deref())?;
    encode_optional_string(payload, entry.replica_status.as_deref())?;
    encode_optional_string(payload, entry.reason.as_deref())?;
    payload.extend_from_slice(
        &u32_from_usize(entry.metadata.len(), "operator diagnostic metadata entries")?
            .to_le_bytes(),
    );
    for (key, value) in &entry.metadata {
        encode_string(payload, key)?;
        encode_string(payload, value)?;
    }
    Ok(())
}

fn decode_diagnostic_entry(
    payload: &[u8],
    pos: &mut usize,
) -> Result<OperatorDiagnosticEntry, String> {
    let sequence = decode_u64(payload, pos)?;
    let transition = decode_string(payload, pos)?;
    let request_key = decode_optional_string(payload, pos)?;
    let attempt_id = decode_optional_string(payload, pos)?;
    let owner_node = decode_optional_string(payload, pos)?;
    let replica_node = decode_optional_string(payload, pos)?;
    let execution_node = decode_optional_string(payload, pos)?;
    let cluster_role = decode_optional_string(payload, pos)?;
    let promotion_epoch = decode_optional_u64(payload, pos)?;
    let replication_health = decode_optional_string(payload, pos)?;
    let replica_status = decode_optional_string(payload, pos)?;
    let reason = decode_optional_string(payload, pos)?;
    let metadata_count = decode_u32(payload, pos)? as usize;
    let mut metadata = Vec::with_capacity(metadata_count);
    for _ in 0..metadata_count {
        let key = decode_string(payload, pos)?;
        let value = decode_string(payload, pos)?;
        metadata.push((key, value));
    }
    Ok(OperatorDiagnosticEntry {
        sequence,
        transition,
        request_key,
        attempt_id,
        owner_node,
        replica_node,
        execution_node,
        cluster_role,
        promotion_epoch,
        replication_health,
        replica_status,
        reason,
        metadata,
    })
}

fn encode_optional_limit(payload: &mut Vec<u8>, limit: Option<usize>) -> Result<(), String> {
    match limit {
        Some(limit) => {
            payload.push(1);
            payload
                .extend_from_slice(&u32_from_usize(limit, "operator query limit")?.to_le_bytes());
        }
        None => payload.push(0),
    }
    Ok(())
}

fn decode_optional_limit(payload: &[u8], pos: &mut usize) -> Result<Option<usize>, String> {
    match decode_byte(payload, pos)? {
        0 => Ok(None),
        1 => Ok(Some(decode_u32(payload, pos)? as usize)),
        other => Err(format!("invalid operator query limit flag {other}")),
    }
}

fn encode_string_list(payload: &mut Vec<u8>, values: &[String]) -> Result<(), String> {
    payload.extend_from_slice(
        &u32_from_usize(values.len(), "operator string list length")?.to_le_bytes(),
    );
    for value in values {
        encode_string(payload, value)?;
    }
    Ok(())
}

fn decode_string_list(payload: &[u8], pos: &mut usize) -> Result<Vec<String>, String> {
    let count = decode_u32(payload, pos)? as usize;
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        values.push(decode_string(payload, pos)?);
    }
    Ok(values)
}

fn encode_string(payload: &mut Vec<u8>, value: &str) -> Result<(), String> {
    payload
        .extend_from_slice(&u32_from_usize(value.len(), "operator string length")?.to_le_bytes());
    payload.extend_from_slice(value.as_bytes());
    Ok(())
}

fn decode_string(payload: &[u8], pos: &mut usize) -> Result<String, String> {
    let len = decode_u32(payload, pos)? as usize;
    if *pos + len > payload.len() {
        return Err("operator string payload truncated".to_string());
    }
    let value = std::str::from_utf8(&payload[*pos..*pos + len])
        .map_err(|_| "operator string payload invalid UTF-8".to_string())?
        .to_string();
    *pos += len;
    Ok(value)
}

fn encode_optional_string(payload: &mut Vec<u8>, value: Option<&str>) -> Result<(), String> {
    match value {
        Some(value) => {
            payload.push(1);
            encode_string(payload, value)
        }
        None => {
            payload.push(0);
            Ok(())
        }
    }
}

fn decode_optional_string(payload: &[u8], pos: &mut usize) -> Result<Option<String>, String> {
    match decode_byte(payload, pos)? {
        0 => Ok(None),
        1 => decode_string(payload, pos).map(Some),
        other => Err(format!("invalid operator optional string flag {other}")),
    }
}

fn encode_optional_u64(payload: &mut Vec<u8>, value: Option<u64>) -> Result<(), String> {
    match value {
        Some(value) => {
            payload.push(1);
            payload.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
        None => {
            payload.push(0);
            Ok(())
        }
    }
}

fn decode_optional_u64(payload: &[u8], pos: &mut usize) -> Result<Option<u64>, String> {
    match decode_byte(payload, pos)? {
        0 => Ok(None),
        1 => decode_u64(payload, pos).map(Some),
        other => Err(format!("invalid operator optional u64 flag {other}")),
    }
}

fn decode_bool(payload: &[u8], pos: &mut usize) -> Result<bool, String> {
    match decode_byte(payload, pos)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(format!("invalid operator bool flag {other}")),
    }
}

fn decode_byte(payload: &[u8], pos: &mut usize) -> Result<u8, String> {
    if *pos >= payload.len() {
        return Err("operator payload truncated".to_string());
    }
    let value = payload[*pos];
    *pos += 1;
    Ok(value)
}

fn decode_u32(payload: &[u8], pos: &mut usize) -> Result<u32, String> {
    if *pos + 4 > payload.len() {
        return Err("operator u32 payload truncated".to_string());
    }
    let value = u32::from_le_bytes(payload[*pos..*pos + 4].try_into().unwrap());
    *pos += 4;
    Ok(value)
}

fn decode_u64(payload: &[u8], pos: &mut usize) -> Result<u64, String> {
    if *pos + 8 > payload.len() {
        return Err("operator u64 payload truncated".to_string());
    }
    let value = u64::from_le_bytes(payload[*pos..*pos + 8].try_into().unwrap());
    *pos += 8;
    Ok(value)
}

fn u32_from_usize(value: usize, label: &str) -> Result<u32, String> {
    value
        .try_into()
        .map_err(|_| format!("{label} exceeds u32 range"))
}

#[cfg(test)]
mod tests {
    use super::*;

    static OPERATOR_TEST_GUARD: std::sync::Mutex<()> = std::sync::Mutex::new(());
    static OPERATOR_QUERY_TEST_INIT: std::sync::Once = std::sync::Once::new();
    static OPERATOR_QUERY_TEST_TARGET: OnceLock<String> = OnceLock::new();
    const OPERATOR_QUERY_TEST_COOKIE: &str = "mesh-operator-query-test-cookie";

    fn fresh_registry() -> ContinuityRegistry {
        ContinuityRegistry::new()
    }

    fn unused_loopback_port() -> u16 {
        std::net::TcpListener::bind(("127.0.0.1", 0))
            .expect("bind ephemeral operator-query test port")
            .local_addr()
            .expect("operator-query test local_addr")
            .port()
    }

    fn ensure_operator_query_test_node() -> String {
        OPERATOR_QUERY_TEST_INIT.call_once(|| {
            let port = unused_loopback_port();
            let target = format!("operator-query-test@127.0.0.1:{port}");
            let start_code = crate::dist::node::mesh_node_start(
                target.as_ptr(),
                target.len() as u64,
                OPERATOR_QUERY_TEST_COOKIE.as_ptr(),
                OPERATOR_QUERY_TEST_COOKIE.len() as u64,
            );
            assert_eq!(
                start_code, 0,
                "mesh_node_start should succeed for operator query test node"
            );
            OPERATOR_QUERY_TEST_TARGET
                .set(target)
                .expect("set operator query test target once");
            std::thread::sleep(Duration::from_millis(150));
        });

        OPERATOR_QUERY_TEST_TARGET
            .get()
            .expect("operator query test node target initialized")
            .clone()
    }

    #[test]
    fn operator_query_status_includes_self_when_zero_records() {
        let registry = fresh_registry();
        let snapshot = execute_local_query(
            Some("alpha@127.0.0.1:9000"),
            &[],
            &registry,
            &OperatorDiagnosticsBuffer::new(8),
            OperatorQuery::Status,
        )
        .expect("status query should succeed");

        let OperatorReply::Status(snapshot) = snapshot else {
            panic!("expected status reply");
        };

        assert_eq!(snapshot.membership.local_node, "alpha@127.0.0.1:9000");
        assert!(snapshot.membership.peer_nodes.is_empty());
        assert_eq!(
            snapshot.membership.nodes,
            vec!["alpha@127.0.0.1:9000".to_string()]
        );
        assert_eq!(snapshot.authority.cluster_role, "primary");
        assert_eq!(snapshot.authority.promotion_epoch, 0);
        assert_eq!(snapshot.authority.replication_health, "local_only");
    }

    #[test]
    fn m047_s07_continuity_list_supports_repeated_runtime_names_without_order_assumptions() {
        let registry = fresh_registry();
        let runtime_name = "Api.Todos.handle_list_todos";
        let base_record = ContinuityRecord {
            request_key: String::new(),
            payload_hash: String::new(),
            attempt_id: String::new(),
            phase: crate::dist::continuity::ContinuityPhase::Completed,
            result: crate::dist::continuity::ContinuityResult::Succeeded,
            ingress_node: "ingress@host".to_string(),
            owner_node: "owner@host".to_string(),
            replica_node: "replica@host".to_string(),
            replication_count: 2,
            replica_status: crate::dist::continuity::ReplicaStatus::Mirrored,
            cluster_role: crate::dist::continuity::ContinuityClusterRole::Primary,
            promotion_epoch: 0,
            replication_health: crate::dist::continuity::ReplicationHealth::Healthy,
            execution_node: "owner@host".to_string(),
            routed_remotely: false,
            fell_back_locally: true,
            error: String::new(),
            declared_handler_runtime_name: runtime_name.to_string(),
        };

        let first = ContinuityRecord {
            request_key: "http-route::Api.Todos.handle_list_todos::2".to_string(),
            payload_hash: "hash-2".to_string(),
            attempt_id: "attempt-2".to_string(),
            ..base_record.clone()
        };
        let second = ContinuityRecord {
            request_key: "http-route::Api.Todos.handle_list_todos::1".to_string(),
            payload_hash: "hash-1".to_string(),
            attempt_id: "attempt-1".to_string(),
            ..base_record
        };

        registry
            .merge_remote_record(3, first.clone())
            .expect("merge first repeated-runtime record");
        registry
            .merge_remote_record(4, second.clone())
            .expect("merge second repeated-runtime record");

        let list = continuity_list_from_registry(&registry, None);
        assert_eq!(list.total_records, 2);
        assert_eq!(list.records.len(), 2);
        assert!(list.records.iter().all(|record| {
            record.declared_handler_runtime_name() == runtime_name && record.replication_count == 2
        }));

        let first_lookup = list
            .records
            .iter()
            .find(|record| record.request_key == first.request_key)
            .expect("find first request key regardless of list order");
        assert_eq!(first_lookup.attempt_id, "attempt-2");

        let second_lookup = list
            .records
            .iter()
            .find(|record| record.request_key == second.request_key)
            .expect("find second request key regardless of list order");
        assert_eq!(second_lookup.attempt_id, "attempt-1");
    }

    #[test]
    fn operator_query_transient_status_does_not_register_peer() {
        let _guard = OPERATOR_TEST_GUARD.lock().unwrap();
        let target = ensure_operator_query_test_node();
        let state = node_state().expect("operator query test node should be started");
        assert_eq!(
            state.sessions.read().len(),
            0,
            "test node should start without peers"
        );

        let snapshot = query_operator_status_remote(
            &target,
            OPERATOR_QUERY_TEST_COOKIE,
            Duration::from_secs(2),
        )
        .expect("transient operator query should succeed");

        assert_eq!(snapshot.membership.local_node, target);
        assert!(snapshot.membership.peer_nodes.is_empty());
        assert_eq!(
            snapshot.membership.nodes,
            vec![snapshot.membership.local_node.clone()]
        );
        assert_eq!(snapshot.authority.cluster_role, "primary");
        assert_eq!(snapshot.authority.promotion_epoch, 0);
        assert_eq!(snapshot.authority.replication_health, "local_only");
        assert_eq!(
            state.sessions.read().len(),
            0,
            "transient operator query must not register a visible peer session"
        );
    }

    #[test]
    fn operator_query_invalid_kind_returns_error_reply() {
        let registry = fresh_registry();
        let diagnostics = OperatorDiagnosticsBuffer::new(8);
        let mut frame = Vec::new();
        frame.push(DIST_OPERATOR_QUERY);
        frame.extend_from_slice(&7u64.to_le_bytes());
        frame.push(0xFF);
        frame.extend_from_slice(&0u32.to_le_bytes());

        let reply = build_query_reply_frame(&frame, &registry, &diagnostics)
            .expect("rejectable malformed query should still produce reply frame");
        let (request_id, result) = decode_query_reply_frame(&reply).expect("decode error reply");
        assert_eq!(request_id, 7);
        let reason = result.expect_err("invalid query kind should reject");
        assert!(
            reason.contains("invalid operator query kind"),
            "unexpected reason: {reason}"
        );
    }

    #[test]
    fn operator_query_missing_request_key_returns_error_reply() {
        let registry = fresh_registry();
        let diagnostics = OperatorDiagnosticsBuffer::new(8);
        let frame = encode_query_frame(
            9,
            &OperatorQuery::ContinuityLookup {
                request_key: String::new(),
            },
        )
        .expect("encode continuity lookup frame");

        let reply = build_query_reply_frame(&frame, &registry, &diagnostics)
            .expect("missing request key should produce error reply");
        let (request_id, result) = decode_query_reply_frame(&reply).expect("decode error reply");
        assert_eq!(request_id, 9);
        assert_eq!(
            result.expect_err("empty request key should reject"),
            "request_key_missing"
        );
    }

    #[test]
    fn operator_query_status_decode_rejects_truncated_payload() {
        let mut payload = Vec::new();
        encode_string(&mut payload, "node@127.0.0.1:9000").expect("encode local node");
        encode_string_list(&mut payload, &[]).expect("encode peers");
        encode_string(&mut payload, "primary").expect("encode role");
        // Intentionally omit promotion_epoch and replication_health.

        let err = decode_query_reply_payload(OperatorQueryKind::Status, &payload)
            .expect_err("truncated status payload should fail decode");
        assert!(
            err.contains("truncated"),
            "expected truncated payload error, got: {err}"
        );
    }

    #[test]
    fn operator_diagnostics_ring_buffer_tracks_truncation() {
        let buffer = OperatorDiagnosticsBuffer::new(2);
        buffer.record(OperatorDiagnosticRecord {
            transition: "submit".to_string(),
            request_key: Some("req-1".to_string()),
            ..OperatorDiagnosticRecord::default()
        });
        buffer.record(OperatorDiagnosticRecord {
            transition: "owner_lost".to_string(),
            request_key: Some("req-2".to_string()),
            ..OperatorDiagnosticRecord::default()
        });
        buffer.record(OperatorDiagnosticRecord {
            transition: "degraded".to_string(),
            request_key: Some("req-3".to_string()),
            ..OperatorDiagnosticRecord::default()
        });

        let snapshot = buffer.snapshot(None);
        assert_eq!(snapshot.total_entries, 2);
        assert_eq!(snapshot.dropped_entries, 1);
        assert!(snapshot.truncated);
        assert_eq!(snapshot.buffer_capacity, 2);
        assert_eq!(snapshot.entries.len(), 2);
        assert_eq!(snapshot.entries[0].transition, "owner_lost");
        assert_eq!(snapshot.entries[1].transition, "degraded");
    }

    #[test]
    fn operator_diagnostics_recent_snapshot_keeps_reason_and_metadata() {
        let _guard = OPERATOR_TEST_GUARD.lock().unwrap();
        diagnostics_buffer().clear();
        record_diagnostic(OperatorDiagnosticRecord {
            transition: "prepare_timeout".to_string(),
            request_key: Some("req-9".to_string()),
            attempt_id: Some("attempt-9".to_string()),
            replica_node: Some("replica@127.0.0.1:9001".to_string()),
            reason: Some("replica_prepare_timeout".to_string()),
            metadata: vec![("query_kind".to_string(), "diagnostics".to_string())],
            ..OperatorDiagnosticRecord::default()
        });

        let snapshot = operator_recent_diagnostics(None);
        assert_eq!(snapshot.entries.len(), 1);
        assert_eq!(snapshot.entries[0].transition, "prepare_timeout");
        assert_eq!(
            snapshot.entries[0].reason.as_deref(),
            Some("replica_prepare_timeout")
        );
        assert_eq!(
            snapshot.entries[0].metadata,
            vec![("query_kind".to_string(), "diagnostics".to_string())]
        );
        diagnostics_buffer().clear();
    }
}
