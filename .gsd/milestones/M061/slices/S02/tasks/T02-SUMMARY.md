---
id: T02
parent: S02
milestone: M061
key_files:
  - ../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs
  - ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs
key_decisions:
  - Preserved `readRouteInventory`/`parseRouteInventoryMarkdown` as top-level-only wrappers and added document-level helpers for mixed-surface consumers.
  - Made mixed-surface drift actionable by comparing expected section/order contracts in the test layer and surfacing exact `routeSection/surfaceKey` failures.
duration: 
verification_result: passed
completed_at: 2026-04-12T06:56:40.944Z
blocker_discovered: false
---

# T02: Extended the Mesher route-inventory verifier to parse and fail closed on mixed-surface Issues, Alerts, and Settings rows.

**Extended the Mesher route-inventory verifier to parse and fail closed on mixed-surface Issues, Alerts, and Settings rows.**

## What Happened

I refactored `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs` into a document-level parser that keeps the S01 top-level route API intact while also parsing the `## mixed-route breakdown` tables for Issues, Alerts, and Settings. The new mixed-surface helpers normalize `routeSection`, `surfaceKey`, `level`, `classification`, `codeEvidence`, `proofEvidence`, `liveSeamSummary`, `boundaryNote`, and a stable `rowKey`, and they fail closed on missing section tables, duplicate surface keys, blank evidence cells, unknown levels/classifications, and unrecognized proof suite references. I then rewrote `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` so it locks the exact mixed-surface row order and classification contract for the three sections, preserves top-level route parity checks, adds malformed-table regressions including rejected `fallback` classifications, and asserts that drift messages name the offending section/surface row. I also recorded decision D529 to preserve layered parser APIs so future slices can inspect mixed-surface rows without replacing the top-level route map as the canonical authority.

## Verification

I ran `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`, which passed all 9 structural contract tests including the new mixed-surface regressions and exact drift-message coverage. I then ran the full slice verification rail `env PLAYWRIGHT_PROJECT=dev npm --prefix ../hyperpush-mono/mesher/client exec -- playwright test --config ../hyperpush-mono/mesher/client/playwright.config.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts`, and all 21 Playwright tests passed in the dev project. This also verified the observability requirement: contract failures now identify exact `Section/surfaceKey` rows instead of generic markdown drift.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` | 0 | ✅ pass | 1193ms |
| 2 | `env PLAYWRIGHT_PROJECT=dev npm --prefix ../hyperpush-mono/mesher/client exec -- playwright test --config ../hyperpush-mono/mesher/client/playwright.config.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts` | 0 | ✅ pass | 233000ms |

## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs`
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
