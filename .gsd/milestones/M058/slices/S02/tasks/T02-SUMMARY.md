---
id: T02
parent: S02
milestone: M058
key_files:
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx
  - ../hyperpush-mono/mesher/frontend-exp/app/page.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/sidebar.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-detail.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-event-list.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/index.test.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/live-shell.test.tsx
key_decisions:
  - D476 — restrict the shell sidebar to router-backed `issues`, `alerts`, and `settings` views so top-level navigation comes entirely from validated route state.
duration:
verification_result: mixed
completed_at: 2026-04-11T01:56:10.656Z
blocker_discovered: false
---

# T02: Wired router-owned live issue drilldown with event detail and resolve/unresolve revalidation.

**Wired router-owned live issue drilldown with event detail and resolve/unresolve revalidation.**

## What Happened

Reworked the TanStack Start dashboard shell so the validated dashboard search model now owns the active `view`, selected `issue`, and selected `event`, then updated the route loader to fetch timeline, event summaries, and optional event detail only when the issue panel is active and the selected IDs still exist. Rebuilt the issue detail panel around the reduced Mesher detail contract, added a dedicated issue-event list, and wired resolve/unresolve through the existing mutation server functions with route invalidation instead of optimistic local updates. Reduced the sidebar to the router-backed live views (`issues`, `alerts`, `settings`) so the shell no longer mixes route-visible and local-only top-level navigation. Extended route and component tests to cover router-controlled navigation, stale issue/event cleanup, detail-path error surfacing, honest empty states, and mutation pending/error visibility.

## Verification

Passed the task-plan verification suite with `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test -- --run src/routes/index.test.tsx components/dashboard/live-shell.test.tsx` and confirmed the frontend still builds with `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build`. Also ran the remaining slice-level checks to capture in-progress status: the mock-import guard still reports the expected alert/settings mock surfaces for later tasks, migrate/smoke are currently blocked by no Postgres listener on `127.0.0.1:5432`, and the planner-referenced `verify-s02-live-detail-alerts.mjs` script does not exist locally.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test -- --run src/routes/index.test.tsx components/dashboard/live-shell.test.tsx` | 0 | ✅ pass | 5810ms |
| 2 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build` | 0 | ✅ pass | 3610ms |
| 3 | `rg -n 'MOCK_ALERTS|MOCK_ALERT_STATS|PROJECT_CONFIG|ALERT_RULES|MOCK_TREASURY' ../hyperpush-mono/mesher/frontend-exp/src ../hyperpush-mono/mesher/frontend-exp/app/page.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-detail.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alerts-page.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alert-list.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alert-detail.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/alert-stats.tsx ../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings` | 0 | ❌ fail | 0ms |
| 4 | `DATABASE_URL=${DATABASE_URL:-postgres://postgres:postgres@127.0.0.1:5432/mesher} bash ../hyperpush-mono/mesher/scripts/migrate.sh up` | 1 | ❌ fail | 0ms |
| 5 | `DATABASE_URL=${DATABASE_URL:-postgres://postgres:postgres@127.0.0.1:5432/mesher} bash ../hyperpush-mono/mesher/scripts/smoke.sh` | 2 | ❌ fail | 0ms |
| 6 | `MESHER_BASE_URL=http://127.0.0.1:18080 node ../hyperpush-mono/mesher/frontend-exp/scripts/verify-s02-live-detail-alerts.mjs` | 1 | ❌ fail | 0ms |

## Deviations

Reduced the sidebar to the three router-backed live views instead of preserving the previous mixed local/router navigation model, because the validated search contract only supports `issues`, `alerts`, and `settings`. Also verified that the planned replay command points to a script path that does not exist in the local repo and recorded that mismatch rather than substituting a guessed command.

## Known Issues

Alerts and settings surfaces remain mock-backed in `components/dashboard/alerts-page.tsx`, `components/dashboard/alert-stats.tsx`, and `components/dashboard/settings/settings-page.tsx`, so the slice-level no-mock guard is still red until later S02 tasks land. Local migrate/smoke verification is blocked by the absence of a Postgres server on `127.0.0.1:5432`. The planned `frontend-exp/scripts/verify-s02-live-detail-alerts.mjs` replay script is missing from the repo.

## Files Created/Modified

- `../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx`
- `../hyperpush-mono/mesher/frontend-exp/app/page.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/sidebar.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-detail.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-event-list.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/index.test.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/live-shell.test.tsx`
