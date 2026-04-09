---
estimated_steps: 5
estimated_files: 8
skills_used: []
---

# T04: Restore the missing typed Continuity builtin seam and retire the stringly public contract

Re-do the unlanded T02 work explicitly instead of pretending it already exists.

1. Register the typed `Continuity` structs and function signatures in both `compiler/mesh-typeck/src/infer.rs` and `compiler/mesh-typeck/src/builtins.rs`, following the existing builtin-struct path rather than leaving the Mesh surface at `String ! String`.
2. Mirror those typed shapes through `compiler/mesh-codegen/src/mir/lower.rs` and `compiler/mesh-codegen/src/codegen/intrinsics.rs` so field access, lowering, and runtime ABI all agree on layout and result payloads.
3. Change `compiler/mesh-rt/src/dist/continuity.rs` and `compiler/mesh-rt/src/lib.rs` to return typed Mesh payloads directly from the exported continuity functions instead of JSON strings.
4. Rewrite the stale stringly public-contract coverage in `compiler/meshc/tests/e2e_m044_s01.rs` and `compiler/meshc/tests/e2e_m043_s02.rs` so compiler/runtime truth no longer depends on app-side JSON decoding.

## Inputs

- `.gsd/milestones/M044/slices/S01/tasks/T03-SUMMARY.md`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-typeck/src/builtins.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/lib.rs`
- `compiler/meshc/tests/e2e_m043_s02.rs`

## Expected Output

- `Typed `Continuity.*` builtin structs/signatures registered across typeck and MIR/codegen`
- `Runtime continuity exports returning typed Mesh payloads instead of JSON strings`
- `Updated compiler e2e coverage proving typed happy-path and compile-fail continuity usage`

## Verification

cargo test -p meshc --test e2e_m044_s01 typed_continuity_ -- --nocapture
cargo test -p meshc --test e2e_m044_s01 continuity_compile_fail_ -- --nocapture

## Observability Impact

- Signals added/changed: phase/status markers and per-phase logs under `.tmp/m044-s01/verify/`.
- How a future agent inspects this: rerun `bash scripts/verify-m044-s01.sh` and inspect the retained manifest/compiler/LSP/cluster-proof phase logs.
- Failure state exposed: which phase failed, whether the failure was a 0-test filter or real contract regression, and which stale helper/command drift triggered it.
