---
id: S02
parent: M061
milestone: M061
provides:
  - A canonical fine-grained truth inventory for Issues, Alerts, and Settings that downstream backend planning can cite directly.
  - A fail-closed parser/test rail that names exact mixed-surface drift.
  - A self-contained dev Playwright proof rail that covers live, mixed, shell-only, and mock-only behavior without external issue seeding.
requires:
  - slice: S01
    provides: The canonical top-level route inventory, route-map parity contract, and retained route-inventory verifier that S02 expanded to mixed surfaces.
affects:
  - S03
  - S04
key_files:
  - ../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md
  - ../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs
  - ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs
  - ../hyperpush-mono/mesher/client/tests/e2e/seeded-live-issue.ts
  - ../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts
  - ../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts
  - ../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts
  - ../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts
  - ../hyperpush-mono/mesher/client/hooks/use-toast.ts
key_decisions:
  - D528 — keep the canonical mixed-surface audit in ROUTE-INVENTORY.md with stable surface keys and normalized classifications
  - D529 — expose mixed-surface rows through document-level parser helpers while preserving the S01 top-level API
  - D530 — seed deterministic issue proof rows inside the Playwright suites through same-origin APIs instead of depending on an external shell pre-step
patterns_established:
  - Represent mixed route truth as stable surface-key rows in the canonical maintainer inventory, then fail closed on parser/test drift.
  - Keep runtime `fallback` as a diagnostic condition rather than promoting it to a canonical support classification.
  - When Playwright boots its own temporary backend, seed deterministic proof entities from inside the suites through the same-origin API contract.
  - Fix shared runtime-warning roots in common infrastructure (`@/hooks/use-toast`) instead of weakening explicit console-clean proof bars.
observability_surfaces:
  - `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` mixed-surface tables
  - `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` structural contract
  - Issues/Alerts/Settings `data-source`, `data-state`, status banners, mock-only banners, and action-error selectors
  - Mounted destructive toast path used by live failure assertions
drill_down_paths:
  - .gsd/milestones/M061/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M061/slices/S02/tasks/T02-SUMMARY.md
  - .gsd/milestones/M061/slices/S02/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-12T08:10:27.172Z
blocker_discovered: false
---

# S02: Mixed-surface audit

**Expanded the canonical Mesher client inventory into fail-closed Issues/Alerts/Settings mixed-surface tables, hardened the parser/proof rail, and made the issue proof suites self-seeding and stable by fixing the shared toast listener leak.**

## What Happened

## What this slice delivered

S02 turned the top-level route inventory from S01 into a fine-grained mixed-surface contract for the three routes that still blend live backend behavior with shell-only chrome: **Issues**, **Alerts**, and **Settings**.

### T01 — Canonical mixed-surface tables
`../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` now contains structured markdown tables under `### Issues`, `### Alerts`, and `### Settings` instead of prose bullets. Each row carries a durable surface key, level, normalized classification, code evidence, proof evidence, live seam summary, and boundary note. The tables now answer the slice demo directly: maintainers can see which panels/controls are `live`, `mixed`, `shell-only`, or `mock-only` without inferring from route-level labels.

### T02 — Fail-closed parser and contract enforcement
`../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs` now parses the mixed-surface tables as document-level sections while preserving the S01 top-level route-map API. `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` now rejects missing sections, duplicate rows, blank evidence cells, unsupported classifications, unrecognized proof references, and section/surface drift with exact `routeSection/surfaceKey` errors. This keeps the maintainer document honest instead of allowing drift back to prose.

### T03 — Runtime proof at row granularity
The existing Playwright suites now explicitly cover the fine-grained rows cited by the inventory, including issue proof-harness diagnostics, alert shell-only controls, settings mixed/mock-only markers, and failure-path diagnostics. During closeout, the final blocker was not documentation drift but proof determinism: the Issues suites still assumed an external `seed-live-issue.sh` pre-step while the Playwright harness booted a fresh temporary Mesher backend. I fixed that by adding `../hyperpush-mono/mesher/client/tests/e2e/seeded-live-issue.ts`, which seeds the read/action issue rows through same-origin `/api/v1/events` and reopens them through `/api/v1/issues/:id/unresolve` inside the suites. The same closeout rail also exposed the remaining real runtime warning path: the live dashboard imports the shared toast store from `@/hooks/use-toast`, and that hook was still re-subscribing listeners on every state change. Fixing `../hyperpush-mono/mesher/client/hooks/use-toast.ts` to keep the listener effect stable (`[]`) removed the intermittent React warning and restored a green console-clean proof bar.

