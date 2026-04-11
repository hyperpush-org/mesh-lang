# S03: Finalize move to `mesher/client` and remove Next.js runtime path

**Goal:** Make `../hyperpush-mono/mesher/client/` the canonical TanStack dashboard package, update the machine-checked path contract around it, and prove the moved app still preserves the S02 route/UI parity on the existing mock-data runtime.
**Demo:** After this: the migrated dashboard runs from `../hyperpush-mono/mesher/client/` with `dev`, `build`, and `start`, and Next.js is no longer on the critical runtime path.

## Must-Haves

- `../hyperpush-mono/mesher/client/` is the canonical dashboard package root and preserves the existing package-local `dev`, `build`, `start`, `test:e2e:dev`, and `test:e2e:prod` contract.
- The machine-checked external path contract in `../hyperpush-mono/.github/workflows/ci.yml`, `../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh`, `../hyperpush-mono/README.md`, `../hyperpush-mono/.github/dependabot.yml`, and `./playwright.config.ts` points at `mesher/client` instead of `mesher/frontend-exp`.
- The moved app still proves direct-entry route parity for `/`, `/performance`, `/solana-programs`, `/releases`, `/alerts`, `/bounties`, `/treasury`, `/settings`, and the unknown-path Issues fallback in both dev and built production.
- The slice stays on the existing mock-data/client-state contract: no TanStack loaders, no server functions, no Mesher backend calls, and no widened URL/search-param semantics are introduced during the move.

## Threat Surface

- **Abuse**: direct-entry pathnames and asset requests must keep resolving through the moved `../hyperpush-mono/mesher/client/` package instead of 404ing or exposing sibling filesystem paths.
- **Data exposure**: none beyond the existing mock dashboard fixtures; this slice must not add server-backed data, secrets, or auth surfaces.
- **Input trust**: pathname segments plus client-side search/filter inputs remain untrusted and must stay client-only with no new loaders, server functions, or backend calls introduced by the move.

## Requirement Impact

- **Requirements touched**: R143, R144, R145, R146, R147.
- **Re-verify**: package-local `dev` / `build` / `start`, direct-entry route parity in dev and built production, sidebar/AI/settings/issues leave-and-return behavior, and the machine-checked path surfaces that currently hardcode `frontend-exp`.
- **Decisions revisited**: D496 (`start` bridge stays truthful after the move), D500 (unknown-path Issues fallback still works after build), D501 (pathname-derived shell behavior stays intact), D502 (repo-owned isolated Playwright rails remain the truthful verification contract), D503 (treat S03 as a path-and-contract migration rather than reopening route/state architecture).

## Proof Level

- This slice proves: final-assembly proof of the renamed package and the surrounding cross-repo runtime/verification contract using the real `mesher/client` entrypoints in dev and built production.
- Real runtime required: yes.
- Human/UAT required: no.

## Verification

- `npm --prefix ../hyperpush-mono/mesher/client run build`
- `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev`
- `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod`
- `! rg -n "mesher/frontend-exp|frontend-exp" ../hyperpush-mono/.github/workflows/ci.yml ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh ../hyperpush-mono/README.md ../hyperpush-mono/.github/dependabot.yml ./playwright.config.ts`
- `rg -n "mesher/client|client" ../hyperpush-mono/.github/workflows/ci.yml ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh ../hyperpush-mono/README.md ../hyperpush-mono/.github/dependabot.yml ./playwright.config.ts`

## Observability / Diagnostics

- Runtime signals: package-local Playwright console/request failures, product CI cache/build failures, maintainer-verifier marker mismatches, and `node server.mjs` boot errors all become `mesher/client`-scoped evidence instead of silently validating `frontend-exp`.
- Inspection surfaces: `../hyperpush-mono/mesher/client/playwright.config.ts`, `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts`, `../hyperpush-mono/mesher/client/server.mjs`, `../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh`, and `playwright.config.ts`.
- Failure visibility: stale old-path matches, wrong working-directory/cache paths, broken prod boot under the moved package, or dev/prod parity regressions after the rename.
- Redaction constraints: stay on mock data only; do not add env dumps, secrets, or backend diagnostics to prove the move.

## Integration Closure

