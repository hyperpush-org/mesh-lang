---
id: T04
parent: S01
milestone: M058
key_files:
  - ../hyperpush-mono/mesher/frontend-exp/app/page.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/stats-bar.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/events-chart.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-list.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-detail.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/live-shell.test.tsx
  - ../hyperpush-mono/mesher/frontend-exp/scripts/verify-s01-live-dashboard.mjs
  - ../hyperpush-mono/mesher/frontend-exp/package.json
  - .gsd/KNOWLEDGE.md
  - .gsd/DECISIONS.md
key_decisions:
  - D472: Keep the S01 overview widgets pinned to the reduced LiveDashboardModel and LiveIssue contract so unsupported mock-era fields cannot leak back into the active dashboard path.
duration: 
verification_result: mixed
completed_at: 2026-04-11T00:56:24.783Z
blocker_discovered: false
---

# T04: Replaced the dashboard’s mock-backed overview widgets with live Mesher stats, volume, and reduced issue components, and added an S01 replay verifier.

**Replaced the dashboard’s mock-backed overview widgets with live Mesher stats, volume, and reduced issue components, and added an S01 replay verifier.**

## What Happened

Rewrote the shared S01 dashboard surfaces so they render only from the reduced Mesher seam: the stats bar now shows backend-supported health/severity counts, the events chart now renders a single live volume series with explicit empty-state handling, and the issue list/detail now consume only the supported LiveIssue fields. Removed the duplicate inline live list/detail implementations from app/page.tsx so the route shell and shared components now exercise the same honest code path. Added components/dashboard/live-shell.test.tsx to lock the honest rendering and boundary states, and added frontend-exp/scripts/verify-s01-live-dashboard.mjs plus a package script alias so the seeded default project can be replay-checked for endpoint shape drift and forbidden MOCK_* imports.

## Verification

Ran the slice frontend verification subset and the production build successfully. Ran the active-path no-mock rg guard successfully. Attempted the Mesher migration and smoke rails exactly as specified, but the current auto-mode environment has no DATABASE_URL, so both commands stopped at their guard clause before any database/runtime proof could run. Ran the new replay verifier, which failed closed with a backend-unavailable error because no live Mesher instance was running in this session.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test -- --run src/routes/__root.test.tsx src/lib/mesher/normalize.test.ts src/routes/index.test.tsx components/dashboard/live-shell.test.tsx` | 0 | ✅ pass | 6975ms |
| 2 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build` | 0 | ✅ pass | 5392ms |
| 3 | `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash ../hyperpush-mono/mesher/scripts/migrate.sh up` | 127 | ❌ fail | 24ms |
| 4 | `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash ../hyperpush-mono/mesher/scripts/smoke.sh` | 127 | ❌ fail | 20ms |
| 5 | `node ../hyperpush-mono/mesher/frontend-exp/scripts/verify-s01-live-dashboard.mjs` | 1 | ❌ fail | 312ms |
| 6 | `! rg -n 'MOCK_(ISSUES|STATS|EVENT_SERIES)' ../hyperpush-mono/mesher/frontend-exp/src ../hyperpush-mono/mesher/frontend-exp/components/dashboard/stats-bar.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/events-chart.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-list.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-detail.tsx` | 0 | ✅ pass | 46ms |

## Deviations

Added the package script alias verify:s01-live-dashboard so the new replay verifier has a stable package-local entrypoint. Otherwise none.

## Known Issues

Full backend verification remains incomplete in this environment because DATABASE_URL is unset and no live Mesher backend was available. The replay verifier intentionally fails closed until a real Mesher instance is running.

## Files Created/Modified

- `../hyperpush-mono/mesher/frontend-exp/app/page.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/stats-bar.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/events-chart.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-list.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-detail.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/live-shell.test.tsx`
- `../hyperpush-mono/mesher/frontend-exp/scripts/verify-s01-live-dashboard.mjs`
- `../hyperpush-mono/mesher/frontend-exp/package.json`
- `.gsd/KNOWLEDGE.md`
- `.gsd/DECISIONS.md`
