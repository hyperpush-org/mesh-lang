#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERIFY_ROOT="$ROOT_DIR/.tmp/m036-s03"
CURRENT_PHASE_PATH="$VERIFY_ROOT/current-phase.txt"
FAILED_PHASE_PATH="$VERIFY_ROOT/failed-phase.txt"
STATUS_PATH="$VERIFY_ROOT/status.txt"
DOCS_CONTRACT_TEST_PATH="$ROOT_DIR/scripts/tests/verify-m036-s03-contract.test.mjs"
TOOLING_DIST_PATH="$ROOT_DIR/website/docs/.vitepress/dist/docs/tooling/index.html"
VSIX_PROOF_SCRIPT_PATH="$ROOT_DIR/scripts/verify-m034-s04-extension.sh"
VSIX_PROOF_ARTIFACT_DIR="$ROOT_DIR/.tmp/m034-s04/verify"
VSCODE_EXTENSION_PACKAGE_JSON="$ROOT_DIR/tools/editors/vscode-mesh/package.json"
VSCODE_SMOKE_ARTIFACT_DIR="$VERIFY_ROOT/vscode-smoke"
VSCODE_SMOKE_LOG_PATH="$VSCODE_SMOKE_ARTIFACT_DIR/smoke.log"
VSCODE_SMOKE_CONTEXT_PATH="$VSCODE_SMOKE_ARTIFACT_DIR/context.json"
NEOVIM_VERIFY_SCRIPT_PATH="$ROOT_DIR/scripts/verify-m036-s02.sh"
NEOVIM_VENDOR_BIN_REL=".tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim"
NEOVIM_VENDOR_BIN="$ROOT_DIR/$NEOVIM_VENDOR_BIN_REL"
NEOVIM_ARTIFACT_DIR="$ROOT_DIR/.tmp/m036-s02/all"
NEOVIM_SMOKE_LOG_PATH="$NEOVIM_ARTIFACT_DIR/neovim-smoke.log"
LAST_STDOUT_PATH=""
LAST_STDERR_PATH=""
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
  : >"$CURRENT_PHASE_PATH"
  rm -f "$FAILED_PHASE_PATH" "$STATUS_PATH"
}

