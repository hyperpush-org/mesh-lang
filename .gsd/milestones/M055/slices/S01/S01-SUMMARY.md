---
id: S01
parent: M055
milestone: M055
provides:
  - One blessed two-repo sibling-workspace contract for M055, one canonical repo-identity source, split-aware public/editor boundary surfaces, and one assembled split-boundary verifier with a repo-local `.gsd` debug path.
requires:
  []
affects:
  - S02
  - S03
  - S04
key_files:
  - WORKSPACE.md
  - README.md
  - CONTRIBUTING.md
  - .gsd/PROJECT.md
  - scripts/lib/repo-identity.json
  - scripts/lib/m034_public_surface_contract.py
  - tools/install/install.sh
  - tools/install/install.ps1
  - website/docs/public/install.sh
  - website/docs/public/install.ps1
  - packages-website/src/routes/+layout.svelte
  - mesher/landing/lib/external-links.ts
  - tools/editors/vscode-mesh/package.json
  - scripts/tests/verify-m055-s01-contract.test.mjs
  - scripts/verify-m055-s01.sh
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Turn D428 and D429 into repo-root contract text before any extraction work begins.
  - Keep canonical repo identity in `scripts/lib/repo-identity.json`, but leave public installers as plain source/copy artifacts validated against it instead of parsing repo-local JSON at runtime.
  - Keep packages, landing, and VS Code repo identity explicit through literal URLs/labels plus fail-closed node:test mutation cases rather than runtime identity imports.
  - Make `bash scripts/verify-m055-s01.sh` the authoritative S01 stop/go rail and keep it narrow: replay only the split-contract node rail, the repo-identity/local-docs helper, the two consumer builds, and the named M046 repo-local `.gsd` seam.
patterns_established:
  - Publish repo-boundary rules in repo-root maintainer docs and guard them with exact-marker mutation tests instead of prose-only guidance.
  - Use one machine-readable repo-identity contract for slugs/URLs/roots, then validate shipped shell/PowerShell/public copies against it rather than adding runtime JSON dependencies to public installers.
  - When a slice only needs a boundary contract, keep the assembled verifier narrow, phase-marked, and anchored to one named downstream regression seam rather than delegating to broader historical wrappers.
observability_surfaces:
  - `.tmp/m055-s01/verify/status.txt`
  - `.tmp/m055-s01/verify/current-phase.txt`
  - `.tmp/m055-s01/verify/phase-report.txt`
  - `.tmp/m055-s01/verify/full-contract.log`
drill_down_paths:
  - .gsd/milestones/M055/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M055/slices/S01/tasks/T02-SUMMARY.md
  - .gsd/milestones/M055/slices/S01/tasks/T03-SUMMARY.md
  - .gsd/milestones/M055/slices/S01/tasks/T04-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-06T18:38:43.151Z
blocker_discovered: false
---

# S01: Two-Repo Boundary & GSD Authority Contract

**Published the two-repo workspace contract, centralized repo identity, made public/editor surfaces split-aware, and shipped one fail-closed verifier for the repo boundary plus repo-local `.gsd` authority.**

## What Happened

S01 turned the planned M055 split into an explicit repo contract before any extraction work starts. `WORKSPACE.md`, `README.md`, `CONTRIBUTING.md`, and `.gsd/PROJECT.md` now all say the same thing: M055 is a two-repo split only, `mesh-lang` keeps language-owned docs/installers/registry/packages/public-site/starter surfaces, `hyperpush-mono` absorbs `mesher/`, and repo-local `.gsd` remains authoritative while cross-repo work goes through a lightweight sibling-workspace coordination layer.

The slice also established one canonical machine-readable repo identity source in `scripts/lib/repo-identity.json`. Instead of making public installers or consumer surfaces parse repo-local JSON at runtime, the slice kept the editable/public installer pair as plain source/copy artifacts and rewired `scripts/lib/m034_public_surface_contract.py` plus the slice-owned node:test rail to validate those files, docs URLs, and VS Code metadata against the canonical identity contract.

On the first user-facing boundary surfaces, the packages footer now points at the language repo/workspace contract, landing external links now point at the product repo, and the VS Code extension metadata stays language-owned. The node:test contract was expanded to fail closed on four-repo drift, umbrella-`.gsd` wording, malformed repo-identity JSON, installer parity drift, packages-footer/product-link drift, landing-link/language-link drift, and VS Code metadata mixing the two repos.

Finally, S01 closed with one obvious stop/go rail: `bash scripts/verify-m055-s01.sh`. That wrapper writes `.tmp/m055-s01/verify/status.txt`, `current-phase.txt`, `phase-report.txt`, and `full-contract.log`, then replays the node contract, the repo-identity/local-docs helper, the packages build, the landing build, and the named M046 repo-local `.gsd` regression (`m046_s03_tiny_cluster_package_contract_remains_source_first_and_route_free`). The wrapper intentionally stays narrow and does not delegate to broader historical rails.

## Verification

