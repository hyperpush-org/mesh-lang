---
id: S02
parent: M060
milestone: M060
provides:
  - A truthful live Issues maintainer loop in the canonical `mesher/client` package.
  - A stable provider-owned seam for future live dashboard mutations and refresh logic.
  - Replay-safe seeded read/action fixtures plus dev/prod Playwright proof for the existing Issues route.
requires:
  - slice: S01
    provides: same-origin `/api/v1` read seam, typed live-overlay adapter, mounted toast surface, and provider-owned Issues bootstrap/detail hydration
affects:
  - S04
key_files:
  - mesher/client/lib/mesher-api.ts
  - mesher/client/components/dashboard/dashboard-issues-state.tsx
  - mesher/client/components/dashboard/issue-detail.tsx
  - mesher/client/lib/issues-live-adapter.ts
  - mesher/client/components/dashboard/stats-bar.tsx
  - mesher/client/tests/e2e/issues-live-actions.spec.ts
  - mesher/client/tests/e2e/issues-live-read.spec.ts
  - mesher/scripts/seed-live-issue.sh
  - mesher/client/README.md
key_decisions:
  - Keep all supported issue mutations inside `DashboardIssuesStateProvider` and refetch provider-owned overview/detail state after success instead of trusting thin mutation payloads or scattering fetches across components.
  - Expose only `Resolve`, `Reopen`, and `Ignore` as live maintainer actions in S02; leave `assign`, `discard`, and `delete` visibly unsupported until the shell can represent those backend contracts honestly.
  - Expose per-card summary source markers (`live`, `derived live`, `fallback`) so mixed live/fallback summary chrome stays truthful.
  - Use separate deterministic seeded issues for read and action proof so the combined dev/prod `issues live` Playwright runs remain stable under parallel workers.
patterns_established:
  - Same-origin `/api/v1` browser transport for dashboard writes as well as reads.
  - Provider-owned mutation orchestration with selected-snapshot invalidation and post-write overview/detail refetch.
  - Per-card summary source observability rather than a single overclaiming global live label.
  - Deterministic split read/action fixtures for parallel browser verification on mutable entities.
observability_surfaces:
  - same-origin `/api/v1/issues/:id/{resolve,unresolve,archive}` requests
  - `issues-shell` and detail-panel `data-*` attributes for mutation phase/error/source state
  - per-card summary source markers in the stats bar
  - live list/detail status badges after provider refresh
  - mounted Radix toast region for destructive failure visibility
drill_down_paths:
  - .gsd/milestones/M060/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M060/slices/S02/tasks/T02-SUMMARY.md
  - .gsd/milestones/M060/slices/S02/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-11T22:50:33.395Z
blocker_discovered: false
---

# S02: Core maintainer loop live

**Made the Issues maintainer loop truthful: the canonical `mesher/client` shell now exposes live Resolve/Reopen/Ignore actions, refreshes provider-owned list/detail/summary state after each write, and proves the combined read/action seam in seeded dev and prod runs.**

## What Happened

## What This Slice Delivered

S02 took the live read seam from S01 and turned it into a real maintainer loop without redesigning the existing dashboard shell. The Issues route now keeps all supported issue mutations inside `DashboardIssuesStateProvider`, sends them through same-origin `/api/v1/issues/:id/...` helpers, and treats the backend's thin mutation success bodies as acknowledgements only. After every successful write, the provider invalidates the selected snapshot, refetches overview/detail state, and rehydrates the shell from backend truth instead of patching UI state optimistically.

The existing issue detail row now exposes only the supported live maintainer actions for this slice: `Resolve`, `Reopen`, and `Ignore` (`archive` on the backend). Unsupported controls (`assign`, `discard`, `delete`) were left visibly non-live instead of being wired to fake behavior. Busy/disabled state is surfaced during writes, the currently selected issue stays in context on failure, and destructive toast feedback is shown for both direct mutation failures and post-mutation refresh failures.

The Issues summary chrome also stopped overclaiming broad mock stats. The live adapter and stats bar now report per-card backing status as `live`, `derived live`, or `fallback`, which lets the shell stay visually intact while remaining honest about which summary signals are actually coming from backend-backed data. The issue list/detail surfaces now expose refreshed live status truth after actions, including filtered-list transitions when an item moves between open/resolved/ignored states.

## Patterns Established

- Keep Mesher dashboard browser traffic on same-origin `/api/v1` even for writes.
- Keep mutation orchestration, selected-snapshot invalidation, and post-write refetch inside `DashboardIssuesStateProvider` rather than scattering fetches across route loaders or leaf components.
- Treat mutation responses as insufficient for optimistic UI; refetch the provider-owned overview/detail state after success.
- Preserve the existing shell and make honesty visible through stable `data-*` attributes, per-card summary source markers, and the mounted Radix toaster instead of adding a new UX path.
- For combined Playwright suites that mutate shared seeded entities, split deterministic read and action fixtures so parallel workers do not race each other.

## What Downstream Slices Should Know

S04 can now rely on a truthful Issues maintainer seam that already proves the full seeded read-plus-action loop in both dev and built-prod. The provider and adapter are the canonical seams to extend; new live dashboard surfaces should reuse the same-origin transport, explicit live/fallback observability, and toast-based failure visibility rather than introducing route-local fetch paths or optimistic local patches.

