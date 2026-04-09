---
estimated_steps: 4
estimated_files: 6
skills_used:
  - vitepress
  - test
---

# T02: Rewrite the distributed/proof docs and proof-page verifiers around the same repo-boundary product handoff

Once the first-contact ladder is fixed, align the public-secondary proof pages with the same boundary. This task should rewrite the distributed/proof docs so they stop teaching local product-source paths as part of the language repo’s public contract while still keeping low-level distributed primitives, clustered-app guidance, and the deeper maintained-app handoff clearly separated.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-production-proof-surface.sh` | Fail closed with the first missing or misordered marker and name the file that drifted. | N/A for local text checks. | Treat stale local-product markers or lost SQLite/Postgres boundary text as proof-surface drift. |
| VitePress docs build | Stop on the first markdown/config break and keep the failing build log. | Fail within the normal build budget instead of hanging on stale `.vitepress` state. | Treat render/text drift as docs-surface breakage, not as a warning. |

## Load Profile

Shared resources are the clustered/proof markdown surfaces and `website/docs/.vitepress/dist`; per-operation cost is one docs build plus local source-contract tests; the first 10x breakpoint is repeated stale wording across multiple docs pages, not CPU or memory.

## Negative Tests

- **Malformed inputs**: reintroducing local `mesher/...` or `scripts/verify-m051-*` markers into public-secondary docs.
- **Error paths**: proof pages update but source verifiers still pin the old local-product markers, or source verifiers update but the docs keep the old public story.
- **Boundary conditions**: [Production Backend Proof] stays secondary, Distributed/Clustered Example ordering stays clear, and the SQLite-local vs Postgres-deployable wording survives the rewrite.

## Steps

1. Rewrite `website/docs/docs/distributed/index.md`, `website/docs/docs/distributed-proof/index.md`, and `website/docs/docs/production-backend-proof/index.md` so they use the repo-boundary product handoff instead of local `mesher/...` source paths while keeping distributed primitives, clustered example guidance, and deeper product handoff clearly separated.
2. Update `scripts/verify-production-proof-surface.sh`, `scripts/tests/verify-m050-s03-secondary-surfaces.test.mjs`, and `scripts/tests/verify-m053-s04-contract.test.mjs` to pin the new wording, ordering, and retained verifier map.
3. Keep the public proof map honest about the current SQLite-local vs Postgres-deployable split and the retained historical verifier chain instead of deleting proof pages outright.
4. Rebuild the docs site and replay the proof-page rails.

## Must-Haves

- [ ] Public-secondary distributed/proof pages no longer teach local product-source paths as part of the mesh-lang public contract.
- [ ] Distributed primitives, clustered example guidance, and deeper product handoff remain clearly separated.
- [ ] Proof-page source verifiers fail on stale local-product markers and on lost SQLite/Postgres boundary text.

## Inputs

- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/production-backend-proof/index.md`
- `scripts/verify-production-proof-surface.sh`
- `scripts/tests/verify-m050-s03-secondary-surfaces.test.mjs`
- `scripts/tests/verify-m053-s04-contract.test.mjs`
- `scripts/lib/repo-identity.json`
- `website/docs/docs/getting-started/clustered-example/index.md`

## Expected Output

- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/production-backend-proof/index.md`
- `scripts/verify-production-proof-surface.sh`
- `scripts/tests/verify-m050-s03-secondary-surfaces.test.mjs`
- `scripts/tests/verify-m053-s04-contract.test.mjs`

## Verification

node --test scripts/tests/verify-m050-s03-secondary-surfaces.test.mjs
node --test scripts/tests/verify-m053-s04-contract.test.mjs
bash scripts/verify-production-proof-surface.sh
npm --prefix website run build

## Observability Impact

- Signals added/changed: exact missing-marker and ordering failures from `scripts/verify-production-proof-surface.sh` plus docs build output.
- How a future agent inspects this: run `bash scripts/verify-production-proof-surface.sh` first, then `npm --prefix website run build` if the source contract passes but rendered docs still look wrong.
- Failure state exposed: the exact drifting file/marker pair, plus any VitePress build failure.
