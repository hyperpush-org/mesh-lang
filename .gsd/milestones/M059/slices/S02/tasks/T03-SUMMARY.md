---
id: T03
parent: S02
milestone: M059
key_files:
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-shell.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/issues-page.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-route-map.ts
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
  - ../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts
  - ../hyperpush-mono/mesher/frontend-exp/playwright.config.ts
  - playwright.config.ts
key_decisions:
  - D500 — Use a pathless `_dashboard` layout plus a matched root splat fallback instead of shell-local nav state or a root `notFoundComponent`.
duration: 
verification_result: passed
completed_at: 2026-04-11T16:37:12.011Z
blocker_discovered: false
---

# T03: Replaced the dashboard adapter with a pathless `_dashboard` route layout, real section file routes, and verified deep-link parity across dev and prod.

**Replaced the dashboard adapter with a pathless `_dashboard` route layout, real section file routes, and verified deep-link parity across dev and prod.**

## What Happened

I removed the temporary `/` adapter route and converted the shared dashboard shell into the single pathless `_dashboard` layout owner for the current top-level sections. The shell now derives active nav state from the TanStack router pathname, keeps the sidebar/header/AI-panel/provider mounted once, and renders child pages through an outlet instead of local branch switching. I added route files for `/`, `/performance`, `/solana-programs`, `/releases`, `/alerts`, `/bounties`, `/treasury`, and `/settings`, moved the Issues page AI hook onto a shell context so the route files stay thin, and replaced the root 404 fallback with a matched splat route that renders the Issues shell without a 404 document. I also extended the Playwright parity suite to cover direct-entry route loading, active-nav state, route-specific landmarks, unknown-path fallback, and clean console/request signals in both dev and prod. Because the slice verification commands run from `mesh-lang` with `npm --prefix ... exec playwright`, I added a workspace-root Playwright config shim so those commands resolve the sibling `frontend-exp` suite without changing directories.

## Verification

Verified the generated file routes and runtime behavior with `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build`, `npm --prefix ../hyperpush-mono/mesher/frontend-exp exec -- playwright test tests/e2e/dashboard-route-parity.spec.ts --project=dev --grep "direct-entry routes"`, the full dev parity suite, and the full prod parity suite. All route-parity checks passed, including unknown-path fallback, direct-entry boot for every top-level route, shell-state persistence, clean console output, and no failed requests.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build` | 0 | ✅ pass | 8400ms |
| 2 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp exec -- playwright test tests/e2e/dashboard-route-parity.spec.ts --project=dev --grep "direct-entry routes"` | 0 | ✅ pass | 32200ms |
| 3 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp exec -- playwright test tests/e2e/dashboard-route-parity.spec.ts --project=dev` | 0 | ✅ pass | 32900ms |
| 4 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp exec -- playwright test tests/e2e/dashboard-route-parity.spec.ts --project=prod` | 0 | ✅ pass | 57200ms |

## Deviations

Added a repo-root `playwright.config.ts` shim so the slice-plan Playwright commands work from `mesh-lang`; `npm --prefix ... exec playwright` keeps the current working directory and otherwise fails to resolve the sibling app's config/projects. No product-scope deviation was introduced.

## Known Issues

None.

## Files Created/Modified

- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-shell.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/issues-page.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-route-map.ts`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.index.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.performance.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.solana-programs.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.releases.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.alerts.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.bounties.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.treasury.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.settings.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/$.tsx`
- `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts`
- `../hyperpush-mono/mesher/frontend-exp/playwright.config.ts`
- `playwright.config.ts`
