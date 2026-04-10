---
estimated_steps: 3
estimated_files: 4
skills_used:
  - bash-scripting
---

# T02: Build the code-evidence index and naming/ownership map

**Slice:** S01 — Audit code reality and build the reconciliation ledger
**Milestone:** M057

## Description

Turn code and milestone history into reusable classification evidence instead of ad hoc judgment calls. This task is where the ledger becomes code-backed: it should explain why an issue is shipped, partial, active, future, misfiled, or missing coverage by pointing to concrete files and milestone summaries, while also normalizing `hyperpush` vs `hyperpush-mono` naming and repo ownership.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `.gsd` milestone summaries and `.gsd/PROJECT.md` | Fail the evidence build if required milestone proof is missing; do not invent shipped-state claims. | N/A — local file reads should complete immediately. | Reject summaries that cannot supply the expected shipped-state markers for the issue families being classified. |
| Repo identity / workspace helper / product/docs source files | Mark naming/ownership as unresolved and fail the build rather than guessing whether a path is language- or product-owned. | N/A — local file reads should complete immediately. | Reject contradictory repo/url/path combinations until the normalization map resolves them explicitly. |

## Load Profile

- **Shared resources**: repo file reads across language-owned sources and symlinked product surfaces.
- **Per-operation cost**: targeted reads over selected milestone summaries and source files, plus one JSON/markdown evidence bundle write.
- **10x breakpoint**: broad text scans across the symlinked product tree would become noisy first, so the script should stay constrained to named files and issue families.

## Negative Tests

- **Malformed inputs**: stale `hyperpush-mono` slug values, missing `/pitch` route files, and docs bug source files that no longer match the misfiled issue.
- **Error paths**: missing milestone summary, unavailable symlinked `mesher/` surface, or evidence that contradicts canonical repo identity.
- **Boundary conditions**: shipped-but-open mesh issues, misfiled `hyperpush#8`, product `/pitch` shipped without dedicated tracker coverage, and mixed `hyperpush`/`hyperpush-mono` wording.

## Steps

1. Add `scripts/lib/m057_evidence_index.py` that reads `.gsd/PROJECT.md`, M053–M056 summaries, repo-identity/workspace helpers, and the targeted docs/product files from research.
2. Encode reusable evidence entries and a naming/ownership map with `ownership_truth`, `delivery_truth`, `workspace_path_truth`, `public_repo_truth`, and `normalized_canonical_destination` fields.
3. Emit `.json` and `.md` evidence outputs that later ledger assembly can consume directly without rereading the source files.

## Must-Haves

- [ ] Evidence rows cite concrete milestone/file refs instead of issue prose or hand-wavy summaries.
- [ ] The naming/ownership map explicitly distinguishes `workspace_path_truth` from public `hyperpush` repo identity and records the normalized canonical destination.
- [ ] Misfiled and missing-coverage cases (`hyperpush#8`, `/pitch`, and stale `hyperpush-mono` tracker wording) are captured as reusable evidence.

## Verification

- `python3 scripts/lib/m057_evidence_index.py --output-dir .gsd/milestones/M057/slices/S01 --check`
- `rg -n "hyperpush#8|/pitch|workspace_path_truth|public_repo_truth" .gsd/milestones/M057/slices/S01/reconciliation-evidence.md .gsd/milestones/M057/slices/S01/naming-ownership-map.json`

## Inputs

- `.gsd/PROJECT.md` — canonical post-split repo-boundary narrative.
- `.gsd/DECISIONS.md` — current planning/truth-source decisions plus the new artifact-shape decision.
- `.gsd/milestones/M053/M053-SUMMARY.md` — shipped deploy/failover/docs evidence for stale mesh tracker rows.
- `.gsd/milestones/M054/M054-SUMMARY.md` — shipped load-balancing and diagnostics evidence.
- `.gsd/milestones/M055/M055-SUMMARY.md` — authoritative repo split and sibling workspace contract.
- `.gsd/milestones/M056/M056-SUMMARY.md` — shipped `/pitch` evidence for missing tracker coverage.
- `scripts/lib/repo-identity.json` — current repo identity data and stale public `hyperpush-mono` naming.
- `scripts/workspace-git.sh` — helper logic that already acknowledges `hyperpush` vs `hyperpush-mono` drift.
- `mesher/frontend-exp/lib/mock-data.ts` — proof that operator surfaces are still mock-backed in product UI.
- `mesher/landing/app/pitch/page.tsx` — proof that `/pitch` shipped in the product repo.
- `website/docs/.vitepress/config.mts` — one of the actual files named in misfiled issue `hyperpush#8`.
- `website/docs/.vitepress/theme/components/NavBar.vue` — second docs bug surface named by `hyperpush#8`.

## Expected Output

- `scripts/lib/m057_evidence_index.py` — deterministic code/milestone evidence builder.
- `.gsd/milestones/M057/slices/S01/reconciliation-evidence.json` — machine-readable evidence entries for later ledger assembly.
- `.gsd/milestones/M057/slices/S01/reconciliation-evidence.md` — concise human-readable evidence summary.
- `.gsd/milestones/M057/slices/S01/naming-ownership-map.json` — normalized naming and ownership truth map for `mesh-lang` vs `hyperpush`.
