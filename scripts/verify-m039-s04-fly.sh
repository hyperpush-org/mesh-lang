#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# shellcheck source=scripts/lib/m039_cluster_proof.sh
source "$ROOT_DIR/scripts/lib/m039_cluster_proof.sh"

ARTIFACT_DIR=".tmp/m039-s04/fly"
PHASE_REPORT_PATH="$ARTIFACT_DIR/phase-report.txt"
STATUS_PATH="$ARTIFACT_DIR/status.txt"
CURRENT_PHASE_PATH="$ARTIFACT_DIR/current-phase.txt"
FULL_LOG_PATH="$ARTIFACT_DIR/full-contract.log"
REQUIRED_RUNNING_MACHINES=2

usage() {
  cat <<'EOF'
Usage:
  bash scripts/verify-m039-s04-fly.sh --help
  CLUSTER_PROOF_FLY_APP=<fly-app> \
  CLUSTER_PROOF_BASE_URL=https://<fly-app>.fly.dev \
    bash scripts/verify-m039-s04-fly.sh

Read-only Fly verifier for the cluster-proof operator path.

Required live-mode environment:
  CLUSTER_PROOF_FLY_APP   Existing Fly app name to inspect.
  CLUSTER_PROOF_BASE_URL  Base URL for the deployed app. Must match
                          https://<CLUSTER_PROOF_FLY_APP>.fly.dev (port allowed).

What it does in live mode:
  - fly status --json
  - fly config show
  - fly logs --no-tail
  - GET /membership
  - GET /work

Artifacts:
  .tmp/m039-s04/fly/

This script does not deploy, scale, mutate secrets, or change Fly state.
EOF
}

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  usage
  exit 0
fi

