# S03: Realign org project #1 to the reconciled issue truth

**Goal:** Realign org project #1 from current live repo truth so the board shows truthful done/active/next state across `mesh-lang` and `hyperpush`, preserves the S02 canonical issue mappings, and backfills missing tracked metadata without hand-editing rows.
**Demo:** After this, org project #1 accurately shows what is done, active, and next across both repos instead of reading as a stale all-Todo launch plan.

## Must-Haves

- Record the retained S02 verifier outcome inside the S03 plan artifact and derive board mutations from current live repo truth plus persisted canonical mappings instead of replaying stale S01 board text.
- Remove stale cleanup rows, handle the canonical `mesh-lang#19` / `hyperpush#58` mappings truthfully, and keep `hyperpush#54/#55/#56` normalized to the public `hyperpush` repo identity.
- Fill missing `Domain`, `Track`, `Delivery Mode`, and deeper tracked metadata through deterministic parent-chain inheritance while preserving explicit live values that are already correct.
- Publish checked plan/results artifacts and a retained verifier so a maintainer can replay board truth and understand done/active/next without reopening `.gsd` archaeology.

## Threat Surface

- **Abuse**: a bad mutation manifest or stale item id could delete the wrong board row, rewrite truthful project metadata, or repoint a row to the wrong canonical issue.
- **Data exposure**: only public issue/project metadata, canonical URLs, field values, and verification diagnostics may be written to artifacts; `gh` auth state, tokens, and local config must never be persisted.
- **Input trust**: live GitHub project rows, live repo issue state, S01 ledger rows, S02 canonical mapping artifacts, project field ids, and inherited parent-chain metadata are all untrusted until validated against expected counts and canonical issue identities.

## Requirement Impact

- **Requirements touched**: `R130`, `R131`, `R132`, `R133`, `R134`
- **Re-verify**: project membership for the stale cleanup rows, canonical handling of `mesh-lang#19` / `hyperpush#58`, representative `Done` / `In Progress` / `Todo` board rows, inherited tracked metadata on parent-chain descendants, and public naming normalization on `hyperpush#54/#55/#56`.
- **Decisions revisited**: `D451`, `D452`, `D453`, `D458`, `D459`, `D460`

## Proof Level

- This slice proves: final-assembly
- Real runtime required: yes
- Human/UAT required: no

## Verification

- `node --test scripts/tests/verify-m057-s03-plan.test.mjs`
- `node --test scripts/tests/verify-m057-s03-results.test.mjs`
- `bash scripts/verify-m057-s03.sh`

## Observability / Diagnostics

- Runtime signals: S03 plan/results artifacts must record repo-precheck verdict, planned/applied project operations, canonical old→new issue identity handling, inherited field sources, and final row state.
- Inspection surfaces: `.gsd/milestones/M057/slices/S03/project-mutation-plan.json`, `.gsd/milestones/M057/slices/S03/project-mutation-plan.md`, `.gsd/milestones/M057/slices/S03/project-mutation-results.json`, `.gsd/milestones/M057/slices/S03/project-mutation-results.md`, and `.tmp/m057-s03/verify/*`.
- Failure visibility: the retained verifier must expose the failed phase, last attempted item or issue handle, command stderr/stdout capture, and whether drift came from repo truth, board membership, canonical mapping, naming normalization, or inherited-field coherence.
- Redaction constraints: never persist auth headers, tokens, or non-public local config; only public issue/project metadata and command summaries may enter artifacts.

## Integration Closure

- Upstream surfaces consumed: `.gsd/milestones/M057/slices/S01/reconciliation-ledger.json`, `.gsd/milestones/M057/slices/S01/project-fields.snapshot.json`, `.gsd/milestones/M057/slices/S01/project-items.snapshot.json`, `.gsd/milestones/M057/slices/S02/repo-mutation-results.json`, `scripts/lib/m057_tracker_inventory.py`, `scripts/lib/m057_project_items.graphql`, and `scripts/verify-m057-s02.sh`.
- New wiring introduced in this slice: a live-truth project mutation planner, a plan-driven project applicator, deterministic parent-chain field inheritance, a retained board verifier, and final handoff artifacts.
- What remains before the milestone is truly usable end-to-end: nothing.

## Tasks

