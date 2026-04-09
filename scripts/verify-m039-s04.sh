#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# shellcheck source=scripts/lib/m039_cluster_proof.sh
source "$ROOT_DIR/scripts/lib/m039_cluster_proof.sh"

ARTIFACT_ROOT=".tmp/m039-s04"
ARTIFACT_DIR="$ARTIFACT_ROOT/verify"
PHASE_REPORT_PATH="$ARTIFACT_DIR/phase-report.txt"
STATUS_PATH="$ARTIFACT_DIR/status.txt"
CURRENT_PHASE_PATH="$ARTIFACT_DIR/current-phase.txt"
LIVE_LOG_DIR="$ARTIFACT_DIR/live-logs"
mkdir -p "$ARTIFACT_DIR" "$LIVE_LOG_DIR"
exec > >(tee "$ARTIFACT_DIR/full-contract.log") 2>&1

: >"$PHASE_REPORT_PATH"
printf 'running\n' >"$STATUS_PATH"
printf 'init\n' >"$CURRENT_PHASE_PATH"

RUN_ID="$(python3 - <<'PY'
import time
print(int(time.time() * 1000))
PY
)"
IMAGE_TAG="mesh-cluster-proof:m039-s04-local"
NETWORK_NAME="m039-s04-net-${RUN_ID}"
SEED_ALIAS="cluster-proof-seed"
SHARED_COOKIE="mesh-m039-s04-cookie"
NETWORK_INSPECT_PATH="$ARTIFACT_DIR/04-network.inspect.json"
IMAGE_INSPECT_PATH="$ARTIFACT_DIR/02-image.inspect.json"

NODE_A_NAME="m039-s04-node-a-${RUN_ID}"
NODE_A_HOSTNAME="node-a"
NODE_A_RUN1_STDOUT="$LIVE_LOG_DIR/${NODE_A_HOSTNAME}-run1.stdout.log"
NODE_A_RUN1_STDERR="$LIVE_LOG_DIR/${NODE_A_HOSTNAME}-run1.stderr.log"
NODE_A_RUN1_PID=""
NODE_A_HTTP_PORT=""
NODE_A_IP=""

NODE_B_RUN1_NAME="m039-s04-node-b-run1-${RUN_ID}"
NODE_B_RUN2_NAME="m039-s04-node-b-run2-${RUN_ID}"
NODE_B_HOSTNAME="node-b"
NODE_B_RUN1_STDOUT="$LIVE_LOG_DIR/${NODE_B_HOSTNAME}-run1.stdout.log"
NODE_B_RUN1_STDERR="$LIVE_LOG_DIR/${NODE_B_HOSTNAME}-run1.stderr.log"
NODE_B_RUN2_STDOUT="$LIVE_LOG_DIR/${NODE_B_HOSTNAME}-run2.stdout.log"
NODE_B_RUN2_STDERR="$LIVE_LOG_DIR/${NODE_B_HOSTNAME}-run2.stderr.log"
NODE_B_RUN1_PID=""
NODE_B_RUN2_PID=""
NODE_B_HTTP_PORT=""
NODE_B_IP=""
NODE_B_RUN2_HTTP_PORT=""
NODE_B_RUN2_IP=""

container_exists() {
  local name="$1"
  docker inspect "$name" >/dev/null 2>&1
}

container_running() {
  local name="$1"
  if ! container_exists "$name"; then
    return 1
  fi
  [[ "$(docker inspect -f '{{.State.Running}}' "$name")" == "true" ]]
}

docker_container_ip() {
  local name="$1"
  docker inspect -f "{{with index .NetworkSettings.Networks \"$NETWORK_NAME\"}}{{.IPAddress}}{{end}}" "$name"
}

docker_host_port() {
  local name="$1"
  docker inspect -f '{{(index (index .NetworkSettings.Ports "8080/tcp") 0).HostPort}}' "$name"
}

wait_for_container_http() {
  local phase="$1"
  local container_name="$2"
  local host_port="$3"
  local artifact_path="$4"
  local timeout_secs="$5"
  local deadline=$((SECONDS + timeout_secs))
  local last_issue=""

  while (( SECONDS < deadline )); do
    if ! container_running "$container_name"; then
      echo "container ${container_name} is not running" >"$artifact_path"
      return 1
    fi

    if curl --silent --show-error --max-time 2 --fail "http://127.0.0.1:${host_port}/membership" >"$artifact_path" 2>"$ARTIFACT_DIR/${phase}.curl.log"; then
      return 0
    fi

    last_issue="$(<"$ARTIFACT_DIR/${phase}.curl.log")"
    sleep 1
  done

  echo "timed out waiting for http://127.0.0.1:${host_port}/membership :: ${last_issue}" >"$artifact_path"
  return 1
}

