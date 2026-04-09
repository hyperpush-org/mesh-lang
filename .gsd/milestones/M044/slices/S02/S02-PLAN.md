# S02: Runtime-Owned Declared Handler Execution

**Goal:** Attach runtime-owned execution semantics to S01’s explicit declared-handler boundary so declared work and service handlers are the only clustered path, while `cluster-proof` becomes a thin HTTP proof consumer instead of a placement/dispatch engine.
**Demo:** After this: After this: the same binary can run on two nodes and execute declared clustered handlers with runtime-owned placement and continuity, while undeclared code stays ordinary local Mesh code.

## Tasks
- [x] **T01: Threaded declared clustered execution metadata through shared manifest validation and into meshc build preparation.** — ---
estimated_steps: 4
estimated_files: 5
skills_used:
  - rust-best-practices
  - test
---

# T01: Carry declared-handler execution metadata past manifest validation

**Slice:** S02 — Runtime-Owned Declared Handler Execution
**Milestone:** M044

## Description

S01 proved the declaration boundary, but `meshc` still throws the manifest away before MIR/codegen. This task creates the compiler-owned execution metadata that later runtime/codegen work can consume without reopening manifest parsing or widening the declared boundary past what `mesh.toml` explicitly names.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Shared manifest/export validation in `mesh-pkg` + `meshc` | Fail compilation with the declaration kind, target, and execution-planning reason; do not silently drop clustered metadata. | N/A — compile-time only. | Reject ambiguous/private/non-executable targets instead of coercing them into local-only behavior. |
| Project-aware `mesh-lsp` parity | Keep editor diagnostics aligned with compiler truth; treat drift as a regression. | N/A — analysis is synchronous. | Reject declarations that validate in one surface but not the other. |
| MIR/codegen handoff | Stop before lowering if a declared handler has no runtime-executable symbol or wrapper plan. | N/A — compile-time only. | Treat missing symbol metadata as a contract failure, not as “undeclared by accident.” |

## Load Profile

- **Shared resources**: manifest parsing, export discovery, project-aware analysis, and compiler metadata threading.
- **Per-operation cost**: one clustered execution-plan derivation per build or analysis.
- **10x breakpoint**: duplicated target-resolution logic drifts first; large-project builds/LSP reanalysis get slower before runtime cost matters.

## Negative Tests

- **Malformed inputs**: malformed service target paths, blank or route-shaped work targets, and ambiguous overloaded public work names.
- **Error paths**: a declared target validates as exported but cannot be mapped to an executable symbol/wrapper; undeclared targets stay absent from the execution plan.
- **Boundary conditions**: manifest absent, cluster section absent, and manifests mixing work plus service declarations.

## Steps

1. Extend the clustered manifest/compiler seam so each validated declaration becomes richer execution metadata: declaration kind, manifest target, executable symbol/wrapper identifier, and enough info for runtime registration.
2. Thread that metadata through `meshc` to the lowering/codegen boundary and mirror any semantic narrowing in `mesh-lsp` so S01’s explicit boundary remains the only clustered boundary.
3. Create `compiler/meshc/tests/e2e_m044_s02.rs` with `m044_s02_metadata_` coverage for manifestless builds, invalid executable targets, and undeclared-target absence.
4. Keep raw HTTP route handlers and undeclared helpers out of this execution plan entirely.

## Must-Haves

- [ ] Declared clustered targets survive past validation as compiler-owned execution metadata.
- [ ] Invalid executable targets fail before codegen with explicit target/reason output.
- [ ] Manifestless and undeclared code paths stay ordinary local behavior.
  - Estimate: 2h
  - Files: compiler/mesh-pkg/src/manifest.rs, compiler/meshc/src/main.rs, compiler/mesh-lsp/src/analysis.rs, compiler/mesh-typeck/src/lib.rs, compiler/meshc/tests/e2e_m044_s02.rs
  - Verify: `cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture`
