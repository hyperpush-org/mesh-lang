# S04: Hard cutover and dogfood migration

**Goal:** End the public compatibility bridge by removing `clustered(work)` / `[cluster]` from the supported clustered authoring model, migrate repo-owned scaffold and proof packages onto `@cluster`, and ship one M047-owned cutover rail plus docs guidance that teach a single source-first route-free clustered story without pretending `HTTP.clustered(...)` already exists.
**Demo:** After this: After this: `tiny-cluster/`, `cluster-proof/`, generated clustered surfaces, and repo proof rails no longer teach `clustered(work)` or `.toml` clustering as the public model.

## Tasks
- [x] **T01: Hard-cut legacy clustered declarations so only `@cluster` stays supported in parser, manifest loading, compiler e2e, and LSP diagnostics.** — Finish the bridge D268 explicitly left in place for S04. Remove `clustered(work)` and manifest `[cluster]` as supported clustered-definition inputs, keep `@cluster` / `@cluster(N)` as the only accepted authoring surface, and turn legacy cases into explicit migration-oriented parser/pkg/compiler diagnostics instead of quiet compatibility.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| parser recovery + AST clustered-declaration extraction | emit a source-local cutover diagnostic and stop before building clustered metadata | N/A | reject mixed or partial legacy tokens instead of synthesizing a clustered declaration |
| mesh-pkg manifest loading / validation | fail manifest parsing with an explicit `[cluster]` migration error and do not generate clustered execution metadata | N/A | treat partial cluster config as invalid input, not as a fallback declaration source |
| compiler/LSP clustered diagnostics | keep errors anchored on the real source or manifest location instead of collapsing to unrelated project-level noise | N/A | prefer one explicit cutover message over cascading legacy-compat follow-on errors |

## Load Profile

- **Shared resources**: compiler parse/validation diagnostic buffers and clustered export-surface construction.
- **Per-operation cost**: one parse plus one manifest/source validation pass per package; no network or runtime work.
- **10x breakpoint**: large mixed-source packages will amplify cascading diagnostic spam first, so the cutover path should keep errors bounded and source-local.

## Negative Tests

- **Malformed inputs**: `clustered(work)` before `fn|def`, mixed `@cluster` + legacy declarations, stale `[cluster]` manifest sections, and malformed legacy target entries.
- **Error paths**: source-only builds using the old syntax fail before codegen, manifest-only legacy declarations fail with migration guidance, and compiler/LSP ranges still point at the right source after compatibility removal.
- **Boundary conditions**: valid `@cluster`, valid `@cluster(3)`, and existing private/decorated validation failures remain source-ranged and truthful after the hard cut.

## Steps

1. Remove the legacy parser/AST clustered declaration acceptance path and replace it with explicit cutover diagnostics that steer users toward `@cluster` / `@cluster(N)`.
2. Delete manifest `[cluster]` declaration support from mesh-pkg validation/export-surface construction and replace it with fail-closed migration guidance.
3. Update compiler/LSP rails that still depend on legacy provenance or wording so the only supported clustered-definition model is source-first.
4. Add parser/pkg/compiler regression cases named for `m047_s04` so the hard cut stays provable instead of being a one-off grep expectation.

## Must-Haves

- [ ] `clustered(work)` no longer produces a valid clustered declaration in parser/pkg/compiler flows.
- [ ] `[cluster]` manifest declarations fail closed with explicit migration guidance and no fallback clustered metadata.
- [ ] `@cluster` / `@cluster(N)` diagnostics remain source-ranged and truthful after the compatibility path is removed.
  - Estimate: 3h
  - Files: compiler/mesh-parser/src/parser/items.rs, compiler/mesh-parser/src/ast/item.rs, compiler/mesh-parser/tests/parser_tests.rs, compiler/mesh-pkg/src/manifest.rs, compiler/meshc/tests/e2e_m047_s01.rs, compiler/mesh-lsp/src/analysis.rs
  - Verify: cargo test -p mesh-parser m047_s04 -- --nocapture && cargo test -p mesh-pkg m047_s04 -- --nocapture && cargo test -p meshc --test e2e_m047_s01 -- --nocapture
