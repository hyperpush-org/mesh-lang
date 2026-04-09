---
id: S03
parent: M042
milestone: M042
provides:
  - Runtime-owned owner-loss continuity status and same-key retry rollover on the stable local-owner rail.
  - Stale-completion and stale-terminal-merge fencing based on newer attempt-token precedence.
  - A fail-closed local acceptance rail with retained owner-loss and rejoin evidence under `.tmp/m042-s03/verify/`.
requires:
  - slice: S02
    provides: Replica-backed admission truth, mirrored/degraded continuity status, and the stable local-owner durability rail that S03 extends into owner-loss recovery and rejoin truth.
affects:
  - S04
key_files:
  - compiler/mesh-rt/src/dist/continuity.rs
  - compiler/mesh-rt/src/dist/node.rs
  - cluster-proof/work.mpl
  - cluster-proof/tests/work.test.mpl
  - compiler/meshc/tests/e2e_m042_s03.rs
  - scripts/verify-m042-s03.sh
  - .gsd/PROJECT.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Kept the Mesh-facing `Continuity.submit/status/mark_completed` API unchanged and modeled owner-loss recovery as ordinary same-key retry that rolls a new `attempt_id` fencing token.
  - Marked pending mirrored records with explicit `replica_status=owner_lost` during node disconnect handling and made ordinary `Continuity.submit(...)` the only recovery entrypoint.
  - Kept `cluster-proof` thin by lowering the submit-time replica requirement to `0` only when runtime status already reports `phase=submitted`, `result=pending`, and `replica_status=owner_lost`.
patterns_established:
  - Use `attempt_id` as the cross-node fencing token before terminal/non-terminal precedence so stale completions and stale replicated terminal records cannot overwrite a newer retry.
  - Keep node-loss detection in `node.rs`, write explicit owner-loss state into runtime continuity records, and let ordinary submit/status APIs consume that state instead of inventing a repair-only Mesh surface.
  - For destructive continuity proofs, replay one shared e2e run and archive the phase-specific artifacts copied from that run rather than relying on isolated timing-sensitive filters.
observability_surfaces:
  - `GET /work/:request_key` status now exposes owner-loss and recovery truth through `attempt_id`, `phase`, `result`, `owner_node`, `replica_node`, and `replica_status`.
  - `[mesh-rt continuity] transition=owner_lost ...` log lines provide the durable runtime-side failure transition signal for owner loss.
  - `.tmp/m042-s03/verify/` preserves the copied owner-loss and rejoin HTTP/status/log artifacts plus phase reports, making the first failing phase obvious without rerunning the whole slice.
drill_down_paths:
  - .gsd/milestones/M042/slices/S03/tasks/T01-SUMMARY.md
  - .gsd/milestones/M042/slices/S03/tasks/T02-SUMMARY.md
  - .gsd/milestones/M042/slices/S03/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-29T01:03:50.825Z
blocker_discovered: false
---

# S03: Owner-loss recovery, same-key retry, and stale-completion safety

**Runtime-native keyed continuity now survives owner loss on the stable local-owner rail by surfacing `owner_lost` status, rolling same-key retries to a newer `attempt_id`, fencing stale completion, and preserving the newer attempt across same-identity rejoin.**

## What Happened

S03 moved the keyed continuity contract from replica-backed degradation truth into actual owner-loss recovery.

First, `mesh-rt`’s continuity registry learned how to treat `attempt_id` as the cluster-wide fencing token. Same-key retry after owner loss now rolls a fresh attempt instead of replaying a permanent duplicate, stale `mark_completed(old_attempt)` fails with `attempt_id_mismatch`, and stale completed or mirrored remote records can no longer overwrite a newer active retry through merge precedence or snapshot replay. The registry coverage now explicitly exercises rollover, stale completion rejection, stale terminal merge rejection, owner-loss idempotence, and monotonic `next_attempt_token` behavior.

Next, the node lifecycle path started writing explicit owner-loss truth into runtime continuity state. When the active owner disappears, surviving mirrored replicas transition to `replica_status=owner_lost` and stay readable through the ordinary continuity status surface. Recovery eligibility stays runtime-owned: the Mesh-facing API did not widen, and same-key recovery continues through normal `Continuity.submit(...)` using the newer attempt token as the authoritative fence.