create_container() {
  local phase="$1"
  local container_name="$2"
  local hostname="$3"
  local log_path="$4"

  if ! docker create \
    --name "$container_name" \
    --hostname "$hostname" \
    --network "$NETWORK_NAME" \
    --network-alias "$hostname" \
    --network-alias "$SEED_ALIAS" \
    -p 127.0.0.1::8080 \
    -e CLUSTER_PROOF_COOKIE="$SHARED_COOKIE" \
    -e MESH_DISCOVERY_SEED="$SEED_ALIAS" \
    "$IMAGE_TAG" >"$log_path" 2>&1; then
    m039_fail_phase "$phase" "docker create failed for ${container_name}" "$log_path"
  fi
}

start_container_attached() {
  local phase="$1"
  local container_name="$2"
  local stdout_path="$3"
  local stderr_path="$4"
  local pid_var_name="$5"

  : >"$stdout_path"
  : >"$stderr_path"
  docker start -a "$container_name" >"$stdout_path" 2>"$stderr_path" &
  local start_pid=$!
  printf -v "$pid_var_name" '%s' "$start_pid"

  sleep 1
  if ! container_running "$container_name"; then
    local inspect_log="$ARTIFACT_DIR/${phase}.${container_name}.inspect.log"
    docker inspect "$container_name" >"$inspect_log" 2>&1 || true
    m039_fail_phase "$phase" "container ${container_name} exited before readiness" "$inspect_log" "$stdout_path"
  fi
}

stop_and_wait_container() {
  local container_name="$1"
  local start_pid="$2"
  local stop_log="$3"

  if container_exists "$container_name"; then
    docker stop -t 2 "$container_name" >"$stop_log" 2>&1 || true
  fi
  if [[ -n "$start_pid" ]]; then
    wait "$start_pid" 2>/dev/null || true
  fi
}

remove_container_if_exists() {
  local container_name="$1"
  local remove_log="$2"
  if container_exists "$container_name"; then
    docker rm -f "$container_name" >"$remove_log" 2>&1 || true
  fi
}

copy_container_snapshot() {
  local phase="$1"
  local phase_dir="$2"
  local container_name="$3"
  local stdout_path="$4"
  local stderr_path="$5"
  local snapshot_prefix="$6"

  mkdir -p "$phase_dir"
  local manifest_path="$phase_dir/manifest.txt"
  local inspect_path="$phase_dir/${snapshot_prefix}.inspect.json"

  if container_exists "$container_name"; then
    docker inspect "$container_name" >"$inspect_path" 2>&1 || true
    printf '%s\n' "$inspect_path" >>"$manifest_path"
  fi

  m039_assert_file_exists "$phase" "$stdout_path" "container stdout log for ${container_name}" "$ARTIFACT_DIR/full-contract.log"
  m039_assert_file_exists "$phase" "$stderr_path" "container stderr log for ${container_name}" "$ARTIFACT_DIR/full-contract.log"

  local copied_stdout="$phase_dir/${snapshot_prefix}.stdout.log"
  local copied_stderr="$phase_dir/${snapshot_prefix}.stderr.log"
  cp "$stdout_path" "$copied_stdout"
  cp "$stderr_path" "$copied_stderr"
  printf '%s\n%s\n' "$copied_stdout" "$copied_stderr" >>"$manifest_path"
}

write_phase_manifest_header() {
  local phase_dir="$1"
  local label="$2"
  mkdir -p "$phase_dir"
  cat >"$phase_dir/manifest.txt" <<EOF
${label}
EOF
}

