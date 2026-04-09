---
estimated_steps: 1
estimated_files: 6
skills_used: []
---

# T02: Implement compiler-known HTTP.clustered typing, metadata handoff, and misuse diagnostics

Make `HTTP.clustered(handler)` and `HTTP.clustered(N, handler)` a real compiler-known wrapper surface. Intercept the calls in typeck, validate arity and handler shape, reject misuse outside route-registration positions, preserve default-versus-explicit replication-count metadata in a structured map parallel to `overloaded_call_targets`, and update diagnostics/LSP coverage if the new wrapper needs dedicated `TypeError` reporting.

## Inputs

- `.gsd/milestones/M047/slices/S03/tasks/T01-SUMMARY.md`
- `D266`
- `D277`
- `D278`

## Expected Output

- `Typeck accepts valid `HTTP.clustered(...)` route wrappers and preserves replication-count metadata for lowering.`
- `Wrapper misuse produces truthful compiler/LSP diagnostics anchored at the wrapper call or handler declaration source.`

## Verification

cargo test -p mesh-typeck m047_s03 -- --nocapture && cargo test -p mesh-lsp -- --nocapture

## Observability Impact

- Signals added/changed: emitted LLVM and declared-handler planning tests should show deterministic clustered route runtime names plus preserved `replication_count` markers.
- How a future agent inspects this: `cargo test -p mesh-codegen m047_s03 -- --nocapture` and any retained LLVM snippets from the new unit rail.
- Failure state exposed: missing synthetic shim registration, name collisions, or count drift fails the build/test surface instead of silently routing locally.