Finally, the proof app and harness were tightened around that runtime contract instead of reintroducing app-authored repair logic. `cluster-proof/work.mpl` only drops the submit-time replica requirement to `0` when the existing runtime status for the same request key is still pending and explicitly `owner_lost`; otherwise the ordinary durability policy remains intact. `compiler/meshc/tests/e2e_m042_s03.rs` added two destructive two-node scenarios: one proving truthful owner-loss status plus retry rollover on the surviving node, and one proving same-identity rejoin plus stale-completion safety. `scripts/verify-m042-s03.sh` now replays `mesh-rt` continuity tests, `cluster-proof` tests/build, the full S02 verifier, then one shared S03 e2e replay before copying the owner-loss and rejoin evidence bundles under `.tmp/m042-s03/verify/`.

The assembled result is a runtime-owned owner-loss recovery contract that stays narrow and truthful: the surviving replica can still answer keyed status, same-key retry converges by rolling to a newer attempt, stale old-attempt completion is fenced out, and rejoin cannot resurrect the superseded attempt.

## Verification

Verified the full slice rail at three levels.

- Runtime correctness: `cargo test -p mesh-rt continuity -- --nocapture` passed with 24 continuity-focused tests covering retry rollover, owner-loss transitions, stale-completion rejection, stale terminal merge rejection, and monotonic attempt-token behavior.
- Thin consumer contract: `cargo run -q -p meshc -- test cluster-proof/tests` passed with the keyed-work and config contract suite green.
- Live destructive proof: `cargo test -p meshc --test e2e_m042_s03 -- --nocapture` passed serially with both named scenarios green, and `bash scripts/verify-m042-s03.sh` passed after replaying runtime continuity tests, cluster-proof tests/build, `bash scripts/verify-m042-s02.sh`, one shared S03 e2e replay, and the retained artifact checks.

Observability was also checked explicitly: `.tmp/m042-s03/verify/status.txt` is `ok`, `.tmp/m042-s03/verify/current-phase.txt` is `complete`, `.tmp/m042-s03/verify/phase-report.txt` shows all phases passed, and the copied owner-loss/rejoin artifact manifests point at the expected HTTP and per-node log bundles.

## Requirements Advanced

- R050 — Retired the remaining single-cluster owner-loss gap on top of S02 by adding runtime-written `owner_lost` state, ordinary-submit retry rollover, stale-completion fencing, and same-identity rejoin truth on the stable local-owner rail.

## Requirements Validated

- R050 — `cargo test -p mesh-rt continuity -- --nocapture`, `cargo run -q -p meshc -- test cluster-proof/tests`, `cargo test -p meshc --test e2e_m042_s03 -- --nocapture`, `bash scripts/verify-m042-s02.sh`, and `bash scripts/verify-m042-s03.sh` all passed. The retained `.tmp/m042-s03/verify/` bundle shows owner-loss status truth, retry rollover to a newer attempt, stale-completion guard behavior, and post-rejoin status convergence on the same newer attempt.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None. The slice closed on the planned runtime-first path: registry fencing first, owner-loss eligibility in the runtime next, then the destructive two-node harness and verifier wrapper.

## Known Limitations

This slice proves owner-loss recovery on the stable local-owner rail only. It does not claim arbitrary process-state migration or exactly-once execution, and the older healthy two-node remote-owner execution path is still blocked by the separate remote `Node.spawn` string-argument/runtime crash. Cross-cluster disaster failover remains M043.

## Follow-ups

S04 should finish the thin-consumer/operator/docs reconciliation so the repo’s public continuity story points at the runtime-owned capability instead of the old app-authored machinery. Separately, the broader healthy two-node remote-owner execution path is still blocked by the older remote `Node.spawn` string-argument/runtime crash and remains outside this slice’s local-owner acceptance rail.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/continuity.rs` — Added recovery-aware retry rollover, attempt-token-first merge precedence, stale-completion fencing, and continuity registry coverage.
- `compiler/mesh-rt/src/dist/node.rs` — Marked pending mirrored records as owner-lost on disconnect and exposed runtime recovery eligibility without widening the Mesh-facing API.
- `cluster-proof/work.mpl` — Kept cluster-proof as a thin continuity consumer by dropping replica requirements only for runtime-reported owner_lost pending records.
- `cluster-proof/tests/work.test.mpl` — Extended thin-consumer contract coverage around keyed recovery submit behavior.
- `compiler/meshc/tests/e2e_m042_s03.rs` — Added the owner-loss recovery and same-identity rejoin end-to-end harness with preserved per-phase artifacts.
- `scripts/verify-m042-s03.sh` — Added the fail-closed S03 verifier that replays prerequisites and archives owner-loss and rejoin proof bundles.
- `.gsd/PROJECT.md` — Refreshed current project state now that S03 is green.
- `.gsd/KNOWLEDGE.md` — Recorded the proof-rail serialization gotcha for future agents.
