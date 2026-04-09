#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ARTIFACT_ROOT=".tmp/m044-s02"
ARTIFACT_DIR="$ARTIFACT_ROOT/verify"
PHASE_REPORT_PATH="$ARTIFACT_DIR/phase-report.txt"
STATUS_PATH="$ARTIFACT_DIR/status.txt"
CURRENT_PHASE_PATH="$ARTIFACT_DIR/current-phase.txt"
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

capture_m044_s02_snapshot() {
  local snapshot_path="$1"
  python3 - "$snapshot_path" <<'PY'
from pathlib import Path
import sys

snapshot_path = Path(sys.argv[1])
root = Path('.tmp/m044-s02')
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

copy_new_m044_s02_artifacts() {
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
source_root = Path('.tmp/m044-s02')

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
    raise SystemExit('expected at least one new .tmp/m044-s02 artifact directory from the cluster-proof e2e replay')

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

assert_segment_absent() {
  local phase="$1"
  local label="$2"
  local path="$3"
  local start_marker="$4"
  local end_marker="$5"
  shift 5
  if ! python3 - "$path" "$start_marker" "$end_marker" "$@" >"$ARTIFACT_DIR/${label}.log" 2>&1 <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
start_marker = sys.argv[2]
end_marker = sys.argv[3]
needles = sys.argv[4:]
text = path.read_text(errors='replace')
start = text.find(start_marker)
if start < 0:
    raise SystemExit(f'missing start marker {start_marker!r} in {path}')
end = text.find(end_marker, start + len(start_marker))
if end < 0:
    raise SystemExit(f'missing end marker {end_marker!r} in {path}')
segment = text[start:end]
present = [needle for needle in needles if needle in segment]
if present:
    raise SystemExit(f'{path}: stale literals present between markers {start_marker!r} -> {end_marker!r}: {present}')
print(f'{path}: forbidden literals absent between markers {start_marker!r} -> {end_marker!r}')
PY
  then
    record_phase "$phase" failed
    fail_phase "$phase" "stale declared-runtime hot-path literals survived in ${path}" "$ARTIFACT_DIR/${label}.log" "$path"
  fi
}

run_expect_success s01-contract 00-s01-contract no 900 \
  bash scripts/verify-m044-s01.sh
assert_file_exists s01-contract .tmp/m044-s01/verify/phase-report.txt "S01 phase report" "$ARTIFACT_DIR/00-s01-contract.log"
assert_file_exists s01-contract .tmp/m044-s01/verify/status.txt "S01 status" "$ARTIFACT_DIR/00-s01-contract.log"
assert_file_exists s01-contract .tmp/m044-s01/verify/current-phase.txt "S01 current phase" "$ARTIFACT_DIR/00-s01-contract.log"
cp .tmp/m044-s01/verify/phase-report.txt "$ARTIFACT_DIR/00-s01-phase-report.txt"
cp .tmp/m044-s01/verify/status.txt "$ARTIFACT_DIR/00-s01-status.txt"
cp .tmp/m044-s01/verify/current-phase.txt "$ARTIFACT_DIR/00-s01-current-phase.txt"
assert_file_contains_regex s01-contract "$ARTIFACT_DIR/00-s01-phase-report.txt" '^manifest-parser\tpassed$' "S01 manifest parser replay did not pass" "$ARTIFACT_DIR/00-s01-contract.log"
assert_file_contains_regex s01-contract "$ARTIFACT_DIR/00-s01-phase-report.txt" '^meshc-typed-runtime\tpassed$' "S01 typed runtime replay did not pass" "$ARTIFACT_DIR/00-s01-contract.log"
assert_file_contains_regex s01-contract "$ARTIFACT_DIR/00-s01-phase-report.txt" '^cluster-proof-tests\tpassed$' "S01 cluster-proof tests replay did not pass" "$ARTIFACT_DIR/00-s01-contract.log"
assert_file_contains_regex s01-contract "$ARTIFACT_DIR/00-s01-status.txt" '^ok$' "S01 status must be ok" "$ARTIFACT_DIR/00-s01-contract.log"
assert_file_contains_regex s01-contract "$ARTIFACT_DIR/00-s01-current-phase.txt" '^complete$' "S01 current phase must be complete" "$ARTIFACT_DIR/00-s01-contract.log"

run_expect_success mesh-rt-staticlib 01-mesh-rt-staticlib no 240 \
  cargo build -q -p mesh-rt
run_expect_success s02-metadata 02-s02-metadata yes 480 \
  cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture
run_expect_success s02-declared-work 03-s02-declared-work yes 900 \
  cargo test -p meshc --test e2e_m044_s02 m044_s02_declared_work_ -- --nocapture
run_expect_success s02-service 04-s02-service yes 900 \
  cargo test -p meshc --test e2e_m044_s02 m044_s02_service_ -- --nocapture

S02_CLUSTER_BEFORE="$ARTIFACT_DIR/05-s02-cluster-proof.before.txt"
capture_m044_s02_snapshot "$S02_CLUSTER_BEFORE"
run_expect_success s02-cluster-proof 05-s02-cluster-proof yes 1200 \
  cargo test -p meshc --test e2e_m044_s02 m044_s02_cluster_proof_ -- --nocapture
record_phase s02-cluster-proof-artifacts started
copy_new_m044_s02_artifacts \
  s02-cluster-proof-artifacts \
  "$S02_CLUSTER_BEFORE" \
  "$ARTIFACT_DIR/05-s02-cluster-proof-artifacts" \
  "$ARTIFACT_DIR/05-s02-cluster-proof-artifacts.txt"
record_phase s02-cluster-proof-artifacts passed

run_expect_success cluster-proof-build 06-cluster-proof-build no 480 \
  cargo run -q -p meshc -- build cluster-proof
run_expect_success cluster-proof-tests 07-cluster-proof-tests no 480 \
  cargo run -q -p meshc -- test cluster-proof/tests

record_phase hot-submit-selection-absence started
assert_segment_absent \
  hot-submit-selection-absence \
  08-hot-submit-selection-absence \
  cluster-proof/work_continuity.mpl \
  'fn handle_valid_submit(submit :: WorkSubmitBody) do' \
  'fn status_response_from_record(record :: ContinuityRecord, source_node :: String) do' \
  'current_target_selection(' \
  'submit_from_selection('
record_phase hot-submit-selection-absence passed

record_phase hot-submit-dispatch-absence started
assert_segment_absent \
  hot-submit-dispatch-absence \
  09-hot-submit-dispatch-absence \
  cluster-proof/work_continuity.mpl \
  'fn created_submit_response(' \
  'fn rejected_submit_response(record :: ContinuityRecord) do' \
  'dispatch_work(' \
  'spawn_remote_work(' \
  'spawn_local_work(' \
  'Node.spawn('
record_phase hot-submit-dispatch-absence passed

record_phase hot-status-legacy-absence started
assert_segment_absent \
  hot-status-legacy-absence \
  10-hot-status-legacy-absence \
  cluster-proof/work_continuity.mpl \
  'fn handle_valid_status(request_key :: String) do' \
  'fn invalid_request_response(request_key :: String, reason :: String) do' \
  'current_target_selection(' \
  'dispatch_work(' \
  'Node.spawn('
record_phase hot-status-legacy-absence passed

assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^s01-contract\tpassed$' "S01 prerequisite replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^mesh-rt-staticlib\tpassed$' "mesh-rt refresh did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^s02-metadata\tpassed$' "S02 metadata filter did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^s02-declared-work\tpassed$' "S02 declared-work filter did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^s02-service\tpassed$' "S02 service filter did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^s02-cluster-proof\tpassed$' "S02 cluster-proof filter did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^s02-cluster-proof-artifacts\tpassed$' "S02 cluster-proof artifacts were not retained" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^cluster-proof-build\tpassed$' "cluster-proof build did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^cluster-proof-tests\tpassed$' "cluster-proof tests did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^hot-submit-selection-absence\tpassed$' "submit hot path still depends on target selection helpers" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^hot-submit-dispatch-absence\tpassed$' "submit hot path still depends on Mesh dispatch helpers" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^hot-status-legacy-absence\tpassed$' "status hot path still references legacy dispatch helpers" "$ARTIFACT_DIR/full-contract.log"

echo "verify-m044-s02: ok"