if [[ $# -ne 0 ]]; then
  usage >&2
  exit 1
fi

mkdir -p "$ARTIFACT_DIR"
: >"$PHASE_REPORT_PATH"
printf 'running\n' >"$STATUS_PATH"
printf 'init\n' >"$CURRENT_PHASE_PATH"
exec > >(tee "$FULL_LOG_PATH") 2>&1

cleanup() {
  local exit_code=$?
  if [[ $exit_code -eq 0 ]]; then
    printf 'ok\n' >"$STATUS_PATH"
    printf 'complete\n' >"$CURRENT_PHASE_PATH"
  elif [[ ! -f "$STATUS_PATH" || "$(<"$STATUS_PATH")" != "failed" ]]; then
    printf 'failed\n' >"$STATUS_PATH"
  fi
}
trap cleanup EXIT

require_command() {
  local phase="$1"
  local name="$2"
  local log_path="$ARTIFACT_DIR/${phase}.${name}.check.log"
  if ! command -v "$name" >"$log_path" 2>&1; then
    m039_fail_phase "$phase" "required command not found: ${name}" "$log_path"
  fi
}

run_capture() {
  local timeout_secs="$1"
  local stdout_path="$2"
  local stderr_path="$3"
  shift 3
  local -a cmd=("$@")

  {
    printf '$'
    printf ' %q' "${cmd[@]}"
    printf '\n'
  } >"$stderr_path"

  "${cmd[@]}" >"$stdout_path" 2>>"$stderr_path" &
  local cmd_pid=$!
  local deadline=$((SECONDS + timeout_secs))

  while kill -0 "$cmd_pid" 2>/dev/null; do
    if (( SECONDS >= deadline )); then
      echo "command timed out after ${timeout_secs}s" >>"$stderr_path"
      kill -TERM "$cmd_pid" 2>/dev/null || true
      sleep 1
      kill -KILL "$cmd_pid" 2>/dev/null || true
      wait "$cmd_pid" 2>/dev/null || true
      return 124
    fi
    sleep 1
  done

  wait "$cmd_pid"
}

run_phase_capture() {
  local phase="$1"
  local label="$2"
  local timeout_secs="$3"
  shift 3
  local stdout_path="$ARTIFACT_DIR/${label}.stdout"
  local stderr_path="$ARTIFACT_DIR/${label}.stderr.log"

  m039_record_phase "$phase" started
  printf '%s\n' "$phase" >"$CURRENT_PHASE_PATH"
  echo "==> ${*}"
  if ! run_capture "$timeout_secs" "$stdout_path" "$stderr_path" "$@"; then
    m039_record_phase "$phase" failed
    m039_fail_phase "$phase" "expected success within ${timeout_secs}s" "$stderr_path" "$stdout_path"
  fi
  m039_record_phase "$phase" passed
}

validate_inputs() {
  local phase="input-validation"
  local validation_log="$ARTIFACT_DIR/${phase}.log"
  local output_json="$ARTIFACT_DIR/${phase}.json"

  m039_record_phase "$phase" started
  printf '%s\n' "$phase" >"$CURRENT_PHASE_PATH"

  if ! python3 - "$output_json" <<'PY' >"$validation_log" 2>&1
from pathlib import Path
from urllib.parse import urlparse
import json
import os
import sys

output_path = Path(sys.argv[1])
app = os.environ.get("CLUSTER_PROOF_FLY_APP", "").strip()
base_url_raw = os.environ.get("CLUSTER_PROOF_BASE_URL", "").strip()

if not app:
    raise SystemExit("CLUSTER_PROOF_FLY_APP is required")
if not base_url_raw:
    raise SystemExit("CLUSTER_PROOF_BASE_URL is required")

parsed = urlparse(base_url_raw)
if parsed.scheme not in {"http", "https"}:
    raise SystemExit(f"CLUSTER_PROOF_BASE_URL must use http or https, found {parsed.scheme!r}")
if parsed.username or parsed.password:
    raise SystemExit("CLUSTER_PROOF_BASE_URL must not contain userinfo")
if not parsed.hostname:
    raise SystemExit("CLUSTER_PROOF_BASE_URL must include a hostname")
if parsed.query or parsed.fragment:
    raise SystemExit("CLUSTER_PROOF_BASE_URL must not include query or fragment components")
if parsed.path not in {"", "/"}:
    raise SystemExit("CLUSTER_PROOF_BASE_URL must be a base URL without a path suffix")

expected_host = f"{app}.fly.dev"
if parsed.hostname != expected_host:
    raise SystemExit(
        "CLUSTER_PROOF_BASE_URL host mismatch: "
        f"expected {expected_host!r} for app {app!r}, found {parsed.hostname!r}"
    )

normalized_base_url = f"{parsed.scheme}://{parsed.netloc}".rstrip("/")
summary = {
    "app": app,
    "base_url": normalized_base_url,
    "expected_host": expected_host,
    "membership_url": f"{normalized_base_url}/membership",
    "work_url": f"{normalized_base_url}/work",
    "required_running_machines": 2,
    "read_only_commands": [
        f"fly status -a {app} --json",
        f"fly config show -a {app}",
        f"fly logs -a {app} --no-tail",
        f"curl -fsS {normalized_base_url}/membership",
        f"curl -fsS {normalized_base_url}/work",
    ],
}
output_path.write_text(json.dumps(summary, indent=2) + "\n")
print(json.dumps(summary, indent=2))
PY
  then
    m039_record_phase "$phase" failed
    m039_fail_phase "$phase" "invalid Fly verification inputs" "$validation_log" "$output_json"
  fi

  CLUSTER_PROOF_FLY_APP="$(python3 - <<'PY'
import json
from pathlib import Path
print(json.loads(Path('.tmp/m039-s04/fly/input-validation.json').read_text())['app'])
PY
)"
  CLUSTER_PROOF_BASE_URL="$(python3 - <<'PY'
import json
from pathlib import Path
print(json.loads(Path('.tmp/m039-s04/fly/input-validation.json').read_text())['base_url'])
PY
)"
  export CLUSTER_PROOF_FLY_APP CLUSTER_PROOF_BASE_URL

  m039_record_phase "$phase" passed
}

