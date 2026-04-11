---
estimated_steps: 4
estimated_files: 4
skills_used:
  - react-best-practices
  - test
---

# T04: Make top-level navigation route-aware and preserve shell parity behaviors

**Slice:** S02 — Route-backed dashboard parity
**Milestone:** M059

## Description

Close the subtle parity traps once the route files exist. The route tree alone is not enough: sidebar active state must derive from the current pathname, navigation clicks must update the URL instead of only local state, the AI panel must close on route changes, sidebar collapse must persist across route transitions, and the shared header must stay hidden on `/settings`.

This task should make the shared shell route-aware without widening state into the router. Keep transient UI state local to the shell/ref logic; only the top-level section selection comes from TanStack Router.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `../hyperpush-mono/mesher/frontend-exp/components/dashboard/sidebar.tsx` route-aware top nav | Use real route targets and active-path derivation; do not keep a second local `activeNav` source of truth. | N/A | Normalize unexpected path matches to no-highlight or Issues instead of crashing. |
| `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-shell.tsx` route-change side effects | Close the AI panel and preserve collapse state on section changes; do not accidentally reset shell-owned Issues state on every navigation. | Bound route-change effects to pathname changes only. | Ignore malformed or unknown path segments rather than clearing unrelated shell state. |
| `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts` navigation assertions | Fail on URL/visible-state divergence instead of asserting only one side. | Wait on explicit URL and landmark changes, not arbitrary sleeps. | Treat missing active-nav or settings-header expectations as failures. |

## Load Profile

- **Shared resources**: one router state source, shell-owned UI state, and nav-triggered panel/Issues state transitions.
- **Per-operation cost**: one client navigation plus small shell side effects (panel close, active-nav recompute).
- **10x breakpoint**: split-brain state between router path and local shell state will break parity long before load becomes interesting.

## Negative Tests

- **Malformed inputs**: rapid repeated nav clicks, unknown path matches, and AI-panel toggles during route changes.
- **Error paths**: URL changes without visible active-nav updates, active-nav changes without URL updates, or shared header appearing on `/settings`.
- **Boundary conditions**: collapsed sidebar survives route switches, AI panel closes when moving between non-settings routes, and returning to `/` keeps Issues filter state.

## Steps

1. Make `sidebar.tsx` consume real route targets/path matching instead of callback-only string slugs, while keeping the existing label order, badges, and footer settings button.
2. Update the shared shell to derive the active section from the current pathname and run only the parity-preserving route-change side effects (AI close, title mapping, settings-header exception).
3. Keep transient UI state local to the shell/context/ref layer; do not move filters, detail panels, or dropdown state into router search params.
4. Extend the Playwright spec to click through the sidebar and footer settings control and assert URL updates, active state, AI close-on-nav, sidebar persistence, settings header behavior, and Issues leave-and-return persistence.

## Must-Haves

- [ ] Top-level navigation is URL-backed and derives the visible active section from router state.
- [ ] Sidebar order, badges, footer settings affordance, and visible chrome remain unchanged.
- [ ] AI panel close-on-nav, sidebar collapse persistence, settings header exception, and Issues leave-and-return behavior match current parity expectations.
- [ ] The task does not widen the slice into router search params or backend work.

## Verification

- `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build`
- `npm --prefix ../hyperpush-mono/mesher/frontend-exp exec playwright test tests/e2e/dashboard-route-parity.spec.ts --project=dev --grep "navigation parity"`

## Observability Impact

- Signals added/changed: route-aware nav assertions now expose URL/state drift explicitly.
- How a future agent inspects this: rerun the navigation-parity Playwright grep and inspect `sidebar.tsx` plus `dashboard-shell.tsx`.
- Failure state exposed: stale active-nav highlighting, AI panel not closing, settings header drift, and reset Issues state.

## Inputs

- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/sidebar.tsx` — current nav UI that still needs router ownership.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-shell.tsx` — shared shell that owns cross-route UI state.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-route-map.ts` — canonical route/title mapping from T01.
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.tsx` — shared layout route from T03.
- `../hyperpush-mono/mesher/frontend-exp/src/routes/_dashboard.index.tsx` — Issues route whose leave-and-return state must stay truthful.
- `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts` — parity suite to extend with route-backed navigation behavior.

## Expected Output

- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/sidebar.tsx` — top-level nav now drives real route targets and active-path matching.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-shell.tsx` — shell now derives active section from router state and preserves parity side effects.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-route-map.ts` — final canonical route metadata used by nav/title logic.
- `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts` — navigation-parity assertions for URL updates and shell behavior.
