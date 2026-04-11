---
estimated_steps: 4
estimated_files: 8
skills_used:
  - react-best-practices
  - tanstack-router-best-practices
  - playwright-best-practices
---

# T01: Add same-origin Mesher transport and live overlay bootstrap for seeded issues reads

**Slice:** S01 — Seeded real context and issues/events live read seam
**Milestone:** M060

## Description

Establish the real read seam first. This task retires the two biggest blockers called out in research: the dashboard has no same-origin `/api/v1` bridge today, and the Mesher API shape is slimmer than the current shell contract. The work here should make the overview portion of the Issues route truthful against the seeded `default` project without changing shell ownership of search, filters, or selection state.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `mesher/client` same-origin `/api/v1` proxy in `vite.config.ts` / `server.mjs` | Stop live boot, keep overview surfaces on explicit fallback data, and expose a typed bootstrap failure instead of silently issuing direct cross-origin fetches. | Surface a bounded bootstrap error state and avoid hanging the dashboard shell indefinitely. | Reject the payload in `mesher-api` / adapter code and keep unsupported shell fields on stable fallback values. |
| `GET /api/v1/projects/default/issues`, `/dashboard/health`, `/dashboard/levels`, `/dashboard/volume` | Mark the live bootstrap as failed, preserve current shell state ownership, and leave the list/chart/stats on explicit fallback data. | Fail the bootstrap path fast enough for Playwright and future agents to localize which read stalled. | Fail normalization with explicit mapping errors instead of rendering backend enums or bucket shapes directly into the UI contract. |

## Load Profile

- **Shared resources**: browser boot-time fetch budget, the local Mesher backend, and the proxy bridge in both dev and built-prod runtimes.
- **Per-operation cost**: one issues read plus three dashboard reads at shell boot, issued in parallel and then normalized into the existing view model.
- **10x breakpoint**: boot-time parallel reads and payload normalization would break before local UI state management, so keep fetch fan-out fixed and avoid N+1 detail requests during overview rendering.

## Negative Tests

- **Malformed inputs**: unknown backend status/level names, empty issue arrays, missing dashboard totals, and sparse volume buckets must map to stable fallback values instead of crashing overview components.
- **Error paths**: proxy target unavailable, `/api/v1/projects/default/*` returns 4xx/5xx, or JSON parsing fails during bootstrap.
- **Boundary conditions**: zero issues in the seeded project, empty dashboard series, and a live list that contains ids not present in the existing mock dataset.

## Steps

1. Add same-origin `/api/v1` proxying to `mesher/client/vite.config.ts` and `mesher/client/server.mjs`, with one backend origin configuration path shared by dev and built-prod verification.
2. Create `mesher/client/lib/mesher-api.ts` for typed default-project reads and `mesher/client/lib/issues-live-adapter.ts` for explicit status/severity normalization plus fallback-overlay rules.
3. Update `mesher/client/components/dashboard/dashboard-issues-state.tsx`, `mesher/client/components/dashboard/stats-bar.tsx`, and `mesher/client/components/dashboard/events-chart.tsx` so the overview shell boots from seeded live data while preserving current filter/selection ownership and current mock-only UI shape.
4. Add the first live Playwright proof in `mesher/client/tests/e2e/issues-live-read.spec.ts` that asserts same-origin `/api/v1` traffic, seeded default-context boot, overview visibility, and clean runtime signals in dev mode.

## Must-Haves

- [ ] No browser code talks directly to `http://127.0.0.1:8080`; live reads go through same-origin `/api/v1` in both dev and built-prod runtimes.
- [ ] The adapter contains explicit status/severity/bucket normalization and explicit fallback rules instead of letting Mesher payloads leak into the richer shell contract.
- [ ] `DashboardIssuesStateProvider` remains the owner of search/filter/selection state while overview data becomes live-backed.
- [ ] `mesher/client/tests/e2e/issues-live-read.spec.ts` exists in this task and can prove seeded-context boot in dev mode.

## Verification

- `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash mesher/scripts/smoke.sh`
- `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live read seam boots seeded context"`

## Observability Impact

- Signals added/changed: same-origin request paths visible in Playwright/network logs plus provider-level loading/error state for seeded live bootstrap.
- How a future agent inspects this: run `bash mesher/scripts/smoke.sh`, inspect browser failed requests for `/api/v1/*`, and use the new live Playwright spec to localize proxy/bootstrap failures.
- Failure state exposed: proxy misrouting, missing seeded project resolution, malformed dashboard payload normalization, and empty-live-list fallback regressions.

## Inputs

- `mesher/client/vite.config.ts` — current dev server config without proxying.
- `mesher/client/server.mjs` — current built-prod bridge that only serves app assets.
- `mesher/client/components/dashboard/dashboard-issues-state.tsx` — current mock-only provider and shell state owner.
- `mesher/client/components/dashboard/stats-bar.tsx` — current mock-only overview cards.
- `mesher/client/components/dashboard/events-chart.tsx` — current mock-only chart.
- `mesher/client/lib/mock-data.ts` — fallback source that must remain in play for unsupported fields.
- `mesher/client/tests/e2e/dashboard-route-parity.spec.ts` — current shell-parity proof that must stay isolated from live assertions.
- `mesher/client/playwright.config.ts` — current dev/prod browser harness.
- `mesher/api/search.mpl` — live issues read contract.
- `mesher/api/dashboard.mpl` — live summary and volume read contracts.
- `mesher/api/helpers.mpl` — seeded project-slug resolution contract.
- `mesher/scripts/smoke.sh` — backend readiness proof surface.

## Expected Output

- `mesher/client/vite.config.ts` — dev same-origin `/api/v1` proxy.
- `mesher/client/server.mjs` — built-prod same-origin `/api/v1` bridge.
- `mesher/client/lib/mesher-api.ts` — typed Mesher read client for seeded default-project calls.
- `mesher/client/lib/issues-live-adapter.ts` — live/mock overlay normalization for overview reads.
- `mesher/client/components/dashboard/dashboard-issues-state.tsx` — seeded live bootstrap while preserving current shell state ownership.
- `mesher/client/components/dashboard/stats-bar.tsx` — overview cards driven by live overlay data.
- `mesher/client/components/dashboard/events-chart.tsx` — event volume chart driven by live overlay data.
- `mesher/client/tests/e2e/issues-live-read.spec.ts` — first live-browser proof for seeded-context overview boot.
