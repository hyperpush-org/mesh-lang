---
id: T02
parent: S01
milestone: M058
key_files:
  - ../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/config.ts
  - ../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/client.ts
  - ../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/types.ts
  - ../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/normalize.ts
  - ../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/dashboard.functions.ts
  - ../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/issues.functions.ts
  - ../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/normalize.test.ts
key_decisions:
  - Mapped Mesher raw statuses and levels into a reduced live UI model only inside src/lib/mesher/normalize.ts while keeping server-function query filters aligned with backend-supported enums.
  - Kept the T02 dashboard seam limited to issues, dashboard/health, dashboard/levels, and dashboard/volume because the current S01 shell does not yet consume dashboard tags or top-issues.
duration:
verification_result: mixed
completed_at: 2026-04-11T00:22:54.187Z
blocker_discovered: false
---

# T02: Added a typed Mesher data seam with config/fetch helpers, reduced live dashboard types, server functions, and normalization tests.

**Added a typed Mesher data seam with config/fetch helpers, reduced live dashboard types, server functions, and normalization tests.**

## What Happened

Implemented the new src/lib/mesher/ seam for S01 by adding authoritative Mesher base-URL/project validation in config.ts, a bounded JSON fetch helper in client.ts, reduced live issue/dashboard types in types.ts, and pure response normalization in normalize.ts. The new normalizers intentionally shrink the mock-era contract: issue rows now carry only backend-supported fields, backend statuses are remapped once into the smaller live status set (open/resolved/ignored), and Mesher event levels are remapped into the live severity set (critical/high/medium/low). On top of that seam, I added typed TanStack Start server functions in dashboard.functions.ts and issues.functions.ts. They validate inputs with Zod, read only the existing Mesher routes used by this slice (issues, dashboard/health, dashboard/levels, and dashboard/volume), perform dashboard reads in parallel, and preserve endpoint-specific adapter failures in [mesher] ... endpoint=<route> project=<slug> messages so later route work can localize the first broken backend contract quickly. I also added normalize.test.ts to cover invalid/missing base URL handling, blank project slug rejection, unsupported status/level query values, empty issue/level states, malformed payload rejection, zero-volume buckets, blank assignee handling, and the intentional backend-to-live status/severity remapping.

## Verification

Verified the task-local Mesher adapter rail with npm --prefix ../hyperpush-mono/mesher/frontend-exp run test -- --run src/lib/mesher/normalize.test.ts and re-ran the slice-level frontend test/build commands to ensure the new server-function files compile inside the TanStack Start app. The named adapter test passed, the broader frontend test command passed with the existing __root smoke plus the new normalize suite, and the app build passed. As expected for an intermediate slice task, backend-dependent rails still failed in auto-mode because DATABASE_URL is unavailable, the replay script has not been created yet, and the visible dashboard widgets still import MOCK_* data pending T04.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test -- --run src/lib/mesher/normalize.test.ts` | 0 | ✅ pass | 4462ms |
| 2 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test -- --run src/routes/__root.test.tsx src/lib/mesher/normalize.test.ts src/routes/index.test.tsx components/dashboard/live-shell.test.tsx` | 0 | ✅ pass | 6156ms |
| 3 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build` | 0 | ✅ pass | 4764ms |
| 4 | `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash ../hyperpush-mono/mesher/scripts/migrate.sh up` | 127 | ❌ fail | 11ms |
| 5 | `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash ../hyperpush-mono/mesher/scripts/smoke.sh` | 127 | ❌ fail | 7ms |
| 6 | `node ../hyperpush-mono/mesher/frontend-exp/scripts/verify-s01-live-dashboard.mjs` | 1 | ❌ fail | 172ms |
| 7 | `! rg -n 'MOCK_(ISSUES|STATS|EVENT_SERIES)' ../hyperpush-mono/mesher/frontend-exp/src ../hyperpush-mono/mesher/frontend-exp/components/dashboard/stats-bar.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/events-chart.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-list.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-detail.tsx` | 1 | ❌ fail | 49ms |

## Deviations

None.

## Known Issues

DATABASE_URL is not available in this auto-mode run, so the Mesher migrate/smoke rails could not be exercised. ../hyperpush-mono/mesher/frontend-exp/scripts/verify-s01-live-dashboard.mjs does not exist yet because it is planned for T04, and the visible dashboard widgets still import MOCK_STATS / MOCK_EVENT_SERIES until the route and live-shell tasks land.

## Files Created/Modified

- `../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/config.ts`
- `../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/client.ts`
- `../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/types.ts`
- `../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/normalize.ts`
- `../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/dashboard.functions.ts`
- `../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/issues.functions.ts`
- `../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/normalize.test.ts`
