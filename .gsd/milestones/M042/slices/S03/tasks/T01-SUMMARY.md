---
id: T01
parent: S03
milestone: M042
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-rt/src/dist/continuity.rs", ".gsd/milestones/M042/slices/S03/tasks/T01-SUMMARY.md"]
key_decisions: ["Kept owner-loss recovery eligibility as an internal callback seam in `continuity.rs` so T01 could prove attempt rollover and fencing semantics without widening the public submit API or prematurely coupling the registry to node liveness.", "Treat `attempt_id` as the continuity fencing token before terminal/non-terminal phase precedence so stale terminal records cannot overwrite newer active retries during remote upsert or snapshot merge."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task-level runtime verification passed with `cargo test -p mesh-rt continuity -- --nocapture`. Existing thin consumer coverage also passed with `cargo run -q -p meshc -- test cluster-proof/tests`. Slice-level later-task rails still fail closed as expected because `compiler/meshc/tests/e2e_m042_s03.rs` and `scripts/verify-m042-s03.sh` do not exist yet: `cargo test -p meshc --test e2e_m042_s03 -- --nocapture` exited 101 and `bash scripts/verify-m042-s03.sh` exited 127."
completed_at: 2026-03-29T00:06:09.914Z
blocker_discovered: false
---

# T01: Added recovery-aware attempt rollover, attempt-token-first merge fencing, and stale-completion rejection coverage in the runtime continuity registry.

> Added recovery-aware attempt rollover, attempt-token-first merge fencing, and stale-completion rejection coverage in the runtime continuity registry.

## What Happened
---
id: T01
parent: S03
milestone: M042
key_files:
  - compiler/mesh-rt/src/dist/continuity.rs
  - .gsd/milestones/M042/slices/S03/tasks/T01-SUMMARY.md
key_decisions:
  - Kept owner-loss recovery eligibility as an internal callback seam in `continuity.rs` so T01 could prove attempt rollover and fencing semantics without widening the public submit API or prematurely coupling the registry to node liveness.
  - Treat `attempt_id` as the continuity fencing token before terminal/non-terminal phase precedence so stale terminal records cannot overwrite newer active retries during remote upsert or snapshot merge.
duration: ""
verification_result: mixed
completed_at: 2026-03-29T00:06:09.917Z
blocker_discovered: false
---

# T01: Added recovery-aware attempt rollover, attempt-token-first merge fencing, and stale-completion rejection coverage in the runtime continuity registry.

**Added recovery-aware attempt rollover, attempt-token-first merge fencing, and stale-completion rejection coverage in the runtime continuity registry.**

## What Happened

Kept all implementation inside `compiler/mesh-rt/src/dist/continuity.rs` and refactored submit handling into an internal hookable path so recovery eligibility can be proven before node-lifecycle wiring lands. Added recovery rollover to issue a fresh attempt ID for eligible same-key same-payload retries, moved merge precedence to compare parsed attempt tokens before terminal/non-terminal phase, hardened remote merge/snapshot handling against invalid attempt IDs while preserving the watermark, and logged stale completion rejection via a structured `[mesh-rt continuity] transition=completion_rejected` signal. Expanded unit tests to cover recovery rollover, duplicate fallback when recovery is not eligible, repeated rollover monotonicity, stale completion rejection, stale remote terminal merge rejection, stale snapshot terminal merge rejection, and malformed attempt IDs.

## Verification

Task-level runtime verification passed with `cargo test -p mesh-rt continuity -- --nocapture`. Existing thin consumer coverage also passed with `cargo run -q -p meshc -- test cluster-proof/tests`. Slice-level later-task rails still fail closed as expected because `compiler/meshc/tests/e2e_m042_s03.rs` and `scripts/verify-m042-s03.sh` do not exist yet: `cargo test -p meshc --test e2e_m042_s03 -- --nocapture` exited 101 and `bash scripts/verify-m042-s03.sh` exited 127.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-rt continuity -- --nocapture` | 0 | ✅ pass | 264ms |
| 2 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 13540ms |
| 3 | `cargo test -p meshc --test e2e_m042_s03 -- --nocapture` | 101 | ❌ fail | 329ms |
| 4 | `bash scripts/verify-m042-s03.sh` | 127 | ❌ fail | 21ms |


## Deviations

None.

## Known Issues

`compiler/meshc/tests/e2e_m042_s03.rs` and `scripts/verify-m042-s03.sh` are still absent, so the slice-level e2e and verifier commands fail closed until T03 lands. This task intentionally stopped at the runtime registry boundary.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/continuity.rs`
- `.gsd/milestones/M042/slices/S03/tasks/T01-SUMMARY.md`


## Deviations
None.

## Known Issues
`compiler/meshc/tests/e2e_m042_s03.rs` and `scripts/verify-m042-s03.sh` are still absent, so the slice-level e2e and verifier commands fail closed until T03 lands. This task intentionally stopped at the runtime registry boundary.
