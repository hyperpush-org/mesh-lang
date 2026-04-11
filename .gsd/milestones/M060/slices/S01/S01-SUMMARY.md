---
id: S01
parent: M060
milestone: M060
provides:
  - A proven same-origin `/api/v1` transport seam for `mesher/client` in dev and built production
  - A typed Mesher read client plus live-overlay adapters for shell-first backend wiring
  - A deterministic seeded live-issue helper for reproducible dashboard verification
  - A mounted toast-backed failure pattern and runtime-signal Playwright harness for future live-route slices
requires:
  []
affects:
  - S02
  - S03
  - S04
key_files:
  - mesher/client/lib/mesher-api.ts
  - mesher/client/lib/issues-live-adapter.ts
  - mesher/client/components/dashboard/dashboard-issues-state.tsx
  - mesher/client/components/dashboard/issue-detail.tsx
  - mesher/client/components/dashboard/issues-page.tsx
  - mesher/client/tests/e2e/issues-live-read.spec.ts
  - mesher/client/src/routes/__root.tsx
  - mesher/scripts/seed-live-issue.sh
  - .gsd/KNOWLEDGE.md
  - .gsd/PROJECT.md
key_decisions:
  - D512 — shared MESHER_BACKEND_ORIGIN resolution across dev, prod bridge, and Playwright
  - D513 — reuse the existing Radix toast surface for selected-issue live read failures
  - D514 — make the deterministic seed helper reuse a running backend when DATABASE_URL is absent
patterns_established:
  - Same-origin `/api/v1` browser transport with runtime-specific proxying hidden behind one shared backend-origin resolver
  - Typed live/mock overlay adapters that normalize Mesher payloads into the richer dashboard shell contract
  - Provider-owned `data-*` observability attributes for runtime-state verification without React internals
  - Reuse of the mounted toast stack for truthful live-read failure feedback instead of a new inline error UX
  - Boundary decoding of Mesher JSONB-backed event-detail fields before adapter validation
observability_surfaces:
  - Health signal — `issues-shell` and detail-panel `data-*` attributes show bootstrap/detail state and source transitions (`ready`, `failed`, `mixed`, `fallback`).
  - Failure signal — request/console tracking plus the visible Radix toast region expose bootstrap/detail failures without silent fallback.
  - Recovery procedure — restore backend availability or remove the failing route override, reload the page, and re-select the issue; the provider retries the live reads and rehydrates the detail panel.
  - Monitoring gaps — no persistent production telemetry or server-side alerting was added in this slice; verification still relies on browser/runtime signals and local Mesher smoke responses.
drill_down_paths:
  - .gsd/milestones/M060/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M060/slices/S01/tasks/T02-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-11T21:12:30.188Z
blocker_discovered: false
---

# S01: Seeded real context and issues/events live read seam

**Connected the canonical `mesher/client` Issues shell to the seeded default Mesher backend through same-origin `/api/v1` reads, keeping the current shell intact via explicit live/mock overlays and visible toast-backed failure feedback.**

## What Happened

S01 delivered the first truthful live backend seam for the canonical Mesher dashboard package without widening into new auth UX or shell redesign. The slice introduced shared same-origin `/api/v1` transport across Vite dev, the built production bridge, and Playwright; a typed Mesher read client plus live-overlay adapters; and provider-owned Issues bootstrap/detail state that merges seeded backend truth into the richer existing shell while leaving unsupported fields visibly mocked.

On the overview path, the Issues route now boots against the seeded `default` project context and renders live-backed issue rows, stats, and chart data with explicit source attribution (`live`, `mixed`, `fallback`) rather than silently treating the mock shell as truth. On the selected-issue path, the dashboard now follows the seeded issue through latest-event lookup, event detail, and timeline reads, keeps the existing detail shell mounted during loading/failure, and exposes backend read regressions through the mounted Radix toast stack plus stable `data-*` observability attributes.

Closeout required one focused root-cause fix beyond the original task implementation: Mesher event-detail payloads can return JSONB-backed fields such as `tags`, `extra`, and `user_context` as JSON strings. The live adapter was initially validating those fields as already-decoded objects, which forced the happy-path detail seam into `invalid-payload` fallback. The fix decodes JSON-encoded event-detail fields before adapter validation. The slice also tightened the Playwright proof to assert the actual preserved fallback shell surfaces and the visible toast region instead of ambiguous broad-text locators.

This slice establishes the pattern downstream slices should reuse: keep browser traffic same-origin, adapt backend payloads through typed overlay seams instead of weakening the UI contract, preserve the current shell visually, and make live-read failures obvious through existing feedback surfaces. S02/S03 can now build on a truthful project/org/API-key context plus a proven Issues read seam instead of reopening transport, selection, or failure-visibility groundwork.

## Verification

Passed all slice-plan verification rails after fixing the selected-issue detail decoding seam and the overly broad browser assertions.

