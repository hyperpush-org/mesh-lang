# M057 S01 Reconciliation Evidence

- Version: `m057-s01-reconciliation-evidence-v1`
- Inventory captured_at: `2026-04-10T06:20:21Z`
- Generated at: `2026-04-10T06:20:36Z`
- Evidence entries: `5`
- Matched issue URLs: `21`

## Decision anchors

- `.gsd/DECISIONS.md:462` — D454 defines the three-layer artifact split for raw snapshots, derived evidence, and the final ledger.
- `.gsd/DECISIONS.md:463` — D455 defines the canonical issue URL inventory shape that this evidence index consumes.
- `.gsd/DECISIONS.md:464` — D456 fixes the downstream naming contract by keeping workspace-path truth separate from public repo truth and canonical tracker destination.

## Rollup

| Evidence ID | Classification | Matched issues | Canonical destination |
| --- | --- | --- | --- |
| `mesh_launch_foundations_shipped` | `shipped` | mesh-lang#3, mesh-lang#4, mesh-lang#5, mesh-lang#6, mesh-lang#8, mesh-lang#9, mesh-lang#10, mesh-lang#11, mesh-lang#13, mesh-lang#14, hyperpush#3, hyperpush#4, hyperpush#5 | `hyperpush-org/mesh-lang | tracker:mesh-lang | workspace:mesh-lang` |
| `frontend_exp_operator_surfaces_partial` | `partial` | hyperpush#15, hyperpush#33, hyperpush#34, hyperpush#51, hyperpush#52, hyperpush#53, hyperpush#57 | `hyperpush-org/hyperpush | tracker:hyperpush | workspace:hyperpush-mono` |
| `hyperpush_8_docs_bug_misfiled` | `misfiled` | hyperpush#8 | `hyperpush-org/mesh-lang | tracker:mesh-lang | workspace:mesh-lang` |
| `pitch_route_missing_tracker_coverage` | `missing_coverage` | _none_ | `hyperpush-org/hyperpush | tracker:hyperpush | workspace:hyperpush-mono` |
| `product_repo_naming_normalization` | `active` | _none_ | `hyperpush-org/hyperpush | tracker:hyperpush | workspace:hyperpush-mono | compat:mesher -> ../hyperpush-mono/mesher` |

## mesh_launch_foundations_shipped

Deploy, failover, diagnostics, release-verification, and public-surface truth for the Mesh launch path were already shipped in M053 and M054 even though multiple mesh-lang tracker rows remain open.

- classification: `shipped`
- ownership_truth: language-owned delivery work in mesh-lang
- delivery_truth: shipped across M053 and M054; the tracker rows are stale shipped-but-open language work
- workspace_path_truth: mesh-lang compiler/scripts/docs surfaces remain tracked directly in this repo
- public_repo_truth: hyperpush-org/mesh-lang
- normalized_canonical_destination: `{"issue_repo": "mesh-lang", "repo_slug": "hyperpush-org/mesh-lang", "workspace_root": "mesh-lang"}`
- matched_issues: mesh-lang#3, mesh-lang#4, mesh-lang#5, mesh-lang#6, mesh-lang#8, mesh-lang#9, mesh-lang#10, mesh-lang#11, mesh-lang#13, mesh-lang#14, hyperpush#3, hyperpush#4, hyperpush#5
- proposed_tracker_action: Close or rewrite the shipped-but-open tracker rows so they point at the delivered M053/M054 contract instead of describing those launch foundations as pending work.
- evidence_refs:
- `.gsd/milestones/M053/M053-SUMMARY.md:46` — M053 summary records the staged deploy bundle and serious deploy contract as shipped.
- `.gsd/milestones/M053/M053-SUMMARY.md:50` — M053 summary records docs/public-surface truth for the serious deployable starter.
- `.gsd/milestones/M054/M054-SUMMARY.md:53` — M054 summary records the bounded one-public-URL runtime contract as shipped.
- `.gsd/milestones/M054/M054-SUMMARY.md:11` — M054 summary records shipped direct request-key follow-through and diagnostics truth.
- `https://github.com/hyperpush-org/mesh-lang/issues/3` — Representative shipped-but-open mesh-lang issue in the deploy/runtime family.

## frontend_exp_operator_surfaces_partial

frontend-exp exists, but the checked-in operator UI is still backed by MOCK_ISSUES and MOCK_STATS, so the app/backend replacement work remains partially delivered rather than shipped.