wait_for_membership_probe() {
  local phase="$1"
  local description="$2"
  local url="$3"
  local artifact_path="$4"
  local timeout_secs="$5"
  local expected_self="$6"
  local expected_membership_csv="$7"
  local expected_peers_csv="$8"
  local deadline=$((SECONDS + timeout_secs))
  local curl_log="$ARTIFACT_DIR/${phase}.curl.log"
  local check_log="$ARTIFACT_DIR/${phase}.membership-check.log"

  while (( SECONDS < deadline )); do
    if curl --silent --show-error --max-time 2 --fail "$url" >"$artifact_path" 2>"$curl_log"; then
      if m039_assert_membership_json "$artifact_path" "$expected_self" "$expected_membership_csv" "$expected_peers_csv" "$description" >"$check_log" 2>&1; then
        return 0
      fi
    else
      cp "$curl_log" "$artifact_path" 2>/dev/null || true
    fi
    sleep 1
  done

  return 1
}

wait_for_work_probe() {
  local phase="$1"
  local description="$2"
  local url="$3"
  local artifact_path="$4"
  local timeout_secs="$5"
  local mode="$6"
  local expected_ingress="$7"
  local expected_target="$8"
  local expected_execution="$9"
  local deadline=$((SECONDS + timeout_secs))
  local curl_log="$ARTIFACT_DIR/${phase}.curl.log"
  local check_log="$ARTIFACT_DIR/${phase}.work-check.log"

  while (( SECONDS < deadline )); do
    if curl --silent --show-error --max-time 2 --fail "$url" >"$artifact_path" 2>"$curl_log"; then
      if m039_assert_work_json "$artifact_path" "$mode" "$expected_ingress" "$expected_target" "$expected_execution" "$description" >"$check_log" 2>&1; then
        return 0
      fi
    else
      cp "$curl_log" "$artifact_path" 2>/dev/null || true
    fi
    sleep 1
  done

  return 1
}

assert_dns_preflight() {
  local phase="$1"
  local output_path="$2"
  local expected_ip_csv="$3"
  local description="$4"
  local check_log="$ARTIFACT_DIR/${phase}.dns-check.log"

  if ! python3 - "$output_path" "$expected_ip_csv" "$description" >"$check_log" 2>&1 <<'PY'
from pathlib import Path
import re
import sys

output_path = Path(sys.argv[1])
expected_ips = sorted(value for value in sys.argv[2].split(',') if value)
description = sys.argv[3]
text = output_path.read_text(errors='replace')
ips = sorted(set(re.findall(r'\b\d+\.\d+\.\d+\.\d+\b', text)))
if len(ips) < 2:
    raise SystemExit(f"{description}: expected at least 2 resolved IPs, found {ips!r}")
if ips != expected_ips:
    raise SystemExit(f"{description}: expected IPs {expected_ips!r}, found {ips!r}")
print(f"{description}: resolved {ips}")
PY
  then
    m039_fail_phase "$phase" "seed-resolution preflight drifted" "$check_log" "$output_path"
  fi
}

cleanup() {
  local exit_code=$?

  for pair in \
    "$NODE_A_NAME|$NODE_A_RUN1_PID|$ARTIFACT_DIR/cleanup-node-a.stop.log" \
    "$NODE_B_RUN1_NAME|$NODE_B_RUN1_PID|$ARTIFACT_DIR/cleanup-node-b-run1.stop.log" \
    "$NODE_B_RUN2_NAME|$NODE_B_RUN2_PID|$ARTIFACT_DIR/cleanup-node-b-run2.stop.log"; do
    IFS='|' read -r name pid stop_log <<<"$pair"
    if container_running "$name"; then
      docker stop -t 2 "$name" >"$stop_log" 2>&1 || true
    fi
    if [[ -n "$pid" ]]; then
      wait "$pid" 2>/dev/null || true
    fi
  done

  remove_container_if_exists "$NODE_A_NAME" "$ARTIFACT_DIR/cleanup-node-a.rm.log"
  remove_container_if_exists "$NODE_B_RUN1_NAME" "$ARTIFACT_DIR/cleanup-node-b-run1.rm.log"
  remove_container_if_exists "$NODE_B_RUN2_NAME" "$ARTIFACT_DIR/cleanup-node-b-run2.rm.log"

  if docker network inspect "$NETWORK_NAME" >"$ARTIFACT_DIR/cleanup-network.inspect.json" 2>&1; then
    docker network rm "$NETWORK_NAME" >"$ARTIFACT_DIR/cleanup-network.rm.log" 2>&1 || true
  fi

  if [[ $exit_code -eq 0 ]]; then
    printf 'ok\n' >"$STATUS_PATH"
    printf 'complete\n' >"$CURRENT_PHASE_PATH"
  elif [[ ! -f "$STATUS_PATH" || "$(<"$STATUS_PATH")" != "failed" ]]; then
    printf 'failed\n' >"$STATUS_PATH"
  fi
}
trap cleanup EXIT

