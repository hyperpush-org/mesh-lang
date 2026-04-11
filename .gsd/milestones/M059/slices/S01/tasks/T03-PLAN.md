---
estimated_steps: 1
estimated_files: 3
skills_used: []
---

# T03: Verify shell parity and command contract

Exercise the in-place migrated shell enough to prove the command contract and visible dashboard shell survived the framework swap groundwork. Record the exact seams that remain for route decomposition in S02 instead of widening S01 into route restructuring.

## Inputs

- `S01 migrated app tree`
- `M059 success criteria`

## Expected Output

- `.gsd/milestones/M059/slices/S01/tasks/T03-PLAN.md`
- `Verified command/build evidence for the in-place TanStack shell`

## Verification

Run `npm run dev`, `npm run build`, and a targeted browser/smoke check against the in-place app and confirm no backend integration drift was introduced.
