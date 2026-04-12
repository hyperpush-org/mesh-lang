# S01 Research — Evidence-backed route inventory

## Summary
- S01 directly advances **R167** (canonical maintainer-facing mock/live inventory) and **R170** (evidence-backed classifications with rerunnable proof). It supports **R168** (fine-grained mixed-route truth) and **R171** (handoff future backend planning can consume), while staying bounded by **R173–R175** (no backend implementation, no shell redesign, no public-docs wave).
- The codebase already has the raw truth needed for S01. What is missing is a **single maintainer document beside `mesher/client`** that turns the existing route map, page-state markers, and Playwright rails into one explicit inventory.
- The safest authoritative root is **`../hyperpush-mono/mesher/client/components/dashboard/dashboard-route-map.ts`**, not route-file globs, not the sidebar, and not README prose. `live-runtime-helpers.ts` already derives `DASHBOARD_ROUTES` from that map, so both the document and any future drift-check should anchor there.
- Current top-level route truth is:

| Route key | Pathname | Classification | Why |
|---|---:|---|---|
| `issues` | `/` | **mixed** | Same-origin live overview/detail/actions exist, but detail chrome still keeps shell-only helpers (`AI Analysis`, `Link Issue`, bounty chrome) and fallback overlay behavior. |
| `performance` | `/performance` | **mock-only** | Page imports only `MOCK_*` performance data. |
| `solana-programs` | `/solana-programs` | **mock-only** | Page imports only `lib/solana-mock-data.ts`. |
| `releases` | `/releases` | **mock-only** | Page imports only `MOCK_RELEASES`. |
| `alerts` | `/alerts` | **mixed** | Same-origin live list/detail/actions exist, but `Silence/Unsnooze` stays shell-only chrome and runtime can fall back without claiming full live parity. |
| `bounties` | `/bounties` | **mock-only** | Page imports only `MOCK_BOUNTY_*`. |
| `treasury` | `/treasury` | **mock-only** | Page imports only `MOCK_TREASURY*`. |
| `settings` | `/settings` | **mixed** | `General` is mixed, `Team` / `API Keys` / `Alerts` are live, and the remaining tabs are explicitly mock-only. |

## Skills Discovered
- No new skills needed installation. Existing directly relevant skills were already present:
  - `react-best-practices`
  - `tanstack-start-best-practices`
  - `playwright-best-practices`
- Guidance that materially affects implementation here:
  - **Playwright best practices — stable locators/assertions/waiting:** reuse existing `data-testid` and `data-*` assertions as evidence anchors instead of prose-only claims or screenshot proof.
  - **React best practices — `async-parallel`:** preserve the code’s existing parallel bootstrap facts (`fetchDefaultProjectDashboardBootstrap`, Settings general bootstrap) as evidence; do not document them as slower/serial flows.
  - **TanStack Start best practices — file separation:** keep the inventory as a maintainer document beside the app, not as runtime strings embedded across route components.

## Implementation Landscape

