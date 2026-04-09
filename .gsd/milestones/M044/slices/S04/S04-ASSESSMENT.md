# S04 closeout assessment

S04 is **not ready for slice completion yet**.

## What changed during this closeout attempt

- Added the missing `compiler/meshc/tests/e2e_m044_s04.rs` target so the planned S04 filters exist.
- Added `automatic_promotion_` / `automatic_recovery_` unit-test names in `compiler/mesh-rt/src/dist/continuity.rs` so the task-plan verification command is now truthful.
- Confirmed the runtime seam is no longer the old T04 blocker: `compiler/mesh-rt/src/dist/node.rs` now emits `automatic_promotion` and `automatic_recovery` transitions from the live disconnect path.
- Confirmed the new unit rails pass:
  - `cargo test -p mesh-rt automatic_promotion_ -- --nocapture`
  - `cargo test -p mesh-rt automatic_recovery_ -- --nocapture`
- Confirmed the new compiler-side manual-surface rail is present and passing:
  - `cargo test -p meshc --test e2e_m044_s04 m044_s04_manual_surface_ -- --nocapture`

## Current blocker

The destructive S04 e2e still fails in a **real runtime execution seam** after promotion succeeds.

Observed live sequence on the promoted standby:

- `[mesh-rt continuity] transition=promote ... next_epoch=1`
- `[mesh-rt continuity] transition=automatic_promotion ...`
- `[mesh-rt continuity] transition=recovery_rollover ... next_attempt_id=attempt-1`
- `[mesh-rt continuity] transition=submit ... attempt-1 ... owner=standby ...`
- `[mesh-rt continuity] transition=automatic_recovery ... runtime_name=WorkContinuity.execute_declared_work`
- process exits early with `ExitStatus(unix_wait_status(10))` **before** the first `[cluster-proof] work executed ...` log

This means the remaining blocker is not missing promotion logic, missing auto-recovery logic, or a missing e2e target. The blocker is the declared-handler execution path after auto-recovery on the promoted standby.

## Resume here

Start in this order:

1. `cluster-proof/work_continuity.mpl::execute_declared_work`
2. `compiler/mesh-rt/src/dist/node.rs::spawn_declared_work_local`
3. any codegen/runtime seam that marshals declared-handler string arguments for locally spawned declared handlers

The harness-side placement predictor was already corrected to match the runtime's current canonical membership hashing, so do **not** spend the next pass re-debugging request-key placement or docs drift first.

## Commands already run in this closeout attempt

- `cargo test -p mesh-rt automatic_promotion_ -- --nocapture` ✅
- `cargo test -p mesh-rt automatic_recovery_ -- --nocapture` ✅
- `cargo test -p meshc --test e2e_m044_s04 m044_s04_manual_surface_ -- --nocapture` ✅
- `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture` ❌ blocked by standby crash after `automatic_recovery`

## Why the slice was not completed

The user asked for honest slice completion. The destructive acceptance rail is still red, so calling `gsd_complete_slice` would overclaim S04.