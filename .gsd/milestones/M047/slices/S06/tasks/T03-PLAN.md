---
estimated_steps: 4
estimated_files: 7
skills_used:
  - vitepress
  - test
  - review
---

# T03: Add the final S06 closeout rail and finish the public source-first docs/migration story

**Slice:** S06 — Docs, migration, and assembled proof closeout
**Milestone:** M047

## Description

With runtime truth green, finish the final user-facing closeout: docs must teach one source-first `@cluster` story, preserve the route-free canonical surfaces, present the Todo template as the fuller starter, and make `scripts/verify-m047-s06.sh` the final assembled authority that wraps rather than replaces S04/S05 subrails.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| docs/readmes/verifier-contract strings | Fail exact file-path assertions; do not leave mixed S04/S05/S06 authority or stale migration wording. | N/A | Treat contradictory or missing markers as contract drift, not optional docs polish. |
| delegated `scripts/verify-m047-s05.sh` replay and retained artifact handoff | Stop at the first red prerequisite and keep copied bundle/log pointers; do not write into `.tmp/m047-s05/verify` from the S06 wrapper. | Bound every replayed command and fail with the captured phase log. | Reject missing `status.txt`, `phase-report.txt`, malformed bundle pointers, or zero-test filters. |

## Load Profile

- **Shared resources**: README/VitePress pages, `.tmp/m047-s05/verify`, new `.tmp/m047-s06/verify`, and the docs build.
- **Per-operation cost**: one contract test, one assembled verifier replay, and one docs build.
- **10x breakpoint**: string drift and bundle-shape mistakes fail before runtime throughput; clarity of authority and artifact ownership matters most.

## Negative Tests

- **Malformed inputs**: stale `execute_declared_work` / `Work.execute_declared_work` public docs, missing `verify-m047-s06.sh` references, docs that claim `HTTP.clustered(...)`, or wrapper scripts that write into `.tmp/m047-s05/`.
- **Error paths**: delegated S05 rail goes red, retained S05 artifact bundle is missing/malformed, docs build fails, or zero-test `m047_s06_` filters pass without running.
- **Boundary conditions**: S04 remains the authoritative cutover rail, S05 remains the delegated Todo/runtime subrail, and S06 becomes the final closeout rail while route-free `@cluster` surfaces stay canonical.

## Steps

1. Update `README.md`, `website/docs/docs/tooling/index.md`, `website/docs/docs/getting-started/clustered-example/index.md`, `website/docs/docs/distributed-proof/index.md`, and `website/docs/docs/distributed/index.md` so they teach the two-layer story: route-free canonical surfaces plus the fuller Todo starter, explicit migration from `clustered(work)` / `[cluster]` / helper-shaped names to ordinary verbs, and explicit `HTTP.clustered(...)` non-goal.
2. Add `compiler/meshc/tests/e2e_m047_s06.rs` to fail-close on the final docs/verifier authority: S04 cutover rail retained, S05 lower-level Todo subrail retained, S06 final closeout rail present, helper-shaped public names removed, and no docs claim route-local clustering shipped.
3. Add `scripts/verify-m047-s06.sh` as the final wrapper: replay `bash scripts/verify-m047-s05.sh`, copy `.tmp/m047-s05/verify` into `.tmp/m047-s06/verify/retained-m047-s05-verify`, run the S06 contract test and docs build, write its own phase/status/bundle files, and keep all artifact ownership under `.tmp/m047-s06/`.
4. Keep historical truth additive, not replacement: S04 still documents cutover, S05 still documents Todo proof, and S06 closes the milestone without reopening legacy or overclaiming `HTTP.clustered(...)`.

## Must-Haves

- [ ] Public docs/readmes show one source-first `@cluster` story with route-free canonical surfaces and the Todo starter as the fuller example.
- [ ] Migration guidance explicitly covers `clustered(work)`, `[cluster]`, and helper-shaped names like `execute_declared_work(...)` / `Work.execute_declared_work`.
- [ ] `scripts/verify-m047-s06.sh` owns the final closeout bundle under `.tmp/m047-s06/verify` while retaining delegated S05 evidence and failing closed on malformed handoff.
- [ ] `compiler/meshc/tests/e2e_m047_s06.rs` turns stale authority or `HTTP.clustered(...)` claims into named failures.

## Verification

- `cargo test -p meshc --test e2e_m047_s06 -- --nocapture`
- `bash scripts/verify-m047-s06.sh`

## Observability Impact

- Signals added/changed: `.tmp/m047-s06/verify/{status.txt,current-phase.txt,phase-report.txt,full-contract.log,latest-proof-bundle.txt}` plus retained S05 verify bundle pointers.
- How a future agent inspects this: rerun the S06 contract test or wrapper, inspect phase logs, then follow retained S05 artifacts into native/container runtime evidence.
- Failure state exposed: stale doc authority, missing migration language, zero-test drift, and malformed retained bundle handoff fail as named phases instead of generic milestone-closeout drift.

## Inputs

- `README.md` — current repo landing-page clustered story.
- `website/docs/docs/tooling/index.md` — current CLI/scaffold docs.
- `website/docs/docs/getting-started/clustered-example/index.md` — current route-free scaffold tutorial.
- `website/docs/docs/distributed-proof/index.md` — current proof map.
- `website/docs/docs/distributed/index.md` — current distributed guide with proof pointers.
- `scripts/verify-m047-s05.sh` — delegated Todo/runtime proof rail to wrap.
- `compiler/meshc/tests/e2e_m047_s05.rs` — lower-level Todo contract context for the S06 wrapper.

## Expected Output

- `README.md` — repo landing page aligned to final S06 closeout authority and migration wording.
- `website/docs/docs/tooling/index.md` — tooling docs with final closeout rail and honest Todo starter framing.
- `website/docs/docs/getting-started/clustered-example/index.md` — route-free canonical surface with final proof pointers.
- `website/docs/docs/distributed-proof/index.md` — proof map showing S04 cutover, S05 subrail, and S06 closeout layering.
- `website/docs/docs/distributed/index.md` — distributed guide aligned to the same authority story.
- `compiler/meshc/tests/e2e_m047_s06.rs` — final docs/verifier contract test.
- `scripts/verify-m047-s06.sh` — final assembled closeout wrapper with retained artifact handoff.
