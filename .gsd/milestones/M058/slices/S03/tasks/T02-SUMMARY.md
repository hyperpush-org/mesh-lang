---
id: T02
parent: S03
milestone: M058
key_files:
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings/api-keys-panel.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings/settings-page.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings/project-settings-card.tsx
  - ../hyperpush-mono/mesher/frontend-exp/app/page.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/sidebar.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-detail.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/lib/dashboard/search.ts
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/live-admin-settings.test.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/live-shell.test.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/__root.test.tsx
key_decisions:
  - Removed the unsupported AI copilot panel from the active-shell route model and UI so dead chrome cannot reappear through URL state.
  - Kept ordinary API-key load errors inspectable with the current list visible, but fail-closed on malformed API-key rows by surfacing an explicit adapter error and hiding those rows.
duration:
verification_result: mixed
completed_at: 2026-04-11T03:40:23.437Z
blocker_discovered: false
---

# T02: Shipped an honest live API-key settings surface, removed fake copilot and identity chrome from the active shell, and locked both behaviors with focused regression tests.

**Shipped an honest live API-key settings surface, removed fake copilot and identity chrome from the active shell, and locked both behaviors with focused regression tests.**

## What Happened

Finished the S03 admin UI pass by hardening the existing project-scoped API-key settings path from T01 into an honest operator surface. The API-key panel now preserves create/revoke pending state, keeps raw created key values confined to one-time local confirmation UI, and fails closed on malformed API-key rows with an explicit endpoint=api-keys error instead of rendering partial data. I updated the settings copy so it truthfully states that API keys, storage, alert rules, and retention controls are live while team membership remains deferred until Mesher exposes safe org discovery. I also removed the fake AI copilot affordance from the dashboard header and issue detail, replaced the hardcoded alex.kim/owner sidebar footer with neutral live-shell copy, reduced the route-owned dashboard panel model to the supported issue detail panel only, and expanded the focused Vitest coverage so shell honesty and API-key error/success boundaries fail fast if they regress.

## Verification

Passed the task-level Vitest verifier for live-alerts-settings, live-admin-settings, live-shell, and __root coverage; passed the broader slice-level frontend Vitest suite including normalize/search/index-route coverage; and passed the frontend production build. The DB-backed slice checks still failed because no Postgres server was listening on 127.0.0.1:5432 in this workspace. The final slice verifier command could not run because ../hyperpush-mono/mesher/frontend-exp/scripts/verify-s03-supported-admin.mjs is missing locally.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test -- --run components/dashboard/live-alerts-settings.test.tsx components/dashboard/live-admin-settings.test.tsx components/dashboard/live-shell.test.tsx src/routes/__root.test.tsx` | 0 | ✅ pass | 5920ms |
| 2 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test -- --run src/lib/mesher/normalize.test.ts src/lib/dashboard/search.test.ts src/routes/index.test.tsx src/routes/__root.test.tsx components/dashboard/live-shell.test.tsx components/dashboard/live-alerts-settings.test.tsx components/dashboard/live-admin-settings.test.tsx` | 0 | ✅ pass | 7968ms |
| 3 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build` | 0 | ✅ pass | 6301ms |
| 4 | `DATABASE_URL=${DATABASE_URL:-postgres://postgres:postgres@127.0.0.1:5432/mesher} bash ../hyperpush-mono/mesher/scripts/migrate.sh up` | 1 | ❌ fail | 578ms |
| 5 | `DATABASE_URL=${DATABASE_URL:-postgres://postgres:postgres@127.0.0.1:5432/mesher} bash ../hyperpush-mono/mesher/scripts/smoke.sh` | 2 | ❌ fail | 277ms |
| 6 | `MESHER_BASE_URL=http://127.0.0.1:18080 node ../hyperpush-mono/mesher/frontend-exp/scripts/verify-s03-supported-admin.mjs` | 1 | ❌ fail | 137ms |

## Deviations

Removed the unsupported AI panel from the route-owned dashboard panel model instead of only deleting the visible button text, so unsupported shell chrome cannot resurface through URL state.

## Known Issues

Local Postgres is unavailable on 127.0.0.1:5432, so migrate.sh up and smoke.sh remain red. The slice verification path ../hyperpush-mono/mesher/frontend-exp/scripts/verify-s03-supported-admin.mjs is also missing locally, so that final admin-flow replay cannot run until the script exists or the verification contract is revised.

## Files Created/Modified

- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings/api-keys-panel.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings/settings-page.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings/project-settings-card.tsx`
- `../hyperpush-mono/mesher/frontend-exp/app/page.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/sidebar.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-detail.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/lib/dashboard/search.ts`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/live-admin-settings.test.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/live-shell.test.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/__root.test.tsx`
