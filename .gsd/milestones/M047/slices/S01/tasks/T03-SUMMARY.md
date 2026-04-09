---
id: T03
parent: S01
milestone: M047
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/src/main.rs", "compiler/meshc/tests/e2e_m047_s01.rs", "compiler/mesh-lsp/src/analysis.rs", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Kept `replication_count` and `origin` on `ClusteredExecutionMetadata` / `PreparedBuild`, but left the codegen-facing declared-handler plan limited to runtime registration names and executable symbols so T03 could add source/count truth without changing runtime behavior.", "Used CLI-only integration tests for the M047 compiler rail instead of importing `compiler/meshc/src/main.rs`, because the latter also runs meshc internal unit tests under `cfg(test)` and contaminates the target with unrelated failures."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran `cargo test -p meshc --test e2e_m047_s01 -- --nocapture` and confirmed all four M047 compiler tests passed. The rail proves source-only `@cluster` / `@cluster(3)` success, stable runtime registration naming in emitted LLVM, human source-ranged private-decorator failure with explicit count context, JSON source-ranged duplicate failure, and malformed-count fail-before-codegen behavior. As a slice-level check after the compatibility fix, I also ran `cargo test -p mesh-lsp m047_s01 -- --nocapture`; the crate now compiles again, but the command still runs 0 tests, so T04 remains outstanding."
completed_at: 2026-04-01T05:56:22.791Z
blocker_discovered: false
---

# T03: Switched meshc to the shared clustered declaration seam and added an M047 compiler rail for source-only builds and source-ranged diagnostics.

> Switched meshc to the shared clustered declaration seam and added an M047 compiler rail for source-only builds and source-ranged diagnostics.

## What Happened
---
id: T03
parent: S01
milestone: M047
key_files:
  - compiler/meshc/src/main.rs
  - compiler/meshc/tests/e2e_m047_s01.rs
  - compiler/mesh-lsp/src/analysis.rs
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Kept `replication_count` and `origin` on `ClusteredExecutionMetadata` / `PreparedBuild`, but left the codegen-facing declared-handler plan limited to runtime registration names and executable symbols so T03 could add source/count truth without changing runtime behavior.
  - Used CLI-only integration tests for the M047 compiler rail instead of importing `compiler/meshc/src/main.rs`, because the latter also runs meshc internal unit tests under `cfg(test)` and contaminates the target with unrelated failures.
duration: ""
verification_result: passed
completed_at: 2026-04-01T05:56:22.792Z
blocker_discovered: false
---

# T03: Switched meshc to the shared clustered declaration seam and added an M047 compiler rail for source-only builds and source-ranged diagnostics.

**Switched meshc to the shared clustered declaration seam and added an M047 compiler rail for source-only builds and source-ranged diagnostics.**

## What Happened

Switched meshc build planning from its local clustered export-surface builder to the shared mesh-pkg declaration/export seam from T02, while preserving the existing runtime registration/executable-symbol boundary that codegen consumes. meshc clustered diagnostics now resolve source-origin failures back to the recorded declaration file and decorator span, emitting a real `file` plus `spans` entry in JSON mode and a `--> file:line:col` header plus source label in human mode. Added `compiler/meshc/tests/e2e_m047_s01.rs` to prove a source-only package with bare `@cluster` and explicit `@cluster(3)` builds without `[cluster]`, that emitted LLVM still carries the stable declared-handler registration markers, that explicit-count private decorators fail before LLVM emission with count/source context, that manifest/source duplicates produce real `work.mpl` JSON spans, and that malformed decorator counts fail before codegen. Local reality required one small mesh-lsp signature update so the meshc rail could compile against the new shared mesh-pkg source declaration type, but the actual LSP range-based diagnostic work remains for T04.

## Verification

Ran `cargo test -p meshc --test e2e_m047_s01 -- --nocapture` and confirmed all four M047 compiler tests passed. The rail proves source-only `@cluster` / `@cluster(3)` success, stable runtime registration naming in emitted LLVM, human source-ranged private-decorator failure with explicit count context, JSON source-ranged duplicate failure, and malformed-count fail-before-codegen behavior. As a slice-level check after the compatibility fix, I also ran `cargo test -p mesh-lsp m047_s01 -- --nocapture`; the crate now compiles again, but the command still runs 0 tests, so T04 remains outstanding.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m047_s01 -- --nocapture` | 0 | ✅ pass | 6632ms |
| 2 | `cargo test -p mesh-lsp m047_s01 -- --nocapture` | 0 | ❌ fail | 615ms |


## Deviations

Added a small compatibility update in `compiler/mesh-lsp/src/analysis.rs` so the meshc rail could compile against T02’s shared mesh-pkg declaration types. No `compiler/mesh-codegen/src/codegen/mod.rs` change was needed because the existing declared-handler boundary already stayed narrow to runtime registration names and executable symbols.

## Known Issues

`cargo test -p mesh-lsp m047_s01 -- --nocapture` still reports `running 0 tests`, so T04 still needs to land the actual range-based clustered LSP diagnostics and matching coverage.

## Files Created/Modified

- `compiler/meshc/src/main.rs`
- `compiler/meshc/tests/e2e_m047_s01.rs`
- `compiler/mesh-lsp/src/analysis.rs`
- `.gsd/KNOWLEDGE.md`


## Deviations
Added a small compatibility update in `compiler/mesh-lsp/src/analysis.rs` so the meshc rail could compile against T02’s shared mesh-pkg declaration types. No `compiler/mesh-codegen/src/codegen/mod.rs` change was needed because the existing declared-handler boundary already stayed narrow to runtime registration names and executable symbols.

## Known Issues
`cargo test -p mesh-lsp m047_s01 -- --nocapture` still reports `running 0 tests`, so T04 still needs to land the actual range-based clustered LSP diagnostics and matching coverage.
