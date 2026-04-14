---
estimated_steps: 4
estimated_files: 5
skills_used:
  - test
  - playwright-best-practices
---

# T02: Add a fail-closed doc-parity parser and structural test

**Slice:** S01 — Evidence-backed route inventory
**Milestone:** M061

## Description

Turn the new document into a contract instead of static prose. This task should add a small parser/helper and a real test file that compares `ROUTE-INVENTORY.md` against `dashboard-route-map.ts`, rejects classification drift, and enforces the presence of proof evidence for every row.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `../hyperpush-mono/mesher/client/components/dashboard/dashboard-route-map.ts` | Fail the test immediately instead of guessing route keys or pathnames from filenames. | N/A — local file read should be synchronous and cheap; treat read failure as a hard error. | Reject the parse if the exported route rows cannot be normalized into eight stable keys and pathnames. |
| `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` | Fail closed when the document is missing or unreadable. | N/A — local file read should be synchronous and cheap; treat read failure as a hard error. | Reject blank evidence cells, unknown classifications, missing rows, duplicate rows, or pathnames that do not match the route map. |

## Negative Tests

- **Malformed inputs**: duplicate route rows, `/issues` instead of `/`, `mixed live` in the classification cell, blank code/proof evidence columns, or an extra ninth row.
- **Error paths**: missing inventory file, renamed route-map export, or a proof reference that no longer names one of the existing suites.
- **Boundary conditions**: exactly eight unique top-level rows, five `mock-only` routes, and three `mixed` routes.

## Steps

1. Add a tiny helper module that parses the canonical route map and the markdown inventory into normalized row objects with stable keys.
2. Add a `node:test` contract file that asserts exact key/path parity, expected top-level classifications, and non-empty code/proof evidence per row.
3. Make the test fail when a row cites no recognized proof suite or when the doc drifts from the route map.
4. Keep the parser narrowly scoped to top-level inventory truth; do not introduce a second runtime classification registry.

## Must-Haves

- [ ] One helper parses both the route map and the inventory markdown into stable top-level rows.
- [ ] The structural test fails on missing/extra rows, pathname drift, unexpected classifications, and blank evidence cells.
- [ ] The task preserves D524/D526 by validating the human document against existing code/tests instead of adding a new runtime source of truth.

## Verification

- `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`

## Inputs

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` — canonical human-maintained inventory from T01.
- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-route-map.ts` — canonical route-map source of truth.
- `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts` — recognized direct-entry/parity proof suite.
- `../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts` — recognized route walkthrough proof suite.
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts` — recognized Issues live-read proof suite.
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts` — recognized Issues live-action proof suite.
- `../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts` — recognized Alerts/Settings live proof suite.

## Expected Output

- `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs` — shared parser for route-map rows and inventory markdown rows.
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` — fail-closed structural contract test for the route inventory.
