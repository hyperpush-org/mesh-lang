---
id: T01
parent: S01
milestone: M046
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-parser/src/parser/mod.rs", "compiler/mesh-parser/src/parser/items.rs", "compiler/mesh-parser/src/ast/item.rs", "compiler/mesh-parser/src/syntax_kind.rs", "compiler/mesh-parser/tests/parser_tests.rs", "compiler/mesh-parser/tests/snapshots/parser_tests__m046_s01_parser_clustered_work_fn_def.snap", "compiler/mesh-parser/tests/snapshots/parser_tests__m046_s01_parser_clustered_work_invalid_prefix.snap", "compiler/meshc/tests/e2e_m046_s01.rs", "compiler/mesh-lsp/src/analysis.rs", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Represent the narrow source marker as a dedicated `CLUSTERED_WORK_DECL` node on `FnDef` instead of introducing a generic annotation system.", "Seed future slice rails with real spec tests in T01 so T02 fixes concrete red contracts instead of missing test targets."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task-level parser verification passed: `cargo test -p mesh-parser --test parser_tests m046_s01_parser_ -- --nocapture` and the required snapshot file exists. Slice-level verification is partial as expected for the first task: the new compiler rail fails with an LLVM verifier error before declared-handler registration appears, the new LSP rail fails because no private decorated-work diagnostic is emitted yet, and both M044 regression rails remain green."
completed_at: 2026-03-31T15:11:15.879Z
blocker_discovered: false
---

# T01: Added fail-closed `clustered(work)` parser intake and seeded real compiler/LSP proof rails for T02.

> Added fail-closed `clustered(work)` parser intake and seeded real compiler/LSP proof rails for T02.

## What Happened
---
id: T01
parent: S01
milestone: M046
key_files:
  - compiler/mesh-parser/src/parser/mod.rs
  - compiler/mesh-parser/src/parser/items.rs
  - compiler/mesh-parser/src/ast/item.rs
  - compiler/mesh-parser/src/syntax_kind.rs
  - compiler/mesh-parser/tests/parser_tests.rs
  - compiler/mesh-parser/tests/snapshots/parser_tests__m046_s01_parser_clustered_work_fn_def.snap
  - compiler/mesh-parser/tests/snapshots/parser_tests__m046_s01_parser_clustered_work_invalid_prefix.snap
  - compiler/meshc/tests/e2e_m046_s01.rs
  - compiler/mesh-lsp/src/analysis.rs
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Represent the narrow source marker as a dedicated `CLUSTERED_WORK_DECL` node on `FnDef` instead of introducing a generic annotation system.
  - Seed future slice rails with real spec tests in T01 so T02 fixes concrete red contracts instead of missing test targets.
duration: ""
verification_result: mixed
completed_at: 2026-03-31T15:11:15.882Z
blocker_discovered: false
---

# T01: Added fail-closed `clustered(work)` parser intake and seeded real compiler/LSP proof rails for T02.

**Added fail-closed `clustered(work)` parser intake and seeded real compiler/LSP proof rails for T02.**

## What Happened

Implemented a narrow contextual `clustered(work)` parser path before `fn|def`, represented valid markers as a dedicated `CLUSTERED_WORK_DECL` child on `FnDef`, and added AST accessors plus parser snapshots and negative tests for malformed forms. Also created the first real `m046_s01_` compiler and LSP proof rails so T02 has concrete red contracts instead of missing test targets.

## Verification

Task-level parser verification passed: `cargo test -p mesh-parser --test parser_tests m046_s01_parser_ -- --nocapture` and the required snapshot file exists. Slice-level verification is partial as expected for the first task: the new compiler rail fails with an LLVM verifier error before declared-handler registration appears, the new LSP rail fails because no private decorated-work diagnostic is emitted yet, and both M044 regression rails remain green.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-parser --test parser_tests m046_s01_parser_ -- --nocapture` | 0 | ✅ pass | 16825ms |
| 2 | `test -f compiler/mesh-parser/tests/snapshots/parser_tests__m046_s01_parser_clustered_work_fn_def.snap` | 0 | ✅ pass | 46ms |
| 3 | `cargo test -p meshc --test e2e_m046_s01 m046_s01_ -- --nocapture` | 101 | ❌ fail | 93405ms |
| 4 | `cargo test -p mesh-lsp m046_s01_ -- --nocapture` | 101 | ❌ fail | 39491ms |
| 5 | `cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture` | 0 | ✅ pass | 21708ms |
| 6 | `cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture` | 0 | ✅ pass | 28857ms |


## Deviations

Created the T02-owned compiler/LSP proof rails during T01 even though the written task plan only listed parser files, because the slice execution contract requires the first task to create the later verification targets.

## Known Issues

`cargo test -p meshc --test e2e_m046_s01 m046_s01_ -- --nocapture` currently fails with `LLVM module verification failed: Function return type does not match operand type of return inst! ret {} zeroinitializer i64`, and `cargo test -p mesh-lsp m046_s01_ -- --nocapture` currently fails because the private decorated-work diagnostic is still absent. Both failures are expected handoff work for T02.

## Files Created/Modified

- `compiler/mesh-parser/src/parser/mod.rs`
- `compiler/mesh-parser/src/parser/items.rs`
- `compiler/mesh-parser/src/ast/item.rs`
- `compiler/mesh-parser/src/syntax_kind.rs`
- `compiler/mesh-parser/tests/parser_tests.rs`
- `compiler/mesh-parser/tests/snapshots/parser_tests__m046_s01_parser_clustered_work_fn_def.snap`
- `compiler/mesh-parser/tests/snapshots/parser_tests__m046_s01_parser_clustered_work_invalid_prefix.snap`
- `compiler/meshc/tests/e2e_m046_s01.rs`
- `compiler/mesh-lsp/src/analysis.rs`
- `.gsd/KNOWLEDGE.md`


## Deviations
Created the T02-owned compiler/LSP proof rails during T01 even though the written task plan only listed parser files, because the slice execution contract requires the first task to create the later verification targets.

## Known Issues
`cargo test -p meshc --test e2e_m046_s01 m046_s01_ -- --nocapture` currently fails with `LLVM module verification failed: Function return type does not match operand type of return inst! ret {} zeroinitializer i64`, and `cargo test -p mesh-lsp m046_s01_ -- --nocapture` currently fails because the private decorated-work diagnostic is still absent. Both failures are expected handoff work for T02.
