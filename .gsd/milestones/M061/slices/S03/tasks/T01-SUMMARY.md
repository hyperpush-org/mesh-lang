---
id: T01
parent: S03
milestone: M061
key_files:
  - ../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md
  - ../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs
  - ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs
key_decisions:
  - Used separate `### Issues/Alerts/Settings backend gaps` tables with stable `route/surface` keys so the parser can distinguish the backend gap map from the existing mixed-surface inventory and fail closed on section drift.
  - Kept lifecycle/member/key/rule flows in `covered`, classified fallback-derived overview/detail data as `missing-payload`, shell-only controls that outrun existing live families as `missing-controls`, and truly absent route families as `no-route-family`.
duration: 
verification_result: passed
completed_at: 2026-04-12T17:03:51.394Z
blocker_discovered: false
---

# T01: Added a fail-closed backend gap map for mixed Issues, Alerts, and Settings surfaces and enforced it in the route-inventory verifier.

**Added a fail-closed backend gap map for mixed Issues, Alerts, and Settings surfaces and enforced it in the route-inventory verifier.**

## What Happened

Updated `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` with a new `## Backend gap map` section that covers the currently mixed Mesher-backed surfaces: Issues overview/detail/actions/shell controls, Alerts overview/detail/actions/shell controls, and Settings general/team/api-keys/alert-rules/alert-channels. Each row now names the visible client promise, the concrete Mesher seam currently in use, one stable support status (`covered`, `missing-payload`, `missing-controls`, or `no-route-family`), and the specific backend work still needed. I based the row claims on `main.mpl`, `client/lib/mesher-api.ts`, `client/lib/issues-live-adapter.ts`, `client/lib/admin-ops-live-adapter.ts`, `components/dashboard/dashboard-issues-state.tsx`, `components/dashboard/alerts-live-state.tsx`, `components/dashboard/issue-detail.tsx`, `components/dashboard/alert-detail.tsx`, and `components/dashboard/settings/settings-page.tsx` / `settings-live-state.tsx`, so fallback-derived fields and shell-only controls were not promoted into live coverage.

To keep the document fail-closed, I extended `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs` to parse backend-gap tables under distinct `### … backend gaps` headings, validate route/surface keys, reject duplicate rows, and enforce the stable backend-gap status vocabulary. I then updated `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` with the expected backend-gap row set plus malformed-section/status/duplicate/blank-seam coverage so future drift is reported against the exact section and row instead of silently landing as markdown prose.

## Verification

Verified the new backend-gap map and fail-closed parser/test contract with `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`, the task-plan markdown presence check for the required mixed-route keys, and the slice-level backend-gap content assertion for the new section/status vocabulary. All checks passed.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` | 0 | ✅ pass | 2297ms |
| 2 | `python3 - <<'PY'
from pathlib import Path
text = Path('../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md').read_text()
for needle in (
    '`issues/overview`',
    '`issues/live-actions`',
    '`alerts/detail`',
    '`alerts/live-actions`',
    '`settings/general`',
    '`settings/team`',
    '`settings/api-keys`',
    '`settings/alert-rules`',
    '`settings/alert-channels`',
):
    assert needle in text, needle
PY` | 0 | ✅ pass | 275ms |
| 3 | `python3 - <<'PY'
from pathlib import Path
text = Path('../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md').read_text()
assert '## Backend gap map' in text
needles = ['`issues/overview`','`alerts/live-actions`','`settings/alert-channels`','`performance`','`treasury`','`covered`','`missing-payload`','`missing-controls`','`no-route-family`']
assert all(needle in text for needle in needles)
PY` | 0 | ✅ pass | 253ms |

## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`
- `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs`
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
