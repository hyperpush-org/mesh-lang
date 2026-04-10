# Slice S01 Research — Audit code reality and build the reconciliation ledger

## Summary
- S01 directly supports `R128`–`R134`. The hard part is not GitHub mechanics; it is source-of-truth conflict. GitHub’s canonical product repo is `hyperpush-org/hyperpush`, while the local workspace path, older docs, and repo-identity helpers still frequently encode `hyperpush-mono`.
- There is no existing tracker-sync tooling in this repo. Repo/project inventory must be fetched live via `gh`, joined to local milestone evidence, and written into a single canonical ledger.
- Because org project `#1` contains only issue-backed items (`63` issues, `0` drafts), one **issue-centric** ledger keyed by canonical issue URL can cover both repo issues and project items. Non-project issues become rows with blank `project_item_id`.
- Highest-confidence stale tracker items are `mesh-lang` `#3-#10` and `#6/#13/#14` (strong M053–M055 evidence but still open), plus `hyperpush#8` (misfiled mesh-lang docs bug). Highest-confidence still-open items are Mesh observability `#15-#18`, Hyperpush operator-app wiring `#34/#52/#57`, and product deployment `#23/#50/#55/#56`.

## Skills Discovered
- Existing skill: `gh` is already installed and directly relevant. No additional skill installs were needed.
- Relevant `gh` skill rules used here:
  - repo-detection rule: pass explicit repo on `gh issue ...` commands; `gh repo view` is the exception and takes a positional repo argument.
  - Projects V2 guidance: use `gh project field-list` plus `gh api graphql` for structured project-item/field inventory instead of inferring project state from issue text.

## Requirements Coverage
- `R128` / `R129`: the ledger must classify every repo issue against actual code, not issue prose.
- `R130`: because every project item is an issue, project reconciliation can be derived from the same ledger if `project_item_id` and field snapshots are captured now.
- `R131`: missing coverage is already visible — open product bug `#8` is not on the project, and shipped `/pitch` work has no dedicated tracker item.
- `R132`: several issues are partially true rather than wholly wrong; planner should prefer rewrite/split over blunt close.
- `R133`: naming drift is real and structural (`hyperpush` vs `hyperpush-mono`).
- `R134`: current public tracker fails this bar: project status is `Todo` for all `63` items and many project fields are missing.

## Ground-truth files and seams
- `.gsd/PROJECT.md`
  - Current canonical narrative of shipped state.
  - Says the public product repo is `hyperpush-org/hyperpush`, with `hyperpush-mono` only the local workspace path.
  - Explicitly frames M057 as tracker reconciliation work.
- `.gsd/milestones/M053/M053-SUMMARY.md`
  - Durable evidence that deploy truth, staged Postgres starter, hosted verification, and docs contract shipped.
  - Strong evidence against leaving mesh issues `#3`, `#4`, `#5`, `#6`, `#7`, `#8`, `#9`, `#10` wholly open as-written.
- `.gsd/milestones/M054/M054-SUMMARY.md`
  - Durable evidence for one-public-URL load-balancing truth and `X-Mesh-Continuity-Request-Key` follow-through.
  - Strong evidence for rewriting/closing runtime-hardening and diagnostics items.
- `.gsd/milestones/M055/M055-SUMMARY.md`
  - Durable evidence for the repo split and sibling workspace contract.
- `.gsd/milestones/M056/M056-SUMMARY.md`
  - Durable evidence that `/pitch` shipped in the product repo.
- `scripts/lib/repo-identity.json`
  - Still encodes product slug/URLs as `hyperpush-org/hyperpush-mono`.
  - This is stale versus GitHub canonical repo identity.
- `scripts/workspace-git.sh`
  - Confirms the drift explicitly: it accepts actual remote `https://github.com/hyperpush-org/hyperpush` when expected value is `.../hyperpush-mono`.
  - Use this as proof that rename drift is known locally, not accidental.
