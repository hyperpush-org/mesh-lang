# S02 Research — Route-backed dashboard parity

## Requirements Targeted

- **R143 (primary)** — move the active dashboard from the temporary single-route adapter to real TanStack file routes **without changing the visible product surface**.
- **R145 (primary)** — preserve current top-level navigation, sidebar behavior, right-panel behavior, filter behavior, and major mock-data interactions while routes become URL-backed.
- **R146 (primary)** — keep the work entirely on the existing mock-data/client-state path; no Mesher backend or new server-function scope belongs in this slice.
- **R147 (supporting)** — new routes must still survive the existing `dev` / `build` / `start` contract, including direct-entry deep links after build.

## Skills Discovered

- **Existing installed skill used:** `react-best-practices`
  - Relevant rule: **`rerender-derived-state-no-effect`** — derive route-selected UI from router state instead of mirroring pathname into local `activeNav` state.
  - Relevant rule: **`rerender-use-ref-transient-values`** — keep transient copilot/sidebar bookkeeping in refs/local shell state instead of pushing it into router/global state.
- **Existing installed skill used:** `vite`
  - Relevant guidance: stay on the existing **ESM + `vite.config.ts`** path; do not widen S02 into custom bundler/server plumbing.
- **New skill installed for later units:** `deckardger/tanstack-agent-skills@tanstack-start-best-practices`
  - Installed globally via `npx skills add ... -g -y`.
  - It does **not** appear in this session’s `Skill` list yet, so this unit could not execute it directly; it should be available to later units after prompt refresh.

## Summary

The current TanStack Start app is still a **one-route parity adapter**. `src/routes/index.tsx` mounts the old Next-era shell from `app/page.tsx`, and `routeTree.gen.ts` confirms there are only two runtime routes right now: `__root__` and `/`.

The code already has strong route seams for S02:

- the **shared chrome** is centralized in `app/page.tsx` (`Sidebar`, header, AI panel, sidebar collapse logic)
- the **leaf screens** for `performance`, `solana-programs`, `releases`, `alerts`, `bounties`, `treasury`, and `settings` already exist as self-contained components under `components/dashboard/**`
- only the **issues screen** still lives inline inside `app/page.tsx`, so it is the one page that must be extracted before route decomposition is clean

The important parity trap is state placement. Today, some state persists across top-level nav changes because it lives in the shared shell, not in the leaf pages:

- `sidebarCollapsed` persists across section switches
- the header’s environment/time-range/project dropdown state persists across non-settings section switches because the header stays mounted
- the issues page’s `search`, `statusFilter`, `severityFilter`, and selected issue state live in `app/page.tsx`, so they persist when you leave Issues and return
- AI panel state is shared across pages and is explicitly closed when nav changes

If S02 turns every section into an independently wrapped route page instead of a shared layout route, those persistence behaviors will drift.

## Implementation Landscape

### Current runtime files and what they do

- `../hyperpush-mono/mesher/frontend-exp/src/routes/__root.tsx`
  - Root document, CSS/font links, icons, analytics gating.
  - Stable S01 plumbing; S02 should not need to widen this.
- `../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx`
  - Current temporary adapter route.
  - Imports `../../app/page` and sets `ssr: false`.
  - This is the current route seam to replace.
- `../hyperpush-mono/mesher/frontend-exp/src/routeTree.gen.ts`
  - Generator-owned; currently only knows `/`.
  - **Do not hand-edit.** Route files drive this.
