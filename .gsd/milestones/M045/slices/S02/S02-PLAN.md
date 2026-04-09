# S02: Tiny End-to-End Clustered Example

**Goal:** Make the scaffold-first clustered example honestly tiny and end-to-end: two local nodes form a cluster, the runtime chooses a remote owner for at least one keyed submit, continuity reaches completed on both nodes, and the public example stays free of app-owned routing, placement, or status-truth logic.
**Demo:** After this: After this: one small local clustered example runs on two nodes and proves runtime-chosen remote execution without app-owned routing or placement logic.

## Tasks
- [x] **T01: Registered manifest-approved declared wrapper symbols for remote spawn and added a two-node remote-owner completion regression.** — Repair the runtime/codegen seam so a manifest-declared work wrapper generated as `__declared_work_*` can be found by the remote `mesh_node_spawn(...)` path without widening undeclared helpers into the public execution surface.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Generic remote function registry in `compiler/mesh-codegen/src/codegen/mod.rs` and `compiler/mesh-rt/src/dist/node.rs` | Fail the build/tests loudly on missing registration or lookup drift; do not fall back to local spawn. | N/A — registration and lookup are synchronous. | Reject unknown wrappers with an explicit runtime error instead of coercing to a different executable symbol. |
| Declared-handler planning in `compiler/mesh-codegen/src/declared.rs` and `compiler/meshc/tests/e2e_m044_s02.rs` | Keep manifest gating intact so undeclared locals remain absent from remote execution. | N/A — compile/e2e coverage is bounded. | Treat wrong wrapper/runtime-name pairing as a contract regression, not a best-effort alias. |

## Load Profile

- **Shared resources**: function registry, declared-handler registry, and remote node session state.
- **Per-operation cost**: one runtime registration plus one remote spawn lookup/dispatch.
- **10x breakpoint**: registry drift and session reconnect churn fail before throughput does; the seam must remain narrow and explicit.

## Negative Tests

- **Malformed inputs**: undeclared runtime target, missing wrapper symbol, and wrapper/runtime-name mismatch.
- **Error paths**: remote spawn rejection on missing function, reconnect/retry drift, and accidental widening of manifestless helpers.
- **Boundary conditions**: local-owner declared work still runs, remote-owner declared work no longer rejects, and service-declaration rails stay protected.

## Steps

1. Update the generated declared-work registration path so manifest-approved wrappers remain remote-spawnable even though the raw function name starts with `__`.
2. Keep the runtime registry/lookup surface explicit and fail closed when a runtime name or wrapper symbol is not manifest-approved.
3. Seed a dedicated `compiler/meshc/tests/e2e_m045_s02.rs` regression for the remote-owner seam while preserving the existing M044 declared-handler coverage.
4. Re-run the focused declared-handler rails to prove remote-owner submits stop failing at `function not found __declared_work_*`.

## Must-Haves

- [ ] Remote-owner declared-work submits can find the generated wrapper symbol on the owner node.
- [ ] Undeclared helpers remain absent from remote execution.
- [ ] Existing declared service/work coverage still protects the manifest gate.
- [ ] A new M045/S02 test file exists from the first task onward.

  - Estimate: 3h
  - Files: compiler/mesh-codegen/src/codegen/mod.rs, compiler/mesh-codegen/src/declared.rs, compiler/mesh-rt/src/dist/node.rs, compiler/meshc/tests/e2e_m044_s02.rs, compiler/meshc/tests/e2e_m045_s02.rs
  - Verify: - `cargo test -p meshc --test e2e_m044_s02 m044_s02_declared_work_ -- --nocapture`
- `cargo test -p meshc --test e2e_m045_s02 m045_s02_declared_work_remote_spawn_ -- --nocapture`
- [x] **T02: Moved declared-work completion into the runtime/codegen seam and shrank the clustered scaffold to a tiny runtime-owned example.** — Keep the public clustered example tiny by making successful declared work complete through the runtime/codegen seam, then rewrite the scaffold around that contract so `meshc init --clustered` no longer needs app-owned completion, placement, or status helpers.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Continuity completion path in `compiler/mesh-rt/src/dist/node.rs` / `compiler/mesh-rt/src/dist/continuity.rs` | Surface an explicit failure and keep the continuity record inspectable; do not fake success in the scaffold. | Bound polls in tests and fail with retained continuity/log artifacts. | Treat mismatched attempt IDs or missing execution-node truth as a runtime contract failure. |
| Clustered scaffold generator in `compiler/mesh-pkg/src/scaffold.rs` and its tooling/source-contract rails | Fail tests if generated source grows example-owned status or completion logic back in. | N/A — generation is synchronous and bounded. | Reject stale `Continuity.mark_completed`, placement helpers, or proof-app literals instead of tolerating them. |

## Load Profile

- **Shared resources**: continuity registry, spawned work actors, temporary scaffold project dirs, local ports, and CLI continuity polling.
- **Per-operation cost**: one declared-work execution plus one completion update and one scaffold init/build per proof case.
- **10x breakpoint**: pending records and process cleanup flake before raw throughput; diagnostics must show whether work ran but completion failed.

