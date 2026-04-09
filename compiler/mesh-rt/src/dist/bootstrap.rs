use std::env;

const CLUSTER_PORT_ENV: &str = "MESH_CLUSTER_PORT";
const CLUSTER_COOKIE_ENV: &str = "MESH_CLUSTER_COOKIE";
const DISCOVERY_SEED_ENV: &str = "MESH_DISCOVERY_SEED";
const NODE_NAME_ENV: &str = "MESH_NODE_NAME";
const FLY_APP_NAME_ENV: &str = "FLY_APP_NAME";
const FLY_REGION_ENV: &str = "FLY_REGION";
const FLY_MACHINE_ID_ENV: &str = "FLY_MACHINE_ID";
const FLY_PRIVATE_IP_ENV: &str = "FLY_PRIVATE_IP";
const DEFAULT_CLUSTER_PORT: u16 = 4370;

/// Startup mode chosen by the runtime bootstrap helper.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootstrapMode {
    Standalone,
    Cluster,
}

impl BootstrapMode {
    /// String label used by higher-level Mesh surfaces.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Standalone => "standalone",
            Self::Cluster => "cluster",
        }
    }
}

/// Typed startup status returned by the runtime bootstrap helper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapStatus {
    pub mode: BootstrapMode,
    pub node_name: String,
    pub cluster_port: u16,
    pub discovery_seed: String,
}