- [x] **T01: Generate a live-truth board mutation manifest with repo-drift precheck and field inheritance** `est:2h`
  - Why: Repo/project drift makes ad hoc board edits unsafe; the slice needs one checked manifest that records current repo truth, canonical identity changes, and inherited field values before anything mutates org project #1.
  - Files: `scripts/lib/m057_project_mutation_plan.py`, `scripts/tests/verify-m057-s03-plan.test.mjs`, `.gsd/milestones/M057/slices/S03/project-mutation-plan.json`, `.gsd/milestones/M057/slices/S03/project-mutation-plan.md`
  - Do: Add a planner that runs `bash scripts/verify-m057-s02.sh` as an S03 preflight, captures/normalizes the live board with the existing inventory helpers, joins live project rows to the S01 ledger plus S02 canonical results, derives delete/add/update operations, and resolves deterministic parent-chain inheritance for incomplete tracked metadata. Use live issue titles/URLs and the S02 canonical mappings instead of replaying stale S01 board titles.
  - Verify: `python3 scripts/lib/m057_project_mutation_plan.py --output-dir .gsd/milestones/M057/slices/S03 --check && node --test scripts/tests/verify-m057-s03-plan.test.mjs`
  - Done when: the plan artifacts exist, the repo-truth precheck outcome is recorded, the stale cleanup rows and canonical replacement rows are explicitly accounted for, and the 23 incomplete metadata rows have deterministic inheritance sources.
- [ ] **T02: Apply project deletes, adds, and field edits from the checked manifest** `est:2h`
  - Why: This is the slice’s real external effect: org project #1 must stop reflecting stale launch-plan rows and instead match the reconciled issue truth published by S02 and the live repo state.
  - Files: `scripts/lib/m057_project_mutation_apply.py`, `.gsd/milestones/M057/slices/S03/project-mutation-plan.json`, `.gsd/milestones/M057/slices/S03/project-mutation-results.json`, `.gsd/milestones/M057/slices/S03/project-mutation-results.md`
  - Do: Add a plan-driven applicator with dry-run default and explicit `--apply`, use only captured project/field ids from `project-fields.snapshot.json`, execute deletes/adds/field edits in deterministic order, preserve explicit live non-null values unless the checked plan changes them, and persist per-command outcomes plus final row state in results artifacts.
  - Verify: `python3 scripts/lib/m057_project_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S03 --check && python3 scripts/lib/m057_project_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S03 --apply`
  - Done when: the live board changes are applied from the checked manifest, rerunning the applicator produces `already_satisfied` outcomes instead of duplicate writes, and the results artifacts record final item ids, canonical issue URLs, and representative done/active/todo row state.
- [ ] **T03: Replay live board truth and publish the retained S03 verifier** `est:90m`
  - Why: S03 is not done when the writes finish; it is done when a read-only verifier can prove the board now matches the reconciled repo truth and exposes drift clearly for future maintainers.
  - Files: `scripts/tests/verify-m057-s03-results.test.mjs`, `scripts/verify-m057-s03.sh`, `.gsd/milestones/M057/slices/S03/project-mutation-results.json`, `.tmp/m057-s03/verify/phase-report.txt`
  - Do: Add a results contract test and retained verifier that re-fetch the live board, assert stale cleanup rows are gone, confirm canonical `mesh-lang#19` / `hyperpush#58` handling, verify representative `Done` / `In Progress` / `Todo` rows, keep naming normalization truthful on `hyperpush#54/#55/#56`, and prove representative inherited rows match their parent-chain metadata. Publish or refresh the compact results markdown as the maintainer handoff.
  - Verify: `node --test scripts/tests/verify-m057-s03-results.test.mjs && bash scripts/verify-m057-s03.sh`
  - Done when: the contract test and retained verifier pass, `.tmp/m057-s03/verify/` contains phase diagnostics and command evidence, and `project-mutation-results.md` explains the final done/active/next board truth without requiring `.gsd` archaeology.

## Files Likely Touched

- `scripts/lib/m057_project_mutation_plan.py`
- `scripts/tests/verify-m057-s03-plan.test.mjs`
- `.gsd/milestones/M057/slices/S03/project-mutation-plan.json`
- `.gsd/milestones/M057/slices/S03/project-mutation-plan.md`
- `scripts/lib/m057_project_mutation_apply.py`
- `.gsd/milestones/M057/slices/S03/project-mutation-results.json`
- `.gsd/milestones/M057/slices/S03/project-mutation-results.md`
- `scripts/tests/verify-m057-s03-results.test.mjs`
- `scripts/verify-m057-s03.sh`
- `.tmp/m057-s03/verify/phase-report.txt`
