---
estimated_steps: 4
estimated_files: 6
skills_used:
  - react-best-practices
  - vite
  - test
---

# T01: Extract a shared dashboard shell and add Playwright route-parity scaffolding

**Slice:** S02 — Route-backed dashboard parity
**Milestone:** M059

## Description

Close the biggest parity risk first: move the mounted dashboard chrome out of the temporary `app/page.tsx` monolith and into an active-runtime shell module, then add a real browser test harness so later route work is measured instead of guessed.

This task should keep `/` rendering the current Issues experience while introducing the route map/title helper, shared shell ownership, and a repo-owned Playwright dev harness. Stay client-only and mock-data-only; do not add loaders, server functions, or URL search-param state.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-shell.tsx` shared chrome extraction | Fail back to the current visible shell behavior; do not partially extract state in a way that remounts the sidebar/header or drops the AI panel close path. | N/A | Treat an unknown route key as `/` / Issues inside the route map instead of crashing the shell. |
| `../hyperpush-mono/mesher/frontend-exp/playwright.config.ts` dev webServer contract | Fail the task on dev boot errors; do not let the spec silently point at a stale manually started server. | Stop with an explicit server-readiness failure instead of skipping assertions. | Reject invalid base URL or project config rather than running the spec against the wrong surface. |
| `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts` initial assertions | Keep the assertions focused on the current `/` shell and explicit UI state so failures localize to shell extraction instead of future route work. | Bound waits to visible dashboard landmarks. | Treat missing selectors/text as task failures, not optional checks. |

## Load Profile

- **Shared resources**: one dev server, one browser session, and the mounted dashboard shell state (sidebar collapse plus AI panel).
- **Per-operation cost**: one client boot plus a small number of UI interactions on mock data.
- **10x breakpoint**: accidental shell remounts or unbounded wait-for conditions will break the harness before raw browser load matters.

## Negative Tests

- **Malformed inputs**: unknown route keys in the shell route map, repeated AI toggle clicks, and small-screen auto-collapse state at first render.
- **Error paths**: dev server fails to boot, extracted shell drops the current header or sidebar, or console errors appear on `/`.
- **Boundary conditions**: root `/` still shows Issues by default, sidebar collapse persists through AI panel open/close, and settings-specific chrome is not widened into the shared header yet.

## Steps

1. Extract a reusable dashboard shell module plus a small route-map/title helper out of `app/page.tsx`, keeping the current client-only shell behavior and leaving `/` on the Issues view for now.
2. Add Playwright to `../hyperpush-mono/mesher/frontend-exp/`, wire a dev-only `webServer` config, and add package scripts so route parity checks can run from the repo without manual server babysitting.
3. Write the first `dashboard-route-parity.spec.ts` assertions around the current `/` shell: visible Issues landmarks, sidebar collapse toggle, AI panel open/close, and zero console/request noise.
4. Point `src/routes/index.tsx` at the extracted shell module so the active runtime stops depending on the temporary monolithic `app/page.tsx` implementation details.

## Must-Haves

- [ ] The mounted dashboard chrome moves into a reusable shell module under the active TanStack runtime path.
- [ ] A repo-owned Playwright config and spec exist with real assertions against the current `/` shell.
- [ ] The task keeps the slice on the client/mock-data path; no loaders, server functions, or backend calls are introduced.
- [ ] `src/routes/index.tsx` renders through the extracted shell rather than importing behavior directly from `app/page.tsx`.

## Verification

- `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build`
- `npm --prefix ../hyperpush-mono/mesher/frontend-exp exec playwright test tests/e2e/dashboard-route-parity.spec.ts --project=dev --grep "issues shell"`

## Observability Impact

- Signals added/changed: Playwright root-shell assertions plus captured console/network failures.
- How a future agent inspects this: run the focused dev Playwright command and inspect `playwright.config.ts`, the spec output, and the extracted shell module.
- Failure state exposed: missing root landmarks, shell remount drift, console errors, and failed requests become explicit test failures.

## Inputs

- `.gsd/milestones/M059/slices/S01/S01-SUMMARY.md` — preserved `dev` / `build` / `start` contract and shell-parity gotchas from the groundwork slice.
- `../hyperpush-mono/mesher/frontend-exp/app/page.tsx` — current monolithic shell and local nav state source.
- `../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx` — current temporary TanStack adapter route.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/sidebar.tsx` — current sidebar/nav model and footer settings affordance.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/header.tsx` — shared header and Issues filter bar.
- `../hyperpush-mono/mesher/frontend-exp/package.json` — current command contract that the test harness must respect.

## Expected Output

- `../hyperpush-mono/mesher/frontend-exp/package.json` — adds repo-owned browser-test scripts/dependency hooks without changing the external command contract.
- `../hyperpush-mono/mesher/frontend-exp/playwright.config.ts` — dev parity test configuration with managed webServer startup.
- `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts` — first real route-parity assertions for the current `/` shell.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-shell.tsx` — extracted shared dashboard chrome under the active runtime path.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-route-map.ts` — canonical top-level route/title mapping for later file-route work.
- `../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx` — temporary `/` route now renders through the extracted shell.
