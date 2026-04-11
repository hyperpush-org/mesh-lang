---
id: S01
parent: M059
milestone: M059
provides:
  - A live TanStack Start/Vite root and router entry in `frontend-exp`.
  - A truthful `npm run dev` / `npm run build` / `npm run start` contract for the in-place migrated app.
  - An explicit downstream seam for S02: sidebar sections still change through local shell state while the URL remains `/`.
  - Recorded migration gotchas in `.gsd/KNOWLEDGE.md` covering Fontsource/Tailwind behavior, current route-state shape, TanStack Start `dist/` production output, and local analytics gating.
requires:
  - slice: M059/S01
    provides: This slice is the groundwork layer and has no upstream slice dependency.
affects:
  - M059/S02
  - M059/S03
  - M059/S04
key_files:
  - ../hyperpush-mono/mesher/frontend-exp/package.json
  - ../hyperpush-mono/mesher/frontend-exp/vite.config.ts
  - ../hyperpush-mono/mesher/frontend-exp/server.mjs
  - ../hyperpush-mono/mesher/frontend-exp/src/router.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/__root.tsx
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx
  - ../hyperpush-mono/mesher/frontend-exp/tsconfig.json
  - ../hyperpush-mono/mesher/frontend-exp/app/globals.css
  - .gsd/KNOWLEDGE.md
  - .gsd/PROJECT.md
key_decisions:
  - D494 — map the old Next document/page split onto `src/routes/__root.tsx` plus `src/routes/index.tsx` so the migration stays in place instead of becoming a parallel rewrite.
  - D495 — keep the current dashboard shell client-only at `src/routes/index.tsx` and load Geist fonts from root-route Fontsource links instead of Next font injection.
  - D496 — preserve `npm run start` with a package-local Node bridge server over TanStack Start’s built `dist/` output instead of depending on Nitro’s `.output/` layout during S01 groundwork.
patterns_established:
  - For this migration, replace framework plumbing first and preserve the current product shell through a thin parity adapter route instead of decomposing routes and runtime plumbing in the same slice.
  - When the visible shell is still authoritative, verify parity at the browser level with explicit no-console-error/no-failed-request assertions instead of assuming build success is enough.
  - Preserve the external command contract (`dev` / `build` / `start`) even if the internal production runner changes during an in-place framework migration.
observability_surfaces:
  - Dev readiness on `http://localhost:3000/` via `npm run dev`.
  - Production readiness on `http://localhost:3001/` via `PORT=3001 npm run start`.
  - Browser assertions for page title, visible shell text, no console errors, and no failed requests.
  - Build output plus chunk-size warnings from `npm run build` as the current route-decomposition pressure signal.
drill_down_paths:
  - .gsd/milestones/M059/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M059/slices/S01/tasks/T02-SUMMARY.md
  - .gsd/milestones/M059/slices/S01/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-11T07:37:44.354Z
blocker_discovered: false
---

# S01: S01

**Converted `../hyperpush-mono/mesher/frontend-exp` from Next.js-rooted plumbing to TanStack Start/Vite groundwork while preserving the visible dashboard shell and restoring a truthful `dev` / `build` / `start` command contract.**

## What Happened

S01 completed the in-place framework groundwork instead of starting a parallel rewrite. The slice first audited the real Next-specific seams in `frontend-exp` and mapped them onto TanStack Start equivalents: the old app-router document root moved to `src/routes/__root.tsx`, the current dashboard shell was preserved through `src/routes/index.tsx`, and the runtime/config boundary moved from Next-specific files into Vite/TanStack Start plumbing. Execution then replaced the active runtime path with TanStack Start/Vite, kept the existing dashboard shell and global CSS rendering intact, restored Geist and Geist Mono through route-linked Fontsource assets, and fixed a latent type-only import issue in the bounty dashboard modules so the migrated build was truthful. During slice closeout, slice-level verification exposed one remaining contract hole: the migrated app’s `npm run start` script still pointed at Nitro’s `.output/server/index.mjs`, while the current TanStack Start build in this setup emitted `dist/client/` plus `dist/server/server.js`. Rather than widen S01 into a deployment-target rewrite, the slice closed that gap with a package-local `server.mjs` bridge that serves the built client assets and forwards all other requests to TanStack Start’s built fetch handler. Closeout also gated Vercel Analytics behind an explicit opt-in env flag so local production smoke no longer emits predictable `/_vercel/insights/script.js` 404 noise. The result is an honest migration groundwork slice: `frontend-exp` now runs on TanStack Start plumbing in place, the visible shell still renders, `npm run dev`, `npm run build`, and `npm run start` all work, and no backend integration work was introduced. The main remaining product seam is now explicit for S02: sidebar section changes like `Bounties` still happen through local `activeNav` state while the URL remains `/`, so downstream work needs real route decomposition rather than more shell-plumbing changes.

