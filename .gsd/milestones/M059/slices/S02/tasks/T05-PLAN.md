---
estimated_steps: 4
estimated_files: 4
skills_used:
  - vite
  - test
  - agent-browser
---

# T05: Prove dev and built-production deep-link parity on the existing command contract

**Slice:** S02 — Route-backed dashboard parity
**Milestone:** M059

## Description

Prove the slice on the actual command contract, not just on dev hot reload. This task extends the parity harness to built production and closes the loop on direct-entry deep links, using the existing `build` / `start` surfaces from S01 instead of inventing a new runner.

Use the same parity spec file across dev and prod projects so failures stay comparable. The production project should boot from `npm run start` on the built output and direct-load at least one non-root route first, while still asserting zero console errors and zero failed requests on the mock-data path.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `../hyperpush-mono/mesher/frontend-exp/playwright.config.ts` prod project/webServer | Fail on build/start boot errors instead of silently falling back to dev. | Stop with an explicit prod readiness failure rather than masking a deep-link contract break. | Reject misconfigured base URLs/ports so the prod project cannot target the wrong server. |
| `../hyperpush-mono/mesher/frontend-exp/server.mjs` existing production bridge | Surface deep-link failures truthfully; only change the bridge if the real built route contract proves broken. | Bound startup waits and keep the failing route URL visible in the test output. | Treat malformed responses or wrong content-type as production failures, not flaky retries. |
| `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts` final parity coverage | Keep the same assertions across dev and prod so a regression cannot hide behind environment-specific gaps. | Bound waits to route landmarks and explicit network-idle points. | Fail on console/request noise rather than suppressing diagnostics. |

## Load Profile

- **Shared resources**: one build step, one prod server, one dev server project, and the shared browser test suite.
- **Per-operation cost**: full app build plus repeated direct-entry navigations across the eight route URLs in two environments.
- **10x breakpoint**: startup/readiness drift and route-bridge failures will break first; the browser assertions are comparatively cheap.

## Negative Tests

- **Malformed inputs**: first-load non-root URLs like `/releases` or `/settings`, unknown paths, and repeated env-specific test runs.
- **Error paths**: prod bridge serves root only, built routes boot with console errors, or a route fails only after build.
- **Boundary conditions**: dev and prod both pass the same route assertions, and non-root first load works before any in-app navigation occurs.

## Steps

1. Extend `playwright.config.ts` and `package.json` so the same spec can run against both a dev server and the built `npm run start` bridge.
2. Finish the parity spec with explicit direct-entry coverage for every current route plus at least one non-root-first production smoke path.
3. Run the full build + dev + prod parity commands, fix any truthful deep-link or route-environment drift surfaced by the suite, and keep the slice scoped to frontend/mock-data behavior.
4. Capture any route-contract gotcha discovered during closure in the relevant repo-local verification surface instead of leaving it implicit.

## Must-Haves

- [ ] The same repo-owned parity spec runs against both dev and built production.
- [ ] Production direct-entry on a non-root route proves the existing `start` bridge still serves the new route tree truthfully.
- [ ] Final assertions cover visible route landmarks, URL-backed nav, zero console errors, and zero failed requests.
- [ ] Any fix made during closure stays within the existing frontend/mock-data/runtime contract.

## Verification

- `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build`
- `npm --prefix ../hyperpush-mono/mesher/frontend-exp exec playwright test tests/e2e/dashboard-route-parity.spec.ts --project=dev`
- `npm --prefix ../hyperpush-mono/mesher/frontend-exp exec playwright test tests/e2e/dashboard-route-parity.spec.ts --project=prod`

## Observability Impact

- Signals added/changed: shared dev/prod route-parity suite with console/request diagnostics and explicit deep-link readiness failures.
- How a future agent inspects this: run the dev/prod Playwright projects and, if prod fails, inspect `playwright.config.ts` plus `server.mjs`.
- Failure state exposed: route-specific prod boot failures, console noise, failed requests, and direct-entry regressions.

## Inputs

- `../hyperpush-mono/mesher/frontend-exp/playwright.config.ts` — dev parity harness from T01/T04.
- `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts` — route parity suite to extend to production.
- `../hyperpush-mono/mesher/frontend-exp/package.json` — existing external command contract.
- `../hyperpush-mono/mesher/frontend-exp/server.mjs` — S01 production bridge that must keep direct-entry deep links truthful.
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.tsx` — shared layout route now served through both dev and prod.
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.settings.tsx` — non-root direct-entry proof surface for the built app.

## Expected Output

- `../hyperpush-mono/mesher/frontend-exp/playwright.config.ts` — dev/prod projects for the shared parity suite.
- `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts` — final deep-link parity assertions across both environments.
- `../hyperpush-mono/mesher/frontend-exp/package.json` — stable scripts/hooks for running the parity suite on the existing command contract.
- `../hyperpush-mono/mesher/frontend-exp/server.mjs` — only if needed, a narrowly-scoped bridge fix to keep built deep links truthful.
