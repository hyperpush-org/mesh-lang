# S05: Language Test Expansion

**Goal:** Cover the 3 remaining R025 test gaps — bare expression statements, `not fn_call()` in conditions, and struct update in service handlers — with dedicated e2e tests.
**Demo:** `cargo test -p meshc --test e2e bare_expression` + `not_fn_call` + `struct_update_in_service` all pass. Full suite stays green (323+ pass, 10 pre-existing `try_*` failures).

## Must-Haves

- At least 2 e2e tests for bare expression statements (side-effect calls without `let _ =`)
- At least 2 e2e tests for `not fn_call()` in control-flow conditions (`if not`, `while not`)
- At least 1 e2e test for struct update `%{state | field: value}` inside service `call`/`cast` handlers
- All new tests pass
- Full e2e suite regression-free (323+ pass, 10 pre-existing failures unchanged)

## Verification

- `cargo test -p meshc --test e2e bare_expression` — new bare expression tests pass
- `cargo test -p meshc --test e2e not_fn_call` — new not-in-condition tests pass
- `cargo test -p meshc --test e2e struct_update_in_service` — new service struct update tests pass
- `cargo test -p meshc --test e2e` — full suite: 328+ pass, 10 pre-existing `try_*` failures

## Tasks

- [x] **T01: Add e2e tests for bare expressions, not-fn-call conditions, and struct update in services** `est:30m`
  - Why: R025 requires coverage for these 3 pattern categories — they work in dogfood code but have no isolated regression tests
  - Files: `compiler/meshc/tests/e2e.rs`
  - Do: Append 5–6 new `#[test]` functions using `compile_and_run` (bare expression, not-fn-call) and `compile_multifile_and_run` (service struct update). Use typed function signatures per KNOWLEDGE.md. Follow existing naming convention `e2e_<category>_<detail>`.
  - Verify: `cargo test -p meshc --test e2e bare_expression && cargo test -p meshc --test e2e not_fn_call && cargo test -p meshc --test e2e struct_update_in_service && cargo test -p meshc --test e2e 2>&1 | tail -5`
  - Done when: all new tests pass and full suite shows 328+ pass with no new failures

## Files Likely Touched

- `compiler/meshc/tests/e2e.rs`

## Observability / Diagnostics

- **Runtime signals:** None — these are compile-time e2e tests that run built binaries and assert stdout. No runtime services, background processes, or persistent state.
- **Inspection surface:** `cargo test -p meshc --test e2e <test_name>` with `-- --nocapture` shows full compiler and binary output for any individual test.
- **Failure visibility:** Test failures print the full meshc build stderr (parse/type errors) or binary execution stderr, plus expected vs actual stdout. The test harness names map directly to the pattern categories: `bare_expression`, `not_fn_call`, `struct_update_in_service`.
- **Redaction:** No secrets or credentials involved.