- `../hyperpush-mono/mesher/frontend-exp/app/page.tsx`
  - The monolithic temporary shell.
  - Owns top-level nav state (`activeNav`), sidebar collapse, issues filters/search, issues detail panel, shared AI panel, and header title switching.
  - This is the main file to decompose.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/sidebar.tsx`
  - Current nav model uses hardcoded nav items and callback-based `onNavigate(href)` with string slugs like `issues`, `performance`, `solana-programs`, etc.
  - It is UI-ready for route-backed nav, but not router-aware yet.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/header.tsx`
  - Shared non-settings header plus the issues-only `FilterBar`.
  - Internal env/time/project dropdown state is local and currently persists because the header remains mounted while switching among non-settings views.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/performance-page.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/solana-programs-page.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/releases-page.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/alerts-page.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/bounties-page.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/treasury-page.tsx`
  - These are already route-leaf-ready. Each owns its own local filters/detail panel.
  - They can be mounted under real file routes with little or no UI rewriting.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings/settings-page.tsx`
  - Self-contained `/settings` leaf with its own internal left nav and header.
  - Internal settings sub-tabs are local state and can stay local for S02.
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/ai-panel.tsx`
  - Shared right panel used across pages.
  - Needs to remain layout-owned, not duplicated into every route.
- `../hyperpush-mono/mesher/frontend-exp/server.mjs`
  - Already serves static assets and forwards every non-static request to `dist/server/server.js`.
  - This is important: **S02 should not need start-command or server changes** to support direct deep links. Once the new routes exist, production direct-entry should already work through this bridge.
- `../hyperpush-mono/mesher/frontend-exp/app/layout.tsx`
  - Dead/stale Next artifact. Not on the active TanStack runtime path.
  - Do not build new route logic around it.

### Best route shape for this codebase

The lowest-risk TanStack structure is a **pathless dashboard layout route** plus flat child route files.

Why this is the best fit:

1. TanStack Router file naming supports:
   - `.` for nested routes
   - `_` prefix for **pathless layout routes**
2. This preserves one mounted dashboard shell while letting each section have a real URL.
3. It matches the existing persistence behavior much better than “each route wraps its own shell”.

Recommended route layout:

- `src/routes/_dashboard.tsx` — shared dashboard layout route (pathless)
- `src/routes/_dashboard.index.tsx` — Issues at `/`
- `src/routes/_dashboard.performance.tsx` — `/performance`
- `src/routes/_dashboard.solana-programs.tsx` — `/solana-programs`
- `src/routes/_dashboard.releases.tsx` — `/releases`
- `src/routes/_dashboard.alerts.tsx` — `/alerts`
- `src/routes/_dashboard.bounties.tsx` — `/bounties`
- `src/routes/_dashboard.treasury.tsx` — `/treasury`
- `src/routes/_dashboard.settings.tsx` — `/settings`

This keeps **Issues on `/`** instead of inventing `/issues`, which is the safest parity move because the current entrypoint is still `/` and the shell defaults to Issues.

### Natural seams to extract first

#### 1. Shared shell/layout seam

Extract the shell concerns out of `app/page.tsx` into a layout-owned module:

- sidebar rendering
- sidebar collapse state
- shared header title switching
- shared AI panel open/close state
- copilot auto-collapse sidebar ref bookkeeping
- route-derived active section

This shared shell should derive the active section from router location, not from duplicated local `activeNav` state. That aligns with `react-best-practices` **`rerender-derived-state-no-effect`** and removes the current split-brain risk where URL and visible selection could diverge.

#### 2. Issues route seam

The Issues view is the only page not already extracted. Pull the inline Issues branch out of `app/page.tsx` into its own component/module.

Important parity note: issues filters/search/detail selection currently live above route level. If the new `/` route owns them locally, leaving `/` and coming back will reset behavior that currently persists. If parity needs to stay exact, keep this state in the shared dashboard layout (or a small layout-owned context/store) and let the `/` route consume it.

#### 3. Sidebar routing seam

`components/dashboard/sidebar.tsx` is the only top-level nav component. It should stop calling callback-only string slugs and become route-aware.

Safest shape:

- keep the hardcoded nav item order, icons, badges, and labels exactly as-is
- change nav items from callback slugs to route targets
- route selection should be derived from pathname
- keep the footer settings button routing to `/settings`

Do **not** introduce new visual affordances, nesting, or badge logic here.

### What can stay local

These should remain local and **not** be widened into route search params or server loaders in S02:

- performance filters/detail panel
- releases filters/detail panel
- alerts filters/detail panel
- bounties filters/detail panel
- treasury filters/detail panel
- settings internal tab state
- mock-data sources in `lib/mock-data.ts` / `lib/solana-mock-data.ts`

S02 is top-level route parity, not a URL-serialization project.

## Recommendation

1. **Replace the temporary `app/page.tsx` adapter with a pathless dashboard layout route.**
   - This is the only approach that preserves the mounted shell behavior closely enough.
2. **Keep `/` as Issues.**
   - Add real routes for the other section slugs, but do not invent `/issues` unless the user explicitly asks for it.
3. **Move the shell out of `app/page.tsx` and stop importing runtime route content from `app/`.**
   - Route composition should live under `src/routes/` + extracted shared components, not in a stale Next page file.
4. **Keep the route layer client-first.**
   - Current page components are mock-data-driven and client-heavy; introducing loaders/server functions here would widen scope and risk hydration drift.
   - S01 already validated `ssr: false` on the adapter route. For S02, explicitly verify the new route structure preserves the same client-first safety rather than assuming layout inheritance.
5. **Do not touch server/package/runtime plumbing unless route deep-link verification proves a real gap.**
   - `server.mjs` already forwards non-static requests; the route tree is the missing piece, not the server bridge.

## Natural Task Boundaries

### Task 1 — Extract the shared dashboard shell

Touch likely files:

- `../hyperpush-mono/mesher/frontend-exp/app/page.tsx` (source of extracted logic or eventual deletion)
- new shared shell module(s) under `../hyperpush-mono/mesher/frontend-exp/components/dashboard/` or `src/`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/sidebar.tsx`
- maybe `../hyperpush-mono/mesher/frontend-exp/components/dashboard/header.tsx`

Goal:

- build a single router-backed shell that still behaves like the current mounted dashboard frame

### Task 2 — Create real TanStack file routes for each top-level section

Touch likely files:

- new `src/routes/_dashboard*.tsx` files
- `../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx` (remove/replace)
- generated `src/routeTree.gen.ts` will update via toolchain

Goal:

- one real route per section, same visible shell

### Task 3 — Extract Issues into its own route component and preserve its special state behavior

Touch likely files:

- `../hyperpush-mono/mesher/frontend-exp/app/page.tsx`
- new issues route/content component
- maybe shared shell state/context module

Goal:

- eliminate the last inline branch in the temporary adapter without losing Issues-specific parity

### Task 4 — Browser + production deep-link verification

No product rewrite here; this is proof work.

Goal:

- prove both dev and built production app can load each new route directly and still show the same shell/interactions

## Risks / Fragile Seams

- **Shell remount drift:** if each route renders its own sidebar/header wrapper, shared header dropdown state and sidebar-local UI will reset on every section change.
- **Issues-state drift:** the Issues route is special today because its filters/search/detail state live in the shell. Moving that state into the `/` route will change leave-and-return behavior.
- **AI panel drift:** current nav changes explicitly close the AI panel. If S02 only swaps buttons for `Link` without preserving close-on-route-change behavior, UI behavior changes.
- **Settings header drift:** settings intentionally hides the shared header and renders its own page header. A generic shell wrapper that always shows the shared header will visibly change `/settings`.
- **Route generator misuse:** `routeTree.gen.ts` is generated. Hand-editing it or choosing invalid route filenames will create churn or broken routes.
- **Accidental URL widening:** encoding every filter/panel into search params would technically work, but it changes visible URL behavior and widens the slice beyond top-level route parity.
- **Hydration/client drift:** the current shell still depends on browser-only behaviors (`matchMedia`, `window.innerWidth`, client refs). Keep the route stack client-safe first; don’t widen into SSR experiments during parity work.

## Verification

### Build / command contract

- `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build`
- `npm --prefix ../hyperpush-mono/mesher/frontend-exp run dev`
- `PORT=3001 npm --prefix ../hyperpush-mono/mesher/frontend-exp run start`

### Browser proof that should exist before closing S02

In **dev** and again in **production**:

1. Direct-load each route URL, not just click from `/`:
   - `/`
   - `/performance`
   - `/solana-programs`
   - `/releases`
   - `/alerts`
   - `/bounties`
   - `/treasury`
   - `/settings`
2. Assert each route shows unique expected content and the same shell chrome:
   - `/` → `Issues` header + issues filter/search UI
   - `/performance` → `Performance` header + transaction/perf filter UI
   - `/solana-programs` → `Solana Programs` + parsed logs/instruction breakdown UI
   - `/releases` → `Releases` + release search/filter UI
   - `/alerts` → `Alerts` + alerts search/filter UI
   - `/bounties` → `Bounties` + claims search/filter UI
   - `/treasury` → `Treasury` + transactions search/filter UI
   - `/settings` → settings-specific header/body and **no shared dashboard header row**
3. Assert navigation clicks update the URL and the visible active section.
4. Assert **zero console errors** and **zero failed requests** on the mock-data path.
5. Regress the subtle behaviors that can drift:
   - open AI panel on a non-settings route, then navigate to another section → panel should close
   - collapse sidebar, switch routes → collapse state should persist
   - change Issues filters, leave `/`, come back → if parity is the bar, those filters should still reflect the current shell-owned behavior
6. Production-only deep-link smoke:
   - load at least one non-root route first (for example `/releases` or `/settings`) under `npm run start` to prove the current `server.mjs` bridge handles built route entries truthfully

## Sources

- TanStack Start / Router docs via Context7:
  - file-based route definition and root-route examples
  - route types and navigation examples
- Official TanStack Router file naming conventions:
  - `https://tanstack.com/router/latest/docs/routing/file-naming-conventions.md`
- Product code inspected in:
  - `../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx`
  - `../hyperpush-mono/mesher/frontend-exp/src/routes/__root.tsx`
  - `../hyperpush-mono/mesher/frontend-exp/src/router.tsx`
  - `../hyperpush-mono/mesher/frontend-exp/src/routeTree.gen.ts`
  - `../hyperpush-mono/mesher/frontend-exp/app/page.tsx`
  - `../hyperpush-mono/mesher/frontend-exp/app/layout.tsx`
  - `../hyperpush-mono/mesher/frontend-exp/components/dashboard/sidebar.tsx`
  - `../hyperpush-mono/mesher/frontend-exp/components/dashboard/header.tsx`
  - `../hyperpush-mono/mesher/frontend-exp/components/dashboard/ai-panel.tsx`
  - `../hyperpush-mono/mesher/frontend-exp/components/dashboard/performance-page.tsx`
  - `../hyperpush-mono/mesher/frontend-exp/components/dashboard/solana-programs-page.tsx`
  - `../hyperpush-mono/mesher/frontend-exp/components/dashboard/releases-page.tsx`
  - `../hyperpush-mono/mesher/frontend-exp/components/dashboard/alerts-page.tsx`
  - `../hyperpush-mono/mesher/frontend-exp/components/dashboard/bounties-page.tsx`
  - `../hyperpush-mono/mesher/frontend-exp/components/dashboard/treasury-page.tsx`
  - `../hyperpush-mono/mesher/frontend-exp/components/dashboard/settings/settings-page.tsx`
  - `../hyperpush-mono/mesher/frontend-exp/server.mjs`
