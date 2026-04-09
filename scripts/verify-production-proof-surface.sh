#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPO_IDENTITY="scripts/lib/repo-identity.json"
CLUSTERED_EXAMPLE_PAGE="website/docs/docs/getting-started/clustered-example/index.md"
DISTRIBUTED_PAGE="website/docs/docs/distributed/index.md"
DISTRIBUTED_PROOF_PAGE="website/docs/docs/distributed-proof/index.md"
PRODUCTION_BACKEND_PROOF_PAGE="website/docs/docs/production-backend-proof/index.md"

PRODUCT_REPO_URL="$(python3 - <<'PY'
import json
from pathlib import Path
identity = json.loads(Path('scripts/lib/repo-identity.json').read_text())
print(identity['productRepo']['repoUrl'])
PY
)"
PRODUCT_RUNBOOK_URL="$(python3 - <<'PY'
import json
from pathlib import Path
identity = json.loads(Path('scripts/lib/repo-identity.json').read_text())
print(f"{identity['productRepo']['blobBaseUrl']}{identity['productHandoff']['relativeRunbookPath']}")
PY
)"
SQLITE_STARTER_URL="$(python3 - <<'PY'
import json
from pathlib import Path
identity = json.loads(Path('scripts/lib/repo-identity.json').read_text())
print(f"{identity['languageRepo']['blobBaseUrl']}examples/todo-sqlite/README.md")
PY
)"
POSTGRES_STARTER_URL="$(python3 - <<'PY'
import json
from pathlib import Path
identity = json.loads(Path('scripts/lib/repo-identity.json').read_text())
print(f"{identity['languageRepo']['blobBaseUrl']}examples/todo-postgres/README.md")
PY
)"
STALE_LOCAL_PRODUCT_RUNBOOK_URL="https://github.com/snowdamiz/mesh-lang/blob/main/mesher/README.md"
STALE_BACKEND_RUNBOOK_URL="https://github.com/snowdamiz/mesh-lang/blob/main/reference-backend/README.md"
STALE_FIXTURE_PATH='scripts/fixtures/backend/reference-backend/'

phase() {
  printf '[proof-docs] %s\n' "$*"
}

fail() {
  printf '[proof-docs] ERROR: %s\n' "$*" >&2
  exit 1
}

require_command() {
  local command_name="$1"
  if ! command -v "$command_name" >/dev/null 2>&1; then
    fail "required command missing from PATH: $command_name"
  fi
}

