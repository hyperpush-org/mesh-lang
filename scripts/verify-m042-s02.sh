#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

source scripts/lib/clustered_fixture_paths.sh
clustered_fixture_require_cluster_proof_root

ARTIFACT_ROOT=".tmp/m042-s02"
ARTIFACT_DIR="$ARTIFACT_ROOT/verify"
PHASE_REPORT_PATH="$ARTIFACT_DIR/phase-report.txt"
STATUS_PATH="$ARTIFACT_DIR/status.txt"
CURRENT_PHASE_PATH="$ARTIFACT_DIR/current-phase.txt"
mkdir -p "$ARTIFACT_DIR"
exec > >(tee "$ARTIFACT_DIR/full-contract.log") 2>&1

: >"$PHASE_REPORT_PATH"
printf 'running\n' >"$STATUS_PATH"
printf 'init\n' >"$CURRENT_PHASE_PATH"

on_exit() {
  local exit_code=$?
  if [[ $exit_code -eq 0 ]]; then
    printf 'ok\n' >"$STATUS_PATH"
    printf 'complete\n' >"$CURRENT_PHASE_PATH"
  elif [[ ! -f "$STATUS_PATH" || "$(<"$STATUS_PATH")" != "failed" ]]; then
    printf 'failed\n' >"$STATUS_PATH"
  fi
}
trap on_exit EXIT

record_phase() {
  printf '%s\t%s\n' "$1" "$2" >>"$PHASE_REPORT_PATH"
}

print_log_excerpt() {
  local log_path="$1"
  python3 - "$log_path" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
if not path.exists():
    print(f"missing log: {path}")
    raise SystemExit(0)

lines = path.read_text(errors="replace").splitlines()
limit = 220
for line in lines[:limit]:
    print(line)
if len(lines) > limit:
    print(f"... truncated after {limit} lines (total {len(lines)})")
PY
}

fail_phase() {
  local phase="$1"
  local reason="$2"
  local log_path="${3:-}"
  local artifact_hint="${4:-}"

  printf 'failed\n' >"$STATUS_PATH"
  printf '%s\n' "$phase" >"$CURRENT_PHASE_PATH"
  echo "verification drift: ${reason}" >&2
  if [[ -n "$artifact_hint" ]]; then
    echo "artifact hint: ${artifact_hint}" >&2
  fi
  if [[ -n "$log_path" ]]; then
    echo "failing log: ${log_path}" >&2
    echo "--- ${log_path} ---" >&2
    print_log_excerpt "$log_path" >&2
  fi
  exit 1
}

assert_file_exists() {
  local phase="$1"
  local path="$2"
  local description="$3"
  local log_path="${4:-}"
  if [[ ! -f "$path" ]]; then
    fail_phase "$phase" "missing ${description}: ${path}" "$log_path" "$path"
  fi
}

assert_file_contains_regex() {
  local phase="$1"
  local path="$2"
  local regex="$3"
  local description="$4"
  local log_path="${5:-}"
  if ! python3 - "$path" "$regex" "$description" >"$ARTIFACT_DIR/${phase}.content-check.log" 2>&1 <<'PY'
from pathlib import Path
import re
import sys

path = Path(sys.argv[1])
regex = sys.argv[2]
description = sys.argv[3]
text = path.read_text(errors="replace")
if not re.search(regex, text, re.MULTILINE):
    raise SystemExit(f"{description}: missing regex {regex!r} in {path}")
print(f"{description}: matched {regex!r}")
PY
  then
    fail_phase "$phase" "$description" "$ARTIFACT_DIR/${phase}.content-check.log" "$path"
  fi
}

