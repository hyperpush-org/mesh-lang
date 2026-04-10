# M057: Cross-Repo Tracker Reconciliation

**Gathered:** 2026-04-09
**Status:** Ready for planning

## Project Description

This milestone is about aligning the project and issues created in the two repos to the actual code state such that it makes sense. The work covers `hyperpush-org/mesh-lang`, `hyperpush-org/hyperpush`, and the shared org roadmap at `https://github.com/orgs/hyperpush-org/projects/1`, using the current local code state in both repos as the truth source.

## Why This Milestone

The code and repo split have moved faster than the public GitHub tracker state. `mesh-lang` has already closed major milestones like M053, M054, M055, and M056 locally, the sibling product repo now owns `mesher/`, `mesher/landing/`, and `mesher/frontend-exp`, and the public Hyperpush launch project still reads as an all-Todo roadmap. That makes the GitHub surface misleading even when the code and repo boundaries are real. This milestone exists to repair that external planning truth without turning tracker cleanup into hidden feature work.

## Codebase Brief

### Technology Stack

- Rust workspace in `mesh-lang` for compiler, runtime, CLI, formatter, LSP, registry, and package tooling
- VitePress docs site under `website/`
- SvelteKit/Vite packages site under `packages-website/`
- Sibling product repo (`../hyperpush-mono`, public repo `hyperpush-org/hyperpush`) containing `mesher/`, `mesher/landing/`, and `mesher/frontend-exp/`
- GitHub Issues, GitHub Projects v2, and `gh` CLI as the live tracker/manipulation surface for this milestone

### Key Modules

- `.gsd/PROJECT.md` — current repo state and milestone sequence
- `.gsd/REQUIREMENTS.md` — explicit capability contract
- `.gsd/milestones/M053/`, `.gsd/milestones/M054/`, `.gsd/milestones/M055/`, `.gsd/milestones/M056/` — recent shipped milestone evidence that the tracker must reflect
- `WORKSPACE.md` and `README.md` — current split-boundary ownership contract in `mesh-lang`
- `../hyperpush-mono/README.md` — current product-repo ownership contract
- GitHub surfaces: `hyperpush-org/mesh-lang`, `hyperpush-org/hyperpush`, and org project `#1`

### Patterns in Use

- Evidence-first planning and closeout through retained milestone summaries and validation artifacts
- Repo-boundary truth after M055: language surfaces stay in `mesh-lang`, product surfaces stay in `hyperpush`
- Public tracker state should follow code and ownership truth, not the other way around
- External mutations should be driven by an explicit audit ledger, not ad hoc cleanup

## User-Visible Outcome

### When this milestone is complete, the user can:

- Open `hyperpush-org/mesh-lang`, `hyperpush-org/hyperpush`, and org project `#1` and understand what is shipped, active, and deferred without relying on `.gsd` archaeology
- Trust that completed work is closed, active work is tracked in the correct repo, and the org roadmap reflects reconciled issue truth instead of stale all-Todo launch planning

### Entry point / environment

- Entry point: GitHub Issues for both repos plus GitHub Projects v2 org project `#1`
- Environment: local dev workspace for evidence gathering plus live GitHub metadata surfaces for reconciliation
- Live dependencies involved: GitHub issues, GitHub Projects v2, `gh` CLI, local sibling repo `../hyperpush-mono`

## Completion Class

- Contract complete means: both repo issue sets and org project `#1` are reconciled against an explicit audit ledger with evidence for every close, rewrite, split, create, or keep decision
- Integration complete means: `mesh-lang`, `hyperpush`, and project `#1` all point at the same canonical issue truth and correct repo ownership
- Operational complete means: a fresh maintainer can use GitHub alone to understand the current roadmap state without the board reading as globally stale or all-Todo

## Architectural Decisions

### Local code state is the source of truth

**Decision:** Use the current local code state in `mesh-lang` and the sibling product repo as the authority for reconciliation, then make GitHub issues and the org project conform to that reality.

**Rationale:** The repos have already shipped and split in ways the tracker still does not express. Using stale GitHub issue text as the truth source would preserve drift instead of repairing it.

**Evidence:** Local `.gsd` milestone history shows M053, M054, M055, and M056 complete; the sibling product repo README now owns `mesher/`, `mesher/landing/`, and `mesher/frontend-exp`; the org project still reports 63 Todo items and 0 Done.

**Alternatives Considered:**
- Treat GitHub issue state as truth — rejected because the tracker is exactly what is stale.
- Use only remote default-branch text, ignoring the local sibling workspace — rejected because the user explicitly chose local code state as the truth source.

