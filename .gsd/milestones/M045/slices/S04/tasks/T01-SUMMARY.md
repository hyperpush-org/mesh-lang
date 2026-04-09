---
id: T01
parent: S04
milestone: M045
provides: []
requires: []
affects: []
key_files: ["cluster-proof/cluster.mpl", "cluster-proof/tests/work.test.mpl", "cluster-proof/tests/config.test.mpl", "/Users/sn0w/Documents/dev/mesh-lang/.gsd/milestones/M045/slices/S04/tasks/T01-SUMMARY.md"]
key_decisions: ["Kept deterministic membership sorting in `cluster-proof/cluster.mpl` because `canonical_membership(...)` and `membership_payload(...)` still depend on it, and deleted only the unused owner/replica placement chain.", "Moved replica-count assertions into `cluster-proof/tests/config.test.mpl` and made `cluster-proof/tests/work.test.mpl` prove the absence of legacy placement fields directly in the membership payload JSON."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified source cleanup with an explicit no-match `rg` check against `cluster-proof/cluster.mpl`, then reran `cargo run -q -p meshc -- build cluster-proof` and `cargo run -q -p meshc -- test cluster-proof/tests`. Also ran the slice verification commands available at this point: `tooling_e2e`, `e2e_m045_s02`, and `e2e_m045_s03` all passed. The future T03 closeout checks fail closed because the `e2e_m045_s04` target and `scripts/verify-m045-s04.sh` script are not present yet."
completed_at: 2026-03-30T23:42:25.275Z
blocker_discovered: false
---

# T01: Removed dead cluster-proof placement helpers and refocused package tests on live membership/config seams.

> Removed dead cluster-proof placement helpers and refocused package tests on live membership/config seams.

## What Happened
---
id: T01
parent: S04
milestone: M045
key_files:
  - cluster-proof/cluster.mpl
  - cluster-proof/tests/work.test.mpl
  - cluster-proof/tests/config.test.mpl
  - /Users/sn0w/Documents/dev/mesh-lang/.gsd/milestones/M045/slices/S04/tasks/T01-SUMMARY.md
key_decisions:
  - Kept deterministic membership sorting in `cluster-proof/cluster.mpl` because `canonical_membership(...)` and `membership_payload(...)` still depend on it, and deleted only the unused owner/replica placement chain.
  - Moved replica-count assertions into `cluster-proof/tests/config.test.mpl` and made `cluster-proof/tests/work.test.mpl` prove the absence of legacy placement fields directly in the membership payload JSON.
duration: ""
verification_result: mixed
completed_at: 2026-03-30T23:42:25.278Z
blocker_discovered: false
---

# T01: Removed dead cluster-proof placement helpers and refocused package tests on live membership/config seams.

**Removed dead cluster-proof placement helpers and refocused package tests on live membership/config seams.**

## What Happened

Removed the dead canonical placement engine from `cluster-proof/cluster.mpl` by deleting `CanonicalPlacement`, `canonical_placement(...)`, and the request-key owner/replica scoring helpers while preserving the live canonical membership and membership payload behavior. Retargeted `cluster-proof/tests/work.test.mpl` to assert runtime-owned membership authority/discovery fields and the explicit absence of legacy placement fields, and moved replica-count assertions into `cluster-proof/tests/config.test.mpl` with explicit accepted durability-policy coverage. Reran the task-level package build/tests plus the current slice verification rail; package checks and the existing M045 scaffold/failover rails stayed green, while the later S04 closeout target/script remain absent in the tree.

## Verification

Verified source cleanup with an explicit no-match `rg` check against `cluster-proof/cluster.mpl`, then reran `cargo run -q -p meshc -- build cluster-proof` and `cargo run -q -p meshc -- test cluster-proof/tests`. Also ran the slice verification commands available at this point: `tooling_e2e`, `e2e_m045_s02`, and `e2e_m045_s03` all passed. The future T03 closeout checks fail closed because the `e2e_m045_s04` target and `scripts/verify-m045-s04.sh` script are not present yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `sh -c 'if rg -n "CanonicalPlacement|canonical_placement\(|build_canonical_placement_from_membership\(|placement_score\(|placement_tie_breaker\(|best_placement_index\(" cluster-proof/cluster.mpl; then echo legacy-placement-helpers-still-present; exit 1; else echo legacy-placement-helpers-removed; fi'` | 0 | ✅ pass | 50ms |
| 2 | `cargo run -q -p meshc -- build cluster-proof` | 0 | ✅ pass | 11090ms |
| 3 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 14880ms |
| 4 | `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture` | 0 | ✅ pass | 7530ms |
| 5 | `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture` | 0 | ✅ pass | 15070ms |
| 6 | `cargo test -p meshc --test e2e_m045_s03 m045_s03_failover_ -- --nocapture` | 0 | ✅ pass | 15510ms |
| 7 | `cargo test -p meshc --test e2e_m045_s04 m045_s04_ -- --nocapture` | 101 | ❌ fail | 1250ms |
| 8 | `bash scripts/verify-m045-s04.sh` | 127 | ❌ fail | 90ms |


## Deviations

None.

## Known Issues

`cargo test -p meshc --test e2e_m045_s04 m045_s04_ -- --nocapture` still fails with `no test target named e2e_m045_s04`, and `bash scripts/verify-m045-s04.sh` still fails because the script is not in the tree yet. Those are downstream S04 gaps owned by later slice work, not regressions introduced by T01.

## Files Created/Modified

- `cluster-proof/cluster.mpl`
- `cluster-proof/tests/work.test.mpl`
- `cluster-proof/tests/config.test.mpl`
- `/Users/sn0w/Documents/dev/mesh-lang/.gsd/milestones/M045/slices/S04/tasks/T01-SUMMARY.md`


## Deviations
None.

## Known Issues
`cargo test -p meshc --test e2e_m045_s04 m045_s04_ -- --nocapture` still fails with `no test target named e2e_m045_s04`, and `bash scripts/verify-m045-s04.sh` still fails because the script is not in the tree yet. Those are downstream S04 gaps owned by later slice work, not regressions introduced by T01.