assert_status_json() {
  local phase="$1"
  local status_json_path="$2"
  local summary_path="$3"
  local check_log="$ARTIFACT_DIR/${phase}.check.log"

  if ! python3 - "$status_json_path" "$summary_path" "$CLUSTER_PROOF_FLY_APP" "$REQUIRED_RUNNING_MACHINES" >"$check_log" 2>&1 <<'PY'
from pathlib import Path
import json
import sys

status_path = Path(sys.argv[1])
summary_path = Path(sys.argv[2])
expected_app = sys.argv[3]
required_running = int(sys.argv[4])

try:
    data = json.loads(status_path.read_text(errors='replace'))
except json.JSONDecodeError as error:
    raise SystemExit(f"malformed fly status JSON: {error}") from error

machines = {}

def walk(node, path):
    if isinstance(node, dict):
        lowered = {str(key).lower(): value for key, value in node.items()}
        state = lowered.get('state')
        path_text = '/'.join(path).lower()
        is_machineish = isinstance(state, str) and (
            any(segment in path_text for segment in ('machine', 'machines', 'allocation', 'allocations', 'vm', 'vms'))
            or 'private_ip' in lowered
            or 'instance_id' in lowered
        )
        if is_machineish:
            ident = str(
                lowered.get('id')
                or lowered.get('machine_id')
                or lowered.get('instance_id')
                or lowered.get('name')
                or f"machine-{len(machines) + 1}"
            )
            machines[ident] = {
                'id': ident,
                'state': state,
                'region': lowered.get('region'),
                'private_ip': lowered.get('private_ip'),
                'name': lowered.get('name'),
            }
        for key, value in node.items():
            walk(value, path + [str(key)])
    elif isinstance(node, list):
        for index, value in enumerate(node):
            walk(value, path + [str(index)])

walk(data, [])
running_states = {'started', 'running'}
running = [machine for machine in machines.values() if str(machine['state']).lower() in running_states]
if len(running) < required_running:
    raise SystemExit(
        f"expected at least {required_running} running machines, found {len(running)} from {len(machines)} machine-like records"
    )

summary = {
    'app': expected_app,
    'machine_records_found': len(machines),
    'running_machine_count': len(running),
    'running_machines': running,
}
summary_path.write_text(json.dumps(summary, indent=2) + '\n')
print(json.dumps(summary, indent=2))
PY
  then
    m039_fail_phase "$phase" "Fly status drifted from the required running-machine contract" "$check_log" "$status_json_path"
  fi
}

assert_config_json() {
  local phase="$1"
  local config_json_path="$2"
  local summary_path="$3"
  local check_log="$ARTIFACT_DIR/${phase}.check.log"

  if ! python3 - "$config_json_path" "$summary_path" "$CLUSTER_PROOF_FLY_APP" >"$check_log" 2>&1 <<'PY'
from pathlib import Path
import json
import sys

config_path = Path(sys.argv[1])
summary_path = Path(sys.argv[2])
expected_app = sys.argv[3]
expected_seed = f"{expected_app}.internal"

try:
    data = json.loads(config_path.read_text(errors='replace'))
except json.JSONDecodeError as error:
    raise SystemExit(f"malformed fly config JSON: {error}") from error

if not isinstance(data, dict):
    raise SystemExit(f"expected config object, found {type(data).__name__}")
if data.get('app') != expected_app:
    raise SystemExit(f"config app mismatch: expected {expected_app!r}, found {data.get('app')!r}")

build = data.get('build')
if not isinstance(build, dict):
    raise SystemExit('config missing build object')
if build.get('dockerfile') != 'cluster-proof/Dockerfile':
    raise SystemExit(
        f"dockerfile drift: expected 'cluster-proof/Dockerfile', found {build.get('dockerfile')!r}"
    )

env = data.get('env')
if not isinstance(env, dict):
    raise SystemExit('config missing env object')
expected_env = {
    'PORT': '8080',
    'MESH_CLUSTER_PORT': '4370',
    'MESH_DISCOVERY_SEED': expected_seed,
}
for key, expected_value in expected_env.items():
    actual_value = env.get(key)
    if actual_value != expected_value:
        raise SystemExit(f"env drift for {key}: expected {expected_value!r}, found {actual_value!r}")

http_service = data.get('http_service')
if not isinstance(http_service, dict):
    raise SystemExit('config missing http_service object')
if http_service.get('internal_port') != 8080:
    raise SystemExit(
        f"internal_port drift: expected 8080, found {http_service.get('internal_port')!r}"
    )
if http_service.get('force_https') is not True:
    raise SystemExit(
        f"force_https drift: expected true, found {http_service.get('force_https')!r}"
    )
auto_stop = http_service.get('auto_stop_machines')
if auto_stop not in (False, 'off'):
    raise SystemExit(
        f"auto_stop_machines drift: expected false/'off', found {auto_stop!r}"
    )

summary = {
    'app': expected_app,
    'dockerfile': build.get('dockerfile'),
    'env': expected_env,
    'http_service': {
        'internal_port': http_service.get('internal_port'),
        'force_https': http_service.get('force_https'),
        'auto_stop_machines': auto_stop,
        'min_machines_running': http_service.get('min_machines_running'),
    },
}
summary_path.write_text(json.dumps(summary, indent=2) + '\n')
print(json.dumps(summary, indent=2))
PY
  then
    m039_fail_phase "$phase" "Fly config drifted from the one-image operator contract" "$check_log" "$config_json_path"
  fi
}

