# S05: Equal-surface scaffold alignment

**Goal:** Align `meshc init --clustered`, `tiny-cluster/`, and `cluster-proof/` around one route-free clustered-work contract so the scaffold, docs, and verifier surfaces all tell the same runtime-owned story and fail closed when any surface drifts.
**Demo:** After this: After this: the scaffold, `tiny-cluster/`, and `cluster-proof/` all show the same clustered-work story, and docs/verifiers fail closed if one drifts.

## Tasks
- [x] **T01: Rewrote `meshc init --clustered` to generate the route-free clustered-work scaffold contract and fail fast on routeful drift.** — Delete the last routeful scaffold shape so the generated clustered app matches the same source-first, route-free contract already proven by `tiny-cluster/` and `cluster-proof/`.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/mesh-pkg/src/scaffold.rs` clustered templates | Fail the unit and CLI scaffold rails if `[cluster]`, `/health`, `/work`, or `Continuity.submit_declared_work(...)` survive. | N/A | Treat mixed manifest/source declaration state or drifted runtime names as contract errors instead of silently preferring one path. |
| `compiler/meshc/tests/tooling_e2e.rs` init smoke | Fail fast if `meshc init --clustered` stops producing the route-free file set or README contract. | N/A | Treat malformed generated contents as scaffold drift rather than relaxing the assertions. |

## Negative Tests

- **Malformed inputs**: duplicate manifest/source declaration hints, missing `declared_work_runtime_name()`, or changed runtime handler strings.
- **Error paths**: any surviving `[cluster]`, `HTTP.serve(...)`, `/health`, `/work`, `Continuity.submit_declared_work(...)`, `Timer.sleep(...)`, or request-key-only continuity guidance must fail the scaffold contract.
- **Boundary conditions**: the generated scaffold may keep scaffold-specific naming/runbook text, but the emitted `main.mpl`/`work.mpl` control flow must stay structurally aligned with the proof packages.

## Steps

1. Rewrite the clustered scaffold templates in `compiler/mesh-pkg/src/scaffold.rs` so `mesh.toml` is package-only, `main.mpl` only logs `Node.start_from_env()` bootstrap success/failure, and `work.mpl` matches the proof packages around `declared_work_runtime_name()`, `clustered(work)`, and `1 + 1`.
2. Rewrite the generated clustered README contract so it explains source-owned `clustered(work)`, package-only `mesh.toml`, automatic startup work, and CLI-only inspection instead of HTTP submit/status routes.
3. Update the embedded scaffold unit test and the `tooling_e2e` init smoke test to assert the new route-free contract and forbid the deleted routeful strings.
4. Keep the scaffold README scaffold-specific where needed (local run env guidance), but do not let it diverge from the shared runtime-owned clustered story.

## Must-Haves

- [ ] `meshc init --clustered` no longer emits `[cluster]`, `HTTP.serve(...)`, `/health`, `/work`, or `Continuity.submit_declared_work(...)`.
- [ ] Generated `work.mpl` contains `declared_work_runtime_name()`, one `clustered(work)` declaration, runtime name `Work.execute_declared_work`, and visible `1 + 1` work.
- [ ] Generated `main.mpl` has one `Node.start_from_env()` bootstrap path and only logs success/failure.
- [ ] The fast scaffold unit/CLI rails assert the route-free contract directly.

## Done When

- [ ] `compiler/mesh-pkg/src/scaffold.rs` emits the same clustered-work story as the proof packages.
- [ ] `compiler/meshc/tests/tooling_e2e.rs` passes against the new scaffold output without retaining routeful expectations.
  - Estimate: 2h
  - Files: compiler/mesh-pkg/src/scaffold.rs, compiler/meshc/tests/tooling_e2e.rs
  - Verify: cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture && cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture
- [x] **T02: Added the authoritative route-free scaffold equal-surface rail, moved generated-scaffold setup into the shared harness, and narrowed the older scaffold regressions so they fail on drift without reviving deleted HTTP routes.** — Bring the still-live scaffold regression suite onto the same equal-surface story as the scaffold and proof packages so old routeful tests stop blocking the slice and future drift fails closed in one place.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/meshc/tests/support/m046_route_free.rs` shared harness | Fail with retained build/log/CLI artifacts instead of forking another scaffold-specific runtime harness. | Bound waits for startup, continuity discovery, and diagnostics convergence. | Reject malformed `meshc cluster` JSON and missing runtime-name discovery as proof failures. |
| Historical scaffold rails (`e2e_m044_s03.rs`, `e2e_m045_s01.rs`, `e2e_m045_s02.rs`, `e2e_m045_s03.rs`) | Fail fast if they still require `/health`, `/work`, or app-owned submit/status helpers. | Stop on the reused harness timeout instead of retrying through deleted routes. | Treat stale routeful assertions as contract drift rather than carrying compatibility shims forward. |
| New `e2e_m046_s05.rs` equal-surface rail | Fail if the generated scaffold cannot be built, booted, and inspected through the same CLI-only surfaces as the proof packages. | Retain the last status/continuity/diagnostics snapshots and node logs in `.tmp/m046-s05/...`. | Treat malformed retained artifacts or missing continuity list/record linkage as proof failure. |

