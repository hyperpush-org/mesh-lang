#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

source scripts/lib/clustered_fixture_paths.sh
clustered_fixture_require_cluster_proof_root
source scripts/lib/m043_cluster_proof.sh

ARTIFACT_ROOT=".tmp/m043-s03"
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
  if [[ -n "${MISCONFIG_ENV_PATH:-}" && -f "$MISCONFIG_ENV_PATH" ]]; then
    rm -f "$MISCONFIG_ENV_PATH"
  fi
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

assert_membership_payload_allowed_healths() {
  local json_path="$1"
  local expected_self="$2"
  local expected_membership="$3"
  local expected_role="$4"
  local expected_epoch="$5"
  local allowed_healths="$6"
  local description="$7"

  python3 - "$json_path" "$expected_self" "$expected_membership" "$expected_role" "$expected_epoch" "$allowed_healths" "$description" <<'PY'
from pathlib import Path
import json
import sys

json_path = Path(sys.argv[1])
expected_self = sys.argv[2]
expected_membership = [value for value in sys.argv[3].split('|') if value]
expected_role = sys.argv[4]
expected_epoch = int(sys.argv[5])
allowed_healths = [value for value in sys.argv[6].split('|') if value]
description = sys.argv[7]

required = {
    'mode': str,
    'self': str,
    'membership': list,
    'cluster_role': str,
    'promotion_epoch': int,
    'replication_health': str,
    'discovery_provider': str,
    'discovery_seed': str,
}

try:
    data = json.loads(json_path.read_text(errors='replace'))
except json.JSONDecodeError as error:
    raise SystemExit(f"{description}: malformed JSON in {json_path}: {error}") from error

if not isinstance(data, dict):
    raise SystemExit(f"{description}: expected object body, found {type(data).__name__}")
for key, expected_type in required.items():
    if key not in data:
        raise SystemExit(f"{description}: missing key {key!r} in {json_path}")
    if not isinstance(data[key], expected_type):
        raise SystemExit(
            f"{description}: key {key!r} expected {expected_type.__name__}, found {type(data[key]).__name__}"
        )

membership = data['membership']
if any(not isinstance(value, str) for value in membership):
    raise SystemExit(f"{description}: membership entries must be strings")
if data['mode'] != 'cluster':
    raise SystemExit(f"{description}: mode mismatch: expected 'cluster', found {data['mode']!r}")
if data['self'] != expected_self:
    raise SystemExit(f"{description}: self mismatch: expected {expected_self!r}, found {data['self']!r}")
if sorted(membership) != sorted(expected_membership):
    raise SystemExit(
        f"{description}: membership mismatch: expected {sorted(expected_membership)!r}, found {sorted(membership)!r}"
    )
if data['cluster_role'] != expected_role:
    raise SystemExit(
        f"{description}: cluster_role mismatch: expected {expected_role!r}, found {data['cluster_role']!r}"
    )
if data['promotion_epoch'] != expected_epoch:
    raise SystemExit(
        f"{description}: promotion_epoch mismatch: expected {expected_epoch!r}, found {data['promotion_epoch']!r}"
    )
if data['replication_health'] not in allowed_healths:
    raise SystemExit(
        f"{description}: replication_health mismatch: expected one of {allowed_healths!r}, found {data['replication_health']!r}"
    )
print(f"{description}: membership truth ok")
PY
}

