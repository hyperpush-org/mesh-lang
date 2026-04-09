# S02: Replication-count semantics for clustered functions

**Goal:** Make ordinary source-declared clustered functions mean real runtime replication counts: bare `@cluster` must default to replication count `2`, explicit `@cluster(N)` counts must survive into runtime/CLI truth, and the proof surface must use generic runtime names on non-HTTP functions instead of the legacy `Work.execute_declared_work` story.
**Demo:** After this: After this: a non-HTTP clustered function using `@cluster` defaults to replication count `2`, explicit counts are preserved, and runtime truth no longer depends on a hardcoded `Work.execute_declared_work` story.

## Tasks
- [x] **T01: Threaded declared-handler replication counts from meshc planning through LLVM registration into the runtime registry, with new m047_s02 unit coverage.** — Carry S01's source-resolved `replication_count` through the compiler/codegen/runtime registration seam so the runtime can look up clustered-function counts by runtime name instead of guessing from startup heuristics.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/meshc/src/main.rs` -> `compiler/mesh-codegen/src/declared.rs` declared-handler plan seam | Fail the build with an explicit missing-count / missing-lowered-symbol error instead of emitting a handler with implicit count `0`. | Not applicable beyond test-time command failure; treat a hung codegen test as a failing rail. | Reject mismatched handler metadata in unit tests and keep runtime registration absent rather than silently defaulting. |
| `compiler/mesh-codegen/src/codegen/mod.rs` -> `compiler/mesh-rt/src/dist/node.rs` registration ABI | Refuse to emit startup/declared registrations when the runtime signature and codegen payload drift. | Not applicable beyond compile/test failure. | Fail LLVM marker tests when the emitted intrinsic arguments do not match the runtime contract. |

## Load Profile

- **Shared resources**: declared-handler registry and startup registration list.
- **Per-operation cost**: one extra integer field carried per declared handler / startup registration.
- **10x breakpoint**: registration drift or duplicate-name collisions, not CPU cost; the hot-path overhead should stay trivial.

## Negative Tests

- **Malformed inputs**: missing lowered handler symbols or missing runtime names fail before registration.
- **Error paths**: declared-handler and startup registration tests fail if count metadata is omitted from the emitted/runtime ABI.
- **Boundary conditions**: bare `@cluster` carries default `2`, explicit `@cluster(N)` preserves `N`, and service handlers do not accidentally become startup-work registrations.

## Steps

1. Extend `DeclaredHandlerPlanEntry`, `DeclaredRuntimeRegistration`, and the meshc planning seam to carry the resolved replication count from `ClusteredExecutionMetadata`.
2. Update LLVM/runtime registration plumbing so declared handlers register runtime name, executable symbol, and replication count together.
3. Store the registered count in the runtime declared-handler registry while keeping the startup registry name-only.
4. Add `m047_s02` unit coverage around codegen/runtime registration markers and declared-handler metadata lookup.

## Must-Haves

- [ ] Bare `@cluster` reaches runtime registration as count `2`.
- [ ] Explicit `@cluster(N)` reaches runtime registration without being clipped or dropped.
- [ ] The runtime can resolve replication-count metadata by declared-handler runtime name without inventing a second startup-only registry.
  - Estimate: 2h
  - Files: compiler/meshc/src/main.rs, compiler/mesh-codegen/src/declared.rs, compiler/mesh-codegen/src/codegen/mod.rs, compiler/mesh-codegen/src/codegen/intrinsics.rs, compiler/mesh-rt/src/dist/node.rs
  - Verify: cargo test -p mesh-codegen m047_s02 -- --nocapture && cargo test -p mesh-rt m047_s02 -- --nocapture
- [x] **T02: Continuity records now preserve replication counts and declared-work runtime truth derives required replicas from registered handler metadata.** — Make the runtime use the registered count honestly: derive required replicas from the declared-handler metadata, preserve the requested count in continuity state and operator surfaces, and fail closed when the current single-replica runtime cannot satisfy a requested replication factor.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Runtime continuity submit/merge/recovery paths in `compiler/mesh-rt/src/dist/continuity.rs` and `compiler/mesh-rt/src/dist/node.rs` | Reject the request with an explicit reason and keep the record queryable instead of silently degrading the count. | Keep the existing startup/recovery timeout path explicit and record the count-bearing rejection/timeout in diagnostics. | Treat malformed count/runtime metadata as invalid continuity state and reject before dispatch. |
| Operator serialization + CLI rendering in `compiler/mesh-rt/src/dist/operator.rs` and `compiler/meshc/src/cluster.rs` | Fail the query/CLI rail with a concrete field-mismatch error rather than dropping the new count field. | Surface the same operator timeout as today; do not invent a fallback record without count truth. | Reject decode drift between runtime/operator/CLI structs instead of silently omitting `replication_count`. |

## Load Profile

- **Shared resources**: continuity registry, operator query buffers, startup/recovery dispatch path.
- **Per-operation cost**: one extra count field carried through record encode/decode and a small amount of validation per submit/recovery.
- **10x breakpoint**: topology/count mismatch handling and record serialization drift, not raw CPU time.

