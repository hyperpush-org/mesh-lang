# S02: Route-backed dashboard parity

**Goal:** Replace the temporary single-route dashboard adapter in `../hyperpush-mono/mesher/frontend-exp/` with real TanStack file routes for the current top-level sections while preserving the existing dashboard shell, mock-data/client-state behavior, and the external `dev` / `build` / `start` contract.
**Demo:** After this: the dashboard uses real TanStack routes for the current sections while preserving the same visible shell, URLs, panels, filters, and mock-data interactions.

## Must-Haves

- The active dashboard shell moves off the temporary single-route adapter and onto router-owned modules under `../hyperpush-mono/mesher/frontend-exp/src/routes/` and `../hyperpush-mono/mesher/frontend-exp/components/dashboard/` without visible shell drift.
- `/`, `/performance`, `/solana-programs`, `/releases`, `/alerts`, `/bounties`, `/treasury`, and `/settings` are real TanStack file routes and direct-entry deep links in both dev and built production.
- Sidebar active state, collapse persistence, AI panel close-on-nav, settings-header behavior, Issues filter/search/detail behavior, and existing mock-data interactions stay at parity with the current shell.
- The slice stays entirely on the existing mock-data/client-state path: no TanStack loaders, no server functions, no Mesher backend calls, and no URL-search-param widening.

## Threat Surface

- **Abuse**: malformed deep links, rapid route changes, and split-brain nav state must not surface blank shells, stale right panels, or URL/view divergence.
- **Data exposure**: none beyond the existing mock dashboard data; no new backend or secret-bearing surfaces are introduced.
- **Input trust**: pathname segments plus client-side search/filter inputs are untrusted and must remain client-only.

## Requirement Impact

- **Requirements touched**: R143, R145, R146, R147
- **Re-verify**: direct-entry deep links in dev/build/start, sidebar/header/right-panel behavior, Issues leave-and-return behavior, zero console errors, and zero failed requests.
- **Decisions revisited**: D494, D495, D496, D497

## Proof Level

- This slice proves: integration-level TanStack route ownership for the current dashboard sections while preserving exact shell/mock-data parity.
- Real runtime required: yes
- Human/UAT required: no

## Verification

- `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build`
- `npm --prefix ../hyperpush-mono/mesher/frontend-exp exec playwright test tests/e2e/dashboard-route-parity.spec.ts --project=dev`
- `npm --prefix ../hyperpush-mono/mesher/frontend-exp exec playwright test tests/e2e/dashboard-route-parity.spec.ts --project=prod`

## Observability / Diagnostics

- Runtime signals: TanStack `src/routeTree.gen.ts` output, Playwright parity assertions, browser console noise, failed network requests, active-nav state, AI-panel visibility, and visible Issues filter/search state.
- Inspection surfaces: `../hyperpush-mono/mesher/frontend-exp/playwright.config.ts`, `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts`, dev on port 3000, and built production on port 3001.
- Failure visibility: failing route URL/selector assertions, deep-link boot failures, stale active-nav highlighting, unexpected settings header rendering, console errors, and failed-request URLs.
- Redaction constraints: stay on mock data only; do not add env dumps or secret-bearing diagnostics.

## Integration Closure

- Upstream surfaces consumed: `../hyperpush-mono/mesher/frontend-exp/src/routes/__root.tsx`, `../hyperpush-mono/mesher/frontend-exp/src/router.tsx`, `../hyperpush-mono/mesher/frontend-exp/server.mjs`, and the existing dashboard page components under `../hyperpush-mono/mesher/frontend-exp/components/dashboard/`.
- New wiring introduced in this slice: a pathless `_dashboard` layout route, real top-level section route files, a route-aware shared shell, and repo-owned parity tests for dev and built production.
- What remains before the milestone is truly usable end-to-end: the later M059 path move / cleanup slices, not more route-parity work inside `frontend-exp`.

## Tasks

- [x] **T01: Extract a shared dashboard shell and add Playwright route-parity scaffolding** `est:90m`
  - Why: close the biggest parity risk first by moving mounted chrome ownership out of `app/page.tsx` and adding an objective browser harness before route work starts.
  - Files: `../hyperpush-mono/mesher/frontend-exp/package.json`, `../hyperpush-mono/mesher/frontend-exp/playwright.config.ts`, `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts`, `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-shell.tsx`, `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-route-map.ts`, `../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx`
  - Do: extract a reusable shared shell plus route map/title helper, keep `/` on the current Issues experience, add a dev Playwright `webServer` harness, and make the active runtime render through the new shell module instead of the temporary monolith.
  - Verify: `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build && npm --prefix ../hyperpush-mono/mesher/frontend-exp exec playwright test tests/e2e/dashboard-route-parity.spec.ts --project=dev --grep "issues shell"`
  - Done when: the current `/` shell boots through the extracted module and a repo-owned browser spec truthfully asserts Issues landmarks, shell chrome, and zero console/request noise.

