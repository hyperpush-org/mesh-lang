//! HTTP server runtime for the Mesh language.
//!
//! Uses a hand-rolled HTTP/1.1 request parser and response writer with the
//! Mesh actor system for per-connection handling. Each incoming connection is
//! dispatched to a lightweight actor (corosensei coroutine on the M:N
//! scheduler) rather than an OS thread, benefiting from 64 KiB stacks and
//! crash isolation via `catch_unwind`.
//!
//! ## History
//!
//! Phase 8 used `std::thread::spawn` for per-connection handling. Phase 15
//! replaced this with actor-per-connection using the existing lightweight
//! actor system, unifying the runtime model. Phase 56-01 replaced the tiny_http
//! library with a hand-rolled HTTP/1.1 parser to eliminate a rustls 0.20
//! transitive dependency conflict with rustls 0.23 used by the rest of the
//! runtime. Phase 56-02 added TLS support via `HttpStream` enum (mirrors the
//! `PgStream` pattern from Phase 55), enabling both HTTP and HTTPS serving
//! through the same actor infrastructure. Blocking I/O is accepted (similar
//! to BEAM NIFs) since each actor runs on a scheduler worker thread.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use rustls::{ServerConfig, ServerConnection, StreamOwned};
use rustls_pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer};
use sha2::{Digest, Sha256};

use crate::actor;
use crate::collections::map;
use crate::gc::mesh_gc_alloc_actor;
use crate::string::{mesh_string_new, MeshString};

use super::router::{MeshRouter, MiddlewareEntry};

// ── Stream Abstraction ──────────────────────────────────────────────────

/// A connection stream that may be plain TCP or TLS-wrapped.
///
/// Mirrors the `PgStream` pattern from `crates/mesh-rt/src/db/pg.rs` (Phase 55).
/// Both variants implement `Read` and `Write`, enabling `parse_request` and
/// `write_response` to operate on either stream type transparently.
enum HttpStream {
    Plain(TcpStream),
    Tls(StreamOwned<ServerConnection, TcpStream>),
}

impl Read for HttpStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            HttpStream::Plain(s) => s.read(buf),
            HttpStream::Tls(s) => s.read(buf),
        }
    }
}

impl Write for HttpStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            HttpStream::Plain(s) => s.write(buf),
            HttpStream::Tls(s) => s.write(buf),
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            HttpStream::Plain(s) => s.flush(),
            HttpStream::Tls(s) => s.flush(),
        }
    }
}

// ── TLS Configuration ───────────────────────────────────────────────────

