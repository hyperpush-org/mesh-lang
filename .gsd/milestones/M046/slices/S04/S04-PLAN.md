# S04: Rebuild `cluster-proof/` as tiny packaged proof

**Goal:** Rebuild `cluster-proof/` as a tiny packaged route-free proof app that matches the same source-level `clustered(work)` + `Node.start_from_env()` contract as `tiny-cluster/`, while proving the packaged path entirely through Mesh-owned CLI/runtime surfaces.
**Demo:** After this: After this: `cluster-proof/` is a packaged route-free proof app on the same tiny `1 + 1` clustered contract instead of a legacy proof app with its own trigger/status layers.

## Tasks
- [x] **T01: Reset `cluster-proof/` to a route-free source-owned clustered contract and aligned the package smoke rail with the deleted legacy modules.** — Delete the legacy proof-app source and make `cluster-proof/` match the tiny route-free package shape so app code only denotes clustered work and delegates startup/inspection to Mesh-owned surfaces.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `cluster-proof/mesh.toml` declaration surface | Fail `meshc build cluster-proof` if `[cluster]` or duplicate declaration state remains. | N/A | Treat any mixed manifest+source declaration state as a contract error instead of silently preferring one. |
| `Node.start_from_env()` bootstrap path in `cluster-proof/main.mpl` | Fail compile/build if deleted modules or route imports survive. | Runtime startup should exit with the existing fail-closed bootstrap error instead of spinning up a partial process. | Do not parse or reshape bootstrap data in package code; surface the runtime error directly. |
| Legacy module deletion (`cluster.mpl`, `config.mpl`, `work_continuity.mpl`) | Fail fast on stale imports or references rather than keeping thin wrappers around deleted seams. | N/A | N/A |

## Load Profile

- **Shared resources**: One runtime-owned startup registration/continuity path plus package bootstrap logs.
- **Per-operation cost**: One trivial declared-work execution returning `2` and no app-owned HTTP/control plane.
- **10x breakpoint**: Duplicate declaration drift or repeated startup boot loops will break proof truth long before CPU or memory matter.

## Negative Tests

- **Malformed inputs**: Mixed manifest+source declaration state, missing deleted modules, or renamed runtime handler strings.
- **Error paths**: Any surviving `HTTP.serve(...)`, `/work`, `/membership`, `Continuity.*`, or package-owned env/timing helpers must fail the source contract.
- **Boundary conditions**: `cluster-proof/work.mpl` keeps exactly one `clustered(work)` declaration and returns `1 + 1` while the runtime name stays `Work.execute_declared_work`.

## Steps

1. Rewrite `cluster-proof/mesh.toml` to a package-only manifest and rewrite `cluster-proof/work.mpl` to own the single source `clustered(work)` declaration plus `declared_work_runtime_name()`.
2. Replace `cluster-proof/main.mpl` with the tiny route-free bootstrap shape: call `Node.start_from_env()`, log success/failure, and omit `HTTP.serve(...)`, routes, and continuity imports.
3. Delete `cluster-proof/cluster.mpl`, `cluster-proof/config.mpl`, and `cluster-proof/work_continuity.mpl`, and remove every remaining source reference to them.
4. Keep the declared handler runtime name stable (`Work.execute_declared_work`) so S02/S03 runtime-owned startup discovery still matches the packaged proof.

## Must-Haves

- [ ] `cluster-proof/mesh.toml` no longer contains `[cluster]` or `declarations`.
- [ ] `cluster-proof/main.mpl` contains exactly one `Node.start_from_env()` boot path and no app-owned routes or continuity calls.
- [ ] `cluster-proof/work.mpl` contains exactly one `clustered(work)` declaration, keeps the runtime name `Work.execute_declared_work`, and returns `1 + 1`.
- [ ] `cluster-proof/cluster.mpl`, `cluster-proof/config.mpl`, and `cluster-proof/work_continuity.mpl` are deleted rather than preserved as thin wrappers.
  - Estimate: 2h
  - Files: cluster-proof/mesh.toml, cluster-proof/main.mpl, cluster-proof/work.mpl, cluster-proof/cluster.mpl, cluster-proof/config.mpl, cluster-proof/work_continuity.mpl
  - Verify: cargo run -q -p meshc -- build cluster-proof && test ! -e cluster-proof/cluster.mpl && test ! -e cluster-proof/config.mpl && test ! -e cluster-proof/work_continuity.mpl