- [x] **T02: Extract the Issues route content and keep Issues state layout-owned** `est:90m`
  - Why: Issues is the last inline branch in the adapter shell, and its filters/detail state must stay above route leaves to preserve leave-and-return behavior.
  - Files: `../hyperpush-mono/mesher/frontend-exp/components/dashboard/issues-page.tsx`, `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-issues-state.tsx`, `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-shell.tsx`, `../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx`, `../hyperpush-mono/mesher/frontend-exp/app/page.tsx`, `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts`
  - Do: extract the Issues column/panel into its own component, move Issues filter/search/detail state into shell-owned client state, and stop depending on inline Issues JSX from `app/page.tsx` on the active runtime path.
  - Verify: `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build && npm --prefix ../hyperpush-mono/mesher/frontend-exp exec playwright test tests/e2e/dashboard-route-parity.spec.ts --project=dev --grep "issues interactions"`
  - Done when: Issues is route-leaf-ready, its state lives in the shared shell, and the browser spec proves filters/detail interactions still match the current mock-data behavior.

- [x] **T03: Replace the single adapter route with a pathless dashboard layout and real section routes** `est:2h`
  - Why: S02 exists to make the current sections real TanStack routes without changing the visible surface or widening into backend/search-param work.
  - Files: `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.tsx`, `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.index.tsx`, `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.performance.tsx`, `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.solana-programs.tsx`, `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.releases.tsx`, `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.alerts.tsx`, `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.bounties.tsx`, `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.treasury.tsx`, `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.settings.tsx`, `../hyperpush-mono/mesher/frontend-exp/src/routeTree.gen.ts`
  - Do: create a pathless `_dashboard` layout route, add flat child route files for each current top-level section, retire `src/routes/index.tsx` from the active runtime path, and let TanStack regenerate the route tree from source files.
  - Verify: `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build && npm --prefix ../hyperpush-mono/mesher/frontend-exp exec playwright test tests/e2e/dashboard-route-parity.spec.ts --project=dev --grep "direct-entry routes"`
  - Done when: every current top-level section mounts from a real file route, `/` still means Issues, `/settings` still renders its self-owned page chrome, and the direct-entry route assertions pass.

- [x] **T04: Make top-level navigation route-aware and preserve shell parity behaviors** `est:90m`
  - Why: the route tree alone is not enough; URL updates, active highlighting, AI close-on-nav, sidebar persistence, and settings-header behavior must stay truthful.
  - Files: `../hyperpush-mono/mesher/frontend-exp/components/dashboard/sidebar.tsx`, `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-shell.tsx`, `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-route-map.ts`, `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts`
  - Do: make the sidebar route-target aware, derive the active section from router state, preserve shell-owned transient UI state outside the router, and assert URL/view parity plus Issues leave-and-return behavior.
  - Verify: `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build && npm --prefix ../hyperpush-mono/mesher/frontend-exp exec playwright test tests/e2e/dashboard-route-parity.spec.ts --project=dev --grep "navigation parity"`
  - Done when: clicking the sidebar updates the URL and visible active section together, AI closes on route changes, sidebar collapse persists, `/settings` hides the shared header, and Issues state survives leave/return.

- [x] **T05: Prove dev and built-production deep-link parity on the existing command contract** `est:75m`
  - Why: the slice is not done until the new route tree survives the actual `build` / `start` contract and non-root direct-entry in production.
  - Files: `../hyperpush-mono/mesher/frontend-exp/playwright.config.ts`, `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts`, `../hyperpush-mono/mesher/frontend-exp/package.json`, `../hyperpush-mono/mesher/frontend-exp/server.mjs`
  - Do: extend the parity suite to dev and prod projects, prove a non-root route first-load under `npm run start`, and fix any truthful deep-link or environment-specific route drift without widening scope beyond the frontend/mock-data contract.
  - Verify: `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build && npm --prefix ../hyperpush-mono/mesher/frontend-exp exec playwright test tests/e2e/dashboard-route-parity.spec.ts --project=dev && npm --prefix ../hyperpush-mono/mesher/frontend-exp exec playwright test tests/e2e/dashboard-route-parity.spec.ts --project=prod`
  - Done when: the same repo-owned parity spec passes in dev and built production, including non-root direct-entry, with zero console errors and zero failed requests.

## Files Likely Touched

- `../hyperpush-mono/mesher/frontend-exp/package.json`
- `../hyperpush-mono/mesher/frontend-exp/playwright.config.ts`
- `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-shell.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-route-map.ts`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-issues-state.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/issues-page.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/sidebar.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.index.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.performance.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.solana-programs.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.releases.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.alerts.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.bounties.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.treasury.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.settings.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routeTree.gen.ts`
- `../hyperpush-mono/mesher/frontend-exp/app/page.tsx`
- `../hyperpush-mono/mesher/frontend-exp/server.mjs`
