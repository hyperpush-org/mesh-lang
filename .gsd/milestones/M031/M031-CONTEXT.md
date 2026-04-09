# M031: Language DX Audit & Rough Edge Fixes

**Gathered:** 2026-03-24
**Status:** Ready for planning

## Project Description

Mesh is a compiled programming language targeting backend applications. The compiler is a Rust workspace under `compiler/` with crates for lexing, parsing, type checking, LLVM codegen, runtime, formatter, LSP, REPL, and CLI. Two dogfood codebases exist: `reference-backend/` (API + DB + jobs) and `mesher/` (error monitoring platform). Both expose DX friction patterns that trace back to language-level rough edges.

## Why This Milestone

M028 proved the backend trust baseline. Now both dogfood codebases are littered with workaround patterns forced by language bugs and missing ergonomics. The `reference-backend/` worker has 60+ `let _ =` bindings, 15 `== true` comparisons, and manually reconstructs an 18-field struct on every state transition. `mesher/` has 72 `let _ =` bindings, 32 `<>` string concatenations where interpolation would work, and 310-character import lines because multiline imports aren't supported. These patterns make Mesh code look worse than it needs to and hide real bugs behind workarounds.

## User-Visible Outcome

### When this milestone is complete, the user can:

- Write `if is_valid(x) do ... end` without workarounds — no temp variables, no parens, no `== true`
- Write `else if` chains that produce correct values
- Split long import lines across multiple lines with parenthesized groups
- Write multiline function calls with args on separate lines
- Read both dogfood codebases and see idiomatic Mesh code instead of ceremony-heavy workarounds

### Entry point / environment

- Entry point: `cargo run -p meshc -- build reference-backend` and `cargo run -p meshc -- build mesher`
- Environment: local dev
- Live dependencies involved: Postgres for reference-backend e2e tests

## Completion Class

- Contract complete means: all fixed patterns compile, run, and produce correct values in e2e tests
- Integration complete means: both dogfood codebases build and pass existing tests with idiomatic code
- Operational complete means: none

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- `cargo run -p meshc -- build reference-backend` succeeds with zero `let _ =`, zero `== true`, struct update syntax throughout
- `cargo run -p meshc -- build mesher` succeeds with zero `let _ =`, interpolation replacing unnecessary `<>`, multiline imports for long lines
- `cargo test -p meshc --test e2e` passes with all existing tests plus new coverage for fixed patterns
- `cargo run -p meshc -- fmt --check reference-backend` passes
- `cargo run -p meshc -- test reference-backend` passes

## Risks and Unknowns

- **Trailing-closure disambiguation is the hardest parser change** — `if fn_call() do` must stop being parsed as a trailing closure while `test("name") do ... end` must keep working. The parser currently uses a simple heuristic: `do` after `)` = trailing closure. The fix must distinguish control-flow context from expression context.
- **`else if` codegen bug may be in type resolution** — The MIR lowering recurses correctly but `self.resolve_range(if_.syntax().text_range())` may return the wrong type for chained if-expressions, causing LLVM to use wrong phi node types. String values crash with misaligned pointer dereference; integers return garbage.
- **Multiline fn call typechecker bug** — The parser produces correct trees for multiline calls (formatter round-trips them), but the typechecker resolves them as `()`. The bug is in span-based type resolution, not parsing.
- **Dogfood cleanup volume** — 157 `let _ =` across reference-backend, 72 in mesher. Mechanical but high-count; needs care to not break existing behavior.

## Existing Codebase / Prior Art