- [x] **T02: Locked `cluster-proof`’s route-free package contract in smoke tests, README, Dockerfile, and Fly config.** — Lock the new package shape in smoke tests/readme and make Docker/Fly honest about a route-free binary by removing the HTTP entrypoint story instead of preserving fake probes.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `cluster-proof/tests/work.test.mpl` package smoke rail | Fail fast on source/readme/packaging drift instead of punting the first truthful failure to the Rust e2e rail. | N/A | Treat unexpected route, delay, or proxy strings as a contract failure. |
| `cluster-proof/Dockerfile` runtime image | Fail `docker build` if the runtime stage still depends on a deleted entrypoint or HTTP-only env. | Docker build timeout should fail the task instead of keeping an unverified packaging story. | Treat missing built binary or wrong copied paths as image-contract failures. |
| `cluster-proof/fly.toml` packaged deployment contract | Fail closed if `http_service`, `PORT`, or proxy-only assumptions remain. | N/A | Treat malformed or contradictory process config as packaging drift rather than patching it in docs. |

## Load Profile

- **Shared resources**: One packaged binary, one Docker image build, and package-level file-content guards.
- **Per-operation cost**: `meshc test cluster-proof/tests` plus one multi-stage Docker build of the repo image.
- **10x breakpoint**: Image bloat or proxy misconfiguration will break the proof before runtime compute cost matters.

## Negative Tests

- **Malformed inputs**: Packaging/readme drift that reintroduces `/membership`, `/work`, `CLUSTER_PROOF_WORK_DELAY_MS`, `PORT`, `http_service`, or `docker-entrypoint.sh`.
- **Error paths**: `cluster-proof/tests/work.test.mpl` must fail if README or packaging files point operators back to old route/Fly HTTP behavior.
- **Boundary conditions**: The packaged story may stay deeper/provisional, but it must still be route-free and honest about having no app-owned HTTP surface.

## Steps

1. Rewrite `cluster-proof/tests/work.test.mpl` to mirror the `tiny-cluster/` smoke style while adding route-free guards for `README.md`, `Dockerfile`, and `fly.toml`.
2. Delete `cluster-proof/tests/config.test.mpl` and move any remaining contract coverage into `work.test.mpl` so package proof lives on the new tiny surface only.
3. Rewrite `cluster-proof/README.md` around the packaged route-free contract: `clustered(work)`, `Node.start_from_env()`, `meshc cluster status|continuity|diagnostics`, and explicit rejection of app-owned routes/timing seams.
4. Simplify `cluster-proof/Dockerfile` and `cluster-proof/fly.toml` so the image copies only the built binary, drops `docker-entrypoint.sh`, and removes fake HTTP proxy requirements from the packaged contract.

## Must-Haves

- [ ] `cluster-proof/tests/work.test.mpl` fails closed on reintroduced routes, delay knobs, or fake Fly HTTP packaging.
- [ ] `cluster-proof/tests/config.test.mpl` is deleted instead of preserved for removed modules.
- [ ] `cluster-proof/README.md` points operators at `meshc cluster status|continuity|diagnostics` and does not mention `/membership`, `/work`, `mesh-cluster-proof.fly.dev`, or `CLUSTER_PROOF_WORK_DELAY_MS`.
- [ ] `cluster-proof/Dockerfile` no longer copies or executes `cluster-proof/docker-entrypoint.sh`, and `cluster-proof/fly.toml` no longer declares `http_service` or `PORT`.
  - Estimate: 2h
  - Files: cluster-proof/tests/work.test.mpl, cluster-proof/tests/config.test.mpl, cluster-proof/README.md, cluster-proof/Dockerfile, cluster-proof/docker-entrypoint.sh, cluster-proof/fly.toml, tiny-cluster/tests/work.test.mpl, tiny-cluster/README.md
  - Verify: cargo run -q -p meshc -- test cluster-proof/tests && docker build -f cluster-proof/Dockerfile -t mesh-cluster-proof:m046-s04-local .
