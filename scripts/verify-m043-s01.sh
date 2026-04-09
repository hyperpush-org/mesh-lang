#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

source scripts/lib/clustered_fixture_paths.sh
clustered_fixture_require_cluster_proof_root
source scripts/lib/m043_cluster_proof.sh

ARTIFACT_ROOT=".tmp/m043-s01"
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

capture_snapshot() {
  local snapshot_path="$1"
  python3 - "$snapshot_path" <<'PY'
from pathlib import Path
import sys

snapshot_path = Path(sys.argv[1])
root = Path('.tmp/m043-s01')
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

select_phase_artifacts() {
  local phase="$1"
  local mode="$2"
  local before_snapshot="$3"
  local selection_path="$4"

  if ! python3 - "$before_snapshot" "$mode" >"$selection_path" 2>"$ARTIFACT_DIR/${phase}.artifact-check.log" <<'PY'
from pathlib import Path
import sys

before_snapshot = Path(sys.argv[1])
mode = sys.argv[2]
source_root = Path('.tmp/m043-s01')

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
    'malformed-contract': {
        'continuity-api-m043-malformed-contract-': [
            'missing-membership-authority.json',
            'missing-work-authority.json',
        ],
    },
    'primary-to-standby': {
        'continuity-api-m043-primary-to-standby-': [
            'membership-primary.http',
            'membership-primary.json',
            'membership-standby.http',
            'membership-standby.json',
            'submit-primary.http',
            'submit-primary.json',
            'pending-primary.http',
            'pending-primary.json',
            'pending-standby.http',
            'pending-standby.json',
            'completed-primary.http',
            'completed-primary.json',
            'completed-standby.http',
            'completed-standby.json',
            'repeated-primary.http',
            'repeated-primary.json',
            'repeated-standby.http',
            'repeated-standby.json',
            'scenario-meta.json',
            'primary.stdout.log',
            'primary.stderr.log',
            'standby.stdout.log',
            'standby.stderr.log',
        ],
    },
}

if mode not in expected:
    raise SystemExit(f'unknown artifact mode: {mode}')
mode_expected = expected[mode]
if len(new_names) < len(mode_expected):
    raise SystemExit(
        f'{mode}: expected at least {len(mode_expected)} new artifact directories, found {len(new_names)} -> {new_names}'
    )

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
        if path.stat().st_size <= 0:
            raise SystemExit(f'{path}: expected non-empty artifact file')
    print(f'{name}\t{src}')
PY
  then
    fail_phase "$phase" "missing or malformed copied evidence" "$ARTIFACT_DIR/${phase}.artifact-check.log"
  fi
}

copy_selected_artifacts() {
  local phase="$1"
  local selection_path="$2"
  local dest_root="$3"
  local manifest_path="$4"

  rm -rf "$dest_root"
  mkdir -p "$dest_root"
  : >"$manifest_path"

  while IFS=$'\t' read -r name source_dir; do
    [[ -n "$name" ]] || continue
    local copied_dir="$dest_root/$name"
    local copied_manifest="$dest_root/${name}.manifest.txt"
    if ! m043_copy_artifact_dir "$source_dir" "$copied_dir" "$copied_manifest"; then
      fail_phase "$phase" "failed to copy retained artifact dir" "$copied_manifest" "$source_dir"
    fi
    printf '%s\t%s\n' "$name" "$source_dir" >>"$manifest_path"
    cat "$copied_manifest" >>"$manifest_path"
  done <"$selection_path"
}

run_expect_success runtime-continuity 00-runtime-continuity yes 240 \
  cargo test -p mesh-rt continuity -- --nocapture
run_expect_success cluster-proof-tests 01-cluster-proof-tests no 240 \
  cargo run -q -p meshc -- test "$CLUSTER_PROOF_FIXTURE_TESTS"
run_expect_success build-cluster-proof 02-build-cluster-proof no 240 \
  cargo run -q -p meshc -- build "$CLUSTER_PROOF_FIXTURE_ROOT"

M043_BEFORE="$ARTIFACT_DIR/03-m043.before.txt"
capture_snapshot "$M043_BEFORE"
run_expect_success m043-e2e 03-m043-e2e yes 480 \
  cargo test -p meshc --test e2e_m043_s01 -- --nocapture
assert_file_contains_regex m043-e2e "$ARTIFACT_DIR/03-m043-e2e.log" 'test e2e_m043_s01_missing_authority_fields_fail_closed \.\.\. ok' "missing-authority negative test did not pass" "$ARTIFACT_DIR/03-m043-e2e.log"
assert_file_contains_regex m043-e2e "$ARTIFACT_DIR/03-m043-e2e.log" 'test e2e_m043_s01_primary_submit_mirrors_truth_to_standby_without_promotion \.\.\. ok' "primary→standby mirrored truth test did not pass" "$ARTIFACT_DIR/03-m043-e2e.log"

MALFORMED_SELECTION="$ARTIFACT_DIR/04-malformed.selection.txt"
record_phase malformed-contract started
select_phase_artifacts malformed-contract malformed-contract "$M043_BEFORE" "$MALFORMED_SELECTION"
copy_selected_artifacts malformed-contract "$MALFORMED_SELECTION" "$ARTIFACT_DIR/04-malformed-artifacts" "$ARTIFACT_DIR/04-malformed-artifacts.txt"
record_phase malformed-contract passed

PRIMARY_SELECTION="$ARTIFACT_DIR/05-primary-standby.selection.txt"
record_phase primary-to-standby started
select_phase_artifacts primary-to-standby primary-to-standby "$M043_BEFORE" "$PRIMARY_SELECTION"
copy_selected_artifacts primary-to-standby "$PRIMARY_SELECTION" "$ARTIFACT_DIR/05-primary-to-standby-artifacts" "$ARTIFACT_DIR/05-primary-to-standby-artifacts.txt"

PRIMARY_DIR="$(find "$ARTIFACT_DIR/05-primary-to-standby-artifacts" -mindepth 1 -maxdepth 1 -type d | head -n 1)"
if [[ -z "$PRIMARY_DIR" ]]; then
  fail_phase primary-to-standby "copied primary→standby artifact directory is missing" "$ARTIFACT_DIR/05-primary-to-standby-artifacts.txt"
fi
SCENARIO_META="$PRIMARY_DIR/scenario-meta.json"
assert_file_exists primary-to-standby "$SCENARIO_META" "scenario metadata" "$ARTIFACT_DIR/05-primary-to-standby-artifacts.txt"
read -r REQUEST_KEY ATTEMPT_ID PRIMARY_NODE STANDBY_NODE < <(python3 - "$SCENARIO_META" <<'PY'
from pathlib import Path
import json
import sys

path = Path(sys.argv[1])
data = json.loads(path.read_text(errors='replace'))
print(data['request_key'], data['attempt_id'], data['primary_node'], data['standby_node'])
PY
)
EXPECTED_MEMBERSHIP="${PRIMARY_NODE}|${STANDBY_NODE}"