- `compiler/mesh-parser/src/parser/expressions.rs` — Pratt parser with trailing-closure heuristic at line ~114 (`if p.at(SyntaxKind::DO_KW) { parse_trailing_closure(p); }`)
- `compiler/mesh-parser/src/parser/items.rs:178` — `parse_from_import_decl` breaks on NEWLINE after comma (line ~212)
- `compiler/mesh-codegen/src/mir/lower.rs:8906` — `lower_if_expr` handles `else if` chains by recursion into `lower_if_expr`; type comes from `resolve_range`
- `compiler/mesh-typeck/` — Type checker that uses span-based resolution; multiline fn calls resolve to `()` instead of the return type
- `reference-backend/jobs/worker.mpl` — Primary dogfood target: 18-field `WorkerState` struct, 60+ `let _ =`, 15 `== true`
- `mesher/ingestion/pipeline.mpl` — `PipelineRegistry` service with full struct reconstruction pattern
- `mesher/api/search.mpl` — Manual JSON construction with `<>` that could use `json {}` or interpolation
- `mesher/storage/schema.mpl` — SQL construction with `<>` chains
- `mesher/main.mpl:13-19` — 150-310 character import lines

> See `.gsd/DECISIONS.md` for all architectural and pattern decisions — it is an append-only register; read it during planning, append to it during execution.

## Relevant Requirements

- R015 — `else if` codegen correctness
- R016 — Trailing-closure disambiguation in control flow
- R017 — Multiline function call typechecking
- R018 — Multiline/parenthesized imports
- R019 — Trailing commas in function args
- R023 — Reference-backend idiomatic dogfood cleanup
- R024 — Mesher idiomatic dogfood cleanup
- R025 — E2e test coverage for fixed language patterns
- R011 — New language work should come from real backend friction (this milestone is exactly that)
- R013 — Blocking language limitations should be fixed in Mesh, not worked around

## Scope

### In Scope

- Parser fix: trailing-closure disambiguation for `if`, `while`, `case`, `for` conditions ending in fn calls
- Codegen fix: `else if` chains producing correct values for all types
- Typechecker fix: multiline function calls resolving correct return types
- Parser enhancement: parenthesized multiline imports `from X import (\n  a,\n  b\n)`
- Parser enhancement: trailing commas in function call arguments
- Dogfood cleanup: remove all `let _ =`, `== true`, use struct update, use interpolation, use pipes
- E2e test expansion for all fixed patterns

### Out of Scope / Non-Goals

- New language features (new keywords, new control flow forms, new type system features)
- Formatter/LSP changes beyond what's needed to support the parser fixes
- Changes to the runtime (`mesh-rt`) beyond what codegen fixes require
- Performance optimization
- New stdlib functions

## Technical Constraints

- The parser uses matklad's event-based approach (rowan green trees). Changes must preserve the CST structure.
- Trailing closures (`describe("...") do ... end`, `test("...") do ... end`) are used by the test framework and must continue working.
- The `<>` operator is still needed for cases where interpolation doesn't work (raw SQL construction with complex expressions, embedding raw JSONB). Only replace where interpolation is clearly better.
- Some `let _ =` patterns may be intentional (binding to suppress unused-value warnings from the typechecker). Verify before removing.

## Integration Points

- `compiler/mesh-parser` — Parser changes for trailing-closure disambiguation, multiline imports, trailing commas
- `compiler/mesh-codegen` — MIR lowering fix for `else if` chains
- `compiler/mesh-typeck` — Span resolution fix for multiline function calls
- `compiler/mesh-fmt` — Formatter must handle new multiline import syntax
- `reference-backend/` — Primary dogfood cleanup target
- `mesher/` — Secondary dogfood cleanup target
- `tests/e2e/` — New test files for fixed patterns

## Open Questions

- Whether the trailing-closure disambiguation should use parser context (we're inside an `if` condition) or a grammar-level rule (e.g., require `do` on same line as call for trailing closures, or require trailing closures to use explicit `do |args|` form). Current thinking: context-based — suppress trailing-closure parsing when inside a control-flow condition position.
- Whether the multiline fn call bug is in the typechecker's span resolution or in the parser's CST span boundaries. The formatter round-trips the code correctly, suggesting the parser is fine and the typechecker is resolving spans wrong.