`cargo test -p meshc --test e2e_m044_s01 m044_s01_manifest_ -- --nocapture`
- [x] **T02: Move declared work placement and continuity dispatch into the runtime** — ---
estimated_steps: 4
estimated_files: 7
skills_used:
  - rust-best-practices
  - test
---

# T02: Move declared work placement and continuity dispatch into the runtime

**Slice:** S02 — Runtime-Owned Declared Handler Execution
**Milestone:** M044

## Description

This is the R064 core for work handlers. Replace the app-owned owner/replica calculation and continuity submit/dispatch flow with runtime APIs that consume the declared execution metadata, choose placement internally, reuse the continuity registry, and leave undeclared work on the ordinary local path.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Continuity registry and node-session transport in `mesh-rt` | Reject the submit with an explicit continuity reason; do not silently fall back to local execution. | Surface the existing continuity timeout path with attempt/request context. | Reject malformed sync/upsert payloads instead of executing with ambiguous ownership. |
| Placement and declared-handler registry state | Fail closed when membership or declared target resolution is invalid; do not invent a local owner. | Treat stale or missing registry state as execution failure, not as undeclared-local fallback. | Reject mismatched target metadata before dispatch. |
| Runtime ABI / codegen intrinsic alignment | Stop at build/test time on missing symbols or stale `mesh-rt` artifacts. | N/A — compile/link time. | Treat wrong payload boxing or intrinsic signatures as ABI regressions, not as alternate code paths. |

## Load Profile

- **Shared resources**: continuity registry, node sessions, declared-handler registry, and request-key placement.
- **Per-operation cost**: one placement calculation, one continuity submit, optional replica prepare/ack work, and one declared-work dispatch per request.
- **10x breakpoint**: continuity registry and node-session pressure fail before CPU cost matters.

## Negative Tests

- **Malformed inputs**: blank request key, unknown declared target id, and missing executable symbol metadata.
- **Error paths**: replica unavailable, invalid target selection, remote dispatch failure, and stale-runtime-library drift.
- **Boundary conditions**: single-node local owner, two-node remote owner, and same-key duplicate/conflict submissions.

## Steps

1. Add runtime-owned declared-work registration and placement/submit/dispatch APIs in `mesh-rt`, reusing the existing continuity registry and node transport instead of preserving `cluster-proof`’s `canonical_placement` and actor-context bridge.
2. Expose those runtime calls through builtin/codegen intrinsics and lower declared work entrypoints onto them using the execution metadata from T01.
3. Keep undeclared work functions on the ordinary local path; only manifest-declared handlers may use the runtime-owned clustered flow.
4. Extend `compiler/meshc/tests/e2e_m044_s02.rs` with `m044_s02_declared_work_` coverage for remote-owner execution, single-node local fallback, duplicate/conflict stability, and undeclared-local behavior.

## Must-Haves

- [ ] Declared work placement and continuity submit/dispatch move into the runtime.
- [ ] Undeclared work stays ordinary local Mesh code.
- [ ] Two-node tests prove remote-owner execution and same-key fencing without app-owned placement helpers.
  - Estimate: 3h
  - Files: compiler/mesh-rt/src/dist/continuity.rs, compiler/mesh-rt/src/dist/node.rs, compiler/mesh-rt/src/lib.rs, compiler/mesh-typeck/src/builtins.rs, compiler/mesh-codegen/src/mir/lower.rs, compiler/mesh-codegen/src/codegen/intrinsics.rs, compiler/meshc/tests/e2e_m044_s02.rs
  - Verify: `cargo test -p meshc --test e2e_m044_s02 m044_s02_declared_work_ -- --nocapture`
`cargo test -p meshc --test e2e_m044_s01 m044_s01_typed_continuity_ -- --nocapture`
- [x] **T03: Stopped at the investigation handoff and recorded the exact clustered service-wrapper seams before implementation.** — ---
estimated_steps: 4
estimated_files: 6
skills_used:
  - rust-best-practices
  - test
---

# T03: Generate clustered service-call and service-cast execution wrappers

**Slice:** S02 — Runtime-Owned Declared Handler Execution
**Milestone:** M044

## Description

