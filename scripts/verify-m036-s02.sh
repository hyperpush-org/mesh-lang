#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PHASE="${1:-all}"
NEOVIM_BIN="${NEOVIM_BIN:-nvim}"
TMP_ROOT="$ROOT_DIR/.tmp/m036-s02"
RUN_DIR="$TMP_ROOT/${PHASE}"
PACK_SOURCE_DIR="$ROOT_DIR/tools/editors/neovim-mesh"
PACK_SITE_DIR="$RUN_DIR/site"
PACK_INSTALL_DIR="$PACK_SITE_DIR/pack/mesh/start/mesh-nvim"
SMOKE_SCRIPT="$PACK_SOURCE_DIR/tests/smoke.lua"
MATERIALIZE_SCRIPT="$ROOT_DIR/scripts/tests/verify-m036-s02-materialize-corpus.mjs"
SOURCE_CASES_JSON="$ROOT_DIR/scripts/fixtures/m036-s01-syntax-corpus.json"
MATERIALIZED_CASES_DIR="$RUN_DIR/corpus"
MATERIALIZED_CASES_JSON="$MATERIALIZED_CASES_DIR/materialized-corpus.json"
RESOLVED_NEOVIM_BIN=""
LAST_STDOUT_PATH=""
LAST_STDERR_PATH=""
LAST_LOG_PATH=""

fail_phase() {
  local phase_name="$1"
  local reason="$2"
  local log_path="${3:-}"

  echo "verification drift: ${reason}" >&2
  echo "first failing phase: ${phase_name}" >&2
  echo "neovim_bin: ${NEOVIM_BIN}" >&2
  echo "artifacts: ${RUN_DIR#$ROOT_DIR/}" >&2
  if [[ -n "$log_path" && -f "$log_path" ]]; then
    echo "--- ${log_path#$ROOT_DIR/} ---" >&2
    cat "$log_path" >&2
  fi
  exit 1
}

