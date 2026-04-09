---
id: T01
parent: S05
milestone: M028
provides:
  - Reproduced the compiled-supervisor false-positive gap and left concrete resume notes for the bridge fix.
key_files:
  - compiler/mesh-codegen/src/codegen/expr.rs
  - compiler/mesh-rt/src/actor/mod.rs
  - compiler/meshc/tests/e2e_supervisors.rs
  - .gsd/milestones/M028/slices/S05/S05-PLAN.md
key_decisions:
  - Stop at investigation-only wrap-up when the context-budget warning fired instead of making an unverified bridge change.
patterns_established:
  - Use direct supervisor calls plus child lifecycle markers; banner-only wrapper-actor tests are false positives for compiled supervisor health.
observability_surfaces:
  - cargo test -p meshc --test e2e_supervisors -- --nocapture
  - ad hoc direct-call supervisor probe under .tmp_supervisor_probe2/
duration: 1h
verification_result: partial
completed_at: 2026-03-23 18:55:37 EDT
blocker_discovered: false
---

# T01: Repair and prove Mesh source-level supervisor child lifecycle

**Reproduced the compiled-supervisor false-positive gap and isolated the likely child-spec serialization mismatch for the next executor.**

## What Happened

I activated the task-relevant debugging/testing/review skills, read the slice/task plans, and traced the current supervisor path through `compiler/mesh-codegen/src/mir/lower.rs`, `compiler/mesh-codegen/src/codegen/expr.rs`, `compiler/mesh-rt/src/actor/mod.rs`, `compiler/mesh-rt/src/actor/supervisor.rs`, and `compiler/meshc/tests/e2e_supervisors.rs`.

The key finding is that the current `e2e_supervisors` suite is still a false-positive surface: it proves that a wrapper actor can be spawned and a banner can print, but it does not prove that `mesh_supervisor_start(...)` successfully created a real supervisor or started any child.

I then reproduced the real runtime gap with a direct-call probe project under `.tmp_supervisor_probe2/`. That program called `BootSup()` directly, used a temporary child that should have printed `child_boot` on startup, slept briefly, and then printed `main_done`. The compiled binary exited `0` and printed only `main_done`, which means the compiled supervisor function returned while no supervised child boot marker ever appeared.

The strongest locally verified hypothesis is that `compiler/mesh-codegen/src/codegen/expr.rs::codegen_supervisor_start(...)` serializes fewer child-spec fields than `compiler/mesh-rt/src/actor/mod.rs::parse_supervisor_config(...)` consumes. Specifically, the runtime parser expects `start_fn`, `start_args_ptr`, `start_args_size`, `restart_type`, `shutdown_type`, `shutdown_timeout_ms`, and `child_type`, while the current serializer only writes the function pointer plus a reduced tail.

I stopped there because the context-budget warning required wrap-up before I could safely implement and re-verify the bridge change.

## Verification

Baseline donor/runtime verification still passes:

- `cargo test -p mesh-rt supervisor::tests:: --lib -- --nocapture`
- `cargo test -p meshc --test e2e_supervisors -- --nocapture`

Those results are important but insufficient, because the source-level suite is still banner-based.

Additional manual repro performed during investigation:

- Built a temporary Mesh project in `.tmp_supervisor_probe2/project/` whose `main()` called `BootSup()` directly.
- Expected output after a healthy compiled supervisor bridge: `child_boot` followed by `main_done`.
- Actual output: only `main_done`.

That direct-call probe is the concrete evidence that the compiled supervisor path is still broken even though the existing `e2e_supervisors` test binary exits successfully.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-rt supervisor::tests:: --lib -- --nocapture` | 0 | ✅ pass | 4.20s |
| 2 | `cargo test -p meshc --test e2e_supervisors -- --nocapture` | 0 | ✅ pass | 17.26s |

## Diagnostics

To resume this task quickly:

1. Start with `compiler/mesh-codegen/src/codegen/expr.rs::codegen_supervisor_start(...)` and align its child-spec byte layout with `compiler/mesh-rt/src/actor/mod.rs::parse_supervisor_config(...)`.
2. Re-run the direct-call reproduction pattern from `.tmp_supervisor_probe2/project/main.mpl` (or recreate it cleanly) and confirm that `child_boot` appears before `main_done`.
3. Only after the bridge is real, harden `compiler/meshc/tests/e2e_supervisors.rs` and the `tests/e2e/supervisor_*.mpl` fixtures so they assert child boot/restart/restart-limit markers instead of supervisor banners.

## Deviations

I did not implement the planned code/test changes because the context-budget warning required immediate wrap-up. I recorded the reproduced failure and the resume path instead.

## Known Issues

- T01 is still incomplete.
- `compiler/meshc/tests/e2e_supervisors.rs` currently passes without proving that compiled supervisors start children.
- A direct-call compiled supervisor probe reproduced missing child boot behavior, strongly implicating the compiler/runtime child-spec serialization bridge.

## Files Created/Modified

- `.gsd/milestones/M028/slices/S05/tasks/T01-SUMMARY.md` — durable task summary with reproduction evidence and resume notes.
- `.gsd/milestones/M028/slices/S05/S05-PLAN.md` — added a resume note under T01 without falsely marking the task complete.
- `.tmp_supervisor_probe2/project/main.mpl` — temporary direct-call supervisor probe used to reproduce the missing child boot signal.
- `.tmp_supervisor_probe2/stdout.txt` — captured probe output showing `main_done` without `child_boot`.
