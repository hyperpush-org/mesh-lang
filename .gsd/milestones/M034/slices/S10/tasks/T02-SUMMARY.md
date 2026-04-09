---
id: T02
parent: S10
milestone: M034
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-codegen/src/link.rs", "compiler/mesh-codegen/src/lib.rs", "scripts/verify-m034-s03.ps1", "scripts/tests/verify-m034-s03-last-exitcode.ps1", "scripts/verify-m034-s02-workflows.sh", "compiler/mesh-rt/src/lib.rs", ".gsd/DECISIONS.md", ".gsd/KNOWLEDGE.md"]
key_decisions: ["D105: use target-aware linker selection and pass the resolved runtime static library path directly to the linker instead of assuming Unix `cc -lmesh_rt` semantics on every host."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the task’s required verification commands directly. `cargo test -p mesh-codegen link -- --nocapture` passed with the new link-path regression coverage in place. `pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1` passed and confirmed the PowerShell helper now preserves exit-code and artifact-path metadata in the combined log. `bash scripts/verify-m034-s02-workflows.sh` passed, confirming the release workflow contract still points at the real staged smoke steps after the wording update."
completed_at: 2026-03-27T20:31:39.042Z
blocker_discovered: false
---

# T02: Made meshc’s linker/runtime discovery target-aware for Windows MSVC and updated staged smoke logs so hosted Windows failures keep a truthful phase log.

> Made meshc’s linker/runtime discovery target-aware for Windows MSVC and updated staged smoke logs so hosted Windows failures keep a truthful phase log.

## What Happened
---
id: T02
parent: S10
milestone: M034
key_files:
  - compiler/mesh-codegen/src/link.rs
  - compiler/mesh-codegen/src/lib.rs
  - scripts/verify-m034-s03.ps1
  - scripts/tests/verify-m034-s03-last-exitcode.ps1
  - scripts/verify-m034-s02-workflows.sh
  - compiler/mesh-rt/src/lib.rs
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - D105: use target-aware linker selection and pass the resolved runtime static library path directly to the linker instead of assuming Unix `cc -lmesh_rt` semantics on every host.
duration: ""
verification_result: passed
completed_at: 2026-03-27T20:31:39.043Z
blocker_discovered: false
---

# T02: Made meshc’s linker/runtime discovery target-aware for Windows MSVC and updated staged smoke logs so hosted Windows failures keep a truthful phase log.

**Made meshc’s linker/runtime discovery target-aware for Windows MSVC and updated staged smoke logs so hosted Windows failures keep a truthful phase log.**

## What Happened

I refactored `compiler/mesh-codegen/src/link.rs` around a small target classifier so the linker path no longer assumes every host behaves like Unix. The linker now receives the requested target triple, keeps `cc` plus direct `libmesh_rt.a` linking on Unix-like targets, and switches Windows MSVC to `clang(.exe)` plus direct `mesh_rt.lib` linking. Runtime lookup is now target-aware as well: when a target triple is provided it searches the target-specific Cargo layout first, falls back to the host profile layout, and emits explicit errors that name the expected runtime filename, the target triple, the searched paths, and the cargo command needed to rebuild `mesh-rt`.

I threaded the target triple through `compiler/mesh-codegen/src/lib.rs` so the linker repair is actually used by `meshc build` and multi-module builds, added focused unit coverage in `link.rs` for Windows filename selection, Unix filename preservation, unsupported target rejection, and missing-runtime error text, and updated the runtime crate doc comment so the platform-specific artifact names are no longer described as Unix-only.

On the staged-smoke side, I kept the workflow shape intact but tightened the PowerShell command-log helper in `scripts/verify-m034-s03.ps1`: every command log now records the display text, exit code, and stdout/stderr artifact paths even when the underlying subprocess is silent. I extended `scripts/tests/verify-m034-s03-last-exitcode.ps1` to assert those new fields, adjusted the workflow contract wording in `scripts/verify-m034-s02-workflows.sh`, and recorded the linking rule in `.gsd/DECISIONS.md` and `.gsd/KNOWLEDGE.md`.

## Verification

Ran the task’s required verification commands directly. `cargo test -p mesh-codegen link -- --nocapture` passed with the new link-path regression coverage in place. `pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1` passed and confirmed the PowerShell helper now preserves exit-code and artifact-path metadata in the combined log. `bash scripts/verify-m034-s02-workflows.sh` passed, confirming the release workflow contract still points at the real staged smoke steps after the wording update.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-codegen link -- --nocapture` | 0 | ✅ pass | 1882ms |
| 2 | `pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1` | 0 | ✅ pass | 2883ms |
| 3 | `bash scripts/verify-m034-s02-workflows.sh` | 0 | ✅ pass | 1255ms |


## Deviations

None.

## Known Issues

`cargo test -p mesh-codegen link -- --nocapture` still reports two pre-existing warnings in `compiler/mesh-codegen/src/mir/lower.rs` (`unused import: crate::mir::MirType` and an always-true `len() >= 0` comparison). They are unrelated to this linker task and remained unchanged.

## Files Created/Modified

- `compiler/mesh-codegen/src/link.rs`
- `compiler/mesh-codegen/src/lib.rs`
- `scripts/verify-m034-s03.ps1`
- `scripts/tests/verify-m034-s03-last-exitcode.ps1`
- `scripts/verify-m034-s02-workflows.sh`
- `compiler/mesh-rt/src/lib.rs`
- `.gsd/DECISIONS.md`
- `.gsd/KNOWLEDGE.md`


## Deviations
None.

## Known Issues
`cargo test -p mesh-codegen link -- --nocapture` still reports two pre-existing warnings in `compiler/mesh-codegen/src/mir/lower.rs` (`unused import: crate::mir::MirType` and an always-true `len() >= 0` comparison). They are unrelated to this linker task and remained unchanged.
