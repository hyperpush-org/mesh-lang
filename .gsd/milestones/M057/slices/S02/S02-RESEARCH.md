# Slice S02 Research — Reconcile repo issues across `mesh-lang` and `hyperpush`

## Summary
- S02 is the **repo-issue mutation** slice for `R128`, `R129`, `R131`, `R132`, and `R133`, and it advances `R134` by making the repo issue surfaces truthful before S03 fixes the org project. The board is downstream; do not let S02 sprawl into broad project-field work.
- S01 already produced the only canonical mutation input needed for this slice: `.gsd/milestones/M057/slices/S01/reconciliation-ledger.json`. The planner should treat that file as immutable audit input and build S02 artifacts beside it under `.gsd/milestones/M057/slices/S02/`, **not** by refreshing S01 snapshots in place.
- The live repo mutation set is larger than “close stale issues”:
  - **10 mesh-lang issues to close as shipped:** `#3 #4 #5 #6 #8 #9 #10 #11 #13 #14`
  - **21 hyperpush issues in `rewrite_scope`:** `#24 #29-#32 #35-#50`
  - **7 hyperpush issues that stay open but should be rewritten to truthful mock-backed follow-through:** `#15 #33 #34 #51 #52 #53 #57`
  - **3 hyperpush issues that stay open but need wording normalization to public `hyperpush` ownership truth:** `#54 #55 #56`
  - **1 misfiled issue:** `hyperpush#8`
  - **1 missing tracker row:** shipped `/pitch`
- `hyperpush#3/#4/#5` are already `CLOSED` and unprojected. They appear in the shipped bucket, but they are **not** the repo-mutation priority. The real closure work is the 10 open `mesh-lang` rows.
- The safest/newest GitHub-path finding: `gh issue transfer` exists in the installed `gh 2.87.3`, so `hyperpush#8` can be moved to `mesh-lang` **without recreating it by hand**. That is the cleanest way to satisfy `R132` (preserve history instead of silently repurposing it).
- `gh auth status` is already green and includes `project`, `read:org`, `repo`, and `workflow` scopes. No secret/user step is needed before execution.

## Skills Discovered
- Existing skill: `gh`
- No additional skill installs were needed.
- Relevant `gh` skill rules that directly constrain this slice:
  - **Repo-detection rule:** pass `-R` / `--repo` on every repo-scoped `gh` command. Do not rely on cwd repo detection.
  - **Projects V2 rule:** use captured `project_item_id` / field metadata for board work instead of inferring project state from issue prose. That reinforces the slice boundary: S02 should fix repo issue truth first and hand precise new issue URLs/IDs forward to S03.

## Requirements Coverage
- **R128 / R129:** repo issues must match actual language/product code truth. S02 is the slice that applies those repo-side corrections.
- **R131:** `/pitch` is explicitly missing tracker coverage. S02 must create one honest product-repo issue for it instead of leaving it implicit in milestone history.
- **R132:** preserve history instead of silently repurposing tracker rows. This especially affects `hyperpush#8` (prefer transfer over delete/recreate) and the `rewrite_scope` family (tighten existing scope instead of flattening history).
- **R133:** normalize tracker ownership/naming to public `hyperpush`, while keeping local `hyperpush-mono` path evidence only as supporting context.
- **R134:** a maintainer should be able to trust the repo issue lists without `.gsd` archaeology. S02 gets repo issues truthful; S03 then makes the board readable.

## Implementation Landscape

### Canonical inputs from S01
- `.gsd/milestones/M057/slices/S01/reconciliation-ledger.json`
  - Canonical per-issue action source.
  - Relevant row fields for S02:
    - `canonical_issue_handle`
    - `repo`
    - `state`
    - `primary_audit_bucket`
    - `proposed_repo_action_kind`
    - `proposed_repo_action`
    - `project_item_id`
    - `project_backed`
    - `ownership_truth`
    - `delivery_truth`
    - `workspace_path_truth`
    - `public_repo_truth`
    - `normalized_canonical_destination`
  - Also carries `derived_gaps[0]` for `/pitch`.
- `.gsd/milestones/M057/slices/S01/reconciliation-audit.md`
  - Human rollup of the exact mutation buckets; fastest read path for spot-checking grouped actions.
- `.gsd/milestones/M057/slices/S01/mesh-lang-issues.snapshot.json`
- `.gsd/milestones/M057/slices/S01/hyperpush-issues.snapshot.json`
  - Preserve the original titles, bodies, states, and labels before mutation. Use these as the “before” side when generating rewrite bodies or verification diffs.
- `.gsd/milestones/M057/slices/S01/project-items.snapshot.json`
  - Needed only as handoff context for S03. S02 should not mutate project items broadly, but it must emit new issue URLs/numbers for transferred/created issues so S03 can add/remove/update the correct project rows later.