## Load Profile

- **Shared resources**: temp scaffold project generation, two runtime processes, repeated `meshc cluster` polling, and copied proof bundles under `.tmp/m046-s05/...`.
- **Per-operation cost**: one scaffold generation/build plus repeated `meshc cluster status`, continuity list, continuity record, and diagnostics queries until the startup record is discovered and completed.
- **10x breakpoint**: artifact churn, slow startup convergence, or duplicate startup records will fail the rail long before CPU or memory matter.

## Negative Tests

- **Malformed inputs**: stale routeful scaffold fixtures, missing temp output parents, malformed CLI JSON, or missing `declared_handler_runtime_name` / request-key discovery.
- **Error paths**: historical rails must fail on `/health`, `/work`, `[cluster]`, `Continuity.submit_declared_work(...)`, or request-key-only continuity assumptions instead of masking drift.
- **Boundary conditions**: the scaffold runtime proof must start from continuity list mode, then inspect the discovered record by request key, without inventing a second control plane.

## Steps

1. Extend the shared route-free harness only where needed so a generated scaffold project can be built to a temp output path, booted on two nodes, and inspected through `meshc cluster status|continuity|diagnostics` the same way `tiny-cluster/` and `cluster-proof/` are.
2. Add `compiler/meshc/tests/e2e_m046_s05.rs` to generate a temp clustered scaffold, assert on-disk parity against the proof packages, and prove startup inspection through continuity list mode followed by single-record inspection.
3. Rewrite or narrow `compiler/meshc/tests/e2e_m044_s03.rs`, `compiler/meshc/tests/e2e_m045_s01.rs`, `compiler/meshc/tests/e2e_m045_s02.rs`, and `compiler/meshc/tests/e2e_m045_s03.rs` so they no longer depend on deleted HTTP submit/health behavior and instead either assert the route-free contract directly or delegate to the new equal-surface rail.
4. Keep failures diagnosable by retaining generated scaffold source, build logs, status/continuity/diagnostics JSON, and per-node stdout/stderr in the S05 artifact bundle.

## Must-Haves

- [ ] `compiler/meshc/tests/e2e_m046_s05.rs` exists and proves generated scaffold parity plus CLI-only runtime inspection.
- [ ] Historical scaffold rails no longer require `/health`, `/work`, `Continuity.submit_declared_work(...)`, or `Timer.sleep(...)` in generated code.
- [ ] The shared route-free harness remains the single runtime/CLI proof seam instead of spawning a second bespoke scaffold harness.
- [ ] Failures retain enough `.tmp/m046-s05/...` evidence to localize whether the drift is in generation, startup, continuity discovery, or diagnostics.

## Done When

