# M060 / S02 — Research

**Date:** 2026-04-11

## Summary

S02 directly targets **R153** and is the first slice that can materially advance **R154** without widening into S03. The good news is that the backend scope is already honest: Mesher already exposes the issue mutation routes and the dashboard summary routes this slice needs. The client does **not** need new routing, new auth, or a second data-loading architecture. The missing seam is client orchestration.

S01 left the canonical Issues route in a good place for this: `DashboardIssuesStateProvider` already owns overview bootstrap, selected-issue hydration, fallback overlays, and destructive toasts. The provider is shell-scoped, not route-loader scoped, so S02 should stay there. Right now that provider is strictly read-only, `mesher-api.ts` is GET-only, and `IssueDetail` still renders decorative action chrome (`AI Analysis`, GitHub link, bounty) rather than real maintainer controls. The backend mutation routes live in a different module than the read routes, so the client has to treat reads and writes as two separate seams.

The main constraint is semantic drift between backend mutation vocabulary and the current shell type system. The UI can represent `open | ignored | resolved | regressed | in-progress`, while live backend reads only normalize `unresolved -> open`, `resolved -> resolved`, and `archived -> ignored`. Backend `discarded` is currently **unmapped**, and `assign` needs a `user_id` plus org-member context that the current default-project bootstrap does not provide. That makes the low-risk S02 action set: **resolve / unresolve / archive-as-ignore**, with assignment only if org member identity can be reached cheaply without pulling S03 forward.

## Recommendation

Keep S02 inside the existing **provider-owned client state** and **same-origin `/api/v1` transport** established in S01.

Concretely:
- Extend `mesher/client/lib/mesher-api.ts` with issue mutation helpers and any extra dashboard summary reads you actually decide to render.
- Extend `mesher/client/components/dashboard/dashboard-issues-state.tsx` with reusable `refreshOverview()` + mutation orchestration + snapshot invalidation + toast feedback.
- Wire minimal maintainer controls into `mesher/client/components/dashboard/issue-detail.tsx` using the existing button row, keeping UI changes small.
- Reuse the existing stats/chart shell before inventing any new summary UI. If a summary surface is not already rendered on the Issues page, do **not** promote the AI side panel or other mock-only summaries to “live” in S02.

This follows the current codebase shape and the loaded skill guidance:
- **React best practices**: keep async work centralized and parallelized instead of scattering fetches through multiple components.
- **TanStack Router best practices**: do not migrate this slice into route loaders just because data is live now; the current shell is intentionally client-owned and mutation-heavy.
- **Playwright best practices**: extend the existing real-backend harness and failure injection patterns instead of inventing a separate test lane.

## Implementation Landscape

### Key Files

