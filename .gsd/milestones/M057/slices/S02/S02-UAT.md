# S02: Reconcile repo issues across `mesh-lang` and `hyperpush` — UAT

**Milestone:** M057
**Written:** 2026-04-10T17:05:31.088Z

# S02: Reconcile repo issues across `mesh-lang` and `hyperpush` — UAT

**Milestone:** M057
**Written:** 2026-04-10

## UAT Type

- UAT mode: live GitHub issue-state replay plus artifact inspection.
- Why this mode is sufficient: this slice's contract is operational, not UI-facing. Acceptance means the checked artifacts, the live repo issue sets, and the retained verifier all agree on the same post-mutation truth.

## Preconditions

- Run from the `mesh-lang` repo root.
- `gh` must be authenticated for read-only issue inspection. Write access is only needed if you intentionally rerun the idempotence apply edge case.
- The checked S01 inputs must still exist under `.gsd/milestones/M057/slices/S01/`.
- Network access to GitHub must be available.

## Smoke Test

1. Run `node --test scripts/tests/verify-m057-s02-plan.test.mjs`.
2. Run `node --test scripts/tests/verify-m057-s02-results.test.mjs`.
3. Run `bash scripts/verify-m057-s02.sh`.
4. Confirm all three commands exit 0.
5. Open `.tmp/m057-s02/verify/phase-report.txt`.
6. **Expected:** status is `ok`, failed phase is `none`, and the phase list shows `artifact-contract`, `repo-totals`, `issue-state-replay`, and `handoff-render` all green.

## Test Cases

### 1. Canonical transfer/create mappings are published for downstream S03 work

1. Open `.gsd/milestones/M057/slices/S02/repo-mutation-results.md`.
2. Confirm the canonical identity table lists:
   - `hyperpush#8` → `mesh-lang#19` with final state `OPEN`
   - `/pitch` derived gap → `hyperpush#58` with final state `CLOSED`
3. Open `.gsd/milestones/M057/slices/S02/repo-mutation-results.json`.
4. Confirm those same canonical URLs/numbers are present in the machine-readable mapping.
5. **Expected:** S03 has one authoritative old→new mapping surface and does not need to guess issue identity from stale URLs.

### 2. `mesh-lang` shipped rows are now closed and the transferred docs issue is preserved in the correct repo

1. Open `.tmp/m057-s02/verify/verification-summary.json`.
2. Confirm `repo-totals.mesh_lang` reports `total=17`, `open=7`, `closed=10`.
3. Confirm the `close_handles` list contains exactly:
   - `mesh-lang#3`, `#4`, `#5`, `#6`, `#8`, `#9`, `#10`, `#11`, `#13`, `#14`
4. Run `gh issue view 3 -R hyperpush-org/mesh-lang --json number,title,state,url`.
5. Run `gh issue view 10 -R hyperpush-org/mesh-lang --json number,title,state,url`.
6. Run `gh issue view 19 -R hyperpush-org/mesh-lang --json number,title,state,url`.
7. **Expected:** `#3` and `#10` are `CLOSED`, `#19` is `OPEN`, and `#19` is the preserved transferred docs issue rather than a recreated replacement.

### 3. `hyperpush` rewrite-scope rows remain open with truthful rewritten scope

1. Open `.tmp/m057-s02/verify/verification-summary.json`.
2. Confirm `repo-totals.hyperpush` reports `total=52`, `open=47`, `closed=5`.
3. Confirm `rewrite_scope_handles` contains 21 issues.
4. Run these spot checks:
   - `gh issue view 24 -R hyperpush-org/hyperpush --json number,title,state,body,url`
   - `gh issue view 29 -R hyperpush-org/hyperpush --json number,title,state,body,url`
   - `gh issue view 50 -R hyperpush-org/hyperpush --json number,title,state,body,url`
5. **Expected:** each issue is still `OPEN`, the body/title text matches the reconciled product-repo scope rather than stale launch-plan wording, and none of these rows were force-closed just because code had partially shipped.

### 4. Mock-backed follow-through rows stay open and naming normalization rows stop leaking stale public `hyperpush-mono` ownership

1. In `.tmp/m057-s02/verify/verification-summary.json`, confirm `mock_follow_handles` contains 7 issues and `naming_handles` contains `hyperpush#54`, `#55`, and `#56`.
2. Run these spot checks:
   - `gh issue view 15 -R hyperpush-org/hyperpush --json number,state,body`
   - `gh issue view 51 -R hyperpush-org/hyperpush --json number,state,body`
   - `gh issue view 54 -R hyperpush-org/hyperpush --json number,state,body`
   - `gh issue view 55 -R hyperpush-org/hyperpush --json number,state,body`
   - `gh issue view 56 -R hyperpush-org/hyperpush --json number,state,body`
