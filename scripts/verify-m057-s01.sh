#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUTPUT_DIR="$ROOT_DIR/.gsd/milestones/M057/slices/S01"
VERIFY_ROOT="$ROOT_DIR/.tmp/m057-s01/verify"
PHASE_REPORT_PATH="$VERIFY_ROOT/phase-report.txt"
CURRENT_PHASE_PATH="$VERIFY_ROOT/current-phase.txt"
STATUS_PATH="$VERIFY_ROOT/status.txt"
FAILED_PHASE_PATH="$VERIFY_ROOT/failed-phase.txt"
LEDGER_JSON_FILENAME="reconciliation-ledger.json"
AUDIT_MD_FILENAME="reconciliation-audit.md"
LAST_LOG_PATH=""

repo_rel() {
  local candidate="$1"
  if [[ "$candidate" == "$ROOT_DIR/"* ]]; then
    printf '%s\n' "${candidate#$ROOT_DIR/}"
  else
    printf '%s\n' "$candidate"
  fi
}

prepare_verify_root() {
  rm -rf "$VERIFY_ROOT"
  mkdir -p "$VERIFY_ROOT"
  : >"$PHASE_REPORT_PATH"
  printf 'init\n' >"$CURRENT_PHASE_PATH"
  printf 'running\n' >"$STATUS_PATH"
  rm -f "$FAILED_PHASE_PATH"
}

record_phase() {
  printf '%s\t%s\n' "$1" "$2" >>"$PHASE_REPORT_PATH"
}

set_phase() {
  printf '%s\n' "$1" >"$CURRENT_PHASE_PATH"
}

combine_command_log() {
  local display="$1"
  local cwd="$2"
  local stdout_path="$3"
  local stderr_path="$4"
  local log_path="$5"

  {
    echo "display: ${display}"
    echo "cwd: $(repo_rel "$cwd")"
    if [[ -s "$stdout_path" ]]; then
      echo
      echo "[stdout]"
      cat "$stdout_path"
    fi
    if [[ -s "$stderr_path" ]]; then
      echo
      echo "[stderr]"
      cat "$stderr_path"
    fi
  } >"$log_path"
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
  local phase_name="$1"
  local reason="$2"
  local log_path="${3:-}"

  printf '%s\n' "$phase_name" >"$FAILED_PHASE_PATH"
  printf '%s\n' "$phase_name" >"$CURRENT_PHASE_PATH"
  printf 'failed\n' >"$STATUS_PATH"

  echo "verification drift: ${reason}" >&2
  echo "first failing phase: ${phase_name}" >&2
  echo "artifacts: $(repo_rel "$VERIFY_ROOT")" >&2
  if [[ -n "$log_path" && -f "$log_path" ]]; then
    echo "phase log: $(repo_rel "$log_path")" >&2
    echo "--- $(repo_rel "$log_path") ---" >&2
    print_log_excerpt "$log_path" >&2
  fi
  exit 1
}

require_file() {
  local phase_name="$1"
  local path="$2"
  local description="$3"

  if [[ -f "$path" ]]; then
    return 0
  fi

  local log_path="$VERIFY_ROOT/${phase_name}-preflight.log"
  {
    echo "preflight: missing required file"
    echo "description: ${description}"
    echo "path: $(repo_rel "$path")"
  } >"$log_path"
  fail_phase "$phase_name" "missing required file: $(repo_rel "$path")" "$log_path"
}

require_grep() {
  local phase_name="$1"
  local path="$2"
  local pattern="$3"
  local description="$4"

  if grep -Fq "$pattern" "$path"; then
    return 0
  fi

  local log_path="$VERIFY_ROOT/${phase_name}-content-check.log"
  {
    echo "content check failed"
    echo "path: $(repo_rel "$path")"
    echo "pattern: ${pattern}"
    echo "description: ${description}"
  } >"$log_path"
  fail_phase "$phase_name" "$description" "$log_path"
}

