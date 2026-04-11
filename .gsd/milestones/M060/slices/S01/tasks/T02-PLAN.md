---
estimated_steps: 4
estimated_files: 8
skills_used:
  - react-best-practices
  - playwright-best-practices
  - bash-scripting
---

# T02: Finish live issue detail, toast failure feedback, and seeded end-to-end proof

**Slice:** S01 — Seeded real context and issues/events live read seam
**Milestone:** M060

## Description

Close the slice on the real user flow: selecting an issue must read truthful live event/detail/timeline data through the unchanged shell, unsupported UI sections must remain visibly present through explicit fallback values, and backend failures must become visible through the existing Radix toast stack. This task also turns the slice into a deterministic proof surface by seeding a known issue/event and asserting the live path in both dev and built-prod modes.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `GET /api/v1/issues/:issue_id/events?limit=1`, `GET /api/v1/events/:event_id`, `GET /api/v1/issues/:issue_id/timeline` | Keep the issue shell rendered, leave unsupported sections on fallback/mock content, and show a Radix toast describing which live read failed. | Preserve the selected issue panel state but expose a toast + loading failure instead of spinning forever. | Reject malformed detail/timeline payloads in the adapter and avoid partially mutating the selected issue model. |
| Mounted Radix toaster surface in `mesher/client/src/routes/__root.tsx` | Treat missing mount/wiring as a regression and fail the live proof because backend read failures would otherwise be silent. | N/A | Reject toast invocations that depend on unavailable provider state or missing shell context. |
| Deterministic live issue seeding helper | Fail verification loudly and preserve the prior shell state; do not hide a broken seed path behind stale backend data. | Abort the proof with a named seed failure rather than letting Playwright assert against unknown backend state. | Refuse malformed seed payloads that would create non-deterministic or untraceable issue rows. |

## Load Profile

- **Shared resources**: selected-issue detail fetch budget, browser runtime state for the issue panel, the deterministic seed-event path on the local Mesher backend, and the toast queue.
- **Per-operation cost**: one seed-event write during verification, then up to three read calls per selected issue (latest-event lookup, event detail, timeline).
- **10x breakpoint**: rapid issue re-selection could flood detail reads first, so scope/cancel stale selections and keep list rendering independent of detail fetch completion.

## Negative Tests

- **Malformed inputs**: timeline entries missing optional fields, detail payloads without stacktrace/breadcrumbs, and sparse user/tag context should still render the existing shell sections with fallback content.
- **Error paths**: selected-issue reads return 404/500, backend becomes unavailable after initial boot, or the seed helper cannot create the deterministic issue.
- **Boundary conditions**: selected issue has zero timeline entries, latest-event lookup returns no events, and intentionally induced backend failure produces a visible toast while the rest of the shell remains interactive.

## Steps

1. Extend `mesher/client/components/dashboard/dashboard-issues-state.tsx` so selecting a live issue fetches latest-event lookup, event detail, and timeline data, merges them into the selected issue model, and preserves existing close/open semantics.
2. Update `mesher/client/components/dashboard/issue-list.tsx`, `mesher/client/components/dashboard/issue-detail.tsx`, and `mesher/client/components/dashboard/issues-page.tsx` so live-backed fields appear wherever Mesher truth exists while unsupported sections remain visibly present through explicit fallback values.
3. Mount `mesher/client/components/ui/toaster.tsx` from `mesher/client/src/routes/__root.tsx` and wire backend read failures through `mesher/client/hooks/use-toast.ts` rather than a new notification system.
4. Add deterministic live verification via `mesher/scripts/seed-live-issue.sh`, expand `mesher/client/tests/e2e/issues-live-read.spec.ts` for detail/failure assertions in dev and prod, and update `mesher/client/README.md` to document the mixed live/mock seam and verification commands.

## Must-Haves

- [ ] Selecting a live issue reads latest event detail and timeline data through the provider instead of inventing a second shell-local state path.
- [ ] `IssueList` and `IssueDetail` keep current UI sections materially intact; when the backend lacks a field, the shell still renders via explicit fallback/mock data rather than removing the section.
- [ ] Backend-backed failures show visible Radix toast feedback from the mounted root toaster.
- [ ] Deterministic live proof exists for both dev and built-prod modes, including a negative-path toast assertion.
- [ ] `mesher/client/README.md` no longer tells maintainers to avoid backend calls for this package.

## Verification

- `bash mesher/scripts/seed-live-issue.sh`
- `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live read seam"`
- `npm --prefix mesher/client run test:e2e:prod -- --grep "issues live read seam"`

## Observability Impact

- Signals added/changed: visible Radix toast failures for live reads, selected-issue loading/error state, and Playwright assertions that capture failed requests plus toast text.
- How a future agent inspects this: reproduce with `bash mesher/scripts/seed-live-issue.sh`, run the live spec in dev/prod, and inspect the mounted toaster plus `issues-shell` data attributes during error handling.
- Failure state exposed: selected-event/detail fetch failures, timeline read regressions, fallback-shape mismatches, and silent-error regressions.

## Inputs

- `mesher/client/components/dashboard/dashboard-issues-state.tsx` — live overview bootstrap from T01 and current shell state owner.
- `mesher/client/components/dashboard/issue-list.tsx` — current list rendering contract that expects richer product-shaped fields.
- `mesher/client/components/dashboard/issue-detail.tsx` — current detail shell that must stay intact while becoming live-backed.
- `mesher/client/components/dashboard/issues-page.tsx` — page composition and panel wiring.
- `mesher/client/src/routes/__root.tsx` — current root route without a mounted toaster surface.
- `mesher/client/components/ui/toaster.tsx` — existing Radix toaster implementation to reuse.
- `mesher/client/hooks/use-toast.ts` — existing toast state hook to reuse.
- `mesher/client/tests/e2e/issues-live-read.spec.ts` — live spec started in T01.
- `mesher/api/detail.mpl` — event-detail payload contract.
- `mesher/api/dashboard.mpl` — issue timeline contract.
- `mesher/client/README.md` — stale mock-only package guidance that must be updated.

## Expected Output

- `mesher/client/components/dashboard/dashboard-issues-state.tsx` — selected-issue live detail/timeline orchestration with error handling.
- `mesher/client/components/dashboard/issue-list.tsx` — list rendering that preserves shell shape while consuming live/fallback fields.
- `mesher/client/components/dashboard/issue-detail.tsx` — live-backed detail shell that retains unsupported sections.
- `mesher/client/components/dashboard/issues-page.tsx` — page wiring aligned with live detail and toast behavior.
- `mesher/client/src/routes/__root.tsx` — mounted Radix toaster surface.
- `mesher/client/tests/e2e/issues-live-read.spec.ts` — dev/prod live-browser proof for detail, runtime cleanliness, and failure toasts.
- `mesher/scripts/seed-live-issue.sh` — deterministic local seed helper for the live verification path.
- `mesher/client/README.md` — maintainer guidance updated for the mixed live/mock dashboard seam.
