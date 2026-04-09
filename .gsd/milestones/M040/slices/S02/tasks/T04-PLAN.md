---
estimated_steps: 4
estimated_files: 5
skills_used:
  - rust-best-practices
  - debug-like-expert
  - test
---

# T04: Add monitor-driven owner-loss continuation and a Rust e2e proof for convergence

**Slice:** S02 — Replica-Backed Admission, Fail-Closed Policy, and Owner-Loss Convergence
**Milestone:** M040

## Description

Replica-backed admission is still incomplete if an already-admitted request dies with its owner. This task makes continuation truthful by adding a long-lived monitor actor that owns `Node.monitor(...)`, reacts to `:nodedown`/`:nodeup`, and only rolls the current `attempt_id` forward when surviving replicated state proves that continuation is real.

Keep this narrowly scoped to restart-by-key continuity. Do not invent checkpoint replay, generic cluster-wide state migration, or deep attempt history. The request key stays stable, the active/current attempt becomes explicit, stale completions from dead attempts must be rejected, and the result must converge to one completed request even when the original owner disappears.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `Node.monitor(...)` / monitor actor loop | leave the request in a visible degraded state and do not claim automatic continuation happened | continuation must stay bounded; if recovery cannot happen, expose that truth explicitly | reject malformed node-event payloads and preserve last truthful continuity state |
| continuation dispatch / attempt rollover | do not overwrite the old record until the new active attempt is valid | reject or surface the failure instead of hanging the request forever | reject stale or mismatched completion payloads instead of marking success |
| Rust e2e harness | preserve artifact/log paths so failures localize to owner-loss behavior | treat readiness/continuation timeout as red; do not weaken assertions into eventual green guesses | fail the test if payloads or logs do not expose owner/replica/attempt truth |

## Load Profile

- **Shared resources**: node monitors, mirrored continuity maps, same-key retry/continuation coordination, and per-node process lifecycle.
- **Per-operation cost**: one monitor registration per watched node plus continuation dispatch and status updates when a node goes down.
- **10x breakpoint**: retry storms and many monitored requests would stress state coordination and stale-completion rejection before raw compute cost dominates.

## Negative Tests

- **Malformed inputs**: stale completion for the dead attempt, invalid continuation state, and retry requests that do not match the stored payload identity.
- **Error paths**: owner dies before completion, continuation dispatch fails, and a second retry arrives while continuation is already in flight.
- **Boundary conditions**: owner-loss with surviving replica, degraded cluster rejecting new durable work, and same-key retry converging to a single completion.

## Steps

1. Add a dedicated continuity monitor actor started from `start_work_services()` and move node-watch ownership there.
2. Define the owner-loss state transition: keep `request_key` stable, roll the active `attempt_id` forward only when surviving mirrored continuity exists, and reject stale completions from the superseded attempt.
3. Keep degraded-mode truth explicit so new durable submits fail closed while already-admitted replicated work can still continue.
4. Add `compiler/meshc/tests/e2e_m040_s02.rs` to prove owner-loss continuation, degraded new-submit rejection, and no duplicate completion leakage on a real two-node cluster.

## Must-Haves

- [ ] Owner-loss handling is driven by a dedicated monitor actor rather than by startup code or HTTP handlers guessing about node state.
- [ ] Continuation rolls the active `attempt_id` forward only when surviving replicated state exists, and stale old-attempt completions are rejected.
- [ ] The new Rust e2e test proves degraded rejection plus same-key convergent completion after owner loss.

## Verification

- Prove owner-loss continuation on the live clustered runtime.
- `cargo test -p meshc --test e2e_m040_s02 -- --nocapture`

## Observability Impact

- Signals added/changed: `nodedown`/`nodeup`, continuation-started, continuation-completed, and stale-completion-rejected logs keyed by `request_key` and `attempt_id`.
- How a future agent inspects this: run the new Rust e2e harness and inspect the `.tmp/m040-s02/` per-node stdout/stderr logs plus keyed status JSON.
- Failure state exposed: degraded-but-not-continuable requests, continuation failures, and stale-attempt rejects become explicit instead of silently disappearing.

## Inputs

- `cluster-proof/main.mpl` — startup hook that must spawn the continuity monitor actor.
- `cluster-proof/work.mpl` — continuity state machine, dispatch path, and keyed status surface.
- `cluster-proof/tests/work.test.mpl` — package-level state transition coverage to extend where possible.
- `compiler/meshc/tests/e2e_m039_s03.rs` — prior clustered lifecycle harness patterns and artifact discipline.

## Expected Output

- `cluster-proof/main.mpl` — startup wiring for the continuity monitor actor.
- `cluster-proof/work.mpl` — monitor-driven owner-loss transitions, active-attempt rollover, and stale-completion rejection.
- `cluster-proof/tests/work.test.mpl` — any package-level transition coverage that can be proven without hiding the real clustered boundary.
- `compiler/meshc/tests/e2e_m040_s02.rs` — live two-node proof for owner-loss continuation and degraded new-submit rejection.
