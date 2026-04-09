---
id: T01
parent: S02
milestone: M047
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/src/main.rs", "compiler/mesh-codegen/src/declared.rs", "compiler/mesh-codegen/src/codegen/mod.rs", "compiler/mesh-codegen/src/codegen/intrinsics.rs", "compiler/mesh-rt/src/dist/node.rs", ".gsd/milestones/M047/slices/S02/tasks/T01-SUMMARY.md"]
key_decisions: ["D274: store replication_count on declared-handler registrations keyed by runtime name while leaving startup-work registration as a name-only list."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task-level verification passed with `cargo test -p mesh-codegen m047_s02 -- --nocapture` and `cargo test -p mesh-rt m047_s02 -- --nocapture`. Slice-level regression replay also kept `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture` green. The new `cargo test -p meshc --test e2e_m047_s02 -- --nocapture` rail still fails because the target does not exist yet; that remains expected T03 work rather than a blocker discovered during T01."
completed_at: 2026-04-01T06:41:05.170Z
blocker_discovered: false
---

# T01: Threaded declared-handler replication counts from meshc planning through LLVM registration into the runtime registry, with new m047_s02 unit coverage.

> Threaded declared-handler replication counts from meshc planning through LLVM registration into the runtime registry, with new m047_s02 unit coverage.

## What Happened
---
id: T01
parent: S02
milestone: M047
key_files:
  - compiler/meshc/src/main.rs
  - compiler/mesh-codegen/src/declared.rs
  - compiler/mesh-codegen/src/codegen/mod.rs
  - compiler/mesh-codegen/src/codegen/intrinsics.rs
  - compiler/mesh-rt/src/dist/node.rs
  - .gsd/milestones/M047/slices/S02/tasks/T01-SUMMARY.md
key_decisions:
  - D274: store replication_count on declared-handler registrations keyed by runtime name while leaving startup-work registration as a name-only list.
duration: ""
verification_result: mixed
completed_at: 2026-04-01T06:41:05.171Z
blocker_discovered: false
---

# T01: Threaded declared-handler replication counts from meshc planning through LLVM registration into the runtime registry, with new m047_s02 unit coverage.

**Threaded declared-handler replication counts from meshc planning through LLVM registration into the runtime registry, with new m047_s02 unit coverage.**

## What Happened

Extended the meshc declared-handler planning seam to carry resolved replication counts into mesh-codegen, widened declared-handler registration metadata and the emitted LLVM/runtime ABI to include the count, and stored the count in the runtime declared-handler registry keyed by runtime name. Added focused m047_s02 unit tests in mesh-codegen and mesh-rt covering default count 2, explicit count preservation, missing lowered-symbol rejection, service-handler startup filtering, LLVM registration markers, and runtime lookup by runtime name.

## Verification

Task-level verification passed with `cargo test -p mesh-codegen m047_s02 -- --nocapture` and `cargo test -p mesh-rt m047_s02 -- --nocapture`. Slice-level regression replay also kept `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture` green. The new `cargo test -p meshc --test e2e_m047_s02 -- --nocapture` rail still fails because the target does not exist yet; that remains expected T03 work rather than a blocker discovered during T01.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-codegen m047_s02 -- --nocapture` | 0 | ✅ pass | 32800ms |
| 2 | `cargo test -p mesh-rt m047_s02 -- --nocapture` | 0 | ✅ pass | 62600ms |
| 3 | `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture` | 0 | ✅ pass | 40500ms |
| 4 | `cargo test -p meshc --test e2e_m047_s02 -- --nocapture` | 101 | ❌ fail | 40400ms |


## Deviations

None.

## Known Issues

`cargo test -p meshc --test e2e_m047_s02 -- --nocapture` still fails with `error: no test target named 'e2e_m047_s02' in 'meshc' package`; the end-to-end proof target is not implemented yet and remains T03 scope.

## Files Created/Modified

- `compiler/meshc/src/main.rs`
- `compiler/mesh-codegen/src/declared.rs`
- `compiler/mesh-codegen/src/codegen/mod.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-rt/src/dist/node.rs`
- `.gsd/milestones/M047/slices/S02/tasks/T01-SUMMARY.md`


## Deviations
None.

## Known Issues
`cargo test -p meshc --test e2e_m047_s02 -- --nocapture` still fails with `error: no test target named 'e2e_m047_s02' in 'meshc' package`; the end-to-end proof target is not implemented yet and remains T03 scope.
