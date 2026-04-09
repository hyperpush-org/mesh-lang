---
id: S02
parent: M043
milestone: M043
provides:
  - Explicit standby promotion backed by runtime-owned authority mutation.
  - Runtime-owned same-key recovery rollover from lost-primary attempt to promoted-standby attempt.
  - Higher-epoch stale-primary fencing on merge/rejoin with retained artifact proof.
requires:
  - slice: S01
    provides: runtime-owned primary/standby role, promotion epoch, and replication-health truth mirrored onto standby membership and keyed status surfaces before promotion
affects:
  - S03
  - S04
key_files:
  - compiler/mesh-rt/src/dist/continuity.rs
  - compiler/mesh-rt/src/lib.rs
  - cluster-proof/work_continuity.mpl
  - compiler/meshc/tests/e2e_m043_s02.rs
  - scripts/verify-m043-s02.sh
  - compiler/meshc/tests/e2e_m043_s01.rs
  - compiler/meshc/tests/e2e_m042_s03.rs
key_decisions:
  - Keep standby promotion and live authority status runtime-owned behind narrow `Continuity.promote()` and `Continuity.authority_status()` APIs instead of adding Mesh-side failover orchestration.
  - Store mutable continuity authority inside the runtime registry so promotion can mutate authority without discarding mirrored request state.
  - Treat pre-submit two-node membership as `replication_health=local_only`; only mirrored continuity records advance runtime authority health to `healthy`.
  - Keep the older M042 owner-loss rejoin regression on an explicit primary/primary topology after M043 so it continues proving single-cluster continuity instead of silently drifting into primary/standby semantics.
patterns_established:
  - Keep failover authority, promotion, merge precedence, and fencing in `mesh-rt`; let Mesh code consume only narrow built-ins and runtime-authored JSON truth.
  - Treat startup role/epoch env as topology input only. After startup, all operator-visible authority truth must come from `Continuity.authority_status()`.
  - For destructive continuity slices, make the shell verifier replay the previous slice rails first, then copy the shared artifact directory and validate the preserved JSON/log contract instead of trusting test exit codes alone.
  - When bracketed IPv6 node names appear in retained logs, verifier checks should use literal matching rather than raw regex interpolation.
observability_surfaces:
  - `GET /membership` exposes runtime-owned `cluster_role`, `promotion_epoch`, and `replication_health`.
  - `GET /work/:request_key` shows attempt rollover, owner/replica truth, and fenced stale-primary replay after rejoin.
  - `POST /promote` is the explicit operator boundary and returns runtime authority truth on success or fail-closed authority errors on rejection.
  - `[mesh-rt continuity] transition=*` stderr logs record `promote`, `recovery_rollover`, and `fenced_rejoin` transitions for postmortem comparison.
  - `.tmp/m043-s02/verify/phase-report.txt` and `07-failover-artifacts/` are the authoritative retained evidence bundle for the destructive failover contract.
drill_down_paths:
  - .gsd/milestones/M043/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M043/slices/S02/tasks/T02-SUMMARY.md
  - .gsd/milestones/M043/slices/S02/tasks/T03-SUMMARY.md
  - .gsd/milestones/M043/slices/S02/tasks/T04-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-29T09:48:05.427Z
blocker_discovered: false
---

# S02: Standby Promotion and Stale-Primary Fencing

**Mesh now survives primary-cluster loss through explicit standby promotion, runtime-owned same-key attempt rollover, and stale-primary fencing, with a fail-closed local verifier bundle that preserves the full failover/rejoin evidence.**

## What Happened

S02 closed the actual disaster-continuity seam rather than just exposing more status. At the runtime layer, continuity authority moved from process-static state into the live registry so explicit promotion could mutate role and epoch without discarding the mirrored in-memory continuity map. Merge and rejoin precedence now fence stale lower-epoch primaries instead of projecting incoming records into local authority before comparison, which is what lets an old primary come back as a deposed standby instead of reclaiming authority.

On the Mesh/compiler seam, S02 added the minimal runtime-owned surface the proof app needed: `Continuity.promote()` to perform the explicit operator-approved promotion step and `Continuity.authority_status()` to read current role/epoch/health directly from `mesh-rt`. That let `cluster-proof` stop deriving live authority from startup env after failover. The app remained a thin consumer: cluster mode still uses env only for startup topology, `/membership` and keyed `/work/:request_key` now reflect runtime-owned authority truth, and `/promote` is the one explicit operator boundary.

The closeout work then hardened the assembled proof rail. `scripts/verify-m043-s02.sh` now replays the stable prerequisites, the S01 mirrored-standby verifier, the targeted M042 same-identity rejoin regression, the new continuity API filter, and the destructive failover harness. It fails closed on missing named-test counts, copies the retained failover artifact directory into `.tmp/m043-s02/verify/07-failover-artifacts/`, and validates the full contract from preserved JSON and logs: pre-failover primary/standby truth, degraded standby after primary loss, explicit promotion to epoch 1, runtime-owned recovery rollover to a new attempt, completion on the promoted standby, and fenced/deposed old-primary rejoin.

During closeout, two older proof surfaces needed repair so the assembled slice could stay truthful. The S01 e2e and verifier still expected two-node pre-submit membership health to be `healthy`, but the shipped S02 runtime now reports `local_only` until mirrored continuity exists; those expectations were updated instead of weakening the runtime. The targeted M042 rejoin regression also needed explicit cluster-mode role/epoch env plus a primary/primary topology so it continued proving single-cluster owner-loss fencing rather than drifting into the newer primary/standby standby-degrade semantics.