/// Build a rustls `ServerConfig` from PEM-encoded certificate and private key files.
///
/// The certificate file may contain a chain (multiple PEM blocks). The private
/// key file must contain exactly one PEM-encoded private key (RSA, ECDSA, or Ed25519).
pub(crate) fn build_server_config(
    cert_path: &str,
    key_path: &str,
) -> Result<Arc<ServerConfig>, String> {
    let certs: Vec<CertificateDer<'static>> = CertificateDer::pem_file_iter(cert_path)
        .map_err(|e| format!("open cert file: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("parse certs: {}", e))?;

    let key = PrivateKeyDer::from_pem_file(key_path).map_err(|e| format!("load key: {}", e))?;

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|e| format!("TLS config: {}", e))?;

    Ok(Arc::new(config))
}

// ── Request/Response structs ────────────────────────────────────────────

/// HTTP request representation passed to Mesh handler functions.
///
/// All fields are opaque pointers at the LLVM level. The Mesh program
/// accesses them via accessor functions (request_method, request_path, etc.).
///
/// IMPORTANT: This struct is `#[repr(C)]` -- new fields MUST be appended
/// at the end to preserve existing field offsets.
#[repr(C)]
pub struct MeshHttpRequest {
    /// HTTP method as MeshString (e.g. "GET", "POST").
    pub method: *mut u8,
    /// Request path as MeshString (e.g. "/api/users").
    pub path: *mut u8,
    /// Request body as MeshString (empty string for GET).
    pub body: *mut u8,
    /// Query parameters as MeshMap (string keys -> string values).
    pub query_params: *mut u8,
    /// Headers as MeshMap (string keys -> string values).
    pub headers: *mut u8,
    /// Path parameters as MeshMap (string keys -> string values).
    /// Populated by the router when matching parameterized routes.
    pub path_params: *mut u8,
}

/// HTTP response returned by Mesh handler functions.
///
/// IMPORTANT: This struct is `#[repr(C)]` -- new fields MUST be appended
/// at the end to preserve existing field offsets.
#[repr(C)]
pub struct MeshHttpResponse {
    /// HTTP status code (e.g. 200, 404).
    pub status: i64,
    /// Response body as MeshString.
    pub body: *mut u8,
    /// Optional response headers as MeshMap (string keys -> string values).
    /// Null when no custom headers are set (backward compatible).
    pub headers: *mut u8,
}

// ── Response constructor ───────────────────────────────────────────────

/// Create a new HTTP response with the given status code and body.
/// Headers are set to null (no custom headers).
#[no_mangle]
pub extern "C" fn mesh_http_response_new(status: i64, body: *const MeshString) -> *mut u8 {
    unsafe {
        let ptr = mesh_gc_alloc_actor(
            std::mem::size_of::<MeshHttpResponse>() as u64,
            std::mem::align_of::<MeshHttpResponse>() as u64,
        ) as *mut MeshHttpResponse;
        (*ptr).status = status;
        (*ptr).body = body as *mut u8;
        (*ptr).headers = std::ptr::null_mut();
        ptr as *mut u8
    }
}

/// Create a new HTTP response with status, body, and custom headers.
///
/// The `headers` parameter is a MeshMap pointer (string keys -> string values).
/// These headers are emitted in the HTTP response alongside the standard headers.
#[no_mangle]
pub extern "C" fn mesh_http_response_with_headers(
    status: i64,
    body: *const MeshString,
    headers: *mut u8,
) -> *mut u8 {
    unsafe {
        let ptr = mesh_gc_alloc_actor(
            std::mem::size_of::<MeshHttpResponse>() as u64,
            std::mem::align_of::<MeshHttpResponse>() as u64,
        ) as *mut MeshHttpResponse;
        (*ptr).status = status;
        (*ptr).body = body as *mut u8;
        (*ptr).headers = headers;
        ptr as *mut u8
    }
}

const CLUSTERED_ROUTE_FAILURE_STATUS: i64 = 503;
const CLUSTERED_ROUTE_REQUEST_KEY_HEADER: &str = "X-Mesh-Continuity-Request-Key";
static CLUSTERED_HTTP_ROUTE_REQUEST_SEQUENCE: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Debug, PartialEq, Eq)]
struct TransportHttpRequest {
    method: String,
    path: String,
    body: String,
    query_params: Vec<(String, String)>,
    headers: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TransportHttpResponse {
    status: i64,
    body: String,
    headers: Vec<(String, String)>,
}

fn mesh_string_ptr_to_owned(ptr: *mut u8) -> String {
    if ptr.is_null() {
        String::new()
    } else {
        unsafe { (*(ptr as *const MeshString)).as_str().to_string() }
    }
}

fn mesh_string_to_ptr(value: &str) -> *mut u8 {
    mesh_string_new(value.as_ptr(), value.len() as u64) as *mut u8
}

fn mesh_map_to_pairs(map_ptr: *mut u8) -> Result<Vec<(String, String)>, String> {
    if map_ptr.is_null() {
        return Ok(Vec::new());
    }

    let len = map::mesh_map_size(map_ptr);
    if len < 0 {
        return Err("mesh_http_map_size_invalid".to_string());
    }

    let mut pairs = Vec::with_capacity(len as usize);
    for index in 0..len {
        let key_ptr = map::mesh_map_entry_key(map_ptr, index) as *mut u8;
        let value_ptr = map::mesh_map_entry_value(map_ptr, index) as *mut u8;
        if key_ptr.is_null() || value_ptr.is_null() {
            return Err(format!("mesh_http_map_entry_missing:{index}"));
        }
        pairs.push((
            mesh_string_ptr_to_owned(key_ptr),
            mesh_string_ptr_to_owned(value_ptr),
        ));
    }
    Ok(pairs)
}

fn pairs_to_mesh_map(pairs: &[(String, String)]) -> *mut u8 {
    let mut map_ptr = map::mesh_map_new_typed(1);
    for (key, value) in pairs {
        let key_ptr = mesh_string_to_ptr(key);
        let value_ptr = mesh_string_to_ptr(value);
        map_ptr = map::mesh_map_put(map_ptr, key_ptr as u64, value_ptr as u64);
    }
    map_ptr
}

fn mesh_request_to_transport(request_ptr: *mut u8) -> Result<TransportHttpRequest, String> {
    if request_ptr.is_null() {
        return Err("mesh_http_request_missing".to_string());
    }

    let request = unsafe { &*(request_ptr as *const MeshHttpRequest) };
    Ok(TransportHttpRequest {
        method: mesh_string_ptr_to_owned(request.method),
        path: mesh_string_ptr_to_owned(request.path),
        body: mesh_string_ptr_to_owned(request.body),
        query_params: mesh_map_to_pairs(request.query_params)?,
        headers: mesh_map_to_pairs(request.headers)?,
        path_params: mesh_map_to_pairs(request.path_params)?,
    })
}

fn transport_request_to_mesh(request: &TransportHttpRequest) -> *mut u8 {
    unsafe {
        let req_ptr = mesh_gc_alloc_actor(
            std::mem::size_of::<MeshHttpRequest>() as u64,
            std::mem::align_of::<MeshHttpRequest>() as u64,
        ) as *mut MeshHttpRequest;
        (*req_ptr).method = mesh_string_to_ptr(&request.method);
        (*req_ptr).path = mesh_string_to_ptr(&request.path);
        (*req_ptr).body = mesh_string_to_ptr(&request.body);
        (*req_ptr).query_params = pairs_to_mesh_map(&request.query_params);
        (*req_ptr).headers = pairs_to_mesh_map(&request.headers);
        (*req_ptr).path_params = pairs_to_mesh_map(&request.path_params);
        req_ptr as *mut u8
    }
}

fn mesh_response_to_transport(response_ptr: *mut u8) -> Result<TransportHttpResponse, String> {
    if response_ptr.is_null() {
        return Err("mesh_http_response_missing".to_string());
    }

    let response = unsafe { &*(response_ptr as *const MeshHttpResponse) };
    Ok(TransportHttpResponse {
        status: response.status,
        body: mesh_string_ptr_to_owned(response.body),
        headers: mesh_map_to_pairs(response.headers)?,
    })
}

fn transport_response_to_mesh(response: &TransportHttpResponse) -> *mut u8 {
    let body = mesh_string_to_ptr(&response.body) as *const MeshString;
    if response.headers.is_empty() {
        mesh_http_response_new(response.status, body)
    } else {
        let headers = pairs_to_mesh_map(&response.headers);
        mesh_http_response_with_headers(response.status, body, headers)
    }
}

fn encode_len_prefixed_string(
    payload: &mut Vec<u8>,
    value: &str,
    label: &str,
) -> Result<(), String> {
    let len = u32::try_from(value.len())
        .map_err(|_| format!("mesh_http_transport_{}_too_large:{}", label, value.len()))?;
    payload.extend_from_slice(&len.to_le_bytes());
    payload.extend_from_slice(value.as_bytes());
    Ok(())
}

fn decode_len_prefixed_string(
    payload: &[u8],
    pos: &mut usize,
    label: &str,
) -> Result<String, String> {
    if *pos + 4 > payload.len() {
        return Err(format!("mesh_http_transport_{}_len_missing", label));
    }
    let len = u32::from_le_bytes(payload[*pos..*pos + 4].try_into().unwrap()) as usize;
    *pos += 4;
    if *pos + len > payload.len() {
        return Err(format!("mesh_http_transport_{}_truncated", label));
    }
    let value = std::str::from_utf8(&payload[*pos..*pos + len])
        .map_err(|_| format!("mesh_http_transport_{}_invalid_utf8", label))?
        .to_string();
    *pos += len;
    Ok(value)
}

fn encode_string_pairs(
    payload: &mut Vec<u8>,
    pairs: &[(String, String)],
    label: &str,
) -> Result<(), String> {
    let len = u32::try_from(pairs.len()).map_err(|_| {
        format!(
            "mesh_http_transport_{}_count_too_large:{}",
            label,
            pairs.len()
        )
    })?;
    payload.extend_from_slice(&len.to_le_bytes());
    for (index, (key, value)) in pairs.iter().enumerate() {
        encode_len_prefixed_string(payload, key, &format!("{}_key_{index}", label))?;
        encode_len_prefixed_string(payload, value, &format!("{}_value_{index}", label))?;
    }
    Ok(())
}

fn decode_string_pairs(
    payload: &[u8],
    pos: &mut usize,
    label: &str,
) -> Result<Vec<(String, String)>, String> {
    if *pos + 4 > payload.len() {
        return Err(format!("mesh_http_transport_{}_count_missing", label));
    }
    let count = u32::from_le_bytes(payload[*pos..*pos + 4].try_into().unwrap()) as usize;
    *pos += 4;
    let mut pairs = Vec::with_capacity(count);
    for index in 0..count {
        let key = decode_len_prefixed_string(payload, pos, &format!("{}_key_{index}", label))?;
        let value = decode_len_prefixed_string(payload, pos, &format!("{}_value_{index}", label))?;
        pairs.push((key, value));
    }
    Ok(pairs)
}

fn encode_transport_request(request: &TransportHttpRequest) -> Result<Vec<u8>, String> {
    let mut payload = Vec::new();
    encode_len_prefixed_string(&mut payload, &request.method, "request_method")?;
    encode_len_prefixed_string(&mut payload, &request.path, "request_path")?;
    encode_len_prefixed_string(&mut payload, &request.body, "request_body")?;
    encode_string_pairs(&mut payload, &request.query_params, "request_query_params")?;
    encode_string_pairs(&mut payload, &request.headers, "request_headers")?;
    encode_string_pairs(&mut payload, &request.path_params, "request_path_params")?;
    Ok(payload)
}

fn decode_transport_request(payload: &[u8]) -> Result<TransportHttpRequest, String> {
    if payload.is_empty() {
        return Err("mesh_http_transport_request_empty".to_string());
    }
    let mut pos = 0usize;
    let request = TransportHttpRequest {
        method: decode_len_prefixed_string(payload, &mut pos, "request_method")?,
        path: decode_len_prefixed_string(payload, &mut pos, "request_path")?,
        body: decode_len_prefixed_string(payload, &mut pos, "request_body")?,
        query_params: decode_string_pairs(payload, &mut pos, "request_query_params")?,
        headers: decode_string_pairs(payload, &mut pos, "request_headers")?,
        path_params: decode_string_pairs(payload, &mut pos, "request_path_params")?,
    };
    if pos != payload.len() {
        return Err("mesh_http_transport_request_trailing_bytes".to_string());
    }
    Ok(request)
}

fn encode_transport_response(response: &TransportHttpResponse) -> Result<Vec<u8>, String> {
    let mut payload = Vec::new();
    payload.extend_from_slice(&response.status.to_le_bytes());
    encode_len_prefixed_string(&mut payload, &response.body, "response_body")?;
    encode_string_pairs(&mut payload, &response.headers, "response_headers")?;
    Ok(payload)
}

fn decode_transport_response(payload: &[u8]) -> Result<TransportHttpResponse, String> {
    if payload.len() < 8 {
        return Err("mesh_http_transport_response_too_short".to_string());
    }
    let mut pos = 0usize;
    let status = i64::from_le_bytes(payload[pos..pos + 8].try_into().unwrap());
    pos += 8;
    let response = TransportHttpResponse {
        status,
        body: decode_len_prefixed_string(payload, &mut pos, "response_body")?,
        headers: decode_string_pairs(payload, &mut pos, "response_headers")?,
    };
    if pos != payload.len() {
        return Err("mesh_http_transport_response_trailing_bytes".to_string());
    }
    Ok(response)
}

pub(crate) fn encode_http_request_payload(request_ptr: *mut u8) -> Result<Vec<u8>, String> {
    encode_transport_request(&mesh_request_to_transport(request_ptr)?)
}

pub(crate) fn decode_http_request_payload(payload: &[u8]) -> Result<*mut u8, String> {
    let request = decode_transport_request(payload)?;
    Ok(transport_request_to_mesh(&request))
}

pub(crate) fn encode_http_response_payload(response_ptr: *mut u8) -> Result<Vec<u8>, String> {
    encode_transport_response(&mesh_response_to_transport(response_ptr)?)
}

pub(crate) fn decode_http_response_payload(payload: &[u8]) -> Result<*mut u8, String> {
    let response = decode_transport_response(payload)?;
    Ok(transport_response_to_mesh(&response))
}

pub(crate) fn invoke_route_handler_from_payload(
    fn_ptr: *mut u8,
    request_payload: &[u8],
) -> Result<Vec<u8>, String> {
    let request_ptr = decode_http_request_payload(request_payload)
        .map_err(|reason| format!("clustered_route_request_decode_failed:{reason}"))?;
    let response_ptr = call_handler(fn_ptr, std::ptr::null_mut(), request_ptr);
    encode_http_response_payload(response_ptr)
        .map_err(|reason| format!("clustered_route_response_encode_failed:{reason}"))
}

pub(crate) fn build_clustered_http_route_identity(
    runtime_name: &str,
    request_payload: &[u8],
) -> Result<(String, String), String> {
    let runtime_name = runtime_name.trim();
    if runtime_name.is_empty() {
        return Err("clustered_route_runtime_name_missing".to_string());
    }
    if request_payload.is_empty() {
        return Err("clustered_route_request_payload_missing".to_string());
    }

    let digest = Sha256::digest(request_payload);
    let payload_hash = digest
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();
    if payload_hash.is_empty() {
        return Err("payload_hash_missing".to_string());
    }

    let request_id = CLUSTERED_HTTP_ROUTE_REQUEST_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let request_key = format!("http-route::{runtime_name}::{request_id}");
    if request_key.is_empty() {
        return Err("request_key_missing".to_string());
    }

    Ok((request_key, payload_hash))
}

fn escape_json_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn set_response_header(response_ptr: *mut u8, name: &str, value: &str) -> *mut u8 {
    if response_ptr.is_null() || name.is_empty() || value.is_empty() {
        return response_ptr;
    }

    unsafe {
        let response = &mut *(response_ptr as *mut MeshHttpResponse);
        let headers = if response.headers.is_null() {
            map::mesh_map_new_typed(1)
        } else {
            response.headers
        };
        response.headers = map::mesh_map_put(
            headers,
            mesh_string_to_ptr(name) as u64,
            mesh_string_to_ptr(value) as u64,
        );
    }

    response_ptr
}

fn attach_clustered_route_request_key_header(response_ptr: *mut u8, request_key: &str) -> *mut u8 {
    set_response_header(
        response_ptr,
        CLUSTERED_ROUTE_REQUEST_KEY_HEADER,
        request_key,
    )
}

fn clustered_route_failure_response(reason: &str, request_key: Option<&str>) -> *mut u8 {
    let body = format!("{{\"error\":\"{}\"}}", escape_json_string(reason));
    let response_ptr = mesh_http_response_new(
        CLUSTERED_ROUTE_FAILURE_STATUS,
        mesh_string_to_ptr(&body) as *const MeshString,
    );
    if let Some(request_key) = request_key.filter(|request_key| !request_key.is_empty()) {
        attach_clustered_route_request_key_header(response_ptr, request_key)
    } else {
        response_ptr
    }
}

fn clustered_route_response_from_request(runtime_name: &str, request_ptr: *mut u8) -> *mut u8 {
    let result = encode_http_request_payload(request_ptr).and_then(|request_payload| {
        let (request_key, payload_hash) =
            build_clustered_http_route_identity(runtime_name, &request_payload)?;
        let response_result = crate::dist::node::execute_clustered_http_route(
            runtime_name,
            &request_key,
            &payload_hash,
            &request_payload,
        )
        .and_then(|response_payload| decode_http_response_payload(&response_payload));
        Ok((request_key, response_result))
    });

    match result {
        Ok((request_key, Ok(response_ptr))) => {
            attach_clustered_route_request_key_header(response_ptr, &request_key)
        }
        Ok((request_key, Err(reason))) => {
            clustered_route_failure_response(&reason, Some(&request_key))
        }
        Err(reason) => clustered_route_failure_response(&reason, None),
    }
}

// ── Request accessors ──────────────────────────────────────────────────

/// Get the HTTP method from a request.
#[no_mangle]
pub extern "C" fn mesh_http_request_method(req: *mut u8) -> *mut u8 {
    unsafe { (*(req as *const MeshHttpRequest)).method }
}

/// Get the URL path from a request.
#[no_mangle]
pub extern "C" fn mesh_http_request_path(req: *mut u8) -> *mut u8 {
    unsafe { (*(req as *const MeshHttpRequest)).path }
}

/// Get the request body.
#[no_mangle]
pub extern "C" fn mesh_http_request_body(req: *mut u8) -> *mut u8 {
    unsafe { (*(req as *const MeshHttpRequest)).body }
}

/// Get the value of a request header by name. Returns MeshOption
/// (tag 0 = Some with MeshString, tag 1 = None).
#[no_mangle]
pub extern "C" fn mesh_http_request_header(req: *mut u8, name: *const MeshString) -> *mut u8 {
    unsafe {
        let request = &*(req as *const MeshHttpRequest);
        let key_str = (*name).as_str();
        // Look up in the headers map. Keys are MeshString pointers stored as u64.
        let key_mesh = mesh_string_new(key_str.as_ptr(), key_str.len() as u64);
        let val = map::mesh_map_get(request.headers, key_mesh as u64);
        if val == 0 {
            // None
            alloc_option(1, std::ptr::null_mut())
        } else {
            // Some -- val is the MeshString pointer stored as u64
            alloc_option(0, val as *mut u8)
        }
    }
}

/// Get the value of a query parameter by name. Returns MeshOption
/// (tag 0 = Some with MeshString, tag 1 = None).
#[no_mangle]
pub extern "C" fn mesh_http_request_query(req: *mut u8, name: *const MeshString) -> *mut u8 {
    unsafe {
        let request = &*(req as *const MeshHttpRequest);
        let key_str = (*name).as_str();
        let key_mesh = mesh_string_new(key_str.as_ptr(), key_str.len() as u64);
        let val = map::mesh_map_get(request.query_params, key_mesh as u64);
        if val == 0 {
            alloc_option(1, std::ptr::null_mut())
        } else {
            alloc_option(0, val as *mut u8)
        }
    }
}

/// Get the value of a path parameter by name. Returns MeshOption
/// (tag 0 = Some with MeshString, tag 1 = None).
///
/// Path parameters are extracted from parameterized route patterns
/// like `/users/:id`. For a request matching this pattern with path
/// `/users/42`, `Request.param(req, "id")` returns `Some("42")`.
#[no_mangle]
pub extern "C" fn mesh_http_request_param(req: *mut u8, name: *const MeshString) -> *mut u8 {
    unsafe {
        let request = &*(req as *const MeshHttpRequest);
        let key_str = (*name).as_str();
        let key_mesh = mesh_string_new(key_str.as_ptr(), key_str.len() as u64);
        let val = map::mesh_map_get(request.path_params, key_mesh as u64);
        if val == 0 {
            alloc_option(1, std::ptr::null_mut())
        } else {
            alloc_option(0, val as *mut u8)
        }
    }
}

// ── Option allocation helper (shared from crate::option) ────────────────

fn alloc_option(tag: u8, value: *mut u8) -> *mut u8 {
    crate::option::alloc_option(tag, value) as *mut u8
}

// ── HTTP/1.1 Request Parser ─────────────────────────────────────────────

/// Parsed HTTP/1.1 request with method, path, headers, and body.
struct ParsedRequest {
    method: String,
    path: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

/// Parse an HTTP/1.1 request from an `HttpStream` (plain TCP or TLS).
///
/// Uses `BufReader<&mut HttpStream>` so the stream can be reused for
/// writing the response after parsing completes (the BufReader borrows
/// the stream mutably, and the borrow ends when this function returns).
///
/// Limits: max 100 headers, max 8KB total header data.
fn parse_request(stream: &mut HttpStream) -> Result<ParsedRequest, String> {
    let mut reader = BufReader::new(stream);
    let mut total_header_bytes: usize = 0;

    // 1. Read request line: "GET /path HTTP/1.1\r\n"
    let mut request_line = String::new();
    reader
        .read_line(&mut request_line)
        .map_err(|e| format!("read request line: {}", e))?;
    total_header_bytes += request_line.len();

    let request_line_trimmed = request_line.trim_end();
    let parts: Vec<&str> = request_line_trimmed.splitn(3, ' ').collect();
    if parts.len() < 2 {
        return Err(format!("malformed request line: {}", request_line_trimmed));
    }
    let method = parts[0].to_string();
    let path = parts[1].to_string();

    // 2. Read headers until blank line (\r\n alone).
    let mut headers = Vec::new();
    let mut content_length: usize = 0;
    loop {
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .map_err(|e| format!("read header: {}", e))?;
        total_header_bytes += line.len();
        if total_header_bytes > 8192 {
            return Err("header section exceeds 8KB limit".to_string());
        }

        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            break; // blank line = end of headers
        }
        if headers.len() >= 100 {
            return Err("too many headers (max 100)".to_string());
        }
        if let Some((name, value)) = trimmed.split_once(':') {
            let name = name.trim().to_string();
            let value = value.trim().to_string();
            if name.eq_ignore_ascii_case("content-length") {
                content_length = value.parse().unwrap_or(0);
            }
            headers.push((name, value));
        }
    }

