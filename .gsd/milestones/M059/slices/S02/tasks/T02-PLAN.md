---
estimated_steps: 4
estimated_files: 6
skills_used:
  - react-best-practices
  - test
---

# T02: Extract the Issues route content and keep Issues state layout-owned

**Slice:** S02 — Route-backed dashboard parity
**Milestone:** M059

## Description

Make the Issues screen route-leaf-ready before real file routes land. Right now search, status/severity filters, and selected issue detail live inline in `app/page.tsx`; if that state moves into a per-route leaf later, leaving `/` and coming back will reset behavior that currently persists.

This task should extract the Issues column/panel into its own component and lift the Issues-specific client state into a shell-owned context/store used by the extracted page. The runtime path after this task should render Issues from the extracted module under the shared shell, not from inline JSX in `app/page.tsx`.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `../hyperpush-mono/mesher/frontend-exp/components/dashboard/issues-page.tsx` extraction | Preserve the existing issue list/detail UI shape; do not rewrite the interaction model while extracting it. | N/A | Treat unknown issue ids as “no detail panel” instead of crashing the shell. |
| `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-issues-state.tsx` layout-owned state | Keep filters/search/selected issue in the shared shell scope; do not fall back to route-local state that resets on leave/return. | N/A | Reject invalid filter values by normalizing them to the current `all` defaults. |
| `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts` Issues assertions | Fail on visible parity drift rather than weakening the assertions to fit a regression. | Bound waits to visible issue list/detail landmarks. | Treat missing filter chips/search values as task failures. |

## Load Profile

- **Shared resources**: in-memory mock issue list, layout-owned filter/search/detail state, and one right-side issue panel.
- **Per-operation cost**: client-side filter passes over `MOCK_ISSUES` plus one detail lookup by id.
- **10x breakpoint**: duplicated state between shell and page or unnecessary re-renders will break interaction parity before list size matters.

## Negative Tests

- **Malformed inputs**: empty search, unknown issue id, invalid status/severity values, and repeated detail-panel toggles.
- **Error paths**: extracted page loses the right panel, filters reset unexpectedly, or the active runtime still imports the old monolithic branch.
- **Boundary conditions**: leaving Issues state untouched still renders the current default dashboard, and selected issue close/reopen behavior matches the old shell.

## Steps

1. Extract the Issues content into `issues-page.tsx` using the existing `StatsBar`, `EventsChart`, `FilterBar`, `IssueList`, and `IssueDetail` components instead of reauthoring the UI.
2. Move Issues search/filter/selected-detail state into a shell-owned provider/hook so the later route swap preserves leave-and-return behavior.
3. Update the shared shell and `/` route to render the extracted Issues module and stop depending on inline Issues JSX from `app/page.tsx`.
4. Extend the Playwright spec with explicit Issues interaction assertions: filter/search changes, detail panel toggle, and persistence across internal re-renders.

## Must-Haves

- [ ] Issues becomes a dedicated route-leaf-ready component under `components/dashboard/`.
- [ ] Issues filter/search/detail state lives in shell-owned client state instead of route-local JSX.
- [ ] The active runtime path no longer needs inline Issues JSX from `app/page.tsx`.
- [ ] The Playwright spec proves current Issues interactions still work on the mock-data path.

## Verification

- `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build`
- `npm --prefix ../hyperpush-mono/mesher/frontend-exp exec playwright test tests/e2e/dashboard-route-parity.spec.ts --project=dev --grep "issues interactions"`

## Observability Impact

- Signals added/changed: visible filter/search/detail state becomes explicit in the parity spec.
- How a future agent inspects this: rerun the focused Issues Playwright grep and inspect the extracted state module plus `issues-page.tsx`.
- Failure state exposed: filter reset drift, missing issue panels, and invalid selected-id handling show up as targeted test failures.

## Inputs

- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-shell.tsx` — extracted shared shell from T01.
- `../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx` — current `/` runtime entry that still renders the Issues view.
- `../hyperpush-mono/mesher/frontend-exp/app/page.tsx` — source of the inline Issues branch to retire from the active runtime path.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-list.tsx` — current Issues list UI.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-detail.tsx` — current Issues right-panel UI.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/header.tsx` — existing `FilterBar` used by Issues.
- `../hyperpush-mono/mesher/frontend-exp/lib/mock-data.ts` — current Issues mock-data source.
- `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts` — parity spec to extend with Issues-state assertions.

## Expected Output

- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/issues-page.tsx` — extracted Issues leaf component ready to mount from a real route.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-issues-state.tsx` — shell-owned Issues client state with filter/search/detail ownership.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-shell.tsx` — shared shell updated to render the extracted Issues module.
- `../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx` — `/` route now composes the extracted Issues path.
- `../hyperpush-mono/mesher/frontend-exp/app/page.tsx` — no longer needed on the active runtime path for Issues rendering.
- `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts` — Issues interaction and persistence assertions.