Direct checks passed: `diff -u tools/install/install.sh website/docs/public/install.sh`, `diff -u tools/install/install.ps1 website/docs/public/install.ps1`, `node --test scripts/tests/verify-m055-s01-contract.test.mjs`, and `python3 scripts/lib/m034_public_surface_contract.py local-docs --root .` all succeeded. `npm --prefix packages-website run build` succeeded. The planned whole-app landing typecheck `./mesher/landing/node_modules/.bin/tsc --noEmit -p mesher/landing/tsconfig.json` was rerun and is still baseline-red in unrelated existing landing files (`components/blog/editor.tsx`, `components/landing/infrastructure.tsx`, and `components/landing/mesh-dataflow.tsx`), so the slice used the documented compensating consumer-facing seam `npm --prefix mesher/landing run build`, which succeeded. The assembled rail `bash scripts/verify-m055-s01.sh` then passed end to end and left `.tmp/m055-s01/verify/status.txt=ok`, `.tmp/m055-s01/verify/current-phase.txt=complete`, and passed markers for `init`, `m055-s01-contract`, `m055-s01-local-docs`, `m055-s01-packages-build`, `m055-s01-landing-build`, and `m055-s01-gsd-regression` in `.tmp/m055-s01/verify/phase-report.txt`.

## Requirements Advanced

- R120 — The slice tightened repo ownership and public/editor identity surfaces so packages, docs/installers, and the product landing no longer point at mixed or implicit repos, which moves the public Mesh story toward one coherent language-vs-product boundary.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

The only material deviation from the written plan was the landing verification seam. The plan named the whole-app `./mesher/landing/node_modules/.bin/tsc --noEmit -p mesher/landing/tsconfig.json` gate, but that command is still baseline-red in unrelated landing files outside S01’s touched boundary. The slice therefore used `npm --prefix mesher/landing run build` as the truthful consumer-facing rail and baked that same choice into the assembled verifier.

## Known Limitations

S01 establishes the ownership, identity, and verifier contract only; it does not extract `mesher/` into `hyperpush-mono` yet or prove the deeper cross-repo toolchain handoff. The whole-app landing `tsc --noEmit` gate remains red at baseline in unrelated files, so later landing-heavy work still cannot treat it as the authoritative slice-level rail until that debt is fixed.

## Follow-ups

S02 should consume `WORKSPACE.md` and `scripts/lib/repo-identity.json` instead of inventing new sibling-path or repo-slug assumptions for the deeper product toolchain contract. S03 and S04 should keep split-aware public surfaces and cross-repo evidence aligned with the narrow `verify-m055-s01.sh` boundary rather than widening this slice’s wrapper into unrelated historical proof stories. A future landing-focused slice can restore the stricter whole-app TypeScript gate once the unrelated baseline debt is retired.

## Files Created/Modified

- `WORKSPACE.md` — Added the maintainer-facing M055 workspace contract with the blessed two-repo layout, ownership matrix, repo-local `.gsd` rule, and split-boundary verifier entrypoint.
- `README.md` — Added maintainer-facing workspace-contract routing from the repo root so the split rules are discoverable outside milestone artifacts.
- `CONTRIBUTING.md` — Documented the M055 workspace contract and named `bash scripts/verify-m055-s01.sh` as the split-boundary verifier plus debug path.
- `.gsd/PROJECT.md` — Updated current-state project text to treat the monorepo layout as transitional and to reflect that S01 shipped the workspace, identity, and verifier contract.
- `scripts/lib/repo-identity.json` — Introduced the canonical language-repo versus product-repo identity contract with slugs, URLs, blob bases, and public roots.
- `scripts/lib/m034_public_surface_contract.py` — Rewired the local-docs/public-surface helper to load repo identity from the canonical JSON contract instead of a second embedded slug table.
- `tools/install/install.sh` — Kept the editable shell installer on the canonical language-repo slug and preserved byte parity with the public-served copy.
- `tools/install/install.ps1` — Kept the editable PowerShell installer on the canonical language-repo slug and preserved byte parity with the public-served copy.
- `website/docs/public/install.sh` — Retained the docs-served public shell installer as a byte-for-byte mirror of the editable source installer.
- `website/docs/public/install.ps1` — Retained the docs-served public PowerShell installer as a byte-for-byte mirror of the editable source installer.
- `packages-website/src/routes/+layout.svelte` — Made the packages footer explicitly language-owned with `mesh-lang repo` and `Workspace` links.
- `mesher/landing/lib/external-links.ts` — Kept landing external links explicitly product-owned and pointed at `hyperpush-org/hyperpush-mono`.
- `tools/editors/vscode-mesh/package.json` — Preserved the VS Code extension’s language-repo repository and issues metadata and made the directory-level ownership explicit.
- `scripts/tests/verify-m055-s01-contract.test.mjs` — Expanded the slice-owned contract test to guard workspace text, repo identity, installer parity, packages/footer drift, landing-link drift, VS Code metadata drift, and wrapper/docs discoverability.
- `scripts/verify-m055-s01.sh` — Added the assembled split-boundary verifier that writes standard phase markers and replays the node contract, local-docs helper, consumer builds, and named M046 repo-local `.gsd` regression seam.
- `.gsd/KNOWLEDGE.md` — Recorded the M055 split-boundary debug path and the narrow verifier-maintenance rule for future slices.
