# M061 Verification Failure Assessment

Date: 2026-04-12

## Outcome
Milestone closeout verification failed. M061 must remain active and must **not** be marked complete.

## What passed
- `git diff --stat $(git merge-base HEAD main) -- ':!.gsd/'` in `mesh-lang` showed no non-`.gsd/` changes, but the owning sibling repo has real product changes.
- `git -C ../hyperpush-mono diff --stat $(git -C ../hyperpush-mono merge-base HEAD main) -- ':!.gsd/'` confirmed non-`.gsd/` code/docs changes in the owning repo.
- `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` now passes after fixing `../hyperpush-mono/mesher/scripts/seed-live-issue.sh` port selection so isolated startup can fall back beyond the immediate `PORT+2..+199` window.
- Milestone DB state shows all slices complete: S01-S04 are `complete` and all task counts are done.
- All slice summaries exist on disk under `.gsd/milestones/M061/slices/S0*/S0*-SUMMARY.md`.

## Verification failure
The milestone still fails the required rerunnable closeout rail for S04/R170/R171.

### Repro
- `bash ../hyperpush-mono/scripts/verify-m061-s04.sh`
- Narrow repro after cancelling the long wrapper run:
  - `env PLAYWRIGHT_PROJECT=dev npm --prefix ../hyperpush-mono/mesher/client exec -- playwright test --config ../hyperpush-mono/mesher/client/playwright.config.ts --project=dev --grep "malformed live payloads as contract failures"`

### Current failing test
- `../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts:355`
- Test name: `admin and ops live alerts treat malformed live payloads as contract failures instead of guessing shell status`

### Observed failure
The targeted repro fails because `page.getByTestId('alerts-shell')` never appears. Playwright’s captured page snapshot shows only the notifications region, so the expected Alerts shell never mounts for this malformed-payload path.

### Evidence
- `../hyperpush-mono/mesher/client/test-results/admin-ops-live-admin-and-o-ed34a-ad-of-guessing-shell-status-dev/error-context.md`
- `../hyperpush-mono/mesher/.tmp/m061-s01/verify-client-route-inventory/route-inventory-dev.log`

## Why milestone completion is blocked
M061 roadmap/S04 promises: "The canonical inventory and backend gap map live beside `mesher/client` with a rerunnable drift-proof rail." The structural/doc rails are now green, but the assembled runtime rail is still red. That blocks validation of:
- R170 — repeatable proof rail
- R171 — actionable final handoff validated by the assembled rerun path

## Resume notes for next attempt
1. Debug why the malformed alerts payload route leaves only the notifications region mounted instead of rendering `alerts-shell` with fallback markers.
2. Reproduce with the narrow Playwright grep above before rerunning the full wrapper.
3. After fixing that runtime/UI path, rerun:
   - `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
   - `bash ../hyperpush-mono/scripts/verify-m061-s04.sh`
4. Only if the full wrapper passes end-to-end should M061 proceed to requirement updates and `gsd_complete_milestone`.
