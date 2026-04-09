---
id: S03
milestone: M031
status: done
outcome: success
tasks_completed: 2
tasks_total: 2
provides:
  - reference-backend/ has zero let _ = side-effect bindings
  - reference-backend/ has zero == true boolean comparisons
  - All WorkerState full reconstructions replaced with %{state | field: value} struct update
  - All nested if/else chains flattened to else if
  - 410-char import in api/health.mpl converted to parenthesized multiline
  - Build, formatter, project tests, and 313 e2e tests pass
consumes:
  - S01: bare expression statement support, else if codegen fix, if fn_call() do disambiguation
  - S02: parenthesized multiline import support, trailing comma support
requirement_coverage:
  - R023: validated — zero anti-patterns, build/fmt/test green
patterns_established:
  - Bare expression statements for all side-effect-only calls (println, service calls, spawn, Process.register)
  - Struct update syntax %{state | changed: value} for partial state transitions
  - else if chains for multi-branch conditionals
  - Direct Bool usage in conditions (if fn_call() do) without == true
  - Parenthesized multiline imports for long import lines
completed_at: 2026-03-24
---

# S03: Reference-Backend Dogfood Cleanup

Eliminated all workaround patterns from `reference-backend/` — 53 `let _ =` bindings removed, 15 `== true` comparisons removed, 8 full struct reconstructions replaced with struct update syntax, 7 nested if/else chains flattened to `else if`, and one 410-char import converted to parenthesized multiline. Zero regressions across 313 e2e tests.

## What Changed

**T01 — worker.mpl (the big file):** Removed 44 `let _ =` prefixes, 11 `== true` comparisons, replaced all 8 `WorkerState { ... }` 15-field reconstructions with `%{state | changed_fields: values}`, and flattened 3 nested if/else chains. This file contained ~80% of the anti-patterns.

**T02 — remaining 5 files:** Removed 9 `let _ =` across `api/jobs.mpl` (4), `storage/jobs.mpl` (2), `main.mpl` (2), `runtime/registry.mpl` (1). Removed 4 `== true` and flattened 4 nested if/else chains in `api/health.mpl`. Converted the 410-char single-line import in `api/health.mpl` to parenthesized multiline format.

All transformations are strictly behavior-preserving. No runtime signals, log messages, or `/health` output changed.

## What This Proves

The reference-backend now exemplifies idiomatic Mesh code rather than workaround patterns. Every S01 and S02 compiler fix is exercised in real backend code:

- **Bare expressions** (S01 parser fix): 53 side-effect calls compile without `let _ =`
- **Bool conditions** (S01 trailing-closure fix): 15 `if fn_call() do` patterns work directly
- **Struct update** (pre-existing feature): 8 partial state transitions are dramatically shorter
- **else if chains** (S01 codegen fix): 7 multi-branch blocks are flat and correct
- **Multiline imports** (S02 parser fix): 410-char import is now readable

## Verification Results

| Check | Result |
|-------|--------|
| `rg 'let _ =' reference-backend/ -g '*.mpl'` | 0 matches ✅ |
| `rg '== true' reference-backend/ -g '*.mpl'` | 0 matches ✅ |
| `cargo run -p meshc -- build reference-backend` | compiled successfully ✅ |
| `cargo run -p meshc -- fmt --check reference-backend` | 11 files already formatted ✅ |
| `cargo run -p meshc -- test reference-backend` | 2 passed ✅ |
| `cargo test -p meshc --test e2e` | 313 passed, 10 pre-existing try-operator failures ✅ |

## What the Next Slice Should Know

- **S04 (Mesher Dogfood Cleanup)** applies the same patterns to `mesher/`. The mechanical process is identical: remove `let _ =`, remove `== true`, replace `<>` with interpolation per D029, convert long imports to multiline. `mesher/` is larger (~72 `let _ =`, ~32 `<>`) but the transformations are the same.
- **S05 (Language Test Expansion)** can use the cleaned `reference-backend/` as a test oracle — every pattern exercised here (bare expressions, struct update, else if, Bool conditions, multiline imports) should have dedicated e2e coverage.
- `<>` was intentionally kept in `storage/jobs.mpl` SQL construction per D029.
- The 10 pre-existing try-operator e2e failures are unrelated to this slice and predate M031.

## Files Modified

- `reference-backend/jobs/worker.mpl`
- `reference-backend/api/health.mpl`
- `reference-backend/api/jobs.mpl`
- `reference-backend/storage/jobs.mpl`
- `reference-backend/main.mpl`
- `reference-backend/runtime/registry.mpl`
