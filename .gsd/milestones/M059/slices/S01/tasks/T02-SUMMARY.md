---
id: T02
parent: S01
milestone: M059
key_files:
  - mesher/frontend-exp/package.json
  - mesher/frontend-exp/package-lock.json
  - mesher/frontend-exp/vite.config.ts
  - mesher/frontend-exp/tsconfig.json
  - mesher/frontend-exp/src/router.tsx
  - mesher/frontend-exp/src/routes/__root.tsx
  - mesher/frontend-exp/src/routes/index.tsx
  - mesher/frontend-exp/app/layout.tsx
  - mesher/frontend-exp/app/globals.css
  - mesher/frontend-exp/components/dashboard/bounties-page.tsx
  - mesher/frontend-exp/components/dashboard/bounty-list.tsx
  - mesher/frontend-exp/components/dashboard/bounty-detail.tsx
  - .gsd/KNOWLEDGE.md
key_decisions:
  - D495: keep the current dashboard shell mounted through `src/routes/index.tsx` with `ssr: false`, and load Geist fonts from Fontsource stylesheet links in `src/routes/__root.tsx` instead of relying on Next font injection or importing font CSS through `app/globals.css`.
duration: 
verification_result: passed
completed_at: 2026-04-11T07:20:50.533Z
blocker_discovered: false
---

# T02: Replaced `frontend-exp`’s Next runtime plumbing with TanStack Start/Vite root and router wiring while preserving the dashboard shell, command contract, and Geist fonts.

**Replaced `frontend-exp`’s Next runtime plumbing with TanStack Start/Vite root and router wiring while preserving the dashboard shell, command contract, and Geist fonts.**

## What Happened

Converted `mesher/frontend-exp` from a Next-rooted app into an in-place TanStack Start/Vite app by rewriting scripts/dependencies, adding `vite.config.ts`, `src/router.tsx`, `src/routes/__root.tsx`, and a thin client-only `src/routes/index.tsx` adapter that mounts the existing dashboard shell at `/`. Updated TypeScript config for the new layout, removed obsolete Next-only root/config files, restored Geist/Geist Mono with Fontsource stylesheet links from the TanStack root route, and fixed a latent build failure where three bounty components imported `BountyClaimStatus` as a runtime value instead of a type. Recorded decision D495 for the client-only shell adapter plus root-asset pattern and added the Tailwind-v4/Fontsource gotcha to `.gsd/KNOWLEDGE.md`.

## Verification

Verified the real plumbing end-to-end with the app’s install/build/dev seams and browser checks. `npm --prefix mesher/frontend-exp install` completed on the new dependency graph, `npm --prefix mesher/frontend-exp run build` passed with both client and SSR output after the TanStack/Tailwind fixes, `npm --prefix mesher/frontend-exp run dev` booted successfully on localhost, browser assertions confirmed the expected dashboard shell text at `/`, runtime font diagnostics confirmed loaded `Geist` and `Geist Mono` faces, and a fresh dev-server restart followed by browser console/network inspection showed a clean runtime state.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix mesher/frontend-exp install` | 0 | ✅ pass | 30800ms |
| 2 | `npm --prefix mesher/frontend-exp run build` | 0 | ✅ pass | 4900ms |
| 3 | `bg_shell start/wait_for_ready -> npm --prefix mesher/frontend-exp run dev` | 0 | ✅ pass | 5000ms |
| 4 | `browser_assert url/text checks against http://localhost:3000/` | 0 | ✅ pass | 0ms |
| 5 | `browser_evaluate document.fonts / computed font family` | 0 | ✅ pass | 0ms |
| 6 | `browser_get_console_logs + browser_get_network_logs after fresh dev-server restart` | 0 | ✅ pass | 0ms |

## Deviations

Added `vite.config.ts` and `src/routes/index.tsx`, removed `next.config.mjs` / `next-env.d.ts`, and fixed the latent type-only import mismatch in the bounty dashboard modules because those local adaptations were required to make the in-place TanStack Start build boot truthfully.

## Known Issues

None.

## Files Created/Modified

- `mesher/frontend-exp/package.json`
- `mesher/frontend-exp/package-lock.json`
- `mesher/frontend-exp/vite.config.ts`
- `mesher/frontend-exp/tsconfig.json`
- `mesher/frontend-exp/src/router.tsx`
- `mesher/frontend-exp/src/routes/__root.tsx`
- `mesher/frontend-exp/src/routes/index.tsx`
- `mesher/frontend-exp/app/layout.tsx`
- `mesher/frontend-exp/app/globals.css`
- `mesher/frontend-exp/components/dashboard/bounties-page.tsx`
- `mesher/frontend-exp/components/dashboard/bounty-list.tsx`
- `mesher/frontend-exp/components/dashboard/bounty-detail.tsx`
- `.gsd/KNOWLEDGE.md`