impl BootstrapStatus {
    /// Stable string label for logging and higher-level status reporting.
    pub fn mode_label(&self) -> &'static str {
        self.mode.as_str()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct BootstrapInputs {
    pub(crate) cluster_port: Option<String>,
    pub(crate) cookie: Option<String>,
    pub(crate) discovery_seed: Option<String>,
    pub(crate) node_name: Option<String>,
    pub(crate) fly_app_name: Option<String>,
    pub(crate) fly_region: Option<String>,
    pub(crate) fly_machine_id: Option<String>,
    pub(crate) fly_private_ip: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BootstrapPlan {
    status: BootstrapStatus,
    cookie: String,
}

impl BootstrapInputs {
    pub(crate) fn from_env() -> Result<Self, String> {
        Ok(Self {
            cluster_port: read_utf8_env(CLUSTER_PORT_ENV)?,
            cookie: read_utf8_env(CLUSTER_COOKIE_ENV)?,
            discovery_seed: read_utf8_env(DISCOVERY_SEED_ENV)?,
            node_name: read_utf8_env(NODE_NAME_ENV)?,
            fly_app_name: read_utf8_env(FLY_APP_NAME_ENV)?,
            fly_region: read_utf8_env(FLY_REGION_ENV)?,
            fly_machine_id: read_utf8_env(FLY_MACHINE_ID_ENV)?,
            fly_private_ip: read_utf8_env(FLY_PRIVATE_IP_ENV)?,
        })
    }
}

pub(crate) fn bootstrap_from_env_with<F>(start_node: F) -> Result<BootstrapStatus, String>
where
    F: FnOnce(&str, &str) -> i64,
{
    bootstrap_with_inputs(BootstrapInputs::from_env()?, start_node)
}

pub(crate) fn bootstrap_with_inputs<F>(
    inputs: BootstrapInputs,
    start_node: F,
) -> Result<BootstrapStatus, String>
where
    F: FnOnce(&str, &str) -> i64,
{
    let plan = resolve_bootstrap(inputs)?;
    if plan.status.mode == BootstrapMode::Standalone {
        return Ok(plan.status);
    }

    let node_name = plan.status.node_name.clone();
    let result = start_node(&node_name, &plan.cookie);
    match result {
        0 => Ok(plan.status),
        -1 => Err(format!(
            "mesh bootstrap start failed node={node_name}: node already started"
        )),
        -2 => Err(format!(
            "mesh bootstrap start failed node={node_name}: listener bind failed"
        )),
        -3 => Err(format!(
            "mesh bootstrap start failed node={node_name}: invalid node name or cookie"
        )),
        other => Err(format!(
            "mesh bootstrap start failed node={node_name}: unexpected start code={other}"
        )),
    }
}

fn resolve_bootstrap(inputs: BootstrapInputs) -> Result<BootstrapPlan, String> {
    let cluster_port = parse_cluster_port(inputs.cluster_port.as_deref())?;
    let cookie = trim_or_empty(inputs.cookie.as_deref());
    let discovery_seed = trim_or_empty(inputs.discovery_seed.as_deref());
    let explicit_node_name = inputs.node_name.unwrap_or_default();
    let fly_app_name = inputs.fly_app_name.unwrap_or_default();
    let fly_region = inputs.fly_region.unwrap_or_default();
    let fly_machine_id = inputs.fly_machine_id.unwrap_or_default();
    let fly_private_ip = inputs.fly_private_ip.unwrap_or_default();

    if cookie.is_empty() {
        if has_cluster_hint(
            &discovery_seed,
            &explicit_node_name,
            &fly_app_name,
            &fly_region,
            &fly_machine_id,
            &fly_private_ip,
        ) {
            return Err(cluster_cookie_required().to_string());
        }

        return Ok(BootstrapPlan {
            cookie,
            status: BootstrapStatus {
                mode: BootstrapMode::Standalone,
                node_name: String::new(),
                cluster_port,
                discovery_seed: String::new(),
            },
        });
    }

    if discovery_seed.is_empty() {
        return Err(missing_required_env(DISCOVERY_SEED_ENV));
    }

    let node_name = resolve_node_name(
        &explicit_node_name,
        &fly_app_name,
        &fly_region,
        &fly_machine_id,
        &fly_private_ip,
        cluster_port,
    )?;

    Ok(BootstrapPlan {
        cookie,
        status: BootstrapStatus {
            mode: BootstrapMode::Cluster,
            node_name,
            cluster_port,
            discovery_seed,
        },
    })
}

fn read_utf8_env(name: &str) -> Result<Option<String>, String> {
    match env::var(name) {
        Ok(value) => Ok(Some(value)),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(env::VarError::NotUnicode(_)) => Err(format!("{name} must be valid UTF-8")),
    }
}

fn parse_cluster_port(raw: Option<&str>) -> Result<u16, String> {
    match raw {
        None => Ok(DEFAULT_CLUSTER_PORT),
        Some("") => Ok(DEFAULT_CLUSTER_PORT),
        Some(value) => {
            parse_positive_u16(value.trim()).map_err(|_| invalid_positive_int(CLUSTER_PORT_ENV))
        }
    }
}

fn parse_positive_u16(raw: &str) -> Result<u16, ()> {
    match raw.parse::<u16>() {
        Ok(value) if value > 0 => Ok(value),
        _ => Err(()),
    }
}

fn trim_or_empty(value: Option<&str>) -> String {
    value.map(str::trim).unwrap_or_default().to_string()
}

fn has_cluster_hint(
    discovery_seed: &str,
    explicit_node_name: &str,
    fly_app_name: &str,
    fly_region: &str,
    fly_machine_id: &str,
    fly_private_ip: &str,
) -> bool {
    !discovery_seed.is_empty()
        || !explicit_node_name.trim().is_empty()
        || any_fly_identity_set(fly_app_name, fly_region, fly_machine_id, fly_private_ip)
}

fn any_fly_identity_set(
    fly_app_name: &str,
    fly_region: &str,
    fly_machine_id: &str,
    fly_private_ip: &str,
) -> bool {
    !fly_app_name.is_empty()
        || !fly_region.is_empty()
        || !fly_machine_id.is_empty()
        || !fly_private_ip.is_empty()
}

fn resolve_node_name(
    explicit_node_name: &str,
    fly_app_name: &str,
    fly_region: &str,
    fly_machine_id: &str,
    fly_private_ip: &str,
    cluster_port: u16,
) -> Result<String, String> {
    let trimmed_node_name = explicit_node_name.trim();
    if !trimmed_node_name.is_empty() {
        validate_explicit_node_name(trimmed_node_name, cluster_port)?;
        return Ok(trimmed_node_name.to_string());
    }

    if any_fly_identity_set(fly_app_name, fly_region, fly_machine_id, fly_private_ip) {
        return compose_fly_node_name(
            fly_app_name,
            fly_region,
            fly_machine_id,
            fly_private_ip,
            cluster_port,
        );
    }

    Err(invalid_cluster_identity(missing_identity_source()))
}

fn validate_explicit_node_name(node_name: &str, cluster_port: u16) -> Result<(), String> {
    if node_name.contains(' ') {
        return Err(invalid_node_name("value cannot contain spaces"));
    }

    let parts: Vec<_> = node_name.split('@').collect();
    if parts.len() != 2 {
        return Err(invalid_node_name("expected name@host:port"));
    }

    let raw_name = parts[0].trim();
    let raw_host_port = parts[1].trim();
    if raw_name.is_empty() {
        return Err(invalid_node_name("node name cannot be blank"));
    }
    if raw_name.contains('@') {
        return Err(invalid_node_name("node name cannot contain @"));
    }

    validate_explicit_node_host_port(raw_host_port, cluster_port)
}

fn validate_explicit_node_host_port(host_port: &str, cluster_port: u16) -> Result<(), String> {
    let trimmed_host_port = host_port.trim();
    if trimmed_host_port.is_empty() {
        return Err(invalid_node_name("host cannot be blank"));
    }

    if let Some(rest) = trimmed_host_port.strip_prefix('[') {
        let Some((raw_host, raw_port)) = rest.split_once("]:") else {
            return Err(invalid_node_name("IPv6 host must use [addr]:port"));
        };
        if raw_host.is_empty() {
            return Err(invalid_node_name("host cannot be blank"));
        }
        if raw_host.contains('@') || raw_host.contains(' ') {
            return Err(invalid_node_name("host is invalid"));
        }
        validate_cluster_port_match(raw_port.trim(), cluster_port)
    } else if trimmed_host_port.contains('[') || trimmed_host_port.contains(']') {
        Err(invalid_node_name("host is invalid"))
    } else {
        let parts: Vec<_> = trimmed_host_port.split(':').collect();
        if parts.len() != 2 {
            return Err(invalid_node_name("expected name@host:port"));
        }

        let raw_host = parts[0].trim();
        let raw_port = parts[1].trim();
        if raw_host.is_empty() {
            return Err(invalid_node_name("host cannot be blank"));
        }
        if raw_host.contains('@') || raw_host.contains(' ') {
            return Err(invalid_node_name("host is invalid"));
        }
        validate_cluster_port_match(raw_port, cluster_port)
    }
}

fn validate_cluster_port_match(raw_port: &str, cluster_port: u16) -> Result<(), String> {
    let port = parse_positive_u16(raw_port)
        .map_err(|_| invalid_node_name("port must be a positive integer"))?;
    if port != cluster_port {
        return Err(invalid_node_name(&format!(
            "port must match {CLUSTER_PORT_ENV}"
        )));
    }
    Ok(())
}

fn compose_fly_node_name(
    fly_app_name: &str,
    fly_region: &str,
    fly_machine_id: &str,
    fly_private_ip: &str,
    cluster_port: u16,
) -> Result<String, String> {
    let app_name = fly_app_name.trim();
    let region = fly_region.trim();
    let machine_id = fly_machine_id.trim();
    let private_ip = fly_private_ip.trim();
    if app_name.is_empty() || region.is_empty() || machine_id.is_empty() || private_ip.is_empty() {
        return Err(invalid_cluster_identity(fly_identity_required()));
    }

    compose_node_name(
        &format!("{app_name}-{region}-{machine_id}"),
        private_ip,
        cluster_port,
    )
}

fn compose_node_name(
    node_basename: &str,
    advertised_host: &str,
    cluster_port: u16,
) -> Result<String, String> {
    let trimmed_basename = node_basename.trim();
    if trimmed_basename.is_empty() {
        return Err(invalid_cluster_identity("node basename cannot be blank"));
    }
    if trimmed_basename.contains('@') {
        return Err(invalid_cluster_identity("node basename cannot contain @"));
    }

    let normalized_host = normalized_host(advertised_host)?;
    Ok(format!(
        "{trimmed_basename}@{normalized_host}:{cluster_port}"
    ))
}

fn normalized_host(advertised_host: &str) -> Result<String, String> {
    let trimmed = advertised_host.trim();
    if trimmed.is_empty() {
        return Err(invalid_cluster_identity("advertised host cannot be blank"));
    }
    if trimmed.contains('@') {
        return Err(invalid_cluster_identity("advertised host cannot contain @"));
    }
    if let Some(rest) = trimmed.strip_prefix('[') {
        if rest.ends_with(']') {
            return Ok(trimmed.to_string());
        }
        return Err(invalid_cluster_identity(
            "advertised host has an opening '[' without a closing ']'",
        ));
    }
    if trimmed.ends_with(']') {
        return Err(invalid_cluster_identity(
            "advertised host has a closing ']' without an opening '['",
        ));
    }
    if trimmed.contains(':') {
        return Ok(format!("[{trimmed}]"));
    }
    Ok(trimmed.to_string())
}

fn missing_required_env(name: &str) -> String {
    format!("Missing required environment variable {name}")
}

fn invalid_positive_int(name: &str) -> String {
    format!("Invalid {name}: expected a positive integer")
}

fn cluster_cookie_required() -> &'static str {
    "MESH_CLUSTER_COOKIE is required when discovery or identity env is set"
}

fn fly_identity_required() -> &'static str {
    "Fly cluster identity requires FLY_APP_NAME, FLY_REGION, FLY_MACHINE_ID, and FLY_PRIVATE_IP"
}

fn missing_identity_source() -> &'static str {
    "cluster mode requires MESH_NODE_NAME or Fly identity env"
}

fn invalid_cluster_identity(reason: &str) -> String {
    format!("Invalid cluster identity: {reason}")
}

fn invalid_node_name(reason: &str) -> String {
    format!("Invalid MESH_NODE_NAME: {reason}")
}
