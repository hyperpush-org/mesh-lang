---
estimated_steps: 4
estimated_files: 5
skills_used: []
---

# T11: Add truthful declared-service proof coverage

1. Land the declared service wrapper surface after the declared-work/runtime plumbing is green.
2. Keep the scope honest: prove wrapper generation/registration and a truthful named `m044_s02_service_` rail instead of inventing a second reply transport.
3. Update the slice verifier only if the proof surface changes; otherwise make the new named service rail satisfy the existing contract.
4. Re-run the full S02 verifier after the service rail exists.

## Inputs

- `.gsd/milestones/M044/slices/S02/tasks/T03-SUMMARY.md`
- `.gsd/milestones/M044/slices/S02/tasks/T06-SUMMARY.md`
- `.gsd/milestones/M044/slices/S02/S02-SUMMARY.md`

## Expected Output

- `truthful `m044_s02_service_` coverage`
- `assembled S02 rail reaches completion`

## Verification

bash scripts/verify-m044-s02.sh
