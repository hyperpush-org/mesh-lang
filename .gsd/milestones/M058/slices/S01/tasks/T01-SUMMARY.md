---
id: T01
parent: S01
milestone: M058
key_files:
  - ../hyperpush-mono/mesher/frontend-exp/package.json
  - ../hyperpush-mono/mesher/frontend-exp/tsconfig.json
  - ../hyperpush-mono/mesher/frontend-exp/vite.config.ts
  - ../hyperpush-mono/mesher/frontend-exp/vitest.config.ts
  - ../hyperpush-mono/mesher/frontend-exp/src/router.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/__root.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/__root.test.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/test/setup.ts
  - ../hyperpush-mono/.gitignore
key_decisions:
  - Pinned @tanstack/react-start to 1.167.20 because 1.167.21 currently publishes a broken dependency on @tanstack/react-start-rsc@0.0.1.
  - Kept the existing app/page.tsx dashboard shell as the route component while moving routing/bootstrap ownership into src/router.tsx and src/routes/*.tsx.
  - Added routeFileIgnorePattern to both Vite configs so colocated src/routes/*.test.tsx files do not get treated as route inputs by TanStack Start.
duration:
verification_result: mixed
completed_at: 2026-04-11T00:11:05.295Z
blocker_discovered: false
---

# T01: Replaced frontend-exp's Next.js bootstrap with a TanStack Start/Vite router scaffold and reusable Vitest route harness.

**Replaced frontend-exp's Next.js bootstrap with a TanStack Start/Vite router scaffold and reusable Vitest route harness.**

## What Happened

Rewrote the frontend-exp bootstrap from Next.js to TanStack Start + Vite by replacing package/runtime config, removing stale Next-only ownership files, and adding the new router entry under src/. The new root route owns the document shell, links the existing app/globals.css theme, restores the old metadata/icons, and mounts the existing dashboard shell from app/page.tsx at /. I also introduced a reusable Vitest + React Testing Library harness with jsdom shims and a real root-route smoke test, then fixed two execution-time issues: TanStack Start test files under src/routes needed an explicit routeFileIgnorePattern, and @tanstack/react-start had to be pinned to 1.167.20 because 1.167.21 currently resolves a missing npm dependency (@tanstack/react-start-rsc@0.0.1).

## Verification

Task-local verification passed with the named route smoke test and production build. Slice-level verification is partially green as expected for the first task: the broader frontend test command exits successfully but only exercises the currently existing __root test, while the live-data replay script, backend-dependent migrate/smoke rails, and the no-MOCK grep remain red until later slice tasks land.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test -- --run src/routes/__root.test.tsx` | 0 | ✅ pass | 5367ms |
| 2 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build` | 0 | ✅ pass | 4734ms |
| 3 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test -- --run src/routes/__root.test.tsx src/lib/mesher/normalize.test.ts src/routes/index.test.tsx components/dashboard/live-shell.test.tsx` | 0 | ✅ pass | 5396ms |
| 4 | `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash ../hyperpush-mono/mesher/scripts/migrate.sh up` | 127 | ❌ fail | 31ms |
| 5 | `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash ../hyperpush-mono/mesher/scripts/smoke.sh` | 127 | ❌ fail | 27ms |
| 6 | `node ../hyperpush-mono/mesher/frontend-exp/scripts/verify-s01-live-dashboard.mjs` | 1 | ❌ fail | 145ms |
| 7 | `! rg -n 'MOCK_(ISSUES|STATS|EVENT_SERIES)' ../hyperpush-mono/mesher/frontend-exp/src ../hyperpush-mono/mesher/frontend-exp/components/dashboard/stats-bar.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/events-chart.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-list.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-detail.tsx` | 1 | ❌ fail | 47ms |

## Deviations

Updated ../hyperpush-mono/.gitignore to ignore mesher/frontend-exp/.output/ because TanStack Start/Nitro now emits build artifacts there. Marked components.json as non-RSC to match the post-Next runtime shape.

## Known Issues

The jsdom smoke test still logs expected warnings about rendering a full-document TanStack Start root inside the test container and about zero-sized Recharts layout during the smoke render; the assertions are stable and the command exits green. Slice-level live-data proof surfaces remain incomplete in this task: DATABASE_URL is unavailable in auto-mode, scripts/verify-s01-live-dashboard.mjs is not present yet, and dashboard widgets still import MOCK_* data pending later tasks.

## Files Created/Modified

- `../hyperpush-mono/mesher/frontend-exp/package.json`
- `../hyperpush-mono/mesher/frontend-exp/tsconfig.json`
- `../hyperpush-mono/mesher/frontend-exp/vite.config.ts`
- `../hyperpush-mono/mesher/frontend-exp/vitest.config.ts`
- `../hyperpush-mono/mesher/frontend-exp/src/router.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/__root.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/__root.test.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/test/setup.ts`
- `../hyperpush-mono/.gitignore`