- [x] **T03: Added shared route-free package e2e support and a packaged cluster-proof CLI/runtime proof rail with retained .tmp/m046-s04 evidence.** — Prove the rebuilt package end to end with a dedicated M046/S04 route-free rail, extracting only the shared M046 helper layer needed to keep `tiny-cluster/` and `cluster-proof/` on the same CLI/runtime proof surfaces.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Temp-path `meshc build ... --output` flow | Fail before launch if the output parent directory was not created or the temp binary is missing. | N/A | Treat missing temp output metadata as a proof-harness failure instead of silently building in-place and churning tracked binaries. |
| Shared helper extraction from `e2e_m046_s03.rs` | Fail the S03 regression rail and archive the last artifacts instead of forking two near-duplicate helper stacks. | Bound the reused waits and record the last observed CLI state from both packages. | Reject malformed CLI JSON or missing required fields in the shared parser helpers. |
| `meshc cluster status|continuity|diagnostics` package proof queries | Fail on `target_not_connected`, missing runtime-name discovery, or duplicate records rather than probing app routes. | Archive the last CLI JSON and node logs for the failed wait. | Treat malformed JSON and missing `declared_handler_runtime_name` as proof failures. |

## Load Profile

- **Shared resources**: Two long-running node processes, repeated CLI polling, copied proof bundles under `.tmp/m046-s04/...`, and one shared Rust helper module used by S03 and S04.
- **Per-operation cost**: Temp build of the package binary plus repeated `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics` queries until one logical startup record completes.
- **10x breakpoint**: Artifact churn, slow convergence, or duplicate startup records will fail the rail before test-runner CPU becomes interesting.

## Negative Tests

- **Malformed inputs**: Missing temp output parent directories, package drift that reintroduces routes or manifest declarations, or malformed CLI JSON.
- **Error paths**: Missing runtime-name discovery, duplicate startup execution, or absent diagnostics must fail with retained raw artifacts.
- **Boundary conditions**: The package proof must stay route-free, use `Work.execute_declared_work`, and keep tracked binaries untouched by building to a temp output path.

## Steps

1. Extract only the reusable M046 route-free build/spawn/CLI JSON wait helpers from `compiler/meshc/tests/e2e_m046_s03.rs` into `compiler/meshc/tests/support/` and update S03 to import them.
2. Add `compiler/meshc/tests/e2e_m046_s04.rs` that archives the `cluster-proof/` source/package files, pre-creates a temp output parent directory, builds `cluster-proof` to a temp binary, boots two nodes, and discovers the startup record by `declared_handler_runtime_name == "Work.execute_declared_work"` through `meshc cluster continuity` list mode.
3. Assert the packaged route-free proof completes and is inspectable only through `meshc cluster status|continuity|diagnostics`, retaining build logs plus per-node stdout/stderr and CLI JSON under `.tmp/m046-s04/...`.
4. Add `scripts/verify-m046-s04.sh` as the direct slice verifier and rerun the focused S03 route-free rail so helper reuse does not regress `tiny-cluster/`.

## Must-Haves

- [ ] Reused helpers live under `compiler/meshc/tests/support/` and do not create a standalone integration-test target.
- [ ] `compiler/meshc/tests/e2e_m046_s04.rs` builds `cluster-proof` to a temp output path whose parent directory is created up front.
- [ ] The packaged proof uses only Mesh-owned CLI/runtime surfaces; no `/membership`, `/work`, `/status`, or Fly HTTP probe is added back.
- [ ] Failures retain build logs, CLI JSON, and node stdout/stderr under `.tmp/m046-s04/...`, and the S03 route-free rail still passes after helper extraction.
  - Estimate: 3h
  - Files: compiler/meshc/tests/support/mod.rs, compiler/meshc/tests/support/m046_route_free.rs, compiler/meshc/tests/e2e_m046_s03.rs, compiler/meshc/tests/e2e_m046_s04.rs, scripts/verify-m046-s04.sh, cluster-proof/mesh.toml, cluster-proof/main.mpl, cluster-proof/work.mpl
  - Verify: cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_ -- --nocapture && cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture && bash scripts/verify-m046-s04.sh