assert_live_membership_json() {
  local phase="$1"
  local json_path="$2"
  local check_log="$ARTIFACT_DIR/${phase}.check.log"

  if ! python3 - "$json_path" "$CLUSTER_PROOF_FLY_APP" "$REQUIRED_RUNNING_MACHINES" >"$check_log" 2>&1 <<'PY'
from pathlib import Path
import json
import sys

json_path = Path(sys.argv[1])
app = sys.argv[2]
required_running = int(sys.argv[3])
expected_seed = f"{app}.internal"

try:
    data = json.loads(json_path.read_text(errors='replace'))
except json.JSONDecodeError as error:
    raise SystemExit(f"malformed JSON: {error}") from error

if not isinstance(data, dict):
    raise SystemExit(f"expected object body, found {type(data).__name__}")
required = {
    'mode': str,
    'self': str,
    'peers': list,
    'membership': list,
    'http_port': str,
    'cluster_port': str,
    'discovery_provider': str,
    'discovery_seed': str,
}
for key, expected_type in required.items():
    if key not in data:
        raise SystemExit(f"missing key {key!r}")
    if not isinstance(data[key], expected_type):
        raise SystemExit(f"key {key!r} expected {expected_type.__name__}, found {type(data[key]).__name__}")

if data['mode'] != 'cluster':
    raise SystemExit(f"expected cluster mode, found {data['mode']!r}")
if data['http_port'] != '8080':
    raise SystemExit(f"expected http_port '8080', found {data['http_port']!r}")
if data['cluster_port'] != '4370':
    raise SystemExit(f"expected cluster_port '4370', found {data['cluster_port']!r}")
if data['discovery_provider'] != 'dns':
    raise SystemExit(f"expected discovery_provider 'dns', found {data['discovery_provider']!r}")
if data['discovery_seed'] != expected_seed:
    raise SystemExit(f"expected discovery_seed {expected_seed!r}, found {data['discovery_seed']!r}")

self_name = data['self']
peers = data['peers']
membership = data['membership']
if '@' not in self_name:
    raise SystemExit(f"self node is malformed: {self_name!r}")
if len(membership) < required_running:
    raise SystemExit(f"expected at least {required_running} membership entries, found {len(membership)}")
if len(peers) < required_running - 1:
    raise SystemExit(f"expected at least {required_running - 1} peers, found {len(peers)}")
if self_name not in membership:
    raise SystemExit(f"self node {self_name!r} missing from membership {membership!r}")
if not all(isinstance(value, str) and '@' in value for value in membership):
    raise SystemExit(f"membership must contain only node identity strings, found {membership!r}")
if not all(isinstance(value, str) and '@' in value for value in peers):
    raise SystemExit(f"peers must contain only node identity strings, found {peers!r}")
if sorted([value for value in membership if value != self_name]) != sorted(peers):
    raise SystemExit('peers must equal membership minus self')

print(json.dumps({
    'self': self_name,
    'membership': membership,
    'peers': peers,
}, indent=2))
PY
  then
    m039_fail_phase "$phase" "live /membership drifted from the cluster contract" "$check_log" "$json_path"
  fi
}

