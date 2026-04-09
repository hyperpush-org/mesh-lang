#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ARTIFACT_ROOT=".tmp/m055-s03"
ARTIFACT_DIR="$ARTIFACT_ROOT/verify"
PHASE_REPORT_PATH="$ARTIFACT_DIR/phase-report.txt"
STATUS_PATH="$ARTIFACT_DIR/status.txt"
CURRENT_PHASE_PATH="$ARTIFACT_DIR/current-phase.txt"
LATEST_PROOF_BUNDLE_PATH="$ARTIFACT_DIR/latest-proof-bundle.txt"
RETAINED_PROOF_BUNDLE_DIR="$ARTIFACT_DIR/retained-proof-bundle"

repo_rel() {
  local candidate="$1"
  if [[ "$candidate" == "$ROOT_DIR/"* ]]; then
    printf '%s\n' "${candidate#$ROOT_DIR/}"
  else
    printf '%s\n' "$candidate"
  fi
}

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

begin_phase() {
  record_phase "$1" started
  printf '%s\n' "$1" >"$CURRENT_PHASE_PATH"
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
    echo "artifact hint: $(repo_rel "$artifact_hint")" >&2
  fi
  if [[ -n "$log_path" ]]; then
    echo "failing log: $(repo_rel "$log_path")" >&2
    echo "--- $(repo_rel "$log_path") ---" >&2
    print_log_excerpt "$log_path" >&2
  fi
  exit 1
}

require_command() {
  local phase="$1"
  local command_name="$2"
  local description="$3"
  local artifact_hint="${4:-}"
  if command -v "$command_name" >/dev/null 2>&1; then
    return 0
  fi

  local log_path="$ARTIFACT_DIR/${phase}.preflight.log"
  {
    echo "preflight: missing required command"
    echo "description: ${description}"
    echo "command: ${command_name}"
  } >"$log_path"
  record_phase "$phase" failed
  fail_phase "$phase" "missing required command: ${command_name}" "$log_path" "$artifact_hint"
}

require_file() {
  local phase="$1"
  local path="$2"
  local description="$3"
  local artifact_hint="${4:-}"
  if [[ -f "$path" ]]; then
    return 0
  fi

  local log_path="$ARTIFACT_DIR/${phase}.preflight.log"
  {
    echo "preflight: missing required file"
    echo "description: ${description}"
    echo "path: $(repo_rel "$path")"
  } >"$log_path"
  record_phase "$phase" failed
  fail_phase "$phase" "missing required file: $(repo_rel "$path")" "$log_path" "$artifact_hint"
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
  local artifact_hint="$4"
  shift 4
  local -a cmd=("$@")
  local log_path="$ARTIFACT_DIR/${label}.log"

  begin_phase "$phase"
  echo "==> ${cmd[*]}"
  if ! run_command "$timeout_secs" "$log_path" "${cmd[@]}"; then
    record_phase "$phase" failed
    fail_phase "$phase" "expected success within ${timeout_secs}s" "$log_path" "$artifact_hint"
  fi
  record_phase "$phase" passed
}

copy_fixed_dir_or_fail() {
  local phase="$1"
  local source_dir="$2"
  local dest_dir="$3"
  local description="$4"
  shift 4
  local log_path="$ARTIFACT_DIR/${phase}.$(basename "$dest_dir").copy.log"

  if ! python3 - "$source_dir" "$dest_dir" "$description" "$@" >"$log_path" 2>&1 <<'PY'
from pathlib import Path
import shutil
import sys

source_dir = Path(sys.argv[1])
dest_dir = Path(sys.argv[2])
description = sys.argv[3]
required = sys.argv[4:]

if not source_dir.is_dir():
    raise SystemExit(f"{description}: missing source directory {source_dir}")
for rel in required:
    if not (source_dir / rel).exists():
        raise SystemExit(f"{description}: missing {rel} in {source_dir}")
if dest_dir.exists():
    shutil.rmtree(dest_dir)
dest_dir.parent.mkdir(parents=True, exist_ok=True)
shutil.copytree(source_dir, dest_dir, symlinks=True)
print(f"copied {source_dir} -> {dest_dir}")
PY
  then
    record_phase "$phase" failed
    fail_phase "$phase" "$description" "$log_path" "$source_dir"
  fi
}

