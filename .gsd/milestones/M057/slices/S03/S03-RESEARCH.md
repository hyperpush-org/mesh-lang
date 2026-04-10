# M057/S03 — Research

**Date:** 2026-04-10

## Summary

S03 should reuse the S02 pattern: generate a checked plan artifact first, then apply project mutations from that manifest, then replay live GitHub state against the recorded final state. The existing M057 plumbing already gives S03 most of what it needs: `scripts/lib/m057_tracker_inventory.py` can normalize project fields/items with null backfill, `reconciliation-ledger.json` already classifies the original 63 project-backed rows, and `repo-mutation-results.json` publishes the canonical identity changes that the board must now follow (`hyperpush#8 -> mesh-lang#19`, `/pitch -> hyperpush#58`).

The important surprise is that live repo truth has already drifted after S02. `bash scripts/verify-m057-s02.sh` currently fails in `issue-state-replay`: `mesh-lang#19` is now `CLOSED`, and `mesh-lang#3` is now `OPEN`, even though S02 recorded the opposite final states. Repo totals still match (`mesh-lang=17`, `hyperpush=52`, combined `69`), so count-only checks are insufficient. Because R130 and R134 require the board to derive from current repo truth, S03 needs a hard precheck/rebaseline step before any project write.

## Recommendation

Build S03 as a deterministic project-mutation pipeline, not as ad hoc `gh project` edits. Follow the installed `gh` skill rule: keep explicit repo/project targeting on every command (`-R <repo>` for repo reads/writes; `--owner hyperpush-org` for project commands), and use `gh project item-edit` only through a helper because it updates one field at a time and requires `--id`, `--field-id`, and `--project-id` per write.

Recommended flow:
1. **Preflight repo truth** — rerun `bash scripts/verify-m057-s02.sh`; if it is red, capture the drift and block or explicitly rebaseline before mutating the project.
2. **Generate a project mutation plan** from live project items + S01 ledger + S02 results. Use S01 for original `project_item_id` coverage and action buckets, but use live repo issue data / S02 final-state data for canonical URLs, titles, and identity-changing rows.
3. **Apply via scripted `gh project` commands** — delete obsolete rows, add new canonical rows, and fill/update fields using field ids from `project-fields.snapshot.json`.
4. **Verify by replay** — fetch the live board again and prove that removed rows are gone, added rows are correct, and representative done/active/next rows have coherent status and metadata.

The board metadata gap is large enough that S03 should also codify **field inheritance** instead of hand-entering values. Today 23 live items are missing `Domain`/`Track`/`Delivery Mode`, and 17 of those also miss `Commitment`/`Priority`/dates/phase; all of those gaps are derivable from parent-chain items already on the board.

## Implementation Landscape

### Key Files

- `scripts/lib/m057_tracker_inventory.py` — Existing GitHub capture/normalization library. Reuse `capture_project_fields`, `normalize_project_item`, `validate_snapshots`, and `load_snapshots` rather than inventing new field-id or null-backfill logic.
- `scripts/lib/m057_project_items.graphql` — Existing paginated ProjectV2 query used by S01. Best source if S03 needs a fresh normalized live board snapshot with stable tracked-field keys.
- `.gsd/milestones/M057/slices/S01/project-fields.snapshot.json` — Authoritative local source for project id `PVT_kwDOEExRVs4BUM59`, field ids, and single-select option ids. Current tracked options include:
  - `Status`: `Todo`, `In Progress`, `Done`
  - `Domain`: `Hyperpush`, `Mesh`, `Shared`
  - `Track`: `Mesh Foundation`, `Core Parity`, `Operator App`, `AI + GitHub`, `Bug Market`, `Solana Economy`, `Deployment`, `SaaS Growth`
  - `Commitment`: `Exploring`, `Planned`, `Committed`
  - `Delivery Mode`: `Shared`, `SaaS-only`, `Self-hosted`
  - `Priority`: `P0`, `P1`, `P2`
  - `Hackathon Phase`: phases 1-6
