---
id: T01
parent: S01
milestone: M047
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-common/src/token.rs", "compiler/mesh-lexer/src/lib.rs", "compiler/mesh-parser/src/syntax_kind.rs", "compiler/mesh-parser/src/parser/mod.rs", "compiler/mesh-parser/src/parser/items.rs", "compiler/mesh-parser/src/ast/item.rs", "compiler/mesh-parser/tests/parser_tests.rs", "compiler/mesh-parser/tests/snapshots/parser_tests__m047_s01_parser_cluster_decorator_fn_def.snap", "compiler/mesh-parser/tests/snapshots/parser_tests__m047_s01_parser_cluster_decorator_with_count_fn_def.snap", "compiler/mesh-parser/tests/snapshots/parser_tests__m047_s01_parser_cluster_decorator_invalid_count_shape.snap", "compiler/mesh-parser/tests/snapshots/parser_tests__m047_s01_parser_legacy_clustered_work_compat_snapshot.snap"]
key_decisions: ["Kept separate CST node kinds for source-first and legacy clustered declarations, but exposed a shared AST-level `ClusteredDecl` wrapper on `FnDef` so later compiler, mesh-pkg, and LSP work can consume semantic clustered metadata without token peeking."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran `cargo test -p mesh-parser m047_s01 -- --nocapture` and confirmed the M047 parser rail passed (14/14 tests). This covered the new decorator snapshots, AST accessors for bare and counted declarations, explicit negative cases, and the legacy compatibility path. The later slice-level `mesh-pkg`, `meshc`, and `mesh-lsp` rails were not run in this task and remain for downstream tasks in S01."
completed_at: 2026-04-01T05:29:12.363Z
blocker_discovered: false
---

# T01: Added parser and AST support for `@cluster` / `@cluster(N)` on ordinary functions while preserving `clustered(work)` compatibility.

> Added parser and AST support for `@cluster` / `@cluster(N)` on ordinary functions while preserving `clustered(work)` compatibility.

## What Happened
---
id: T01
parent: S01
milestone: M047
key_files:
  - compiler/mesh-common/src/token.rs
  - compiler/mesh-lexer/src/lib.rs
  - compiler/mesh-parser/src/syntax_kind.rs
  - compiler/mesh-parser/src/parser/mod.rs
  - compiler/mesh-parser/src/parser/items.rs
  - compiler/mesh-parser/src/ast/item.rs
  - compiler/mesh-parser/tests/parser_tests.rs
  - compiler/mesh-parser/tests/snapshots/parser_tests__m047_s01_parser_cluster_decorator_fn_def.snap
  - compiler/mesh-parser/tests/snapshots/parser_tests__m047_s01_parser_cluster_decorator_with_count_fn_def.snap
  - compiler/mesh-parser/tests/snapshots/parser_tests__m047_s01_parser_cluster_decorator_invalid_count_shape.snap
  - compiler/mesh-parser/tests/snapshots/parser_tests__m047_s01_parser_legacy_clustered_work_compat_snapshot.snap
key_decisions:
  - Kept separate CST node kinds for source-first and legacy clustered declarations, but exposed a shared AST-level `ClusteredDecl` wrapper on `FnDef` so later compiler, mesh-pkg, and LSP work can consume semantic clustered metadata without token peeking.
duration: ""
verification_result: passed
completed_at: 2026-04-01T05:29:12.366Z
blocker_discovered: false
---

# T01: Added parser and AST support for `@cluster` / `@cluster(N)` on ordinary functions while preserving `clustered(work)` compatibility.

**Added parser and AST support for `@cluster` / `@cluster(N)` on ordinary functions while preserving `clustered(work)` compatibility.**

## What Happened

Added `@` token support in the shared token vocabulary and lexer, routed source-first `@cluster` / `@cluster(N)` and legacy `clustered(work)` through the same function parser entrypoint, introduced a distinct `CLUSTER_DECORATOR_DECL` CST node for the new spelling, and added an AST-level `ClusteredDecl` wrapper on `FnDef` so downstream code can read syntax style, semantic kind, optional explicit replica count, and declaration span without reparsing raw tokens. Added M047 parser tests and snapshots for bare/count-decorated functions, malformed decorator shapes, stray `@`, missing `fn|def`, non-function attachment, missing `)`, and legacy compatibility.

## Verification

Ran `cargo test -p mesh-parser m047_s01 -- --nocapture` and confirmed the M047 parser rail passed (14/14 tests). This covered the new decorator snapshots, AST accessors for bare and counted declarations, explicit negative cases, and the legacy compatibility path. The later slice-level `mesh-pkg`, `meshc`, and `mesh-lsp` rails were not run in this task and remain for downstream tasks in S01.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-parser m047_s01 -- --nocapture` | 0 | ✅ pass | 350ms |


## Deviations

None.

## Known Issues

The malformed `@cluster(1, 2)` snapshot currently includes the primary decorator-shape error plus a follow-on `expected end` recovery error. The form still fails closed and the task rail passes, but parser recovery could be tightened in a later cleanup.

## Files Created/Modified

- `compiler/mesh-common/src/token.rs`
- `compiler/mesh-lexer/src/lib.rs`
- `compiler/mesh-parser/src/syntax_kind.rs`
- `compiler/mesh-parser/src/parser/mod.rs`
- `compiler/mesh-parser/src/parser/items.rs`
- `compiler/mesh-parser/src/ast/item.rs`
- `compiler/mesh-parser/tests/parser_tests.rs`
- `compiler/mesh-parser/tests/snapshots/parser_tests__m047_s01_parser_cluster_decorator_fn_def.snap`
- `compiler/mesh-parser/tests/snapshots/parser_tests__m047_s01_parser_cluster_decorator_with_count_fn_def.snap`
- `compiler/mesh-parser/tests/snapshots/parser_tests__m047_s01_parser_cluster_decorator_invalid_count_shape.snap`
- `compiler/mesh-parser/tests/snapshots/parser_tests__m047_s01_parser_legacy_clustered_work_compat_snapshot.snap`


## Deviations
None.

## Known Issues
The malformed `@cluster(1, 2)` snapshot currently includes the primary decorator-shape error plus a follow-on `expected end` recovery error. The form still fails closed and the task rail passes, but parser recovery could be tightened in a later cleanup.
