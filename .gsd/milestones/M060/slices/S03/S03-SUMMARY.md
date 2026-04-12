---
id: S03
parent: M060
milestone: M060
provides:
  - A truthful Alerts route with live fired-alert lifecycle actions.
  - A truthful Settings shell for General/storage, API keys, alert rules, and Team while unsupported areas remain visibly non-live.
  - Org-slug-backed Team routing through `/api/v1/orgs/default/*` without hardcoded org UUIDs.
  - A deterministic admin/ops seed + dev/prod proof rail that downstream S04 can reuse for full-shell assembly.
requires:
  - slice: S01
    provides: Same-origin `/api/v1` client seam, toast mounting, and live-shell overlay patterns reused by S03 state providers.
  - slice: S02
    provides: Proof style and same-origin dashboard mutation patterns reused for the broader admin/ops Playwright rail.
affects:
  - S04
key_files:
  - mesher/client/lib/admin-ops-live-adapter.ts
  - mesher/client/components/dashboard/alerts-live-state.tsx
  - mesher/client/components/dashboard/settings/settings-live-state.tsx
  - mesher/client/components/dashboard/settings/settings-page.tsx
  - mesher/client/lib/mesher-api.ts
  - mesher/api/team.mpl
  - mesher/api/helpers.mpl
  - mesher/storage/queries.mpl
  - mesher/scripts/seed-live-admin-ops.sh
  - mesher/client/tests/e2e/admin-ops-live.spec.ts
key_decisions:
  - D518 — Use subsection-scoped live state for backend-backed Settings areas and remove the page-wide fake save affordance.
  - D519 — Keep the admin/ops seed helper database-first and deterministic instead of reusing whichever Mesher runtime is already listening on the default port.
patterns_established:
  - Typed admin/ops payload normalization between Mesher backend responses and the existing dashboard shell contract.
  - Slice-owned live state providers that perform read-after-write refresh and expose stable `data-*` markers for verification.
  - DB-first deterministic seeding plus dev/prod same-origin Playwright proof as the closeout rail for product-backed dashboard slices.
observability_surfaces:
  - `alerts-shell` and `settings-shell` `data-*` bootstrap/source/mutation markers
  - Subsection source badges for General, Team, API Keys, Alert Rules, and Alerts detail
  - Mounted Radix toast notification region for destructive read/write failures
  - Playwright same-origin request tracking in `mesher/client/tests/e2e/admin-ops-live.spec.ts`
  - Redacted deterministic readback from `bash mesher/scripts/seed-live-admin-ops.sh`
drill_down_paths:
  - .gsd/milestones/M060/slices/S03/tasks/T01-SUMMARY.md
  - .gsd/milestones/M060/slices/S03/tasks/T02-SUMMARY.md
  - .gsd/milestones/M060/slices/S03/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-12T00:39:26.786Z
blocker_discovered: false
---

# S03: Admin and ops surfaces live

**Wired Alerts, Settings admin/ops subsections, Team, API keys, and alert rules to real Mesher routes through the same-origin client seam, added deterministic DB-first admin/ops seeding, and closed the slice with passing dev/prod Playwright proof.**

## What Happened

S03 completed the admin and ops live-backend seam without redesigning the dashboard shell. On the client side, the slice introduced typed admin/ops payload adapters plus slice-owned live state for Alerts and Settings so the existing shell can consume real Mesher data while still keeping unsupported chrome visibly present and explicitly non-live. The Alerts route now boots through same-origin `/api/v1` reads, exposes truthful live/fallback source markers, and supports the existing fired-alert lifecycle actions the backend already owns. The Settings shell now treats General/storage, API keys, alert rules, and Team as separate truthful subsections instead of pretending the whole page is globally writable. General writes real retention/sample-rate values, storage metrics are live, API keys list/create/revoke through the backend, alert rules list/create/toggle/delete through the backend, and the Team tab resolves the seeded `default` org slug and supports real member list/add/role/remove behavior through the same-origin seam without hardcoded UUID routing. On the backend side, the slice added org-slug resolution plus a safer API-key response serialization path so the product UI receives real string values instead of runtime slot identifiers. The deterministic seed helper was finalized as a DB-first proof rail: it seeds canonical Postgres rows for settings, team fixtures, API keys, alert rules, and alerts directly, prints only redacted readback, and no longer depends on reusing whichever Mesher runtime might already be listening on the default port. Operationally, the slice now exposes explicit `data-*` state/source markers on the alerts and settings shells, keeps subsection-level last-read/last-write failure markers mounted, and uses the existing Radix toast region for destructive failure feedback instead of silent fallback. The remaining milestone work is compositional rather than first-principles admin/ops wiring: S04 still needs the full assembled seeded-shell walkthrough across every currently live route plus any narrow seam repair discovered while exercising the whole dashboard together.

