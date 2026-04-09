#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# shellcheck source=scripts/lib/m055-workspace.sh
source "$ROOT_DIR/scripts/lib/m055-workspace.sh"

ARTIFACT_ROOT=".tmp/m055-s01"
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
for line in lines[:220]:
    print(line)
if len(lines) > 220:
    print(f"... truncated after 220 lines (total {len(lines)})")
PY
}

fail_phase() {
  local phase="$1"
  local reason="$2"
  local log_path="${3:-}"
  printf 'failed\n' >"$STATUS_PATH"
  printf '%s\n' "$phase" >"$CURRENT_PHASE_PATH"
  echo "verification drift: ${reason}" >&2
  if [[ -n "$log_path" ]]; then
    echo "failing log: ${log_path}" >&2
    echo "--- ${log_path} ---" >&2
    print_log_excerpt "$log_path" >&2
  fi
  exit 1
}

require_command() {
  local command_name="$1"
  if ! command -v "$command_name" >/dev/null 2>&1; then
    fail_phase init "required command missing from PATH: ${command_name}"
  fi
}

run_command() {
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

require_nonempty_log() {
  local phase="$1"
  local log_path="$2"
  if [[ ! -s "$log_path" ]]; then
    fail_phase "$phase" "missing or empty child log" "$log_path"
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

text = Path(sys.argv[1]).read_text(errors="replace")
label = sys.argv[2]
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

run_expect_success() {
  local phase="$1"
  local label="$2"
  local require_tests="$3"
  local timeout_secs="$4"
  shift 4
  local -a cmd=("$@")
  local log_path="$ARTIFACT_DIR/${label}.log"
  local exit_code

  record_phase "$phase" started
  printf '%s\n' "$phase" >"$CURRENT_PHASE_PATH"
  echo "==> ${cmd[*]}"
  if run_command "$timeout_secs" "$log_path" "${cmd[@]}"; then
    :
  else
    exit_code=$?
    require_nonempty_log "$phase" "$log_path"
    if [[ $exit_code -eq 124 ]]; then
      record_phase "$phase" timed_out
      fail_phase "$phase" "command timed out after ${timeout_secs}s" "$log_path"
    fi
    record_phase "$phase" failed
    fail_phase "$phase" "child command exited with status ${exit_code}" "$log_path"
  fi

  require_nonempty_log "$phase" "$log_path"
  if [[ "$require_tests" == "yes" ]]; then
    assert_test_filter_ran "$phase" "$log_path" "$label"
  fi
  record_phase "$phase" passed
}

record_phase init started
require_command bash
require_command node
require_command python3
require_command cargo
require_command npm
require_command rg
HYPERPUSH_ROOT="$(m055_resolve_hyperpush_root "$ROOT_DIR")" || fail_phase init "missing sibling product repo root or stale in-repo mesher path" "$ARTIFACT_DIR/init.hyperpush-root.log"
record_phase init passed

run_expect_success m055-s01-contract m055-s01-contract no 600 \
  node --test scripts/tests/verify-m055-s01-contract.test.mjs
run_expect_success m055-s01-local-docs m055-s01-local-docs no 600 \
  python3 scripts/lib/m034_public_surface_contract.py local-docs --root .
run_expect_success m055-s01-packages-build m055-s01-packages-build no 2400 \
  npm --prefix packages-website run build
run_expect_success m055-s01-landing-build m055-s01-landing-build no 3600 \
  bash -c 'cd "$1" && npm --prefix mesher/landing run build' _ "$HYPERPUSH_ROOT"
run_expect_success m055-s01-gsd-regression m055-s01-gsd-regression yes 2400 \
  cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_package_contract_remains_source_first_and_route_free -- --nocapture

for expected_phase in \
  init \
  m055-s01-contract \
  m055-s01-local-docs \
  m055-s01-packages-build \
  m055-s01-landing-build \
  m055-s01-gsd-regression
  do
  if ! rg -Fq "${expected_phase}	passed" "$PHASE_REPORT_PATH"; then
    fail_phase final-phase-report "phase report missing passed marker for ${expected_phase}" "$PHASE_REPORT_PATH"
  fi
done

echo "verify-m055-s01: ok"
