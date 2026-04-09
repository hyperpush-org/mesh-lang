---
estimated_steps: 4
estimated_files: 6
skills_used: []
---

# T10: Finish declared-work runtime path and cluster-proof rewrite

1. Finish the runtime-owned declared work submit/dispatch seam on top of the repaired registry plumbing.
2. Retarget `cluster-proof` so the new keyed submit hot path calls the runtime-owned declared work boundary instead of `current_target_selection(...)` / direct dispatch helpers.
3. Keep `WorkLegacy` and the old probe path explicit and isolated.
4. Add or repair the named declared-work and cluster-proof e2e rails so the slice verifier can reach the later phases truthfully.

## Inputs

- `.gsd/milestones/M044/slices/S02/tasks/T04-SUMMARY.md`
- `.gsd/milestones/M044/slices/S02/tasks/T07-SUMMARY.md`
- `.gsd/milestones/M044/slices/S02/S02-SUMMARY.md`

## Expected Output

- ``cluster-proof` manifest no longer declares HTTP ingress handlers`
- `new keyed submit path is runtime-owned and leaves artifacts under `.tmp/m044-s02/``

## Verification

bash scripts/verify-m044-s02.sh
