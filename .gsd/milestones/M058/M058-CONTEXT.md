# M058: Frontend Framework Migration to TanStack Start

**Gathered:** 2026-04-11
**Status:** Ready for planning

## Project Description

This milestone migrates the product dashboard app in the sibling Hyperpush product repo from Next.js to TanStack Start. The app currently lives at `../hyperpush-mono/mesher/frontend-exp/` and will move to `../hyperpush-mono/mesher/client/` as part of the migration. The migration is intentionally narrow: the dashboard should look the same, behave the same, serve the same URLs, keep the same mock-data behavior, and expose the same maintainer command surface. The framework is the thing that changes.

## Why This Milestone

The current dashboard app is a product-owned frontend surface that still runs on Next.js 16 while the desired future stack is TanStack Start. This is the right time to change frameworks because the app is still largely self-contained and mock-data-driven: the risk is mostly in routing, shell, styling, and runtime conventions, not in deep backend coupling. Doing the migration now keeps later real-data work from becoming entangled with framework replacement and preserves a clean behavioral baseline for future integration.

## Codebase Brief

### Technology Stack

- Product-owned app in the sibling repo: `../hyperpush-mono/mesher/frontend-exp/`
- Current framework: Next.js 16, React 19, TypeScript
- UI stack: shadcn/Radix primitives, Tailwind CSS v4, Recharts, lucide-react
- Planned framework: TanStack Start with TanStack Router file-based routing
- Current data path: local mock-data modules (`lib/mock-data.ts`, `lib/solana-mock-data.ts`)

### Key Modules

