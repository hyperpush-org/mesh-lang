---
id: T02
parent: S01
milestone: M047
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-pkg/src/manifest.rs", "compiler/mesh-pkg/src/lib.rs", "compiler/mesh-pkg/Cargo.toml", ".gsd/milestones/M047/slices/S01/tasks/T02-SUMMARY.md"]
key_decisions: ["Moved clustered declaration count/provenance truth and export-surface construction into mesh-pkg so meshc and mesh-lsp can consume one shared seam instead of rebuilding divergent answers."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran `cargo test -p mesh-pkg m047_s01 -- --nocapture` and confirmed all 7 M047 mesh-pkg tests passed. The rail covered default replication count `2`, explicit count preservation, source provenance capture, duplicate manifest/source rejection, private-source rejection, malformed target rejection with default-count context, and shared export-surface construction for work functions plus service-generated handlers."
completed_at: 2026-04-01T05:41:12.971Z
blocker_discovered: false
---

# T02: Added mesh-pkg-owned clustered source records, replication-count/origin metadata, and a shared export-surface helper for work functions plus service-generated handlers.

> Added mesh-pkg-owned clustered source records, replication-count/origin metadata, and a shared export-surface helper for work functions plus service-generated handlers.

## What Happened
---
id: T02
parent: S01
milestone: M047
key_files:
  - compiler/mesh-pkg/src/manifest.rs
  - compiler/mesh-pkg/src/lib.rs
  - compiler/mesh-pkg/Cargo.toml
  - .gsd/milestones/M047/slices/S01/tasks/T02-SUMMARY.md
key_decisions:
  - Moved clustered declaration count/provenance truth and export-surface construction into mesh-pkg so meshc and mesh-lsp can consume one shared seam instead of rebuilding divergent answers.
duration: ""
verification_result: passed
completed_at: 2026-04-01T05:41:12.971Z
blocker_discovered: false
---

# T02: Added mesh-pkg-owned clustered source records, replication-count/origin metadata, and a shared export-surface helper for work functions plus service-generated handlers.

**Added mesh-pkg-owned clustered source records, replication-count/origin metadata, and a shared export-surface helper for work functions plus service-generated handlers.**

## What Happened

Expanded `compiler/mesh-pkg/src/manifest.rs` from the old string-only source collector into a richer clustered declaration seam. Source discovery now returns `SourceClusteredDeclaration` records with qualified target, resolved replication count, source syntax (`@cluster` vs legacy `clustered(work)`), relative file path, module name, and declaration span. Validation now threads that information through both `ClusteredDeclarationError` and `ClusteredExecutionMetadata`, so later compiler/LSP tasks can read default-versus-explicit counts and source provenance without reopening parser details. I also extracted `build_clustered_export_surface(...)` into mesh-pkg so public work functions and service-generated handlers are resolved in one shared place, re-exported the clustered APIs from `compiler/mesh-pkg/src/lib.rs`, and added focused M047 tests for source collection, duplicate/private failures, malformed targets, and service/work export-surface construction.

## Verification

Ran `cargo test -p mesh-pkg m047_s01 -- --nocapture` and confirmed all 7 M047 mesh-pkg tests passed. The rail covered default replication count `2`, explicit count preservation, source provenance capture, duplicate manifest/source rejection, private-source rejection, malformed target rejection with default-count context, and shared export-surface construction for work functions plus service-generated handlers.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-pkg m047_s01 -- --nocapture` | 0 | ✅ pass | 11070ms |


## Deviations

No code change was needed in `compiler/mesh-typeck/src/lib.rs`; the existing `ServiceExportInfo.method_exports` shape was already sufficient for the shared mesh-pkg helper once mesh-pkg consumed it directly.

## Known Issues

`meshc` and `mesh-lsp` still call their local export-surface builders and do not yet consume the richer source-origin/count metadata for diagnostics. That consumer migration remains for T03/T04.

## Files Created/Modified

- `compiler/mesh-pkg/src/manifest.rs`
- `compiler/mesh-pkg/src/lib.rs`
- `compiler/mesh-pkg/Cargo.toml`
- `.gsd/milestones/M047/slices/S01/tasks/T02-SUMMARY.md`


## Deviations
No code change was needed in `compiler/mesh-typeck/src/lib.rs`; the existing `ServiceExportInfo.method_exports` shape was already sufficient for the shared mesh-pkg helper once mesh-pkg consumed it directly.

## Known Issues
`meshc` and `mesh-lsp` still call their local export-surface builders and do not yet consume the richer source-origin/count metadata for diagnostics. That consumer migration remains for T03/T04.
