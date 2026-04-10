# S01: Audit code reality and build the reconciliation ledger — UAT

**Milestone:** M057
**Written:** 2026-04-10T06:26:24.509Z

# S01: Audit code reality and build the reconciliation ledger — UAT

**Milestone:** M057
**Written:** 2026-04-10

## UAT Type

- UAT mode: repo artifact + live tracker replay
- Why this mode is sufficient: this slice produces a deterministic reconciliation ledger rather than a user-facing app flow. The meaningful acceptance path is to confirm that the checked-in snapshots, evidence bundle, joined ledger, and retained verifier all agree on the same issue/project truth.

## Preconditions

- Run from the `mesh-lang` repo root.
- GitHub CLI access for read-only issue/project queries must be available to replay the live snapshot refresh.
- The sibling product workspace must still expose `mesher/` as the compatibility path into `../hyperpush-mono/mesher`.

## Smoke Test

1. Run `bash scripts/verify-m057-s01.sh`.
2. Confirm it exits 0.
3. Confirm `.tmp/m057-s01/verify/status.txt` is `ok` and `.tmp/m057-s01/verify/current-phase.txt` is `complete`.
4. **Expected:** the retained replay refreshes inventory, rebuilds evidence/ledger outputs, reruns both Node test rails, and leaves a green retained verify bundle.

## Test Cases

### 1. Raw inventory snapshots capture the expected repo/project totals and canonical repo identity

1. Open `.gsd/milestones/M057/slices/S01/mesh-lang-issues.snapshot.json` and `.gsd/milestones/M057/slices/S01/hyperpush-issues.snapshot.json`.
2. Confirm the combined issue inventory totals 68 rows.
3. Open `.gsd/milestones/M057/slices/S01/project-items.snapshot.json`.
4. Confirm `rollup.total_items` is 63.
5. Confirm `canonical_repos.hyperpush_alias.canonical_slug` resolves to `hyperpush-org/hyperpush` even though the requested alias is `hyperpush-org/hyperpush-mono`.
6. **Expected:** the committed snapshots prove both repo totals, the org-project item total, and the public `hyperpush` canonical identity used for tracker joins.

### 2. Project field maps are schema-seeded and safe against sparse GraphQL rows

1. Open `.gsd/milestones/M057/slices/S01/project-fields.snapshot.json` and note the tracked field keys.
2. Open `.gsd/milestones/M057/slices/S01/project-items.snapshot.json`.
3. Inspect several item rows, including one with missing optional fields.
4. Confirm tracked keys are present in the normalized field map even when the source GitHub item omitted the field in `fieldValues`.
5. **Expected:** missing board fields are represented as `null`, not by missing keys, so later reconciliation logic can distinguish sparse data from schema drift.

### 3. Evidence bundle captures naming drift, misfiled work, and missing `/pitch` coverage

1. Open `.gsd/milestones/M057/slices/S01/reconciliation-evidence.md`.
2. Confirm it includes evidence for `hyperpush#8`, `/pitch`, and the split between `workspace_path_truth` and `public_repo_truth`.
3. Open `.gsd/milestones/M057/slices/S01/naming-ownership-map.json`.
4. Confirm the product repo surfaces normalize to public repo `hyperpush` while still retaining the local `hyperpush-mono` workspace path evidence.
5. **Expected:** the evidence layer makes ownership and naming normalization explicit instead of hiding it in prose.

### 4. Joined ledger preserves exact issue/project counts and proposes concrete next actions

1. Open `.gsd/milestones/M057/slices/S01/reconciliation-ledger.json`.
2. Confirm `rollup.rows_total` is 68, `project_backed_rows` is 63, `non_project_rows` is 5, and `orphan_project_rows` is 0.
3. Confirm `rollup.primary_bucket_counts` reports 13 `shipped-but-open`, 21 `rewrite-split`, 33 `keep-open`, and 1 `misfiled` row.
4. Inspect at least these representative rows:
   - `mesh-lang#3` → proposed repo action `close_as_shipped`
   - `hyperpush#8` → proposed repo action `move_to_mesh_lang`
   - one `hyperpush#24`/`#29` style row → proposed repo action `rewrite_scope`
