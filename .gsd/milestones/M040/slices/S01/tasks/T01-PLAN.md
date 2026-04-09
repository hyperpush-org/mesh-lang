---
estimated_steps: 4
estimated_files: 4
skills_used:
  - test
  - best-practices
---

# T01: Implement keyed submit/status state on the existing `/work` rail

**Slice:** S01 — Keyed Submit/Status Contract on the Existing Proof Rail
**Milestone:** M040

## Description

Replace the anonymous one-shot `/work` proof with a real keyed contract that later continuity slices can extend honestly. The task should keep the existing route-selection logic, but add a durable in-memory request registry keyed by caller-supplied `request_key`, distinct `attempt_id` issuance, truthful owner/replica visibility, and fail-closed duplicate semantics.

Do not paper over the contract with vague "accepted" responses. A fresh executor should leave `cluster-proof` able to answer two concrete questions: "what logical request does this key identify?" and "what is the latest truthful status for that request right now?"

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| keyed submit JSON parsing and key binding | reject the request with a clear client-visible error and do not mutate registry state | N/A — parsing is synchronous | fail closed with 400/409-style contract behavior instead of inventing defaults |
| in-memory request/attempt registry | surface the conflict or missing-state reason and keep the old status untouched | status lookup should return the last truthful phase rather than hanging | reject partial records or impossible state transitions instead of synthesizing a "success" snapshot |
| membership and route selection helpers | keep keyed state truthful but mark assignment fields from the real current selection only | bounded status/submit flows must return promptly instead of waiting on nonexistent replica durability | never claim owner/replica assignments that cannot be derived from the current cluster view |

## Load Profile

- **Shared resources**: in-memory request registry, attempt counter/state maps, and current membership snapshot data.
- **Per-operation cost**: one JSON parse, one registry lookup/update, one route-selection pass, and one status serialization.
- **10x breakpoint**: registry growth and repeated same-key polling will stress memory/state churn before CPU; keep writes bounded and avoid duplicating payload data per attempt.

## Negative Tests

- **Malformed inputs**: empty `request_key`, invalid JSON body, missing required keyed-work fields, and oversized/blank status lookup keys.
- **Error paths**: same key with a different payload, status lookup for an unknown key, and impossible state transitions in the request registry.
- **Boundary conditions**: first submit, idempotent same-key resubmit with identical payload, and single-node fallback where owner/replica placeholders must stay truthful.

## Steps

1. Extend `cluster-proof/work.mpl` so the `/work` namespace exposes keyed submit plus keyed status behavior, while keeping route-selection helpers reusable for later replica-backed admission.
2. Add request/attempt/status payload structs and registry helpers that bind one logical payload to each `request_key`, issue distinct `attempt_id` values, and preserve truthful owner/replica fields.
3. Fail-close conflicting same-key reuse, keep idempotent same-key same-payload behavior stable, and add request-scoped logs that expose `request_key`, `attempt_id`, and conflict/status transitions without echoing raw payloads.
4. Cover the new contract in `cluster-proof/tests/work.test.mpl` (and `cluster-proof/tests/config.test.mpl` if route/config wiring changes) so parsing, idempotency, and payload-shape rules are proven inside the Mesh package.

## Must-Haves

- [ ] Keyed submit returns a stable `request_key`, a distinct `attempt_id`, and a truthful initial status/admission snapshot.
- [ ] Keyed status lookup returns the latest durable keyed state with honest phase/result and owner/replica visibility fields.
- [ ] Same-key same-payload retries stay idempotent, while same-key different-payload retries fail closed without corrupting the stored request.

## Verification

- `cargo run -q -p meshc -- test cluster-proof/tests`
- `cargo run -q -p meshc -- build cluster-proof`

## Observability Impact

- Signals added/changed: keyed submit, dedupe/conflict, and status logs carrying `request_key`, `attempt_id`, assignment fields, and conflict/error reason.
- How a future agent inspects this: hit `POST /work` and `GET /work/:request_key`, then inspect `cluster-proof` stdout for the matching identifiers.
- Failure state exposed: unknown-key lookups, conflicting same-key reuse, and inconsistent assignment/state transitions become visible without reading internal state directly.

## Inputs

- `cluster-proof/main.mpl` — current HTTP router and startup wiring for the proof rail.
- `cluster-proof/work.mpl` — existing one-shot routing/state helpers that must evolve into keyed submit/status behavior.
- `cluster-proof/tests/work.test.mpl` — current package-level routing helper coverage to extend with keyed contract assertions.
- `cluster-proof/tests/config.test.mpl` — config/route expectations if the `/work` namespace wiring changes.

## Expected Output

- `cluster-proof/main.mpl` — updated route wiring for keyed submit/status on the `/work` rail.
- `cluster-proof/work.mpl` — keyed request/attempt/status registry plus handler logic.
- `cluster-proof/tests/work.test.mpl` — package tests covering keyed submit/status semantics.
- `cluster-proof/tests/config.test.mpl` — any route/config expectation updates required by the new contract.
