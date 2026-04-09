#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

source scripts/lib/clustered_fixture_paths.sh
clustered_fixture_require_cluster_proof_root
source scripts/lib/m043_cluster_proof.sh

ARTIFACT_ROOT=".tmp/m043-s02"
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

assert_file_contains_literal() {
  local phase="$1"
  local path="$2"
  local needle="$3"
  local description="$4"
  local log_path="${5:-}"
  if ! python3 - "$path" "$needle" "$description" >"$ARTIFACT_DIR/${phase}.literal-check.log" 2>&1 <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
needle = sys.argv[2]
description = sys.argv[3]
text = path.read_text(errors="replace")
if needle not in text:
    raise SystemExit(f"{description}: missing literal {needle!r} in {path}")
print(f"{description}: matched literal {needle!r}")
PY
  then
    fail_phase "$phase" "$description" "$ARTIFACT_DIR/${phase}.literal-check.log" "$path"
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
root = Path('.tmp/m043-s02')
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

select_failover_artifacts() {
  local phase="$1"
  local before_snapshot="$2"
  local selection_path="$3"

  if ! python3 - "$before_snapshot" >"$selection_path" 2>"$ARTIFACT_DIR/${phase}.artifact-check.log" <<'PY'
from pathlib import Path
import sys

before_snapshot = Path(sys.argv[1])
source_root = Path('.tmp/m043-s02')

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
prefix = 'continuity-api-failover-promotion-rejoin-'
matches = [name for name in new_names if name.startswith(prefix)]
if len(matches) != 1:
    raise SystemExit(f'expected exactly one new failover artifact dir for prefix {prefix!r}, found {matches}; new_names={new_names}')
name = matches[0]
src = after_paths[name]
required = [
    'scenario-meta.json',
    'submit-primary.http',
    'submit-primary.json',
    'membership-primary-run1.http',
    'membership-primary-run1.json',
    'membership-standby-run1.http',
    'membership-standby-run1.json',
    'pending-primary.http',
    'pending-primary.json',
    'pending-standby.http',
    'pending-standby.json',
    'primary-run1.stdout.log',
    'primary-run1.stderr.log',
    'degraded-membership-standby.http',
    'degraded-membership-standby.json',
    'pre-promote-standby-status.http',
    'pre-promote-standby-status.json',
    'promote-standby.http',
    'promote-standby.json',
    'promoted-membership-standby.http',
    'promoted-membership-standby.json',
    'promoted-owner-lost-status.http',
    'promoted-owner-lost-status.json',
    'failover-retry.http',
    'failover-retry.json',
    'failover-pending-status.http',
    'failover-pending-status.json',
    'failover-completed-standby.http',
    'failover-completed-standby.json',
    'membership-primary-run2.http',
    'membership-primary-run2.json',
    'membership-standby-run2.http',
    'membership-standby-run2.json',
    'post-rejoin-primary-status.http',
    'post-rejoin-primary-status.json',
    'post-rejoin-standby-status.http',
    'post-rejoin-standby-status.json',
    'stale-guard-primary.http',
    'stale-guard-primary.json',
    'primary-run2.stdout.log',
    'primary-run2.stderr.log',
    'standby-run1.stdout.log',
    'standby-run1.stderr.log',
    'failover-promotion-rejoin-search/chosen.json',
    'failover-promotion-rejoin-search/selected.http',
    'failover-promotion-rejoin-search/selected.json',
]
missing = [rel for rel in required if not (src / rel).is_file()]
if missing:
    raise SystemExit(f'{src}: missing required failover artifacts {missing}')
for rel in required:
    path = src / rel
    if path.stat().st_size <= 0:
        raise SystemExit(f'{path}: expected non-empty artifact file')
print(f'{name}\t{src}')
PY
  then
    fail_phase "$phase" "missing or malformed failover evidence" "$ARTIFACT_DIR/${phase}.artifact-check.log"
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

run_expect_success runtime-continuity 00-runtime-continuity yes 360 \
  cargo test -p mesh-rt continuity -- --nocapture
run_expect_success cluster-proof-tests 01-cluster-proof-tests no 360 \
  cargo run -q -p meshc -- test "$CLUSTER_PROOF_FIXTURE_TESTS"
run_expect_success build-cluster-proof 02-build-cluster-proof no 360 \
  cargo run -q -p meshc -- build "$CLUSTER_PROOF_FIXTURE_ROOT"

run_expect_success s01-contract 03-s01-contract no 720 \
  bash scripts/verify-m043-s01.sh
assert_file_exists s01-contract .tmp/m043-s01/verify/phase-report.txt "S01 phase report" "$ARTIFACT_DIR/03-s01-contract.log"
assert_file_exists s01-contract .tmp/m043-s01/verify/status.txt "S01 status" "$ARTIFACT_DIR/03-s01-contract.log"
cp .tmp/m043-s01/verify/phase-report.txt "$ARTIFACT_DIR/03-s01-phase-report.txt"
cp .tmp/m043-s01/verify/status.txt "$ARTIFACT_DIR/03-s01-status.txt"
assert_file_contains_regex s01-contract "$ARTIFACT_DIR/03-s01-status.txt" '^ok$' "S01 verifier status must be ok" "$ARTIFACT_DIR/03-s01-contract.log"
assert_file_contains_regex s01-contract "$ARTIFACT_DIR/03-s01-phase-report.txt" '^primary-to-standby\tpassed$' "S01 primary→standby phase did not pass" "$ARTIFACT_DIR/03-s01-contract.log"

run_expect_success m042-rejoin 04-m042-rejoin yes 480 \
  cargo test -p meshc --test e2e_m042_s03 continuity_api_same_identity_rejoin_preserves_newer_attempt_truth -- --nocapture
assert_file_contains_regex m042-rejoin "$ARTIFACT_DIR/04-m042-rejoin.log" 'test continuity_api_same_identity_rejoin_preserves_newer_attempt_truth \.\.\. ok' "M042 same-identity rejoin regression did not pass" "$ARTIFACT_DIR/04-m042-rejoin.log"

run_expect_success m043-api 05-m043-api yes 600 \
  cargo test -p meshc --test e2e_m043_s02 continuity_api_ -- --nocapture
assert_file_contains_regex m043-api "$ARTIFACT_DIR/05-m043-api.log" 'test continuity_api_authority_status_and_promote_round_trip_runtime_truth \.\.\. ok' "runtime authority-status/promote API proof did not pass" "$ARTIFACT_DIR/05-m043-api.log"
assert_file_contains_regex m043-api "$ARTIFACT_DIR/05-m043-api.log" 'test continuity_api_primary_promotion_rejection_preserves_authority_truth \.\.\. ok' "primary promotion rejection proof did not pass" "$ARTIFACT_DIR/05-m043-api.log"
assert_file_contains_regex m043-api "$ARTIFACT_DIR/05-m043-api.log" 'test continuity_api_promote_wrong_arity_fails_at_compile_time \.\.\. ok' "compile-time promote arity guard did not pass" "$ARTIFACT_DIR/05-m043-api.log"
assert_file_contains_regex m043-api "$ARTIFACT_DIR/05-m043-api.log" 'test continuity_api_authority_status_wrong_result_shape_fails_at_compile_time \.\.\. ok' "compile-time authority-status shape guard did not pass" "$ARTIFACT_DIR/05-m043-api.log"

FAILOVER_BEFORE="$ARTIFACT_DIR/06-failover.before.txt"
capture_snapshot "$FAILOVER_BEFORE"
run_expect_success failover-contract 06-failover-contract yes 900 \
  cargo test -p meshc --test e2e_m043_s02 e2e_m043_s02_failover_promotion_and_fenced_rejoin_keep_newer_truth -- --nocapture
assert_file_contains_regex failover-contract "$ARTIFACT_DIR/06-failover-contract.log" 'test e2e_m043_s02_failover_promotion_and_fenced_rejoin_keep_newer_truth \.\.\. ok' "destructive failover harness did not pass" "$ARTIFACT_DIR/06-failover-contract.log"

FAILOVER_SELECTION="$ARTIFACT_DIR/07-failover.selection.txt"
record_phase failover-artifacts started
select_failover_artifacts failover-artifacts "$FAILOVER_BEFORE" "$FAILOVER_SELECTION"
copy_selected_artifacts failover-artifacts "$FAILOVER_SELECTION" "$ARTIFACT_DIR/07-failover-artifacts" "$ARTIFACT_DIR/07-failover-artifacts.txt"

FAILOVER_DIR="$(find "$ARTIFACT_DIR/07-failover-artifacts" -mindepth 1 -maxdepth 1 -type d | head -n 1)"
if [[ -z "$FAILOVER_DIR" ]]; then
  fail_phase failover-artifacts "copied failover artifact directory is missing" "$ARTIFACT_DIR/07-failover-artifacts.txt"
fi
SCENARIO_META="$FAILOVER_DIR/scenario-meta.json"
assert_file_exists failover-artifacts "$SCENARIO_META" "scenario metadata" "$ARTIFACT_DIR/07-failover-artifacts.txt"
read -r REQUEST_KEY ORIGINAL_ATTEMPT FAILOVER_ATTEMPT PRIMARY_NODE STANDBY_NODE PAYLOAD < <(python3 - "$SCENARIO_META" "$FAILOVER_DIR/failover-retry.json" <<'PY'
from pathlib import Path
import json
import sys

scenario = json.loads(Path(sys.argv[1]).read_text(errors='replace'))
retry = json.loads(Path(sys.argv[2]).read_text(errors='replace'))
print(
    scenario['request_key'],
    scenario['attempt_id'],
    retry['attempt_id'],
    scenario['primary_node'],
    scenario['standby_node'],
    scenario['payload'],
)
PY
)
EXPECTED_MEMBERSHIP="${PRIMARY_NODE}|${STANDBY_NODE}"

m043_assert_membership_payload_json "$FAILOVER_DIR/membership-primary-run1.json" "$PRIMARY_NODE" "$EXPECTED_MEMBERSHIP" primary 0 local_only "primary pre-failover membership truth" || fail_phase failover-artifacts "primary pre-failover membership truth drifted" "$FAILOVER_DIR/membership-primary-run1.json"
m043_assert_membership_payload_json "$FAILOVER_DIR/membership-standby-run1.json" "$STANDBY_NODE" "$EXPECTED_MEMBERSHIP" standby 0 local_only "standby pre-failover membership truth" || fail_phase failover-artifacts "standby pre-failover membership truth drifted" "$FAILOVER_DIR/membership-standby-run1.json"
m043_assert_keyed_payload_json "$FAILOVER_DIR/submit-primary.json" "$REQUEST_KEY" "$ORIGINAL_ATTEMPT" submitted pending "$PRIMARY_NODE" "$PRIMARY_NODE" "$STANDBY_NODE" mirrored primary 0 healthy '' false true true '' '' "initial primary submit truth" || fail_phase failover-artifacts "initial primary submit truth drifted" "$FAILOVER_DIR/submit-primary.json"
m043_assert_keyed_payload_json "$FAILOVER_DIR/pending-primary.json" "$REQUEST_KEY" "$ORIGINAL_ATTEMPT" submitted pending "$PRIMARY_NODE" "$PRIMARY_NODE" "$STANDBY_NODE" mirrored primary 0 healthy '' false true true '' '' "pending primary truth before failover" || fail_phase failover-artifacts "pending primary truth before failover drifted" "$FAILOVER_DIR/pending-primary.json"
m043_assert_keyed_payload_json "$FAILOVER_DIR/pending-standby.json" "$REQUEST_KEY" "$ORIGINAL_ATTEMPT" submitted pending "$PRIMARY_NODE" "$PRIMARY_NODE" "$STANDBY_NODE" mirrored standby 0 healthy '' false true true '' '' "pending standby mirrored truth before failover" || fail_phase failover-artifacts "pending standby mirrored truth before failover drifted" "$FAILOVER_DIR/pending-standby.json"
m043_assert_membership_payload_json "$FAILOVER_DIR/degraded-membership-standby.json" "$STANDBY_NODE" "$STANDBY_NODE" standby 0 degraded "standby degraded membership after primary loss" || fail_phase failover-artifacts "standby degraded membership truth drifted" "$FAILOVER_DIR/degraded-membership-standby.json"
m043_assert_keyed_payload_json "$FAILOVER_DIR/pre-promote-standby-status.json" "$REQUEST_KEY" "$ORIGINAL_ATTEMPT" submitted pending "$PRIMARY_NODE" "$PRIMARY_NODE" "$STANDBY_NODE" mirrored standby 0 degraded '' false true true '' '' "pre-promotion degraded standby status" || fail_phase failover-artifacts "pre-promotion degraded standby status drifted" "$FAILOVER_DIR/pre-promote-standby-status.json"

m043_assert_membership_payload_json "$FAILOVER_DIR/promoted-membership-standby.json" "$STANDBY_NODE" "$STANDBY_NODE" primary 1 unavailable "promoted standby membership truth" || fail_phase failover-artifacts "promoted standby membership truth drifted" "$FAILOVER_DIR/promoted-membership-standby.json"
m043_assert_keyed_payload_json "$FAILOVER_DIR/promoted-owner-lost-status.json" "$REQUEST_KEY" "$ORIGINAL_ATTEMPT" submitted pending "$PRIMARY_NODE" "$PRIMARY_NODE" "$STANDBY_NODE" owner_lost primary 1 unavailable '' false true true '' '' "promoted owner-lost status truth" || fail_phase failover-artifacts "promoted owner-lost status truth drifted" "$FAILOVER_DIR/promoted-owner-lost-status.json"
m043_assert_keyed_payload_json "$FAILOVER_DIR/failover-retry.json" "$REQUEST_KEY" "$FAILOVER_ATTEMPT" submitted pending "$STANDBY_NODE" "$STANDBY_NODE" '' unassigned primary 1 local_only '' false true true '' '' "failover retry submit truth" || fail_phase failover-artifacts "failover retry submit truth drifted" "$FAILOVER_DIR/failover-retry.json"
m043_assert_keyed_payload_json "$FAILOVER_DIR/failover-pending-status.json" "$REQUEST_KEY" "$FAILOVER_ATTEMPT" submitted pending "$STANDBY_NODE" "$STANDBY_NODE" '' unassigned primary 1 local_only '' false true true '' '' "promoted authority pending truth" || fail_phase failover-artifacts "promoted authority pending truth drifted" "$FAILOVER_DIR/failover-pending-status.json"
m043_assert_keyed_payload_json "$FAILOVER_DIR/failover-completed-standby.json" "$REQUEST_KEY" "$FAILOVER_ATTEMPT" completed succeeded "$STANDBY_NODE" "$STANDBY_NODE" '' unassigned primary 1 local_only "$STANDBY_NODE" false true true '' '' "promoted authority completion truth" || fail_phase failover-artifacts "promoted authority completion truth drifted" "$FAILOVER_DIR/failover-completed-standby.json"

m043_assert_membership_payload_json "$FAILOVER_DIR/membership-primary-run2.json" "$PRIMARY_NODE" "$EXPECTED_MEMBERSHIP" standby 1 healthy "old primary fenced membership after rejoin" || fail_phase failover-artifacts "old primary fenced membership truth drifted" "$FAILOVER_DIR/membership-primary-run2.json"
m043_assert_membership_payload_json "$FAILOVER_DIR/membership-standby-run2.json" "$STANDBY_NODE" "$EXPECTED_MEMBERSHIP" primary 1 local_only "promoted standby membership after rejoin" || fail_phase failover-artifacts "promoted standby membership after rejoin drifted" "$FAILOVER_DIR/membership-standby-run2.json"
m043_assert_keyed_payload_json "$FAILOVER_DIR/post-rejoin-primary-status.json" "$REQUEST_KEY" "$FAILOVER_ATTEMPT" completed succeeded "$STANDBY_NODE" "$STANDBY_NODE" '' unassigned standby 1 healthy "$STANDBY_NODE" false true true '' '' "fenced old primary status after rejoin" || fail_phase failover-artifacts "fenced old primary status after rejoin drifted" "$FAILOVER_DIR/post-rejoin-primary-status.json"
m043_assert_keyed_payload_json "$FAILOVER_DIR/post-rejoin-standby-status.json" "$REQUEST_KEY" "$FAILOVER_ATTEMPT" completed succeeded "$STANDBY_NODE" "$STANDBY_NODE" '' unassigned primary 1 local_only "$STANDBY_NODE" false true true '' '' "promoted standby status after rejoin" || fail_phase failover-artifacts "promoted standby status after rejoin drifted" "$FAILOVER_DIR/post-rejoin-standby-status.json"
m043_assert_keyed_payload_json "$FAILOVER_DIR/stale-guard-primary.json" "$REQUEST_KEY" "$FAILOVER_ATTEMPT" completed succeeded "$STANDBY_NODE" "$STANDBY_NODE" '' unassigned standby 1 healthy "$STANDBY_NODE" false true true '' '' "stale-primary same-key guard truth" || fail_phase failover-artifacts "stale-primary same-key guard truth drifted" "$FAILOVER_DIR/stale-guard-primary.json"

assert_file_contains_regex failover-artifacts "$FAILOVER_DIR/promote-standby.json" '"ok": true' "promotion response must report ok=true" "$FAILOVER_DIR/promote-standby.json"
assert_file_contains_regex failover-artifacts "$FAILOVER_DIR/promote-standby.json" '"cluster_role": "primary"' "promotion response must report primary authority" "$FAILOVER_DIR/promote-standby.json"
assert_file_contains_regex failover-artifacts "$FAILOVER_DIR/promote-standby.json" '"promotion_epoch": 1' "promotion response must report epoch 1" "$FAILOVER_DIR/promote-standby.json"
assert_file_contains_regex failover-artifacts "$FAILOVER_DIR/promote-standby.json" '"replication_health": "unavailable"' "promotion response must report unavailable replication health" "$FAILOVER_DIR/promote-standby.json"

assert_file_contains_regex failover-artifacts "$FAILOVER_DIR/standby-run1.stdout.log" "\\[cluster-proof\\] continuity promote cluster_role=primary promotion_epoch=1 replication_health=unavailable" "standby log is missing promotion truth" "$FAILOVER_DIR/standby-run1.stdout.log"
assert_file_contains_regex failover-artifacts "$FAILOVER_DIR/standby-run1.stderr.log" "\\[mesh-rt continuity\\] transition=promote previous_role=standby previous_epoch=0 next_role=primary next_epoch=1" "standby log is missing runtime promote transition" "$FAILOVER_DIR/standby-run1.stderr.log"
assert_file_contains_literal failover-artifacts "$FAILOVER_DIR/standby-run1.stderr.log" "[mesh-rt continuity] transition=recovery_rollover request_key=${REQUEST_KEY} previous_attempt_id=${ORIGINAL_ATTEMPT} next_attempt_id=${FAILOVER_ATTEMPT}" "standby log is missing recovery-rollover truth" "$FAILOVER_DIR/standby-run1.stderr.log"
assert_file_contains_literal failover-artifacts "$FAILOVER_DIR/standby-run1.stdout.log" "[cluster-proof] work executed request_key=${REQUEST_KEY} attempt_id=${FAILOVER_ATTEMPT} execution=${STANDBY_NODE}" "standby log is missing promoted execution truth" "$FAILOVER_DIR/standby-run1.stdout.log"
assert_file_contains_regex failover-artifacts "$FAILOVER_DIR/primary-run2.stderr.log" "\\[mesh-rt continuity\\] transition=fenced_rejoin request_key=${REQUEST_KEY} attempt_id=${FAILOVER_ATTEMPT} previous_role=primary previous_epoch=0 next_role=standby next_epoch=1" "old-primary rejoin log is missing fenced_rejoin transition" "$FAILOVER_DIR/primary-run2.stderr.log"
assert_file_contains_literal failover-artifacts "$FAILOVER_DIR/primary-run2.stdout.log" "[cluster-proof] keyed status request_key=${REQUEST_KEY} attempt_id=${FAILOVER_ATTEMPT} phase=completed result=succeeded owner=${STANDBY_NODE} replica= source=${PRIMARY_NODE} replica_status=unassigned cluster_role=standby promotion_epoch=1" "old-primary rejoin log is missing deposed status truth" "$FAILOVER_DIR/primary-run2.stdout.log"
if grep -Fq "[mesh-rt continuity] transition=completed request_key=${REQUEST_KEY} attempt_id=${ORIGINAL_ATTEMPT}" "$FAILOVER_DIR/primary-run1.stderr.log"; then
  fail_phase failover-artifacts "old primary completed the pre-failover attempt after it was killed" "$FAILOVER_DIR/primary-run1.stderr.log"
fi
if grep -Fq "[cluster-proof] work executed request_key=${REQUEST_KEY} attempt_id=${FAILOVER_ATTEMPT} execution=${PRIMARY_NODE}" "$FAILOVER_DIR/primary-run2.stdout.log"; then
  fail_phase failover-artifacts "old primary resumed execution after fenced rejoin" "$FAILOVER_DIR/primary-run2.stdout.log"
fi
record_phase failover-artifacts passed

assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^runtime-continuity\tpassed$' "runtime continuity replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^cluster-proof-tests\tpassed$' "cluster-proof tests replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^build-cluster-proof\tpassed$' "cluster-proof build replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^s01-contract\tpassed$' "S01 prerequisite replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m042-rejoin\tpassed$' "M042 rejoin regression replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^m043-api\tpassed$' "M043 continuity API replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^failover-contract\tpassed$' "M043 destructive failover replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^failover-artifacts\tpassed$' "M043 failover artifact validation did not pass" "$ARTIFACT_DIR/full-contract.log"

echo "verify-m043-s02: ok"
