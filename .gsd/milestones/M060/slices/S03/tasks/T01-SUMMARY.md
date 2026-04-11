---
id: T01
parent: S03
milestone: M060
key_files:
  - mesher/client/lib/admin-ops-live-adapter.ts
  - mesher/client/components/dashboard/alerts-live-state.tsx
  - mesher/client/components/dashboard/alerts-page.tsx
  - mesher/client/components/dashboard/alert-detail.tsx
  - mesher/client/components/dashboard/alert-stats.tsx
  - mesher/client/components/dashboard/alert-list.tsx
  - mesher/client/lib/mock-data.ts
  - mesher/client/tests/e2e/admin-ops-live.spec.ts
  - .gsd/KNOWLEDGE.md
key_decisions:
  - (none)
duration: 
verification_result: passed
completed_at: 2026-04-11T23:26:03.572Z
blocker_discovered: false
---

# T01: Wired the Alerts route to same-origin live Mesher reads/actions and added admin/ops live alert proof coverage.

**Wired the Alerts route to same-origin live Mesher reads/actions and added admin/ops live alert proof coverage.**

## What Happened

I added a focused admin/ops adapter at `mesher/client/lib/admin-ops-live-adapter.ts` to normalize Mesher alert payloads into the existing Alerts shell while keeping unsupported fields visibly fallback-backed instead of pretending they came from the backend. I then added `mesher/client/components/dashboard/alerts-live-state.tsx` as the centralized owner for alerts bootstrap, selected-alert state, same-origin acknowledge/resolve actions, read-after-write refresh, destructive toast failures, and explicit `data-*` diagnostics.

With that seam in place, I rewired `alerts-page.tsx`, `alert-stats.tsx`, `alert-list.tsx`, and `alert-detail.tsx` to consume live state instead of `MOCK_ALERTS`, surface live/fallback source markers, expose truthful live status vocabulary (`firing`, `acknowledged`, `resolved`), and keep unsupported silence/unsnooze affordances visible only as shell-only disabled chrome. The detail panel now only exposes real backend actions when the selected alert is truly live, and failed mutations keep the selected alert mounted with destructive toast/error diagnostics rather than optimistic local patching.

I also updated the alert shell type contract in `mesher/client/lib/mock-data.ts` so the frontend can represent the backend’s acknowledged lifecycle cleanly without losing the existing fallback shell. Finally, I created `mesher/client/tests/e2e/admin-ops-live.spec.ts`, which seeds real alerts through the same-origin API by creating a live `new_issue` alert rule and ingesting a real event, then proves the happy path plus bootstrap failure, mutation failure, malformed payload, and empty-live-list cases in both dev and prod harnesses. During the first dev run I found that `DashboardShell` still mounts the Issues provider on `/alerts`, which produces expected aborted issue-overview requests; I tightened the Alerts-specific request filter in the proof and recorded that gotcha in `.gsd/KNOWLEDGE.md`.

## Verification

Verified the live Alerts seam with a production build plus targeted Playwright proofs. `npm --prefix mesher/client run build` passed. `npm --prefix mesher/client run test:e2e:dev -- --grep "admin and ops live alerts"` passed with five Alerts-focused cases: real acknowledge/resolve lifecycle, bootstrap failure fallback state, destructive-toast mutation failure, malformed payload contract failure, and truthful empty-live-list handling. `npm --prefix mesher/client run test:e2e:prod -- --grep "admin and ops live alerts"` also passed with the same five cases. 

Slice-level advancement this task provides: `mesher/client/tests/e2e/admin-ops-live.spec.ts` now exists and contains real Alerts assertions, and both targeted dev/prod Alerts proofs are green. The broader slice verification rail is still partial at T01 because `mesher/scripts/seed-live-admin-ops.sh` does not exist yet and the later combined `--grep "admin and ops live"` coverage for settings/team/API keys belongs to subsequent slice tasks.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix mesher/client run build` | 0 | ✅ pass | 5117ms |
| 2 | `npm --prefix mesher/client run test:e2e:dev -- --grep "admin and ops live alerts"` | 0 | ✅ pass | 22700ms |
| 3 | `npm --prefix mesher/client run test:e2e:prod -- --grep "admin and ops live alerts"` | 0 | ✅ pass | 21900ms |

## Deviations

Used APIRequestContext-based live alert seeding inside `mesher/client/tests/e2e/admin-ops-live.spec.ts` instead of creating a standalone `mesher/scripts/seed-live-admin-ops.sh` helper in T01. The task contract required the first real admin/ops proof file now, while the slice-wide seed helper is scheduled for later work in this slice.

## Known Issues

`DashboardShell` still mounts `DashboardIssuesStateProvider` globally, so opening `/alerts` can emit expected aborted same-origin requests for the hidden Issues bootstrap (`/api/v1/projects/default/issues` and dashboard overview endpoints). The new Alerts proof filters that noise, but future cleanup could scope the Issues provider to the Issues route. Also, the slice-wide helper `mesher/scripts/seed-live-admin-ops.sh` is still absent; later slice tasks must add it before the full combined admin/ops verification command can pass.

## Files Created/Modified

- `mesher/client/lib/admin-ops-live-adapter.ts`
- `mesher/client/components/dashboard/alerts-live-state.tsx`
- `mesher/client/components/dashboard/alerts-page.tsx`
- `mesher/client/components/dashboard/alert-detail.tsx`
- `mesher/client/components/dashboard/alert-stats.tsx`
- `mesher/client/components/dashboard/alert-list.tsx`
- `mesher/client/lib/mock-data.ts`
- `mesher/client/tests/e2e/admin-ops-live.spec.ts`
- `.gsd/KNOWLEDGE.md`
