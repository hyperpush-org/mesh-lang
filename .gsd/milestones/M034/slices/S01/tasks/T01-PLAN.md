---
estimated_steps: 4
estimated_files: 3
skills_used:
  - rust-best-practices
  - test
---

# T01: Make scoped installed packages discoverable to meshc and mesh-lsp

**Slice:** S01 — Real registry publish/install proof
**Milestone:** M034

## Description

Close the hard blocker from research: real registry installs already extract scoped packages to `.mesh/packages/<owner>/<package>@<version>`, but both `meshc` and `mesh-lsp` stop one directory too early and therefore treat the owner directory as the package root. Preserve the scoped on-disk layout, teach both analyzers to recurse to leaf package roots containing `mesh.toml`, and pin the contract with named regressions that prove a consumer can import a module from a scoped installed package without manual flattening.

## Load Profile

- **Shared resources**: repeated filesystem walks under `.mesh/packages` during builds and editor analysis.
- **Per-operation cost**: recursive directory traversal plus parse/typecheck of each discovered package root.
- **10x breakpoint**: deeply nested or version-heavy package caches would hurt build/LSP latency first, so discovery must stay deterministic and skip non-package directories cheaply.

## Negative Tests

- **Malformed inputs**: owner directories with no `mesh.toml`, hidden directories/files, and package trees that contain only `main.mpl`.
- **Error paths**: nested non-package directories should be skipped instead of panicking or compiling package-root `main.mpl` as a normal module.
- **Boundary conditions**: both `.mesh/packages/<owner>/<package>@<version>` and flat `.mesh/packages/<package>@<version>` layouts resolve the same import/module naming rules.

## Steps

1. Add a package-root discovery rule in `compiler/meshc/src/discovery.rs` that finds leaf package directories containing `mesh.toml` anywhere under `.mesh/packages`, while still ignoring hidden paths and package-root `main.mpl`.
2. Mirror the same scoped-package discovery semantics in `compiler/mesh-lsp/src/analysis.rs` so editor diagnostics, hover, and navigation stay aligned with `meshc build`.
3. Add `compiler/meshc/tests/e2e_m034_s01.rs` coverage that lays out a temp consumer plus scoped installed package tree and proves the consumer builds without manual filesystem moves.
4. Add or extend `compiler/mesh-lsp/src/analysis.rs` unit tests so nested scoped package roots analyze cleanly and do not regress back to the owner-directory bug.

## Must-Haves

- [ ] Scoped installed packages build from their natural nested owner/package cache path.
- [ ] `meshc` and `mesh-lsp` share the same package-root expectation for scoped installs.
- [ ] Named regressions fail specifically on discovery drift instead of surfacing later as a vague `module not found` release failure.

## Verification

- `cargo test -p meshc --test e2e_m034_s01 scoped_installed_package_builds -- --nocapture`
- `cargo test -p mesh-lsp scoped_installed_package -- --nocapture`

## Observability Impact

- Signals added/changed: named scoped-package build and analysis regressions.
- How a future agent inspects this: rerun the `scoped_installed_package` test filters in `compiler/meshc/tests/e2e_m034_s01.rs` and `compiler/mesh-lsp/src/analysis.rs`.
- Failure state exposed: whether drift came from package-root discovery, module naming, or editor/runtime divergence.

## Inputs

- `compiler/meshc/src/discovery.rs` — current one-level package scan that stops at owner directories.
- `compiler/mesh-lsp/src/analysis.rs` — duplicated package discovery logic that must stay aligned with `meshc`.

## Expected Output

- `compiler/meshc/src/discovery.rs` — leaf package-root discovery for nested scoped installs.
- `compiler/mesh-lsp/src/analysis.rs` — matching nested package-root discovery for editor analysis.
- `compiler/meshc/tests/e2e_m034_s01.rs` — named regressions proving scoped installed packages compile.