assert_live_work_json() {
  local phase="$1"
  local json_path="$2"
  local check_log="$ARTIFACT_DIR/${phase}.check.log"
  local summary_path="$ARTIFACT_DIR/${phase}.summary.json"

  if ! python3 - "$json_path" "$summary_path" >"$check_log" 2>&1 <<'PY'
from pathlib import Path
import json
import sys

json_path = Path(sys.argv[1])
summary_path = Path(sys.argv[2])

try:
    data = json.loads(json_path.read_text(errors='replace'))
except json.JSONDecodeError as error:
    raise SystemExit(f"malformed JSON: {error}") from error

if not isinstance(data, dict):
    raise SystemExit(f"expected object body, found {type(data).__name__}")
required = {
    'ok': bool,
    'request_id': str,
    'ingress_node': str,
    'target_node': str,
    'execution_node': str,
    'routed_remotely': bool,
    'fell_back_locally': bool,
    'timed_out': bool,
    'error': str,
}
for key, expected_type in required.items():
    if key not in data:
        raise SystemExit(f"missing key {key!r}")
    if not isinstance(data[key], expected_type):
        raise SystemExit(f"key {key!r} expected {expected_type.__name__}, found {type(data[key]).__name__}")

if not data['ok']:
    raise SystemExit(f"expected ok=true, found body {data!r}")
if data['timed_out']:
    raise SystemExit(f"expected timed_out=false, found body {data!r}")
if data['error'] != '':
    raise SystemExit(f"expected empty error string, found {data['error']!r}")
if not data['request_id'].startswith('work-'):
    raise SystemExit(f"request_id must start with 'work-', found {data['request_id']!r}")
for key in ('ingress_node', 'target_node', 'execution_node'):
    value = data[key]
    if '@' not in value:
        raise SystemExit(f"{key} is malformed: {value!r}")
if data['ingress_node'] == data['target_node']:
    raise SystemExit(f"expected remote target distinct from ingress, found {data['ingress_node']!r}")
if data['execution_node'] != data['target_node']:
    raise SystemExit(
        f"execution node must equal target node, found execution={data['execution_node']!r} target={data['target_node']!r}"
    )
if not data['routed_remotely']:
    raise SystemExit(f"expected routed_remotely=true, found body {data!r}")
if data['fell_back_locally']:
    raise SystemExit(f"expected fell_back_locally=false, found body {data!r}")

summary = {
    'request_id': data['request_id'],
    'ingress_node': data['ingress_node'],
    'target_node': data['target_node'],
    'execution_node': data['execution_node'],
}
summary_path.write_text(json.dumps(summary, indent=2) + '\n')
print(json.dumps(summary, indent=2))
PY
  then
    m039_fail_phase "$phase" "live /work drifted from the remote-routing contract" "$check_log" "$json_path"
  fi
}

wait_for_probe() {
  local phase="$1"
  local url="$2"
  local json_path="$3"
  local curl_log="$ARTIFACT_DIR/${phase}.curl.log"
  local timeout_secs="$4"
  local kind="$5"
  local deadline=$((SECONDS + timeout_secs))

  while (( SECONDS < deadline )); do
    if curl --silent --show-error --fail --max-time 5 "$url" >"$json_path" 2>"$curl_log"; then
      if [[ "$kind" == "membership" ]]; then
        if python3 -c 'import json,sys; json.load(open(sys.argv[1]))' "$json_path" >/dev/null 2>&1; then
          return 0
        fi
      else
        if python3 -c 'import json,sys; json.load(open(sys.argv[1]))' "$json_path" >/dev/null 2>&1; then
          return 0
        fi
      fi
    fi
    sleep 2
  done

  return 1
}

wait_for_log_proof() {
  local phase="$1"
  local logs_path="$2"
  local stderr_path="$3"
  local timeout_secs="$4"
  local request_id="$5"
  local target_node="$6"
  local execution_node="$7"
  local check_log="$ARTIFACT_DIR/${phase}.check.log"
  local deadline=$((SECONDS + timeout_secs))

  while (( SECONDS < deadline )); do
    if run_capture 30 "$logs_path" "$stderr_path" fly logs -a "$CLUSTER_PROOF_FLY_APP" --no-tail; then
      if python3 - "$logs_path" "$request_id" "$target_node" "$execution_node" >"$check_log" 2>&1 <<'PY'
from pathlib import Path
import re
import sys

logs_path = Path(sys.argv[1])
request_id = sys.argv[2]
expected_target = sys.argv[3]
expected_execution = sys.argv[4]
text = logs_path.read_text(errors='replace')

request_pattern = re.compile(
    rf"\[cluster-proof\] work dispatched request_id={re.escape(request_id)} .*target={re.escape(expected_target)} .*routed_remotely=true"
)
execution_pattern = re.compile(
    rf"\[cluster-proof\] work executed execution={re.escape(expected_execution)}"
)
if not request_pattern.search(text):
    raise SystemExit(
        f"missing dispatched log for request_id={request_id!r} target={expected_target!r}"
    )
if not execution_pattern.search(text):
    raise SystemExit(
        f"missing execution log for execution={expected_execution!r}"
    )
print('log proof ok')
PY
      then
        return 0
      fi
    fi
    sleep 2
  done

  return 1
}

