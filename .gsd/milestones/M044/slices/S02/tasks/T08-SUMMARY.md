---
id: T08
parent: S02
milestone: M044
provides: []
requires: []
affects: []
key_files: ["scripts/verify-m044-s02.sh", ".gsd/milestones/M044/slices/S02/tasks/T08-SUMMARY.md"]
key_decisions: ["Added `scripts/verify-m044-s02.sh` as a truth surface and stopped at the verifier boundary instead of spending time fixing app-owned clustering code that later slices are expected to remove.", "Scoped the stale-path checks to the submit/status hot-path ranges in `cluster-proof/work_continuity.mpl` so legacy probe helpers can remain until S05 without making the new path look green."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the live metadata filter plus the existing `cluster-proof` dogfood commands to confirm the current tree still behaves as expected on the surviving surfaces. Then ran the new assembled verifier. It replayed S01, refreshed `mesh-rt`, passed the metadata phase, and failed closed at the first missing S02 rail (`m044_s02_declared_work_` running 0 tests). The authoritative stop-point artifacts are `.tmp/m044-s02/verify/phase-report.txt`, `.tmp/m044-s02/verify/status.txt`, `.tmp/m044-s02/verify/current-phase.txt`, and `.tmp/m044-s02/verify/03-s02-declared-work.test-count.log`."
completed_at: 2026-03-29T21:00:26.453Z
blocker_discovered: true
---

# T08: Added scripts/verify-m044-s02.sh as a fail-closed S02 rail and recorded that the declared-runtime substrate it expects is still missing.

> Added scripts/verify-m044-s02.sh as a fail-closed S02 rail and recorded that the declared-runtime substrate it expects is still missing.

## What Happened
---
id: T08
parent: S02
milestone: M044
key_files:
  - scripts/verify-m044-s02.sh
  - .gsd/milestones/M044/slices/S02/tasks/T08-SUMMARY.md
key_decisions:
  - Added `scripts/verify-m044-s02.sh` as a truth surface and stopped at the verifier boundary instead of spending time fixing app-owned clustering code that later slices are expected to remove.
  - Scoped the stale-path checks to the submit/status hot-path ranges in `cluster-proof/work_continuity.mpl` so legacy probe helpers can remain until S05 without making the new path look green.
duration: ""
verification_result: mixed
completed_at: 2026-03-29T21:00:26.454Z
blocker_discovered: true
---

# T08: Added scripts/verify-m044-s02.sh as a fail-closed S02 rail and recorded that the declared-runtime substrate it expects is still missing.

**Added scripts/verify-m044-s02.sh as a fail-closed S02 rail and recorded that the declared-runtime substrate it expects is still missing.**

## What Happened

Loaded the T08 contract, the existing S01 verifier, and the live `e2e_m044_s02.rs` / `cluster-proof/work_continuity.mpl` state before changing anything. The tree still matched the earlier blocker summaries: there was no `scripts/verify-m044-s02.sh`, `compiler/meshc/tests/e2e_m044_s02.rs` only contained the `m044_s02_metadata_` tests, and `cluster-proof/work_continuity.mpl` still owned the submit hot path through `current_target_selection(...)`, `submit_from_selection(...)`, `created_submit_response(...) -> dispatch_work(...)`, and `dispatch_remote_work(...) -> Node.spawn(...)`. I added `scripts/verify-m044-s02.sh` as the assembled S02 rail. It replays `bash scripts/verify-m044-s01.sh`, refreshes `mesh-rt`, runs the named `m044_s02_metadata_`, `m044_s02_declared_work_`, `m044_s02_service_`, and `m044_s02_cluster_proof_` filters with fail-closed `running N test` checks, snapshots and copies any new `.tmp/m044-s02/*` e2e artifact directories, rebuilds and retests `cluster-proof`, and scopes the stale-path checks to the submit/status ranges inside `cluster-proof/work_continuity.mpl` instead of banning legacy helpers globally. Then I ran the live checks. The metadata prefix is healthy, and `cluster-proof` still builds and its package tests still pass. The new verifier also behaved correctly: it stopped on `m044_s02_declared_work_` because Cargo exited 0 while running 0 tests, and it recorded that stop in `.tmp/m044-s02/verify/current-phase.txt` and `.tmp/m044-s02/verify/03-s02-declared-work.test-count.log`. Per user steer, I did not spend time trying to rehabilitate the legacy Mesh clustering code in `work_continuity.mpl`; the useful artifact here is the red gate that proves the declared-runtime substrate is not landed yet.

## Verification

Ran the live metadata filter plus the existing `cluster-proof` dogfood commands to confirm the current tree still behaves as expected on the surviving surfaces. Then ran the new assembled verifier. It replayed S01, refreshed `mesh-rt`, passed the metadata phase, and failed closed at the first missing S02 rail (`m044_s02_declared_work_` running 0 tests). The authoritative stop-point artifacts are `.tmp/m044-s02/verify/phase-report.txt`, `.tmp/m044-s02/verify/status.txt`, `.tmp/m044-s02/verify/current-phase.txt`, and `.tmp/m044-s02/verify/03-s02-declared-work.test-count.log`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture` | 0 | ✅ pass | 8038ms |
| 2 | `cargo run -q -p meshc -- build cluster-proof` | 0 | ✅ pass | 11318ms |
| 3 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 15275ms |
| 4 | `bash scripts/verify-m044-s02.sh` | 1 | ❌ fail | 321579ms |


## Deviations

Did not try to align nonexistent `m044_s02_declared_work_`, `m044_s02_service_`, or `m044_s02_cluster_proof_` tests by patching old runtime/app code just to make the verifier greener. Given the current tree and user guidance, the honest deviation was to ship the fail-closed rail and stop at the missing declared-runtime substrate.

## Known Issues

`compiler/meshc/tests/e2e_m044_s02.rs` still only contains the metadata tests, so the verifier stops at `m044_s02_declared_work_` before it can reach the service, cluster-proof, or hot-path absence phases. Separately, `cluster-proof/work_continuity.mpl` still contains the old app-owned submit/dispatch flow (`current_target_selection(...)`, `submit_from_selection(...)`, `dispatch_work(...)`, and `dispatch_remote_work(...) -> Node.spawn(...)`), so even after the missing named tests land, the later absence checks will remain red until T05-T07 actually remove that path.

## Files Created/Modified

- `scripts/verify-m044-s02.sh`
- `.gsd/milestones/M044/slices/S02/tasks/T08-SUMMARY.md`


## Deviations
Did not try to align nonexistent `m044_s02_declared_work_`, `m044_s02_service_`, or `m044_s02_cluster_proof_` tests by patching old runtime/app code just to make the verifier greener. Given the current tree and user guidance, the honest deviation was to ship the fail-closed rail and stop at the missing declared-runtime substrate.

## Known Issues
`compiler/meshc/tests/e2e_m044_s02.rs` still only contains the metadata tests, so the verifier stops at `m044_s02_declared_work_` before it can reach the service, cluster-proof, or hot-path absence phases. Separately, `cluster-proof/work_continuity.mpl` still contains the old app-owned submit/dispatch flow (`current_target_selection(...)`, `submit_from_selection(...)`, `dispatch_work(...)`, and `dispatch_remote_work(...) -> Node.spawn(...)`), so even after the missing named tests land, the later absence checks will remain red until T05-T07 actually remove that path.
