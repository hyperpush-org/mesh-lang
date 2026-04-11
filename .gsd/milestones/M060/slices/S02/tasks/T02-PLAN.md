---
estimated_steps: 4
estimated_files: 4
skills_used:
  - react-best-practices
  - playwright-best-practices
---

# T02: Wire supported maintainer controls and backend-backed Issues summary signals

**Slice:** S02 — Core maintainer loop live
**Milestone:** M060

## Description

With the shared mutation seam in place, make the maintainer loop visible in the existing shell. This task wires the supported action buttons into the current detail action row and tightens the Issues summary chrome so the visible operator signals come from backend-backed data instead of broad mock stats.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `DashboardIssuesStateProvider` action state and selectors | Disable the affected action, keep the previous visible issue state, and surface a destructive toast instead of leaving the row ambiguous. | Return controls to idle and keep the shell interactive. | Refuse to render unsupported status transitions or summary values as live. |
| Existing Issues shell components (`issue-detail`, `issue-list`, `stats-bar`) | Preserve current layout and fallback fields instead of removing sections or inventing new UI chrome. | N/A | Reject unsupported backend state names by mapping them explicitly or deferring the control. |

## Load Profile

- **Shared resources**: the selected issue action row, filtered issue list, provider-derived summary data, and chart/list refresh path.
- **Per-operation cost**: one user-triggered action updates one selected issue, one filtered list view, and the summary cards backed by current provider data.
- **10x breakpoint**: repeated action/refilter combinations will reveal stale-list or stale-summary bugs first, so selectors must be derived from refreshed provider state rather than local component guesses.

## Negative Tests

- **Malformed inputs**: unsupported status mapping, missing selected issue, or a selected issue that disappears from the active filter after mutation.
- **Error paths**: action pending state never clears, detail buttons stay enabled during an in-flight write, or summary cards silently fall back without surfacing mixed/live source.
- **Boundary conditions**: resolve under an `open` filter removes or reclassifies the row, unresolve reintroduces it, and archive maps to the shell's `ignored` status without exposing backend `archived` vocabulary directly.

## Steps

1. Update `mesher/client/components/dashboard/issue-detail.tsx` to add supported maintainer controls in the existing action row with busy and disabled affordances and labels that match the shell vocabulary (`Resolve`, `Reopen`, `Ignore`).
2. Tighten `mesher/client/lib/issues-live-adapter.ts`, `mesher/client/components/dashboard/issue-list.tsx`, and `mesher/client/components/dashboard/stats-bar.tsx` so post-mutation status and summary surfaces derive from refreshed live data while unsupported shell fields remain visibly fallback-backed.
3. Keep unsupported actions (`assign`, `discard`, `delete`) out of the live S02 surface so the shell does not overclaim backend support.
4. Reuse the new action proof rail to assert visible control state, filtered-list transitions, and summary-source markers after supported mutations.

## Must-Haves

- [ ] The issue-detail action row exposes only the supported live actions and shows pending/disabled state during writes.
- [ ] Issue rows, status badges, filters, and summary cards all reflect the refreshed live status after a supported mutation.
- [ ] Broad mock-only summary signals are replaced with backend-backed or explicitly mixed/fallback values inside the existing Issues chrome.
- [ ] Unsupported shell-only affordances remain visibly present where appropriate but do not pretend to be live S02 actions.

## Verification

- `bash mesher/scripts/seed-live-issue.sh`
- `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live actions"`

## Observability Impact

- Signals added/changed: visible action-button busy/disabled state, refreshed issue status badges, and summary-source markers that stay truthful after mutation.
- How a future agent inspects this: inspect the issue-detail action row, list badges, stats-bar source labels, and rerun the action spec under an `open` filter.
- Failure state exposed: stale status badges, silent summary fallback, or controls that stay interactive during an in-flight write.

## Inputs

- `mesher/client/components/dashboard/dashboard-issues-state.tsx` — provider-owned mutation and refresh seam from T01.
- `mesher/client/components/dashboard/issue-detail.tsx` — existing detail action row where live maintainer controls must land.
- `mesher/client/lib/issues-live-adapter.ts` — current status/summary mapping seam that must stay truthful.
- `mesher/client/components/dashboard/issue-list.tsx` — visible list/status/filter surface that must update after live mutations.
- `mesher/client/components/dashboard/stats-bar.tsx` — existing Issues summary chrome that currently still carries broad mock signals.
- `mesher/client/tests/e2e/issues-live-actions.spec.ts` — action proof rail created in T01.

## Expected Output

- `mesher/client/components/dashboard/issue-detail.tsx` — live `Resolve`, `Reopen`, and `Ignore` controls in the current action row.
- `mesher/client/lib/issues-live-adapter.ts` — status and summary derivations aligned with refreshed live payloads.
- `mesher/client/components/dashboard/issue-list.tsx` — list/status/filter rendering that reflects post-mutation live truth.
- `mesher/client/components/dashboard/stats-bar.tsx` — summary chrome driven by backend-backed or explicitly mixed values.