- [x] **T02: Switched `meshc init --clustered` to emit `@cluster` source-first work declarations and tightened scaffold contract tests.** — Update the generated clustered scaffold so `meshc init --clustered` emits the source-first contract the repo now expects to dogfood. Keep the route-free `main.mpl` bootstrap path, keep the visible work body minimal, and preserve runtime-name continuity by keeping the function name `execute_declared_work` instead of keeping the old helper.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| scaffold file generation | fail the init command instead of creating a half-migrated project tree | N/A | never emit mixed `@cluster` + legacy helper output |
| scaffold contract tests | fail loudly on text drift rather than letting generated files silently regress | N/A | treat mismatched generated content as contract breakage, not a soft warning |
| runtime-name continuity assumption | keep the function name stable so downstream route-free rails still see `Work.execute_declared_work` without a helper | N/A | do not invent a second runtime-name helper surface while migrating the syntax |

## Load Profile

- **Shared resources**: temp project directories and textual scaffold fixtures asserted by unit/tooling tests.
- **Per-operation cost**: one project generation plus a handful of file reads/assertions; no long-lived runtime state.
- **10x breakpoint**: contract drift across generated files and tests breaks faster than performance does; correctness matters more than throughput here.

## Negative Tests

- **Malformed inputs**: existing project directory collisions and clustered init reruns still fail cleanly.
- **Error paths**: generated `mesh.toml` must stay package-only and generated `work.mpl` must omit the old helper/legacy syntax completely.
- **Boundary conditions**: generated README still describes runtime-owned inspection, generated `main.mpl` stays route-free, and generated `work.mpl` preserves `1 + 1` plus `execute_declared_work` naming.

## Steps

1. Rewrite the clustered scaffold template in `scaffold.rs` to emit `@cluster pub fn execute_declared_work(...)` with no `declared_work_runtime_name()` helper and no manifest clustering text.
2. Update the generated README copy so it teaches the new source-first route-free contract and preserves the runtime-owned inspection commands.
3. Update mesh-pkg scaffold tests and `tooling_e2e` expectations so `meshc init --clustered` becomes the canonical source-first generator surface.
4. Keep the scaffold contract intentionally narrow: no `HTTP.clustered(...)`, no app-owned routes, and no extra helper seams added during the migration.

## Must-Haves

- [ ] `meshc init --clustered` generates `@cluster` source, not `clustered(work)` or `declared_work_runtime_name()`.
- [ ] Generated scaffold output still keeps `main.mpl` route-free and runtime-owned via `Node.start_from_env()`.
- [ ] Generated README text matches the new source-first model without claiming the missing HTTP route wrapper exists.
  - Estimate: 2h
  - Files: compiler/mesh-pkg/src/scaffold.rs, compiler/meshc/tests/tooling_e2e.rs
  - Verify: cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture && cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture
- [x] **T03: Migrated tiny-cluster and cluster-proof to the @cluster source-first contract and hardened their package smoke tests against legacy markers.** — Migrate the canonical route-free example packages together so the repo stops teaching one thing in parser/scaffold land and another thing in package land. Both packages should use the same `@cluster`-based `execute_declared_work` shape, keep the visible `1 + 1` body, and continue proving the general-function clustered model without inventing HTTP-route claims.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| package source + package test parity | fail package tests and build smoke instead of leaving the examples subtly divergent | N/A | do not keep one package on `@cluster` and the other on `clustered(work)` |
| README contract wording | treat stale wording as a contract failure because the examples are part of the public model | N/A | reject README/package mismatches instead of documenting two clustered stories |
| stale `tiny-cluster-prefered` prior art | remove or rewrite the contradictory manifest surface so it stops competing with the canonical examples | N/A | do not leave an obsolete manifest example named "prefered" in-tree |

## Load Profile

- **Shared resources**: repo-owned example files, package test fixtures, and build outputs.
- **Per-operation cost**: package build + package tests for two tiny route-free apps.
- **10x breakpoint**: textual drift across the paired packages is the first failure mode; runtime load is negligible.

