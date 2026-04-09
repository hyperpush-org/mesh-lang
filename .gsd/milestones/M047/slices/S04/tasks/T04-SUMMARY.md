---
id: T04
parent: S04
milestone: M047
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/support/m046_route_free.rs", "compiler/meshc/tests/e2e_m045_s01.rs", "compiler/meshc/tests/e2e_m045_s02.rs", "compiler/meshc/tests/e2e_m045_s03.rs", "compiler/meshc/tests/e2e_m046_s03.rs", "compiler/meshc/tests/e2e_m046_s04.rs", "compiler/meshc/tests/e2e_m046_s05.rs", ".gsd/milestones/M047/slices/S04/tasks/T04-SUMMARY.md"]
key_decisions: ["Centralized the shared `@cluster` contract strings in `compiler/meshc/tests/support/m046_route_free.rs` and reused them from the historical M045/M046 rails instead of duplicating wording in each test file."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the task-plan historical e2e matrix. `cargo test -p meshc --test e2e_m045_s01 m045_s01_ -- --nocapture`, `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture`, `cargo test -p meshc --test e2e_m045_s03 m045_s03_ -- --nocapture`, `cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture`, and `cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture` all passed with the rewritten `@cluster` assertions. `cargo test -p meshc --test e2e_m046_s03 m046_s03_ -- --nocapture` failed twice in the pre-existing two-node failover runtime path (`replica_required_unavailable` after standby promotion). `cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture` failed because the docs/runbooks still lack the later `verify-m046-s06.sh` mention."
completed_at: 2026-04-01T10:00:12.343Z
blocker_discovered: false
---

# T04: Rewired the historical route-free rails to the shared `@cluster` source-first contract.

> Rewired the historical route-free rails to the shared `@cluster` source-first contract.

## What Happened
---
id: T04
parent: S04
milestone: M047
key_files:
  - compiler/meshc/tests/support/m046_route_free.rs
  - compiler/meshc/tests/e2e_m045_s01.rs
  - compiler/meshc/tests/e2e_m045_s02.rs
  - compiler/meshc/tests/e2e_m045_s03.rs
  - compiler/meshc/tests/e2e_m046_s03.rs
  - compiler/meshc/tests/e2e_m046_s04.rs
  - compiler/meshc/tests/e2e_m046_s05.rs
  - .gsd/milestones/M047/slices/S04/tasks/T04-SUMMARY.md
key_decisions:
  - Centralized the shared `@cluster` contract strings in `compiler/meshc/tests/support/m046_route_free.rs` and reused them from the historical M045/M046 rails instead of duplicating wording in each test file.
duration: ""
verification_result: mixed
completed_at: 2026-04-01T10:00:12.355Z
blocker_discovered: false
---

# T04: Rewired the historical route-free rails to the shared `@cluster` source-first contract.

**Rewired the historical route-free rails to the shared `@cluster` source-first contract.**

## What Happened

Added shared source-first contract literals to `compiler/meshc/tests/support/m046_route_free.rs` so the historical route-free rails can all pin the same `@cluster` declaration, runtime-name guidance, and runtime autostart wording instead of carrying stale copied `clustered(work)` / `declared_work_runtime_name()` expectations. Rewrote the file-content assertions in the M045 scaffold/runtime rails and the M046 tiny-cluster, cluster-proof, and equal-surface rails so generated `work.mpl` and package `work.mpl` now require `@cluster pub fn execute_declared_work(...)`, reject the deleted helper and legacy marker, and keep runtime-name continuity through the README/runbook guidance that still names `Work.execute_declared_work`. Preserved the route-free runtime checks, CLI continuity/status assertions, and retained artifact behavior; the support helper still archives generated trees and runtime bundles the same way, but failures now localize against the new source-first contract instead of the removed compatibility seam.

## Verification

Ran the task-plan historical e2e matrix. `cargo test -p meshc --test e2e_m045_s01 m045_s01_ -- --nocapture`, `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture`, `cargo test -p meshc --test e2e_m045_s03 m045_s03_ -- --nocapture`, `cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture`, and `cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture` all passed with the rewritten `@cluster` assertions. `cargo test -p meshc --test e2e_m046_s03 m046_s03_ -- --nocapture` failed twice in the pre-existing two-node failover runtime path (`replica_required_unavailable` after standby promotion). `cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture` failed because the docs/runbooks still lack the later `verify-m046-s06.sh` mention.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m045_s01 m045_s01_ -- --nocapture` | 0 | ✅ pass | 9020ms |
| 2 | `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture` | 0 | ✅ pass | 7950ms |
| 3 | `cargo test -p meshc --test e2e_m045_s03 m045_s03_ -- --nocapture` | 0 | ✅ pass | 4130ms |
| 4 | `cargo test -p meshc --test e2e_m046_s03 m046_s03_ -- --nocapture` | 101 | ❌ fail | 28200ms |
| 5 | `cargo test -p meshc --test e2e_m046_s03 m046_s03_ -- --nocapture` | 101 | ❌ fail | 27330ms |
| 6 | `cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture` | 0 | ✅ pass | 14810ms |
| 7 | `cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture` | 0 | ✅ pass | 11520ms |
| 8 | `cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture` | 101 | ❌ fail | 1360ms |


## Deviations

Inspected `compiler/meshc/tests/e2e_m046_s06.rs` but left its remaining docs-closeout assertion unchanged because the red path is owned by the later S04 docs task, not by this route-free harness rewrite. No broader runtime helper changes were needed beyond centralizing the shared source-first contract strings in `m046_route_free.rs`.

## Known Issues

`cargo test -p meshc --test e2e_m046_s03 m046_s03_ -- --nocapture` still fails in the pre-existing two-node tiny-cluster failover runtime rail: after standby promotion, the recovery submit is rejected with `replica_required_unavailable`, so post-kill standby status never reaches the expected promoted-primary truth. `cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture` still fails because `tiny-cluster/README.md` does not yet mention `bash scripts/verify-m046-s06.sh`; that public docs/runbook update belongs to the later T05 docs work.

## Files Created/Modified

- `compiler/meshc/tests/support/m046_route_free.rs`
- `compiler/meshc/tests/e2e_m045_s01.rs`
- `compiler/meshc/tests/e2e_m045_s02.rs`
- `compiler/meshc/tests/e2e_m045_s03.rs`
- `compiler/meshc/tests/e2e_m046_s03.rs`
- `compiler/meshc/tests/e2e_m046_s04.rs`
- `compiler/meshc/tests/e2e_m046_s05.rs`
- `.gsd/milestones/M047/slices/S04/tasks/T04-SUMMARY.md`


## Deviations
Inspected `compiler/meshc/tests/e2e_m046_s06.rs` but left its remaining docs-closeout assertion unchanged because the red path is owned by the later S04 docs task, not by this route-free harness rewrite. No broader runtime helper changes were needed beyond centralizing the shared source-first contract strings in `m046_route_free.rs`.

## Known Issues
`cargo test -p meshc --test e2e_m046_s03 m046_s03_ -- --nocapture` still fails in the pre-existing two-node tiny-cluster failover runtime rail: after standby promotion, the recovery submit is rejected with `replica_required_unavailable`, so post-kill standby status never reaches the expected promoted-primary truth. `cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture` still fails because `tiny-cluster/README.md` does not yet mention `bash scripts/verify-m046-s06.sh`; that public docs/runbook update belongs to the later T05 docs work.