combine_command_log() {
  local display="$1"
  local stdout_path="$2"
  local stderr_path="$3"
  local log_path="$4"

  {
    echo "display: ${display}"
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

run_command() {
  local phase_name="$1"
  local label="$2"
  local display="$3"
  local timeout_seconds="$4"
  shift 4

  local stdout_path="$RUN_DIR/${label}.stdout"
  local stderr_path="$RUN_DIR/${label}.stderr"
  local log_path="$RUN_DIR/${label}.log"

  echo "==> [${phase_name}] ${display}"
  if ! python3 - "$timeout_seconds" "$stdout_path" "$stderr_path" "$@" <<'PY'; then
import pathlib
import subprocess
import sys

timeout = float(sys.argv[1])
stdout_path = pathlib.Path(sys.argv[2])
stderr_path = pathlib.Path(sys.argv[3])
cmd = sys.argv[4:]

with stdout_path.open('wb') as stdout_handle, stderr_path.open('wb') as stderr_handle:
    try:
        completed = subprocess.run(cmd, stdout=stdout_handle, stderr=stderr_handle, timeout=timeout, check=False)
    except subprocess.TimeoutExpired:
        stderr_handle.write(f'timeout after {timeout}s for command: {cmd!r}\n'.encode())
        sys.exit(124)

sys.exit(completed.returncode)
PY
    combine_command_log "$display" "$stdout_path" "$stderr_path" "$log_path"
    if [[ -s "$stderr_path" ]] && grep -q 'timeout after' "$stderr_path"; then
      fail_phase "$phase_name" "${display} timed out" "$log_path"
    fi
    fail_phase "$phase_name" "${display} failed" "$log_path"
  fi

  combine_command_log "$display" "$stdout_path" "$stderr_path" "$log_path"
  LAST_STDOUT_PATH="$stdout_path"
  LAST_STDERR_PATH="$stderr_path"
  LAST_LOG_PATH="$log_path"
}

resolve_neovim_bin() {
  if [[ -x "$NEOVIM_BIN" ]]; then
    RESOLVED_NEOVIM_BIN="$NEOVIM_BIN"
    return 0
  fi

  local resolved
  resolved="$(command -v "$NEOVIM_BIN" 2>/dev/null || true)"
  if [[ -n "$resolved" ]]; then
    RESOLVED_NEOVIM_BIN="$resolved"
    return 0
  fi

  fail_phase "preflight" "missing Neovim binary: ${NEOVIM_BIN}"
}

verify_neovim_version() {
  local version_line
  version_line="$($RESOLVED_NEOVIM_BIN --version 2>&1 | head -n 1)"
  if [[ "$version_line" != NVIM\ v* ]]; then
    fail_phase "preflight" "unsupported editor binary: ${version_line}"
  fi

  local version
  version="$(VERSION_LINE="$version_line" python3 <<'PY'
import os
import re

line = os.environ['VERSION_LINE']
match = re.search(r'v([0-9]+\.[0-9]+(?:\.[0-9]+)?)', line)
if not match:
    raise SystemExit(1)
print(match.group(1))
PY
)"

  if ! python3 - "$version" <<'PY'; then
import sys
parts = [int(part) for part in sys.argv[1].split('.')]
while len(parts) < 3:
    parts.append(0)
raise SystemExit(0 if tuple(parts) >= (0, 11, 0) else 1)
PY
    fail_phase "preflight" "unsupported Neovim version ${version}; require >= 0.11.0"
  fi

  echo "[m036-s02] phase=preflight neovim_bin=${RESOLVED_NEOVIM_BIN} version=${version}"
}

prepare_pack_install() {
  rm -rf "$PACK_INSTALL_DIR"
  mkdir -p "$(dirname "$PACK_INSTALL_DIR")" "$RUN_DIR"
  ln -s "$PACK_SOURCE_DIR" "$PACK_INSTALL_DIR"
  echo "[m036-s02] phase=neovim pack_install=${PACK_INSTALL_DIR#$ROOT_DIR/} pack_source=${PACK_SOURCE_DIR#$ROOT_DIR/}"
}

run_corpus_phase() {
  mkdir -p "$MATERIALIZED_CASES_DIR"
  run_command \
    "corpus" \
    "corpus-materialize" \
    "node scripts/tests/verify-m036-s02-materialize-corpus.mjs --root-dir $ROOT_DIR --corpus-path ${SOURCE_CASES_JSON#$ROOT_DIR/} --out-dir ${MATERIALIZED_CASES_DIR#$ROOT_DIR/} --manifest-path ${MATERIALIZED_CASES_JSON#$ROOT_DIR/}" \
    20 \
    node \
    "$MATERIALIZE_SCRIPT" \
    --root-dir "$ROOT_DIR" \
    --corpus-path "$SOURCE_CASES_JSON" \
    --out-dir "$MATERIALIZED_CASES_DIR" \
    --manifest-path "$MATERIALIZED_CASES_JSON"

  if [[ -s "$LAST_STDOUT_PATH" ]]; then
    cat "$LAST_STDOUT_PATH"
  fi
  if [[ -s "$LAST_STDERR_PATH" ]]; then
    cat "$LAST_STDERR_PATH" >&2
  fi
  echo "[m036-s02] phase=corpus artifacts=${RUN_DIR#$ROOT_DIR/} manifest=${MATERIALIZED_CASES_JSON#$ROOT_DIR/}"
}

run_shared_grammar_phase() {
  run_command \
    "shared-grammar" \
    "shared-grammar" \
    "bash scripts/verify-m036-s01.sh" \
    90 \
    bash \
    "$ROOT_DIR/scripts/verify-m036-s01.sh"

  if [[ -s "$LAST_STDOUT_PATH" ]]; then
    cat "$LAST_STDOUT_PATH"
  fi
  if [[ -s "$LAST_STDERR_PATH" ]]; then
    cat "$LAST_STDERR_PATH" >&2
  fi
  echo "[m036-s02] phase=shared-grammar artifacts=${RUN_DIR#$ROOT_DIR/} log=${LAST_LOG_PATH#$ROOT_DIR/}"
}

run_upstream_lsp_phase() {
  run_command \
    "upstream-lsp" \
    "upstream-lsp" \
    "cargo test -q -p meshc --test e2e_lsp --manifest-path Cargo.toml -- --nocapture" \
    120 \
    cargo \
    test \
    -q \
    -p meshc \
    --test e2e_lsp \
    --manifest-path "$ROOT_DIR/Cargo.toml" \
    -- \
    --nocapture

  if [[ -s "$LAST_STDOUT_PATH" ]]; then
    cat "$LAST_STDOUT_PATH"
  fi
  if [[ -s "$LAST_STDERR_PATH" ]]; then
    cat "$LAST_STDERR_PATH" >&2
  fi
  echo "[m036-s02] phase=upstream-lsp artifacts=${RUN_DIR#$ROOT_DIR/} log=${LAST_LOG_PATH#$ROOT_DIR/}"
}

run_neovim_smoke() {
  local smoke_phase="$1"
  resolve_neovim_bin
  verify_neovim_version
  prepare_pack_install

  local display
  display="MESH_REPO_ROOT=$ROOT_DIR MESH_NVIM_CASES_JSON=${MATERIALIZED_CASES_JSON#$ROOT_DIR/} MESH_NVIM_SMOKE_PHASE=${smoke_phase} $RESOLVED_NEOVIM_BIN --headless -u NONE -i NONE --cmd <packpath setup> -l ${SMOKE_SCRIPT#$ROOT_DIR/}"

  run_command \
    "neovim" \
    "neovim-smoke" \
    "$display" \
    45 \
    env \
    "MESH_REPO_ROOT=$ROOT_DIR" \
    "MESH_NVIM_CASES_JSON=$MATERIALIZED_CASES_JSON" \
    "MESH_NVIM_SMOKE_PHASE=$smoke_phase" \
    "$RESOLVED_NEOVIM_BIN" \
    --headless \
    -u NONE \
    -i NONE \
    --cmd "set packpath^=$PACK_SITE_DIR" \
    --cmd 'packloadall' \
    --cmd 'filetype on' \
    --cmd 'syntax enable' \
    -l "$SMOKE_SCRIPT"

  if [[ -s "$LAST_STDOUT_PATH" ]]; then
    cat "$LAST_STDOUT_PATH"
  fi
  if [[ -s "$LAST_STDERR_PATH" ]]; then
    cat "$LAST_STDERR_PATH" >&2
  fi
  echo "[m036-s02] phase=neovim artifacts=${RUN_DIR#$ROOT_DIR/} log=${LAST_LOG_PATH#$ROOT_DIR/}"
}

mkdir -p "$TMP_ROOT" "$RUN_DIR"

case "$PHASE" in
  syntax)
    run_corpus_phase
    run_neovim_smoke syntax
    ;;
  lsp)
    run_upstream_lsp_phase
    run_neovim_smoke lsp
    ;;
  neovim)
    run_corpus_phase
    run_neovim_smoke all
    ;;
  all)
    run_corpus_phase
    run_shared_grammar_phase
    run_upstream_lsp_phase
    run_neovim_smoke all
    ;;
  *)
    fail_phase "$PHASE" "unsupported verify phase: ${PHASE}"
    ;;
esac
