# S01: In-place TanStack Start conversion groundwork

**Goal:** Replace the current Next-specific project plumbing inside `../hyperpush-mono/mesher/frontend-exp/` with TanStack Start/Vite-compatible plumbing while keeping the visible dashboard shell and normal command contract intact.
**Demo:** After this: the existing dashboard codebase runs on TanStack Start migration plumbing in place, with the current shell still visible and the normal command contract preserved while the app remains at `frontend-exp`.

## Must-Haves

- The existing app tree in `../hyperpush-mono/mesher/frontend-exp/` boots on TanStack Start migration plumbing.
- Global CSS and the current shell render without visible regression.
- The project still exposes `npm run dev`, `npm run build`, and `npm run start`.
- No backend integration work is introduced as part of the framework groundwork.

## Proof Level

- This slice proves: Bootable in-place TanStack Start shell with preserved visible UI and `dev` / `build` / `start` contract.

## Integration Closure

The existing app tree boots on TanStack Start migration plumbing in place, with the root route, router entrypoint, CSS import path, and command scripts established for downstream route work.

## Verification

- Creates a concrete active slice/task unit that the engine can point at, and establishes command/build seams that can be verified immediately instead of leaving M059 as milestone-only metadata.

## Tasks

- [x] **T01: Audit current Next-specific seams** `est:20-30m`
  Read the current `frontend-exp` package/config/root shell files and identify every Next-specific seam that must be replaced for TanStack Start/Vite. Establish the target file/module mapping before editing so the conversion stays in-place rather than turning into a parallel app rewrite.
  - Files: `../hyperpush-mono/mesher/frontend-exp/package.json`, `../hyperpush-mono/mesher/frontend-exp/next.config.mjs`, `../hyperpush-mono/mesher/frontend-exp/app/page.tsx`, `../hyperpush-mono/mesher/frontend-exp/app/layout.tsx`, `../hyperpush-mono/mesher/frontend-exp/app/globals.css`
  - Verify: Confirm the task plan names the existing framework seams and target TanStack replacements without changing visible product behavior.

- [x] **T02: Stand up TanStack Start plumbing in place** `est:60-90m`
  Replace the framework plumbing in place: create the TanStack Start root route/router entry, move global CSS import to the new root, update package/config scripts and dependencies, and preserve aliases/components/mock-data imports so the current dashboard shell can boot under the new framework.
  - Files: `../hyperpush-mono/mesher/frontend-exp/package.json`, `../hyperpush-mono/mesher/frontend-exp/src/routes/__root.tsx`, `../hyperpush-mono/mesher/frontend-exp/src/router.tsx`, `../hyperpush-mono/mesher/frontend-exp/app/globals.css`, `../hyperpush-mono/mesher/frontend-exp/tsconfig.json`
  - Verify: Run the in-place app’s install/build/dev smoke and confirm the visible shell still mounts with the preserved command contract.

- [x] **T03: Verify shell parity and command contract** `est:30-45m`
  Exercise the in-place migrated shell enough to prove the command contract and visible dashboard shell survived the framework swap groundwork. Record the exact seams that remain for route decomposition in S02 instead of widening S01 into route restructuring.
  - Files: `../hyperpush-mono/mesher/frontend-exp/package.json`, `../hyperpush-mono/mesher/frontend-exp/src/routes/__root.tsx`, `../hyperpush-mono/mesher/frontend-exp/src/router.tsx`
  - Verify: Run `npm run dev`, `npm run build`, and a targeted browser/smoke check against the in-place app and confirm no backend integration drift was introduced.

## Files Likely Touched

- ../hyperpush-mono/mesher/frontend-exp/package.json
- ../hyperpush-mono/mesher/frontend-exp/next.config.mjs
- ../hyperpush-mono/mesher/frontend-exp/app/page.tsx
- ../hyperpush-mono/mesher/frontend-exp/app/layout.tsx
- ../hyperpush-mono/mesher/frontend-exp/app/globals.css
- ../hyperpush-mono/mesher/frontend-exp/src/routes/__root.tsx
- ../hyperpush-mono/mesher/frontend-exp/src/router.tsx
- ../hyperpush-mono/mesher/frontend-exp/tsconfig.json
