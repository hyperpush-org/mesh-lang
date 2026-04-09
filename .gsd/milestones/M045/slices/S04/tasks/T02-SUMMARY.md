---
id: T02
parent: S04
milestone: M045
provides: []
requires: []
affects: []
key_files: ["cluster-proof/mesh.toml", "cluster-proof/work.mpl", "cluster-proof/work_continuity.mpl", "cluster-proof/tests/work.test.mpl", "compiler/meshc/tests/e2e_m044_s04.rs", "compiler/meshc/tests/e2e_m044_s01.rs", ".gsd/milestones/M045/slices/S04/tasks/T02-SUMMARY.md"]
key_decisions: ["`cluster-proof/work.mpl` now owns both `declared_work_target()` and `execute_declared_work(...)`; `work_continuity.mpl` only translates submit/status HTTP behavior.", "The destructive `m044_s04_auto_promotion_` rail now includes a lightweight source-contract test under the same prefix so one verification command catches both ownership drift and runtime failover drift."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task rail passed with `cargo run -q -p meshc -- build cluster-proof`, `cargo run -q -p meshc -- test cluster-proof/tests`, and `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture`; the package tests now exercise the Work-owned declared handler directly, and the destructive failover rail confirmed runtime-owned `automatic_recovery` plus `completed` truth with `runtime_name=Work.execute_declared_work` and no wrapper completion-failure log lines. Current slice rails from T01 still pass (`tooling_e2e`, `e2e_m045_s02`, `e2e_m045_s03`), while the future T03 rail still fails closed because `e2e_m045_s04` and `scripts/verify-m045-s04.sh` are not in the tree yet."
completed_at: 2026-03-30T23:54:35.270Z
blocker_discovered: false
---

# T02: Moved cluster-proof declared work into `Work` and dropped wrapper-side completion glue.

> Moved cluster-proof declared work into `Work` and dropped wrapper-side completion glue.

## What Happened
---
id: T02
parent: S04
milestone: M045
key_files:
  - cluster-proof/mesh.toml
  - cluster-proof/work.mpl
  - cluster-proof/work_continuity.mpl
  - cluster-proof/tests/work.test.mpl
  - compiler/meshc/tests/e2e_m044_s04.rs
  - compiler/meshc/tests/e2e_m044_s01.rs
  - .gsd/milestones/M045/slices/S04/tasks/T02-SUMMARY.md
key_decisions:
  - `cluster-proof/work.mpl` now owns both `declared_work_target()` and `execute_declared_work(...)`; `work_continuity.mpl` only translates submit/status HTTP behavior.
  - The destructive `m044_s04_auto_promotion_` rail now includes a lightweight source-contract test under the same prefix so one verification command catches both ownership drift and runtime failover drift.
duration: ""
verification_result: mixed
completed_at: 2026-03-30T23:54:35.272Z
blocker_discovered: false
---

# T02: Moved cluster-proof declared work into `Work` and dropped wrapper-side completion glue.

**Moved cluster-proof declared work into `Work` and dropped wrapper-side completion glue.**

## What Happened

Moved the `cluster-proof` manifest target from `WorkContinuity.execute_declared_work` to `Work.execute_declared_work`, implemented the declared handler in `cluster-proof/work.mpl`, and kept only the retained execution log plus proof delay there so declared work now returns through the runtime-owned completion path. Removed the wrapper-era target helper, dead actor path, manual `Continuity.mark_completed(...)` call, and completion-failure logging from `cluster-proof/work_continuity.mpl`, leaving it as keyed submit/status HTTP translation. Updated package tests to prove declared work now lives in `Work`, tightened `compiler/meshc/tests/e2e_m044_s04.rs` so the existing auto-promotion rail also asserts the source contract and absence of wrapper completion glue, and aligned the older `e2e_m044_s01` manifest assertion with the new target string.

## Verification

Task rail passed with `cargo run -q -p meshc -- build cluster-proof`, `cargo run -q -p meshc -- test cluster-proof/tests`, and `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture`; the package tests now exercise the Work-owned declared handler directly, and the destructive failover rail confirmed runtime-owned `automatic_recovery` plus `completed` truth with `runtime_name=Work.execute_declared_work` and no wrapper completion-failure log lines. Current slice rails from T01 still pass (`tooling_e2e`, `e2e_m045_s02`, `e2e_m045_s03`), while the future T03 rail still fails closed because `e2e_m045_s04` and `scripts/verify-m045-s04.sh` are not in the tree yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- build cluster-proof` | 0 | ✅ pass | 11300ms |
| 2 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 15560ms |
| 3 | `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture` | 0 | ✅ pass | 29420ms |
| 4 | `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture` | 0 | ✅ pass | 8110ms |
| 5 | `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture` | 0 | ✅ pass | 14050ms |
| 6 | `cargo test -p meshc --test e2e_m045_s03 m045_s03_failover_ -- --nocapture` | 0 | ✅ pass | 13350ms |
| 7 | `cargo test -p meshc --test e2e_m045_s04 m045_s04_ -- --nocapture` | 101 | ❌ fail | 1140ms |
| 8 | `bash scripts/verify-m045-s04.sh` | 127 | ❌ fail | 10ms |
| 9 | `cargo test -p meshc --test e2e_m044_s01 m044_s01_cluster_proof_manifest_declares_clustered_boundary -- --nocapture` | 0 | ✅ pass | 9600ms |


## Deviations

Also updated `compiler/meshc/tests/e2e_m044_s01.rs` so the older manifest-source assertion matches the new declared-work target. That file was outside the task's listed outputs, but leaving it stale would have created an obvious false red path after the move.

## Known Issues

`cargo test -p meshc --test e2e_m045_s04 m045_s04_ -- --nocapture` still fails with `no test target named e2e_m045_s04`, and `bash scripts/verify-m045-s04.sh` still fails because the script does not exist yet. Those are downstream T03 gaps, not regressions introduced by this task.

## Files Created/Modified

- `cluster-proof/mesh.toml`
- `cluster-proof/work.mpl`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/tests/work.test.mpl`
- `compiler/meshc/tests/e2e_m044_s04.rs`
- `compiler/meshc/tests/e2e_m044_s01.rs`
- `.gsd/milestones/M045/slices/S04/tasks/T02-SUMMARY.md`


## Deviations
Also updated `compiler/meshc/tests/e2e_m044_s01.rs` so the older manifest-source assertion matches the new declared-work target. That file was outside the task's listed outputs, but leaving it stale would have created an obvious false red path after the move.

## Known Issues
`cargo test -p meshc --test e2e_m045_s04 m045_s04_ -- --nocapture` still fails with `no test target named e2e_m045_s04`, and `bash scripts/verify-m045-s04.sh` still fails because the script does not exist yet. Those are downstream T03 gaps, not regressions introduced by this task.
