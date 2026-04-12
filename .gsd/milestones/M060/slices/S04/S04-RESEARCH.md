# M060/S04 — Research

**Date:** 2026-04-11

## Summary

S04 is light research. The live wiring work is already done; the remaining gap is assembled proof. Current end-to-end coverage is split across three rails: `dashboard-route-parity.spec.ts` proves shell/direct-entry behavior for all dashboard routes, `issues-live-read.spec.ts` + `issues-live-actions.spec.ts` prove the live Issues seam, and `admin-ops-live.spec.ts` proves live Alerts + Settings admin/ops subsections. There is **no** single seeded-local walkthrough spec that traverses the full dashboard shell and proves the integrated route-to-route experience in one runtime.

That makes the primary S04 deliverable a proof-assembly slice for R160, with R159 only activating if the composed walkthrough exposes a narrow blocking seam. The existing client already exposes the right observability surfaces for this: `dashboard-shell[data-route-key]`, sidebar `data-testid`s, `issues-shell`, `alerts-shell`, `settings-shell`, and subsection `data-state` / `data-source` markers. The route inventory is already centralized in `mesher/client/components/dashboard/dashboard-route-map.ts`.

One integration constraint is already documented by S03: `DashboardShell` still mounts `DashboardIssuesStateProvider` globally, so non-Issues routes can emit expected aborted hidden-Issues requests during some failure-path tests. For S04, treat that as known noise to filter consistently with the existing specs, not as a reason to redesign shell composition unless the normal seeded walkthrough itself proves a real user-visible break.

## Skills Discovered

- Existing installed skill used: `playwright-best-practices`
- Relevant rules applied from that skill:
  - keep assertions explicit instead of inferring success from navigation alone
  - prefer stable selectors / `data-*` markers over brittle text-only locators
  - keep real-service E2E deterministic with seeded fixtures and route-level helpers
- No new skill installs are needed for this slice. The directly relevant skills (`playwright-best-practices`, `react-best-practices`, `tanstack-router-best-practices`, `vite`) are already available.

## Recommendation

Implement one new Playwright spec, likely `mesher/client/tests/e2e/seeded-walkthrough.spec.ts`, that walks a single browser session through every dashboard route in `dashboard-route-map.ts` while reusing the existing seeded/local proof rails.

Use the already-established Playwright pattern rather than inventing new harness machinery:
- reuse the issue lookup/reset logic from `issues-live-read.spec.ts`
- reuse the same-origin runtime tracking and failure filtering pattern already present in the live specs
- reuse the dynamic live alert creation helper from `admin-ops-live.spec.ts` when the walkthrough needs a guaranteed actionable alert
- keep route navigation deterministic through sidebar `data-testid`s and `dashboard-shell[data-route-key]`

The walkthrough should prove both kinds of truth the milestone cares about:
- live-backed routes stay live and actionable (`/`, `/alerts`, `/settings`)
- mock-only routes remain visually stable and reachable without implying fake backend behavior (`/performance`, `/solana-programs`, `/releases`, `/bounties`, `/treasury`)

If the new spec needs helpers from multiple files, a small shared helper under `mesher/client/tests/e2e/` for runtime tracking and filtered failure assertions is the natural extraction seam. Do not broaden into provider or backend refactors unless the composed walkthrough proves a concrete blocker.

## Implementation Landscape

### Key Files

