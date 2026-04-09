---
estimated_steps: 5
estimated_files: 1
skills_used: []
---

# T01: Clean up worker.mpl — bare expressions, struct update, else-if, Bool conditions

**Slice:** S03 — Reference-Backend Dogfood Cleanup
**Milestone:** M031

## Description

Mechanical cleanup of `reference-backend/jobs/worker.mpl` (653 lines), the largest file and 80% of the anti-pattern surface. Four transformations:

1. **Remove `let _ =`** (44 instances): Delete the `let _ = ` prefix, leaving the expression as a bare statement. Bare expression statements are confirmed working (D028).
2. **Remove `== true`** (11 instances): Drop `== true` from Bool-returning expressions. Where it appears in conditions (`if fn_call() == true do`), change to `if fn_call() do` — trailing-closure disambiguation (S01) makes this safe.
3. **Struct update** (8 instances): Replace full `WorkerState { poll_ms: state.poll_ms, boot_id: state.boot_id, ... }` 18-field reconstructions with `%{state | changed_field: value}`. Most service call handlers only change 2-4 fields. Keep the one `WorkerState { ... }` that is the struct definition itself.
4. **Flatten nested if/else** (3 chains): Convert `if ... do ... else\n  if ... do` to `if ... do ... else if ... do`. Chains identified: NoteBoot handler, `worker_needs_restart`, `handle_claim_error`.

This is strictly behavior-preserving — same log messages, same JSON output, same control flow. The e2e tests should pass unchanged.

## Steps

1. Read `reference-backend/jobs/worker.mpl` fully. Identify all 44 `let _ =` lines, 11 `== true` lines, 8 struct reconstruction blocks, and 3 nested if/else chains.
2. Remove all `let _ = ` prefixes. Each becomes a bare expression statement.
3. Remove all `== true` suffixes. In conditions, `if fn_call() == true do` becomes `if fn_call() do`. In variable bindings like `let x = fn_call() == true`, just use `let x = fn_call()`.
4. Replace each full `WorkerState { ... }` reconstruction with `%{state | field1: val1, field2: val2}`, keeping only the fields that actually change. Use the preceding code to determine which fields change — in most service call handlers, the pattern is: compute new values, then build the next state.
5. Flatten the 3 nested if/else chains into `else if` form. Verify `cargo run -p meshc -- build reference-backend` succeeds.

## Must-Haves

- [ ] Zero `let _ =` in worker.mpl (`rg 'let _ =' reference-backend/jobs/worker.mpl` → 0)
- [ ] Zero `== true` in worker.mpl (`rg '== true' reference-backend/jobs/worker.mpl` → 0)
- [ ] Only 1 `WorkerState {` (the definition) remains (`rg 'WorkerState \{' reference-backend/jobs/worker.mpl` → 1)
- [ ] No nested if/else chains — all use `else if`
- [ ] `cargo run -p meshc -- build reference-backend` succeeds

## Verification

- `cargo run -p meshc -- build reference-backend` → success
- `rg 'let _ =' reference-backend/jobs/worker.mpl` → 0 matches
- `rg '== true' reference-backend/jobs/worker.mpl` → 0 matches
- `rg 'WorkerState \{' reference-backend/jobs/worker.mpl` → exactly 1 match (struct def)

## Inputs

- `reference-backend/jobs/worker.mpl` — the 653-line file containing 44 `let _ =`, 11 `== true`, 8 struct reconstructions, 3 nested if/else chains

## Expected Output

- `reference-backend/jobs/worker.mpl` — cleaned up with zero `let _ =`, zero `== true`, struct update syntax, `else if` chains

## Observability Impact

No runtime signals change — this is a strictly behavior-preserving refactoring. All log messages, `/health` JSON fields, and error shapes remain identical. A future agent inspects this task by re-running the anti-pattern greps (`rg 'let _ =' ...`, `rg '== true' ...`, `rg 'WorkerState \{' ...`) and confirming the build succeeds.