- `../hyperpush-mono/mesher/frontend-exp/app/page.tsx` — current dashboard entrypoint and shell composition
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/*` — current dashboard pages, panels, and interaction surfaces
- `../hyperpush-mono/mesher/frontend-exp/components/ui/*` — shared UI primitives to preserve where possible
- `../hyperpush-mono/mesher/frontend-exp/lib/mock-data.ts` — current behavior/data truth source
- `../hyperpush-mono/mesher/frontend-exp/lib/solana-mock-data.ts` — Solana-specific mock surface that must migrate unchanged
- `../hyperpush-mono/README.md` — repo-root product ownership and maintainer contract

### Patterns in Use

- Client-heavy React app mounted from one dominant dashboard route
- Local mock-data imports as the current data boundary
- Many reusable UI components already separated from framework shell concerns
- Current maintainer contract expects stable `npm run dev`, `npm run build`, and `npm run start` commands
- Split-repo ownership from M055: product surfaces stay in the product repo, not in `mesh-lang`

## User-Visible Outcome

### When this milestone is complete, the user can:

- Open the migrated dashboard app under `mesher/client` and see the same interface, routes, sections, and interactions they saw before the migration
- Run the same `npm run dev`, `npm run build`, and `npm run start` commands and get the same product behavior, with TanStack Start underneath instead of Next.js

### Entry point / environment

- Entry point: the product dashboard app currently rooted at `../hyperpush-mono/mesher/frontend-exp/`, renamed to `../hyperpush-mono/mesher/client/`
- Environment: local browser + product-repo build/start commands
- Live dependencies involved: none required for this milestone beyond the app runtime itself; current mock-data behavior remains authoritative

## Completion Class

- Contract complete means: the app is framework-migrated, renamed to `client`, and preserves current routes, visuals, interactions, and command contract
- Integration complete means: the app shell, route tree, component imports, styling, and runtime commands all work together under TanStack Start without relying on Next.js
- Operational complete means: maintainers can use the same `dev` / `build` / `start` command surface from the new app path and the product repo’s direct references point at the new truthful location

## Architectural Decisions

### Framework-only migration boundary

**Decision:** Treat M058 as an equivalence-focused framework migration, not as a feature, backend, or design milestone.

**Rationale:** The user explicitly wants the app to look identical and work exactly the same, with the framework as the only substantive difference.

**Evidence:** The current app is mostly mock-data-driven and client-heavy, so the migration risk is concentrated in framework seams rather than product logic.

**Alternatives Considered:**
- Expand into backend integration during the migration — rejected because it would make equivalence proof ambiguous.
- Redesign the dashboard while migrating — rejected because it would violate the milestone boundary.

### File-based TanStack Start adoption

**Decision:** Use TanStack Start with file-based TanStack Router routes rather than a manual route tree.

**Rationale:** This keeps the migration close to the current file-router mental model, reduces review complexity, and matches the current TanStack guidance for Next.js migrations.

**Evidence:** Current TanStack Start migration documentation maps Next App Router concepts cleanly into file-based TanStack route files and root-route structure.

**Alternatives Considered:**
- Manual/code-defined route tree — rejected because it adds migration complexity without helping the parity bar.
- Partial hybrid runtime with lingering Next routing — rejected because the milestone should leave one truthful framework path.

### Rename to `mesher/client` while preserving command parity

**Decision:** Rename the app directory from `mesher/frontend-exp` to `mesher/client`, but keep the external `dev` / `build` / `start` command names stable.

**Rationale:** The new path should reflect the app’s canonical role after the migration, while the maintainer/operator contract should remain simple and familiar.

**Evidence:** The user explicitly approved the rename and explicitly requested that the commands remain the same.

**Alternatives Considered:**
- Keep the old `frontend-exp` path — rejected because the milestone intentionally renames the surface.
- Change command names to match framework internals — rejected because it would create needless operational drift.

## Interface Contracts

- The canonical app path changes from `../hyperpush-mono/mesher/frontend-exp/` to `../hyperpush-mono/mesher/client/`.
- The migrated app must preserve the same user-facing URLs and route outcomes as the current app.
- The migrated app must preserve current mock-data module semantics: `lib/mock-data.ts` and `lib/solana-mock-data.ts` remain the current truth source.
- The migrated app must preserve sidebar, header, filter, detail-panel, and section-switching behavior.
- The maintainer contract remains `npm run dev`, `npm run build`, and `npm run start` from the app directory.

## Error Handling Strategy

The critical failure mode for M058 is silent drift. The milestone should fail closed on route drift, visible render drift, interaction drift, or command/runtime drift.

- Route mismatches are blockers.
- Meaningful visual differences are blockers.
- Sidebar/panel/filter/navigation regressions are blockers.
- Stale references to `frontend-exp` or Next.js in product-repo docs/workflows are blockers where they directly describe the migrated app.
- Internal file moves and import rewrites are acceptable if they do not change user-visible behavior.
- New backend integration, new product features, or intentional route/data changes are out of scope and should be deferred instead of smuggled in as “cleanup.”

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- the app has moved to `../hyperpush-mono/mesher/client/` and runs on TanStack Start instead of Next.js
- the migrated app serves the same URLs, renders the same dashboard surface, and preserves the same interactions under the current mock-data contract
- maintainers can run `npm run dev`, `npm run build`, and `npm run start` from the new app path and get the same product behavior they had before the migration

## Testing Requirements

- Build/smoke verification for the new TanStack Start app shell and root route
- Browser verification for route parity, sidebar/nav parity, filter/detail-panel parity, and major section rendering parity
- Command/runtime verification for `dev`, `build`, and `start`
- Equivalence verification that compares pre-migration and post-migration behavior on the main dashboard surface
- Direct reference verification for product-repo docs/workflows that mention the old path or framework

## Acceptance Criteria

### S01 — TanStack Start shell cutover
- `mesher/client` exists as the new app root.
- The app boots under TanStack Start and shows the same top-level dashboard shell.
- Global styles, fonts, and providers preserve the current visual shell.
- No Next.js root runtime is required for the migrated app to render.

### S02 — Route and interaction parity
- The main dashboard route resolves at the same URL as before.
- Major dashboard sections reachable from the existing nav still render.
- Sidebar, header, panel, and filter interactions behave the same as before.
- Current mock-data-backed pages retain the same displayed structure and interaction behavior.

### S03 — Package and command contract parity
- `npm run dev` works from `mesher/client`.
- `npm run build` succeeds for the migrated app.
- `npm run start` serves the built app successfully.
- Next.js is no longer the app framework/runtime on the critical path.

### S04 — Behavioral equivalence proof
- Before/after checks show no meaningful visual drift on the main dashboard surface.
- Before/after checks show no meaningful interaction drift on navigation, filters, or detail panels.
- Product-repo docs/workflows now point to the truthful `client` + TanStack Start contract.

## Risks and Unknowns

- Root layout/document/head differences between Next.js and TanStack Start could create subtle shell drift — this matters because the milestone is judged on equivalence, not just successful boot.
- Route-file conversion could accidentally change URL behavior or default page mounting — this matters because current URLs are part of the explicit promise.
- Global CSS/provider import changes could create visible spacing, font, or panel drift — this matters because visual parity is a blocker.
- Rename fallout from `frontend-exp` to `client` could leave stale docs/workflows or broken scripts — this matters because operational truth is part of the milestone.

## Existing Codebase / Prior Art

- `../hyperpush-mono/mesher/frontend-exp/app/page.tsx` — current dashboard shell and interaction entrypoint
- `../hyperpush-mono/mesher/frontend-exp/lib/mock-data.ts` — current behavior/data truth source
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/*` — existing pages and interactions to preserve
- `../hyperpush-mono/README.md` — product-repo ownership and maintainer contract
- `WORKSPACE.md` — split-repo ownership rules that keep this work product-owned

> See `.gsd/DECISIONS.md` for all architectural and pattern decisions — it is an append-only register; read it during planning, append to it during execution.

## Relevant Requirements

- R143 — framework migration without meaningful user-visible change
- R144 — rename to `mesher/client` while preserving command parity
- R145 — preserve URLs and interaction behavior
- R146 — preserve current mock-data semantics and keep backend expansion out of scope
- R147 — app builds and starts under TanStack Start without Next.js on the critical path
- R148 — update direct docs/workflow references to the new truthful contract
- R149 — later real-data integration remains deferred until after parity is proven
- R150, R151, R152 — redesign, feature expansion, and intentional URL/data changes remain out of scope

## Scope

### In Scope

- Migrating the product dashboard app from Next.js to TanStack Start
- Renaming the app path from `mesher/frontend-exp` to `mesher/client`
- Preserving current URLs, shell, sections, interactions, and mock-data behavior
- Preserving the current external `dev` / `build` / `start` command contract
- Updating direct product-repo docs/workflows that describe the migrated app

### Out of Scope / Non-Goals

- Redesigning the dashboard UI or information architecture
- Adding new product features, pages, or data domains during the migration
- Expanding into Mesher backend integration in the same milestone
- Intentionally changing current URLs or current mock-data behavior

## Technical Constraints

- The work stays in the sibling product repo; product ownership does not move back into `mesh-lang`.
- The migration must preserve the same user-visible behavior and current route contract.
- The migration keeps current mock-data modules authoritative for this milestone.
- Command names remain `npm run dev`, `npm run build`, and `npm run start`.
- Internal file structure may change as needed to fit TanStack Start.

## Integration Points

- `../hyperpush-mono/mesher/frontend-exp/` — current app source to migrate and rename
- `../hyperpush-mono/mesher/client/` — destination app path after migration
- `../hyperpush-mono/README.md` — direct maintainer-facing path references that may need updating
- product CI/workflows in `../hyperpush-mono/.github/workflows/` — direct references to the old app path or framework contract if present

## Ecosystem Notes

- Current TanStack Start migration guidance provides a direct Next.js migration path, especially around root document/layout replacement and route-file naming.
- File-based TanStack Router routes are the cleanest migration target for this app because they preserve the same mental model as the current file-based shell without requiring a manual route-tree rewrite.
- Recent migration guidance consistently recommends not redesigning while migrating frameworks; keeping routing/data behavior fixed is the safest path.
- Package script parity under TanStack Start typically maps cleanly to `vite dev`, `vite build`, and `node .output/server/index.mjs`, which supports the user’s request to preserve command names even while framework internals change.

## Open Questions

- Whether any direct product CI/deploy workflows assume Next.js-specific output semantics beyond the app-local `dev` / `build` / `start` contract — current expectation is that this will be limited and should be handled in S03/S04.