m039_record_phase preflight started
printf 'preflight\n' >"$CURRENT_PHASE_PATH"
require_command preflight fly
require_command preflight curl
require_command preflight python3
m039_record_phase preflight passed

validate_inputs

run_phase_capture fly-status 01-fly-status 60 fly status -a "$CLUSTER_PROOF_FLY_APP" --json
mv "$ARTIFACT_DIR/01-fly-status.stdout" "$ARTIFACT_DIR/01-fly-status.json"
assert_status_json fly-status "$ARTIFACT_DIR/01-fly-status.json" "$ARTIFACT_DIR/01-fly-status.summary.json"

run_phase_capture fly-config 02-fly-config 60 fly config show -a "$CLUSTER_PROOF_FLY_APP"
mv "$ARTIFACT_DIR/02-fly-config.stdout" "$ARTIFACT_DIR/02-fly-config.json"
assert_config_json fly-config "$ARTIFACT_DIR/02-fly-config.json" "$ARTIFACT_DIR/02-fly-config.summary.json"

m039_record_phase membership-probe started
printf 'membership-probe\n' >"$CURRENT_PHASE_PATH"
MEMBERSHIP_JSON="$ARTIFACT_DIR/03-membership.json"
if ! wait_for_probe membership-probe "$CLUSTER_PROOF_BASE_URL/membership" "$MEMBERSHIP_JSON" 45 membership; then
  m039_record_phase membership-probe failed
  m039_fail_phase membership-probe "timed out waiting for a valid /membership response" "$ARTIFACT_DIR/membership-probe.curl.log" "$MEMBERSHIP_JSON"
fi
assert_live_membership_json membership-probe "$MEMBERSHIP_JSON"
m039_record_phase membership-probe passed

m039_record_phase work-probe started
printf 'work-probe\n' >"$CURRENT_PHASE_PATH"
WORK_JSON="$ARTIFACT_DIR/04-work.json"
if ! wait_for_probe work-probe "$CLUSTER_PROOF_BASE_URL/work" "$WORK_JSON" 45 work; then
  m039_record_phase work-probe failed
  m039_fail_phase work-probe "timed out waiting for a valid /work response" "$ARTIFACT_DIR/work-probe.curl.log" "$WORK_JSON"
fi
assert_live_work_json work-probe "$WORK_JSON"
m039_record_phase work-probe passed

REQUEST_ID="$(python3 - <<'PY'
import json
from pathlib import Path
summary = json.loads(Path('.tmp/m039-s04/fly/work-probe.summary.json').read_text())
print(summary['request_id'])
PY
)"
TARGET_NODE="$(python3 - <<'PY'
import json
from pathlib import Path
summary = json.loads(Path('.tmp/m039-s04/fly/work-probe.summary.json').read_text())
print(summary['target_node'])
PY
)"
EXECUTION_NODE="$(python3 - <<'PY'
import json
from pathlib import Path
summary = json.loads(Path('.tmp/m039-s04/fly/work-probe.summary.json').read_text())
print(summary['execution_node'])
PY
)"

m039_record_phase fly-logs started
printf 'fly-logs\n' >"$CURRENT_PHASE_PATH"
LOGS_PATH="$ARTIFACT_DIR/05-fly-logs.log"
LOGS_STDERR_PATH="$ARTIFACT_DIR/05-fly-logs.stderr.log"
if ! wait_for_log_proof fly-logs "$LOGS_PATH" "$LOGS_STDERR_PATH" 45 "$REQUEST_ID" "$TARGET_NODE" "$EXECUTION_NODE"; then
  m039_record_phase fly-logs failed
  m039_fail_phase fly-logs "recent Fly logs did not prove routed dispatch plus execution on the target node" "$ARTIFACT_DIR/fly-logs.check.log" "$LOGS_PATH"
fi
m039_record_phase fly-logs passed

echo "verify-m039-s04-fly: ok"
