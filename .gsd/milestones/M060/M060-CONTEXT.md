# M060: Mesher Client Live Backend Wiring

**Gathered:** 2026-04-11
**Status:** Ready for planning

## Project Description

`../hyperpush-mono/mesher/client` is now the canonical dashboard package, but it is still broadly a shell built on `lib/mock-data.ts` and `lib/solana-mock-data.ts`. This milestone turns that shell into a real Mesher-backed dashboard by wiring every surface that already has an existing Mesher backend route, while changing as little UI as possible.

The intent is not to redesign the frontend and not to expand the backend to cover every product-shaped mock screen. The intent is to replace the fake seams with real seams wherever the backend already exists, keep the rest of the shell intact, and fix backend breakage only when it directly blocks the live wiring path.

## Why This Milestone

M059 finished the framework migration and package move, but the dashboard is still mostly mock-data driven. That means the canonical app path is structurally correct while still not proving the backend/client contract that future work depends on.

This milestone needs to happen now to unblock future product work on top of a real data seam. Once `mesher/client` is actually wired to the existing Mesher backend, later frontend and product work can build on truthful client/server behavior instead of a static shell.

## User-Visible Outcome

### When this milestone is complete, the user can:

- open the canonical dashboard and exercise every existing backend-backed surface against live Mesher data in a seeded local environment
- keep using the current shell layout and mocked product-only areas while the real backend-backed routes, lists, detail panels, forms, and actions operate against the existing Mesher backend

### Entry point / environment

- Entry point: `npm --prefix ../hyperpush-mono/mesher/client run dev` together with the Mesher backend entrypoint under `../hyperpush-mono/mesher/`
- Environment: seeded local dev browser environment
- Live dependencies involved: Mesher HTTP backend, seeded project/org/API-key reality, and the backend database used by Mesher

## Completion Class

- Contract complete means: the client-side adapter seam, route data loading, action wiring, and failure handling all match the backend routes that already exist
- Integration complete means: the canonical dashboard shell works against the real Mesher backend across all backend-backed areas without redesigning the shell
- Operational complete means: a seeded local environment can boot and support the full backend-backed shell walkthrough end to end

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- the dashboard boots against a real seeded/default project/org/API-key context without adding a polished login/session flow
- every dashboard surface with an existing Mesher backend route uses the real backend for reads and existing writes
- the full backend-backed shell walkthrough works in a seeded local environment while still-mocked UI remains present and visually stable

## Architectural Decisions

### Existing backend routes define the live scope

**Decision:** Wire `mesher/client` to the existing Mesher backend functionality only, and allow backend fixes only where a currently existing route or contract is broken enough to block the live shell.

**Rationale:** The user explicitly wants this milestone centered on connecting the client to the existing backend, fixing just enough to work, and not widening into a new backend feature wave.

**Alternatives Considered:**
- Add new backend routes for currently mocked product screens — rejected because it expands scope and blurs the milestone into product growth instead of integration
- Keep the client mostly mock-driven and prove only one route — rejected because the user wants the full backend-backed shell, not a thin proof

### Real context means seeded backend reality, not new auth UX

**Decision:** Use the backend's existing project/org/API-key reality with a seeded or default real context instead of adding a polished dashboard login/session flow.

**Rationale:** The user explicitly chose the existing backend reality and does not want this milestone to widen into a login/session product surface.

**Alternatives Considered:**
- Build a new dashboard login/session flow — rejected because it is a separate milestone-sized concern
- Hardcode fake auth/session state — rejected because the milestone is supposed to create a real integration seam

### Mixed live/mock shell stays visually stable

**Decision:** Keep the current shell layout and mocked UI in place, make backend-backed areas live, and leave mock-only product areas quiet instead of removing or loudly splitting them.

**Rationale:** The user explicitly said to change as little UI as possible and to not remove UI that is still mocked.

**Alternatives Considered:**
- Remove unsupported sections from the shell — rejected because it shrinks scope the wrong way and breaks shell continuity
- Redesign mixed screens to call out real versus mock sections aggressively — rejected because the milestone is not a UX rewrite

### Failure visibility should be minimal but truthful

**Decision:** Use straightforward in-place failure handling and shadcn/Radix toast-style notifications when backend-backed calls fail.

**Rationale:** Once the shell goes live, silent mock-like fallback would be dishonest. The user wants visible but minimal failure handling without redesigning the shell.

**Alternatives Considered:**
- Quiet fallback to empty or stale mock-style state — rejected because it hides real integration failures
- Build a louder operational/debug console into the shell — rejected because it over-rotates the milestone into diagnostics UX

## Error Handling Strategy

Backend-backed reads and writes should fail visibly but minimally. The client should keep the current shell intact, preserve operator context where practical, and surface failures through toast-style notifications and local empty/error states rather than by tearing down the page or silently falling back to fake data.

When the backend contract is slightly broken, fix the backend only enough to make the current live path work. Do not use backend breakage as a pretext for broad cleanup or route expansion. Unsupported mock-only surfaces should remain visually present and should not pretend to be live.

## Risks and Unknowns

