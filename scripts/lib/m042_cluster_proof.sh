#!/usr/bin/env bash

m042_http_json_request() {
  local phase="$1"
  local method="$2"
  local url="$3"
  local request_body="$4"
  local expected_status="$5"
  local raw_path="$6"
  local json_path="$7"
  local description="$8"
  local check_log="$ARTIFACT_DIR/${phase}.http-check.log"

  if ! python3 - "$method" "$url" "$request_body" "$expected_status" "$raw_path" "$json_path" "$description" >"$check_log" 2>&1 <<'PY'
from pathlib import Path
import json
import sys
import urllib.error
import urllib.request

method = sys.argv[1]
url = sys.argv[2]
request_body = sys.argv[3]
expected_status = int(sys.argv[4])
raw_path = Path(sys.argv[5])
json_path = Path(sys.argv[6])
description = sys.argv[7]

body_bytes = None if request_body == '-' else request_body.encode('utf-8')
request = urllib.request.Request(url, data=body_bytes, method=method)
if body_bytes is not None:
    request.add_header('Content-Type', 'application/json')

response = None
try:
    response = urllib.request.urlopen(request, timeout=5)
except urllib.error.HTTPError as error:
    response = error

status = response.getcode()
headers = response.headers.as_string()
body = response.read().decode('utf-8', errors='replace')
raw_path.write_text(f"HTTP/1.1 {status}\n{headers}\n{body}")
if status != expected_status:
    raise SystemExit(
        f"{description}: expected HTTP {expected_status}, found {status}; raw response archived at {raw_path}"
    )
try:
    parsed = json.loads(body)
except json.JSONDecodeError as error:
    raise SystemExit(
        f"{description}: malformed JSON response: {error}; raw response archived at {raw_path}"
    ) from error
json_path.write_text(json.dumps(parsed, indent=2) + "\n")
print(json.dumps({"status": status, "url": url, "description": description}, indent=2))
PY
  then
    return 1
  fi
}

m042_assert_keyed_payload_json() {
  local json_path="$1"
  local expected_request_key="$2"
  local expected_attempt_id="$3"
  local expected_phase="$4"
  local expected_result="$5"
  local expected_ingress="$6"
  local expected_owner="$7"
  local expected_replica="$8"
  local expected_replica_status="$9"
  local expected_execution="${10}"
  local expected_routed="${11}"
  local expected_fell_back="${12}"
  local expected_ok="${13}"
  local expected_error="${14}"
  local expected_conflict="${15}"
  local description="${16}"

  python3 - "$json_path" "$expected_request_key" "$expected_attempt_id" "$expected_phase" "$expected_result" "$expected_ingress" "$expected_owner" "$expected_replica" "$expected_replica_status" "$expected_execution" "$expected_routed" "$expected_fell_back" "$expected_ok" "$expected_error" "$expected_conflict" "$description" <<'PY'
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
expected_execution = sys.argv[10]
expected_routed = sys.argv[11] == 'true'
expected_fell_back = sys.argv[12] == 'true'
expected_ok = sys.argv[13] == 'true'
expected_error = sys.argv[14]
expected_conflict = sys.argv[15]
description = sys.argv[16]

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
if expected_attempt_id not in {'__format_only__', '__any__'} and attempt_id != expected_attempt_id:
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
    'execution_node': expected_execution,
    'error': expected_error,
    'conflict_reason': expected_conflict,
}
for key, expected_value in checks.items():
    if data[key] != expected_value:
        raise SystemExit(
            f"{description}: {key} mismatch: expected {expected_value!r}, found {data[key]!r}"
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

print(f"{description}: keyed payload ok")
PY
}

