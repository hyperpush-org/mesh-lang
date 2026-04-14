---
id: T03
parent: S03
milestone: M061
key_files:
  - ../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs
  - ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs
  - .gsd/DECISIONS.md
key_decisions:
  - D533: scan route-inventory section headings in canonical order so reordered backend-gap tables fail closed without widening the top-level wrapper API.
duration: 
verification_result: passed
completed_at: 2026-04-12T17:18:47.117Z
blocker_discovered: false
---

# T03: Locked backend-gap section ordering and fail-closed contract coverage in Mesher's route-inventory verifier.

**Locked backend-gap section ordering and fail-closed contract coverage in Mesher's route-inventory verifier.**

## What Happened

Updated `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs` to scan mixed-surface and backend-gap headings in canonical order so reordered markdown sections now fail with named heading errors, while `readRouteInventory()` and `parseRouteInventoryMarkdown()` remain top-level-row wrappers for existing callers. Extended `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` to assert wrapper stability, document-level backend-gap exposure, backend-gap section-order drift, blank remaining-work cells, and exact row-localized backend-gap drift messages. Recorded decision D533 so downstream work keeps backend-gap rows document-scoped and fail-closed on heading order drift instead of silently accepting markdown reshuffles.

## Verification

Ran `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`; all 11 contract tests passed, including the new backend-gap section-order, blank-cell, and exact-row drift cases. Ran a Python smoke check against `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`; the `## Backend gap map` heading and the required route/status needles were all present.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` | 0 | ✅ pass | 964ms |
| 2 | `python3 - <<'PY' (assert Backend gap map heading and required support-status needles in ../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md) PY` | 0 | ✅ pass | 0ms |

## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs`
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
- `.gsd/DECISIONS.md`
