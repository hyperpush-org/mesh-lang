# S03: `mesh-lang` Public Surface & Starter Contract Consolidation

**Goal:** Make `mesh-lang` stand on its own for evaluator-facing generated examples, starter guidance, public docs/install surfaces, and the packages/public-site deploy contract by replacing local product-source-path handoffs with a repo-boundary product handoff and by removing Hyperpush landing from the language repo’s hosted proof graph.
**Demo:** After this: After this, `mesh-lang` stands on its own for evaluator-facing generated examples, scaffolded starter docs, the public docs/install surface, and the packages deploy contract, with repo-local proof rails that do not require product-repo source paths.

## Tasks
- [x] **T01: Replaced first-contact local Mesher handoffs with a repo-boundary Hyperpush handoff derived from repo identity.** — Make the highest-leverage public surfaces truthful first. This task should introduce or extend the canonical product-handoff marker in `scripts/lib/repo-identity.json`, then use it to rewrite the generated clustered README and first-contact docs so `mesh-lang` stops teaching local `mesher/...` source paths or `bash scripts/verify-m051-*` commands as part of the evaluator-facing starter ladder.

## Steps

1. Extend `scripts/lib/repo-identity.json` with the product-handoff fields the public generator/docs/tests should consume, keeping S01’s language-vs-product repo split as the canonical source of truth.
2. Rewrite `compiler/mesh-pkg/src/scaffold.rs`, `README.md`, `website/docs/docs/getting-started/index.md`, `website/docs/docs/getting-started/clustered-example/index.md`, and `website/docs/docs/tooling/index.md` so the evaluator path stays scaffold/examples-first and stops teaching local product-source paths as the public follow-on step.
3. Update `scripts/tests/verify-m049-s04-onboarding-contract.test.mjs` and `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` to pin the new repo-boundary handoff and fail closed on stale local-product markers.
4. Re-run example parity rails so checked-in examples remain the authoritative public starting surface.

## Must-Haves

- [ ] Generated clustered README text and first-contact docs use one repo-boundary product handoff derived from the canonical repo identity contract.
- [ ] Public starter guidance preserves the SQLite-local vs Postgres-deployable split instead of collapsing back to one generic todo starter.
- [ ] Onboarding/first-contact mutation rails fail on stale `mesher/...` or `scripts/verify-m051-*` public handoff markers.
  - Estimate: 2h
  - Files: scripts/lib/repo-identity.json, compiler/mesh-pkg/src/scaffold.rs, README.md, website/docs/docs/getting-started/index.md, website/docs/docs/getting-started/clustered-example/index.md, website/docs/docs/tooling/index.md, scripts/tests/verify-m049-s04-onboarding-contract.test.mjs, scripts/tests/verify-m050-s02-first-contact-contract.test.mjs
  - Verify: node scripts/tests/verify-m049-s03-materialize-examples.mjs --check
cargo test -p meshc --test e2e_m049_s03 -- --nocapture
node --test scripts/tests/verify-m049-s04-onboarding-contract.test.mjs
node --test scripts/tests/verify-m050-s02-first-contact-contract.test.mjs
- [x] **T02: Rewrote the distributed proof pages to hand off into the Hyperpush product repo and repinned the proof-surface verifiers to that boundary.** — Once the first-contact ladder is fixed, align the public-secondary proof pages with the same boundary. This task should rewrite the distributed/proof docs so they stop teaching local product-source paths as part of the language repo’s public contract while still keeping low-level distributed primitives, clustered-app guidance, and the deeper maintained-app handoff clearly separated.

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
  - Estimate: 2h
  - Files: website/docs/docs/distributed/index.md, website/docs/docs/distributed-proof/index.md, website/docs/docs/production-backend-proof/index.md, scripts/verify-production-proof-surface.sh, scripts/tests/verify-m050-s03-secondary-surfaces.test.mjs, scripts/tests/verify-m053-s04-contract.test.mjs
  - Verify: node --test scripts/tests/verify-m050-s03-secondary-surfaces.test.mjs