m042_find_remote_submit() {
  local phase="$1"
  local submit_url="$2"
  local request_prefix="$3"
  local expected_ingress="$4"
  local expected_owner="$5"
  local expected_replica="$6"
  local expected_replica_status="$7"
  local search_dir="$8"
  local raw_path="$9"
  local json_path="${10}"
  local meta_path="${11}"
  local max_attempts="${12}"
  local check_log="$ARTIFACT_DIR/${phase}.submit-search.log"

  mkdir -p "$search_dir"
  if ! python3 - "$submit_url" "$request_prefix" "$expected_ingress" "$expected_owner" "$expected_replica" "$expected_replica_status" "$search_dir" "$raw_path" "$json_path" "$meta_path" "$max_attempts" >"$check_log" 2>&1 <<'PY'
from pathlib import Path
import json
import shutil
import sys
import urllib.error
import urllib.request

submit_url = sys.argv[1]
request_prefix = sys.argv[2]
expected_ingress = sys.argv[3]
expected_owner = sys.argv[4]
expected_replica = sys.argv[5]
expected_replica_status = sys.argv[6]
search_dir = Path(sys.argv[7])
raw_path = Path(sys.argv[8])
json_path = Path(sys.argv[9])
meta_path = Path(sys.argv[10])
max_attempts = int(sys.argv[11])

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
    'execution_node': str,
    'routed_remotely': bool,
    'fell_back_locally': bool,
    'error': str,
    'conflict_reason': str,
}

for index in range(max_attempts):
    request_key = f"{request_prefix}-{index}"
    payload = f"payload-{index}"
    body = json.dumps({'request_key': request_key, 'payload': payload}).encode('utf-8')
    request = urllib.request.Request(submit_url, data=body, method='POST')
    request.add_header('Content-Type', 'application/json')

    response = None
    try:
        response = urllib.request.urlopen(request, timeout=5)
    except urllib.error.HTTPError as error:
        response = error

    status = response.getcode()
    headers = response.headers.as_string()
    response_body = response.read().decode('utf-8', errors='replace')
    candidate_raw = search_dir / f"candidate-{index}.http"
    candidate_json = search_dir / f"candidate-{index}.json"
    candidate_raw.write_text(f"HTTP/1.1 {status}\n{headers}\n{response_body}")
    if status != 200:
        raise SystemExit(
            f"candidate {index}: expected HTTP 200 from packaged submit, found {status}; see {candidate_raw}"
        )
    try:
        parsed = json.loads(response_body)
    except json.JSONDecodeError as error:
        raise SystemExit(f"candidate {index}: malformed JSON: {error}; see {candidate_raw}") from error
    candidate_json.write_text(json.dumps(parsed, indent=2) + '\n')
    if not isinstance(parsed, dict):
        raise SystemExit(f"candidate {index}: expected JSON object, found {type(parsed).__name__}")
    for key, expected_type in required.items():
        if key not in parsed:
            raise SystemExit(f"candidate {index}: missing key {key!r}")
        if not isinstance(parsed[key], expected_type):
            raise SystemExit(
                f"candidate {index}: key {key!r} expected {expected_type.__name__}, found {type(parsed[key]).__name__}"
            )

    if (
        parsed['request_key'] == request_key
        and parsed['ok'] is True
        and parsed['phase'] == 'submitted'
        and parsed['result'] == 'pending'
        and parsed['ingress_node'] == expected_ingress
        and parsed['owner_node'] == expected_owner
        and parsed['replica_node'] == expected_replica
        and parsed['replica_status'] == expected_replica_status
        and parsed['execution_node'] == ''
        and parsed['routed_remotely'] is True
        and parsed['fell_back_locally'] is False
        and parsed['error'] == ''
        and parsed['conflict_reason'] == ''
    ):
        shutil.copyfile(candidate_raw, raw_path)
        shutil.copyfile(candidate_json, json_path)
        meta_path.write_text(
            json.dumps(
                {
                    'request_key': request_key,
                    'payload': payload,
                    'attempt_id': parsed['attempt_id'],
                    'status_code': status,
                    'candidate_index': index,
                },
                indent=2,
            )
            + '\n'
        )
        print(json.dumps({'request_key': request_key, 'attempt_id': parsed['attempt_id'], 'candidate_index': index}, indent=2))
        raise SystemExit(0)

raise SystemExit(
    f"no packaged submit matched ingress={expected_ingress!r} owner={expected_owner!r} replica={expected_replica!r} after {max_attempts} candidates; search_dir={search_dir}"
)
PY
  then
    return 1
  fi
}

