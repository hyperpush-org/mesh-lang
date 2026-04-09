---
estimated_steps: 35
estimated_files: 7
skills_used: []
---

# T02: Move declared work placement and continuity dispatch into the runtime

---
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

## Inputs

- ``compiler/mesh-rt/src/dist/continuity.rs` — existing continuity registry and typed public payload ABI from S01.`
- ``compiler/mesh-rt/src/dist/node.rs` — current remote spawn transport and node-session continuity hooks.`
- ``compiler/mesh-codegen/src/mir/lower.rs` — lowering seam that must map declared work entrypoints onto runtime-owned execution.`
- ``cluster-proof/work.mpl` — current app-owned placement logic being moved behind the runtime boundary.`

## Expected Output

- ``compiler/mesh-rt/src/dist/continuity.rs` — runtime-owned declared work placement/submit/dispatch state transitions.`
- ``compiler/mesh-rt/src/dist/node.rs` — declared-work registration/dispatch transport that no longer depends on app-owned routing helpers.`
- ``compiler/mesh-codegen/src/codegen/intrinsics.rs` — runtime ABI declarations for the new declared-work execution path.`
- ``compiler/meshc/tests/e2e_m044_s02.rs` — named `m044_s02_declared_work_` coverage for the runtime-owned work path.`

## Verification

`cargo test -p meshc --test e2e_m044_s02 m044_s02_declared_work_ -- --nocapture`
`cargo test -p meshc --test e2e_m044_s01 m044_s01_typed_continuity_ -- --nocapture`

## Observability Impact

- Signals added/changed: runtime logs identify declared target, request key, attempt id, owner/replica, and dispatch decision for clustered work submission.
- How a future agent inspects this: run the named `m044_s02_declared_work_` filter and inspect the retained e2e bundle plus continuity logs.
- Failure state exposed: submit outcome, execution node, replica requirement, and reject/conflict reason.
