---
id: T04
parent: S02
milestone: M059
key_files:
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/sidebar.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-shell.tsx
  - ../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts
key_decisions:
  - D501 — Let TanStack links own section navigation while DashboardShell derives the active route from pathname and only keeps transient UI state locally.
duration: 
verification_result: passed
completed_at: 2026-04-11T16:51:04.403Z
blocker_discovered: false
---

# T04: Made the dashboard sidebar URL-backed, moved AI close behavior onto pathname changes, and extended parity coverage for URL/nav/settings-shell alignment.

**Made the dashboard sidebar URL-backed, moved AI close behavior onto pathname changes, and extended parity coverage for URL/nav/settings-shell alignment.**

## What Happened

I converted the dashboard sidebar’s top-level section controls to real TanStack links, kept the visual chrome unchanged, and let DashboardShell keep only transient UI state. The shell now derives the active section from the current pathname and closes the AI panel from a pathname-change effect so route changes, deep links, and history navigation stay in parity without pushing Issues filters/detail state into the router. I also expanded the parity suite to assert URL/state alignment, AI close-on-nav, collapsed-sidebar persistence, settings header suppression, and Issues leave-and-return persistence.

## Verification

`npm --prefix ../hyperpush-mono/mesher/frontend-exp run build`, the targeted dev navigation-parity Playwright grep, and the full dev/prod route-parity suites all passed. I also exercised the live dev server in the browser and confirmed URL-backed navigation, active-nav updates, settings header suppression, collapsed-sidebar persistence, preserved Issues search/filter state, no console errors, and no failed network requests.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build` | 0 | ✅ pass | 8571ms |
| 2 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp exec -- playwright test tests/e2e/dashboard-route-parity.spec.ts --project=dev --grep "navigation parity"` | 0 | ✅ pass | 17400ms |
| 3 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp exec -- playwright test tests/e2e/dashboard-route-parity.spec.ts --project=dev` | 0 | ✅ pass | 41800ms |
| 4 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp exec -- playwright test tests/e2e/dashboard-route-parity.spec.ts --project=prod` | 0 | ✅ pass | 36000ms |

## Deviations

Used `npm exec -- playwright ...` for verification so npm 10 forwarded `--project` and `--grep` to Playwright correctly. No product-scope deviation was introduced.

## Known Issues

None.

## Files Created/Modified

- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/sidebar.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-shell.tsx`
- `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts`