assert_test_filter_ran() {
  local phase="$1"
  local log_path="$2"
  local label="$3"
  if ! python3 - "$log_path" "$label" >"$ARTIFACT_DIR/${label}.test-count.log" 2>&1 <<'PY'
import re
import sys
from pathlib import Path

log_path = Path(sys.argv[1])
label = sys.argv[2]
text = log_path.read_text(errors="replace")
counts = [int(value) for value in re.findall(r"running (\d+) test", text)]
if not counts:
    raise SystemExit(f"{label}: missing 'running N test' line")
if max(counts) <= 0:
    raise SystemExit(f"{label}: test filter ran 0 tests")
print(f"{label}: running-counts={counts}")
PY
  then
    fail_phase "$phase" "named test filter ran 0 tests or produced malformed output" "$ARTIFACT_DIR/${label}.test-count.log"
  fi
}

run_command_with_timeout() {
  local timeout_secs="$1"
  local log_path="$2"
  shift 2
  local -a cmd=("$@")

  {
    printf '$'
    printf ' %q' "${cmd[@]}"
    printf '\n'
    "${cmd[@]}"
  } >"$log_path" 2>&1 &
  local cmd_pid=$!
  local deadline=$((SECONDS + timeout_secs))

  while kill -0 "$cmd_pid" 2>/dev/null; do
    if (( SECONDS >= deadline )); then
      echo "command timed out after ${timeout_secs}s" >>"$log_path"
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

run_expect_success() {
  local phase="$1"
  local label="$2"
  local require_tests="$3"
  local timeout_secs="$4"
  shift 4
  local -a cmd=("$@")
  local log_path="$ARTIFACT_DIR/${label}.log"
  local command_text="${cmd[*]}"

  record_phase "$phase" started
  printf '%s\n' "$phase" >"$CURRENT_PHASE_PATH"
  echo "==> ${command_text}"
  if ! run_command_with_timeout "$timeout_secs" "$log_path" "${cmd[@]}"; then
    record_phase "$phase" failed
    fail_phase "$phase" "expected success within ${timeout_secs}s" "$log_path"
  fi
  if [[ "$require_tests" == "yes" ]]; then
    assert_test_filter_ran "$phase" "$log_path" "$label"
  fi
  record_phase "$phase" passed
}

capture_s02_snapshot() {
  local snapshot_path="$1"
  python3 - "$snapshot_path" <<'PY'
from pathlib import Path
import sys

snapshot_path = Path(sys.argv[1])
root = Path('.tmp/m042-s02')
names = []
if root.exists():
    names = sorted(
        path.name
        for path in root.iterdir()
        if path.is_dir() and path.name != 'verify'
    )
snapshot_path.write_text(''.join(f"{name}\n" for name in names))
PY
}

collect_phase_artifacts() {
  local phase="$1"
  local mode="$2"
  local before_snapshot="$3"
  local dest_root="$4"
  local manifest_path="$5"

  if ! python3 - "$before_snapshot" "$dest_root" "$mode" >"$manifest_path" 2>"$ARTIFACT_DIR/${phase}.artifact-check.log" <<'PY'
from pathlib import Path
import json
import shutil
import sys

before_snapshot = Path(sys.argv[1])
dest_root = Path(sys.argv[2])
mode = sys.argv[3]
source_root = Path('.tmp/m042-s02')

before = {
    line.strip()
    for line in before_snapshot.read_text(errors='replace').splitlines()
    if line.strip()
}
after_paths = {
    path.name: path
    for path in source_root.iterdir()
    if path.is_dir() and path.name != 'verify'
}
new_names = sorted(name for name in after_paths if name not in before)

expected = {
    'malformed': {
        'continuity-api-malformed-response-': [
            'malformed-response.http',
            'malformed-response.body.txt',
        ],
    },
    'rejection': {
        'continuity-api-single-node-rejection-': [
            'membership-node-solo.json',
            'invalid-submit.json',
            'rejected-submit.json',
            'rejected-duplicate.json',
            'rejected-status.json',
            'rejected-conflict.json',
            'node-solo.stdout.log',
            'node-solo.stderr.log',
        ],
    },
    'mirrored': {
        'continuity-api-two-node-mirrored-': [
            'membership-node-a.json',
            'membership-node-b.json',
            'pending-owner-status.json',
            'pending-replica-status.json',
            'completed-owner-status.json',
            'completed-replica-status.json',
            'completed-duplicate.json',
            'node-a.stdout.log',
            'node-a.stderr.log',
            'node-b.stdout.log',
            'node-b.stderr.log',
        ],
    },
    'degraded': {
        'continuity-api-two-node-degraded-': [
            'membership-node-a.json',
            'membership-node-b.json',
            'pending-owner-status.json',
            'degraded-owner-status.json',
            'node-a.stdout.log',
            'node-a.stderr.log',
            'node-b.stdout.log',
            'node-b.stderr.log',
        ],
    },
}

membership_required = {'self': str, 'membership': list}
work_required = {
    'ok': bool,
    'request_key': str,
    'attempt_id': str,
    'phase': str,
    'result': str,
    'ingress_node': str,
    'owner_node': str,
    'replica_node': str,
    'replica_status': str,
    'execution_node': str,
    'routed_remotely': bool,
    'fell_back_locally': bool,
    'error': str,
    'conflict_reason': str,
}

if mode not in expected:
    raise SystemExit(f'unknown artifact mode: {mode}')
mode_expected = expected[mode]

if len(new_names) != len(mode_expected):
    raise SystemExit(
        f'{mode}: expected {len(mode_expected)} new artifact directories, found {len(new_names)} -> {new_names}'
    )


def ensure_json_shape(path: Path, required: dict[str, type]) -> None:
    try:
        data = json.loads(path.read_text(errors='replace'))
    except json.JSONDecodeError as error:
        raise SystemExit(f'{path}: malformed JSON: {error}') from error
    if not isinstance(data, dict):
        raise SystemExit(f'{path}: expected JSON object, found {type(data).__name__}')
    for key, expected_type in required.items():
        if key not in data:
            raise SystemExit(f'{path}: missing key {key!r}')
        if not isinstance(data[key], expected_type):
            raise SystemExit(
                f'{path}: key {key!r} expected {expected_type.__name__}, found {type(data[key]).__name__}'
            )

if dest_root.exists():
    shutil.rmtree(dest_root)
dest_root.mkdir(parents=True, exist_ok=True)
manifest_lines = []
for prefix, required_files in mode_expected.items():
    matches = [name for name in new_names if name.startswith(prefix)]
    if len(matches) != 1:
        raise SystemExit(f'{mode}: expected exactly one artifact dir for prefix {prefix!r}, found {matches}')
    name = matches[0]
    src = after_paths[name]
    missing = [file_name for file_name in required_files if not (src / file_name).is_file()]
    if missing:
        raise SystemExit(f'{mode}: artifact dir {src} is missing files {missing}')
    for file_name in required_files:
        path = src / file_name
        if path.suffix == '.json':
            if 'membership' in file_name:
                ensure_json_shape(path, membership_required)
            else:
                ensure_json_shape(path, work_required)
        else:
            if path.stat().st_size <= 0:
                raise SystemExit(f'{path}: expected non-empty artifact file')
    dst = dest_root / name
    shutil.copytree(src, dst)
    manifest_lines.append(f'{name}\t{src}')
    for file_name in required_files:
        manifest_lines.append(f'  - {src / file_name}')

print('\n'.join(manifest_lines))
PY
  then
    fail_phase "$phase" "missing or malformed copied evidence" "$ARTIFACT_DIR/${phase}.artifact-check.log" "$dest_root"
  fi
}

run_expect_success mesh-rt 00-mesh-rt no 180 \
  cargo build -q -p mesh-rt
assert_file_exists mesh-rt target/debug/libmesh_rt.a "mesh-rt static library" "$ARTIFACT_DIR/00-mesh-rt.log"

run_expect_success cluster-proof-tests 01-cluster-proof-tests no 180 \
  cargo run -q -p meshc -- test "$CLUSTER_PROOF_FIXTURE_TESTS"
run_expect_success build-cluster-proof 02-build-cluster-proof no 180 \
  cargo run -q -p meshc -- build "$CLUSTER_PROOF_FIXTURE_ROOT"
run_expect_success s01-standalone 03-s01-standalone yes 180 \
  cargo test -p meshc --test e2e_m042_s01 continuity_api_standalone_keyed_submit_status_and_retry_contract -- --nocapture

MALFORMED_BEFORE="$ARTIFACT_DIR/04-malformed.before.txt"
capture_s02_snapshot "$MALFORMED_BEFORE"
run_expect_success s02-malformed-response 04-s02-malformed-response yes 120 \
  cargo test -p meshc --test e2e_m042_s02 continuity_api_archives_non_json_http_response_as_contract_failure -- --nocapture
collect_phase_artifacts \
  s02-malformed-response \
  malformed \
  "$MALFORMED_BEFORE" \
  "$ARTIFACT_DIR/04-malformed-artifacts" \
  "$ARTIFACT_DIR/04-malformed-artifacts.txt"

REJECTION_BEFORE="$ARTIFACT_DIR/05-rejection.before.txt"
capture_s02_snapshot "$REJECTION_BEFORE"
run_expect_success s02-single-node-rejection 05-s02-single-node-rejection yes 180 \
  cargo test -p meshc --test e2e_m042_s02 continuity_api_single_node_cluster_rejects_replica_required_submit_and_replays_status -- --nocapture
collect_phase_artifacts \
  s02-single-node-rejection \
  rejection \
  "$REJECTION_BEFORE" \
  "$ARTIFACT_DIR/05-rejection-artifacts" \
  "$ARTIFACT_DIR/05-rejection-artifacts.txt"

MIRRORED_BEFORE="$ARTIFACT_DIR/06-mirrored.before.txt"
capture_s02_snapshot "$MIRRORED_BEFORE"
run_expect_success s02-mirrored-admission 06-s02-mirrored-admission yes 240 \
  cargo test -p meshc --test e2e_m042_s02 continuity_api_two_node_local_owner_mirrors_status_between_owner_and_replica -- --nocapture
collect_phase_artifacts \
  s02-mirrored-admission \
  mirrored \
  "$MIRRORED_BEFORE" \
  "$ARTIFACT_DIR/06-mirrored-artifacts" \
  "$ARTIFACT_DIR/06-mirrored-artifacts.txt"

DEGRADED_BEFORE="$ARTIFACT_DIR/07-degraded.before.txt"
capture_s02_snapshot "$DEGRADED_BEFORE"
run_expect_success s02-degraded-status 07-s02-degraded-status yes 240 \
  cargo test -p meshc --test e2e_m042_s02 continuity_api_replica_loss_degrades_pending_mirrored_status -- --nocapture
collect_phase_artifacts \
  s02-degraded-status \
  degraded \
  "$DEGRADED_BEFORE" \
  "$ARTIFACT_DIR/07-degraded-artifacts" \
  "$ARTIFACT_DIR/07-degraded-artifacts.txt"

assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^mesh-rt\tpassed$' "mesh-rt replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^cluster-proof-tests\tpassed$' "cluster-proof tests replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^build-cluster-proof\tpassed$' "cluster-proof build replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^s01-standalone\tpassed$' "S01 standalone replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^s02-malformed-response\tpassed$' "Malformed-response phase did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^s02-single-node-rejection\tpassed$' "Single-node rejection phase did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^s02-mirrored-admission\tpassed$' "Mirrored-admission phase did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^s02-degraded-status\tpassed$' "Degraded-status phase did not pass" "$ARTIFACT_DIR/full-contract.log"

echo "verify-m042-s02: ok"