m042_wait_for_keyed_status() {
  local phase="$1"
  local status_url="$2"
  local expected_attempt_id="$3"
  local expected_phase="$4"
  local expected_result="$5"
  local expected_ingress="$6"
  local expected_owner="$7"
  local expected_replica="$8"
  local expected_replica_status="$9"
  local expected_execution="${10}"
  local expected_routed="${11}"
  local expected_fell_back="${12}"
  local expected_ok="${13}"
  local expected_error="${14}"
  local expected_conflict="${15}"
  local raw_path="${16}"
  local json_path="${17}"
  local timeout_secs="${18}"
  local description="${19}"
  local check_log="$ARTIFACT_DIR/${phase}.wait.log"

  if ! python3 - "$status_url" "$expected_attempt_id" "$expected_phase" "$expected_result" "$expected_ingress" "$expected_owner" "$expected_replica" "$expected_replica_status" "$expected_execution" "$expected_routed" "$expected_fell_back" "$expected_ok" "$expected_error" "$expected_conflict" "$raw_path" "$json_path" "$timeout_secs" "$description" >"$check_log" 2>&1 <<'PY'
from pathlib import Path
import json
import sys
import time
import urllib.error
import urllib.request

status_url = sys.argv[1]
expected_attempt_id = sys.argv[2]
expected_phase = sys.argv[3]
expected_result = sys.argv[4]
expected_ingress = sys.argv[5]
expected_owner = sys.argv[6]
expected_replica = sys.argv[7]
expected_replica_status = sys.argv[8]
expected_execution = sys.argv[9]
expected_routed = sys.argv[10] == 'true'
expected_fell_back = sys.argv[11] == 'true'
expected_ok = sys.argv[12] == 'true'
expected_error = sys.argv[13]
expected_conflict = sys.argv[14]
raw_path = Path(sys.argv[15])
json_path = Path(sys.argv[16])
timeout_secs = int(sys.argv[17])
description = sys.argv[18]

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
    'execution_node': str,
    'routed_remotely': bool,
    'fell_back_locally': bool,
    'error': str,
    'conflict_reason': str,
}

def matches(parsed):
    if not isinstance(parsed, dict):
        return False
    for key, expected_type in required.items():
        if key not in parsed or not isinstance(parsed[key], expected_type):
            return False
    return (
        parsed['attempt_id'] == expected_attempt_id
        and parsed['phase'] == expected_phase
        and parsed['result'] == expected_result
        and parsed['ingress_node'] == expected_ingress
        and parsed['owner_node'] == expected_owner
        and parsed['replica_node'] == expected_replica
        and parsed['replica_status'] == expected_replica_status
        and parsed['execution_node'] == expected_execution
        and parsed['routed_remotely'] is expected_routed
        and parsed['fell_back_locally'] is expected_fell_back
        and parsed['ok'] is expected_ok
        and parsed['error'] == expected_error
        and parsed['conflict_reason'] == expected_conflict
    )

last_raw = ''
start = time.time()
while time.time() - start < timeout_secs:
    response = None
    try:
        response = urllib.request.urlopen(status_url, timeout=5)
    except urllib.error.HTTPError as error:
        response = error

    status = response.getcode()
    headers = response.headers.as_string()
    body = response.read().decode('utf-8', errors='replace')
    last_raw = f"HTTP/1.1 {status}\n{headers}\n{body}"
    raw_path.write_text(last_raw)

    if status == 200:
        try:
            parsed = json.loads(body)
        except json.JSONDecodeError:
            parsed = None
        if parsed is not None:
            json_path.write_text(json.dumps(parsed, indent=2) + '\n')
            if matches(parsed):
                print(json.dumps({'status_url': status_url, 'description': description}, indent=2))
                raise SystemExit(0)
    time.sleep(0.25)

raw_path.write_text(last_raw)
raise SystemExit(
    f"{description}: timed out after {timeout_secs}s waiting for expected keyed status; raw response archived at {raw_path}"
)
PY
  then
    return 1
  fi
}
