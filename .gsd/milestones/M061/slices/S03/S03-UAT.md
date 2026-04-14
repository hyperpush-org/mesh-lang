# S03: Backend gap map — UAT

**Milestone:** M061
**Written:** 2026-04-12T17:23:15.269Z

# S03 UAT — Backend gap map

## Preconditions
- Work from `/Users/sn0w/Documents/dev/mesh-lang` with the sibling repo available at `../hyperpush-mono`.
- The canonical inventory file exists at `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`.
- Node is available for the Mesher route-inventory contract tests.

## Test Case 1 — Backend-gap contract rail passes
1. Run `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`.
   - Expected: the suite passes with all backend-gap tests green, including malformed-section, duplicate-row, blank-cell, row-drift, and section-order cases.
2. Review the test names in the output.
   - Expected: the output explicitly includes backend-gap contract coverage such as malformed backend-gap rows/sections and exact section/surface drift reporting.
3. Confirm the suite does not require any live runtime boot or browser session.
   - Expected: this slice remains a documentation + parser/test contract proof surface only.

## Test Case 2 — Mixed-route backend gaps are actionable
1. Open `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` and locate `## Backend gap map`.
   - Expected: the section exists and appears after the mixed-route breakdown.
2. Inspect `### Issues backend gaps`, `### Alerts backend gaps`, and `### Settings backend gaps`.
   - Expected: each section uses stable backticked `route/surface` keys and has columns for client promise, current backend seam, support status, and remaining backend work.
3. Verify representative rows:
   - `issues/live-actions` is `covered`.
   - `issues/overview` and `issues/detail` are `missing-payload`.
   - `alerts/shell-controls` is `missing-controls`.
   - `settings/team`, `settings/api-keys`, and `settings/alert-rules` are `covered`.
   - `settings/alert-channels` is `no-route-family`.
   - Expected: the statuses match the real backend support boundary and do not overstate derived/fallback fields or shell-only controls as live-backed.

## Test Case 3 — Mock-only dashboard routes are mapped to missing route families
1. In the same backend-gap section, inspect `### Performance backend gaps`, `### Solana Programs backend gaps`, `### Releases backend gaps`, `### Bounties backend gaps`, and `### Treasury backend gaps`.
   - Expected: every remaining mock-only route family is present.
2. Verify the rows stay grouped at route or major-subsection scope rather than exploding every CTA into a separate row.
   - Expected: examples include `performance/transactions`, `solana-programs/log-inspection`, `releases/actions`, `bounties/review-payouts`, and `treasury/transactions`.
3. Check the support status for those rows.
   - Expected: they are `no-route-family` until `main.mpl` registers a same-origin backend family.

## Test Case 4 — Existing top-level readers remain stable while document helpers expose backend gaps
1. Confirm the inventory parser still preserves the older top-level contract by reading the assertions in `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` around `readRouteInventory()`, `parseRouteInventoryMarkdown()`, and `readRouteInventoryDocument()`.
   - Expected: `readRouteInventory()` and `parseRouteInventoryMarkdown()` still operate as top-level-row wrappers.
2. Confirm the document helper exposes backend-gap sections for the verifier.
   - Expected: the contract asserts ordered `BACKEND_GAP_ROUTE_SECTIONS` and exact backend-gap row parity without forcing top-level callers to change shape.

## Edge Cases
- If a maintainer deletes or reorders a backend-gap section heading, the contract rail should fail with a named heading error instead of silently accepting the markdown drift.
- If a maintainer duplicates a route/surface key, leaves the backend seam blank, leaves the remaining backend work blank, or invents a new support status outside `covered`, `missing-payload`, `missing-controls`, and `no-route-family`, the contract rail should fail and identify the exact offending row.
- If a future backend slice adds a new backend-gap section or row for a mock-only route without updating `BACKEND_GAP_ROUTE_SECTIONS` and the expected row list in the verifier, the new markdown should be treated as drift rather than silently trusted.
