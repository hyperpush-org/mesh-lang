---
id: T02
parent: S06
milestone: M053
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/e2e_m053_s02.rs", "scripts/verify-m053-s02.sh", ".gsd/milestones/M053/slices/S06/tasks/T02-SUMMARY.md"]
key_decisions: ["Treat the staged Postgres failover rail as invalid unless pre-kill diagnostics show exactly one startup_dispatch_window entry with pending_window_ms=20000 and no startup_completed for the startup request."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the retained hosted failure fixture, reran the authoritative staged Postgres failover e2e against a disposable local Postgres URL, and reran scripts/verify-m053-s02.sh to green so .tmp/m053-s02/verify/status.txt is ok and the retained proof bundle pointer refreshed."
completed_at: 2026-04-06T02:19:00.439Z
blocker_discovered: false
---

# T02: Made the staged Postgres failover rail fail closed on pre-kill startup-window evidence.

> Made the staged Postgres failover rail fail closed on pre-kill startup-window evidence.

## What Happened
---
id: T02
parent: S06
milestone: M053
key_files:
  - compiler/meshc/tests/e2e_m053_s02.rs
  - scripts/verify-m053-s02.sh
  - .gsd/milestones/M053/slices/S06/tasks/T02-SUMMARY.md
key_decisions:
  - Treat the staged Postgres failover rail as invalid unless pre-kill diagnostics show exactly one startup_dispatch_window entry with pending_window_ms=20000 and no startup_completed for the startup request.
duration: ""
verification_result: passed
completed_at: 2026-04-06T02:19:00.442Z
blocker_discovered: false
---

# T02: Made the staged Postgres failover rail fail closed on pre-kill startup-window evidence.

**Made the staged Postgres failover rail fail closed on pre-kill startup-window evidence.**

## What Happened

Tightened the staged Postgres failover rail around the runtime-owned startup window instead of trusting a green recovery path alone. The Rust e2e now reads the retained hosted red bundle as a negative regression fixture, then proves the local failover rail only proceeds when pre-kill diagnostics expose the startup request's runtime-owned startup_dispatch_window with pending_window_ms=20000 and no startup_completed before the forced owner stop. The assembled scripts/verify-m053-s02.sh rail now requires those retained pre-kill diagnostics snapshots and the matching primary-run1 startup log evidence in the copied proof bundle.

## Verification

Verified the retained hosted failure fixture, reran the authoritative staged Postgres failover e2e against a disposable local Postgres URL, and reran scripts/verify-m053-s02.sh to green so .tmp/m053-s02/verify/status.txt is ok and the retained proof bundle pointer refreshed.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m053_s02 m053_s02_hosted_failure_bundle_proves_completed_before_owner_stop -- --nocapture` | 0 | ✅ pass | 1980ms |
| 2 | `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_m053_s02 m053_s02_staged_postgres_failover_proves_clustered_http_and_runtime_recovery -- --nocapture` | 0 | ✅ pass | 12800ms |
| 3 | `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash scripts/verify-m053-s02.sh` | 0 | ✅ pass | 124690ms |


## Deviations

Added a retained hosted-failure regression test and corresponding verifier bundle-shape assertions beyond the original live-rail edits. This tightened the proof surface without widening starter/runtime scope.

## Known Issues

None.

## Files Created/Modified

- `compiler/meshc/tests/e2e_m053_s02.rs`
- `scripts/verify-m053-s02.sh`
- `.gsd/milestones/M053/slices/S06/tasks/T02-SUMMARY.md`


## Deviations
Added a retained hosted-failure regression test and corresponding verifier bundle-shape assertions beyond the original live-rail edits. This tightened the proof surface without widening starter/runtime scope.

## Known Issues
None.
