# S02: Route-backed dashboard parity — UAT

**Milestone:** M059
**Written:** 2026-04-11T17:32:06.851Z

# S02 UAT — Route-backed dashboard parity

## Preconditions
- Run from `mesh-lang`.
- Install dependencies for `../hyperpush-mono/mesher/frontend-exp/`.
- Keep ports `3000` and `3001` free.

## Test Case 1 — Build and generated routes
1. Run `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build`.
   - Expected: build exits with code `0` and generates `src/routeTree.gen.ts` entries for `/`, `/performance`, `/solana-programs`, `/releases`, `/alerts`, `/bounties`, `/treasury`, and `/settings`.

## Test Case 2 — Dev direct entry
1. Start dev with `npm --prefix ../hyperpush-mono/mesher/frontend-exp run dev -- --host 127.0.0.1 --port 3000`.
2. Open `/`, `/performance`, `/solana-programs`, `/releases`, `/alerts`, `/bounties`, `/treasury`, and `/settings` directly.
   - Expected: the shared dashboard shell stays visible and each route shows its own heading.
3. Open `/not-a-real-route`.
   - Expected: the app returns the Issues shell.

## Test Case 3 — URL-backed navigation parity
1. Open `/` and launch AI Copilot.
2. Click `Performance`.
   - Expected: URL changes to `/performance`, the heading changes to `Performance`, and the AI panel closes.
3. Click `Settings`.
   - Expected: URL changes to `/settings`, `General` is visible, and shared dashboard header controls such as `Last 24h` are hidden.
4. Click `Issues`.
   - Expected: URL returns to `/` and the Issues shell returns with no visible chrome drift.

## Test Case 4 — Issues leave-and-return behavior
1. On `/`, search for `HPX-1039` and open the matching issue.
2. Navigate to `/performance`, then return to `/`.
   - Expected: the search text remains `HPX-1039` and the same issue detail panel remains open.

## Test Case 5 — Built production deep links
1. Start built production with `PORT=3001 npm --prefix ../hyperpush-mono/mesher/frontend-exp run start` after a successful build.
2. Open `/settings` directly on port `3001`.
   - Expected: `General` and `Save` are visible before any in-app navigation.
3. Open `/alerts` and an unknown path directly on port `3001`.
   - Expected: alerts renders under the same shell, and the unknown path returns the Issues shell.

