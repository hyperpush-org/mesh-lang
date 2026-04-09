---
id: T01
parent: S02
milestone: M044
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-pkg/src/manifest.rs", "compiler/mesh-typeck/src/lib.rs", "compiler/mesh-typeck/src/infer.rs", "compiler/meshc/src/main.rs", "compiler/mesh-lsp/src/analysis.rs", "compiler/meshc/tests/e2e_m044_s02.rs", ".gsd/milestones/M044/slices/S02/tasks/T01-SUMMARY.md"]
key_decisions: ["D195: return shared ClusteredExecutionMetadata from manifest validation and retain it alongside prepared MIR in meshc.", "Use explicit ServiceMethodExport kind metadata instead of re-deriving service call/cast/start roles from generated symbol prefixes in meshc and mesh-lsp."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed the slice-plan verification commands `cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture` and `cargo test -p meshc --test e2e_m044_s01 m044_s01_manifest_ -- --nocapture`. Also ran `cargo test -p mesh-pkg m044_s0 -- --nocapture` and `cargo test -p mesh-lsp m044_s01_clustered_manifest_ -- --nocapture` to confirm the shared validator and editor diagnostics stayed aligned."
completed_at: 2026-03-29T19:57:23.919Z
blocker_discovered: false
---

# T01: Threaded declared clustered execution metadata through shared manifest validation and into meshc build preparation.

> Threaded declared clustered execution metadata through shared manifest validation and into meshc build preparation.

## What Happened
---
id: T01
parent: S02
milestone: M044
key_files:
  - compiler/mesh-pkg/src/manifest.rs
  - compiler/mesh-typeck/src/lib.rs
  - compiler/mesh-typeck/src/infer.rs
  - compiler/meshc/src/main.rs
  - compiler/mesh-lsp/src/analysis.rs
  - compiler/meshc/tests/e2e_m044_s02.rs
  - .gsd/milestones/M044/slices/S02/tasks/T01-SUMMARY.md
key_decisions:
  - D195: return shared ClusteredExecutionMetadata from manifest validation and retain it alongside prepared MIR in meshc.
  - Use explicit ServiceMethodExport kind metadata instead of re-deriving service call/cast/start roles from generated symbol prefixes in meshc and mesh-lsp.
duration: ""
verification_result: passed
completed_at: 2026-03-29T19:57:23.920Z
blocker_discovered: false
---

# T01: Threaded declared clustered execution metadata through shared manifest validation and into meshc build preparation.

**Threaded declared clustered execution metadata through shared manifest validation and into meshc build preparation.**

## What Happened

Changed the shared clustered-manifest seam so successful validation now returns compiler-owned execution metadata instead of a bare success flag. Added explicit service-method export kind metadata in mesh-typeck, updated meshc and mesh-lsp to build their clustered export surfaces from that richer data, and retained the validated declared-handler plan alongside prepared MIR before codegen. Added a new `e2e_m044_s02` rail covering manifestless local behavior, declared metadata contents, explicit execution-planning failures, and undeclared-target absence while keeping the S01 manifest CLI rail green.

## Verification

Passed the slice-plan verification commands `cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture` and `cargo test -p meshc --test e2e_m044_s01 m044_s01_manifest_ -- --nocapture`. Also ran `cargo test -p mesh-pkg m044_s0 -- --nocapture` and `cargo test -p mesh-lsp m044_s01_clustered_manifest_ -- --nocapture` to confirm the shared validator and editor diagnostics stayed aligned.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture` | 0 | ✅ pass | 8793ms |
| 2 | `cargo test -p meshc --test e2e_m044_s01 m044_s01_manifest_ -- --nocapture` | 0 | ✅ pass | 8728ms |


## Deviations

Touched `compiler/mesh-typeck/src/infer.rs` in addition to the planned files because `ServiceExportInfo` is constructed there; the richer service-method export metadata had to be populated at its source.

## Known Issues

None.

## Files Created/Modified

- `compiler/mesh-pkg/src/manifest.rs`
- `compiler/mesh-typeck/src/lib.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/meshc/src/main.rs`
- `compiler/mesh-lsp/src/analysis.rs`
- `compiler/meshc/tests/e2e_m044_s02.rs`
- `.gsd/milestones/M044/slices/S02/tasks/T01-SUMMARY.md`


## Deviations
Touched `compiler/mesh-typeck/src/infer.rs` in addition to the planned files because `ServiceExportInfo` is constructed there; the richer service-method export metadata had to be populated at its source.

## Known Issues
None.
