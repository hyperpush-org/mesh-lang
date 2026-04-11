# M059: Frontend Framework Migration to TanStack Start

**Gathered:** 2026-04-11
**Status:** Ready for planning

## Project Description

M059 migrates the product dashboard app from Next.js to TanStack Start. The target code lives in the sibling product repo under `../hyperpush-mono/mesher/frontend-exp/` today and must land as `../hyperpush-mono/mesher/client/` at the end of the milestone. The user-defined bar is strict: **the design should be identical at the end, only difference should be underlying framework change**.

## Why This Milestone

The current dashboard is still on Next.js 16 even though the product direction is to standardize this surface on TanStack Start. This milestone exists to swap the framework without turning the work into a stealth redesign, data-model rewrite, or backend integration project. Doing this now keeps the frontend stack aligned with the intended framework while the app is still mock-data-driven and largely client-state-based, which makes behavioral equivalence much easier to prove honestly.

## Codebase Brief

### Technology Stack

- Product app target: React 19 + Next.js 16 + Tailwind CSS 4 + TypeScript
- UI/component stack: Radix primitives, lucide-react, recharts, react-hook-form, local utility/components structure
- Data model during migration: local mock-data modules (`lib/mock-data.ts`, `lib/solana-mock-data.ts`)
- Migration target: TanStack Start on the current Vite-based toolchain path

### Key Modules

