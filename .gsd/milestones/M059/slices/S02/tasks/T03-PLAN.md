---
estimated_steps: 4
estimated_files: 10
skills_used:
  - react-best-practices
  - vite
  - test
---

# T03: Replace the single adapter route with a pathless dashboard layout and real section routes

**Slice:** S02 — Route-backed dashboard parity
**Milestone:** M059

## Description

Swap the temporary one-route adapter for real TanStack file routes without changing visible product scope. This task creates the pathless `_dashboard` layout route and concrete route files for `/`, `/performance`, `/solana-programs`, `/releases`, `/alerts`, `/bounties`, `/treasury`, and `/settings`, all rendered through the shared dashboard shell and existing leaf page components.

Keep the route layer client-first. Route files should compose the already-extracted shell and page modules only; do not add loaders, server functions, backend fetches, or search-param serialization. The goal here is truthful route ownership and direct-entry deep-link rendering, not URL-encoding every internal filter.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.tsx` pathless layout route | Fail the task if the shared shell remounts per route or the layout stops owning the right-panel/header/sidebar seams. | N/A | Treat an unmapped pathname as the index route instead of rendering a blank shell. |
| `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard*.tsx` child route files | Keep each file thin and route-only; if a leaf cannot mount cleanly, fail on that route instead of falling back to the old local-nav branch. | Bound assertions to route-specific landmarks. | Reject invalid imports/route names rather than hand-editing `routeTree.gen.ts`. |
| `../hyperpush-mono/mesher/frontend-exp/src/routeTree.gen.ts` generated route tree | Regenerate from file routes; never hand-edit the generated file. | N/A | Treat generation drift as a build failure, not as a manual patch target. |

## Load Profile

- **Shared resources**: one mounted dashboard shell, TanStack router state, and the existing mock-data page components.
- **Per-operation cost**: client-side route match plus mounting one existing page leaf inside the shell.
- **10x breakpoint**: route-shell remounts or oversized shared imports will fail parity/build truth before router matching cost matters.

## Negative Tests

- **Malformed inputs**: direct-entry deep links to each expected pathname and unknown path fallbacks handled by the router.
- **Error paths**: route file naming mistakes, stale `index.tsx` adapter ownership, or build failures from generated route-tree drift.
- **Boundary conditions**: `/` stays Issues, `/settings` renders its self-owned header/body, and all other routes mount beneath the shared shell.

## Steps

1. Create the pathless `_dashboard.tsx` layout route that renders the shared shell and an outlet without reintroducing per-route chrome wrappers.
2. Add flat child route files for each current top-level section, keeping Issues on `/` and mapping every existing dashboard leaf component to its current slug.
3. Remove the temporary `src/routes/index.tsx` adapter from the active runtime path and let TanStack regenerate `src/routeTree.gen.ts` from the new file routes.
4. Extend the Playwright spec to direct-load every route URL and assert the unique route landmarks plus zero console/request errors on the mock-data path.

## Must-Haves

- [ ] `_dashboard.tsx` becomes the single shared shell owner for all current top-level dashboard sections.
- [ ] `/`, `/performance`, `/solana-programs`, `/releases`, `/alerts`, `/bounties`, `/treasury`, and `/settings` exist as real TanStack file routes.
- [ ] No route introduces backend integration, loaders, server functions, or URL search-param serialization.
- [ ] The route tree is generated from file routes, not hand-edited.

## Verification

- `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build`
- `npm --prefix ../hyperpush-mono/mesher/frontend-exp exec playwright test tests/e2e/dashboard-route-parity.spec.ts --project=dev --grep "direct-entry routes"`

## Observability Impact

- Signals added/changed: direct-entry route assertions plus route-tree generation become first-class verification surfaces.
- How a future agent inspects this: inspect `_dashboard*.tsx`, rerun the direct-entry Playwright grep, and verify `src/routeTree.gen.ts` regenerated from source files.
- Failure state exposed: missing route files, wrong route content, build-time route-tree drift, and route-specific console/request failures.

## Inputs

- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-shell.tsx` — shared shell extracted in T01/T02.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/issues-page.tsx` — route-ready Issues leaf from T02.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/performance-page.tsx` — existing Performance leaf.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/solana-programs-page.tsx` — existing Solana Programs leaf.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/releases-page.tsx` — existing Releases leaf.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/alerts-page.tsx` — existing Alerts leaf.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/bounties-page.tsx` — existing Bounties leaf.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/treasury-page.tsx` — existing Treasury leaf.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings/settings-page.tsx` — existing Settings leaf.
- `../hyperpush-mono/mesher/frontend-exp/src/routes/__root.tsx` — S01 root route/document surface.
- `../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx` — temporary adapter route to retire.
- `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts` — parity spec to extend with direct-entry coverage.

## Expected Output

- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.tsx` — pathless shared dashboard layout route.
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.index.tsx` — real Issues route at `/`.
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.performance.tsx` — real `/performance` route.
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.solana-programs.tsx` — real `/solana-programs` route.
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.releases.tsx` — real `/releases` route.
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.alerts.tsx` — real `/alerts` route.
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.bounties.tsx` — real `/bounties` route.
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.treasury.tsx` — real `/treasury` route.
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.settings.tsx` — real `/settings` route.
- `../hyperpush-mono/mesher/frontend-exp/src/routeTree.gen.ts` — regenerated TanStack route tree from the new file routes.
