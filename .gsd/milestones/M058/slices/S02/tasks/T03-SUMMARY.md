---
id: T03
parent: S02
milestone: M058
key_files:
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx
  - ../hyperpush-mono/mesher/frontend-exp/app/page.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alerts-page.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alert-list.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alert-detail.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alert-stats.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/index.test.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/live-alerts-settings.test.tsx
key_decisions:
  - D477 — keep fired-alert read failures inside the alerts view as explicit in-page error state instead of failing the entire dashboard route.
duration: 
verification_result: mixed
completed_at: 2026-04-11T02:09:26.887Z
blocker_discovered: false
---

# T03: Replaced the mock alerts page with router-owned live fired alerts, real acknowledge/resolve mutations, and explicit stale/error alert states.

**Replaced the mock alerts page with router-owned live fired alerts, real acknowledge/resolve mutations, and explicit stale/error alert states.**

## What Happened

Updated the dashboard route so `view=alerts` now performs a fired-alert read only for the alerts surface, keeps the selected alert in validated router search state, and clears stale alert ids only after a successful refresh proves the selected item disappeared. Rebuilt the alerts page, list, detail, and stats components around the reduced Mesher fired-alert contract, removed the old mock-only alert affordances, and wired acknowledge/resolve through the existing Mesher mutation server functions with route invalidation instead of local optimistic state. Added route and component coverage for alert navigation, alert read failure visibility, stale-selection cleanup, empty states, and action-driven refresh plus pending/error banners.

## Verification

Passed the task verification suite with `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test -- --run src/routes/index.test.tsx components/dashboard/live-alerts-settings.test.tsx` and confirmed the frontend still builds with `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build`. The focused alert-only mock guard now passes, while the slice-wide mock guard remains red only because settings is still mock-backed for the next task. A real local browser run against `http://localhost:3000/?project=default&view=alerts` was attempted, but runtime verification of successful live alert interactions is blocked in this workspace by missing `MESHER_BASE_URL` configuration.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test -- --run src/routes/index.test.tsx components/dashboard/live-alerts-settings.test.tsx` | 0 | ✅ pass | 8202ms |
| 2 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build` | 0 | ✅ pass | 6027ms |
| 3 | `bash -lc "! rg -n 'MOCK_ALERTS|MOCK_ALERT_STATS|PROJECT_CONFIG|ALERT_RULES|MOCK_TREASURY' ../hyperpush-mono/mesher/frontend-exp/src ../hyperpush-mono/mesher/frontend-exp/app/page.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-detail.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alerts-page.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alert-list.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alert-detail.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alert-stats.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings"` | 1 | ❌ fail | 58ms |
| 4 | `bash -lc "! rg -n 'MOCK_ALERTS|MOCK_ALERT_STATS' ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alerts-page.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alert-list.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alert-detail.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alert-stats.tsx ../hyperpush-mono/mesher/frontend-exp/app/page.tsx ../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx"` | 0 | ✅ pass | 54ms |

## Deviations

Created `components/dashboard/live-alerts-settings.test.tsx` because the planned verification target did not exist locally. The new file follows the existing Vitest + Testing Library dashboard patterns.

## Known Issues

The slice-wide no-mock guard still fails because `components/dashboard/settings/settings-page.tsx` remains mock-backed for the later settings/storage task. Real browser verification of successful live alerts is currently blocked by missing runtime `MESHER_BASE_URL` configuration, so the browser could only confirm the route-level configuration failure state.

## Files Created/Modified

- `../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx`
- `../hyperpush-mono/mesher/frontend-exp/app/page.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/alerts-page.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/alert-list.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/alert-detail.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/alert-stats.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/index.test.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/live-alerts-settings.test.tsx`
