# S01: Audit code reality and build the reconciliation ledger

**Goal:** Build one code-backed reconciliation ledger that joins live `mesh-lang` + `hyperpush` issue inventories and org project #1 items to actual repo evidence, canonical ownership/naming truth, and concrete proposed tracker actions.
**Demo:** After this, a maintainer can open `.gsd/milestones/M057/slices/S01/reconciliation-ledger.json` and `.gsd/milestones/M057/slices/S01/reconciliation-audit.md` to see every relevant repo issue and project item classified against actual `mesh-lang` / `hyperpush` code state with evidence refs, ownership truth, delivery truth, and proposed repo/project actions.

## Must-Haves

- Capture durable live snapshots for both repos plus org project #1, including `project_item_id` and field snapshots, under `.gsd/milestones/M057/slices/S01/`.
- Build a code/milestone evidence index that classifies shipped, partial, active, future, misfiled, and naming-drift cases from actual files rather than issue prose.
- Emit one joined reconciliation ledger keyed by canonical issue URL that covers all `68` repo issues, all `63` project-backed items, and the `5` explicit non-project issue rows.
- Publish a human-readable audit summary that highlights shipped-but-open rows, rewrite/split candidates, misfiled items, missing coverage, and naming/ownership drift.
- Advance `R128`–`R134` by making later issue/project mutation work deterministic from one canonical row set instead of fresh `.gsd` archaeology.

## Threat Surface

- **Abuse**: stale issue text, malformed GraphQL responses, or symlink/ownership confusion could produce false-close recommendations if the ledger trusts issue prose instead of code evidence.
- **Data exposure**: only public repo/project metadata should be persisted; auth tokens, CLI config, and local environment state must never be written into snapshots or logs.
- **Input trust**: GitHub issue bodies, project field values, milestone summaries, repo-identity helpers, and symlinked `mesher/` surfaces are all untrusted inputs that must be normalized before classification.

## Requirement Impact

- **Requirements touched**: `R128`, `R129`, `R130`, `R131`, `R132`, `R133`, `R134`
- **Re-verify**: raw inventory counts, captured `project_item_id` coverage, naming normalization fields, misfiled/missing-coverage rollups, and non-empty evidence/action columns for every ledger row.
- **Decisions revisited**: `D450`, `D451`, `D452`, `D453`, `D454`

## Proof Level

- This slice proves: integration
- Real runtime required: yes — live `gh` issue/project queries plus the sibling product repo surfaces are part of the proof.
- Human/UAT required: no

## Verification

- `node --test scripts/tests/verify-m057-s01-inventory.test.mjs`
- `node --test scripts/tests/verify-m057-s01-ledger.test.mjs`
- `bash scripts/verify-m057-s01.sh`

## Observability / Diagnostics

- Runtime signals: snapshot `captured_at` metadata, canonical repo identity capture, ledger rollup counts, and verifier phase/status files under `.tmp/m057-s01/verify/`.
- Inspection surfaces: raw snapshot JSON files, evidence/ledger markdown summaries, `node --test scripts/tests/verify-m057-s01-inventory.test.mjs`, `node --test scripts/tests/verify-m057-s01-ledger.test.mjs`, and `bash scripts/verify-m057-s01.sh`.
- Failure visibility: missing `project_item_id`, orphan project rows, duplicate canonical issue URLs, empty evidence/action fields, and canonical repo drift must all fail closed with a named phase or invariant.
- Redaction constraints: persist only public GitHub metadata and file refs; never serialize auth tokens, local gh config, or other secrets.

## Integration Closure

- Upstream surfaces consumed: `gh issue list`, `gh repo view`, `gh project field-list`, `gh api graphql`, `.gsd/PROJECT.md`, `.gsd/milestones/M053/M053-SUMMARY.md`, `.gsd/milestones/M054/M054-SUMMARY.md`, `.gsd/milestones/M055/M055-SUMMARY.md`, `.gsd/milestones/M056/M056-SUMMARY.md`, `scripts/lib/repo-identity.json`, `scripts/workspace-git.sh`, `mesher/frontend-exp/lib/mock-data.ts`, `mesher/landing/app/pitch/page.tsx`, `website/docs/.vitepress/config.mts`, and `website/docs/.vitepress/theme/components/NavBar.vue`.
- New wiring introduced in this slice: a live snapshot capture helper, a code-evidence index builder, a joined reconciliation ledger builder, and a fail-closed verifier that preserves retained audit logs.
- What remains before the milestone is truly usable end-to-end: S02 must mutate repo issues from the ledger, and S03 must realign org project fields/statuses from the same canonical row set.

## Tasks