- `../hyperpush-mono/mesher/client/lib/mesher-api.ts` — Current browser transport boundary. It already normalizes Mesher GET failures into `MesherApiError` and centralizes same-origin `/api/v1` fetches. It is **read-only** today. S02 should add POST helpers for supported issue actions here, not in component code.
- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-issues-state.tsx` — Current source of truth for issues overview bootstrap, selected-issue hydration, snapshot caching, filter state, and destructive toasts. This is the natural place for action pending state, refresh/invalidation, and post-mutation reconciliation.
- `../hyperpush-mono/mesher/client/components/dashboard/issue-detail.tsx` — Current detail panel UI. The action row is present, but it only renders AI/GitHub/bounty affordances. `ActionButton` has no disabled/loading state yet, so real issue actions will need a small local button-state expansion here.
- `../hyperpush-mono/mesher/client/lib/issues-live-adapter.ts` — Owns the shell-status translation seam. Important detail: it maps `archived -> ignored` but does **not** map `discarded`. Any action that changes live issue status must still flow back through this adapter or equivalent provider normalization.
- `../hyperpush-mono/mesher/main.mpl` — Router truth. Confirms the existing live scope for S02: issue writes at `/api/v1/issues/:id/{resolve,archive,unresolve,assign,discard,delete}` plus dashboard reads at `/api/v1/projects/:project_id/dashboard/{health,levels,volume,top-issues,tags}`.
- `../hyperpush-mono/mesher/ingestion/routes.mpl` — Current issue mutation handlers. They return only `{"status":"ok"}` success bodies and broadcast websocket updates that the client does not currently consume. That strongly favors provider refetch/invalidation over optimistic patching.
- `../hyperpush-mono/mesher/api/search.mpl` — Current live issues read path (`handle_search_issues`). This is the route S01 already uses. Reads and writes come from different backend modules, so S02 should not assume one shared backend contract file.
- `../hyperpush-mono/mesher/api/dashboard.mpl` — Existing summary routes. `health`, `levels`, and `volume` already feed S01. `top-issues` and `tags` exist if S02 truly needs richer summary data without adding backend scope.
- `../hyperpush-mono/mesher/storage/queries.mpl` — Backend mutation semantics. `resolve_issue`, `archive_issue`, `unresolve_issue`, `assign_issue`, `discard_issue`, and `delete_issue` already exist. Note that `discard_issue` writes status `discarded`, which the current client cannot represent.
- `../hyperpush-mono/mesher/client/components/dashboard/stats-bar.tsx` — Current Issues-page summary chrome. It already uses provider data and exposes observability attributes. Some cards are still explicitly fallback-backed (`Affected Users`, `Crash-Free`, `MTTR`).
- `../hyperpush-mono/mesher/client/components/dashboard/events-chart.tsx` — Current live chart surface. Already provider-driven and stable; no routing work needed.
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts` — Canonical live-backend browser harness. It already tracks same-origin requests, failed requests, and visible toasts; S02 should extend or parallel this pattern for issue actions and summary failure cases.
- `../hyperpush-mono/mesher/client/playwright.config.ts` — Existing seeded-local dev/prod harness. It already boots the backend plus app and honors `PLAYWRIGHT_PROJECT` / `npm_config_project` for isolated `dev`/`prod` replays.
- `../hyperpush-mono/mesher/client/src/routes/__root.tsx` — Root document mounts the shared `Toaster`, so S02 can reuse the existing toast path for action success/failure without adding a new notification surface.

### Build Order

1. **Lock the supported S02 action set first.**
   - Lowest-risk set: `resolve`, `unresolve`, `archive` (present as “ignore” in the shell).
   - Treat `assign` as conditional because it needs org-member lookup not present in the current bootstrap.
   - Treat `discard`/`delete` as optional or defer, because `discarded` is unmapped and `delete` is destructive without an existing UI affordance.

2. **Add transport + refresh primitives before touching UI.**
   - Extend `mesher-api.ts` with mutation helpers.
   - Extract a reusable provider-owned overview reload path from the current bootstrap effect (`refreshOverview()` rather than one-shot bootstrap logic).
   - Add selected-issue snapshot invalidation so a successful mutation cannot leave cached stale detail on screen.

3. **Add provider-owned mutation orchestration.**
   - Track pending action state per selected issue/action.
   - On success: invalidate selected snapshot, rerun overview fetch, and if the issue still exists, optionally rehydrate selected detail.
   - On failure: reuse the existing toast path; do not silently fall back.

4. **Wire minimal issue controls into `IssueDetail`.**
   - Extend `ActionButton` with `disabled` / busy affordances.
   - Keep controls in the existing detail action row to avoid layout drift.
   - Do not move action state into route files or route loaders.

5. **Then decide whether any summary surface still needs work after actions are live.**
   - The existing stats/chart surfaces are already partially live from S01.
   - If S02 must reduce “broad mock stats,” do it by improving the existing provider-fed summary chrome, not by making the AI panel or unrelated routes live.
   - Existing backend candidates for richer summary truth are `dashboard/top-issues` and `dashboard/tags`; use them only if a currently rendered Issues-page summary slot truly needs them.

6. **Finish with real-backend browser verification in both runtimes.**
   - Extend the seeded-local Playwright lane rather than inventing a second harness.

### Verification Approach

Use the same seeded-local proof envelope S01 established.

Recommended verification stack:

- Seed/live backend sanity:
  - `bash mesher/scripts/seed-live-issue.sh`
- Dev runtime browser proof:
  - `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live"`
- Prod runtime browser proof:
  - `npm --prefix mesher/client run test:e2e:prod -- --grep "issues live"`

