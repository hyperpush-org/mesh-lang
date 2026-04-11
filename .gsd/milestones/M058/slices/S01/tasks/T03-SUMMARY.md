---
id: T03
parent: S01
milestone: M058
key_files:
  - ../hyperpush-mono/mesher/frontend-exp/src/lib/dashboard/search.ts
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx
  - ../hyperpush-mono/mesher/frontend-exp/app/page.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/header.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/sidebar.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/index.test.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/__root.test.tsx
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Kept URL normalization, loader-deps derivation, and client-side fallback filtering in one src/lib/dashboard/search.ts helper so the route can stay canonical even when Mesher cannot express every reduced UI filter value one-to-one.
  - Updated the root-route smoke test to mock Mesher server functions now that / performs real route loading instead of rendering a static shell.
duration: 
verification_result: mixed
completed_at: 2026-04-11T00:40:02.570Z
blocker_discovered: false
---

# T03: Wired the dashboard route loader to validated URL state and removed duplicate header/sidebar project ownership.

**Wired the dashboard route loader to validated URL state and removed duplicate header/sidebar project ownership.**

## What Happened

Added src/lib/dashboard/search.ts as the route-owned search authority for project, status, level, q, and panel/issue state, including normalization, patch helpers, backend-filter mapping, and client-side fallback filtering for reduced UI values that Mesher cannot express one-to-one. Rewired src/routes/index.tsx to use validateSearch, loaderDeps, and a parallel Promise.all loader over the Mesher dashboard/issues server functions, then simplified header/sidebar state so the current project slug and filters come from the router instead of local hooks. Reworked app/page.tsx into a loader-fed shell that preserves the existing layout/nav affordances while reading live issue data, selection, and AI/detail panel state from route-owned handlers. Added src/routes/index.test.tsx for invalid search normalization, stale selected-issue cleanup, empty-state rendering, route-level loader errors, and canonical URL round-tripping, and updated src/routes/__root.test.tsx to mock the new loader dependencies so the root smoke test remains green.

## Verification

The task-local verification command passed: npm --prefix ../hyperpush-mono/mesher/frontend-exp run test -- --run src/routes/index.test.tsx. I also ran the slice-level frontend test and build rails; both passed. Slice-level backend/env-gated rails remain red at this intermediate task because DATABASE_URL is unavailable in auto-mode, the replay script is still a T04 deliverable, and the no-MOCK grep still fails while stats-bar.tsx and events-chart.tsx remain mock-backed.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test -- --run src/routes/index.test.tsx` | 0 | ✅ pass | 3640ms |
| 2 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test -- --run src/routes/__root.test.tsx src/lib/mesher/normalize.test.ts src/routes/index.test.tsx components/dashboard/live-shell.test.tsx` | 0 | ✅ pass | 6593ms |
| 3 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build` | 0 | ✅ pass | 5019ms |
| 4 | `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash ../hyperpush-mono/mesher/scripts/migrate.sh up` | 127 | ❌ fail | 12ms |
| 5 | `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash ../hyperpush-mono/mesher/scripts/smoke.sh` | 127 | ❌ fail | 8ms |
| 6 | `node ../hyperpush-mono/mesher/frontend-exp/scripts/verify-s01-live-dashboard.mjs` | 1 | ❌ fail | 133ms |
| 7 | `! rg -n 'MOCK_(ISSUES|STATS|EVENT_SERIES)' ../hyperpush-mono/mesher/frontend-exp/src ../hyperpush-mono/mesher/frontend-exp/components/dashboard/stats-bar.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/events-chart.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-list.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-detail.tsx` | 1 | ❌ fail | 30ms |

## Deviations

Rendered the temporary live issue list/detail directly inside app/page.tsx instead of pulling the full T04 issue-list/issue-detail rewrite forward. This let T03 land the route-owned state model without widening scope into the planned widget cleanup task.

## Known Issues

components/dashboard/stats-bar.tsx and components/dashboard/events-chart.tsx still import MOCK_* data, so the slice no-mock guard remains red until T04. Route tests also emit pre-existing jsdom warnings about rendering the TanStack Start document root and zero-sized Recharts containers during non-visual test renders, but the assertions are stable and the commands exit green.

## Files Created/Modified

- `../hyperpush-mono/mesher/frontend-exp/src/lib/dashboard/search.ts`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx`
- `../hyperpush-mono/mesher/frontend-exp/app/page.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/header.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/sidebar.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/index.test.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/__root.test.tsx`
- `.gsd/DECISIONS.md`
- `.gsd/KNOWLEDGE.md`
