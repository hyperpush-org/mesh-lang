#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ARTIFACT_ROOT=".tmp/m046-s05"
ARTIFACT_DIR="$ARTIFACT_ROOT/verify"
PHASE_REPORT_PATH="$ARTIFACT_DIR/phase-report.txt"
STATUS_PATH="$ARTIFACT_DIR/status.txt"
CURRENT_PHASE_PATH="$ARTIFACT_DIR/current-phase.txt"
LATEST_PROOF_BUNDLE_PATH="$ARTIFACT_DIR/latest-proof-bundle.txt"
RETAINED_M047_S04_VERIFY_DIR="$ARTIFACT_DIR/retained-m047-s04-verify"

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

run_expect_success() {
  local phase="$1"
  local label="$2"
  local timeout_secs="$3"
  shift 3
  local -a cmd=("$@")
  local log_path="$ARTIFACT_DIR/${label}.log"
  record_phase "$phase" started
  printf '%s\n' "$phase" >"$CURRENT_PHASE_PATH"
  echo "==> ${cmd[*]}"
  if ! run_command "$timeout_secs" "$log_path" "${cmd[@]}"; then
    record_phase "$phase" failed
    fail_phase "$phase" "expected success within ${timeout_secs}s" "$log_path"
  fi
  record_phase "$phase" passed
}

retain_delegated_verify_or_fail() {
  local phase="$1"
  local source_dir="$2"
  local log_path="$ARTIFACT_DIR/${phase}.log"
  record_phase "$phase" started
  printf '%s\n' "$phase" >"$CURRENT_PHASE_PATH"

  if [[ ! -d "$source_dir" ]]; then
    printf 'missing delegated verify dir: %s\n' "$source_dir" >"$log_path"
    record_phase "$phase" failed
    fail_phase "$phase" "missing delegated verify directory" "$log_path"
  fi

  rm -rf "$RETAINED_M047_S04_VERIFY_DIR"
  cp -R "$source_dir" "$RETAINED_M047_S04_VERIFY_DIR" >"$log_path" 2>&1 || {
    record_phase "$phase" failed
    fail_phase "$phase" "failed to retain delegated verify directory" "$log_path" "$source_dir"
  }

  for required in status.txt current-phase.txt phase-report.txt full-contract.log latest-proof-bundle.txt; do
    if [[ ! -f "$RETAINED_M047_S04_VERIFY_DIR/$required" ]]; then
      printf 'missing retained delegated file: %s\n' "$RETAINED_M047_S04_VERIFY_DIR/$required" >"$log_path"
      record_phase "$phase" failed
      fail_phase "$phase" "delegated verifier retention is malformed" "$log_path" "$RETAINED_M047_S04_VERIFY_DIR"
    fi
  done

  if [[ "$(<"$RETAINED_M047_S04_VERIFY_DIR/status.txt")" != "ok" ]]; then
    printf 'delegated verifier status drifted: %s\n' "$(<"$RETAINED_M047_S04_VERIFY_DIR/status.txt")" >"$log_path"
    record_phase "$phase" failed
    fail_phase "$phase" "delegated verifier did not finish successfully" "$log_path" "$RETAINED_M047_S04_VERIFY_DIR/status.txt"
  fi

  if [[ "$(<"$RETAINED_M047_S04_VERIFY_DIR/current-phase.txt")" != "complete" ]]; then
    printf 'delegated verifier current-phase drifted: %s\n' "$(<"$RETAINED_M047_S04_VERIFY_DIR/current-phase.txt")" >"$log_path"
    record_phase "$phase" failed
    fail_phase "$phase" "delegated verifier did not reach complete phase" "$log_path" "$RETAINED_M047_S04_VERIFY_DIR/current-phase.txt"
  fi

  for expected_phase in contract-guards m047-s04-parser m047-s04-pkg m047-s04-compiler m047-s04-scaffold-unit m047-s04-scaffold-smoke m047-s04-tiny-cluster-tests m047-s04-tiny-cluster-build m047-s04-cluster-proof-tests m047-s04-cluster-proof-build m047-s04-docs-build m047-s04-e2e m047-s04-artifacts m047-s04-bundle-shape; do
    if ! rg -q "^${expected_phase}\\tpassed$" "$RETAINED_M047_S04_VERIFY_DIR/phase-report.txt"; then
      printf 'delegated phase report missing %s pass marker\n' "$expected_phase" >"$log_path"
      record_phase "$phase" failed
      fail_phase "$phase" "delegated verifier phase report drifted" "$log_path" "$RETAINED_M047_S04_VERIFY_DIR/phase-report.txt"
    fi
  done

  local delegated_bundle_path retained_bundle_name retained_bundle_path
  delegated_bundle_path="$(<"$RETAINED_M047_S04_VERIFY_DIR/latest-proof-bundle.txt")"
  if [[ -z "$delegated_bundle_path" ]]; then
    printf 'delegated latest-proof-bundle pointer was empty\n' >"$log_path"
    record_phase "$phase" failed
    fail_phase "$phase" "delegated verifier bundle pointer was empty" "$log_path" "$RETAINED_M047_S04_VERIFY_DIR/latest-proof-bundle.txt"
  fi

  retained_bundle_name="$(basename "$delegated_bundle_path")"
  retained_bundle_path="$RETAINED_M047_S04_VERIFY_DIR/$retained_bundle_name"
  if [[ ! -d "$retained_bundle_path" ]]; then
    printf 'retained delegated bundle directory missing: %s\n' "$retained_bundle_path" >"$log_path"
    record_phase "$phase" failed
    fail_phase "$phase" "delegated verifier bundle directory was not retained" "$log_path" "$retained_bundle_path"
  fi

  printf '%s\n' "$retained_bundle_path" >"$LATEST_PROOF_BUNDLE_PATH"
  record_phase "$phase" passed
}

run_expect_success m047-s04-replay 00-m047-s04-replay 7200 \
  bash scripts/verify-m047-s04.sh
retain_delegated_verify_or_fail retain-m047-s04-verify .tmp/m047-s04/verify

echo "verify-m046-s05: ok"
