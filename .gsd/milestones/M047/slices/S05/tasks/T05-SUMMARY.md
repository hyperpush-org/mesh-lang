---
id: T05
parent: S05
milestone: M047
provides: []
requires: []
affects: []
key_files: [".gsd/milestones/M047/slices/S05/tasks/T05-SUMMARY.md", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Do not land an opt-in Todo scaffold on the stale declared-work contract; resume only after ordinary `@cluster pub fn add() -> Int` builds without public continuity args."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the blocker with a minimal temp package under `.tmp/m047-s05-t05-probe/` containing `@cluster pub fn add() -> Int do 1 + 1 end`; `cargo run -q -p meshc -- build .tmp/m047-s05-t05-probe --emit-llvm` still fails in declared-work wrapper codegen before any Todo scaffold work can be exercised. Then ran the exact T05 verification filters from the plan and confirmed both currently execute zero tests, so they are not yet an honest acceptance surface for this task."
completed_at: 2026-04-01T16:27:50.343Z
blocker_discovered: true
---

# T05: Documented that the Todo starter is still blocked by missing no-ceremony `@cluster` support and zero-test verifier filters; no scaffold source changes landed.

> Documented that the Todo starter is still blocked by missing no-ceremony `@cluster` support and zero-test verifier filters; no scaffold source changes landed.

## What Happened
---
id: T05
parent: S05
milestone: M047
key_files:
  - .gsd/milestones/M047/slices/S05/tasks/T05-SUMMARY.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Do not land an opt-in Todo scaffold on the stale declared-work contract; resume only after ordinary `@cluster pub fn add() -> Int` builds without public continuity args.
duration: ""
verification_result: passed
completed_at: 2026-04-01T16:27:50.345Z
blocker_discovered: true
---

# T05: Documented that the Todo starter is still blocked by missing no-ceremony `@cluster` support and zero-test verifier filters; no scaffold source changes landed.

**Documented that the Todo starter is still blocked by missing no-ceremony `@cluster` support and zero-test verifier filters; no scaffold source changes landed.**

## What Happened

Read the T05 task contract, slice plan, prior task summaries, the current `meshc init`/scaffold sources, and the tooling test target. The local tree still has the stale route-free clustered contract in `compiler/mesh-pkg/src/scaffold.rs`, `tiny-cluster/work.mpl`, `cluster-proof/work.mpl`, and the declared-work codegen seam in `compiler/mesh-codegen/src/codegen/expr.rs` still hard-fails on ordinary no-ceremony `@cluster` work with `declared work wrapper '__declared_work_work_add' expected request_key and attempt_id arguments`. Because the Todo template contract explicitly requires clustered work on the no-ceremony `@cluster` model, I verified that prerequisite before changing CLI or scaffold code. The probe still fails exactly as the earlier slice tasks predicted, so there is no honest way to generate a real Todo starter that matches the task contract yet. I also checked the planned T05 verification commands and confirmed they do not currently prove anything: both filtered Cargo invocations exit 0 while printing `running 0 tests`. I recorded that verifier gotcha in `.gsd/KNOWLEDGE.md`, then stopped instead of landing a half-true selector/template path on top of an unsupported runtime/compiler contract.

## Verification

Verified the blocker with a minimal temp package under `.tmp/m047-s05-t05-probe/` containing `@cluster pub fn add() -> Int do 1 + 1 end`; `cargo run -q -p meshc -- build .tmp/m047-s05-t05-probe --emit-llvm` still fails in declared-work wrapper codegen before any Todo scaffold work can be exercised. Then ran the exact T05 verification filters from the plan and confirmed both currently execute zero tests, so they are not yet an honest acceptance surface for this task.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- build .tmp/m047-s05-t05-probe --emit-llvm` | 1 | ✅ pass | 6000ms |
| 2 | `cargo test -p mesh-pkg m047_s05 -- --nocapture` | 0 | ✅ pass | 1000ms |
| 3 | `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture` | 0 | ✅ pass | 3000ms |


## Deviations

The task plan assumed the no-ceremony `@cluster` prerequisite had already landed and that T05-specific verification filters existed. Local reality did not match: the compiler still rejects ordinary `@cluster` work before any Todo template can be generated honestly, and both planned verification filters currently match zero tests.

## Known Issues

The upstream blocker from T03/T04 is still unresolved: declared-work wrapper generation still requires public `request_key` and `attempt_id` arguments, so an opt-in Todo scaffold built on the intended contract would be false. There is also no current T05 proof rail in `mesh-pkg` or `tooling_e2e`; both planned filters return success with zero matched tests.

## Files Created/Modified

- `.gsd/milestones/M047/slices/S05/tasks/T05-SUMMARY.md`
- `.gsd/KNOWLEDGE.md`


## Deviations
The task plan assumed the no-ceremony `@cluster` prerequisite had already landed and that T05-specific verification filters existed. Local reality did not match: the compiler still rejects ordinary `@cluster` work before any Todo template can be generated honestly, and both planned verification filters currently match zero tests.

## Known Issues
The upstream blocker from T03/T04 is still unresolved: declared-work wrapper generation still requires public `request_key` and `attempt_id` arguments, so an opt-in Todo scaffold built on the intended contract would be false. There is also no current T05 proof rail in `mesh-pkg` or `tooling_e2e`; both planned filters return success with zero matched tests.
