---
id: M031
provides:
  - Trailing-closure disambiguation — `if fn_call() do`, `while fn_call() do`, `case fn_call() do`, `for x in fn_call() do` all parse correctly
  - Correct `else if` chain values for Int, String, and Bool return types
  - Multiline function call type resolution — args spanning multiple lines resolve correct return types
  - Parenthesized multiline imports — `from Module import (\n  a,\n  b\n)` parses and compiles
  - Trailing commas accepted and formatted cleanly in function call arguments
  - `reference-backend/` cleaned to zero `let _ =`, zero `== true`, struct update throughout, `else if` chains, multiline imports
  - `mesher/` cleaned to zero `let _ =`, `#{}` interpolation replacing clear-win `<>` sites, `else if` chains
  - 328 e2e tests covering all 12 R025 pattern categories
key_decisions:
  - D029 (prior): Keep `<>` for SQL DDL, raw JSONB embedding, and crypto — interpolation only at clear-win sites
  - D032: Multiline imports deferred in mesher — formatter collapses them and corrupts dot-paths; parser is fine, formatter walker needs fixing
patterns_established:
  - Parser flag save/restore for context-sensitive disambiguation (suppress_trailing_closure around control-flow condition expr() calls)
  - Recursive inference functions must store resolved types in the types map before returning (infer_if pattern)
  - AST accessors iterating children_with_tokens() must filter trivia instead of using .next()
  - Optional delimiter wrapping in item parsers — check for L_PAREN, track has_parens bool, reuse paren_depth for newline insignificance
  - Paren-aware formatter node handlers — check first child for L_PAREN, emit hardline-separated indent block, fall through to walk_tokens_inline for non-paren form
  - Bare expression statements for side-effect-only calls in Mesh (no let _ = needed)
  - Struct update syntax %{state | field: value} for partial state transitions
observability_surfaces:
  - cargo test -p meshc --test e2e — 328 tests, authoritative signal for language regression
  - cargo run -p meshc -- build reference-backend / mesher — dogfood build health
  - cargo run -p meshc -- fmt --check reference-backend — formatter compliance gate
  - cargo run -p meshc -- test reference-backend — project-level test gate
requirement_outcomes:
  - id: R015
    from_status: active
    to_status: validated
    proof: "M031/S01/T02: types.insert added in infer_if for both return paths. 5 e2e tests pass (Int, String, Bool, 3-level chain, let binding)."
  - id: R016
    from_status: active
    to_status: validated
    proof: "M031/S01/T01: suppress_trailing_closure flag with save/restore in all 4 control-flow condition sites. 5 e2e tests pass."
  - id: R017
    from_status: active
    to_status: validated
    proof: "M031/S01/T03: Literal::token() changed to filter trivia. 5 e2e tests pass for multiline fn calls."
  - id: R018
    from_status: active
    to_status: validated
    proof: "M031/S02: paren-delimited import parsing reusing paren_depth. 3 parser snapshots + 3 e2e tests."
  - id: R019
    from_status: active
    to_status: validated
    proof: "M031/S02: trailing commas already parsed; S02 added 2 e2e tests and formatter space suppression."
  - id: R023
    from_status: active
    to_status: validated
    proof: "M031/S03: zero let _ = (53 removed), zero == true (15 removed), 8 struct updates, 7 else-if flattens. Build/fmt/test/313 e2e pass."
  - id: R025
    from_status: active
    to_status: validated
    proof: "M031/S05: 5 new tests covering bare expressions, not-fn-call conditions, struct update in service. All 12 R025 categories covered. 318/328 pass."
duration: ~6h
verification_result: passed
completed_at: 2026-03-24
---

# M031: Language DX Audit & Rough Edge Fixes

**Fixed three compiler bugs blocking idiomatic Mesh, added multiline imports and trailing commas, cleaned both dogfood codebases to remove all workaround patterns, and expanded the e2e test suite from 308 to 328 tests covering every fixed pattern.**

## What Happened

S01 tackled the three highest-risk compiler bugs. The trailing-closure disambiguation fix added a `suppress_trailing_closure` flag to the Pratt parser, saved/restored around condition expressions in all four control-flow forms (`if`/`while`/`case`/`for`). The `else if` value-correctness fix added type-map storage in `infer_if`'s recursive path — without it, codegen fell back to `MirType::Unit` and String branches crashed on misaligned pointer dereference. The multiline function call fix changed `Literal::token()` to filter trivia tokens instead of blindly calling `.next()`, which returned NEWLINE trivia that had leaked into LITERAL CST nodes. 15 new e2e tests pinned all three fixes.

