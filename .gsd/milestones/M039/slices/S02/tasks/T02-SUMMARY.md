---
id: T02
parent: S02
milestone: M039
provides: []
requires: []
affects: []
key_files: ["cluster-proof/work.mpl", "cluster-proof/cluster.mpl", "cluster-proof/main.mpl", "cluster-proof/tests/work.test.mpl", ".gsd/milestones/M039/slices/S02/tasks/T02-SUMMARY.md"]
key_decisions: ["Switched the route helper surface toward exported accessor functions instead of direct cross-module struct field access because Mesh continued rejecting external field access on `TargetSelection` and related state.", "Kept the coordinator/result-registry design scalar-only across the distributed boundary; the unresolved work is now in Mesh service/Pid typing and block-shape compilation, not in the routing protocol choice."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the task verification commands after the refactor attempt. `cargo run -q -p meshc -- test cluster-proof/tests` failed during compile/type-check in `cluster-proof/work.mpl` and the updated helper tests. `cargo run -q -p meshc -- build cluster-proof` failed with the same `cluster-proof/work.mpl` errors, including unresolved `Pid`/service-state mismatches and coordinator block-shape problems. No runtime route proof was possible because the app does not build."
completed_at: 2026-03-28T10:49:13.257Z
blocker_discovered: false
---

# T02: Started the coordinator/result-registry refactor for `/work`, but the new Mesh service/actor path still does not compile or verify.

> Started the coordinator/result-registry refactor for `/work`, but the new Mesh service/actor path still does not compile or verify.

## What Happened
---
id: T02
parent: S02
milestone: M039
key_files:
  - cluster-proof/work.mpl
  - cluster-proof/cluster.mpl
  - cluster-proof/main.mpl
  - cluster-proof/tests/work.test.mpl
  - .gsd/milestones/M039/slices/S02/tasks/T02-SUMMARY.md
key_decisions:
  - Switched the route helper surface toward exported accessor functions instead of direct cross-module struct field access because Mesh continued rejecting external field access on `TargetSelection` and related state.
  - Kept the coordinator/result-registry design scalar-only across the distributed boundary; the unresolved work is now in Mesh service/Pid typing and block-shape compilation, not in the routing protocol choice.
duration: ""
verification_result: mixed
completed_at: 2026-03-28T10:49:13.258Z
blocker_discovered: false
---

# T02: Started the coordinator/result-registry refactor for `/work`, but the new Mesh service/actor path still does not compile or verify.

**Started the coordinator/result-registry refactor for `/work`, but the new Mesh service/actor path still does not compile or verify.**

## What Happened

Rewrote `cluster-proof/work.mpl` away from the original handler-side `self()` / `receive` attempt and moved the design toward a local coordinator service plus a result-registry service, while keeping the intended distributed routing protocol scalar-only. Added exported selection accessor helpers so `cluster-proof/tests/work.test.mpl` no longer depends on direct cross-module field access, and wired `start_work_services()` into `cluster-proof/main.mpl` so the eventual registered services will start before the HTTP server. The task did not reach a compiling state: verification still fails in the rewritten `cluster-proof/work.mpl` on Mesh-specific service/Pid typing and block-shape issues, and the updated helper tests remain blocked behind that broken route module. I stopped here because of the context-budget wrap-up instruction rather than continuing another debugging loop without a fresh context.

## Verification

Ran the task verification commands after the refactor attempt. `cargo run -q -p meshc -- test cluster-proof/tests` failed during compile/type-check in `cluster-proof/work.mpl` and the updated helper tests. `cargo run -q -p meshc -- build cluster-proof` failed with the same `cluster-proof/work.mpl` errors, including unresolved `Pid`/service-state mismatches and coordinator block-shape problems. No runtime route proof was possible because the app does not build.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- test cluster-proof/tests` | 1 | ❌ fail | 0ms |
| 2 | `cargo run -q -p meshc -- build cluster-proof` | 1 | ❌ fail | 0ms |


## Deviations

Stopped at the context-budget wrap-up point with a partial refactor on disk instead of a compiling coordinator-backed `/work` implementation.

## Known Issues

`cluster-proof/work.mpl` still fails to compile after the coordinator/result-registry rewrite; `cluster-proof/main.mpl` startup wiring depends on that module, so `cluster-proof` does not build; `cluster-proof/tests/work.test.mpl` now uses exported accessors but still does not pass because the imported work module remains broken and some assertions still need a clean compile surface to validate against.

## Files Created/Modified

- `cluster-proof/work.mpl`
- `cluster-proof/cluster.mpl`
- `cluster-proof/main.mpl`
- `cluster-proof/tests/work.test.mpl`
- `.gsd/milestones/M039/slices/S02/tasks/T02-SUMMARY.md`


## Deviations
Stopped at the context-budget wrap-up point with a partial refactor on disk instead of a compiling coordinator-backed `/work` implementation.

## Known Issues
`cluster-proof/work.mpl` still fails to compile after the coordinator/result-registry rewrite; `cluster-proof/main.mpl` startup wiring depends on that module, so `cluster-proof` does not build; `cluster-proof/tests/work.test.mpl` now uses exported accessors but still does not pass because the imported work module remains broken and some assertions still need a clean compile surface to validate against.
