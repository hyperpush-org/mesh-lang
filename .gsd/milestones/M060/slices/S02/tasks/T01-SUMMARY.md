---
id: T01
parent: S02
milestone: M060
key_files:
  - mesher/client/lib/mesher-api.ts
  - mesher/client/components/dashboard/dashboard-issues-state.tsx
  - mesher/client/components/dashboard/issues-page.tsx
  - mesher/client/tests/e2e/issues-live-actions.spec.ts
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Kept all supported issue mutations inside the existing provider-owned same-origin seam instead of introducing component-level fetch calls or route-loader state.
  - Used a temporary proof-only harness in `issues-page.tsx` to exercise real provider mutations now, while leaving the final maintainer control UX to T02 as planned.
  - Treated same-origin `ERR_ABORTED` overview requests as expected evidence of centralized refresh cancellation in the live action Playwright proof rather than as unexpected failures.
duration: 
verification_result: passed
completed_at: 2026-04-11T22:07:39.069Z
blocker_discovered: false
---

# T01: Added provider-owned same-origin issue mutations, action diagnostics, and a Playwright proof harness for live issue actions.

**Added provider-owned same-origin issue mutations, action diagnostics, and a Playwright proof harness for live issue actions.**

## What Happened

I extended `mesher/client/lib/mesher-api.ts` with a shared same-origin POST path for supported issue mutations and kept the existing timeout/network/invalid-payload semantics intact. I then refactored `DashboardIssuesStateProvider` in `mesher/client/components/dashboard/dashboard-issues-state.tsx` so the provider owns mutation phase/error state, centralized overview refresh, selected snapshot invalidation, and post-mutation detail rehydration instead of leaving any component to issue ad hoc fetches. In `mesher/client/components/dashboard/issues-page.tsx` I published stable `data-*` diagnostics for the current mutation phase/action/error and added a temporary proof-only harness so Playwright can exercise the real provider mutation seam before T02 moves the supported controls into the existing detail action row. Finally, I created `mesher/client/tests/e2e/issues-live-actions.spec.ts` to seed live issue state, track same-origin browser traffic, prove successful status refreshes, verify destructive mutation and refresh-failure toasts, and cover unsupported-action / unknown-issue validation paths. During verification I found two non-obvious runtime truths and aligned the implementation/tests to them: the overview route returns live issues across statuses when called without a `status` query, and dev-mode duplicate bootstrap requests mean refresh-failure mocks must only arm after the page is already ready.

## Verification

Verified the package build still succeeds, the seed helper still produces the deterministic live issue/event pair, and the new `issues live actions` dev Playwright suite passes end to end. The Playwright proof exercised real browser-triggered provider mutations, confirmed same-origin `/api/v1/issues/:id/{resolve,unresolve}` traffic, observed refreshed live status text in the list/detail shell, and confirmed destructive toast/error diagnostics for mutation failures, post-write overview refresh failures, unsupported actions, and unknown issue ids. Slice-level prod verification remains for later tasks in S02.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix mesher/client run build` | 0 | ✅ pass | 4152ms |
| 2 | `bash mesher/scripts/seed-live-issue.sh` | 0 | ✅ pass | 849ms |
| 3 | `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live actions"` | 0 | ✅ pass | 23270ms |

## Deviations

Added a temporary proof-only action harness in `mesher/client/components/dashboard/issues-page.tsx` so Playwright can trigger the provider-owned mutation seam before T02 wires the final supported controls into the existing detail action row.

## Known Issues

The proof harness is intentionally temporary and still visible in the issue detail shell until T02 replaces it with the final maintainer action row. `npm --prefix mesher/client run test:e2e:prod -- --grep "issues live"` was not run in this task because T01 is an intermediate slice task; prod suite closure remains for later tasks.

## Files Created/Modified

- `mesher/client/lib/mesher-api.ts`
- `mesher/client/components/dashboard/dashboard-issues-state.tsx`
- `mesher/client/components/dashboard/issues-page.tsx`
- `mesher/client/tests/e2e/issues-live-actions.spec.ts`
- `.gsd/KNOWLEDGE.md`
