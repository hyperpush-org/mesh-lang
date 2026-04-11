---
id: T04
parent: S02
milestone: M058
key_files:
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx
  - ../hyperpush-mono/mesher/frontend-exp/app/page.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings/settings-page.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings/project-settings-card.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings/storage-summary.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings/alert-rules-panel.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/live-alerts-settings.test.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/index.test.tsx
  - ../hyperpush-mono/mesher/frontend-exp/scripts/verify-s02-live-detail-alerts.mjs
  - ../hyperpush-mono/mesher/frontend-exp/package.json
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Keep settings reads fail-soft per endpoint inside the settings view, but only revalidate router-owned state after successful settings and alert-rule mutations so failed saves keep local form input visible.
  - Make the S02 replay verifier throw on failure instead of process.exit so temporary seeded-rule cleanup can still run in finally blocks.
duration: 
verification_result: passed
completed_at: 2026-04-11T02:33:44.331Z
blocker_discovered: false
---

# T04: Replaced the mock settings shell with live project settings/storage/alert-rule flows and added the S02 Mesher replay verifier.

**Replaced the mock settings shell with live project settings/storage/alert-rule flows and added the S02 Mesher replay verifier.**

## What Happened

I replaced the old mock-heavy settings surface in ../hyperpush-mono/mesher/frontend-exp with a reduced live settings experience that only exposes backend-supported project settings, storage, and alert-rule management. The dashboard route now loads project settings, storage, and alert rules in parallel for view=settings, keeps per-endpoint read failures inside the settings view, and routes successful settings/rule mutations back through router invalidation so the UI refreshes from Mesher truth. I added focused settings subcomponents, expanded settings/alerts tests, updated route integration tests for honest navigation and revalidation, and added a live replay verifier that checks issue/event/alert/settings/storage endpoint shapes, seeds a real issue/event when needed, and fails closed on mock-era identifiers in the active S02 path. During verification I found the seeded alert could arrive after the original settle window, so I raised the replay verifier settle timeout and changed failure handling to throw so temporary seeded-rule cleanup can still run.

## Verification

Verified the full slice frontend proof rail with Vitest and a production Vite build, then ran Mesher migration and smoke rails against a temporary local Postgres 16 container and executed the new S02 replay verifier against a real Mesher runtime on http://127.0.0.1:18080. Also checked the active S02 path for forbidden mock-era identifiers. All required checks passed.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test -- --run src/lib/mesher/normalize.test.ts src/lib/dashboard/search.test.ts src/routes/index.test.tsx components/dashboard/live-shell.test.tsx components/dashboard/live-alerts-settings.test.tsx` | 0 | ✅ pass | 9139ms |
| 2 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build` | 0 | ✅ pass | 5460ms |
| 3 | `DATABASE_URL=${DATABASE_URL:-postgres://postgres:postgres@127.0.0.1:5432/mesher} bash ../hyperpush-mono/mesher/scripts/migrate.sh up` | 0 | ✅ pass | 0ms |
| 4 | `DATABASE_URL=${DATABASE_URL:-postgres://postgres:postgres@127.0.0.1:5432/mesher} bash ../hyperpush-mono/mesher/scripts/smoke.sh` | 0 | ✅ pass | 0ms |
| 5 | `MESHER_BASE_URL=http://127.0.0.1:18080 node ../hyperpush-mono/mesher/frontend-exp/scripts/verify-s02-live-detail-alerts.mjs` | 0 | ✅ pass | 0ms |
| 6 | `! rg -n 'MOCK_ALERTS|MOCK_ALERT_STATS|PROJECT_CONFIG|ALERT_RULES|MOCK_TREASURY' ../hyperpush-mono/mesher/frontend-exp/src ../hyperpush-mono/mesher/frontend-exp/app/page.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-detail.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alerts-page.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alert-list.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alert-detail.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alert-stats.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings` | 0 | ✅ pass | 0ms |

## Deviations

The planned smoke→replay sequence needed a local runtime adaptation because mesher/scripts/smoke.sh did not guarantee a follow-on verifier-ready listener in this workspace. I documented that gotcha in .gsd/KNOWLEDGE.md and reran the replay verifier against a real Mesher listener on 127.0.0.1:18080 without changing the slice contract.

## Known Issues

src/routes/index.test.tsx still emits the existing TanStack/React stderr noise about <html> in the test container and intentional route-error logs during failure-path assertions, even though the tests pass.

## Files Created/Modified

- `../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx`
- `../hyperpush-mono/mesher/frontend-exp/app/page.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings/settings-page.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings/project-settings-card.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings/storage-summary.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings/alert-rules-panel.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/live-alerts-settings.test.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/index.test.tsx`
- `../hyperpush-mono/mesher/frontend-exp/scripts/verify-s02-live-detail-alerts.mjs`
- `../hyperpush-mono/mesher/frontend-exp/package.json`
- `.gsd/KNOWLEDGE.md`
