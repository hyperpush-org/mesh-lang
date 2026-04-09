use super::m046_route_free as route_free;
use super::m049_todo_postgres_scaffold as postgres;
use serde::Serialize;
use serde_json::Value;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread::{self, JoinHandle};
use std::time::Duration;

const ACCEPT_POLL_INTERVAL: Duration = Duration::from_millis(25);
const CLIENT_IO_TIMEOUT: Duration = Duration::from_secs(10);
const BACKEND_IO_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Clone, Serialize)]
pub struct PublicIngressTarget {
    pub label: String,
    pub node_name: String,
    pub host: String,
    pub port: u16,
}

impl PublicIngressTarget {
    pub fn for_todo_runtime(label: &str, config: &postgres::TodoRuntimeConfig) -> Self {
        Self {
            label: label.to_string(),
            node_name: config.node_name.clone(),
            host: route_free::LOOPBACK_V4.to_string(),
            port: config.http_port,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PublicIngressRequestRecord {
    pub request_id: u64,
    pub method: String,
    pub path: String,
    pub request_line: String,
    pub target_label: String,
    pub target_node: String,
    pub target_host: String,
    pub target_port: u16,
    pub status_code: u16,
    pub response_status_line: String,
    pub request_raw: String,
    pub response_raw: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PublicIngressSnapshot {
    pub base_url: String,
    pub listen_host: String,
    pub listen_port: u16,
    pub request_count: usize,
    pub next_target_label: String,
    pub last_error: String,
    pub targets: Vec<PublicIngressTarget>,
    pub requests: Vec<PublicIngressRequestRecord>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RouteContinuitySummary {
    pub public_request_id: u64,
    pub public_method: String,
    pub public_path: String,
    pub public_target_label: String,
    pub public_target_node: String,
    pub public_target_port: u16,
    pub public_status_code: u16,
    pub request_key: String,
    pub attempt_id: String,
    pub runtime_name: String,
    pub replication_count: u64,
    pub ingress_node: String,
    pub owner_node: String,
    pub replica_node: String,
    pub execution_node: String,
    pub phase: String,
    pub result: String,
}

#[derive(Debug)]
struct ParsedHttpMessage {
    method: Option<String>,
    path: Option<String>,
    request_line: String,
    status_code: Option<u16>,
    status_line: String,
    raw_bytes: Vec<u8>,
}

#[derive(Debug)]
struct PublicIngressState {
    next_target_index: usize,
    requests: Vec<PublicIngressRequestRecord>,
    events: Vec<String>,
    last_error: Option<String>,
}

#[derive(Debug)]
pub struct RunningPublicIngress {
    base_url: String,
    listen_port: u16,
    targets: Vec<PublicIngressTarget>,
    stop: Arc<AtomicBool>,
    state: Arc<Mutex<PublicIngressState>>,
    join: Option<JoinHandle<()>>,
    meta_path: PathBuf,
    log_path: PathBuf,
    snapshot_path: PathBuf,
    requests_path: PathBuf,
}

pub fn start_public_ingress(
    artifacts: &Path,
    label: &str,
    targets: Vec<PublicIngressTarget>,
    first_target_index: usize,
) -> RunningPublicIngress {
    assert!(
        !targets.is_empty(),
        "public ingress requires at least one backend target"
    );
    assert!(
        first_target_index < targets.len(),
        "public ingress first_target_index={} is out of range for {} target(s)",
        first_target_index,
        targets.len()
    );

    let mut seen_labels = std::collections::HashSet::new();
    for target in &targets {
        assert!(
            !target.label.trim().is_empty(),
            "public ingress target labels must be non-empty"
        );
        assert!(
            seen_labels.insert(target.label.clone()),
            "public ingress target labels must be unique; duplicate label `{}`",
            target.label
        );
        assert!(
            !target.node_name.trim().is_empty(),
            "public ingress target `{}` must have a node_name",
            target.label
        );
        assert!(
            !target.host.trim().is_empty(),
            "public ingress target `{}` must have a host",
            target.label
        );
        assert!(
            target.port > 0,
            "public ingress target `{}` must have a positive port",
            target.label
        );
    }

    let listener = TcpListener::bind((route_free::LOOPBACK_V4, 0)).unwrap_or_else(|error| {
        panic!(
            "failed to bind public ingress listener on {}: {error}",
            route_free::LOOPBACK_V4
        )
    });
    listener
        .set_nonblocking(true)
        .expect("failed to set public ingress listener nonblocking");
    let listen_port = listener
        .local_addr()
        .expect("public ingress listener should have a local addr")
        .port();

    let stop = Arc::new(AtomicBool::new(false));
    let state = Arc::new(Mutex::new(PublicIngressState {
        next_target_index: first_target_index,
        requests: Vec::new(),
        events: vec![format!(
            "startup base_url=http://{}:{} first_target={} targets={}",
            route_free::LOOPBACK_V4,
            listen_port,
            targets[first_target_index].label,
            targets
                .iter()
                .map(|target| format!(
                    "{}:{}@{}:{}",
                    target.label, target.node_name, target.host, target.port
                ))
                .collect::<Vec<_>>()
                .join(", "),
        )],
        last_error: None,
    }));

    let base_url = format!("http://{}:{}", route_free::LOOPBACK_V4, listen_port);
    let meta_path = artifacts.join(format!("{label}.meta.json"));
    let log_path = artifacts.join(format!("{label}.log"));
    let snapshot_path = artifacts.join(format!("{label}.snapshot.json"));
    let requests_path = artifacts.join(format!("{label}.requests.json"));

    route_free::write_json_artifact(
        &meta_path,
        &serde_json::json!({
            "base_url": &base_url,
            "listen_host": route_free::LOOPBACK_V4,
            "listen_port": listen_port,
            "selection_strategy": "round_robin",
            "first_target_index": first_target_index,
            "first_target_label": &targets[first_target_index].label,
            "targets": &targets,
        }),
    );

    let thread_stop = Arc::clone(&stop);
    let thread_state = Arc::clone(&state);
    let thread_targets = targets.clone();
    let join = thread::spawn(move || {
        while !thread_stop.load(Ordering::Relaxed) {
            match listener.accept() {
                Ok((stream, _)) => handle_client_connection(stream, &thread_targets, &thread_state),
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(ACCEPT_POLL_INTERVAL);
                }
                Err(error) => {
                    let mut state = thread_state
                        .lock()
                        .expect("public ingress state mutex poisoned");
                    state.last_error = Some(format!("accept error: {error}"));
                    state.events.push(format!("accept_error error={error}"));
                    thread::sleep(ACCEPT_POLL_INTERVAL);
                }
            }
        }
    });

    RunningPublicIngress {
        base_url,
        listen_port,
        targets,
        stop,
        state,
        join: Some(join),
        meta_path,
        log_path,
        snapshot_path,
        requests_path,
    }
}

impl RunningPublicIngress {
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn port(&self) -> u16 {
        self.listen_port
    }

    pub fn force_next_target(&self, label: &str) {
        let next_index = self
            .targets
            .iter()
            .position(|target| target.label == label)
            .unwrap_or_else(|| panic!("public ingress has no target labeled `{label}`"));
        let mut state = self
            .state
            .lock()
            .expect("public ingress state mutex poisoned");
        state.next_target_index = next_index;
        state.events.push(format!(
            "force_next_target label={} node={}",
            label, self.targets[next_index].node_name
        ));
    }

    pub fn snapshot(&self) -> PublicIngressSnapshot {
        let state = self
            .state
            .lock()
            .expect("public ingress state mutex poisoned");
        PublicIngressSnapshot {
            base_url: self.base_url.clone(),
            listen_host: route_free::LOOPBACK_V4.to_string(),
            listen_port: self.listen_port,
            request_count: state.requests.len(),
            next_target_label: self.targets[state.next_target_index].label.clone(),
            last_error: state.last_error.clone().unwrap_or_default(),
            targets: self.targets.clone(),
            requests: state.requests.clone(),
        }
    }

    pub fn stop(mut self) -> PublicIngressSnapshot {
        self.shutdown();
        self.persist_artifacts();
        self.snapshot()
    }

    fn shutdown(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }

    fn persist_artifacts(&self) {
        let state = self
            .state
            .lock()
            .expect("public ingress state mutex poisoned");
        let snapshot = PublicIngressSnapshot {
            base_url: self.base_url.clone(),
            listen_host: route_free::LOOPBACK_V4.to_string(),
            listen_port: self.listen_port,
            request_count: state.requests.len(),
            next_target_label: self.targets[state.next_target_index].label.clone(),
            last_error: state.last_error.clone().unwrap_or_default(),
            targets: self.targets.clone(),
            requests: state.requests.clone(),
        };
        let log = if state.events.is_empty() {
            "no public ingress events recorded\n".to_string()
        } else {
            format!("{}\n", state.events.join("\n"))
        };
        drop(state);

        route_free::write_artifact(
            &self.log_path,
            format!(
                "meta: {}\nrequests: {}\nsnapshot: {}\n{}",
                self.meta_path.display(),
                self.requests_path.display(),
                self.snapshot_path.display(),
                log
            ),
        );
        route_free::write_json_artifact(&self.snapshot_path, &snapshot);
        route_free::write_json_artifact(&self.requests_path, &snapshot.requests);
    }
}

impl Drop for RunningPublicIngress {
    fn drop(&mut self) {
        self.shutdown();
        self.persist_artifacts();
    }
}

pub fn single_new_request(
    before: &PublicIngressSnapshot,
    after: &PublicIngressSnapshot,
) -> PublicIngressRequestRecord {
    assert!(
        after.request_count == before.request_count + 1,
        "expected exactly one new public ingress request, before_count={} after_count={}",
        before.request_count,
        after.request_count
    );
    after
        .requests
        .last()
        .cloned()
        .expect("public ingress snapshot should retain the newest request")
}

pub fn build_route_continuity_summary(
    public_request: &PublicIngressRequestRecord,
    primary_record: &Value,
    standby_record: &Value,
    expected_runtime_name: &str,
) -> RouteContinuitySummary {
    assert_eq!(
        public_request.method, "GET",
        "selected public request must be a GET /todos request"
    );
    assert_eq!(
        public_request.path, "/todos",
        "selected public request must target /todos"
    );
    assert_eq!(
        public_request.status_code, 200,
        "selected public request must have succeeded before continuity summary extraction"
    );
    assert!(
        public_request.error.is_empty(),
        "selected public request recorded a proxy error: {}",
        public_request.error
    );

    let request_key = required_shared_str(primary_record, standby_record, "request_key");
    let attempt_id = required_shared_str(primary_record, standby_record, "attempt_id");
    let runtime_name = required_shared_str(
        primary_record,
        standby_record,
        "declared_handler_runtime_name",
    );
    let replication_count =
        required_shared_u64(primary_record, standby_record, "replication_count");
    let ingress_node = required_shared_str(primary_record, standby_record, "ingress_node");
    let owner_node = required_shared_str(primary_record, standby_record, "owner_node");
    let replica_node = required_shared_str(primary_record, standby_record, "replica_node");
    let execution_node = required_shared_str(primary_record, standby_record, "execution_node");
    let phase = required_shared_str(primary_record, standby_record, "phase");
    let result = required_shared_str(primary_record, standby_record, "result");
    let error = required_shared_str(primary_record, standby_record, "error");

    assert_eq!(
        runtime_name, expected_runtime_name,
        "selected continuity record drifted to the wrong runtime"
    );
    assert_eq!(
        replication_count, 1,
        "selected route continuity record must stay on the replication_count=1 HTTP path"
    );
    assert_eq!(
        phase, "completed",
        "selected route continuity record must be completed"
    );
    assert_eq!(
        result, "succeeded",
        "selected route continuity record must succeed"
    );
    assert!(
        error.is_empty(),
        "selected route continuity record must not retain an error: {error}"
    );
    assert_eq!(
        ingress_node, public_request.target_node,
        "public ingress target metadata drifted from the runtime-owned ingress_node"
    );

    RouteContinuitySummary {
        public_request_id: public_request.request_id,
        public_method: public_request.method.clone(),
        public_path: public_request.path.clone(),
        public_target_label: public_request.target_label.clone(),
        public_target_node: public_request.target_node.clone(),
        public_target_port: public_request.target_port,
        public_status_code: public_request.status_code,
        request_key,
        attempt_id,
        runtime_name,
        replication_count,
        ingress_node,
        owner_node,
        replica_node,
        execution_node,
        phase,
        result,
    }
}

fn handle_client_connection(
    stream: TcpStream,
    targets: &[PublicIngressTarget],
    state: &Arc<Mutex<PublicIngressState>>,
) {
    let (request_id, target) = {
        let mut state = state.lock().expect("public ingress state mutex poisoned");
        let request_id = (state.requests.len() as u64) + 1;
        let target = targets[state.next_target_index].clone();
        state.next_target_index = (state.next_target_index + 1) % targets.len();
        state.events.push(format!(
            "request_id={} select_target label={} node={} port={}",
            request_id, target.label, target.node_name, target.port
        ));
        (request_id, target)
    };

    let mut stream = stream;
    stream
        .set_nonblocking(false)
        .expect("failed to switch accepted public ingress socket to blocking mode");
    let record = match proxy_request(&mut stream, request_id, &target) {
        Ok(record) => record,
        Err(error) => {
            write_error_response(&mut stream, &error);
            PublicIngressRequestRecord {
                request_id,
                method: String::new(),
                path: String::new(),
                request_line: String::new(),
                target_label: target.label.clone(),
                target_node: target.node_name.clone(),
                target_host: target.host.clone(),
                target_port: target.port,
                status_code: 502,
                response_status_line: "HTTP/1.1 502 Bad Gateway".to_string(),
                request_raw: String::new(),
                response_raw: format!(
                    "HTTP/1.1 502 Bad Gateway\\r\\nContent-Length: {}\\r\\nContent-Type: text/plain; charset=utf-8\\r\\nConnection: close\\r\\n\\r\\n{}",
                    error.as_bytes().len(),
                    error
                ),
                error,
            }
        }
    };

    let mut state = state.lock().expect("public ingress state mutex poisoned");
    if !record.error.is_empty() {
        state.last_error = Some(record.error.clone());
        state.events.push(format!(
            "request_id={} target={} status={} error={}",
            record.request_id, record.target_label, record.status_code, record.error
        ));
    } else {
        state.events.push(format!(
            "request_id={} target={} status={} path={}",
            record.request_id, record.target_label, record.status_code, record.path
        ));
    }
    state.requests.push(record);
}

fn proxy_request(
    client_stream: &mut TcpStream,
    request_id: u64,
    target: &PublicIngressTarget,
) -> Result<PublicIngressRequestRecord, String> {
    client_stream
        .set_read_timeout(Some(CLIENT_IO_TIMEOUT))
        .map_err(|error| format!("failed to set client read timeout: {error}"))?;
    client_stream
        .set_write_timeout(Some(CLIENT_IO_TIMEOUT))
        .map_err(|error| format!("failed to set client write timeout: {error}"))?;

    let request = read_http_message(client_stream, true)
        .map_err(|error| format!("request_id={request_id} invalid client request: {error}"))?;
    let method = request
        .method
        .clone()
        .ok_or_else(|| format!("request_id={request_id} client request is missing a method"))?;
    let path = request
        .path
        .clone()
        .ok_or_else(|| format!("request_id={request_id} client request is missing a path"))?;

    let mut backend = TcpStream::connect((target.host.as_str(), target.port)).map_err(|error| {
        format!(
            "request_id={request_id} failed to connect to {}:{} ({}) : {error}",
            target.host, target.port, target.label
        )
    })?;
    backend
        .set_read_timeout(Some(BACKEND_IO_TIMEOUT))
        .map_err(|error| format!("failed to set backend read timeout: {error}"))?;
    backend
        .set_write_timeout(Some(BACKEND_IO_TIMEOUT))
        .map_err(|error| format!("failed to set backend write timeout: {error}"))?;
    backend.write_all(&request.raw_bytes).map_err(|error| {
        format!("request_id={request_id} failed to forward request bytes: {error}")
    })?;
    let _ = backend.shutdown(Shutdown::Write);

    let response = read_http_message(&mut backend, false).map_err(|error| {
        format!(
            "request_id={request_id} target={} returned a malformed response: {error}",
            target.label
        )
    })?;
    let status_code = response.status_code.ok_or_else(|| {
        format!(
            "request_id={request_id} target={} returned a response without a status code",
            target.label
        )
    })?;

    client_stream
        .write_all(&response.raw_bytes)
        .map_err(|error| {
            format!("request_id={request_id} failed to write proxied response: {error}")
        })?;

    Ok(PublicIngressRequestRecord {
        request_id,
        method,
        path,
        request_line: request.request_line,
        target_label: target.label.clone(),
        target_node: target.node_name.clone(),
        target_host: target.host.clone(),
        target_port: target.port,
        status_code,
        response_status_line: response.status_line,
        request_raw: String::from_utf8_lossy(&request.raw_bytes).to_string(),
        response_raw: String::from_utf8_lossy(&response.raw_bytes).to_string(),
        error: String::new(),
    })
}

fn read_http_message(
    stream: &mut TcpStream,
    is_request: bool,
) -> Result<ParsedHttpMessage, String> {
    let mut raw = Vec::new();
    let mut buffer = [0_u8; 8192];
    let mut content_length: Option<usize> = None;

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                if raw.is_empty() {
                    return Err("peer closed the connection before sending data".to_string());
                }
                if let Some((header_len, expected_body_len)) =
                    content_length_state(&raw, content_length)?
                {
                    let actual_body_len = raw.len().saturating_sub(header_len);
                    if actual_body_len < expected_body_len {
                        let what = if is_request { "request" } else { "response" };
                        return Err(format!(
                            "truncated {what} body: expected {} bytes, got {}",
                            expected_body_len, actual_body_len
                        ));
                    }
                }
                break;
            }
            Ok(read) => {
                raw.extend_from_slice(&buffer[..read]);
                if content_length.is_none() {
                    content_length = parse_content_length(&raw)?;
                }
                if let Some((header_len, expected_body_len)) =
                    content_length_state(&raw, content_length)?
                {
                    let actual_body_len = raw.len().saturating_sub(header_len);
                    if actual_body_len == expected_body_len {
                        break;
                    }
                    if actual_body_len > expected_body_len {
                        let what = if is_request { "request" } else { "response" };
                        return Err(format!(
                            "ambiguous {what} body length: expected {} bytes, got {}",
                            expected_body_len, actual_body_len
                        ));
                    }
                } else if header_end(&raw).is_some() && content_length.is_none() && !is_request {
                    continue;
                } else if header_end(&raw).is_some() && content_length.is_none() {
                    break;
                }
            }
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) =>
            {
                let what = if is_request { "request" } else { "response" };
                return Err(format!("timed out while reading {what}"));
            }
            Err(error) => {
                let what = if is_request { "request" } else { "response" };
                return Err(format!("failed to read {what}: {error}"));
            }
        }
    }