5. **Expected:** every row carries non-empty evidence refs, ownership truth, delivery truth, and repo/project action text, and the rollup matches the audit summary.

### 5. Missing tracker coverage stays in `derived_gaps` instead of synthetic issue rows

1. In `reconciliation-ledger.json`, inspect the `derived_gaps` section.
2. Confirm `/pitch` appears there with `create_missing_issue` and `create_project_item` actions.
3. Confirm there is no synthetic extra issue row that would change the 68-row issue-backed ledger count.
4. **Expected:** missing coverage is surfaced explicitly without breaking the exact issue/project join contract.

### 6. Human-readable audit summary is sufficient for downstream reconciliation work

1. Open `.gsd/milestones/M057/slices/S01/reconciliation-audit.md`.
2. Confirm it shows the same top-level counts as the ledger rollup.
3. Confirm it includes grouped tables for `shipped-but-open`, `rewrite/split`, `keep-open`, `misfiled`, `missing-coverage`, and `naming-drift`.
4. Confirm the audit calls out the `/pitch` gap and the `hyperpush#8` misfiled row.
5. **Expected:** a maintainer can use the audit alone to see what to close, rewrite, keep, move, or create before reading the raw JSON.

## Edge Cases

### GitHub alias drift

1. Rerun `python3 scripts/lib/m057_tracker_inventory.py --output-dir .gsd/milestones/M057/slices/S01 --refresh --check`.
2. **Expected:** the helper fails closed if `hyperpush-org/hyperpush-mono` stops canonicalizing to `hyperpush-org/hyperpush`; the slice should not silently join against an unverified alias.

### Orphan project items

1. Temporarily tamper with a copy of the committed ledger inputs so one project item points to no matching canonical issue URL, then run `node --test scripts/tests/verify-m057-s01-ledger.test.mjs` against the negative fixture path.
2. **Expected:** the contract test fails and names the orphan join problem instead of generating a partial ledger.

### Missing `project_item_id` on a project-backed row

1. Use the existing negative fixtures exercised by `node --test scripts/tests/verify-m057-s01-ledger.test.mjs`.
2. **Expected:** the contract test fails closed rather than allowing a project-backed row to degrade into an untracked row silently.

## Failure Signals

- Any slice replay leaves `.tmp/m057-s01/verify/status.txt` not equal to `ok`.
- `reconciliation-ledger.json` row counts no longer match 68 / 63 / 5 / 0.
- `/pitch` disappears from `derived_gaps` without a replacement issue row being intentionally created in later slices.
- `hyperpush#8` is no longer identified as misfiled language work.
- Naming/ownership artifacts collapse local `hyperpush-mono` workspace truth into public repo truth or vice versa.

## Requirements Proved By This UAT

- R128 — language-repo issues are now classified against actual mesh-lang code state with evidence.
- R129 — product-repo issues are now classified against actual sibling Hyperpush code state with evidence.
- R130 — org project #1 rows are joined to reconciled issue truth through captured `project_item_id` values.
- R131 — missing tracker coverage is surfaced explicitly through the `/pitch` derived gap.
- R132 — the ledger distinguishes close/rewrite/keep/move actions instead of hiding drift.
- R133 — tracker normalization now preserves separate workspace-path and public-repo truth.
- R134 — the human-readable audit makes shipped, active, and missing work intelligible from tracker artifacts.

## Not Proven By This UAT

- That GitHub issues or project items have already been mutated; S02 and S03 perform those external writes.
- That future tracker state still matches this snapshot without rerunning the live capture.

## Notes for Tester

- Use the retained wrapper as the first replay surface before reading JSON by hand: `bash scripts/verify-m057-s01.sh` leaves named logs under `.tmp/m057-s01/verify/` for each phase.
- Do not hand-edit the generated snapshot, evidence, or ledger JSON to “fix” counts. Repair the source script and rerun the wrapper so the retained evidence stays truthful.
