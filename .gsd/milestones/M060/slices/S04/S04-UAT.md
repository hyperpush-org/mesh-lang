# S04: Full backend-backed shell assembly — UAT

**Milestone:** M060
**Written:** 2026-04-12T01:49:34.602Z

# S04 UAT — Full backend-backed shell assembly

## Preconditions
1. Local Postgres for Mesher is reachable at the configured `DATABASE_URL`.
2. The repo has a built `meshc` available to Mesher's scripts in this workspace.
3. Seed the canonical local proof state:
   1. Run `bash mesher/scripts/seed-live-issue.sh`.
   2. Run `bash mesher/scripts/seed-live-admin-ops.sh`.
4. No user-specific secrets need to be entered during the flow; do not copy or record raw API-key values.

## Test Case 1 — Canonical route inventory and unknown-path fallback
1. Run `npm --prefix mesher/client run test:e2e:dev -- --grep "seeded walkthrough"`.
   - Expected: the `dashboard route parity derives direct-entry coverage from the canonical route map` test passes.
2. Confirm the test covers direct entry for Issues, Performance, Solana Programs, Releases, Alerts, Bounties, Treasury, and Settings from `dashboard-route-map.ts`.
   - Expected: every route renders `dashboard-shell[data-route-key]` for the matching route key.
3. Confirm the same test covers an unknown path.
   - Expected: `/does-not-exist/deep-link` falls back to the Issues shell with the Issues heading and search input visible.

## Test Case 2 — Live Issues shell continuity
1. Run `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live|seeded walkthrough"`.
   - Expected: the Issues read/action tests and the walkthrough Issues step pass.
2. In the walkthrough evidence, verify the seeded read issue is found and selected.
   - Expected: `issues-shell` reports ready/live state, selected detail hydrates through same-origin `/api/v1/projects/default/issues` and `/api/v1/events/:id`, and detail panels remain mounted even when live detail is sparse.
3. Verify the seeded action issue receives supported maintainer actions.
   - Expected: resolve/reopen/archive flows complete, summary markers remain truthful, and no unsupported issue action is silently treated as live.

## Test Case 3 — Live Alerts route in the assembled shell
1. Run `npm --prefix mesher/client run test:e2e:dev -- --grep "admin and ops live|seeded walkthrough"`.
   - Expected: Alerts tests and walkthrough Alerts step pass.
2. Confirm the walkthrough navigates to Alerts through the sidebar after other routes have already mounted.
   - Expected: the Alerts shell still shows ready/live state, reads from same-origin `/api/v1/projects/default/alerts`, and acknowledge/resolve operations refresh the selected alert truthfully.
3. Exercise a failure-path test from the suite.
   - Expected: malformed payloads or failed mutations surface as contract failures/destructive toasts instead of silently guessing status.

## Test Case 4 — Live Settings/admin operations with mock-only stability
1. Run `npm --prefix mesher/client run test:e2e:dev -- --grep "admin and ops live settings|seeded walkthrough"`.
   - Expected: Settings tests and walkthrough Settings step pass.
2. Verify the General and Storage subsections load and save through same-origin `/api/v1/projects/default/settings` and `/api/v1/projects/default/storage`.
   - Expected: subsection `data-state`/`data-source` markers show live/ready state before and after writes.
3. Verify Team, API key, and alert-rule flows in the same shell session.
   - Expected: Team list/add/role/remove, API key list/create/revoke, and alert-rule list/create/toggle/delete all pass without a page-global fake save affordance.
4. Confirm unsupported Settings subsections stay visible.
   - Expected: mock-only areas still render explicit non-live badges/banners and remain shell-stable.

## Test Case 5 — Full assembled dev and prod rails
1. Run the canonical dev rail:
   - `bash mesher/scripts/seed-live-issue.sh`
   - `bash mesher/scripts/seed-live-admin-ops.sh`
   - `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live|admin and ops live|seeded walkthrough"`
   - Expected: 21/21 tests pass.
2. Re-seed the local state.
   - `bash mesher/scripts/seed-live-issue.sh`
   - `bash mesher/scripts/seed-live-admin-ops.sh`
3. Run the canonical prod rail:
   - `npm --prefix mesher/client run test:e2e:prod -- --grep "issues live|admin and ops live|seeded walkthrough"`
   - Expected: 21/21 tests pass.
4. Review runtime diagnostics from the passing runs.
   - Expected: no unexpected console errors, no unexpected failed requests, no direct browser requests to the backend origin, and only the explicitly filtered hidden-Issues/font abort noise is ignored.

## Edge Cases
1. Sparse issue detail payloads.
   - Expected: stack/breadcrumb tabs stay mounted with explicit shell-backed markers rather than disappearing or inventing fake live content.
2. Unknown route deep links.
   - Expected: they fall back to the Issues shell, not a blank page or a new error UX.
3. Mock-only routes after live navigation.
   - Expected: Performance, Solana Programs, Releases, Bounties, and Treasury remain reachable and shell-stable after visiting live Issues/Alerts/Settings surfaces.
4. Shared seeded runtime contention.
   - Expected: the suite runs serially (`workers=1`) so live route mutations do not race each other and create false negatives.
