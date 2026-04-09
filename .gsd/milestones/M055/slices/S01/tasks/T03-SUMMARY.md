---
id: T03
parent: S01
milestone: M055
provides: []
requires: []
affects: []
key_files: ["packages-website/src/routes/+layout.svelte", "mesher/landing/lib/external-links.ts", "tools/editors/vscode-mesh/package.json", "scripts/tests/verify-m055-s01-contract.test.mjs", ".gsd/DECISIONS.md", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Kept the public repo-identity surfaces as literal URLs/labels and enforced the language-vs-product split with the node:test contract rather than runtime JSON imports.", "Recorded D434: `mesher/frontend-exp/` is product-owned mesher repo material that will replace the existing frontend later."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "`npm --prefix packages-website run build` passed. `node --test scripts/tests/verify-m055-s01-contract.test.mjs` passed with 12/12 checks green, including the new packages-footer, landing-link, and VS Code metadata mutation cases. The requested whole-app landing typecheck `./mesher/landing/node_modules/.bin/tsc --noEmit -p mesher/landing/tsconfig.json` still failed at a documented unrelated baseline in `mesher/landing/components/blog/editor.tsx`, `components/landing/infrastructure.tsx`, and `components/landing/mesh-dataflow.tsx`, not in `lib/external-links.ts`, so I added a compensating consumer-facing proof with `npm --prefix mesher/landing run build`, which passed. I also verified the rendered surfaces in a browser against local servers: the packages preview showed `mesh-lang repo` and `Workspace` footer links with the expected language-repo hrefs, and the landing privacy page rendered `GitHub: github.com/hyperpush-org/hyperpush-mono` with a GitHub link pointing at the product repo."
completed_at: 2026-04-06T18:19:21.080Z
blocker_discovered: false
---

# T03: Made packages, landing, and VS Code identity surfaces explicit about the M055 language-vs-product repo split.

> Made packages, landing, and VS Code identity surfaces explicit about the M055 language-vs-product repo split.

## What Happened
---
id: T03
parent: S01
milestone: M055
key_files:
  - packages-website/src/routes/+layout.svelte
  - mesher/landing/lib/external-links.ts
  - tools/editors/vscode-mesh/package.json
  - scripts/tests/verify-m055-s01-contract.test.mjs
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Kept the public repo-identity surfaces as literal URLs/labels and enforced the language-vs-product split with the node:test contract rather than runtime JSON imports.
  - Recorded D434: `mesher/frontend-exp/` is product-owned mesher repo material that will replace the existing frontend later.
duration: ""
verification_result: mixed
completed_at: 2026-04-06T18:19:21.084Z
blocker_discovered: false
---

# T03: Made packages, landing, and VS Code identity surfaces explicit about the M055 language-vs-product repo split.

**Made packages, landing, and VS Code identity surfaces explicit about the M055 language-vs-product repo split.**

## What Happened

Updated `packages-website/src/routes/+layout.svelte` so the footer now exposes the language-owned boundary directly with `mesh-lang repo` and `Workspace` links into the `mesh-lang` repo instead of a generic GitHub label. Kept `mesher/landing/lib/external-links.ts` explicitly product-owned with literal `hyperpush-org/hyperpush-mono` GitHub markers, and tightened `tools/editors/vscode-mesh/package.json` with `repository.directory` while preserving the language repo and issues URLs as the editor source of truth. Expanded `scripts/tests/verify-m055-s01-contract.test.mjs` so the repo-identity rail now fails closed when the packages footer drifts toward product URLs, when landing links drift back to `mesh-lang`, or when VS Code metadata mixes the two repos. Recorded D434 plus a knowledge entry for the user’s clarification that `mesher/frontend-exp/` stays product-owned and is intended to replace the existing frontend later.

## Verification

`npm --prefix packages-website run build` passed. `node --test scripts/tests/verify-m055-s01-contract.test.mjs` passed with 12/12 checks green, including the new packages-footer, landing-link, and VS Code metadata mutation cases. The requested whole-app landing typecheck `./mesher/landing/node_modules/.bin/tsc --noEmit -p mesher/landing/tsconfig.json` still failed at a documented unrelated baseline in `mesher/landing/components/blog/editor.tsx`, `components/landing/infrastructure.tsx`, and `components/landing/mesh-dataflow.tsx`, not in `lib/external-links.ts`, so I added a compensating consumer-facing proof with `npm --prefix mesher/landing run build`, which passed. I also verified the rendered surfaces in a browser against local servers: the packages preview showed `mesh-lang repo` and `Workspace` footer links with the expected language-repo hrefs, and the landing privacy page rendered `GitHub: github.com/hyperpush-org/hyperpush-mono` with a GitHub link pointing at the product repo.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix packages-website run build` | 0 | ✅ pass | 43810ms |
| 2 | `./mesher/landing/node_modules/.bin/tsc --noEmit -p mesher/landing/tsconfig.json` | 2 | ❌ fail | 12990ms |
| 3 | `node --test scripts/tests/verify-m055-s01-contract.test.mjs` | 0 | ✅ pass | 3730ms |
| 4 | `npm --prefix mesher/landing run build` | 0 | ✅ pass | 28700ms |


## Deviations

Added `npm --prefix mesher/landing run build` plus local browser spot-checks because the plan’s whole-app landing `tsc` command is a known baseline-red gate outside this task’s touched file.

## Known Issues

`./mesher/landing/node_modules/.bin/tsc --noEmit -p mesher/landing/tsconfig.json` remains red in unrelated existing files (`components/blog/editor.tsx`, `components/landing/infrastructure.tsx`, and `components/landing/mesh-dataflow.tsx`). The local `next start` browser proof also logged a missing `/_vercel/insights/script.js` request, but the page itself rendered and the repo-identity surfaces were correct.

## Files Created/Modified

- `packages-website/src/routes/+layout.svelte`
- `mesher/landing/lib/external-links.ts`
- `tools/editors/vscode-mesh/package.json`
- `scripts/tests/verify-m055-s01-contract.test.mjs`
- `.gsd/DECISIONS.md`
- `.gsd/KNOWLEDGE.md`


## Deviations
Added `npm --prefix mesher/landing run build` plus local browser spot-checks because the plan’s whole-app landing `tsc` command is a known baseline-red gate outside this task’s touched file.

## Known Issues
`./mesher/landing/node_modules/.bin/tsc --noEmit -p mesher/landing/tsconfig.json` remains red in unrelated existing files (`components/blog/editor.tsx`, `components/landing/infrastructure.tsx`, and `components/landing/mesh-dataflow.tsx`). The local `next start` browser proof also logged a missing `/_vercel/insights/script.js` request, but the page itself rendered and the repo-identity surfaces were correct.