    // 3. Read body based on Content-Length.
    let mut body = vec![0u8; content_length];
    if content_length > 0 {
        reader
            .read_exact(&mut body)
            .map_err(|e| format!("read body: {}", e))?;
    }

    Ok(ParsedRequest {
        method,
        path,
        headers,
        body,
    })
}

// ── HTTP/1.1 Response Writer ────────────────────────────────────────────

/// Write an HTTP/1.1 response to an `HttpStream` (plain TCP or TLS).
///
/// Format: status line, Content-Type, Content-Length, Connection: close,
/// optional extra headers, blank line, body bytes.
///
/// When `extra_headers` is `Some`, each header is emitted as `{name}: {value}\r\n`
/// between the standard headers and the blank line.
fn write_response(
    stream: &mut HttpStream,
    status: u16,
    body: &[u8],
    extra_headers: Option<Vec<(String, String)>>,
) -> Result<(), String> {
    let status_text = match status {
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        204 => "No Content",
        301 => "Moved Permanently",
        302 => "Found",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        503 => "Service Unavailable",
        _ => "OK",
    };

    let mut header = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n",
        status, status_text, body.len()
    );

    if let Some(ref headers) = extra_headers {
        for (name, value) in headers {
            header.push_str(&format!("{}: {}\r\n", name, value));
        }
    }