node --test scripts/tests/verify-m053-s04-contract.test.mjs
bash scripts/verify-production-proof-surface.sh
npm --prefix website run build
- [x] **T03: Realign generic guide callouts, the Mesh clustering skill, and retained docs wrappers to the new public boundary** — The public contract is still incomplete if the generic guide callouts, the auto-loaded clustering skill, or the retained docs wrappers keep teaching the old local-product path. This task should align those secondary surfaces and historical rails to the same repo-boundary handoff so future agents and retained milestone verifiers do not drag the old story back in.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/tests/verify-m048-s04-skill-contract.test.mjs` | Fail closed on the first stale skill marker so executor guidance cannot silently drift. | N/A for local source checks. | Treat a mixed old/new handoff in the skill as contract drift. |
| `scripts/verify-m051-s04.sh` and `scripts/verify-m047-s06.sh` | Stop on the first retained-wrapper mismatch and preserve the failing phase under their existing `.tmp/` roots. | Use each wrapper’s bounded timeout budget and stop before stale bundle reuse. | Treat missing expected markers or malformed retained bundle pointers as wrapper drift. |

## Load Profile

Shared resources are `website/docs/.vitepress/dist`, `.tmp/m051-s04/verify/`, and `.tmp/m047-s06/verify/`; per-operation cost is one docs build, one skill mutation test, and two retained wrapper replays; the first 10x breakpoint is wrapper churn and stale docs markers across historical rails.

## Negative Tests

- **Malformed inputs**: generic guides or the clustering skill still naming local `mesher/...` paths as the public follow-on step.
- **Error paths**: the public docs are corrected, but a retained wrapper or skill still demands the old wording and makes the closeout rails false-red.
- **Boundary conditions**: generic guides stay subsystem-focused, the clustering skill stays examples-first, and historical wrappers still keep their retained proof surfaces intact.

## Steps

1. Rewrite `website/docs/docs/web/index.md`, `website/docs/docs/databases/index.md`, `website/docs/docs/testing/index.md`, `website/docs/docs/concurrency/index.md`, and `tools/skill/mesh/skills/clustering/SKILL.md` so they match the repo-boundary product handoff from T01/T02 without reintroducing local product-source-path teaching.
2. Update `scripts/tests/verify-m048-s04-skill-contract.test.mjs` plus the retained wrapper expectations in `scripts/verify-m047-s06.sh` and `scripts/verify-m051-s04.sh` so the new boundary does not make older assembled docs rails false-red.
3. Rebuild the docs site and replay the skill/historical wrapper rails.

## Must-Haves

- [ ] Generic guide callouts and the clustering skill match the same public boundary as the first-contact and proof pages.
- [ ] Retained M047/M051 docs wrappers stay green without reintroducing local product-source-path teaching.
- [ ] Skill and wrapper drift remain fail-closed.
  - Estimate: 90m
  - Files: website/docs/docs/web/index.md, website/docs/docs/databases/index.md, website/docs/docs/testing/index.md, website/docs/docs/concurrency/index.md, tools/skill/mesh/skills/clustering/SKILL.md, scripts/tests/verify-m048-s04-skill-contract.test.mjs, scripts/verify-m047-s06.sh, scripts/verify-m051-s04.sh
  - Verify: node --test scripts/tests/verify-m048-s04-skill-contract.test.mjs
bash scripts/verify-m051-s04.sh
bash scripts/verify-m047-s06.sh
npm --prefix website run build
- [x] **T04: Split the language-owned deploy/public-surface workflow from Hyperpush landing and add the assembled S03 verifier** — Finish the slice by making the hosted/public proof graph match the language-owned boundary. This task should remove Hyperpush landing deployment and landing health checks from the mesh-lang deploy contract, update the `m034`/`m053` verifier stack to the language-only workflow graph, and add one slice-owned preflight plus assembled verifier for the full mesh-lang public-surface/starter contract.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `.github/workflows/deploy-services.yml` plus workflow verifiers | Fail closed on missing or extra jobs/steps so the language repo cannot silently keep deploying product surfaces. | Stop within the existing workflow-verifier timeout budgets. | Treat lingering landing jobs/checks or a malformed health-check graph as workflow drift. |
| `scripts/lib/m034_public_surface_contract.py` and hosted/public verifiers | Stop on the first helper/verifier mismatch and preserve the failing artifact dir. | Fail within the helper’s retry budget instead of hanging on stale public endpoints. | Treat a mismatch between helper, workflow tests, and assembled wrapper as contract drift. |
| `scripts/verify-m055-s03.sh` and `scripts/tests/verify-m055-s03-contract.test.mjs` | Fail if the wrapper omits a required phase, reuses stale bundles, or publishes malformed `.tmp/m055-s03/verify/` pointers. | Stop on the first failing phase and keep the exact phase marker. | Treat missing `status.txt`, `phase-report.txt`, or `latest-proof-bundle.txt` semantics as assembled-verifier drift. |

## Load Profile

Shared resources are `website/docs/.vitepress/dist`, `packages-website/` build output, `.tmp/m034-s05/workflows/`, and `.tmp/m055-s03/verify/`; per-operation cost is workflow source tests plus one docs helper replay, one packages build, and one assembled wrapper replay; the first 10x breakpoint is build time and repeated workflow/helper replays.

## Negative Tests

- **Malformed inputs**: landing jobs or landing health checks reappear in `deploy-services.yml`; workflow tests still require `deploy-hyperpush-landing` or `Verify hyperpush landing`; the assembled wrapper points at malformed retained bundle files.
- **Error paths**: workflow YAML changes but `m034`/`m053` verifiers still pin the old graph, or the verifiers change without the wrapper adopting the same contract.
- **Boundary conditions**: mesh-lang still owns registry + packages/public-site proof, uses the existing shared helper and retry budget, and keeps the public runbook/build surfaces truthful.

## Steps

1. Rewrite `.github/workflows/deploy-services.yml` so mesh-lang owns registry + packages-website deployment and public-surface checks only, with no landing deployment or Hyperpush endpoint checks.
2. Update `scripts/lib/m034_public_surface_contract.py`, `scripts/tests/verify-m034-s05-contract.test.mjs`, `scripts/verify-m034-s05-workflows.sh`, `scripts/verify-m034-s05.sh`, `scripts/tests/verify-m053-s03-contract.test.mjs`, and `scripts/verify-m053-s03.sh` so the workflow graph, helper contract, and hosted/public proof all match the language-only boundary.
3. Add `scripts/tests/verify-m055-s03-contract.test.mjs` and `scripts/verify-m055-s03.sh` as the slice-owned fast preflight and assembled replay, publishing the standard `.tmp/m055-s03/verify/` markers and retained bundle pointer.
4. Re-run the packages build, the public-surface helper, and the assembled wrapper.

## Must-Haves

- [ ] `mesh-lang` hosted deploy/public proof no longer requires `mesher/landing` deployment or landing health checks.
- [ ] The packages/public-site contract stays inside the normal mesh-lang hosted proof and uses the same shared helper instead of ad hoc checks.
- [ ] `scripts/verify-m055-s03.sh` proves the full mesh-lang-only public/starter contract end to end and retains standard verifier markers.
  - Estimate: 3h
  - Files: .github/workflows/deploy-services.yml, scripts/lib/m034_public_surface_contract.py, scripts/tests/verify-m034-s05-contract.test.mjs, scripts/verify-m034-s05-workflows.sh, scripts/verify-m034-s05.sh, scripts/tests/verify-m053-s03-contract.test.mjs, scripts/verify-m053-s03.sh, scripts/tests/verify-m055-s03-contract.test.mjs, scripts/verify-m055-s03.sh
  - Verify: node --test scripts/tests/verify-m034-s05-contract.test.mjs
bash scripts/verify-m034-s05-workflows.sh
node --test scripts/tests/verify-m053-s03-contract.test.mjs
node --test scripts/tests/verify-m055-s03-contract.test.mjs
python3 scripts/lib/m034_public_surface_contract.py local-docs --root .
npm --prefix packages-website run build
bash scripts/verify-m055-s03.sh
