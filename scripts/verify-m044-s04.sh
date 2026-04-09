#!/usr/bin/env bash
set -euo pipefail

ARTIFACT_DIR=".tmp/m044-s04/verify"
PHASE_REPORT="$ARTIFACT_DIR/phase-report.txt"
STATUS_FILE="$ARTIFACT_DIR/status.txt"
CURRENT_PHASE_FILE="$ARTIFACT_DIR/current-phase.txt"
LATEST_BUNDLE_FILE="$ARTIFACT_DIR/latest-proof-bundle.txt"
CURRENT_PHASE="bootstrap"

rm -rf "$ARTIFACT_DIR"
mkdir -p "$ARTIFACT_DIR"
: > "$PHASE_REPORT"
printf 'running\n' > "$STATUS_FILE"
printf '%s\n' "$CURRENT_PHASE" > "$CURRENT_PHASE_FILE"

fail() {
  local message="$1"
  local detail="${2:-}"
  printf 'failed\n' > "$STATUS_FILE"
  printf '%s\n' "$CURRENT_PHASE" > "$CURRENT_PHASE_FILE"
  {
    printf '[%s] FAIL %s\n' "$CURRENT_PHASE" "$message"
    if [[ -n "$detail" ]]; then
      printf '  detail: %s\n' "$detail"
    fi
  } | tee -a "$PHASE_REPORT" >&2
  exit 1
}

run_phase() {
  local phase="$1"
  shift
  CURRENT_PHASE="$phase"
  printf '%s\n' "$CURRENT_PHASE" > "$CURRENT_PHASE_FILE"
  local log="$ARTIFACT_DIR/${phase}.log"
  {
    printf '== %s ==\n' "$phase"
    printf '$ %s\n' "$*"
  } | tee -a "$PHASE_REPORT"
  "$@" 2>&1 | tee "$log"
}

assert_regex() {
  local file="$1"
  local pattern="$2"
  local message="$3"
  if ! rg -q "$pattern" "$file"; then
    fail "$message" "$file"
  fi
}

assert_no_regex() {
  local file="$1"
  local pattern="$2"
  local message="$3"
  if rg -n "$pattern" "$file" >/dev/null; then
    fail "$message" "$file"
  fi
}

run_phase 01-mesh-rt-automatic-promotion \
  cargo test -p mesh-rt automatic_promotion_ -- --nocapture
assert_regex "$ARTIFACT_DIR/01-mesh-rt-automatic-promotion.log" 'running [1-9][0-9]* test' 'mesh-rt automatic promotion filter ran zero tests'
assert_regex "$ARTIFACT_DIR/01-mesh-rt-automatic-promotion.log" 'test result: ok\.' 'mesh-rt automatic promotion rail did not pass'

run_phase 02-mesh-rt-automatic-recovery \
  cargo test -p mesh-rt automatic_recovery_ -- --nocapture
assert_regex "$ARTIFACT_DIR/02-mesh-rt-automatic-recovery.log" 'running [1-9][0-9]* test' 'mesh-rt automatic recovery filter ran zero tests'
assert_regex "$ARTIFACT_DIR/02-mesh-rt-automatic-recovery.log" 'test result: ok\.' 'mesh-rt automatic recovery rail did not pass'

run_phase 03-e2e-auto-promotion \
  cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture
assert_regex "$ARTIFACT_DIR/03-e2e-auto-promotion.log" 'running [1-9][0-9]* test' 'm044_s04 auto-promotion filter ran zero tests'
assert_regex "$ARTIFACT_DIR/03-e2e-auto-promotion.log" 'test result: ok\.' 'm044_s04 auto-promotion rail did not pass'

run_phase 04-e2e-auto-resume \
  cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_resume_ -- --nocapture
assert_regex "$ARTIFACT_DIR/04-e2e-auto-resume.log" 'running [1-9][0-9]* test' 'm044_s04 auto-resume filter ran zero tests'
assert_regex "$ARTIFACT_DIR/04-e2e-auto-resume.log" 'test result: ok\.' 'm044_s04 auto-resume rail did not pass'

run_phase 05-e2e-manual-surface \
  cargo test -p meshc --test e2e_m044_s04 m044_s04_manual_surface_ -- --nocapture
assert_regex "$ARTIFACT_DIR/05-e2e-manual-surface.log" 'running [1-9][0-9]* test' 'm044_s04 manual-surface filter ran zero tests'
assert_regex "$ARTIFACT_DIR/05-e2e-manual-surface.log" 'test result: ok\.' 'm044_s04 manual-surface rail did not pass'

run_phase 06-e2e-typed-authority \
  cargo test -p meshc --test e2e_m044_s01 m044_s01_typed_continuity_ -- --nocapture
assert_regex "$ARTIFACT_DIR/06-e2e-typed-authority.log" 'running [1-9][0-9]* test' 'm044_s01 typed continuity filter ran zero tests'
assert_regex "$ARTIFACT_DIR/06-e2e-typed-authority.log" 'test result: ok\.' 'm044_s01 typed continuity rail did not pass'

run_phase 07-cluster-proof-build \
  cargo run -q -p meshc -- build cluster-proof

run_phase 08-cluster-proof-tests \
  cargo run -q -p meshc -- test cluster-proof/tests

run_phase 09-docs-truth bash -c '
  set -euo pipefail
  DOCS=(README.md cluster-proof/README.md website/docs/docs/distributed/index.md website/docs/docs/distributed-proof/index.md)
  rg -n "/promote|Continuity\\.promote" "${DOCS[@]}" && exit 1 || true
  rg -q "bounded automatic promotion" README.md
  rg -q "There is no public authority-mutation route in this package" cluster-proof/README.md
  rg -q "automatic recovery re-dispatches surviving keyed work after a safe promotion without a second submit" website/docs/docs/distributed-proof/index.md
  rg -q "bounded automatic promotion, automatic recovery" website/docs/docs/distributed/index.md
'

run_phase 10-proof-bundle-record bash -c '
  set -euo pipefail
  latest=$(find .tmp/m044-s04 -maxdepth 1 -type d -name "continuity-api-failover-promotion-rejoin-*" | sort | tail -n 1)
  [[ -n "$latest" ]]
  test -f "$latest/auto-recovery-pending-standby.json"
  test -f "$latest/auto-recovery-completed-standby.json"
  test -f "$latest/stale-guard-primary.json"
  test -f "$latest/standby-run1.stderr.log"
  test -f "$latest/primary-run2.stderr.log"
  printf "%s\n" "$latest"
'
tail -n 1 "$ARTIFACT_DIR/10-proof-bundle-record.log" > "$LATEST_BUNDLE_FILE"

printf 'ok\n' > "$STATUS_FILE"
printf 'complete\n' > "$CURRENT_PHASE_FILE"
printf '[complete] PASS verify-m044-s04\n' | tee -a "$PHASE_REPORT"
