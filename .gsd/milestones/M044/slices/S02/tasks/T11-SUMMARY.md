---
id: T11
parent: S02
milestone: M044
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-codegen/src/declared.rs", "compiler/meshc/tests/e2e_m044_s02.rs", "cluster-proof/work_continuity.mpl", "cluster-proof/main.mpl", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M044/slices/S02/tasks/T11-SUMMARY.md"]
key_decisions: ["Generate distinct `__declared_service_*` wrapper symbols that delegate to the existing `__service_*` call/cast helpers instead of registering raw `__service_*` helpers as declared runtime executables.", "Treat `meshc build --emit-llvm` as a pre-registration snapshot for M044/S02 proof work; wrapper definitions appear there, but startup `mesh_register_declared_handler(...)` calls do not."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the new wrapper generation narrowly in `mesh-codegen`, replayed the named S02 service rail to the first honest failure, and rechecked the `cluster-proof` build/package-test surfaces after the app-seam fixes. I did not re-run `bash scripts/verify-m044-s02.sh` because the named `m044_s02_service_` filter is still red and the context-budget stop arrived at that point."
completed_at: 2026-03-29T22:36:40.042Z
blocker_discovered: false
---

# T11: Stopped after landing declared-service wrapper generation and `cluster-proof` compile fixes; the named service rail still fails because the registration assertion is checking the wrong emitted LLVM surface.

> Stopped after landing declared-service wrapper generation and `cluster-proof` compile fixes; the named service rail still fails because the registration assertion is checking the wrong emitted LLVM surface.

## What Happened
---
id: T11
parent: S02
milestone: M044
key_files:
  - compiler/mesh-codegen/src/declared.rs
  - compiler/meshc/tests/e2e_m044_s02.rs
  - cluster-proof/work_continuity.mpl
  - cluster-proof/main.mpl
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M044/slices/S02/tasks/T11-SUMMARY.md
key_decisions:
  - Generate distinct `__declared_service_*` wrapper symbols that delegate to the existing `__service_*` call/cast helpers instead of registering raw `__service_*` helpers as declared runtime executables.
  - Treat `meshc build --emit-llvm` as a pre-registration snapshot for M044/S02 proof work; wrapper definitions appear there, but startup `mesh_register_declared_handler(...)` calls do not.
duration: ""
verification_result: mixed
completed_at: 2026-03-29T22:36:40.044Z
blocker_discovered: false
---

# T11: Stopped after landing declared-service wrapper generation and `cluster-proof` compile fixes; the named service rail still fails because the registration assertion is checking the wrong emitted LLVM surface.

**Stopped after landing declared-service wrapper generation and `cluster-proof` compile fixes; the named service rail still fails because the registration assertion is checking the wrong emitted LLVM surface.**

## What Happened

I repaired the proof-app seam first by adding a real declared-work target in `cluster-proof/work_continuity.mpl` and rewriting `cluster-proof/main.mpl`’s HTTP startup block into a parser-safe form; after that both `meshc build cluster-proof` and `meshc test cluster-proof/tests` were green again. On the compiler side, `compiler/mesh-codegen/src/declared.rs` now generates distinct `__declared_service_call_*` / `__declared_service_cast_*` thunk functions that delegate to the existing `__service_*` helpers, and `compiler/meshc/tests/e2e_m044_s02.rs` now carries metadata, declared-work, service-wrapper, and cluster-proof proof rails. The remaining failure is in the new service proof itself: `meshc build --emit-llvm` only shows the pre-registration MIR snapshot, so the test that expects startup `mesh_register_declared_handler(...)` calls in that file is checking the wrong surface. I recorded that resume rule in `.gsd/KNOWLEDGE.md` and stopped at that exact boundary when the context-budget warning arrived.

## Verification

Verified the new wrapper generation narrowly in `mesh-codegen`, replayed the named S02 service rail to the first honest failure, and rechecked the `cluster-proof` build/package-test surfaces after the app-seam fixes. I did not re-run `bash scripts/verify-m044-s02.sh` because the named `m044_s02_service_` filter is still red and the context-budget stop arrived at that point.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-codegen declared_service_handlers --lib -- --nocapture` | 0 | ✅ pass | 0ms |
| 2 | `cargo test -p meshc --test e2e_m044_s02 m044_s02_service_ -- --nocapture` | 101 | ❌ fail | 0ms |
| 3 | `cargo run -q -p meshc -- build cluster-proof` | 0 | ✅ pass | 0ms |
| 4 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 0ms |


## Deviations

Did not reach the planned full `bash scripts/verify-m044-s02.sh` replay. The unit stopped at the first honest `m044_s02_service_` failure when the context-budget warning arrived, so the result is a precise partial handoff rather than a green closeout.

## Known Issues

`cargo test -p meshc --test e2e_m044_s02 m044_s02_service_ -- --nocapture` is still failing in `m044_s02_service_llvm_registers_declared_wrappers_without_widening_manifestless_builds` because the test is asserting declared-handler registration on the wrong emitted LLVM surface. `bash scripts/verify-m044-s02.sh` was not replayed after that failure.

## Files Created/Modified

- `compiler/mesh-codegen/src/declared.rs`
- `compiler/meshc/tests/e2e_m044_s02.rs`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/main.mpl`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M044/slices/S02/tasks/T11-SUMMARY.md`


## Deviations
Did not reach the planned full `bash scripts/verify-m044-s02.sh` replay. The unit stopped at the first honest `m044_s02_service_` failure when the context-budget warning arrived, so the result is a precise partial handoff rather than a green closeout.

## Known Issues
`cargo test -p meshc --test e2e_m044_s02 m044_s02_service_ -- --nocapture` is still failing in `m044_s02_service_llvm_registers_declared_wrappers_without_widening_manifestless_builds` because the test is asserting declared-handler registration on the wrong emitted LLVM surface. `bash scripts/verify-m044-s02.sh` was not replayed after that failure.