- `.gsd/milestones/M057/slices/S01/reconciliation-ledger.json` — Canonical original project membership/action intent for 68 issue-backed rows / 63 project-backed rows. Use for `remove_from_project` / `update_project_item` / `keep_in_project` intent and the original `project_item_id` mapping, but **do not** trust it as final title/status truth after S02.
- `.gsd/milestones/M057/slices/S02/repo-mutation-results.json` — Canonical post-S02 repo identity truth. Required for:
  - `hyperpush#8 -> mesh-lang#19`
  - `/pitch -> hyperpush#58`
  - live normalized titles for `hyperpush#54/#55/#56`
- `.gsd/milestones/M057/slices/S02/repo-mutation-results.md` — Human-readable S03 handoff for canonical mappings and repo totals.
- `scripts/lib/m057_repo_mutation_plan.py` — Existing checked plan-artifact pattern to copy for S03.
- `scripts/lib/m057_repo_mutation_apply.py` — Existing `gh` command wrapper / results / command-log pattern to copy for S03.
- `.tmp/m057-s02/verify/phase-report.txt` — Current red preflight evidence: S02 verifier now fails.
- `.tmp/m057-s02/verify/verification-summary.json` — Shows the exact failure mode: repo totals still pass, but `issue-state-replay` fails on `mesh-lang#19`.
- `.tmp/m057-s02/verify/commands/003-issue-view-mesh-lang-19.stdout.txt` — Confirms `mesh-lang#19` is currently `CLOSED`.

### Current Live Board Facts

- `gh project item-list 1 --owner hyperpush-org --limit 200 --format json` returns **63** items.
- Current status rollup: **50 `Todo` / 2 `In Progress` / 11 `Done`**.
- The 10 `remove_from_project` mesh rows are still present on the board (`mesh-lang#3/#4/#5/#6/#8/#9/#10/#11/#13/#14`).
- The board still has **no** item for `mesh-lang#19` and **no** item for `hyperpush#58`.
- The board already reflects some live naming changes from S02 (`hyperpush#54/#55/#56`), so blindly replaying S01 titles would regress naming normalization.

### Field Inheritance Seam

Live board metadata is incomplete but structurally recoverable.

- **23 items** can fill missing `Domain`, `Track`, and `Delivery Mode` from existing parent-chain items.
- **17 of those 23** also need `Commitment`, `Priority`, `Start date`, `Target date`, and `Hackathon Phase` from the same parent chain.

Natural inheritance groups:

- `hyperpush#29-30` ← parent `hyperpush#13`
- `hyperpush#31-32` ← parent `hyperpush#14`
- `hyperpush#33-34` ← parent `hyperpush#15`
- `hyperpush#35-36` ← parent `hyperpush#16`
- `hyperpush#37-38` ← parent `hyperpush#17`
- `hyperpush#39-40` ← parent `hyperpush#18`
- `hyperpush#41-42` ← parent `hyperpush#19`
- `hyperpush#43-44` ← parent `hyperpush#20`
- `hyperpush#45-46` ← parent `hyperpush#21`
- `hyperpush#47-48` ← parent `hyperpush#22`
- `hyperpush#49-50` ← parent `hyperpush#23`
- `hyperpush#51-53` and `hyperpush#57` ← parent `hyperpush#34` ← grandparent `hyperpush#15`
- `hyperpush#54` ← parent `hyperpush#49` ← grandparent `hyperpush#23`
- `hyperpush#55-56` ← parent `hyperpush#50` ← grandparent `hyperpush#23`

This is the cleanest place to divide the work: one helper resolves nearest-parent tracked field values; the plan generator consumes it; the apply step just writes already-computed values.

### Recommended New Files

- `scripts/lib/m057_project_mutation_plan.py` — Build the deterministic S03 plan artifact from live board state + S01 ledger + S02 results.
- `scripts/lib/m057_project_mutation_apply.py` — Apply project deletes/adds/field edits and persist a results artifact with command logs.
- `scripts/tests/verify-m057-s03-plan.test.mjs` — Lock plan counts, canonical mapping handling, and parent-field derivation.
- `scripts/tests/verify-m057-s03-results.test.mjs` — Lock results shape, add/delete/update rollups, and representative final-state assertions.
- `scripts/verify-m057-s03.sh` — Replay live project truth after mutation.
- `.gsd/milestones/M057/slices/S03/project-mutation-plan.json`
- `.gsd/milestones/M057/slices/S03/project-mutation-plan.md`
- `.gsd/milestones/M057/slices/S03/project-mutation-results.json`
- `.gsd/milestones/M057/slices/S03/project-mutation-results.md`

