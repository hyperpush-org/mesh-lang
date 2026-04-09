#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# shellcheck source=scripts/lib/m039_cluster_proof.sh
source "$ROOT_DIR/scripts/lib/m039_cluster_proof.sh"
# shellcheck source=scripts/lib/m043_cluster_proof.sh
source "$ROOT_DIR/scripts/lib/m043_cluster_proof.sh"

ARTIFACT_DIR=".tmp/m043-s04/fly"
PHASE_REPORT_PATH="$ARTIFACT_DIR/phase-report.txt"
STATUS_PATH="$ARTIFACT_DIR/status.txt"
CURRENT_PHASE_PATH="$ARTIFACT_DIR/current-phase.txt"
FULL_LOG_PATH="$ARTIFACT_DIR/full-contract.log"
REQUIRED_RUNNING_MACHINES=2

usage() {
  cat <<'EOF'
Usage:
  bash scripts/verify-m043-s04-fly.sh --help
  CLUSTER_PROOF_FLY_APP=<fly-app> \
  CLUSTER_PROOF_BASE_URL=https://<fly-app>.fly.dev \
  [CLUSTER_PROOF_REQUEST_KEY=<existing-request-key>] \
    bash scripts/verify-m043-s04-fly.sh

Read-only Fly verifier for the retained `cluster-proof` reference rail.
This help surface documents a bounded reference/proof lane; it does not define a public starter surface.

Required live-mode environment:
  CLUSTER_PROOF_FLY_APP    Existing Fly app name to inspect.
  CLUSTER_PROOF_BASE_URL   Base URL for the deployed app. Must match
                           https://<CLUSTER_PROOF_FLY_APP>.fly.dev (port allowed).

Optional live-mode environment:
  CLUSTER_PROOF_REQUEST_KEY  Existing keyed continuity request to inspect with
                             GET /work/:request_key. If omitted, the verifier stays
                             read-only at config/membership/log scope only.

What it does in live mode:
  - fly status --json
  - fly config show
  - fly logs --no-tail
  - GET /membership
  - optional GET /work/:request_key for an existing request key

What it does not do:
  - no deploys
  - no machine restarts, scale changes, or secret writes
  - no POST /work
  - no POST /promote
  - no destructive failover proof

Artifacts:
  .tmp/m043-s04/fly/

This script is a retained reference sanity/config/log/probe rail. The destructive
same-image local authority remains `bash scripts/verify-m043-s03.sh`; this lane
only inspects an already-deployed Fly app, verifies that live payloads expose
runtime-owned `cluster_role`, `promotion_epoch`, and `replication_health` truth,
and does not promote Fly or `cluster-proof` into a public starter surface.
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
from urllib.parse import quote, urlparse
import json
import os
import sys

output_path = Path(sys.argv[1])
app = os.environ.get('CLUSTER_PROOF_FLY_APP', '').strip()
base_url_raw = os.environ.get('CLUSTER_PROOF_BASE_URL', '').strip()
request_key_raw = os.environ.get('CLUSTER_PROOF_REQUEST_KEY', '')

if not app:
    raise SystemExit('CLUSTER_PROOF_FLY_APP is required')
if not base_url_raw:
    raise SystemExit('CLUSTER_PROOF_BASE_URL is required')

parsed = urlparse(base_url_raw)
if parsed.scheme not in {'http', 'https'}:
    raise SystemExit(f'CLUSTER_PROOF_BASE_URL must use http or https, found {parsed.scheme!r}')
if parsed.username or parsed.password:
    raise SystemExit('CLUSTER_PROOF_BASE_URL must not contain userinfo')
if not parsed.hostname:
    raise SystemExit('CLUSTER_PROOF_BASE_URL must include a hostname')
if parsed.query or parsed.fragment:
    raise SystemExit('CLUSTER_PROOF_BASE_URL must not include query or fragment components')
if parsed.path not in {'', '/'}:
    raise SystemExit('CLUSTER_PROOF_BASE_URL must be a base URL without a path suffix')

expected_host = f'{app}.fly.dev'
if parsed.hostname != expected_host:
    raise SystemExit(
        'CLUSTER_PROOF_BASE_URL host mismatch: '
        f'expected {expected_host!r} for app {app!r}, found {parsed.hostname!r}'
    )

request_key = request_key_raw
if request_key:
    if request_key != request_key.strip():
        raise SystemExit('CLUSTER_PROOF_REQUEST_KEY must not include leading or trailing whitespace')
    if len(request_key) > 128:
        raise SystemExit('CLUSTER_PROOF_REQUEST_KEY must be 1..128 characters when provided')
    if any(ord(ch) < 32 for ch in request_key):
        raise SystemExit('CLUSTER_PROOF_REQUEST_KEY must not include control characters')

normalized_base_url = f'{parsed.scheme}://{parsed.netloc}'.rstrip('/')
encoded_request_key = quote(request_key, safe='') if request_key else ''
summary = {
    'app': app,
    'base_url': normalized_base_url,
    'expected_host': expected_host,
    'membership_url': f'{normalized_base_url}/membership',
    'status_url': f'{normalized_base_url}/work/{encoded_request_key}' if request_key else '',
    'request_key': request_key,
    'required_running_machines': 2,
    'read_only_commands': [
        f'fly status -a {app} --json',
        f'fly config show -a {app}',
        f'fly logs -a {app} --no-tail',
        f'curl -fsS {normalized_base_url}/membership',
    ] + ([f'curl -fsS {normalized_base_url}/work/{encoded_request_key}'] if request_key else []),
}
output_path.write_text(json.dumps(summary, indent=2) + '\n')
print(json.dumps(summary, indent=2))
PY
  then
    m039_record_phase "$phase" failed
    m039_fail_phase "$phase" "invalid Fly verification inputs" "$validation_log" "$output_json"
  fi

  CLUSTER_PROOF_FLY_APP="$(python3 - <<'PY'
import json
from pathlib import Path
print(json.loads(Path('.tmp/m043-s04/fly/input-validation.json').read_text())['app'])
PY
)"
  CLUSTER_PROOF_BASE_URL="$(python3 - <<'PY'