## Negative Tests

- **Malformed inputs**: package tests should fail if `clustered(work)`, `declared_work_runtime_name()`, or `[cluster]` reappear in the dogfood surfaces.
- **Error paths**: route-free builds/tests must stay green without any app-owned `/work`/`/status`/`/health` routes or helper-managed submit flows.
- **Boundary conditions**: both packages preserve `execute_declared_work`, visible `1 + 1`, route-free `main.mpl`, and runtime-owned continuity inspection text.

## Steps

1. Rewrite `tiny-cluster/work.mpl` and `cluster-proof/work.mpl` to the shared `@cluster` form while preserving the function name that keeps the runtime registration name stable.
2. Update both package READMEs and package tests so they assert the new source-first contract and still reject app-owned route/proof seams.
3. Clean up `tiny-cluster-prefered/mesh.toml` so it no longer teaches stale manifest clustering or contradictory count syntax.
4. Replay package build/test smoke on both packages so the dogfood surfaces stay truthful, route-free, and byte-level consistent where the harness expects it.

## Must-Haves

- [ ] `tiny-cluster/` and `cluster-proof/` both dogfood `@cluster` instead of `clustered(work)`.
- [ ] Package tests/readmes reject the old helper/manifest story and keep the route-free runtime-owned inspection contract.
- [ ] `tiny-cluster-prefered/` stops teaching obsolete manifest clustering as if it were preferred current practice.
  - Estimate: 2h
  - Files: tiny-cluster/work.mpl, tiny-cluster/tests/work.test.mpl, tiny-cluster/README.md, cluster-proof/work.mpl, cluster-proof/tests/work.test.mpl, cluster-proof/README.md, tiny-cluster-prefered/mesh.toml
  - Verify: cargo run -q -p meshc -- test tiny-cluster/tests && cargo run -q -p meshc -- build tiny-cluster && cargo run -q -p meshc -- test cluster-proof/tests && cargo run -q -p meshc -- build cluster-proof
- [x] **T04: Rewired the historical route-free rails to the shared `@cluster` source-first contract.** — Once the scaffold and package surfaces move, the old route-free harness and historical exact-string rails need to stop pinning the M046 wording. Update them so they keep proving runtime-name continuity and route-free bootstrap truth, but now assert the source-first contract instead of the old helper/marker text.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| shared `m046_route_free` harness | fail with retained artifacts that show which generated/package file drifted instead of masking the cutover behind generic assertions | preserve existing bounded waits and artifact retention so false timeouts stay debuggable | treat missing or malformed continuity/status payloads as proof drift, not as soft skips |
| historical e2e file-content assertions | fail closed when stale `clustered(work)`/helper text remains or when runtime-name continuity disappears | N/A | keep assertions specific enough to distinguish wording drift from runtime-name/runtime-surface drift |
| retained artifact shape | keep bundle pointers and copied package outputs stable so later verifiers can still localize failures | bounded waits stay the same; do not add slower or broader probes unnecessarily | malformed retained bundle pointers should stay explicit failures rather than being ignored |

## Load Profile

- **Shared resources**: temp workspaces, retained `.tmp` artifact trees, spawned route-free test processes, and CLI continuity/status polling.
- **Per-operation cost**: one scaffold generation/build smoke path plus several file-content and artifact-shape assertions across historical e2e targets.
- **10x breakpoint**: `.tmp` artifact churn and duplicated build smoke dominate before CPU does; preserve the existing shared helper rather than multiplying harnesses.

## Negative Tests

- **Malformed inputs**: stale generated files containing `clustered(work)` or the helper, malformed bundle pointers, and missing copied artifact directories.
- **Error paths**: continuity/status probes must still fail closed when runtime truth drifts, and historical tests must not silently downgrade to zero-test or no-op checks.
- **Boundary conditions**: runtime-name continuity stays `Work.execute_declared_work`, route-free `Node.start_from_env()` remains the only bootstrap path, and no historical rail starts teaching `HTTP.clustered(...)`.

## Steps

