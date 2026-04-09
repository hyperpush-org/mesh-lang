use std::collections::HashSet;
use std::env;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::sync::atomic::Ordering;
use std::time::Duration;

use super::node::{mesh_node_connect, node_state};

const DEFAULT_DISCOVERY_RECONCILE_MS: u64 = 5_000;
const DISCOVERY_PROVIDER: &str = "dns";
const DISCOVERY_SEED_ENV: &str = "MESH_DISCOVERY_SEED";
const DISCOVERY_CLUSTER_PORT_ENV: &str = "MESH_CLUSTER_PORT";
const DISCOVERY_RECONCILE_MS_ENV: &str = "MESH_DISCOVERY_INTERVAL_MS";
const DISCOVERY_CONNECT_NAME: &str = "discovery";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DiscoveryConfig {
    pub(crate) seed: String,
    pub(crate) cluster_port: u16,
    pub(crate) reconcile_interval: Duration,
}

impl DiscoveryConfig {
    pub(crate) fn from_env(default_cluster_port: u16) -> Result<Option<Self>, String> {
        let seed = match env::var(DISCOVERY_SEED_ENV) {
            Ok(value) => value,
            Err(env::VarError::NotPresent) => return Ok(None),
            Err(env::VarError::NotUnicode(_)) => {
                return Err(format!("{DISCOVERY_SEED_ENV} must be valid UTF-8"));
            }
        };

        Self::from_parts(
            Some(seed.as_str()),
            env::var(DISCOVERY_CLUSTER_PORT_ENV).ok().as_deref(),
            default_cluster_port,
            env::var(DISCOVERY_RECONCILE_MS_ENV).ok().as_deref(),
        )
        .map(Some)
    }

