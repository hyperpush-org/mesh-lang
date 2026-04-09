---
id: S02
parent: M044
milestone: M044
provides:
  - A runtime-owned declared-handler execution seam for clustered work and service handlers.
  - A `cluster-proof` dogfood path that declares a real clustered work target instead of clustering HTTP ingress handlers.
  - An authoritative fail-closed S02 verifier that replays the full slice contract and retains proof artifacts for downstream debugging.
requires:
  - slice: S01
    provides: clustered manifest declarations, shared compiler/LSP validator parity, and typed Continuity/authority surfaces consumed by the S02 runtime-owned execution path
affects:
  - S03
  - S04
  - S05
key_files:
  - scripts/verify-m044-s02.sh
  - compiler/meshc/tests/e2e_m044_s02.rs
  - compiler/meshc/src/main.rs
  - compiler/mesh-codegen/src/declared.rs
  - compiler/mesh-codegen/src/mir/mono.rs
  - compiler/mesh-codegen/src/codegen/mod.rs
  - compiler/mesh-codegen/src/codegen/intrinsics.rs
  - compiler/mesh-rt/src/dist/node.rs
  - cluster-proof/mesh.toml
  - cluster-proof/work_continuity.mpl
  - cluster-proof/main.mpl
  - .gsd/PROJECT.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - D195: keep declared clustered execution metadata as a shared compiler-owned plan from manifest validation through meshc preparation instead of re-parsing declarations later.
  - D198: preserve every manifest-declared executable symbol as an explicit merged-MIR monomorphization root so declared handlers survive pruning until runtime registration.
  - D196: expose declared service handlers to the runtime through distinct `__declared_service_*` wrapper symbols instead of registering raw compiler-internal `__service_*` helpers.
patterns_established:
  - Use a fail-closed slice-level verifier before closeout so named proof rails, retained artifacts, and source-boundary checks fail loudly instead of being hidden behind task-local success.
  - Treat manifest-declared executable symbols as explicit MIR roots whenever runtime registration depends on helpers the entrypoint may never call locally.
  - Scope stale-literal absence checks to the exact declared-runtime submit/status ranges so legacy probe helpers can survive temporarily without making the new clustered hot path look green.
observability_surfaces:
  - .tmp/m044-s02/verify/phase-report.txt
  - .tmp/m044-s02/verify/status.txt
  - .tmp/m044-s02/verify/current-phase.txt
  - .tmp/m044-s02/verify/full-contract.log
  - .tmp/m044-s02/verify/03-s02-declared-work.test-count.log
  - .tmp/m044-s02/verify/04-s02-service.test-count.log
  - .tmp/m044-s02/verify/05-s02-cluster-proof.test-count.log
  - .tmp/m044-s02/verify/05-s02-cluster-proof-artifacts.txt
drill_down_paths:
  - .gsd/milestones/M044/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M044/slices/S02/tasks/T08-SUMMARY.md
  - .gsd/milestones/M044/slices/S02/tasks/T09-SUMMARY.md
  - .gsd/milestones/M044/slices/S02/tasks/T10-SUMMARY.md
  - .gsd/milestones/M044/slices/S02/tasks/T11-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-29T23:27:16.613Z
blocker_discovered: false
---

# S02: Runtime-Owned Declared Handler Execution

**S02 made declared clustered execution real: meshc/codegen/runtime now carry and register only manifest-declared handlers, cluster-proof dogfoods the runtime-owned declared-work path, and the assembled S02 verifier is green.**

## What Happened

S02 started with only the S01 declaration metadata boundary. The durable implementation that finally closed the slice landed in three layers.

First, T01 made clustered declarations survive validation as compiler-owned execution metadata instead of a boolean pass/fail result. `mesh-pkg` now returns `ClusteredExecutionMetadata`, `mesh-typeck` exports explicit service method kinds, and both `meshc` and `mesh-lsp` consume the same shared declaration surface.

Second, the compiler/runtime seam was completed. `meshc` now threads the prepared clustered execution plan into codegen, merged MIR keeps manifest-declared executable symbols alive as explicit roots so locally-unused declared handlers are not pruned, and codegen registers declared runtime handlers through `mesh_register_declared_handler`. For declared work, the runtime-owned path is live through `mesh_continuity_submit_declared_work(...) -> node::submit_declared_work(...)`, which resolves manifest-approved registrations, computes placement internally, submits continuity state, and dispatches without app-owned target selection. For declared services, `mesh-codegen` now generates distinct `__declared_service_call_*` / `__declared_service_cast_*` wrapper symbols that delegate to the existing `__service_*` helpers so the runtime registry does not expose compiler-internal service helper names.

Third, `cluster-proof` was retargeted onto that runtime-owned boundary. `cluster-proof/mesh.toml` now declares `WorkContinuity.execute_declared_work` instead of clustering HTTP ingress handlers, `cluster-proof/work_continuity.mpl` submits declared work through `Continuity.submit_declared_work(...)`, and the new submit/status hot path no longer computes placement or calls `Node.spawn(...)` directly. The legacy probe path still exists, but it is now explicitly outside the new declared-runtime hot path and the verifier checks that boundary precisely instead of pretending the entire file has already been rewritten.

The slice now delivers what the roadmap promised: the same binary can run on multiple nodes, declared clustered work/service handlers are the only runtime-owned clustered path, and undeclared code stays ordinary local Mesh code. The proof surface is no longer a hand-waved summary or a blocked closeout note; it is the green assembled verifier plus retained artifact bundles under `.tmp/m044-s02/verify/`.

## Verification