m039_run_expect_success s03-contract 01-s03-contract no 1200 \
  bash scripts/verify-m039-s03.sh
m039_assert_file_exists s03-contract .tmp/m039-s03/verify/phase-report.txt "S03 phase report" "$ARTIFACT_DIR/01-s03-contract.log"
m039_assert_file_exists s03-contract .tmp/m039-s03/verify/status.txt "S03 status" "$ARTIFACT_DIR/01-s03-contract.log"
m039_assert_file_exists s03-contract .tmp/m039-s03/verify/current-phase.txt "S03 current phase" "$ARTIFACT_DIR/01-s03-contract.log"
cp .tmp/m039-s03/verify/phase-report.txt "$ARTIFACT_DIR/01-s03-phase-report.txt"
cp .tmp/m039-s03/verify/status.txt "$ARTIFACT_DIR/01-s03-status.txt"
cp .tmp/m039-s03/verify/current-phase.txt "$ARTIFACT_DIR/01-s03-current-phase.txt"
m039_assert_file_contains_regex s03-contract "$ARTIFACT_DIR/01-s03-phase-report.txt" '^cluster-proof-tests	passed$' "S03 cluster-proof test replay did not pass" "$ARTIFACT_DIR/01-s03-contract.log"
m039_assert_file_contains_regex s03-contract "$ARTIFACT_DIR/01-s03-phase-report.txt" '^build-cluster-proof	passed$' "S03 build replay did not pass" "$ARTIFACT_DIR/01-s03-contract.log"
m039_assert_file_contains_regex s03-contract "$ARTIFACT_DIR/01-s03-phase-report.txt" '^s01-contract	passed$' "S03 S01 replay did not pass" "$ARTIFACT_DIR/01-s03-contract.log"
m039_assert_file_contains_regex s03-contract "$ARTIFACT_DIR/01-s03-phase-report.txt" '^s02-contract	passed$' "S03 S02 replay did not pass" "$ARTIFACT_DIR/01-s03-contract.log"
m039_assert_file_contains_regex s03-contract "$ARTIFACT_DIR/01-s03-phase-report.txt" '^s03-degrade	passed$' "S03 degrade phase did not pass" "$ARTIFACT_DIR/01-s03-contract.log"
m039_assert_file_contains_regex s03-contract "$ARTIFACT_DIR/01-s03-phase-report.txt" '^s03-rejoin	passed$' "S03 rejoin phase did not pass" "$ARTIFACT_DIR/01-s03-contract.log"
m039_assert_file_contains_regex s03-contract "$ARTIFACT_DIR/01-s03-status.txt" '^ok$' "S03 status must be ok" "$ARTIFACT_DIR/01-s03-contract.log"
m039_assert_file_contains_regex s03-contract "$ARTIFACT_DIR/01-s03-current-phase.txt" '^complete$' "S03 current phase must be complete" "$ARTIFACT_DIR/01-s03-contract.log"

m039_run_expect_success docker-image-build 02-docker-image-build no 1800 \
  docker build --progress=plain -f cluster-proof/Dockerfile -t "$IMAGE_TAG" .
docker image inspect "$IMAGE_TAG" >"$IMAGE_INSPECT_PATH" 2>&1 || m039_fail_phase docker-image-build "image inspect failed after build" "$ARTIFACT_DIR/02-docker-image-build.log" "$IMAGE_INSPECT_PATH"

m039_record_phase docker-network started
printf 'docker-network\n' >"$CURRENT_PHASE_PATH"
echo "==> docker network create ${NETWORK_NAME}"
if ! docker network create "$NETWORK_NAME" >"$ARTIFACT_DIR/03-docker-network.log" 2>&1; then
  m039_record_phase docker-network failed
  m039_fail_phase docker-network "failed to create docker bridge network" "$ARTIFACT_DIR/03-docker-network.log"
fi
m039_record_phase docker-network passed

