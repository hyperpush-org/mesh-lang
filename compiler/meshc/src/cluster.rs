use std::time::Duration;

use clap::{Args, Subcommand};
use mesh_rt::{
    query_operator_continuity_list_remote, query_operator_continuity_status_remote,
    query_operator_diagnostics_remote, query_operator_status_remote, ContinuityRecord,
    DEFAULT_OPERATOR_QUERY_TIMEOUT,
};
use serde_json::json;

#[derive(Subcommand, Debug)]
pub enum ClusterCommand {
    /// Show runtime-owned membership and authority for a clustered node.
    Status(ClusterStatusArgs),
    /// Show runtime-owned continuity status for one request key, or list recent continuity records.
    Continuity(ClusterContinuityArgs),
    /// Show recent runtime-owned failover and continuity diagnostics.
    Diagnostics(ClusterDiagnosticsArgs),
}

#[derive(Args, Debug)]
pub struct ClusterStatusArgs {
    /// Cluster node to inspect (name@host:port)
    pub target: String,

    /// Shared cluster cookie (defaults to MESH_CLUSTER_COOKIE)
    #[arg(long)]
    pub cookie: Option<String>,

    /// Query timeout in milliseconds
    #[arg(long, default_value_t = DEFAULT_OPERATOR_QUERY_TIMEOUT.as_millis() as u64)]
    pub timeout_ms: u64,

    /// Emit JSON instead of human-readable output
    #[arg(long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct ClusterContinuityArgs {
    /// Cluster node to inspect (name@host:port)
    pub target: String,

    /// Optional request key. When omitted, lists recent continuity records.
    pub request_key: Option<String>,

    /// Max records to return when listing continuity records.
    #[arg(long)]
    pub limit: Option<usize>,

    /// Shared cluster cookie (defaults to MESH_CLUSTER_COOKIE)
    #[arg(long)]
    pub cookie: Option<String>,

    /// Query timeout in milliseconds
    #[arg(long, default_value_t = DEFAULT_OPERATOR_QUERY_TIMEOUT.as_millis() as u64)]
    pub timeout_ms: u64,

    /// Emit JSON instead of human-readable output
    #[arg(long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct ClusterDiagnosticsArgs {
    /// Cluster node to inspect (name@host:port)
    pub target: String,

    /// Max diagnostic entries to return
    #[arg(long)]
    pub limit: Option<usize>,

    /// Shared cluster cookie (defaults to MESH_CLUSTER_COOKIE)
    #[arg(long)]
    pub cookie: Option<String>,

    /// Query timeout in milliseconds
    #[arg(long, default_value_t = DEFAULT_OPERATOR_QUERY_TIMEOUT.as_millis() as u64)]
    pub timeout_ms: u64,

    /// Emit JSON instead of human-readable output
    #[arg(long)]
    pub json: bool,
}

pub fn run_cluster_command(command: ClusterCommand) -> Result<(), String> {
    match command {
        ClusterCommand::Status(args) => run_status(args),
        ClusterCommand::Continuity(args) => run_continuity(args),
        ClusterCommand::Diagnostics(args) => run_diagnostics(args),
    }
}

fn run_status(args: ClusterStatusArgs) -> Result<(), String> {
    let cookie = cluster_cookie(args.cookie.as_deref())?;
    let timeout = timeout(args.timeout_ms);
    let snapshot = query_operator_status_remote(&args.target, &cookie, timeout)
        .map_err(|error| error.to_string())?;

    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "target": args.target,
                "membership": {
                    "local_node": snapshot.membership.local_node,
                    "peer_nodes": snapshot.membership.peer_nodes,
                    "nodes": snapshot.membership.nodes,
                },
                "authority": {
                    "cluster_role": snapshot.authority.cluster_role,
                    "promotion_epoch": snapshot.authority.promotion_epoch,
                    "replication_health": snapshot.authority.replication_health,
                },
            }))
            .expect("serialize cluster status json")
        );
        return Ok(());
    }

    println!("target: {}", args.target);
    println!("local_node: {}", snapshot.membership.local_node);
    println!("peer_nodes:");
    if snapshot.membership.peer_nodes.is_empty() {
        println!("  - (none)");
    } else {
        for peer in &snapshot.membership.peer_nodes {
            println!("  - {}", peer);
        }
    }
    println!("nodes:");
    for node in &snapshot.membership.nodes {
        println!("  - {}", node);
    }
    println!("cluster_role: {}", snapshot.authority.cluster_role);
    println!("promotion_epoch: {}", snapshot.authority.promotion_epoch);
    println!(
        "replication_health: {}",
        snapshot.authority.replication_health
    );
    Ok(())
}

