---
estimated_steps: 4
estimated_files: 4
skills_used:
  - react-best-practices
  - tanstack-router-best-practices
  - playwright-best-practices
---

# T01: Add same-origin issue mutation orchestration and the live action-proof harness

**Slice:** S02 — Core maintainer loop live
**Milestone:** M060

## Description

Close the highest-risk seam first: live issue writes must use the same transport and error boundary that S01 already proved, and they cannot leave cached list/detail state stale. This task creates the provider-owned mutation/refetch path and seeds the first real action-proof file so later UI work has a truthful contract to target.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `POST /api/v1/issues/:id/{resolve,archive,unresolve}` | Keep the current issue selected, surface a destructive toast, and do not silently patch local status. | Clear the pending state, expose the timeout as a mutation failure, and preserve operator context. | Treat malformed success/error bodies as contract failures and stop the mutation path rather than guessing state. |
| Follow-up overview/detail reads after mutation | Keep explicit refresh failure state visible, invalidate stale snapshots, and avoid mixing a mutated list with stale detail. | Abort the refresh, expose timeout state, and leave the issue shell inspectable. | Reject malformed payloads through the existing typed parser and error path. |

## Load Profile

- **Shared resources**: same-origin issue mutation routes, the overview/detail refetch budget, selected-issue snapshot cache, and the mounted toast queue.
- **Per-operation cost**: one POST mutation plus one overview refetch and optional selected-issue detail rehydrate.
- **10x breakpoint**: rapid repeated issue actions or filter changes would exhaust refetch churn before UI rendering does, so stale requests must be cancelled and pending state must stay centralized.

## Negative Tests

- **Malformed inputs**: unknown issue id, unsupported action name, or malformed mutation body/response.
- **Error paths**: 500/404 mutation response, timeout during post-mutation refresh, and refresh read failure after a nominal `{"status":"ok"}` write.
- **Boundary conditions**: resolving an already-resolved issue, archiving an issue under an `open` filter, and re-opening an issue that is currently filtered out.

## Steps

1. Extend `mesher/client/lib/mesher-api.ts` with a shared POST helper and typed `resolve`, `unresolve`, and `archive` issue mutation helpers that preserve the S01 same-origin timeout and error semantics.
2. Refactor `mesher/client/components/dashboard/dashboard-issues-state.tsx` to expose reusable `refreshOverview()` and selected-detail rehydrate paths, per-action pending/error state, and selected snapshot invalidation after successful mutations.
3. Publish provider-owned action diagnostics in `mesher/client/components/dashboard/issues-page.tsx` so Playwright can inspect mutation phase, last action, and error state without React internals.
4. Create `mesher/client/tests/e2e/issues-live-actions.spec.ts` with seeded helper/runtime tracking and the first assertions for same-origin mutation requests, refresh-driven status transitions, and destructive mutation-failure toasts.

## Must-Haves

- [ ] `mesher/client/lib/mesher-api.ts` owns the supported issue POST mutations; no component issues ad hoc `fetch()` calls.
- [ ] `DashboardIssuesStateProvider` owns mutation pending/error state, overview refresh, and selected snapshot invalidation.
- [ ] `issues-page.tsx` exposes stable action-state diagnostics for browser verification.
- [ ] `mesher/client/tests/e2e/issues-live-actions.spec.ts` exists in this task and asserts on real action behavior instead of placeholder text.

## Verification

- `bash mesher/scripts/seed-live-issue.sh`
- `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live actions"`

## Observability Impact

- Signals added/changed: provider-owned mutation phase/action/error state plus same-origin action request tracking in the new Playwright file.
- How a future agent inspects this: read `issues-shell` / detail-panel `data-*` attributes and replay `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live actions"`.
- Failure state exposed: mutation timeouts, refresh failures after successful writes, and same-origin routing regressions.

## Inputs

- `mesher/client/lib/mesher-api.ts` — existing typed same-origin read client from S01 that must absorb the supported write helpers.
- `mesher/client/components/dashboard/dashboard-issues-state.tsx` — current provider-owned overview/detail state seam that must stay authoritative.
- `mesher/client/components/dashboard/issues-page.tsx` — current shell diagnostics surface for provider state.
- `mesher/ingestion/routes.mpl` — backend issue mutation contract and thin `{"status":"ok"}` success bodies.
- `mesher/client/tests/e2e/issues-live-read.spec.ts` — proven request/toast/runtime-signal harness pattern to reuse.

## Expected Output

- `mesher/client/lib/mesher-api.ts` — supported issue mutation helpers and shared POST plumbing.
- `mesher/client/components/dashboard/dashboard-issues-state.tsx` — provider-owned mutation orchestration, refresh, and snapshot invalidation.
- `mesher/client/components/dashboard/issues-page.tsx` — stable `data-*` action diagnostics for browser proof.
- `mesher/client/tests/e2e/issues-live-actions.spec.ts` — first live action verification file.