Declared service handlers cannot reuse the raw remote-spawn path as-is because the real handler bodies are compiler-internal `__service_*` functions. This task turns S01’s service declarations into runtime-executable clustered wrapper/thunk symbols, keeps the internal service loop private, and proves declared-vs-undeclared behavior stays honest.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `ServiceExportInfo` method mapping | Fail compilation if a declared service target cannot be mapped to a clustered wrapper. | N/A — compile-time only. | Do not expose raw `__service_*` internals as a fallback. |
| Runtime registration/dispatch for service wrappers | Reject undeclared or stale wrapper ids explicitly. | Surface reply timeout or missing remote result as an error, not as a silent cast. | Reject malformed reply payloads instead of corrupting service state. |
| Existing local service start/call/cast lowering | Preserve ordinary local behavior for undeclared services and `start` helpers. | N/A — local path is synchronous except existing service scheduler behavior. | Treat wrapper/local-path mismatches as lowering regressions. |

## Load Profile

- **Shared resources**: actor scheduler, service mailboxes, node sessions, and declared-handler registry state.
- **Per-operation cost**: one wrapper dispatch plus service message encode/decode per clustered call or cast.
- **10x breakpoint**: remote service mailbox pressure and reply waiters fail before codegen complexity matters.

## Negative Tests

- **Malformed inputs**: wrong arity, wrong service/method kind, and undeclared method targets.
- **Error paths**: remote reply timeout, missing reply, and wrapper-to-handler symbol mismatch.
- **Boundary conditions**: declared call vs declared cast, undeclared local service methods, and `start` helpers staying non-clustered.

## Steps

1. Generate declared clustered wrapper/thunk symbols for service call/cast handlers from `ServiceExportInfo.methods` instead of exposing compiler-internal `__service_*` bodies directly.
2. Register only declared service wrappers with the runtime declared-handler registry and lower clustered service calls/casts through that path.
3. Preserve ordinary local service start/call/cast behavior for undeclared services and start helpers.
4. Extend `compiler/meshc/tests/e2e_m044_s02.rs` with `m044_s02_service_` coverage for declared remote call/cast behavior and undeclared local service behavior.

## Must-Haves

- [ ] Declared `service_call` and `service_cast` targets map to runtime-executable wrappers, not raw `__service_*` internals.
- [ ] Undeclared service methods and `start` helpers stay on the ordinary local path.
- [ ] Named tests prove declared-vs-undeclared service behavior in a clustered runtime.
  - Estimate: 3h
  - Files: compiler/mesh-typeck/src/lib.rs, compiler/mesh-codegen/src/mir/lower.rs, compiler/mesh-codegen/src/codegen/mod.rs, compiler/mesh-codegen/src/codegen/expr.rs, compiler/mesh-rt/src/dist/node.rs, compiler/meshc/tests/e2e_m044_s02.rs
  - Verify: `cargo test -p meshc --test e2e_m044_s02 m044_s02_service_ -- --nocapture`
- [x] **T04: Stopped before a dishonest `cluster-proof` rewrite and recorded the missing S02 declared-execution seams.** — ---
estimated_steps: 4
estimated_files: 7
skills_used:
  - test
---

# T04: Rewrite cluster-proof onto runtime-owned declared execution

**Slice:** S02 — Runtime-Owned Declared Handler Execution
**Milestone:** M044

## Description

`cluster-proof` is the dogfood consumer for this slice, so it must stop declaring route handlers as clustered work and stop computing placement/dispatch on the new submit/status path. This task retargets the manifest to the real business handlers or generated wrappers from T02/T03, keeps HTTP parsing/JSON shaping local, and shrinks the proof app’s clustering logic to the legacy surfaces that still belong to S05.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Retargeted `cluster-proof/mesh.toml` declarations | Fail the build if the proof app still declares route wrappers or undeclared helpers. | N/A — manifest validation is synchronous. | Reject stale targets rather than broadening the clustered boundary to keep the app green. |
| Runtime-owned submit/status path | Return truthful HTTP errors from runtime authority/continuity failures; do not recreate app-local placement or `Node.spawn(...)` on error. | Preserve continuity timeout/error surfaces at HTTP level. | Reject malformed runtime payloads at the app boundary instead of reintroducing JSON shims or fake local fallback. |
| Legacy probe coexistence | Keep the existing legacy proof path honest without letting it become the clustered hot path again. | Preserve existing timeout/error reporting for the legacy probe. | Scope any remaining legacy helpers to the old probe only. |