3. **Expected:** `#15` and `#51` remain `OPEN` with wording that still exposes the unfinished operator/app/backend work, while `#54/#55/#56` remain `OPEN` but use public `hyperpush-org/hyperpush` naming instead of stale public `hyperpush-mono` ownership wording.

### 5. The shipped `/pitch` surface is now explicitly tracked and closed as completed

1. Run `gh issue view 58 -R hyperpush-org/hyperpush --json number,title,state,url,body,comments`.
2. Confirm the issue exists and is `CLOSED`.
3. Confirm `.gsd/milestones/M057/slices/S02/repo-mutation-results.md` describes it as the retrospective issue for the already-shipped evaluator route.
4. **Expected:** the missing tracker coverage discovered in S01 is no longer implicit in milestone prose; it has one canonical hyperpush issue with closeout evidence.

### 6. The transferred source issue is absent in the old repo and the verifier treats that absence as expected proof

1. Run `gh issue view 8 -R hyperpush-org/hyperpush --json number,title,state,url`.
2. Confirm the command fails.
3. Open `.tmp/m057-s02/verify/verification-summary.json` and inspect `phases[].transfer_source_absence`.
4. **Expected:** the source lookup failure is recorded as expected absence proof for the transfer, while the canonical destination remains `mesh-lang#19` in the results artifact.

### 7. Retained diagnostics are sufficient for a maintainer to debug drift without reopening slice archaeology

1. List `.tmp/m057-s02/verify/`.
2. Confirm it contains at least:
   - `phase-report.txt`
   - `verification-summary.json`
   - `last-target.txt`
   - `phase-01-artifact-contract.json`
   - `phase-02-repo-totals.json`
   - `phase-03-issue-state-replay.json`
   - `phase-04-handoff-render.json`
3. Open `phase-report.txt` and `last-target.txt`.
4. **Expected:** the verifier names the failed phase/last issue if anything drifts, so a maintainer can start from one retained surface instead of reconstructing the mutation batch by hand.

## Edge Cases

### Idempotent reapply

1. If you have write access and want to reprove resume safety, run `python3 scripts/lib/m057_repo_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S02 --apply`.
2. Open `.gsd/milestones/M057/slices/S02/repo-mutation-results.json`.
3. **Expected:** the run succeeds without creating duplicate issues or replaying transfers, and the operations remain `already_satisfied` rather than mutating state again.

### Canonical mapping drift should fail closed

1. Use the existing negative fixtures exercised by `node --test scripts/tests/verify-m057-s02-results.test.mjs`.
2. Remove a canonical destination URL from a copy of the results artifact fixture and rerun the Node test.
3. **Expected:** the contract test fails immediately instead of silently accepting a results artifact that would leave S03 without a truthful old→new issue mapping.

## Failure Signals

- `repo-mutation-results.md` no longer lists `hyperpush#8 -> mesh-lang#19` or `/pitch -> hyperpush#58`.
- `repo-totals` drifts away from `mesh-lang=17` / `hyperpush=52` / `combined=69`.
- Any of the 10 shipped `mesh-lang` rows reopen.
- Any `rewrite_scope` or mock-follow-through row closes unexpectedly or regains stale wording.
- `hyperpush#54/#55/#56` regress to stale public `hyperpush-mono` naming.
- `.tmp/m057-s02/verify/phase-report.txt` reports a failed phase or names a last target other than the expected replay flow.

## Requirements Proved By This UAT

- R128 — `mesh-lang` issues now reflect actual language-repo code state.
- R129 — `hyperpush` issues now reflect actual product-repo code state.
- R131 — missing tracker coverage is created explicitly through retrospective issue `hyperpush#58`.
- R132 — reconciliation preserves history through transfer, closeout evidence, and in-place rewrites.
- R133 — repo naming is normalized on live tracker surfaces.

## Not Proven By This UAT

- R130 or the final board-level part of R134; org project #1 still needs S03 realignment.
- Any continuous monitoring loop; this verifier is a point-in-time live replay.

## Notes for Tester

- Start with `bash scripts/verify-m057-s02.sh`; it is the fastest truthful replay seam because it rechecks artifact invariants, repo totals, all touched issues, and the S03 handoff in one run.
- Treat `gh issue view 8 -R hyperpush-org/hyperpush` failing as expected after transfer. The truthful canonical mapping lives in `repo-mutation-results.json` / `.md`, not in the old repo/number pair.
- Do not hand-edit generated plan/results artifacts to make counts pass. Repair the planner/applicator/verifier or the live GitHub issue state and rerun the checks.