assert_keyed_payload_allowed_healths() {
  local json_path="$1"
  local expected_request_key="$2"
  local expected_attempt_id="$3"
  local expected_phase="$4"
  local expected_result="$5"
  local expected_ingress="$6"
  local expected_owner="$7"
  local expected_replica="$8"
  local expected_replica_status="$9"
  local expected_cluster_role="${10}"
  local expected_epoch="${11}"
  local allowed_healths="${12}"
  local expected_execution="${13}"
  local expected_routed="${14}"
  local expected_fell_back="${15}"
  local expected_ok="${16}"
  local expected_error="${17}"
  local expected_conflict="${18}"
  local description="${19}"

  python3 - "$json_path" "$expected_request_key" "$expected_attempt_id" "$expected_phase" "$expected_result" "$expected_ingress" "$expected_owner" "$expected_replica" "$expected_replica_status" "$expected_cluster_role" "$expected_epoch" "$allowed_healths" "$expected_execution" "$expected_routed" "$expected_fell_back" "$expected_ok" "$expected_error" "$expected_conflict" "$description" <<'PY'
from pathlib import Path
import json
import re
import sys

json_path = Path(sys.argv[1])
expected_request_key = sys.argv[2]
expected_attempt_id = sys.argv[3]
expected_phase = sys.argv[4]
expected_result = sys.argv[5]
expected_ingress = sys.argv[6]
expected_owner = sys.argv[7]
expected_replica = sys.argv[8]
expected_replica_status = sys.argv[9]
expected_cluster_role = sys.argv[10]
expected_epoch = int(sys.argv[11])
allowed_healths = [value for value in sys.argv[12].split('|') if value]
expected_execution = sys.argv[13]
expected_routed = sys.argv[14] == 'true'
expected_fell_back = sys.argv[15] == 'true'
expected_ok = sys.argv[16] == 'true'
expected_error = sys.argv[17]
expected_conflict = sys.argv[18]
description = sys.argv[19]

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

try:
    data = json.loads(json_path.read_text(errors='replace'))
except json.JSONDecodeError as error:
    raise SystemExit(f"{description}: malformed JSON in {json_path}: {error}") from error

if not isinstance(data, dict):
    raise SystemExit(f"{description}: expected object body, found {type(data).__name__}")
for key, expected_type in required.items():
    if key not in data:
        raise SystemExit(f"{description}: missing key {key!r} in {json_path}")
    if not isinstance(data[key], expected_type):
        raise SystemExit(
            f"{description}: key {key!r} expected {expected_type.__name__}, found {type(data[key]).__name__}"
        )

attempt_id = data['attempt_id']
if not re.fullmatch(r'attempt-\d+', attempt_id):
    raise SystemExit(f"{description}: attempt_id must match attempt-<int>, found {attempt_id!r}")
if expected_attempt_id != '__any__' and attempt_id != expected_attempt_id:
    raise SystemExit(
        f"{description}: attempt_id mismatch: expected {expected_attempt_id!r}, found {attempt_id!r}"
    )

checks = {
    'request_key': expected_request_key,
    'phase': expected_phase,
    'result': expected_result,
    'ingress_node': expected_ingress,
    'owner_node': expected_owner,
    'replica_node': expected_replica,
    'replica_status': expected_replica_status,
    'cluster_role': expected_cluster_role,
    'promotion_epoch': expected_epoch,
    'execution_node': expected_execution,
    'error': expected_error,
    'conflict_reason': expected_conflict,
}
for key, expected_value in checks.items():
    if data[key] != expected_value:
        raise SystemExit(
            f"{description}: {key} mismatch: expected {expected_value!r}, found {data[key]!r}"
        )
if data['replication_health'] not in allowed_healths:
    raise SystemExit(
        f"{description}: replication_health mismatch: expected one of {allowed_healths!r}, found {data['replication_health']!r}"
    )
if data['routed_remotely'] != expected_routed:
    raise SystemExit(
        f"{description}: routed_remotely mismatch: expected {expected_routed!r}, found {data['routed_remotely']!r}"
    )
if data['fell_back_locally'] != expected_fell_back:
    raise SystemExit(
        f"{description}: fell_back_locally mismatch: expected {expected_fell_back!r}, found {data['fell_back_locally']!r}"
    )
if data['ok'] != expected_ok:
    raise SystemExit(
        f"{description}: ok mismatch: expected {expected_ok!r}, found {data['ok']!r}"
    )
print(f"{description}: keyed payload truth ok")
PY
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
root = Path('.tmp/m043-s03')
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

select_same_image_artifacts() {
  local phase="$1"
  local before_snapshot="$2"
  local selection_path="$3"

  if ! python3 - "$before_snapshot" >"$selection_path" 2>"$ARTIFACT_DIR/${phase}.artifact-check.log" <<'PY'
from pathlib import Path
import sys

before_snapshot = Path(sys.argv[1])
source_root = Path('.tmp/m043-s03')

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
prefix = 'continuity-api-same-image-failover-'
candidates = []
for name in new_names:
    src = after_paths[name]
    if not name.startswith(prefix):
        continue
    if not (src / 'scenario-meta.json').is_file():
        continue
    candidates.append((name, src))
if len(candidates) != 1:
    raise SystemExit(
        f'expected exactly one new same-image failover artifact dir with scenario-meta.json, found {[name for name, _ in candidates]}; new_names={new_names}'
    )
name, src = candidates[0]
required = [
    'scenario-meta.json',
    'docker-build.log',
    'image.inspect.json',
    'network.inspect.json',
    'network.create.log',
    'network.rm.log',
    'primary-run1.create.log',
    'primary-run1.inspect.json',
    'primary-run1.post-kill.inspect.json',
    'primary-run1.kill.log',
    'primary-run1.remove.log',
    'primary-run1.stdout.log',
    'primary-run1.stderr.log',
    'standby-run1.create.log',
    'standby-run1.inspect.json',
    'standby-run1.stop.log',
    'standby-run1.remove.log',
    'standby-run1.stdout.log',
    'standby-run1.stderr.log',
    'primary-run2.create.log',
    'primary-run2.inspect.json',
    'primary-run2.stop.log',
    'primary-run2.remove.log',
    'primary-run2.stdout.log',
    'primary-run2.stderr.log',
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
    'same-image-failover-search/chosen.json',
    'same-image-failover-search/selected.http',
    'same-image-failover-search/selected.json',
]
missing = [rel for rel in required if not (src / rel).is_file()]
if missing:
    raise SystemExit(f'{src}: missing required same-image artifacts {missing}')
for rel in required:
    path = src / rel
    if path.stat().st_size <= 0:
        raise SystemExit(f'{path}: expected non-empty artifact file')
print(f'{name}\t{src}')
PY
  then
    fail_phase "$phase" "missing or malformed same-image failover evidence" "$ARTIFACT_DIR/${phase}.artifact-check.log"
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

run_expect_success s02-contract 03-s02-contract no 1200 \
  bash scripts/verify-m043-s02.sh
assert_file_exists s02-contract .tmp/m043-s02/verify/phase-report.txt "S02 phase report" "$ARTIFACT_DIR/03-s02-contract.log"
assert_file_exists s02-contract .tmp/m043-s02/verify/status.txt "S02 status" "$ARTIFACT_DIR/03-s02-contract.log"
assert_file_exists s02-contract .tmp/m043-s02/verify/current-phase.txt "S02 current phase" "$ARTIFACT_DIR/03-s02-contract.log"
cp .tmp/m043-s02/verify/phase-report.txt "$ARTIFACT_DIR/03-s02-phase-report.txt"
cp .tmp/m043-s02/verify/status.txt "$ARTIFACT_DIR/03-s02-status.txt"
cp .tmp/m043-s02/verify/current-phase.txt "$ARTIFACT_DIR/03-s02-current-phase.txt"
assert_file_contains_regex s02-contract "$ARTIFACT_DIR/03-s02-status.txt" '^ok$' "S02 verifier status must be ok" "$ARTIFACT_DIR/03-s02-contract.log"
assert_file_contains_regex s02-contract "$ARTIFACT_DIR/03-s02-current-phase.txt" '^complete$' "S02 verifier current phase must be complete" "$ARTIFACT_DIR/03-s02-contract.log"
assert_file_contains_regex s02-contract "$ARTIFACT_DIR/03-s02-phase-report.txt" '^failover-artifacts\tpassed$' "S02 failover artifact replay did not pass" "$ARTIFACT_DIR/03-s02-contract.log"

SAME_IMAGE_BEFORE="$ARTIFACT_DIR/04-same-image.before.txt"
capture_snapshot "$SAME_IMAGE_BEFORE"
run_expect_success same-image-contract 04-same-image-contract yes 1800 \
  cargo test -p meshc --test e2e_m043_s03 -- --nocapture
assert_file_contains_regex same-image-contract "$ARTIFACT_DIR/04-same-image-contract.log" 'test e2e_m043_s03_request_key_without_primary_owner_and_standby_replica_is_rejected \.\.\. ok' "same-image placement guard test did not pass" "$ARTIFACT_DIR/04-same-image-contract.log"
assert_file_contains_regex same-image-contract "$ARTIFACT_DIR/04-same-image-contract.log" 'test e2e_m043_s03_malformed_or_incomplete_http_responses_fail_closed \.\.\. ok' "same-image malformed-response guard test did not pass" "$ARTIFACT_DIR/04-same-image-contract.log"
assert_file_contains_regex same-image-contract "$ARTIFACT_DIR/04-same-image-contract.log" 'test e2e_m043_s03_same_image_failover_fences_stale_primary \.\.\. ok' "same-image destructive failover proof did not pass" "$ARTIFACT_DIR/04-same-image-contract.log"

MISCONFIG_ENV_PATH="$(mktemp "${TMPDIR%/}/m043-s03-invalid-continuity.XXXXXX")"
MISCONFIG_LOG_PATH="$ARTIFACT_DIR/04a-invalid-continuity.log"
cat >"$MISCONFIG_ENV_PATH" <<'EOF'
CLUSTER_PROOF_COOKIE=verify-cookie
MESH_DISCOVERY_SEED=cluster-proof-seed
MESH_CONTINUITY_ROLE=standby
MESH_CONTINUITY_PROMOTION_EPOCH=1
EOF
record_phase entrypoint-misconfig started
printf '%s\n' 'entrypoint-misconfig' >"$CURRENT_PHASE_PATH"
echo "==> docker run --rm --env-file $MISCONFIG_ENV_PATH mesh-cluster-proof:m043-s03-local"
misconfig_status=0
if run_command_with_timeout 120 "$MISCONFIG_LOG_PATH" \
  docker run --rm --env-file "$MISCONFIG_ENV_PATH" mesh-cluster-proof:m043-s03-local; then
  fail_phase entrypoint-misconfig "invalid continuity env unexpectedly booted the same-image container" "$MISCONFIG_LOG_PATH" "$MISCONFIG_LOG_PATH"
else
  misconfig_status=$?
fi
if [[ "$misconfig_status" -eq 124 ]]; then
  fail_phase entrypoint-misconfig "invalid continuity env timed out instead of failing immediately" "$MISCONFIG_LOG_PATH" "$MISCONFIG_LOG_PATH"
fi
assert_file_contains_literal entrypoint-misconfig "$MISCONFIG_LOG_PATH" "[cluster-proof] Config error: Invalid continuity topology: standby role requires promotion epoch 0 before promotion" "entrypoint misconfig probe did not surface the early continuity error" "$MISCONFIG_LOG_PATH"
if grep -Eq '\[cluster-proof\] (Runtime authority ready|HTTP server starting|Node started:)' "$MISCONFIG_LOG_PATH"; then
  fail_phase entrypoint-misconfig "invalid continuity env reached runtime startup before failing" "$MISCONFIG_LOG_PATH" "$MISCONFIG_LOG_PATH"
fi
if grep -Fq 'verify-cookie' "$MISCONFIG_LOG_PATH" || grep -Fq 'CLUSTER_PROOF_COOKIE=' "$MISCONFIG_LOG_PATH"; then
  fail_phase entrypoint-misconfig "misconfig probe leaked cookie material into retained logs" "$MISCONFIG_LOG_PATH" "$MISCONFIG_LOG_PATH"
fi
record_phase entrypoint-misconfig passed

SAME_IMAGE_SELECTION="$ARTIFACT_DIR/05-same-image.selection.txt"
record_phase same-image-artifacts started
select_same_image_artifacts same-image-artifacts "$SAME_IMAGE_BEFORE" "$SAME_IMAGE_SELECTION"
copy_selected_artifacts same-image-artifacts "$SAME_IMAGE_SELECTION" "$ARTIFACT_DIR/05-same-image-artifacts" "$ARTIFACT_DIR/05-same-image-artifacts.txt"

FAILOVER_DIR="$(find "$ARTIFACT_DIR/05-same-image-artifacts" -mindepth 1 -maxdepth 1 -type d | head -n 1)"
if [[ -z "$FAILOVER_DIR" ]]; then
  fail_phase same-image-artifacts "copied same-image artifact directory is missing" "$ARTIFACT_DIR/05-same-image-artifacts.txt"
fi
SCENARIO_META="$FAILOVER_DIR/scenario-meta.json"
assert_file_exists same-image-artifacts "$SCENARIO_META" "scenario metadata" "$ARTIFACT_DIR/05-same-image-artifacts.txt"

BUNDLE_VALUES_PATH="$ARTIFACT_DIR/05-same-image-bundle-values.txt"
BUNDLE_CHECK_LOG="$ARTIFACT_DIR/05-same-image-bundle-check.log"
if ! python3 - \
  "$SCENARIO_META" \
  "$FAILOVER_DIR/image.inspect.json" \
  "$FAILOVER_DIR/network.inspect.json" \
  "$FAILOVER_DIR/primary-run1.inspect.json" \
  "$FAILOVER_DIR/standby-run1.inspect.json" \
  "$FAILOVER_DIR/primary-run1.post-kill.inspect.json" \
  "$FAILOVER_DIR/primary-run2.inspect.json" \
  >"$BUNDLE_VALUES_PATH" 2>"$BUNDLE_CHECK_LOG" <<'PY'
from pathlib import Path
import json
import sys

scenario_path = Path(sys.argv[1])
image_path = Path(sys.argv[2])
network_path = Path(sys.argv[3])
primary_run1_path = Path(sys.argv[4])
standby_run1_path = Path(sys.argv[5])
primary_post_kill_path = Path(sys.argv[6])
primary_run2_path = Path(sys.argv[7])


def load_json(path: Path):
    try:
        return json.loads(path.read_text(errors='replace'))
    except json.JSONDecodeError as error:
        raise SystemExit(f"malformed JSON in {path}: {error}") from error


def require_nonempty_str(data, key, label):
    value = data.get(key)
    if not isinstance(value, str) or not value:
        raise SystemExit(f"{label}: missing non-empty string field {key!r}")
    return value


def require_object_list(path: Path, label: str):
    data = load_json(path)
    if not isinstance(data, list) or len(data) != 1 or not isinstance(data[0], dict):
        raise SystemExit(f"{label}: expected one-object JSON array in {path}")
    return data[0]


def validate_inspect(path: Path, expected_hostname: str, expected_role: str, expected_epoch: int, label: str):
    inspect = require_object_list(path, label)
    config = inspect.get('Config')
    if not isinstance(config, dict):
        raise SystemExit(f"{label}: missing Config object")
    if config.get('Hostname') != expected_hostname:
        raise SystemExit(f"{label}: hostname mismatch: expected {expected_hostname!r}, found {config.get('Hostname')!r}")
    env = config.get('Env')
    if not isinstance(env, list) or any(not isinstance(item, str) for item in env):
        raise SystemExit(f"{label}: Config.Env must be a list of strings")
    required_env = {
        f'MESH_CONTINUITY_ROLE={expected_role}',
        f'MESH_CONTINUITY_PROMOTION_EPOCH={expected_epoch}',
        'MESH_DISCOVERY_SEED=cluster-proof-seed',
        'CLUSTER_PROOF_DURABILITY=replica-backed',
        'CLUSTER_PROOF_COOKIE=[REDACTED]',
    }
    missing = sorted(required_env - set(env))
    if missing:
        raise SystemExit(f"{label}: missing expected env entries {missing!r}")
    return inspect

scenario = load_json(scenario_path)
if not isinstance(scenario, dict):
    raise SystemExit(f"scenario-meta: expected object body, found {type(scenario).__name__}")
for key in [
    'image_tag',
    'image_id',
    'network_name',
    'request_key',
    'payload',
    'original_attempt_id',
    'failover_attempt_id',
    'primary_node',
    'standby_node',
    'primary_run1_container',
    'standby_run1_container',
    'primary_run2_container',
    'primary_host_port_run1',
    'standby_host_port_run1',
    'primary_host_port_run2',
]:
    require_nonempty_str(scenario, key, 'scenario-meta')
if scenario['image_tag'] != 'mesh-cluster-proof:m043-s03-local':
    raise SystemExit(f"scenario-meta: image_tag drifted: {scenario['image_tag']!r}")
if scenario['original_attempt_id'] == scenario['failover_attempt_id']:
    raise SystemExit('scenario-meta: original_attempt_id and failover_attempt_id must differ')

image_inspect = require_object_list(image_path, 'image.inspect')
repo_tags = image_inspect.get('RepoTags')
if not isinstance(repo_tags, list) or scenario['image_tag'] not in repo_tags:
    raise SystemExit(f"image.inspect: missing repo tag {scenario['image_tag']!r}")
if image_inspect.get('Id') != scenario['image_id']:
    raise SystemExit('image.inspect: image id does not match scenario-meta')

network_inspect = require_object_list(network_path, 'network.inspect')
if network_inspect.get('Name') != scenario['network_name']:
    raise SystemExit('network.inspect: network name does not match scenario-meta')

validate_inspect(primary_run1_path, 'primary', 'primary', 0, 'primary-run1.inspect')
validate_inspect(standby_run1_path, 'standby', 'standby', 0, 'standby-run1.inspect')
validate_inspect(primary_post_kill_path, 'primary', 'primary', 0, 'primary-run1.post-kill.inspect')
validate_inspect(primary_run2_path, 'primary', 'primary', 0, 'primary-run2.inspect')

print('\t'.join([
    scenario['request_key'],
    scenario['original_attempt_id'],
    scenario['failover_attempt_id'],
    scenario['primary_node'],
    scenario['standby_node'],
    scenario['payload'],
]))
PY
then
  fail_phase same-image-artifacts "same-image bundle metadata is malformed or inconsistent" "$BUNDLE_CHECK_LOG" "$SCENARIO_META"
fi
read -r REQUEST_KEY ORIGINAL_ATTEMPT FAILOVER_ATTEMPT PRIMARY_NODE STANDBY_NODE PAYLOAD <"$BUNDLE_VALUES_PATH"
if [[ "$ORIGINAL_ATTEMPT" == "$FAILOVER_ATTEMPT" ]]; then
  fail_phase same-image-artifacts "same-image bundle reused the original attempt id after promotion" "$BUNDLE_VALUES_PATH" "$SCENARIO_META"
fi
EXPECTED_MEMBERSHIP="${PRIMARY_NODE}|${STANDBY_NODE}"

m043_assert_membership_payload_json "$FAILOVER_DIR/membership-primary-run1.json" "$PRIMARY_NODE" "$EXPECTED_MEMBERSHIP" primary 0 local_only "primary pre-failover membership truth" || fail_phase same-image-artifacts "primary pre-failover membership truth drifted" "$FAILOVER_DIR/membership-primary-run1.json"
m043_assert_membership_payload_json "$FAILOVER_DIR/membership-standby-run1.json" "$STANDBY_NODE" "$EXPECTED_MEMBERSHIP" standby 0 local_only "standby pre-failover membership truth" || fail_phase same-image-artifacts "standby pre-failover membership truth drifted" "$FAILOVER_DIR/membership-standby-run1.json"
m043_assert_keyed_payload_json "$FAILOVER_DIR/submit-primary.json" "$REQUEST_KEY" "$ORIGINAL_ATTEMPT" submitted pending "$PRIMARY_NODE" "$PRIMARY_NODE" "$STANDBY_NODE" mirrored primary 0 healthy '' false true true '' '' "initial primary submit truth" || fail_phase same-image-artifacts "initial primary submit truth drifted" "$FAILOVER_DIR/submit-primary.json"
m043_assert_keyed_payload_json "$FAILOVER_DIR/pending-primary.json" "$REQUEST_KEY" "$ORIGINAL_ATTEMPT" submitted pending "$PRIMARY_NODE" "$PRIMARY_NODE" "$STANDBY_NODE" mirrored primary 0 healthy '' false true true '' '' "pending primary truth before failover" || fail_phase same-image-artifacts "pending primary truth before failover drifted" "$FAILOVER_DIR/pending-primary.json"
m043_assert_keyed_payload_json "$FAILOVER_DIR/pending-standby.json" "$REQUEST_KEY" "$ORIGINAL_ATTEMPT" submitted pending "$PRIMARY_NODE" "$PRIMARY_NODE" "$STANDBY_NODE" mirrored standby 0 healthy '' false true true '' '' "pending standby mirrored truth before failover" || fail_phase same-image-artifacts "pending standby mirrored truth before failover drifted" "$FAILOVER_DIR/pending-standby.json"
m043_assert_membership_payload_json "$FAILOVER_DIR/degraded-membership-standby.json" "$STANDBY_NODE" "$STANDBY_NODE" standby 0 degraded "standby degraded membership after primary loss" || fail_phase same-image-artifacts "standby degraded membership truth drifted" "$FAILOVER_DIR/degraded-membership-standby.json"
m043_assert_keyed_payload_json "$FAILOVER_DIR/pre-promote-standby-status.json" "$REQUEST_KEY" "$ORIGINAL_ATTEMPT" submitted pending "$PRIMARY_NODE" "$PRIMARY_NODE" "$STANDBY_NODE" mirrored standby 0 degraded '' false true true '' '' "pre-promotion degraded standby status" || fail_phase same-image-artifacts "pre-promotion degraded standby status drifted" "$FAILOVER_DIR/pre-promote-standby-status.json"

m043_assert_membership_payload_json "$FAILOVER_DIR/promoted-membership-standby.json" "$STANDBY_NODE" "$STANDBY_NODE" primary 1 unavailable "promoted standby membership truth" || fail_phase same-image-artifacts "promoted standby membership truth drifted" "$FAILOVER_DIR/promoted-membership-standby.json"
m043_assert_keyed_payload_json "$FAILOVER_DIR/promoted-owner-lost-status.json" "$REQUEST_KEY" "$ORIGINAL_ATTEMPT" submitted pending "$PRIMARY_NODE" "$PRIMARY_NODE" "$STANDBY_NODE" owner_lost primary 1 unavailable '' false true true '' '' "promoted owner-lost status truth" || fail_phase same-image-artifacts "promoted owner-lost status truth drifted" "$FAILOVER_DIR/promoted-owner-lost-status.json"
m043_assert_keyed_payload_json "$FAILOVER_DIR/failover-retry.json" "$REQUEST_KEY" "$FAILOVER_ATTEMPT" submitted pending "$STANDBY_NODE" "$STANDBY_NODE" '' unassigned primary 1 local_only '' false true true '' '' "failover retry submit truth" || fail_phase same-image-artifacts "failover retry submit truth drifted" "$FAILOVER_DIR/failover-retry.json"
m043_assert_keyed_payload_json "$FAILOVER_DIR/failover-pending-status.json" "$REQUEST_KEY" "$FAILOVER_ATTEMPT" submitted pending "$STANDBY_NODE" "$STANDBY_NODE" '' unassigned primary 1 local_only '' false true true '' '' "promoted authority pending truth" || fail_phase same-image-artifacts "promoted authority pending truth drifted" "$FAILOVER_DIR/failover-pending-status.json"
m043_assert_keyed_payload_json "$FAILOVER_DIR/failover-completed-standby.json" "$REQUEST_KEY" "$FAILOVER_ATTEMPT" completed succeeded "$STANDBY_NODE" "$STANDBY_NODE" '' unassigned primary 1 local_only "$STANDBY_NODE" false true true '' '' "promoted authority completion truth" || fail_phase same-image-artifacts "promoted authority completion truth drifted" "$FAILOVER_DIR/failover-completed-standby.json"

m043_assert_membership_payload_json "$FAILOVER_DIR/membership-primary-run2.json" "$PRIMARY_NODE" "$EXPECTED_MEMBERSHIP" standby 1 healthy "old primary fenced membership after rejoin" || fail_phase same-image-artifacts "old primary fenced membership truth drifted" "$FAILOVER_DIR/membership-primary-run2.json"
assert_membership_payload_allowed_healths "$FAILOVER_DIR/membership-standby-run2.json" "$STANDBY_NODE" "$EXPECTED_MEMBERSHIP" primary 1 'local_only|healthy' "promoted standby membership after rejoin" || fail_phase same-image-artifacts "promoted standby membership after rejoin drifted" "$FAILOVER_DIR/membership-standby-run2.json"
m043_assert_keyed_payload_json "$FAILOVER_DIR/post-rejoin-primary-status.json" "$REQUEST_KEY" "$FAILOVER_ATTEMPT" completed succeeded "$STANDBY_NODE" "$STANDBY_NODE" '' unassigned standby 1 healthy "$STANDBY_NODE" false true true '' '' "fenced old primary status after rejoin" || fail_phase same-image-artifacts "fenced old primary status after rejoin drifted" "$FAILOVER_DIR/post-rejoin-primary-status.json"
assert_keyed_payload_allowed_healths "$FAILOVER_DIR/post-rejoin-standby-status.json" "$REQUEST_KEY" "$FAILOVER_ATTEMPT" completed succeeded "$STANDBY_NODE" "$STANDBY_NODE" '' unassigned primary 1 'local_only|healthy' "$STANDBY_NODE" false true true '' '' "promoted standby status after rejoin" || fail_phase same-image-artifacts "promoted standby status after rejoin drifted" "$FAILOVER_DIR/post-rejoin-standby-status.json"
m043_assert_keyed_payload_json "$FAILOVER_DIR/stale-guard-primary.json" "$REQUEST_KEY" "$FAILOVER_ATTEMPT" completed succeeded "$STANDBY_NODE" "$STANDBY_NODE" '' unassigned standby 1 healthy "$STANDBY_NODE" false true true '' '' "stale-primary same-key guard truth" || fail_phase same-image-artifacts "stale-primary same-key guard truth drifted" "$FAILOVER_DIR/stale-guard-primary.json"

assert_file_contains_regex same-image-artifacts "$FAILOVER_DIR/promote-standby.json" '"ok": true' "promotion response must report ok=true" "$FAILOVER_DIR/promote-standby.json"
assert_file_contains_regex same-image-artifacts "$FAILOVER_DIR/promote-standby.json" '"cluster_role": "primary"' "promotion response must report primary authority" "$FAILOVER_DIR/promote-standby.json"
assert_file_contains_regex same-image-artifacts "$FAILOVER_DIR/promote-standby.json" '"promotion_epoch": 1' "promotion response must report epoch 1" "$FAILOVER_DIR/promote-standby.json"
assert_file_contains_regex same-image-artifacts "$FAILOVER_DIR/promote-standby.json" '"replication_health": "unavailable"' "promotion response must report unavailable replication health" "$FAILOVER_DIR/promote-standby.json"

assert_file_contains_regex same-image-artifacts "$FAILOVER_DIR/standby-run1.stdout.log" "\\[cluster-proof\\] continuity promote cluster_role=primary promotion_epoch=1 replication_health=unavailable" "standby log is missing promotion truth" "$FAILOVER_DIR/standby-run1.stdout.log"
assert_file_contains_regex same-image-artifacts "$FAILOVER_DIR/standby-run1.stderr.log" "\\[mesh-rt continuity\\] transition=promote previous_role=standby previous_epoch=0 next_role=primary next_epoch=1" "standby log is missing runtime promote transition" "$FAILOVER_DIR/standby-run1.stderr.log"
assert_file_contains_literal same-image-artifacts "$FAILOVER_DIR/standby-run1.stderr.log" "[mesh-rt continuity] transition=recovery_rollover request_key=${REQUEST_KEY} previous_attempt_id=${ORIGINAL_ATTEMPT} next_attempt_id=${FAILOVER_ATTEMPT}" "standby log is missing recovery-rollover truth" "$FAILOVER_DIR/standby-run1.stderr.log"
assert_file_contains_literal same-image-artifacts "$FAILOVER_DIR/standby-run1.stdout.log" "[cluster-proof] work executed request_key=${REQUEST_KEY} attempt_id=${FAILOVER_ATTEMPT} execution=${STANDBY_NODE}" "standby log is missing promoted execution truth" "$FAILOVER_DIR/standby-run1.stdout.log"
assert_file_contains_regex same-image-artifacts "$FAILOVER_DIR/primary-run2.stderr.log" "\\[mesh-rt continuity\\] transition=fenced_rejoin request_key=${REQUEST_KEY} attempt_id=${FAILOVER_ATTEMPT} previous_role=primary previous_epoch=0 next_role=standby next_epoch=1" "old-primary rejoin log is missing fenced_rejoin transition" "$FAILOVER_DIR/primary-run2.stderr.log"
assert_file_contains_literal same-image-artifacts "$FAILOVER_DIR/primary-run2.stdout.log" "[cluster-proof] keyed status request_key=${REQUEST_KEY} attempt_id=${FAILOVER_ATTEMPT} phase=completed result=succeeded owner=${STANDBY_NODE} replica= source=${PRIMARY_NODE} replica_status=unassigned cluster_role=standby promotion_epoch=1" "old-primary rejoin log is missing deposed status truth" "$FAILOVER_DIR/primary-run2.stdout.log"
if grep -Fq "[mesh-rt continuity] transition=completed request_key=${REQUEST_KEY} attempt_id=${ORIGINAL_ATTEMPT}" "$FAILOVER_DIR/primary-run1.stderr.log"; then
  fail_phase same-image-artifacts "old primary completed the pre-failover attempt after it was killed" "$FAILOVER_DIR/primary-run1.stderr.log"
fi
if grep -Fq "[mesh-rt continuity] transition=completed request_key=${REQUEST_KEY} attempt_id=${FAILOVER_ATTEMPT}" "$FAILOVER_DIR/primary-run2.stderr.log"; then
  fail_phase same-image-artifacts "old primary logged completion for the promoted attempt after fenced rejoin" "$FAILOVER_DIR/primary-run2.stderr.log"
fi
if grep -Fq "[cluster-proof] work executed request_key=${REQUEST_KEY} attempt_id=${FAILOVER_ATTEMPT} execution=${PRIMARY_NODE}" "$FAILOVER_DIR/primary-run2.stdout.log"; then
  fail_phase same-image-artifacts "old primary resumed execution after fenced rejoin" "$FAILOVER_DIR/primary-run2.stdout.log"
fi
record_phase same-image-artifacts passed

assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^runtime-continuity\tpassed$' "runtime continuity replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^cluster-proof-tests\tpassed$' "cluster-proof tests replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^build-cluster-proof\tpassed$' "cluster-proof build replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^s02-contract\tpassed$' "S02 prerequisite replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^same-image-contract\tpassed$' "same-image Docker target did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^entrypoint-misconfig\tpassed$' "entrypoint misconfig probe did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^same-image-artifacts\tpassed$' "same-image artifact validation did not pass" "$ARTIFACT_DIR/full-contract.log"

echo "verify-m043-s03: ok"
