---
id: T02
parent: S02
milestone: M059
key_files:
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-issues-state.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/issues-page.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-shell.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/header.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-list.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-detail.tsx
  - ../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts
  - .gsd/milestones/M059/slices/S02/tasks/T02-SUMMARY.md
key_decisions:
  - D499 — Keep Issues state in a shell-owned provider and expose explicit shell-state attributes for route-parity verification.
duration: 
verification_result: passed
completed_at: 2026-04-11T16:18:09.242Z
blocker_discovered: false
---

# T02: Extracted the Issues dashboard into a route-ready page with shell-owned persistent state and parity coverage for search/filter/detail behavior.

**Extracted the Issues dashboard into a route-ready page with shell-owned persistent state and parity coverage for search/filter/detail behavior.**

## What Happened

I extracted the Issues runtime into a dedicated `components/dashboard/issues-page.tsx` leaf, moved Issues search/status/severity/selected-detail ownership into a new shell-owned `DashboardIssuesStateProvider`, and updated `dashboard-shell.tsx` to compose that extracted page instead of inline Issues JSX. The new provider normalizes invalid filter values back to `all`, treats unknown issue ids as no-detail-panel instead of a crash, and keeps Issues state alive across shell navigation so leaving and returning to Issues preserves the active search, filters, and selected issue. I also added stable Issues test and inspection hooks to the extracted page and extended the Playwright parity suite with explicit Issues interaction coverage for filter/search changes, detail-panel toggles, and leave-and-return persistence; the same flow was exercised live in the browser with zero console errors and zero failed requests.

## Verification

Verified the change with `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build`, a focused Playwright Issues interaction run using the corrected sibling-app `npm exec --` form, and the full current dev parity suite, all passing. I also exercised the live dev app in the browser by searching for HPX-1039, applying regressed + critical filters, opening the detail panel, navigating to Performance and back, and confirming state persistence with no console errors or failed requests.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build` | 0 | ✅ pass | 4500ms |
| 2 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp exec -- playwright test ../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts --config ../hyperpush-mono/mesher/frontend-exp/playwright.config.ts --project=dev --grep "issues interactions"` | 0 | ✅ pass | 14200ms |
| 3 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp exec -- playwright test ../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts --config ../hyperpush-mono/mesher/frontend-exp/playwright.config.ts --project=dev` | 0 | ✅ pass | 16700ms |

## Deviations

Did not need to modify src/routes/index.tsx or app/page.tsx because T01 had already moved the active runtime onto DashboardShell; the extraction stayed inside the shell-owned Issues path. Playwright verification also needed the sibling-app explicit `npm exec -- ... --config ../hyperpush-mono/.../playwright.config.ts` form because auto-mode runs from the mesh-lang repo root.

## Known Issues

Slice-level production parity remains pending later S02 work; the current Playwright config still defines only the dev project even though the active Issues route path is now green in build, dev parity tests, and live browser verification.

## Files Created/Modified

- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-issues-state.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/issues-page.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-shell.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/header.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-list.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/issue-detail.tsx`
- `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts`
- `.gsd/milestones/M059/slices/S02/tasks/T02-SUMMARY.md`
