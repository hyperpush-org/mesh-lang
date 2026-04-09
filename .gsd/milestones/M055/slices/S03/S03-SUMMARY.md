---
id: S03
parent: M055
milestone: M055
provides:
  - Repo-boundary product handoff metadata and public contract across scaffold/docs/proof surfaces.
  - Language-only deploy/public workflow graph and fail-closed source/hosted verifiers that forbid landing drift.
  - A slice-owned assembled verifier and retained proof bundle under `.tmp/m055-s03/verify/`.
requires:
  - slice: S01
    provides: Canonical repo identity split, workspace authority contract, and blessed sibling-repo boundary.
affects:
  - S04
key_files:
  - scripts/lib/repo-identity.json
  - compiler/mesh-pkg/src/scaffold.rs
  - README.md
  - website/docs/docs/getting-started/index.md
  - website/docs/docs/getting-started/clustered-example/index.md
  - website/docs/docs/tooling/index.md
  - website/docs/docs/distributed/index.md
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/production-backend-proof/index.md
  - website/docs/docs/web/index.md
  - website/docs/docs/databases/index.md
  - website/docs/docs/testing/index.md
  - website/docs/docs/concurrency/index.md
  - tools/skill/mesh/skills/clustering/SKILL.md
  - .github/workflows/deploy-services.yml
  - scripts/lib/m034_public_surface_contract.py
  - scripts/verify-m034-s05.sh
  - scripts/verify-m034-s05-workflows.sh
  - scripts/verify-m047-s06.sh
  - scripts/verify-m051-s04.sh
  - scripts/verify-m053-s03.sh
  - scripts/verify-m055-s03.sh
  - scripts/tests/verify-m034-s05-contract.test.mjs
  - scripts/tests/verify-m048-s04-skill-contract.test.mjs
  - scripts/tests/verify-m049-s04-onboarding-contract.test.mjs
  - scripts/tests/verify-m050-s02-first-contact-contract.test.mjs
  - scripts/tests/verify-m050-s03-secondary-surfaces.test.mjs
  - scripts/tests/verify-m053-s03-contract.test.mjs
  - scripts/tests/verify-m053-s04-contract.test.mjs
  - scripts/tests/verify-m055-s03-contract.test.mjs
  - compiler/meshc/tests/e2e_m047_s06.rs
  - compiler/meshc/tests/e2e_m051_s04.rs
  - .gsd/PROJECT.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - D442: add a root-level `productHandoff` object to `scripts/lib/repo-identity.json` and derive public product-handoff markers from repo identity instead of hardcoding local `mesher/...` or `verify-m051-*` paths.
  - D443: make public-secondary proof pages hand off into the Hyperpush repo and its Mesher runbook while keeping local `verify-m051*` rails as retained mesh-lang compatibility wrappers only.
  - D444: treat `deploy-services.yml` as the mesh-lang-owned registry + packages/public-surface workflow on main, forbid landing jobs/checks in source and hosted-evidence verifiers, and keep the shared public-surface helper as the only health-check implementation.
patterns_established:
  - Public scaffold/docs/proof surfaces should derive the product handoff from repo identity instead of hardcoding local product-source paths.
  - Historical docs wrappers must move with their paired Rust docs-contract tests or they will drift in opposite directions and make assembled rails false-red.
  - Hosted and source workflow verifiers must fail on forbidden landing jobs/checks, not just on missing required language-owned jobs.
observability_surfaces:
  - .tmp/m055-s03/verify/status.txt
  - .tmp/m055-s03/verify/current-phase.txt
  - .tmp/m055-s03/verify/phase-report.txt
  - .tmp/m055-s03/verify/latest-proof-bundle.txt
  - .tmp/m051-s04/verify/phase-report.txt
  - .tmp/m047-s06/verify/phase-report.txt
  - python3 scripts/lib/m034_public_surface_contract.py local-docs --root .
drill_down_paths:
  - .gsd/milestones/M055/slices/S03/tasks/T01-SUMMARY.md
  - .gsd/milestones/M055/slices/S03/tasks/T02-SUMMARY.md
  - .gsd/milestones/M055/slices/S03/tasks/T03-SUMMARY.md
  - .gsd/milestones/M055/slices/S03/tasks/T04-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-07T03:09:33.226Z
blocker_discovered: false
---

# S03: `mesh-lang` Public Surface & Starter Contract Consolidation

**Public `mesh-lang` scaffold/docs/proof surfaces now hand off across the repo boundary to Hyperpush, the language repo’s deploy/public workflow graph is landing-free, and the full contract closes through `bash scripts/verify-m055-s03.sh`.**

## What Happened

T01 extended `scripts/lib/repo-identity.json` with a canonical `productHandoff` contract and rewired the clustered scaffold README, root README, Getting Started, Clustered Example, Tooling, and the first-contact mutation rails around that single source of truth. Public starter guidance now stays scaffold/examples-first, preserves the SQLite-local vs PostgreSQL-deployable split, and no longer teaches local `mesher/...` paths or `verify-m051-*` commands as evaluator-facing next steps.

