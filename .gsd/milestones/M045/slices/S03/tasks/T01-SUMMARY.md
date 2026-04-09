---
id: T01
parent: S03
milestone: M045
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/e2e_m045_s03.rs", "compiler/meshc/Cargo.toml", ".gsd/milestones/M045/slices/S03/tasks/T01-SUMMARY.md"]
key_decisions: ["Parse scaffold HTTP and `meshc cluster ... --json` surfaces as typed test data so malformed CLI/HTTP responses fail closed with retained artifacts.", "Keep the scaffold app source untouched and push all failover truth through runtime CLI status/continuity/diagnostics plus node logs."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "`cargo test -p meshc --test e2e_m045_s03 -- --list` passed and exposed the expected three tests. `cargo test -p meshc --test e2e_m045_s03 m045_s03_failover_ -- --nocapture` failed after the bounded batch search exhausted the pre-kill candidate space. The retained artifacts under `.tmp/m045-s03/scaffold-failover-runtime-truth-1774907789136358000/` contain the search logs, per-candidate continuity/status output, and node logs showing that owner-primary requests completed too quickly for the current harness."
completed_at: 2026-03-30T22:01:49.279Z
blocker_discovered: false
---

# T01: Added the M045/S03 scaffold failover e2e harness with retained runtime diagnostics, but the generated scaffold still fails the primary-owned pending-window verification.

> Added the M045/S03 scaffold failover e2e harness with retained runtime diagnostics, but the generated scaffold still fails the primary-owned pending-window verification.

## What Happened
---
id: T01
parent: S03
milestone: M045
key_files:
  - compiler/meshc/tests/e2e_m045_s03.rs
  - compiler/meshc/Cargo.toml
  - .gsd/milestones/M045/slices/S03/tasks/T01-SUMMARY.md
key_decisions:
  - Parse scaffold HTTP and `meshc cluster ... --json` surfaces as typed test data so malformed CLI/HTTP responses fail closed with retained artifacts.
  - Keep the scaffold app source untouched and push all failover truth through runtime CLI status/continuity/diagnostics plus node logs.
duration: ""
verification_result: mixed
completed_at: 2026-03-30T22:01:49.281Z
blocker_discovered: false
---

# T01: Added the M045/S03 scaffold failover e2e harness with retained runtime diagnostics, but the generated scaffold still fails the primary-owned pending-window verification.

**Added the M045/S03 scaffold failover e2e harness with retained runtime diagnostics, but the generated scaffold still fails the primary-owned pending-window verification.**

## What Happened

Added a new `compiler/meshc/tests/e2e_m045_s03.rs` target around the generated clustered scaffold instead of `cluster-proof`. The harness initializes and builds a temporary `meshc init --clustered` project, boots two local scaffold nodes, waits for `/health` plus `meshc cluster status --json` convergence, and searches for a runtime-confirmed pre-kill continuity record using only runtime-owned `meshc cluster continuity --json` truth. It retains pre-kill/post-kill/post-rejoin artifacts, parses CLI and HTTP JSON as typed data, and fail-closes on malformed responses. I also added two helper rails in the same target for malformed JSON and the `replica_status=preparing|mirrored` boundary. On the real failover run, the harness never found a request key that stayed in a runtime-confirmed primary-owned pending state long enough on both nodes to cross the destructive step: primary-owned requests were created with `replica_status=preparing` and then immediately completed on the primary before the harness could observe a stable pending window on both nodes, while standby-owned requests completed on the standby. Because selection never succeeded, the kill/promote/recover/rejoin half of the test body did not execute in the failing run.

## Verification

`cargo test -p meshc --test e2e_m045_s03 -- --list` passed and exposed the expected three tests. `cargo test -p meshc --test e2e_m045_s03 m045_s03_failover_ -- --nocapture` failed after the bounded batch search exhausted the pre-kill candidate space. The retained artifacts under `.tmp/m045-s03/scaffold-failover-runtime-truth-1774907789136358000/` contain the search logs, per-candidate continuity/status output, and node logs showing that owner-primary requests completed too quickly for the current harness.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m045_s03 -- --list` | 0 | ✅ pass | 12050ms |
| 2 | `cargo test -p meshc --test e2e_m045_s03 m045_s03_failover_ -- --nocapture` | 101 | ❌ fail | 181460ms |


## Deviations

Added two helper-focused tests inside `e2e_m045_s03.rs` so malformed CLI JSON and the `preparing|mirrored` boundary are covered explicitly in the new target. Added `serde = { workspace = true }` to `compiler/meshc/Cargo.toml` dev-dependencies so the new integration test can use typed `Deserialize`/`Serialize` parsing.

## Known Issues

The required verification command is still red. The bounded batch search exhausted the current scaffold behavior without finding a runtime-confirmed primary-owned pending record that stayed pending long enough to trigger the destructive failover step. Because pre-kill selection failed, the promoted-standby recovery and stale-primary rejoin sections of the new test did not execute in the failing run. Resume from `.tmp/m045-s03/scaffold-failover-runtime-truth-1774907789136358000/` and decide whether the harness needs a smaller observation change or whether T02 must add a runtime-owned timing seam below the scaffold surface.

## Files Created/Modified

- `compiler/meshc/tests/e2e_m045_s03.rs`
- `compiler/meshc/Cargo.toml`
- `.gsd/milestones/M045/slices/S03/tasks/T01-SUMMARY.md`


## Deviations
Added two helper-focused tests inside `e2e_m045_s03.rs` so malformed CLI JSON and the `preparing|mirrored` boundary are covered explicitly in the new target. Added `serde = { workspace = true }` to `compiler/meshc/Cargo.toml` dev-dependencies so the new integration test can use typed `Deserialize`/`Serialize` parsing.

## Known Issues
The required verification command is still red. The bounded batch search exhausted the current scaffold behavior without finding a runtime-confirmed primary-owned pending record that stayed pending long enough to trigger the destructive failover step. Because pre-kill selection failed, the promoted-standby recovery and stale-primary rejoin sections of the new test did not execute in the failing run. Resume from `.tmp/m045-s03/scaffold-failover-runtime-truth-1774907789136358000/` and decide whether the harness needs a smaller observation change or whether T02 must add a runtime-owned timing seam below the scaffold surface.
