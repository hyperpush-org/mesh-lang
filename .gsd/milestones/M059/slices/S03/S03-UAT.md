# S03: Finalize move to `mesher/client` and remove Next.js runtime path — UAT

**Milestone:** M059
**Written:** 2026-04-11T18:21:53.932Z

# S03 UAT — `mesher/client` canonical path cutover and parity

**Milestone:** M059  
**Slice:** S03  
**Written:** 2026-04-11

## Preconditions
- Run from `mesh-lang`.
- Install dependencies for `../hyperpush-mono/mesher/client/`.
- Keep ports `3000` and `3001` free.
- Do not run the old `frontend-exp` package as the app-under-test.

## Test Case 1 — Canonical path and machine-checked contract
1. Run:
   - `test -f ../hyperpush-mono/mesher/client/package.json`
   - `test ! -e ../hyperpush-mono/mesher/frontend-exp/package.json`
   - `! rg -n "mesher/frontend-exp|frontend-exp" ../hyperpush-mono/.github/workflows/ci.yml ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh ../hyperpush-mono/README.md ../hyperpush-mono/.github/dependabot.yml ./playwright.config.ts`
   - `rg -n "mesher/client|client" ../hyperpush-mono/.github/workflows/ci.yml ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh ../hyperpush-mono/README.md ../hyperpush-mono/.github/dependabot.yml ./playwright.config.ts`
2. Expected:
   - `mesher/client` exists as the canonical package root.
   - `frontend-exp/package.json` is gone.
   - No stale `frontend-exp` hits remain in the machine-checked caller surfaces.
   - Updated surfaces show `mesher/client` markers.

## Test Case 2 — Build and production bridge boot from `mesher/client`
1. Run `npm --prefix ../hyperpush-mono/mesher/client run build`.
   - Expected: build exits `0` and emits `dist/client` plus `dist/server/server.js`.
2. Run `PORT=3001 npm --prefix ../hyperpush-mono/mesher/client run start`.
3. Open `http://127.0.0.1:3001/settings`.
   - Expected: settings content renders directly, `Project name` and `Save` are visible, and the shared dashboard heading/AI toggle chrome stays suppressed.
4. Open `http://127.0.0.1:3001/does-not-exist/deep-link`.
   - Expected: the URL stays on the unknown path, but the dashboard renders the Issues shell instead of a 404 document.

## Test Case 3 — Dev direct-entry parity from the new package path
1. Run `npm --prefix ../hyperpush-mono/mesher/client run dev -- --host 127.0.0.1 --port 3000`.
2. Open each direct-entry route on port `3000`:
   - `/`
   - `/performance`
   - `/solana-programs`
   - `/releases`
   - `/alerts`
   - `/bounties`
   - `/treasury`
   - `/settings`
3. Expected:
   - The shared dashboard shell renders on every route.
   - Each route shows its expected heading or settings content.
   - The active sidebar item matches the pathname.
   - No console errors or failed requests appear while the app boots and renders.

## Test Case 4 — Issues leave-and-return behavior survived the move
1. Open `http://127.0.0.1:3000/`.
2. Search for `HPX-1039`.
3. Set status to `Regressed` and severity to `Critical`.
4. Open the `HPX-1039` issue detail panel.
5. Navigate to `/performance`, then return to `/`.
6. Expected:
   - The search value remains `HPX-1039`.
   - The status and severity filters remain selected.
   - The same issue detail panel remains open when returning to Issues.

## Test Case 5 — Package-local parity rails still isolate dev and prod correctly
1. Run `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev`.
   - Expected: exactly 7 tests pass under the `dev` project only.
2. Run `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod`.
   - Expected: exactly 7 tests pass under the `prod` project only.
3. Expected for both:
   - Direct-entry checks pass for `/`, `/performance`, `/solana-programs`, `/releases`, `/alerts`, `/bounties`, `/treasury`, `/settings`, and the unknown-path fallback.
   - The suite reports no console errors and no failed requests.

## Test Case 6 — Root cross-repo harness resolves `mesher/client`
1. Run `bash -n ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh`.
   - Expected: shell syntax check exits `0`.
2. Run `PLAYWRIGHT_PROJECT=dev npx --prefix ../hyperpush-mono/mesher/client playwright test --config ./playwright.config.ts --project=dev --list` from `mesh-lang`.
   - Expected: the root harness lists the 7 dashboard parity tests from `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts` and only the `dev` project appears.

## Edge Cases to Watch
- Unknown-path direct entry must still render the Issues shell in both dev and built production.
- The root harness must not start both dev and prod servers when `PLAYWRIGHT_PROJECT` or `--project` selects only one project.
- `app/globals.css` is still part of the live runtime contract because `src/routes/__root.tsx` imports it; do not delete it during follow-up cleanup.

