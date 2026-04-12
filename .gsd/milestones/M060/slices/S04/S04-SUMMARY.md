---
id: S04
parent: M060
milestone: M060
provides:
  - A single authoritative full-shell verification rail that future Mesher client work can rerun without rebuilding route lists by hand.
  - A shared runtime-diagnostics helper for same-origin route-level and assembled-shell Playwright suites.
  - A documented maintainer runbook for reproducing seeded dev/prod shell proof locally.
requires:
  - slice: S01
    provides: Same-origin Issues live-read seam, shell source/state markers, and toast-based read-failure surfacing.
  - slice: S02
    provides: Live Issues summary and supported issue-action orchestration through `DashboardIssuesStateProvider`.
  - slice: S03
    provides: Live Alerts, Settings/storage, Team, API key, and alert-rule routes plus deterministic admin/ops seed state.
affects:
  []
key_files:
  - mesher/client/tests/e2e/live-runtime-helpers.ts
  - mesher/client/tests/e2e/seeded-walkthrough.spec.ts
  - mesher/client/components/dashboard/issue-detail.tsx
  - mesher/client/playwright.config.ts
  - mesher/client/README.md
key_decisions:
  - D520 — Close M060 with one route-map-driven seeded full-shell walkthrough plus narrow proof-seam fixes only.
  - D521 — Run Mesher client Playwright E2E rails with workers=1 by default because the live suites share one seeded runtime.
  - D522 — Centralize same-origin runtime diagnostics and route-parity assertions in `tests/e2e/live-runtime-helpers.ts` and derive coverage from `dashboard-route-map.ts`.
patterns_established:
  - Use `dashboard-route-map.ts` as the canonical route inventory for both shell navigation and E2E parity assertions.
  - Keep same-origin API tracking, direct-backend rejection, and known abort filtering in one shared Playwright helper so route-level and assembled-shell suites use the same truth surface.
  - Expose explicit `data-*` state/source markers on mixed live/fallback shell regions so sparse payloads can be asserted truthfully without redesigning the UI.
  - Treat known abort noise as a narrow allowlist; fail closed on any unexpected 4xx/5xx, console error, or direct-backend browser call.
observability_surfaces:
  - `dashboard-shell[data-route-key]` across direct entry and in-app navigation.
  - `issues-shell`, `alerts-shell`, and `settings-shell` `data-state` / `data-source` markers.
  - Issue detail tab markers for stack and breadcrumbs sparse/fallback state.
  - Playwright-tracked same-origin API paths/calls, failed requests, and direct-backend request detection in `tests/e2e/live-runtime-helpers.ts`.
  - The canonical seed + dev/prod verification commands documented in `mesher/client/README.md`.
drill_down_paths:
  - .gsd/milestones/M060/slices/S04/tasks/T01-SUMMARY.md
  - .gsd/milestones/M060/slices/S04/tasks/T02-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-12T01:49:34.602Z
blocker_discovered: false
---

# S04: Full backend-backed shell assembly

**Added a route-map-driven seeded walkthrough and stabilized the Mesher client Playwright harness so the full dashboard shell now passes the canonical live dev/prod backend-backed verification rail.**

## What Happened

S04 closed the final assembly gap in M060 by turning the existing per-route live seams into one authoritative full-shell proof. The slice introduced a shared `mesher/client/tests/e2e/live-runtime-helpers.ts` layer for same-origin `/api/v1` request tracking, direct-backend rejection, known abort filtering, and canonical dashboard route assertions sourced from `mesher/client/components/dashboard/dashboard-route-map.ts`. On top of that helper, `mesher/client/tests/e2e/seeded-walkthrough.spec.ts` now walks one seeded browser session across every current dashboard route, proving live Issues reads and supported issue actions, live Alerts acknowledge/resolve behavior, live Settings/admin operations, and the continued visibility and shell stability of mock-only routes.

During closeout verification, the assembled rail exposed only narrow proof-seam blockers rather than product-scope gaps. The fixes stayed at those seams: `mesher/client/components/dashboard/issue-detail.tsx` now exposes explicit stack and breadcrumb state markers so sparse live detail can be asserted truthfully, `live-runtime-helpers.ts` filters only the already-known hidden-Issues abort noise plus same-origin aborted Geist asset fetches that appear in the built prod app, and `mesher/client/playwright.config.ts` now runs the Mesher client E2E suite serially by default because the live suites mutate one seeded backend/runtime. `mesher/client/README.md` was updated to document the canonical seed + dev/prod verification rail and point maintainers at the walkthrough plus shared helper as the authoritative assembly proof.

The slice delivered the milestone goal without redesign drift. Live-backed routes stayed truthful inside the assembled shell, mock-only routes remained visibly present, browser traffic stayed on same-origin `/api/v1`, and the only changes beyond the new walkthrough itself were the minimal selector, diagnostics, and harness fixes required to make the existing backend-backed flows provably stable in one seeded local environment.

## Verification

