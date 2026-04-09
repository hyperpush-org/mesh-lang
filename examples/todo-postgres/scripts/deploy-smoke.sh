#!/usr/bin/env bash
set -euo pipefail

DEFAULT_PORT="8080"
PORT_VALUE="${PORT:-$DEFAULT_PORT}"
BASE_URL="${BASE_URL:-http://127.0.0.1:${PORT_VALUE}}"
LAST_HEALTH_RESPONSE=""
LAST_RESPONSE=""

usage() {
  echo "usage: bash deploy-smoke.sh" >&2
}

fail() {
  echo "[deploy-smoke] $1" >&2
  exit 1
}

require_command() {
  local command_name="$1"
  if ! command -v "$command_name" >/dev/null 2>&1; then
    fail "required command missing from PATH: $command_name"
  fi
}

json_field() {
  local field="$1"
  python3 -c '
import json
import sys

field = sys.argv[1]
data = json.load(sys.stdin)
value = data
for key in field.split("."):
    if not isinstance(value, dict):
        sys.exit(1)
    value = value.get(key)
    if value is None:
        sys.exit(1)
if isinstance(value, bool):
    print("true" if value else "false")
elif isinstance(value, (dict, list)):
    print(json.dumps(value, separators=(",", ":")))
else:
    print(value)
' "$field"
}

list_length() {
  python3 -c '
import json
import sys

data = json.load(sys.stdin)
if not isinstance(data, list):
    sys.exit(1)
print(len(data))
'
}

list_contains_id() {
  local expected_id="$1"
  python3 -c '
import json
import sys

expected_id = sys.argv[1]
data = json.load(sys.stdin)
if not isinstance(data, list):
    sys.exit(1)
print("true" if any(isinstance(item, dict) and item.get("id") == expected_id for item in data) else "false")
' "$expected_id"
}

if [[ $# -ne 0 ]]; then
  usage
  exit 1
fi

for required_command in curl python3; do
  require_command "$required_command"
done

if [[ ! "$PORT_VALUE" =~ ^[1-9][0-9]*$ ]]; then
  fail "PORT must be a positive integer, got: $PORT_VALUE"
fi

case "$BASE_URL" in
  http://*|https://*) ;;
  *) fail "BASE_URL must start with http:// or https://, got: $BASE_URL" ;;
esac

printf '[deploy-smoke] waiting for health base_url=%s\n' "$BASE_URL"
for attempt in $(seq 1 80); do
  if health_response="$(curl -fsS "$BASE_URL/health" 2>/dev/null)"; then
    LAST_HEALTH_RESPONSE="$health_response"
    health_status="$(printf '%s' "$health_response" | json_field status || true)"
    db_backend="$(printf '%s' "$health_response" | json_field db_backend || true)"
    clustered_handler="$(printf '%s' "$health_response" | json_field clustered_handler || true)"
    printf '[deploy-smoke] health poll=%s status=%s db_backend=%s clustered_handler=%s\n' \
      "$attempt" "${health_status:-missing}" "${db_backend:-missing}" "${clustered_handler:-missing}"
    if [[ "$health_status" == "ok" && "$db_backend" == "postgres" && "$clustered_handler" == "Work.sync_todos" ]]; then
      printf '[deploy-smoke] health ready body=%s\n' "$health_response"
      break
    fi
  fi
  sleep 0.25
  if [[ "$attempt" == "80" ]]; then
    fail "/health never became ready at $BASE_URL; last_body=${LAST_HEALTH_RESPONSE:-unavailable}"
  fi
done

payload='{"title":"deploy smoke todo"}'
printf '[deploy-smoke] creating todo via POST %s/todos\n' "$BASE_URL"
create_response="$(curl -fsS -X POST "$BASE_URL/todos" -H 'content-type: application/json' -d "$payload")"
printf '[deploy-smoke] created todo body=%s\n' "$create_response"
TODO_ID="$(printf '%s' "$create_response" | json_field id || true)"
TITLE="$(printf '%s' "$create_response" | json_field title || true)"
if [[ -z "$TODO_ID" ]]; then
  fail "created todo response did not include id"
fi
if [[ "$TITLE" != "deploy smoke todo" ]]; then
  fail "created todo response title drifted: $create_response"
fi

printf '[deploy-smoke] fetching todo id=%s\n' "$TODO_ID"
get_response="$(curl -fsS "$BASE_URL/todos/$TODO_ID")"
get_title="$(printf '%s' "$get_response" | json_field title || true)"
if [[ "$get_title" != "deploy smoke todo" ]]; then
  fail "GET /todos/$TODO_ID returned unexpected body: $get_response"
fi

printf '[deploy-smoke] toggling todo id=%s\n' "$TODO_ID"
toggle_response="$(curl -fsS -X PUT "$BASE_URL/todos/$TODO_ID")"
completed="$(printf '%s' "$toggle_response" | json_field completed || true)"
if [[ "$completed" != "true" ]]; then
  fail "toggle response did not mark the todo completed: $toggle_response"
fi

printf '[deploy-smoke] listing todos\n'
list_response="$(curl -fsS "$BASE_URL/todos")"
list_count="$(printf '%s' "$list_response" | list_length || true)"
list_has_id="$(printf '%s' "$list_response" | list_contains_id "$TODO_ID" || true)"
if [[ -z "$list_count" || "$list_count" == "0" ]]; then
  fail "expected GET /todos to return at least one todo, got: $list_response"
fi
if [[ "$list_has_id" != "true" ]]; then
  fail "expected GET /todos to include id=$TODO_ID, got: $list_response"
fi

printf '[deploy-smoke] deleting todo id=%s\n' "$TODO_ID"
delete_response="$(curl -fsS -X DELETE "$BASE_URL/todos/$TODO_ID")"
deleted_id="$(printf '%s' "$delete_response" | json_field id || true)"
if [[ "$deleted_id" != "$TODO_ID" ]]; then
  fail "delete response did not report the created todo id: $delete_response"
fi

final_list="$(curl -fsS "$BASE_URL/todos")"
final_has_id="$(printf '%s' "$final_list" | list_contains_id "$TODO_ID" || true)"
if [[ "$final_has_id" != "false" ]]; then
  fail "expected deleted todo id=$TODO_ID to disappear from GET /todos, got: $final_list"
fi

printf '[deploy-smoke] CRUD smoke passed id=%s\n' "$TODO_ID"
printf '%s\n' "$final_list"
