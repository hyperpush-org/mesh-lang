---
estimated_steps: 4
estimated_files: 6
skills_used: []
---

# T09: Repair and land declared-handler registry plumbing

1. Repair the partial `mesh-codegen` refactor so the workspace compiles again.
2. Thread `PreparedBuild.clustered_execution_plan` into a clean declared-handler preparation step that returns runtime registrations instead of being dropped.
3. Keep the work bounded to `meshc`/`mesh-codegen`/`mesh-rt` plumbing first; do not touch cluster-proof again until the compile path is green.
4. Verify with the metadata rail before moving on.

## Inputs

- `.gsd/milestones/M044/slices/S02/tasks/T03-SUMMARY.md`
- `.gsd/milestones/M044/slices/S02/tasks/T05-SUMMARY.md`
- `.gsd/milestones/M044/slices/S02/S02-SUMMARY.md`

## Expected Output

- `compiler/runtime declared-handler preparation compiles cleanly`
- ``m044_s02_metadata_` passes again`

## Verification

cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture
