#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

PROOF_PAGE="website/docs/docs/distributed-proof/index.md"
GUIDE_PAGE="website/docs/docs/distributed/index.md"
README_FILE="README.md"
RUNBOOK_FILE="cluster-proof/README.md"
SIDEBAR_FILE="website/docs/.vitepress/config.mts"
LOCAL_VERIFIER_SCRIPT="scripts/verify-m039-s04.sh"
FLY_VERIFIER_SCRIPT="scripts/verify-m039-s04-fly.sh"
SELF_VERIFIER_SCRIPT="scripts/verify-m039-s04-proof-surface.sh"
PROOF_LINK="/docs/distributed-proof/"
PROOF_LINK_PUBLIC="https://meshlang.dev/docs/distributed-proof/"
RUNBOOK_REF="cluster-proof/README.md"
RUNBOOK_LINK="https://github.com/hyperpush-org/mesh-lang/blob/main/cluster-proof/README.md"
CANONICAL_PUBLIC_PROOF_COMMANDS=(
  'docker build -f cluster-proof/Dockerfile -t mesh-cluster-proof .'
  'bash scripts/verify-m039-s04.sh'
  'fly deploy . --config cluster-proof/fly.toml --dockerfile cluster-proof/Dockerfile'
  "CLUSTER_PROOF_FLY_APP=mesh-cluster-proof \\"
  "CLUSTER_PROOF_BASE_URL=https://mesh-cluster-proof.fly.dev \\"
  '  bash scripts/verify-m039-s04-fly.sh'
  'bash scripts/verify-m039-s04-proof-surface.sh'
)
PROOF_PAGE_REQUIRED_STRINGS=(
  'title: Distributed Proof'
  "description: Canonical proof surface for Mesh's real distributed operator package, repo-root image build, local continuity verifier, Fly contract, and documentation-truth checks"
  '## Canonical surfaces'
  '## Named proof commands'
  '## Operator contract summary'
  '## When to use this page vs the generic distributed guide'
  '## Failure inspection map'
  'scripts/verify-m039-s04.sh'
  'scripts/verify-m039-s04-fly.sh'
  'scripts/verify-m039-s04-proof-surface.sh'
  'Use the generic [Distributed Actors](/docs/distributed/) guide when you want to learn the language/runtime primitives'
  'Use this page and `cluster-proof/README.md` when you want to evaluate whether Mesh currently proves a real distributed operator workflow end to end instead of inferring readiness from tutorial examples.'
  'repo-root Docker build'
  'shared-cookie, shared-seed local two-container runtime'
  'remote `/work` routing when both nodes are healthy'
  'local `/work` fallback during a degraded one-node window'
  'restored remote routing after the second node rejoins'
)
GUIDE_REQUIRED_STRINGS=(
  '> **Distributed operator proof:** This guide teaches the distribution primitives.'
  'For the concrete one-image operator path — repo-root Docker build, shared-seed local verifier, Fly deploy contract, and read-only Fly proof — start with [Distributed Proof](/docs/distributed-proof/)'
  'https://github.com/hyperpush-org/mesh-lang/blob/main/cluster-proof/README.md'
  '- [Distributed Proof](/docs/distributed-proof/) -- the canonical public proof surface for the real `cluster-proof/` operator package'
)
README_REQUIRED_STRINGS=(
  '[Distributed Proof](https://meshlang.dev/docs/distributed-proof/)'
  '[`cluster-proof/README.md`](https://github.com/hyperpush-org/mesh-lang/blob/main/cluster-proof/README.md)'
  'If you want the operator proof surface instead of inferring readiness from `Node.start` / `Node.connect` examples, start here:'
  'For the canonical distributed operator proof story, use **[Distributed Proof](https://meshlang.dev/docs/distributed-proof/)** and the repo runbook at [`cluster-proof/README.md`](https://github.com/hyperpush-org/mesh-lang/blob/main/cluster-proof/README.md).'
)
REJECTED_STRINGS=(
  'placeholder link'
)

phase() {
  printf '[distributed-proof-docs] %s\n' "$*"
}

fail() {
  printf '[distributed-proof-docs] ERROR: %s\n' "$*" >&2
  exit 1
}

require_command() {
  local name="$1"
  command -v "$name" >/dev/null 2>&1 || fail "required command missing from PATH: $name"
}

require_file() {
  local relative_path="$1"
  [[ -f "$ROOT/$relative_path" ]] || fail "missing file: $relative_path"
}

require_contains() {
  local relative_path="$1"
  local needle="$2"
  local description="$3"
  if ! rg -Fq -- "$needle" "$ROOT/$relative_path"; then
    fail "$relative_path missing ${description}: $needle"
  fi
}

require_not_contains() {
  local relative_path="$1"
  local needle="$2"
  local description="$3"
  if rg -Fq -- "$needle" "$ROOT/$relative_path"; then
    fail "$relative_path still contains ${description}: $needle"
  fi
}

phase "checking prerequisites"
require_command rg

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
  require_file "$file"
done

phase "checking proof page structure and links"
for needle in "${PROOF_PAGE_REQUIRED_STRINGS[@]}"; do
  require_contains "$PROOF_PAGE" "$needle" "proof-page contract wording"
done
require_contains "$PROOF_PAGE" "$RUNBOOK_REF" "runbook path reference"
require_contains "$PROOF_PAGE" "$RUNBOOK_LINK" "runbook public link"

phase "checking the generic guide routes operator claims to the proof surface"
for needle in "${GUIDE_REQUIRED_STRINGS[@]}"; do
  require_contains "$GUIDE_PAGE" "$needle" "distributed guide proof routing"
done
require_contains "$GUIDE_PAGE" "$PROOF_LINK" "proof-page link"
require_contains "$GUIDE_PAGE" "$RUNBOOK_LINK" "runbook public link"

phase "checking README routes operator claims to the proof surface"
for needle in "${README_REQUIRED_STRINGS[@]}"; do
  require_contains "$README_FILE" "$needle" "README proof routing"
done
require_contains "$README_FILE" "$PROOF_LINK_PUBLIC" "public proof-page link"
require_contains "$README_FILE" "$RUNBOOK_LINK" "public runbook link"

phase "checking sidebar wiring"
require_contains "$SIDEBAR_FILE" "{ text: 'Distributed Proof', link: '/docs/distributed-proof/', icon: 'ShieldCheck' } as any" "sidebar distributed-proof entry"

phase "checking the proof page and runbook share the authoritative command list"
for needle in "${CANONICAL_PUBLIC_PROOF_COMMANDS[@]}"; do
  require_contains "$PROOF_PAGE" "$needle" "canonical proof command"
  require_contains "$RUNBOOK_FILE" "$needle" "canonical proof command"
done

phase "checking the proof page does not invent commands that the runbook does not back"
while IFS= read -r command_text; do
  [[ -n "$command_text" ]] || continue
  require_contains "$RUNBOOK_FILE" "$command_text" "proof-page command mirrored in runbook"
done < <(python3 - "$ROOT/$PROOF_PAGE" <<'PY'
from pathlib import Path
import re
import sys

text = Path(sys.argv[1]).read_text()
for block in re.findall(r"```bash\n(.*?)```", text, flags=re.S):
    for line in block.splitlines():
        if line.strip():
            print(line)
PY
)

phase "checking stale operator wording is gone"
for needle in "${REJECTED_STRINGS[@]}"; do
  require_not_contains "$README_FILE" "$needle" "stale README wording"
  require_not_contains "$PROOF_PAGE" "$needle" "stale proof-page wording"
  require_not_contains "$GUIDE_PAGE" "$needle" "stale guide wording"
done

phase "distributed proof surface verified"
