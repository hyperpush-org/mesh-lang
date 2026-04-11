# S01: Seeded real context and issues/events live read seam

**Goal:** Boot the existing dashboard issues shell against the seeded Mesher `default` context through same-origin `/api/v1` reads, keeping unsupported shell fields visually intact via explicit live/mock overlay and making backend read failures visible through the existing Radix toast pattern.
**Demo:** Against a seeded local Mesher backend, the dashboard boots in a truthful real project-org/API-key context and the Issues route loads live issue, detail, and event data through the existing shell.

## Must-Haves

- Boot the issues route against the seeded Mesher `default` project/org/API-key reality through same-origin `/api/v1` reads without adding a new login or session flow. (`R155`)
- Keep the existing issues shell materially intact while list, stats, chart, selected detail, and timeline surfaces consume live backend truth wherever the Mesher API already provides it, preserving fallback/mock-only fields and UI sections when the backend is slimmer. (`R156`)
- Surface backend-backed read failures through the existing Radix toast stack without replacing the current shell or silently swallowing errors. (`R158`)
- Prove the seeded live seam in both dev and built-prod dashboard runtimes with deterministic issue seeding, explicit request/console assertions, and no direct browser cross-origin calls to `:8080`.

## Threat Surface

- **Abuse**: direct browser calls to `:8080`, arbitrary host proxying, or unvalidated issue/event identifiers could turn a read seam into a transport confusion bug; keep the client on same-origin `/api/v1` only and normalize selected issue ids before building follow-up reads.
- **Data exposure**: `GET /api/v1/events/:event_id` can include tags, breadcrumbs, `user_context`, and `extra`; verification and UI changes must avoid logging or toasting raw sensitive payloads, and the seeded API key belongs only in backend verification helpers.
- **Input trust**: search/filter values, issue ids selected from live list data, dashboard/issue/event JSON, and seeded smoke-event payloads are all untrusted inputs that need explicit mapping/fallback rather than direct rendering into the richer shell model.

## Requirement Impact

- **Requirements touched**: `R155`, `R156`, `R158`
- **Re-verify**: seeded default-context boot without auth UI, live list/stats/chart/detail reads through same-origin `/api/v1`, preserved mock-only UI sections/fallback fields, and visible Radix toasts for induced backend failures in both dev and built-prod runs.
- **Decisions revisited**: `D508`, `D509`, `D510`, `D511`

## Proof Level

- This slice proves: integration
- Real runtime required: yes
- Human/UAT required: no

## Verification

- `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash mesher/scripts/migrate.sh up && DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash mesher/scripts/smoke.sh`
- `bash mesher/scripts/seed-live-issue.sh`
- `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live read seam"`
- `npm --prefix mesher/client run test:e2e:prod -- --grep "issues live read seam"`

## Observability / Diagnostics

- Runtime signals: Playwright console/request trackers, `issues-shell` data attributes, mounted Radix toast output, and Mesher smoke responses.
- Inspection surfaces: `bash mesher/scripts/smoke.sh`, `bash mesher/scripts/seed-live-issue.sh`, `mesher/client/tests/e2e/issues-live-read.spec.ts`, browser network logs, and the visible toaster surface in the root route.
- Failure visibility: proxy misrouting, bootstrap/read errors, selected-detail fetch failures, and induced backend failures must all show either failed-request assertions or visible toast text instead of silent fallback.
- Redaction constraints: never print the seeded API key or dump raw `user_context` / `extra` payloads into logs, toasts, or committed docs.

## Integration Closure

- Upstream surfaces consumed: `mesher/api/search.mpl`, `mesher/api/dashboard.mpl`, `mesher/api/detail.mpl`, `mesher/api/helpers.mpl`, `mesher/scripts/smoke.sh`, the seeded `default` migration context, the existing dashboard shell/components, and `mesher/client/playwright.config.ts`.
- New wiring introduced in this slice: same-origin `/api/v1` proxying in dev/prod, a typed Mesher read client, a live-overlay adapter rooted in `DashboardIssuesStateProvider`, mounted Radix toast feedback, and a dedicated live-backend Playwright proof path.
- What remains before the milestone is truly usable end-to-end: later slices still need broader backend-backed surfaces beyond Issues, but after this slice the Issues route should truthfully boot and read against the seeded default project through the existing shell.

