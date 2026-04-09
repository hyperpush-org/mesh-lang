#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ARTIFACT_DIR="$ROOT/.tmp/m043-s04/proof-surface"
PHASE_REPORT_PATH="$ARTIFACT_DIR/phase-report.txt"
STATUS_PATH="$ARTIFACT_DIR/status.txt"
CURRENT_PHASE_PATH="$ARTIFACT_DIR/current-phase.txt"
FULL_LOG_PATH="$ARTIFACT_DIR/full-contract.log"

PROOF_PAGE="website/docs/docs/distributed-proof/index.md"
GUIDE_PAGE="website/docs/docs/distributed/index.md"
README_FILE="README.md"
RUNBOOK_FILE="cluster-proof/README.md"
SIDEBAR_FILE="website/docs/.vitepress/config.mts"
LOCAL_VERIFIER_SCRIPT="scripts/verify-m043-s03.sh"
FLY_VERIFIER_SCRIPT="scripts/verify-m043-s04-fly.sh"
SELF_VERIFIER_SCRIPT="scripts/verify-m043-s04-proof-surface.sh"
PROOF_LINK="/docs/distributed-proof/"
PROOF_LINK_PUBLIC="https://meshlang.dev/docs/distributed-proof/"
RUNBOOK_REF="cluster-proof/README.md"
RUNBOOK_LINK="https://github.com/hyperpush-org/mesh-lang/blob/main/cluster-proof/README.md"

CANONICAL_PUBLIC_PROOF_COMMANDS=(
  'bash scripts/verify-m043-s03.sh'
  'docker build -f cluster-proof/Dockerfile -t mesh-cluster-proof .'
  'fly deploy . --config cluster-proof/fly.toml --dockerfile cluster-proof/Dockerfile'
  'bash scripts/verify-m043-s04-fly.sh --help'
  'CLUSTER_PROOF_FLY_APP=mesh-cluster-proof \\'
  'CLUSTER_PROOF_BASE_URL=https://mesh-cluster-proof.fly.dev \\'
  '  bash scripts/verify-m043-s04-fly.sh'
  'bash scripts/verify-m043-s04-proof-surface.sh'
)

PROOF_PAGE_REQUIRED_STRINGS=(
  'title: Distributed Proof'
  '## Canonical surfaces'
  '## What this public rail proves now'
  '## Named proof commands'
  '## Failover contract summary'
  '## Supported topology and non-goals'
  '## Failure inspection map'
  'scripts/verify-m043-s03.sh'
  'scripts/verify-m043-s04-fly.sh'
  'scripts/verify-m043-s04-proof-surface.sh'
  '`POST /promote` is the explicit authority boundary'
  '`cluster_role`, `promotion_epoch`, and `replication_health` come from the runtime-owned authority record'
  'the same-image local authority remains `bash scripts/verify-m043-s03.sh`'
  'stale-primary fencing and fenced rejoin are part of the verified failover story'
  'Fly remains read-only evidence instead of destructive failover proof'
  'Use the generic [Distributed Actors](/docs/distributed/) guide when you want to learn the language/runtime primitives.'
  'Use this page and `cluster-proof/README.md` when you want the verified failover/operator contract instead of inferring readiness from tutorial examples.'
)

GUIDE_REQUIRED_STRINGS=(
  '> **Distributed operator proof:** This guide teaches the distribution primitives.'
  'For the verified failover/operator rail — the same-image local authority, the explicit `/promote` authority boundary, the runtime-owned `cluster_role` / `promotion_epoch` / `replication_health` truth, and the read-only Fly evidence path — start with [Distributed Proof](/docs/distributed-proof/)'
  'https://github.com/hyperpush-org/mesh-lang/blob/main/cluster-proof/README.md'
  '- [Distributed Proof](/docs/distributed-proof/) -- the canonical public proof surface for the M043 failover contract and bounded Fly operator rail'
)

README_REQUIRED_STRINGS=(
  '[Distributed Proof](https://meshlang.dev/docs/distributed-proof/)'
  '[`cluster-proof/README.md`](https://github.com/hyperpush-org/mesh-lang/blob/main/cluster-proof/README.md)'
  'If you want the verified failover/operator story instead of inferring readiness from `Node.start` / `Node.connect` examples, start here:'
  'public map of the same-image local authority, explicit `/promote` boundary, runtime-owned authority fields, and read-only Fly evidence rail'
  'the fenced stale-primary rejoin contract and one-primary-plus-one-standby topology limits'
  'For the canonical distributed failover proof story, use **[Distributed Proof](https://meshlang.dev/docs/distributed-proof/)** and the repo runbook at [`cluster-proof/README.md`](https://github.com/hyperpush-org/mesh-lang/blob/main/cluster-proof/README.md).'
)

RUNBOOK_REQUIRED_STRINGS=(
  '`GET /membership`'
  '`GET /work` — legacy routing probe'
  '`POST /work` — keyed continuity submit surface backed by the runtime `Continuity` API'
  '`GET /work/:request_key` — keyed continuity status lookup backed by the same runtime record'
  '`POST /promote` — explicit authority boundary for manual failover'
  '`cluster_role`, `promotion_epoch`, and `replication_health` come from the runtime authority record, not app-owned failover state'
  'The destructive same-image local authority remains `bash scripts/verify-m043-s03.sh`.'
  'bash scripts/verify-m043-s04-fly.sh --help'
  'bash scripts/verify-m043-s04-proof-surface.sh'
  'one active primary plus one live standby'
  'the old primary must rejoin fenced/deposed instead of resuming authority'
  'Fly remains read-only evidence, not destructive failover proof'
)

REJECTED_STRINGS=(
  'scripts/verify-m042-s03.sh'
  'scripts/verify-m042-s04-fly.sh'
  'scripts/verify-m042-s04-proof-surface.sh'
  'scripts/verify-m039-s04.sh'
  'scripts/verify-m039-s04-fly.sh'
  'scripts/verify-m039-s04-proof-surface.sh'
  'owner-loss/rejoin truth established in M042'
  'the current local authority for the keyed continuity contract, including the owner-loss/rejoin truth established in M042'
)

mkdir -p "$ARTIFACT_DIR"
: >"$PHASE_REPORT_PATH"
printf 'running\n' >"$STATUS_PATH"
printf 'init\n' >"$CURRENT_PHASE_PATH"
exec > >(tee "$FULL_LOG_PATH") 2>&1

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
limit = 220
for line in lines[:limit]:
    print(line)
if len(lines) > limit:
    print(f"... truncated after {limit} lines (total {len(lines)})")
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

cleanup() {
  local exit_code=$?
  if [[ $exit_code -eq 0 ]]; then
    printf 'ok\n' >"$STATUS_PATH"
    printf 'complete\n' >"$CURRENT_PHASE_PATH"
  elif [[ ! -f "$STATUS_PATH" || "$(<"$STATUS_PATH")" != "failed" ]]; then
    printf 'failed\n' >"$STATUS_PATH"
  fi
}
trap cleanup EXIT

phase() {
  printf '[m043-proof-surface] %s\n' "$*"
}

require_command() {
  local phase_name="$1"
  local name="$2"
  local log_path="$ARTIFACT_DIR/${phase_name}.${name}.check.log"
  if ! command -v "$name" >"$log_path" 2>&1; then
    fail_phase "$phase_name" "required command missing from PATH: ${name}" "$log_path"
  fi
}

require_file() {
  local phase_name="$1"
  local relative_path="$2"
  if [[ ! -f "$ROOT/$relative_path" ]]; then
    fail_phase "$phase_name" "missing file: ${relative_path}" "" "$ROOT/$relative_path"
  fi
}

require_contains() {
  local phase_name="$1"
  local relative_path="$2"
  local needle="$3"
  local description="$4"
  local log_path="$ARTIFACT_DIR/${phase_name}.content-check.log"
  if ! python3 - "$ROOT/$relative_path" "$needle" "$description" >"$log_path" 2>&1 <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
needle = sys.argv[2]
description = sys.argv[3]
text = path.read_text(errors='replace')
if needle not in text:
    raise SystemExit(f"{description}: missing literal {needle!r} in {path}")
print(f"{description}: matched literal {needle!r}")
PY
  then
    fail_phase "$phase_name" "$description" "$log_path" "$ROOT/$relative_path"
  fi
}

require_not_contains() {
  local phase_name="$1"
  local relative_path="$2"
  local needle="$3"
  local description="$4"
  local log_path="$ARTIFACT_DIR/${phase_name}.reject-check.log"
  if ! python3 - "$ROOT/$relative_path" "$needle" "$description" >"$log_path" 2>&1 <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
needle = sys.argv[2]
description = sys.argv[3]
text = path.read_text(errors='replace')
if needle in text:
    raise SystemExit(f"{description}: found rejected literal {needle!r} in {path}")
print(f"{description}: rejected literal {needle!r} absent")
PY
  then
    fail_phase "$phase_name" "$description" "$log_path" "$ROOT/$relative_path"
  fi
}

record_phase preflight started
printf 'preflight\n' >"$CURRENT_PHASE_PATH"
phase "checking prerequisites"
require_command preflight rg
require_command preflight python3
record_phase preflight passed