import json
from pathlib import Path
print(json.loads(Path('.tmp/m043-s04/fly/input-validation.json').read_text())['base_url'])
PY
)"
  CLUSTER_PROOF_REQUEST_KEY="$(python3 - <<'PY'
import json
from pathlib import Path
print(json.loads(Path('.tmp/m043-s04/fly/input-validation.json').read_text())['request_key'])
PY
)"
  CLUSTER_PROOF_STATUS_URL="$(python3 - <<'PY'
import json
from pathlib import Path
print(json.loads(Path('.tmp/m043-s04/fly/input-validation.json').read_text())['status_url'])
PY
)"
  export CLUSTER_PROOF_FLY_APP CLUSTER_PROOF_BASE_URL CLUSTER_PROOF_REQUEST_KEY CLUSTER_PROOF_STATUS_URL

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
    raise SystemExit(f'malformed fly status JSON: {error}') from error

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
                or f'machine-{len(machines) + 1}'
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
        f'expected at least {required_running} running machines, found {len(running)} from {len(machines)} machine-like records'
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
    m039_fail_phase "$phase" "Fly status drifted from the supported two-node operator topology" "$check_log" "$status_json_path"
  fi
}

assert_config_toml() {
  local phase="$1"
  local config_path="$2"
  local summary_path="$3"
  local check_log="$ARTIFACT_DIR/${phase}.check.log"

  if ! python3 - "$config_path" "$summary_path" "$CLUSTER_PROOF_FLY_APP" >"$check_log" 2>&1 <<'PY'
from pathlib import Path
import json
import sys
import tomllib

config_path = Path(sys.argv[1])
summary_path = Path(sys.argv[2])
expected_app = sys.argv[3]
expected_seed = f'{expected_app}.internal'

try:
    data = tomllib.loads(config_path.read_text(errors='replace'))
except tomllib.TOMLDecodeError as error:
    raise SystemExit(f'malformed fly config TOML: {error}') from error

if not isinstance(data, dict):
    raise SystemExit(f'expected config object, found {type(data).__name__}')
if data.get('app') != expected_app:
    raise SystemExit(f'config app mismatch: expected {expected_app!r}, found {data.get("app")!r}')

build = data.get('build')
if not isinstance(build, dict):
    raise SystemExit('config missing build table')
if build.get('dockerfile') != 'cluster-proof/Dockerfile':
    raise SystemExit(
        f"dockerfile drift: expected 'cluster-proof/Dockerfile', found {build.get('dockerfile')!r}"
    )

env = data.get('env')
if not isinstance(env, dict):
    raise SystemExit('config missing env table')
expected_env = {
    'PORT': '8080',
    'MESH_CLUSTER_PORT': '4370',
    'MESH_DISCOVERY_SEED': expected_seed,
    'CLUSTER_PROOF_DURABILITY': 'replica-backed',
}
for key, expected_value in expected_env.items():
    actual_value = env.get(key)
    if actual_value != expected_value:
        raise SystemExit(f'env drift for {key}: expected {expected_value!r}, found {actual_value!r}')

http_service = data.get('http_service')
if not isinstance(http_service, dict):
    raise SystemExit('config missing http_service table')
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
    m039_fail_phase "$phase" "Fly config drifted from the packaged read-only operator contract" "$check_log" "$config_path"
  fi
}