- `mesher -> ../hyperpush-mono/mesher`
  - Local `mesh-lang/mesher` is a symlink into the sibling repo, not mesh-lang-owned code.
  - Any tracker row referencing `mesh-lang/mesher/...` should be treated as product-owned.
- `../hyperpush-mono/README.md`
  - Still claims canonical product repo URL is `https://github.com/hyperpush-org/hyperpush-mono`.
  - This conflicts with GitHub canonical identity and `.gsd/PROJECT.md`.
- `../hyperpush-mono/mesher/README.md`
  - Confirms Mesher is real, current, and backed by PostgreSQL/runtime plus maintainer proof rails.
- `../hyperpush-mono/mesher/frontend-exp/README.md`
  - Still generic v0 scaffold copy.
- `../hyperpush-mono/mesher/frontend-exp/app/page.tsx`
  - Operator UI is still mock-driven and local-state-only.
- `../hyperpush-mono/mesher/frontend-exp/lib/mock-data.ts`
  - Strong evidence that issue/dashboard/performance/Solana/Copilot surfaces are not yet real product truth.
- `../hyperpush-mono/mesher/api/*.mpl`
  - Real backend surfaces already exist for dashboard, search/issues, detail, alerts, settings, team/API keys.
- `../hyperpush-mono/mesher/ingestion/ws_handler.mpl`
  - Real WebSocket stream surface exists (`/stream/projects/:id`) for live updates.
- `../hyperpush-mono/mesher/services/stream_manager.mpl`
  - Real per-connection subscription/backpressure/live-stream plumbing exists; this matters for `#52`.
- `../hyperpush-mono/mesher/landing/app/page.tsx`
  - Landing is marketing content, not the real operator app.
- `../hyperpush-mono/mesher/landing/app/pitch/page.tsx`
  - `/pitch` exists in the product repo.
- `../hyperpush-mono/.github/workflows/deploy-landing.yml`
  - Only landing has an actual deploy workflow.
- `../hyperpush-mono/mesher/landing/Dockerfile`
  - Only landing has a production container path.
- `../hyperpush-mono/mesher/landing/lib/external-links.ts`
  - Still exposes `hyperpush-mono` slug publicly.
- `website/docs/.vitepress/config.mts`
- `website/docs/.vitepress/theme/components/NavBar.vue`
  - These are the exact current bug surface cited by product issue `hyperpush#8`; the issue is misowned and still reproducible from code.

## Live GitHub inventory
- `mesh-lang`
  - `16` total issues, `16` open, `0` closed, `0` milestones.
  - All `16` are labeled `roadmap`; `5` are epics.
  - All `16` appear on org project `#1`.
- `hyperpush`
  - GitHub canonical repo is `hyperpush-org/hyperpush` (`gh repo view hyperpush-org/hyperpush-mono` resolves to `hyperpush-org/hyperpush`).
  - `52` total issues, `48` open, `4` closed, `0` milestones.
  - `47` issues appear on org project `#1`.
  - `5` issues are not on the project: closed `#2-#5` plus open bug `#8`.
- org project `hyperpush-org` project `#1`
  - `63` items total.
  - All `63` items are real issues; there are **0 draft items**.
  - Repo split on the board: `16` `mesh-lang`, `47` `hyperpush`.
  - `Status` is `Todo` for all `63` items.
  - Field completeness is poor:
    - `Domain` missing on `23` items
    - `Track` missing on `24` items
    - `Commitment` missing on `17` items
    - `Delivery Mode` missing on `23` items
    - `Priority` missing on `17` items
    - `Hackathon Phase` missing on `17` items
    - `Start date` missing on `17` items
    - `Target date` missing on `17` items
- Project readme already encodes intended cross-repo semantics and manual view setup.
  - S03 should realign items/fields/statuses, not try to automate saved-view creation.

