#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ARTIFACT_DIR=".tmp/m044-s03/verify"
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
  record_phase "$phase" started
  printf '%s\n' "$phase" >"$CURRENT_PHASE_PATH"
  echo "==> ${cmd[*]}"
  if ! run_command "$timeout_secs" "$log_path" "${cmd[@]}"; then
    record_phase "$phase" failed
    fail_phase "$phase" "expected success within ${timeout_secs}s" "$log_path"
  fi
  if [[ "$require_tests" == "yes" ]]; then
    assert_test_filter_ran "$phase" "$log_path" "$label"
  fi
  record_phase "$phase" passed
}

assert_docs_surface() {
  local phase="$1"
  local log_path="$ARTIFACT_DIR/${phase}.docs-check.log"
  if ! python3 - >"$log_path" 2>&1 <<'PY'
from pathlib import Path

checks = {
    Path("README.md"): [
        "meshc init --clustered",
        "meshc cluster status",
        "meshc cluster continuity",
        "meshc cluster diagnostics",
    ],
    Path("website/docs/docs/getting-started/index.md"): [
        "meshc init --clustered",
        "meshc cluster status",
        "meshc cluster continuity",
        "meshc cluster diagnostics",
    ],
    Path("website/docs/docs/tooling/index.md"): [
        "meshc init --clustered",
        "meshc cluster status",
        "meshc cluster continuity",
        "meshc cluster diagnostics",
        "These commands are read-only inspection surfaces.",
    ],
}
for path, needles in checks.items():
    text = path.read_text(errors="replace")
    for needle in needles:
        if needle not in text:
            raise SystemExit(f"missing {needle!r} in {path}")
print("docs-surface: ok")
PY
  then
    fail_phase "$phase" "docs surface missing required S03 markers" "$log_path"
  fi
}

assert_scaffold_contract() {
  local phase="$1"
  local log_path="$ARTIFACT_DIR/${phase}.scaffold-check.log"
  local temp_root="$ARTIFACT_DIR/scaffold-check"
  rm -rf "$temp_root"
  mkdir -p "$temp_root"
  if ! (
    cd "$temp_root"
    "$ROOT_DIR/target/debug/meshc" init --clustered verifier_clustered
  ) >"$ARTIFACT_DIR/${phase}.init.log" 2>&1; then
    fail_phase "$phase" "meshc init --clustered failed during verifier scaffold replay" "$ARTIFACT_DIR/${phase}.init.log"
  fi
  if ! python3 - "$temp_root/verifier_clustered" >"$log_path" 2>&1 <<'PY'
from pathlib import Path
import sys

root = Path(sys.argv[1])
manifest = (root / "mesh.toml").read_text(errors="replace")
main = (root / "main.mpl").read_text(errors="replace")
work = (root / "work.mpl").read_text(errors="replace")
readme = (root / "README.md").read_text(errors="replace")
required = [
    (manifest, "[cluster]"),
    (manifest, 'Work.execute_declared_work'),
    (main, 'Node.start_from_env()'),
    (main, 'BootstrapStatus'),
    (main, 'runtime bootstrap'),
    (main, 'Continuity.submit_declared_work'),
    (readme, 'Node.start_from_env()'),
    (readme, 'MESH_CONTINUITY_ROLE'),
    (readme, 'MESH_CONTINUITY_PROMOTION_EPOCH'),
    (readme, 'meshc cluster status'),
    (readme, 'meshc cluster continuity'),
    (readme, 'meshc cluster diagnostics'),
    (work, 'execute_declared_work'),
]
for text, needle in required:
    if needle not in text:
        raise SystemExit(f"missing {needle!r} in scaffold output")
for needle in ('MESH_CLUSTER_COOKIE', 'MESH_NODE_NAME', 'MESH_DISCOVERY_SEED', 'Node.start('):
    if needle in main:
        raise SystemExit(f"scaffold main.mpl still contains stale bootstrap literal {needle!r}")
for text in (manifest, main, work, readme):
    if 'CLUSTER_PROOF_' in text:
        raise SystemExit('scaffold output still contains CLUSTER_PROOF_ literals')
print('scaffold-contract: ok')
PY
  then
    fail_phase "$phase" "scaffold output drifted from the public MESH_* contract" "$log_path"
  fi
}

run_expect_success s02-contract 00-s02-contract no 1200 bash scripts/verify-m044-s02.sh
run_expect_success mesh-rt-operator-query 01-mesh-rt-operator-query yes 1200 cargo test -p mesh-rt operator_query_ -- --nocapture
run_expect_success mesh-rt-operator-diagnostics 02-mesh-rt-operator-diagnostics yes 1200 cargo test -p mesh-rt operator_diagnostics_ -- --nocapture
run_expect_success tooling-clustered-init 03-tooling-clustered-init yes 1200 cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture
run_expect_success operator-e2e 04-operator-e2e yes 1800 cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture
run_expect_success scaffold-e2e 05-scaffold-e2e yes 1800 cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture
record_phase docs-surface started
assert_docs_surface docs-surface
record_phase docs-surface passed
record_phase scaffold-contract started
assert_scaffold_contract scaffold-contract
record_phase scaffold-contract passed

echo "verify-m044-s03: ok"
