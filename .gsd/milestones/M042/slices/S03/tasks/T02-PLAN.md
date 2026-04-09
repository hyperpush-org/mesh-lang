---
estimated_steps: 24
estimated_files: 4
skills_used:
  - rust-best-practices
  - test
---

# T02: Plumb owner-loss and rejoin-safe recovery through node lifecycle hooks

Once the registry can roll attempts safely, wire real node-loss knowledge into that transition path. This task keeps owner-loss detection in `node.rs`, preserves the existing Mesh-facing `Continuity` API, and only touches `cluster-proof` if the runtime-owned status model needs a thin parsing or log-surface adjustment.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Node disconnect/reconnect hooks in `compiler/mesh-rt/src/dist/node.rs` | Mark owner loss explicitly and keep the surviving replica serving truthful status instead of leaving the lost owner authoritative forever. | A reconnecting node must not block the surviving node from keeping the newer attempt active. | Invalid sync/upsert payloads must be ignored in favor of the safer local record. |
| Thin `cluster-proof` continuity consumer in `cluster-proof/work.mpl` | Preserve the runtime-owned contract and fail closed on invalid JSON rather than reintroducing app-authored repair logic. | Pending status should stay readable from the surviving node during retry and rejoin polling. | Completion/log parsing must reject impossible field shapes instead of inventing success. |

## Load Profile

- **Shared resources**: Node session map, disconnect/connect callbacks, continuity sync payloads, and per-request status polling.
- **Per-operation cost**: One owner-loss transition per affected record plus the existing sync-on-connect upsert flow.
- **10x breakpoint**: Churny reconnects and repeated same-key retries will stress sync ordering first; reconnect paths must preserve the newest attempt without widening the API.

## Negative Tests

- **Malformed inputs**: Disconnect for an unrelated node, reconnect snapshot carrying an older attempt, and status JSON missing expected continuity fields.
- **Error paths**: Owner disappears while the request is still pending, retry arrives on the surviving node, and the old owner rejoins with stale replicated state.
- **Boundary conditions**: Owner-loss when the surviving node was the mirrored replica, owner-loss while the request is already terminal, and repeated disconnect/reconnect cycles for the same node identity.

## Steps

1. Add owner-loss continuity handling to `handle_node_disconnect(...)` and related runtime helpers so records whose `owner_node` disappears become recovery-eligible on the surviving replica while replica-loss downgrade behavior still works.
2. Keep recovery on the ordinary `Continuity.submit(...)` path by threading the minimal liveness/eligibility seam needed for same-key retry to roll a new attempt without widening the Mesh-facing API.
3. Verify that connect-time `send_continuity_sync(...)` plus the new merge precedence keep the latest attempt authoritative after same-identity rejoin.
4. Update `cluster-proof/work.mpl` and `cluster-proof/tests/work.test.mpl` only if the runtime-owned status/log contract needs thin parsing or failure-surface adjustments for owner-loss recovery.

## Must-Haves

- [ ] The runtime notices owner loss, not just replica loss, and leaves the surviving node able to serve truthful continuity status.
- [ ] Same-key recovery still goes through `Continuity.submit(...)`; no new Mesh-facing owner-repair API is introduced.
- [ ] Same-identity rejoin cannot resurrect an older attempt or stale owner mapping over the newer retry.
- [ ] Any `cluster-proof` changes remain thin consumer work: parsing, status mapping, or log-surface updates only.

## Inputs

- ``compiler/mesh-rt/src/dist/node.rs``
- ``compiler/mesh-rt/src/dist/continuity.rs``
- ``cluster-proof/work.mpl``
- ``cluster-proof/tests/work.test.mpl``

## Expected Output

- ``compiler/mesh-rt/src/dist/node.rs``
- ``compiler/mesh-rt/src/dist/continuity.rs``
- ``cluster-proof/work.mpl``
- ``cluster-proof/tests/work.test.mpl``

## Verification

cargo test -p mesh-rt continuity -- --nocapture && cargo run -q -p meshc -- test cluster-proof/tests

## Observability Impact

Makes owner-loss recovery and post-rejoin authority visible on the same runtime-owned status/log rail that operators already use, without pushing repair logic back into Mesh app code.
