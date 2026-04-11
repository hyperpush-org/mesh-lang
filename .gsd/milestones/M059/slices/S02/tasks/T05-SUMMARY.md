---
id: T05
parent: S02
milestone: M059
key_files:
  - ../hyperpush-mono/mesher/frontend-exp/playwright.config.ts
  - ../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts
  - ../hyperpush-mono/mesher/frontend-exp/package.json
  - .gsd/KNOWLEDGE.md
  - .gsd/DECISIONS.md
key_decisions:
  - D502 — Use PLAYWRIGHT_PROJECT-backed npm scripts as the stable isolated dev/prod parity contract because current npm `exec playwright ... --project=<name>` still runs both Playwright projects.
duration: 
verification_result: passed
completed_at: 2026-04-11T17:04:12.410Z
blocker_discovered: false
---

# T05: Added isolated dev/prod Playwright parity scripts, tightened the shared config, and proved non-root dashboard deep links against both dev and built production.

**Added isolated dev/prod Playwright parity scripts, tightened the shared config, and proved non-root dashboard deep links against both dev and built production.**

## What Happened

I reproduced the slice-plan verification commands against `../hyperpush-mono/mesher/frontend-exp` and found a truthful contract issue: `npm exec playwright ... --project=<name>` emitted npm warnings and still ran both Playwright projects, so the command shape did not isolate dev from prod even though the shared suite passed. I tightened `playwright.config.ts` to validate localhost base URLs and expected ports, select named projects/web servers through one shared helper, and document the npm flag-forwarding gotcha in the verification surface. I added repo-owned `test:e2e:dev` and `test:e2e:prod` scripts that set `PLAYWRIGHT_PROJECT`, and I extended the shared parity spec with an explicit settings non-root deep-link smoke so the built-production direct-entry requirement is visible in the owned test surface rather than only implied by the route loop. I reran the production build, the isolated repo-owned dev/prod parity scripts, and the exact slice verification commands, then recorded the npm-exec caveat in `.gsd/KNOWLEDGE.md` and decision D502.

## Verification

Verified `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build` succeeds after the Playwright/config updates. Verified `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test:e2e:dev` and `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test:e2e:prod` each run only the intended Playwright project and pass the full shared route-parity suite, including the explicit settings direct-entry smoke and the console/request-clean assertions. Verified the exact slice-plan commands `npm --prefix ../hyperpush-mono/mesher/frontend-exp exec playwright test tests/e2e/dashboard-route-parity.spec.ts --project=dev` and `--project=prod` still exit successfully, while also confirming from their output that current npm continues to treat `--project=<name>` as npm config and runs both Playwright projects.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build` | 0 | ✅ pass | 5900ms |
| 2 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test:e2e:dev` | 0 | ✅ pass | 34500ms |
| 3 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test:e2e:prod` | 0 | ✅ pass | 32500ms |
| 4 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp exec playwright test tests/e2e/dashboard-route-parity.spec.ts --project=dev` | 0 | ✅ pass | 47100ms |
| 5 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp exec playwright test tests/e2e/dashboard-route-parity.spec.ts --project=prod` | 0 | ✅ pass | 36100ms |

## Deviations

Added explicit repo-owned `test:e2e:dev` / `test:e2e:prod` verification runs and a named settings direct-entry smoke because the unchanged slice-plan `npm exec ... --project=<name>` commands still do not isolate environments on the current npm version.

## Known Issues

`npm --prefix ../hyperpush-mono/mesher/frontend-exp exec playwright test ... --project=<name>` still shows npm unknown-config warnings and runs both Playwright projects on this npm release. The repo-owned scripts added in this task are the stable isolated workaround, but the upstream npm CLI behavior itself remains unchanged.

## Files Created/Modified

- `../hyperpush-mono/mesher/frontend-exp/playwright.config.ts`
- `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts`
- `../hyperpush-mono/mesher/frontend-exp/package.json`
- `.gsd/KNOWLEDGE.md`
- `.gsd/DECISIONS.md`
