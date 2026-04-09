#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ARTIFACT_ROOT=".tmp/m046-s03"
ARTIFACT_DIR="$ARTIFACT_ROOT/verify"
PHASE_REPORT_PATH="$ARTIFACT_DIR/phase-report.txt"
STATUS_PATH="$ARTIFACT_DIR/status.txt"
CURRENT_PHASE_PATH="$ARTIFACT_DIR/current-phase.txt"
LATEST_PROOF_BUNDLE_PATH="$ARTIFACT_DIR/latest-proof-bundle.txt"

rm -rf "$ARTIFACT_DIR"
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
for line in lines[:220]:
    print(line)
if len(lines) > 220:
    print(f"... truncated after 220 lines (total {len(lines)})")
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

assert_file_lacks_regex() {
  local phase="$1"
  local path="$2"
  local regex="$3"
  local description="$4"
  if ! python3 - "$path" "$regex" "$description" >"$ARTIFACT_DIR/${phase}.content-check.log" 2>&1 <<'PY'
from pathlib import Path
import re
import sys

path = Path(sys.argv[1])
regex = sys.argv[2]
description = sys.argv[3]
text = path.read_text(errors="replace")
match = re.search(regex, text, re.MULTILINE)
if match:
    raise SystemExit(
        f"{description}: matched forbidden regex {regex!r} in {path} at {match.start()}..{match.end()}"
    )
print(f"{description}: no match for {regex!r}")
PY
  then
    fail_phase "$phase" "$description" "$ARTIFACT_DIR/${phase}.content-check.log" "$path"
  fi
}