1. `DATABASE_URL` was loaded from the running local Mesher process without printing it, `MESHER_MESHC_BIN` was pointed at the existing sibling `../hyperpush-mono/target/debug/meshc` binary to satisfy the split-workspace toolchain contract, and `bash mesher/scripts/migrate.sh up && bash mesher/scripts/smoke.sh` then passed. Result: no pending migrations, Mesher rebuilt successfully, `/api/v1/projects/default/settings` returned the seeded default project, and `/api/v1/projects/default/storage` returned event/storage metrics.
2. `bash mesher/scripts/seed-live-issue.sh` passed. Result: deterministic seeded issue/event readback succeeded against the running local Mesher backend, including issue lookup, latest-event lookup, event detail, and timeline confirmation.
3. `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live read seam"` passed. Result: all 5 dev Playwright cases passed, including seeded happy path, bootstrap failure fallback, sparse-detail fallback preservation, selected-issue failure toast visibility, and unknown-payload normalization.
4. `npm --prefix mesher/client run test:e2e:prod -- --grep "issues live read seam"` passed. Result: the same 5 cases passed against the built production runtime.

Observability/diagnostics confirmed during verification:
- request tracking only saw same-origin `/api/v1` reads; no browser traffic went directly to `:8080` or `:18080`
- console-error assertions stayed clean outside the intentionally induced 500-path allowances
- `issues-shell`, stats, chart, and detail panel `data-*` attributes reflected bootstrap/detail source and state transitions truthfully
- the mounted toast region showed visible destructive feedback for selected-issue timeline failures instead of silent fallback

## Requirements Advanced

- R155 — Booted the dashboard against the seeded default project/org/API-key reality through same-origin `/api/v1` reads without adding auth UX.
- R156 — Kept the existing Issues shell materially intact while overlaying live list/stats/chart/detail/timeline data and leaving unsupported fields visibly mocked.
- R158 — Mounted the existing Radix toaster and routed backend-backed selected-issue failures through visible destructive toasts.

## Requirements Validated

- R155 — `bash mesher/scripts/seed-live-issue.sh` plus passing dev/prod `issues live read seam` Playwright runs proved seeded default-context boot through same-origin `/api/v1` reads.
- R156 — Passing dev/prod happy-path and sparse-detail Playwright cases proved the existing shell stayed intact while live data overlaid fallback-only fields.
- R158 — Passing dev/prod selected-issue failure Playwright cases proved backend read failures surface through the mounted Radix toast region.

## New Requirements Surfaced

- None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

Task T02 originally stopped with the selected-issue happy path still red. Slice closeout added one focused repair to decode JSON-encoded Mesher event-detail fields before adapter validation and tightened the Playwright assertions to target the actual preserved fallback shell surfaces and notification region. Verification also required setting `MESHER_MESHC_BIN`/`MESHER_MESHC_SOURCE` to an existing sibling `meshc` binary because the split-workspace toolchain contract did not find `target/debug/meshc` under the current repo root.

## Known Limitations

Only the Issues route is live in this slice. Later slices still need to wire the remaining dashboard sections, actions, and admin/ops surfaces to the backend. The shell intentionally remains mixed live/mock where the backend does not yet expose equivalent fields.

## Follow-ups

S02 should build on the now-stable same-origin transport, state-provider seam, and toast-backed failure pattern to make issue actions and summary surfaces live. S03 should reuse the same overlay/failure-visibility pattern for admin and ops routes instead of inventing a separate transport or error UX. If Mesher broadens event-detail payload variants further, keep decoding/normalization at the client boundary before UI adapters consume the data.

## Files Created/Modified

- `mesher/client/lib/mesher-api.ts` — Decoded JSON-encoded event-detail fields at the client boundary and kept typed Mesher read helpers for default-project/issues/event/timeline reads.
- `mesher/client/lib/issues-live-adapter.ts` — Continued the live/mock normalization seam that maps Mesher issue/detail/timeline payloads into the existing dashboard shell contract.
- `mesher/client/components/dashboard/dashboard-issues-state.tsx` — Owns overview bootstrap, selected-issue live hydration, source/state tracking, and toast-backed failure handling.
- `mesher/client/components/dashboard/issue-detail.tsx` — Shows live/fallback status banners, recent live timeline entries, and preserved fallback shell sections in the detail panel.
- `mesher/client/components/dashboard/issues-page.tsx` — Publishes stable `issues-shell` and detail-panel observability attributes used by verification and downstream slices.
- `mesher/client/src/routes/__root.tsx` — Mounts the shared Radix toaster at the app root so backend-backed failure feedback is visible.
- `mesher/client/tests/e2e/issues-live-read.spec.ts` — Provides the canonical dev/prod live-seam proof for happy-path hydration, sparse fallback preservation, failure toasts, and payload normalization.
- `mesher/scripts/seed-live-issue.sh` — Seeds and verifies a deterministic live issue/event/timeline against the running Mesher backend, reusing an existing backend when available.
- `.gsd/KNOWLEDGE.md` — Recorded the JSONB-string event-detail gotcha and the split-workspace `meshc` override needed for Mesher script verification.
- `.gsd/PROJECT.md` — Refreshed project state to reflect that M060/S01 is complete and the first live Mesher client read seam is now proven.