## Patterns established

- **Canonical maintainer doc + fail-closed contract test + runtime proof** is now the pattern for mixed client truth work.
- **Stable surface keys** are the durable contract; transient runtime `fallback` remains diagnostic state, not an inventory classification.
- **Same-origin in-suite seeding** is the right proof pattern when Playwright boots its own temporary backend.
- **Shared error surfaces matter**: the toast store is a cross-route proof dependency, so root-cause fixes belong in the shared hook instead of weakening assertions.

## Operational Readiness

- **Health signal:** `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` passes; the dev Playwright rail passes 21/21; UI diagnostics stay truthful through `data-source`, `data-state`, `*-status-banner`, `*-mock-only-banner`, `*-action-error`, and destructive toast surfaces.
- **Failure signal:** route-inventory drift now fails with named `routeSection/surfaceKey` errors; runtime regressions surface as explicit selector/test failures or console-error assertions instead of screenshot-only mismatch.
- **Recovery procedure:** rerun the structural verifier first, then rerun the exact Playwright subset that failed; if the issue surfaces fail on missing seeded rows, keep the self-seeding helper in the suite rather than relying on an external seed script; if failure paths emit the React mounted-state warning again, inspect `../hyperpush-mono/mesher/client/hooks/use-toast.ts` before changing route providers.
- **Monitoring gaps:** the proof rail is still suite-level rather than per-row generated, so S04 can still improve milestone-level handoff and proof compression; S03 still needs the backend gap map that turns these truthful client boundaries into backend expansion slices.


## Verification

- `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` ✅ pass (9/9 tests)
- `env PLAYWRIGHT_PROJECT=dev npm --prefix ../hyperpush-mono/mesher/client exec -- playwright test --config ../hyperpush-mono/mesher/client/playwright.config.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts` ✅ pass (21/21 tests)

Closeout note: the final green run required two root-cause fixes discovered during slice completion — self-contained issue seeding in `tests/e2e/seeded-live-issue.ts` and the stable-listener fix in `hooks/use-toast.ts` — after initial replay showed missing seeded issue rows on fresh backend boots and an intermittent React mounted-state warning on the live mutation/toast path.

## Requirements Advanced

- R170 — Added fine-grained proof citations, fail-closed mixed-surface parsing, and runtime assertions that materially advance the repeatable proof-rail requirement ahead of the final S04 handoff.

## Requirements Validated

- R168 — Validated by the mixed-surface tables in `ROUTE-INVENTORY.md`, the passing route-inventory parser test, and the passing dev Playwright suite for Issues/Alerts/Settings.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

The slice plan only called for fine-grained proof additions, but closeout uncovered two real cross-cutting proof blockers: missing deterministic issue rows on fresh Playwright backend boots and the remaining shared toast-store listener leak. Both were fixed at the root cause rather than weakening the proof bar.

## Known Limitations

None.

## Follow-ups

S03 still needs to turn the now-truthful client inventory into a backend gap map, and S04 still needs the final maintainer handoff and milestone-level proof compression.

## Files Created/Modified

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` — Replaced mixed-route prose with structured Issues/Alerts/Settings tables keyed by stable surface ids.
- `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs` — Added document-level mixed-surface parsing while keeping top-level route readers stable.
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` — Locked mixed-surface contracts and exact section/surface drift failures.
- `../hyperpush-mono/mesher/client/tests/e2e/seeded-live-issue.ts` — Added self-contained same-origin seeding helper for deterministic issue proof rows.
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts` — Switched read-seam proof to the in-suite deterministic issue helper.
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts` — Switched action-seam proof to the in-suite deterministic issue helper.
- `../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts` — Switched walkthrough issue checks to the in-suite deterministic issue helper.
- `../hyperpush-mono/mesher/client/hooks/use-toast.ts` — Fixed the shared toast store to keep listener subscription stable and eliminate the React mounted-state warning on failure-path proofs.
- `.gsd/KNOWLEDGE.md` — Recorded the self-contained seeding pattern and the authoritative toast-hook location for future agents.
- `.gsd/PROJECT.md` — Refreshed project state to reflect S02 completion and the current M061 status.
