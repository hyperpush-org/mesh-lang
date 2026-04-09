# S01: Source decorator reset for clustered functions

**Goal:** Make `@cluster` / `@cluster(N)` the source-first declaration spellings for ordinary clustered functions, with parser/AST support, shared mesh-pkg validation metadata, and compiler/LSP diagnostics that carry replication counts and real source locations while the old `clustered(work)` syntax remains compatibility-only until the later cutover slice.
**Demo:** After this: After this: `@cluster` and `@cluster(3)` compile on ordinary functions and surface clustered metadata/counts through compiler and LSP diagnostics without relying on `clustered(work)` or `.toml`.

## Tasks
- [x] **T01: Added parser and AST support for `@cluster` / `@cluster(N)` on ordinary functions while preserving `clustered(work)` compatibility.** — Why: The slice cannot become source-first until the lexer/parser/AST accept `@cluster` and `@cluster(N)` on ordinary functions.
Files: `compiler/mesh-common/src/token.rs`, `compiler/mesh-lexer/src/lib.rs`, `compiler/mesh-parser/src/parser/mod.rs`, `compiler/mesh-parser/src/parser/items.rs`, `compiler/mesh-parser/src/ast/item.rs`, `compiler/mesh-parser/src/syntax_kind.rs`, `compiler/mesh-parser/tests/parser_tests.rs`
Do: Add an `@` token and a dedicated clustered decorator parser for `@cluster` / `@cluster(N)` ahead of `fn|def`, expose AST accessors for optional explicit counts and declaration spans, keep `clustered(work)` parsing green as a temporary compatibility path, and add parser coverage for valid decorators plus malformed/non-function uses.
Verify: `cargo test -p mesh-parser m047_s01 -- --nocapture`
Done when: parser tests prove `@cluster` / `@cluster(3)` produce stable AST/CST output, bad decorator shapes fail with explicit parse errors, and legacy `clustered(work)` cases still pass.
  - Estimate: 2h
  - Files: compiler/mesh-common/src/token.rs, compiler/mesh-lexer/src/lib.rs, compiler/mesh-parser/src/parser/mod.rs, compiler/mesh-parser/src/parser/items.rs, compiler/mesh-parser/src/ast/item.rs, compiler/mesh-parser/src/syntax_kind.rs, compiler/mesh-parser/tests/parser_tests.rs
  - Verify: cargo test -p mesh-parser m047_s01 -- --nocapture
- [x] **T02: Added mesh-pkg-owned clustered source records, replication-count/origin metadata, and a shared export-surface helper for work functions plus service-generated handlers.** — Why: meshc and mesh-lsp can only stay coherent if clustered source declarations, replication counts, and exported-target discovery are validated in one shared mesh-pkg seam.
Files: `compiler/mesh-pkg/src/manifest.rs`, `compiler/mesh-pkg/src/lib.rs`, `compiler/mesh-typeck/src/lib.rs`
Do: Replace the lossy source collector with a richer source declaration record that stores qualified target, declaration kind, default/explicit replication count, and declaration provenance; add count support to validated execution metadata; extract one shared clustered export-surface helper that understands ordinary work functions and service-generated handlers; and add mesh-pkg tests for source-only success, duplicate manifest/source declarations, private targets, and count preservation.
Verify: `cargo test -p mesh-pkg m047_s01 -- --nocapture`
Done when: validated metadata carries default `2` or explicit `N`, source-origin errors retain enough provenance to point back to the declaration, and one shared helper can build the clustered export surface for both compiler and LSP consumers.
  - Estimate: 2h
  - Files: compiler/mesh-pkg/src/manifest.rs, compiler/mesh-pkg/src/lib.rs, compiler/mesh-typeck/src/lib.rs
  - Verify: cargo test -p mesh-pkg m047_s01 -- --nocapture
- [x] **T03: Switched meshc to the shared clustered declaration seam and added an M047 compiler rail for source-only builds and source-ranged diagnostics.** — Why: The compiler story is only honest when a source-only `@cluster` package builds and clustered validation errors point at the actual declaration instead of an empty manifest fallback.
Files: `compiler/meshc/src/main.rs`, `compiler/meshc/tests/e2e_m047_s01.rs`, `compiler/mesh-codegen/src/codegen/mod.rs`
Do: Switch meshc build planning to the shared source declaration + export-surface helpers, propagate the new replication-count metadata through the prepared build plan without changing runtime semantics, update plain and JSON clustered diagnostics to use source file/range when the declaration came from code, and add an M047 e2e rail for source-only `@cluster`, `@cluster(3)`, private/duplicate failures, and stable runtime registration naming.
Verify: `cargo test -p meshc --test e2e_m047_s01 -- --nocapture`
Done when: a source-only package using `@cluster` builds, count metadata survives compiler planning, and duplicate/private declarations fail with explicit source-ranged diagnostics instead of empty-file JSON errors.
  - Estimate: 2h
  - Files: compiler/meshc/src/main.rs, compiler/meshc/tests/e2e_m047_s01.rs, compiler/mesh-codegen/src/codegen/mod.rs
  - Verify: cargo test -p meshc --test e2e_m047_s01 -- --nocapture
- [x] **T04: Anchored mesh-lsp clustered diagnostics on decorated source declarations and added the M047 editor rail for clean and range-accurate `@cluster` analysis.** — Why: The slice still fails R106 if the editor path keeps a duplicate export-surface builder or continues to pin clustered errors at `(0,0)`.
Files: `compiler/mesh-lsp/src/analysis.rs`, `compiler/mesh-pkg/src/manifest.rs`
Do: Reuse the shared clustered export-surface and source declaration helpers inside project analysis, translate clustered validation errors into range-based LSP diagnostics using the recorded declaration provenance, and add M047 LSP tests for source-only `@cluster` success plus duplicate/private diagnostics landing on the decorated function line.
Verify: `cargo test -p mesh-lsp m047_s01 -- --nocapture`
Done when: clustered LSP diagnostics point at the declaration range for source-origin issues, valid source-only `@cluster` code stays diagnostics-clean, and the analysis path no longer reimplements export-surface building.
  - Estimate: 90m
  - Files: compiler/mesh-lsp/src/analysis.rs, compiler/mesh-pkg/src/manifest.rs
  - Verify: cargo test -p mesh-lsp m047_s01 -- --nocapture
