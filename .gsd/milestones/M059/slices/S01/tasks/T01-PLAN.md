---
estimated_steps: 1
estimated_files: 5
skills_used: []
---

# T01: Audit current Next-specific seams

Read the current `frontend-exp` package/config/root shell files and identify every Next-specific seam that must be replaced for TanStack Start/Vite. Establish the target file/module mapping before editing so the conversion stays in-place rather than turning into a parallel app rewrite.

## Inputs

- `.gsd/milestones/M059/M059-CONTEXT.md`
- `.gsd/milestones/M059/M059-ROADMAP.md`
- `../hyperpush-mono/mesher/frontend-exp/package.json`
- `../hyperpush-mono/mesher/frontend-exp/app/page.tsx`

## Expected Output

- `.gsd/milestones/M059/slices/S01/tasks/T01-PLAN.md`

## Verification

Confirm the task plan names the existing framework seams and target TanStack replacements without changing visible product behavior.

## Audit Findings

### Confirmed Next-specific seams in `frontend-exp`

1. **Runtime / command contract**
   - `package.json` uses `next dev`, `next build`, and `next start`.
   - `next` is a direct runtime dependency.
2. **Root framework shell**
   - `app/layout.tsx` is the current Next app-router root.
   - It uses `Metadata` from `next`, `Geist` / `Geist_Mono` from `next/font/google`, and `Analytics` from `@vercel/analytics/next`.
   - It imports `./globals.css` from the Next `app/` tree.
3. **Root route content**
   - `app/page.tsx` is the current `/` route implementation and contains the entire visible dashboard shell.
   - It is explicitly client-side today via `"use client"`.
4. **TypeScript / generated-type coupling**
   - `tsconfig.json` includes the Next TypeScript plugin and `.next/**` generated types.
   - `next-env.d.ts` references Next image/global route types.
5. **Framework config file**
   - `next.config.mjs` carries current framework-only behavior (`ignoreBuildErrors`, `images.unoptimized`).
6. **Tooling metadata coupled to the Next app tree**
   - `components.json` points shadcn CSS at `app/globals.css` and marks the project as `rsc: true`.
   - `tailwind.config.ts` still scans `./app/**/*` and uses Next font-variable names in `fontFamily`.

### Confirmed non-seams / migration constraints

- No active `next/link`, `next/image`, or `next/navigation` imports were found in the dashboard/component tree.
- The current shell is mock-data-driven and lives almost entirely in plain React components under `components/**` and `lib/**`.
- The many `"use client"` directives are a migration cleanup concern, but they are not the primary blocker for the framework plumbing swap.

## Target In-Place Mapping for TanStack Start/Vite

| Current file / seam | TanStack Start / Vite target | Why this is the right in-place replacement |
|---|---|---|
| `package.json` scripts using `next *` | `vite dev`, `vite build`, `node .output/server/index.mjs` | Preserves the maintainer-facing `dev` / `build` / `start` contract while removing Next from the runtime path. |
| `next` dependency | `@tanstack/react-router`, `@tanstack/react-start`, `vite`, and required TanStack/Vite glue | Replaces the framework/runtime host instead of creating a parallel app. |
| `app/layout.tsx` | `src/routes/__root.tsx` | TanStack Start’s root route owns document structure, head metadata, stylesheet links, and script injection. |
| `app/globals.css` import from Next layout | CSS linked from `src/routes/__root.tsx` (imported as stylesheet URL) | Preserves the exact styling contract while moving the import to the Start root document. |
| `app/page.tsx` | `src/routes/index.tsx` that initially renders the same dashboard shell | Keeps the current visible `/` page intact while moving route ownership to file-based TanStack routes. |
| Next `metadata` export in `app/layout.tsx` | `head()` config on `src/routes/__root.tsx` | TanStack Start replaces Next metadata with router-managed head definitions. |
| `@vercel/analytics/next` usage | Remove for initial plumbing unless a Start-compatible analytics integration is added later | Prevents keeping a Next-only import on the critical runtime path. |
| `next/font/google` usage | Fallback to CSS/font-stack-defined fonts in `globals.css` during plumbing, then optionally reintroduce non-Next font loading later | Avoids carrying a Next-only font loader into the migrated root. |
| `next.config.mjs` | Remove/replace with Vite/TanStack config only if a live setting is still needed | The current file is framework-specific and should not survive the runtime swap unchanged. |
| `tsconfig.json` Next plugin / `.next` includes | Plain Vite/TanStack TypeScript settings plus generated router types | Removes stale Next build-type coupling. |
| `next-env.d.ts` | Remove once Vite/Start typing is in place | It only exists to support Next-generated ambient types. |
| `components.json` CSS + `rsc` settings | Point CSS to the migrated global stylesheet location and drop stale Next/RSC assumptions if they conflict | Keeps the UI generator config aligned with the new root without touching visible UI. |
| `tailwind.config.ts` `app/**/*` scan | Include `src/**/*` and preserve existing component/lib coverage | Ensures utilities continue to compile after routes move out of the Next app tree. |

## Execution Notes for T02

- Keep the app **in place** at `../hyperpush-mono/mesher/frontend-exp/`; do not scaffold a second frontend.
- Preserve `app/globals.css`, `components/**`, `hooks/**`, and `lib/**` as the visual/behavioral source of truth.
- Treat `src/routes/index.tsx` as a temporary parity adapter around the current dashboard shell so S02 can later decompose the sections into proper TanStack routes without a second shell rewrite.
- Do **not** introduce backend loaders/server functions during the plumbing pass; the current app is client-first and mock-data-backed.
