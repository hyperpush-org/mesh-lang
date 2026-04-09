#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

source scripts/lib/clustered_fixture_paths.sh
clustered_fixture_require_cluster_proof_root

ARTIFACT_ROOT=".tmp/m039-s03"
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

capture_s03_snapshot() {
  local snapshot_path="$1"
  python3 - "$snapshot_path" <<'PY'
from pathlib import Path
import sys

snapshot_path = Path(sys.argv[1])
root = Path('.tmp/m039-s03')
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
source_root = Path('.tmp/m039-s03')

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

if mode == 'degrade':
    expected = {
        'e2e-m039-s03-degrade-': [
            'pre-loss-node-a-membership.json',
            'pre-loss-node-b-membership.json',
            'pre-loss-work.json',
            'degraded-node-a-membership.json',
            'degraded-work.json',
            'node-a-run1.stdout.log',
            'node-a-run1.stderr.log',
            'node-b-run1.stdout.log',
            'node-b-run1.stderr.log',
        ],
    }
elif mode == 'rejoin':
    expected = {
        'e2e-m039-s03-rejoin-': [
            'pre-loss-node-a-membership.json',
            'pre-loss-node-b-membership.json',
            'pre-loss-work.json',
            'degraded-node-a-membership.json',
            'degraded-work.json',
            'post-rejoin-node-a-membership.json',
            'post-rejoin-node-b-membership.json',
            'post-rejoin-work.json',
            'node-a-run1.stdout.log',
            'node-a-run1.stderr.log',
            'node-b-run1.stdout.log',
            'node-b-run1.stderr.log',
            'node-b-run2.stdout.log',
            'node-b-run2.stderr.log',
        ],
    }
else:
    raise SystemExit(f'unknown artifact mode: {mode}')

if len(new_names) != len(expected):
    raise SystemExit(
        f'{mode}: expected {len(expected)} new artifact directories, found {len(new_names)} -> {new_names}'
    )

membership_required = {'self': str, 'peers': list, 'membership': list}
work_required = {
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
    if 'request_id' in data and not data['request_id'].startswith('work-'):
        raise SystemExit(f'{path}: request_id must start with work-, found {data["request_id"]!r}')


if dest_root.exists():
    shutil.rmtree(dest_root)
dest_root.mkdir(parents=True, exist_ok=True)
manifest_lines = []
for prefix, required_files in expected.items():
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
                raise SystemExit(f'{path}: expected non-empty log file')
    dst = dest_root / name
    shutil.copytree(src, dst)
    manifest_lines.append(f'{name}\t{src}')
    for file_name in required_files:
        manifest_lines.append(f'  - {src / file_name}')

print('\n'.join(manifest_lines))
PY
  then
    fail_phase "$phase" "missing or malformed copied S03 evidence" "$ARTIFACT_DIR/${phase}.artifact-check.log" "$dest_root"
  fi
}

run_expect_success build-tooling 00-build-tooling no 120 \
  cargo build -q -p mesh-rt
assert_file_exists build-tooling target/debug/libmesh_rt.a "mesh-rt static library" "$ARTIFACT_DIR/00-build-tooling.log"

run_expect_success cluster-proof-tests 01-cluster-proof-tests no 120 \
  cargo run -q -p meshc -- test "$CLUSTER_PROOF_FIXTURE_TESTS"
run_expect_success build-cluster-proof 02-build-cluster-proof no 120 \
  cargo run -q -p meshc -- build "$CLUSTER_PROOF_FIXTURE_ROOT"
run_expect_success s01-contract 03-s01-contract no 300 \
  bash scripts/verify-m039-s01.sh
assert_file_exists s01-contract .tmp/m039-s01/verify/phase-report.txt "S01 phase report" "$ARTIFACT_DIR/03-s01-contract.log"
cp .tmp/m039-s01/verify/phase-report.txt "$ARTIFACT_DIR/03-s01-phase-report.txt"
assert_file_contains_regex s01-contract "$ARTIFACT_DIR/03-s01-phase-report.txt" '^convergence	passed$' "S01 convergence phase did not pass" "$ARTIFACT_DIR/03-s01-contract.log"
assert_file_contains_regex s01-contract "$ARTIFACT_DIR/03-s01-phase-report.txt" '^node-loss	passed$' "S01 node-loss phase did not pass" "$ARTIFACT_DIR/03-s01-contract.log"

run_expect_success s02-contract 04-s02-contract no 420 \
  bash scripts/verify-m039-s02.sh
assert_file_exists s02-contract .tmp/m039-s02/verify/phase-report.txt "S02 phase report" "$ARTIFACT_DIR/04-s02-contract.log"
assert_file_exists s02-contract .tmp/m039-s02/verify/status.txt "S02 status" "$ARTIFACT_DIR/04-s02-contract.log"
assert_file_exists s02-contract .tmp/m039-s02/verify/current-phase.txt "S02 current phase" "$ARTIFACT_DIR/04-s02-contract.log"
cp .tmp/m039-s02/verify/phase-report.txt "$ARTIFACT_DIR/04-s02-phase-report.txt"
cp .tmp/m039-s02/verify/status.txt "$ARTIFACT_DIR/04-s02-status.txt"
cp .tmp/m039-s02/verify/current-phase.txt "$ARTIFACT_DIR/04-s02-current-phase.txt"
assert_file_contains_regex s02-contract "$ARTIFACT_DIR/04-s02-phase-report.txt" '^cluster-proof-tests	passed$' "S02 cluster-proof test replay did not pass" "$ARTIFACT_DIR/04-s02-contract.log"
assert_file_contains_regex s02-contract "$ARTIFACT_DIR/04-s02-phase-report.txt" '^build-cluster-proof	passed$' "S02 build replay did not pass" "$ARTIFACT_DIR/04-s02-contract.log"
assert_file_contains_regex s02-contract "$ARTIFACT_DIR/04-s02-phase-report.txt" '^s01-contract	passed$' "S02 prerequisite replay did not pass" "$ARTIFACT_DIR/04-s02-contract.log"
assert_file_contains_regex s02-contract "$ARTIFACT_DIR/04-s02-phase-report.txt" '^s02-remote-route	passed$' "S02 remote-route phase did not pass" "$ARTIFACT_DIR/04-s02-contract.log"
assert_file_contains_regex s02-contract "$ARTIFACT_DIR/04-s02-phase-report.txt" '^s02-local-fallback	passed$' "S02 local-fallback phase did not pass" "$ARTIFACT_DIR/04-s02-contract.log"
assert_file_contains_regex s02-contract "$ARTIFACT_DIR/04-s02-status.txt" '^ok$' "S02 status must be ok" "$ARTIFACT_DIR/04-s02-contract.log"
assert_file_contains_regex s02-contract "$ARTIFACT_DIR/04-s02-current-phase.txt" '^complete$' "S02 current phase must be complete" "$ARTIFACT_DIR/04-s02-contract.log"

DEGRADE_BEFORE="$ARTIFACT_DIR/05-s03-degrade.before.txt"
capture_s03_snapshot "$DEGRADE_BEFORE"
run_expect_success s03-degrade 05-s03-degrade yes 180 \
  cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_degrades_safely_and_serves_locally_after_peer_loss -- --nocapture
collect_phase_artifacts \
  s03-degrade \
  degrade \
  "$DEGRADE_BEFORE" \
  "$ARTIFACT_DIR/05-s03-degrade-artifacts" \
  "$ARTIFACT_DIR/05-s03-degrade-artifacts.txt"

REJOIN_BEFORE="$ARTIFACT_DIR/06-s03-rejoin.before.txt"
capture_s03_snapshot "$REJOIN_BEFORE"
run_expect_success s03-rejoin 06-s03-rejoin yes 180 \
  cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_rejoins_and_routes_to_peer_again_without_manual_repair -- --nocapture
collect_phase_artifacts \
  s03-rejoin \
  rejoin \
  "$REJOIN_BEFORE" \
  "$ARTIFACT_DIR/06-s03-rejoin-artifacts" \
  "$ARTIFACT_DIR/06-s03-rejoin-artifacts.txt"

echo "verify-m039-s03: ok"
