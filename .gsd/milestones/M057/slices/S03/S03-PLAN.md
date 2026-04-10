# S03: S03

**Goal:** Realign org project #1 from current live repo truth so the board shows truthful done/active/next state across `mesh-lang` and `hyperpush`, preserves the S02 canonical issue mappings, and backfills missing tracked metadata without hand-editing rows.
**Demo:** After this, org project #1 accurately shows what is done, active, and next across both repos instead of reading as a stale all-Todo launch plan.

## Must-Haves

- Record the retained S02 verifier outcome inside the S03 plan artifact and derive board mutations from current live repo truth plus persisted canonical mappings, not stale S01/S02 board text.
- Remove stale cleanup rows, handle the canonical `mesh-lang#19` / `hyperpush#58` mappings truthfully, keep `hyperpush#54/#55/#56` naming normalized, and preserve representative active/todo rows.
- Fill missing `Domain` / `Track` / `Delivery Mode` and deeper tracked metadata through deterministic parent-chain inheritance while preserving explicit live values.
- Publish checked plan/results artifacts plus a retained verifier and handoff so a maintainer can replay the board truth without reopening `.gsd` archaeology.

## Proof Level

- This slice proves: - This slice proves: final-assembly
- Real runtime required: yes — live GitHub project reads/writes are the contract.
- Human/UAT required: no

## Integration Closure

- Upstream surfaces consumed: `.gsd/milestones/M057/slices/S01/reconciliation-ledger.json`, `.gsd/milestones/M057/slices/S01/project-fields.snapshot.json`, `.gsd/milestones/M057/slices/S01/project-items.snapshot.json`, `.gsd/milestones/M057/slices/S02/repo-mutation-results.json`, `scripts/lib/m057_tracker_inventory.py`, `scripts/lib/m057_project_items.graphql`, and `scripts/verify-m057-s02.sh`.
- New wiring introduced in this slice: a live-truth project mutation planner, a plan-driven project applicator, field-inheritance resolution, a retained board verifier, and final handoff artifacts.
- What remains before the milestone is truly usable end-to-end: nothing.

## Verification

- Runtime signals: S03 plan/results artifacts must record repo-precheck status, planned/applied project operations, canonical old→new issue identity handling, inherited field sources, and final row state.
- Inspection surfaces: `.gsd/milestones/M057/slices/S03/project-mutation-plan.json`, `.gsd/milestones/M057/slices/S03/project-mutation-results.json`, `.gsd/milestones/M057/slices/S03/project-mutation-results.md`, and `.tmp/m057-s03/verify/*`.
- Failure visibility: the retained verifier must expose the failed phase, last attempted item/issue handle, command stderr/stdout capture, and whether drift came from repo truth, project membership, or field coherence.
- Redaction constraints: never persist auth headers, tokens, or local gh config; only public issue/project metadata and command summaries may enter artifacts.

## Tasks

- [x] **T01: Added the initial S03 project-mutation planner and blocked-plan artifacts, then stopped on a real upstream drift because the retained S02 verifier is stale against live mesh-lang#19 state.** `est:2h`
  Build the S03 planner before any project writes so board edits come from one checked manifest. Run the retained S02 verifier as a preflight and record any drift, normalize the current live project rows with the existing inventory helpers, join them to the S01 ledger plus S02 canonical mapping/results artifacts, resolve delete/add/update operations, and derive parent-chain tracked-field inheritance for rows missing board metadata. The plan artifact must make the `mesh-lang#19` / `hyperpush#58` identity changes explicit and preserve truthful naming on `hyperpush#54/#55/#56`.
  - Files: `scripts/lib/m057_project_mutation_plan.py`, `scripts/tests/verify-m057-s03-plan.test.mjs`, `.gsd/milestones/M057/slices/S03/project-mutation-plan.json`, `.gsd/milestones/M057/slices/S03/project-mutation-plan.md`
  - Verify: python3 scripts/lib/m057_project_mutation_plan.py --output-dir .gsd/milestones/M057/slices/S03 --check && node --test scripts/tests/verify-m057-s03-plan.test.mjs

- [x] **T02: Refresh the retained S02 verifier and results to current live repo truth** `est:90m`
  Why: S03 cannot safely mutate org project #1 while its required S02 preflight is red. The blocker showed the retained S02 verifier and published results still assert mesh-lang#19 is OPEN even though the live canonical issue is now CLOSED.

Do:
1. Update the retained S02 results contract and verifier expectations to replay live repo truth instead of the stale pre-merge state.
2. Keep the canonical old→new issue identity handling from S02 intact while treating the closed state of mesh-lang#19 as the current truthful outcome.
3. Refresh the S02 results artifacts and retained verification diagnostics so S03 can consume a green, replayable upstream truth source.

Done when: node-based S02 results verification and the retained S02 verifier both pass against live GitHub state, and the refreshed S02 results artifacts explicitly encode the current mesh-lang#19 truth without losing canonical mappings.
  - Files: `scripts/tests/verify-m057-s02-results.test.mjs`, `scripts/verify-m057-s02.sh`, `.gsd/milestones/M057/slices/S02/repo-mutation-results.json`, `.gsd/milestones/M057/slices/S02/repo-mutation-results.md`, `.tmp/m057-s02/verify/phase-report.txt`
  - Verify: node --test scripts/tests/verify-m057-s02-results.test.mjs && bash scripts/verify-m057-s02.sh