copy_file_or_fail() {
  local phase="$1"
  local source_path="$2"
  local dest_path="$3"
  local description="$4"
  local log_path="$ARTIFACT_DIR/${phase}.copy.log"

  if [[ ! -f "$source_path" ]]; then
    {
      echo "copy: missing source file"
      echo "description: ${description}"
      echo "source: $(repo_rel "$source_path")"
    } >"$log_path"
    record_phase "$phase" failed
    fail_phase "$phase" "$description" "$log_path" "$source_path"
  fi

  mkdir -p "$(dirname "$dest_path")"
  cp "$source_path" "$dest_path"
  if [[ ! -s "$dest_path" ]]; then
    {
      echo "copy: destination file is empty"
      echo "description: ${description}"
      echo "source: $(repo_rel "$source_path")"
      echo "destination: $(repo_rel "$dest_path")"
    } >"$log_path"
    record_phase "$phase" failed
    fail_phase "$phase" "$description" "$log_path" "$dest_path"
  fi

  printf 'copied %s -> %s\n' "$(repo_rel "$source_path")" "$(repo_rel "$dest_path")" >>"$log_path"
}

assert_retained_bundle_shape() {
  local phase="$1"
  local bundle_root="$2"
  local pointer_path="$3"
  local log_path="$ARTIFACT_DIR/${phase}.bundle-check.log"

  if ! python3 - "$bundle_root" "$pointer_path" >"$log_path" 2>&1 <<'PY'
from pathlib import Path
import sys

bundle_root = Path(sys.argv[1])
pointer_path = Path(sys.argv[2])
expected_pointer = str(bundle_root)
actual_pointer = pointer_path.read_text(errors='replace').strip()
if actual_pointer != expected_pointer:
    raise SystemExit(
        f"latest-proof-bundle pointer drifted: expected {expected_pointer!r}, got {actual_pointer!r}"
    )

for rel in [
    'deploy-services.yml',
    'm034_public_surface_contract.py',
    'verify-m034-s05-workflows.sh',
    'verify-m034-s05.sh',
    'verify-m053-s03.sh',
    'verify-m055-s03.sh',
    'verify-m055-s03-contract.test.mjs',
]:
    if not (bundle_root / rel).is_file():
        raise SystemExit(f'{bundle_root}: missing required retained file {rel}')

checks = {
    'retained-m055-s01-verify': {
        'files': ['status.txt', 'current-phase.txt', 'phase-report.txt', 'full-contract.log'],
        'status': 'ok',
        'current_phase': 'complete',
        'phase_markers': [
            'm055-s01-contract\tpassed',
            'm055-s01-local-docs\tpassed',
            'm055-s01-packages-build\tpassed',
            'm055-s01-landing-build\tpassed',
            'm055-s01-gsd-regression\tpassed',
        ],
    },
    'retained-m050-s02-verify': {
        'files': ['status.txt', 'current-phase.txt', 'phase-report.txt', 'full-contract.log', 'latest-proof-bundle.txt', 'built-html/summary.json'],
        'status': 'ok',
        'current_phase': 'complete',
        'phase_markers': ['first-contact-contract\tpassed', 'm050-s02-bundle-shape\tpassed'],
    },
    'retained-m050-s03-verify': {
        'files': ['status.txt', 'current-phase.txt', 'phase-report.txt', 'full-contract.log', 'latest-proof-bundle.txt', 'built-html/summary.json'],
        'status': 'ok',
        'current_phase': 'complete',
        'phase_markers': ['secondary-surfaces-contract\tpassed', 'm050-s03-bundle-shape\tpassed'],
    },
    'retained-m051-s04-verify': {
        'files': ['status.txt', 'current-phase.txt', 'phase-report.txt', 'full-contract.log', 'latest-proof-bundle.txt', 'built-html/summary.json'],
        'status': 'ok',
        'current_phase': 'complete',
        'phase_markers': ['m051-s04-contract\tpassed', 'm051-s04-bundle-shape\tpassed'],
    },
    'retained-m034-s05-workflows': {
        'files': ['phase-report.txt', 'docs.log', 'services.log', 'full-contract.log'],
        'phase_markers': ['docs\tpassed', 'services\tpassed', 'full-contract\tpassed'],
    },
}

for rel_dir, contract in checks.items():
    base = bundle_root / rel_dir
    if not base.is_dir():
        raise SystemExit(f'{bundle_root}: missing {rel_dir}')
    for rel in contract['files']:
        if not (base / rel).exists():
            raise SystemExit(f'{base}: missing {rel}')
    if 'status' in contract:
        actual_status = (base / 'status.txt').read_text(errors='replace').strip()
        if actual_status != contract['status']:
            raise SystemExit(f'{base / "status.txt"}: expected {contract["status"]!r}, got {actual_status!r}')
    if 'current_phase' in contract:
        actual_phase = (base / 'current-phase.txt').read_text(errors='replace').strip()
        if actual_phase != contract['current_phase']:
            raise SystemExit(f'{base / "current-phase.txt"}: expected {contract["current_phase"]!r}, got {actual_phase!r}')
    phase_report = (base / 'phase-report.txt').read_text(errors='replace')
    for marker in contract['phase_markers']:
        if marker not in phase_report:
            raise SystemExit(f'{base / "phase-report.txt"}: missing phase marker {marker!r}')

print('retained-bundle-shape: ok')
PY
  then
    record_phase "$phase" failed
    fail_phase "$phase" "retained proof bundle pointer or artifact shape drifted" "$log_path" "$bundle_root"
  fi
}

