---
id: M057
title: "Cross-Repo Tracker Reconciliation"
status: complete
completed_at: 2026-04-10T20:27:33.340Z
key_decisions:
  - D455/D456 — Canonicalize tracker truth by canonical issue URL while preserving separate workspace-path truth, public repo truth, and normalized destination fields.
  - D457/D458/D459 — Keep the issue-backed ledger exact, surface missing coverage through derived gaps, and preserve transfer/create canonical mappings in checked results artifacts.
  - D460/D464 — Replay repo truth before board truth and delegate the retained S02 verifier first in S03 so upstream drift is classified at the correct layer.
  - D461/D462/D463 — Fail closed on board/planner misclassification, retry transient Projects V2 visibility lag locally, and normalize blank GitHub CLI `stateReason` noise to avoid false drift.
key_files:
  - scripts/lib/m057_tracker_inventory.py
  - scripts/lib/m057_evidence_index.py
  - scripts/lib/m057_reconciliation_ledger.py
  - scripts/lib/m057_repo_mutation_plan.py
  - scripts/lib/m057_repo_mutation_apply.py
  - scripts/lib/m057_project_mutation_plan.py
  - scripts/lib/m057_project_mutation_apply.py
  - scripts/verify-m057-s01.sh
  - scripts/verify-m057-s02.sh
  - scripts/verify-m057-s03.sh
  - .gsd/milestones/M057/slices/S01/reconciliation-ledger.json
  - .gsd/milestones/M057/slices/S02/repo-mutation-results.json
  - .gsd/milestones/M057/slices/S03/project-mutation-results.json
lessons_learned:
  - On local `main` auto-mode closeout, use `origin/main` rather than local `main` for the non-`.gsd` code-diff verification baseline.
  - For cross-repo tracker cleanup, checked snapshot/evidence/ledger artifacts make later live issue and board mutations auditable and rerun-safe.
  - Canonical identity mappings for transfers and retrospective creates must persist through both repo and board reconciliation or later verification becomes ambiguous.
  - Delegating upstream repo verification before board verification keeps drift diagnosis layered and prevents stale board text from masking repo-truth regressions.
---

# M057: Cross-Repo Tracker Reconciliation

**Reconciled `mesh-lang`, `hyperpush`, and org project #1 to the actual shipped code and ownership state, then closed the milestone with replayable repo-truth and board-truth verification.**

## What Happened

M057 repaired the public planning surfaces without reshaping the code to fit stale tracker wording. S01 established the canonical audit base: live repo and org-project snapshots, a naming/ownership evidence map, and a 68-row joined reconciliation ledger with explicit derived-gap handling for `/pitch`. S02 turned that checked ledger into live repo issue truth by closing shipped `mesh-lang` issues, rewriting stale `hyperpush` issues in place, preserving history through the `hyperpush#8 -> mesh-lang#19` transfer, materializing the missing `/pitch` tracker row as `hyperpush#58`, and proving idempotence with a second apply pass. S03 then consumed the refreshed repo-truth handoff to realign org project #1 to live issue truth, remove stale cleanup rows, add the canonical replacement rows, and backfill missing tracked metadata through deterministic inheritance while preserving explicit live board values.

Verification for closeout used the repo-local auto-mode rule from `.gsd/KNOWLEDGE.md`: because completion is running on local `main`, the non-`.gsd` code-change check used `git diff --stat HEAD $(git merge-base HEAD origin/main) -- ':!.gsd/'`. That diff showed real non-`.gsd` changes under `scripts/lib/`, `scripts/tests/`, and `scripts/verify-m057-*.sh`, so the milestone is not planning-only. Milestone validation already passed in `.gsd/milestones/M057/M057-VALIDATION.md`, and the slice summaries plus retained verifiers show the cross-slice chain working end to end: S01 produced the canonical ledger, S02 replayed repo truth from it, and S03 delegated S02’s verifier before asserting board truth.

### Decision re-evaluation

| Decision | Re-evaluation | Result |
|---|---|---|
| D455/D456 — keep canonical snapshots keyed by canonical issue URL and preserve `workspace_path_truth`, `public_repo_truth`, and normalized destination separately | The milestone needed those separate truth surfaces to reconcile `hyperpush-mono` compatibility-path evidence with public `hyperpush` tracker wording without losing provenance. | Keep |
| D457/D458/D459 — keep the issue-backed ledger exact, surface `/pitch` as a derived gap, and preserve transfer/create mappings in results artifacts | This made the later live mutation steps truthful and auditable; `/pitch -> hyperpush#58` and `hyperpush#8 -> mesh-lang#19` stayed inspectable throughout repo and board reconciliation. | Keep |
| D460/D464 — replay upstream repo truth before board truth and delegate the retained S02 verifier first in S03 | This caught and repaired upstream repo-truth drift before the board layer was trusted, which is the right layering for future maintenance too. | Keep |
| D461/D462/D463 — fail closed on `leave_untracked` misclassification, retry transient Projects V2 lag locally, and normalize blank CLI `stateReason` noise | These decisions reduced false positives while preserving strict contracts, and the retained verifiers stayed diagnostic instead of flaky. | Keep |

No decision from M057 currently needs immediate revisit, but R135 remains deferred if future automation is considered.

## Success Criteria Results

