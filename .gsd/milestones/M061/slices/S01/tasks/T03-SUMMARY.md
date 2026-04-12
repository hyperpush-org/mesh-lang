---
id: T03
parent: S01
milestone: M061
key_files:
  - ../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh
  - ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs
  - ../hyperpush-mono/mesher/client/package.json
  - ../hyperpush-mono/mesher/client/README.md
  - ../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Run Playwright from `mesh-lang` through `npm --prefix ../hyperpush-mono/mesher/client exec -- playwright ... --config ../hyperpush-mono/mesher/client/playwright.config.ts` so the verifier preserves the client package boundary and the grep/config arguments survive npm forwarding.
  - Force `seed-live-issue.sh` onto isolated local ports inside the retained verifier instead of letting it reuse whatever is listening on `http://127.0.0.1:18080`, because wrapper proof needs deterministic seed/runtime pairing.
duration: 
verification_result: mixed
completed_at: 2026-04-12T04:00:06.778Z
blocker_discovered: false
---

# T03: Added the retained client route-inventory verifier, wired it into the maintainer workflow, and fixed the live Issues route-parity row-race.

**Added the retained client route-inventory verifier, wired it into the maintainer workflow, and fixed the live Issues route-parity row-race.**

## What Happened

Implemented `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` as the retained maintainer entrypoint for this slice. The wrapper now preflights required inventory/proof files, records `phase-report.txt`, `status.txt`, and `current-phase.txt` under `../hyperpush-mono/mesher/.tmp/m061-s01/verify-client-route-inventory/`, runs the structural `node:test` contract first, runs both seed helpers with retained logs, and then runs targeted dev/prod Playwright proof rails with fail-closed suite-coverage validation against the five expected proof files.

Wired the entrypoint into `../hyperpush-mono/mesher/client/package.json` as `verify:route-inventory` and updated `../hyperpush-mono/mesher/client/README.md` so maintainers are pointed at `ROUTE-INVENTORY.md` as the canonical map and at the dedicated verifier as the primary proof command.

Extended `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` so the cheap structural contract now also guards the verifier wiring, retained-log surfaces, and README/package workflow references.

While validating the wrapper, I found and fixed two real proof-harness issues: (1) `seed-live-issue.sh` could hang or verify against the wrong runtime by reusing a stray backend on `:18080`, so the wrapper now forces isolated seed ports plus a concrete local `DATABASE_URL` fallback; and (2) `dashboard-route-parity.spec.ts` could capture a fallback-only issue row before the live overview finished loading, so I added an explicit ready-state wait before taking the row id used in the search/history assertions. That isolated previously failing dev route-parity proof now passes.

The remaining blocker is outside the wrapper itself: the full retained verifier still fails in the prod `seeded-walkthrough` alerts step. In the final slice-level run, every wrapper phase passed through `route-inventory-dev`, but `route-inventory-prod` failed because `seeded-walkthrough.spec.ts` hit `alerts-shell` with `data-bootstrap-state="failed"` and `data-bootstrap-error-code="invalid-json"` during the alerts proof step even though the narrower prod `admin-ops-live` suite passed earlier in the same verifier run. The retained failing surfaces are `../hyperpush-mono/mesher/.tmp/m061-s01/verify-client-route-inventory/route-inventory-prod.log`, `phase-report.txt`, and the Playwright artifacts under `../hyperpush-mono/mesher/client/test-results/seeded-walkthrough-seeded--480ac-ruthful-live-and-mock-state-prod/`.

## Verification

Passed: `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` after adding the wrapper/workflow contract checks; `bash ../hyperpush-mono/mesher/scripts/seed-live-issue.sh >/tmp/m061-seed-live-issue.log 2>&1 && env PLAYWRIGHT_PROJECT=dev npm --prefix ../hyperpush-mono/mesher/client exec -- playwright test --config ../hyperpush-mono/mesher/client/playwright.config.ts --project=dev --grep "issues interactions persist across shell re-renders and browser history"` after fixing the live-overview race in `dashboard-route-parity.spec.ts`.

Failed: the slice-level verifier command `bash ../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` still fails in phase `route-inventory-prod`. The retained phase report shows `init`, `route-inventory-structure`, `seed-live-issue`, `seed-live-admin-ops`, and `route-inventory-dev` all passed, then `route-inventory-prod` failed because prod `seeded-walkthrough` alerts bootstrap regressed to `invalid-json`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` | 0 | ✅ pass | 556ms |
| 2 | `bash ../hyperpush-mono/mesher/scripts/seed-live-issue.sh >/tmp/m061-seed-live-issue.log 2>&1 && env PLAYWRIGHT_PROJECT=dev npm --prefix ../hyperpush-mono/mesher/client exec -- playwright test --config ../hyperpush-mono/mesher/client/playwright.config.ts --project=dev --grep "issues interactions persist across shell re-renders and browser history"` | 0 | ✅ pass | 58211ms |
| 3 | `bash ../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` | 1 | ❌ fail | 318030ms |

## Deviations

Used `npm --prefix ../hyperpush-mono/mesher/client exec -- playwright test --config ../hyperpush-mono/mesher/client/playwright.config.ts ...` inside the wrapper instead of `npm --prefix ... run test:e2e:* -- --grep ...` because npm argument forwarding from the `mesh-lang` cwd stripped the grep/config boundary badly enough to produce `test.describe()` / `No tests found` false failures. Also forced isolated ports plus a local `DATABASE_URL` fallback for `seed-live-issue.sh` because the helper's default `:18080` reuse behavior could hang or verify against a stale backend during retained wrapper runs.

## Known Issues

Full slice verification is still red on the existing prod `seeded-walkthrough` alerts proof. In `../hyperpush-mono/mesher/.tmp/m061-s01/verify-client-route-inventory/route-inventory-prod.log`, the `seeded walkthrough traverses the canonical dashboard shell with truthful live and mock state` test fails at the alerts step because `alerts-shell` reaches `data-bootstrap-state="failed"` with `data-bootstrap-error-code="invalid-json"`. This appears after the wrapper, seed helpers, and prod `admin-ops-live` suite have already succeeded, so the remaining issue is a prod walkthrough/app bootstrap regression rather than a retained-verifier wiring failure.

## Files Created/Modified

- `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh`
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
- `../hyperpush-mono/mesher/client/package.json`
- `../hyperpush-mono/mesher/client/README.md`
- `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts`
- `.gsd/KNOWLEDGE.md`
