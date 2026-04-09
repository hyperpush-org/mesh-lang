---
estimated_steps: 4
estimated_files: 7
skills_used: []
---

# T01: Parse `@cluster` and `@cluster(N)` on ordinary functions

**Slice:** S01 — Source decorator reset for clustered functions
**Milestone:** M047

## Description

Introduce the source-first clustered decorator syntax at the lexer/parser/AST boundary without breaking the temporary `clustered(work)` compatibility bridge. Keep this task narrowly focused on ordinary `fn` / `def` items and make the AST expose the decorator, optional explicit count, and declaration span cleanly enough that later tasks can consume them without re-reading raw tokens.

## Negative Tests

- **Malformed inputs**: `@cluster(`, `@cluster(foo)`, `@cluster(1, 2)`, and decorator text attached to non-function items fail with explicit parse errors.
- **Error paths**: Missing `fn|def` after the decorator, missing closing `)`, and stray `@` tokens do not get silently accepted as other syntax.
- **Boundary conditions**: Bare `@cluster` records no explicit count, `@cluster(3)` records exactly one explicit count token, and legacy `clustered(work)` parsing still stays green.

## Steps

1. Add lexer/token support for `@` and a parser path that recognizes `@cluster` / `@cluster(N)` before `fn` / `def` items.
2. Extend the clustered declaration CST/AST accessors so downstream code can read the decorator kind, optional count, and declaration span from a function item.
3. Keep the old `clustered(work)` path working as a compatibility-only branch instead of folding the old and new forms into ad hoc token peeking.
4. Add parser tests and snapshots for valid decorators, invalid decorator shapes, and the compatibility bridge.

## Must-Haves

- [ ] `@cluster` and `@cluster(N)` parse before ordinary `fn` / `def` items.
- [ ] AST accessors expose the declaration and optional explicit count without reparsing raw tokens downstream.
- [ ] Parser coverage proves malformed decorator shapes fail explicitly and `clustered(work)` still parses.

## Verification

- `cargo test -p mesh-parser m047_s01 -- --nocapture`
- Confirm the M047 parser cases assert on both new decorator shapes and the legacy compatibility path.

## Inputs

- `compiler/mesh-common/src/token.rs` — token vocabulary currently lacks an `@` token.
- `compiler/mesh-lexer/src/lib.rs` — lexer rules need the new decorator token path.
- `compiler/mesh-parser/src/parser/mod.rs` — item dispatch currently only knows the contextual `clustered(work)` prefix.
- `compiler/mesh-parser/src/parser/items.rs` — clustered declaration parsing and function-item entrypoint live here.
- `compiler/mesh-parser/src/ast/item.rs` — AST accessors need to expose decorator/count/span data.
- `compiler/mesh-parser/src/syntax_kind.rs` — the CST needs a stable node kind for the source declaration form.
- `compiler/mesh-parser/tests/parser_tests.rs` — parser regression coverage and snapshots currently cover only `clustered(work)`.

## Expected Output

- `compiler/mesh-common/src/token.rs` — token definitions include the decorator punctuation needed for `@cluster`.
- `compiler/mesh-lexer/src/lib.rs` — lexer output can produce the new decorator token stream.
- `compiler/mesh-parser/src/parser/mod.rs` — item parsing recognizes decorated functions as a first-class path.
- `compiler/mesh-parser/src/parser/items.rs` — clustered decorator parsing handles bare and counted forms plus the compatibility bridge.
- `compiler/mesh-parser/src/ast/item.rs` — AST accessors expose clustered declaration/count/span information for later tasks.
- `compiler/mesh-parser/src/syntax_kind.rs` — syntax kinds stay explicit enough for tests and downstream tooling.
- `compiler/mesh-parser/tests/parser_tests.rs` — M047 parser assertions cover success, failure, and compatibility cases.
