# S05: Language Test Expansion — Research

## Summary

S05 adds e2e tests for the remaining R025 gaps — patterns fixed in S01–S04 that lack dedicated test coverage. The test infrastructure is well-established (`compile_and_run` and `compile_multifile_and_run` helpers in `compiler/meshc/tests/e2e.rs`). This is mechanical work: write `.mpl` test programs, add `#[test]` harness functions, verify they pass.

## Requirement Mapping

**R025 (active, owner: M031/S05):** New e2e tests must cover bare expression statements, `else if` chains, `if fn_call() do`, `while fn_call() do`, `case fn_call() do`, `for x in fn_call() do`, `not fn_call()` in conditions, multiline fn calls, multiline imports, trailing commas, struct update in service handlers, pipe chains.

### Coverage Audit

| Pattern | Existing Tests | Gap |
|---------|---------------|-----|
| bare expression statements | None dedicated — dogfood code proves it works but no isolated e2e | **Need 1–2 tests** |
| `else if` chains (Int/String/Bool) | 5 tests (S01) | ✅ Covered |
| `if fn_call() do` | 1 test (S01) | ✅ Covered |
| `while fn_call() do` | 1 test (S01) | ✅ Covered |
| `case fn_call() do` | 1 test (S01) | ✅ Covered |
| `for x in fn_call() do` | 1 test (S01) | ✅ Covered |
| `not fn_call()` in conditions | Only `not f` with variable in `comprehensive.mpl` | **Need 1–2 tests** |
| multiline fn calls | 5 tests (S01) | ✅ Covered |
| multiline imports | 3 tests (S02) | ✅ Covered |
| trailing commas | 2 tests (S02) | ✅ Covered |
| struct update in service handlers | 3 struct update tests (plain structs), 0 in service context | **Need 1–2 tests** |
| pipe chains | 24+ tests across pipe/slot_pipe/query_builder | ✅ Covered |

**Net gap: 3 pattern categories need new tests.** Everything else was covered by S01/S02 or pre-existing tests.

## Recommendation

One task. Add 4–6 new e2e tests covering the three gaps:

1. **Bare expression statements** — a test with `println()`, service calls, and multi-expression blocks all without `let _ =`. Proves bare expressions compile and execute as side effects.
2. **`not fn_call()` in conditions** — `if not is_empty(list) do ... end` and `while not done() do ... end`. Exercises the trailing-closure disambiguation with the `not` unary operator preceding a function call in control-flow position.
3. **Struct update in service handlers** — a service with a multi-field state struct that uses `%{state | field: value}` in `call` and `cast` handlers, proving struct update works inside the `(next_state, value)` tuple return pattern.

## Implementation Landscape

### Test harness

- **File:** `compiler/meshc/tests/e2e.rs`
- **Single-file tests:** `compile_and_run(source: &str) -> String` — writes `main.mpl`, builds, runs, returns stdout
- **Multi-file tests:** `compile_multifile_and_run(files: &[(&str, &str)]) -> String` — writes multiple `.mpl` files, builds, runs
- **Pattern:** `#[test] fn e2e_<name>() { let output = compile_and_run(r#"..."#); assert_eq!(output, "expected\n"); }`
- **Convention:** Test functions are `fn e2e_<category>_<detail>()`. S01 used `trailing_closure_*`, `else_if_*`, `multiline_call_*`. S02 used `multiline_import_*`, `trailing_comma_*`.

### Naming for new tests

- `e2e_bare_expression_side_effects` — multiple bare calls (println, function returning unit)
- `e2e_bare_expression_in_block` — bare expressions inside `do ... end` blocks, if branches, etc.
- `e2e_not_fn_call_if_condition` — `if not fn_call() do`
- `e2e_not_fn_call_while_condition` — `while not fn_call() do`
- `e2e_struct_update_in_service_call` — service `call` handler using `%{state | field: new_value}`
- `e2e_struct_update_in_service_cast` — service `cast` handler using struct update (optional, may combine with above)

### Verification

- `cargo test -p meshc --test e2e bare_expression` — new bare expr tests
- `cargo test -p meshc --test e2e not_fn_call` — new not-in-condition tests
- `cargo test -p meshc --test e2e struct_update_in_service` — new service struct update tests
- `cargo test -p meshc --test e2e` — full suite (expect 319 + new tests passing, 10 pre-existing `try_*` failures)

### Current baseline

319 test functions in `e2e.rs`. 313 pass, 10 pre-existing `try_*`/`from_try_*` failures (runtime crashes, exit code None). The 10 failures are unrelated to M031 work.

## Constraints

- All new tests must use typed function signatures (untyped polymorphic functions still produce wrong runtime values — KNOWLEDGE.md).
- Tests should use `compile_and_run` for single-file patterns; `compile_multifile_and_run` is only needed if testing import patterns.
- No `.mpl` files are needed in `tests/e2e/` — the inline-source pattern in `e2e.rs` is the convention for these targeted tests.

## Skills Discovered

No new skills needed — this is Rust test code using established project patterns.