m039_record_phase docker-start started
printf 'docker-start\n' >"$CURRENT_PHASE_PATH"
create_container docker-start "$NODE_A_NAME" "$NODE_A_HOSTNAME" "$ARTIFACT_DIR/04-node-a.create.log"
create_container docker-start "$NODE_B_RUN1_NAME" "$NODE_B_HOSTNAME" "$ARTIFACT_DIR/04-node-b-run1.create.log"
start_container_attached docker-start "$NODE_A_NAME" "$NODE_A_RUN1_STDOUT" "$NODE_A_RUN1_STDERR" NODE_A_RUN1_PID
start_container_attached docker-start "$NODE_B_RUN1_NAME" "$NODE_B_RUN1_STDOUT" "$NODE_B_RUN1_STDERR" NODE_B_RUN1_PID
NODE_A_HTTP_PORT="$(docker_host_port "$NODE_A_NAME")"
NODE_B_HTTP_PORT="$(docker_host_port "$NODE_B_RUN1_NAME")"
NODE_A_IP="$(docker_container_ip "$NODE_A_NAME")"
NODE_B_IP="$(docker_container_ip "$NODE_B_RUN1_NAME")"
if [[ -z "$NODE_A_HTTP_PORT" || -z "$NODE_B_HTTP_PORT" || -z "$NODE_A_IP" || -z "$NODE_B_IP" ]]; then
  docker network inspect "$NETWORK_NAME" >"$NETWORK_INSPECT_PATH" 2>&1 || true
  m039_record_phase docker-start failed
  m039_fail_phase docker-start "missing host-port or IP metadata after container start" "$ARTIFACT_DIR/04-node-a.create.log" "$NETWORK_INSPECT_PATH"
fi
if ! wait_for_container_http docker-start "$NODE_A_NAME" "$NODE_A_HTTP_PORT" "$ARTIFACT_DIR/04-node-a.ready.json" 45; then
  docker inspect "$NODE_A_NAME" >"$ARTIFACT_DIR/04-node-a.inspect.json" 2>&1 || true
  m039_record_phase docker-start failed
  m039_fail_phase docker-start "node-a never served /membership" "$ARTIFACT_DIR/04-node-a.inspect.json" "$ARTIFACT_DIR/04-node-a.ready.json"
fi
if ! wait_for_container_http docker-start "$NODE_B_RUN1_NAME" "$NODE_B_HTTP_PORT" "$ARTIFACT_DIR/04-node-b-run1.ready.json" 45; then
  docker inspect "$NODE_B_RUN1_NAME" >"$ARTIFACT_DIR/04-node-b-run1.inspect.json" 2>&1 || true
  m039_record_phase docker-start failed
  m039_fail_phase docker-start "node-b run1 never served /membership" "$ARTIFACT_DIR/04-node-b-run1.inspect.json" "$ARTIFACT_DIR/04-node-b-run1.ready.json"
fi
docker network inspect "$NETWORK_NAME" >"$NETWORK_INSPECT_PATH" 2>&1 || true
m039_record_phase docker-start passed

m039_record_phase dns-preflight started
printf 'dns-preflight\n' >"$CURRENT_PHASE_PATH"
DNS_DIR="$ARTIFACT_DIR/05-dns-preflight"
mkdir -p "$DNS_DIR"
write_phase_manifest_header "$DNS_DIR" "dns preflight"
printf '%s\n' "$NETWORK_INSPECT_PATH" >>"$DNS_DIR/manifest.txt"
if ! docker exec "$NODE_A_NAME" getent ahostsv4 "$SEED_ALIAS" >"$DNS_DIR/node-a-seed-resolution.txt" 2>"$DNS_DIR/node-a-seed-resolution.stderr.txt"; then
  m039_record_phase dns-preflight failed
  m039_fail_phase dns-preflight "node-a could not resolve shared discovery alias" "$DNS_DIR/node-a-seed-resolution.stderr.txt" "$DNS_DIR/node-a-seed-resolution.txt"
fi
if ! docker exec "$NODE_B_RUN1_NAME" getent ahostsv4 "$SEED_ALIAS" >"$DNS_DIR/node-b-seed-resolution.txt" 2>"$DNS_DIR/node-b-seed-resolution.stderr.txt"; then
  m039_record_phase dns-preflight failed
  m039_fail_phase dns-preflight "node-b could not resolve shared discovery alias" "$DNS_DIR/node-b-seed-resolution.stderr.txt" "$DNS_DIR/node-b-seed-resolution.txt"