## Highest-confidence classification signals
### Mesh tracker
- Likely stale / rewrite-or-close candidates:
  - `#3/#7/#8` — deploy baseline and VM deployment claims now have strong M053 evidence.
  - `#4/#9/#10` — failover/diagnostics claims now have strong M053+M054 evidence.
  - `#6/#13/#14` — docs/install/packages/product-handoff work now has strong M053+M055 evidence, but naming drift means some rows are probably “rewrite to remaining cleanup” rather than pure close.
- Ambiguous / needs manual row audit:
  - `#5/#11/#12` — release/deploy verification signals clearly shipped in M053, but “rollback/regression primitives” may only be partially satisfied. Do not auto-close this family.
- Likely still-open future work:
  - `#15/#16/#17/#18` — no strong local evidence of a built observability canvas, actor lifecycle event stream, or message-flow visualization. Existing docs explicitly treat client-visible topology as a non-goal today.

### Hyperpush tracker
- Misfiled:
  - `hyperpush#8` is a `mesh-lang` docs-site bug; body references `website/docs/.vitepress/config.mts` and `website/docs/.vitepress/theme/components/NavBar.vue`.
- Clearly still open:
  - `#34/#52/#57` — frontend-exp is still wired to `lib/mock-data.ts`; Mesher APIs already exist, so these are wiring/integration tasks, not backend invention.
  - `#23/#50/#55/#56` — product deployment is incomplete: landing has Dockerfile/Fly deploy, but there is no frontend-exp Dockerfile or compose-style full-stack VM deployment path.
  - JS/Go SDK, SaaS onboarding, AI Copilot, GitHub loop, bug market, and Solana economy epics have no obvious shipped code surfaces in the local repo.
- Likely partial / rewrite candidates:
  - `#15/#33` — landing is already marketing-only in structure, but frontend-exp is not yet a real operator app. These should likely be narrowed to the remaining app-promotion work, not left as if both halves are untouched.
  - `#29/#30` — issue bodies already acknowledge Mesher has real backend surfaces. These are “audit gaps and close product parity gaps,” not greenfield items. Keep open, but preserve that truthful narrower meaning.
- Missing tracker coverage already visible:
  - `/pitch` shipped (M056) but there is no dedicated product issue for it.
  - If M056 needs public tracker history, S02 may need a retrospective closed issue or a rewrite of a broader landing item.

## Naming/ownership drift to preserve in the ledger
- Canonical external repo identity on GitHub is `hyperpush-org/hyperpush`.
- Local workspace path is still `../hyperpush-mono` and `mesh-lang/mesher` is a symlink into it.
- Many local public docs/scripts still say `hyperpush-org/hyperpush-mono`.
- Quantitatively, `rg -l` found `40` mesh-lang files and `7` product-repo files still referencing the old `hyperpush-mono` slug, so this is mixed-state, not a single stray reference.
- Recommendation: the ledger should separate:
  - `workspace_path_truth`: `hyperpush-mono`
  - `public_repo_truth`: `hyperpush`
  - `current_tracker_wording`
  - `normalized_canonical_destination`
- Do not use README text alone as truth for naming. Rank evidence:
  1. GitHub canonical repo (`gh repo view`)
  2. `.gsd/PROJECT.md`
  3. workspace helper alias logic (`scripts/workspace-git.sh`)
  4. older README/docs/test references

## Recommendation
Build **one issue-centric reconciliation ledger**, not separate issue and project ledgers.

Recommended row shape:
- `repo`
- `issue_number`
- `issue_url`
- `title`
- `state`
- `labels`
- `project_item_id` (blank if not on project)
- `project_status`
- `project_domain`
- `project_track`
- `project_commitment`
- `project_delivery_mode`
- `project_priority`
- `project_start_date`
- `project_target_date`
- `evidence_refs` (milestone summaries, file paths, workflow files, API modules, mock-data files)
- `ownership_truth` (`mesh-lang`, `hyperpush`, `misfiled`, `ambiguous`)
- `delivery_truth` (`shipped`, `partial`, `active`, `future`, `closed-correctly`)
- `proposed_repo_action` (`close`, `rewrite`, `split`, `keep-open`, `move/create-canonical`, `no-change`)
- `proposed_project_action` (`mark-done`, `set-active-fields`, `remove-from-project`, `add-to-project`, `defer`)
- `canonical_issue_url`
- `notes`

