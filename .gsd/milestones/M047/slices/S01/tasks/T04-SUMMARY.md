---
id: T04
parent: S01
milestone: M047
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-lsp/src/analysis.rs", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Source-origin clustered validation issues in mesh-lsp now reuse mesh-pkg provenance but only emit on the currently analyzed relative file; manifest-origin issues keep the project-level fallback.", "mesh-lsp converts stored clustered declaration byte spans directly into LSP positions instead of rebuilding a second range source locally."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the full S01 verification stack because T04 is the final task in the slice. `cargo test -p mesh-parser m047_s01 -- --nocapture`, `cargo test -p mesh-pkg m047_s01 -- --nocapture`, `cargo test -p meshc --test e2e_m047_s01 -- --nocapture`, and `cargo test -p mesh-lsp m047_s01 -- --nocapture` all passed. The mesh-lsp rail now runs three real `m047_s01_*` tests and proves diagnostics-clean source-only `@cluster` code plus range-anchored duplicate/private clustered failures."
completed_at: 2026-04-01T06:02:57.509Z
blocker_discovered: false
---

# T04: Anchored mesh-lsp clustered diagnostics on decorated source declarations and added the M047 editor rail for clean and range-accurate `@cluster` analysis.

> Anchored mesh-lsp clustered diagnostics on decorated source declarations and added the M047 editor rail for clean and range-accurate `@cluster` analysis.

## What Happened
---
id: T04
parent: S01
milestone: M047
key_files:
  - compiler/mesh-lsp/src/analysis.rs
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Source-origin clustered validation issues in mesh-lsp now reuse mesh-pkg provenance but only emit on the currently analyzed relative file; manifest-origin issues keep the project-level fallback.
  - mesh-lsp converts stored clustered declaration byte spans directly into LSP positions instead of rebuilding a second range source locally.
duration: ""
verification_result: passed
completed_at: 2026-04-01T06:02:57.510Z
blocker_discovered: false
---

# T04: Anchored mesh-lsp clustered diagnostics on decorated source declarations and added the M047 editor rail for clean and range-accurate `@cluster` analysis.

**Anchored mesh-lsp clustered diagnostics on decorated source declarations and added the M047 editor rail for clean and range-accurate `@cluster` analysis.**

## What Happened

Updated `compiler/mesh-lsp/src/analysis.rs` so project analysis keeps using the shared mesh-pkg clustered declaration/export-surface seam, but source-origin clustered validation issues no longer collapse to `(0,0)` project diagnostics. The LSP path now filters source-origin issues to the currently analyzed relative file, clamps the recorded provenance span against that file's source, and converts the byte span directly into an LSP range. I also replaced the old legacy-focused editor tests with an M047 rail that analyzes `work.mpl` directly, proving clean source-only `@cluster` / `@cluster(3)` analysis and range-anchored duplicate/private failures on the decorated declaration line.

## Verification

Ran the full S01 verification stack because T04 is the final task in the slice. `cargo test -p mesh-parser m047_s01 -- --nocapture`, `cargo test -p mesh-pkg m047_s01 -- --nocapture`, `cargo test -p meshc --test e2e_m047_s01 -- --nocapture`, and `cargo test -p mesh-lsp m047_s01 -- --nocapture` all passed. The mesh-lsp rail now runs three real `m047_s01_*` tests and proves diagnostics-clean source-only `@cluster` code plus range-anchored duplicate/private clustered failures.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-parser m047_s01 -- --nocapture` | 0 | ✅ pass | 1000ms |
| 2 | `cargo test -p mesh-pkg m047_s01 -- --nocapture` | 0 | ✅ pass | 0ms |
| 3 | `cargo test -p meshc --test e2e_m047_s01 -- --nocapture` | 0 | ✅ pass | 13000ms |
| 4 | `cargo test -p mesh-lsp m047_s01 -- --nocapture` | 0 | ✅ pass | 1000ms |


## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `compiler/mesh-lsp/src/analysis.rs`
- `.gsd/KNOWLEDGE.md`


## Deviations
None.

## Known Issues
None.