- [x] **T04: Repointed the stale M044/M045 cluster-proof wrapper rails at the M046 route-free packaged verifier and removed the deleted routeful contract checks.** — Retire the still-live M044/M045 wrapper contracts that assume routeful `cluster-proof/` behavior so historical compatibility rails fail closed on the new packaged proof instead of silently testing a deleted package shape.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m045-s04.sh` / `scripts/verify-m045-s05.sh` historical wrappers | Fail fast if the wrapper no longer points at a real packaged rail instead of replaying deleted HTTP/package steps. | Wrapper execution should stop on the delegated verifier timeout instead of masking it as a historical no-op. | Treat missing phase files or bundle pointers from the delegated verifier as wrapper failures. |
| `compiler/meshc/tests/e2e_m045_s04.rs` / `e2e_m045_s05.rs` content assertions | Fail on stale `/work`, Fly HTTP, or delay-hook assumptions rather than accepting a deleted package story. | N/A | Treat malformed or missing alias/verifier references as contract drift. |
| `compiler/meshc/tests/e2e_m044_s05.rs` / `scripts/verify-m044-s05.sh` historical closeout rails | Fail closed if they still require `cluster-proof/README.md` to describe routes, same-image HTTP probes, or `CLUSTER_PROOF_WORK_DELAY_MS`. | N/A | Treat mismatched historical alias text as legacy-wrapper drift rather than rewriting docs back to old behavior. |

## Load Profile

- **Shared resources**: Rust source scans plus historical shell wrappers that delegate to the new packaged verifier.
- **Per-operation cost**: Three focused Rust integration test targets and any delegated phase-file checks in the historical wrappers.
- **10x breakpoint**: Content-drift across wrapper/test files will break first; this task is about proof-surface integrity, not runtime scale.

## Negative Tests

- **Malformed inputs**: Historical wrappers that still mention `/membership`, `/work`, `CLUSTER_PROOF_WORK_DELAY_MS`, `http_service`, or `mesh-cluster-proof.fly.dev` as current packaged truth.
- **Error paths**: Missing `scripts/verify-m046-s04.sh` delegation, missing retained phase/bundle files, or assertions that still require the deleted HTTP surface.
- **Boundary conditions**: The old M044/M045 rails may remain as historical aliases, but they must name the M046 packaged route-free proof they now depend on and must not quietly resurrect the legacy package shape.

## Steps

1. Rewrite `compiler/meshc/tests/e2e_m045_s04.rs` and `scripts/verify-m045-s04.sh` so the historical assembled subrail becomes an alias/wrapper around `scripts/verify-m046-s04.sh` instead of the deleted HTTP contract.
2. Rewrite `compiler/meshc/tests/e2e_m045_s05.rs` and `scripts/verify-m045-s05.sh` so the closeout wrapper tracks the new packaged rail and its retained verifier artifacts, not deleted package HTTP steps.
3. Narrow `compiler/meshc/tests/e2e_m044_s05.rs` and `scripts/verify-m044-s05.sh` so any retained historical closeout assertions stop requiring `/work`, `CLUSTER_PROOF_WORK_DELAY_MS`, or Fly HTTP packaging from `cluster-proof/README.md`.
4. Leave broad scaffold/docs parity to S05, but make every retained historical wrapper/test fail closed on the new packaged proof rail it now depends on.

## Must-Haves

- [ ] No M044/M045 wrapper test still asserts the deleted `/membership`, `/work`, delay-hook, or Fly HTTP package story as current truth.
- [ ] Historical wrapper scripts either execute or inspect `scripts/verify-m046-s04.sh` and its phase/bundle artifacts instead of replaying removed package steps.
- [ ] `cluster-proof/README.md` may change to the route-free packaged story without immediately breaking older wrapper suites.
- [ ] Broad docs/scaffold parity remains explicitly deferred to S05 rather than being half-reintroduced through legacy wrapper assertions.
  - Estimate: 2h
  - Files: compiler/meshc/tests/e2e_m044_s05.rs, scripts/verify-m044-s05.sh, compiler/meshc/tests/e2e_m045_s04.rs, scripts/verify-m045-s04.sh, compiler/meshc/tests/e2e_m045_s05.rs, scripts/verify-m045-s05.sh
  - Verify: cargo test -p meshc --test e2e_m044_s05 m044_s05_ -- --nocapture && cargo test -p meshc --test e2e_m045_s04 m045_s04_ -- --nocapture && cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture
