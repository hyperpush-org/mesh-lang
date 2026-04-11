---
id: T02
parent: S02
milestone: M060
key_files:
  - mesher/client/components/dashboard/issue-detail.tsx
  - mesher/client/components/dashboard/issue-list.tsx
  - mesher/client/components/dashboard/stats-bar.tsx
  - mesher/client/components/dashboard/issues-page.tsx
  - mesher/client/components/dashboard/dashboard-issues-state.tsx
  - mesher/client/lib/issues-live-adapter.ts
  - mesher/client/tests/e2e/issues-live-actions.spec.ts
  - mesher/client/tests/e2e/issues-live-read.spec.ts
  - .gsd/KNOWLEDGE.md
  - .gsd/DECISIONS.md
key_decisions:
  - Expose per-card Issues summary source markers (`live`, `derived live`, `fallback`) instead of relying on a single global stats-source label.
  - Keep only unsupported/unknown validation in the retained proof rail once the real detail-row actions are live, so the shell stays honest about what S02 actually supports.
duration: 
verification_result: passed
completed_at: 2026-04-11T22:28:17.111Z
blocker_discovered: false
---

# T02: Moved live Issues maintainer actions into the real detail row and made list and summary status chrome report backend-backed truth.

**Moved live Issues maintainer actions into the real detail row and made list and summary status chrome report backend-backed truth.**

## What Happened

I replaced the temporary supported-action proof buttons with real maintainer controls in `mesher/client/components/dashboard/issue-detail.tsx`, wiring only the supported same-origin actions (`Resolve`, `Reopen`, `Ignore`) into the existing detail action row with busy labels, disabled state during writes, and inline failure visibility that preserves the prior visible issue state. I kept the existing shell helpers visible but explicitly marked them as shell-only so the UI no longer overclaims live support. I then tightened the live/fallback adapter and shell surfaces: `mesher/client/lib/issues-live-adapter.ts` now carries per-card summary source metadata, `mesher/client/components/dashboard/stats-bar.tsx` exposes truthful `live` / `derived live` / `fallback` markers on each summary card, and `mesher/client/components/dashboard/issue-list.tsx` now shows explicit live-backed status badges for open/resolved/ignored/regressed/in-progress rows plus row-level status diagnostics. In `mesher/client/components/dashboard/issues-page.tsx` and `mesher/client/components/dashboard/dashboard-issues-state.tsx` I passed the provider-owned mutation phase/error/source signals into the real detail controls, kept the reduced proof rail only for unsupported/unknown validation paths, and aligned the user-facing action vocabulary to `Reopen`. Finally, I rewrote `mesher/client/tests/e2e/issues-live-actions.spec.ts` to drive the real detail-row controls, assert filtered-list transitions and summary-source markers, and retained the proof rail only for negative-path validation. I also updated `mesher/client/tests/e2e/issues-live-read.spec.ts` so the prod suite asserts stable provider signals (`data-latest-event-id` and per-card summary sources) instead of brittle exact-event-id assumptions under parallel workers.

## Verification

Verified the client still builds, the deterministic live issue seed helper still produces a reachable live issue/event/timeline, the targeted dev Playwright suite for `issues live actions` passes against the real same-origin mutation seam, and the full slice-level prod `issues live` suite passes after updating the read-side expectation to the new truthful mixed-summary behavior. The browser proofs exercised real detail-row controls, confirmed same-origin `/api/v1/issues/:id/{resolve,unresolve,archive}` traffic, checked disabled/busy control state during writes, verified filter transitions and refreshed row badges after mutations, and confirmed per-card summary source markers remain honest across both happy-path and failure-path flows.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix mesher/client run build` | 0 | ✅ pass | 8800ms |
| 2 | `bash mesher/scripts/seed-live-issue.sh` | 0 | ✅ pass | 2800ms |
| 3 | `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live actions"` | 0 | ✅ pass | 34200ms |
| 4 | `npm --prefix mesher/client run test:e2e:prod -- --grep "issues live"` | 0 | ✅ pass | 25200ms |

## Deviations

Kept a reduced proof rail in `mesher/client/components/dashboard/issues-page.tsx` for unsupported-action and unknown-issue provider validation after moving the supported controls into the real issue-detail action row. This preserves the negative-path proof surface without presenting those buttons as live maintainer actions.

## Known Issues

None.

## Files Created/Modified

- `mesher/client/components/dashboard/issue-detail.tsx`
- `mesher/client/components/dashboard/issue-list.tsx`
- `mesher/client/components/dashboard/stats-bar.tsx`
- `mesher/client/components/dashboard/issues-page.tsx`
- `mesher/client/components/dashboard/dashboard-issues-state.tsx`
- `mesher/client/lib/issues-live-adapter.ts`
- `mesher/client/tests/e2e/issues-live-actions.spec.ts`
- `mesher/client/tests/e2e/issues-live-read.spec.ts`
- `.gsd/KNOWLEDGE.md`
- `.gsd/DECISIONS.md`
