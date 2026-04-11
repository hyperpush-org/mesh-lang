# S01: S01 — UAT

**Milestone:** M059
**Written:** 2026-04-11T07:37:44.355Z

# S01 UAT — In-place TanStack Start conversion groundwork

## Preconditions

- Workspace root is `mesh-lang/`.
- Dependencies for `../hyperpush-mono/mesher/frontend-exp/` are installed.
- Ports `3000` and `3001` are available.
- Use a clean browser session so stale console/network noise does not mask current results.

## Test Case 1 — Production build still succeeds after the framework swap

1. Run `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build`.
   - Expected: the build completes successfully with TanStack Start/Vite output under `dist/client/` and `dist/server/`.
2. Review the command exit status.
   - Expected: exit code `0`.
3. Note any warnings.
   - Expected: large chunk warnings may appear because the full dashboard shell still lives behind one route, but there are no fatal build errors.

## Test Case 2 — Dev server preserves the visible dashboard shell at `/`

1. Start the dev server with `npm --prefix ../hyperpush-mono/mesher/frontend-exp run dev`.
   - Expected: the server becomes reachable at `http://localhost:3000/`.
2. Open `http://localhost:3000/`.
   - Expected: page title is `hyperpush — Error Tracking Dashboard`.
   - Expected: the initial visible heading is `Issues`.
   - Expected: the dashboard shell shows the existing project switcher, sidebar, filters, and mock dashboard content.
3. Check browser diagnostics.
   - Expected: no console errors.
   - Expected: no failed network requests.

## Test Case 3 — Existing shell state still works after the runtime swap

1. From the dev server at `http://localhost:3000/`, click the `Bounties` sidebar item.
   - Expected: the main heading changes to `Bounties`.
   - Expected: bounty metrics such as `TOTAL CLAIMS`, `PENDING`, and rows like `BNT-1042` become visible.
2. Observe the URL after the click.
   - Expected: the URL remains `http://localhost:3000/`.
3. Check browser diagnostics again.
   - Expected: no console errors.
   - Expected: no failed network requests.
4. Interpret the result.
   - Expected: this confirms shell parity for the current stateful dashboard while also proving the remaining S02 seam: sidebar section changes are still local state, not route-backed URLs.

## Test Case 4 — `npm run start` truthfully serves the built app

1. After a successful build, start the production server with `PORT=3001 npm --prefix ../hyperpush-mono/mesher/frontend-exp run start`.
   - Expected: the server becomes reachable at `http://localhost:3001/`.
2. Open `http://localhost:3001/`.
   - Expected: page title is `hyperpush — Error Tracking Dashboard`.
   - Expected: the `Issues` shell renders with the same visible dashboard surface as the dev server.
3. Check browser diagnostics.
   - Expected: no console errors.
   - Expected: no failed network requests.
4. Confirm command-contract parity.
   - Expected: the production app starts without any dependency on Next.js or a missing `.output/server/index.mjs` path.

## Edge Cases

### Edge Case A — Local production smoke should not depend on Vercel analytics being live

1. Run the production server locally without setting `VITE_ENABLE_VERCEL_ANALYTICS=true`.
   - Expected: the dashboard still loads normally.
   - Expected: there is no `/_vercel/insights/script.js` 404 noise in console/network logs.

### Edge Case B — No backend-integration drift is introduced by the migration groundwork

1. Exercise the dev or production shell without configuring any backend services.
   - Expected: the dashboard still renders from its existing mock-data/stateful shell.
   - Expected: no new API/bootstrap failure appears as part of the framework migration groundwork.