fi
printf '%s\n%s\n%s\n%s\n' \
  "$DNS_DIR/node-a-seed-resolution.txt" \
  "$DNS_DIR/node-a-seed-resolution.stderr.txt" \
  "$DNS_DIR/node-b-seed-resolution.txt" \
  "$DNS_DIR/node-b-seed-resolution.stderr.txt" >>"$DNS_DIR/manifest.txt"
EXPECTED_IPS_CSV="$(printf '%s,%s' "$NODE_A_IP" "$NODE_B_IP")"
assert_dns_preflight dns-preflight "$DNS_DIR/node-a-seed-resolution.txt" "$EXPECTED_IPS_CSV" "node-a shared-seed resolution"
assert_dns_preflight dns-preflight "$DNS_DIR/node-b-seed-resolution.txt" "$EXPECTED_IPS_CSV" "node-b shared-seed resolution"
m039_record_phase dns-preflight passed

EXPECTED_NODE_A="${NODE_A_HOSTNAME}@${NODE_A_HOSTNAME}:4370"
EXPECTED_NODE_B="${NODE_B_HOSTNAME}@${NODE_B_HOSTNAME}:4370"

m039_record_phase convergence started
printf 'convergence\n' >"$CURRENT_PHASE_PATH"
PRE_LOSS_DIR="$ARTIFACT_DIR/06-pre-loss"
mkdir -p "$PRE_LOSS_DIR"
write_phase_manifest_header "$PRE_LOSS_DIR" "pre-loss cluster proof"
PRE_LOSS_A_MEMBERSHIP="$PRE_LOSS_DIR/pre-loss-node-a-membership.json"
PRE_LOSS_B_MEMBERSHIP="$PRE_LOSS_DIR/pre-loss-node-b-membership.json"
PRE_LOSS_WORK="$PRE_LOSS_DIR/pre-loss-work.json"
if ! wait_for_membership_probe convergence "pre-loss membership on node-a" "http://127.0.0.1:${NODE_A_HTTP_PORT}/membership" "$PRE_LOSS_A_MEMBERSHIP" 60 "$EXPECTED_NODE_A" "${EXPECTED_NODE_A},${EXPECTED_NODE_B}" "$EXPECTED_NODE_B"; then
  m039_record_phase convergence failed
  m039_fail_phase convergence "node-a never converged to two-node membership" "$ARTIFACT_DIR/convergence.membership-check.log" "$PRE_LOSS_A_MEMBERSHIP"
fi
if ! wait_for_membership_probe convergence "pre-loss membership on node-b" "http://127.0.0.1:${NODE_B_HTTP_PORT}/membership" "$PRE_LOSS_B_MEMBERSHIP" 60 "$EXPECTED_NODE_B" "${EXPECTED_NODE_A},${EXPECTED_NODE_B}" "$EXPECTED_NODE_A"; then
  m039_record_phase convergence failed
  m039_fail_phase convergence "node-b never converged to two-node membership" "$ARTIFACT_DIR/convergence.membership-check.log" "$PRE_LOSS_B_MEMBERSHIP"
fi
if ! wait_for_work_probe convergence "pre-loss /work on node-a" "http://127.0.0.1:${NODE_A_HTTP_PORT}/work" "$PRE_LOSS_WORK" 30 remote "$EXPECTED_NODE_A" "$EXPECTED_NODE_B" "$EXPECTED_NODE_B"; then
  m039_record_phase convergence failed
  m039_fail_phase convergence "pre-loss /work never proved remote execution" "$ARTIFACT_DIR/convergence.work-check.log" "$PRE_LOSS_WORK"
fi
printf '%s\n%s\n%s\n' "$PRE_LOSS_A_MEMBERSHIP" "$PRE_LOSS_B_MEMBERSHIP" "$PRE_LOSS_WORK" >>"$PRE_LOSS_DIR/manifest.txt"
copy_container_snapshot convergence "$PRE_LOSS_DIR" "$NODE_A_NAME" "$NODE_A_RUN1_STDOUT" "$NODE_A_RUN1_STDERR" "node-a-run1"
copy_container_snapshot convergence "$PRE_LOSS_DIR" "$NODE_B_RUN1_NAME" "$NODE_B_RUN1_STDOUT" "$NODE_B_RUN1_STDERR" "node-b-run1"
m039_record_phase convergence passed

