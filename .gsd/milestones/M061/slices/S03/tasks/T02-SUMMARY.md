---
id: T02
parent: S03
milestone: M061
key_files:
  - ../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md
  - ../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs
  - ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs
  - .gsd/KNOWLEDGE.md
  - .gsd/DECISIONS.md
key_decisions:
  - Extended backend-gap parsing with a dedicated `BACKEND_GAP_ROUTE_SECTIONS` contract so mock-only route sections are verified as strictly as mixed-route sections.
  - Kept the new mock-only backend-gap rows at route or major-subsection scope and classified them `no-route-family` until `main.mpl` registers a real same-origin family.
duration: 
verification_result: passed
completed_at: 2026-04-12T17:11:59.415Z
blocker_discovered: false
---

# T02: Documented the missing backend route families behind Mesher’s remaining mock-only dashboard routes and extended the route-inventory verifier to fail closed on them.

**Documented the missing backend route families behind Mesher’s remaining mock-only dashboard routes and extended the route-inventory verifier to fail closed on them.**

## What Happened

Extended `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` so the backend gap map now covers the five top-level mock-only routes that were still outside the maintainer-facing contract: Performance, Solana Programs, Releases, Bounties, and Treasury. I kept the new rows at route or major-subsection scope instead of per-button scope, and each row now names the visible client promise, the fact that `../hyperpush-mono/mesher/main.mpl` registers no matching same-origin family today, and the concrete route family backend maintainers would need before the page can become truthful. The new sections cover performance overview and transaction drill-down, Solana program overview and log inspection, release list/detail/actions, bounty list and review-payout flows, and treasury balances/allocations/transactions.

To keep that map fail-closed instead of leaving it as prose, I updated `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs` so backend-gap parsing is no longer tied only to `MIXED_ROUTE_SECTIONS`. The parser now uses a dedicated `BACKEND_GAP_ROUTE_SECTIONS` list that includes the new mock-only route sections, so missing headings, duplicate rows, unsupported statuses, or row-order drift fail against the exact backend-gap section. I then updated `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` with the expected row order and `no-route-family` statuses for the new sections. I also tightened the inventory invariants to preserve the rule that top-level mock-only routes stay grouped at route or major-subsection scope until a real Mesher seam lands, rather than exploding shell CTAs like Rollback or Process Payout into per-button rows.

Because this changed a downstream documentation contract, I recorded the scope decision in `.gsd/DECISIONS.md` and added a `KNOWLEDGE.md` note that future backend-gap work must extend `BACKEND_GAP_ROUTE_SECTIONS` and the verifier’s expected row list together; otherwise new markdown rows will remain unverified prose.

## Verification

Verified the updated fail-closed contract with the Node route-inventory test rail and two markdown-presence checks. `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` passed with all 10 assertions green after the parser/test updates. A task-scoped Python check confirmed the document now contains `performance`, `solana-programs`, `releases`, `bounties`, `treasury`, and `no-route-family`. The slice-level Python check also passed, confirming `## Backend gap map` still contains the previously required mixed-route needles and stable status vocabulary (`covered`, `missing-payload`, `missing-controls`, `no-route-family`).

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` | 0 | ✅ pass | 750ms |
| 2 | `python3 - <<'PY'
from pathlib import Path
text = Path('../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md').read_text()
for needle in (
    '`performance`',
    '`solana-programs`',
    '`releases`',
    '`bounties`',
    '`treasury`',
    '`no-route-family`',
):
    assert needle in text, needle
PY` | 0 | ✅ pass | 90ms |
| 3 | `python3 - <<'PY'
from pathlib import Path
text = Path('../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md').read_text()
assert '## Backend gap map' in text
needles = ['`issues/overview`','`alerts/live-actions`','`settings/alert-channels`','`performance`','`treasury`','`covered`','`missing-payload`','`missing-controls`','`no-route-family`']
assert all(needle in text for needle in needles)
PY` | 0 | ✅ pass | 90ms |

## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`
- `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs`
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
- `.gsd/KNOWLEDGE.md`
- `.gsd/DECISIONS.md`