### 1. Canonical top-level route source already exists
- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-route-map.ts:1-66` defines the eight route keys and their canonical pathnames.
- Important nuance: **Issues lives at `/`, not `/issues`** (`dashboard-route-map.ts:24`). Any inventory doc or drift check must use the route map instead of inferring from filenames or labels.
- `../hyperpush-mono/mesher/client/tests/e2e/live-runtime-helpers.ts` imports that route map and exports `DASHBOARD_ROUTES`; both direct-entry proof rails already consume it.
- Secondary drift surface: `../hyperpush-mono/mesher/client/components/dashboard/sidebar.tsx:40-48` duplicates the same route keys in `NAV_ITEMS`. That duplication is worth calling out, but it should **not** become the inventory source of truth.

### 2. Top-level route classification is currently implicit, not documented
#### Mixed routes
- **Issues**
  - `../hyperpush-mono/mesher/client/components/dashboard/issues-page.tsx:9,160,164` mounts provider-owned live state and emits `data-overview-source` / `data-selected-issue-source` markers.
  - `../hyperpush-mono/mesher/client/components/dashboard/dashboard-issues-state.tsx` owns live bootstrap, selected-issue hydration, and supported actions (`resolve`, `unresolve`, `archive`).
  - `../hyperpush-mono/mesher/client/components/dashboard/issue-detail.tsx:385-412` explicitly leaves `AI Analysis`, `Link Issue`, and bounty chrome as shell-only while `Resolve`, `Reopen`, and `Ignore` go through same-origin live actions.
- **Alerts**
  - `../hyperpush-mono/mesher/client/components/dashboard/alerts-page.tsx:6,163,166` mounts provider-owned live state and emits `data-overview-source` / `data-selected-alert-source` markers.
  - `../hyperpush-mono/mesher/client/components/dashboard/alerts-live-state.tsx` owns live overview plus supported `acknowledge` / `resolve` actions.
  - `../hyperpush-mono/mesher/client/components/dashboard/alert-detail.tsx:169,245` keeps `Silence` / `Unsnooze` as shell-only chrome while `Acknowledge` / `Resolve` go to `/api/v1/alerts/...`.
- **Settings**
  - `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-page.tsx:153-160` already contains a human-readable support map: `general => mixed live`, `team/api-keys/alerts => live`, everything else => `mock-only`.
  - `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-page.tsx:479-491` exposes shell-level state via `data-current-tab`, `data-shell-source`, and last-mutation markers.
  - `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-live-state.tsx:439,489,539,589,1250-1263` shows the actual section-source truth: `general` is `mixed`; `team`, `apiKeys`, and `alertRules` are `live`; shell aggregates to `mixed` unless every section is fully live or fully fallback.

#### Mock-only routes
These five routes are straightforward and do **not** need over-engineered decomposition for S01:
- `../hyperpush-mono/mesher/client/components/dashboard/performance-page.tsx:5` imports `MOCK_TRANSACTIONS` / `MOCK_PERF_STATS`.
- `../hyperpush-mono/mesher/client/components/dashboard/solana-programs-page.tsx:3-14` imports `lib/solana-mock-data.ts`.
- `../hyperpush-mono/mesher/client/components/dashboard/releases-page.tsx:5` imports `MOCK_RELEASES`.
- `../hyperpush-mono/mesher/client/components/dashboard/bounties-page.tsx:5` imports `MOCK_BOUNTY_CLAIMS` / `MOCK_BOUNTY_STATS`.
- `../hyperpush-mono/mesher/client/components/dashboard/treasury-page.tsx:3-13` imports `MOCK_TREASURY*`.

### 3. Existing Playwright rails already prove most of the inventory
There are **two** complementary top-level proof surfaces plus the route-specific suites:

#### A. Route-map / shell parity rail
- `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts`
  - proves direct entry across every route (`:direct entry ...`, `:unknown path`)
  - proves sidebar/history/AI shell behavior
  - uses `DASHBOARD_ROUTES` from `live-runtime-helpers.ts`
- `../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts:417-437`
  - duplicates route-map direct-entry coverage using the canonical route map
  - also proves unknown-path fallback to Issues

#### B. Truthful live/mock walkthrough rail
- `../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts:442-758`
  - **Issues** live read/write proof: `461+`
  - **mock-only top-level route stays reachable** for `performance`, `solana-programs`, `releases`: `542+`
  - **Alerts** live read/write proof: `552+`
  - **mock-only top-level route stays reachable** for `bounties`, `treasury`: `608+`
  - **Settings** mixed proof with explicit mock-only subsection banners: `618+`
- This is the closest thing to the future S01 inventory proof rail already in the repo.

#### C. Route-specific live rails
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts:167+`
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts:177+`
- `../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts:180+` (alerts) and `473+` (settings)
- These suites are better evidence for subsection/control-level mixed truth than for the top-level inventory table itself.

### 4. Stable evidence anchors already exist in the UI
These are the best anchors for the maintainer doc because they are already asserted in Playwright:
- `dashboard-shell[data-route-key]` — route identity (`dashboard-shell.tsx:145+`, `live-runtime-helpers.ts:186`)
- Issues: `issues-shell[data-overview-source]`, `issue-detail-panel[data-source]`
- Alerts: `alerts-shell[data-overview-source]`, `alert-detail-panel[data-source]`
- Settings: `settings-shell[data-current-tab]`, `settings-shell-support-badge`, section `data-source`, and explicit mock-only banners such as:
  - `settings-general-mock-only-banner`
  - `settings-alert-channels-mock-only-banner`
  - `settings-bounty-mock-only-banner`
  - `settings-integrations-mock-only-banner`
  - `settings-billing-mock-only-banner`
  - `settings-security-mock-only-banner`
  - `settings-notifications-mock-only-banner`
  - `settings-profile-mock-only-banner`

These markers mean S01 likely **does not need new runtime markers** just to document current truth.

### 5. Backend seam mapping is already explicit for the live surfaces
Useful for inventory rows even before S03 does the full backend gap map:
- `../hyperpush-mono/mesher/client/lib/mesher-api.ts:719-906` exposes the current same-origin client boundary.
- Existing backend handlers exist for every currently-live client seam:
  - Issues overview/events/timeline: `search.mpl:216`, `search.mpl:363`, `dashboard.mpl:205`, `detail.mpl:136`
  - Alerts / acknowledge / resolve / alert-rules: `alerts.mpl:70-170`
  - Settings / storage: `settings.mpl:53-94`
  - Team / role / remove / API keys: `team.mpl:237-348`
- This confirms S01 can truthfully say the mixed/live routes are backed by real backend seams today, while the mock-only routes are not.

## What is missing right now
- There is **no single maintainer-facing inventory document** in `../hyperpush-mono/mesher/client/` that lists all eight routes with classifications and evidence.
- `../hyperpush-mono/mesher/client/README.md` is still mostly workflow/setup + Issues-first narrative. It references the full-shell proof rail (`README.md:75-87`) but does **not** serve as a clear route inventory.
- There is also **no dedicated route-inventory verifier script** yet. Product-side precedent exists in `../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh`, but nothing currently checks doc ↔ route-map parity for this milestone.

## Recommendation
1. **Create a dedicated maintainer document in the client root** rather than overloading `README.md`.
   - Recommended file: `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`
   - Reason: the README is already a runbook; the new doc should be a reference surface.
2. **Drive the document from `dashboard-route-map.ts` semantics**, with one row per `DashboardRouteKey`.
3. **Use a top-level table for S01**, with columns like:
   - route key
   - pathname
   - classification (`live` / `mixed` / `mock-only`)
   - code evidence
   - proof evidence
   - current backend seam summary
   - follow-up note (`see mixed-route breakdown` for Issues / Alerts / Settings)
4. **Only summarize mixed boundaries in S01**; leave full subsection/control decomposition for S02.
5. **Update `README.md` to link to the new inventory doc** and to clarify that README is setup/proof guidance, not the canonical live/mock map.
6. If the planner wants an early drift-check in S01, keep it **small and structural**:
   - compare the doc’s route keys/pathnames against `dashboard-route-map.ts`
   - optionally confirm every inventory row cites at least one proof rail
   - do **not** invent a second runtime classification registry yet

## Natural seams for planning
### Seam A — Inventory authoring (lowest risk, highest value)
Files likely touched:
- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` (new)
- `../hyperpush-mono/mesher/client/README.md`