m039_record_phase degrade started
printf 'degrade\n' >"$CURRENT_PHASE_PATH"
DEGRADED_DIR="$ARTIFACT_DIR/07-degraded"
mkdir -p "$DEGRADED_DIR"
write_phase_manifest_header "$DEGRADED_DIR" "degraded cluster proof"
stop_and_wait_container "$NODE_B_RUN1_NAME" "$NODE_B_RUN1_PID" "$DEGRADED_DIR/node-b-run1.stop.log"
docker inspect "$NODE_B_RUN1_NAME" >"$DEGRADED_DIR/node-b-run1.inspect.json" 2>&1 || true
remove_container_if_exists "$NODE_B_RUN1_NAME" "$DEGRADED_DIR/node-b-run1.rm.log"
DEGRADED_MEMBERSHIP="$DEGRADED_DIR/degraded-node-a-membership.json"
DEGRADED_WORK="$DEGRADED_DIR/degraded-work.json"
if ! wait_for_membership_probe degrade "degraded membership on node-a" "http://127.0.0.1:${NODE_A_HTTP_PORT}/membership" "$DEGRADED_MEMBERSHIP" 60 "$EXPECTED_NODE_A" "$EXPECTED_NODE_A" ""; then
  m039_record_phase degrade failed
  m039_fail_phase degrade "node-a never reported self-only degraded membership" "$ARTIFACT_DIR/degrade.membership-check.log" "$DEGRADED_MEMBERSHIP"
fi
if ! wait_for_work_probe degrade "degraded /work on node-a" "http://127.0.0.1:${NODE_A_HTTP_PORT}/work" "$DEGRADED_WORK" 30 local "$EXPECTED_NODE_A" "$EXPECTED_NODE_A" "$EXPECTED_NODE_A"; then
  m039_record_phase degrade failed
  m039_fail_phase degrade "degraded /work never fell back locally" "$ARTIFACT_DIR/degrade.work-check.log" "$DEGRADED_WORK"
fi
printf '%s\n%s\n%s\n%s\n' \
  "$DEGRADED_MEMBERSHIP" \
  "$DEGRADED_WORK" \
  "$DEGRADED_DIR/node-b-run1.stop.log" \
  "$DEGRADED_DIR/node-b-run1.inspect.json" >>"$DEGRADED_DIR/manifest.txt"
copy_container_snapshot degrade "$DEGRADED_DIR" "$NODE_A_NAME" "$NODE_A_RUN1_STDOUT" "$NODE_A_RUN1_STDERR" "node-a-run1"
cp "$NODE_B_RUN1_STDOUT" "$DEGRADED_DIR/node-b-run1.stdout.log"
cp "$NODE_B_RUN1_STDERR" "$DEGRADED_DIR/node-b-run1.stderr.log"
printf '%s\n%s\n' "$DEGRADED_DIR/node-b-run1.stdout.log" "$DEGRADED_DIR/node-b-run1.stderr.log" >>"$DEGRADED_DIR/manifest.txt"
NODE_B_RUN1_PID=""
m039_record_phase degrade passed

m039_record_phase rejoin started
printf 'rejoin\n' >"$CURRENT_PHASE_PATH"
POST_REJOIN_DIR="$ARTIFACT_DIR/08-post-rejoin"
mkdir -p "$POST_REJOIN_DIR"
write_phase_manifest_header "$POST_REJOIN_DIR" "post-rejoin cluster proof"
create_container rejoin "$NODE_B_RUN2_NAME" "$NODE_B_HOSTNAME" "$ARTIFACT_DIR/08-node-b-run2.create.log"
start_container_attached rejoin "$NODE_B_RUN2_NAME" "$NODE_B_RUN2_STDOUT" "$NODE_B_RUN2_STDERR" NODE_B_RUN2_PID
NODE_B_RUN2_HTTP_PORT="$(docker_host_port "$NODE_B_RUN2_NAME")"
NODE_B_RUN2_IP="$(docker_container_ip "$NODE_B_RUN2_NAME")"
if [[ -z "$NODE_B_RUN2_HTTP_PORT" || -z "$NODE_B_RUN2_IP" ]]; then
  m039_record_phase rejoin failed
  m039_fail_phase rejoin "missing node-b run2 host-port or IP metadata" "$ARTIFACT_DIR/08-node-b-run2.create.log"