- The current client prop shapes may not match the raw Mesher route payloads cleanly — that can force a larger normalization seam than the current shell suggests
- Some existing backend routes may exist but still fail on real dashboard usage — the milestone allows seam repairs, but too many breakages would increase slice scope
- The settings shell is broader than the currently visible Mesher backend contract — preserving the UI while making only the backed subsections real will require careful boundary handling
- The project/org selector UI currently looks richer than the confirmed seeded/default backend context — keeping that shell honest without redesign is a real integration constraint

## Existing Codebase / Prior Art

- `../hyperpush-mono/mesher/client/components/dashboard/` — existing shell, route pages, and mock-data-driven UI that should change as little as possible
- `../hyperpush-mono/mesher/client/lib/mock-data.ts` — current fake data contract that will need selective replacement with live adapter models
- `../hyperpush-mono/mesher/client/src/routes/` — current TanStack route structure established in M059
- `../hyperpush-mono/mesher/main.mpl` — existing Mesher HTTP route registration for the backend-backed dashboard surfaces
- `../hyperpush-mono/mesher/api/search.mpl` — issue and event search/listing routes already available to the dashboard
- `../hyperpush-mono/mesher/api/dashboard.mpl` — dashboard summary routes already available to the dashboard
- `../hyperpush-mono/mesher/api/alerts.mpl`, `api/settings.mpl`, `api/team.mpl` — existing admin/ops routes for alerts, settings/storage, team membership, and API keys
- `.gsd/DECISIONS.md` decisions D465-D485 and D503-D505 — prior frontend/backend integration work in `frontend-exp` that already established the existing-backend-only scope discipline and the canonical `mesher/client` package path

## Relevant Requirements

- R153 — make every existing backend-backed dashboard surface use the real Mesher backend
- R154 — make existing backend-backed dashboard writes work end to end
- R155 — use seeded/default real backend context instead of new auth UX
- R156 — keep UI changes minimal
- R157 — keep mocked UI present and visually stable
- R158 — use minimal truthful failure visibility with toast-style feedback
- R159 — limit backend fixes to seam repairs
- R160 — prove the full backend-backed shell in a seeded local environment

## Scope

### In Scope

- wiring all existing Mesher backend routes into `mesher/client`
- replacing mock seams with live adapter seams where the backend already exists
- making existing backend-backed mutations/actions work from the client
- using a seeded/default real context based on existing backend project/org/API-key reality
- preserving the shell layout, route structure, and mocked UI that does not yet have backend support
- making small backend fixes only when they directly unblock the live shell

### Out of Scope / Non-Goals

- building a polished human login/session flow
- redesigning or substantially restyling the dashboard shell
- deleting mocked UI because it is not yet backend-backed
- expanding the backend to support product-only screens that still have no existing route
- turning this into a broad backend cleanup campaign

## Technical Constraints

- Change as little UI as possible while still connecting the existing backend functionality
- Some things in the backend may be broken; fix just enough to work
- Do not remove UI that is still mocked; keep everything as is and wire it together as best as possible
- Keep the existing project selector UI and shell structure even if the real context is narrower than the current mock shell implies
- The final proof environment is seeded local dev, not a new production deployment story

## Integration Points

- `../hyperpush-mono/mesher/client` — canonical TanStack dashboard package that must stop depending on broad mock-data seams
- `../hyperpush-mono/mesher/main.mpl` — Mesher HTTP entrypoint registering the backend-backed dashboard routes
- `../hyperpush-mono/mesher/api/search.mpl` — issue/event list and detail-adjacent data
- `../hyperpush-mono/mesher/api/dashboard.mpl` — dashboard cards/charts/summaries
- `../hyperpush-mono/mesher/api/alerts.mpl` — alert rules and fired alert lifecycle
- `../hyperpush-mono/mesher/api/settings.mpl` — project settings and storage visibility
- `../hyperpush-mono/mesher/api/team.mpl` — team membership and API key management
- Mesher backing database — seeded local data required for truthful browser proof

## Testing Requirements

Use the strongest proof tier available in the canonical package path. Verification should include route-level/browser-level proof in `mesher/client`, targeted contract checks for any client-side normalization seam, and live replay against the seeded Mesher backend. The milestone should end with one assembled seeded-local shell walkthrough covering all backend-backed surfaces, plus targeted regression checks for any backend seam repairs discovered during wiring.

## Acceptance Criteria

- S01 proves the client can boot against a seeded/default real context and render live issues/events data through the existing shell
- S02 proves issue actions and dashboard summaries are sourced from the real backend and survive real writes/fetch failures honestly
- S03 proves alerts, settings/storage, team, and API-key surfaces use the real backend wherever a route already exists, while mock-only sections remain visually stable
- S04 proves the full backend-backed shell works in a seeded local environment end to end, with only minimal backend seam repairs and no redesign/removal drift

## Open Questions

- Exactly how much of the current settings shell can be made fully real without any visible UI adjustment beyond wiring and failure handling — answer this during execution by mapping each tab/control against existing backend routes
- Whether the existing project selector can stay purely cosmetic on top of the seeded/default context or whether a low-cost truthful switching seam already exists — answer this from the actual backend data model during S01