assert_live_membership_json() {
  local phase="$1"
  local json_path="$2"
  local summary_path="$3"
  local check_log="$ARTIFACT_DIR/${phase}.check.log"

  if ! python3 - "$json_path" "$summary_path" "$CLUSTER_PROOF_FLY_APP" "$REQUIRED_RUNNING_MACHINES" >"$check_log" 2>&1 <<'PY'
from pathlib import Path
import json
import sys

json_path = Path(sys.argv[1])
summary_path = Path(sys.argv[2])
app = sys.argv[3]
required_running = int(sys.argv[4])
expected_seed = f'{app}.internal'
allowed_roles = {'primary', 'standby'}
allowed_healths = {'healthy', 'degraded', 'local_only', 'unavailable'}

try:
    data = json.loads(json_path.read_text(errors='replace'))
except json.JSONDecodeError as error:
    raise SystemExit(f'malformed JSON: {error}') from error

if not isinstance(data, dict):
    raise SystemExit(f'expected object body, found {type(data).__name__}')
required = {
    'mode': str,
    'self': str,
    'peers': list,
    'membership': list,
    'http_port': str,
    'cluster_port': str,
    'discovery_provider': str,
    'discovery_seed': str,
    'cluster_role': str,
    'promotion_epoch': int,
    'replication_health': str,
}
for key, expected_type in required.items():
    if key not in data:
        raise SystemExit(f'missing key {key!r}')
    if not isinstance(data[key], expected_type):
        raise SystemExit(f'key {key!r} expected {expected_type.__name__}, found {type(data[key]).__name__}')

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
if data['cluster_role'] not in allowed_roles:
    raise SystemExit(f"expected cluster_role in {sorted(allowed_roles)!r}, found {data['cluster_role']!r}")
if data['promotion_epoch'] < 0:
    raise SystemExit(f"promotion_epoch must be non-negative, found {data['promotion_epoch']!r}")
if data['replication_health'] not in allowed_healths:
    raise SystemExit(
        f"expected replication_health in {sorted(allowed_healths)!r}, found {data['replication_health']!r}"
    )

self_name = data['self']
peers = data['peers']
membership = data['membership']
if '@' not in self_name:
    raise SystemExit(f'self node is malformed: {self_name!r}')
if len(membership) < required_running:
    raise SystemExit(f'expected at least {required_running} membership entries, found {len(membership)}')
if len(peers) < required_running - 1:
    raise SystemExit(f'expected at least {required_running - 1} peers, found {len(peers)}')
if self_name not in membership:
    raise SystemExit(f'self node {self_name!r} missing from membership {membership!r}')
if not all(isinstance(value, str) and '@' in value for value in membership):
    raise SystemExit(f'membership must contain only node identity strings, found {membership!r}')
if not all(isinstance(value, str) and '@' in value for value in peers):
    raise SystemExit(f'peers must contain only node identity strings, found {peers!r}')
if sorted([value for value in membership if value != self_name]) != sorted(peers):
    raise SystemExit('peers must equal membership minus self')

summary = {
    'self': self_name,
    'membership': membership,
    'peers': peers,
    'cluster_role': data['cluster_role'],
    'promotion_epoch': data['promotion_epoch'],
    'replication_health': data['replication_health'],
}
summary_path.write_text(json.dumps(summary, indent=2) + '\n')
print(json.dumps(summary, indent=2))
PY
  then
    m039_fail_phase "$phase" "live /membership drifted from the M043 authority contract" "$check_log" "$json_path"
  fi
}