- `mesher/client/tests/e2e/dashboard-route-parity.spec.ts` — Existing shallow shell coverage for all 8 routes. It already encodes the canonical route list, direct-entry assertions, nav state checks, and shell persistence expectations, but it is not seeded/live proof.
- `mesher/client/tests/e2e/issues-live-read.spec.ts` — Canonical seeded issue lookup/reset helper and same-origin request tracking for the live Issues read path.
- `mesher/client/tests/e2e/issues-live-actions.spec.ts` — Canonical issue mutation proof for Resolve/Reopen/Ignore, including summary-source assertions and post-write refresh truth.
- `mesher/client/tests/e2e/admin-ops-live.spec.ts` — Canonical alerts/settings/team/API key/alert-rule live proof plus same-origin runtime tracking and fresh alert creation helper.
- `mesher/client/components/dashboard/dashboard-route-map.ts` — Authoritative inventory of the 8 dashboard routes. Use this instead of hand-maintaining a second route list for the walkthrough.
- `mesher/client/components/dashboard/dashboard-shell.tsx` — Shell root; exposes `dashboard-shell[data-route-key]`, keeps route-level nav state visible, and still globally mounts `DashboardIssuesStateProvider`.
- `mesher/client/components/dashboard/sidebar.tsx` — Stable sidebar navigation selectors (`sidebar-nav-*`, `sidebar-footer-settings`) for route traversal.
- `mesher/client/components/dashboard/issues-page.tsx` — Exposes `issues-shell` state markers, selected-issue markers, detail panel markers, and proof-rail selectors.
- `mesher/client/components/dashboard/alerts-page.tsx` — Exposes `alerts-shell` markers, live alert count, filter state, and detail-panel selectors for lifecycle actions.
- `mesher/client/components/dashboard/alert-detail.tsx` — Exposes stable alert action selectors and source/status banners.
- `mesher/client/components/dashboard/settings/settings-page.tsx` — Exposes `settings-shell`, subsection `data-state` / `data-source`, and explicit mock-only banners that the walkthrough should preserve.
- `mesher/scripts/seed-live-issue.sh` — Deterministic seed/reset rail for the read/action issue fixtures.
- `mesher/scripts/seed-live-admin-ops.sh` — Deterministic DB-first admin/ops reset for settings/team/API key/alert-rule fixture state.
- `mesher/client/playwright.config.ts` — Already supports dev/prod split runs, same-origin backend proxying, and grep-filtered project execution.
- `mesher/client/README.md` — Still documents only the Issues verification rail; likely needs S04 verification commands added once the walkthrough lands.
- `mesher/main.mpl`, `mesher/api/*.mpl`, `mesher/storage/queries.mpl` — Only touch these if the assembled walkthrough uncovers a real blocking seam. That is the R159 boundary.

### Natural Seams

1. **Proof assembly**
   - Add the new full-shell walkthrough spec.
   - Optionally extract a small E2E helper if the new spec would otherwise duplicate runtime tracking / filtering logic.

2. **Selector / observability patching**
   - If one route lacks a durable anchor for the walkthrough, add a minimal `data-testid` or `data-*` marker in that route component.
   - Prefer this over brittle locator logic.

3. **Narrow seam repair**
   - Only if the composed walkthrough exposes a real bug, patch the exact client/provider/backend seam that fails.
   - Do not widen into new backend routes or shell redesign.

### Build Order

1. **Define the full-shell route inventory from `dashboard-route-map.ts` and existing route parity expectations.**
   - This retires the main S04 ambiguity: what counts as the "full backend-backed shell walkthrough".

2. **Implement the seeded walkthrough spec against existing live selectors and shell markers.**
   - Follow the `playwright-best-practices` guidance already used in this repo: explicit assertions, stable selectors, deterministic seeded fixtures, and same-origin request checks.

3. **Run the composed walkthrough in dev and prod, then patch only the exact failures it reveals.**
   - Missing selectors => tiny UI observability patch.
   - Runtime contract break => local client/provider/backend seam repair.

4. **Update verification docs after the proof rail is stable.**
   - `mesher/client/README.md` is the obvious place.

### Verification

Required S04 verification rail:

- `bash mesher/scripts/seed-live-issue.sh`
- `bash mesher/scripts/seed-live-admin-ops.sh`
- `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live|admin and ops live|seeded walkthrough"`
- `npm --prefix mesher/client run test:e2e:prod -- --grep "issues live|admin and ops live|seeded walkthrough"`

Useful adjunct smoke while iterating:

- `npm --prefix mesher/client run test:e2e:dev -- --grep "dashboard route parity|seeded walkthrough"`
- `npm --prefix mesher/client run test:e2e:prod -- --grep "dashboard route parity|seeded walkthrough"`

### Watch-outs

- `DashboardShell` still mounts `DashboardIssuesStateProvider` globally. Non-Issues routes can produce expected aborted hidden-Issues requests in some failure-path tests; keep failure filtering aligned with the existing live specs.
- `seed-live-admin-ops.sh` is DB-first and does not itself prove runtime reachability; runtime proof still comes from Playwright booting the backend/frontend pair in `playwright.config.ts`.
- `mesher/client/README.md` currently lags the S03/S04 proof shape and should be treated as expected documentation drift, not as the source of truth for current verification.
