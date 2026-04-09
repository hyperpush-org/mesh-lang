---
estimated_steps: 25
estimated_files: 6
skills_used:
  - rust-best-practices
  - test
---

# T02: Plumb the durability-aware Continuity API and remove app-authored mirrored truth from cluster-proof

Once the runtime owns admission truth, the Mesh-facing API and `/work` handlers need to stop papering over it. This task keeps the compiler/runtime seam small, passes the durability requirement through `Continuity.submit(...)`, and makes `cluster-proof` replay rejected or mirrored state exactly as stored.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Continuity runtime ABI / compiler intrinsic mapping | Fail the build or tests loudly; do not leave partially updated arity or symbol mappings. | N/A for compile-time plumbing. | Treat unexpected JSON or field shape from runtime as a parse failure and return the stored failure payload instead of inventing success. |
| `cluster-proof` submit/status response mapping | Return the truthful HTTP failure (`503` or `409`) with stored status payload, not a synthetic success path. | Keep status reads on the existing runtime lookup path; do not add app-local fallback state. | Reject invalid runtime JSON with an explicit failure response instead of silently dropping fields. |

## Load Profile

- **Shared resources**: Mesh compiler intrinsic declarations, runtime ABI surface, and the `/work` HTTP entrypoint that may see repeated retries for the same key.
- **Per-operation cost**: Trivial compile-time changes plus one runtime submit/status JSON parse per HTTP request.
- **10x breakpoint**: Retry storms against the same key must still replay stored duplicate/rejected truth without dispatching extra work.

## Negative Tests

- **Malformed inputs**: Bad request key / payload, unexpected JSON from runtime, and invalid durability-policy-derived replica requirement.
- **Error paths**: Runtime submit returns rejected durable admission, duplicate replay of rejected record, and conflicting same-key reuse.
- **Boundary conditions**: Standalone/local-only submit still works, cluster-mode replica-backed submit with no replica returns `503`, and duplicate success vs duplicate rejection map to different `ok` values.

## Steps

1. Update the `Continuity` stdlib typing and LLVM intrinsic declaration/lowering so `Continuity.submit(...)` carries the new durability argument while keeping the API continuity-specific.
2. Export any adjusted runtime symbols from `compiler/mesh-rt/src/lib.rs` and make `cluster-proof/work.mpl` pass `required_replica_count(current_durability_policy())` into runtime submit.
3. Remove `acknowledged_replica_record(...)` from the live submit path so `cluster-proof` no longer manufactures `mirrored` state after the fact.
4. Add truthful submit/duplicate/status response mapping for rejected durable admission and preserve the existing same-key conflict contract.
5. Update `cluster-proof/tests/work.test.mpl` for helper-level response and policy behavior that can be exercised without the live cluster harness.

## Must-Haves

- [ ] The compiler/runtime seam compiles with the new `Continuity.submit(...)` arity and no stale acknowledge-based happy path.
- [ ] `cluster-proof` passes the runtime durability requirement instead of inferring safety from `replica_node` alone.
- [ ] `POST /work` returns stored rejected truth when durability admission fails and does not dispatch work in that branch.
- [ ] Duplicate replay is truthful for both successful and rejected stored records.

## Inputs

- ``compiler/mesh-rt/src/dist/continuity.rs``
- ``compiler/mesh-typeck/src/infer.rs``
- ``compiler/mesh-codegen/src/codegen/intrinsics.rs``
- ``compiler/mesh-codegen/src/mir/lower.rs``
- ``compiler/mesh-rt/src/lib.rs``
- ``cluster-proof/work.mpl``
- ``cluster-proof/config.mpl``

## Expected Output

- ``compiler/mesh-typeck/src/infer.rs``
- ``compiler/mesh-codegen/src/codegen/intrinsics.rs``
- ``compiler/mesh-codegen/src/mir/lower.rs``
- ``compiler/mesh-rt/src/lib.rs``
- ``cluster-proof/work.mpl``
- ``cluster-proof/tests/work.test.mpl``

## Verification

cargo run -q -p meshc -- test cluster-proof/tests

## Observability Impact

Sharpens the `/work` failure surface so operators can distinguish conflict, rejected admission, and mirrored acceptance from ordinary status responses and submit logs.