### Build Order

1. **Retire the repo-truth unknown first.**
   - Run `bash scripts/verify-m057-s02.sh` at the start of S03.
   - Capture the current live repo delta (`mesh-lang#19` closed, `mesh-lang#3` open) instead of assuming the preloaded green S02 state is still live.
   - Decide whether S03 blocks on red S02 or explicitly rebases from the new live repo truth.

2. **Build the plan generator before any writes.**
   - Inputs: live project snapshot, `reconciliation-ledger.json`, `repo-mutation-results.json`, `project-fields.snapshot.json`.
   - Outputs: delete/add/update sets, field-edit counts, and canonical live targets.
   - Important rule: use live issue titles/URLs and S02 canonical mappings, not stale S01 project titles.

3. **Build the apply helper.**
   - `gh project item-delete` for removal rows.
   - `gh project item-add` for canonical rows not yet on the board.
   - `gh project item-edit` loops for field updates, keyed by field ids/option ids from the snapshot.
   - Preserve explicit live non-null field values unless canonical repo truth or naming normalization says they must change.

4. **Build the verifier and handoff surface.**
   - Replay live project state against the results artifact.
   - Prove representative shipped / active / next rows.
   - Record final counts and any intentionally kept Done items.

### Verification Approach

Use the same fail-closed structure as S02:

- `bash scripts/verify-m057-s02.sh`
  - Must pass, or S03 must explicitly surface/block on repo drift.
- `node --test scripts/tests/verify-m057-s03-plan.test.mjs`
- `node --test scripts/tests/verify-m057-s03-results.test.mjs`
- `bash scripts/verify-m057-s03.sh`

`verify-m057-s03.sh` should minimally assert:

- the plan/results artifacts are internally coherent
- the 10 `remove_from_project` rows are no longer on the board **if** repo truth still says they should be removed
- identity-changing rows are handled through the S02 canonical mapping surface (`mesh-lang#19`, `hyperpush#58`)
- at least one truthful shipped row, one active row, and one deferred/todo row are present and correctly classified
- naming normalization remains correct on `hyperpush#54/#55/#56`
- representative field-coherence checks pass for parent-derived rows (for example `hyperpush#29`, `#33`, `#35`, `#54`, `#55`, `#57`)

## Constraints

- `gh project item-edit` updates **one field per invocation** and requires `--id`, `--field-id`, and `--project-id`. Bulk project updates must be scripted.
- `project-fields.snapshot.json` is the only stable local source of field ids and single-select option ids; do not hardcode ids in new scripts.
- S01 project snapshots are historical inputs, not current truth. Live board and live repo state have already drifted from them.
- Current live repo truth does **not** fully match S02 results (`mesh-lang#19` closed, `mesh-lang#3` open), so S03 cannot safely treat S02 as immutable ground truth without a precheck.

## Common Pitfalls

- **Blindly replaying S01 `project_fields.title/status`** — this would regress S02 naming/title updates (`hyperpush#54/#55/#56`) and ignore live repo/project drift. Use live issue/result data for canonical titles and URLs.
- **Hand-entering missing board metadata** — 23 rows can inherit field values from existing parent chains. Manual editing makes the board inconsistent and explodes command count.
- **Trusting counts instead of replaying identities** — repo totals still pass even though two issue states drifted. R130/R134 need replay checks, not just totals.
- **Ignoring identity-changing rows** — `mesh-lang#19` and `hyperpush#58` are downstream of S02 mappings, not original S01 board URLs.

## Open Risks

- S03 currently has a real upstream blocker/risk: live repo truth drifted after S02. Planner/executor needs an explicit policy for whether S03 blocks, rebases, or partially proceeds when `verify-m057-s02.sh` is red.
- `mesh-lang#19` is now closed and `mesh-lang#3` reopened, which may change whether those rows should be added/removed/kept on the project. That decision cannot be made honestly from S01 alone.
- `hyperpush#58` is a retrospective closed issue. The board policy for closed-but-meaningful shipped rows should be explicit: keep as `Done` for visibility, or leave untracked to reduce roadmap clutter.

## Skills Discovered

| Technology | Skill | Status |
|------------|-------|--------|
| GitHub CLI / Projects V2 | `gh` | available |
