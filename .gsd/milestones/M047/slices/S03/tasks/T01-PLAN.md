---
estimated_steps: 1
estimated_files: 5
skills_used: []
---

# T01: Stopped under context-budget pressure after mapping the compiler seam for HTTP.clustered wrapper typing; no source changes shipped in this unit.

Establish `HTTP.clustered(...)` as a compiler-known HTTP wrapper surface instead of pretending it is an ordinary closure helper. The task should add the `HTTP` module typing/inference contract, preserve default-versus-explicit replication-count metadata for downstream lowering, and emit truthful misuse diagnostics when the wrapper is used with the wrong handler shape or outside route registration.

## Inputs

- ``compiler/mesh-typeck/src/builtins.rs``
- ``compiler/mesh-typeck/src/infer.rs``
- ``compiler/mesh-codegen/src/mir/lower.rs``

## Expected Output

- ``compiler/mesh-typeck/src/builtins.rs``
- ``compiler/mesh-typeck/src/infer.rs``
- ``compiler/mesh-codegen/src/mir/lower.rs``

## Verification

cargo test -p mesh-typeck m047_s03 -- --nocapture

## Observability Impact

- Signals added/changed: clustered-wrapper type errors should point at the `HTTP.clustered(...)` call site instead of collapsing into a generic function-pointer mismatch.
- How a future agent inspects this: `cargo test -p mesh-typeck m047_s03 -- --nocapture` plus any new misuse diagnostics in the compiler output.
- Failure state exposed: invalid handler shape, invalid count form, or non-route use of the wrapper becomes explicit and source-local.
