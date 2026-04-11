---
id: S02
parent: M059
milestone: M059
provides:
  - A real TanStack route tree for the current dashboard sections inside `../hyperpush-mono/mesher/frontend-exp/`, including direct-entry deep links for `/`, `/performance`, `/solana-programs`, `/releases`, `/alerts`, `/bounties`, `/treasury`, and `/settings`.
  - A shared route-aware dashboard shell with shell-owned Issues state persistence, pathname-derived active nav, AI close-on-nav behavior, and settings-specific header suppression.
  - Repo-owned dev and built-production route-parity verification surfaces that downstream slices can reuse during the `mesher/client` move and final equivalence cleanup.
  - Recorded npm/Playwright and sibling-workspace verification gotchas in `.gsd/KNOWLEDGE.md` and `.gsd/DECISIONS.md` so later slices do not rediscover the same command-contract trap.
requires:
  - slice: M059/S01
    provides: TanStack Start/Vite groundwork, the package-local production bridge server, root route plumbing, and the preserved external `dev` / `build` / `start` contract inside `frontend-exp`.
affects:
  - M059/S03
  - M059/S04
key_files:
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-shell.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-route-map.ts
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-issues-state.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/issues-page.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/sidebar.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.index.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.performance.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.solana-programs.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.releases.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.alerts.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.bounties.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.treasury.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.settings.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/$.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/routeTree.gen.ts
  - ../hyperpush-mono/mesher/frontend-exp/playwright.config.ts
  - ../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts
  - ../hyperpush-mono/mesher/frontend-exp/package.json
  - playwright.config.ts
key_decisions:
  - D497 — Use a pathless `_dashboard` layout route with flat child route files for the current top-level sections while keeping Issues on `/` and keeping shared shell state in the layout.
  - D498 — Move the active dashboard runtime into a shared client shell plus route-map helper and have TanStack render that shell directly instead of importing the legacy monolithic page.
  - D499 — Keep Issues state in a shell-owned provider and expose explicit shell-state attributes for route-parity verification.
  - D500 — Use a pathless `_dashboard` layout plus a matched root splat fallback instead of shell-local nav state or a root `notFoundComponent`.
  - D501 — Let TanStack links own section navigation while DashboardShell derives the active route from pathname and keeps only transient UI state locally.
  - D502 — Use PLAYWRIGHT_PROJECT-backed npm scripts as the stable isolated dev/prod parity contract because current npm `exec playwright ... --project=<name>` still runs both Playwright projects.
patterns_established:
  - Use a pathless TanStack layout route to keep one mounted dashboard shell/provider across top-level section changes when visual parity depends on persistent shell-owned UI state.
  - Keep Issues search/filter/detail ownership above route leaves when leave-and-return behavior matters, and expose stable shell-state hooks so parity verification does not depend on framework internals.
  - When verification must run from `mesh-lang` against the sibling product app, keep the exact contractual commands passing but add repo-owned isolated wrappers when npm CLI forwarding is not truthful enough for per-environment diagnosis.
observability_surfaces:
  - `../hyperpush-mono/mesher/frontend-exp/src/routeTree.gen.ts` for generated route ownership and deep-link coverage.
  - `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts` plus `playwright.config.ts` / `../hyperpush-mono/mesher/frontend-exp/playwright.config.ts` for dev and built-production parity assertions.
  - Dev runtime on `http://127.0.0.1:3000/` and built production on `http://127.0.0.1:3001/`.
  - Browser console logs, request errors, active-nav state, AI-panel visibility, settings-header suppression, and visible Issues search/detail state during live browser verification.
  - Repo-owned `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test:e2e:dev` and `... run test:e2e:prod` commands as the authoritative isolated parity rails.
drill_down_paths:
  - .gsd/milestones/M059/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M059/slices/S02/tasks/T02-SUMMARY.md
  - .gsd/milestones/M059/slices/S02/tasks/T03-SUMMARY.md
  - .gsd/milestones/M059/slices/S02/tasks/T04-SUMMARY.md
  - .gsd/milestones/M059/slices/S02/tasks/T05-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-11T17:32:06.849Z
blocker_discovered: false
---

# S02: Route-backed dashboard parity

**Replaced the temporary single-route dashboard adapter in `../hyperpush-mono/mesher/frontend-exp/` with real TanStack file routes while preserving the current dashboard shell, mock-data interactions, and the external `dev` / `build` / `start` contract.**

## What Happened

S02 completed the route-backed parity slice inside the existing `frontend-exp` app without widening into backend work. The slice moved the active TanStack runtime off the temporary `app/page.tsx` monolith and into a shared `DashboardShell` plus route map, extracted the Issues experience into its own route-ready page with shell-owned search/filter/detail state, and replaced the temporary single-route adapter with a pathless `_dashboard` layout and real TanStack file routes for `/`, `/performance`, `/solana-programs`, `/releases`, `/alerts`, `/bounties`, `/treasury`, and `/settings`. The sidebar now uses router-backed links, active section state derives from pathname, AI Copilot closes on pathname changes, `/settings` keeps its self-owned page chrome, and a matched root splat route returns the Issues shell for unknown paths. The slice stayed entirely on mock data and client state while also adding repo-owned Playwright parity rails for dev and built production.

## Verification

Build passed. Dev parity passed. Prod parity passed. Browser checks showed zero console errors and zero request errors.

## Requirements Advanced

- R143 — Moved the active dashboard from the temporary adapter onto real TanStack file routes while keeping the visible shell, headings, panels, and mock-data interactions aligned with the pre-migration experience.
- R145 — Proved URL structure, sidebar navigation, AI panel behavior, settings header behavior, and Issues filter/search/detail leave-and-return behavior across dev and built production.
- R146 — Kept the slice entirely on the existing mock-data and client-state path: no TanStack loaders, no server functions, no Mesher backend calls, and no widened URL-search-param contract were introduced.
- R147 — Strengthened the no-Next-runtime path by proving the new route tree boots under `npm run build` and `npm run start`, including non-root direct-entry routes in built production.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

The exact slice-plan Playwright commands still work from `mesh-lang`, but current npm treats `--project=<name>` as npm config and launches both Playwright projects instead of one isolated environment. S02 kept those commands passing for contractual parity, and also added repo-owned `test:e2e:dev` and `test:e2e:prod` scripts as the truthful isolated verification surface.

## Known Limitations

The app still lives at `../hyperpush-mono/mesher/frontend-exp/`; the canonical move to `../hyperpush-mono/mesher/client/` remains S03 work. This slice intentionally stayed on mock data and client state only: there are still no TanStack loaders, server functions, or Mesher backend integrations in the migrated dashboard.

## Follow-ups

S03 should move the now route-backed app from `frontend-exp` to `mesher/client` while preserving the same external `dev` / `build` / `start` command contract and keeping the deep-link parity suite green. S04 should update product-repo docs and workflows that still reference `frontend-exp` or Next.js, then use the same route-parity/browser surfaces for final equivalence and operational cleanup.

## Files Created/Modified

None.