1. Update `compiler/meshc/tests/support/m046_route_free.rs` so generated/package file assertions expect the `@cluster` source-first work contract while preserving retained artifact behavior.
2. Rewrite the historical M045/M046 e2e file-content assertions to the new source-first wording, keeping runtime-name continuity and route-free bootstrap expectations intact.
3. Preserve the existing artifact and CLI continuity checks so failures still localize to scaffold/package/runtime truth instead of to generic text mismatches.
4. Replay the named historical e2e filters so the cutover proves itself on the same rails users and later slices already rely on.

## Must-Haves

- [ ] Shared route-free harness assertions now expect `@cluster`-based work sources and new README wording.
- [ ] Historical M045/M046 e2e rails still prove runtime-name continuity and route-free runtime truth without pinning the old helper/marker text.
- [ ] Retained artifact shapes and failure localization stay stable enough for later wrapper scripts to reuse.

  - Estimate: 3h
  - Files: compiler/meshc/tests/support/m046_route_free.rs, compiler/meshc/tests/e2e_m045_s01.rs, compiler/meshc/tests/e2e_m045_s02.rs, compiler/meshc/tests/e2e_m045_s03.rs, compiler/meshc/tests/e2e_m046_s03.rs, compiler/meshc/tests/e2e_m046_s04.rs, compiler/meshc/tests/e2e_m046_s05.rs, compiler/meshc/tests/e2e_m046_s06.rs
  - Verify: cargo test -p meshc --test e2e_m045_s01 m045_s01_ -- --nocapture && cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture && cargo test -p meshc --test e2e_m045_s03 m045_s03_ -- --nocapture && cargo test -p meshc --test e2e_m046_s03 m046_s03_ -- --nocapture && cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture && cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture && cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture
- [x] **T05: Rewrote the public clustered docs to the source-first `@cluster` model and repaired startup failover recovery so the historical cutover rails stay green.** — Update the public docs surface so new and existing users see one clustered model: route-free ordinary clustered functions are declared with `@cluster`, the generated scaffold and proof packages all share that contract, migration off `clustered(work)` / `[cluster]` is explicit, and the docs do not over-claim the unshipped `HTTP.clustered(...)` wrapper.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| VitePress content build | fail the docs build rather than shipping broken markdown/config references | bounded by the docs build; do not add extra long-running site tasks here | malformed code fences or links should fail build/link checks instead of silently rendering bad guidance |
| README / docs parity | treat contradictory wording as a public-contract regression | N/A | do not leave one page teaching legacy syntax while another page teaches the new model |
| migration guidance wording | keep the old surface clearly marked as legacy migration context, not as a coequal supported syntax | N/A | ambiguous wording should be treated as drift because this slice exists to cut ambiguity down |

## Load Profile

- **Shared resources**: markdown pages and the VitePress site build output.
- **Per-operation cost**: one site build plus markdown/code-fence validation.
- **10x breakpoint**: build failures from broken links/code fences appear before any meaningful performance issue.

## Negative Tests

- **Malformed inputs**: stale code blocks still showing `clustered(work)` or manifest clustering as current practice.
- **Error paths**: docs must explicitly say the HTTP route wrapper is not shipped yet instead of implying it exists.
- **Boundary conditions**: generated scaffold, `tiny-cluster`, and `cluster-proof` are all described as the same route-free source-first contract, with explicit migration language for existing users.

## Steps

1. Rewrite the clustered example, tooling, distributed proof, distributed overview, and top-level README text/code samples to the `@cluster` route-free contract.
2. Add explicit migration guidance off `clustered(work)` / `[cluster]` for existing users instead of simply deleting the old words with no explanation.
3. Make the docs truthful about current scope: ordinary clustered functions and route-free startup work are shipped; `HTTP.clustered(...)` is still not.
4. Rebuild the docs site so broken code fences, links, or page references fail before the verifier layer tries to reuse the pages.

## Must-Haves

