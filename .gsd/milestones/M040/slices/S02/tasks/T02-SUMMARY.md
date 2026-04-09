---
id: T02
parent: S02
milestone: M040
provides: []
requires: []
affects: []
key_files: ["cluster-proof/cluster.mpl", "cluster-proof/work.mpl", "cluster-proof/config.mpl", "cluster-proof/main.mpl", "cluster-proof/tests/work.test.mpl", "cluster-proof/tests/config.test.mpl", "cluster-proof/docker-entrypoint.sh", "cluster-proof/fly.toml", ".gsd/milestones/M040/slices/S02/tasks/T02-SUMMARY.md"]
key_decisions: ["Keep the durability policy on the existing env rail via `CLUSTER_PROOF_DURABILITY` instead of introducing a second bootstrap path.", "Preserve the legacy `GET /work` proof rail as a compatibility adapter that can still dispatch to the peer when healthy membership exists."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Re-ran the exact failing M039/S03 e2e gate, then re-ran the slice-level `cluster-proof` package tests and full package build after the partial implementation. All three checks are still red. The current concrete failure points are: `cargo run -q -p meshc -- test cluster-proof/tests` fails because the new tests still use unsupported non-string `assert_eq(...)` shapes, and `cargo run -q -p meshc -- build cluster-proof` fails with `Unsupported binop type: String`, which points at the attempted canonical string-ordering path."
completed_at: 2026-03-28T19:26:04.229Z
blocker_discovered: false
---

# T02: Partially wired canonical placement and durability-policy env handling for cluster-proof, but verification is still failing and needs a fresh follow-up unit.

> Partially wired canonical placement and durability-policy env handling for cluster-proof, but verification is still failing and needs a fresh follow-up unit.

## What Happened
---
id: T02
parent: S02
milestone: M040
key_files:
  - cluster-proof/cluster.mpl
  - cluster-proof/work.mpl
  - cluster-proof/config.mpl
  - cluster-proof/main.mpl
  - cluster-proof/tests/work.test.mpl
  - cluster-proof/tests/config.test.mpl
  - cluster-proof/docker-entrypoint.sh
  - cluster-proof/fly.toml
  - .gsd/milestones/M040/slices/S02/tasks/T02-SUMMARY.md
key_decisions:
  - Keep the durability policy on the existing env rail via `CLUSTER_PROOF_DURABILITY` instead of introducing a second bootstrap path.
  - Preserve the legacy `GET /work` proof rail as a compatibility adapter that can still dispatch to the peer when healthy membership exists.
duration: ""
verification_result: mixed
completed_at: 2026-03-28T19:26:04.230Z
blocker_discovered: false
---

# T02: Partially wired canonical placement and durability-policy env handling for cluster-proof, but verification is still failing and needs a fresh follow-up unit.

**Partially wired canonical placement and durability-policy env handling for cluster-proof, but verification is still failing and needs a fresh follow-up unit.**

## What Happened

Reproduced the exact red path first and found that the immediate blocker was a full `cluster-proof` build failure, not yet a routing assertion mismatch: `cluster-proof/work.mpl` still had a boolean `and` shape that tripped LLVM verification in the full app build, which also caused the M039/S03 rejoin e2e to fail. Then started the T02 implementation: `cluster-proof/cluster.mpl` now has an in-progress canonical-membership / placement layer, `cluster-proof/work.mpl` was updated toward owner/replica selection plus durability-policy logging, `cluster-proof/config.mpl` gained a small `CLUSTER_PROOF_DURABILITY` seam, and the existing operator rail was threaded through `cluster-proof/docker-entrypoint.sh` and `cluster-proof/fly.toml`. The implementation did not reach a green state before wrap-up because two Mesh-language/tooling constraints surfaced during the rerun: Mesh `case` arms only allow a single expression, and the current compiler/runtime path does not support string ordering operators for the canonical sorter/scorer shape that was attempted. The multi-step `case` branches were flattened into helper-call shapes, but the canonical placement strategy and new tests still need a follow-up unit.

## Verification

Re-ran the exact failing M039/S03 e2e gate, then re-ran the slice-level `cluster-proof` package tests and full package build after the partial implementation. All three checks are still red. The current concrete failure points are: `cargo run -q -p meshc -- test cluster-proof/tests` fails because the new tests still use unsupported non-string `assert_eq(...)` shapes, and `cargo run -q -p meshc -- build cluster-proof` fails with `Unsupported binop type: String`, which points at the attempted canonical string-ordering path.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_rejoins_and_routes_to_peer_again_without_manual_repair -- --nocapture` | 101 | ❌ fail | 17800ms |
| 2 | `cargo run -q -p meshc -- test cluster-proof/tests` | 1 | ❌ fail | 10800ms |
| 3 | `cargo run -q -p meshc -- build cluster-proof` | 1 | ❌ fail | 6300ms |


## Deviations

Had to spend part of the unit on a pre-existing full-build failure (`PHI node entries do not match predecessors`) before the task-specific placement/policy work could be verified. Also touched `cluster-proof/main.mpl` for startup-policy logging/validation even though it was not listed in the task’s expected-output file list, because the policy surface needed a real startup seam.

## Known Issues

`cluster-proof/cluster.mpl` still contains an attempted canonical placement implementation that relies on unsupported `String` ordering in the current Mesh compiler path. The new test files use unsupported `assert_eq(...)` shapes on `Int` / `List` values and do not compile yet. The task is not actually verified complete; a fresh follow-up unit is required to finish the canonical placement strategy and get the package build + M039/S03 regression proof back to green.

## Files Created/Modified

- `cluster-proof/cluster.mpl`
- `cluster-proof/work.mpl`
- `cluster-proof/config.mpl`
- `cluster-proof/main.mpl`
- `cluster-proof/tests/work.test.mpl`
- `cluster-proof/tests/config.test.mpl`
- `cluster-proof/docker-entrypoint.sh`
- `cluster-proof/fly.toml`
- `.gsd/milestones/M040/slices/S02/tasks/T02-SUMMARY.md`


## Deviations
Had to spend part of the unit on a pre-existing full-build failure (`PHI node entries do not match predecessors`) before the task-specific placement/policy work could be verified. Also touched `cluster-proof/main.mpl` for startup-policy logging/validation even though it was not listed in the task’s expected-output file list, because the policy surface needed a real startup seam.

## Known Issues
`cluster-proof/cluster.mpl` still contains an attempted canonical placement implementation that relies on unsupported `String` ordering in the current Mesh compiler path. The new test files use unsupported `assert_eq(...)` shapes on `Int` / `List` values and do not compile yet. The task is not actually verified complete; a fresh follow-up unit is required to finish the canonical placement strategy and get the package build + M039/S03 regression proof back to green.
