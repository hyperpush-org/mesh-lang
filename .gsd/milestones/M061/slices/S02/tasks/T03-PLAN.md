---
estimated_steps: 4
estimated_files: 6
skills_used:
  - playwright-best-practices
  - test
---

# T03: Add minimal runtime assertions for fine-grained mixed-surface proof

**Slice:** S02 — Mixed-surface audit
**Milestone:** M061

## Description

Audit the proof suites cited by the new mixed-surface rows and add the minimum missing assertions so proof evidence is honest at control granularity. Reuse the shipped Issues, Alerts, and Settings suites from `mesh-lang` with the explicit sibling Playwright config path. Prefer existing stable selectors such as `data-source`, `*-status-banner`, `*-mock-only-banner`, `*-action-error`, and `*-action-*` test ids; add new selectors only when a documented row cannot be proven honestly without them. Likely proof gaps to close are grouped issue shell-only controls, alert shell-only controls such as `alert-detail-copy-link`, and any row-level Settings support/mixed markers that are still only implied rather than asserted.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Same-origin Issues/Alerts/Settings seeded runtime APIs | Fail the targeted suite with explicit selector/runtime diagnostics and inspect the failing route's existing fallback markers before broadening the test. | Use the existing seeded helpers and explicit waits; do not add raw sleeps as a recovery strategy. | Assert fallback/error banners and `data-source` markers instead of pretending the malformed live payload is a passing shell-only state. |
| Existing Playwright selectors/test ids | Add or update stable selectors in the owning component when a documented row cannot be asserted honestly. | Treat missing selectors as a test-contract failure and patch the selector rather than weakening the assertion. | Prefer grouped row assertions if the UI intentionally exposes multiple shell-only controls through one shared source note or banner. |

## Load Profile

- **Shared resources**: seeded dev runtime, Playwright browser workers, same-origin API routes for issue/alert/settings reads and writes, and retained client test-results artifacts.
- **Per-operation cost**: one targeted Playwright run across four existing suites with seeded reads/writes and explicit selectors.
- **10x breakpoint**: the first pressure points are seeded data setup and route refresh timing, so new assertions must stay surgical and reuse existing helpers instead of multiplying broad end-to-end flows.

## Negative Tests

- **Malformed inputs**: invalid Settings General values, blank API key labels, malformed alert-rule JSON, or missing raw team `user_id` should continue to expose validation errors rather than being documented as live success.
- **Error paths**: bootstrap/mutation failures for Issues and Alerts must still surface `fallback`/`*-action-error` diagnostics when routes fail or same-origin writes return errors.
- **Boundary conditions**: empty live alerts lists, sparse issue detail data, and mock-only Settings tabs must remain truthful without rehydrating fallback rows or implying unsupported writes.

## Steps

1. Map the T01 row set against the current assertions in `issues-live-read.spec.ts`, `issues-live-actions.spec.ts`, `admin-ops-live.spec.ts`, and `seeded-walkthrough.spec.ts` to find proof gaps.
2. Add the smallest possible assertions for grouped issue shell-only controls/proof harness, alert shell-only controls, and any unasserted Settings mixed/mock-only boundaries; add test ids in the owning component only if an honest assertion cannot be written otherwise.
3. Keep one or more explicit fallback/error-path assertions in place for Issues, Alerts, and Settings so future regressions still expose `data-source`, `data-state`, or `*-error` diagnostics when live reads/writes fail.
4. Run the targeted suites from `mesh-lang` with the explicit sibling config path and update proof references in the doc only if the actual exercised suite set changed.

## Must-Haves

- [ ] Every fine-grained row that cites Playwright proof has at least one explicit runtime assertion in an existing suite, or the row is regrouped so the cited proof remains honest.
- [ ] No new proof relies on sleep-only timing, screenshots, or cwd inference; selectors and explicit assertions must stay resilient and rerunnable from `mesh-lang`.
- [ ] At least one failure-path diagnostic assertion remains for mixed/fallback behavior so proof still localizes real seam regressions.

## Verification

- `env PLAYWRIGHT_PROJECT=dev npm --prefix ../hyperpush-mono/mesher/client exec -- playwright test --config ../hyperpush-mono/mesher/client/playwright.config.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts`

## Observability Impact

- Signals added/changed: any new or regrouped selectors should expose shell-only/live/fallback state through stable test ids or existing `data-source`/`data-state` attributes.
- How a future agent inspects this: rerun the targeted Playwright command from `mesh-lang` and inspect failing selectors plus `../hyperpush-mono/mesher/client/test-results/` artifacts.
- Failure state exposed: shell-only control drift, missing mixed-state badges, or lost fallback diagnostics fail on explicit selectors rather than on broad page-level behavior.

## Inputs

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` — fine-grained mixed-surface rows and proof citations from T01.
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` — structural contract from T02 that locks the row set.
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts` — Issues read/fallback proof suite.
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts` — Issues action/proof-harness suite.
- `../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts` — Alerts and Settings live proof suite.
- `../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts` — cross-route walkthrough used as shared proof anchor.

## Expected Output

- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts` — updated Issues row-level runtime assertions where needed.
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts` — updated Issues action/shell-control/proof-harness assertions where needed.
- `../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts` — updated Alerts/Settings row-level runtime assertions where needed.
- `../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts` — updated cross-route proof assertions where needed.
- `../hyperpush-mono/mesher/client/components/dashboard/issue-detail.tsx` — any additional stable test ids required for honest issue shell-control assertions.
- `../hyperpush-mono/mesher/client/components/dashboard/alert-detail.tsx` — any additional stable test ids required for honest alert shell-control assertions.