- [ ] Public docs and README teach `@cluster` as the clustered source surface and explain migration off the old model.
- [ ] No public page claims `HTTP.clustered(...)` already shipped.
- [ ] Scaffold/example/proof-package wording is consistent across README and VitePress pages.

  - Estimate: 2h
  - Files: README.md, website/docs/docs/getting-started/clustered-example/index.md, website/docs/docs/tooling/index.md, website/docs/docs/distributed-proof/index.md, website/docs/docs/distributed/index.md
  - Verify: npm --prefix website run build
- [x] **T06: Added the M047 cutover verifier, repointed M045/M046 wrapper scripts to it, and updated docs/tests so the source-first `@cluster` story has one authoritative rail.** — Ship one M047-owned acceptance rail for the hard cutover, then make the historical M045/M046 wrapper scripts defer to it instead of continuing to assert the obsolete model directly. The new rail should replay the parser/pkg/scaffold/package/docs proofs, retain one coherent `.tmp/m047-s04/...` bundle, and keep historical script names usable as compatibility aliases only.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| new `m047_s04` verifier composition | stop at the first failing phase with a retained log and status/current-phase markers | kill hung subprocesses and mark the failing phase explicitly | malformed bundle pointers or phase reports should fail closed |
| new `e2e_m047_s04` contract rail | fail loudly when the assembled source-first contract drifts instead of depending on doc-only greps | bounded test timeouts must still leave retained artifacts | zero-test or missing-target output must be treated as a real proof gap |
| historical wrapper scripts | delegate to the new rail and preserve compatibility names, not duplicate the old assertions | delegated wrapper runs should preserve the delegated verifier's explicit timeout/failure status | malformed retained-delegation state should stay an explicit failure |

## Load Profile

- **Shared resources**: retained `.tmp` verifier bundles, delegated script runs, package build/test smoke, and docs build phases composed under one rail.
- **Per-operation cost**: one assembled verifier replay plus targeted script/e2e contract checks.
- **10x breakpoint**: duplicated wrapper composition and artifact copying become the main cost first, so old scripts should delegate instead of re-running bespoke cutover logic.

## Negative Tests

- **Malformed inputs**: zero-test outputs, missing retained bundle pointers, malformed phase reports, and stale script references to the old verifier names.
- **Error paths**: failing subrails must stop the assembled verifier with a retained phase marker, and historical wrappers must fail when delegated state is malformed instead of silently passing.
- **Boundary conditions**: the new rail still proves parser/pkg/scaffold/package/docs cutover together, and historical script names remain replayable aliases rather than disappearing outright.

## Steps

1. Add `compiler/meshc/tests/e2e_m047_s04.rs` to prove the assembled cutover contract at the Rust e2e layer, reusing the route-free harness where that keeps artifact retention honest.
2. Add `scripts/verify-m047-s04.sh` to replay the parser/pkg/scaffold/package/docs proofs, retain one coherent M047 bundle, and fail closed on malformed phase output or zero-test filters.
3. Repoint the historical M045/M046 wrapper scripts to delegate to the new M047 rail while preserving retained alias semantics instead of teaching the obsolete model directly.
4. Update the script-focused e2e rails so they assert the new delegation graph and public verifier story instead of the old M046/M045 one.

## Must-Haves

- [ ] `scripts/verify-m047-s04.sh` becomes the authoritative local cutover rail for S04.
- [ ] Historical M045/M046 wrapper scripts remain replayable but delegate to the new M047 rail instead of reasserting the old model.
- [ ] Rust e2e coverage proves the new verifier graph and fail-closed artifact/bundle behavior.
  - Estimate: 3h
  - Files: compiler/meshc/tests/e2e_m047_s04.rs, compiler/meshc/tests/e2e_m045_s04.rs, compiler/meshc/tests/e2e_m045_s05.rs, scripts/verify-m047-s04.sh, scripts/verify-m045-s04.sh, scripts/verify-m045-s05.sh, scripts/verify-m046-s04.sh, scripts/verify-m046-s05.sh, scripts/verify-m046-s06.sh
  - Verify: cargo test -p meshc --test e2e_m047_s04 -- --nocapture && cargo test -p meshc --test e2e_m045_s04 -- --nocapture && cargo test -p meshc --test e2e_m045_s05 -- --nocapture && bash scripts/verify-m047-s04.sh