run_command() {
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

assert_test_filter_ran() {
  local phase="$1"
  local log_path="$2"
  local label="$3"
  if ! python3 - "$log_path" "$label" >"$ARTIFACT_DIR/${label}.test-count.log" 2>&1 <<'PY'
import re
import sys
from pathlib import Path

text = Path(sys.argv[1]).read_text(errors="replace")
label = sys.argv[2]
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

run_expect_success() {
  local phase="$1"
  local label="$2"
  local require_tests="$3"
  local timeout_secs="$4"
  shift 4
  local -a cmd=("$@")
  local log_path="$ARTIFACT_DIR/${label}.log"
  record_phase "$phase" started
  printf '%s\n' "$phase" >"$CURRENT_PHASE_PATH"
  echo "==> ${cmd[*]}"
  if ! run_command "$timeout_secs" "$log_path" "${cmd[@]}"; then
    record_phase "$phase" failed
    fail_phase "$phase" "expected success within ${timeout_secs}s" "$log_path"
  fi
  if [[ "$require_tests" == "yes" ]]; then
    assert_test_filter_ran "$phase" "$log_path" "$label"
  fi
  record_phase "$phase" passed
}

capture_m046_s03_snapshot() {
  local snapshot_path="$1"
  python3 - "$snapshot_path" <<'PY'
from pathlib import Path
import sys

snapshot_path = Path(sys.argv[1])
root = Path('.tmp/m046-s03')
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

copy_new_m046_s03_artifacts() {
  local phase="$1"
  local before_snapshot="$2"
  local dest_root="$3"
  local manifest_path="$4"

  if ! python3 - "$before_snapshot" "$dest_root" >"$manifest_path" 2>"$ARTIFACT_DIR/${phase}.artifact-check.log" <<'PY'
from pathlib import Path
import shutil
import sys

before_snapshot = Path(sys.argv[1])
dest_root = Path(sys.argv[2])
source_root = Path('.tmp/m046-s03')

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
if not new_names:
    raise SystemExit('expected fresh .tmp/m046-s03 artifact directories from the S03 failover replay')

if dest_root.exists():
    shutil.rmtree(dest_root)
dest_root.mkdir(parents=True, exist_ok=True)
manifest_lines = []
for name in new_names:
    src = after_paths[name]
    if not any(src.iterdir()):
        raise SystemExit(f'{src}: expected non-empty artifact directory')
    dst = dest_root / name
    shutil.copytree(src, dst)
    manifest_lines.append(f'{name}\t{src}')
    for child in sorted(src.rglob('*')):
        if child.is_file():
            manifest_lines.append(f'  - {child}')

print('\n'.join(manifest_lines))
PY
  then
    fail_phase "$phase" "missing or malformed copied evidence" "$ARTIFACT_DIR/${phase}.artifact-check.log" "$dest_root"
  fi
}

assert_retained_bundle_shape() {
  local phase="$1"
  local dest_root="$2"
  local manifest_path="$3"
  local pointer_path="$4"
  local log_path="$ARTIFACT_DIR/${phase}.bundle-check.log"
  if ! python3 - "$dest_root" "$manifest_path" "$pointer_path" >"$log_path" 2>&1 <<'PY'
from pathlib import Path
import json
import sys

bundle_root = Path(sys.argv[1])
manifest_path = Path(sys.argv[2])
pointer_path = Path(sys.argv[3])
expected_pointer = str(bundle_root)
actual_pointer = pointer_path.read_text(errors='replace').strip()
if actual_pointer != expected_pointer:
    raise SystemExit(
        f'latest-proof-bundle pointer drifted: expected {expected_pointer!r}, got {actual_pointer!r}'
    )
manifest_lines = [line for line in manifest_path.read_text(errors='replace').splitlines() if line.strip()]
if not manifest_lines:
    raise SystemExit(f'{manifest_path}: expected non-empty copied-artifact manifest')

children = sorted(path for path in bundle_root.iterdir() if path.is_dir())
if not children:
    raise SystemExit(f'{bundle_root}: expected copied artifact directories')

scenario_dirs = [path for path in children if (path / 'scenario-meta.json').is_file()]
if len(scenario_dirs) != 1:
    raise SystemExit(
        f'{bundle_root}: expected exactly one copied failover bundle with scenario-meta.json, found {[path.name for path in scenario_dirs]}'
    )
scenario_dir = scenario_dirs[0]
if not scenario_dir.name.startswith('tiny-cluster-failover-runtime-truth-'):
    raise SystemExit(
        f'{scenario_dir}: expected tiny-cluster failover runtime truth artifact prefix'
    )

scenario = json.loads((scenario_dir / 'scenario-meta.json').read_text())
if not scenario.get('request_key'):
    raise SystemExit(f'{scenario_dir}/scenario-meta.json: missing request_key')
if not scenario.get('initial_attempt_id'):
    raise SystemExit(f'{scenario_dir}/scenario-meta.json: missing initial_attempt_id')
if not scenario.get('failover_attempt_id'):
    raise SystemExit(f'{scenario_dir}/scenario-meta.json: missing failover_attempt_id')

required_files = [
    'scenario-meta.json',
    'pre-kill-status-primary.json',
    'pre-kill-status-standby.json',
    'pre-kill-continuity-list-primary.json',
    'pre-kill-continuity-list-standby.json',
    'pre-kill-continuity-primary.json',
    'pre-kill-continuity-standby.json',
    'post-kill-status-standby.json',
    'post-kill-diagnostics-standby.json',
    'post-kill-continuity-standby-completed.json',
    'post-rejoin-status-primary.json',
    'post-rejoin-status-standby.json',
    'post-rejoin-diagnostics-primary.json',
    'post-rejoin-continuity-primary.json',
    'post-rejoin-continuity-standby.json',
    'primary-run1.stdout.log',
    'primary-run1.stderr.log',
    'primary-run2.stdout.log',
    'primary-run2.stderr.log',
    'standby-run1.stdout.log',
    'standby-run1.stderr.log',
]
for name in required_files:
    if not (scenario_dir / name).exists():
        raise SystemExit(f'{scenario_dir}: missing required retained file {name}')

print('retained-bundle-shape: ok')
PY
  then
    fail_phase "$phase" "missing retained proof artifacts or malformed bundle pointer" "$log_path" "$dest_root"
  fi
}

record_phase contract-guards started
printf 'contract-guards\n' >"$CURRENT_PHASE_PATH"
assert_file_lacks_regex \
  contract-work \
  tiny-cluster/work.mpl \
  'Env\.get_int|Timer\.sleep|TINY_CLUSTER_.*DELAY|MESH_STARTUP_WORK_DELAY_MS' \
  'tiny-cluster work reintroduced a package or user timing seam'
assert_file_contains_regex \
  contract-smoke \
  tiny-cluster/tests/work.test.mpl \
  '(?s)assert_not_contains\(work_source, "Env\.get_int"\).*assert_not_contains\(work_source, "Timer\.sleep"\).*assert_not_contains\(work_source, "TINY_CLUSTER_WORK_DELAY_MS"\).*assert_not_contains\(work_source, "MESH_STARTUP_WORK_DELAY_MS"\)' \
  'tiny-cluster smoke rail lost the timing-seam regression guards'
assert_file_lacks_regex \
  contract-readme \
  tiny-cluster/README.md \
  'TINY_CLUSTER_.*DELAY|MESH_STARTUP_WORK_DELAY_MS|/work|/health|HTTP\.serve|Continuity\.' \
  'tiny-cluster README reintroduced timing guidance or routes'
assert_file_lacks_regex \
  contract-e2e \
  compiler/meshc/tests/e2e_m046_s03.rs \
  '\\.env\("MESH_STARTUP_WORK_DELAY_MS"|startup_work_delay_ms' \
  'S03 e2e rail reintroduced runtime delay injection'
assert_file_contains_regex \
  contract-plan \
  .gsd/milestones/M046/slices/S03/S03-PLAN.md \
  '\*\*T04: Replaced the last tiny-cluster failover timing knob with a language-owned startup dispatch window and fail-closed contract guards\.\*\*' \
  'S03 plan lost the completed T04 timing-seam closure task'
assert_file_contains_regex \
  contract-plan \
  .gsd/milestones/M046/slices/S03/S03-PLAN.md \
  'user-directed `MESH_STARTUP_WORK_DELAY_MS` guidance' \
  'S03 plan stopped tracking runtime-env timing guidance removal'
assert_file_contains_regex \
  contract-plan \
  .gsd/milestones/M046/slices/S03/S03-PLAN.md \
  'without app/user-owned timing seams' \
  'S03 plan stopped requiring the app-owned timing seam to stay retired'
record_phase contract-guards passed

run_expect_success mesh-rt-build 00-mesh-rt-build no 3600 \
  cargo build -q -p mesh-rt
run_expect_success m046-s02-startup 01-m046-s02-startup yes 2400 \
  cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_tiny_route_free_startup_dedupes_on_two_nodes -- --nocapture
run_expect_success tiny-cluster-build 02-tiny-cluster-build no 1200 \
  cargo run -q -p meshc -- build tiny-cluster
run_expect_success tiny-cluster-tests 03-tiny-cluster-tests no 1200 \
  cargo run -q -p meshc -- test tiny-cluster/tests

S03_BEFORE="$ARTIFACT_DIR/04-m046-s03.before.txt"
capture_m046_s03_snapshot "$S03_BEFORE"
run_expect_success m046-s03-e2e 04-m046-s03-e2e yes 3600 \
  cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_failover_ -- --nocapture
record_phase m046-s03-artifacts started
BUNDLE_ROOT="$ARTIFACT_DIR/retained-m046-s03-artifacts"
copy_new_m046_s03_artifacts \
  m046-s03-artifacts \
  "$S03_BEFORE" \
  "$BUNDLE_ROOT" \
  "$ARTIFACT_DIR/04-m046-s03-artifacts.txt"
printf '%s\n' "$BUNDLE_ROOT" >"$LATEST_PROOF_BUNDLE_PATH"
record_phase m046-s03-artifacts passed
record_phase m046-s03-bundle-shape started
assert_retained_bundle_shape \
  m046-s03-bundle-shape \
  "$BUNDLE_ROOT" \
  "$ARTIFACT_DIR/04-m046-s03-artifacts.txt" \
  "$LATEST_PROOF_BUNDLE_PATH"
record_phase m046-s03-bundle-shape passed

assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^contract-guards\tpassed$' "contract guards did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^mesh-rt-build\tpassed$' "mesh-rt build did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m046-s02-startup\tpassed$' "M046 S02 startup rail did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^tiny-cluster-build\tpassed$' "tiny-cluster build did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^tiny-cluster-tests\tpassed$' "tiny-cluster package tests did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m046-s03-e2e\tpassed$' "M046 S03 failover rail did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m046-s03-artifacts\tpassed$' "M046 S03 artifacts were not retained" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m046-s03-bundle-shape\tpassed$' "M046 S03 bundle shape check did not pass" "$ARTIFACT_DIR/full-contract.log"

echo "verify-m046-s03: ok"
