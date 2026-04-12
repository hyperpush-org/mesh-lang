---
id: T02
parent: S01
milestone: M061
key_files:
  - ../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs
  - ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs
key_decisions:
  - Kept the verifier text-driven and fail-closed against `dashboard-route-map.ts` plus `ROUTE-INVENTORY.md` instead of creating a second classification registry.
  - Restricted proof references to the five recognized existing Playwright suites and required those suite files to exist on disk before accepting the inventory.
duration: 
verification_result: mixed
completed_at: 2026-04-12T03:11:25.024Z
blocker_discovered: false
---

# T02: Added a fail-closed route inventory parser and structural contract test for mesher/client top-level routes.

**Added a fail-closed route inventory parser and structural contract test for mesher/client top-level routes.**

## What Happened

Implemented `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs` as a narrowly scoped helper that reads the canonical dashboard route map and the maintainer-facing `ROUTE-INVENTORY.md` into stable top-level rows without introducing a second runtime source of truth. The helper fails closed on unreadable files, missing/extra rows, duplicate keys, unknown classifications, invalid pathnames, blank code/proof evidence cells, and proof references that are not one of the recognized existing Playwright suites.

Added `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` as the structural contract test. It asserts exact key/path parity against `dashboard-route-map.ts`, locks the expected top-level classifications to three `mixed` routes and five `mock-only` routes, verifies non-empty code/proof evidence per row, confirms the recognized proof-suite files still exist, and exercises negative cases for pathname drift, `mixed live`, blank evidence cells, duplicate rows, an extra ninth row, missing inventory files, a renamed route-map export, and stale proof references.

I kept the implementation text-driven and fail-closed instead of importing client runtime code into the verifier, so downstream work still treats the route map plus the human-maintained inventory document as the only authoritative inputs. The slice-level wrapper verifier remains absent and is still owned by T03, so the wrapper command continues to fail with a missing-file error at this intermediate task boundary.

## Verification

Task-level verification passed with `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`, covering the live contract and the planned malformed/error-path cases. For slice-level verification, I also ran `bash ../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh`; it failed with `No such file or directory` because the wrapper verifier script is not part of T02 and is still planned for T03. That partial slice-verification state is expected for this task boundary rather than a regression in the new parser/test.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` | 0 | ✅ pass | 1132ms |
| 2 | `bash ../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` | 127 | ❌ fail | 16ms |

## Deviations

None.

## Known Issues

`../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` does not exist yet, so the slice-level wrapper verification command still fails until T03 lands.

## Files Created/Modified

- `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs`
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