assert_live_status_json() {
  local phase="$1"
  local json_path="$2"
  local request_key="$3"
  local summary_path="$4"
  local check_log="$ARTIFACT_DIR/${phase}.check.log"

  if ! python3 - "$json_path" "$request_key" "$summary_path" >"$check_log" 2>&1 <<'PY'
from pathlib import Path
import json
import re
import sys

json_path = Path(sys.argv[1])
request_key = sys.argv[2]
summary_path = Path(sys.argv[3])
allowed_roles = {'primary', 'standby'}
allowed_healths = {'healthy', 'degraded', 'local_only', 'unavailable'}
allowed_phases = {'submitted', 'completed', 'rejected', 'missing', 'invalid'}
allowed_results = {'pending', 'succeeded', 'rejected', 'missing', 'unknown'}
allowed_replica_status = {'mirrored', 'owner_lost', 'unassigned'}

try:
    data = json.loads(json_path.read_text(errors='replace'))
except json.JSONDecodeError as error:
    raise SystemExit(f'malformed JSON: {error}') from error

if not isinstance(data, dict):
    raise SystemExit(f'expected object body, found {type(data).__name__}')
required = {
    'ok': bool,
    'request_key': str,
    'attempt_id': str,
    'phase': str,
    'result': str,
    'ingress_node': str,
    'owner_node': str,
    'replica_node': str,
    'replica_status': str,
    'cluster_role': str,
    'promotion_epoch': int,
    'replication_health': str,
    'execution_node': str,
    'routed_remotely': bool,
    'fell_back_locally': bool,
    'error': str,
    'conflict_reason': str,
}
for key, expected_type in required.items():
    if key not in data:
        raise SystemExit(f'missing key {key!r}')
    if not isinstance(data[key], expected_type):
        raise SystemExit(f'key {key!r} expected {expected_type.__name__}, found {type(data[key]).__name__}')

if data['request_key'] != request_key:
    raise SystemExit(f"request_key mismatch: expected {request_key!r}, found {data['request_key']!r}")
if not re.fullmatch(r'attempt-\d+', data['attempt_id']):
    raise SystemExit(f"attempt_id must match attempt-<int>, found {data['attempt_id']!r}")
if data['phase'] not in allowed_phases:
    raise SystemExit(f"unexpected phase {data['phase']!r}")
if data['result'] not in allowed_results:
    raise SystemExit(f"unexpected result {data['result']!r}")
if data['replica_status'] not in allowed_replica_status:
    raise SystemExit(f"unexpected replica_status {data['replica_status']!r}")
if data['cluster_role'] not in allowed_roles:
    raise SystemExit(f"unexpected cluster_role {data['cluster_role']!r}")
if data['promotion_epoch'] < 0:
    raise SystemExit(f"promotion_epoch must be non-negative, found {data['promotion_epoch']!r}")
if data['replication_health'] not in allowed_healths:
    raise SystemExit(f"unexpected replication_health {data['replication_health']!r}")
for key in ('ingress_node', 'owner_node', 'replica_node', 'execution_node'):
    value = data[key]
    if value and '@' not in value:
        raise SystemExit(f'{key} must be empty or a node identity, found {value!r}')

summary = {
    'request_key': data['request_key'],
    'attempt_id': data['attempt_id'],
    'phase': data['phase'],
    'result': data['result'],
    'cluster_role': data['cluster_role'],
    'promotion_epoch': data['promotion_epoch'],
    'replication_health': data['replication_health'],
}
summary_path.write_text(json.dumps(summary, indent=2) + '\n')
print(json.dumps(summary, indent=2))
PY
  then
    m039_fail_phase "$phase" "live /work/:request_key drifted from the M043 authority/status contract" "$check_log" "$json_path"
  fi
}

