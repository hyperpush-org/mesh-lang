---
id: T02
parent: S05
milestone: M028
provides:
  - Restored the compiled supervisor child-spec bridge so `reference-backend` can be wired onto a real source-level supervisor in the next unit.
key_files:
  - compiler/mesh-codegen/src/codegen/expr.rs
  - .gsd/milestones/M028/slices/S05/S05-PLAN.md
  - .gsd/milestones/M028/slices/S05/tasks/T02-SUMMARY.md
key_decisions:
  - Fix the known supervisor config serialization mismatch first instead of attempting backend supervision wiring on top of a still-broken source-level supervisor path.
  - Stop at an honest wrap-up when the context-budget warning fired rather than claiming the `reference-backend` wiring or `/health` contract was done.
patterns_established:
  - Source-level supervisor work should still be checked with a direct supervisor call, not only the existing banner-based `e2e_supervisors` smoke tests.
  - Mesh `Pid` values are not directly stringifiable or comparable to `0` in user code, so future health metadata should use boot identities and tick-age-derived liveness rather than raw PID serialization.
observability_surfaces:
  - cargo test -p mesh-rt supervisor::tests:: --lib -- --nocapture
  - cargo test -p meshc --test e2e_supervisors -- --nocapture
  - .gsd/milestones/M028/slices/S05/S05-PLAN.md
duration: 1h
verification_result: partial
completed_at: 2026-03-23 02:35:00 EDT
blocker_discovered: false
---

# T02: Wire `reference-backend` worker startup under supervision and expose restart bookkeeping

**Aligned compiled supervisor child-spec bytes with the runtime parser; the actual `reference-backend` supervision wiring remains unfinished.**

## What Happened

I activated the required skills, read the slice/task contract, and inspected the current `reference-backend` worker startup path. Before changing the backend files, I verified the T01 carry-forward hypothesis against the runtime: `compiler/mesh-codegen/src/codegen/expr.rs::codegen_supervisor_start(...)` was still emitting a shorter child-spec layout than `compiler/mesh-rt/src/actor/mod.rs::parse_supervisor_config(...)` consumes.

I fixed that compiler bridge by adding the missing `start_args_ptr`, `start_args_size`, `shutdown_type`, and `shutdown_timeout_ms` fields to the serialized child-spec bytes while keeping source-level supervisor children on zero captured args. After that change, the runtime supervisor donor tests and compiled supervisor e2e tests both passed again.

I also ran two short throwaway probes inside the worktree to validate Mesh surface constraints before touching the backend:

- a `Pid` probe showed that Mesh user code cannot directly `String.from(pid)` or compare a `Pid` to `0`, which matters for how `/health` should expose worker identity/liveness later;
- a direct-call supervisor probe no longer stayed silent and emitted repeated `child_boot` lines, which is materially different from T01’s “child never boots” repro, but I did not have enough remaining context budget to debug that behavior further or finish the backend rewrite safely.

At the wrap-up point, the only shipped code change was the supervisor config serialization fix in `compiler/mesh-codegen/src/codegen/expr.rs`. I did **not** finish the planned edits to `reference-backend/main.mpl`, `reference-backend/runtime/registry.mpl`, `reference-backend/jobs/worker.mpl`, `reference-backend/api/health.mpl`, or `compiler/meshc/tests/e2e_reference_backend.rs`.

## Verification

I reran the supervisor-focused Rust verification after the compiler fix:

- `cargo test -p mesh-rt supervisor::tests:: --lib -- --nocapture`
- `cargo test -p meshc --test e2e_supervisors -- --nocapture`

Both passed after the serialization change.

I then used short local probe packages in the worktree to check runtime assumptions:

- a `Pid` probe intentionally failed compilation, confirming raw `Pid` values are not directly stringifiable or comparable in Mesh source;
- a direct-call supervisor probe built successfully and, when run, produced repeated `child_boot` output until timeout instead of the earlier “silent child” behavior.

Those probes were deleted during wrap-up so they do not remain as repo noise.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-rt supervisor::tests:: --lib -- --nocapture` | 0 | ✅ pass | 1.30s |
| 2 | `cargo test -p meshc --test e2e_supervisors -- --nocapture` | 0 | ✅ pass | 34.11s |
| 3 | `cargo run -p meshc -- build .tmp_pid_probe` | 1 | ❌ fail | 3.00s |
| 4 | `cargo run -p meshc -- build .tmp_supervisor_probe3` | 0 | ✅ pass | 5.22s |
| 5 | `./.tmp_supervisor_probe3/.tmp_supervisor_probe3` | 124 | ❌ fail | 30.00s timeout |

## Diagnostics

Fastest resume path for the next unit:

1. Start from the now-fixed compiler bridge in `compiler/mesh-codegen/src/codegen/expr.rs`; do **not** spend more time rediscovering the old child-spec mismatch.
2. Update `reference-backend/runtime/registry.mpl` first so the worker can resolve `pool` / `poll_ms` via runtime lookups instead of captured supervisor args.
3. Then rewrite `reference-backend/jobs/worker.mpl` around a zero-arg supervised worker actor plus durable state fields such as boot identity, restart count, and tick-derived liveness.
4. Change `reference-backend/main.mpl` to call the supervisor-backed worker start path directly rather than detached `spawn(...)`.
5. Extend `compiler/meshc/tests/e2e_reference_backend.rs` with the ignored `e2e_reference_backend_worker_supervision_starts` proof and only then run the slice’s backend verification command.

## Deviations

I made an unplanned prerequisite fix in `compiler/mesh-codegen/src/codegen/expr.rs` before touching the backend package because the local codebase still matched T01’s broken supervisor bridge. I did not complete the planned backend/package/test edits after that because the context-budget warning required immediate wrap-up.

## Known Issues

- T02 is still incomplete.
- `reference-backend` still uses the old detached worker startup path because its Mesh files were not updated in this unit.
- The direct-call supervisor probe changed from “silent child” to “repeated child boot”, so the next unit should treat source-level supervisor behavior as improved-but-not-fully-characterized rather than fully closed.
- The required ignored backend proof `e2e_reference_backend_worker_supervision_starts` has not been added yet.

## Files Created/Modified

- `compiler/mesh-codegen/src/codegen/expr.rs` — aligned source-level supervisor child-spec serialization with the runtime parser’s expected layout.
- `.gsd/milestones/M028/slices/S05/S05-PLAN.md` — added a T02 resume note describing the verified prerequisite fix and the unfinished backend work.
- `.gsd/milestones/M028/slices/S05/tasks/T02-SUMMARY.md` — durable partial handoff for the next unit.