T02 moved the public-secondary proof pages onto the same boundary. `website/docs/docs/distributed/index.md`, `distributed-proof/index.md`, and `production-backend-proof/index.md` now separate three jobs cleanly: public clustered/starter teaching, the retained proof map, and the deeper maintained-app handoff into the Hyperpush repo. The proof-surface verifiers were repinned to fail closed on stale mesh-lang-local product paths while preserving the SQLite-local vs PostgreSQL-deployable story.

T03 finished the alignment on secondary surfaces. The generic guide callouts and the clustering skill now use Production Backend Proof as the public bridge into the Hyperpush repo and its Mesher runbook. Closeout also exposed and fixed a real retained-rail drift seam: the older shell wrappers and their paired Rust docs-contract tests had to move together (`scripts/verify-m047-s06.sh` with `compiler/meshc/tests/e2e_m047_s06.rs`, plus the M051 retained docs rail), or the assembled wrappers stayed false-red even when the public docs were already correct.

T04 removed Hyperpush landing from the language-owned deploy graph and finished the slice-owned proof surface. `.github/workflows/deploy-services.yml` is now the mesh-lang-owned registry + packages/public-surface workflow only, `scripts/lib/m034_public_surface_contract.py` plus the M034/M053 source contracts now forbid landing jobs/checks explicitly, and the slice added `scripts/tests/verify-m055-s03-contract.test.mjs` plus `scripts/verify-m055-s03.sh` as the new fast preflight and assembled replay.

Final closeout reran the full unique slice verification set and then the retained wrapper chain. The retained M047/M051 docs wrappers now pass again, and the assembled S03 verifier is green with `.tmp/m055-s03/verify/status.txt=ok`, `current-phase.txt=complete`, a fully passed `phase-report.txt`, and a retained bundle pointer at `.tmp/m055-s03/verify/retained-proof-bundle`. The slice now leaves `mesh-lang` standing on its own for public scaffold/docs/proof/deploy surfaces, while S04 can focus on the actual `hyperpush-mono` extraction and cross-repo evidence assembly instead of more boundary cleanup inside this repo.

## Verification

Ran the full unique slice verification set defined across the S03 tasks and closed on the assembled wrapper.

Passed direct source/build rails:
- `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check`
- `cargo test -p meshc --test e2e_m049_s03 -- --nocapture`
- `node --test scripts/tests/verify-m049-s04-onboarding-contract.test.mjs`
- `node --test scripts/tests/verify-m050-s02-first-contact-contract.test.mjs`
- `node --test scripts/tests/verify-m050-s03-secondary-surfaces.test.mjs`
- `node --test scripts/tests/verify-m053-s04-contract.test.mjs`
- `bash scripts/verify-production-proof-surface.sh`
- `npm --prefix website run build`
- `node --test scripts/tests/verify-m048-s04-skill-contract.test.mjs`
- `node --test scripts/tests/verify-m034-s05-contract.test.mjs`
- `bash scripts/verify-m034-s05-workflows.sh`
- `node --test scripts/tests/verify-m053-s03-contract.test.mjs`
- `node --test scripts/tests/verify-m055-s03-contract.test.mjs`
- `python3 scripts/lib/m034_public_surface_contract.py local-docs --root .`
- `npm --prefix packages-website run build`

Passed retained wrapper / docs rails:
- `bash scripts/verify-m047-s06.sh`
- `bash scripts/verify-m051-s04.sh`

Passed assembled slice closeout rail:
- `bash scripts/verify-m055-s03.sh`

Assembled verifier markers after closeout:
- `.tmp/m055-s03/verify/status.txt` -> `ok`
- `.tmp/m055-s03/verify/current-phase.txt` -> `complete`
- `.tmp/m055-s03/verify/phase-report.txt` shows all required phases passed
- `.tmp/m055-s03/verify/latest-proof-bundle.txt` points at `.tmp/m055-s03/verify/retained-proof-bundle`

## Requirements Advanced

- R008 — Public starter/docs/proof surfaces now stay production-oriented without teaching local proof-app paths as the evaluator-facing follow-on step.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

The plan named shell wrappers in T03/T04, but the truthful repair required updating the paired Rust docs-contract tests and one retained M047 public-surface test in the same unit. Leaving the Rust-owned guards stale would have kept the wrapper stack false-red even after the docs and shell verifiers were corrected.

## Known Limitations

The repo-boundary product handoff is now truthful inside `mesh-lang`, but the actual product extraction into `hyperpush-mono` is still S04 work. The green S03 wrapper proves the mesh-lang-owned source/build/proof contract locally; hosted freshness across separate repos and final cross-repo evidence assembly still belong to the next slice.

