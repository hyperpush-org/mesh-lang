---
estimated_steps: 4
estimated_files: 2
skills_used:
  - rust-testing
---

# T04: Run the assembled closeout rail and render M046 validation from it

Finish the slice by proving the final S06 rail is green, then turn that evidence into the milestone validation artifact instead of claiming closeout from planning alone.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m046-s06.sh` full replay | Stop on the first failed phase and inspect `.tmp/m046-s06/verify/` instead of papering over a red proof surface. | Keep the bounded S06 timeout; do not mark the milestone validated if the assembled proof never converges. | Treat missing phase logs, stale bundle pointers, or absent S03/S04/S05/S06 retained artifacts as blocker-level failures. |
| Checked-in requirements and slice summaries | Fail validation if the evidence chain does not directly cover R086, R087, R088, R089, R090, R091, R092, and R093. | N/A | Treat requirements/summary mismatches as validation gaps, not wording issues to hand-wave away. |
| `gsd_validate_milestone` output | Do not claim milestone pass if the tool call or rendered markdown fails. | N/A | Treat a malformed or incomplete validation artifact as unfinished closeout. |

## Load Profile

- **Shared resources**: the full S06 replay, nested retained proof bundles, and the checked-in milestone docs/state files.
- **Per-operation cost**: one full assembled verifier run plus one milestone-validation render grounded in the resulting evidence.
- **10x breakpoint**: the verifier replay dominates cost; validation writing itself is cheap once the evidence is green.

## Negative Tests

- **Malformed inputs**: missing retained bundle pointer, stale S05-only authority claims, or incomplete requirement evidence for docs/verification closeout.
- **Error paths**: red verifier status, missing `phase-report.txt`, missing S06 bundle members, or validation content that cannot explain how the active M046 requirements were re-proved.
- **Boundary conditions**: historical wrapper rails may still exist, but milestone validation must cite the green S06 rail as the final truthful closeout surface.

## Steps

1. Run `bash scripts/verify-m046-s06.sh` and inspect `.tmp/m046-s06/verify/status.txt`, `current-phase.txt`, `phase-report.txt`, and `latest-proof-bundle.txt` to confirm the assembled S03/S04/S05/S06 evidence chain is complete.
2. Update `.gsd/PROJECT.md` so the current project state reflects that S06 is the final M046 closeout rail and that milestone validation is now grounded in the S06 bundle.
3. Use the green S06 evidence, the checked-in requirements, the M046 roadmap, and the completed S03/S04/S05 summaries to populate the M046 success-criteria checklist, slice delivery audit, cross-slice integration, and requirement coverage with current-state proof instead of planning intent.
4. Call `gsd_validate_milestone` for `M046` only after the rail is green, writing `.gsd/milestones/M046/M046-VALIDATION.md` with a `pass` verdict tied directly to the S06 evidence chain.

## Must-Haves

- [ ] The full S06 verifier is green before milestone validation is recorded.
- [ ] `.gsd/PROJECT.md` names S06 as the final M046 closeout rail and points future agents at the S06 retained bundle.
- [ ] `.gsd/milestones/M046/M046-VALIDATION.md` exists and explicitly covers R086, R087, R088, R089, R090, R091, R092, and R093.
- [ ] The validation artifact cites the S06 rail as the final authoritative proof surface instead of S05.

## Done When

- [ ] `bash scripts/verify-m046-s06.sh` passes and leaves a non-empty retained bundle pointer.
- [ ] The M046 validation artifact is rendered and matches the green S06 evidence chain.

## Inputs

- `scripts/verify-m046-s06.sh`
- `.gsd/PROJECT.md`
- `.gsd/REQUIREMENTS.md`
- `.gsd/milestones/M046/M046-ROADMAP.md`
- `.gsd/milestones/M046/slices/S03/S03-SUMMARY.md`
- `.gsd/milestones/M046/slices/S04/S04-SUMMARY.md`
- `.gsd/milestones/M046/slices/S05/S05-SUMMARY.md`
- `.gsd/milestones/M045/M045-VALIDATION.md`

## Expected Output

- `.gsd/PROJECT.md`
- `.gsd/milestones/M046/M046-VALIDATION.md`

## Verification

bash scripts/verify-m046-s06.sh && test -s .gsd/milestones/M046/M046-VALIDATION.md && rg -n "verify-m046-s06|R086|R091|R092" .gsd/milestones/M046/M046-VALIDATION.md .gsd/PROJECT.md

## Observability Impact

Uses `.tmp/m046-s06/verify/status.txt`, `current-phase.txt`, `phase-report.txt`, and `latest-proof-bundle.txt` as the milestone-closeout health surface and records the final evidence route in `.gsd/PROJECT.md` / `.gsd/milestones/M046/M046-VALIDATION.md` so future agents know exactly where to inspect a red or historical closeout.
