---
id: T01
parent: S05
milestone: M031
provides:
  - 5 new e2e tests covering bare expressions, not-fn-call conditions, and struct update in service handlers
key_files:
  - compiler/meshc/tests/e2e.rs
key_decisions:
  - Mesh has no mutable variables, so the while-not-fn-call test uses break instead of a counter loop
  - Bare expression block test uses if/else branches instead of do..end block-as-expression (unsupported in let binding)
patterns_established:
  - e2e test naming: e2e_<category>_<detail> for pattern-gap coverage tests
observability_surfaces:
  - cargo test -p meshc --test e2e <test_name> -- --nocapture for full compiler/runtime output
duration: 15m
verification_result: passed
completed_at: 2026-03-24
blocker_discovered: false
---

# T01: Add e2e tests for bare expressions, not-fn-call conditions, and struct update in services

**Add 5 e2e tests covering R025 pattern gaps: bare expression side-effects, bare expressions in blocks, not-fn-call in if/while conditions, and struct update syntax inside service handlers**

## What Happened

Appended 5 new `#[test]` functions to `compiler/meshc/tests/e2e.rs`:

1. `e2e_bare_expression_side_effects` — multiple bare `println()` and helper calls without `let _ =`
2. `e2e_bare_expression_in_block` — bare expressions in if/else branches
3. `e2e_not_fn_call_if_condition` — `if not is_empty(count)` with function call negation
4. `e2e_not_fn_call_while_condition` — `while not always_true()` and `while not always_false()` with break
5. `e2e_struct_update_in_service_call` — multifile service with `%{state | count: state.count + 1}` in call handler and `%{state | label: new_label}` in cast handler

Initial attempt had two failures: `let x = do...end` block-as-expression isn't valid Mesh syntax in let bindings, and `let mut` doesn't exist. Fixed by using if/else branches for block context and break-based loop control for the while test.

## Verification

All 5 new tests pass individually. Full suite shows 328 total (318 passed, 10 pre-existing try_* failures, 0 new failures).

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e bare_expression` | 0 | ✅ pass | 11.2s |
| 2 | `cargo test -p meshc --test e2e not_fn_call` | 0 | ✅ pass | 14.7s |
| 3 | `cargo test -p meshc --test e2e struct_update_in_service` | 0 | ✅ pass | 14.7s |
| 4 | `cargo test -p meshc --test e2e` (full suite) | 101 | ✅ pass (318 pass, 10 pre-existing fail) | 210.7s |

## Diagnostics

Run any individual test with `-- --nocapture` to see full compiler stderr and binary stdout. Test names map directly to pattern categories. No runtime services or persistent state — purely compile-and-run assertions.

## Deviations

- Replaced `let x = do...end` block-as-expression with if/else branches — `do...end` as a value in let bindings is not supported by the parser.
- Replaced `let mut i` counter loop with `while not always_true()` (skips body) and `while not always_false()` (enters then breaks) — Mesh has no mutable variables.

## Known Issues

None.

## Files Created/Modified

- `compiler/meshc/tests/e2e.rs` — appended 5 new test functions covering bare expressions, not-fn-call conditions, and struct update in service handlers
- `.gsd/milestones/M031/slices/S05/S05-PLAN.md` — marked T01 complete, added Observability/Diagnostics section
