---
estimated_steps: 5
estimated_files: 7
skills_used:
  - playwright-best-practices
---

# T01: Add the seeded full-shell walkthrough proof rail

**Slice:** S04 — Full backend-backed shell assembly
**Milestone:** M060

## Description

Close the missing R160 proof gap first. This task adds the canonical assembled browser walkthrough and extracts only the minimum helper needed to keep route inventory, same-origin request tracking, and failure filtering consistent with the existing live suites.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `bash mesher/scripts/seed-live-issue.sh` plus seeded issue APIs | Fail fast with a clear seeded-fixture expectation and do not switch to ad hoc issue selection. | Abort the walkthrough early and surface the issue seed/runtime mismatch in Playwright output. | Reject the fixture and fail rather than guessing fallback issue ids or detail state. |
| `bash mesher/scripts/seed-live-admin-ops.sh` plus seeded alerts/settings/team APIs | Fail fast with deterministic setup expectations and do not fabricate admin/ops fallback proof state. | Stop the walkthrough and surface that the admin/ops runtime is unavailable. | Treat the payload as a contract failure and keep assertions explicit. |
| Same-origin `/api/v1` traffic across all dashboard routes | Record the failing pathname/call, allow only the known hidden-Issues abort noise, and fail on every other 4xx/5xx or direct-backend browser request. | Surface the stalled route/action in helper output instead of adding blind retries. | Fail the test instead of masking malformed live/fallback state markers. |

## Load Profile

- **Shared resources**: one browser session, seeded backend records, the same-origin API proxy, and repeated route transitions reusing global providers.
- **Per-operation cost**: one seeded boot plus route-to-route navigation, issue and alert selection, supported live actions, and settings subsection reads/writes inside a single spec.
- **10x breakpoint**: repeated full walkthroughs will expose stale selected state or helper-level false positives before raw rendering cost becomes the bottleneck, so route inventory and failure filtering must stay centralized.

## Negative Tests

- **Malformed inputs**: unknown route keys, missing seeded issue or alert ids, and unexpected shell `data-source` values.
- **Error paths**: unexpected 4xx/5xx responses, direct browser calls to Mesher backend ports, and hidden-Issues aborts that escape the allowlist.
- **Boundary conditions**: mock-only routes stay reachable without live claims, still-mock-only settings subsections remain visible, and shell navigation preserves route-key truth across direct entry and in-app traversal.

## Steps

1. Extract a small shared helper under `mesher/client/tests/e2e/` for same-origin request tracking, filtered failure assertions, and route-navigation utilities so the new walkthrough reuses the same observability contract as the existing live suites.
2. Create `mesher/client/tests/e2e/seeded-walkthrough.spec.ts` that sources its route inventory from `mesher/client/components/dashboard/dashboard-route-map.ts` and walks a single session through every dashboard route using sidebar and direct-entry markers rather than a duplicated hardcoded list.
3. In the walkthrough, prove live-backed truth on `/`, `/alerts`, and `/settings` by reusing the seeded Issues and admin/ops expectations: same-origin reads, supported actions, subsection `data-state` / `data-source` markers, and truthful post-write refresh behavior.
4. In the same spec, assert that `/performance`, `/solana-programs`, `/releases`, `/bounties`, and `/treasury` remain reachable, shell-stable, and visibly non-live or mock-only where applicable rather than implying fake backend mutations.
5. Keep the spec dev-friendly by filtering only the already-known globally mounted hidden-Issues abort noise and failing on any new console or request regression.

## Must-Haves

- [ ] `mesher/client/tests/e2e/seeded-walkthrough.spec.ts` proves the full route-to-route shell walkthrough in one browser session instead of splitting proof across unrelated specs.
- [ ] The walkthrough derives its route inventory from `mesher/client/components/dashboard/dashboard-route-map.ts`, not a second hand-maintained route list.
- [ ] The browser proof explicitly fails on unexpected 4xx/5xx, direct backend browser traffic, or console errors while allowing only the known hidden-Issues abort noise.
- [ ] Live-backed routes and mock-only routes are both asserted truthfully: live areas must act through same-origin `/api/v1`, while mock-only areas stay visible without fake live claims.

## Verification

- `bash mesher/scripts/seed-live-issue.sh && bash mesher/scripts/seed-live-admin-ops.sh`
- `npm --prefix mesher/client run test:e2e:dev -- --grep "seeded walkthrough|dashboard route parity"`

## Observability Impact

- Signals added/changed: shared same-origin request tracking, filtered failed-request assertions, and route-by-route shell/source verification inside the new walkthrough.
- How a future agent inspects this: run the seeded dev walkthrough grep, then read the helper-reported failing path plus `dashboard-shell`, `issues-shell`, `alerts-shell`, or `settings-shell` markers.
- Failure state exposed: the first unexpected console error, failed request, direct-backend browser call, or route state mismatch in the assembled flow.

## Inputs

- `mesher/client/components/dashboard/dashboard-route-map.ts` — canonical route inventory for all dashboard screens.
- `mesher/client/tests/e2e/dashboard-route-parity.spec.ts` — existing route traversal and shell-state parity coverage to reuse rather than duplicate.
- `mesher/client/tests/e2e/issues-live-read.spec.ts` — seeded Issues read helpers and same-origin runtime tracking patterns.
- `mesher/client/tests/e2e/issues-live-actions.spec.ts` — seeded Issues mutation assertions and filtered request handling.
- `mesher/client/tests/e2e/admin-ops-live.spec.ts` — seeded Alerts and Settings live-proof helpers plus request tracking patterns.
- `mesher/scripts/seed-live-issue.sh` — deterministic issue seed/reset rail the walkthrough must depend on.
- `mesher/scripts/seed-live-admin-ops.sh` — deterministic admin/ops seed/reset rail the walkthrough must depend on.

## Expected Output

- `mesher/client/tests/e2e/live-runtime-helpers.ts` — shared helper for same-origin tracking, filtered failure assertions, and route-level diagnostics.
- `mesher/client/tests/e2e/seeded-walkthrough.spec.ts` — single-session full-shell walkthrough proving both live-backed and mock-only route truth.
