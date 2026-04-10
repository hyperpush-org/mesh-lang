# S03: S03 — UAT

**Milestone:** M057
**Written:** 2026-04-10T20:12:49.503Z

# S03: Realign org project #1 to live repo truth — UAT

**Milestone:** M057  
**Written:** 2026-04-10

## UAT Type

- UAT mode: live GitHub Projects V2 replay plus checked artifact inspection.
- Why this mode is sufficient: this slice's contract is operational rather than UI-facing. Acceptance means the checked plan/results artifacts, the live org project board, and the retained verifier all agree on the same done/active/next truth.

## Preconditions

- Run from the `mesh-lang` repo root.
- `gh` must be authenticated for GitHub project and issue reads. Write access is only needed if you intentionally rerun the idempotence apply check.
- Network access to GitHub must be available.
- The S01 and S02 artifact inputs must still exist under `.gsd/milestones/M057/slices/S01/` and `.gsd/milestones/M057/slices/S02/`.

## Smoke Test

1. Run `bash scripts/verify-m057-s02.sh`.
2. Run `node --test scripts/tests/verify-m057-s03-plan.test.mjs`.
3. Run `python3 scripts/lib/m057_project_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S03 --check`.
4. Run `node --test scripts/tests/verify-m057-s03-results.test.mjs`.
5. Run `bash scripts/verify-m057-s03.sh`.
6. Confirm all commands exit 0.
7. Open `.tmp/m057-s03/verify/phase-report.txt`.
8. **Expected:** status is `ok`, failed phase is `none`, and the phase list shows `repo-precheck`, `artifact-contract`, `live-project-capture`, `project-membership`, `field-coherence`, and `handoff-render` all green.

## Test Cases

### 1. Canonical replacement rows are present and truthful on the live board

1. Open `.gsd/milestones/M057/slices/S03/project-mutation-results.md`.
2. Confirm the canonical mapping table lists:
   - `hyperpush#8 -> mesh-lang#19` with destination board membership `present`
   - `/pitch -> hyperpush#58` with destination board membership `present`
3. Open `.gsd/milestones/M057/slices/S03/project-mutation-results.json`.
4. Confirm `representative_rows.done.issue_handle` is `mesh-lang#19` and `field_values.status.value` is `Done`.
5. Confirm the results JSON includes a row for `hyperpush#58` with `status=Done` and `domain=Hyperpush`.
6. **Expected:** the canonical replacement rows exist on org project #1 under the correct issue identities instead of leaving the board on stale `hyperpush#8` or no `/pitch` row at all.

### 2. Stale Mesh cleanup rows are no longer present on org project #1

1. Open `.gsd/milestones/M057/slices/S03/project-mutation-results.md`.
2. Confirm the removed cleanup list contains exactly:
   - `mesh-lang#3`
   - `mesh-lang#4`
   - `mesh-lang#5`
   - `mesh-lang#6`
   - `mesh-lang#8`
   - `mesh-lang#9`
   - `mesh-lang#10`
   - `mesh-lang#11`
   - `mesh-lang#13`
   - `mesh-lang#14`
3. Run `bash scripts/verify-m057-s03.sh`.
4. **Expected:** the retained verifier's `project-membership` phase confirms those rows are absent, so shipped Mesh cleanup work no longer appears as active roadmap scope.

### 3. The board now shows truthful done / active / next state

1. Open `.gsd/milestones/M057/slices/S03/project-mutation-results.md`.
2. Confirm the final verified board truth reports:
   - total rows = `55`
   - repo counts = `{hyperpush-org/hyperpush: 48, hyperpush-org/mesh-lang: 7}`
   - status counts = `{Done: 2, In Progress: 3, Todo: 50}`
3. Confirm the representative slots are:
   - `Done` → `mesh-lang#19`
   - `Active` → `hyperpush#54`
   - `Next` → `hyperpush#29`
4. **Expected:** the portfolio board reads as truthful current state instead of a stale all-Todo launch plan.

### 4. Active deployment rows keep normalized public naming and explicit live values

1. Inspect the `Naming-normalized active rows` table in `.gsd/milestones/M057/slices/S03/project-mutation-results.md`.
2. Confirm it lists `hyperpush#54`, `hyperpush#55`, and `hyperpush#56`.
3. Confirm their titles refer to the public product repo identity and current operator/marketing/backend reality rather than stale public `hyperpush-mono` wording.
4. Confirm their status remains `In Progress`.
5. **Expected:** S03 preserves truthful current active work while normalizing public naming instead of treating these rows as stale cleanup.