## Negative Tests

- **Malformed inputs**: negative/oversized requested counts and missing runtime-name metadata reject explicitly.
- **Error paths**: unsupported replication factors or insufficient topology fail closed with durable continuity truth instead of local success.
- **Boundary conditions**: bare `@cluster` maps to replication count `2`, single-node clustered startup stays valid with count-aware semantics, and explicit count truth survives direct submit plus automatic recovery surfaces.

## Steps

1. Add `replication_count` to continuity record / Mesh continuity payload structs and update encode/decode, typeck builtins, and MIR builtin struct definitions together.
2. Derive `required_replica_count` from the registered declared-handler count for startup, direct submit, and automatic recovery instead of the current hardcoded `0`/`1` behavior.
3. Fail closed for unsupported requested replication factors or topologies instead of pretending the old single-replica runtime honored them.
4. Extend operator and `meshc cluster continuity` JSON/human output to render runtime name plus replication count, then add `m047_s02` runtime tests covering default count, explicit count preservation, and rejection paths.

## Must-Haves

- [ ] Runtime continuity records preserve requested replication count for ordinary clustered functions.
- [ ] Startup, direct submit, and recovery all derive replica requirements from declared-handler metadata instead of hardcoded defaults.
- [ ] Unsupported replication factors/topologies reject explicitly and stay visible through continuity/operator truth.
  - Estimate: 3h
  - Files: compiler/mesh-rt/src/dist/continuity.rs, compiler/mesh-rt/src/dist/node.rs, compiler/mesh-rt/src/dist/operator.rs, compiler/meshc/src/cluster.rs, compiler/mesh-typeck/src/builtins.rs, compiler/mesh-typeck/src/infer.rs, compiler/mesh-codegen/src/mir/lower.rs
  - Verify: cargo test -p mesh-rt m047_s02 -- --nocapture
- [x] **T03: Added M047 end-to-end coverage proving ordinary `@cluster` functions keep generic runtime names and truthful replication counts through LLVM registration and `meshc cluster continuity`.** — Prove the slice on the real user-facing seam: ordinary source-declared clustered functions using `@cluster` / `@cluster(N)` must surface generic runtime names and truthful replication counts through emitted LLVM markers and runtime-owned `meshc cluster continuity` output, without depending on the old `Work.execute_declared_work` package story.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Temp-project build/runtime harness in `compiler/meshc/tests/e2e_m047_s02.rs` and `compiler/meshc/tests/support/m046_route_free.rs` | Archive the failing build/runtime output and fail the e2e test with the concrete command stderr instead of masking it behind a generic assertion. | Reuse the existing continuity/status wait helpers so timeouts leave the last JSON/diagnostic observation on disk. | Treat malformed continuity JSON/human output as a failing proof of runtime truth, not as a parser quirk. |
| Shared M046 route-free helpers/regression rail | Keep the M046 regression rail green or fail closed before merging helper changes. | Treat helper timeout drift as a red regression and keep the last observation artifact. | Fail the helper assertions if runtime name/count fields disappear or change shape. |

## Load Profile

- **Shared resources**: temporary build output, spawned route-free runtime processes, operator query polling loops.
- **Per-operation cost**: one temp-project build plus one or two short-lived runtime processes per scenario.
- **10x breakpoint**: process startup/polling flake before business logic; preserve artifacts so the failure remains diagnosable.

## Negative Tests

- **Malformed inputs**: unsupported explicit counts reject with explicit continuity/runtime errors.
- **Error paths**: continuity JSON/human output missing `replication_count` or showing the legacy runtime name fails the rail.
- **Boundary conditions**: bare `@cluster` proves runtime count `2`, explicit `@cluster(3)` preserves `3` in truth even when runtime execution rejects unsupported fanout, and the shared M046 helper path still works if reused.

## Steps

1. Add `compiler/meshc/tests/e2e_m047_s02.rs` with temp-project fixtures using ordinary `@cluster` / `@cluster(N)` functions and generic runtime names like `Work.handle_submit`.
2. Reuse or extend `compiler/meshc/tests/support/m046_route_free.rs` only where needed so the new rail can poll continuity/status/diagnostics without reviving the old package story.
3. Assert emitted LLVM registration truth, runtime continuity JSON/human output, and explicit rejection behavior for unsupported counts/topologies.
4. Replay the shared `e2e_m046_s02` rail if helper plumbing changed so the new M047 proof does not regress the existing route-free startup contract.

## Must-Haves

- [ ] The new proof rail uses `@cluster` / `@cluster(N)` on ordinary functions, not `clustered(work)` fixtures.
- [ ] `meshc cluster continuity` JSON and human output prove both runtime name and replication count for the M047 fixture.
- [ ] Unsupported explicit counts fail closed with explicit runtime truth instead of silently appearing as successful mirrored execution.
  - Estimate: 3h
  - Files: compiler/meshc/tests/e2e_m047_s02.rs, compiler/meshc/tests/support/m046_route_free.rs, compiler/meshc/tests/e2e_m046_s02.rs
  - Verify: cargo test -p meshc --test e2e_m047_s02 -- --nocapture && cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture
