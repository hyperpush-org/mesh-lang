---
id: T01
parent: S03
milestone: M058
key_files:
  - ../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/types.ts
  - ../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/normalize.ts
  - ../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/admin.functions.ts
  - ../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/mutations.functions.ts
  - ../hyperpush-mono/mesher/frontend-exp/src/lib/dashboard/search.ts
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings/settings-page.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings/api-keys-panel.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/live-admin-settings.test.tsx
key_decisions:
  - Kept API-key list data masked in normalization and confined raw created key values to component-local one-time confirmation state.
  - Normalized invalid settings search back into safe router state and kept all settings subreads parallelized instead of gating them by tab.
duration: 
verification_result: mixed
completed_at: 2026-04-11T03:27:43.577Z
blocker_discovered: false
---

# T01: Added a live project API-key settings path with masked route state, create/revoke mutations, and router-owned settingsTab=api-keys behavior.

**Added a live project API-key settings path with masked route state, create/revoke mutations, and router-owned settingsTab=api-keys behavior.**

## What Happened

Extended the Mesher frontend with reduced API-key types, masked normalization, and a new project-scoped API-key read adapter in admin.functions.ts without adding any new backend route family. Updated mutations.functions.ts with typed create/revoke server functions, threaded API-key data through the dashboard route and page props, normalized invalid settings search back into safe router state, and kept the settings loader parallelized via Promise.allSettled. Added a dedicated API keys settings panel that validates labels, exposes pending/error state, shows revoked rows explicitly, and limits raw created key values to one-time component-local confirmation state. Expanded normalization, search, route, and settings UI tests, and added a new live-admin-settings.test.tsx slice verifier target.

## Verification

Passed the task-level Vitest target for normalize/search/index route coverage, passed the broader slice-level frontend Vitest target including live-shell, live-alerts-settings, live-admin-settings, and __root coverage, and passed the frontend production build. The DB-backed slice checks did not pass because no Postgres server was listening on 127.0.0.1:5432 in this workspace, so migrate.sh up and smoke.sh failed before stack-level verification could continue. The final verify-s03-supported-admin.mjs command still needs to run once the Mesher listener is available.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test -- --run src/lib/mesher/normalize.test.ts src/lib/dashboard/search.test.ts src/routes/index.test.tsx` | 0 | ✅ pass | 5380ms |
| 2 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test -- --run src/lib/mesher/normalize.test.ts src/lib/dashboard/search.test.ts src/routes/index.test.tsx src/routes/__root.test.tsx components/dashboard/live-shell.test.tsx components/dashboard/live-alerts-settings.test.tsx components/dashboard/live-admin-settings.test.tsx` | 0 | ✅ pass | 6130ms |
| 3 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build` | 0 | ✅ pass | 3000ms |
| 4 | `DATABASE_URL=${DATABASE_URL:-postgres://postgres:postgres@127.0.0.1:5432/mesher} bash ../hyperpush-mono/mesher/scripts/migrate.sh up` | 1 | ❌ fail | 1000ms |
| 5 | `DATABASE_URL=${DATABASE_URL:-postgres://postgres:postgres@127.0.0.1:5432/mesher} bash ../hyperpush-mono/mesher/scripts/smoke.sh` | 2 | ❌ fail | 1000ms |

## Deviations

Touched additional UI files beyond the task plan’s expected-output list so the live API-key route state could actually render in SettingsPage and so the slice-level live-admin-settings.test.tsx verifier target existed locally.

## Known Issues

Local Postgres was unavailable at 127.0.0.1:5432, which blocked the migrate.sh and smoke.sh slice checks. MESHER_BASE_URL=http://127.0.0.1:18080 node ../hyperpush-mono/mesher/frontend-exp/scripts/verify-s03-supported-admin.mjs still needs to run once the Mesher listener is up; the auto-wrapup interrupt landed before that command executed.

## Files Created/Modified

- `../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/types.ts`
- `../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/normalize.ts`
- `../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/admin.functions.ts`
- `../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/mutations.functions.ts`
- `../hyperpush-mono/mesher/frontend-exp/src/lib/dashboard/search.ts`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings/settings-page.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings/api-keys-panel.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/live-admin-settings.test.tsx`
