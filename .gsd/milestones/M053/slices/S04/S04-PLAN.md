# S04: Public docs and Fly reference assets match the shipped contract

**Goal:** Make evaluator-facing docs and retained Fly reference assets tell the same honest M053 public contract: SQLite stays local-only, the generated Postgres starter owns the serious deployable path, Fly stays a bounded reference/proof environment, and packages/public-surface checks sit in the same hosted contract as starter proof.
**Demo:** After this: Read the generated/example/public docs surfaces for the starters and packages story, then verify they present SQLite as local-only, Postgres as the serious deployable starter, and Fly as the current reference proof environment without replacing the portable contract.

## Tasks
- [x] **T01: Aligned first-contact starter docs and verifiers with the M053 SQLite-local/Postgres-proof contract.** — Add light evaluator-facing wording to the repo README, Getting Started, Clustered Example, and Tooling docs so readers still move scaffold → SQLite → Postgres, while the serious Postgres starter’s staged deploy/failover truth becomes visible without turning first-contact pages into a verifier maze.

## Steps

1. Update `README.md`, `website/docs/docs/getting-started/index.md`, `website/docs/docs/getting-started/clustered-example/index.md`, and `website/docs/docs/tooling/index.md` so they keep the starter/examples-first order while naming SQLite as local-only and Postgres as the serious shared/deployable starter.
2. Add only the minimum M053 language needed for evaluators: the Postgres starter owns a staged deploy + failover proof chain, packages/public-surface checks now live in the same hosted contract, and deeper proof commands stay behind the proof pages.
3. Update the first-contact contract guardrails in `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` and `scripts/verify-m050-s02.sh` only as needed to pin the new wording/order without re-promoting retained proof apps or Fly as first-contact surfaces.

## Must-Haves

- [ ] First-contact docs still lead with generated scaffold/examples instead of retained proof fixtures.
- [ ] SQLite stays explicitly local/single-node only while Postgres is named as the serious shared/deployable starter.
- [ ] The first-contact verifier remains green and guards the new M053 wording/order.
  - Estimate: 1h
  - Files: README.md, website/docs/docs/getting-started/index.md, website/docs/docs/getting-started/clustered-example/index.md, website/docs/docs/tooling/index.md, scripts/tests/verify-m050-s02-first-contact-contract.test.mjs, scripts/verify-m050-s02.sh
  - Verify: bash scripts/verify-m050-s02.sh
- [x] **T02: Reframed Distributed Proof around the M053 Postgres starter chain and demoted Fly/`cluster-proof` to retained reference surfaces.** — Update the proof-map docs and retained Fly reference asset copy so they match the shipped M053 contract: the generated Postgres starter owns staged deploy + failover truth, SQLite remains outside clustered proof, and Fly is a bounded read-only reference environment rather than a coequal public starter surface.

## Steps

1. Rewrite `website/docs/docs/distributed-proof/index.md` around the M053 proof chain: `scripts/verify-m053-s01.sh`, `scripts/verify-m053-s02.sh`, and `scripts/verify-m053-s03.sh`, plus the SQLite local-only boundary and the hosted packages/public-surface contract.
2. Update `website/docs/docs/distributed/index.md` so its proof-page handoff points at the new M053 story instead of older proof-rail emphasis.
3. Reframe `scripts/fixtures/clustered/cluster-proof/README.md` and `scripts/verify-m043-s04-fly.sh` help text so the Fly rail is explicitly a retained reference/proof asset, not one of the equal canonical starter surfaces.
4. Adjust `scripts/fixtures/clustered/cluster-proof/tests/work.test.mpl` to pin the new retained-reference wording without widening Fly or `cluster-proof` into the public starter contract.

## Must-Haves