run_command() {
  local phase_name="$1"
  local label="$2"
  local timeout_seconds="$3"
  local cwd="$4"
  local display="$5"
  shift 5

  local stdout_path="$VERIFY_ROOT/${label}.stdout"
  local stderr_path="$VERIFY_ROOT/${label}.stderr"
  local log_path="$VERIFY_ROOT/${label}.log"
  local status=0

  set_phase "$phase_name"
  record_phase "$phase_name" started
  echo "==> [${phase_name}] ${display}"

  python3 - "$timeout_seconds" "$cwd" "$stdout_path" "$stderr_path" "$@" <<'PY' || status=$?
from pathlib import Path
import subprocess
import sys

try:
    timeout_seconds = float(sys.argv[1])
except ValueError as exc:
    raise SystemExit(f"invalid timeout {sys.argv[1]!r}: {exc}")

cwd = Path(sys.argv[2])
stdout_path = Path(sys.argv[3])
stderr_path = Path(sys.argv[4])
command = sys.argv[5:]
if not command:
    raise SystemExit("missing command")

stdout_path.parent.mkdir(parents=True, exist_ok=True)
with stdout_path.open('w', encoding='utf8') as stdout_handle, stderr_path.open('w', encoding='utf8') as stderr_handle:
    completed = subprocess.run(
        command,
        cwd=cwd,
        stdout=stdout_handle,
        stderr=stderr_handle,
        text=True,
        timeout=timeout_seconds,
        check=False,
    )
raise SystemExit(completed.returncode)
PY

  combine_command_log "$display" "$cwd" "$stdout_path" "$stderr_path" "$log_path"
  LAST_LOG_PATH="$log_path"

  if [[ "$status" -ne 0 ]]; then
    record_phase "$phase_name" failed
    fail_phase "$phase_name" "command failed: ${display}" "$log_path"
  fi

  record_phase "$phase_name" passed
}

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

prepare_verify_root

run_command \
  "inventory-refresh" \
  "01-inventory-refresh" \
  180 \
  "$ROOT_DIR" \
  "python3 scripts/lib/m057_tracker_inventory.py --output-dir .gsd/milestones/M057/slices/S01 --refresh --check" \
  python3 scripts/lib/m057_tracker_inventory.py --output-dir "$OUTPUT_DIR" --refresh --check

run_command \
  "evidence-build" \
  "02-evidence-build" \
  90 \
  "$ROOT_DIR" \
  "python3 scripts/lib/m057_evidence_index.py --output-dir .gsd/milestones/M057/slices/S01 --check" \
  python3 scripts/lib/m057_evidence_index.py --output-dir "$OUTPUT_DIR" --check

run_command \
  "ledger-build" \
  "03-ledger-build" \
  90 \
  "$ROOT_DIR" \
  "python3 scripts/lib/m057_reconciliation_ledger.py --output-dir .gsd/milestones/M057/slices/S01 --check" \
  python3 scripts/lib/m057_reconciliation_ledger.py --output-dir "$OUTPUT_DIR" --check

run_command \
  "inventory-contract" \
  "04-inventory-contract" \
  180 \
  "$ROOT_DIR" \
  "node --test scripts/tests/verify-m057-s01-inventory.test.mjs" \
  node --test scripts/tests/verify-m057-s01-inventory.test.mjs

run_command \
  "ledger-contract" \
  "05-ledger-contract" \
  180 \
  "$ROOT_DIR" \
  "node --test scripts/tests/verify-m057-s01-ledger.test.mjs" \
  node --test scripts/tests/verify-m057-s01-ledger.test.mjs

record_phase "ledger-surfaces" started
set_phase "ledger-surfaces"
require_file "ledger-surfaces" "$OUTPUT_DIR/$LEDGER_JSON_FILENAME" "final reconciliation ledger JSON"
require_file "ledger-surfaces" "$OUTPUT_DIR/$AUDIT_MD_FILENAME" "final reconciliation audit markdown"
require_grep "ledger-surfaces" "$OUTPUT_DIR/$AUDIT_MD_FILENAME" "## naming-drift" "audit markdown must expose the naming-drift bucket"
require_grep "ledger-surfaces" "$OUTPUT_DIR/$AUDIT_MD_FILENAME" "/pitch" "audit markdown must expose the /pitch missing-coverage gap"
record_phase "ledger-surfaces" passed
set_phase "complete"
printf 'ok\n' >"$STATUS_PATH"

echo "verification ok: $(repo_rel "$VERIFY_ROOT")"