Authoritative slice verification passed via `bash scripts/verify-m044-s02.sh`.

That rail now replays:
- `bash scripts/verify-m044-s01.sh`
- `cargo build -q -p mesh-rt`
- `cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s02 m044_s02_declared_work_ -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s02 m044_s02_service_ -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s02 m044_s02_cluster_proof_ -- --nocapture`
- `cargo run -q -p meshc -- build cluster-proof`
- `cargo run -q -p meshc -- test cluster-proof/tests`

It also proved the retained observability contract:
- `.tmp/m044-s02/verify/phase-report.txt` ended with every phase passed
- `.tmp/m044-s02/verify/status.txt` = `ok`
- `.tmp/m044-s02/verify/current-phase.txt` = `complete`
- the named test-count logs recorded real test execution (`declared_work` = 1 test, `service` = 2 tests, `cluster_proof` = 2 tests)
- the copied cluster-proof artifact bundle manifest in `.tmp/m044-s02/verify/05-s02-cluster-proof-artifacts.txt` points at a retained `scenario-meta.json`, build log, package-test log, and `work_continuity.mpl` snapshot
- the hot-path absence logs confirm the new submit/status path omits `current_target_selection(...)`, `submit_from_selection(...)`, direct `dispatch_work(...)`, and `Node.spawn(...)` in the declared-runtime boundary

## Requirements Advanced

- R062 — S02 moved the typed continuity/authority contract from metadata-only validation into the real declared-runtime execution path. `cluster-proof` now consumes the typed `Continuity` surfaces while submitting declared work through the runtime-owned boundary instead of app-owned placement helpers.
- R064 — S02 made the runtime own declared-handler placement/submission/dispatch for the new clustered hot path. `meshc`/codegen/runtime now prepare and register declared handlers, `Continuity.submit_declared_work(...)` drives the proof-app submit path, and the assembled verifier proves the app no longer computes placement or direct node dispatch in the new declared-work submit/status path.

## Requirements Validated

- R063 — Validated by `bash scripts/verify-m044-s02.sh`, which replays S01, runs the named `m044_s02_metadata_`, `m044_s02_declared_work_`, `m044_s02_service_`, and `m044_s02_cluster_proof_` filters, rebuilds/tests `cluster-proof`, and proves only manifest-declared handlers enter the clustered runtime path while undeclared work/service behavior stays local.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

The slice did not close in the original straight-line task order. The real implementation completed through the later recovery tasks: T08 established the fail-closed assembled verifier, T09 repaired the registry/monomorphization seam, T10 identified that the remaining break had moved to `cluster-proof`, and T11 landed the declared-service wrapper generation plus the proof-app fixes that let the full S02 rail turn green. T02 itself never received a written task summary even though its planned behavior now exists in the shipped code.

## Known Limitations

Built-in operator/CLI surfaces, clustered scaffolding, bounded automatic promotion, and the final removal of the legacy `cluster-proof` path are still out of scope for S02 and remain owned by later M044 slices. `WorkLegacy` and the old probe helpers still exist, but they are now explicitly outside the new declared-runtime submit/status hot path and no longer define the product boundary for clustered execution.

## Follow-ups

S03 can now build the standard runtime/CLI operator surfaces and `meshc init --clustered` scaffold on top of a real declared-handler execution substrate. S04 can layer bounded automatic promotion on top of the runtime-owned declared-handler path instead of a proof-app dispatch engine. S05 can finish the `cluster-proof` rewrite by deleting the remaining legacy path and aligning the public docs around the new standard clustered-app model.

## Files Created/Modified

- `scripts/verify-m044-s02.sh` — Added the authoritative fail-closed S02 acceptance rail with retained phase, artifact, and hot-path absence checks.
- `compiler/meshc/tests/e2e_m044_s02.rs` — Expanded the slice proof suite from metadata-only coverage to named declared-work, declared-service, and cluster-proof rails with retained artifact generation.
- `compiler/meshc/src/main.rs` — Threaded the clustered execution plan and declared executable-symbol roots into codegen preparation instead of dropping them after manifest validation.
- `compiler/mesh-codegen/src/mir/mono.rs` — Preserved manifest-declared executable symbols as explicit monomorphization roots so locally-unused declared helpers survive MIR pruning.
- `compiler/mesh-codegen/src/declared.rs` — Prepared declared runtime handler registrations and generated distinct `__declared_service_*` wrapper functions for service call/cast targets.
- `compiler/mesh-codegen/src/codegen/mod.rs` — Registered declared runtime handlers during code generation through `mesh_register_declared_handler`.
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — Added the intrinsic declarations for declared-handler registration and declared-work submission.
- `compiler/mesh-rt/src/dist/node.rs` — Exposed the runtime-owned declared-work submit/dispatch path that resolves declared handlers, computes placement internally, and dispatches through runtime-owned spawn surfaces.
- `cluster-proof/mesh.toml` — Retargeted clustered declarations from HTTP ingress handlers to `WorkContinuity.execute_declared_work`.
- `cluster-proof/work_continuity.mpl` — Moved the new submit path onto `Continuity.submit_declared_work(...)` and removed app-owned placement/dispatch helpers from the declared-runtime submit/status hot path.
- `cluster-proof/main.mpl` — Reworked the proof-app startup/router block into a parser-safe form that builds cleanly with the new declared-runtime dogfood path.
- `.gsd/PROJECT.md` — Updated the living project state to reflect that M044/S02 is now complete and to name the new authoritative verifier.
- `.gsd/KNOWLEDGE.md` — Captured the non-obvious S02 proof rules around emitted LLVM snapshots and the authoritative assembled verifier scope.
