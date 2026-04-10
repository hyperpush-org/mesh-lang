# S02: Reconcile repo issues across `mesh-lang` and `hyperpush`

**Goal:** Mutate the live `mesh-lang` and `hyperpush` repo issue sets from the S01 ledger so each repo’s issue list matches actual code truth, preserves cross-repo history, and exposes the missing shipped `/pitch` surface explicitly before S03 realigns the org project.
**Demo:** After this, both repos’ issue sets are truthful on their own: completed work is closed, missing work is tracked, wrong-repo items are corrected, and drifted issues are rewritten or split.

## Must-Haves

- Apply repo mutations only from `.gsd/milestones/M057/slices/S01/reconciliation-ledger.json` and adjacent S01 snapshots; do not refresh or hand-edit S01 truth in place during this slice.
- Close the 10 shipped `mesh-lang` issues, rewrite the 21 `hyperpush` `rewrite_scope` rows, rewrite the 7 open mock-backed follow-through rows, and normalize public `hyperpush` naming on `#54/#55/#56` without flattening their history.
- Preserve history for the misfiled docs bug by transferring `hyperpush#8` into `mesh-lang` instead of delete/recreate, and capture the new canonical issue URL/number for S03.
- Create one explicit `hyperpush` issue for the shipped `/pitch` route, record its canonical URL/number, and close it as completed with code- and milestone-backed evidence so the missing surface is no longer implicit.
- Publish durable plan/results artifacts under `.gsd/milestones/M057/slices/S02/` plus a retained verifier under `.tmp/m057-s02/verify/` so a maintainer can trust repo issue truth without reopening `.gsd` archaeology.

## Threat Surface

- **Abuse**: a bad mutation manifest could close the wrong issue, overwrite truthful context, or recreate transferred work in a way that destroys history instead of preserving it.
- **Data exposure**: only public issue metadata, canonical URLs, plan diffs, and verification results may be written to artifacts; `gh` auth state, tokens, and local config must never be persisted.
- **Input trust**: S01 ledger rows, issue snapshots, live GitHub issue state, existing issue bodies, and template-derived title/body text are all untrusted until validated against the expected mutation buckets and destination repo truth.

## Requirement Impact

- **Requirements touched**: `R128`, `R129`, `R131`, `R132`, `R133`, `R134`
- **Re-verify**: the touched repo issue buckets, old→new canonical issue mapping for transferred/created issues, post-mutation repo totals, and rewritten issue text that must stop leaking stale `hyperpush-mono` public ownership.
- **Decisions revisited**: `D450`, `D451`, `D453`, `D457`

## Proof Level

- This slice proves: operational
- Real runtime required: yes — live GitHub issue mutation and read-only post-mutation inspection are the contract.
- Human/UAT required: no

## Verification

- `node --test scripts/tests/verify-m057-s02-plan.test.mjs`
- `node --test scripts/tests/verify-m057-s02-results.test.mjs`
- `bash scripts/verify-m057-s02.sh`

## Observability / Diagnostics

- Runtime signals: per-operation mutation/result rows with operation kind, repo, issue handle, apply/skipped status, timestamps, and old/new canonical URLs where transfer/create changes identity.
- Inspection surfaces: `.gsd/milestones/M057/slices/S02/repo-mutation-plan.json`, `.gsd/milestones/M057/slices/S02/repo-mutation-plan.md`, `.gsd/milestones/M057/slices/S02/repo-mutation-results.json`, `.gsd/milestones/M057/slices/S02/repo-mutation-results.md`, `node --test scripts/tests/verify-m057-s02-plan.test.mjs`, `node --test scripts/tests/verify-m057-s02-results.test.mjs`, and `.tmp/m057-s02/verify/*`.
- Failure visibility: the retained verifier must expose the failed phase, last attempted issue handle, `gh` stderr/stdout capture, and whether a mutation was skipped because the target state already existed.
- Redaction constraints: never write auth headers, tokens, or non-public local config into plan/results artifacts or retained logs.

## Integration Closure

- Upstream surfaces consumed: `.gsd/milestones/M057/slices/S01/reconciliation-ledger.json`, `.gsd/milestones/M057/slices/S01/reconciliation-audit.md`, `.gsd/milestones/M057/slices/S01/mesh-lang-issues.snapshot.json`, `.gsd/milestones/M057/slices/S01/hyperpush-issues.snapshot.json`, `scripts/lib/m057_tracker_inventory.py`, `scripts/lib/m057_reconciliation_ledger.py`, `.github/ISSUE_TEMPLATE/feature_request.yml`, `../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml`, `website/docs/.vitepress/config.mts`, `website/docs/.vitepress/theme/components/NavBar.vue`, and `mesher/landing/app/pitch/page.tsx`.
- New wiring introduced in this slice: a dry-run mutation planner, a plan-driven GitHub applicator, a live-state verifier, and S03 handoff artifacts carrying old/new canonical issue mappings.
- What remains before the milestone is truly usable end-to-end: S03 must update org project items/statuses from the reconciled repo issue truth and the new canonical URLs produced here.