S03 still needs to wire the remaining admin and ops surfaces live. This slice deliberately did **not** widen into unsupported issue actions or new backend routes. `assign`, `discard`, and `delete` remain shell-only because the current seeded bootstrap and shell vocabulary cannot represent those backend contracts honestly yet.

## Operational Readiness (Q8)

- **Health signal:** passing seeded `issues live` Playwright runs in both dev and prod; same-origin `/api/v1/issues/:id/{resolve,unresolve,archive}` traffic; refreshed issue-row/detail status; per-card summary source markers; and mounted destructive toasts when failures are induced.
- **Failure signal:** visible destructive toast after mutation or refresh failure, retained selected-issue context, and `issues-shell` / detail-panel `data-*` attributes reporting mutation phase/error/source state instead of silently showing stale success.
- **Recovery procedure:** rerun `bash mesher/scripts/seed-live-issue.sh` to reseed deterministic read/action fixtures, then rerun the canonical verifier commands. If the local shell lacks `DATABASE_URL`, the seed helper can reuse an already-running Mesher backend instead of requiring manual env setup.
- **Monitoring gaps:** this slice still relies on browser/test-visible diagnostics rather than a broader server-side operational dashboard; unsupported issue actions remain intentionally non-live; and broader admin/ops routes are still outside this slice, so milestone-level assembled readiness still depends on S03 and S04.


## Verification

Executed the exact slice verification contract and all checks passed.

1. `bash mesher/scripts/seed-live-issue.sh`
   - Passed.
   - Seeded and verified separate deterministic read/action issues against a running local Mesher at `http://127.0.0.1:18080`.
   - Confirmed detail and timeline readback for both fixtures and reset the action issue to an open state for replay-safe action proofs.

2. `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live"`
   - Passed: 10/10 tests.
   - Proved seeded same-origin issue reads plus Resolve/Reopen/Ignore action replay, disabled-state handling during writes, destructive toast visibility on mutation and refresh failure, truthful sparse/fallback handling, and unsupported-action/unknown-issue rejection.

3. `npm --prefix mesher/client run test:e2e:prod -- --grep "issues live"`
   - Passed: 10/10 tests.
   - Reconfirmed the same read/action seam against the built production server with the same observability and failure-visibility assertions.

Observability confirmation: the passing suite exercised the planned diagnostic surfaces — same-origin issue-mutation requests, `issues-shell` and detail-panel `data-*` state, per-card summary source markers, live list/detail status badges, and the mounted Radix toast region.

## Requirements Advanced

- R153 — Replaced broad mock-only Issues summary and action behavior with backend-backed same-origin reads and supported issue mutations on an existing backend route.
- R154 — Proved the first end-to-end backend-backed dashboard maintainer actions from the client by wiring Resolve/Reopen/Ignore through the real Mesher API seam.
- R156 — Kept the existing Issues shell materially intact while making list/detail/summary/action areas live and explicitly marking fallback-backed summary cards.
- R158 — Extended visible failure feedback from S01 reads into write and post-write refresh failure cases using the mounted Radix toaster plus provider diagnostics.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None from the slice plan. The shipped seam matches the planned provider-owned same-origin mutations, truthful post-write refetch, backend-backed summary chrome, and seeded dev/prod verification contract.

## Known Limitations

`assign`, `discard`, and `delete` remain intentionally non-live because the current seeded bootstrap and shell contract cannot represent those backend actions truthfully yet. Some summary cards are still fallback-backed by design, but they now advertise that backing explicitly instead of overclaiming live data. Broader admin/ops surfaces remain for S03 and assembled milestone walkthrough work remains for S04.

## Follow-ups

S03 should apply the same same-origin/provider-owned/failure-visible pattern to admin and ops surfaces that already have backend routes. S04 should reuse the deterministic seeded Issues seam as a stable sub-proof inside the full backend-backed shell walkthrough.

## Files Created/Modified

- `mesher/client/lib/mesher-api.ts` — Added typed same-origin issue mutation helpers for resolve, unresolve, and archive.
- `mesher/client/components/dashboard/dashboard-issues-state.tsx` — Centralized mutation orchestration, selected-snapshot invalidation, post-write refetch, and mutation diagnostics in the provider.
- `mesher/client/components/dashboard/issue-detail.tsx` — Wired the supported live maintainer actions into the existing detail action row with busy/disabled and failure-visible behavior.
- `mesher/client/lib/issues-live-adapter.ts` — Extended Issues adapter output with truthful summary-source metadata for mixed live/fallback cards.
- `mesher/client/components/dashboard/stats-bar.tsx` — Rendered per-card live/derived/fallback summary markers in the existing stats chrome.
- `mesher/client/tests/e2e/issues-live-actions.spec.ts` — Proved the real same-origin action seam, disabled-state behavior, destructive toasts, refresh failures, and unsupported-action rejection.
- `mesher/client/tests/e2e/issues-live-read.spec.ts` — Aligned read-side proof with the replay-safe two-fixture seed contract and truthful sparse/fallback assertions.
- `mesher/scripts/seed-live-issue.sh` — Made the seed helper replay-safe by creating separate deterministic read/action fixtures and resetting the action issue for repeatable proofs.
- `mesher/client/README.md` — Documented the supported S02 live action set and canonical verification commands.
