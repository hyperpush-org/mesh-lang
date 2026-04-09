#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

export M034_S01_LIB_ONLY=1
# shellcheck disable=SC1091
source scripts/verify-m034-s01.sh
unset M034_S01_LIB_ONLY

TEST_ROOT="$ROOT_DIR/.tmp/m034-s01/fetch-retry-test"
rm -rf "$TEST_ROOT"
mkdir -p "$TEST_ROOT"
RUN_DIR="$TEST_ROOT"
ROOT_DIR="$ROOT_DIR"

sleep() {
  :
}

write_attempt_artifacts() {
  local label="$1"
  local body_text="$2"
  local status_text="$3"
  local stderr_text="$4"

  printf '%s\n' "$body_text" >"$RUN_DIR/${label}.body"
  printf 'HTTP/1.1 %s\n' "${status_text:-000}" >"$RUN_DIR/${label}.headers"
  printf '%s' "$status_text" >"$RUN_DIR/${label}.status"
  printf '%s\n' "$stderr_text" >"$RUN_DIR/${label}.stderr"
  printf 'label=%s\nstatus=%s\nstderr=%s\n' "$label" "$status_text" "$stderr_text" >"$RUN_DIR/${label}.log"
}

assert_contains() {
  local needle="$1"
  local path="$2"
  if ! grep -Fq "$needle" "$path"; then
    echo "missing ${needle} in ${path#$ROOT_DIR/}" >&2
    exit 1
  fi
}

attempt_counter=0
fetch_url() {
  local label="$1"
  local _url="$2"
  local _timeout="$3"
  attempt_counter=$((attempt_counter + 1))
  if [[ "$attempt_counter" -eq 1 ]]; then
    write_attempt_artifacts "$label" "" "" "curl: (28) SSL connection timeout"
    LAST_BODY_PATH="$RUN_DIR/${label}.body"
    LAST_HEADERS_PATH="$RUN_DIR/${label}.headers"
    LAST_STATUS=""
    LAST_LOG_PATH="$RUN_DIR/${label}.log"
    return 1
  fi

  write_attempt_artifacts "$label" '{"status":"ok"}' '200' ''
  LAST_BODY_PATH="$RUN_DIR/${label}.body"
  LAST_HEADERS_PATH="$RUN_DIR/${label}.headers"
  LAST_STATUS='200'
  LAST_LOG_PATH="$RUN_DIR/${label}.log"
  return 0
}

fetch_required_status_with_retry metadata 04-package-meta https://example.test/package 200 1 3 0
[[ "$attempt_counter" -eq 2 ]]
[[ -f "$RUN_DIR/04-package-meta-attempt1.log" ]]
[[ -f "$RUN_DIR/04-package-meta-attempt2.log" ]]
[[ -f "$RUN_DIR/04-package-meta.log" ]]
assert_contains 'label=04-package-meta-attempt2' "$RUN_DIR/04-package-meta.log"
assert_contains '{"status":"ok"}' "$RUN_DIR/04-package-meta.body"

attempt_counter=0
fail_phase_calls=0
failed_phase=''
failed_reason=''
failed_log=''
fail_phase() {
  fail_phase_calls=$((fail_phase_calls + 1))
  failed_phase="$1"
  failed_reason="$2"
  failed_log="$3"
  return 23
}
fetch_url() {
  local label="$1"
  local _url="$2"
  local _timeout="$3"
  attempt_counter=$((attempt_counter + 1))
  write_attempt_artifacts "$label" "" "" "curl: (28) SSL connection timeout"
  LAST_BODY_PATH="$RUN_DIR/${label}.body"
  LAST_HEADERS_PATH="$RUN_DIR/${label}.headers"
  LAST_STATUS=""
  LAST_LOG_PATH="$RUN_DIR/${label}.log"
  return 1
}

if fetch_required_status_with_retry metadata 05-search https://example.test/search 200 1 3 0; then
  echo "expected fetch_required_status_with_retry to fail after exhausting retries" >&2
  exit 1
else
  rc=$?
fi

[[ "$rc" -eq 23 ]]
[[ "$attempt_counter" -eq 3 ]]
[[ "$fail_phase_calls" -eq 1 ]]
[[ "$failed_phase" == 'metadata' ]]
[[ "$failed_reason" == 'GET https://example.test/search timed out or failed to connect after 3 attempts' ]]
[[ "$failed_log" == "$RUN_DIR/05-search.log" ]]
[[ -f "$RUN_DIR/05-search-attempt1.log" ]]
[[ -f "$RUN_DIR/05-search-attempt2.log" ]]
[[ -f "$RUN_DIR/05-search-attempt3.log" ]]
assert_contains 'label=05-search-attempt3' "$RUN_DIR/05-search.log"
assert_contains 'curl: (28) SSL connection timeout' "$RUN_DIR/05-search.log"

echo 'verify-m034-s01-fetch-retry: ok'