record_phase init started
for command_name in node npm python3 bash rg ruby cargo; do
  require_command init "$command_name" "required command for the M055 S03 assembled replay"
done
for path in \
  "$ROOT_DIR/.github/workflows/deploy-services.yml" \
  "$ROOT_DIR/scripts/lib/m034_public_surface_contract.py" \
  "$ROOT_DIR/scripts/tests/verify-m055-s03-contract.test.mjs" \
  "$ROOT_DIR/scripts/verify-m055-s01.sh" \
  "$ROOT_DIR/scripts/verify-m050-s02.sh" \
  "$ROOT_DIR/scripts/verify-m050-s03.sh" \
  "$ROOT_DIR/scripts/verify-m051-s04.sh" \
  "$ROOT_DIR/scripts/verify-m034-s05-workflows.sh" \
  "$ROOT_DIR/scripts/verify-m034-s05.sh" \
  "$ROOT_DIR/scripts/verify-m053-s03.sh"; do
  require_file init "$path" "required M055 S03 surface"
done
record_phase init passed

run_expect_success m055-s01-wrapper m055-s01-wrapper 3600 ".tmp/m055-s01/verify" \
  bash scripts/verify-m055-s01.sh
run_expect_success m050-s02-wrapper m050-s02-wrapper 3600 ".tmp/m050-s02/verify" \
  bash scripts/verify-m050-s02.sh
run_expect_success m050-s03-wrapper m050-s03-wrapper 3600 ".tmp/m050-s03/verify" \
  bash scripts/verify-m050-s03.sh
run_expect_success m051-s04-wrapper m051-s04-wrapper 5400 ".tmp/m051-s04/verify" \
  bash scripts/verify-m051-s04.sh
run_expect_success m034-s05-workflows m034-s05-workflows 300 ".tmp/m034-s05/workflows" \
  bash scripts/verify-m034-s05-workflows.sh
run_expect_success local-docs local-docs 300 "scripts/lib/m034_public_surface_contract.py" \
  python3 scripts/lib/m034_public_surface_contract.py local-docs --root .
run_expect_success packages-build packages-build 2400 "packages-website" \
  npm --prefix packages-website run build

rm -rf "$RETAINED_PROOF_BUNDLE_DIR"
mkdir -p "$RETAINED_PROOF_BUNDLE_DIR"

begin_phase retain-m055-s01-verify
copy_fixed_dir_or_fail retain-m055-s01-verify \
  "$ROOT_DIR/.tmp/m055-s01/verify" \
  "$RETAINED_PROOF_BUNDLE_DIR/retained-m055-s01-verify" \
  "M055 S01 verify artifacts are missing or malformed" \
  status.txt \
  current-phase.txt \
  phase-report.txt \
  full-contract.log
record_phase retain-m055-s01-verify passed

begin_phase retain-m050-s02-verify
copy_fixed_dir_or_fail retain-m050-s02-verify \
  "$ROOT_DIR/.tmp/m050-s02/verify" \
  "$RETAINED_PROOF_BUNDLE_DIR/retained-m050-s02-verify" \
  "M050 S02 verify artifacts are missing or malformed" \
  status.txt \
  current-phase.txt \
  phase-report.txt \
  full-contract.log \
  latest-proof-bundle.txt \
  built-html/summary.json
record_phase retain-m050-s02-verify passed

begin_phase retain-m050-s03-verify
copy_fixed_dir_or_fail retain-m050-s03-verify \
  "$ROOT_DIR/.tmp/m050-s03/verify" \
  "$RETAINED_PROOF_BUNDLE_DIR/retained-m050-s03-verify" \
  "M050 S03 verify artifacts are missing or malformed" \
  status.txt \
  current-phase.txt \
  phase-report.txt \
  full-contract.log \
  latest-proof-bundle.txt \
  built-html/summary.json
