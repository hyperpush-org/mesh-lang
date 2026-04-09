# M029: Mesher & Reference-Backend Dogfood Completion

**Gathered:** 2026-03-24
**Status:** Ready for planning

## Project Description

Mesh is a programming language and application platform with two dogfood codebases: `reference-backend/` (API + DB + jobs) and `mesher/` (error monitoring platform). M031 fixed three compiler bugs and cleaned both codebases of `let _ =` and `== true` patterns, but left significant DX cleanup undone: the formatter corrupts module dot-paths and collapses multiline imports (D032), mesher still has ~21 `<>` concatenation chains where `json {}` or `#{}` interpolation would be better, pipe operators aren't used idiomatically in several files, and multiline imports can't be applied until the formatter is fixed.

## Why This Milestone

M031 claimed mesher cleanup was done but the actual delivery was partial — `let _ =` removal was thorough, but interpolation was minimal (11 of 21+ sites), pipes weren't touched, and multiline imports were blocked by a formatter bug that also corrupted reference-backend's dot-paths. The user explicitly flagged this gap. This milestone finishes what M031 started.

## User-Visible Outcome

### When this milestone is complete, the user can:

- Run `meshc fmt --check mesher` and see 0 files needing reformatting (currently 35 fail)
- Run `meshc fmt --check reference-backend` and see correct `Api.Router` dot-paths (currently has `Api. Router` corruption)
- Read mesher source and see idiomatic Mesh: `json {}` macro for JSON serialization, `#{}` interpolation where appropriate, `|>` pipe chains instead of nested function calls, parenthesized multiline imports for readability

### Entry point / environment

- Entry point: `cargo run -p meshc -- fmt --check mesher`, `cargo run -p meshc -- build mesher`
- Environment: local dev
- Live dependencies involved: none (formatter and code cleanup work)

## Completion Class

- Contract complete means: `meshc fmt --check` passes on both codebases, both build clean, e2e tests don't regress
- Integration complete means: formatter changes produce correct output across all existing Mesh files
- Operational complete means: none

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- `meshc fmt --check reference-backend` passes with no spurious spaces in dot-paths
- `meshc fmt --check mesher` passes (0 files needing reformatting)
- `cargo run -p meshc -- build reference-backend` and `cargo run -p meshc -- build mesher` both succeed
- `cargo test -p meshc --test e2e` shows 318+ pass with no regressions from M031 baseline
- `rg '<>' mesher/ -g '*.mpl'` shows only D029-designated SQL DDL / raw JSONB sites
- `rg 'List\.map(' mesher/ -g '*.mpl'` shows zero wrapping-style calls (all converted to pipe)
- All mesher import lines >120 chars use parenthesized multiline form

## Risks and Unknowns

- **Formatter dot-path bug (high)** — The `walk_tokens_inline` function inserts spaces before IDENT tokens after DOT in PATH nodes. The fix looks straightforward (exclude IDENT-after-DOT from space insertion, or handle PATH nodes with a dedicated walker that suppresses spacing), but formatter changes can cascade — a fix for PATH spacing could affect FIELD_ACCESS, CALL_EXPR chains, or other dot-separated forms. Need careful regression testing.
- **Formatter multiline import preservation** — `walk_import_list` already has paren-aware logic from M031/S02, but `walk_from_import_decl` may not be delegating correctly, or the paren-aware path may not trigger for the right CST shapes. The D032 investigation noted the formatter "collapses them back to single-line and introduces spurious spaces in module dot-paths."
- **json macro limitations** — The `json {}` macro can't embed raw JSONB fields (they'd get double-quoted). Files like `detail.mpl` with raw `exception`, `stacktrace`, `breadcrumbs`, `tags`, `extra`, `user_context` fields must use interpolation instead. Need to confirm which serializers can use `json {}` and which need `#{}`.

## Existing Codebase / Prior Art