    header.push_str("\r\n");

    stream
        .write_all(header.as_bytes())
        .map_err(|e| format!("write response header: {}", e))?;
    stream
        .write_all(body)
        .map_err(|e| format!("write response body: {}", e))?;
    stream
        .flush()
        .map_err(|e| format!("flush response: {}", e))?;
    Ok(())
}

// ── Actor-per-connection infrastructure ────────────────────────────────

/// Arguments passed to the connection handler actor via raw pointer.
#[repr(C)]
struct ConnectionArgs {
    /// Router address as usize (for Send safety across thread boundaries).
    router_addr: usize,
    /// Raw pointer to a boxed `HttpStream`, transferred as usize.
    request_ptr: usize,
}

/// Actor entry function for handling a single HTTP connection.
///
/// Receives a raw pointer to `ConnectionArgs` containing the router
/// address and a boxed `HttpStream`. Wraps the handler call in
/// `catch_unwind` for crash isolation -- a panic in one handler does
/// not affect other connections.
///
/// The read timeout is already set on the underlying TcpStream before
/// wrapping in `HttpStream` (both Plain and Tls variants). For TLS
/// connections, the actual TLS handshake happens lazily on the first
/// `read` call (via `StreamOwned`), which occurs inside this actor --
/// not in the accept loop.
extern "C" fn connection_handler_entry(args: *const u8) {
    if args.is_null() {
        return;
    }

    let args = unsafe { Box::from_raw(args as *mut ConnectionArgs) };
    let router_ptr = args.router_addr as *mut u8;
    let mut stream = unsafe { *Box::from_raw(args.request_ptr as *mut HttpStream) };

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        match parse_request(&mut stream) {
            Ok(parsed) => {
                let (status, body, headers) = process_request(router_ptr, parsed);
                let _ = write_response(&mut stream, status, &body, headers);
            }
            Err(e) => {
                eprintln!("[mesh-rt] HTTP parse error: {}", e);
            }
        }
    }));

    if let Err(panic_info) = result {
        eprintln!("[mesh-rt] HTTP handler panicked: {:?}", panic_info);
        let _ = write_response(&mut stream, 500, b"Internal Server Error", None);
    }
}

// ── Server ─────────────────────────────────────────────────────────────

