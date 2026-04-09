#!/usr/bin/env bash

m043_http_json_request() {
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

m043_copy_artifact_dir() {
  local source_dir="$1"
  local dest_dir="$2"
  local manifest_path="$3"

  if ! python3 - "$source_dir" "$dest_dir" "$manifest_path" <<'PY'
from pathlib import Path
import shutil
import sys

source_dir = Path(sys.argv[1])
dest_dir = Path(sys.argv[2])
manifest_path = Path(sys.argv[3])

if not source_dir.is_dir():
    raise SystemExit(f"artifact source is missing: {source_dir}")
if dest_dir.exists():
    shutil.rmtree(dest_dir)
dest_dir.parent.mkdir(parents=True, exist_ok=True)
shutil.copytree(source_dir, dest_dir)
lines = [str(dest_dir)]
for path in sorted(dest_dir.rglob('*')):
    rel = path.relative_to(dest_dir)
    if path.is_file():
        lines.append(f"FILE\t{rel}\t{path.stat().st_size}")
    else:
        lines.append(f"DIR\t{rel}")
manifest_path.write_text("\n".join(lines) + "\n")
PY
  then
    return 1
  fi
}

m043_assert_membership_payload_json() {
  local json_path="$1"
  local expected_self="$2"
  local expected_membership="$3"
  local expected_role="$4"
  local expected_epoch="$5"
  local expected_health="$6"
  local description="$7"

  python3 - "$json_path" "$expected_self" "$expected_membership" "$expected_role" "$expected_epoch" "$expected_health" "$description" <<'PY'
from pathlib import Path
import json
import sys

json_path = Path(sys.argv[1])
expected_self = sys.argv[2]
expected_membership = [value for value in sys.argv[3].split('|') if value]
expected_role = sys.argv[4]
expected_epoch = int(sys.argv[5])
expected_health = sys.argv[6]
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
if data['replication_health'] != expected_health:
    raise SystemExit(
        f"{description}: replication_health mismatch: expected {expected_health!r}, found {data['replication_health']!r}"
    )
print(f"{description}: membership truth ok")
PY
}

m043_assert_keyed_payload_json() {
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
  local expected_health="${12}"
  local expected_execution="${13}"
  local expected_routed="${14}"
  local expected_fell_back="${15}"
  local expected_ok="${16}"
  local expected_error="${17}"
  local expected_conflict="${18}"
  local description="${19}"

  python3 - "$json_path" "$expected_request_key" "$expected_attempt_id" "$expected_phase" "$expected_result" "$expected_ingress" "$expected_owner" "$expected_replica" "$expected_replica_status" "$expected_cluster_role" "$expected_epoch" "$expected_health" "$expected_execution" "$expected_routed" "$expected_fell_back" "$expected_ok" "$expected_error" "$expected_conflict" "$description" <<'PY'
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
expected_health = sys.argv[12]
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
    'replication_health': expected_health,
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
print(f"{description}: keyed payload truth ok")
PY
}
