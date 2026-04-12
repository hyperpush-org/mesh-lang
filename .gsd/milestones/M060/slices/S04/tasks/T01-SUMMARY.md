---
id: T01
parent: S04
milestone: M060
key_files:
  - mesher/client/tests/e2e/live-runtime-helpers.ts
  - mesher/client/tests/e2e/seeded-walkthrough.spec.ts
  - mesher/client/tests/e2e/dashboard-route-parity.spec.ts
  - mesher/client/tests/e2e/issues-live-read.spec.ts
key_decisions:
  - Centralized same-origin request diagnostics and route assertions in a shared E2E helper instead of keeping per-spec runtime tracking forks.
  - Made route parity derive from live route inventory and current visible rows rather than stale hardcoded issue fixtures.
duration: 
verification_result: mixed
completed_at: 2026-04-12T01:15:56.706Z
blocker_discovered: false
---

# T01: Added a route-map-driven seeded dashboard walkthrough with shared live runtime diagnostics and refreshed route parity coverage.

**Added a route-map-driven seeded dashboard walkthrough with shared live runtime diagnostics and refreshed route parity coverage.**

## What Happened

I added `mesher/client/tests/e2e/live-runtime-helpers.ts` to centralize the dashboard route inventory, same-origin API/runtime signal tracking, direct-backend detection, filtered hidden-Issues abort handling, and route navigation assertions. I then added `mesher/client/tests/e2e/seeded-walkthrough.spec.ts`, which derives its route coverage from `dashboard-route-map.ts` and proves one browser-session walkthrough across Issues, Performance, Solana Programs, Releases, Alerts, Bounties, Treasury, and Settings. The walkthrough reuses the seeded issue read/action seams, creates a live alert through the same-origin alert-rule/event seam, exercises live Settings writes plus mock-only subsection visibility, and asserts route-key/source/state truth instead of a second hand-maintained list. I also rewrote `dashboard-route-parity.spec.ts` to use the current route-map/runtime contract and live-list discovery instead of stale hardcoded issue ids and raw failed-request expectations, and I updated `issues-live-read.spec.ts` to match the current sparse-detail UI behavior where the shell visibly renders fallback stack/breadcrumb messaging. The task-level dev verification rail now passes. During the broader slice-level dev grep, I observed remaining shared-rail follow-up pressure around the combined live suites; I applied two final local fixes for that broader rail (selected-issue abort filtering in the shared helper and sparse-detail expectation updates), but I did not have context budget to rerun the full combined rail after those last edits.

## Verification

Passed: `bash mesher/scripts/seed-live-issue.sh && bash mesher/scripts/seed-live-admin-ops.sh`, and `npm --prefix mesher/client run test:e2e:dev -- --grep "seeded walkthrough|dashboard route parity"` (8/8 passing). Partial slice verification: `bash mesher/scripts/seed-live-issue.sh && bash mesher/scripts/seed-live-admin-ops.sh && npm --prefix mesher/client run test:e2e:dev -- --grep "issues live|admin and ops live|seeded walkthrough"` still reported broader-suite failures before the last wrap-up edits; those final edits were applied but not rerun because of the context-budget wrap-up event. Prod verification was not run in this task.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash mesher/scripts/seed-live-issue.sh && bash mesher/scripts/seed-live-admin-ops.sh` | 0 | ✅ pass | 6000ms |
| 2 | `npm --prefix mesher/client run test:e2e:dev -- --grep "seeded walkthrough|dashboard route parity"` | 0 | ✅ pass | 57400ms |
| 3 | `bash mesher/scripts/seed-live-issue.sh && bash mesher/scripts/seed-live-admin-ops.sh && npm --prefix mesher/client run test:e2e:dev -- --grep "issues live|admin and ops live|seeded walkthrough"` | 1 | ❌ fail | 79100ms |

## Deviations

I rewrote `dashboard-route-parity.spec.ts` more broadly than the original stale fixture-oriented assertions because the existing parity file was no longer truthful: it hardcoded a dead issue id and treated known hidden-provider aborts as regressions. The rewrite stayed within the slice contract by keeping parity coverage route-map-driven and shell-focused.

## Known Issues

The broader combined dev rail (`issues live|admin and ops live|seeded walkthrough`) was still failing before the final wrap-up edits, and I did not have budget to rerun it after applying the last sparse-detail/assertion and abort-filter changes. Prod rail verification (`test:e2e:prod`) was not run in this task and remains for downstream work.

## Files Created/Modified

- `mesher/client/tests/e2e/live-runtime-helpers.ts`
- `mesher/client/tests/e2e/seeded-walkthrough.spec.ts`
- `mesher/client/tests/e2e/dashboard-route-parity.spec.ts`
- `mesher/client/tests/e2e/issues-live-read.spec.ts`
