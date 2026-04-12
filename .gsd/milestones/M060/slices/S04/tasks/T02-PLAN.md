---
estimated_steps: 4
estimated_files: 8
skills_used:
  - playwright-best-practices
  - react-best-practices
---

# T02: Close assembly blockers and document the canonical full-shell rail

**Slice:** S04 — Full backend-backed shell assembly
**Milestone:** M060

## Description

Use the new walkthrough as the milestone's truth source: run it against dev and prod, repair only the exact blocker it exposes, and then document the final assembled verification rail. R159 applies here — do not redesign the shell or invent new backend surfaces just to satisfy the walkthrough.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `mesher/client/tests/e2e/seeded-walkthrough.spec.ts` in dev and prod | Inspect the failing route/action/marker and patch only that exact shell or backend seam; do not broaden into unrelated refactors. | Treat hung navigation or write refresh as an integration failure and surface the blocked route/action in test output plus shell markers. | Fail with typed assertion context and patch the exact parser, selector, or route-contract mismatch. |
| Minimal selector or observability patches in dashboard components | Keep new selectors and `data-*` markers backward-compatible with the existing specs; if a route lacks a stable anchor, add the smallest truthful surface needed. | Avoid adding flaky sleeps or broad polling loops; rely on existing shell state markers and explicit expectations. | Do not invent fake live labels or mock writes just to make the walkthrough green. |
| Narrow Mesher backend seam repairs | Touch only the exact route/helper/query seam proven to block the assembled walkthrough and keep the rest of the backend unchanged. | Surface the blocking route in the failing proof and stop widening scope; no speculative backend cleanup. | Treat malformed backend bodies as contract bugs to fix at the seam instead of compensating inside the test. |
| `mesher/client/README.md` verification docs | Keep commands aligned with the actual seed scripts and grep names so maintainers can reproduce the exact final rail. | N/A | N/A |

## Load Profile

- **Shared resources**: seeded issue and admin/ops fixtures, dev and prod dashboard runtimes, the same-origin proxy, and whichever backend route the walkthrough mutates.
- **Per-operation cost**: one full seeded walkthrough plus the existing `issues live` and `admin and ops live` suites in both runtimes.
- **10x breakpoint**: repeated seeded local runs will reveal flaky selector timing or stale post-write refresh state before raw CPU cost becomes a problem, so fixes must prefer deterministic markers over broader waits.

## Negative Tests

- **Malformed inputs**: broken route markers, unsupported still-mock-only control states, or backend payload fields that no longer match the existing adapters.
- **Error paths**: unexpected console errors, unfiltered 4xx/5xx responses, direct-backend browser requests, and write-refresh failures after the walkthrough performs supported actions.
- **Boundary conditions**: dev and prod parity, direct-entry and in-app navigation parity, and mock-only route stability after visiting live-backed routes in the same session.

## Steps

1. Run the new walkthrough alongside the existing live suites in dev, inspect the first failure, and patch only the smallest client-side selector, shell-state, or backend seam needed to make the assembled walkthrough truthful.
2. If the walkthrough uncovers a real backend blocker, repair only that exact Mesher route/helper/query seam and keep the client shell unchanged except for the minimal truthful state or selector update needed to expose the fix.
3. Re-run the combined dev and prod verification rails until `issues live`, `admin and ops live`, and `seeded walkthrough` all pass together without unexpected console or request failures.
4. Update `mesher/client/README.md` so the assembled seed plus dev/prod verification rail is documented alongside the route-level live-seam notes.

## Must-Haves

- [ ] Any fix stays within the exact failing seam exposed by the walkthrough; no shell redesign or new backend surface is added just to satisfy the test. (`R159`)
- [ ] Seeded dev and prod combined greps for `issues live|admin and ops live|seeded walkthrough` pass after seeding, proving the final assembled shell in a local runtime. (`R160`)
- [ ] Existing shell continuity promises remain intact: unsupported routes and controls stay visible and explicitly non-live where applicable. (`R156`, `R157`)
- [ ] `mesher/client/README.md` documents the seeded full-shell verification commands and points maintainers at the canonical walkthrough spec.

## Verification

- `bash mesher/scripts/seed-live-issue.sh && bash mesher/scripts/seed-live-admin-ops.sh`
- `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live|admin and ops live|seeded walkthrough"`
- `npm --prefix mesher/client run test:e2e:prod -- --grep "issues live|admin and ops live|seeded walkthrough"`

## Observability Impact

- Signals added/changed: any minimal new `data-testid` or `data-*` marker needed to localize assembled-route failures, plus README guidance pointing maintainers at the canonical seeded proof rail.
- How a future agent inspects this: rerun the combined dev/prod greps, then inspect the failing route's shell markers and same-origin call list from `live-runtime-helpers.ts`.
- Failure state exposed: the precise route/action/marker or backend seam that still blocks assembled verification, rather than a generic milestone-level red test.

## Inputs

- `mesher/client/tests/e2e/live-runtime-helpers.ts` — helper introduced by T01 for same-origin call tracking and filtered failure assertions.
- `mesher/client/tests/e2e/seeded-walkthrough.spec.ts` — assembled full-shell proof rail that drives the blocker hunt.
- `mesher/client/components/dashboard/dashboard-shell.tsx` — shell composition seam if route-level state markers or provider interactions need a minimal fix.
- `mesher/client/components/dashboard/alerts-page.tsx` — live Alerts shell if assembled navigation exposes a route-level truth gap.
- `mesher/client/components/dashboard/settings/settings-page.tsx` — live/mock Settings shell if assembled navigation exposes a subsection or marker gap.
- `mesher/client/README.md` — maintainer documentation that must advertise the canonical full-shell verification rail.
- `mesher/api/helpers.mpl` — narrow backend helper seam if the walkthrough proves the client is blocked by an existing route contract.
- `mesher/storage/queries.mpl` — narrow backend query seam if the walkthrough proves the client is blocked by persisted data shape or lookup behavior.

## Expected Output

- `mesher/client/tests/e2e/seeded-walkthrough.spec.ts` — final assembled walkthrough spec updated to match any repaired blocker and final verification expectations.
- `mesher/client/tests/e2e/live-runtime-helpers.ts` — final shared diagnostics helper with any additional filtering or route instrumentation required by the passing assembled rail.
- `mesher/client/components/dashboard/dashboard-shell.tsx` — minimal shell fix if composed navigation exposes a real state or provider seam.
- `mesher/client/components/dashboard/alerts-page.tsx` — minimal route-level fix if the walkthrough exposes an Alerts truth or selector gap.
- `mesher/client/components/dashboard/settings/settings-page.tsx` — minimal route-level fix if the walkthrough exposes a Settings truth or selector gap.
- `mesher/client/README.md` — documented seed plus dev/prod verification commands for the canonical full-shell rail.
- `mesher/api/helpers.mpl` — narrow backend seam repair if an existing helper contract blocks the assembled walkthrough.
- `mesher/storage/queries.mpl` — narrow backend seam repair if an existing query contract blocks the assembled walkthrough.