    parse_http_message(&raw, is_request)
}

fn parse_http_message(raw: &[u8], is_request: bool) -> Result<ParsedHttpMessage, String> {
    let header_end = header_end(raw)
        .ok_or_else(|| "missing HTTP header terminator (\\r\\n\\r\\n)".to_string())?;
    let headers = std::str::from_utf8(&raw[..header_end])
        .map_err(|error| format!("HTTP headers are not valid UTF-8: {error}"))?;
    let mut lines = headers.lines();
    let first_line = lines
        .next()
        .ok_or_else(|| "HTTP message is missing a request/status line".to_string())?
        .trim()
        .to_string();

    if is_request {
        let mut parts = first_line.split_whitespace();
        let method = parts
            .next()
            .ok_or_else(|| "HTTP request line is missing a method".to_string())?;
        let path = parts
            .next()
            .ok_or_else(|| "HTTP request line is missing a path".to_string())?;
        let version = parts
            .next()
            .ok_or_else(|| "HTTP request line is missing a version".to_string())?;
        if !version.starts_with("HTTP/") {
            return Err(format!(
                "HTTP request line has an invalid version: {first_line}"
            ));
        }
        Ok(ParsedHttpMessage {
            method: Some(method.to_string()),
            path: Some(path.to_string()),
            request_line: first_line,
            status_code: None,
            status_line: String::new(),
            raw_bytes: raw.to_vec(),
        })
    } else {
        let mut parts = first_line.split_whitespace();
        let version = parts
            .next()
            .ok_or_else(|| "HTTP response line is missing a version".to_string())?;
        if !version.starts_with("HTTP/") {
            return Err(format!("invalid HTTP status line: {first_line}"));
        }
        let code = parts
            .next()
            .ok_or_else(|| "HTTP response line is missing a status code".to_string())?
            .parse::<u16>()
            .map_err(|_| format!("invalid HTTP status code in `{first_line}`"))?;
        Ok(ParsedHttpMessage {
            method: None,
            path: None,
            request_line: String::new(),
            status_code: Some(code),
            status_line: first_line,
            raw_bytes: raw.to_vec(),
        })
    }
}

