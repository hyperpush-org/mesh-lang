---
id: T01
parent: S12
milestone: M034
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-codegen/Cargo.toml", "compiler/mesh-codegen/src/lib.rs", "compiler/mesh-codegen/src/link.rs", "compiler/meshc/tests/e2e_m034_s12.rs", "scripts/verify-m034-s03.ps1", "scripts/tests/verify-m034-s03-installed-build.ps1", ".tmp/m034-s12/t01/diagnostic-summary.json", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M034/slices/S12/tasks/T01-SUMMARY.md"]
key_decisions: ["D109: classify Windows staged-smoke failures through an env-driven `MESH_BUILD_TRACE_PATH` compiler trace instead of relying on opaque exit codes alone.", "Treat the archived S11 hosted crash as `pre-object` until a fresh Windows verifier rerun produces the new trace artifact."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed the existing PowerShell LASTEXITCODE regression, passed the new PowerShell installed-build classifier regression, passed the new `cargo test -p meshc --test e2e_m034_s12 -- --nocapture` target, and passed a slice-level `cargo test -p mesh-codegen link -- --nocapture` run. Verified that `.tmp/m034-s12/t01/diagnostic-summary.json` was written and that it truthfully records the current hosted S11 artifact as `pre-object` pending a fresh Windows rerun with trace capture."
completed_at: 2026-03-27T23:31:03.709Z
blocker_discovered: false
---

# T01: Added compiler build traces and Windows smoke classification regressions.

> Added compiler build traces and Windows smoke classification regressions.

## What Happened
---
id: T01
parent: S12
milestone: M034
key_files:
  - compiler/mesh-codegen/Cargo.toml
  - compiler/mesh-codegen/src/lib.rs
  - compiler/mesh-codegen/src/link.rs
  - compiler/meshc/tests/e2e_m034_s12.rs
  - scripts/verify-m034-s03.ps1
  - scripts/tests/verify-m034-s03-installed-build.ps1
  - .tmp/m034-s12/t01/diagnostic-summary.json
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M034/slices/S12/tasks/T01-SUMMARY.md
key_decisions:
  - D109: classify Windows staged-smoke failures through an env-driven `MESH_BUILD_TRACE_PATH` compiler trace instead of relying on opaque exit codes alone.
  - Treat the archived S11 hosted crash as `pre-object` until a fresh Windows verifier rerun produces the new trace artifact.
duration: ""
verification_result: passed
completed_at: 2026-03-27T23:31:03.712Z
blocker_discovered: false
---

# T01: Added compiler build traces and Windows smoke classification regressions.

**Added compiler build traces and Windows smoke classification regressions.**

## What Happened

Added a compiler-side JSON trace seam for `meshc build` and wired the Windows staged-smoke PowerShell verifier to parse logs, read traces, classify failures, and emit a dedicated diagnostic summary. Added a new `meshc` e2e target that proves the trace on a real smoke build and on a forced missing-runtime path, plus a PowerShell regression that validates hosted-log parsing, malformed-anchor handling, and all classification buckets. The current hosted S11 crash is now preserved in `.tmp/m034-s12/t01/diagnostic-summary.json` as a truthful `pre-object` artifact because the older hosted run has no trace file.

## Verification

Passed the existing PowerShell LASTEXITCODE regression, passed the new PowerShell installed-build classifier regression, passed the new `cargo test -p meshc --test e2e_m034_s12 -- --nocapture` target, and passed a slice-level `cargo test -p mesh-codegen link -- --nocapture` run. Verified that `.tmp/m034-s12/t01/diagnostic-summary.json` was written and that it truthfully records the current hosted S11 artifact as `pre-object` pending a fresh Windows rerun with trace capture.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1` | 0 | ✅ pass | 5788ms |
| 2 | `pwsh -NoProfile -File scripts/tests/verify-m034-s03-installed-build.ps1` | 0 | ✅ pass | 5999ms |
| 3 | `cargo test -p meshc --test e2e_m034_s12 -- --nocapture` | 0 | ✅ pass | 44377ms |
| 4 | `cargo test -p mesh-codegen link -- --nocapture` | 0 | ✅ pass | 38332ms |


## Deviations

Because this host is macOS, the local proof could not execute the staged Windows binaries directly. Instead, the task proved the compiler trace on native builds and used the real S11 hosted crash log plus synthetic PowerShell trace fixtures to classify the Windows boundary truthfully.

## Known Issues

The archived S11 hosted Windows smoke artifact predates `MESH_BUILD_TRACE_PATH`, so `.tmp/m034-s12/t01/diagnostic-summary.json` can only classify that existing failure as `pre-object`. T02 still needs a fresh Windows verifier rerun to determine whether the live blocker is runtime lookup or link-time.

## Files Created/Modified

- `compiler/mesh-codegen/Cargo.toml`
- `compiler/mesh-codegen/src/lib.rs`
- `compiler/mesh-codegen/src/link.rs`
- `compiler/meshc/tests/e2e_m034_s12.rs`
- `scripts/verify-m034-s03.ps1`
- `scripts/tests/verify-m034-s03-installed-build.ps1`
- `.tmp/m034-s12/t01/diagnostic-summary.json`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M034/slices/S12/tasks/T01-SUMMARY.md`


## Deviations
Because this host is macOS, the local proof could not execute the staged Windows binaries directly. Instead, the task proved the compiler trace on native builds and used the real S11 hosted crash log plus synthetic PowerShell trace fixtures to classify the Windows boundary truthfully.

## Known Issues
The archived S11 hosted Windows smoke artifact predates `MESH_BUILD_TRACE_PATH`, so `.tmp/m034-s12/t01/diagnostic-summary.json` can only classify that existing failure as `pre-object`. T02 still needs a fresh Windows verifier rerun to determine whether the live blocker is runtime lookup or link-time.
