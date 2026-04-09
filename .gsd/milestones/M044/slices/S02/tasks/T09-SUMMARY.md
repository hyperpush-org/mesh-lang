---
id: T09
parent: S02
milestone: M044
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/src/main.rs", "compiler/mesh-codegen/src/lib.rs", "compiler/mesh-codegen/src/mir/mono.rs", "compiler/mesh-codegen/src/codegen/mod.rs", "compiler/mesh-codegen/src/codegen/intrinsics.rs", "compiler/mesh-codegen/src/declared.rs", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M044/slices/S02/tasks/T09-SUMMARY.md"]
key_decisions: ["Preserve manifest-declared executable symbols as explicit merged-MIR monomorphization roots so locally-unused declared helpers survive until runtime-registration preparation.", "Keep T09 bounded to runtime-name -> existing lowered-symbol registration instead of inventing actor-style work wrappers or truthful clustered service wrappers before the later slice tasks land."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the bounded T09 contract with the `mesh-codegen` library suite, the exact S01 clustered-manifest build regression that the first draft introduced, and the task’s `m044_s02_metadata_` rail. Also replayed the assembled S02 verifier once to confirm the remaining red state is now in out-of-scope `cluster-proof` recovery edits instead of the declared-handler registry seam."
completed_at: 2026-03-29T22:13:07.190Z
blocker_discovered: false
---

# T09: Repaired declared-handler registry plumbing so declared executable symbols survive MIR pruning and the metadata rail passes again.

> Repaired declared-handler registry plumbing so declared executable symbols survive MIR pruning and the metadata rail passes again.

## What Happened
---
id: T09
parent: S02
milestone: M044
key_files:
  - compiler/meshc/src/main.rs
  - compiler/mesh-codegen/src/lib.rs
  - compiler/mesh-codegen/src/mir/mono.rs
  - compiler/mesh-codegen/src/codegen/mod.rs
  - compiler/mesh-codegen/src/codegen/intrinsics.rs
  - compiler/mesh-codegen/src/declared.rs
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M044/slices/S02/tasks/T09-SUMMARY.md
key_decisions:
  - Preserve manifest-declared executable symbols as explicit merged-MIR monomorphization roots so locally-unused declared helpers survive until runtime-registration preparation.
  - Keep T09 bounded to runtime-name -> existing lowered-symbol registration instead of inventing actor-style work wrappers or truthful clustered service wrappers before the later slice tasks land.
duration: ""
verification_result: mixed
completed_at: 2026-03-29T22:13:07.191Z
blocker_discovered: false
---

# T09: Repaired declared-handler registry plumbing so declared executable symbols survive MIR pruning and the metadata rail passes again.

**Repaired declared-handler registry plumbing so declared executable symbols survive MIR pruning and the metadata rail passes again.**

## What Happened

Reproduced the real break in the partial declared-handler refactor, then repaired the `meshc` -> `mesh-codegen` -> runtime registration seam. `mesh-codegen` now re-exports the declared-handler plan/registration types, `CodeGen` stores declared registrations, `compile_mir_to_binary(...)` threads them into LLVM codegen, and the intrinsic table now declares `mesh_register_declared_handler`. Replaying the broader proofs exposed the next real seam: valid manifest-declared helper symbols were being pruned during merged-MIR monomorphization before runtime-registration preparation ran. I fixed that by passing the declared `executable_symbol` list from `compiler/meshc/src/main.rs` into `mesh_codegen::merge_mir_modules(...)` and adding `monomorphize_with_roots(...)` so those declared symbols survive pruning even when the Mesh entrypoint never calls them locally. I then kept T09 bounded to compile-safe registry plumbing by making declared-handler preparation validate and return runtime-name -> existing lowered-symbol registrations instead of assuming the later actor-style work wrapper ABI or truthful clustered service wrapper semantics were already implemented. The minimal T09 proof set is green again, while the slice rail now fails later in out-of-scope `cluster-proof` recovery edits rather than in this registry seam.

## Verification

Verified the bounded T09 contract with the `mesh-codegen` library suite, the exact S01 clustered-manifest build regression that the first draft introduced, and the task’s `m044_s02_metadata_` rail. Also replayed the assembled S02 verifier once to confirm the remaining red state is now in out-of-scope `cluster-proof` recovery edits instead of the declared-handler registry seam.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-codegen --lib --quiet` | 0 | ✅ pass | 20200ms |
| 2 | `cargo test -p meshc --test e2e_m044_s01 m044_s01_manifest_clustered_valid_declarations_build_succeeds -- --nocapture` | 0 | ✅ pass | 49300ms |
| 3 | `cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture` | 0 | ✅ pass | 45300ms |
| 4 | `bash scripts/verify-m044-s02.sh` | 1 | ❌ fail | 190300ms |


## Deviations

Did not continue into the actual declared-work or declared-service execution ABI after the first draft showed those wrapper assumptions were ahead of the current slice state. Kept T09 bounded to compile-safe registry plumbing and left the later execution-surface work to T10/T11.

## Known Issues

`bash scripts/verify-m044-s02.sh` is still red, but the remaining failure is outside T09's bounded compiler/runtime plumbing scope. The retained stop-point is `.tmp/m044-s02/verify/00-s01-contract.log`, which now shows pre-existing `cluster-proof/work_continuity.mpl` and `cluster-proof/main.mpl` compile/type errors rather than another declared-handler registry failure. Later tasks still need the real declared-work runtime path and truthful declared-service wrapper path.

## Files Created/Modified

- `compiler/meshc/src/main.rs`
- `compiler/mesh-codegen/src/lib.rs`
- `compiler/mesh-codegen/src/mir/mono.rs`
- `compiler/mesh-codegen/src/codegen/mod.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-codegen/src/declared.rs`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M044/slices/S02/tasks/T09-SUMMARY.md`


## Deviations
Did not continue into the actual declared-work or declared-service execution ABI after the first draft showed those wrapper assumptions were ahead of the current slice state. Kept T09 bounded to compile-safe registry plumbing and left the later execution-surface work to T10/T11.

## Known Issues
`bash scripts/verify-m044-s02.sh` is still red, but the remaining failure is outside T09's bounded compiler/runtime plumbing scope. The retained stop-point is `.tmp/m044-s02/verify/00-s01-contract.log`, which now shows pre-existing `cluster-proof/work_continuity.mpl` and `cluster-proof/main.mpl` compile/type errors rather than another declared-handler registry failure. Later tasks still need the real declared-work runtime path and truthful declared-service wrapper path.