### Existing script/library seams
- `scripts/lib/m057_tracker_inventory.py`
  - Already has reusable GH execution and JSON validation helpers (`run_gh_json`, repo constants, atomic JSON writing style).
  - Do **not** change S01 expected-count constants in place; they describe the captured audit snapshot, not post-S02 live state.
- `scripts/lib/m057_reconciliation_ledger.py`
  - Defines the canonical action enums already in use:
    - `close_as_shipped`
    - `keep_open`
    - `rewrite_scope`
    - `move_to_mesh_lang`
    - `create_missing_issue`
- `scripts/verify-m057-s01.sh`
  - Good model for retained verification logging under `.tmp/.../verify/`.
  - Bad model for post-S02 inventory counts: S01 hard-codes the pre-mutation `68`/`63` snapshot invariants and will intentionally drift once S02 creates/transfers issues.

### Repo ownership / naming truth surfaces
- `scripts/workspace-git.sh`
  - Explicitly accepts actual remote `https://github.com/hyperpush-org/hyperpush` even when local identity still says `hyperpush-mono`.
- `scripts/lib/repo-identity.json`
  - Still records product repo as `hyperpush-org/hyperpush-mono`; that is useful evidence, but **not** the public tracker truth.
- `mesher -> ../hyperpush-mono/mesher`
  - The `mesher` path inside this checkout is a symlink into the sibling product repo. Any path-based tracker claim touching `mesher/*` is product-owned unless it explicitly points into `website/` or other real `mesh-lang` surfaces.

### Issue template / body-shape seams
- `.github/ISSUE_TEMPLATE/bug_report.yml`
- `.github/ISSUE_TEMPLATE/feature_request.yml`
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml`
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml`
  - Both repos already use a simple heading-driven issue shape (`## Outcome`, `## Acceptance criteria`, optionally `## Parent epic` / `## Current state` / `## Scope`).
  - The planner should reuse those shapes for generated rewrite bodies rather than inventing a new issue-body dialect.

### Concrete code-backed proof surfaces referenced by the ledger
- `website/docs/.vitepress/config.mts`
- `website/docs/.vitepress/theme/components/NavBar.vue`
  - Misfiled docs bug (`hyperpush#8`) is genuinely mesh-lang-owned and still reproducible from code.
- `mesher/landing/app/pitch/page.tsx`
  - `/pitch` really exists in the sibling product repo and is the basis for the missing-coverage issue.
- `mesher/frontend-exp/lib/mock-data.ts`
  - Strong proof for the “keep open but rewrite to mock-backed follow-through” family (`#15 #33 #34 #51 #52 #53 #57`).

## Recommendation
Build S02 as **three explicit layers**, with a dry-run first:

1. **Mutation planner (dry-run by default)**
   - Input: S01 ledger + issue snapshots
   - Output under `.gsd/milestones/M057/slices/S02/`:
     - `repo-mutation-plan.json` — one row per live repo mutation with before/after metadata
     - `repo-mutation-plan.md` — human-readable action summary
     - generated body/comment artifacts for rewrites/closures (either inline in JSON or as markdown files)
   - This planner should resolve the exact touched set, including rows where `keep_open` still implies a body/title rewrite.

2. **Repo issue applicator (`--apply`)**
   - Should execute only from the explicit plan.
   - Recommended command families:
     - `gh issue close ... -R <repo> --reason completed --comment <text>` for the 10 open mesh-lang closeouts
     - `gh issue edit ... -R <repo> --title ... --body-file ...` for `rewrite_scope` and “keep-open but rewrite wording” rows
     - `gh issue transfer 8 hyperpush-org/mesh-lang -R hyperpush-org/hyperpush` for the misfiled docs bug
     - `gh issue create -R hyperpush-org/hyperpush ...` for `/pitch`, followed by `gh issue close ... --reason completed --comment ...` if the chosen truthful representation is a retrospective shipped issue
   - Capture stdout for transfer/create commands; those operations produce **new canonical issue URLs/numbers** that S03 will need.

3. **Read-only verification + handoff artifact**
   - Output under `.gsd/milestones/M057/slices/S02/`:
     - `repo-mutation-results.json` — actual applied operations, including old/new issue URL mapping
     - `repo-mutation-results.md` — compact human summary
     - retained verification tree under `.tmp/m057-s02/verify/`
   - This artifact is the S03 handoff for `hyperpush#8`’s new mesh-lang issue URL and the newly created `/pitch` issue URL.

### Strong execution recommendation
Prefer **transfer** over recreate for `hyperpush#8`.