    fn from_parts(
        seed: Option<&str>,
        cluster_port: Option<&str>,
        default_cluster_port: u16,
        reconcile_interval_ms: Option<&str>,
    ) -> Result<Self, String> {
        let seed = seed
            .ok_or_else(|| format!("{DISCOVERY_SEED_ENV} is required when discovery is enabled"))?
            .trim();
        if seed.is_empty() {
            return Err(format!("{DISCOVERY_SEED_ENV} cannot be blank"));
        }

        let cluster_port = match cluster_port {
            Some(raw) => parse_positive_u16(raw.trim(), DISCOVERY_CLUSTER_PORT_ENV)?,
            None => default_cluster_port,
        };

        let reconcile_ms = match reconcile_interval_ms {
            Some(raw) => parse_positive_u64(raw.trim(), DISCOVERY_RECONCILE_MS_ENV)?,
            None => DEFAULT_DISCOVERY_RECONCILE_MS,
        };

        Ok(Self {
            seed: seed.to_string(),
            cluster_port,
            reconcile_interval: Duration::from_millis(reconcile_ms),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CandidateRejectReason {
    Duplicate,
    SelfAddress,
    AlreadyConnected,
}

impl CandidateRejectReason {
    fn label(&self) -> &'static str {
        match self {
            Self::Duplicate => "duplicate",
            Self::SelfAddress => "self",
            Self::AlreadyConnected => "already_connected",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CandidateRejection {
    pub(crate) candidate: SocketAddr,
    pub(crate) reason: CandidateRejectReason,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FilteredCandidates {
    pub(crate) accepted: Vec<SocketAddr>,
    pub(crate) rejected: Vec<CandidateRejection>,
}

pub(crate) fn start_from_env() {
    let Some(state) = node_state() else {
        return;
    };

    let config = match DiscoveryConfig::from_env(state.port) {
        Ok(Some(config)) => config,
        Ok(None) => return,
        Err(err) => {
            eprintln!(
                "mesh discovery: provider={} status=disabled reason={} last_error={}",
                DISCOVERY_PROVIDER, err, err
            );
            return;
        }
    };

    let thread_name = format!("mesh-discovery-{}", sanitize_thread_name(&config.seed));
    std::thread::Builder::new()
        .name(thread_name)
        .spawn(move || discovery_loop(config))
        .expect("failed to spawn mesh discovery thread");
}

fn discovery_loop(config: DiscoveryConfig) {
    let mut last_error: Option<String> = None;

    loop {
        let Some(state) = node_state() else {
            break;
        };
        if state.listener_shutdown.load(Ordering::Relaxed) {
            break;
        }

        reconcile_once(&config, &mut last_error);
        std::thread::sleep(config.reconcile_interval);
    }
}

fn reconcile_once(config: &DiscoveryConfig, last_error: &mut Option<String>) {
    let Some(state) = node_state() else {
        return;
    };

    let resolved = match resolve_dns_candidates(config) {
        Ok(candidates) => candidates,
        Err(err) => {
            *last_error = Some(err.clone());
            log_reconcile(
                config,
                0,
                &FilteredCandidates::default(),
                last_error.as_deref(),
            );
            return;
        }
    };

    let connected_names: Vec<String> = {
        let sessions = state.sessions.read();
        sessions.keys().cloned().collect()
    };

    let filtered = filter_candidates(&state.name, &connected_names, resolved);
    let mut tick_error = None;

    for candidate in &filtered.accepted {
        let target = synthesize_connect_target(*candidate);
        let code = mesh_node_connect(target.as_ptr(), target.len() as u64);
        if code != 0 {
            tick_error = Some(format!(
                "connect target={} result={} seed={}",
                candidate, code, config.seed
            ));
        }
    }

    *last_error = tick_error;
    log_reconcile(
        config,
        filtered.accepted.len() + filtered.rejected.len(),
        &filtered,
        last_error.as_deref(),
    );
}

fn resolve_dns_candidates(config: &DiscoveryConfig) -> Result<Vec<SocketAddr>, String> {
    (config.seed.as_str(), config.cluster_port)
        .to_socket_addrs()
        .map(|iter| iter.collect())
        .map_err(|err| format!("seed={} resolve_failed={}", config.seed, err))
}

pub(crate) fn filter_candidates<I>(
    self_name: &str,
    connected_names: &[String],
    candidates: I,
) -> FilteredCandidates
where
    I: IntoIterator<Item = SocketAddr>,
{
    let self_targets = resolve_node_targets(self_name);
    let connected_targets: HashSet<SocketAddr> = connected_names
        .iter()
        .flat_map(|name| resolve_node_targets(name))
        .collect();

    let mut seen = HashSet::new();
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();

    for candidate in candidates {
        if !seen.insert(candidate) {
            rejected.push(CandidateRejection {
                candidate,
                reason: CandidateRejectReason::Duplicate,
            });
            continue;
        }

        if self_targets.contains(&candidate) {
            rejected.push(CandidateRejection {
                candidate,
                reason: CandidateRejectReason::SelfAddress,
            });
            continue;
        }

        if connected_targets.contains(&candidate) {
            rejected.push(CandidateRejection {
                candidate,
                reason: CandidateRejectReason::AlreadyConnected,
            });
            continue;
        }

        accepted.push(candidate);
    }

    FilteredCandidates { accepted, rejected }
}

pub(crate) fn parse_host_port<'a>(
    host_port: &'a str,
    default_port: u16,
    full_value: &str,
) -> Result<(&'a str, u16), String> {
    if host_port.is_empty() {
        return Err(format!(
            "invalid node name '{}': empty host part",
            full_value
        ));
    }

    if let Some(rest) = host_port.strip_prefix('[') {
        let end = rest.find(']').ok_or_else(|| {
            format!(
                "invalid node name '{}': missing closing ']' for bracketed host",
                full_value
            )
        })?;
        let host = &rest[..end];
        if host.is_empty() {
            return Err(format!(
                "invalid node name '{}': empty host part",
                full_value
            ));
        }
        let suffix = &rest[end + 1..];
        if suffix.is_empty() {
            return Ok((host, default_port));
        }
        let port = suffix.strip_prefix(':').ok_or_else(|| {
            format!(
                "invalid node name '{}': unexpected characters after bracketed host",
                full_value
            )
        })?;
        return Ok((host, parse_positive_u16(port, full_value)?));
    }

    if host_port.matches(':').count() > 1 {
        if host_port.parse::<IpAddr>().is_ok() {
            return Ok((host_port, default_port));
        }
        return Err(format!(
            "invalid node name '{}': IPv6 hosts with ports must use brackets",
            full_value
        ));
    }

    if let Some(colon_pos) = host_port.rfind(':') {
        let host = &host_port[..colon_pos];
        let port_str = &host_port[colon_pos + 1..];
        if host.is_empty() {
            return Err(format!(
                "invalid node name '{}': empty host part",
                full_value
            ));
        }
        return Ok((host, parse_positive_u16(port_str, full_value)?));
    }

    Ok((host_port, default_port))
}

#[cfg(test)]
pub(crate) fn socket_addr_from_candidate_host(host: &str, port: u16) -> Result<SocketAddr, String> {
    let trimmed = host.trim();
    if trimmed.is_empty() {
        return Err("candidate host cannot be blank".to_string());
    }

    let without_brackets = if let Some(rest) = trimmed.strip_prefix('[') {
        let end = rest
            .find(']')
            .ok_or_else(|| format!("candidate host '{}' is missing closing ']'", host))?;
        if end + 1 != rest.len() {
            return Err(format!(
                "candidate host '{}' must not include a port suffix",
                host
            ));
        }
        &rest[..end]
    } else {
        trimmed
    };

    let ip: IpAddr = without_brackets
        .parse()
        .map_err(|_| format!("candidate host '{}' is not a valid IP literal", host))?;
    Ok(SocketAddr::new(ip, port))
}

fn resolve_node_targets(node_name: &str) -> HashSet<SocketAddr> {
    split_node_name(node_name)
        .ok()
        .and_then(|(_, host, port)| (host, port).to_socket_addrs().ok())
        .map(|addrs| addrs.collect())
        .unwrap_or_default()
}

fn split_node_name(name: &str) -> Result<(&str, &str, u16), String> {
    let at_pos = name
        .find('@')
        .ok_or_else(|| format!("invalid node name '{}': missing '@' separator", name))?;

    let name_part = &name[..at_pos];
    let host_port = &name[at_pos + 1..];

    if name_part.is_empty() {
        return Err(format!("invalid node name '{}': empty name part", name));
    }

    let (host, port) = parse_host_port(host_port, 9000, name)?;
    Ok((name_part, host, port))
}

fn synthesize_connect_target(candidate: SocketAddr) -> String {
    format!(
        "{}@{}",
        DISCOVERY_CONNECT_NAME,
        format_socket_addr(candidate)
    )
}

fn format_socket_addr(candidate: SocketAddr) -> String {
    candidate.to_string()
}

fn log_reconcile(
    config: &DiscoveryConfig,
    resolved_count: usize,
    filtered: &FilteredCandidates,
    last_error: Option<&str>,
) {
    let rejected = if filtered.rejected.is_empty() {
        "none".to_string()
    } else {
        filtered
            .rejected
            .iter()
            .map(|item| format!("{}:{}", item.candidate, item.reason.label()))
            .collect::<Vec<_>>()
            .join(",")
    };

    let accepted = if filtered.accepted.is_empty() {
        "none".to_string()
    } else {
        filtered
            .accepted
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",")
    };

    eprintln!(
        "mesh discovery: provider={} seed={} cluster_port={} resolved={} accepted={} accepted_targets={} rejected_targets={} last_error={}",
        DISCOVERY_PROVIDER,
        config.seed,
        config.cluster_port,
        resolved_count,
        filtered.accepted.len(),
        accepted,
        rejected,
        last_error.unwrap_or("none")
    );
}

fn sanitize_thread_name(seed: &str) -> String {
    let sanitized: String = seed
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect();
    sanitized.chars().take(24).collect()
}

fn parse_positive_u16(raw: &str, label: &str) -> Result<u16, String> {
    let value: u16 = raw
        .parse()
        .map_err(|_| format!("{} must be a valid u16", label))?;
    if value == 0 {
        return Err(format!("{} must be greater than zero", label));
    }
    Ok(value)
}

fn parse_positive_u64(raw: &str, label: &str) -> Result<u64, String> {
    let value: u64 = raw
        .parse()
        .map_err(|_| format!("{} must be a valid u64", label))?;
    if value == 0 {
        return Err(format!("{} must be greater than zero", label));
    }
    Ok(value)
}

impl Default for FilteredCandidates {
    fn default() -> Self {
        Self {
            accepted: Vec::new(),
            rejected: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn discovery_config_rejects_blank_seed_and_zero_interval() {
        let blank_seed = DiscoveryConfig::from_parts(Some("   "), None, 9000, None).unwrap_err();
        assert!(blank_seed.contains(DISCOVERY_SEED_ENV));

        let zero_interval =
            DiscoveryConfig::from_parts(Some("cluster.internal"), None, 9000, Some("0"))
                .unwrap_err();
        assert!(zero_interval.contains(DISCOVERY_RECONCILE_MS_ENV));
    }

    #[test]
    fn discovery_config_rejects_invalid_cluster_port() {
        let err =
            DiscoveryConfig::from_parts(Some("cluster.internal"), Some("not-a-port"), 9000, None)
                .unwrap_err();

        assert!(err.contains(DISCOVERY_CLUSTER_PORT_ENV));
    }

    #[test]
    fn discovery_socket_addr_from_candidate_host_accepts_bracketed_ipv6_literals() {
        let candidate = socket_addr_from_candidate_host("[::1]", 9100).unwrap();

        assert_eq!(
            candidate,
            SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 9100)
        );
    }

    #[test]
    fn discovery_socket_addr_from_candidate_host_rejects_blank_or_malformed_hosts() {
        let blank = socket_addr_from_candidate_host("   ", 9100).unwrap_err();
        assert!(blank.contains("blank"));

        let malformed = socket_addr_from_candidate_host("[::1", 9100).unwrap_err();
        assert!(malformed.contains("missing closing ']'"));
    }

    #[test]
    fn discovery_filter_candidates_dedupes_self_and_connected_peers() {
        let self_name = "self@127.0.0.1:9000";
        let connected = vec!["peer@[::1]:9000".to_string()];
        let candidates = vec![
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 9000),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 9000),
            SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 9000),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 8)), 9000),
        ];

        let filtered = filter_candidates(self_name, &connected, candidates);

        assert_eq!(
            filtered.accepted,
            vec![SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(10, 0, 0, 8)),
                9000
            )]
        );
        assert_eq!(filtered.rejected.len(), 3);
        assert!(filtered
            .rejected
            .iter()
            .any(|entry| entry.reason == CandidateRejectReason::SelfAddress));
        assert!(filtered
            .rejected
            .iter()
            .any(|entry| entry.reason == CandidateRejectReason::Duplicate));
        assert!(filtered
            .rejected
            .iter()
            .any(|entry| entry.reason == CandidateRejectReason::AlreadyConnected));
    }

    #[test]
    fn discovery_parse_host_port_accepts_bracketed_ipv6_names() {
        let (host, port) =
            parse_host_port("[fd00::1234]:9400", 9000, "node@[fd00::1234]:9400").unwrap();

        assert_eq!(host, "fd00::1234");
        assert_eq!(port, 9400);
    }
}
