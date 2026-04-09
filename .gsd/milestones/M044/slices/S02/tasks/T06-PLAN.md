---
estimated_steps: 4
estimated_files: 7
skills_used: []
---

# T06: Stopped at the declared service wrapper handoff and recorded that remote service replies already use mesh_actor_send, so T06 still needs wrapper registration and lowering rather than a second reply transport.

1. Turn declared service_call/service_cast targets into runtime-executable wrapper metadata instead of relying on the generic service helper surface by accident.
2. Register only manifest-declared service wrappers with the runtime declared-handler registry and lower clustered service call/cast through that path.
3. Preserve ordinary local start helpers and undeclared service methods exactly as local Mesh code.
4. Extend compiler/meshc/tests/e2e_m044_s02.rs with m044_s02_service_ coverage for declared remote call/cast behavior and undeclared local service behavior.

## Inputs

- `.gsd/milestones/M044/slices/S02/tasks/T03-SUMMARY.md`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-rt/src/dist/node.rs`

## Expected Output

- `Declared service call/cast targets resolve to runtime-executable wrapper registrations`
- `Undeclared service methods and start helpers stay on the local path`
- `m044_s02_service_ tests exist and pass in compiler/meshc/tests/e2e_m044_s02.rs`

## Verification

cargo test -p meshc --test e2e_m044_s02 m044_s02_service_ -- --nocapture