Why:
- it preserves the existing bug history/comments/body (`R132`)
- it avoids a stale closed source issue plus a manually recreated duplicate
- the destination repo (`mesh-lang`) already has the relevant `bug` and `documentation` labels, so transfer should not block on missing basic labels

### `/pitch` recommendation
The ledger’s gap action is `create_missing_issue`, and there is no existing canonical product issue already identified as the honest `/pitch` home. The smallest truthful action is:
- create a dedicated product-repo issue for the shipped `/pitch` surface
- body should state it was shipped in M056 and exists at `mesher/landing/app/pitch/page.tsx`
- close it immediately as `completed` with an evidence comment pointing to `.gsd/milestones/M056/M056-SUMMARY.md`

That preserves history without pretending `/pitch` is still active work.

## Natural Task Seams
1. **Plan generation first**
   - Highest-risk step because this slice mutates live GitHub state.
   - Must distinguish:
     - rows needing real repo edits
     - rows already in desired repo/state (`hyperpush#3/#4/#5`)
     - rows whose project updates are deferred to S03
2. **Identity-changing mutations second**
   - Apply `hyperpush#8` transfer and `/pitch` creation early.
   - Reason: they produce new issue URLs/numbers that later verification and S03 need.
3. **Bulk edit/close pass third**
   - Close the 10 mesh-lang shipped rows.
   - Rewrite the 21 `rewrite_scope` rows.
   - Rewrite the 7 mock-backed follow-through rows.
   - Normalize wording on `#54/#55/#56` only if the current issue text still leaks stale ownership/path framing.
4. **Verification and handoff last**
   - Persist exact old→new mapping and live repo state; do not leave S03 to rediscover it from scratch.

## Verification / Replay
Use **read-only GH checks** after mutation; do not reuse S01’s fixed-count verifier as-is.

### Readiness / environment
- `gh --version`
- `gh auth status`

### Repo state verification
- `gh issue view <n-or-url> -R <repo> --json number,title,state,closedAt,labels,projectItems,url`
  - Best single-command verification for touched rows.
- `gh issue list -R hyperpush-org/mesh-lang --state all --limit 200 --json number,title,state,url`
- `gh issue list -R hyperpush-org/hyperpush --state all --limit 200 --json number,title,state,url`

### Expected live state after S02 repo mutations
- **Repo totals:**
  - `mesh-lang`: `17` issues total (`16` original + transferred `hyperpush#8`)
  - `hyperpush`: `52` issues total (`52` original - transferred `#8` + created `/pitch`)
  - combined: `69`
- **New non-project issue mapping must be explicit in results artifact**
  - old `hyperpush#8` → new `mesh-lang#?`
  - new `/pitch` issue → `hyperpush#?`
- **10 mesh-lang closeouts** must be `CLOSED`
- **21 rewrite-scope rows** must still be `OPEN` but have updated title/body text
- **7 mock-backed follow-through rows** must still be `OPEN` and no longer imply shipped operator-app truth
- **`hyperpush#3/#4/#5`** may remain untouched if they are already closed and unprojected

### Important verification edge cases
- **Do not assert S01’s old `68`-row snapshot invariant after S02.** The slice intentionally creates one new issue and transfers another.
- `gh issue transfer` changes the canonical issue URL/number. Verification must use the returned destination URL/number, not hard-coded assumptions.
- If `/pitch` is created then immediately closed, verification must confirm both:
  - the new issue exists
  - it is `CLOSED` with a close reason/comment path recorded in the local results artifact

## Risks / Unknowns
- **Closed parent with open child risk:**
  - `mesh-lang#3` would close while `mesh-lang#7` stays open.
  - `mesh-lang#5` would close while `mesh-lang#12` stays open.
  - Decide up front whether that is acceptable historical truth or whether the applicator should also rewrite the parent body/checklist so the remaining child scope is explicitly residual follow-up instead of looking like an incomplete close.
- **Transfer label drift:** `hyperpush#8` currently carries `priority: low`, but `mesh-lang` does not have that label. Transfer may drop repo-specific labels. That is acceptable if the results artifact records the final label set, because portfolio priority lives in project fields anyway.
- **S01 verifier drift is intentional after S02:** any executor that reflexively reruns `scripts/verify-m057-s01.sh` after repo mutations will hit count mismatches. S02 needs its own retained verifier.
- **Board drift after repo truth is expected until S03:** project item removal/addition/update is not the primary S02 deliverable. The planner should keep the slice boundary sharp and hand exact new URLs forward.

## Planner-Specific Next Move
Build the dry-run planner first. The main failure mode is not GitHub CLI syntax; it is mutating the wrong rows or failing to preserve the new canonical issue URLs created by transfer/create operations. If the planner outputs one explicit repo mutation manifest plus a results handoff artifact for S03, the rest of the slice is straightforward.