- Upstream surfaces consumed: the S02 TanStack route tree and parity harness inside the moved package, the product-repo CI/verifier surfaces, and `mesh-lang` root Playwright verification.
- New wiring introduced in this slice: the canonical app path becomes `../hyperpush-mono/mesher/client/`, and all machine-checked callers that launch or verify the dashboard are rewired to that path.
- What remains before the milestone is truly usable end-to-end: only broader non-critical stale-reference cleanup that S04 may absorb; the runtime/path contract itself should be complete here.

## Tasks

- [x] **T01: Move the TanStack dashboard package to `mesher/client` and make the package-local contract truthful** `est:90m`
  - Why: close the highest-risk gap first by moving the already-migrated TanStack package wholesale instead of reconstructing it file-by-file.
  - Files: `../hyperpush-mono/mesher/frontend-exp/package.json`, `../hyperpush-mono/mesher/frontend-exp/server.mjs`, `../hyperpush-mono/mesher/frontend-exp/vite.config.ts`, `../hyperpush-mono/mesher/frontend-exp/playwright.config.ts`, `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts`, `../hyperpush-mono/mesher/frontend-exp/README.md`
  - Do: move the real runtime/test tree from `../hyperpush-mono/mesher/frontend-exp/` to `../hyperpush-mono/mesher/client/`, keep the package-local `dev` / `build` / `start` / parity scripts structurally unchanged unless the rename exposes a real path break, treat `dist/`, `node_modules/`, `.tanstack/`, and `test-results/` as disposable artifacts rather than canonical content, and rewrite the package README to describe the TanStack/Vite runtime instead of the stale Next.js/v0 boilerplate.
  - Verify: `test -f ../hyperpush-mono/mesher/client/package.json && test -f ../hyperpush-mono/mesher/client/server.mjs && test -f ../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts && test ! -e ../hyperpush-mono/mesher/frontend-exp/package.json && rg -n "vite dev|vite build|node server\.mjs|test:e2e:dev|test:e2e:prod" ../hyperpush-mono/mesher/client/README.md && ! rg -n "Next\.js|v0|frontend-exp" ../hyperpush-mono/mesher/client/README.md`
  - Done when: the canonical app lives under `../hyperpush-mono/mesher/client/`, the moved package still exposes the same package-local command contract, and the package README truthfully documents the new path without Next.js/v0 claims.

- [ ] **T02: Update the machine-checked `mesher/client` contract and prove dev/prod parity from the new path** `est:105m`
  - Why: the rename is not done until every machine-checked caller launches or verifies the dashboard from `mesher/client` instead of preserving a stale `frontend-exp` contract.
  - Files: `../hyperpush-mono/.github/workflows/ci.yml`, `../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh`, `../hyperpush-mono/README.md`, `../hyperpush-mono/.github/dependabot.yml`, `playwright.config.ts`, `../hyperpush-mono/mesher/client/package.json`, `../hyperpush-mono/mesher/client/playwright.config.ts`, `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts`
  - Do: update the product CI workflow, product maintainer verifier, product root README, Dependabot npm directory, and `mesh-lang` root Playwright config to point at `../hyperpush-mono/mesher/client/`; keep `mesher/landing` as the only legitimate Next.js app in scope; then rerun the build and isolated dev/prod parity rails from the new package path.
  - Verify: `npm --prefix ../hyperpush-mono/mesher/client run build && npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev && npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod && ! rg -n "mesher/frontend-exp|frontend-exp" ../hyperpush-mono/.github/workflows/ci.yml ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh ../hyperpush-mono/README.md ../hyperpush-mono/.github/dependabot.yml ./playwright.config.ts`
  - Done when: the external path contract is self-consistent, the moved package passes build plus both parity projects from `mesher/client`, and the touched machine-checked surfaces no longer require `frontend-exp`.

## Files Likely Touched

- `../hyperpush-mono/mesher/frontend-exp/package.json`
- `../hyperpush-mono/mesher/frontend-exp/server.mjs`
- `../hyperpush-mono/mesher/frontend-exp/vite.config.ts`
- `../hyperpush-mono/mesher/frontend-exp/playwright.config.ts`
- `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts`
- `../hyperpush-mono/mesher/frontend-exp/README.md`
- `../hyperpush-mono/.github/workflows/ci.yml`
- `../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh`
- `../hyperpush-mono/README.md`
- `../hyperpush-mono/.github/dependabot.yml`
- `playwright.config.ts`
- `../hyperpush-mono/mesher/client/package.json`
- `../hyperpush-mono/mesher/client/playwright.config.ts`
- `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts`