fn run_continuity(args: ClusterContinuityArgs) -> Result<(), String> {
    if args.request_key.is_some() && args.limit.is_some() {
        return Err(
            "meshc cluster continuity does not accept --limit when request_key is provided"
                .to_string(),
        );
    }

    let cookie = cluster_cookie(args.cookie.as_deref())?;
    let timeout = timeout(args.timeout_ms);

    if let Some(request_key) = args.request_key.as_deref() {
        let record =
            query_operator_continuity_status_remote(&args.target, &cookie, request_key, timeout)
                .map_err(|error| error.to_string())?;
        if args.json {
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "target": args.target,
                    "record": continuity_record_json(&record),
                }))
                .expect("serialize cluster continuity json")
            );
            return Ok(());
        }

        print_continuity_record(&record);
        return Ok(());
    }

    let list = query_operator_continuity_list_remote(&args.target, &cookie, args.limit, timeout)
        .map_err(|error| error.to_string())?;
    if args.json {
        let records: Vec<_> = list.records.iter().map(continuity_record_json).collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "target": args.target,
                "total_records": list.total_records,
                "truncated": list.truncated,
                "records": records,
            }))
            .expect("serialize cluster continuity list json")
        );
        return Ok(());
    }

    println!("target: {}", args.target);
    println!("total_records: {}", list.total_records);
    println!("truncated: {}", list.truncated);
    if list.records.is_empty() {
        println!("records: (none)");
        return Ok(());
    }

    println!("records:");
    for record in &list.records {
        println!(
            "- request_key={} attempt_id={} phase={} result={} owner={} replica={} replication_count={} execution={} declared_handler_runtime_name={} replica_status={} cluster_role={} promotion_epoch={} replication_health={} error={}",
            record.request_key,
            record.attempt_id,
            record.phase.as_str(),
            record.result.as_str(),
            record.owner_node,
            record.replica_node,
            record.replication_count,
            record.execution_node,
            record.declared_handler_runtime_name(),
            record.replica_status.as_str(),
            record.cluster_role.as_str(),
            record.promotion_epoch,
            record.replication_health.as_str(),
            record.error,
        );
    }
    Ok(())
}

fn run_diagnostics(args: ClusterDiagnosticsArgs) -> Result<(), String> {
    let cookie = cluster_cookie(args.cookie.as_deref())?;
    let timeout = timeout(args.timeout_ms);
    let snapshot = query_operator_diagnostics_remote(&args.target, &cookie, args.limit, timeout)
        .map_err(|error| error.to_string())?;

    if args.json {
        let entries: Vec<_> = snapshot
            .entries
            .iter()
            .map(|entry| {
                json!({
                    "sequence": entry.sequence,
                    "transition": entry.transition,
                    "request_key": entry.request_key,
                    "attempt_id": entry.attempt_id,
                    "owner_node": entry.owner_node,
                    "replica_node": entry.replica_node,
                    "execution_node": entry.execution_node,
                    "cluster_role": entry.cluster_role,
                    "promotion_epoch": entry.promotion_epoch,
                    "replication_health": entry.replication_health,
                    "replica_status": entry.replica_status,
                    "reason": entry.reason,
                    "metadata": entry
                        .metadata
                        .iter()
                        .map(|(key, value)| json!({"key": key, "value": value}))
                        .collect::<Vec<_>>(),
                })
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "target": args.target,
                "total_entries": snapshot.total_entries,
                "dropped_entries": snapshot.dropped_entries,
                "buffer_capacity": snapshot.buffer_capacity,
                "truncated": snapshot.truncated,
                "entries": entries,
            }))
            .expect("serialize cluster diagnostics json")
        );
        return Ok(());
    }

    println!("target: {}", args.target);
    println!("total_entries: {}", snapshot.total_entries);
    println!("dropped_entries: {}", snapshot.dropped_entries);
    println!("buffer_capacity: {}", snapshot.buffer_capacity);
    println!("truncated: {}", snapshot.truncated);
    if snapshot.entries.is_empty() {
        println!("entries: (none)");
        return Ok(());
    }

    println!("entries:");
    for entry in &snapshot.entries {
        println!(
            "- seq={} transition={} request_key={} attempt_id={} owner={} replica={} execution={} cluster_role={} promotion_epoch={} replication_health={} replica_status={} reason={}",
            entry.sequence,
            entry.transition,
            entry.request_key.as_deref().unwrap_or(""),
            entry.attempt_id.as_deref().unwrap_or(""),
            entry.owner_node.as_deref().unwrap_or(""),
            entry.replica_node.as_deref().unwrap_or(""),
            entry.execution_node.as_deref().unwrap_or(""),
            entry.cluster_role.as_deref().unwrap_or(""),
            entry
                .promotion_epoch
                .map(|value| value.to_string())
                .unwrap_or_default(),
            entry.replication_health.as_deref().unwrap_or(""),
            entry.replica_status.as_deref().unwrap_or(""),
            entry.reason.as_deref().unwrap_or(""),
        );
        for (key, value) in &entry.metadata {
            println!("    {}={}", key, value);
        }
    }
    Ok(())
}