- [ ] The scaffold regression suite fails closed on routeful drift and passes against the new scaffold contract.
- [ ] The new S05 equal-surface rail proves the scaffold on the same CLI-only surfaces as `tiny-cluster/` and `cluster-proof`.
  - Estimate: 3h
  - Files: compiler/meshc/tests/support/m046_route_free.rs, compiler/meshc/tests/e2e_m044_s03.rs, compiler/meshc/tests/e2e_m045_s01.rs, compiler/meshc/tests/e2e_m045_s02.rs, compiler/meshc/tests/e2e_m045_s03.rs, compiler/meshc/tests/e2e_m046_s05.rs
  - Verify: cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture && cargo test -p meshc --test e2e_m045_s01 m045_s01_ -- --nocapture && cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture && cargo test -p meshc --test e2e_m045_s03 m045_s03_ -- --nocapture && cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture
- [x] **T03: Aligned the scaffold docs, tiny-cluster, and cluster-proof around one route-free clustered-work story and repointed public verifier references to the S05 equal-surface rail.** — Update the public clustered story so the scaffold, `tiny-cluster/`, and `cluster-proof/` all teach the same route-free runtime-owned contract instead of splitting into routeful scaffold docs and route-free proof docs.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| VitePress docs pages and repo `README.md` | Fail `npm --prefix website run build` if rewritten pages break navigation or markdown structure. | Treat a slow docs build as task failure; do not leave the public story half-rewritten. | Treat contradictory routeful and route-free instructions as documentation drift rather than keeping both stories alive. |
| `tiny-cluster/README.md` and `cluster-proof/README.md` runbooks | Fail docs/content guards if the proof-package READMEs keep diverging on continuity list vs single-record inspection. | N/A | Treat request-key-only continuity guidance as incomplete operator documentation. |

## Negative Tests

- **Malformed inputs**: stale references to `[cluster]`, `Continuity.submit_declared_work(...)`, `/health`, `/work`, `Timer.sleep(...)` failover edits, or old verifier names as current truth.
- **Error paths**: the docs must explicitly reject app-owned proof/status routes instead of silently omitting them.
- **Boundary conditions**: the three surfaces may keep scope-specific notes, but they must share the same canonical operator flow: status, continuity list, continuity record, diagnostics.

## Steps

1. Rewrite `website/docs/docs/getting-started/clustered-example/index.md` and `website/docs/docs/tooling/index.md` around the new scaffold output: package-only manifest, source-owned `clustered(work)`, automatic startup work, and CLI-only inspection.
2. Rewrite `website/docs/docs/distributed-proof/index.md`, `website/docs/docs/distributed/index.md`, and repo `README.md` so they present the scaffold, `tiny-cluster/`, and `cluster-proof/` as equal canonical surfaces and point at the authoritative S05 verifier rather than the old S04 wrapper story.
3. Align `tiny-cluster/README.md` and `cluster-proof/README.md` on the same operator sequence: `meshc cluster status`, continuity list, continuity record, then diagnostics.
4. Remove the last routeful tutorial language (`[cluster]`, HTTP submit/health routes, delay-edit failover steps) and keep all cross-links pointed at the route-free clustered story.

## Must-Haves

- [ ] The public docs no longer teach `[cluster]`, HTTP submit/status routes, or proof-only delay edits as part of the clustered-example story.
- [ ] The scaffold, `tiny-cluster/`, and `cluster-proof/` are all named as equally canonical clustered-example surfaces.
- [ ] Every owned/package README that talks about inspection uses the same continuity list-then-record workflow.
- [ ] The docs/reference surfaces point at `scripts/verify-m046-s05.sh` as the authoritative closeout rail and keep `scripts/verify-m045-s05.sh` clearly historical.

## Done When

- [ ] The docs build cleanly and no slice-owned docs page still teaches the deleted routeful clustered contract.
- [ ] A reader following any of the three surface runbooks would learn the same runtime-owned clustered story.
  - Estimate: 2h
  - Files: website/docs/docs/getting-started/clustered-example/index.md, website/docs/docs/tooling/index.md, website/docs/docs/distributed-proof/index.md, website/docs/docs/distributed/index.md, README.md, tiny-cluster/README.md, cluster-proof/README.md
  - Verify: npm --prefix website run build && ! rg -n "\[cluster\]|Continuity\.submit_declared_work|/health|/work/:request_key|Timer\.sleep\(5000\)" website/docs/docs/getting-started/clustered-example/index.md website/docs/docs/tooling/index.md website/docs/docs/distributed-proof/index.md website/docs/docs/distributed/index.md README.md
