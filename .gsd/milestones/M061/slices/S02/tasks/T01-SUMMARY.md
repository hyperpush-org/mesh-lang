---
id: T01
parent: S02
milestone: M061
key_files:
  - ../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Used stable surface keys (`overview`, `list`, `detail`, `live-actions`, `shell-controls`, `proof-harness`, etc.) as the durable mixed-surface contract instead of prose-only route notes.
  - Kept runtime `fallback` as a note-level diagnostic state in the tables rather than promoting it to a canonical classification.
duration: 
verification_result: mixed
completed_at: 2026-04-12T06:45:33.256Z
blocker_discovered: false
---

# T01: Replaced the mixed-route prose in the Mesher route inventory with fail-closed Issues, Alerts, and Settings surface tables.

**Replaced the mixed-route prose in the Mesher route inventory with fail-closed Issues, Alerts, and Settings surface tables.**

## What Happened

I audited the current Issues, Alerts, and Settings seams in `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` alongside the live dashboard sources (`issues-page.tsx`, `issue-detail.tsx`, `alerts-page.tsx`, `alert-detail.tsx`, `settings-page.tsx`, and `settings-live-state.tsx`). I then replaced the prose-only `## mixed-route breakdown` bullets with three structured markdown tables under `### Issues`, `### Alerts`, and `### Settings`. Each row now uses one stable surface key, an explicit level (`panel`, `subsection`, `tab`, or `control`), a normalized classification (`mixed`, `live`, `mock-only`, or `shell-only`), backticked code anchors, backticked proof suites, a live-seam summary, and a boundary note. For Settings, the tables now distinguish `general` as mixed, `team` / `api-keys` / `alert-rules` as live, `alert-channels` as shell-only, and the remaining tabs as mock-only. I kept runtime `fallback` semantics in the live-seam and boundary-note cells instead of using `fallback` as a durable classification. I also appended `.gsd/KNOWLEDGE.md` with a note about the combined mixed-surface Playwright rail being flaky in ways that are not tied to this markdown-only change.

## Verification

The task-level markdown contract passed: the headings and required table rows are present in `ROUTE-INVENTORY.md`. The structural verifier `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` also passed cleanly. I then ran the slice-level Playwright command twice. The first combined run failed in two unrelated runtime paths (`admin-ops-live` alert seeding/request-context disposal and a missing `alerts-shell` fallback-state assertion). I reran the failing admin-alert subset in isolation and it passed cleanly, indicating the document change did not introduce a deterministic alert regression. I then reran the full slice Playwright command once more; that rerun reduced to a single pre-existing runtime failure in `issues-live-actions.spec.ts` caused by an unexpected React console warning (`Can't perform a React state update on a component that hasn't mounted yet`) on the mutation-failure path. No application/runtime files were changed in this task, so the markdown work is complete while the broad browser rail remains partially flaky/non-green.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `python3 - <<'PY' ... ROUTE-INVENTORY heading/row assertions ... PY` | 0 | ✅ pass | 0ms |
| 2 | `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` | 0 | ✅ pass | 1389ms |
| 3 | `env PLAYWRIGHT_PROJECT=dev npm --prefix ../hyperpush-mono/mesher/client exec -- playwright test --config ../hyperpush-mono/mesher/client/playwright.config.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts` | 1 | ❌ fail | 407200ms |
| 4 | `env PLAYWRIGHT_PROJECT=dev npm --prefix ../hyperpush-mono/mesher/client exec -- playwright test --config ../hyperpush-mono/mesher/client/playwright.config.ts ../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts --grep "admin and ops live alerts"` | 0 | ✅ pass | 60900ms |

## Deviations

None.

## Known Issues

The combined slice-level Playwright rail remains flaky outside the scope of this documentation task. Evidence from this task: (1) the first full run failed in `admin-ops-live.spec.ts` with `Request context disposed` during seeded alert creation and then missed `alerts-shell`; (2) the isolated `admin and ops live alerts` subset passed on immediate rerun; (3) the second full rerun failed only in `issues-live-actions.spec.ts` because the mutation-failure path surfaced an unexpected React console warning (`Can't perform a React state update on a component that hasn't mounted yet`).

## Files Created/Modified

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`
- `.gsd/KNOWLEDGE.md`
