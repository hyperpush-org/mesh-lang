---
id: S01
parent: M046
milestone: M046
provides:
  - A narrow source-level `clustered(work)` declaration form on `fn|def`.
  - A shared manifest+source clustered declaration planner that still emits the existing declared-handler runtime metadata.
  - Origin-aware compiler/LSP diagnostics and proof rails for private decorated targets, duplicate source+manifest declarations, and source-only declared-handler registration.
requires:
  []
affects:
  - S02
  - S03
  - S04
  - S05
key_files:
  - compiler/mesh-parser/src/parser/items.rs
  - compiler/mesh-parser/src/ast/item.rs
  - compiler/mesh-parser/src/syntax_kind.rs
  - compiler/mesh-pkg/src/manifest.rs
  - compiler/meshc/src/main.rs
  - compiler/mesh-lsp/src/analysis.rs
  - compiler/meshc/tests/e2e_m046_s01.rs
  - compiler/mesh-parser/tests/parser_tests.rs
  - .gsd/PROJECT.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Represent the narrow source marker as a dedicated `CLUSTERED_WORK_DECL` node attached to `FnDef` instead of widening Mesh into a generic annotation/decorator system.
  - Merge source `clustered(work)` declarations inside `mesh-pkg::manifest` so `meshc` and `mesh-lsp` share one duplicate policy, validation path, and clustered execution plan.
  - Keep the final green compiler happy-path proof on the known-good M044-shaped work fixture until the unrelated single-function LLVM verifier bug is fixed.
patterns_established:
  - Model a new narrow language surface as a dedicated CST/AST node on the existing item type instead of introducing a generic annotation system before the shape is proven.
  - Centralize manifest+source declaration merging inside `mesh-pkg` so compiler and LSP share identical duplicate handling, validation, and execution metadata.
  - When an unrelated backend bug would make a feature rail falsely red, keep the proof on a richer known-good fixture but assert the real downstream boundary explicitly.
observability_surfaces:
  - Compiler diagnostics now distinguish `mesh.toml` declarations from source `clustered(work)` markers for duplicate/private-target failures.
  - `mesh-lsp` project analysis mirrors the compiler’s clustered declaration diagnostics for source-only invalid cases.
  - `compiler/meshc/tests/e2e_m046_s01.rs` asserts emitted LLVM contains `mesh_register_declared_handler` plus the source-declared runtime registration name/wrapper, which is the authoritative proof that both declaration forms converge on the same runtime boundary.
drill_down_paths:
  - .gsd/milestones/M046/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M046/slices/S01/tasks/T02-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-31T15:46:31.470Z
blocker_discovered: false
---

# S01: Dual clustered-work declaration

**Mesh can now declare clustered work either in `mesh.toml` or with a narrow source-level `clustered(work)` marker, and both forms compile through the same declared-handler planning/runtime boundary.**

## What Happened

S01 delivered the declaration half of M046. T01 added a narrow contextual `clustered(work)` item prefix before `fn|def`, represented valid markers as a dedicated `CLUSTERED_WORK_DECL` node attached to `FnDef`, and locked the fail-closed parser contract with snapshots plus malformed-prefix tests so undecorated functions and the existing `@` rejection surface stayed unchanged. T02 then taught `mesh-pkg` to collect source-declared work from parsed modules, merge those declarations with `mesh.toml` declarations through one origin-aware validation helper, and emit the same `ClusteredExecutionMetadata` records already consumed by the declared-handler planner. `meshc` now uses that shared helper during build planning, `mesh-lsp` uses the same helper during project analysis, and the `e2e_m046_s01` compiler rail proves a source-only project still emits `mesh_register_declared_handler` with the same runtime registration name and wrapper shape as the manifest path. Invalid private decorated targets and same-target manifest+source duplicates now fail closed with explicit origin-tagged diagnostics in both compiler and LSP output, while the retained M044 S01/S02 suites confirm manifest-only declared-handler planning and registration did not regress.

## Verification

Verified the assembled slice with the full promised proof surface plus the focused shared-helper rail: `cargo test -p mesh-parser --test parser_tests m046_s01_parser_ -- --nocapture` (10 passed), `cargo test -p mesh-pkg m046_s01_ -- --nocapture` (5 passed), `cargo test -p meshc --test e2e_m046_s01 m046_s01_ -- --nocapture` (3 passed), `cargo test -p mesh-lsp m046_s01_ -- --nocapture` (3 passed), `cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture` (15 passed), and `cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture` (9 passed). The compiler happy-path rail explicitly proves emitted LLVM still registers source-declared work through `mesh_register_declared_handler` with the same runtime registration name/wrapper shape as manifest declarations.

## Requirements Advanced

- R085 — Added the source-level `clustered(work)` declaration form, merged it with manifest declarations through the shared clustered planner, and proved both surfaces converge on the same declared-handler runtime boundary without regressing the M044 manifest path.

## Requirements Validated

- R085 — Validated by `cargo test -p mesh-parser --test parser_tests m046_s01_parser_ -- --nocapture`, `cargo test -p mesh-pkg m046_s01_ -- --nocapture`, `cargo test -p meshc --test e2e_m046_s01 m046_s01_ -- --nocapture`, `cargo test -p mesh-lsp m046_s01_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture`, and `cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture`.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

T01 seeded the compiler and LSP proof rails early so T02 had concrete red contracts instead of missing test targets. T02 also kept the final source-only happy-path compiler proof on the broader M044-shaped work fixture because a smaller single-function source-only fixture still reproduces an older unrelated LLVM verifier failure.

## Known Limitations

- The new source surface is intentionally narrow: only contextual `clustered(work)` immediately before `fn|def` is supported in S01; there is still no broader decorator or annotation system.
- A smaller source-only compiler fixture that defines only `handle_submit` still reproduces an older LLVM verifier failure, so the green happy-path proof currently uses the broader M044-shaped work module.
- This slice only covers declaration and planning. Runtime-owned startup triggering, route-free status/tooling truth, `tiny-cluster/`, and the rebuilt route-free `cluster-proof/` remain for later slices.

## Follow-ups

- S02 should consume the shared clustered execution plan instead of inventing a second declaration path when runtime-owned startup triggering and status surfaces land.
- Future compiler/codegen work should isolate the unrelated single-function LLVM verifier failure so the M046 happy-path rail can shrink to the minimal source-only fixture without losing truthful coverage.
- The GSD requirements DB still cannot see `R085`–`R096`; repair that mapping before later M046 slices try to move requirement status through `gsd_requirement_update`.

## Files Created/Modified

- `compiler/mesh-parser/src/parser/items.rs` — Added contextual `clustered(work)` intake before `fn|def`, including fail-closed prefix parsing and recovery.
- `compiler/mesh-parser/src/ast/item.rs` — Exposed the new source marker as an optional typed AST child on `FnDef` for downstream compiler/LSP consumers.
- `compiler/mesh-parser/src/syntax_kind.rs` — Added the dedicated `CLUSTERED_WORK_DECL` syntax kind for the narrow clustered-work marker.
- `compiler/mesh-pkg/src/manifest.rs` — Collected source-level clustered declarations and merged them with manifest declarations through shared origin-aware validation/planning helpers.
- `compiler/meshc/src/main.rs` — Fed merged manifest/source clustered declarations into build planning so source-only work reaches the existing declared-handler runtime path.
- `compiler/mesh-lsp/src/analysis.rs` — Reused the shared clustered declaration validation path during project analysis so editor diagnostics match compiler behavior.
- `compiler/meshc/tests/e2e_m046_s01.rs` — Added parser, compiler, and regression rails for valid source-declared work, private-target rejection, duplicate source+manifest rejection, and declared-handler registration proof.
- `compiler/mesh-parser/tests/parser_tests.rs` — Locked the narrow parser surface and malformed-prefix recovery with dedicated M046 parser tests and snapshots.