S02-specific browser assertions should prove:
- selected live issue actions hit **same-origin** `/api/v1/issues/:id/...` routes, never `:8080`/`:18080` directly
- successful action updates the visible issue status after provider refresh
- filtered issue lists behave truthfully after action (for example, resolving under an “open” filter removes or reclassifies the issue after refresh)
- failure paths show a visible toast and keep operator context intact
- summary chrome stays wired to provider-backed data and does not regress to silent fallback

If you add separate action-focused Playwright coverage, reuse the same runtime-signal patterns from `issues-live-read.spec.ts`:
- request tracking
- response/failed-request collection
- route-level failure injection
- explicit visible assertions instead of generic page-text matching

## Don't Hand-Roll

| Problem | Existing Solution | Why Use It |
|---------|------------------|------------|
| Same-origin fetch + timeout + typed Mesher errors | `../hyperpush-mono/mesher/client/lib/mesher-api.ts` | S01 already centralized `/api/v1` transport, timeout handling, and `MesherApiError` normalization. Extend it instead of sprinkling `fetch()` calls into components. |
| Cross-panel issue state | `../hyperpush-mono/mesher/client/components/dashboard/dashboard-issues-state.tsx` | The provider already coordinates overview, selected detail, filters, and toasts. Mutations belong here, not in route files or ad hoc component state. |
| Failure visibility | `toast()` via `../hyperpush-mono/mesher/client/hooks/use-toast.ts` and mounted `Toaster` in `src/routes/__root.tsx` | This is the existing truthful failure surface from S01. Reuse it for mutation failures/success feedback instead of adding inline banners or new notification systems. |
| Real-backend browser harness | `../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts` + `playwright.config.ts` | The live-seam harness already proves same-origin transport, seeded issue lookup, and failure injection in dev and prod. Extend it rather than creating a second test architecture. |

## Constraints

- **Reads and writes are split across backend modules.** Live issue reads come from `api/search.mpl` / `api/dashboard.mpl`, while issue mutations live in `ingestion/routes.mpl`.
- **The current shell type system cannot represent all backend mutation states.** `IssueStatus` supports `open | ignored | resolved | regressed | in-progress`; it does not support backend `discarded`.
- **Assignment is not free.** `POST /api/v1/issues/:id/assign` needs a `user_id`, while member lookup is org-scoped (`/api/v1/orgs/:org_id/members`) and the current S01 bootstrap does not expose org identity.
- **Selected issue detail is cached.** `selectedIssueSnapshots` memoizes hydrated detail by issue id; any successful mutation that changes status/assignee must invalidate or rebuild that cache.
- **The client does not consume Mesher websocket broadcasts.** Backend mutation handlers broadcast updates, but the current dashboard shell has no subscription path, so UI truth must come from refetch/invalidation.

## Common Pitfalls

- **Assuming mutation success responses are rich enough for optimistic updates** — they are not. The issue mutation handlers mostly return only `{"status":"ok"}`. Refresh provider state after success unless you are also explicitly patching local state.
- **Forgetting snapshot invalidation** — if `selectedIssueSnapshots[selectedIssueId]` stays cached after a resolve/archive/unresolve, the detail panel can remain stale even if the list refreshes correctly.
- **Treating `discard` like `archive` without deciding semantics** — the adapter maps `archived -> ignored`, but `discarded` is currently unmapped. Either add an explicit UI/status translation or defer that action.
- **Letting assignment drag S03 forward** — assignment is the only action that wants org-member context and a picker. If that context is not already cheaply available from seeded default reality, defer it.
- **Moving this slice into TanStack route loaders** — the shell is already client-only and mutation-driven. Route-loader migration would add complexity without solving the real state-invalidation problem.

## Open Risks

- The roadmap language around “dashboard summaries” is broader than the code. The safest interpretation is the existing Issues-page stats/chart surfaces; widening into AI summaries or other mock-only chrome would overreach S02.
- If real issue action use exposes backend contract bugs (for example, success response but stale follow-up read), the slice may need a narrow backend seam repair in `ingestion/routes.mpl` or `storage/queries.mpl`, but only after reproducing it through the live client path.

## Skills Discovered

| Technology | Skill | Status |
|------------|-------|--------|
| React dashboard state / data flow | `react-best-practices` | installed |
| TanStack Router route ownership | `tanstack-router-best-practices` | installed |
| Playwright live-browser verification | `playwright-best-practices` | installed |