fn cluster_cookie(explicit: Option<&str>) -> Result<String, String> {
    if let Some(value) = explicit.map(str::trim) {
        if value.is_empty() {
            return Err("meshc cluster: --cookie must not be blank".to_string());
        }
        return Ok(value.to_string());
    }

    match std::env::var("MESH_CLUSTER_COOKIE") {
        Ok(value) if !value.trim().is_empty() => Ok(value),
        Ok(_) => Err("meshc cluster: MESH_CLUSTER_COOKIE must not be blank".to_string()),
        Err(_) => Err(
            "meshc cluster: MESH_CLUSTER_COOKIE is required unless --cookie is provided"
                .to_string(),
        ),
    }
}

fn timeout(timeout_ms: u64) -> Duration {
    Duration::from_millis(timeout_ms.max(1))
}

fn continuity_record_json(record: &ContinuityRecord) -> serde_json::Value {
    json!({
        "request_key": record.request_key,
        "payload_hash": record.payload_hash,
        "attempt_id": record.attempt_id,
        "phase": record.phase.as_str(),
        "result": record.result.as_str(),
        "ingress_node": record.ingress_node,
        "owner_node": record.owner_node,
        "replica_node": record.replica_node,
        "replication_count": record.replication_count,
        "replica_status": record.replica_status.as_str(),
        "cluster_role": record.cluster_role.as_str(),
        "promotion_epoch": record.promotion_epoch,
        "replication_health": record.replication_health.as_str(),
        "execution_node": record.execution_node,
        "declared_handler_runtime_name": record.declared_handler_runtime_name(),
        "routed_remotely": record.routed_remotely,
        "fell_back_locally": record.fell_back_locally,
        "error": record.error,
    })
}

fn print_continuity_record(record: &ContinuityRecord) {
    println!("request_key: {}", record.request_key);
    println!("attempt_id: {}", record.attempt_id);
    println!("phase: {}", record.phase.as_str());
    println!("result: {}", record.result.as_str());
    println!("ingress_node: {}", record.ingress_node);
    println!("owner_node: {}", record.owner_node);
    println!("replica_node: {}", record.replica_node);
    println!("replication_count: {}", record.replication_count);
    println!("execution_node: {}", record.execution_node);
    println!(
        "declared_handler_runtime_name: {}",
        record.declared_handler_runtime_name()
    );
    println!("replica_status: {}", record.replica_status.as_str());
    println!("cluster_role: {}", record.cluster_role.as_str());
    println!("promotion_epoch: {}", record.promotion_epoch);
    println!("replication_health: {}", record.replication_health.as_str());
    println!("routed_remotely: {}", record.routed_remotely);
    println!("fell_back_locally: {}", record.fell_back_locally);
    println!("error: {}", record.error);
}
