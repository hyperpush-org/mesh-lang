---
id: T01
parent: S01
milestone: M044
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-pkg/src/manifest.rs", "compiler/meshc/src/main.rs", "compiler/mesh-lsp/src/analysis.rs", "compiler/mesh-lsp/Cargo.toml", "compiler/meshc/tests/e2e_m044_s01.rs", ".gsd/milestones/M044/slices/S01/tasks/T01-SUMMARY.md"]
key_decisions: ["D189: use `[cluster] enabled = true` with explicit `{ kind, target }` declarations and fully-qualified public target strings; reject ambiguous overloaded work names and non-call/non-cast service helpers fail-closed."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed `cargo test -p mesh-pkg clustered_manifest_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s01 manifest_ -- --nocapture`, and `cargo test -p mesh-lsp clustered_manifest_ -- --nocapture`. Also replayed the full current compiler target with `cargo test -p meshc --test e2e_m044_s01 -- --nocapture` and it passed (6/6). The slice-level assembled verifier `bash scripts/verify-m044-s01.sh` was not run because T04 owns that script and it does not exist yet."
completed_at: 2026-03-29T18:10:11.070Z
blocker_discovered: false
---

# T01: Added optional clustered manifest declarations with shared compiler/LSP validation and the first M044 contract tests.

> Added optional clustered manifest declarations with shared compiler/LSP validation and the first M044 contract tests.

## What Happened
---
id: T01
parent: S01
milestone: M044
key_files:
  - compiler/mesh-pkg/src/manifest.rs
  - compiler/meshc/src/main.rs
  - compiler/mesh-lsp/src/analysis.rs
  - compiler/mesh-lsp/Cargo.toml
  - compiler/meshc/tests/e2e_m044_s01.rs
  - .gsd/milestones/M044/slices/S01/tasks/T01-SUMMARY.md
key_decisions:
  - D189: use `[cluster] enabled = true` with explicit `{ kind, target }` declarations and fully-qualified public target strings; reject ambiguous overloaded work names and non-call/non-cast service helpers fail-closed.
duration: ""
verification_result: passed
completed_at: 2026-03-29T18:10:11.072Z
blocker_discovered: false
---

# T01: Added optional clustered manifest declarations with shared compiler/LSP validation and the first M044 contract tests.

**Added optional clustered manifest declarations with shared compiler/LSP validation and the first M044 contract tests.**

## What Happened

Extended `compiler/mesh-pkg/src/manifest.rs` with an optional `[cluster]` section that requires `enabled = true` and a non-empty `declarations` array of `{ kind, target }` entries. The shared validator now defines the public clustered boundary: `work` uses fully-qualified public function targets, `service_call`/`service_cast` use fully-qualified service handler targets, service start helpers are rejected, and overloaded public work names fail closed instead of leaking internal mangled names into the manifest contract. Wired `compiler/meshc/src/main.rs` to load `mesh.toml` only when present, preserve manifestless builds, and validate clustered declarations after export collection and before MIR lowering with explicit target/reason diagnostics. Added `mesh-pkg` to `mesh-lsp` and mirrored the same manifest-aware validation in `compiler/mesh-lsp/src/analysis.rs` so editor truth stays aligned with the compiler path. Created `compiler/meshc/tests/e2e_m044_s01.rs` with stable `manifest_` coverage, plus clustered-manifest regression tests in `mesh-pkg` and `mesh-lsp`.

## Verification

Passed `cargo test -p mesh-pkg clustered_manifest_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s01 manifest_ -- --nocapture`, and `cargo test -p mesh-lsp clustered_manifest_ -- --nocapture`. Also replayed the full current compiler target with `cargo test -p meshc --test e2e_m044_s01 -- --nocapture` and it passed (6/6). The slice-level assembled verifier `bash scripts/verify-m044-s01.sh` was not run because T04 owns that script and it does not exist yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-pkg clustered_manifest_ -- --nocapture` | 0 | ✅ pass | 107300ms |
| 2 | `cargo test -p meshc --test e2e_m044_s01 manifest_ -- --nocapture` | 0 | ✅ pass | 104100ms |
| 3 | `cargo test -p mesh-lsp clustered_manifest_ -- --nocapture` | 0 | ✅ pass | 92700ms |


## Deviations

Used the named task rails plus a full `cargo test -p meshc --test e2e_m044_s01 -- --nocapture` replay as the intermediate slice-level verification surface because `scripts/verify-m044-s01.sh` belongs to T04 and is not present yet.

## Known Issues

None.

## Files Created/Modified

- `compiler/mesh-pkg/src/manifest.rs`
- `compiler/meshc/src/main.rs`
- `compiler/mesh-lsp/src/analysis.rs`
- `compiler/mesh-lsp/Cargo.toml`
- `compiler/meshc/tests/e2e_m044_s01.rs`
- `.gsd/milestones/M044/slices/S01/tasks/T01-SUMMARY.md`


## Deviations
Used the named task rails plus a full `cargo test -p meshc --test e2e_m044_s01 -- --nocapture` replay as the intermediate slice-level verification surface because `scripts/verify-m044-s01.sh` belongs to T04 and is not present yet.

## Known Issues
None.
