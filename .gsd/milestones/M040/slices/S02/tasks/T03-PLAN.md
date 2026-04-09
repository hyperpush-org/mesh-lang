---
estimated_steps: 4
estimated_files: 4
skills_used:
  - rust-best-practices
  - test
---

# T03: Require replica-backed admission and mirrored continuity status

**Slice:** S02 — Replica-Backed Admission, Fail-Closed Policy, and Owner-Loss Convergence
**Milestone:** M040

## Description

This is the core durability seam. S01 proved a keyed request/status contract, but it is still ingress-local: the current submit path writes only to the local registry, returns HTTP 200 before any replica acknowledgment, and keyed status only reads local state. That is not honest continuity.

Keep continuity state app-owned inside `cluster-proof/work.mpl`. Add a prepare/ack flow that mirrors the minimal request record onto the chosen owner and replica before durable admission succeeds. Status must report current owner/replica truth from that mirrored continuity state, not from front-door guesses. Reuse the global registry only to discover per-node continuity actors; do not promote it into a state database.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| local/remote continuity registry lookup | reject durable admission and surface a replica-unavailable style reason without mutating misleading state | reject durable admission instead of hanging the caller | reject malformed registry replies and keep the request unaccepted |
| owner/replica prepare/ack flow | keep the request out of the accepted durable set and surface the failure reason | fail closed; do not return HTTP 200 without replica acknowledgment | reject partial records or mismatched payload hashes rather than fabricating continuity |
| keyed status lookup across involved nodes | return the last truthful mirrored record or an explicit missing/rejected state | keep lookup bounded and return an explicit failure if continuity cannot be resolved | reject impossible state shapes instead of synthesizing a “healthy” status |

## Load Profile

- **Shared resources**: mirrored request maps on the owner and replica, cross-node prepare/ack messaging, and same-key contention on continuity records.
- **Per-operation cost**: one local prepare, one remote prepare/ack, and one keyed status serialization from mirrored state.
- **10x breakpoint**: repeated same-key retries and mirrored record churn will stress registry messaging/state coordination before CPU becomes a concern.

## Negative Tests

- **Malformed inputs**: same `request_key` with a different payload, malformed mirrored record fields, and empty owner/replica identity when a durable admission is attempted.
- **Error paths**: remote replica lookup failure, replica prepare timeout, and status lookup when only a partial/unaccepted record exists.
- **Boundary conditions**: first durable submit, idempotent same-key same-payload retry, and healthy two-node status lookup from either involved node.

## Steps

1. Define the minimal mirrored continuity record for S02: stable `request_key`, current `attempt_id`, payload identity, owner/replica identity, phase/result, and explicit rejection/error reason.
2. Add a prepare/ack path that writes mirrored continuity state onto owner and replica before `POST /work` can return durable success.
3. Make `GET /work/:request_key` answer from mirrored continuity truth rather than only the ingress-local registry.
4. Update package tests and preserve the S01 standalone keyed verifier so the healthy standalone contract stays green while clustered durability gets stricter.

## Must-Haves

- [ ] New durable keyed work only returns HTTP 200 after both owner and replica continuity state exist.
- [ ] Under-replicated durable submissions fail closed with an explicit reason instead of fake success.
- [ ] Keyed status surfaces truthful `owner_node`, `replica_node`, `replica_status`, and the current `attempt_id` from mirrored continuity state.

## Verification

- Prove the new mirrored admission/state rules in package tests.
- `cargo run -q -p meshc -- test cluster-proof/tests`
- `bash scripts/verify-m040-s01.sh`

## Observability Impact

- Signals added/changed: replica prepare, replica ack, durable rejection, and continuity-status logs keyed by `request_key` and `attempt_id`.
- How a future agent inspects this: use the keyed HTTP contract plus `cluster-proof` stdout to trace owner/replica prepare and status answers.
- Failure state exposed: missing replica ack, partial continuity records, and under-replicated rejection become visible without opening internal state directly.

## Inputs

- `cluster-proof/main.mpl` — current HTTP wiring for submit/status on the `/work` rail.
- `cluster-proof/work.mpl` — current keyed registry, submit path, and status path to extend into replica-backed continuity.
- `cluster-proof/tests/work.test.mpl` — package-level keyed contract coverage to extend with mirrored-admission truth.
- `scripts/verify-m040-s01.sh` — baseline keyed contract verifier that must remain green.

## Expected Output

- `cluster-proof/main.mpl` — HTTP/status wiring updated only if the status path needs new continuity-aware lookup behavior.
- `cluster-proof/work.mpl` — mirrored continuity record, prepare/ack flow, and truthful status logic.
- `cluster-proof/tests/work.test.mpl` — package tests covering replica-backed admission and truthful mirrored status.
