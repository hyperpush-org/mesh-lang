---
id: T01
parent: S03
milestone: M031
provides:
  - worker.mpl cleaned of all let _ =, == true, full struct reconstructions, nested if/else chains
key_files:
  - reference-backend/jobs/worker.mpl
key_decisions:
  - Struct update syntax %{state | field: value} used for all service call state transitions
patterns_established:
  - Bare expression statements for side-effect-only calls (println, service calls, spawn)
  - Struct update syntax for partial state changes instead of full reconstruction
  - else if chains instead of nested if/else blocks
observability_surfaces:
  - none (behavior-preserving refactoring — all existing log messages and /health fields unchanged)
duration: 15m
verification_result: passed
completed_at: 2026-03-24
blocker_discovered: false
---

# T01: Clean up worker.mpl — bare expressions, struct update, else-if, Bool conditions

**Removed all 44 `let _ =` prefixes, 11 `== true` comparisons, 8 full struct reconstructions, and 3 nested if/else chains from worker.mpl — build and formatter pass clean.**

## What Happened

Applied four mechanical transformations to `reference-backend/jobs/worker.mpl`:

1. **Bare expressions (44 sites):** Removed all `let _ = ` prefixes from side-effect-only calls — `println(...)`, `JobWorkerState.note_*(...)`, `log_*(...)`, `record_*(...)`, `spawn(...)`, `Process.register(...)`. Each becomes a bare expression statement.

2. **Bool conditions (11 sites):** Removed `== true` from all boolean checks — `if had_boot == true do` → `if had_boot do`, `String.contains(...) == true` → `String.contains(...)`, `should_hold_after_claim(job) == true` → `should_hold_after_claim(job)`, `continue_loop == true` → `continue_loop`.

3. **Struct update (8 sites):** Replaced all full `WorkerState { poll_ms: state.poll_ms, ... }` 15-field reconstructions with `%{state | changed_field: value}`. Most service call handlers only change 2-4 fields, so the updates are dramatically shorter. The one `WorkerState { ... }` in the `init` function is preserved since it's the actual construction.

4. **else if chains (3 sites):** Flattened nested `else\n  if` to `else if` in NoteBoot handler (exit reason logic), `worker_needs_restart`, and `handle_claim_error`.

Ran `meshc fmt` to ensure formatter compliance after the rewrite.

## Verification

- `rg 'let _ =' reference-backend/jobs/worker.mpl` → 0 matches ✅
- `rg '== true' reference-backend/jobs/worker.mpl` → 0 matches ✅
- `rg 'WorkerState \{' reference-backend/jobs/worker.mpl` → 1 match (init only) ✅
- `rg -n 'else if' reference-backend/jobs/worker.mpl` → 3 matches (all three chains) ✅
- `cargo run -p meshc -- build reference-backend` → success ✅
- `cargo run -p meshc -- fmt --check reference-backend` → 11 files already formatted ✅

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `rg 'let _ =' reference-backend/jobs/worker.mpl` | 1 (no match) | ✅ pass | <1s |
| 2 | `rg '== true' reference-backend/jobs/worker.mpl` | 1 (no match) | ✅ pass | <1s |
| 3 | `rg -c 'WorkerState \{' reference-backend/jobs/worker.mpl` | 0 (1 match) | ✅ pass | <1s |
| 4 | `cargo run -p meshc -- build reference-backend` | 0 | ✅ pass | ~3s |
| 5 | `cargo run -p meshc -- fmt --check reference-backend` | 0 | ✅ pass | ~3s |

## Diagnostics

No runtime signals changed. Inspect via the same anti-pattern greps or by running the e2e suite. If a regression surfaces in T02's full suite run, `worker.mpl` is the first file to diff.

## Deviations

- The `handle_claimed_job` function originally wrapped the `should_hold_after_claim` check in `let _ = if ... do ... end`. Since removing `let _ =` makes this a bare `if` expression, I kept the if/else with an explicit `0` return in the else branch to maintain the same control flow. The subsequent `if should_crash_after_claim(job) do` determines the function's return value.

## Known Issues

- Remaining anti-patterns in other files (9 `let _ =` across 4 files, 4 `== true` in health.mpl) — addressed by T02.

## Files Created/Modified

- `reference-backend/jobs/worker.mpl` — all four cleanup transformations applied and formatter-compliant
- `.gsd/milestones/M031/slices/S03/S03-PLAN.md` — added Observability/Diagnostics section and diagnostic verification step
- `.gsd/milestones/M031/slices/S03/tasks/T01-PLAN.md` — added Observability Impact section