m039_record_phase preflight started
printf 'preflight\n' >"$CURRENT_PHASE_PATH"
require_command preflight fly
require_command preflight python3
m039_record_phase preflight passed

validate_inputs

run_phase_capture fly-status 01-fly-status 60 fly status -a "$CLUSTER_PROOF_FLY_APP" --json
mv "$ARTIFACT_DIR/01-fly-status.stdout" "$ARTIFACT_DIR/01-fly-status.json"
assert_status_json fly-status "$ARTIFACT_DIR/01-fly-status.json" "$ARTIFACT_DIR/01-fly-status.summary.json"

run_phase_capture fly-config 02-fly-config 60 fly config show -a "$CLUSTER_PROOF_FLY_APP"
mv "$ARTIFACT_DIR/02-fly-config.stdout" "$ARTIFACT_DIR/02-fly-config.toml"
assert_config_toml fly-config "$ARTIFACT_DIR/02-fly-config.toml" "$ARTIFACT_DIR/02-fly-config.summary.json"

run_phase_capture fly-logs 03-fly-logs 60 fly logs -a "$CLUSTER_PROOF_FLY_APP" --no-tail
mv "$ARTIFACT_DIR/03-fly-logs.stdout" "$ARTIFACT_DIR/03-fly-logs.log"

m039_record_phase membership-probe started
printf 'membership-probe\n' >"$CURRENT_PHASE_PATH"
if ! m043_http_json_request membership-probe GET "$CLUSTER_PROOF_BASE_URL/membership" - 200 "$ARTIFACT_DIR/04-membership.http" "$ARTIFACT_DIR/04-membership.json" "live /membership"; then
  m039_record_phase membership-probe failed
  m039_fail_phase membership-probe "failed to fetch read-only /membership" "$ARTIFACT_DIR/membership-probe.http-check.log" "$ARTIFACT_DIR/04-membership.http"
fi
assert_live_membership_json membership-probe "$ARTIFACT_DIR/04-membership.json" "$ARTIFACT_DIR/04-membership.summary.json"
m039_record_phase membership-probe passed

if [[ -n "$CLUSTER_PROOF_REQUEST_KEY" ]]; then
  m039_record_phase keyed-status-probe started
  printf 'keyed-status-probe\n' >"$CURRENT_PHASE_PATH"
  if ! m043_http_json_request keyed-status-probe GET "$CLUSTER_PROOF_STATUS_URL" - 200 "$ARTIFACT_DIR/05-keyed-status.http" "$ARTIFACT_DIR/05-keyed-status.json" "live /work/:request_key"; then
    m039_record_phase keyed-status-probe failed
    m039_fail_phase keyed-status-probe "failed to fetch read-only /work/:request_key" "$ARTIFACT_DIR/keyed-status-probe.http-check.log" "$ARTIFACT_DIR/05-keyed-status.http"
  fi
  assert_live_status_json keyed-status-probe "$ARTIFACT_DIR/05-keyed-status.json" "$CLUSTER_PROOF_REQUEST_KEY" "$ARTIFACT_DIR/05-keyed-status.summary.json"
  m039_record_phase keyed-status-probe passed
fi

echo "verify-m043-s04-fly: ok"
