---
id: S03
parent: M061
milestone: M061
provides:
  - A canonical backend-gap map for every currently shipped Mesher client promise.
  - A fail-closed parser/test contract that rejects backend-gap markdown drift with route/surface-localized errors.
  - Validated evidence for R169 so later backend planning can scope follow-up seams directly from the inventory.
requires:
  []
affects:
  - S04
key_files:
  - ../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md
  - ../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs
  - ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
  - .gsd/PROJECT.md
key_decisions:
  - D531 — keep the backend gap map inside the canonical route inventory and parse it through document-level helpers while preserving top-level wrappers.
  - D532 — extend the backend-gap contract to dedicated Performance, Solana Programs, Releases, Bounties, and Treasury sections keyed at route or major-subsection scope.
  - D533 — scan mixed-surface and backend-gap headings in canonical order so reordered sections fail closed with named heading errors.
  - D534 — use the fixed support-status vocabulary `covered` / `missing-payload` / `missing-controls` / `no-route-family` so partial backend support is described precisely rather than vaguely.
patterns_established:
  - Backend-gap rows live beside the canonical client route inventory instead of in a second registry.
  - Document-level parser helpers can expose richer mixed-surface/backend-gap structure while `readRouteInventory()` and `parseRouteInventoryMarkdown()` remain stable wrapper APIs for existing callers.
  - Backend support classification should distinguish missing payloads from missing controls and completely absent route families so backend planning can pick the next slice without re-auditing the UI.
observability_surfaces:
  - `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` backend gap map
  - `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` contract rail with exact section/row drift reporting
drill_down_paths:
  - .gsd/milestones/M061/slices/S03/tasks/T01-SUMMARY.md
  - .gsd/milestones/M061/slices/S03/tasks/T02-SUMMARY.md
  - .gsd/milestones/M061/slices/S03/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-12T17:23:15.269Z
blocker_discovered: false
---

# S03: Backend gap map

**Added a fail-closed backend gap map to the canonical Mesher client route inventory so backend maintainers can trace each shipped client promise to an existing seam, a partial seam, or a missing route family without re-auditing the dashboard shell.**

## What Happened

S03 extended `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` with a canonical `## Backend gap map` that now covers both sides of the current product reality. For the mixed routes, the map adds stable `route/surface` rows for Issues, Alerts, and Settings and records the exact client promise, the current Mesher seam, one support status (`covered`, `missing-payload`, `missing-controls`, or `no-route-family`), and the remaining backend work. That keeps lifecycle/member/key/rule flows marked as truly covered, while overview/detail surfaces that still depend on derived or fallback data stay explicitly `missing-payload`, and visible controls that outrun current writes stay `missing-controls` or `no-route-family` instead of being overstated as live.

The slice also closed the gap for the remaining mock-only dashboard routes. Performance, Solana Programs, Releases, Bounties, and Treasury now each have dedicated backend-gap sections at route or major-subsection scope so maintainers can see, without reopening the UI code, that those route families still have no same-origin Mesher seam today and which backend family would need to exist before those pages can become truthful. The map stays redaction-safe by pointing only at route families, code anchors, and client promises rather than copying payloads, ids, or secrets.

To keep the inventory actionable rather than prose-only, S03 extended `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs` and `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` with document-level backend-gap parsing layered on top of the existing S01/S02 contract. `readRouteInventory()` and `parseRouteInventoryMarkdown()` still return the stable top-level rows for existing callers, while the document helpers expose ordered backend-gap sections and rows for verification. The proof rail now fails closed on missing backend-gap headings, out-of-order sections, duplicate keys, blank seam/work cells, unsupported support statuses, and row-set drift with exact route/surface-localized errors. During closeout I also recorded D534 for the support-status vocabulary, added a knowledge note so future edits preserve the `missing-payload` vs `missing-controls` vs `no-route-family` distinctions, refreshed `.gsd/PROJECT.md`, and validated requirement R169 based on the passing contract rail and markdown evidence.

Assumption carried through this slice: the backend gap map remains a maintained canonical document rather than a fully generated report, so future backend slices must update the inventory markdown and the parser/test expectations together when new seams land.

## Verification

Passed both slice-plan verification checks and confirmed the intended diagnostic surface remains actionable. `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` passed all 11 contract tests, including the backend-gap malformed-section, duplicate-row, blank-cell, row-drift, and section-order cases that prove failures name the exact drifting section or route/surface row. The markdown presence checks also passed: one confirmed the required mixed-route backend-gap keys (`issues/overview`, `issues/live-actions`, `alerts/detail`, `alerts/live-actions`, `settings/general`, `settings/team`, `settings/api-keys`, `settings/alert-rules`, `settings/alert-channels`), and another confirmed the mock-only route-family rows (`performance`, `solana-programs`, `releases`, `bounties`, `treasury`) plus the stable support-status vocabulary (`covered`, `missing-payload`, `missing-controls`, `no-route-family`). No runtime or browser verification was required because this slice changes the canonical documentation and structural proof rail rather than interactive behavior.

## Requirements Advanced

- R169 — Mapped mixed-route and mock-only client promises to existing seams or explicit missing seams in the canonical backend gap map, backed by the fail-closed parser/test contract.

## Requirements Validated

- R169 — `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` passed all 11 contract tests, and markdown presence checks confirmed the required backend-gap rows and stable support-status vocabulary in `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None.

## Known Limitations

The backend gap map is still a maintained canonical document rather than a fully generated inventory; future backend expansion work must update both the markdown and the parser/test expectations together when new seams land.

## Follow-ups

S04 should package the now-complete route inventory, mixed-surface map, and backend gap map into the final maintainer handoff and close the milestone with validation evidence that points backend expansion work at the canonical proof rail.

## Files Created/Modified

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` — Added the canonical backend gap map covering mixed Issues/Alerts/Settings surfaces and all remaining mock-only route families with stable support statuses and remaining backend work.
- `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs` — Extended the document parser with ordered backend-gap section parsing, stable status validation, duplicate/blank-cell rejection, and preserved top-level wrapper APIs.
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` — Locked exact backend-gap row parity, allowed support statuses, section order, and fail-closed drift diagnostics.
- `.gsd/DECISIONS.md` — Recorded D534 for the backend-gap support-status vocabulary used by the canonical inventory.
- `.gsd/KNOWLEDGE.md` — Added a maintainer note explaining how to classify backend gaps without overstating partially live surfaces as covered.
- `.gsd/PROJECT.md` — Refreshed current project state to mark M061/S03 complete and leave S04 as the remaining milestone closeout work.
