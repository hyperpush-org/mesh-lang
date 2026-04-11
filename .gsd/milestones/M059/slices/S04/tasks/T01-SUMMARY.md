---
id: T01
parent: S04
milestone: M059
key_files:
  - ../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts
  - ../hyperpush-mono/mesher/client/components/dashboard/events-chart.tsx
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Keep the new proof inside the existing dashboard-route-parity Playwright rail and fix surfaced console regressions in runtime code rather than weakening runtime-signal assertions.
duration: 
verification_result: passed
completed_at: 2026-04-11T18:52:40.042Z
blocker_discovered: false
---

# T01: Extended the canonical dashboard parity suite for Solana AI/sidebar restoration and Issues browser-history state, and fixed the clean-boot Recharts console warning.

**Extended the canonical dashboard parity suite for Solana AI/sidebar restoration and Issues browser-history state, and fixed the clean-boot Recharts console warning.**

## What Happened

Extended ../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts instead of creating a second verifier. Added one proof path for Solana Programs where opening AI auto-collapses the sidebar and closing AI restores it, and one proof path for Issues search/filter/detail state surviving real browser back/forward navigation. The first dev parity run surfaced a real runtime regression through the existing runtime-signal tracker: a React console warning during the initial Issues-shell boot. I traced that to the chart mount path and made the minimum runtime fix in ../hyperpush-mono/mesher/client/components/dashboard/events-chart.tsx by disabling Recharts Area animations. After that fix, the canonical dev and prod parity commands both passed cleanly. I also recorded the Recharts/Playwright console-warning gotcha in .gsd/KNOWLEDGE.md for future parity work.

## Verification

Passed the task-plan commands `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev -- tests/e2e/dashboard-route-parity.spec.ts` and `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod -- tests/e2e/dashboard-route-parity.spec.ts`, plus the slice-level harness listing command `PLAYWRIGHT_PROJECT=dev npx --prefix ../hyperpush-mono/mesher/client playwright test --config ./playwright.config.ts --project=dev --list`. Also exercised the live dev app with browser assertions to confirm Solana AI/sidebar collapse+restore and Issues history-state recovery without console or failed-request noise.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev -- tests/e2e/dashboard-route-parity.spec.ts` | 0 | ✅ pass | 48481ms |
| 2 | `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod -- tests/e2e/dashboard-route-parity.spec.ts` | 0 | ✅ pass | 39904ms |
| 3 | `PLAYWRIGHT_PROJECT=dev npx --prefix ../hyperpush-mono/mesher/client playwright test --config ./playwright.config.ts --project=dev --list` | 0 | ✅ pass | 2489ms |

## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts`
- `../hyperpush-mono/mesher/client/components/dashboard/events-chart.tsx`
- `.gsd/KNOWLEDGE.md`