## Follow-ups

S04 should perform the actual `hyperpush-mono` extraction, move product-owned proof ownership there, and assemble one cross-repo evidence chain that links the green `mesh-lang` S03 bundle to the product-side bundle without reintroducing local-product-path teaching inside `mesh-lang`.

## Files Created/Modified

- `scripts/lib/repo-identity.json` — Added canonical `productHandoff` metadata consumed by scaffold/docs/tests.
- `compiler/mesh-pkg/src/scaffold.rs` — Rewired clustered scaffold public links to repo-identity-driven handoff metadata.
- `README.md` — Kept the public ladder scaffold/examples-first and stopped the deeper handoff at Production Backend Proof.
- `website/docs/docs/getting-started/index.md` — Kept first-contact guidance on the split starter path and repo-boundary product handoff.
- `website/docs/docs/getting-started/clustered-example/index.md` — Preserved the SQLite-local vs PostgreSQL-deployable split and the repo-boundary maintained-app handoff.
- `website/docs/docs/tooling/index.md` — Moved tooling guidance to the Production Backend Proof -> Hyperpush handoff instead of local product-source paths.
- `website/docs/docs/distributed/index.md` — Separated public distributed primitives from the deeper maintained-app handoff.
- `website/docs/docs/distributed-proof/index.md` — Repinned the proof map to the M053 chain and the Hyperpush repo boundary.
- `website/docs/docs/production-backend-proof/index.md` — Turned Production Backend Proof into the compact public-secondary bridge into the Hyperpush repo.
- `website/docs/docs/web/index.md` — Updated the generic web guide callout to the repo-boundary maintained-app path.
- `website/docs/docs/databases/index.md` — Updated the generic databases guide callout and next-step wording to the repo-boundary path.
- `website/docs/docs/testing/index.md` — Updated the generic testing guide callout to the repo-boundary maintained-app path.
- `website/docs/docs/concurrency/index.md` — Updated the generic concurrency guide callout to the repo-boundary maintained-app path.
- `tools/skill/mesh/skills/clustering/SKILL.md` — Aligned the clustering skill with the public scaffold/examples-first story and Production Backend Proof -> Hyperpush handoff.
- `.github/workflows/deploy-services.yml` — Removed Hyperpush landing deployment/checks so the workflow now owns only registry + packages/public-surface deployment.
- `scripts/lib/m034_public_surface_contract.py` — Updated the shared deploy/public-surface contract metadata to the language-only graph and forbidden landing markers.
- `scripts/verify-m034-s05.sh` — Repinned hosted/public proof to the language-only deploy graph.
- `scripts/verify-m034-s05-workflows.sh` — Repinned the workflow-source verifier to the landing-free language-owned graph.
- `scripts/verify-m047-s06.sh` — Updated the retained docs wrapper to match the repo-boundary public contract.
- `scripts/verify-m051-s04.sh` — Updated the retained docs wrapper to match the repo-boundary public contract.
- `scripts/verify-m053-s03.sh` — Repinned starter/packages/public-surface hosted evidence to the language-only deploy graph.
- `scripts/verify-m055-s03.sh` — Added the slice-owned assembled replay and retained-bundle contract.
- `scripts/tests/verify-m034-s05-contract.test.mjs` — Updated the shared workflow/helper contract tests to the landing-free language-owned graph.
- `scripts/tests/verify-m048-s04-skill-contract.test.mjs` — Repinned the clustering-skill mutation rail to the repo-boundary maintained-app handoff.
- `scripts/tests/verify-m049-s04-onboarding-contract.test.mjs` — Repinned onboarding mutation checks to the repo-boundary product handoff.
- `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` — Repinned the first-contact contract to the scaffold/examples-first repo-boundary path.
- `scripts/tests/verify-m050-s03-secondary-surfaces.test.mjs` — Repinned the proof-page contract to the repo-boundary maintained-app story.
- `scripts/tests/verify-m053-s03-contract.test.mjs` — Updated hosted starter/packages/public-surface contract tests to forbid landing drift.
- `scripts/tests/verify-m053-s04-contract.test.mjs` — Updated distributed proof/Fly reference contract tests to the new public boundary.
- `scripts/tests/verify-m055-s03-contract.test.mjs` — Added the fast slice-owned source contract for the assembled S03 wrapper and language-only workflow graph.
- `compiler/meshc/tests/e2e_m047_s06.rs` — Updated the retained M047 docs contract to the repo-boundary public handoff.
- `compiler/meshc/tests/e2e_m051_s04.rs` — Updated the retained M051 docs contract to the repo-boundary public handoff.
- `.gsd/PROJECT.md` — Refreshed current-state project context to mark M055/S03 complete and point at the remaining S04 extraction work.
- `.gsd/KNOWLEDGE.md` — Recorded the forbidden-landing-job verifier rule alongside the existing S03 contract gotchas.
