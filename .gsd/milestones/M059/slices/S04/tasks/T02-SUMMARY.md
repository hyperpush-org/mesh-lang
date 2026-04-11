---
id: T02
parent: S04
milestone: M059
key_files:
  - AGENTS.md
  - ../hyperpush-mono/AGENTS.md
  - ../hyperpush-mono/CONTRIBUTING.md
  - ../hyperpush-mono/SUPPORT.md
  - ../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml
  - ../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml
  - ../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml
  - .gsd/milestones/M059/slices/S04/tasks/T02-SUMMARY.md
key_decisions:
  - Keep the cleanup scoped to the selected direct-operational surfaces and prove it with targeted stale-path/positive-path greps plus the existing `mesher/client` build, parity, and root-harness rails.
duration: 
verification_result: passed
completed_at: 2026-04-11T19:02:04.037Z
blocker_discovered: false
---

# T02: Repointed the remaining maintainer guidance and issue templates to mesher/client and re-ran the full dashboard closeout rails.

**Repointed the remaining maintainer guidance and issue templates to mesher/client and re-ran the full dashboard closeout rails.**

## What Happened

Updated only the selected maintainer-facing operational surfaces so the split-workspace guidance, contributing docs, support triage text, and GitHub issue templates now point dashboard work at `mesher/client` while preserving `mesher/landing` as the intentional Next.js app. Then reran the full slice closeout contract from `mesh-lang`: the canonical `mesher/client` build, targeted dev/prod `dashboard-route-parity.spec.ts` runs, the full dev/prod parity suites, the scoped stale-path and positive-path greps over the touched files, and the root-harness `playwright ... --list` check. Every required verification command passed.

## Verification

Passed the slice-level closeout rails from `mesh-lang`: `npm --prefix ../hyperpush-mono/mesher/client run build`, `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev -- tests/e2e/dashboard-route-parity.spec.ts`, `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod -- tests/e2e/dashboard-route-parity.spec.ts`, `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev`, `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod`, the scoped stale-path and positive-path greps over the touched guidance files, and `PLAYWRIGHT_PROJECT=dev npx --prefix ../hyperpush-mono/mesher/client playwright test --config ./playwright.config.ts --project=dev --list`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev -- tests/e2e/dashboard-route-parity.spec.ts` | 0 | ✅ pass | 39937ms |
| 2 | `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod -- tests/e2e/dashboard-route-parity.spec.ts` | 0 | ✅ pass | 37517ms |
| 3 | `npm --prefix ../hyperpush-mono/mesher/client run build` | 0 | ✅ pass | 6318ms |
| 4 | `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev` | 0 | ✅ pass | 43589ms |
| 5 | `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod` | 0 | ✅ pass | 37937ms |
| 6 | `! rg -n "mesher/frontend-exp|frontend-exp" ../hyperpush-mono/AGENTS.md ../hyperpush-mono/CONTRIBUTING.md ../hyperpush-mono/SUPPORT.md ../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml ./AGENTS.md` | 0 | ✅ pass | 95ms |
| 7 | `rg -n "mesher/client" ../hyperpush-mono/AGENTS.md ../hyperpush-mono/CONTRIBUTING.md ../hyperpush-mono/SUPPORT.md ../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml ./AGENTS.md` | 0 | ✅ pass | 71ms |
| 8 | `PLAYWRIGHT_PROJECT=dev npx --prefix ../hyperpush-mono/mesher/client playwright test --config ./playwright.config.ts --project=dev --list` | 0 | ✅ pass | 2307ms |

## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `AGENTS.md`
- `../hyperpush-mono/AGENTS.md`
- `../hyperpush-mono/CONTRIBUTING.md`
- `../hyperpush-mono/SUPPORT.md`
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml`
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml`
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml`
- `.gsd/milestones/M059/slices/S04/tasks/T02-SUMMARY.md`