## Verification

Verified by `bash scripts/verify-m043-s02.sh` after repairing the stale prerequisite proof surfaces. That replay passed end to end and reran the slice-plan checks inside the assembled contract: `cargo test -p mesh-rt continuity -- --nocapture`, `cargo run -q -p meshc -- test cluster-proof/tests`, `cargo run -q -p meshc -- build cluster-proof`, `bash scripts/verify-m043-s01.sh`, `cargo test -p meshc --test e2e_m042_s03 continuity_api_same_identity_rejoin_preserves_newer_attempt_truth -- --nocapture`, `cargo test -p meshc --test e2e_m043_s02 continuity_api_ -- --nocapture`, and `cargo test -p meshc --test e2e_m043_s02 e2e_m043_s02_failover_promotion_and_fenced_rejoin_keep_newer_truth -- --nocapture`. The retained evidence lives under `.tmp/m043-s02/verify/`, with `phase-report.txt` / `status.txt` as the summary surface and the copied destructive failover bundle under `07-failover-artifacts/`.

## Requirements Advanced

- R051 — S02 is the first slice that proves primary-cluster loss can be survived through live mirrored standby continuity state, explicit promotion, runtime-owned attempt rollover, and fenced old-primary rejoin.

## Requirements Validated

- R051 — `bash scripts/verify-m043-s02.sh` passed and preserved `.tmp/m043-s02/verify/07-failover-artifacts/`, showing mirrored standby truth before failover, degraded standby after primary loss, explicit promotion to epoch 1, recovery rollover to a new attempt on the promoted standby, successful completion there, and a restarted old primary rejoining as fenced/deposed standby.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

Closeout exposed two prerequisite truth drifts that had to be repaired before the slice verifier could pass: `e2e_m043_s01.rs` and `scripts/verify-m043-s01.sh` still expected pre-submit two-node membership `replication_health=healthy` even though the shipped S02 runtime truth is `local_only` until mirrored continuity exists, and `compiler/meshc/tests/e2e_m042_s03.rs` needed explicit cluster-mode role/epoch env plus both nodes set to the single-cluster `primary` role so the old owner-loss rejoin regression kept exercising the M042 contract instead of the newer primary/standby standby-degrade path. The S02 verifier itself also needed literal matching for bracketed IPv6 node names in retained logs.

## Known Limitations

This slice proves disaster continuity locally through the destructive compiler/verifier rail, not yet through the same-image packaged two-cluster operator flow or public proof/docs surfaces. Live Fly evidence for the broader one-image operator contract remains outside this slice. Promotion is still explicit operator action; there is no quorum-backed automatic promotion or active-active intake claim.

## Follow-ups

S03 should package `scripts/verify-m043-s02.sh` into the same-image two-cluster operator rail with a small env surface and retained artifacts. S04 should update README/docs/help/Fly proof wording to the shipped explicit-promotion and stale-primary-fencing contract.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/continuity.rs` — Moved mutable continuity authority into the runtime registry, enforced higher-epoch fencing on merge/rejoin, and kept promoted pending records on the runtime-owned recovery path.
- `compiler/mesh-rt/src/dist/node.rs` — Supported the runtime-owned continuity authority changes at the distributed node seam.
- `compiler/mesh-rt/src/lib.rs` — Exposed runtime-backed continuity promotion and live authority-status intrinsics to Mesh code.
- `compiler/mesh-typeck/src/infer.rs` — Taught type checking and MIR lowering about `Continuity.promote()` and `Continuity.authority_status()`.
- `compiler/mesh-codegen/src/mir/lower.rs` — Lowered the new continuity intrinsics through codegen so Mesh code can consume the runtime-owned failover seam.
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — Emitted the new continuity promotion intrinsic calls in LLVM codegen.
- `cluster-proof/main.mpl` — Switched cluster-proof to runtime-backed authority reads and exposed the explicit `/promote` operator route without adding Mesh-side DR orchestration.
- `cluster-proof/config.mpl` — Kept cluster-mode topology fail-closed with explicit role/epoch parsing and validation.
- `cluster-proof/work.mpl` — Kept keyed work as a thin consumer of runtime-owned authority truth after promotion and fencing.
- `cluster-proof/work_continuity.mpl` — Surfaced promotion success/rejection and post-failover keyed-status truth through runtime-backed payloads and logs.
- `compiler/meshc/tests/e2e_m043_s02.rs` — Extended the compiler e2e harness into a destructive failover proof with retained primary-loss, promotion, rollover, and fenced-rejoin artifacts.
- `scripts/verify-m043-s02.sh` — Added the fail-closed S02 verifier that replays prerequisites, runs the destructive failover harness, and validates the retained artifact bundle.
- `scripts/verify-m043-s01.sh` — Aligned the S01 mirrored-membership verifier with the shipped pre-submit `local_only` authority truth.
- `compiler/meshc/tests/e2e_m043_s01.rs` — Aligned the S01 compiler e2e with the same pre-submit authority truth so the prerequisite replay stays honest.
- `compiler/meshc/tests/e2e_m042_s03.rs` — Kept the M042 same-identity rejoin regression usable after M043's explicit cluster-mode role contract by passing explicit role/epoch env and preserving the single-cluster primary/primary shape.