### 5. Metadata inheritance backfills sparse rows without overwriting explicit values

1. Open `.gsd/milestones/M057/slices/S03/project-mutation-results.md`.
2. Inspect the `Inherited metadata spot checks` table.
3. Confirm these rows have the expected inherited tracked values:
   - `hyperpush#29` → `Domain=Hyperpush`, `Track=Core Parity`, `Commitment=Committed`, `Delivery=Shared`, `Priority=P0`
   - `hyperpush#33` → `Track=Operator App`, `Phase=Phase 3 — Operator App`
   - `hyperpush#35` → `Track=SaaS Growth`, `Delivery=SaaS-only`
   - `hyperpush#57` → `Track=Operator App`
4. **Expected:** the board no longer has misleading sparse metadata on representative rows, and inheritance follows the persisted parent chain instead of hand-editing values arbitrarily.

### 6. The apply step is resumable and rerun-safe

1. Run `python3 scripts/lib/m057_project_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S03 --check`.
2. Confirm the command reports `pending.delete=0`, `pending.add=0`, and `pending.update=0`.
3. Run `python3 scripts/lib/m057_project_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S03 --apply`.
4. Confirm the resulting rollup reports `applied=0` and `already_satisfied=35`.
5. **Expected:** rerunning the applicator does not duplicate rows or reapply field mutations; it collapses to steady-state `already_satisfied` outcomes.

### 7. Retained diagnostics are sufficient for future drift debugging

1. List `.tmp/m057-s03/verify/`.
2. Confirm it contains at least:
   - `phase-report.txt`
   - `verification-summary.json`
   - `commands/`
3. Open `phase-report.txt`.
4. Confirm it names the failed phase, drift surface, and last checked target when a phase goes red.
5. **Expected:** a maintainer can debug repo-truth drift, board-membership drift, or field-coherence drift from the retained bundle without reopening slice archaeology.

## Edge Cases

### Upstream repo truth drifts before the board does

1. Treat `bash scripts/verify-m057-s03.sh` as the first replay surface.
2. If `repo-precheck` fails, open `.tmp/m057-s02/verify/phase-report.txt` before touching any S03 artifacts.
3. **Expected:** drift is classified as upstream repo truth first, not misdiagnosed as a board problem.

### Pre-apply plan contract vs post-apply steady state

1. Run `node --test scripts/tests/verify-m057-s03-plan.test.mjs` to lock the checked 63-row pre-apply manifest.
2. Run `python3 scripts/lib/m057_project_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S03 --check` to verify the current 55-row steady state.
3. **Expected:** the plan artifact remains the authoritative historical mutation manifest, while the apply/results/verifier chain is the authoritative final-live replay.

## Failure Signals

- `mesh-lang#19` or `hyperpush#58` is missing from org project #1.
- Any stale cleanup row from the 10-item Mesh list reappears.
- The board rollup drifts away from `55` total rows / `2` done / `3` in progress / `50` todo.
- `hyperpush#54/#55/#56` regress to stale naming or lose their `In Progress` state.
- Inherited tracked metadata on representative rows drifts away from the checked results artifact.
- `.tmp/m057-s03/verify/phase-report.txt` reports a failed phase or drift surface.

## Requirements Proved By This UAT

- R130 — org project #1 now truthfully shows done, active, and next across both repos.
- R134 — a maintainer can understand final board truth from the published results markdown plus the retained verifier bundle.

## Requirements Advanced By This UAT

- R133 — board-level titles and ownership wording preserve public `hyperpush` naming on the representative deployment rows.

## Not Proven By This UAT

- Continuous monitoring; this is an on-demand live replay rather than a scheduled watchdog.
- Any future tracker changes made after the retained verifier was last run.

## Notes for Tester

- Start with `bash scripts/verify-m057-s03.sh`; it is the fastest truthful replay seam because it delegates the S02 repo precheck, re-fetches the board, checks membership and field coherence, and rewrites the handoff markdown in one run.
- Treat `python3 scripts/lib/m057_project_mutation_plan.py --output-dir .gsd/milestones/M057/slices/S03 --check` as a pre-apply baseline validator, not as the final steady-state verifier after the live board has already been reconciled.
- Do not hand-edit generated plan/results artifacts to make counts pass. Repair the planner/applicator/verifier or the live GitHub project state and rerun the checks.