record_phase retain-m050-s03-verify passed

begin_phase retain-m051-s04-verify
copy_fixed_dir_or_fail retain-m051-s04-verify \
  "$ROOT_DIR/.tmp/m051-s04/verify" \
  "$RETAINED_PROOF_BUNDLE_DIR/retained-m051-s04-verify" \
  "M051 S04 verify artifacts are missing or malformed" \
  status.txt \
  current-phase.txt \
  phase-report.txt \
  full-contract.log \
  latest-proof-bundle.txt \
  built-html/summary.json
record_phase retain-m051-s04-verify passed

begin_phase retain-m034-s05-workflows
copy_fixed_dir_or_fail retain-m034-s05-workflows \
  "$ROOT_DIR/.tmp/m034-s05/workflows" \
  "$RETAINED_PROOF_BUNDLE_DIR/retained-m034-s05-workflows" \
  "M034 S05 workflow artifacts are missing or malformed" \
  phase-report.txt \
  docs.log \
  services.log \
  full-contract.log
record_phase retain-m034-s05-workflows passed

begin_phase m055-s03-bundle-shape
copy_file_or_fail m055-s03-bundle-shape "$ROOT_DIR/.github/workflows/deploy-services.yml" "$RETAINED_PROOF_BUNDLE_DIR/deploy-services.yml" "missing deploy-services workflow snapshot"
copy_file_or_fail m055-s03-bundle-shape "$ROOT_DIR/scripts/lib/m034_public_surface_contract.py" "$RETAINED_PROOF_BUNDLE_DIR/m034_public_surface_contract.py" "missing public surface helper snapshot"
copy_file_or_fail m055-s03-bundle-shape "$ROOT_DIR/scripts/verify-m034-s05-workflows.sh" "$RETAINED_PROOF_BUNDLE_DIR/verify-m034-s05-workflows.sh" "missing workflow verifier snapshot"
copy_file_or_fail m055-s03-bundle-shape "$ROOT_DIR/scripts/verify-m034-s05.sh" "$RETAINED_PROOF_BUNDLE_DIR/verify-m034-s05.sh" "missing release assembly verifier snapshot"
copy_file_or_fail m055-s03-bundle-shape "$ROOT_DIR/scripts/verify-m053-s03.sh" "$RETAINED_PROOF_BUNDLE_DIR/verify-m053-s03.sh" "missing hosted evidence verifier snapshot"
copy_file_or_fail m055-s03-bundle-shape "$ROOT_DIR/scripts/verify-m055-s03.sh" "$RETAINED_PROOF_BUNDLE_DIR/verify-m055-s03.sh" "missing assembled verifier snapshot"
copy_file_or_fail m055-s03-bundle-shape "$ROOT_DIR/scripts/tests/verify-m055-s03-contract.test.mjs" "$RETAINED_PROOF_BUNDLE_DIR/verify-m055-s03-contract.test.mjs" "missing S03 contract test snapshot"
printf '%s\n' "$RETAINED_PROOF_BUNDLE_DIR" >"$LATEST_PROOF_BUNDLE_PATH"
assert_retained_bundle_shape \
  m055-s03-bundle-shape \
  "$RETAINED_PROOF_BUNDLE_DIR" \
  "$LATEST_PROOF_BUNDLE_PATH"
record_phase m055-s03-bundle-shape passed

for expected_phase in \
  init \
  m055-s01-wrapper \
  m050-s02-wrapper \
  m050-s03-wrapper \
  m051-s04-wrapper \
  m034-s05-workflows \
  local-docs \
  packages-build \
  retain-m055-s01-verify \
  retain-m050-s02-verify \
  retain-m050-s03-verify \
  retain-m051-s04-verify \
  retain-m034-s05-workflows \
  m055-s03-bundle-shape; do
  if ! rg -q "^${expected_phase}\\tpassed$" "$PHASE_REPORT_PATH"; then
    fail_phase verifier-status "phase report missing passed marker for ${expected_phase}" "$PHASE_REPORT_PATH"
  fi
done

echo "verify-m055-s03: ok"
echo "artifacts: $(repo_rel "$ARTIFACT_DIR")"
echo "proof bundle: $(repo_rel "$RETAINED_PROOF_BUNDLE_DIR")"
