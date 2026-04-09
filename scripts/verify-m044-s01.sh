#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ARTIFACT_ROOT=".tmp/m044-s01"
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

run_assert_absent_literals() {
  local phase="$1"
  local label="$2"
  local path="$3"
  shift 3
  local log_path="$ARTIFACT_DIR/${label}.log"

  record_phase "$phase" started
  printf '%s\n' "$phase" >"$CURRENT_PHASE_PATH"
  if ! python3 - "$path" "$@" >"$log_path" 2>&1 <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
needles = sys.argv[2:]
text = path.read_text(errors="replace")
present = [needle for needle in needles if needle in text]
if present:
    raise SystemExit(f"{path}: stale literals present: {present}")
print(f"{path}: forbidden literals absent ({len(needles)} checks)")
PY
  then
    record_phase "$phase" failed
    fail_phase "$phase" "stale continuity shim literals survived in ${path}" "$log_path" "$path"
  fi
  record_phase "$phase" passed
}

run_expect_success mesh-rt-staticlib 00-mesh-rt-staticlib no 240 \
  cargo build -q -p mesh-rt
run_expect_success manifest-parser 01-manifest-parser yes 240 \
  cargo test -p mesh-pkg m044_s01_clustered_manifest_ -- --nocapture
run_expect_success lsp-clustered-manifest 02-lsp-clustered-manifest yes 240 \
  cargo test -p mesh-lsp m044_s01_clustered_manifest_ -- --nocapture
run_expect_success meshc-manifest 03-meshc-manifest yes 360 \
  cargo test -p meshc --test e2e_m044_s01 m044_s01_manifest_ -- --nocapture
run_expect_success meshc-typed-runtime 04-meshc-typed-runtime yes 360 \
  cargo test -p meshc --test e2e_m044_s01 m044_s01_typed_continuity_ -- --nocapture
run_expect_success meshc-typed-compile-fail 05-meshc-typed-compile-fail yes 360 \
  cargo test -p meshc --test e2e_m044_s01 m044_s01_continuity_compile_fail_ -- --nocapture
run_expect_success cluster-proof-build 06-cluster-proof-build no 240 \
  cargo run -q -p meshc -- build cluster-proof
run_expect_success cluster-proof-tests 07-cluster-proof-tests no 240 \
  cargo run -q -p meshc -- test cluster-proof/tests
run_assert_absent_literals cluster-proof-shim-absence 08-cluster-proof-shim-absence cluster-proof/work_continuity.mpl \
  ContinuityAuthorityStatus.from_json \
  ContinuitySubmitDecision.from_json \
  WorkRequestRecord.from_json \
  parse_authority_status_json \
  parse_continuity_submit_response \
  parse_continuity_record

assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^mesh-rt-staticlib\tpassed$' "mesh-rt staticlib refresh did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^manifest-parser\tpassed$' "manifest parser replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^lsp-clustered-manifest\tpassed$' "clustered manifest LSP replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^meshc-manifest\tpassed$' "meshc manifest replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^meshc-typed-runtime\tpassed$' "typed continuity runtime replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^meshc-typed-compile-fail\tpassed$' "typed continuity compile-fail replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^cluster-proof-build\tpassed$' "cluster-proof build replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^cluster-proof-tests\tpassed$' "cluster-proof tests replay did not pass" "$ARTIFACT_DIR/full-contract.log"
assert_file_contains_regex verifier-status "$PHASE_REPORT_PATH" '^cluster-proof-shim-absence\tpassed$' "cluster-proof shim absence check did not pass" "$ARTIFACT_DIR/full-contract.log"

echo "verify-m044-s01: ok"