- [ ] `Distributed Proof` names the M053 starter-owned staged deploy, failover, and hosted-contract verifiers.
- [ ] Retained Fly reference assets describe read-only/reference proof only and stop claiming equal public-starter status.
- [ ] Old proof-app-first wording is removed or demoted behind retained/reference language.
  - Estimate: 1h
  - Files: website/docs/docs/distributed/index.md, website/docs/docs/distributed-proof/index.md, scripts/fixtures/clustered/cluster-proof/README.md, scripts/verify-m043-s04-fly.sh, scripts/fixtures/clustered/cluster-proof/tests/work.test.mpl
  - Verify: bash scripts/verify-production-proof-surface.sh && cargo run -q -p meshc -- test scripts/fixtures/clustered/cluster-proof/tests
- [x] **T03: Added the M053 S04 docs/reference verifier and fixed retained docs rails so public docs and Fly reference assets fail closed on contract drift.** — Create one slice-owned verifier surface that builds the docs, runs the existing first-contact/proof-page checks, and asserts the new M053-specific wording across public docs plus retained Fly reference assets.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `npm --prefix website run build` | Fail the verifier immediately and keep the build log under `.tmp/m053-s04/verify/`; do not treat stale `dist/` output as evidence. | Stop the phase and mark the slice verifier failed instead of continuing with half-built docs. | Treat missing built HTML or missing summary files as drift and stop before later assertions run. |
| Existing contract rails (`scripts/verify-m050-s02.sh`, `scripts/verify-production-proof-surface.sh`, and `cargo run -q -p meshc -- test scripts/fixtures/clustered/cluster-proof/tests`) | Stop on the first failing underlying rail and preserve its log path in the assembled verifier output. | Treat timeouts as contract drift; do not skip a slow rail and continue. | Fail closed if a wrapped rail stops producing the expected phase/status artifacts or command output shape. |
| New Node contract test (`scripts/tests/verify-m053-s04-contract.test.mjs`) | Fail closed on the first missing marker or stale string and report the offending file. | Stop the verifier if the test runner hangs; do not rely on partial TAP output. | Treat truncated/corrupted doc text or fixture copies as malformed input and fail with the exact surface name. |

## Load Profile

- **Shared resources**: VitePress build output under `website/docs/.vitepress/dist`, Node test runner state, Cargo test execution for the retained `cluster-proof` package, and `.tmp/m053-s04/verify/` artifact storage.
- **Per-operation cost**: one docs build, two existing shell verifiers, one retained fixture test run, and one new Node contract suite.
- **10x breakpoint**: docs build time and file-parse volume, not application throughput.

## Negative Tests

- **Malformed inputs**: missing M053 verifier names in `Distributed Proof`, retained Fly README still claiming equal canonical status, or corrupted built-doc output.
- **Error paths**: first-contact docs drift back toward proof-maze-first wording, retained Fly help text widens into a starter contract, or the assembled verifier stops before writing phase/status artifacts.
- **Boundary conditions**: duplicated trailing lines in docs, stale historical proof markers surviving beside M053 wording, and slice verifier output existing but missing the latest phase pointer.

## Steps

1. Implement `scripts/tests/verify-m053-s04-contract.test.mjs` as a fixture-backed contract suite that reads the targeted docs/reference files and rejects Fly-first, proof-maze-first, or SQLite/Postgres boundary drift.
2. Implement `scripts/verify-m053-s04.sh` so it writes `.tmp/m053-s04/verify/` phase/status artifacts, runs the docs build plus the existing proof rails, and then runs the new Node contract suite with failing-log hints.
3. Keep the verifier scoped to docs/reference surfaces only: it should not require live Fly credentials, mutate hosted infrastructure, or widen the generated starter contract.

## Must-Haves

- [ ] `scripts/tests/verify-m053-s04-contract.test.mjs` pins the M053 docs/reference wording across public docs and retained Fly assets.
- [ ] `scripts/verify-m053-s04.sh` assembles build + existing verifiers + the new test into one fail-closed surface under `.tmp/m053-s04/verify/`.
- [ ] Failure artifacts make it obvious whether drift came from first-contact docs, distributed-proof routing, retained Fly assets, or the docs build.
  - Estimate: 1h
  - Files: scripts/tests/verify-m053-s04-contract.test.mjs, scripts/verify-m053-s04.sh
  - Verify: node --test scripts/tests/verify-m053-s04-contract.test.mjs && bash scripts/verify-m053-s04.sh
