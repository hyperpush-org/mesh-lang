---
estimated_steps: 4
estimated_files: 3
skills_used:
  - test
---

# T02: Extend the route-inventory parser and contract test for mixed-surface rows

**Slice:** S02 — Mixed-surface audit
**Milestone:** M061

## Description

Turn the new mixed-route tables into a fail-closed contract. Update `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs` so it parses section-scoped mixed-surface rows in addition to the top-level table, and update `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` so it locks exact expected `(route section, surface key)` pairs, allowed classifications, non-empty evidence cells, and recognized proof references. The contract should reject `fallback` as a canonical row classification, reject duplicate surface keys within a section, and name the offending section/surface row when drift occurs.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` | Fail closed instead of guessing missing section/surface rows. | N/A — local file read should be synchronous and cheap; treat read failure as a hard error. | Reject missing tables, duplicate surface keys, blank evidence cells, or `fallback` used as a canonical classification. |
| `../hyperpush-mono/mesher/client/tests/e2e/*.spec.ts` proof references | Fail the structural contract when a cited proof file no longer exists or is not recognized. | N/A — existence checks are local and synchronous. | Reject unrecognized proof references instead of accepting arbitrary strings in the proof column. |

## Load Profile

- **Shared resources**: local filesystem reads for the canonical doc, proof-suite files, and parser/test modules.
- **Per-operation cost**: one markdown parse plus existence checks across a small fixed set of suite files.
- **10x breakpoint**: row-count growth should remain linear; if future sections explode in size, the parser must still avoid nested backtracking or duplicate-scan behavior.

## Negative Tests

- **Malformed inputs**: duplicate surface keys inside a section, missing `### Issues`/`### Alerts`/`### Settings` tables, blank code/proof evidence cells, or `fallback` used in the classification column.
- **Error paths**: missing inventory file, renamed/removed proof suite files, or a section row that cites a proof file not in the recognized suite list.
- **Boundary conditions**: exact expected surface-key sets for Issues, Alerts, and Settings; exact allowed classifications `mixed`, `live`, `mock-only`, and `shell-only`; and preserved top-level route parity from S01.

## Steps

1. Add parser helpers for section-scoped mixed-surface tables that normalize route section, surface key, level, classification, code evidence, proof evidence, live-seam summary, and boundary note.
2. Add explicit expected row sets for Issues, Alerts, and Settings and make the contract fail on missing, duplicate, extra, or reordered surface keys where order matters to the human document.
3. Preserve and extend evidence validation so blank code/proof cells, unknown classifications, and unrecognized proof suite references fail with actionable messages.
4. Add regression tests for malformed mixed-surface rows, including accidental `fallback` classifications, duplicate keys, missing sections, and proof references that no longer map to shipped suites.

## Must-Haves

- [ ] The parser returns stable mixed-surface row objects keyed by route section plus surface key, without replacing the route map as the top-level authority.
- [ ] `verify-client-route-inventory.test.mjs` fails on mixed-surface row drift, blank evidence, unknown classifications, duplicate rows, and unrecognized proof references.
- [ ] The test output points maintainers at the exact section/surface row that drifted so future slices can localize failures quickly.

## Verification

- `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`

## Observability Impact

- Signals added/changed: parser/test failures now identify the exact mixed-route section and surface key that drifted.
- How a future agent inspects this: run `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` and read the named failing row from the thrown error.
- Failure state exposed: missing sections, extra rows, bad classifications, blank evidence, or stale proof references become explicit contract failures instead of silent markdown rot.

## Inputs

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` — structured mixed-surface tables from T01.
- `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs` — existing top-level parser that needs mixed-surface support.
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` — existing fail-closed structural contract from S01.

## Expected Output

- `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs` — parser helpers that understand both top-level and mixed-surface inventory rows.
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` — mixed-surface contract assertions and regression cases.
