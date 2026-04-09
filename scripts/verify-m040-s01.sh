#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

source scripts/lib/clustered_fixture_paths.sh
clustered_fixture_require_cluster_proof_root

ARTIFACT_DIR="$ROOT_DIR/.tmp/m040-s01/verify"
if [[ -n "${M040_S01_HTTP_PORT:-}" ]]; then
  HTTP_PORT="$M040_S01_HTTP_PORT"
else
  HTTP_PORT="$(python3 - <<'PY'
import socket
sock = socket.socket()
sock.bind(('127.0.0.1', 0))
print(sock.getsockname()[1])
sock.close()
PY
)"
fi
REQUEST_KEY="m040-s01-verify-key"
SERVER_PID=""

rm -rf "$ARTIFACT_DIR"
mkdir -p "$ARTIFACT_DIR"

cleanup() {
  if [[ -n "$SERVER_PID" ]] && kill -0 "$SERVER_PID" 2>/dev/null; then
    kill "$SERVER_PID" 2>/dev/null || true
    wait "$SERVER_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT

record_phase() {
  printf '%s\n' "$1" >> "$ARTIFACT_DIR/phase-report.txt"
}

record_phase "build-cluster-proof\tstarted"
cargo run -q -p meshc -- build "$CLUSTER_PROOF_FIXTURE_ROOT" >"$ARTIFACT_DIR/build.log" 2>&1
record_phase "build-cluster-proof\tpassed"

(
  cd "$CLUSTER_PROOF_FIXTURE_ROOT"
  PORT="$HTTP_PORT" "$CLUSTER_PROOF_FIXTURE_BINARY"
) >"$ARTIFACT_DIR/cluster-proof.stdout.log" 2>"$ARTIFACT_DIR/cluster-proof.stderr.log" &
SERVER_PID="$!"

record_phase "wait-for-http\tstarted"
python3 - <<'PY' "$HTTP_PORT" "$ARTIFACT_DIR/membership.json"
import json, sys, time, urllib.request
port = int(sys.argv[1])
out_path = sys.argv[2]
url = f'http://127.0.0.1:{port}/membership'
last_error = None
for _ in range(100):
    try:
        with urllib.request.urlopen(url, timeout=2) as response:
            body = response.read().decode()
            data = json.loads(body)
            with open(out_path, 'w', encoding='utf-8') as fh:
                fh.write(json.dumps(data, indent=2, sort_keys=True))
            raise SystemExit(0)
    except Exception as exc:  # noqa: BLE001
        last_error = str(exc)
        time.sleep(0.1)
raise SystemExit(f'cluster-proof never became ready on {url}: {last_error}')
PY
record_phase "wait-for-http\tpassed"

record_phase "submit-status-contract\tstarted"
python3 - <<'PY' "$HTTP_PORT" "$REQUEST_KEY" "$ARTIFACT_DIR"
import json, sys, time, urllib.error, urllib.request

port = int(sys.argv[1])
request_key = sys.argv[2]
artifact_dir = sys.argv[3]
base = f'http://127.0.0.1:{port}'
expected_node = 'standalone@local'


def write_json(name, value):
    path = f'{artifact_dir}/{name}'
    with open(path, 'w', encoding='utf-8') as fh:
        fh.write(json.dumps(value, indent=2, sort_keys=True))


def get_json(path):
    with urllib.request.urlopen(base + path, timeout=3) as response:
        return response.status, json.loads(response.read().decode())


def post_json(path, body):
    req = urllib.request.Request(
        base + path,
        data=json.dumps(body).encode(),
        headers={'content-type': 'application/json'},
        method='POST',
    )
    try:
        with urllib.request.urlopen(req, timeout=3) as response:
            return response.status, json.loads(response.read().decode())
    except urllib.error.HTTPError as error:
        return error.code, json.loads(error.read().decode())

create_status, create = post_json('/work', {'request_key': request_key, 'payload': 'hello'})
assert create_status == 200, create
assert create['request_key'] == request_key, create
assert create['phase'] == 'submitted', create
assert create['result'] == 'pending', create
assert isinstance(create['attempt_id'], str) and create['attempt_id'].startswith('attempt-'), create
assert create['ingress_node'] == expected_node, create
assert create['owner_node'] == expected_node, create
assert create['replica_node'] == '', create
assert create['replica_status'] == 'unassigned', create
assert create['execution_node'] == '', create
assert create['routed_remotely'] is False, create
assert create['fell_back_locally'] is True, create
assert create['ok'] is True, create
write_json('01-create.json', create)

completed = None
for _ in range(100):
    status_code, status = get_json(f'/work/{request_key}')
    assert status_code == 200, status
    if status['phase'] == 'completed':
        completed = status
        break
    time.sleep(0.1)
assert completed is not None, status
assert completed['attempt_id'] == create['attempt_id'], completed
assert completed['result'] == 'succeeded', completed
assert completed['execution_node'] == expected_node, completed
write_json('02-completed.json', completed)

duplicate_status, duplicate = post_json('/work', {'request_key': request_key, 'payload': 'hello'})
assert duplicate_status == 200, duplicate
assert duplicate['attempt_id'] == create['attempt_id'], duplicate
assert duplicate['phase'] == 'completed', duplicate
assert duplicate['result'] == 'succeeded', duplicate
assert duplicate['conflict_reason'] == '', duplicate
assert duplicate['ok'] is True, duplicate
write_json('03-duplicate.json', duplicate)

conflict_status, conflict = post_json('/work', {'request_key': request_key, 'payload': 'different'})
assert conflict_status == 409, conflict
assert conflict['attempt_id'] == create['attempt_id'], conflict
assert conflict['phase'] == 'completed', conflict
assert conflict['result'] == 'succeeded', conflict
assert conflict['conflict_reason'] == 'request_key_conflict', conflict
assert conflict['ok'] is False, conflict
write_json('04-conflict.json', conflict)

try:
    with urllib.request.urlopen(base + '/work/missing-key', timeout=3) as response:
        missing_status = response.status
        missing = json.loads(response.read().decode())
except urllib.error.HTTPError as error:
    missing_status = error.code
    missing = json.loads(error.read().decode())
assert missing_status == 404, missing
assert missing['request_key'] == 'missing-key', missing
assert missing['phase'] == 'missing', missing
assert missing['result'] == 'unknown', missing
assert missing['error'] == 'request_key_not_found', missing
assert missing['ok'] is False, missing
write_json('05-missing.json', missing)

summary = {
    'request_key': request_key,
    'attempt_id': create['attempt_id'],
    'initial_phase': create['phase'],
    'final_phase': completed['phase'],
    'duplicate_phase': duplicate['phase'],
    'conflict_reason': conflict['conflict_reason'],
    'owner_node': completed['owner_node'],
    'execution_node': completed['execution_node'],
    'replica_status': completed['replica_status'],
}
write_json('summary.json', summary)
PY
record_phase "submit-status-contract\tpassed"

echo "M040/S01 verifier passed. Artifacts: $ARTIFACT_DIR" | tee "$ARTIFACT_DIR/result.txt"
