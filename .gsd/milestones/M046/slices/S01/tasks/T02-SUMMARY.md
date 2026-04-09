---
id: T02
parent: S01
milestone: M046
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-pkg/src/manifest.rs", "compiler/mesh-pkg/Cargo.toml", "compiler/meshc/src/main.rs", "compiler/mesh-lsp/src/analysis.rs", "compiler/meshc/tests/e2e_m046_s01.rs", ".gsd/milestones/M046/slices/S01/tasks/T02-SUMMARY.md"]
key_decisions: ["Merge source `clustered(work)` declarations inside `mesh-pkg::manifest` so compiler and LSP share one duplicate/validation path.", "Keep the compiler happy-path proof on the known-good M044-shaped work fixture until the unrelated single-function LLVM verifier bug is fixed."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed the focused shared-helper unit rail (`cargo test -p mesh-pkg m046_s01_ -- --nocapture`), the required LSP proof rail (`cargo test -p mesh-lsp m046_s01_ -- --nocapture`), the required compiler proof rail (`cargo test -p meshc --test e2e_m046_s01 m046_s01_ -- --nocapture`), and both retained manifest regression suites (`cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture` and `cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture`). The compiler happy-path rail still verifies emitted `mesh_register_declared_handler` registration text for the source-only case."
completed_at: 2026-03-31T15:36:51.568Z
blocker_discovered: false
---

# T02: Merged source `clustered(work)` declarations into the shared clustered planner for meshc and mesh-lsp.

> Merged source `clustered(work)` declarations into the shared clustered planner for meshc and mesh-lsp.

## What Happened
---
id: T02
parent: S01
milestone: M046
key_files:
  - compiler/mesh-pkg/src/manifest.rs
  - compiler/mesh-pkg/Cargo.toml
  - compiler/meshc/src/main.rs
  - compiler/mesh-lsp/src/analysis.rs
  - compiler/meshc/tests/e2e_m046_s01.rs
  - .gsd/milestones/M046/slices/S01/tasks/T02-SUMMARY.md
key_decisions:
  - Merge source `clustered(work)` declarations inside `mesh-pkg::manifest` so compiler and LSP share one duplicate/validation path.
  - Keep the compiler happy-path proof on the known-good M044-shaped work fixture until the unrelated single-function LLVM verifier bug is fixed.
duration: ""
verification_result: passed
completed_at: 2026-03-31T15:36:51.572Z
blocker_discovered: false
---

# T02: Merged source `clustered(work)` declarations into the shared clustered planner for meshc and mesh-lsp.

**Merged source `clustered(work)` declarations into the shared clustered planner for meshc and mesh-lsp.**

## What Happened

Added a shared source-declaration collector and merged manifest/source clustered validation path in `mesh-pkg`, including origin-tagged diagnostics that distinguish `mesh.toml` declarations from source `clustered(work)` markers. Wired `meshc` build planning and `mesh-lsp` project analysis to call that shared helper so source-only decorated work now reaches the same declared-handler execution plan as manifest declarations, same-target source+manifest duplicates fail closed before codegen, and private/ambiguous decorated work produces explicit diagnostics. Expanded the compiler and LSP M046 proof rails plus focused `mesh-pkg` unit tests, then reran the retained M044 compiler suites to confirm manifest-only declared-handler behavior stayed green.

## Verification

Passed the focused shared-helper unit rail (`cargo test -p mesh-pkg m046_s01_ -- --nocapture`), the required LSP proof rail (`cargo test -p mesh-lsp m046_s01_ -- --nocapture`), the required compiler proof rail (`cargo test -p meshc --test e2e_m046_s01 m046_s01_ -- --nocapture`), and both retained manifest regression suites (`cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture` and `cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture`). The compiler happy-path rail still verifies emitted `mesh_register_declared_handler` registration text for the source-only case.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-pkg m046_s01_ -- --nocapture` | 0 | ✅ pass | 20500ms |
| 2 | `cargo test -p mesh-lsp m046_s01_ -- --nocapture` | 0 | ✅ pass | 64500ms |
| 3 | `cargo test -p meshc --test e2e_m046_s01 m046_s01_ -- --nocapture` | 0 | ✅ pass | 12230ms |
| 4 | `cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture` | 0 | ✅ pass | 17980ms |
| 5 | `cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture` | 0 | ✅ pass | 27120ms |


## Deviations

Adjusted the compiler happy-path fixture to reuse the known-good M044-shaped work module because the smaller single-function source-only fixture still reproduces an unrelated LLVM verifier failure. The shipped source-declaration path itself is covered by the final green e2e rail.

## Known Issues

A smaller source-only work fixture that only defines `handle_submit` still reproduces `LLVM module verification failed: Function return type does not match operand type of return inst! ret {} zeroinitializer i64`. T02 does not fix that older codegen issue; the green proof rail uses the M044-shaped fixture so the source-declaration planning surface is still honestly covered.

## Files Created/Modified

- `compiler/mesh-pkg/src/manifest.rs`
- `compiler/mesh-pkg/Cargo.toml`
- `compiler/meshc/src/main.rs`
- `compiler/mesh-lsp/src/analysis.rs`
- `compiler/meshc/tests/e2e_m046_s01.rs`
- `.gsd/milestones/M046/slices/S01/tasks/T02-SUMMARY.md`


## Deviations
Adjusted the compiler happy-path fixture to reuse the known-good M044-shaped work module because the smaller single-function source-only fixture still reproduces an unrelated LLVM verifier failure. The shipped source-declaration path itself is covered by the final green e2e rail.

## Known Issues
A smaller source-only work fixture that only defines `handle_submit` still reproduces `LLVM module verification failed: Function return type does not match operand type of return inst! ret {} zeroinitializer i64`. T02 does not fix that older codegen issue; the green proof rail uses the M044-shaped fixture so the source-declaration planning surface is still honestly covered.
