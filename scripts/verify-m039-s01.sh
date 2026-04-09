#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

source scripts/lib/clustered_fixture_paths.sh
clustered_fixture_require_cluster_proof_root

ARTIFACT_DIR=".tmp/m039-s01/verify"
PHASE_REPORT_PATH="$ARTIFACT_DIR/phase-report.txt"
mkdir -p "$ARTIFACT_DIR"
: >"$PHASE_REPORT_PATH"

fail_with_log() {
  local command_text="$1"
  local reason="$2"
  local log_path="${3:-}"

  echo "verification drift: ${reason}" >&2
  echo "failing command: ${command_text}" >&2
  if [[ -n "$log_path" && -f "$log_path" ]]; then
    echo "--- ${log_path} ---" >&2
    sed -n '1,220p' "$log_path" >&2
  fi
  exit 1
}

record_phase() {
  printf '%s\t%s\n' "$1" "$2" >>"$PHASE_REPORT_PATH"
}

assert_test_filter_ran() {
  local log_path="$1"
  local label="$2"
  if ! python3 - "$log_path" "$label" >"$ARTIFACT_DIR/${label}.test-count.log" 2>&1 <<'PY'
import re
import sys
from pathlib import Path

log_path = Path(sys.argv[1])
label = sys.argv[2]
text = log_path.read_text()
counts = [int(value) for value in re.findall(r"running (\d+) test", text)]
if not counts:
    raise SystemExit(f"{label}: missing 'running N test' line")
if max(counts) <= 0:
    raise SystemExit(f"{label}: test filter ran 0 tests")
print(f"{label}: running-counts={counts}")
PY
  then
    fail_with_log "$label" "named test filter ran 0 tests or produced malformed output" "$ARTIFACT_DIR/${label}.test-count.log"
  fi
}

run_expect_success() {
  local phase="$1"
  local label="$2"
  local require_tests="$3"
  shift 3
  local -a cmd=("$@")
  local log_path="$ARTIFACT_DIR/${label}.log"
  local command_text="${cmd[*]}"

  record_phase "$phase" started
  echo "==> ${command_text}"
  if ! "${cmd[@]}" >"$log_path" 2>&1; then
    record_phase "$phase" failed
    fail_with_log "$command_text" "expected success" "$log_path"
  fi
  if [[ "$require_tests" == "yes" ]]; then
    assert_test_filter_ran "$log_path" "$label"
  fi
  record_phase "$phase" passed
}

run_expect_success build-tooling 00-build-tooling no \
  cargo build -q -p mesh-rt
[[ -f "$ROOT_DIR/target/debug/libmesh_rt.a" ]] || fail_with_log "cargo build -q -p mesh-rt" "mesh-rt static library was not built" "$ARTIFACT_DIR/00-build-tooling.log"

run_expect_success build-cluster-proof 01-build-cluster-proof no \
  cargo run -q -p meshc -- build "$CLUSTER_PROOF_FIXTURE_ROOT"
run_expect_success mesh-rt-discovery 02-mesh-rt-discovery yes \
  cargo test -p mesh-rt discovery_ -- --nocapture
run_expect_success convergence 03-e2e-converges yes \
  cargo test -p meshc --test e2e_m039_s01 e2e_m039_s01_converges_without_manual_peers -- --nocapture
run_expect_success node-loss 04-e2e-node-loss yes \
  cargo test -p meshc --test e2e_m039_s01 e2e_m039_s01_membership_updates_after_node_loss -- --nocapture

echo "verify-m039-s01: ok"
