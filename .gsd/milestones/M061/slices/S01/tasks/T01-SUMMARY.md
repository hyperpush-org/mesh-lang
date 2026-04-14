---
id: T01
parent: S01
milestone: M061
key_files:
  - ../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md
key_decisions:
  - Normalized the app-facing Settings label `mixed live` to top-level inventory classification `mixed` so the document stays aligned with slice terminology.
  - Kept T01 scoped to the canonical inventory document and left the parser/test and wrapper verifier to T02/T03 as planned.
duration: 
verification_result: mixed
completed_at: 2026-04-12T03:05:00.893Z
blocker_discovered: false
---

# T01: Added mesher/client/ROUTE-INVENTORY.md with canonical top-level route classifications, code anchors, and rerunnable proof references.

**Added mesher/client/ROUTE-INVENTORY.md with canonical top-level route classifications, code anchors, and rerunnable proof references.**

## What Happened

Created `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` as the maintainer-facing canonical inventory for the eight top-level dashboard routes defined by `components/dashboard/dashboard-route-map.ts`. The document mirrors the route map exactly, keeps `issues` at `/`, classifies `issues`, `alerts`, and `settings` as `mixed`, classifies `performance`, `solana-programs`, `releases`, `bounties`, and `treasury` as `mock-only`, and avoids claiming any top-level route is fully live.

For each route row, I added a canonical pathname, normalized classification, concrete component anchors, existing Playwright proof rails, a concise backend seam summary, and a short boundary note. I also added a dedicated mixed-route breakdown for Issues, Alerts, and Settings so future backend work can see which files and proof suites already cover the live-backed seams.

Finally, I documented the required invariants: Issues remains canonically rooted at `/`, the app’s Settings label `mixed live` normalizes to slice language `mixed`, and runtime `fallback` should not be confused with canonical `mock-only`. I intentionally did not create the structural parser test or wrapper verifier in this task because T02 and T03 explicitly own those artifacts in the slice plan.

## Verification

Task-level verification passed: the Python contract confirmed exactly eight inventory rows, the expected route-key set, `issues` at `/`, and `settings` at `/settings`; the ripgrep contract confirmed the document contains the required mixed-route breakdown section and evidence references for `dashboard-route-parity.spec.ts`, `seeded-walkthrough.spec.ts`, `issues-page.tsx`, `alerts-page.tsx`, and `settings-page.tsx`.

Slice-level verification is only partially available at this task boundary. Running `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` failed because the planned T02 structural test file does not exist yet, and running `bash ../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` failed because the planned T03 verifier script does not exist yet. Those failures are expected for T01 and document the remaining slice work rather than a regression in the inventory document itself.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `python3 - <<'INNER'
from pathlib import Path
import re
text = Path('../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md').read_text()
rows = re.findall(r'^\| `([^`]+)` \| `([^`]+)` \| `(mixed|mock-only)` \|', text, re.M)
assert len(rows) == 8, rows
mapping = {key: path for key, path, _ in rows}
assert mapping['issues'] == '/'
assert mapping['settings'] == '/settings'
assert set(mapping) == {'issues','performance','solana-programs','releases','alerts','bounties','treasury','settings'}
print('inventory-row-check: ok')
INNER` | 0 | ✅ pass | 266ms |
| 2 | `rg -n "mixed-route breakdown|dashboard-route-parity.spec.ts|seeded-walkthrough.spec.ts|issues-page.tsx|alerts-page.tsx|settings-page.tsx" ../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` | 0 | ✅ pass | 97ms |
| 3 | `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` | 1 | ❌ fail | 1096ms |
| 4 | `bash ../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` | 127 | ❌ fail | 70ms |

## Deviations

None.

## Known Issues

The slice-level structural verifier and retained wrapper command are still absent, so the slice verification commands currently fail with missing-file errors until T02 and T03 land.

## Files Created/Modified

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`
