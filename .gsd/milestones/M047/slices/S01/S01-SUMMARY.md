---
id: S01
parent: M047
milestone: M047
provides:
  - Source-first `@cluster` / `@cluster(N)` parser and AST support for ordinary clustered functions.
  - Shared mesh-pkg clustered declaration metadata with default-versus-explicit replication counts and source provenance.
  - Source-ranged meshc clustered validation diagnostics and source-only compiler coverage for decorated clustered functions.
  - Range-accurate mesh-lsp clustered diagnostics anchored on decorated source declarations.
requires:
  []
affects:
  - S02
  - S03
  - S04
  - S06
key_files:
  - compiler/mesh-common/src/token.rs
  - compiler/mesh-lexer/src/lib.rs
  - compiler/mesh-parser/src/parser/mod.rs
  - compiler/mesh-parser/src/parser/items.rs
  - compiler/mesh-parser/src/ast/item.rs
  - compiler/mesh-parser/src/syntax_kind.rs
  - compiler/mesh-parser/tests/parser_tests.rs
  - compiler/mesh-pkg/src/manifest.rs
  - compiler/mesh-pkg/src/lib.rs
  - compiler/meshc/src/main.rs
  - compiler/meshc/tests/e2e_m047_s01.rs
  - compiler/mesh-lsp/src/analysis.rs
  - .gsd/PROJECT.md
  - .gsd/DECISIONS.md
key_decisions:
  - D268: use `@cluster` / `@cluster(N)` as the S01 proof surface, keep `clustered(work)` parser-compatible until S04, and force both spellings through one mesh-pkg declaration/export seam carrying provenance and replication counts.
  - D269: keep distinct CST node kinds for decorator and legacy compatibility syntax, but expose a shared AST-level `ClusteredDecl` wrapper on `FnDef`.
  - D270: store resolved replication-count plus declaration origin/provenance in mesh-pkg and expose one shared `build_clustered_export_surface(...)` helper for compiler and LSP consumers.
  - D271: keep `replication_count` and `origin` on meshc metadata while leaving the codegen-facing declared-handler plan limited to runtime registration names and executable symbols.
  - D272: in mesh-lsp, emit source-origin clustered validation issues only on the currently analyzed relative file and convert the stored declaration byte span directly into the LSP range.
patterns_established:
  - Use a dedicated source-first syntax path first instead of inventing a generic decorator framework when there is only one real decorator user.
  - Keep compatibility syntax and source-first syntax as separate CST truths, but give downstream compiler/editor consumers one semantic AST wrapper so they never need token peeking.
  - Centralize clustered declaration provenance, replication counts, and export-surface construction in mesh-pkg so meshc and mesh-lsp cannot drift.
  - For editor diagnostics, reuse stored source provenance directly and gate source-origin issues to the active file before converting spans into LSP ranges.
observability_surfaces:
  - meshc clustered validation diagnostics now emit real source file and source span information for source-origin declaration failures in both human and JSON modes.
  - mesh-lsp clustered diagnostics now land on the decorated declaration range for duplicate/private source-origin failures instead of collapsing to project-level `(0,0)` errors.
  - The M047 slice rails (`mesh-parser`, `mesh-pkg`, `meshc --test e2e_m047_s01`, and `mesh-lsp`) are the authoritative diagnostic/proof surfaces for this source-first declaration reset.
drill_down_paths:
  - .gsd/milestones/M047/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M047/slices/S01/tasks/T02-SUMMARY.md
  - .gsd/milestones/M047/slices/S01/tasks/T03-SUMMARY.md
  - .gsd/milestones/M047/slices/S01/tasks/T04-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-01T06:06:56.808Z
blocker_discovered: false
---

# S01: Source decorator reset for clustered functions

**`@cluster` and `@cluster(N)` now compile on ordinary functions and carry shared replication-count plus source-provenance metadata through mesh-pkg, meshc, and mesh-lsp while legacy `clustered(work)` stays compatibility-only until the later cutover.**

## What Happened

S01 reset the clustered-function authoring seam from a manifest/`clustered(work)`-shaped model toward the source-first model M047 needs, without pretending the repo-wide cutover is done already. The parser and AST now recognize `@cluster` and `@cluster(N)` on ordinary `fn` / `def` items via a dedicated decorator path, while keeping `clustered(work)` green as a temporary compatibility bridge. Downstream consumers do not need to peek at raw tokens anymore: `FnDef` exposes a shared clustered declaration wrapper that carries syntax style, semantic kind, optional explicit replication count, and declaration span.

The slice also moved clustered declaration truth into one shared mesh-pkg seam. Source collection no longer collapses declarations down to a target string; it records qualified target, replication count with default-versus-explicit origin, source syntax, module/file provenance, and declaration span. Validation threads that richer origin/count information through `ClusteredExecutionMetadata` and `ClusteredDeclarationError`, and one shared `build_clustered_export_surface(...)` helper now resolves public work functions and service-generated handlers for both meshc and mesh-lsp.