Passed the slice-plan verification rail against the seeded local Mesher environment.

Commands run:
- `bash mesher/scripts/seed-live-issue.sh && bash mesher/scripts/seed-live-admin-ops.sh`
- `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live|admin and ops live|seeded walkthrough"` → 21/21 passing
- `bash mesher/scripts/seed-live-issue.sh && bash mesher/scripts/seed-live-admin-ops.sh && npm --prefix mesher/client run test:e2e:prod -- --grep "issues live|admin and ops live|seeded walkthrough"` → 21/21 passing

What was verified:
- Route-map-driven direct-entry parity across every current dashboard route plus unknown-path fallback to Issues.
- Same-origin live Issues reads, selected-detail hydration, sparse-detail shell continuity, and supported issue actions.
- Same-origin live Alerts reads and acknowledge/resolve writes.
- Same-origin live Settings/storage, Team, API key, and alert-rule reads/writes.
- Mock-only routes and unsupported controls remain reachable, visibly present, and shell-stable.
- Runtime diagnostics stay clean apart from explicitly filtered known abort noise; no unexpected direct-backend browser calls were observed.
- Observability surfaces remained truthful through `dashboard-shell[data-route-key]`, `issues-shell`, `alerts-shell`, `settings-shell`, issue-detail tab state markers, subsection `data-state`/`data-source` markers, and Playwright-tracked same-origin API paths/calls.

### Operational Readiness (Q8)
- **Health signal:** Passing seeded dev/prod Playwright rails; `dashboard-shell[data-route-key]` and route-local `data-state`/`data-source` markers show ready/live state after navigation and writes.
- **Failure signal:** A failing route step, destructive toast, unexpected 4xx/5xx or request failure in the runtime tracker, or any browser request that bypasses same-origin `/api/v1` and hits the backend directly.
- **Recovery procedure:** Re-run `bash mesher/scripts/seed-live-issue.sh` and `bash mesher/scripts/seed-live-admin-ops.sh`, then rerun the canonical dev/prod grep commands from `mesher/client/README.md`; if only route parity fails in prod, inspect `tests/e2e/live-runtime-helpers.ts` request filtering before changing product code.
- **Monitoring gaps:** The proof still depends on Playwright/runtime diagnostics rather than a dedicated app-level health endpoint for the assembled client shell, and the live suites intentionally run serially because they share one seeded runtime.

## Requirements Advanced

None.

## Requirements Validated

- R153 — Passing seeded dev/prod `issues live|admin and ops live|seeded walkthrough` rails prove every existing backend-backed dashboard surface now uses the real Mesher backend inside the assembled shell.
- R159 — The slice only repaired proof seams exposed by the walkthrough — sparse-detail markers, narrow runtime-helper abort filtering, and serial Playwright execution — without introducing new backend routes or redesigning the shell.
- R160 — `mesher/client/tests/e2e/seeded-walkthrough.spec.ts` plus the passing seeded dev/prod rails prove the full backend-backed shell walkthrough across every current dashboard route in a seeded local environment.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

Needed one additional runtime-helper fix during closeout: prod route parity surfaced aborted hashed Geist font fetches from the built app as false failed-request regressions, so the helper now filters those same-origin `/assets/geist-*` and `/assets/geist-mono-*` aborts alongside the already-known hidden-Issues noise. No product behavior or backend surface was widened.

## Known Limitations

The canonical live E2E rail intentionally runs with Playwright `workers=1` because the live Issues, Alerts, Settings, and walkthrough suites mutate one seeded local Mesher runtime. The globally mounted Issues provider still produces known hidden bootstrap abort noise during non-Issues routes; the shared runtime helper filters only that explicit class of abort plus built-font abort noise.

## Follow-ups

No additional M060 functional slice work remains. The next step is milestone validation/completion; any future refactor of the globally mounted Issues provider or richer client-shell health surfacing is optional follow-on work outside this milestone.

## Files Created/Modified

- `mesher/client/tests/e2e/live-runtime-helpers.ts` — Centralized same-origin runtime diagnostics, route inventory helpers, direct-backend detection, and narrow known-abort filtering used by route-level and full-shell suites.
- `mesher/client/tests/e2e/seeded-walkthrough.spec.ts` — Added the route-map-driven seeded full-shell walkthrough and direct-entry parity proof across every current dashboard route.
- `mesher/client/components/dashboard/issue-detail.tsx` — Added explicit sparse stack/breadcrumb state markers so issue-detail continuity can be asserted truthfully under mixed live/fallback payloads.
- `mesher/client/playwright.config.ts` — Set Mesher client Playwright suites to run serially by default so one seeded runtime remains deterministic across live route suites.
- `mesher/client/README.md` — Documented the canonical seed + dev/prod full-shell verification rail and the authoritative walkthrough/helper files.
- `.gsd/KNOWLEDGE.md` — Recorded the single-worker shared-runtime constraint and the prod hashed-font abort filtering gotcha for future agents.
