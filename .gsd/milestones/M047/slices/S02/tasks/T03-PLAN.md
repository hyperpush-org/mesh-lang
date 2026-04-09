---
estimated_steps: 4
estimated_files: 3
skills_used:
  - rust-testing
  - llvm
---

# T03: Add M047 end-to-end proofs for ordinary `@cluster` count semantics

Prove the slice on the real user-facing seam: ordinary source-declared clustered functions using `@cluster` / `@cluster(N)` must surface generic runtime names and truthful replication counts through emitted LLVM markers and runtime-owned `meshc cluster continuity` output, without depending on the old `Work.execute_declared_work` package story.

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

## Inputs

- ``compiler/meshc/tests/support/m046_route_free.rs``
- ``compiler/meshc/tests/e2e_m046_s02.rs``
- ``compiler/meshc/src/cluster.rs``
- ``compiler/mesh-rt/src/dist/continuity.rs``
- ``compiler/mesh-rt/src/dist/node.rs``

## Expected Output

- ``compiler/meshc/tests/e2e_m047_s02.rs``
- ``compiler/meshc/tests/support/m046_route_free.rs``
- ``compiler/meshc/tests/e2e_m046_s02.rs``

## Verification

cargo test -p meshc --test e2e_m047_s02 -- --nocapture && cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture

## Observability Impact

The M047 e2e rail should archive LLVM output, continuity JSON/human output, and last-observation timeout artifacts so count/runtime-name drift stays debuggable after a failure.
