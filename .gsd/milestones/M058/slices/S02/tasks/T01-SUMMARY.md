---
id: T01
parent: S02
milestone: M058
key_files:
  - ../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/types.ts
  - ../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/normalize.ts
  - ../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/detail.functions.ts
  - ../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/mutations.functions.ts
  - ../hyperpush-mono/mesher/frontend-exp/src/lib/dashboard/search.ts
  - ../hyperpush-mono/mesher/frontend-exp/src/lib/dashboard/search.test.ts
key_decisions:
  - D475 — keep both project-scoped and top-level Mesher routes behind the shared adapter seam and reduce event detail into summary/count fields instead of raw JSONB payloads.
duration:
verification_result: mixed
completed_at: 2026-04-11T01:42:13.788Z
blocker_discovered: false
---

# T01: Added typed Mesher detail/alert/settings adapters and S02 router search validation for live selections.

**Added typed Mesher detail/alert/settings adapters and S02 router search validation for live selections.**

## What Happened

Extended the frontend Mesher seam with reduced live contracts plus new `detail.functions.ts`, `alerts.functions.ts`, `settings.functions.ts`, and `mutations.functions.ts` wrappers over the existing Mesher detail, alerts, settings, storage, and mutation endpoints. Updated the shared Mesher URL/fetch layer so both project-scoped and top-level backend routes stay behind one typed adapter seam, expanded `src/lib/dashboard/search.ts` to validate `view`, `issue`, `event`, `alert`, and `settingsTab`, added stale-selection cleanup helpers, and added Vitest coverage for malformed payloads, unsupported enums, timeout/HTTP/invalid-JSON failures, and invalid route state cleanup.

## Verification

Task verification passed with `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test -- --run src/lib/mesher/normalize.test.ts src/lib/dashboard/search.test.ts` and `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build`. The slice-level mock-import guard was also checked and still fails on the current alerts/settings mock UI surfaces, which is expected until T03/T04 replace those components. Database migrate/smoke and live replay checks remain for later slice tasks because T01 only delivers the adapter/search seam.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test -- --run src/lib/mesher/normalize.test.ts src/lib/dashboard/search.test.ts` | 0 | ✅ pass | 2280ms |
| 2 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build` | 0 | ✅ pass | 3140ms |
| 3 | `rg -n 'MOCK_ALERTS|MOCK_ALERT_STATS|PROJECT_CONFIG|ALERT_RULES|MOCK_TREASURY' ../hyperpush-mono/mesher/frontend-exp/src ../hyperpush-mono/mesher/frontend-exp/app/page.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-detail.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alerts-page.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alert-list.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alert-detail.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alert-stats.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings — exit 0 with matches found; expected slice-level guard remains red until T03/T04.` | -1 | unknown (coerced from string) | 0ms |

## Deviations

Also updated `src/lib/mesher/config.ts` and `src/lib/mesher/client.ts` so one shared adapter can call both project-scoped and top-level Mesher routes; this was required by the real backend path mix even though those files were not listed in the expected-output section.

## Known Issues

Alerts and settings components still contain mock-backed imports/usages (`MOCK_ALERTS`, `MOCK_ALERT_STATS`, `PROJECT_CONFIG`, `ALERT_RULES`, `MOCK_TREASURY`), so the slice-level no-mock guard is intentionally still red at T01. Live migrate/smoke/replay verification is still pending the later route/UI tasks.

## Files Created/Modified

- `../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/types.ts`
- `../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/normalize.ts`
- `../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/detail.functions.ts`
- `../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/mutations.functions.ts`
- `../hyperpush-mono/mesher/frontend-exp/src/lib/dashboard/search.ts`
- `../hyperpush-mono/mesher/frontend-exp/src/lib/dashboard/search.test.ts`
