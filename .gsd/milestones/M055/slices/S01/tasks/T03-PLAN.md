---
estimated_steps: 4
estimated_files: 4
skills_used:
  - best-practices
  - test
---

# T03: Apply the split-aware identity contract to packages, landing, and VS Code metadata

**Slice:** S01 — Two-Repo Boundary & GSD Authority Contract
**Milestone:** M055

## Description

Use the new identity contract on the first public boundary surfaces that already drift today: language-owned packages/editor metadata versus product-owned landing links. This task should make the split visible in real user-facing surfaces, not just in contract files.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `packages-website` footer surface | fail the build on broken Svelte/template edits or missing language-repo link markers | use the normal build timeout and stop on the first error | treat a product-repo GitHub link in the packages footer as public-surface drift |
| `mesher/landing` external links | fail closed on TypeScript errors or missing product-repo markers | bounded local typecheck only | treat language-repo fallback links in product CTA surfaces as identity drift |
| VS Code package metadata | stop on missing repository or bugs URLs for the language repo | N/A for JSON edits | treat stale or product-owned URLs as extension-metadata drift |

## Load Profile

- **Shared resources**: `packages-website/node_modules`, `mesher/landing/node_modules`, and the slice-owned Node contract.
- **Per-operation cost**: one Svelte build, one landing typecheck, and a few metadata/file edits.
- **10x breakpoint**: frontend builds and typechecks dominate before the contract test does.

## Negative Tests

- **Malformed inputs**: the packages footer points at `hyperpush-mono`, landing external links point back at `mesh-lang`, or VS Code metadata mixes the two repos.
- **Error paths**: the repo-identity contract is correct but one of the user-facing surfaces still carries a stale hardcoded URL outside the contract test allowlist.
- **Boundary conditions**: language-owned public surfaces keep the language repo identity while product-owned landing surfaces keep the product repo identity.

## Steps

1. Update `packages-website/src/routes/+layout.svelte` so the footer uses the language-owned Mesh repo identity and not the product repo.
2. Update `mesher/landing/lib/external-links.ts` so the landing site keeps the product-owned `hyperpush-mono` identity explicit and does not drift back to the language repo.
3. Keep `tools/editors/vscode-mesh/package.json` on the language-repo metadata contract and extend `scripts/tests/verify-m055-s01-contract.test.mjs` so these three user-facing surfaces fail closed if they mix repo identities.
4. Rebuild the packages site and typecheck the landing app so the split-aware public identity is proven in real consumer surfaces instead of only in docs/tests.

## Must-Haves

- [ ] `packages-website/src/routes/+layout.svelte` points GitHub/docs-maintainer links at the language repo where appropriate.
- [ ] `mesher/landing/lib/external-links.ts` keeps the product landing CTA on the product repo identity.
- [ ] `tools/editors/vscode-mesh/package.json` still presents the language repo and issue tracker as the editor host source of truth.
- [ ] `scripts/tests/verify-m055-s01-contract.test.mjs` fails on any of those surfaces using the wrong repo identity.

## Verification

- `npm --prefix packages-website run build`
- `./mesher/landing/node_modules/.bin/tsc --noEmit -p mesher/landing/tsconfig.json`
- `node --test scripts/tests/verify-m055-s01-contract.test.mjs`

## Inputs

- `scripts/lib/repo-identity.json` — canonical repo-identity contract from T02
- `packages-website/src/routes/+layout.svelte` — current language-owned packages footer with stale GitHub identity
- `mesher/landing/lib/external-links.ts` — current product landing external-link identity surface
- `tools/editors/vscode-mesh/package.json` — current editor metadata contract
- `scripts/tests/verify-m055-s01-contract.test.mjs` — slice-owned contract rail to extend for public-surface identity

## Expected Output

- `packages-website/src/routes/+layout.svelte` — packages footer aligned to the language repo identity
- `mesher/landing/lib/external-links.ts` — landing external links aligned to the product repo identity
- `tools/editors/vscode-mesh/package.json` — editor metadata pinned to the language repo identity
- `scripts/tests/verify-m055-s01-contract.test.mjs` — public-surface identity assertions for packages, landing, and VS Code metadata