## Load Profile

- **Shared resources**: HTTP routes, runtime continuity/status reads, cluster-proof package tests, and proof-app logs.
- **Per-operation cost**: one HTTP parse/encode plus one runtime clustered submit/status operation per request.
- **10x breakpoint**: proof-app HTTP/log volume should become the first bottleneck, not app-owned placement math.

## Negative Tests

- **Malformed inputs**: invalid JSON, blank request keys, and malformed status lookups.
- **Error paths**: same-key duplicate/conflict at HTTP level, runtime authority unavailable, and declared target mismatch in the manifest.
- **Boundary conditions**: standalone local execution, two-node remote-owner execution, and coexistence with the legacy probe path.

## Steps

1. Retarget `cluster-proof/mesh.toml` declarations from `handle_work_*` ingress handlers to the real runtime-safe declared work/service targets introduced in T02/T03.
2. Rewrite `cluster-proof/work_continuity.mpl` so submit/status/promotion call the runtime-owned declared execution/status surfaces and no longer compute `current_target_selection`, `canonical_placement`, or actor-context `Node.spawn(...)` on the new path.
3. Shrink `cluster-proof/work.mpl` and `cluster-proof/cluster.mpl` to the surfaces still needed for status/legacy proof behavior; keep HTTP request parsing and JSON response shaping local.
4. Update `cluster-proof/tests/work.test.mpl` and proof-app wiring in `cluster-proof/main.mpl` / `work_legacy.mpl` to assert the new declared-runtime boundary and the remaining legacy boundary honestly.

## Must-Haves

- [ ] `cluster-proof` no longer declares HTTP ingress handlers as clustered work.
- [ ] The new submit/status hot path does not compute placement or call `Node.spawn(...)` in Mesh code.
- [ ] HTTP behavior stays the same externally while runtime-owned execution truth drives the result.
  - Estimate: 3h
  - Files: cluster-proof/mesh.toml, cluster-proof/work_continuity.mpl, cluster-proof/work.mpl, cluster-proof/cluster.mpl, cluster-proof/work_legacy.mpl, cluster-proof/main.mpl, cluster-proof/tests/work.test.mpl
  - Verify: `cargo test -p meshc --test e2e_m044_s02 m044_s02_cluster_proof_ -- --nocapture`
`cargo run -q -p meshc -- build cluster-proof`
`cargo run -q -p meshc -- test cluster-proof/tests`
  - Blocker: The remaining S02 work is still blocked on the missing declared-handler execution substrate: there are no `m044_s02_declared_work_`, `m044_s02_service_`, or `m044_s02_cluster_proof_` tests in `compiler/meshc/tests/e2e_m044_s02.rs`, `meshc build` still ignores `PreparedBuild.clustered_execution_plan`, and `cluster-proof/work_continuity.mpl` still computes keyed target selection plus direct `Node.spawn(...)` dispatch on the new hot path.
- [x] **T05: Stopped at the declared-work execution handoff and recorded the actor-wrapper seam before implementation.** — 1. Thread PreparedBuild.clustered_execution_plan through meshc lowering/codegen instead of dropping it after validation.
2. Add a runtime-owned declared-work registration/dispatch seam that consumes manifest-approved runtime_registration_name/executable_symbol metadata without widening ordinary Node.spawn or undeclared local execution.
3. Lower declared work entrypoints onto that runtime path while keeping undeclared work on the existing local path.
4. Extend compiler/meshc/tests/e2e_m044_s02.rs with m044_s02_declared_work_ coverage for single-node local owner, two-node remote owner, duplicate/conflict stability, and undeclared-local behavior.
  - Estimate: 3h
  - Files: compiler/meshc/src/main.rs, compiler/mesh-codegen/src/mir/lower.rs, compiler/mesh-codegen/src/codegen/mod.rs, compiler/mesh-codegen/src/codegen/intrinsics.rs, compiler/mesh-rt/src/dist/node.rs, compiler/mesh-rt/src/lib.rs, compiler/meshc/tests/e2e_m044_s02.rs
  - Verify: cargo test -p meshc --test e2e_m044_s02 m044_s02_declared_work_ -- --nocapture
cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture
- [x] **T06: Stopped at the declared service wrapper handoff and recorded that remote service replies already use mesh_actor_send, so T06 still needs wrapper registration and lowering rather than a second reply transport.** — 1. Turn declared service_call/service_cast targets into runtime-executable wrapper metadata instead of relying on the generic service helper surface by accident.
2. Register only manifest-declared service wrappers with the runtime declared-handler registry and lower clustered service call/cast through that path.
3. Preserve ordinary local start helpers and undeclared service methods exactly as local Mesh code.
4. Extend compiler/meshc/tests/e2e_m044_s02.rs with m044_s02_service_ coverage for declared remote call/cast behavior and undeclared local service behavior.
  - Estimate: 3h
  - Files: compiler/mesh-typeck/src/lib.rs, compiler/mesh-typeck/src/infer.rs, compiler/meshc/src/main.rs, compiler/mesh-codegen/src/mir/lower.rs, compiler/mesh-codegen/src/codegen/mod.rs, compiler/mesh-rt/src/dist/node.rs, compiler/meshc/tests/e2e_m044_s02.rs
  - Verify: cargo test -p meshc --test e2e_m044_s02 m044_s02_service_ -- --nocapture
- [x] **T07: Recorded that T07 remains blocked by the missing declared-handler execution substrate; `cluster-proof` still cannot be rewritten honestly.** — 1. Retarget cluster-proof declarations from HTTP ingress handlers to the real declared work/service targets or generated wrappers introduced by T05/T06.
2. Replace the new submit/status hot path in cluster-proof/work_continuity.mpl so it calls the runtime-owned declared execution/status surfaces instead of computing keyed placement or direct Node.spawn(...) dispatch in Mesh code.
3. Keep HTTP parsing/JSON shaping local and confine any remaining current_target_selection(...) or Node.spawn(...) logic to explicitly legacy proof surfaces only.
4. Add m044_s02_cluster_proof_ coverage and update cluster-proof package tests/build expectations around the new boundary.
  - Estimate: 3h
  - Files: cluster-proof/mesh.toml, cluster-proof/work_continuity.mpl, cluster-proof/work.mpl, cluster-proof/cluster.mpl, cluster-proof/work_legacy.mpl, cluster-proof/main.mpl, cluster-proof/tests/work.test.mpl, compiler/meshc/tests/e2e_m044_s02.rs
  - Verify: cargo test -p meshc --test e2e_m044_s02 m044_s02_cluster_proof_ -- --nocapture
cargo run -q -p meshc -- build cluster-proof
cargo run -q -p meshc -- test cluster-proof/tests
  - Blocker: The missing declared-handler execution substrate is still the blocker for the remainder of S02. Until `meshc` actually consumes `clustered_execution_plan`, declared service targets lower to real clustered wrappers, and `cluster-proof` can call runtime-owned submit/status surfaces, T07 cannot land honestly.
