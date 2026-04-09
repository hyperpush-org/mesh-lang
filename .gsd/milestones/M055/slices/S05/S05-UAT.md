# S05: Validation Remediation: Contract Truth & Two-Repo Evidence Closure — UAT

**Milestone:** M055
**Written:** 2026-04-08T04:04:43.155Z

## Preconditions

- Repo root is the working directory.
- The supported GSD closeout path is available (`gsd_slice_complete`, `gsd_complete_milestone`).

## Test Cases

### TC-01 — S05 no longer uses the placeholder slice summary
1. Open `.gsd/milestones/M055/slices/S05/S05-SUMMARY.md`.
2. Confirm it is not the auto-mode placeholder beginning with `# BLOCKER — auto-mode recovery failed`.
3. Expected: the file is a real slice summary describing the remediation-closeout work.

### TC-02 — S01 wrapper state is green
1. Read `.tmp/m055-s01/verify/status.txt`.
2. Read `.tmp/m055-s01/verify/current-phase.txt`.
3. Read `.tmp/m055-s01/verify/phase-report.txt`.
4. Expected: `status.txt` is `ok`, `current-phase.txt` is `complete`, and every phase in `phase-report.txt` ends in `passed`.

### TC-03 — S03 wrapper state is green
1. Read `.tmp/m055-s03/verify/status.txt`.
2. Read `.tmp/m055-s03/verify/current-phase.txt`.
3. Expected: both read `ok` / `complete`.

### TC-04 — Milestone validation remains pass
1. Open `.gsd/milestones/M055/M055-VALIDATION.md`.
2. Expected: frontmatter contains `verdict: pass` and `remediation_round: 1`.

### TC-05 — Roadmap marks S05 complete after slice completion
1. Open `.gsd/milestones/M055/M055-ROADMAP.md`.
2. Expected: the S05 row is marked complete.

### Notes

- The current top-level `.tmp/m055-s04/verify/` tree is not the authoritative closeout seam. Use the durable slice/milestone artifacts plus the live S01/S03 wrapper state when checking that the validation loop is resolved.
