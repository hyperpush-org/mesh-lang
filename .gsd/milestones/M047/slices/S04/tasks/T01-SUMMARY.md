---
id: T01
parent: S04
milestone: M047
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-parser/src/parser/items.rs", "compiler/mesh-parser/src/ast/item.rs", "compiler/mesh-parser/tests/parser_tests.rs", "compiler/mesh-pkg/src/manifest.rs", "compiler/meshc/tests/e2e_m047_s01.rs", "compiler/mesh-lsp/src/analysis.rs"]
key_decisions: ["Kept legacy clustered AST/types compiled for compatibility, but cut live parser and manifest acceptance so only `@cluster` can produce supported clustered metadata.", "Preferred one migration-oriented legacy parser error over cascading follow-on diagnostics, then pinned that behavior in compiler and LSP rails."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the task-plan verification rails for parser, package, and compiler e2e: `cargo test -p mesh-parser m047_s04 -- --nocapture`, `cargo test -p mesh-pkg m047_s04 -- --nocapture`, and `cargo test -p meshc --test e2e_m047_s01 -- --nocapture`. Added `cargo test -p mesh-lsp m047_s04 -- --nocapture` because `compiler/mesh-lsp/src/analysis.rs` changed. All four commands passed."
completed_at: 2026-04-01T09:34:25.928Z
blocker_discovered: false
---

# T01: Hard-cut legacy clustered declarations so only `@cluster` stays supported in parser, manifest loading, compiler e2e, and LSP diagnostics.

> Hard-cut legacy clustered declarations so only `@cluster` stays supported in parser, manifest loading, compiler e2e, and LSP diagnostics.

## What Happened
---
id: T01
parent: S04
milestone: M047
key_files:
  - compiler/mesh-parser/src/parser/items.rs
  - compiler/mesh-parser/src/ast/item.rs
  - compiler/mesh-parser/tests/parser_tests.rs
  - compiler/mesh-pkg/src/manifest.rs
  - compiler/meshc/tests/e2e_m047_s01.rs
  - compiler/mesh-lsp/src/analysis.rs
key_decisions:
  - Kept legacy clustered AST/types compiled for compatibility, but cut live parser and manifest acceptance so only `@cluster` can produce supported clustered metadata.
  - Preferred one migration-oriented legacy parser error over cascading follow-on diagnostics, then pinned that behavior in compiler and LSP rails.
duration: ""
verification_result: passed
completed_at: 2026-04-01T09:34:25.929Z
blocker_discovered: false
---

# T01: Hard-cut legacy clustered declarations so only `@cluster` stays supported in parser, manifest loading, compiler e2e, and LSP diagnostics.

**Hard-cut legacy clustered declarations so only `@cluster` stays supported in parser, manifest loading, compiler e2e, and LSP diagnostics.**

## What Happened

Removed the live `clustered(work)` parser acceptance path and replaced it with one migration-oriented parser error that keeps the function parse recoverable without producing clustered metadata. Restricted the generic clustered AST accessor to the supported `@cluster` decorator path, rejected manifest `[cluster]` sections during manifest loading with explicit migration guidance, and updated parser/pkg/compiler/LSP regression rails to pin the new cutover behavior while keeping valid `@cluster` flows green.

## Verification

Ran the task-plan verification rails for parser, package, and compiler e2e: `cargo test -p mesh-parser m047_s04 -- --nocapture`, `cargo test -p mesh-pkg m047_s04 -- --nocapture`, and `cargo test -p meshc --test e2e_m047_s01 -- --nocapture`. Added `cargo test -p mesh-lsp m047_s04 -- --nocapture` because `compiler/mesh-lsp/src/analysis.rs` changed. All four commands passed.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-parser m047_s04 -- --nocapture` | 0 | ✅ pass | 1900ms |
| 2 | `cargo test -p mesh-pkg m047_s04 -- --nocapture` | 0 | ✅ pass | 2350ms |
| 3 | `cargo test -p meshc --test e2e_m047_s01 -- --nocapture` | 0 | ✅ pass | 6180ms |
| 4 | `cargo test -p mesh-lsp m047_s04 -- --nocapture` | 0 | ✅ pass | 2030ms |


## Deviations

Added one extra `mesh-lsp` filter run because this task changed `compiler/mesh-lsp/src/analysis.rs`; otherwise followed the task plan.

## Known Issues

`compiler/mesh-parser/src/parser/items.rs` still emits a dead-code warning for the now-unused `LegacyCompatValid` enum variant, and `compiler/mesh-lsp/src/analysis.rs` still has a few unused historical test helpers. The warnings do not affect the cutover behavior proved here.

## Files Created/Modified

- `compiler/mesh-parser/src/parser/items.rs`
- `compiler/mesh-parser/src/ast/item.rs`
- `compiler/mesh-parser/tests/parser_tests.rs`
- `compiler/mesh-pkg/src/manifest.rs`
- `compiler/meshc/tests/e2e_m047_s01.rs`
- `compiler/mesh-lsp/src/analysis.rs`


## Deviations
Added one extra `mesh-lsp` filter run because this task changed `compiler/mesh-lsp/src/analysis.rs`; otherwise followed the task plan.

## Known Issues
`compiler/mesh-parser/src/parser/items.rs` still emits a dead-code warning for the now-unused `LegacyCompatValid` enum variant, and `compiler/mesh-lsp/src/analysis.rs` still has a few unused historical test helpers. The warnings do not affect the cutover behavior proved here.
