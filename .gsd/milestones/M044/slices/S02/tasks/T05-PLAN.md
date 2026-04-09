---
estimated_steps: 4
estimated_files: 7
skills_used: []
---

# T05: Stopped at the declared-work execution handoff and recorded the actor-wrapper seam before implementation.

1. Thread PreparedBuild.clustered_execution_plan through meshc lowering/codegen instead of dropping it after validation.
2. Add a runtime-owned declared-work registration/dispatch seam that consumes manifest-approved runtime_registration_name/executable_symbol metadata without widening ordinary Node.spawn or undeclared local execution.
3. Lower declared work entrypoints onto that runtime path while keeping undeclared work on the existing local path.
4. Extend compiler/meshc/tests/e2e_m044_s02.rs with m044_s02_declared_work_ coverage for single-node local owner, two-node remote owner, duplicate/conflict stability, and undeclared-local behavior.

## Inputs

- `.gsd/milestones/M044/slices/S02/tasks/T04-SUMMARY.md`
- `compiler/meshc/src/main.rs`
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/meshc/tests/e2e_m044_s02.rs`

## Expected Output

- `PreparedBuild.clustered_execution_plan is consumed by the declared-work lowering/codegen path`
- `Runtime-owned declared work registration/dispatch exists for manifest-declared work only`
- `m044_s02_declared_work_ tests exist and pass in compiler/meshc/tests/e2e_m044_s02.rs`

## Verification

cargo test -p meshc --test e2e_m044_s02 m044_s02_declared_work_ -- --nocapture
cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture
