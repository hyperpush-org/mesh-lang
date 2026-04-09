---
id: T03
parent: S02
milestone: M043
provides: []
requires: []
affects: []
key_files: ["cluster-proof/main.mpl", "cluster-proof/config.mpl", "cluster-proof/work_continuity.mpl", "cluster-proof/tests/config.test.mpl", "cluster-proof/tests/work.test.mpl", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M043/slices/S02/tasks/T03-SUMMARY.md"]
key_decisions: ["D178: Promotion and missing-status error paths now re-read runtime authority truth and fail closed with explicit authority-status errors instead of falling back to startup env-derived role or epoch fields."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed cargo run -q -p meshc -- test cluster-proof/tests, cargo run -q -p meshc -- build cluster-proof, and the exact combined slice/task verification command cargo run -q -p meshc -- test cluster-proof/tests && cargo run -q -p meshc -- build cluster-proof."
completed_at: 2026-03-29T08:51:56.377Z
blocker_discovered: false
---

# T03: Switched cluster-proof to runtime-backed authority status and added the explicit /promote operator route.

> Switched cluster-proof to runtime-backed authority status and added the explicit /promote operator route.

## What Happened
---
id: T03
parent: S02
milestone: M043
key_files:
  - cluster-proof/main.mpl
  - cluster-proof/config.mpl
  - cluster-proof/work_continuity.mpl
  - cluster-proof/tests/config.test.mpl
  - cluster-proof/tests/work.test.mpl
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M043/slices/S02/tasks/T03-SUMMARY.md
key_decisions:
  - D178: Promotion and missing-status error paths now re-read runtime authority truth and fail closed with explicit authority-status errors instead of falling back to startup env-derived role or epoch fields.
duration: ""
verification_result: passed
completed_at: 2026-03-29T08:51:56.378Z
blocker_discovered: false
---

# T03: Switched cluster-proof to runtime-backed authority status and added the explicit /promote operator route.

**Switched cluster-proof to runtime-backed authority status and added the explicit /promote operator route.**

## What Happened

Reworked cluster-proof so startup continuity env is only topology input and no longer the source of live role, epoch, or replication-health truth after promotion. In cluster-proof/work_continuity.mpl I added a narrow runtime-authority seam around Continuity.authority_status() and Continuity.promote(), reused that truth for missing-status and invalid-selection payloads, and made authority failures fail closed with explicit authority-status errors. In cluster-proof/main.mpl I replaced env-derived membership truth with runtime authority reads, added POST /promote, and moved authority logging onto the runtime-backed startup path. I also removed the env-derived live-authority getters from cluster-proof/config.mpl, extended the package tests for promotion and fenced-rejoin truth, and fixed the one router-level compiler regression by annotating the imported authority struct parameter type in main.mpl.

## Verification

Passed cargo run -q -p meshc -- test cluster-proof/tests, cargo run -q -p meshc -- build cluster-proof, and the exact combined slice/task verification command cargo run -q -p meshc -- test cluster-proof/tests && cargo run -q -p meshc -- build cluster-proof.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 10000ms |
| 2 | `cargo run -q -p meshc -- build cluster-proof` | 0 | ✅ pass | 7602ms |
| 3 | `cargo run -q -p meshc -- test cluster-proof/tests && cargo run -q -p meshc -- build cluster-proof` | 0 | ✅ pass | 16464ms |


## Deviations

cluster-proof/work.mpl did not need direct code changes even though it was on the planned touch list. The payload structs already carried the needed authority fields, so the actual drift lived in the consumer and router layers.

## Known Issues

None.

## Files Created/Modified

- `cluster-proof/main.mpl`
- `cluster-proof/config.mpl`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/tests/config.test.mpl`
- `cluster-proof/tests/work.test.mpl`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M043/slices/S02/tasks/T03-SUMMARY.md`


## Deviations
cluster-proof/work.mpl did not need direct code changes even though it was on the planned touch list. The payload structs already carried the needed authority fields, so the actual drift lived in the consumer and router layers.

## Known Issues
None.