require_file() {
  local relative_path="$1"
  if [[ ! -f "$ROOT/$relative_path" ]]; then
    fail "missing file: $relative_path"
  fi
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

require_order() {
  local relative_path="$1"
  local first="$2"
  local second="$3"
  local description="$4"
  local output
  if ! output=$(python3 - "$ROOT/$relative_path" "$first" "$second" "$description" 2>&1 <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
first = sys.argv[2]
second = sys.argv[3]
description = sys.argv[4]
text = path.read_text(errors='replace')
first_index = text.find(first)
if first_index == -1:
    raise SystemExit(f"{description}: missing first marker {first!r} in {path}")
second_index = text.find(second)
if second_index == -1:
    raise SystemExit(f"{description}: missing second marker {second!r} in {path}")
if first_index >= second_index:
    raise SystemExit(
        f"{description}: expected {first!r} before {second!r} in {path}, got indexes {first_index} >= {second_index}"
    )
print(f"{description}: {first!r} precedes {second!r}")
PY
); then
    fail "$output"
  fi
}

phase "checking prerequisites"
require_command rg
require_command python3

phase "checking canonical proof files exist"
for relative_path in \
  "$REPO_IDENTITY" \
  "$CLUSTERED_EXAMPLE_PAGE" \
  "$DISTRIBUTED_PAGE" \
  "$DISTRIBUTED_PROOF_PAGE" \
  "$PRODUCTION_BACKEND_PROOF_PAGE"; do
  require_file "$relative_path"
done

phase "checking clustered-example boundary ordering"
require_contains "$CLUSTERED_EXAMPLE_PAGE" '## After the scaffold, pick the follow-on starter' 'clustered-example starter split heading'
require_contains "$CLUSTERED_EXAMPLE_PAGE" 'meshc init --template todo-api --db sqlite my_local_todo' 'clustered-example sqlite starter command'
require_contains "$CLUSTERED_EXAMPLE_PAGE" 'meshc init --template todo-api --db postgres my_shared_todo' 'clustered-example postgres starter command'
require_contains "$CLUSTERED_EXAMPLE_PAGE" '[Production Backend Proof](/docs/production-backend-proof/)' 'clustered-example production proof handoff'
require_contains "$CLUSTERED_EXAMPLE_PAGE" '[Distributed Proof](/docs/distributed-proof/)' 'clustered-example distributed proof handoff'
require_order "$CLUSTERED_EXAMPLE_PAGE" 'meshc init --template todo-api --db sqlite my_local_todo' 'meshc init --template todo-api --db postgres my_shared_todo' 'clustered-example keeps sqlite before postgres'
require_order "$CLUSTERED_EXAMPLE_PAGE" 'meshc init --template todo-api --db postgres my_shared_todo' '[Production Backend Proof](/docs/production-backend-proof/)' 'clustered-example keeps postgres before Production Backend Proof'
require_order "$CLUSTERED_EXAMPLE_PAGE" '[Production Backend Proof](/docs/production-backend-proof/)' '[Distributed Proof](/docs/distributed-proof/)' 'clustered-example keeps Production Backend Proof before Distributed Proof'

phase "checking Distributed Actors stays on primitives and hands off across the repo boundary"
for needle in \
  '> **Clustered proof surfaces:**' \
  '[Clustered Example](/docs/getting-started/clustered-example/)' \
  '[Distributed Proof](/docs/distributed-proof/)' \
  '[Production Backend Proof](/docs/production-backend-proof/)' \
  "$PRODUCT_REPO_URL" \
  "$PRODUCT_RUNBOOK_URL" \
  "$SQLITE_STARTER_URL" \
  "$POSTGRES_STARTER_URL"; do
  require_contains "$DISTRIBUTED_PAGE" "$needle" 'Distributed Actors proof marker'
done
for needle in \
  "$STALE_LOCAL_PRODUCT_RUNBOOK_URL" \
  'bash scripts/verify-m051-s01.sh' \
  'bash scripts/verify-m051-s02.sh' \
  "$STALE_BACKEND_RUNBOOK_URL" \
  "$STALE_FIXTURE_PATH"; do
  require_not_contains "$DISTRIBUTED_PAGE" "$needle" 'stale local product handoff'
done
require_order "$DISTRIBUTED_PAGE" '[Clustered Example](/docs/getting-started/clustered-example/)' '[Distributed Proof](/docs/distributed-proof/)' 'Distributed Actors keeps Clustered Example ahead of Distributed Proof'
require_order "$DISTRIBUTED_PAGE" '[Distributed Proof](/docs/distributed-proof/)' '[Production Backend Proof](/docs/production-backend-proof/)' 'Distributed Actors keeps Distributed Proof ahead of Production Backend Proof'
require_order "$DISTRIBUTED_PAGE" '[Production Backend Proof](/docs/production-backend-proof/)' "$PRODUCT_REPO_URL" 'Distributed Actors keeps Production Backend Proof ahead of the product repo handoff'
require_order "$DISTRIBUTED_PAGE" "$PRODUCT_REPO_URL" "$PRODUCT_RUNBOOK_URL" 'Distributed Actors keeps the product repo ahead of the product runbook'

phase "checking Distributed Proof keeps the M053 chain primary and local compatibility rails secondary"
for needle in \
  'This is the only public-secondary docs page that carries the named clustered verifier rails.' \
  'The clustered proof story now centers the generated PostgreSQL starter' \
  '## Public surfaces and verifier rails' \
  '## Retained reference rails' \
  '## Named proof commands' \
  '[Clustered Example](/docs/getting-started/clustered-example/)' \
  "$POSTGRES_STARTER_URL" \
  "$SQLITE_STARTER_URL" \
  'bash scripts/verify-m053-s01.sh' \
  'bash scripts/verify-m053-s02.sh' \
  'Keep hosted/public-surface checks as operational follow-up instead of the routine public proof chain.' \
  '[Production Backend Proof](/docs/production-backend-proof/)' \
  "$PRODUCT_REPO_URL" \
  "$PRODUCT_RUNBOOK_URL" \
  'bash scripts/verify-m051-s01.sh' \
  'bash scripts/verify-m051-s02.sh' \
  'bash scripts/verify-m043-s04-fly.sh --help' \
  'keep Fly as a retained read-only reference/proof lane for already-deployed environments instead of treating it as a coequal public starter surface' \
  '> **Note:** The Fly verifier is intentionally read-only and intentionally secondary.'; do
  require_contains "$DISTRIBUTED_PROOF_PAGE" "$needle" 'Distributed Proof marker'
done
for needle in \
  "$STALE_LOCAL_PRODUCT_RUNBOOK_URL" \
  "$STALE_BACKEND_RUNBOOK_URL" \
  "$STALE_FIXTURE_PATH"; do
  require_not_contains "$DISTRIBUTED_PROOF_PAGE" "$needle" 'stale local product handoff'
done
require_order "$DISTRIBUTED_PROOF_PAGE" '[Clustered Example](/docs/getting-started/clustered-example/)' '- [`examples/todo-postgres/README.md`](https://github.com/hyperpush-org/mesh-lang/blob/main/examples/todo-postgres/README.md) — the serious shared/deployable starter that owns the shipped clustered contract' 'Distributed Proof keeps Clustered Example ahead of the Postgres starter'
require_order "$DISTRIBUTED_PROOF_PAGE" '- [`examples/todo-postgres/README.md`](https://github.com/hyperpush-org/mesh-lang/blob/main/examples/todo-postgres/README.md) — the serious shared/deployable starter that owns the shipped clustered contract' '- [`examples/todo-sqlite/README.md`](https://github.com/hyperpush-org/mesh-lang/blob/main/examples/todo-sqlite/README.md) — the honest local single-node SQLite starter, not a clustered/operator proof surface' 'Distributed Proof keeps the Postgres starter ahead of the SQLite starter'
require_order "$DISTRIBUTED_PROOF_PAGE" '- [`examples/todo-sqlite/README.md`](https://github.com/hyperpush-org/mesh-lang/blob/main/examples/todo-sqlite/README.md) — the honest local single-node SQLite starter, not a clustered/operator proof surface' '- `bash scripts/verify-m053-s01.sh` — starter-owned staged deploy proof that retains the generated PostgreSQL bundle plus bundled artifacts' 'Distributed Proof keeps starter README markers ahead of the M053 proof chain'
require_order "$DISTRIBUTED_PROOF_PAGE" '- `bash scripts/verify-m053-s02.sh` — starter-owned failover proof that replays S01, exercises the staged PostgreSQL starter under failover, and retains the failover proof bundle' '- [Production Backend Proof](/docs/production-backend-proof/) — the compact backend proof handoff before any maintainer-only surface' 'Distributed Proof keeps the failover proof ahead of Production Backend Proof'
require_order "$DISTRIBUTED_PROOF_PAGE" '- [Production Backend Proof](/docs/production-backend-proof/) — the compact backend proof handoff before any maintainer-only surface' '- [Hyperpush product repo](https://github.com/hyperpush-org/hyperpush-mono) — repo-boundary maintained-app/backend handoff' 'Distributed Proof keeps Production Backend Proof ahead of the product repo handoff'
require_order "$DISTRIBUTED_PROOF_PAGE" '- [Hyperpush product repo](https://github.com/hyperpush-org/hyperpush-mono) — repo-boundary maintained-app/backend handoff' '- [`mesher/README.md`](https://github.com/hyperpush-org/hyperpush-mono/blob/main/mesher/README.md) — deeper maintained app runbook after the repo-boundary handoff' 'Distributed Proof keeps the product repo ahead of the product runbook'
require_order "$DISTRIBUTED_PROOF_PAGE" '## Public surfaces and verifier rails' '## Retained reference rails' 'Distributed Proof keeps public surfaces ahead of retained rails'
require_order "$DISTRIBUTED_PROOF_PAGE" '## Retained reference rails' '## Named proof commands' 'Distributed Proof keeps retained rails ahead of named commands'
require_order "$DISTRIBUTED_PROOF_PAGE" 'bash scripts/verify-m051-s01.sh' 'bash scripts/verify-m051-s02.sh' 'Distributed Proof keeps the compatibility wrapper ahead of the retained backend-only replay'

phase "checking Production Backend Proof is the repo-boundary handoff"
for needle in \
  'This is the compact public-secondary handoff for Mesh'\''s backend proof story.' \
  'This page is the repo-boundary handoff from mesh-lang into the maintained backend/app contract.' \
  'The maintained app runbook and primary verifier live in the [Hyperpush product repo]' \
  '## Canonical surfaces' \
  '## Named maintainer verifiers' \
  '## Retained backend-only recovery signals' \
  '## When to use this page vs the generic guides' \
  '## Failure inspection map' \
  '[Clustered Example](/docs/getting-started/clustered-example/)' \
  "$SQLITE_STARTER_URL" \
  "$POSTGRES_STARTER_URL" \
  "$PRODUCT_REPO_URL" \
  "$PRODUCT_RUNBOOK_URL" \
  'bash mesher/scripts/verify-maintainer-surface.sh' \
  'bash scripts/verify-m051-s01.sh' \
  'bash scripts/verify-m051-s02.sh' \
  'bash scripts/verify-production-proof-surface.sh' \
  'restart_count' \
  'last_exit_reason' \
  'recovered_jobs' \
  'last_recovery_at' \
  'last_recovery_job_id' \
  'last_recovery_count' \
  'recovery_active'; do
  require_contains "$PRODUCTION_BACKEND_PROOF_PAGE" "$needle" 'Production Backend Proof marker'
done
for needle in \
  "$STALE_LOCAL_PRODUCT_RUNBOOK_URL" \
  "$STALE_BACKEND_RUNBOOK_URL" \
  "$STALE_FIXTURE_PATH"; do
  require_not_contains "$PRODUCTION_BACKEND_PROOF_PAGE" "$needle" 'stale local product handoff'
done
require_order "$PRODUCTION_BACKEND_PROOF_PAGE" '## Canonical surfaces' '## Named maintainer verifiers' 'Production Backend Proof keeps surfaces ahead of named verifiers'
require_order "$PRODUCTION_BACKEND_PROOF_PAGE" '## Named maintainer verifiers' '## Retained backend-only recovery signals' 'Production Backend Proof keeps named verifiers ahead of retained recovery signals'
require_order "$PRODUCTION_BACKEND_PROOF_PAGE" '## Retained backend-only recovery signals' '## When to use this page vs the generic guides' 'Production Backend Proof keeps retained recovery signals ahead of the guide handoff'
require_order "$PRODUCTION_BACKEND_PROOF_PAGE" '## When to use this page vs the generic guides' '## Failure inspection map' 'Production Backend Proof keeps the guide handoff ahead of the failure map'
require_order "$PRODUCTION_BACKEND_PROOF_PAGE" '[Clustered Example](/docs/getting-started/clustered-example/)' "$SQLITE_STARTER_URL" 'Production Backend Proof keeps Clustered Example ahead of the SQLite starter'
require_order "$PRODUCTION_BACKEND_PROOF_PAGE" "$SQLITE_STARTER_URL" "$POSTGRES_STARTER_URL" 'Production Backend Proof keeps the SQLite starter ahead of the Postgres starter'
require_order "$PRODUCTION_BACKEND_PROOF_PAGE" "$POSTGRES_STARTER_URL" "$PRODUCT_REPO_URL" 'Production Backend Proof keeps the Postgres starter ahead of the product repo handoff'
require_order "$PRODUCTION_BACKEND_PROOF_PAGE" "$PRODUCT_REPO_URL" "$PRODUCT_RUNBOOK_URL" 'Production Backend Proof keeps the product repo ahead of the product runbook'
require_order "$PRODUCTION_BACKEND_PROOF_PAGE" "$PRODUCT_RUNBOOK_URL" 'bash mesher/scripts/verify-maintainer-surface.sh' 'Production Backend Proof keeps the product runbook ahead of the product verifier command'
require_order "$PRODUCTION_BACKEND_PROOF_PAGE" 'bash mesher/scripts/verify-maintainer-surface.sh' 'bash scripts/verify-m051-s01.sh' 'Production Backend Proof keeps the product-owned verifier ahead of the compatibility wrapper'
require_order "$PRODUCTION_BACKEND_PROOF_PAGE" 'bash scripts/verify-m051-s01.sh' 'bash scripts/verify-m051-s02.sh' 'Production Backend Proof keeps the compatibility wrapper ahead of the retained backend-only replay'

phase "production proof surface verified"
end-only replay'

phase "production proof surface verified"