fn write_error_response(stream: &mut TcpStream, message: &str) {
    let body = message.as_bytes();
    let headers = format!(
        "HTTP/1.1 502 Bad Gateway\\r\\nContent-Length: {}\\r\\nContent-Type: text/plain; charset=utf-8\\r\\nConnection: close\\r\\n\\r\\n",
        body.len()
    );
    let _ = stream.write_all(headers.as_bytes());
    let _ = stream.write_all(body);
}

fn header_end(raw: &[u8]) -> Option<usize> {
    raw.windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|index| index + 4)
}

fn parse_content_length(raw: &[u8]) -> Result<Option<usize>, String> {
    let Some(header_end) = header_end(raw) else {
        return Ok(None);
    };
    let headers = std::str::from_utf8(&raw[..header_end])
        .map_err(|error| format!("HTTP headers are not valid UTF-8: {error}"))?;
    for line in headers.lines() {
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        if name.eq_ignore_ascii_case("content-length") {
            let trimmed = value.trim();
            let parsed = trimmed
                .parse::<usize>()
                .map_err(|_| format!("invalid Content-Length header value `{trimmed}`"))?;
            return Ok(Some(parsed));
        }
    }
    Ok(None)
}

fn content_length_state(
    raw: &[u8],
    content_length: Option<usize>,
) -> Result<Option<(usize, usize)>, String> {
    let Some(header_len) = header_end(raw) else {
        return Ok(None);
    };
    Ok(content_length.map(|expected_body_len| (header_len, expected_body_len)))
}

fn required_json_str(record: &Value, field: &str) -> String {
    record[field]
        .as_str()
        .unwrap_or_else(|| {
            panic!("route continuity record is missing string field `{field}` in {record}")
        })
        .to_string()
}

fn required_json_u64(record: &Value, field: &str) -> u64 {
    record[field].as_u64().unwrap_or_else(|| {
        panic!("route continuity record is missing u64 field `{field}` in {record}")
    })
}

fn required_shared_str(primary_record: &Value, standby_record: &Value, field: &str) -> String {
    let primary = required_json_str(primary_record, field);
    let standby = required_json_str(standby_record, field);
    assert_eq!(
        primary, standby,
        "route continuity field `{field}` drifted between primary and standby records"
    );
    primary
}

fn required_shared_u64(primary_record: &Value, standby_record: &Value, field: &str) -> u64 {
    let primary = required_json_u64(primary_record, field);
    let standby = required_json_u64(standby_record, field);
    assert_eq!(
        primary, standby,
        "route continuity field `{field}` drifted between primary and standby records"
    );
    primary
}