## Tasks

- [x] **T01: Add same-origin Mesher transport and live overlay bootstrap for seeded issues reads** `est:2h`
  - Why: Same-origin transport plus a typed overlay seam is the main blocker; without it every live browser read either fails on transport or forces the existing shell to accept a slimmer backend model directly.
  - Files: `mesher/client/vite.config.ts`, `mesher/client/server.mjs`, `mesher/client/lib/mesher-api.ts`, `mesher/client/lib/issues-live-adapter.ts`, `mesher/client/components/dashboard/dashboard-issues-state.tsx`, `mesher/client/components/dashboard/stats-bar.tsx`, `mesher/client/components/dashboard/events-chart.tsx`, `mesher/client/tests/e2e/issues-live-read.spec.ts`
  - Do: Add same-origin `/api/v1` proxying in dev and built-prod, create a typed Mesher default-project read client plus explicit status/severity/fallback normalization, wire the issues state provider and overview surfaces to seeded live list/stats/chart data while preserving current filter/selection ownership, and add the first live Playwright spec around seeded-context boot.
  - Verify: `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash mesher/scripts/smoke.sh && npm --prefix mesher/client run test:e2e:dev -- --grep "issues live read seam boots seeded context"`
  - Done when: the issues route boots against project `default` through same-origin `/api/v1` reads, list/stats/chart surfaces render live-backed data with explicit fallback rules, and the first live spec passes without failed requests or console errors.
- [x] **T02: Finish live issue detail, toast failure feedback, and seeded end-to-end proof** `est:2h`
  - Why: The slice is only true once selected issue detail/event reads, truthful failure feedback, and dev/prod proof all work through the unchanged shell instead of stopping at the overview cards.
  - Files: `mesher/client/components/dashboard/dashboard-issues-state.tsx`, `mesher/client/components/dashboard/issue-list.tsx`, `mesher/client/components/dashboard/issue-detail.tsx`, `mesher/client/components/dashboard/issues-page.tsx`, `mesher/client/src/routes/__root.tsx`, `mesher/client/tests/e2e/issues-live-read.spec.ts`, `mesher/scripts/seed-live-issue.sh`, `mesher/client/README.md`
  - Do: Extend the provider to read latest event detail and issue timeline for the selected live issue, merge those fields into the existing list/detail shell without removing unsupported sections, mount and use the Radix toaster for backend failures, add deterministic issue seeding plus dev/prod live assertions for detail and failure behavior, and update the client README to document the mixed live/mock seam.
  - Verify: `bash mesher/scripts/seed-live-issue.sh && npm --prefix mesher/client run test:e2e:dev -- --grep "issues live read seam" && npm --prefix mesher/client run test:e2e:prod -- --grep "issues live read seam"`
  - Done when: selecting a live issue shows live-backed detail/event/timeline data, unsupported UI remains present through explicit fallback values, backend read failures produce visible Radix toasts, and the seeded live spec passes in both dev and built-prod modes.

## Files Likely Touched

- `mesher/client/vite.config.ts`
- `mesher/client/server.mjs`
- `mesher/client/lib/mesher-api.ts`
- `mesher/client/lib/issues-live-adapter.ts`
- `mesher/client/components/dashboard/dashboard-issues-state.tsx`
- `mesher/client/components/dashboard/stats-bar.tsx`
- `mesher/client/components/dashboard/events-chart.tsx`
- `mesher/client/tests/e2e/issues-live-read.spec.ts`
- `mesher/client/components/dashboard/issue-list.tsx`
- `mesher/client/components/dashboard/issue-detail.tsx`
- `mesher/client/components/dashboard/issues-page.tsx`
- `mesher/client/src/routes/__root.tsx`
- `mesher/scripts/seed-live-issue.sh`
- `mesher/client/README.md`
