---
id: T01
parent: S02
milestone: M046
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/src/main.rs", "compiler/mesh-codegen/src/declared.rs", "compiler/mesh-codegen/src/lib.rs", "compiler/mesh-codegen/src/codegen/mod.rs", "compiler/mesh-codegen/src/codegen/intrinsics.rs", "compiler/mesh-rt/src/dist/node.rs", "compiler/meshc/tests/e2e_m046_s02.rs"]
key_decisions: ["D235: Register startup work by declared runtime name and require a matching declared-handler registration during codegen instead of introducing a second executable-identity path."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task-level verification passed via `cargo test -p meshc --test e2e_m046_s02 m046_s02_codegen_ -- --nocapture` and `cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture`. Slice-level snapshot: `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture`, and `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture` all passed. `cargo test -p mesh-rt startup_work_ -- --nocapture` ran 0 tests, so that slice-level runtime rail remains a known coverage gap rather than passing proof."
completed_at: 2026-03-31T16:44:01.177Z
blocker_discovered: false
---

# T01: Threaded work-only startup registrations through meshc/codegen and added LLVM rails for ordered startup hooks.

> Threaded work-only startup registrations through meshc/codegen and added LLVM rails for ordered startup hooks.

## What Happened
---
id: T01
parent: S02
milestone: M046
key_files:
  - compiler/meshc/src/main.rs
  - compiler/mesh-codegen/src/declared.rs
  - compiler/mesh-codegen/src/lib.rs
  - compiler/mesh-codegen/src/codegen/mod.rs
  - compiler/mesh-codegen/src/codegen/intrinsics.rs
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/meshc/tests/e2e_m046_s02.rs
key_decisions:
  - D235: Register startup work by declared runtime name and require a matching declared-handler registration during codegen instead of introducing a second executable-identity path.
duration: ""
verification_result: passed
completed_at: 2026-03-31T16:44:01.180Z
blocker_discovered: false
---

# T01: Threaded work-only startup registrations through meshc/codegen and added LLVM rails for ordered startup hooks.

**Threaded work-only startup registrations through meshc/codegen and added LLVM rails for ordered startup hooks.**

## What Happened

Added a dedicated startup-work registration surface that is derived from the declared-handler plan but filters strictly to clustered `work` entries. `meshc` now threads those registrations through both LLVM and native compile entrypoints, `CodeGen` emits startup-work registration calls after declared-handler registration and triggers startup work after `mesh_main` returns but before scheduler handoff, and codegen now fails explicitly if a startup work item cannot resolve back to an existing declared-handler wrapper. I also declared the new runtime hook ABI, added minimal runtime exports so work-declared builds keep linking, and created a focused `e2e_m046_s02` rail that proves work builds emit startup hooks, service builds do not, manifest/source work share the same runtime identity, and missing startup metadata fails closed. The first stripped-down work fixture hit an unrelated LLVM verifier path, so I aligned the new work fixture with the already-green M044 declared-work shape instead of widening this task into a separate codegen investigation.

## Verification

Task-level verification passed via `cargo test -p meshc --test e2e_m046_s02 m046_s02_codegen_ -- --nocapture` and `cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture`. Slice-level snapshot: `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture`, and `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture` all passed. `cargo test -p mesh-rt startup_work_ -- --nocapture` ran 0 tests, so that slice-level runtime rail remains a known coverage gap rather than passing proof.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m046_s02 m046_s02_codegen_ -- --nocapture` | 0 | ✅ pass | 18800ms |
| 2 | `cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture` | 0 | ✅ pass | 7800ms |
| 3 | `cargo test -p mesh-rt startup_work_ -- --nocapture` | 0 | ❌ fail | 76300ms |
| 4 | `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture` | 0 | ✅ pass | 66200ms |
| 5 | `cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture` | 0 | ✅ pass | 19200ms |
| 6 | `cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture` | 0 | ✅ pass | 14900ms |
| 7 | `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture` | 0 | ✅ pass | 28700ms |


## Deviations

Added minimal runtime ABI exports and startup-registration storage in `compiler/mesh-rt/src/dist/node.rs` so work-declared `meshc build --emit-llvm` paths keep linking before T02 implements the runtime-owned startup submission behavior.

## Known Issues

`cargo test -p mesh-rt startup_work_ -- --nocapture` currently exits 0 while running 0 tests, so there is still no authoritative runtime-owned startup-work verification rail until T02 lands real runtime behavior and tests.

## Files Created/Modified

- `compiler/meshc/src/main.rs`
- `compiler/mesh-codegen/src/declared.rs`
- `compiler/mesh-codegen/src/lib.rs`
- `compiler/mesh-codegen/src/codegen/mod.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/meshc/tests/e2e_m046_s02.rs`


## Deviations
Added minimal runtime ABI exports and startup-registration storage in `compiler/mesh-rt/src/dist/node.rs` so work-declared `meshc build --emit-llvm` paths keep linking before T02 implements the runtime-owned startup submission behavior.

## Known Issues
`cargo test -p mesh-rt startup_work_ -- --nocapture` currently exits 0 while running 0 tests, so there is still no authoritative runtime-owned startup-work verification rail until T02 lands real runtime behavior and tests.