## Verification

Passed the slice-plan verification rail end to end. `bash mesher/scripts/seed-live-admin-ops.sh` now succeeds and returns deterministic redacted seeded-state readback from the canonical Postgres database. `npm --prefix mesher/client run test:e2e:dev -- --grep "admin and ops live"` passed all 9 admin/ops Playwright cases in the dev runtime, covering real alerts acknowledge/resolve, alerts bootstrap and mutation failure handling, truthful empty/malformed live payload behavior, same-origin settings general/team/API-key/alert-rule happy paths, malformed local validation, revoke failure toasts, and zero-storage/empty-list states. `npm --prefix mesher/client run test:e2e:prod -- --grep "admin and ops live"` passed the same 9 cases against the built production runtime. The proof also exercised the observability contract by asserting stable `data-*` state markers, subsection source badges, same-origin request tracking, and mounted notification toasts.

## Requirements Advanced

- R154 — Closed the remaining backend-backed admin/ops action seams by wiring alerts lifecycle actions, settings writes, API-key mutations, alert-rule mutations, and Team mutations end to end through same-origin `/api/v1`.
- R157 — Kept unsupported settings/alerts affordances visibly present and stable while explicitly marking them non-live instead of removing them or implying fake writes.

## Requirements Validated

- R154 — Validated by the passing seeded dev/prod `admin and ops live` Playwright suites plus `bash mesher/scripts/seed-live-admin-ops.sh`, covering same-origin alerts/actions, settings writes, API keys, alert rules, and Team mutations.
- R157 — Validated by the same dev/prod suites asserting unsupported silence/channel and other mock-only controls remain present, shell-stable, and explicitly non-live while live-backed subsections operate truthfully.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

T03 originally stopped short of full verification. Closeout work completed the missing backend fix and seed/proof rail, then reran the full slice verification bar to green. The seed helper also shifted from runtime-dependent HTTP readback to deterministic DB-first seeding and verification so it no longer flakes on unrelated already-running Mesher instances.

## Known Limitations

Unsupported settings areas outside the backend-backed admin/ops subsections remain intentionally mock-only. `DashboardShell` still mounts the Issues provider globally, so `/alerts` can emit expected aborted hidden-Issues bootstrap requests during certain mocked failure paths; the Playwright rail filters that noise, but a future shell-composition cleanup could scope the provider more narrowly. S04 still needs the full assembled live-shell walkthrough across all currently supported dashboard routes.

## Follow-ups

Run the S04 composed seeded-shell walkthrough across every live route, retire any remaining narrow seam issues discovered during full-shell composition, and consider scoping the global Issues provider more narrowly so non-Issues routes stop producing hidden bootstrap abort noise.

## Files Created/Modified

- `mesher/client/lib/admin-ops-live-adapter.ts` — Normalized alerts, settings, team, API-key, and alert-rule payloads into the existing dashboard shell contract.
- `mesher/client/components/dashboard/alerts-live-state.tsx` — Owns live Alerts bootstrap, selection, lifecycle mutations, refresh, state markers, and failure handling.
- `mesher/client/components/dashboard/settings/settings-live-state.tsx` — Owns subsection-scoped General, Team, API-key, and alert-rule reads/writes plus mutation/error state.
- `mesher/client/components/dashboard/settings/settings-page.tsx` — Replaced fake page-wide save behavior with truthful mixed live/mock admin/ops rendering.
- `mesher/api/helpers.mpl` — Added org-slug resolution support consumed by Team routes.
- `mesher/api/team.mpl` — Updated Team handlers for org slugs and fixed API-key JSON serialization to emit real string fields.
- `mesher/storage/queries.mpl` — Added org-slug lookup and continued admin/ops query support for Team and API-key routes.
- `mesher/scripts/seed-live-admin-ops.sh` — Finalized deterministic DB-first seeding/reset/readback for admin/ops proof state without depending on a pre-running backend.
- `mesher/client/tests/e2e/admin-ops-live.spec.ts` — Closed the dev/prod admin/ops proof rail across Alerts, Settings General/storage, API keys, alert rules, Team, and failure states.
