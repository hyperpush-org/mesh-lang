---
id: T01
parent: S02
milestone: M043
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-rt/src/dist/continuity.rs", "compiler/mesh-rt/src/dist/node.rs", ".gsd/milestones/M043/slices/S02/tasks/T01-SUMMARY.md"]
key_decisions: ["D176: store mutable continuity authority inside ContinuityRegistry state instead of a separate process-static OnceLock so promotion can preserve mirrored in-memory truth.", "Compare raw incoming role/epoch before local projection and demote to standby on higher-epoch remote truth rather than projecting stale primary records into local authority first.", "Reuse the existing owner-loss retry-rollover seam by converting promoted mirrored pending records into OwnerLost state instead of adding a second failover loop."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task gate passed with cargo test -p mesh-rt continuity -- --nocapture (33 continuity tests green after the new promotion/fencing coverage). Adjacent slice regression checks showed cargo run -q -p meshc -- test cluster-proof/tests && cargo run -q -p meshc -- build cluster-proof passing and bash scripts/verify-m043-s01.sh passing. Slice-level downstream gaps remain outside T01: cargo test -p meshc --test e2e_m042_s03 continuity_api_same_identity_rejoin_preserves_newer_attempt_truth -- --nocapture currently fails before continuity assertions because cluster-proof is launched in cluster mode without MESH_CONTINUITY_ROLE, cargo test -p meshc --test e2e_m043_s02 -- --nocapture fails because the target does not exist yet, and bash scripts/verify-m043-s02.sh fails because the verifier script is not present yet."
completed_at: 2026-03-29T08:11:19.741Z
blocker_discovered: false
---

# T01: Moved continuity authority into the runtime registry and fenced stale lower-epoch primaries before projection.

> Moved continuity authority into the runtime registry and fenced stale lower-epoch primaries before projection.

## What Happened
---
id: T01
parent: S02
milestone: M043
key_files:
  - compiler/mesh-rt/src/dist/continuity.rs
  - compiler/mesh-rt/src/dist/node.rs
  - .gsd/milestones/M043/slices/S02/tasks/T01-SUMMARY.md
key_decisions:
  - D176: store mutable continuity authority inside ContinuityRegistry state instead of a separate process-static OnceLock so promotion can preserve mirrored in-memory truth.
  - Compare raw incoming role/epoch before local projection and demote to standby on higher-epoch remote truth rather than projecting stale primary records into local authority first.
  - Reuse the existing owner-loss retry-rollover seam by converting promoted mirrored pending records into OwnerLost state instead of adding a second failover loop.
duration: ""
verification_result: mixed
completed_at: 2026-03-29T08:11:19.742Z
blocker_discovered: false
---

# T01: Moved continuity authority into the runtime registry and fenced stale lower-epoch primaries before projection.

**Moved continuity authority into the runtime registry and fenced stale lower-epoch primaries before projection.**

## What Happened

Updated compiler/mesh-rt/src/dist/continuity.rs so authority is mutable runtime-owned state inside ContinuityRegistry rather than a separate process-static OnceLock. Added promotion that mutates role/epoch without losing mirrored records, reprojects records on authority change, converts mirrored pending standby records into owner-loss state so the existing retry-rollover seam can recover them, and fences lower-epoch incoming upserts before projection. Updated compiler/mesh-rt/src/dist/node.rs so owner-loss recovery eligibility is epoch-aware. Added runtime unit coverage for promotion without mirrored state, promotion plus retry-rollover reuse, fenced same-identity rejoin on higher-epoch merge, lower-epoch completion rejection before projection, and repeated promotion refusal.

## Verification

Task gate passed with cargo test -p mesh-rt continuity -- --nocapture (33 continuity tests green after the new promotion/fencing coverage). Adjacent slice regression checks showed cargo run -q -p meshc -- test cluster-proof/tests && cargo run -q -p meshc -- build cluster-proof passing and bash scripts/verify-m043-s01.sh passing. Slice-level downstream gaps remain outside T01: cargo test -p meshc --test e2e_m042_s03 continuity_api_same_identity_rejoin_preserves_newer_attempt_truth -- --nocapture currently fails before continuity assertions because cluster-proof is launched in cluster mode without MESH_CONTINUITY_ROLE, cargo test -p meshc --test e2e_m043_s02 -- --nocapture fails because the target does not exist yet, and bash scripts/verify-m043-s02.sh fails because the verifier script is not present yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-rt continuity -- --nocapture` | 0 | ✅ pass | 23754ms |
| 2 | `cargo run -q -p meshc -- test cluster-proof/tests && cargo run -q -p meshc -- build cluster-proof` | 0 | ✅ pass | 23748ms |
| 3 | `cargo test -p meshc --test e2e_m042_s03 continuity_api_same_identity_rejoin_preserves_newer_attempt_truth -- --nocapture` | 101 | ❌ fail | 33010ms |
| 4 | `cargo test -p meshc --test e2e_m043_s02 -- --nocapture` | 101 | ❌ fail | 327ms |
| 5 | `bash scripts/verify-m043-s01.sh` | 0 | ✅ pass | 82279ms |
| 6 | `bash scripts/verify-m043-s02.sh` | 127 | ❌ fail | 52ms |


## Deviations

No implementation deviation from the task plan. Slice-level verification exposed downstream harness gaps outside T01: the targeted M042 replay is not yet aligned with S01's explicit cluster-role env contract, and the S02 e2e target plus verifier script do not exist yet.

## Known Issues

The targeted M042 regression replay currently exits early with cluster-proof config errors because the harness omits MESH_CONTINUITY_ROLE in cluster mode. The S02 e2e target and scripts/verify-m043-s02.sh do not exist yet, so those slice-level checks remain red until later tasks land.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/dist/node.rs`
- `.gsd/milestones/M043/slices/S02/tasks/T01-SUMMARY.md`


## Deviations
No implementation deviation from the task plan. Slice-level verification exposed downstream harness gaps outside T01: the targeted M042 replay is not yet aligned with S01's explicit cluster-role env contract, and the S02 e2e target plus verifier script do not exist yet.

## Known Issues
The targeted M042 regression replay currently exits early with cluster-proof config errors because the harness omits MESH_CONTINUITY_ROLE in cluster mode. The S02 e2e target and scripts/verify-m043-s02.sh do not exist yet, so those slice-level checks remain red until later tasks land.