fi
if ! wait_for_container_http rejoin "$NODE_B_RUN2_NAME" "$NODE_B_RUN2_HTTP_PORT" "$POST_REJOIN_DIR/node-b-run2.ready.json" 45; then
  docker inspect "$NODE_B_RUN2_NAME" >"$POST_REJOIN_DIR/node-b-run2.inspect.json" 2>&1 || true
  m039_record_phase rejoin failed
  m039_fail_phase rejoin "node-b run2 never served /membership" "$POST_REJOIN_DIR/node-b-run2.inspect.json" "$POST_REJOIN_DIR/node-b-run2.ready.json"
fi
docker network inspect "$NETWORK_NAME" >"$POST_REJOIN_DIR/network.inspect.json" 2>&1 || true
POST_REJOIN_A_MEMBERSHIP="$POST_REJOIN_DIR/post-rejoin-node-a-membership.json"
POST_REJOIN_B_MEMBERSHIP="$POST_REJOIN_DIR/post-rejoin-node-b-membership.json"
POST_REJOIN_WORK="$POST_REJOIN_DIR/post-rejoin-work.json"
if ! wait_for_membership_probe rejoin "post-rejoin membership on node-a" "http://127.0.0.1:${NODE_A_HTTP_PORT}/membership" "$POST_REJOIN_A_MEMBERSHIP" 60 "$EXPECTED_NODE_A" "${EXPECTED_NODE_A},${EXPECTED_NODE_B}" "$EXPECTED_NODE_B"; then
  m039_record_phase rejoin failed
  m039_fail_phase rejoin "node-a never restored two-node membership after rejoin" "$ARTIFACT_DIR/rejoin.membership-check.log" "$POST_REJOIN_A_MEMBERSHIP"
fi
if ! wait_for_membership_probe rejoin "post-rejoin membership on node-b" "http://127.0.0.1:${NODE_B_RUN2_HTTP_PORT}/membership" "$POST_REJOIN_B_MEMBERSHIP" 60 "$EXPECTED_NODE_B" "${EXPECTED_NODE_A},${EXPECTED_NODE_B}" "$EXPECTED_NODE_A"; then
  m039_record_phase rejoin failed
  m039_fail_phase rejoin "node-b run2 never rejoined with the same identity" "$ARTIFACT_DIR/rejoin.membership-check.log" "$POST_REJOIN_B_MEMBERSHIP"
fi
if ! wait_for_work_probe rejoin "post-rejoin /work on node-a" "http://127.0.0.1:${NODE_A_HTTP_PORT}/work" "$POST_REJOIN_WORK" 30 remote "$EXPECTED_NODE_A" "$EXPECTED_NODE_B" "$EXPECTED_NODE_B"; then
  m039_record_phase rejoin failed
  m039_fail_phase rejoin "post-rejoin /work never restored remote execution" "$ARTIFACT_DIR/rejoin.work-check.log" "$POST_REJOIN_WORK"
fi
printf '%s\n%s\n%s\n%s\n' \
  "$POST_REJOIN_A_MEMBERSHIP" \
  "$POST_REJOIN_B_MEMBERSHIP" \
  "$POST_REJOIN_WORK" \
  "$POST_REJOIN_DIR/network.inspect.json" >>"$POST_REJOIN_DIR/manifest.txt"
copy_container_snapshot rejoin "$POST_REJOIN_DIR" "$NODE_A_NAME" "$NODE_A_RUN1_STDOUT" "$NODE_A_RUN1_STDERR" "node-a-run1"
cp "$NODE_B_RUN1_STDOUT" "$POST_REJOIN_DIR/node-b-run1.stdout.log"
cp "$NODE_B_RUN1_STDERR" "$POST_REJOIN_DIR/node-b-run1.stderr.log"
printf '%s\n%s\n' "$POST_REJOIN_DIR/node-b-run1.stdout.log" "$POST_REJOIN_DIR/node-b-run1.stderr.log" >>"$POST_REJOIN_DIR/manifest.txt"
copy_container_snapshot rejoin "$POST_REJOIN_DIR" "$NODE_B_RUN2_NAME" "$NODE_B_RUN2_STDOUT" "$NODE_B_RUN2_STDERR" "node-b-run2"
m039_record_phase rejoin passed

echo "verify-m039-s04: ok"
