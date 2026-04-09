---
estimated_steps: 24
estimated_files: 3
skills_used:
  - rust-best-practices
  - debug-like-expert
---

# T01: Implement runtime-owned durable admission, replica-state transitions, and disconnect downgrade truth

Introduce the real S02 runtime boundary inside `mesh-rt`: continuity submit must know whether replica safety is required, record rejected admissions durably, and downgrade mirrored work to `degraded_continuing` when replica safety is later lost.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Inter-node continuity message handling in `compiler/mesh-rt/src/dist/node.rs` | Keep the owner-side record truthful and reject or degrade instead of assuming replica safety. | Treat missing replica confirmation as non-admission or post-admission degradation; never silently keep `mirrored`. | Ignore invalid remote continuity payloads after validation and keep the safer local record. |
| Continuity registry merge rules in `compiler/mesh-rt/src/dist/continuity.rs` | Prefer terminal / safer replica truth and return explicit errors for invalid transitions. | Do not block the registry indefinitely on peer state; record the local durable decision and surface it. | Reject mismatched attempt/payload/owner data instead of merging it into the request record. |

## Load Profile

- **Shared resources**: Continuity registry lock, node session map, continuity upsert/sync traffic, and per-request log volume.
- **Per-operation cost**: One registry mutation plus replica prepare/ack or reject bookkeeping, followed by one or more continuity messages.
- **10x breakpoint**: Session write failures and lock contention will show up first; the task must keep merge precedence monotonic so extra message volume cannot regress truth.

## Negative Tests

- **Malformed inputs**: Missing request key / payload hash / owner node, replica equal to owner, invalid attempt ID, and empty replica on replica-required submit.
- **Error paths**: Replica prepare or ack unavailable, stale mirrored upsert arriving after degradation, and disconnect occurring while work is still pending.
- **Boundary conditions**: `required_replica_count = 0` vs `1`, rejected duplicate replay, conflict after rejection, and repeated degrade/ack transitions.

## Steps

1. Extend `SubmitRequest` / `SubmitDecision` and the continuity transition helpers so submit admission is durability-aware instead of inferring policy from `replica_node` shape alone.
2. Add the runtime-owned prepare/ack-or-reject path that persists rejected records with stable `phase`, `result`, `replica_status`, and `error` fields instead of relying on fire-and-forget upserts.
3. Update disconnect handling and merge precedence so accepted mirrored work downgrades to `degraded_continuing` when replica safety is lost, and stale mirrored data cannot overwrite later safer truth.
4. Expand the continuity unit tests around rejection replay, conflict preservation, mirrored acceptance, degrade-on-disconnect, and merge monotonicity.

## Must-Haves

- [ ] Submitting with replica safety required can return durable rejected truth, not just `created` / `duplicate` / `conflict`.
- [ ] Rejected records are stored and replayed on same-key same-payload retry.
- [ ] Disconnect handling can surface `degraded_continuing` for surviving records that previously had replica safety.
- [ ] `preferred_record(...)` / snapshot merge logic does not let older mirrored state overwrite newer degraded or rejected truth.

## Inputs

- ``compiler/mesh-rt/src/dist/continuity.rs``
- ``compiler/mesh-rt/src/dist/node.rs``
- ``compiler/mesh-rt/src/lib.rs``
- ``cluster-proof/work.mpl``

## Expected Output

- ``compiler/mesh-rt/src/dist/continuity.rs``
- ``compiler/mesh-rt/src/dist/node.rs``
- ``compiler/mesh-rt/src/lib.rs``

## Verification

cargo test -p mesh-rt continuity -- --nocapture

## Observability Impact

Adds explicit runtime rejection/degrade transitions and keeps continuity logs useful for distinguishing replica-admission failure from later replica-loss degradation.
