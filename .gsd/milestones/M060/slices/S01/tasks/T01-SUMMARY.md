---
id: T01
parent: S01
milestone: M060
key_files:
  - mesher/client/mesher-backend-origin.mjs
  - mesher/client/vite.config.ts
  - mesher/client/server.mjs
  - mesher/client/playwright.config.ts
  - mesher/client/lib/mesher-api.ts
  - mesher/client/lib/issues-live-adapter.ts
  - mesher/client/components/dashboard/dashboard-issues-state.tsx
  - mesher/client/components/dashboard/stats-bar.tsx
  - mesher/client/components/dashboard/events-chart.tsx
  - mesher/client/components/dashboard/issues-page.tsx
  - mesher/client/tests/e2e/issues-live-read.spec.ts
key_decisions:
  - D512 — share `MESHER_BACKEND_ORIGIN` resolution across Vite, the prod server bridge, and Playwright so same-origin `/api/v1` behavior cannot drift by runtime.
  - Expose bootstrap status/source through visible overview copy plus `issues-shell`/component data attributes so backend read failures are inspectable without silent fallback.
duration: 
verification_result: mixed
completed_at: 2026-04-11T20:34:40.481Z
blocker_discovered: false
---

# T01: Added a shared same-origin Mesher read seam with live/mock overview bootstrapping and live Playwright proof for the Issues dashboard.

**Added a shared same-origin Mesher read seam with live/mock overview bootstrapping and live Playwright proof for the Issues dashboard.**

## What Happened

I added a shared `MESHER_BACKEND_ORIGIN` resolver and wired it into the Vite dev proxy, the built-prod Node bridge, and the Playwright harness so browser traffic stays on same-origin `/api/v1` in every runtime. I then built `mesher-api.ts` for bounded, typed default-project reads and `issues-live-adapter.ts` for explicit Mesher-to-shell normalization: backend `level` and `status` values are mapped into the richer dashboard severity/status contract, live issue rows overlay onto deterministic mock templates to keep unsupported shell fields visually intact, and overview stats/chart data fall back explicitly instead of silently leaking backend payload shape into the UI. Finally, I updated `DashboardIssuesStateProvider`, `StatsBar`, `EventsChart`, and `IssuesPage` so overview data boots from live Mesher reads while search/filter/selection state stays provider-owned, and I added `issues-live-read.spec.ts` with live, fallback, and malformed-payload coverage in both dev and prod runtimes.

## Verification

Verified the client production build, the new live Playwright suite in dev and prod, and a direct browser sanity check against the local app. The live proof asserted same-origin `/api/v1` traffic, seeded-context boot, visible overview surfaces, explicit fallback state on induced backend failure, and normalization of sparse/unknown payloads without crashes. I also confirmed in-browser shell attributes showed `data-bootstrap-state=ready`, `data-overview-source=mixed`, and a live issue count from the current Mesher backend. `mesher/scripts/smoke.sh` was not rerun from this task shell because `DATABASE_URL` was not exported there, but the live E2E/browser checks exercised the existing Mesher backend already listening on `:18080`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live read seam"` | 0 | ✅ pass | 14000ms |
| 2 | `npm --prefix mesher/client run test:e2e:prod -- --grep "issues live read seam"` | 0 | ✅ pass | 11000ms |
| 3 | `npm --prefix mesher/client run build — ✅ pass (Vite client + SSR production build completed successfully before the final E2E reruns).` | -1 | unknown (coerced from string) | 0ms |
| 4 | `Browser sanity check against http://127.0.0.1:3000 — ✅ pass (`issues-shell` reported `data-bootstrap-state=ready`, `data-overview-source=mixed`, and `data-live-issue-count=5`; console output only contained normal Vite/React DevTools logs).` | -1 | unknown (coerced from string) | 0ms |

## Deviations

I also updated `mesher/client/playwright.config.ts` and added `mesher/client/mesher-backend-origin.mjs` so the verification harness starts or reuses the same Mesher backend origin the app proxies to. That extra wiring was necessary to keep dev, built-prod, and automated verification on one truthful backend target.

## Known Issues

`bash mesher/scripts/smoke.sh` could not be rerun directly from this task shell because `DATABASE_URL` was unset in the shell environment. Live verification still passed against the existing Mesher backend already bound to `127.0.0.1:18080`.

## Files Created/Modified

- `mesher/client/mesher-backend-origin.mjs`
- `mesher/client/vite.config.ts`
- `mesher/client/server.mjs`
- `mesher/client/playwright.config.ts`
- `mesher/client/lib/mesher-api.ts`
- `mesher/client/lib/issues-live-adapter.ts`
- `mesher/client/components/dashboard/dashboard-issues-state.tsx`
- `mesher/client/components/dashboard/stats-bar.tsx`
- `mesher/client/components/dashboard/events-chart.tsx`
- `mesher/client/components/dashboard/issues-page.tsx`
- `mesher/client/tests/e2e/issues-live-read.spec.ts`
