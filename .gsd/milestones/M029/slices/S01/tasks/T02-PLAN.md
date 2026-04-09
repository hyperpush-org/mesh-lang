---
estimated_steps: 4
estimated_files: 3
skills_used:
  - test
  - review
---

# T02: Add truthful library and CLI regressions for dotted imports

**Slice:** S01 — Formatter dot-path and multiline import fix
**Milestone:** M029

## Description

Close the semantic proof gap above the walker unit tests. This task makes the formatter regressions fail on the old dotted-path corruption even when `fmt --check` would otherwise pass on already-normalized output.

## Steps

1. Extend `compiler/mesh-fmt/src/lib.rs` idempotence and snapshot coverage to dotted from-import text so canonical dotted paths are asserted at the library layer too.
2. Add CLI integration tests in `compiler/meshc/tests/e2e_fmt.rs` that write temp files with dotted single-line imports, parenthesized multiline imports, and qualified impl headers, run `meshc fmt`, and assert the exact output text.
3. Re-run the targeted multiline-import compiler proof in `compiler/meshc/tests/e2e.rs` so the formatter change does not regress the existing parenthesized import contract.
4. Keep the assertions semantic: they must fail on `Api. Router` / `Foo. Bar`, not just on `fmt --check` exit codes.

## Must-Haves

- [ ] `compiler/mesh-fmt/src/lib.rs` asserts canonical dotted import text at the library layer.
- [ ] `compiler/meshc/tests/e2e_fmt.rs` would fail on `Api. Router` / `Foo. Bar` output.
- [ ] Parenthesized multiline import preservation remains proven outside the walker-local tests.

## Verification

- `cargo test -q -p meshc --test e2e_fmt -- --nocapture`
- `cargo test -q -p meshc --test e2e e2e_multiline_import_paren -- --nocapture`

## Inputs

- `compiler/mesh-fmt/src/walker.rs` — `PATH` formatting change from T01
- `compiler/mesh-fmt/src/lib.rs` — current snapshot/idempotence test surface
- `compiler/meshc/tests/e2e_fmt.rs` — CLI formatter integration tests
- `compiler/meshc/tests/e2e.rs` — targeted multiline-import compiler e2e guardrail

## Expected Output

- `compiler/mesh-fmt/src/lib.rs` — dotted-path idempotence and snapshot regression coverage
- `compiler/meshc/tests/e2e_fmt.rs` — CLI exact-output regression coverage for dotted paths and parenthesized multiline imports

## Observability Impact

- The cheapest new inspection surface is a targeted CLI exact-output test in `compiler/meshc/tests/e2e_fmt.rs`; when dotted-path formatting regresses, it must show the actual rewritten file text so `Api. Router` / `Foo. Bar` corruption is visible without relying on `fmt --check` exit codes alone.
- Library-layer coverage in `compiler/mesh-fmt/src/lib.rs` now asserts canonical dotted import text directly, so future regressions can be isolated between walker/lib formatting and `meshc fmt` CLI plumbing instead of hiding behind idempotent but semantically wrong output.
- The existing compiler proof `cargo test -q -p meshc --test e2e e2e_multiline_import_paren -- --nocapture` remains the cross-check that parser/compiler support for parenthesized multiline imports still holds after formatter-focused changes.