### Reconcile through an explicit audit ledger before mutating GitHub

**Decision:** S01 produces a row-by-row reconciliation ledger first; no issue or project writes are considered valid until that ledger exists.

**Rationale:** The dangerous failure mode here is false-green cleanup: a cleaner-looking board that is less truthful. An audit ledger keeps evidence, action type, and canonical ownership explicit before any mutation happens.

**Evidence:** The current project is all Todo, repo issues have no milestone linkage, and some Mesh issues still describe work already closed locally by recent milestones.

**Alternatives Considered:**
- Edit issues and project items ad hoc while auditing — rejected because it makes tracker drift harder to reason about and easier to overfit.
- Rebuild the board from scratch without a ledger — rejected because it would destroy historical continuity and make closure decisions opaque.

### Reconcile repo issues before org project items

**Decision:** Fix repo issue truth first, then realign org project `#1` to the reconciled issue set.

**Rationale:** The org project is the portfolio layer, not the canonical work-record layer. If issue truth is still wrong, board truth cannot be reliable.

**Evidence:** The project currently shows all 63 items as Todo; issue state and board state are already clearly drifting independently.

**Alternatives Considered:**
- Repair the project first and leave repo issues for later — rejected because the board would still be downstream of stale issue meaning.
- Treat project items as the only truth and ignore repo issues — rejected because execution and ownership still live at the repo issue level.

### Use project fields for cross-repo portfolio truth and keep repo milestones optional

**Decision:** Use GitHub Projects fields/status as the cross-repo portfolio layer after reconciliation, while using repo milestones only where they help local grouping.

**Rationale:** Cross-repo milestone synchronization is weak, but project fields are designed to express the portfolio view. That keeps issue ownership local and the board org-wide.

**Evidence:** GitHub’s current Issues/Projects model emphasizes issue types, sub-issues, and project fields for structured planning, while cross-repo milestone synchronization remains awkward.

**Alternatives Considered:**
- Lean on repo milestones as the main cross-repo planning surface — rejected because that does not scale cleanly across both repos.
- Ignore milestones and fields entirely and use labels only — rejected because the current tracker already shows the limits of label-only planning.

## Interface Contracts

- **Audit ledger contract:** each row must include subject (`repo issue` or `project item`), current state, code/repo evidence, proposed action, and canonical destination after reconciliation.
- **Repo issue contract:** every active initiative should have one canonical issue in the correct repo; completed issues close with evidence; drifted issues are rewritten or split instead of silently repurposed.
- **Project contract:** every active org-project item must point to a canonical repo issue, and project status is derived from the reconciled issue truth instead of set independently.
- **Naming/ownership contract:** tracker wording must distinguish language-owned work from product-owned work and normalize stale `hyperpush-mono` naming to the public `hyperpush` repo identity where that is the truthful external surface.

## Error Handling Strategy

- Close issues only with concrete code/repo evidence.
- Never close work based on project text alone.
- Preserve history: when an item is partially shipped, rewrite it to the remaining truthful scope or split it into follow-up issues.
- If ownership is wrong, move to the correct canonical issue path instead of forcing the stale issue to keep an inaccurate meaning.
- Update org project state only after repo issue truth is corrected.
- If evidence is ambiguous, keep the item open with clarified scope rather than force-closing it.
- Treat concurrent GitHub edits as a real risk: the audit ledger is the guardrail, and project writes should happen last.

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- `mesh-lang` issues now match the actual language-repo code, docs, workflow, and split-boundary reality
- `hyperpush` issues now match the actual product-repo code and ownership reality
- org project `#1` now reflects the reconciled cross-repo issue set instead of remaining a stale all-Todo launch board

## Testing Requirements

- Produce and retain an explicit reconciliation ledger before any external tracker mutation.
- Verify representative issue closures, rewrites, splits, and creations directly in both repos after mutation.
- Verify representative project items from both repos after reconciliation, including at least one done, one active, and one deferred/future path.
- Spot-check tracker claims against code/repo evidence in `.gsd`, `README.md`, `WORKSPACE.md`, and the sibling product repo.
- Prefer tracker-state verification and evidence review over synthetic unit tests; this milestone is about truthful external state, not code-path coverage.

## Acceptance Criteria

### S01 — Audit code reality and build the reconciliation ledger
- Every in-scope repo issue and org project item is classified against current code reality.
- The reconciliation ledger records evidence, proposed action, and canonical ownership per item.
- Naming and ownership drift between `mesh-lang`, `hyperpush`, and historical `hyperpush-mono` wording is explicitly resolved.
- No GitHub mutation is required to understand the full planned cleanup.

