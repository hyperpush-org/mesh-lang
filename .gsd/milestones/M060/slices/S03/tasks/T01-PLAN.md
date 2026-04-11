---
estimated_steps: 4
estimated_files: 8
skills_used:
  - react-best-practices
  - playwright-best-practices
---

# T01: Wire the Alerts route to live Mesher reads/actions and seed the admin/ops proof file

**Slice:** S03 — Admin and ops surfaces live
**Milestone:** M060

## Description

Close the cleanest end-user seam first: the standalone Alerts route already has its own page, so it should become the first truthful admin/ops surface. This task extends the shared same-origin client boundary for admin/ops payloads, adds a small alerts-owned live state layer, and creates the first real browser-proof file so later settings/team work lands on an exercised harness instead of new mock assumptions. Keep the visual shell intact, but make backend-backed alert reads and the supported fired-alert lifecycle (`acknowledge`, `resolve`) real.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `GET /api/v1/projects/default/alerts` | Keep the Alerts shell mounted, expose a destructive toast plus explicit failed state, and do not silently revert to `MOCK_ALERTS` as if they were live. | Clear loading state, keep filters usable, and expose timeout diagnostics on the alerts shell. | Reject the payload through typed parsing/adapters and surface the contract failure instead of guessing status/history fields. |
| `POST /api/v1/alerts/:id/acknowledge` and `POST /api/v1/alerts/:id/resolve` | Keep the selected alert visible, show a destructive toast, and leave local status unchanged until a real refresh succeeds. | Return action state to idle, keep the detail panel inspectable, and expose the timeout in UI diagnostics. | Treat malformed mutation responses as contract failures and do not optimistic-patch list/detail state. |

## Load Profile

- **Shared resources**: same-origin alerts list reads, selected-alert detail/action refreshes, and the mounted toast queue.
- **Per-operation cost**: one alerts list read at bootstrap, local filter/sort work in the client, and one POST plus a list/detail refresh per alert action.
- **10x breakpoint**: repeated alert actions with large lists will reveal stale selected-alert snapshots and refresh churn before rendering becomes the bottleneck, so state ownership must stay centralized.

## Negative Tests

- **Malformed inputs**: unknown alert id, unsupported live status, or malformed `condition_snapshot` / timestamp fields.
- **Error paths**: alerts list 500s, selected-alert action 500s, and post-action refresh failures that must show destructive toasts.
- **Boundary conditions**: empty live alerts list, acknowledged alerts that must stay truthful in the shell vocabulary, and resolved alerts that should no longer expose active actions.

## Steps

1. Extend `mesher/client/lib/mesher-api.ts` with typed alert, alert-rule, settings/storage, member, and API-key parsers/helpers needed by this slice, and add a focused `mesher/client/lib/admin-ops-live-adapter.ts` seam for mapping lean Mesher payloads into the richer Alerts shell contract.
2. Add `mesher/client/components/dashboard/alerts-live-state.tsx` to own alerts bootstrap, selected-alert state, same-origin action refreshes, and destructive-toast failures instead of scattering fetches through leaf components.
3. Refactor `mesher/client/components/dashboard/alerts-page.tsx`, `mesher/client/components/dashboard/alert-detail.tsx`, `mesher/client/components/dashboard/alert-stats.tsx`, and `mesher/client/lib/mock-data.ts` so the route uses live rows/stats/status mapping where the backend has truth, preserves unsupported fallback-only fields visibly, and replaces the dishonest `silenced` action semantics with truthful live action copy.
4. Create `mesher/client/tests/e2e/admin-ops-live.spec.ts` with an Alerts-focused first proof that tracks same-origin `/api/v1` requests, asserts visible live/failure state, and leaves room for later settings/team coverage in the same file.

## Must-Haves

- [ ] The Alerts route reads real fired alerts through the shared same-origin Mesher client; no component issues ad hoc `fetch()` calls.
- [ ] Supported fired-alert actions are limited to the real backend lifecycle (`acknowledge`, `resolve`) and refresh visible list/detail state after success.
- [ ] Unsupported alert affordances such as silence/unsnooze or channel configuration stay visible only where they are still mock/fallback and do not pretend to be live.
- [ ] `mesher/client/tests/e2e/admin-ops-live.spec.ts` exists after this task and already proves an Alerts happy path plus a destructive-toast failure path.

## Verification

- `npm --prefix mesher/client run test:e2e:dev -- --grep "admin and ops live alerts"`
- The Alerts route shows same-origin live/failure state with visible toast feedback and no direct browser calls to Mesher backend ports.

## Observability Impact

- Signals added/changed: `alerts-shell` / detail-panel `data-*` state for bootstrap, selected alert, action phase, source, and last error.
- How a future agent inspects this: run `npm --prefix mesher/client run test:e2e:dev -- --grep "admin and ops live alerts"`, inspect browser network logs for same-origin `/api/v1/alerts/...`, and read the visible Radix toast region.
- Failure state exposed: alerts bootstrap failure, selected-alert action failure, and post-action refresh drift instead of silent mock fallback.

## Inputs

- `mesher/client/lib/mesher-api.ts` — existing typed same-origin read client from S01 that should absorb the admin/ops payload parsers and request helpers.
- `mesher/client/components/dashboard/alerts-page.tsx` — current standalone Alerts route that still reads `MOCK_ALERTS`.
- `mesher/client/components/dashboard/alert-detail.tsx` — current detail panel with mock-only action semantics and unsupported controls.
- `mesher/client/components/dashboard/alert-stats.tsx` — current mock stats bar that must derive at least live-backed counts.
- `mesher/client/lib/mock-data.ts` — current Alerts shell contract whose fallback-only fields must stay explicit after live wiring.
- `mesher/client/tests/e2e/issues-live-read.spec.ts` — proven same-origin request/toast/runtime-signal Playwright pattern to reuse for admin/ops proof.

## Expected Output

- `mesher/client/lib/mesher-api.ts` — typed admin/ops request helpers and payload parsers shared by later tasks.
- `mesher/client/lib/admin-ops-live-adapter.ts` — alert/status/source adapters that normalize Mesher payloads into the existing shell contract.
- `mesher/client/components/dashboard/alerts-live-state.tsx` — centralized alerts bootstrap/action/toast state owner.
- `mesher/client/components/dashboard/alerts-page.tsx` — live alerts list/filter/selection surface with explicit source diagnostics.
- `mesher/client/components/dashboard/alert-detail.tsx` — truthful live action controls and failure visibility for the selected alert.
- `mesher/client/components/dashboard/alert-stats.tsx` — live-derived stats markers inside the existing chrome.
- `mesher/client/lib/mock-data.ts` — updated alert shell types/fallback data that remain compatible with truthful live status mapping.
- `mesher/client/tests/e2e/admin-ops-live.spec.ts` — first admin/ops browser proof covering the Alerts seam.