- `compiler/mesh-fmt/src/walker.rs` (2588 lines) — The formatter walker. Key functions: `walk_from_import_decl` (line 1381), `walk_import_list` (line 1333), `walk_tokens_inline` (line 2013), `needs_space_before` (line 2057), `add_token_with_context` (line 2106).
- `compiler/mesh-parser/src/syntax_kind.rs` — Defines `PATH`, `DOT`, `IDENT`, `FROM_IMPORT_DECL`, `IMPORT_LIST`, `IMPORT_KW` kinds.
- `mesher/api/alerts.mpl` — 3 `<>` chains (JSON serializers with raw JSONB `condition_json`, `action_json`, `condition_snapshot` fields)
- `mesher/api/detail.mpl` — 4 `<>` chains (JSON serializer with 6 raw JSONB fields: exception, stacktrace, breadcrumbs, tags, extra, user_context)
- `mesher/api/search.mpl` — 7 `<>` chains (mix of `json {}` already used and `<>` for raw JSONB tag embedding + pagination cursors)
- `mesher/storage/queries.mpl` — 4 `<>` sites (2 non-SQL: `"mshr_" <> Crypto.uuid4()`, `uuid1 <> uuid2`; 2 SQL-adjacent: `Query.select_raw`, `DROP TABLE`)
- `mesher/storage/schema.mpl` — 3 `<>` sites (all SQL DDL — D029 keep-as-is)
- `mesher/ingestion/routes.mpl` — 310-char import line, longest in codebase
- `mesher/main.mpl` — 14 imports, 5 over 120 chars

> See `.gsd/DECISIONS.md` for all architectural and pattern decisions — it is an append-only register; read it during planning, append to it during execution.

## Relevant Requirements

- R024 — Mesher idiomatic cleanup: json macro adoption, remaining `<>` → interpolation, pipe operators, multiline imports
- R026 — Formatter must preserve module dot-paths and multiline imports
- R027 — Reference-backend must have correct dot-paths after formatter fix

## Scope

### In Scope

- Fix `meshc fmt` dot-path spacing bug (IDENT-after-DOT gets spurious space in PATH nodes)
- Fix `meshc fmt` multiline import preservation (paren imports collapse to single line)
- Convert mesher `<>` JSON serialization to `json {}` macro where all fields are simple, `#{}` interpolation where raw JSONB prevents macro use
- Convert obvious non-SQL `<>` in queries.mpl to interpolation (`"mshr_" <> Crypto.uuid4()` → `"mshr_#{Crypto.uuid4()}"`)
- Convert `List.map(rows, fn...)` wrapping to `rows |> List.map(fn...)` pipe style
- Apply parenthesized multiline imports to all mesher files with >120 char import lines
- Fix reference-backend `Api. Router` style dot-path corruption
- Formatter regression tests for both fixes

### Out of Scope / Non-Goals

- New language features, keywords, or syntax forms (R034 still applies)
- Changes to the json macro's capabilities (e.g., raw JSONB embedding support)
- Touching `schema.mpl` SQL DDL `<>` chains (D029 designates these as keep-as-is)
- Broad formatter improvements beyond the dot-path and multiline import bugs

## Technical Constraints

- `needs_space_before` in `walker.rs` line 2057 already excludes `DOT` from space-before — the problem is IDENT after DOT still gets space-before because IDENT isn't excluded
- The fix must not break FIELD_ACCESS formatting (`foo.bar`), CALL_EXPR chains (`Foo.bar()`), or any other dot-separated syntax
- `walk_import_list` already has paren-detection and per-line emission logic from M031/S02 — the issue may be in how `walk_from_import_decl` routes to it
- The `json {}` macro auto-quotes string values and bare-embeds int values, but has no raw/unquoted mode for JSONB fields
- `cargo test -p mesh-fmt --lib` (119 tests) is the primary formatter regression gate

## Integration Points

- `compiler/mesh-fmt/` — formatter crate, the main code change target
- `mesher/` — 35 `.mpl` files, the main dogfood cleanup target
- `reference-backend/` — 11 `.mpl` files, dot-path correction target
- `compiler/meshc/tests/e2e.rs` — 328 e2e tests, regression gate

## Open Questions

- Whether `walk_tokens_inline` needs a look-behind check (was previous token DOT?) or whether PATH should get its own walker that joins children without spaces — need to check which approach is less fragile
- Whether `walk_from_import_decl` properly delegates IMPORT_LIST to `walk_import_list` or whether the node routing is broken at a different level