## Negative Tests

- **Malformed inputs**: stale attempt ID, missing execution-node truth, and generated source that still contains `Continuity.mark_completed`, placement helpers, or app-owned status routes.
- **Error paths**: work runs but continuity never completes, completion update rejects, or the generated scaffold build/source rails drift.
- **Boundary conditions**: local-owner completion, remote-owner completion, and a generated app that still builds while staying tiny.

## Steps

1. Extend the runtime/codegen declared-work path so a successful work execution records completion with the truthful execution node instead of leaving the record pending.
2. Rewrite the scaffolded `work.mpl`/`main.mpl`/README contract around that runtime-owned completion path, keeping only bootstrap, `/health`, and submit logic local.
3. Update tooling and source-contract rails so the generated example is pinned to the tiny runtime-owned shape instead of the older incomplete behavior.
4. Extend `compiler/meshc/tests/e2e_m045_s02.rs` with scaffold contract and completion assertions that guard R077, R079, and R080 directly.

## Must-Haves

- [ ] Scaffolded declared work reaches `phase=completed` without app-owned `Continuity.mark_completed(...)` glue.
- [ ] The generated example stays tiny and language-first.
- [ ] The scaffold README points users at runtime-owned `meshc cluster status` / `meshc cluster continuity` truth.
- [ ] Source/tooling rails fail if placement/status/completion logic leaks back into the example.

  - Estimate: 3h
  - Files: compiler/mesh-codegen/src/declared.rs, compiler/mesh-rt/src/dist/node.rs, compiler/mesh-rt/src/dist/continuity.rs, compiler/mesh-pkg/src/scaffold.rs, compiler/meshc/tests/tooling_e2e.rs, compiler/meshc/tests/e2e_m045_s01.rs, compiler/meshc/tests/e2e_m045_s02.rs
  - Verify: - `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`
- `cargo test -p meshc --test e2e_m045_s02 m045_s02_scaffold_runtime_completion_ -- --nocapture`
- [x] **T03: Added the tiny two-node scaffold proof rail and fail-closed verifier, with retained evidence bundles and runtime-owned continuity checks.** — Finish the slice with the public proof surface: init a clustered scaffold project, run two local nodes, let the runtime choose a remote owner via honest request-key retries, assert runtime-owned continuity truth on both nodes, and package that flow into an assembled verifier that replays upstream rails fail-closed.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Two-node scaffold runtime and CLI inspection surfaces in `compiler/meshc/tests/e2e_m045_s02.rs` | Stop with retained HTTP, CLI, and node-log artifacts; do not infer success from startup logs alone. | Bound membership/remote-owner/completion polling and fail with the last observed payload. | Treat malformed `meshc cluster status` / `meshc cluster continuity` JSON as a hard proof failure. |
| Assembled verifier in `scripts/verify-m045-s02.sh` | Fail closed on zero-test, stale-artifact, or upstream-rail drift. | Preserve per-phase logs and copied evidence bundles instead of hanging. | Reject malformed manifests, missing copied artifacts, or pointer drift rather than claiming green. |

## Load Profile

- **Shared resources**: temporary scaffold dirs, dual-stack ports, spawned node processes, CLI subprocesses, and `.tmp/m045-s02` artifact roots.
- **Per-operation cost**: one scaffold init/build, two node boots, repeated CLI/HTTP polls, and one verifier replay.
- **10x breakpoint**: port/process cleanup and proof-artifact churn fail before runtime throughput; retained evidence must make the failure legible.

## Negative Tests

- **Malformed inputs**: zero-test filter drift, malformed continuity/status JSON, and missing copied artifact directories.
- **Error paths**: remote owner never chosen, continuity remains pending, or ingress/owner disagree about completion truth.
- **Boundary conditions**: two-node convergence on loopback IPv4/IPv6, remote-owner retry selection without local placement reimplementation, and duplicate submit stability after completion.

## Steps

1. Build out `compiler/meshc/tests/e2e_m045_s02.rs` so it creates a scaffolded project, runs two nodes, retries request keys until the runtime chooses a remote owner, and then asserts completion truth through runtime inspection surfaces.
2. Retain the resulting CLI/HTTP/node-log artifacts under `.tmp/m045-s02/...` so later slices can compare runtime state without rerunning the cluster immediately.
3. Add `scripts/verify-m045-s02.sh` as the slice stopping condition; replay `scripts/verify-m045-s01.sh`, the relevant declared-handler rail, the scaffold init contract, and the new S02 e2e while failing closed on zero-test or artifact drift.

## Must-Haves

- [ ] The new two-node proof trusts runtime-chosen remote ownership instead of reimplementing placement locally.
- [ ] Both ingress and owner nodes report the same completed continuity truth for the remote-owner request.
- [ ] The assembled verifier is authoritative and preserves copied evidence for later debugging.

  - Estimate: 3h
  - Files: compiler/meshc/tests/e2e_m045_s02.rs, scripts/verify-m045-s02.sh
  - Verify: - `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture`
- `bash scripts/verify-m045-s02.sh`
