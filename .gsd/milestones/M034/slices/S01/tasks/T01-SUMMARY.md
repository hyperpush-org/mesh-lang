---
id: T01
parent: S01
milestone: M034
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/src/discovery.rs", "compiler/mesh-lsp/src/analysis.rs", "compiler/meshc/tests/e2e_m034_s01.rs", "compiler/mesh-lsp/Cargo.toml", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Treat the first descendant under `.mesh/packages` containing `mesh.toml` as the installed package root and stop descending there.", "Preserve the existing scoped cache layout and align `meshc`/`mesh-lsp` around manifest-leaf discovery instead of flattening installed packages."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed `cargo test -p meshc --test e2e_m034_s01 -- --nocapture`, `cargo test -p mesh-lsp scoped_installed_package -- --nocapture`, and `cargo test -p meshc test_build_project_discovers_scoped_installed_package_modules -- --nocapture`. These cover the scoped consumer build/run path, direct compiler package-root drift, and LSP analysis of scoped plus flat installed-package layouts. Slice-level verifier/script and docs-contract checks remain owned by T02/T03 and were not expected to pass yet."
completed_at: 2026-03-26T19:54:27.148Z
blocker_discovered: false
---

# T01: Taught meshc and mesh-lsp to discover scoped installed package roots from mesh.toml leaves and added scoped package regressions that prove the natural cache layout builds and analyzes cleanly.

> Taught meshc and mesh-lsp to discover scoped installed package roots from mesh.toml leaves and added scoped package regressions that prove the natural cache layout builds and analyzes cleanly.

## What Happened
---
id: T01
parent: S01
milestone: M034
key_files:
  - compiler/meshc/src/discovery.rs
  - compiler/mesh-lsp/src/analysis.rs
  - compiler/meshc/tests/e2e_m034_s01.rs
  - compiler/mesh-lsp/Cargo.toml
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Treat the first descendant under `.mesh/packages` containing `mesh.toml` as the installed package root and stop descending there.
  - Preserve the existing scoped cache layout and align `meshc`/`mesh-lsp` around manifest-leaf discovery instead of flattening installed packages.
duration: ""
verification_result: passed
completed_at: 2026-03-26T19:54:27.148Z
blocker_discovered: false
---

# T01: Taught meshc and mesh-lsp to discover scoped installed package roots from mesh.toml leaves and added scoped package regressions that prove the natural cache layout builds and analyzes cleanly.

**Taught meshc and mesh-lsp to discover scoped installed package roots from mesh.toml leaves and added scoped package regressions that prove the natural cache layout builds and analyzes cleanly.**

## What Happened

Added manifest-driven installed-package root discovery to compiler and LSP analysis so both walk `.mesh/packages` recursively until they reach the first descendant containing `mesh.toml`, then resolve modules relative to that leaf. This preserves the scoped on-disk cache layout while naturally skipping package-root `main.mpl`. Switched `meshc` and `mesh-lsp` package scans to use those roots, added direct compiler and LSP regressions for scoped and flat layouts, added a new `e2e_m034_s01` consumer build/run proof, and recorded the repo-local gotcha in `.gsd/KNOWLEDGE.md`. During verification I corrected the test fixtures to export `message` publicly and added `tempfile` to `mesh-lsp` dev-dependencies so the new temp-project LSP tests compile.

## Verification

Passed `cargo test -p meshc --test e2e_m034_s01 -- --nocapture`, `cargo test -p mesh-lsp scoped_installed_package -- --nocapture`, and `cargo test -p meshc test_build_project_discovers_scoped_installed_package_modules -- --nocapture`. These cover the scoped consumer build/run path, direct compiler package-root drift, and LSP analysis of scoped plus flat installed-package layouts. Slice-level verifier/script and docs-contract checks remain owned by T02/T03 and were not expected to pass yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m034_s01 -- --nocapture` | 0 | ✅ pass | 15900ms |
| 2 | `cargo test -p mesh-lsp scoped_installed_package -- --nocapture` | 0 | ✅ pass | 7000ms |
| 3 | `cargo test -p meshc test_build_project_discovers_scoped_installed_package_modules -- --nocapture` | 0 | ✅ pass | 20000ms |


## Deviations

Added a direct `meshc` unit regression in `compiler/meshc/src/discovery.rs` so package-root drift fails earlier than the end-to-end import proof. Added `tempfile` to `compiler/mesh-lsp/Cargo.toml` dev-dependencies to support temp-project analysis fixtures.

## Known Issues

None.

## Files Created/Modified

- `compiler/meshc/src/discovery.rs`
- `compiler/mesh-lsp/src/analysis.rs`
- `compiler/meshc/tests/e2e_m034_s01.rs`
- `compiler/mesh-lsp/Cargo.toml`
- `.gsd/KNOWLEDGE.md`


## Deviations
Added a direct `meshc` unit regression in `compiler/meshc/src/discovery.rs` so package-root drift fails earlier than the end-to-end import proof. Added `tempfile` to `compiler/mesh-lsp/Cargo.toml` dev-dependencies to support temp-project analysis fixtures.

## Known Issues
None.