- **Canonical audit truth published for every relevant repo issue and project item — PASS.** S01 produced the canonical 68-row reconciliation ledger, 63 project-backed rows, raw repo/project snapshots, and the naming/ownership evidence bundle. `bash scripts/verify-m057-s01.sh` plus the S01 Node contract tests passed, and `reconciliation-audit.md` / `reconciliation-ledger.json` are the persistent inspection surfaces.
- **Repo issue sets reconciled to code and ownership reality — PASS.** S02 closed the 10 shipped `mesh-lang` issues, rewrote 21 `hyperpush` `rewrite_scope` rows, kept 7 truthful follow-through rows open, normalized `hyperpush#54/#55/#56`, preserved `hyperpush#8 -> mesh-lang#19`, and created/closed `/pitch` as `hyperpush#58`. Validation is retained in `node --test scripts/tests/verify-m057-s02-plan.test.mjs`, `node --test scripts/tests/verify-m057-s02-results.test.mjs`, and `bash scripts/verify-m057-s02.sh`.
- **Org project #1 realigned to reconciled cross-repo truth — PASS.** S03 reduced the board to 55 truthful live rows, removed stale Mesh cleanup rows, added canonical presence for `mesh-lang#19` and `hyperpush#58`, and backfilled missing tracked metadata while preserving explicit live values. Validation is retained in `node --test scripts/tests/verify-m057-s03-results.test.mjs` and `bash scripts/verify-m057-s03.sh`.
- **A fresh maintainer can understand done, active, and next from canonical artifacts without reopening tracker archaeology — PASS.** `reconciliation-audit.md`, `repo-mutation-results.md`, and `project-mutation-results.md` together expose canonical shipped/active/next truth, and `.gsd/milestones/M057/M057-VALIDATION.md` records a milestone-wide PASS across requirements, slice boundaries, acceptance evidence, and verification classes.
- **Horizontal checklist — none rendered in the checked-in roadmap.** No separate horizontal-checklist block was present to evaluate during closeout.

## Definition of Done Results

- **All slices complete — PASS.** `gsd_milestone_status` reports S01, S02, and S03 all `complete`, with task counts 3/3, 3/3, and 5/5 respectively.
- **All slice summaries exist — PASS.** `.gsd/milestones/M057/slices/S01/S01-SUMMARY.md`, `.gsd/milestones/M057/slices/S02/S02-SUMMARY.md`, and `.gsd/milestones/M057/slices/S03/S03-SUMMARY.md` are present on disk.
- **Cross-slice integration works correctly — PASS.** `.gsd/milestones/M057/M057-VALIDATION.md` records PASS for S01 → S02, S01 → S03, and S02 → S03 producer/consumer boundaries. S03’s retained verifier explicitly delegates `bash scripts/verify-m057-s02.sh` first, proving repo truth before board truth.
- **Milestone contains real non-`.gsd` code changes — PASS.** Using the repo-local closeout baseline `git diff --stat HEAD $(git merge-base HEAD origin/main) -- ':!.gsd/'`, the diff includes non-`.gsd` changes in `scripts/lib/m057_*`, `scripts/tests/verify-m057-*`, and `scripts/verify-m057-*.sh`.
- **Milestone validation passed before completion — PASS.** `.gsd/milestones/M057/M057-VALIDATION.md` records verdict `pass` with reviewer coverage for requirements, cross-slice integration, acceptance criteria, and verification class compliance.

## Requirement Outcomes

- **R133 — Active → Validated.** Supported by S01’s explicit `workspace_path_truth` / `public_repo_truth` / normalized destination fields, S02’s live normalization on `hyperpush#54/#55/#56`, and S03’s preservation of that normalized public naming on the reconciled board. Persisted verification now lives in `node --test scripts/tests/verify-m057-s02-results.test.mjs`, `node --test scripts/tests/verify-m057-s03-results.test.mjs`, `bash scripts/verify-m057-s02.sh`, and `bash scripts/verify-m057-s03.sh`.
- **R134 — Active → Validated.** Supported by the canonical audit and handoff surfaces: `reconciliation-audit.md`, `reconciliation-ledger.json`, `repo-mutation-results.md`, `project-mutation-results.md`, and the retained `.tmp/m057-s03/verify/` replay bundle. These artifacts now let a maintainer understand shipped, active, and next state without reopening earlier `.gsd` archaeology.
- **R128–R132 — Remain validated.** Milestone closeout reconfirmed the persisted validation evidence already recorded in `.gsd/REQUIREMENTS.md` and in the passing M057 validation artifact; no downgrade or rescope was required.
- **R135–R137 — No status transition.** R135 remains deferred, and R136/R137 remain out of scope as milestone guardrails rather than newly validated delivery requirements.

## Deviations

No substantive milestone-scope deviation from the roadmap. The closeout verification baseline used `origin/main` rather than local `main` because auto-mode completion is running on local `main` in this repo; that matches the repo-local knowledge-base rule for truthful non-`.gsd` diff verification.

## Follow-ups

Future tracker maintenance should start with `bash scripts/verify-m057-s02.sh` and `bash scripts/verify-m057-s03.sh` before any manual GitHub edits. If later work pursues automation (R135), build it on the persisted S01 ledger + S02/S03 results contracts instead of bypassing the manual taxonomy proven in M057.