- classification: `partial`
- ownership_truth: product-owned operator app work
- delivery_truth: partial only; real backend/data workflows for the operator app remain open
- workspace_path_truth: mesher/frontend-exp/lib/mock-data.ts via the compatibility symlink into the sibling product repo
- public_repo_truth: hyperpush-org/hyperpush
- normalized_canonical_destination: `{"issue_repo": "hyperpush", "repo_slug": "hyperpush-org/hyperpush", "surface_path": "mesher/frontend-exp/lib/mock-data.ts", "workspace_root": "hyperpush-mono"}`
- matched_issues: hyperpush#15, hyperpush#33, hyperpush#34, hyperpush#51, hyperpush#52, hyperpush#53, hyperpush#57
- proposed_tracker_action: Keep the product app/backend issues open, and describe them as mock-backed frontend-exp follow-through instead of implying the real operator app already shipped.
- evidence_refs:
- `mesher/frontend-exp/lib/mock-data.ts:44` — frontend-exp still exports hard-coded issue data.
- `mesher/frontend-exp/lib/mock-data.ts:211` — frontend-exp still exports hard-coded dashboard stats.
- `.gsd/PROJECT.md:7` — Project contract names frontend-exp as a product-owned surface.

## hyperpush_8_docs_bug_misfiled

hyperpush#8 is a real website/docs bug, but the issue body points only at mesh-lang-owned VitePress files, so the tracker row belongs in mesh-lang rather than the product repo.

- classification: `misfiled`
- ownership_truth: language-owned docs surface
- delivery_truth: active docs bug on mesh-lang-owned files; current repo placement is wrong
- workspace_path_truth: website/docs/.vitepress/* tracked directly in mesh-lang
- public_repo_truth: hyperpush-org/mesh-lang
- normalized_canonical_destination: `{"issue_repo": "mesh-lang", "repo_slug": "hyperpush-org/mesh-lang", "surface_paths": ["website/docs/.vitepress/config.mts", "website/docs/.vitepress/theme/components/NavBar.vue"], "workspace_root": "mesh-lang"}`
- matched_issues: hyperpush#8
- proposed_tracker_action: Move or recreate hyperpush#8 under mesh-lang and relabel it as a language-repo docs/packages-nav bug.
- evidence_refs:
- `https://github.com/hyperpush-org/hyperpush/issues/8` — Issue body explicitly cites mesh-lang docs files and asks for packages.meshlang.dev routing.
- `website/docs/.vitepress/config.mts:218` — The nav config cited by hyperpush#8 is a mesh-lang docs file.
- `website/docs/.vitepress/theme/components/NavBar.vue:17` — The custom NavBar cited by hyperpush#8 is also a mesh-lang docs file.

## pitch_route_missing_tracker_coverage

The evaluator-facing /pitch route already shipped in the sibling product repo during M056, but there is still no dedicated repo issue or project row that records that delivered surface.

- classification: `missing_coverage`
- ownership_truth: product-owned landing surface
- delivery_truth: shipped in M056 without dedicated tracker coverage
- workspace_path_truth: mesher/landing/app/pitch/page.tsx via compatibility symlink into the sibling product repo
- public_repo_truth: hyperpush-org/hyperpush
- normalized_canonical_destination: `{"issue_repo": "hyperpush", "repo_slug": "hyperpush-org/hyperpush", "surface_path": "mesher/landing/app/pitch/page.tsx", "workspace_root": "hyperpush-mono"}`
- matched_issues: _none_
- proposed_tracker_action: Create or repoint one hyperpush tracker row so the shipped /pitch route is represented explicitly instead of being implied only by milestone history.
- evidence_refs:
- `.gsd/milestones/M056/M056-SUMMARY.md:36` — M056 summary records /pitch as shipped.
- `mesher/landing/app/pitch/page.tsx:4` — The shipped /pitch route still exists in the product surface.
- `.gsd/PROJECT.md:20` — Project contract says /pitch lives in the sibling product repo after the split.

## product_repo_naming_normalization

Tracker wording still has to normalize stale hyperpush-mono references to the public hyperpush repo identity whenever a product-owned row points at mesher, landing, or frontend-exp work.

- classification: `active`
- ownership_truth: product-owned in the Hyperpush product repo
- delivery_truth: naming normalization remains active reconciliation work, even though the repo split itself already shipped in M055
- workspace_path_truth: local compatibility path mesher -> ../hyperpush-mono/mesher; workspace helper canonicalizes both hyperpush and hyperpush-mono to workspace root hyperpush-mono
- public_repo_truth: hyperpush-org/hyperpush
- normalized_canonical_destination: `{"compatibility_path": "mesher -> ../hyperpush-mono/mesher", "issue_repo": "hyperpush", "repo_slug": "hyperpush-org/hyperpush", "workspace_root": "hyperpush-mono"}`
- matched_issues: _none_
- proposed_tracker_action: Rewrite stale hyperpush-mono repo mentions on repo issues and project items to the canonical public hyperpush destination while preserving the local hyperpush-mono workspace path only as implementation detail.
- evidence_refs:
- `.gsd/PROJECT.md:7` — Project contract distinguishes the local workspace alias from the public product repo identity.
- `scripts/workspace-git.sh:32` — workspace-git already accepts the public hyperpush remote as the canonical repo identity.
- `hyperpush-issues.snapshot.json/canonical_redirect` — Live GitHub inventory capture canonicalized hyperpush-mono to hyperpush.
- `.gsd/milestones/M055/M055-SUMMARY.md:31` — M055 summary records the split and naming/ownership reset as shipped.