- [x] **T04: Added the authoritative S05 closeout verifier and repointed the historical M045 wrapper to delegate to it with one retained proof-bundle chain.** — Finish the slice with one fail-closed verifier that replays the route-free proof surfaces, scaffold alignment rails, and docs build, then make the old M045 closeout wrapper a thin alias to that truthful S05 rail.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m046-s05.sh` direct verifier | Fail on the first replay/build/docs error and keep phase/status/current-phase/latest-proof-bundle artifacts plus command logs. | Bound delegated verifier and test phases; do not report success if any replay hangs. | Treat missing phase files, missing retained bundles, or malformed artifact pointers as verifier failures. |
| `scripts/verify-m045-s05.sh` historical wrapper | Fail closed if it still delegates only to S04 or if it omits the new docs/scaffold alignment phases. | Let the delegated S05 timeout propagate instead of masking it as a historical no-op. | Treat missing retained S05 artifacts as alias drift. |
| `compiler/meshc/tests/e2e_m045_s05.rs` and `compiler/meshc/tests/e2e_m046_s05.rs` content guards | Fail fast when wrapper/script assertions drift away from the authoritative S05 phase names, retained bundle shape, or doc/scaffold scope. | N/A | Reject stale S04-only expectations and malformed verifier references as contract failures. |

## Load Profile

- **Shared resources**: delegated S03/S04 verifiers, fast scaffold tests, docs build, route-free runtime rails, and copied `.tmp/m046-s05/verify` artifacts.
- **Per-operation cost**: one full closeout replay including Rust tests, docs build, and artifact copy/shape checks.
- **10x breakpoint**: verifier artifact churn and replay timeouts will fail first; this task is about proof-surface integrity rather than runtime throughput.

## Negative Tests

- **Malformed inputs**: missing `status.txt`, `current-phase.txt`, `phase-report.txt`, `latest-proof-bundle.txt`, or copied retained bundles.
- **Error paths**: stale wrapper delegation to `verify-m046-s04.sh`, missing docs-build phase, or verifier scripts that stop checking scaffold/docs alignment.
- **Boundary conditions**: the historical alias may stay, but it must clearly depend on the authoritative S05 verifier and not revive deleted routeful/product-story checks.

## Steps

1. Add `scripts/verify-m046-s05.sh` as the direct equal-surface verifier: replay the focused scaffold tests, the route-free proof regressions, the new `e2e_m046_s05` rail, the docs build, and retained-artifact copy/shape checks.
2. Repoint `scripts/verify-m045-s05.sh` so it delegates to `scripts/verify-m046-s05.sh`, retains the delegated verify directory locally, and fails closed on missing S05 phase/bundle artifacts.
3. Update `compiler/meshc/tests/e2e_m045_s05.rs` (and `compiler/meshc/tests/e2e_m046_s05.rs` if needed) so the Rust-side contract assertions pin the new S05 verifier phases, retained artifacts, and docs/scaffold scope.
4. Preserve the S03/S04 retained bundle chain so S06 can assemble milestone-wide replay from one truthful closeout seam.

## Must-Haves

- [ ] `scripts/verify-m046-s05.sh` is the authoritative verifier for scaffold/docs/proof alignment.
- [ ] `scripts/verify-m045-s05.sh` becomes a thin historical alias that retains and checks S05 verifier artifacts instead of reasserting S04-only truth.
- [ ] Rust content-guard tests pin the new S05 verifier shape and fail on stale S04-only expectations.
- [ ] The verifier emits retained S05 phase/status/current-phase/latest-proof-bundle artifacts that future slices can inspect directly.

## Done When

- [ ] The direct S05 verifier passes and the historical alias passes only by delegating to it.
- [ ] The retained S05 verify directory is sufficient for a future agent to diagnose which replay phase drifted.
  - Estimate: 2h
  - Files: scripts/verify-m046-s05.sh, scripts/verify-m045-s05.sh, compiler/meshc/tests/e2e_m045_s05.rs, compiler/meshc/tests/e2e_m046_s05.rs
  - Verify: cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture && bash scripts/verify-m046-s05.sh && bash scripts/verify-m045-s05.sh