- [x] **T03: Re-run the S03 planner on green upstream truth and add the plan contract test** `est:90m`
  Why: Once the retained S02 preflight is truthful again, S03 still needs its missing plan contract test and a verified ready-path planner output before any board mutation is allowed.

Do:
1. Finish the S03 planner ready path so it records the retained S02 preflight verdict, derives mutations from live repo/project truth plus canonical mappings, and persists deterministic parent-chain inheritance sources.
2. Add scripts/tests/verify-m057-s03-plan.test.mjs to lock the checked plan artifact shape, canonical replacement handling, stale-row deletions, and representative inherited metadata coverage.
3. Regenerate the S03 plan artifacts from the now-green S02 preflight and confirm the checked manifest is no longer blocked.

Done when: the checked S03 plan artifacts exist, the retained S02 verifier outcome is recorded as green, the stale cleanup and canonical replacement rows are explicitly accounted for, and the plan contract test passes.
  - Files: `scripts/lib/m057_project_mutation_plan.py`, `scripts/tests/verify-m057-s03-plan.test.mjs`, `.gsd/milestones/M057/slices/S03/project-mutation-plan.json`, `.gsd/milestones/M057/slices/S03/project-mutation-plan.md`
  - Verify: python3 scripts/lib/m057_project_mutation_plan.py --output-dir .gsd/milestones/M057/slices/S03 --check && node --test scripts/tests/verify-m057-s03-plan.test.mjs

- [x] **T04: Apply project deletes, adds, and field edits from the refreshed checked manifest** `est:2h`
  Why: After the planner is green, org project #1 still has to be brought into line with the reconciled repo truth through a deterministic, resumable apply step.

Do:
1. Build or finish the plan-driven applicator with dry-run default and explicit --apply.
2. Use only captured project and field ids from the S01 schema snapshot, preserve explicit live non-null values unless the checked plan changes them, and execute deletes/adds/field edits in deterministic order.
3. Persist per-command outcomes, final item ids, canonical issue URLs, and representative done/active/todo row state in results artifacts, then prove rerun safety through already_satisfied outcomes.

Done when: the live board changes are applied from the checked manifest, rerunning the applicator does not duplicate writes, and the results artifacts capture the final canonical row state for the touched set.
  - Files: `scripts/lib/m057_project_mutation_apply.py`, `.gsd/milestones/M057/slices/S03/project-mutation-plan.json`, `.gsd/milestones/M057/slices/S03/project-mutation-results.json`, `.gsd/milestones/M057/slices/S03/project-mutation-results.md`
  - Verify: python3 scripts/lib/m057_project_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S03 --check && python3 scripts/lib/m057_project_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S03 --apply

- [ ] **T05: Replay live board truth and publish the retained S03 verifier** `est:90m`
  Why: S03 is only complete once a read-only verifier can replay the final live board state, prove it matches reconciled repo truth, and expose drift clearly for future maintainers.

Do:
1. Add the S03 results contract test to lock touched-set coverage, canonical mapping handling, representative row states, and inherited metadata expectations.
2. Add or finish the retained live verifier so it re-fetches org project #1, checks stale cleanup row removal, canonical mesh-lang#19 / hyperpush#58 handling, public naming normalization on hyperpush#54/#55/#56, and representative inherited rows.
3. Persist phase diagnostics, last-target evidence, and a maintainer-readable results markdown handoff that explains the final done/active/next board truth without reopening .gsd archaeology.

Done when: the results contract test and retained verifier pass, .tmp/m057-s03/verify contains clear diagnostics, and the published results markdown explains the final board truth from the verified live state.
  - Files: `scripts/tests/verify-m057-s03-results.test.mjs`, `scripts/verify-m057-s03.sh`, `.gsd/milestones/M057/slices/S03/project-mutation-results.json`, `.tmp/m057-s03/verify/phase-report.txt`
  - Verify: node --test scripts/tests/verify-m057-s03-results.test.mjs && bash scripts/verify-m057-s03.sh

## Files Likely Touched

- scripts/lib/m057_project_mutation_plan.py
- scripts/tests/verify-m057-s03-plan.test.mjs
- .gsd/milestones/M057/slices/S03/project-mutation-plan.json
- .gsd/milestones/M057/slices/S03/project-mutation-plan.md
- scripts/tests/verify-m057-s02-results.test.mjs
- scripts/verify-m057-s02.sh
- .gsd/milestones/M057/slices/S02/repo-mutation-results.json
- .gsd/milestones/M057/slices/S02/repo-mutation-results.md
- .tmp/m057-s02/verify/phase-report.txt
- scripts/lib/m057_project_mutation_apply.py
- .gsd/milestones/M057/slices/S03/project-mutation-results.json
- .gsd/milestones/M057/slices/S03/project-mutation-results.md
- scripts/tests/verify-m057-s03-results.test.mjs
- scripts/verify-m057-s03.sh
- .tmp/m057-s03/verify/phase-report.txt