/// Start an HTTP server on the given port, blocking the calling thread.
///
/// The server listens for incoming connections and dispatches each
/// request to a lightweight actor via the Mesh actor scheduler. Each
/// connection handler runs as a coroutine (64 KiB stack) with crash
/// isolation via `catch_unwind` in `connection_handler_entry`.
///
/// Handler calling convention (same as closures in collections):
/// - If handler_env is null: `fn(request_ptr) -> response_ptr`
/// - If handler_env is non-null: `fn(handler_env, request_ptr) -> response_ptr`
#[no_mangle]
pub extern "C" fn mesh_http_serve(router: *mut u8, port: i64) {
    // Ensure the actor scheduler is initialized (idempotent).
    crate::actor::mesh_rt_init_actor(0);

    let addr = format!("[::]:{}", port);
    let listener = match std::net::TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("[mesh-rt] Failed to start HTTP server on {}: {}", addr, e);
            return;
        }
    };

    eprintln!("[mesh-rt] HTTP server listening on {}", addr);
    crate::dist::node::mesh_trigger_startup_work();

    let router_addr = router as usize;

    for tcp_stream in listener.incoming() {
        let tcp_stream = match tcp_stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[mesh-rt] accept error: {}", e);
                continue;
            }
        };

        // Set read timeout BEFORE wrapping in HttpStream.
        tcp_stream
            .set_read_timeout(Some(Duration::from_secs(30)))
            .ok();

        let http_stream = HttpStream::Plain(tcp_stream);
        let stream_ptr = Box::into_raw(Box::new(http_stream)) as usize;
        let args = ConnectionArgs {
            router_addr,
            request_ptr: stream_ptr,
        };
        let args_ptr = Box::into_raw(Box::new(args)) as *const u8;
        let args_size = std::mem::size_of::<ConnectionArgs>() as u64;

        let sched = actor::global_scheduler();
        sched.spawn(
            connection_handler_entry as *const u8,
            args_ptr,
            args_size,
            1, // Normal priority
        );
    }
}

// ── HTTPS Server ────────────────────────────────────────────────────────

/// Start an HTTPS server on the given port with TLS, blocking the calling thread.
///
/// Loads PEM-encoded certificate and private key files, builds a rustls
/// `ServerConfig`, and enters the same accept loop as `mesh_http_serve`.
/// Each accepted connection is wrapped in `HttpStream::Tls` and dispatched
/// to a lightweight actor.
///
/// The TLS handshake is lazy: `StreamOwned::new()` does NO I/O. The actual
/// handshake occurs on the first `read` call inside the actor's coroutine,
/// ensuring the accept loop is never blocked by slow TLS clients.
#[no_mangle]
pub extern "C" fn mesh_http_serve_tls(
    router: *mut u8,
    port: i64,
    cert_path: *const MeshString,
    key_path: *const MeshString,
) {
    crate::actor::mesh_rt_init_actor(0);

    let cert_str = unsafe { (*cert_path).as_str() };
    let key_str = unsafe { (*key_path).as_str() };

    let tls_config = match build_server_config(cert_str, key_str) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[mesh-rt] Failed to load TLS certificates: {}", e);
            return;
        }
    };

    let addr = format!("[::]:{}", port);
    let listener = match std::net::TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("[mesh-rt] Failed to bind {}: {}", addr, e);
            return;
        }
    };

    eprintln!("[mesh-rt] HTTPS server listening on {}", addr);
    crate::dist::node::mesh_trigger_startup_work();

    let router_addr = router as usize;
    // Leak the Arc<ServerConfig> as a raw pointer for transfer into the loop.
    // The server runs forever, so this is intentional (no cleanup needed).
    let config_ptr = Arc::into_raw(tls_config) as usize;

    for tcp_stream in listener.incoming() {
        let tcp_stream = match tcp_stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[mesh-rt] accept error: {}", e);
                continue;
            }
        };

        // Set read timeout BEFORE wrapping in TLS (Pitfall 7 from research).
        tcp_stream
            .set_read_timeout(Some(Duration::from_secs(30)))
            .ok();

        // Reconstruct the Arc without dropping it (we leaked it intentionally).
        let tls_config = unsafe { Arc::from_raw(config_ptr as *const ServerConfig) };
        let conn = match ServerConnection::new(Arc::clone(&tls_config)) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[mesh-rt] TLS connection setup failed: {}", e);
                // Re-leak the Arc so it's available for the next connection.
                std::mem::forget(tls_config);
                continue;
            }
        };
        // Re-leak the Arc so it's available for the next connection.
        std::mem::forget(tls_config);

        // StreamOwned::new does NO I/O -- handshake is lazy on first read/write.
        // The actual handshake happens inside the actor when parse_request calls
        // BufReader::read_line -> HttpStream::Tls::read -> StreamOwned::read.
        let tls_stream = StreamOwned::new(conn, tcp_stream);
        let http_stream = HttpStream::Tls(tls_stream);

        let stream_ptr = Box::into_raw(Box::new(http_stream)) as usize;
        let args = ConnectionArgs {
            router_addr,
            request_ptr: stream_ptr,
        };
        let args_ptr = Box::into_raw(Box::new(args)) as *const u8;
        let args_size = std::mem::size_of::<ConnectionArgs>() as u64;

        let sched = actor::global_scheduler();
        sched.spawn(
            connection_handler_entry as *const u8,
            args_ptr,
            args_size,
            1,
        );
    }
}

// ── Middleware chain infrastructure ──────────────────────────────────

/// State for the middleware chain trampoline.
///
/// Each step in the chain creates a new ChainState with `index + 1`,
/// builds a Mesh closure wrapping `chain_next`, and calls the current
/// middleware with (request, next_closure).
struct ChainState {
    middlewares: Vec<MiddlewareEntry>,
    index: usize,
    handler_fn: *mut u8,
    handler_env: *mut u8,
    declared_handler_runtime_name: Option<String>,
}

/// Trampoline for the middleware `next` function.
///
/// This is what Mesh calls when middleware invokes `next(request)`.
/// If all middleware has been traversed, calls the route handler.
/// Otherwise, calls the next middleware with a new `next` closure.
extern "C" fn chain_next(env_ptr: *mut u8, request_ptr: *mut u8) -> *mut u8 {
    unsafe {
        let state = &*(env_ptr as *const ChainState);
        if state.index >= state.middlewares.len() {
            if let Some(runtime_name) = state.declared_handler_runtime_name.as_deref() {
                clustered_route_response_from_request(runtime_name, request_ptr)
            } else {
                call_handler(state.handler_fn, state.handler_env, request_ptr)
            }
        } else {
            let mw = &state.middlewares[state.index];
            let next_state = Box::new(ChainState {
                middlewares: state.middlewares.clone(),
                index: state.index + 1,
                handler_fn: state.handler_fn,
                handler_env: state.handler_env,
                declared_handler_runtime_name: state.declared_handler_runtime_name.clone(),
            });
            let next_env = Box::into_raw(next_state) as *mut u8;
            let next_closure = build_mesh_closure(chain_next as *mut u8, next_env);
            call_middleware(mw.fn_ptr, mw.env_ptr, request_ptr, next_closure)
        }
    }
}

/// Build a Mesh-compatible closure struct (GC-allocated).
///
/// Layout: `{ fn_ptr: *mut u8, env_ptr: *mut u8 }` -- 16 bytes, 8-byte aligned.
/// This matches Mesh's closure representation used by the codegen.
fn build_mesh_closure(fn_ptr: *mut u8, env_ptr: *mut u8) -> *mut u8 {
    unsafe {
        let closure = mesh_gc_alloc_actor(16, 8) as *mut *mut u8;
        *closure = fn_ptr;
        *closure.add(1) = env_ptr;
        closure as *mut u8
    }
}

/// Call a route handler function.
///
/// If env_ptr is null: bare function `fn(request) -> response`.
/// If non-null: closure `fn(env, request) -> response`.
fn call_handler(fn_ptr: *mut u8, env_ptr: *mut u8, request: *mut u8) -> *mut u8 {
    unsafe {
        if env_ptr.is_null() {
            let f: fn(*mut u8) -> *mut u8 = std::mem::transmute(fn_ptr);
            f(request)
        } else {
            let f: fn(*mut u8, *mut u8) -> *mut u8 = std::mem::transmute(fn_ptr);
            f(env_ptr, request)
        }
    }
}