### S02 — Reconcile repo issues across `mesh-lang` and `hyperpush`
- Completed work identified in S01 is closed or rewritten into truthful follow-up scope.
- Active work is represented in the correct repo.
- Missing tracker coverage identified in S01 is created where necessary.
- Duplicate or overlapping issues are resolved into one canonical issue path.

### S03 — Realign org project `#1` to the reconciled issue truth
- Project status and fields reflect the reconciled repo issue set instead of stale pre-reconciliation state.
- Done work is marked done or removed from active roadmap views.
- Domain, priority, status, and date fields are coherent enough that the board reads meaningfully.
- A fresh maintainer can use the project to understand what is done, active, and next.

## Risks and Unknowns

- Partially shipped work may look complete in one repo and still active in another — closing too aggressively would destroy truthful history.
- Project items can diverge from issue truth if project reconciliation happens before repo issue reconciliation.
- Cross-repo naming and ownership drift (`hyperpush-mono` vs `hyperpush`) can keep work filed in the wrong place even when the code is correct.
- Human edits during the reconciliation pass could create races between the audit ledger and the live GitHub state.

## Existing Codebase / Prior Art

- `.gsd/milestones/M053/M053-SUMMARY.md` — recent hosted/docs/packages closeout that GitHub tracker state should now reflect
- `.gsd/milestones/M055/M055-SUMMARY.md` — split-boundary ownership contract between `mesh-lang` and `hyperpush`
- `.gsd/milestones/M056/M056-SUMMARY.md` — recent product-repo `/pitch` work that may still be misrepresented in current trackers
- `WORKSPACE.md` — current repo-boundary ownership contract
- `README.md` — current language-repo public and maintainer-facing contract
- `../hyperpush-mono/README.md` — current product-repo ownership and tooling contract

> See `.gsd/DECISIONS.md` for all architectural and pattern decisions — it is an append-only register; read it during planning, append to it during execution.

## Relevant Requirements

- R128 — `mesh-lang` issues reflect actual language-repo code state
- R129 — `hyperpush` issues reflect actual product-repo code state
- R130 — org project `#1` reflects reconciled cross-repo issue truth
- R131 — missing tracker coverage is created where reality has no honest issue or project item
- R132 — reconciliation preserves history instead of hiding drift
- R133 — repo ownership and naming are normalized across tracker surfaces
- R134 — a fresh maintainer can understand shipped, active, and deferred work from GitHub surfaces alone

## Scope

### In Scope

- Auditing both repos and org project `#1` against actual local code and ownership reality
- Closing, rewriting, splitting, or creating GitHub issues where the audit proves it is necessary
- Normalizing repo ownership, naming, and cross-repo grouping
- Realigning org project statuses and fields to the reconciled issue truth

### Out of Scope / Non-Goals

- Implementing stale roadmap items as part of reconciliation
- Changing code solely to make old issue wording appear accurate
- Inventing a brand-new strategic roadmap unrelated to current repo reality

## Technical Constraints

- The milestone uses local code state in both repos as the truth source.
- GitHub writes must follow the audit ledger; ad hoc tracker cleanup is out of scope.
- Repo milestones may be used where helpful, but project fields remain the cross-repo portfolio surface.
- The milestone must preserve historical continuity rather than flattening everything into a new board.

## Integration Points

- `hyperpush-org/mesh-lang` — language-repo issues and labels
- `hyperpush-org/hyperpush` — product-repo issues and labels
- `https://github.com/orgs/hyperpush-org/projects/1` — cross-repo portfolio surface
- `gh` CLI — read/write path for issue and project reconciliation
- local sibling repo `../hyperpush-mono` — product truth source during audit

## Ecosystem Notes

- GitHub’s current Issues/Projects model emphasizes structured issue management through issue types and sub-issues, which fits this milestone’s need to keep real umbrella issues without flattening everything into labels.
- Projects v2 is the right cross-repo portfolio layer, but cross-repo milestone synchronization is still awkward; that supports keeping project fields as the portfolio truth and repo milestones as optional local grouping only.
- `gh` CLI is sufficient for most issue/project work but project operations remain clunkier than ordinary issue edits, so the mutation order matters: issue truth first, project truth second.

## Open Questions

- Should repo milestones be created broadly during reconciliation, or only when they clearly help local grouping? — Current recommendation: keep them selective and let the org project carry the main cross-repo portfolio truth.