### Seam B — Optional lightweight drift check
Files likely touched only if the planner chooses to start S04-style guardrails early:
- `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` (recommended product-side location if a new script is added)
- possibly `../hyperpush-mono/mesher/client/package.json` to expose a script alias

### Seam C — Only if missing evidence links are needed
Likely avoid for S01 unless absolutely necessary:
- `../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts`
- `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts`
- `../hyperpush-mono/mesher/client/tests/e2e/live-runtime-helpers.ts`

Current anchors look sufficient, so S01 should probably stay documentation-first.

## Risks / Gotchas
- **Do not treat `fallback` as `mock-only`.**
  - Issues, Alerts, and Settings all use `fallback` as a runtime failure/degradation source state (`issues-live-adapter.ts`, `admin-ops-live-adapter.ts`, `settings-live-state.tsx`), not as the canonical route classification.
- **Settings uses the label `mixed live`, while the milestone language says `mixed`.**
  - Normalize this carefully in the doc/verifier; otherwise future checks will fail on wording instead of truth.
- **Issues is rooted at `/`.**
  - Any inventory row or verifier that assumes `/issues` is wrong.
- **The sidebar duplicates route keys.**
  - Good as a drift target, bad as a primary source.
- **Direct-entry proof is already duplicated across `dashboard-route-parity.spec.ts` and `seeded-walkthrough.spec.ts`.**
  - Don’t add a third copy of route enumeration unless a later verifier truly needs it.
- **No top-level route is fully live today.**
  - Issues, Alerts, and Settings are all mixed; the other five are mock-only. If a doc row says a top-level route is fully live, it is overstating current truth.

## Verification
### Structural/doc verification for S01
Minimum check for the slice’s actual deliverable:
- confirm the document has exactly the eight route keys from `dashboard-route-map.ts`
- confirm each row uses the canonical pathname from `dashboard-route-map.ts`
- confirm each row cites at least one code anchor and one proof rail

### Existing runtime proof rails worth reusing
From the **product repo root** (`../hyperpush-mono`):

```bash
bash mesher/scripts/seed-live-issue.sh
bash mesher/scripts/seed-live-admin-ops.sh
npm --prefix mesher/client run test:e2e:dev -- --grep "issues live|admin and ops live|seeded walkthrough"
npm --prefix mesher/client run test:e2e:prod -- --grep "issues live|admin and ops live|seeded walkthrough"
```

If the planner wants the direct-entry/nav rail included too, extend the grep to include `dashboard route parity`:

```bash
npm --prefix mesher/client run test:e2e:dev -- --grep "dashboard route parity|issues live|admin and ops live|seeded walkthrough"
npm --prefix mesher/client run test:e2e:prod -- --grep "dashboard route parity|issues live|admin and ops live|seeded walkthrough"
```

## Recommended execution order for the planner
1. Build the route table from `dashboard-route-map.ts`.
2. Fill top-level classifications using page-component code evidence.
3. Add proof-rail references from `seeded-walkthrough.spec.ts`, `dashboard-route-parity.spec.ts`, and the route-specific suites.
4. Add short “mixed boundaries” notes for Issues / Alerts / Settings with pointers forward to S02-worthy files.
5. Link the new doc from `client/README.md`.
6. Only then decide whether S01 also needs a lightweight doc-vs-route-map drift check or whether that should stay in S04.