/// Call a middleware function.
///
/// Mesh compiles middleware with signature `fn(request: ptr, next: {ptr, ptr}) -> ptr`.
/// The `next` parameter is a closure struct `{fn_ptr, env_ptr}` which LLVM's calling
/// convention decomposes into two separate register-passed arguments. So the actual
/// ABI signature is `fn(request, next_fn_ptr, next_env_ptr) -> response`.
///
/// If env_ptr (middleware's own env) is non-null, it's a closure middleware:
/// `fn(env, request, next_fn_ptr, next_env_ptr) -> response`.
fn call_middleware(
    fn_ptr: *mut u8,
    env_ptr: *mut u8,
    request: *mut u8,
    next_closure: *mut u8,
) -> *mut u8 {
    unsafe {
        // Dereference the next_closure pointer to extract fn_ptr and env_ptr fields.
        // The closure struct layout is { fn_ptr: *mut u8, env_ptr: *mut u8 } -- 16 bytes.
        let next_fn_ptr = *(next_closure as *const *mut u8);
        let next_env_ptr = *(next_closure as *const *mut u8).add(1);

        if env_ptr.is_null() {
            let f: fn(*mut u8, *mut u8, *mut u8) -> *mut u8 = std::mem::transmute(fn_ptr);
            f(request, next_fn_ptr, next_env_ptr)
        } else {
            let f: fn(*mut u8, *mut u8, *mut u8, *mut u8) -> *mut u8 = std::mem::transmute(fn_ptr);
            f(env_ptr, request, next_fn_ptr, next_env_ptr)
        }
    }
}

