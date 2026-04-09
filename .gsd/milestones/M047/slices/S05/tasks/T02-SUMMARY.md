---
id: T02
parent: S05
milestone: M047
provides: []
requires: []
affects: []
key_files: [".gsd/milestones/M047/slices/S05/tasks/T02-SUMMARY.md", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Do not rewrite `meshc init --clustered`, `tiny-cluster/`, or `cluster-proof/` first; resume from the declared-work wrapper/codegen seam that still requires public `request_key` and `attempt_id` arguments."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the prerequisite gap with a minimal temp package build under `.tmp/m047-s05-t02-probe/`: created `mesh.toml`, `main.mpl`, and `work.mpl` containing `@cluster pub fn add() -> Int`, then ran `cargo run -q -p meshc -- build .tmp/m047-s05-t02-probe --emit-llvm`. The build failed before any scaffold/package rebaseline work with `declared work wrapper '__declared_work_work_add' expected request_key and attempt_id arguments`, which is the expected blocking signal for this unit."
completed_at: 2026-04-01T16:13:12.543Z
blocker_discovered: true
---

# T02: Confirmed that the route-free scaffold/example cutover is blocked because the current compiler still rejects zero-ceremony `@cluster` work before any scaffold or package rebaseline can land.

> Confirmed that the route-free scaffold/example cutover is blocked because the current compiler still rejects zero-ceremony `@cluster` work before any scaffold or package rebaseline can land.

## What Happened
---
id: T02
parent: S05
milestone: M047
key_files:
  - .gsd/milestones/M047/slices/S05/tasks/T02-SUMMARY.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Do not rewrite `meshc init --clustered`, `tiny-cluster/`, or `cluster-proof/` first; resume from the declared-work wrapper/codegen seam that still requires public `request_key` and `attempt_id` arguments.
duration: ""
verification_result: passed
completed_at: 2026-04-01T16:13:12.545Z
blocker_discovered: true
---

# T02: Confirmed that the route-free scaffold/example cutover is blocked because the current compiler still rejects zero-ceremony `@cluster` work before any scaffold or package rebaseline can land.

**Confirmed that the route-free scaffold/example cutover is blocked because the current compiler still rejects zero-ceremony `@cluster` work before any scaffold or package rebaseline can land.**

## What Happened

Activated the test skill, read the T02 plan and prior T01 summary, and inspected the current route-free scaffold/example/test surfaces. The local public contract is still stale across the scaffold, tooling rails, `tiny-cluster/`, `cluster-proof/`, and the equal-surface helpers/tests: they all teach `@cluster pub fn execute_declared_work(_request_key :: String, _attempt_id :: String)` plus the legacy `Work.execute_declared_work` runtime name. Before changing those public surfaces, I verified whether the prerequisite no-ceremony compiler support had actually landed. It had not: a minimal temp package containing `@cluster pub fn add() -> Int do ... end` still fails in codegen with `declared work wrapper '__declared_work_work_add' expected request_key and attempt_id arguments`. I then read the declared-work wrapper/codegen seam in `compiler/mesh-codegen/src/declared.rs` and `compiler/mesh-codegen/src/codegen/expr.rs`, plus the current M047 compiler/runtime rails, and confirmed the missing capability is still the same T01 seam: the runtime injects hidden continuity metadata, but the generated wrapper still assumes those hidden values are public function parameters. I stopped there instead of forcing scaffold/example edits onto a contract the compiler does not yet support, wrote the blocker summary, and added a Knowledge entry so the next unit resumes from the actual compiler seam rather than redoing the scaffold-side investigation.

## Verification

Verified the prerequisite gap with a minimal temp package build under `.tmp/m047-s05-t02-probe/`: created `mesh.toml`, `main.mpl`, and `work.mpl` containing `@cluster pub fn add() -> Int`, then ran `cargo run -q -p meshc -- build .tmp/m047-s05-t02-probe --emit-llvm`. The build failed before any scaffold/package rebaseline work with `declared work wrapper '__declared_work_work_add' expected request_key and attempt_id arguments`, which is the expected blocking signal for this unit.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- build .tmp/m047-s05-t02-probe --emit-llvm` | 1 | ✅ pass | 0ms |


## Deviations

The task plan assumed the no-ceremony clustered-function contract already existed. Local reality did not match: I had to verify the prerequisite capability first, and that probe showed the task cannot honestly proceed as written until the T01 compiler/codegen seam lands.

## Known Issues

Zero-ceremony `@cluster` work is still unsupported in the current tree; even a minimal `@cluster pub fn add() -> Int` build fails in declared-work wrapper codegen. Because of that missing capability, the route-free scaffold, example packages, package smoke tests, and equal-surface rails still advertise the stale `execute_declared_work(request_key, attempt_id)` contract.

## Files Created/Modified

- `.gsd/milestones/M047/slices/S05/tasks/T02-SUMMARY.md`
- `.gsd/KNOWLEDGE.md`


## Deviations
The task plan assumed the no-ceremony clustered-function contract already existed. Local reality did not match: I had to verify the prerequisite capability first, and that probe showed the task cannot honestly proceed as written until the T01 compiler/codegen seam lands.

## Known Issues
Zero-ceremony `@cluster` work is still unsupported in the current tree; even a minimal `@cluster pub fn add() -> Int` build fails in declared-work wrapper codegen. Because of that missing capability, the route-free scaffold, example packages, package smoke tests, and equal-surface rails still advertise the stale `execute_declared_work(request_key, attempt_id)` contract.