m043_assert_membership_payload_json "$PRIMARY_DIR/membership-primary.json" "$PRIMARY_NODE" "$EXPECTED_MEMBERSHIP" primary 0 local_only "primary membership truth" || fail_phase primary-to-standby "primary membership truth drifted" "$PRIMARY_DIR/membership-primary.json"
m043_assert_membership_payload_json "$PRIMARY_DIR/membership-standby.json" "$STANDBY_NODE" "$EXPECTED_MEMBERSHIP" standby 0 local_only "standby membership truth" || fail_phase primary-to-standby "standby membership truth drifted" "$PRIMARY_DIR/membership-standby.json"
m043_assert_keyed_payload_json "$PRIMARY_DIR/submit-primary.json" "$REQUEST_KEY" "$ATTEMPT_ID" submitted pending "$PRIMARY_NODE" "$PRIMARY_NODE" "$STANDBY_NODE" mirrored primary 0 healthy '' false true true '' '' "primary submit truth" || fail_phase primary-to-standby "primary submit truth drifted" "$PRIMARY_DIR/submit-primary.json"
m043_assert_keyed_payload_json "$PRIMARY_DIR/pending-primary.json" "$REQUEST_KEY" "$ATTEMPT_ID" submitted pending "$PRIMARY_NODE" "$PRIMARY_NODE" "$STANDBY_NODE" mirrored primary 0 healthy '' false true true '' '' "pending primary truth" || fail_phase primary-to-standby "pending primary truth drifted" "$PRIMARY_DIR/pending-primary.json"
m043_assert_keyed_payload_json "$PRIMARY_DIR/pending-standby.json" "$REQUEST_KEY" "$ATTEMPT_ID" submitted pending "$PRIMARY_NODE" "$PRIMARY_NODE" "$STANDBY_NODE" mirrored standby 0 healthy '' false true true '' '' "pending standby truth" || fail_phase primary-to-standby "pending standby truth drifted" "$PRIMARY_DIR/pending-standby.json"
m043_assert_keyed_payload_json "$PRIMARY_DIR/completed-primary.json" "$REQUEST_KEY" "$ATTEMPT_ID" completed succeeded "$PRIMARY_NODE" "$PRIMARY_NODE" "$STANDBY_NODE" mirrored primary 0 healthy "$PRIMARY_NODE" false true true '' '' "completed primary truth" || fail_phase primary-to-standby "completed primary truth drifted" "$PRIMARY_DIR/completed-primary.json"
m043_assert_keyed_payload_json "$PRIMARY_DIR/completed-standby.json" "$REQUEST_KEY" "$ATTEMPT_ID" completed succeeded "$PRIMARY_NODE" "$PRIMARY_NODE" "$STANDBY_NODE" mirrored standby 0 healthy "$PRIMARY_NODE" false true true '' '' "completed standby truth" || fail_phase primary-to-standby "completed standby truth drifted" "$PRIMARY_DIR/completed-standby.json"
m043_assert_keyed_payload_json "$PRIMARY_DIR/repeated-primary.json" "$REQUEST_KEY" "$ATTEMPT_ID" completed succeeded "$PRIMARY_NODE" "$PRIMARY_NODE" "$STANDBY_NODE" mirrored primary 0 healthy "$PRIMARY_NODE" false true true '' '' "repeated primary truth" || fail_phase primary-to-standby "repeated primary truth drifted" "$PRIMARY_DIR/repeated-primary.json"
m043_assert_keyed_payload_json "$PRIMARY_DIR/repeated-standby.json" "$REQUEST_KEY" "$ATTEMPT_ID" completed succeeded "$PRIMARY_NODE" "$PRIMARY_NODE" "$STANDBY_NODE" mirrored standby 0 healthy "$PRIMARY_NODE" false true true '' '' "repeated standby truth" || fail_phase primary-to-standby "repeated standby truth drifted" "$PRIMARY_DIR/repeated-standby.json"
assert_file_contains_regex primary-to-standby "$PRIMARY_DIR/primary.stdout.log" "keyed submit request_key=${REQUEST_KEY} attempt_id=${ATTEMPT_ID} .*cluster_role=primary promotion_epoch=0 replication_health=healthy" "primary log is missing primary-role submit truth" "$PRIMARY_DIR/primary.stdout.log"
assert_file_contains_regex primary-to-standby "$PRIMARY_DIR/standby.stdout.log" "keyed status request_key=${REQUEST_KEY} attempt_id=${ATTEMPT_ID} .*cluster_role=standby promotion_epoch=0 replication_health=healthy" "standby log is missing standby-role mirrored truth" "$PRIMARY_DIR/standby.stdout.log"
if grep -q "work executed request_key=${REQUEST_KEY} attempt_id=${ATTEMPT_ID} execution=${STANDBY_NODE}" "$PRIMARY_DIR/standby.stdout.log"; then
  fail_phase primary-to-standby "standby log claims local execution during pre-promotion mirror proof" "$PRIMARY_DIR/standby.stdout.log"
fi
record_phase primary-to-standby passed

assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^runtime-continuity\tpassed$' "runtime continuity replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^cluster-proof-tests\tpassed$' "cluster-proof tests replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^build-cluster-proof\tpassed$' "cluster-proof build replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m043-e2e\tpassed$' "M043 e2e replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^malformed-contract\tpassed$' "Malformed-contract artifact validation did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^primary-to-standby\tpassed$' "Primary→standby artifact validation did not pass" "$ARTIFACT_DIR/full-contract.log"

echo "verify-m043-s01: ok"