/// Process a single HTTP request by matching it against the router
/// and calling the appropriate handler function.
///
/// Returns `(status_code, body_bytes, optional_extra_headers)` for the response.
fn process_request(
    router_ptr: *mut u8,
    parsed: ParsedRequest,
) -> (u16, Vec<u8>, Option<Vec<(String, String)>>) {
    unsafe {
        let router = &*(router_ptr as *const MeshRouter);

        // Build the MeshHttpRequest.
        let method_str = parsed.method;
        let method = mesh_string_new(method_str.as_ptr(), method_str.len() as u64) as *mut u8;

        let url = parsed.path;
        // Split URL into path and query string.
        let (path_str, query_str) = match url.find('?') {
            Some(idx) => (&url[..idx], &url[idx + 1..]),
            None => (url.as_str(), ""),
        };
        let path = mesh_string_new(path_str.as_ptr(), path_str.len() as u64) as *mut u8;

        // Body from parsed request.
        let body_bytes = parsed.body;
        let body = mesh_string_new(body_bytes.as_ptr(), body_bytes.len() as u64) as *mut u8;

        // Parse query params into a MeshMap (string keys for content-based lookup).
        let mut query_map = map::mesh_map_new_typed(1);
        if !query_str.is_empty() {
            for param in query_str.split('&') {
                if let Some((k, v)) = param.split_once('=') {
                    let key = mesh_string_new(k.as_ptr(), k.len() as u64);
                    let val = mesh_string_new(v.as_ptr(), v.len() as u64);
                    query_map = map::mesh_map_put(query_map, key as u64, val as u64);
                }
            }
        }

        // Parse headers into a MeshMap (string keys for content-based lookup).
        let mut headers_map = map::mesh_map_new_typed(1);
        for (name, value_str) in &parsed.headers {
            let key = mesh_string_new(name.as_ptr(), name.len() as u64);
            let val = mesh_string_new(value_str.as_ptr(), value_str.len() as u64);
            headers_map = map::mesh_map_put(headers_map, key as u64, val as u64);
        }

        // Build the request struct (needed for both matched and 404 paths when middleware is present).
        let build_mesh_request = |path_params_map: *mut u8| -> *mut u8 {
            let mesh_req = mesh_gc_alloc_actor(
                std::mem::size_of::<MeshHttpRequest>() as u64,
                std::mem::align_of::<MeshHttpRequest>() as u64,
            ) as *mut MeshHttpRequest;
            (*mesh_req).method = method;
            (*mesh_req).path = path;
            (*mesh_req).body = body;
            (*mesh_req).query_params = query_map;
            (*mesh_req).headers = headers_map;
            (*mesh_req).path_params = path_params_map;
            mesh_req as *mut u8
        };

        // Match against router (now with method and path params).
        let matched = router.match_route(path_str, &method_str);
        let has_middleware = !router.middlewares.is_empty();

        let response_ptr = if let Some((entry, params)) = matched {
            let mut path_params_map = map::mesh_map_new_typed(1);
            for (k, v) in &params {
                let key = mesh_string_new(k.as_ptr(), k.len() as u64);
                let val = mesh_string_new(v.as_ptr(), v.len() as u64);
                path_params_map = map::mesh_map_put(path_params_map, key as u64, val as u64);
            }

            let req_ptr = build_mesh_request(path_params_map);
            let clustered_runtime_name = entry.declared_handler_runtime_name.clone();

            if has_middleware {
                let state = Box::new(ChainState {
                    middlewares: router.middlewares.clone(),
                    index: 0,
                    handler_fn: entry.handler_fn,
                    handler_env: entry.handler_env,
                    declared_handler_runtime_name: clustered_runtime_name,
                });
                chain_next(Box::into_raw(state) as *mut u8, req_ptr)
            } else if let Some(runtime_name) = clustered_runtime_name.as_deref() {
                clustered_route_response_from_request(runtime_name, req_ptr)
            } else {
                call_handler(entry.handler_fn, entry.handler_env, req_ptr)
            }
        } else if has_middleware {
            let path_params_map = map::mesh_map_new_typed(1);
            let req_ptr = build_mesh_request(path_params_map);

            extern "C" fn not_found_handler(_request: *mut u8) -> *mut u8 {
                let body_text = b"Not Found";
                let body = mesh_string_new(body_text.as_ptr(), body_text.len() as u64);
                mesh_http_response_new(404, body)
            }

            let state = Box::new(ChainState {
                middlewares: router.middlewares.clone(),
                index: 0,
                handler_fn: not_found_handler as *mut u8,
                handler_env: std::ptr::null_mut(),
                declared_handler_runtime_name: None,
            });
            chain_next(Box::into_raw(state) as *mut u8, req_ptr)
        } else {
            return (404, b"Not Found".to_vec(), None);
        };

        // Extract response from the Mesh response pointer.
        let resp = &*(response_ptr as *const MeshHttpResponse);
        let status_code = resp.status as u16;
        let body_str = if resp.body.is_null() {
            ""
        } else {
            let body_mesh = &*(resp.body as *const MeshString);
            body_mesh.as_str()
        };

        // Extract custom headers from the response if present.
        let extra_headers = if resp.headers.is_null() {
            None
        } else {
            let headers_map = resp.headers;
            let len = map::mesh_map_size(headers_map) as usize;
            if len == 0 {
                None
            } else {
                let mut headers_vec = Vec::with_capacity(len);
                // Iterate the MeshMap's internal entries array.
                // Layout: [u64; 2] per entry where [0] = key, [1] = value.
                // Both are MeshString pointers cast to u64 (string-keyed map).
                let entries = (headers_map as *const u8).add(16) as *const [u64; 2];
                for i in 0..len {
                    let entry = &*entries.add(i);
                    let key_ptr = entry[0] as *const MeshString;
                    let val_ptr = entry[1] as *const MeshString;
                    let key_str = (*key_ptr).as_str().to_string();
                    let val_str = (*val_ptr).as_str().to_string();
                    headers_vec.push((key_str, val_str));
                }
                Some(headers_vec)
            }
        };

        (status_code, body_str.as_bytes().to_vec(), extra_headers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dist::continuity::{continuity_registry, ContinuityPhase, ContinuityResult};
    use crate::dist::node::{
        clear_declared_handler_registry_for_test, mesh_register_declared_handler,
    };
    use crate::gc::mesh_rt_init;
    use crate::http::router::{mesh_http_route_get, mesh_http_router};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Mutex, MutexGuard, OnceLock};

    fn http_server_test_lock() -> MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    fn reset_clustered_runtime_state() {
        clear_declared_handler_registry_for_test();
        continuity_registry().clear_for_test();
    }

    fn owned_pairs(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
        pairs
            .iter()
            .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
            .collect()
    }

    fn build_test_request(
        method: &str,
        path: &str,
        body: &str,
        query_params: &[(&str, &str)],
        headers: &[(&str, &str)],
        path_params: &[(&str, &str)],
    ) -> *mut u8 {
        unsafe {
            let req_ptr = mesh_gc_alloc_actor(
                std::mem::size_of::<MeshHttpRequest>() as u64,
                std::mem::align_of::<MeshHttpRequest>() as u64,
            ) as *mut MeshHttpRequest;
            (*req_ptr).method = mesh_string_to_ptr(method);
            (*req_ptr).path = mesh_string_to_ptr(path);
            (*req_ptr).body = mesh_string_to_ptr(body);
            (*req_ptr).query_params = pairs_to_mesh_map(&owned_pairs(query_params));
            (*req_ptr).headers = pairs_to_mesh_map(&owned_pairs(headers));
            (*req_ptr).path_params = pairs_to_mesh_map(&owned_pairs(path_params));
            req_ptr as *mut u8
        }
    }

    fn build_test_response(status: i64, body: &str, headers: &[(&str, &str)]) -> *mut u8 {
        let body_ptr = mesh_string_to_ptr(body) as *const MeshString;
        if headers.is_empty() {
            mesh_http_response_new(status, body_ptr)
        } else {
            let headers_ptr = pairs_to_mesh_map(&owned_pairs(headers));
            mesh_http_response_with_headers(status, body_ptr, headers_ptr)
        }
    }

    fn required_response_header(headers: &Option<Vec<(String, String)>>, name: &str) -> String {
        let headers = headers
            .as_ref()
            .unwrap_or_else(|| panic!("missing response headers while looking up {name}"));
        let mut matches = headers
            .iter()
            .filter(|(header_name, _)| header_name.eq_ignore_ascii_case(name));
        let value = matches
            .next()
            .unwrap_or_else(|| panic!("missing response header {name} in {headers:?}"))
            .1
            .clone();
        assert!(
            matches.next().is_none(),
            "duplicate response header {name} in {headers:?}"
        );
        assert!(
            !value.is_empty(),
            "response header {name} should not be empty"
        );
        value
    }

    extern "C" fn simple_ok_handler(_request: *mut u8) -> *mut u8 {
        build_test_response(200, "ok", &[])
    }

    static CLUSTERED_ROUTE_HANDLER_CALLS: AtomicU64 = AtomicU64::new(0);

    extern "C" fn clustered_route_test_handler(request: *mut u8) -> *mut u8 {
        CLUSTERED_ROUTE_HANDLER_CALLS.fetch_add(1, Ordering::Relaxed);
        let request = unsafe { &*(request as *const MeshHttpRequest) };
        let body = mesh_string_ptr_to_owned(request.body);
        build_test_response(
            200,
            &format!("{{\"echo\":\"{}\"}}", body),
            &[("X-Clustered", "true")],
        )
    }

    #[test]
    fn test_response_creation() {
        mesh_rt_init();
        let body = mesh_string_new(b"Hello".as_ptr(), 5);
        let resp_ptr = mesh_http_response_new(200, body);
        assert!(!resp_ptr.is_null());
        unsafe {
            let resp = &*(resp_ptr as *const MeshHttpResponse);
            assert_eq!(resp.status, 200);
            let body_str = &*(resp.body as *const MeshString);
            assert_eq!(body_str.as_str(), "Hello");
            assert!(resp.headers.is_null());
        }
    }

    #[test]
    fn test_response_with_headers() {
        mesh_rt_init();
        let resp_ptr = build_test_response(429, "{\"retry_after\":60}", &[("Retry-After", "60")]);
        assert!(!resp_ptr.is_null());
        unsafe {
            let resp = &*(resp_ptr as *const MeshHttpResponse);
            assert_eq!(resp.status, 429);
            let body_str = &*(resp.body as *const MeshString);
            assert_eq!(body_str.as_str(), "{\"retry_after\":60}");
            assert!(!resp.headers.is_null());
            assert_eq!(map::mesh_map_size(resp.headers), 1);
        }
    }

    #[test]
    fn test_request_accessors() {
        mesh_rt_init();
        let req = build_test_request("GET", "/test", "", &[], &[], &[]);

        unsafe {
            let m = mesh_http_request_method(req);
            let m_str = &*(m as *const MeshString);
            assert_eq!(m_str.as_str(), "GET");

            let p = mesh_http_request_path(req);
            let p_str = &*(p as *const MeshString);
            assert_eq!(p_str.as_str(), "/test");

            let b = mesh_http_request_body(req);
            let b_str = &*(b as *const MeshString);
            assert_eq!(b_str.as_str(), "");
        }
    }

    #[test]
    fn m047_s07_http_request_transport_roundtrip_preserves_method_body_headers_and_params() {
        mesh_rt_init();
        let request_ptr = build_test_request(
            "POST",
            "/todos/42",
            "{\"title\":\"mesh\"}",
            &[("limit", "10")],
            &[("Content-Type", "application/json")],
            &[("id", "42")],
        );

        let encoded = encode_http_request_payload(request_ptr).expect("encode request payload");
        let decoded_ptr = decode_http_request_payload(&encoded).expect("decode request payload");
        let decoded = mesh_request_to_transport(decoded_ptr).expect("decoded request");

        assert_eq!(decoded.method, "POST");
        assert_eq!(decoded.path, "/todos/42");
        assert_eq!(decoded.body, "{\"title\":\"mesh\"}");
        assert_eq!(decoded.query_params, owned_pairs(&[("limit", "10")]));
        assert_eq!(
            decoded.headers,
            owned_pairs(&[("Content-Type", "application/json")])
        );
        assert_eq!(decoded.path_params, owned_pairs(&[("id", "42")]));
    }

    #[test]
    fn m047_s07_http_response_transport_roundtrip_preserves_status_body_and_headers() {
        mesh_rt_init();
        let response_ptr = build_test_response(201, "{\"created\":true}", &[("X-Test", "yes")]);

        let encoded = encode_http_response_payload(response_ptr).expect("encode response payload");
        let decoded_ptr = decode_http_response_payload(&encoded).expect("decode response payload");
        let decoded = mesh_response_to_transport(decoded_ptr).expect("decoded response");

        assert_eq!(decoded.status, 201);
        assert_eq!(decoded.body, "{\"created\":true}");
        assert_eq!(decoded.headers, owned_pairs(&[("X-Test", "yes")]));
    }

    #[test]
    fn m047_s07_http_transport_rejects_malformed_request_and_response_payloads() {
        assert!(decode_http_request_payload(&[]).is_err());
        assert!(decode_http_response_payload(&[1, 2, 3]).is_err());
    }

    #[test]
    fn m047_s07_clustered_route_identity_rejects_empty_runtime_name_and_payload() {
        assert!(build_clustered_http_route_identity("", b"payload").is_err());
        assert!(build_clustered_http_route_identity("Api.Todos.handle", b"").is_err());
    }

    #[test]
    fn m054_s02_process_request_attaches_correlation_header_on_clustered_success_and_preserves_handler_headers(
    ) {
        let _guard = http_server_test_lock();
        mesh_rt_init();
        reset_clustered_runtime_state();
        CLUSTERED_ROUTE_HANDLER_CALLS.store(0, Ordering::Relaxed);

        let runtime_name = "Api.Todos.handle_list_todos";
        let executable_name = "__declared_route_api_todos_handle_list_todos";
        mesh_register_declared_handler(
            runtime_name.as_ptr(),
            runtime_name.len() as u64,
            executable_name.as_ptr(),
            executable_name.len() as u64,
            1,
            clustered_route_test_handler as *const u8,
        );

        let router = mesh_http_router();
        let pattern = mesh_string_new(b"/todos".as_ptr(), 6);
        let router = mesh_http_route_get(router, pattern, clustered_route_test_handler as *mut u8);

        let first_request = ParsedRequest {
            method: "GET".to_string(),
            path: "/todos".to_string(),
            headers: vec![("X-Request-Id".to_string(), "first".to_string())],
            body: b"first".to_vec(),
        };
        let second_request = ParsedRequest {
            method: "GET".to_string(),
            path: "/todos".to_string(),
            headers: vec![("X-Request-Id".to_string(), "second".to_string())],
            body: b"second".to_vec(),
        };

        let (first_status, first_body, first_headers) = process_request(router, first_request);
        let (second_status, second_body, second_headers) = process_request(router, second_request);

        assert_eq!(first_status, 200);
        assert_eq!(
            String::from_utf8(first_body).unwrap(),
            "{\"echo\":\"first\"}"
        );
        assert_eq!(second_status, 200);
        assert_eq!(
            String::from_utf8(second_body).unwrap(),
            "{\"echo\":\"second\"}"
        );
        assert_eq!(
            required_response_header(&first_headers, "X-Clustered"),
            "true"
        );
        assert_eq!(
            required_response_header(&second_headers, "X-Clustered"),
            "true"
        );

        let first_request_key =
            required_response_header(&first_headers, CLUSTERED_ROUTE_REQUEST_KEY_HEADER);
        let second_request_key =
            required_response_header(&second_headers, CLUSTERED_ROUTE_REQUEST_KEY_HEADER);
        assert!(
            first_request_key.starts_with("http-route::Api.Todos.handle_list_todos::"),
            "unexpected first request key: {first_request_key}"
        );
        assert!(
            second_request_key.starts_with("http-route::Api.Todos.handle_list_todos::"),
            "unexpected second request key: {second_request_key}"
        );
        assert_ne!(first_request_key, second_request_key);
        assert_eq!(CLUSTERED_ROUTE_HANDLER_CALLS.load(Ordering::Relaxed), 2);

        let snapshot = continuity_registry().snapshot();
        assert_eq!(snapshot.records.len(), 2);

        let first_record = snapshot
            .records
            .iter()
            .find(|record| record.request_key == first_request_key)
            .expect("first continuity record should exist");
        assert_eq!(first_record.phase, ContinuityPhase::Completed);
        assert_eq!(first_record.result, ContinuityResult::Succeeded);
        assert_eq!(first_record.declared_handler_runtime_name(), runtime_name);
        assert_eq!(first_record.replication_count, 1);

        let second_record = snapshot
            .records
            .iter()
            .find(|record| record.request_key == second_request_key)
            .expect("second continuity record should exist");
        assert_eq!(second_record.phase, ContinuityPhase::Completed);
        assert_eq!(second_record.result, ContinuityResult::Succeeded);
        assert_eq!(second_record.declared_handler_runtime_name(), runtime_name);
        assert_eq!(second_record.replication_count, 1);

        reset_clustered_runtime_state();
    }

    #[test]
    fn m054_s02_process_request_returns_503_with_correlation_header_and_durable_rejection_for_unsupported_clustered_route_count(
    ) {
        let _guard = http_server_test_lock();
        mesh_rt_init();
        reset_clustered_runtime_state();
        CLUSTERED_ROUTE_HANDLER_CALLS.store(0, Ordering::Relaxed);

        let runtime_name = "Api.Todos.handle_list_todos";
        let executable_name = "__declared_route_api_todos_handle_list_todos";
        mesh_register_declared_handler(
            runtime_name.as_ptr(),
            runtime_name.len() as u64,
            executable_name.as_ptr(),
            executable_name.len() as u64,
            3,
            clustered_route_test_handler as *const u8,
        );

        let router = mesh_http_router();
        let pattern = mesh_string_new(b"/todos".as_ptr(), 6);
        let router = mesh_http_route_get(router, pattern, clustered_route_test_handler as *mut u8);

        let (status, body, headers) = process_request(
            router,
            ParsedRequest {
                method: "GET".to_string(),
                path: "/todos".to_string(),
                headers: vec![],
                body: Vec::new(),
            },
        );

        assert_eq!(status, 503);
        let request_key = required_response_header(&headers, CLUSTERED_ROUTE_REQUEST_KEY_HEADER);
        assert!(
            request_key.starts_with("http-route::Api.Todos.handle_list_todos::"),
            "unexpected rejected request key: {request_key}"
        );
        let body = String::from_utf8(body).expect("response body utf8");
        assert!(body.contains("unsupported_replication_count:3"), "{body}");
        assert_eq!(CLUSTERED_ROUTE_HANDLER_CALLS.load(Ordering::Relaxed), 0);

        let snapshot = continuity_registry().snapshot();
        assert_eq!(snapshot.records.len(), 1);
        let record = &snapshot.records[0];
        assert_eq!(record.request_key, request_key);
        assert_eq!(record.phase, ContinuityPhase::Rejected);
        assert_eq!(record.result, ContinuityResult::Rejected);
        assert_eq!(record.error, "unsupported_replication_count:3");
        assert_eq!(record.declared_handler_runtime_name(), runtime_name);
        assert_eq!(record.replication_count, 3);

        reset_clustered_runtime_state();
    }

    #[test]
    fn m047_s07_invoke_route_handler_from_payload_executes_real_handler_boundary() {
        let _guard = http_server_test_lock();
        mesh_rt_init();
        CLUSTERED_ROUTE_HANDLER_CALLS.store(0, Ordering::Relaxed);

        let request_ptr = build_test_request(
            "POST",
            "/clustered",
            "payload",
            &[],
            &[("Content-Type", "text/plain")],
            &[],
        );
        let request_payload = encode_http_request_payload(request_ptr).expect("encode request");
        let response_payload = invoke_route_handler_from_payload(
            clustered_route_test_handler as *mut u8,
            &request_payload,
        )
        .expect("invoke clustered handler from payload");
        let response_ptr =
            decode_http_response_payload(&response_payload).expect("decode response");
        let response = mesh_response_to_transport(response_ptr).expect("transport response");

        assert_eq!(CLUSTERED_ROUTE_HANDLER_CALLS.load(Ordering::Relaxed), 1);
        assert_eq!(response.status, 200);
        assert_eq!(response.body, "{\"echo\":\"payload\"}");
        assert_eq!(response.headers, owned_pairs(&[("X-Clustered", "true")]));
    }
}
