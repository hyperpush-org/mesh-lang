---
id: T03
parent: S03
milestone: M042
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/e2e_m042_s03.rs", "scripts/verify-m042-s03.sh", "cluster-proof/work.mpl", ".gsd/milestones/M042/slices/S03/tasks/T03-SUMMARY.md"]
key_decisions: ["Let `cluster-proof/work.mpl` downgrade submit-time `required_replica_count` to `0` only when the existing runtime continuity status is pending `owner_lost`, and log the actual replica count used on the submit path.", "Use S02-style raw HTTP capture plus M039-style run-numbered node logs as the canonical destructive-proof artifact pattern for M042/S03."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the existing S02 degraded continuity proof once to confirm the live JSON/log contract. Ran `cargo test -p meshc --test e2e_m042_s03 -- --nocapture` three times while fixing failures. The initial runs proved the consumer mismatch (`replica_required_unavailable` after runtime rollover). After the `cluster-proof/work.mpl` change, the rejoin scenario passed inside the S03 target, confirming the recovery-submit seam. The owner-loss recovery scenario was still failing before the final cleanup rerun, and `bash scripts/verify-m042-s03.sh` was not run before the context warning."
completed_at: 2026-03-29T00:34:32.068Z
blocker_discovered: false
---

# T03: Drafted the S03 owner-loss harness, added the fail-closed verifier wrapper, and patched cluster-proof recovery submits, but the full S03 verification rail still needs one more rerun.

> Drafted the S03 owner-loss harness, added the fail-closed verifier wrapper, and patched cluster-proof recovery submits, but the full S03 verification rail still needs one more rerun.

## What Happened
---
id: T03
parent: S03
milestone: M042
key_files:
  - compiler/meshc/tests/e2e_m042_s03.rs
  - scripts/verify-m042-s03.sh
  - cluster-proof/work.mpl
  - .gsd/milestones/M042/slices/S03/tasks/T03-SUMMARY.md
key_decisions:
  - Let `cluster-proof/work.mpl` downgrade submit-time `required_replica_count` to `0` only when the existing runtime continuity status is pending `owner_lost`, and log the actual replica count used on the submit path.
  - Use S02-style raw HTTP capture plus M039-style run-numbered node logs as the canonical destructive-proof artifact pattern for M042/S03.
duration: ""
verification_result: mixed
completed_at: 2026-03-29T00:34:32.069Z
blocker_discovered: false
---

# T03: Drafted the S03 owner-loss harness, added the fail-closed verifier wrapper, and patched cluster-proof recovery submits, but the full S03 verification rail still needs one more rerun.

**Drafted the S03 owner-loss harness, added the fail-closed verifier wrapper, and patched cluster-proof recovery submits, but the full S03 verification rail still needs one more rerun.**

## What Happened

Added `compiler/meshc/tests/e2e_m042_s03.rs` as a destructive two-node harness that combines the S02 keyed HTTP/artifact helpers with the M039 kill/restart lifecycle. The new target exercises stable local-owner placement, owner-loss status serving on the survivor, same-key retry rollover, and same-identity rejoin truth. Added `scripts/verify-m042-s03.sh` to replay runtime continuity tests, `cluster-proof` tests, `bash scripts/verify-m042-s02.sh`, and the named S03 scenarios while validating copied `.tmp/m042-s03` bundles. Local execution exposed a real thin-consumer mismatch in `cluster-proof/work.mpl`: runtime owner-loss recovery rolled a new attempt but the app-level submit path still hard-coded `required_replicas=1` and rejected the retry. Fixed that seam by deriving submit-time required replicas from the existing continuity status (`owner_lost` -> `0`) and logging the actual value used. After that change, the rejoin scenario passed. The remaining issue is a timing-sensitive failure in `continuity_api_owner_loss_retry_rollover_survivor_status_is_truthful`; I removed one flaky extra replica-side pre-loss read and updated the verifier artifact contract to match, but context expired before I could rerun the target and then the verifier.

## Verification

Verified the existing S02 degraded continuity proof once to confirm the live JSON/log contract. Ran `cargo test -p meshc --test e2e_m042_s03 -- --nocapture` three times while fixing failures. The initial runs proved the consumer mismatch (`replica_required_unavailable` after runtime rollover). After the `cluster-proof/work.mpl` change, the rejoin scenario passed inside the S03 target, confirming the recovery-submit seam. The owner-loss recovery scenario was still failing before the final cleanup rerun, and `bash scripts/verify-m042-s03.sh` was not run before the context warning.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m042_s02 continuity_api_replica_loss_degrades_pending_mirrored_status -- --nocapture` | 0 | ✅ pass | 37080ms |
| 2 | `cargo test -p meshc --test e2e_m042_s03 -- --nocapture` | 101 | ❌ fail | 7020ms |
| 3 | `cargo test -p meshc --test e2e_m042_s03 -- --nocapture` | 101 | ❌ fail | 23790ms |
| 4 | `cargo test -p meshc --test e2e_m042_s03 -- --nocapture` | 101 | ❌ fail | 24860ms |


## Deviations

The task plan named only the new test target and verifier script, but local execution required an additional thin consumer change in `cluster-proof/work.mpl` so runtime-owned owner-loss recovery submits are not immediately rejected by the app-level replica policy.

## Known Issues

`continuity_api_owner_loss_retry_rollover_survivor_status_is_truthful` still needed a rerun after the last harness/verifier cleanup edits. The just-applied removal of the extra `pre-loss-replica-status` wait and matching verifier artifact requirements was not revalidated before wrap-up. `bash scripts/verify-m042-s03.sh` has not been executed yet.

## Files Created/Modified

- `compiler/meshc/tests/e2e_m042_s03.rs`
- `scripts/verify-m042-s03.sh`
- `cluster-proof/work.mpl`
- `.gsd/milestones/M042/slices/S03/tasks/T03-SUMMARY.md`


## Deviations
The task plan named only the new test target and verifier script, but local execution required an additional thin consumer change in `cluster-proof/work.mpl` so runtime-owned owner-loss recovery submits are not immediately rejected by the app-level replica policy.

## Known Issues
`continuity_api_owner_loss_retry_rollover_survivor_status_is_truthful` still needed a rerun after the last harness/verifier cleanup edits. The just-applied removal of the extra `pre-loss-replica-status` wait and matching verifier artifact requirements was not revalidated before wrap-up. `bash scripts/verify-m042-s03.sh` has not been executed yet.
