---
estimated_steps: 4
estimated_files: 4
skills_used:
  - test
---

# T02: Resume declared work automatically after promoted failover

**Slice:** S04 — Bounded Automatic Promotion
**Milestone:** M044

## Description

A promoted standby that still needs a client retry does not satisfy R068. This task closes the runtime-owned recovery gap by preserving the declared-handler identity needed to redispatch work after owner loss, then proving the promoted standby completes the request without `POST /promote` or a second submit.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Declared-handler registry and runtime-name lookup in `compiler/mesh-rt/src/dist/node.rs` | Reject recovery and leave an explicit recovery error on the continuity record. | Keep the request pending with diagnosable recovery state; do not fabricate completion. | Reject redispatch metadata as invalid and fail closed. |
| Continuity retry-rollover state in `compiler/mesh-rt/src/dist/continuity.rs` | Preserve the owner-lost record and surface the failed recovery reason. | Keep the request observable as pending, owner-lost, or degraded instead of dropping it. | Refuse rollover if request/attempt metadata is inconsistent. |
| Same-image destructive proof in `compiler/meshc/tests/e2e_m044_s04.rs` | Fail the test rail and retain artifacts/logs instead of masking the missing recovery. | Bound the wait around recovery transitions and surface which stage stalled. | Reject malformed JSON/artifact state instead of passing on missing proof. |

## Load Profile

- **Shared resources**: continuity record storage, declared-handler registry lookups, remote spawn/session transport, and same-image artifact capture.
- **Per-operation cost**: one recovery metadata lookup plus one redispatch for each eligible owner-lost request.
- **10x breakpoint**: repeated owner-loss events on many mirrored requests can amplify redispatch pressure; the recovery path must stay keyed, bounded, and idempotent.

## Negative Tests

- **Malformed inputs**: missing runtime handler metadata, mismatched request or attempt ids, and duplicate or stale recovery records.
- **Error paths**: redispatch failure on the promoted standby, recovery attempted while promotion is still ambiguous, and stale-primary same-key traffic after rejoin.
- **Boundary conditions**: one pending mirrored request, already-completed request, and owner-loss followed by a healthy rejoin with no duplicate execution.

## Steps

1. Persist the declared-handler recovery metadata the runtime needs at submit time so owner-loss recovery does not depend on an external caller still holding the handler name.
2. Extend the runtime recovery path to redispatch eligible `owner_lost` declared work automatically after a safe promotion using a new attempt fence.
3. Prove that the promoted standby completes the request without a second submit while the old primary never executes the recovered attempt after rejoin.
4. Retain artifact and log assertions that distinguish auto-resume from the old manual-promote-plus-retry path.

## Must-Haves

- [ ] Declared clustered work survives primary loss through runtime-owned auto-resume, not just through retry eligibility.
- [ ] Recovery uses a new fenced attempt id and remains idempotent under stale-primary rejoin.
- [ ] The proof rail shows completion on the promoted standby with no manual promote call and no same-key client retry.

## Verification

- `cargo test -p mesh-rt automatic_recovery_ -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_resume_ -- --nocapture`
- `cargo run -q -p meshc -- test cluster-proof/tests`

## Observability Impact

- Signals added/changed: recovery-specific attempt, handler, and authority evidence in runtime diagnostics and proof artifacts.
- How a future agent inspects this: inspect the `e2e_m044_s04` bundle for pre-loss, auto-promoted, recovered, and rejoin status JSON plus the corresponding runtime logs.
- Failure state exposed: whether recovery failed at metadata capture, auto-promotion, redispatch, or stale-primary fencing.

## Inputs

- `compiler/mesh-rt/src/dist/node.rs` — current declared-handler registry and dispatch path.
- `compiler/mesh-rt/src/dist/continuity.rs` — current retry-rollover and owner-loss record logic.
- `compiler/meshc/tests/e2e_m043_s03.rs` — current failover proof that still relies on a manual retry.
- `cluster-proof/tests/work.test.mpl` — existing package-level continuity payload assertions.

## Expected Output

- `compiler/mesh-rt/src/dist/node.rs` — runtime-owned recovery metadata capture and redispatch.
- `compiler/mesh-rt/src/dist/continuity.rs` — recovery-aware continuity transitions and tests.
- `compiler/meshc/tests/e2e_m044_s04.rs` — destructive proof that the promoted standby finishes automatically.
- `cluster-proof/tests/work.test.mpl` — package tests aligned to the new auto-resume payload truth.