- [x] **T01: Capture live issue and project snapshots with baseline inventory assertions** `est:2h`
  - Why: Later classification and mutation work must operate on durable raw GitHub data with `project_item_id` and field snapshots, not ad hoc CLI output or memory of the current board.
  - Files: `scripts/lib/m057_tracker_inventory.py`, `scripts/lib/m057_project_items.graphql`, `scripts/tests/verify-m057-s01-inventory.test.mjs`, `.gsd/milestones/M057/slices/S01/mesh-lang-issues.snapshot.json`, `.gsd/milestones/M057/slices/S01/hyperpush-issues.snapshot.json`, `.gsd/milestones/M057/slices/S01/project-items.snapshot.json`, `.gsd/milestones/M057/slices/S01/project-fields.snapshot.json`
  - Do: Add a refresh helper that captures both repo issue inventories, canonical `hyperpush` repo identity, project field schema, and project item data with `project_item_id` + field values, then writes stable JSON snapshots keyed by canonical issue URL and annotated with capture metadata; add an inventory contract test that fails if counts or canonical repo assumptions drift.
  - Verify: `python3 scripts/lib/m057_tracker_inventory.py --output-dir .gsd/milestones/M057/slices/S01 --refresh --check && node --test scripts/tests/verify-m057-s01-inventory.test.mjs`
  - Done when: snapshot files exist under the slice directory, the inventory test proves `68` repo issue rows / `63` project-backed rows / `5` explicit non-project rows, and canonical `hyperpush` repo identity is captured rather than inferred from stale `hyperpush-mono` prose.
- [x] **T02: Build the code-evidence index and naming/ownership map** `est:2h`
  - Why: `R128`/`R129` require classification against actual code and milestone proof, and `R133` requires naming/ownership normalization before any tracker action is proposed.
  - Files: `scripts/lib/m057_evidence_index.py`, `.gsd/milestones/M057/slices/S01/reconciliation-evidence.json`, `.gsd/milestones/M057/slices/S01/reconciliation-evidence.md`, `.gsd/milestones/M057/slices/S01/naming-ownership-map.json`
  - Do: Build an evidence index from `.gsd/PROJECT.md`, M053–M056 summaries, repo-identity/workspace helpers, and the targeted product/docs files from research; encode `ownership_truth`, `delivery_truth`, `workspace_path_truth`, `public_repo_truth`, and `normalized_canonical_destination`; emit machine-readable and human-readable evidence bundles that later ledger assembly can reuse without rereading source files.
  - Verify: `python3 scripts/lib/m057_evidence_index.py --output-dir .gsd/milestones/M057/slices/S01 --check && rg -n "hyperpush#8|/pitch|workspace_path_truth|public_repo_truth" .gsd/milestones/M057/slices/S01/reconciliation-evidence.md .gsd/milestones/M057/slices/S01/naming-ownership-map.json`
  - Done when: the evidence bundle cites concrete milestone/file refs for shipped-but-open and still-open product work, includes misfiled `hyperpush#8`, the `/pitch` gap, and `hyperpush-mono` naming drift, and exposes normalized naming fields for later action generation.
- [x] **T03: Assemble the reconciliation ledger and publish the fail-closed audit proof** `est:2h`
  - Why: This closes the slice contract: one canonical row set must explain repo truth, project truth, evidence, ownership, and proposed actions for every issue.
  - Files: `scripts/lib/m057_reconciliation_ledger.py`, `scripts/tests/verify-m057-s01-ledger.test.mjs`, `scripts/verify-m057-s01.sh`, `.gsd/milestones/M057/slices/S01/reconciliation-ledger.json`, `.gsd/milestones/M057/slices/S01/reconciliation-audit.md`
  - Do: Add a ledger builder that joins raw snapshots and evidence by canonical issue URL, fails on blanks/orphans/duplicates, emits the final reconciliation ledger and audit summary, and wraps the full refresh/build/assert chain in a retained verifier under `.tmp/m057-s01/verify/`.
  - Verify: `node --test scripts/tests/verify-m057-s01-ledger.test.mjs && bash scripts/verify-m057-s01.sh`
  - Done when: every ledger row has non-empty evidence/ownership/delivery/action fields, `63` rows carry `project_item_id`, `5` explicit repo rows remain blank by design, there are `0` orphan project items, and the retained wrapper passes end to end.

## Files Likely Touched

- `scripts/lib/m057_tracker_inventory.py`
- `scripts/lib/m057_project_items.graphql`
- `scripts/tests/verify-m057-s01-inventory.test.mjs`
- `.gsd/milestones/M057/slices/S01/mesh-lang-issues.snapshot.json`
- `.gsd/milestones/M057/slices/S01/hyperpush-issues.snapshot.json`
- `.gsd/milestones/M057/slices/S01/project-items.snapshot.json`
- `.gsd/milestones/M057/slices/S01/project-fields.snapshot.json`
- `scripts/lib/m057_evidence_index.py`
- `.gsd/milestones/M057/slices/S01/reconciliation-evidence.json`
- `.gsd/milestones/M057/slices/S01/reconciliation-evidence.md`
- `.gsd/milestones/M057/slices/S01/naming-ownership-map.json`
- `scripts/lib/m057_reconciliation_ledger.py`
- `scripts/tests/verify-m057-s01-ledger.test.mjs`
- `scripts/verify-m057-s01.sh`
- `.gsd/milestones/M057/slices/S01/reconciliation-ledger.json`
- `.gsd/milestones/M057/slices/S01/reconciliation-audit.md`