Because the project has only issue-backed items, this single ledger can satisfy S01’s “every repo issue and project item” contract if it covers all `68` repo issues and includes project columns for the `63` represented issues.

## Natural task seams for the planner
1. **Inventory capture**
   - Fetch raw issue inventories from both repos and a raw project inventory with field values + `project_item_id`.
   - Persist raw JSON snapshots under the slice directory so S02/S03 do not requery live state blindly.
2. **Evidence mapping**
   - Build a compact evidence index from `.gsd/PROJECT.md`, M053/M054/M055/M056 summaries, repo identity files, product README, frontend-exp mock surfaces, Mesher API/service files, landing/deploy surfaces, and the mesh docs nav bug files.
3. **Ledger assembly**
   - Join repo issues + project items into one row set keyed by canonical issue URL.
   - Add evidence refs and proposed actions.
4. **Human-readable audit summary**
   - Derive rollups: shipped-but-open candidates, misfiled items, naming-drift items, missing project coverage, ambiguous rows needing careful judgment.

## Verification / replay commands
Use these commands as the stable audit/read path:
- repo issue inventories:
  - `gh issue list -R hyperpush-org/mesh-lang --state all --limit 200 --json number,title,state,labels,body,url,createdAt,updatedAt,closedAt`
  - `gh issue list -R hyperpush-org/hyperpush --state all --limit 200 --json number,title,state,labels,body,url,createdAt,updatedAt,closedAt`
- canonical repo identity:
  - `gh repo view hyperpush-org/hyperpush --json nameWithOwner,url`
  - `gh repo view hyperpush-org/hyperpush-mono --json nameWithOwner,url` (confirms redirect/canonicalization)
- project schema:
  - `gh project field-list 1 --owner hyperpush-org`
- project item inventory:
  - use `gh api graphql` against `organization(login:"hyperpush-org") { projectV2(number:1) { items(first:100) { nodes { id content { ... on Issue { number title state url repository { nameWithOwner } } } fieldValues(first:30) { ...single-select/date values... } } } } }`
  - capture `id` for later item mutation; `gh project item-list 1 --owner hyperpush-org` is fine for quick inspection, but GraphQL is the reliable structured path.
- sanity checks that should pass once the ledger exists:
  - `68` repo issue rows total (`16` mesh-lang + `52` hyperpush)
  - `63` rows with non-empty `project_item_id`
  - `5` repo issue rows with blank `project_item_id` (`hyperpush` closed `#2-#5` and open `#8`)
  - `0` project items without a matching repo issue row
  - every row has non-empty `evidence_refs`, `ownership_truth`, `delivery_truth`, and proposed repo/project actions

## Risks / unknowns
- The codebase itself is not fully normalized on `hyperpush` vs `hyperpush-mono`; some rows will need naming-conflict notes instead of simplistic closure decisions.
- Mesh `#5/#11/#12` are the main false-close risk. Verification signals shipped; rollback/regression semantics may not have.
- If S01 only writes a markdown narrative and not raw JSON snapshots + item IDs, S02/S03 will waste time and may mutate the wrong live objects.
- Project view/readme setup is partly manual by design; do not turn missing saved views into scope creep.
- Since `mesh-lang` contains a `mesher` symlink, any path-based audit that ignores symlink ownership will misclassify product work as language work.

## Planner-specific next move
Build the raw inventories first, then the joined ledger. The critical proof bar is not “can we talk to GitHub,” it is “can one canonical row explain issue truth, project truth, repo ownership, and evidence without live re-investigation.” Once that exists, S02/S03 can execute deterministically.