meshc now plans clustered builds from that shared seam instead of a local export-surface builder. A source-only package using bare `@cluster` and explicit `@cluster(3)` builds without `[cluster]`, the prepared build keeps `replication_count` and `origin` metadata without changing runtime registration behavior, and clustered validation failures now point back to the decorated source declaration in both human and JSON output instead of falling back to empty-file/project-level diagnostics. mesh-lsp finishes the same reset on the editor path: source-origin clustered issues are filtered to the analyzed file and converted from stored declaration byte spans into real LSP ranges, so valid source-only `@cluster` code stays clean and duplicate/private failures land on the decorated declaration line instead of `(0,0)`.

The slice deliberately stops short of runtime replication-count semantics and hard cutover. `@cluster(3)` is now truthful in parser/pkg/compiler/LSP metadata, but runtime submit semantics still belong to S02. Likewise, `clustered(work)` and manifest declarations remain compatibility-only public baggage until S04 removes them from examples, scaffolds, and docs.

## Verification

Fresh closeout replay passed all four slice rails: `cargo test -p mesh-parser m047_s01 -- --nocapture` (14 passed), `cargo test -p mesh-pkg m047_s01 -- --nocapture` (7 passed), `cargo test -p meshc --test e2e_m047_s01 -- --nocapture` (4 passed), and `cargo test -p mesh-lsp m047_s01 -- --nocapture` (3 passed). Together those rails prove parser/AST support for bare and counted decorators plus malformed-shape failures, mesh-pkg default-count/provenance/export-surface truth, meshc source-only builds and source-ranged diagnostics, and mesh-lsp range-accurate clustered diagnostics on decorated declarations.

## Requirements Advanced

- R097 — S01 makes `@cluster` / `@cluster(N)` the real source-first clustered function syntax in parser, compiler, and editor flows for ordinary functions, while keeping the old spelling only as a temporary compatibility bridge.
- R098 — S01 defines replication counts as explicit clustered declaration metadata with default `2` versus explicit `N` preserved through mesh-pkg, meshc, and mesh-lsp, setting up S02’s runtime semantics work.
- R099 — S01 proves the new clustered syntax first on ordinary non-HTTP functions, keeping clustering a general function capability instead of a route-only feature.
- R106 — S01 improves the source-first teaching surface inside the toolchain itself by making compiler and LSP diagnostics point at decorated declarations with real count/source context instead of manifest-shaped fallback errors.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None. The completed task outputs already matched the slice plan; closeout work only reran the full verification stack, recorded the missing LSP diagnostic decision, and compressed the results into the slice artifacts.

## Known Limitations

`clustered(work)` and manifest declarations still remain parser-compatible and visible as compatibility paths; S01 does not hard-cut them from the public model yet. `@cluster(N)` counts are metadata-only in this slice: parser, mesh-pkg, meshc, and mesh-lsp now preserve and report replication counts with default `2`, but runtime continuity submit semantics still reject replica counts above `1` until S02 lands. There is also one parser-recovery rough edge left: malformed `@cluster(1, 2)` still produces the primary decorator-shape error plus a follow-on recovery error even though it fails closed.

## Follow-ups

S02 should start from the existing `ClusteredExecutionMetadata.replication_count` / `PreparedBuild` metadata and connect it to runtime continuity semantics instead of reopening parser or mesh-pkg seams. S03 should lower `HTTP.clustered(...)` onto the same shared declaration/export-surface model rather than inventing a route-only clustered path. S04 should remove the legacy `clustered(work)` and manifest clustering surfaces from examples, scaffolds, docs, and proof rails now that S01 established the new source-first compiler/editor truth.

## Files Created/Modified

- `compiler/mesh-common/src/token.rs` — Added the shared `@` token vocabulary entry needed for source-first clustered decorators.
- `compiler/mesh-lexer/src/lib.rs` — Lexes `@cluster` / `@cluster(N)` instead of treating `@` as an error token.
- `compiler/mesh-parser/src/parser/items.rs` — Parses dedicated clustered decorators ahead of ordinary function items while preserving legacy `clustered(work)` compatibility.
- `compiler/mesh-parser/src/ast/item.rs` — Exposes the shared `ClusteredDecl` AST wrapper with syntax kind, optional count, and declaration span.
- `compiler/mesh-parser/tests/parser_tests.rs` — Adds the M047 parser rail for bare/count decorators, malformed shapes, and legacy compatibility snapshots.
- `compiler/mesh-pkg/src/manifest.rs` — Introduces source-cluster declaration provenance/count records, validation metadata, and the shared clustered export-surface helper.
- `compiler/mesh-pkg/src/lib.rs` — Re-exports the shared clustered declaration/export APIs for compiler and LSP consumers.
- `compiler/meshc/src/main.rs` — Switches compiler planning and clustered diagnostics onto the shared mesh-pkg declaration/export seam.
- `compiler/meshc/tests/e2e_m047_s01.rs` — Adds compiler e2e coverage for source-only `@cluster`, explicit counts, source-ranged failures, and stable registration markers.
- `compiler/mesh-lsp/src/analysis.rs` — Anchors source-origin clustered diagnostics on the decorated declaration range and removes the local export-surface duplication.
- `.gsd/PROJECT.md` — Refreshes project state to reflect that M047/S01 landed source-first parser/compiler/LSP truth while later slices still own runtime semantics and hard cutover.
- `.gsd/DECISIONS.md` — Records the missing mesh-lsp diagnostic anchoring decision for the source-first clustered declaration model.
