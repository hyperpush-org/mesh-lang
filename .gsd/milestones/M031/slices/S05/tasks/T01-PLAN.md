---
estimated_steps: 4
estimated_files: 1
skills_used: []
---

# T01: Add e2e tests for bare expressions, not-fn-call conditions, and struct update in services

**Slice:** S05 — Language Test Expansion
**Milestone:** M031

## Description

Add 5–6 new e2e tests to `compiler/meshc/tests/e2e.rs` covering the 3 remaining R025 pattern gaps: bare expression statements, `not fn_call()` in control-flow conditions, and struct update syntax inside service handlers. These patterns are exercised in the dogfood codebases but have no isolated regression tests.

## Steps

1. Read the end of `compiler/meshc/tests/e2e.rs` to find the append point (file is ~6647 lines, append after the last test).

2. Add bare expression tests (2 tests):
   - `e2e_bare_expression_side_effects`: Multiple bare `println()` calls and a helper function call without `let _ =`. Proves bare expressions compile and execute as side effects.
   - `e2e_bare_expression_in_block`: Bare expressions inside `do ... end` blocks, if branches, and function bodies. Proves bare expressions work in nested block contexts.
   Both use `compile_and_run`. All functions must have typed signatures.

3. Add `not fn_call()` condition tests (2 tests):
   - `e2e_not_fn_call_if_condition`: `if not is_empty(list) do ... end` style — a function returning Bool, negated with `not` in an `if` condition. Exercises trailing-closure disambiguation with unary `not` preceding a function call.
   - `e2e_not_fn_call_while_condition`: `while not done() do ... end` style — uses a mutable counter pattern to prove `while not fn_call() do` loops execute correctly.
   Both use `compile_and_run`. All functions must have typed signatures.

4. Add struct update in service handler test (1–2 tests):
   - `e2e_struct_update_in_service_call`: A service with struct state (e.g., `CounterState { count :: Int, label :: String }`). The `call` handler uses `%{state | count: state.count + 1}` to return updated state. The `cast` handler uses `%{state | label: new_label}`. Proves struct update works inside the `(next_state, value)` tuple return pattern of service handlers.
   Use `compile_multifile_and_run` with a service module and a main module. All functions must have typed signatures.

## Must-Haves

- [ ] 2 bare expression tests pass: `cargo test -p meshc --test e2e bare_expression`
- [ ] 2 not-fn-call condition tests pass: `cargo test -p meshc --test e2e not_fn_call`
- [ ] 1+ struct update in service tests pass: `cargo test -p meshc --test e2e struct_update_in_service`
- [ ] Full suite regression-free: 328+ pass, 10 pre-existing `try_*` failures unchanged

## Verification

- `cargo test -p meshc --test e2e bare_expression` — 2 tests pass
- `cargo test -p meshc --test e2e not_fn_call` — 2 tests pass
- `cargo test -p meshc --test e2e struct_update_in_service` — 1+ tests pass
- `cargo test -p meshc --test e2e` — full suite, no new failures

## Inputs

- `compiler/meshc/tests/e2e.rs` — existing 323-test e2e harness with `compile_and_run` and `compile_multifile_and_run` helpers

## Expected Output

- `compiler/meshc/tests/e2e.rs` — 5–6 new `#[test]` functions appended, covering all 3 R025 gap categories
