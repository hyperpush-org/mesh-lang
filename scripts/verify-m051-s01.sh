#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# shellcheck source=scripts/lib/m055-workspace.sh
source "$ROOT_DIR/scripts/lib/m055-workspace.sh"

fail() {
  echo "verification drift: $1" >&2
  exit 1
}

require_file() {
  local path="$1"
  local description="$2"
  if [[ ! -f "$path" ]]; then
    fail "missing ${description}: ${path}"
  fi
}

require_phase_marker() {
  local marker="$1"
  if ! rg -Fq "$marker" "$PHASE_REPORT_PATH"; then
    fail "delegated verifier phase report drifted: missing ${marker} in ${PHASE_REPORT_PATH}"
  fi
}

display_path() {
  local candidate="$1"
  python3 - "$ROOT_DIR" "$candidate" <<'PY'
import os
import sys

root = os.path.abspath(sys.argv[1])
candidate = os.path.abspath(sys.argv[2])
relative = os.path.relpath(candidate, root)
if relative == '.':
    print('.')
elif relative.startswith('..' + os.sep) or relative == '..':
    print(candidate)
else:
    print(relative.replace('\\', '/'))
PY
}

if ! m055_resolve_hyperpush_root "$ROOT_DIR" >/dev/null; then
  exit 1
fi
HYPERPUSH_ROOT="$M055_HYPERPUSH_ROOT_RESOLVED"
VERIFY_DIR="$HYPERPUSH_ROOT/.tmp/m051-s01/verify"
STATUS_PATH="$VERIFY_DIR/status.txt"
CURRENT_PHASE_PATH="$VERIFY_DIR/current-phase.txt"
PHASE_REPORT_PATH="$VERIFY_DIR/phase-report.txt"
FULL_CONTRACT_LOG_PATH="$VERIFY_DIR/full-contract.log"
LATEST_PROOF_BUNDLE_PATH="$VERIFY_DIR/latest-proof-bundle.txt"
DELEGATED_VERIFIER="$HYPERPUSH_ROOT/mesher/scripts/verify-maintainer-surface.sh"

require_file "$DELEGATED_VERIFIER" "delegated verifier"

echo "[verify-m051-s01] resolved product repo root: $(display_path "$HYPERPUSH_ROOT") (source=${M055_HYPERPUSH_ROOT_SOURCE})"
echo "[verify-m051-s01] compatibility wrapper delegating to $(display_path "$DELEGATED_VERIFIER")"
bash "$DELEGATED_VERIFIER"

for required in \
  "$STATUS_PATH" \
  "$CURRENT_PHASE_PATH" \
  "$PHASE_REPORT_PATH" \
  "$FULL_CONTRACT_LOG_PATH" \
  "$LATEST_PROOF_BUNDLE_PATH"; do
  require_file "$required" "delegated verifier artifact"
done

[[ "$(<"$STATUS_PATH")" == "ok" ]] || fail "delegated verifier did not finish ok: ${STATUS_PATH}=$(<"$STATUS_PATH")"
[[ "$(<"$CURRENT_PHASE_PATH")" == "complete" ]] || fail "delegated verifier did not finish complete: ${CURRENT_PHASE_PATH}=$(<"$CURRENT_PHASE_PATH")"

for expected_phase in \
  $'init\tpassed' \
  $'mesher-package-tests\tpassed' \
  $'mesher-package-build\tpassed' \
  $'mesher-postgres-start\tpassed' \
  $'mesher-migrate-status\tpassed' \
  $'mesher-migrate-up\tpassed' \
  $'mesher-runtime-smoke\tpassed' \
  $'mesher-bundle-shape\tpassed'; do
  require_phase_marker "$expected_phase"
done

DELEGATED_BUNDLE_PATH="$(<"$LATEST_PROOF_BUNDLE_PATH")"
[[ -n "$DELEGATED_BUNDLE_PATH" ]] || fail "delegated verifier latest-proof-bundle pointer was empty: ${LATEST_PROOF_BUNDLE_PATH}"
[[ -d "$DELEGATED_BUNDLE_PATH" ]] || fail "delegated verifier latest-proof-bundle path does not exist: ${DELEGATED_BUNDLE_PATH}"

echo "verify-m051-s01: ok"
echo "artifacts: $(display_path "$VERIFY_DIR")"
echo "proof bundle: $(display_path "$DELEGATED_BUNDLE_PATH")"