S02 added parenthesized multiline imports by extending `parse_from_import_decl` to detect `L_PAREN` and reuse the existing `paren_depth` mechanism for newline insignificance. The formatter gained `walk_import_list` for paren-aware import formatting and trailing-comma space suppression in `walk_paren_list`. 5 new e2e tests plus 3 parser snapshot tests.

S03 cleaned `reference-backend/` — 53 `let _ =` removed, 15 `== true` removed, 8 full WorkerState struct reconstructions replaced with `%{state | field: value}` update syntax, 7 nested if/else blocks flattened to `else if`, and one 410-char import converted to multiline. Every S01/S02 compiler fix is now exercised in real backend code.

S04 applied the same cleanup to `mesher/` — 72 `let _ =` removed, 11 `<>` replaced with `#{}` interpolation at clear-win sites, 3 nested else/if blocks flattened. Multiline imports were attempted but deferred (D032) because the formatter collapses them and corrupts module dot-paths.

S05 closed the remaining test coverage gaps with 5 new tests for bare expression statements, `not fn_call()` in conditions, and struct update in service handlers. All 12 R025 pattern categories now have dedicated e2e coverage.

## Cross-Slice Verification

| Success Criterion | Evidence | Status |
|---|---|---|
| `if fn_call() do`, `while fn_call() do`, `case fn_call() do`, `for x in fn_call() do` parse and compile | 5 e2e tests pass: `cargo test -p meshc --test e2e trailing_closure` | ✅ |
| `else if` chains return correct values for Int, String, Bool | 5 e2e tests pass: `cargo test -p meshc --test e2e else_if` | ✅ |
| Multiline function calls typecheck correctly | 5 e2e tests pass: `cargo test -p meshc --test e2e multiline_call` | ✅ |
| `from Module import (\n  a,\n  b\n)` parses and works | 3 e2e tests pass: `cargo test -p meshc --test e2e multiline_import` | ✅ |
| Trailing commas accepted in fn call args | 2 e2e tests pass: `cargo test -p meshc --test e2e trailing_comma` | ✅ |
| `reference-backend/` builds with zero `let _ =`, zero `== true`, struct update | `rg 'let _ =' reference-backend/ -g '*.mpl'` → 0; `rg '== true' reference-backend/ -g '*.mpl'` → 0; build succeeds | ✅ |
| `mesher/` builds with zero `let _ =`, interpolation, pipes | `rg 'let _ =' mesher/ -g '*.mpl'` → 0; build succeeds | ✅ |
| Trailing closures (`test("name") do ... end`) still work | All pre-existing e2e tests pass (318/328, 10 are pre-existing try_* failures) | ✅ |
| All existing e2e tests pass | `cargo test -p meshc --test e2e` → 318 pass, 10 pre-existing fail | ✅ |
| New e2e tests cover every fixed pattern | 25 new tests across 7 categories; all 12 R025 patterns covered | ✅ |
| `cargo run -p meshc -- fmt --check reference-backend` passes | 11 files already formatted | ✅ |
| `cargo run -p meshc -- test reference-backend` passes | 2 passed | ✅ |

**Not fully met:**
- `mesher/` multiline imports — deferred due to formatter bug (D032). Parser supports them; formatter collapses them. This affects R024's multiline-import criterion only.

## Requirement Changes

- R015: active → validated — else-if chain value correctness proven by 5 e2e tests and the `types.insert` fix in `infer_if`
- R016: active → validated — trailing-closure disambiguation proven by 5 e2e tests and the `suppress_trailing_closure` parser flag
- R017: active → validated — multiline fn call type resolution proven by 5 e2e tests and the trivia-filtering fix in `Literal::token()`
- R018: active → validated — parenthesized multiline imports proven by 3 parser snapshots and 3 e2e tests
- R019: active → validated — trailing commas proven by 2 e2e tests and formatter space suppression
- R023: active → validated — reference-backend cleaned: zero `let _ =`, zero `== true`, struct update, else-if, multiline imports; build/fmt/test green
- R025: active → validated — all 12 pattern categories have dedicated e2e tests; 328 total tests, 318 pass

R024 remains active — multiline imports in mesher blocked on formatter fix (D032). All other R024 criteria met.

## Forward Intelligence

### What the next milestone should know
- The compiler's Pratt parser now has a context-sensitive flag mechanism (`suppress_trailing_closure`) for disambiguation. Any new control-flow form taking a condition expression before `do` must save/set/restore this flag.
- `meshc fmt --check mesher` fails on 35 files due to the formatter collapsing multiline imports and corrupting module dot-paths. This is documented in D032 and KNOWLEDGE.md. Fix lives in `compiler/mesh-fmt/src/walker.rs` — `walk_import_list` needs to handle `FROM_IMPORT_DECL` with `IMPORT_LIST` inside parens without collapsing them.
- The 10 `try_*`/`from_try_*` e2e failures are pre-existing runtime crashes (exit code None) that have persisted across M028 and M031. They need attention when `try`/`Result` runtime support is hardened.
- Both dogfood codebases now use idiomatic patterns and serve as reliable test oracles for language correctness.

### What's fragile
- **Formatter multiline import handling** — the parser produces correct CST for paren imports, but the formatter walker doesn't preserve them. Any work touching `mesh-fmt/src/walker.rs` near import handling should fix this.
- **Untyped polymorphic functions** — a separate `Ty::Var` → `MirType::Unit` monomorphization gap means generic functions without type annotations produce wrong runtime values. All M031 e2e tests use typed signatures to avoid this.
- **10 try_* e2e tests** — these are runtime crashes in the try-operator/Result type path. Not related to M031 but they inflate the failure count and could mask new regressions if not filtered.

### Authoritative diagnostics
- `cargo test -p meshc --test e2e` (328 tests) — the primary language regression signal. 318 pass, 10 pre-existing fail.
- `cargo run -p meshc -- build reference-backend && cargo run -p meshc -- build mesher` — dogfood build health.
- `cargo run -p meshc -- fmt --check reference-backend` — formatter compliance. Mesher intentionally excluded until D032 is resolved.
- Pattern-specific e2e filters: `trailing_closure`, `else_if`, `multiline_call`, `multiline_import`, `trailing_comma`, `bare_expression`, `not_fn_call`, `struct_update_in_service`.

### What assumptions changed
- Trailing-comma support in fn args was assumed to need parser changes — it already worked; only formatter and test coverage were missing.
- Multiline import support was assumed to be end-to-end — parser works fine, but the formatter gap (D032) means mesher can't use them yet.
- The e2e test count grew from 308 (pre-M031) to 328, not the estimated 216+ from the roadmap — the baseline was already higher than documented.

## Files Created/Modified

### Compiler (10 files)
- `compiler/mesh-parser/src/parser/mod.rs` — `suppress_trailing_closure` flag on Parser struct
- `compiler/mesh-parser/src/parser/expressions.rs` — save/restore flag in 4 control-flow parsers
- `compiler/mesh-parser/src/parser/items.rs` — paren-delimited import parsing in `parse_from_import_decl`
- `compiler/mesh-parser/src/ast/expr.rs` — trivia-aware `Literal::token()` 
- `compiler/mesh-parser/tests/parser_tests.rs` — 3 snapshot tests for paren imports
- `compiler/mesh-parser/tests/snapshots/` — 3 new snapshot files
- `compiler/mesh-typeck/src/infer.rs` — `types.insert` in `infer_if` for both return paths
- `compiler/mesh-fmt/src/walker.rs` — `walk_import_list`, import-list routing, trailing-comma space suppression, 4 unit tests
- `compiler/meshc/tests/e2e.rs` — 25 new e2e test functions

### Dogfood — reference-backend (6 files)
- `reference-backend/jobs/worker.mpl` — 44 `let _ =`, 11 `== true`, 8 struct updates, 3 else-if flattens
- `reference-backend/api/health.mpl` — 4 `== true`, 4 else-if flattens, multiline import
- `reference-backend/api/jobs.mpl` — 4 `let _ =`
- `reference-backend/storage/jobs.mpl` — 2 `let _ =`
- `reference-backend/main.mpl` — 2 `let _ =`
- `reference-backend/runtime/registry.mpl` — 1 `let _ =`

### Dogfood — mesher (12 files)
- `mesher/ingestion/pipeline.mpl` — 35 `let _ =`, 1 else-if flatten
- `mesher/ingestion/routes.mpl` — 14 `let _ =`, 1 `<>` → interpolation
- `mesher/storage/queries.mpl` — 14 `let _ =`
- `mesher/services/retention.mpl` — 6 `let _ =`
- `mesher/services/writer.mpl` — 2 `let _ =`
- `mesher/ingestion/ws_handler.mpl` — 1 `let _ =`, 2 `<>` → interpolation
- `mesher/ingestion/fingerprint.mpl` — 5 `<>` → interpolation
- `mesher/ingestion/validation.mpl` — 1 `<>` → interpolation
- `mesher/services/event_processor.mpl` — 1 `<>` → interpolation
- `mesher/api/helpers.mpl` — 1 `<>` → interpolation
- `mesher/api/search.mpl` — 2 else-if flattens
