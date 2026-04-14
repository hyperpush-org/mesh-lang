---
estimated_steps: 4
estimated_files: 3
skills_used:
  - test
---

# T03: Lock the backend gap map with parser and contract coverage

**Slice:** S03 — Backend gap map
**Milestone:** M061

## Description

Turn the new backend gap map into a fail-closed contract. Extend `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs` so it parses backend-gap sections and rows on top of the existing S01/S02 document model, and extend `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` so it locks the exact expected row keys and support statuses for both mixed-route and mock-only backend-gap entries. Preserve `readRouteInventory()` / `parseRouteInventoryMarkdown()` as top-level wrappers so existing callers do not break.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` | Fail closed when the backend-gap section, row keys, or seam summaries drift instead of guessing missing rows. | N/A — local file reads are synchronous and cheap. | Reject missing tables, duplicate gap keys, blank code/seam/missing-support cells, or unsupported support-status values. |
| `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs` | Keep older top-level readers stable while adding document-level helpers for backend-gap rows. | N/A — local module load is synchronous and cheap. | If the parser shape drifts, fail the tests instead of silently changing the public wrapper contract. |
| `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` | Make the test output name the offending gap row or section so maintainers can localize drift quickly. | N/A — local test execution is bounded and deterministic. | Reject unexpected extra/missing rows, stale status vocabulary, or section order drift instead of accepting free-form markdown. |

## Load Profile

- **Shared resources**: local filesystem reads for the canonical doc, parser module, and proof-suite references.
- **Per-operation cost**: one markdown parse plus a fixed-size assertion set over expected backend-gap rows.
- **10x breakpoint**: backend-gap parsing must stay linear in row count and should reuse the existing document-parse pattern rather than stacking ad-hoc regex passes for each section.

## Negative Tests

- **Malformed inputs**: duplicate backend-gap row keys, blank seam summaries, blank missing-support cells, missing backend-gap headings, or unsupported status values such as `partial` or `live-ish`.
- **Error paths**: renamed section headings, row-order drift, or accidental removal of a required mock-only route row should all fail with actionable messages.
- **Boundary conditions**: existing top-level inventory readers still return only top-level rows; document-level helpers expose backend-gap rows; expected row/status sets stay exact for mixed and mock-only entries.

## Steps

1. Add backend-gap parse helpers and exported constants to `client-route-inventory.mjs`, following the layered S02 pattern instead of replacing the top-level reader API.
2. Define the expected backend-gap row sets and allowed support-status values in `verify-client-route-inventory.test.mjs` for the mixed-route and mock-only sections added in T01/T02.
3. Add regression cases for missing sections, duplicate keys, blank seam/missing-support cells, unsupported status values, and any drift between expected row keys and the canonical markdown.
4. Run the structural contract and tighten any doc wording or parser behavior until the new backend-gap rail is green from `mesh-lang`.

## Must-Haves

- [ ] The parser exposes backend-gap rows through document-level helpers without breaking the existing S01/S02 top-level APIs.
- [ ] The contract test locks exact backend-gap row keys and support-status values for the canonical markdown.
- [ ] Failure output points maintainers at the exact backend-gap row or section that drifted.

## Verification

`node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`

## Observability Impact

- Signals added/changed: structural contract failures now identify the exact backend-gap row or section that drifted.
- How a future agent inspects this: run `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` and read the named failing row from the thrown error.
- Failure state exposed: missing sections, duplicate rows, unsupported status values, blank seam cells, or stale row sets become explicit contract failures instead of silent markdown rot.

## Inputs

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`
- `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs`
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`

## Expected Output

- `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs`
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