- [x] **T08: Added scripts/verify-m044-s02.sh as a fail-closed S02 rail and recorded that the declared-runtime substrate it expects is still missing.** — 1. Rewrite scripts/verify-m044-s02.sh to replay the S01 rail, refresh mesh-rt, run the named metadata/declared_work/service/cluster_proof filters, then rebuild and retest cluster-proof in order.
2. Fail closed on missing running-N-test evidence or zero-test filters, and preserve per-phase logs plus copied e2e bundles under .tmp/m044-s02/verify/.
3. Add narrow absence checks proving the new declared-runtime submit/status hot path in cluster-proof/work_continuity.mpl no longer depends on current_target_selection(...) or direct Node.spawn(...), while allowing the explicitly legacy surfaces that survive until S05.
4. Treat bash scripts/verify-m044-s02.sh as the slice stop condition and align the named test prefixes in compiler/meshc/tests/e2e_m044_s02.rs with the verifier.
  - Estimate: 90m
  - Files: scripts/verify-m044-s01.sh, scripts/verify-m044-s02.sh, compiler/meshc/tests/e2e_m044_s02.rs, cluster-proof/work_continuity.mpl
  - Verify: bash scripts/verify-m044-s02.sh
  - Blocker: `compiler/meshc/tests/e2e_m044_s02.rs` still only contains the metadata tests, so the verifier stops at `m044_s02_declared_work_` before it can reach the service, cluster-proof, or hot-path absence phases. Separately, `cluster-proof/work_continuity.mpl` still contains the old app-owned submit/dispatch flow (`current_target_selection(...)`, `submit_from_selection(...)`, `dispatch_work(...)`, and `dispatch_remote_work(...) -> Node.spawn(...)`), so even after the missing named tests land, the later absence checks will remain red until T05-T07 actually remove that path.
- [x] **T09: Repaired declared-handler registry plumbing so declared executable symbols survive MIR pruning and the metadata rail passes again.** — 1. Repair the partial `mesh-codegen` refactor so the workspace compiles again.
2. Thread `PreparedBuild.clustered_execution_plan` into a clean declared-handler preparation step that returns runtime registrations instead of being dropped.
3. Keep the work bounded to `meshc`/`mesh-codegen`/`mesh-rt` plumbing first; do not touch cluster-proof again until the compile path is green.
4. Verify with the metadata rail before moving on.
  - Estimate: 2h
  - Files: compiler/meshc/src/main.rs, compiler/mesh-codegen/src/lib.rs, compiler/mesh-codegen/src/codegen/mod.rs, compiler/mesh-codegen/src/codegen/intrinsics.rs, compiler/mesh-rt/src/dist/node.rs, compiler/mesh-rt/src/lib.rs
  - Verify: cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture
- [x] **T10: Captured the real T10 stop-point: runtime-owned declared work exists locally, but `cluster-proof` still fails before the S02 verifier can reach the new rails.** — 1. Finish the runtime-owned declared work submit/dispatch seam on top of the repaired registry plumbing.
2. Retarget `cluster-proof` so the new keyed submit hot path calls the runtime-owned declared work boundary instead of `current_target_selection(...)` / direct dispatch helpers.
3. Keep `WorkLegacy` and the old probe path explicit and isolated.
4. Add or repair the named declared-work and cluster-proof e2e rails so the slice verifier can reach the later phases truthfully.
  - Estimate: 3h
  - Files: cluster-proof/mesh.toml, cluster-proof/work_continuity.mpl, cluster-proof/work_legacy.mpl, cluster-proof/main.mpl, compiler/meshc/tests/e2e_m044_s02.rs, scripts/verify-m044-s02.sh
  - Verify: bash scripts/verify-m044-s02.sh
- [x] **T11: Stopped after landing declared-service wrapper generation and `cluster-proof` compile fixes; the named service rail still fails because the registration assertion is checking the wrong emitted LLVM surface.** — 1. Land the declared service wrapper surface after the declared-work/runtime plumbing is green.
2. Keep the scope honest: prove wrapper generation/registration and a truthful named `m044_s02_service_` rail instead of inventing a second reply transport.
3. Update the slice verifier only if the proof surface changes; otherwise make the new named service rail satisfy the existing contract.
4. Re-run the full S02 verifier after the service rail exists.
  - Estimate: 2h
  - Files: compiler/meshc/tests/e2e_m044_s02.rs, compiler/mesh-codegen/src/declared.rs, compiler/mesh-typeck/src/builtins.rs, compiler/mesh-typeck/src/infer.rs, scripts/verify-m044-s02.sh
  - Verify: bash scripts/verify-m044-s02.sh