record_phase required-files started
printf 'required-files\n' >"$CURRENT_PHASE_PATH"
phase "checking proof-surface files exist"
for file in \
  "$PROOF_PAGE" \
  "$GUIDE_PAGE" \
  "$README_FILE" \
  "$RUNBOOK_FILE" \
  "$SIDEBAR_FILE" \
  "$LOCAL_VERIFIER_SCRIPT" \
  "$FLY_VERIFIER_SCRIPT" \
  "$SELF_VERIFIER_SCRIPT"; do
  require_file required-files "$file"
done
record_phase required-files passed

record_phase proof-page-contract started
printf 'proof-page-contract\n' >"$CURRENT_PHASE_PATH"
phase "checking proof page structure and contract wording"
for needle in "${PROOF_PAGE_REQUIRED_STRINGS[@]}"; do
  require_contains proof-page-contract "$PROOF_PAGE" "$needle" "proof page contract drift"
done
require_contains proof-page-contract "$PROOF_PAGE" "$RUNBOOK_REF" "proof page runbook path reference" "proof page runbook path reference"
require_contains proof-page-contract "$PROOF_PAGE" "$RUNBOOK_LINK" "proof page runbook public link" "proof page runbook public link"
record_phase proof-page-contract passed

record_phase guide-routing started
printf 'guide-routing\n' >"$CURRENT_PHASE_PATH"
phase "checking generic distributed guide routes operator claims to the proof surface"
for needle in "${GUIDE_REQUIRED_STRINGS[@]}"; do
  require_contains guide-routing "$GUIDE_PAGE" "$needle" "distributed guide proof routing drift"
done
require_contains guide-routing "$GUIDE_PAGE" "$PROOF_LINK" "distributed guide proof-page link" "distributed guide proof-page link"
require_contains guide-routing "$GUIDE_PAGE" "$RUNBOOK_LINK" "distributed guide runbook public link" "distributed guide runbook public link"
record_phase guide-routing passed

record_phase readme-routing started
printf 'readme-routing\n' >"$CURRENT_PHASE_PATH"
phase "checking repo README routes operator claims to the proof surface"
for needle in "${README_REQUIRED_STRINGS[@]}"; do
  require_contains readme-routing "$README_FILE" "$needle" "README proof routing drift"
done
require_contains readme-routing "$README_FILE" "$PROOF_LINK_PUBLIC" "README public proof-page link" "README public proof-page link"
require_contains readme-routing "$README_FILE" "$RUNBOOK_LINK" "README public runbook link" "README public runbook link"
record_phase readme-routing passed

record_phase runbook-contract started
printf 'runbook-contract\n' >"$CURRENT_PHASE_PATH"
phase "checking runbook failover wording"
for needle in "${RUNBOOK_REQUIRED_STRINGS[@]}"; do
  require_contains runbook-contract "$RUNBOOK_FILE" "$needle" "runbook failover contract drift"
done
record_phase runbook-contract passed

record_phase sidebar-wiring started
printf 'sidebar-wiring\n' >"$CURRENT_PHASE_PATH"
phase "checking sidebar wiring"
require_contains sidebar-wiring "$SIDEBAR_FILE" "{ text: 'Distributed Proof', link: '/docs/distributed-proof/', icon: 'ShieldCheck' } as any" "sidebar distributed-proof entry"
record_phase sidebar-wiring passed

record_phase command-list started
printf 'command-list\n' >"$CURRENT_PHASE_PATH"
phase "checking the proof page and runbook share the authoritative command list"
for needle in "${CANONICAL_PUBLIC_PROOF_COMMANDS[@]}"; do
  require_contains command-list "$PROOF_PAGE" "$needle" "proof-page canonical command drift"
  require_contains command-list "$RUNBOOK_FILE" "$needle" "runbook canonical command drift"
done
record_phase command-list passed

record_phase stale-wording started
printf 'stale-wording\n' >"$CURRENT_PHASE_PATH"
phase "checking stale command and wiring text is gone"
for needle in "${REJECTED_STRINGS[@]}"; do
  require_not_contains stale-wording "$README_FILE" "$needle" "README still contains stale wording"
  require_not_contains stale-wording "$PROOF_PAGE" "$needle" "proof page still contains stale wording"
  require_not_contains stale-wording "$GUIDE_PAGE" "$needle" "distributed guide still contains stale wording"
  require_not_contains stale-wording "$RUNBOOK_FILE" "$needle" "runbook still contains stale wording"
done
record_phase stale-wording passed

phase "distributed proof surface verified"