## Verification

Ran `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build` successfully after the closeout fixes; started `npm --prefix ../hyperpush-mono/mesher/frontend-exp run dev` on port 3000 and verified the dashboard shell in the browser; clicked into the `Bounties` subview and confirmed the existing shell content changes while the URL remains `/`; started `PORT=3001 npm --prefix ../hyperpush-mono/mesher/frontend-exp run start` and verified the built production app in the browser; and confirmed both dev and production browser sessions had zero console errors and zero failed requests after gating local analytics noise. Repository-level checks also confirmed the expected `dev` / `build` / `start` scripts are present and there is no remaining authored Next runtime/config file on the active app path.

## Requirements Advanced

- R143 — Advanced the framework-migration parity goal by replacing the active runtime plumbing with TanStack Start/Vite while keeping the visible dashboard shell intact for downstream route-parity work.
- R147 — Advanced the no-Next-runtime goal by making `frontend-exp` build, boot in dev, and boot in production without Next.js on the active runtime path.
- R146 — Reinforced the no-backend-expansion constraint by verifying the migrated shell still renders through the existing mock-data/stateful dashboard surface without introducing backend integration work.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

Closeout verification uncovered two real gaps that the task summaries had not fully closed: `npm run start` still pointed at Nitro’s `.output/server/index.mjs` even though the current TanStack Start build emitted `dist/`, and local production smoke emitted predictable Vercel Analytics 404 noise. S01 fixed both gaps with a package-local bridge server plus analytics opt-in gating rather than widening the slice into route decomposition or deployment-target work.

## Known Limitations

The dashboard still mounts as one TanStack Start `/` route, and sidebar section changes like `Bounties` still come from `app/page.tsx` local `activeNav` state rather than router-backed URLs. `vite build` still warns about large route chunks because the full shell remains mounted behind that one route. The app also still lives at `../hyperpush-mono/mesher/frontend-exp/`; the canonical path move to `mesher/client` is still future work for later M059 slices.

## Follow-ups

S02 should decompose the current shell state branches into real TanStack route files while preserving the same visible URLs, panels, and interactions. S03 should move the canonical app path from `frontend-exp` to `mesher/client` while keeping the same external command names. S04 should clean up any product-repo docs and workflows that still reference the old Next.js or `frontend-exp` operational contract.

## Files Created/Modified

- `../hyperpush-mono/mesher/frontend-exp/package.json` — Replaced the stale Nitro `.output` start target with the closeout-verified package-local production server entry while keeping the external command names unchanged.
- `../hyperpush-mono/mesher/frontend-exp/server.mjs` — Added a Node bridge server that serves built client assets and forwards all non-static requests to TanStack Start’s built fetch handler so `npm run start` works truthfully from `dist/` output.
- `../hyperpush-mono/mesher/frontend-exp/vite.config.ts` — Holds the in-place TanStack Start/Vite runtime configuration established earlier in the slice.
- `../hyperpush-mono/mesher/frontend-exp/src/router.tsx` — Defines the TanStack router entry used by the migrated app.
- `../hyperpush-mono/mesher/frontend-exp/src/routes/__root.tsx` — Provides the migrated root route, linked CSS/font assets, document metadata, and local analytics opt-in gating for clean production smoke.
- `../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx` — Keeps the existing dashboard shell mounted at `/` as the temporary parity adapter for later route decomposition.
- `../hyperpush-mono/mesher/frontend-exp/tsconfig.json` — Supports the new TanStack Start/Vite source layout and bundler-oriented module resolution.
- `../hyperpush-mono/mesher/frontend-exp/app/globals.css` — Continues to carry the existing global dashboard theme and Tailwind-v4 styling under the new runtime path.
- `.gsd/KNOWLEDGE.md` — Captured the production-output and local analytics gotchas future M059 slices should not rediscover.
- `.gsd/PROJECT.md` — Refreshed project state to reflect that M059/S01 is now complete and the migration is in the route-decomposition/path-move phase.