## Tasks

- [x] **T01: Generate a dry-run repo mutation manifest from the S01 ledger** `est:2h`
  - Why: Live repo edits are the risky part of this slice, so execution needs one checked manifest that expands the S01 ledger into explicit touched rows, payloads, and identity-changing operations before anything mutates GitHub.
  - Files: `scripts/lib/m057_repo_mutation_plan.py`, `scripts/tests/verify-m057-s02-plan.test.mjs`, `.gsd/milestones/M057/slices/S02/repo-mutation-plan.json`, `.gsd/milestones/M057/slices/S02/repo-mutation-plan.md`
  - Do: Build a planner that consumes the S01 ledger and snapshots, generates truthful replacement titles/bodies/comments using existing repo issue shapes, excludes already-correct rows, and records the `hyperpush#8` transfer plus the retrospective `/pitch` issue creation in one deterministic manifest.
  - Verify: `python3 scripts/lib/m057_repo_mutation_plan.py --output-dir .gsd/milestones/M057/slices/S02 --check && node --test scripts/tests/verify-m057-s02-plan.test.mjs`
  - Done when: the plan artifacts exist, the contract test proves the touched set is exactly 10 closeouts + 31 open-issue rewrites/normalizations + 1 transfer + 1 create, and no already-correct closed rows enter the apply set.
- [x] **T02: Apply transfer, retrospective create, closeouts, and rewrites from the checked plan** `est:2h`
  - Why: This is the slice’s real external effect: the live GitHub repo issue sets must become truthful, history-preserving, and safe to resume if the batch stops mid-run.
  - Files: `scripts/lib/m057_repo_mutation_apply.py`, `scripts/lib/m057_repo_mutation_plan.py`, `.gsd/milestones/M057/slices/S02/repo-mutation-plan.json`, `.gsd/milestones/M057/slices/S02/repo-mutation-results.json`
  - Do: Add a plan-driven applicator with dry-run default and explicit `--apply`, run `hyperpush#8` transfer and `/pitch` create/close first so new canonical URLs are captured early, then execute the remaining closes and rewrites in deterministic order while recording every outcome and already-satisfied skip.
  - Verify: `python3 scripts/lib/m057_repo_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S02 --check && python3 scripts/lib/m057_repo_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S02 --apply`
  - Done when: the live repo mutations have been applied from the checked manifest, `repo-mutation-results.json` records old/new canonical mappings for transfer/create, and reruns do not duplicate already-satisfied repo changes.
- [x] **T03: Verify the live repo state and publish the S03 handoff artifacts** `est:90m`
  - Why: S02 is not done when the batch finishes; it is done when read-only GH checks prove the repo issue sets now match the S01 truth buckets and S03 has the exact new canonical issue URLs it must use.
  - Files: `scripts/tests/verify-m057-s02-results.test.mjs`, `scripts/verify-m057-s02.sh`, `.gsd/milestones/M057/slices/S02/repo-mutation-results.json`, `.gsd/milestones/M057/slices/S02/repo-mutation-results.md`
  - Do: Add a results contract test and retained verifier that check repo totals, bucket-level states, transfer/create mappings, and rewritten-text expectations, then publish a compact S03 handoff markdown with the new canonical issue URLs/numbers and any remaining board-only drift.
  - Verify: `node --test scripts/tests/verify-m057-s02-results.test.mjs && bash scripts/verify-m057-s02.sh`
  - Done when: read-only verification proves the expected closed/open counts and identity changes, the retained verifier leaves `.tmp/m057-s02/verify/` diagnostics, and `repo-mutation-results.md` gives S03 an exact mapping/handoff surface.

## Files Likely Touched

- `scripts/lib/m057_repo_mutation_plan.py`
- `scripts/tests/verify-m057-s02-plan.test.mjs`
- `.gsd/milestones/M057/slices/S02/repo-mutation-plan.json`
- `.gsd/milestones/M057/slices/S02/repo-mutation-plan.md`
- `scripts/lib/m057_repo_mutation_apply.py`
- `.gsd/milestones/M057/slices/S02/repo-mutation-results.json`
- `scripts/tests/verify-m057-s02-results.test.mjs`
- `scripts/verify-m057-s02.sh`
- `.gsd/milestones/M057/slices/S02/repo-mutation-results.md`