- `../hyperpush-mono/mesher/frontend-exp/app/page.tsx` — current dashboard shell, nav state, sidebar/panel orchestration, filter state
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/**` — dashboard views and detail panels
- `../hyperpush-mono/mesher/frontend-exp/components/ui/**` — reusable UI primitives currently used by the dashboard
- `../hyperpush-mono/mesher/frontend-exp/lib/mock-data.ts` — authoritative mock data and current domain types during migration
- `../hyperpush-mono/mesher/frontend-exp/app/globals.css` — global styling contract that must remain visually identical after the migration
- `../hyperpush-mono/mesher/frontend-exp/next.config.mjs` and `package.json` — existing framework/runtime contract that must be replaced

### Patterns in Use

- Client-first dashboard shell with local `useState` navigation and panel state
- Mock-data-driven rendering with no real backend dependency on the active path
- Large component reuse surface with one top-level shell page coordinating multiple views
- Current command contract exposed through `npm run dev`, `npm run build`, and `npm run start`

## User-Visible Outcome

### When this milestone is complete, the user can:

- run the dashboard from `../hyperpush-mono/mesher/client/` and see the same design, layout, and styling they see today
- navigate across the same dashboard sections with the same visible URLs, sidebar behavior, panels, filters, and mock-data interactions under TanStack Start

### Entry point / environment

- Entry point: product dashboard web app in `../hyperpush-mono/mesher/client/`
- Environment: local dev and production-like browser runtime
- Live dependencies involved: none on the active path; mock-data semantics remain authoritative

## Completion Class

- Contract complete means: the migrated app lives at `../hyperpush-mono/mesher/client/`, uses TanStack Start instead of Next.js, preserves the `dev` / `build` / `start` contract, and keeps the visible dashboard behavior unchanged
- Integration complete means: TanStack routes, assets, styles, aliases, and the current dashboard shell/components all work together without Next.js on the critical runtime path
- Operational complete means: maintainers can run the migrated app with the normal commands and direct product-repo references to the old path/framework are updated where needed

## Architectural Decisions

### Behavior-Preserving Migration

**Decision:** Treat M059 as a framework migration with exact visual and behavioral parity, not as a redesign or feature milestone.

**Rationale:** The user was explicit: the design should be identical at the end and the only intended change is the underlying framework.

**Evidence:** The current app is mock-data-driven and largely client-state-based, so parity is realistic and easier to verify than if the milestone also changed data sources or product behavior.

**Alternatives Considered:**
- Redesign during migration — rejected because it would make equivalence impossible to prove honestly
- Broaden into backend integration — rejected because it would add scope and hide framework risk behind product changes

### Mutate In Place, Then Rename to `mesher/client`

**Decision:** Convert the existing `frontend-exp` app off Next.js in place, then rename/move the finished app to `../hyperpush-mono/mesher/client/` once parity is proven.

**Rationale:** This keeps the live source of truth visible during the migration and avoids a parallel “fresh port” branch where parity drift is harder to spot.

**Evidence:** The current app is self-contained, with one main shell page and a reusable component tree; direct conversion is mechanically simpler than a second app tree plus long-lived divergence.

**Alternatives Considered:**
- Fresh TanStack app in `mesher/client` from day one — rejected by user preference and because it increases duplicate-tree risk
- Keep final app in `frontend-exp` — rejected because the desired end state is `mesher/client`

### Use Proper TanStack Routes Without Changing the UI

**Decision:** Decompose the dashboard into real TanStack routes while preserving the same visible shell, route destinations, nav structure, and interactions.

**Rationale:** The user explicitly rejected a one-route shell and asked for proper routes, but without any visible UI change.

**Evidence:** The current dashboard already has clear section boundaries (`issues`, `performance`, `solana-programs`, `releases`, `alerts`, `bounties`, `treasury`, `settings`) that can map to route modules while reusing the same shell/layout components.

**Alternatives Considered:**
- Single TanStack route with internal nav state — rejected because it would not satisfy the “proper routes” requirement
- Large route redesign — rejected because it would violate the parity constraint

### Client-First TanStack Start

**Decision:** Keep the migration client-first and avoid introducing server functions/loaders unless a framework seam truly requires them.

**Rationale:** The current app is already mock-data-driven and client-state-based, so introducing server behavior would add risk without helping the migration goal.

**Evidence:** `app/page.tsx` coordinates the dashboard through local state, and the active data path is static/mock-based rather than server-dependent.

**Alternatives Considered:**
- Recast the app around loaders/server functions during migration — rejected as unnecessary churn that could create hydration and behavior drift
- Backend-integrated migration — rejected by scope

## Interface Contracts

- Route contract: the current top-level sections must map to real TanStack routes while preserving the same visible navigation destinations and selected-state behavior
- UI contract: `components/dashboard/**`, `components/ui/**`, `lib/mock-data.ts`, and `app/globals.css` remain the behavioral and visual source surface during migration; migration work should adapt framework seams around them rather than rewrite their product meaning
- Command contract: `npm run dev`, `npm run build`, and `npm run start` remain the maintainer-facing contract on the final `../hyperpush-mono/mesher/client/` app
- Data contract: `lib/mock-data.ts` and related mock modules remain authoritative; no new real Mesher backend integration belongs on the M059 active path

## Error Handling Strategy

The critical failure class in M059 is migration drift, not business-logic failure. The work must fail closed on:

- route drift: same app but changed URLs, changed selected-nav behavior, or broken deep-link semantics
- hydration/render drift: TanStack route decomposition changes visible panel/sidebar/filter behavior after navigation or reload
- styling/asset drift: CSS order, asset resolution, spacing, or layout changes that alter the visible design
- command/runtime drift: `dev` / `build` / `start` no longer behave like the current app contract or Next.js remains on the critical runtime path
- false-success migration: the framework is swapped, but the app has quietly changed behavior or product scope

Recovery strategy:

- keep the current dashboard as the comparison surface until the TanStack app proves parity
- verify route and UI behavior after each migration wave instead of assuming framework changes are neutral
- treat any visible drift as a blocker to fix, not an acceptable side effect of moving off Next.js

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- a maintainer can run the migrated app from `../hyperpush-mono/mesher/client/` using `npm run dev`, `npm run build`, and `npm run start`
- the dashboard still shows the same design, route destinations, navigation structure, sidebar/panel behavior, filters, and mock-data interactions as the current app
- Next.js is no longer required on the critical runtime path for the canonical frontend app

## Testing Requirements

- S01: install/build smoke plus command-contract verification while the app is being converted off Next plumbing
- S02: browser-based verification for route navigation, shell layout, filters, and detail-panel behavior after route decomposition
- S03: final runtime verification from `../hyperpush-mono/mesher/client/` plus static checks that Next.js is no longer the critical runtime dependency
- S04: explicit equivalence verification across key screens/flows plus direct stale-reference checks for maintainer-facing scripts/docs/workflows

## Acceptance Criteria

- S01 must leave the dashboard bootable under TanStack Start migration plumbing without changing the visible shell or the `dev` / `build` / `start` command contract
- S02 must replace the old internal nav-only structure with proper TanStack routes while preserving the same visible UI, URLs, panels, filters, and interactions
- S03 must finalize the move to `../hyperpush-mono/mesher/client/`, remove Next.js from the critical runtime path, and keep the app buildable/startable under the normal commands
- S04 must prove the migrated app is visually and behaviorally equivalent in the important screens/flows and update only the direct operational references that would otherwise become stale or broken

## Risks and Unknowns

- Route decomposition could subtly change selected-nav, panel, or filter behavior — this would violate the scope even if the app still “works”
- CSS/asset import differences between Next.js and TanStack Start could create visible visual drift — exact design parity is the bar
- TanStack Start route generation/hydration issues could appear if the conversion mixes stale framework assumptions with the new route/layout structure
- The product repo path move from `frontend-exp` to `client` could leave stale scripts or maintainer references behind if handled too late

## Existing Codebase / Prior Art

- `../hyperpush-mono/mesher/frontend-exp/app/page.tsx` — current dashboard shell and the main parity reference for navigation/state behavior
- `../hyperpush-mono/mesher/frontend-exp/lib/mock-data.ts` — authoritative current mock-data contract that must stay unchanged during migration
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/sidebar.tsx` — current navigation structure that should become real route-backed navigation without changing the visible UI
- `../hyperpush-mono/mesher/frontend-exp/app/globals.css` — current styling contract that must stay visually identical after the move
- `../hyperpush-mono/mesher/frontend-exp/next.config.mjs` — current framework-specific config that will be replaced as part of the migration

> See `.gsd/DECISIONS.md` for all architectural and pattern decisions — it is an append-only register; read it during planning, append to it during execution.

## Relevant Requirements

- R143 — Migrate the dashboard to TanStack Start with exact visual and behavioral parity
- R144 — Move the canonical app path to `mesher/client` while preserving `dev` / `build` / `start`
- R145 — Preserve current URLs, nav structure, sidebar/panel behavior, filters, and major interactions
- R146 — Keep mock-data semantics authoritative and avoid backend integration expansion
- R147 — Ensure the final app builds/starts without Next.js on the runtime path
- R148 — Update direct product-repo operational references to the new path/framework contract

## Scope

### In Scope

- Converting the existing `frontend-exp` dashboard off Next.js onto TanStack Start
- Decomposing the current dashboard sections into real TanStack routes without changing the visible shell
- Preserving the current design, route destinations, filters, panels, and mock-data semantics
- Finalizing the canonical app at `../hyperpush-mono/mesher/client/`
- Updating direct scripts/docs/workflows that would otherwise break due to the path/framework change

### Out of Scope / Non-Goals

- Any design refresh or UX rethink
- Any new dashboard features or information-architecture changes beyond route decomposition needed for parity
- Any real backend/API integration or data-model rewrite
- Broader product-docs redesign outside direct operational references

## Technical Constraints

- The design should be identical at the end; the only intended change is the underlying framework
- The final canonical path must be `../hyperpush-mono/mesher/client/`
- `npm run dev`, `npm run build`, and `npm run start` must remain the maintainer-facing contract
- Proper TanStack routes are required; a single giant route shell is not the accepted end state
- The migration should stay client-first unless a specific framework seam forces a small exception

## Integration Points

- `../hyperpush-mono/mesher/frontend-exp` — source app being migrated
- `../hyperpush-mono/mesher/client` — final canonical location after rename/move
- Product-repo scripts/docs/workflows — must be updated if they directly reference `frontend-exp` or Next.js
- TanStack Start/Vite route and build tooling — must replace Next app-router/runtime assumptions without changing the user-visible app

## Ecosystem Notes

- The official TanStack Start “Migrate from Next.js” guide recommends moving Next app-router layout/page structure onto TanStack root/file routes, `head()` metadata, `HeadContent`, `Scripts`, and explicit route modules
- Current TanStack Start guidance is Vite-based rather than the older Vinxi path; stale Vinxi-era examples/config should not drive this migration
- Known migration pitfalls include hydration mismatches, route-tree generation/build issues, and stale framework config surviving the swap
- Field reports from recent real migrations emphasize the same pattern this milestone uses: keep components mostly intact, convert framework seams first, and avoid redesigning the app while moving frameworks

## Open Questions

- None at planning time. The remaining discretion is implementation-level: pick the smallest route/config/component changes that preserve exact parity while landing on the TanStack Start contract.