set_phase() {
  local phase_name="$1"
  printf '%s\n' "$phase_name" >"$CURRENT_PHASE_PATH"
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

fail_phase() {
  local phase_name="$1"
  local reason="$2"
  local log_path="${3:-}"
  local artifact_hint="${4:-}"

  printf '%s\n' "$phase_name" >"$FAILED_PHASE_PATH"
  printf 'failed\n' >"$STATUS_PATH"

  echo "verification drift: ${reason}" >&2
  echo "first failing phase: ${phase_name}" >&2
  echo "artifacts: $(repo_rel "$VERIFY_ROOT")" >&2
  if [[ -n "$artifact_hint" ]]; then
    echo "upstream artifacts: $(repo_rel "$artifact_hint")" >&2
  fi
  if [[ -n "$log_path" && -f "$log_path" ]]; then
    echo "phase log: $(repo_rel "$log_path")" >&2
    echo "--- $(repo_rel "$log_path") ---" >&2
    sed -n '1,320p' "$log_path" >&2
  fi
  exit 1
}

require_file() {
  local phase_name="$1"
  local path="$2"
  local description="$3"
  local artifact_hint="${4:-}"

  if [[ -f "$path" ]]; then
    return 0
  fi

  local log_path="$VERIFY_ROOT/${phase_name}-preflight.log"
  {
    echo "preflight: missing required file"
    echo "description: ${description}"
    echo "path: $(repo_rel "$path")"
  } >"$log_path"
  fail_phase "$phase_name" "missing required file: $(repo_rel "$path")" "$log_path" "$artifact_hint"
}

require_executable() {
  local phase_name="$1"
  local path="$2"
  local description="$3"
  local artifact_hint="${4:-}"

  if [[ -x "$path" ]]; then
    return 0
  fi

  local log_path="$VERIFY_ROOT/${phase_name}-preflight.log"
  {
    echo "preflight: missing required executable"
    echo "description: ${description}"
    echo "path: $(repo_rel "$path")"
  } >"$log_path"
  fail_phase "$phase_name" "missing required executable: $(repo_rel "$path")" "$log_path" "$artifact_hint"
}

require_package_script() {
  local phase_name="$1"
  local package_json_path="$2"
  local script_name="$3"
  local artifact_hint="${4:-}"
  local stdout_path="$VERIFY_ROOT/${phase_name}-script-check.stdout"
  local stderr_path="$VERIFY_ROOT/${phase_name}-script-check.stderr"
  local log_path="$VERIFY_ROOT/${phase_name}-script-check.log"
  local status=0

  python3 - "$package_json_path" "$script_name" >"$stdout_path" 2>"$stderr_path" <<'PY' || status=$?
import json
import sys
from pathlib import Path

package_path = Path(sys.argv[1])
script_name = sys.argv[2]

try:
    package_json = json.loads(package_path.read_text())
except Exception as exc:
    print(f"failed to read {package_path}: {exc}", file=sys.stderr)
    raise SystemExit(1)

scripts = package_json.get("scripts") or {}
if script_name not in scripts:
    print(f"missing npm script {script_name!r} in {package_path}", file=sys.stderr)
    raise SystemExit(1)

print(scripts[script_name])
PY

  combine_command_log \
    "python3 <package-json script check> $(repo_rel "$package_json_path") :: ${script_name}" \
    "$ROOT_DIR" \
    "$stdout_path" \
    "$stderr_path" \
    "$log_path"

  if [[ "$status" -ne 0 ]]; then
    fail_phase "$phase_name" "missing npm script ${script_name} in $(repo_rel "$package_json_path")" "$log_path" "$artifact_hint"
  fi
}

require_grep() {
  local phase_name="$1"
  local target_path="$2"
  local pattern="$3"
  local reason="$4"
  local artifact_hint="${5:-}"

  if grep -Fq "$pattern" "$target_path"; then
    return 0
  fi

  local log_path="$VERIFY_ROOT/${phase_name}-postcheck.log"
  {
    echo "postcheck: pattern missing"
    echo "path: $(repo_rel "$target_path")"
    echo "pattern: ${pattern}"
    echo "reason: ${reason}"
  } >"$log_path"
  fail_phase "$phase_name" "$reason" "$log_path" "$artifact_hint"
}

run_command() {
  local phase_name="$1"
  local label="$2"
  local timeout_seconds="$3"
  local cwd="$4"
  local display="$5"
  local artifact_hint="${6:-}"
  shift 6

  local stdout_path="$VERIFY_ROOT/${label}.stdout"
  local stderr_path="$VERIFY_ROOT/${label}.stderr"
  local log_path="$VERIFY_ROOT/${label}.log"
  local status=0

  set_phase "$phase_name"
  echo "==> [${phase_name}] ${display}"

  python3 - "$timeout_seconds" "$cwd" "$stdout_path" "$stderr_path" "$@" <<'PY' || status=$?
from pathlib import Path
import subprocess
import sys

try:
    timeout_seconds = float(sys.argv[1])
except ValueError as exc:
    raise SystemExit(f"invalid timeout {sys.argv[1]!r}: {exc}")

cwd = sys.argv[2]
stdout_path = Path(sys.argv[3])
stderr_path = Path(sys.argv[4])
command = sys.argv[5:]

try:
    completed = subprocess.run(
        command,
        cwd=cwd,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        timeout=timeout_seconds,
        check=False,
    )
except subprocess.TimeoutExpired as exc:
    stdout_path.write_text(exc.stdout or "")
    timeout_note = f"\n[verify-timeout] command exceeded {timeout_seconds:g} seconds\n"
    stderr_path.write_text((exc.stderr or "") + timeout_note)
    raise SystemExit(124)

stdout_path.write_text(completed.stdout)
stderr_path.write_text(completed.stderr)
raise SystemExit(completed.returncode)
PY

  combine_command_log "$display" "$cwd" "$stdout_path" "$stderr_path" "$log_path"

  if [[ "$status" -ne 0 ]]; then
    if [[ "$status" -eq 124 ]]; then
      fail_phase "$phase_name" "${display} timed out" "$log_path" "$artifact_hint"
    fi
    fail_phase "$phase_name" "${display} failed" "$log_path" "$artifact_hint"
  fi

  LAST_STDOUT_PATH="$stdout_path"
  LAST_STDERR_PATH="$stderr_path"
  LAST_LOG_PATH="$log_path"
}

prepare_verify_root

require_file "docs-contract" "$DOCS_CONTRACT_TEST_PATH" "S03 support-tier contract test"
run_command \
  "docs-contract" \
  "docs-contract" \
  120 \
  "$ROOT_DIR" \
  "node --test scripts/tests/verify-m036-s03-contract.test.mjs" \
  "" \
  node --test "$DOCS_CONTRACT_TEST_PATH"

run_command \
  "docs-build" \
  "docs-build" \
  900 \
  "$ROOT_DIR" \
  "npm --prefix website run build" \
  "" \
  npm --prefix website run build
require_file "docs-build" "$TOOLING_DIST_PATH" "built tooling page" "$VERIFY_ROOT"

require_file "vsix-proof" "$VSIX_PROOF_SCRIPT_PATH" "M034 VSIX proof wrapper" "$VSIX_PROOF_ARTIFACT_DIR"
run_command \
  "vsix-proof" \
  "vsix-proof" \
  1800 \
  "$ROOT_DIR" \
  "bash scripts/verify-m034-s04-extension.sh" \
  "$VSIX_PROOF_ARTIFACT_DIR" \
  bash "$VSIX_PROOF_SCRIPT_PATH"
require_file "vsix-proof" "$VSIX_PROOF_ARTIFACT_DIR/status.txt" "M034 VSIX proof status" "$VSIX_PROOF_ARTIFACT_DIR"
require_file "vsix-proof" "$VSIX_PROOF_ARTIFACT_DIR/verified-vsix-path.txt" "M034 verified VSIX path" "$VSIX_PROOF_ARTIFACT_DIR"
require_grep \
  "vsix-proof" \
  "$VSIX_PROOF_ARTIFACT_DIR/status.txt" \
  "ok" \
  "VSIX proof completed without writing an ok status" \
  "$VSIX_PROOF_ARTIFACT_DIR"

require_file "vscode-smoke" "$VSCODE_EXTENSION_PACKAGE_JSON" "VS Code extension package.json" "$VSCODE_SMOKE_ARTIFACT_DIR"
require_package_script "vscode-smoke" "$VSCODE_EXTENSION_PACKAGE_JSON" "test:smoke" "$VSCODE_SMOKE_ARTIFACT_DIR"
run_command \
  "vscode-smoke" \
  "vscode-smoke" \
  2400 \
  "$ROOT_DIR" \
  "npm --prefix tools/editors/vscode-mesh run test:smoke" \
  "$VSCODE_SMOKE_ARTIFACT_DIR" \
  npm --prefix tools/editors/vscode-mesh run test:smoke
require_file "vscode-smoke" "$VSCODE_SMOKE_CONTEXT_PATH" "VS Code smoke context" "$VSCODE_SMOKE_ARTIFACT_DIR"
require_file "vscode-smoke" "$VSCODE_SMOKE_LOG_PATH" "VS Code smoke log" "$VSCODE_SMOKE_ARTIFACT_DIR"
require_grep \
  "vscode-smoke" \
  "$VSCODE_SMOKE_LOG_PATH" \
  "Extension Development Host smoke passed" \
  "VS Code smoke completed without the expected pass marker" \
  "$VSCODE_SMOKE_ARTIFACT_DIR"

require_file "neovim" "$NEOVIM_VERIFY_SCRIPT_PATH" "M036 Neovim verifier" "$NEOVIM_ARTIFACT_DIR"
require_executable \
  "neovim" \
  "$NEOVIM_VENDOR_BIN" \
  "repo-local Neovim vendor binary override" \
  "$NEOVIM_ARTIFACT_DIR"
run_command \
  "neovim" \
  "neovim" \
  2400 \
  "$ROOT_DIR" \
  "NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh" \
  "$NEOVIM_ARTIFACT_DIR" \
  env "NEOVIM_BIN=$NEOVIM_VENDOR_BIN" bash "$NEOVIM_VERIFY_SCRIPT_PATH"
require_file "neovim" "$NEOVIM_SMOKE_LOG_PATH" "Neovim smoke log" "$NEOVIM_ARTIFACT_DIR"
require_grep \
  "neovim" \
  "$NEOVIM_SMOKE_LOG_PATH" \
  "[m036-s02] phase=syntax result=pass" \
  "Neovim replay log is missing the syntax pass marker" \
  "$NEOVIM_ARTIFACT_DIR"
require_grep \
  "neovim" \
  "$NEOVIM_SMOKE_LOG_PATH" \
  "[m036-s02] phase=lsp result=pass" \
  "Neovim replay log is missing the LSP pass marker" \
  "$NEOVIM_ARTIFACT_DIR"

printf 'ok\n' >"$STATUS_PATH"
printf 'complete\n' >"$CURRENT_PHASE_PATH"
rm -f "$FAILED_PHASE_PATH"

echo "verify-m036-s03: ok"
echo "artifacts: $(repo_rel "$VERIFY_ROOT")"
echo "vsix-proof artifacts: $(repo_rel "$VSIX_PROOF_ARTIFACT_DIR")"
echo "vscode-smoke artifacts: $(repo_rel "$VSCODE_SMOKE_ARTIFACT_DIR")"
echo "neovim artifacts: $(repo_rel "$NEOVIM_ARTIFACT_DIR")"
