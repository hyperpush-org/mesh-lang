---
estimated_steps: 1
estimated_files: 5
skills_used: []
---

# T02: Expose a Mesh-facing Continuity API through typechecking and codegen

Add a dedicated `Continuity` Mesh module backed by new runtime intrinsics so Mesh code can submit keyed work, read truthful keyed status, and mark healthy-path completions without talking directly to runtime internals. Keep the API small and continuity-specific rather than overloading `Node` or `Global` with workload semantics.

## Inputs

- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-codegen/src/codegen/expr.rs`
- `compiler/mesh-rt/src/dist/continuity.rs`

## Expected Output

- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-codegen/src/codegen/expr.rs`
- `compiler/mesh-rt/src/lib.rs`
- `compiler/meshc/tests/e2e_m042_s01.rs`

## Verification

cargo test -p meshc --test e2e_m042_s01 continuity_api -- --nocapture

## Observability Impact

Preserves continuity failure reasons and status fields through the Mesh-facing API so compiled programs do not have to infer runtime state from opaque integers or missing fields.
