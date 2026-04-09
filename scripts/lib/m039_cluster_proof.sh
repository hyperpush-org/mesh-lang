#!/usr/bin/env bash

m039_print_log_excerpt() {
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

m039_record_phase() {
  printf '%s\t%s\n' "$1" "$2" >>"$PHASE_REPORT_PATH"
}

m039_fail_phase() {
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
    m039_print_log_excerpt "$log_path" >&2
  fi
  exit 1
}

m039_assert_file_exists() {
  local phase="$1"
  local path="$2"
  local description="$3"
  local log_path="${4:-}"
  if [[ ! -f "$path" ]]; then
    m039_fail_phase "$phase" "missing ${description}: ${path}" "$log_path" "$path"
  fi
}

m039_assert_file_contains_regex() {
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
    m039_fail_phase "$phase" "$description" "$ARTIFACT_DIR/${phase}.content-check.log" "$path"
  fi
}

m039_assert_test_filter_ran() {
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
    m039_fail_phase "$phase" "named test filter ran 0 tests or produced malformed output" "$ARTIFACT_DIR/${label}.test-count.log"
  fi
}

m039_run_command_with_timeout() {
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

m039_run_expect_success() {
  local phase="$1"
  local label="$2"
  local require_tests="$3"
  local timeout_secs="$4"
  shift 4
  local -a cmd=("$@")
  local log_path="$ARTIFACT_DIR/${label}.log"
  local command_text="${cmd[*]}"

  m039_record_phase "$phase" started
  printf '%s\n' "$phase" >"$CURRENT_PHASE_PATH"
  echo "==> ${command_text}"
  if ! m039_run_command_with_timeout "$timeout_secs" "$log_path" "${cmd[@]}"; then
    m039_record_phase "$phase" failed
    m039_fail_phase "$phase" "expected success within ${timeout_secs}s" "$log_path"
  fi
  if [[ "$require_tests" == "yes" ]]; then
    m039_assert_test_filter_ran "$phase" "$log_path" "$label"
  fi
  m039_record_phase "$phase" passed
}

m039_assert_membership_json() {
  local json_path="$1"
  local expected_self="$2"
  local expected_membership_csv="$3"
  local expected_peers_csv="$4"
  local description="$5"

  python3 - "$json_path" "$expected_self" "$expected_membership_csv" "$expected_peers_csv" "$description" <<'PY'
from pathlib import Path
import json
import sys

json_path = Path(sys.argv[1])
expected_self = sys.argv[2]
expected_membership = [value for value in sys.argv[3].split(',') if value]
expected_peers = [value for value in sys.argv[4].split(',') if value]
description = sys.argv[5]

try:
    data = json.loads(json_path.read_text(errors='replace'))
except json.JSONDecodeError as error:
    raise SystemExit(f"{description}: malformed JSON in {json_path}: {error}") from error

if not isinstance(data, dict):
    raise SystemExit(f"{description}: expected object body in {json_path}, found {type(data).__name__}")

required_keys = {
    'self': str,
    'peers': list,
    'membership': list,
}
for key, expected_type in required_keys.items():
    if key not in data:
        raise SystemExit(f"{description}: missing key {key!r} in {json_path}")
    if not isinstance(data[key], expected_type):
        raise SystemExit(
            f"{description}: key {key!r} expected {expected_type.__name__}, found {type(data[key]).__name__}"
        )

if data['self'] != expected_self:
    raise SystemExit(f"{description}: self mismatch: expected {expected_self!r}, found {data['self']!r}")

for field in ('peers', 'membership'):
    values = data[field]
    if not all(isinstance(value, str) for value in values):
        raise SystemExit(f"{description}: key {field!r} must contain only strings")
    for value in values:
        if '@' not in value:
            raise SystemExit(f"{description}: malformed node identity {value!r} in {field!r}")

if sorted(data['membership']) != sorted(expected_membership):
    raise SystemExit(
        f"{description}: membership mismatch: expected {sorted(expected_membership)!r}, found {sorted(data['membership'])!r}"
    )
if sorted(data['peers']) != sorted(expected_peers):
    raise SystemExit(
        f"{description}: peers mismatch: expected {sorted(expected_peers)!r}, found {sorted(data['peers'])!r}"
    )

print(f"{description}: membership ok")
PY
}

m039_assert_work_json() {
  local json_path="$1"
  local mode="$2"
  local expected_ingress="$3"
  local expected_target="$4"
  local expected_execution="$5"
  local description="$6"

  python3 - "$json_path" "$mode" "$expected_ingress" "$expected_target" "$expected_execution" "$description" <<'PY'
from pathlib import Path
import json
import sys

json_path = Path(sys.argv[1])
mode = sys.argv[2]
expected_ingress = sys.argv[3]
expected_target = sys.argv[4]
expected_execution = sys.argv[5]
description = sys.argv[6]

try:
    data = json.loads(json_path.read_text(errors='replace'))
except json.JSONDecodeError as error:
    raise SystemExit(f"{description}: malformed JSON in {json_path}: {error}") from error

if not isinstance(data, dict):
    raise SystemExit(f"{description}: expected object body in {json_path}, found {type(data).__name__}")

required_keys = {
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
for key, expected_type in required_keys.items():
    if key not in data:
        raise SystemExit(f"{description}: missing key {key!r} in {json_path}")
    if not isinstance(data[key], expected_type):
        raise SystemExit(
            f"{description}: key {key!r} expected {expected_type.__name__}, found {type(data[key]).__name__}"
        )

request_id = data['request_id']
if not request_id.startswith('work-'):
    raise SystemExit(f"{description}: request_id must start with 'work-', found {request_id!r}")

if not data['ok']:
    raise SystemExit(f"{description}: expected ok=true, body={data!r}")
if data['timed_out']:
    raise SystemExit(f"{description}: expected timed_out=false, body={data!r}")
if data['error'] != '':
    raise SystemExit(f"{description}: expected empty error string, found {data['error']!r}")

if data['ingress_node'] != expected_ingress:
    raise SystemExit(
        f"{description}: ingress mismatch: expected {expected_ingress!r}, found {data['ingress_node']!r}"
    )
if data['target_node'] != expected_target:
    raise SystemExit(
        f"{description}: target mismatch: expected {expected_target!r}, found {data['target_node']!r}"
    )
if data['execution_node'] != expected_execution:
    raise SystemExit(
        f"{description}: execution mismatch: expected {expected_execution!r}, found {data['execution_node']!r}"
    )

if mode == 'remote':
    if not data['routed_remotely']:
        raise SystemExit(f"{description}: expected routed_remotely=true, body={data!r}")
    if data['fell_back_locally']:
        raise SystemExit(f"{description}: expected fell_back_locally=false, body={data!r}")
elif mode == 'local':
    if data['routed_remotely']:
        raise SystemExit(f"{description}: expected routed_remotely=false, body={data!r}")
    if not data['fell_back_locally']:
        raise SystemExit(f"{description}: expected fell_back_locally=true, body={data!r}")
else:
    raise SystemExit(f"unknown work mode: {mode}")

print(f"{description}: work payload ok")
PY
}
