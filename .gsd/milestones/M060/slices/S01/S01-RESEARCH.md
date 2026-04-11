# M060/S01 — Research

**Date:** 2026-04-11

## Summary

S01 should be implemented as a **mixed live/mock adapter seam**, not a UI rewrite. The backend already has the live read routes needed for this slice — seeded default project slug resolution, issue listing, event search/listing, event detail, issue timeline, and dashboard summary reads — but the current issues shell expects a much richer product-shaped model than the Mesher API returns directly. The correct move for this slice is therefore: keep the issues UI exactly as it is, wire the real backend data into the existing shell where the contract exists, and preserve unsupported fields/screens as fake data inside the same view model.

The seeded real-context story is already present in the backend. `mesher/migrations/20260226000000_seed_default_org.mpl`, `mesher/README.md`, and `mesher/scripts/smoke.sh` all establish the canonical local context: org slug `default`, project slug `default`, and seeded dev API key `mshr_devdefaultapikey000000000000000000000000000`. There is no HTTP route today to enumerate orgs or projects, and the team-members route requires an `org_id` UUID, not a slug. For S01, the truthful context seam is therefore a seeded single-project boot path behind the existing selector shell, while the richer selector UI remains visually intact and can stay partially cosmetic.

The other hard blocker is transport, not UI. `mesher/client/vite.config.ts` has no dev proxy, `mesher/client/server.mjs` has no production proxy, and the Mesher backend shows no CORS handling. Browser-side calls from `localhost:3000` or `:3001` directly to `:8080` would therefore be cross-origin and likely fail. The first slice task should be same-origin `/api/v1/*` proxying plus a seeded default-context adapter, then live issues/detail/events/dashboard reads behind a typed normalization + fallback layer.

## Recommendation

Keep the existing shell and keep unsupported UI/data fake.

Concretely: preserve `DashboardIssuesStateProvider` as the owner of search/filter/selection state so the current in-memory shell behavior survives navigation, back/forward, and detail-panel toggles. Add a small typed Mesher client layer plus an **issues live-overlay adapter** underneath it. That adapter should merge backend truth into the existing `Issue`/stats/chart shell model, filling whatever the backend does not supply with current mock data or stable placeholders, so the planner/executor never simplify or remove the UI.

For reads, use the existing backend only:
- list/issues: `GET /api/v1/projects/default/issues`
- summary cards: `GET /api/v1/projects/default/dashboard/health` + `GET /api/v1/projects/default/dashboard/levels`
- event chart: `GET /api/v1/projects/default/dashboard/volume`
- selected issue detail: `GET /api/v1/issues/:issue_id/events?limit=1` → `GET /api/v1/events/:event_id`
- optional selected-issue history: `GET /api/v1/issues/:issue_id/timeline`

Fetch independent reads in parallel (`react-best-practices` `async-parallel`) and surface failures through the already-present Radix toast stack, not a new notification system. Do **not** hand-roll a new toast layer, and do **not** use Sonner first — `components/ui/sonner.tsx` depends on `next-themes`, but no `ThemeProvider` is mounted anywhere. `components/ui/toaster.tsx` + `hooks/use-toast.ts` are already usable once mounted from `src/routes/__root.tsx`.

## Implementation Landscape

### Key Files

- `mesher/client/components/dashboard/dashboard-issues-state.tsx` — current global issues-shell state owner; preserves search/filter/selection across route changes and is the cleanest place to replace direct `MOCK_ISSUES` reads with a live+mock merged resource while keeping the shell contract intact.
- `mesher/client/components/dashboard/issues-page.tsx` — page composition for stats/chart/filter/list/detail; should stay mostly structural if the provider exposes live list/detail/loading/error state through the existing props.
- `mesher/client/components/dashboard/stats-bar.tsx` — currently hard-coded to `MOCK_STATS`; for S01, planner should treat this as a merge target, not a simplification target: live backend-backed metrics can overwrite the matching cards while unsupported cards remain mock.
- `mesher/client/components/dashboard/events-chart.tsx` — currently expects per-bucket `critical/high/medium/low` series from `MOCK_EVENT_SERIES`; backend only provides total bucket counts, so the likely seam is to merge/replace what can be truthful while preserving the existing chart UI shape rather than deleting or simplifying it.
- `mesher/client/components/dashboard/issue-list.tsx` — assumes every row has subtitle/file/project/environment/tags/users; the live adapter should populate what it can from backend truth and fall back to existing mock-shaped values for the rest.
- `mesher/client/components/dashboard/issue-detail.tsx` — selected-detail surface can be enriched from `GET /api/v1/events/:event_id`; where the live backend lacks a field, preserve the existing shell field with fake/mock fallback instead of removing UI sections.
- `mesher/client/src/routes/__root.tsx` — current root document has no mounted toast UI; mount `components/ui/toaster.tsx` here so R158 has an actual visible surface.
- `mesher/client/vite.config.ts` — no dev proxy today; add same-origin proxying for `/api/v1` to the Mesher backend.
- `mesher/client/server.mjs` — production bridge serves built assets only; add matching `/api/v1` proxy behavior here so built-app verification does not diverge from dev.
- `mesher/client/lib/mock-data.ts` — stays important in S01; this is the fallback source for unsupported fields and must not be treated as a delete/refactor target for this slice.
- `mesher/client/tests/e2e/dashboard-route-parity.spec.ts` — current spec is intentionally mock-coupled (hard-coded `HPX-1039`, zero failed requests). Keep it as shell-parity proof or split it; do not silently replace it with live-backend assertions.
- `mesher/main.mpl` — authoritative live scope for S01; confirms which read routes already exist and that issue/event/detail/dashboard reads are in-scope.
- `mesher/api/search.mpl` — live issue listing, event search, tag filter, and issue-event pagination handlers; note the response shapes are slim and need adapter merge/fallback.
- `mesher/api/dashboard.mpl` — health, level breakdown, volume, top-issues, and issue timeline reads; key source for truthful stats/chart overlays.
- `mesher/api/detail.mpl` — full event detail payload including `stacktrace`, `breadcrumbs`, `tags`, `extra`, and `user_context`; this is the best source for selected-issue enrichment.
- `mesher/api/helpers.mpl` — resolves project slug → UUID; this is why the seeded `default` slug can be used directly from the client.
- `mesher/migrations/20260226000000_seed_default_org.mpl` — authoritative seeded context for `default` org/project and dev API key.
- `mesher/README.md` — canonical local runbook and readiness check (`/api/v1/projects/default/settings`).
- `mesher/scripts/smoke.sh` — executable proof of the seeded project readiness contract; useful for live verification sequencing.
- `mesher/client/README.md` — currently contains a stale M059 rule: “Do not add backend calls…”; S01 implementation should update this documentation once the live seam lands.
- `mesher/client/lib/mesher-api.ts` *(new, likely)* — typed fetch/proxy client for `/api/v1/*` reads with consistent error handling.
- `mesher/client/lib/issues-live-adapter.ts` *(new, likely)* — translation layer that overlays Mesher issue/event/dashboard payloads onto the current UI model, preserving fake fields when the backend does not provide them.

### Build Order

1. **Transport + truthful context first.** Add same-origin `/api/v1` proxying in `vite.config.ts` and `server.mjs`, and define the seeded default dashboard context (`projectSlug: 'default'`). This retires the biggest hidden blocker: no proxy + no CORS.
2. **Create the live-overlay adapter seam before touching UI.** Build typed fetch helpers plus backend→UI normalization for issue list rows, selected issue detail, health cards, and volume data. This adapter should explicitly preserve current fake fields when the backend lacks them.
3. **Swap `dashboard-issues-state.tsx` from pure mock reads to live orchestration with mock fallback.** Preserve search/filter/selection state ownership here so navigation/back-forward behavior stays stable. If bootstrap reads are added at the route boundary, keep them narrow and keyed only by the seeded project slug; do not widen search-param semantics.
4. **Then wire current components to merged data, not slimmer data.** `stats-bar.tsx`, `events-chart.tsx`, `issue-list.tsx`, and `issue-detail.tsx` should keep their current UI shape. Backend-backed values can override existing mock values where truth exists; unsupported fields stay fake.
5. **Mount failure visibility and add live proof last.** Mount the existing toaster in `__root.tsx`, wire fetch failures through it, and add a separate live-backend verification path rather than mutating the mock-parity spec into something brittle.

### Verification Approach

Backend readiness and seed truth:

```bash
DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash mesher/scripts/migrate.sh up
DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash mesher/scripts/smoke.sh
```

Deterministic live issue creation for browser proof (use the seeded API key from the migration/runbook; prefer a custom fingerprint so the issue is easy to identify and coalesces predictably):

```bash
curl -sSf \
  -X POST http://127.0.0.1:8080/api/v1/events \
  -H 'Content-Type: application/json' \
  -H 'x-sentry-auth: mshr_devdefaultapikey000000000000000000000000000' \
  -d '{
    "message":"S01 live issue smoke",
    "level":"error",
    "fingerprint":"s01-live-issue-smoke",
    "tags":{"environment":"production","route":"issues-smoke"},
    "stacktrace":[{"filename":"src/live-smoke.ts","function_name":"runSmoke","lineno":12,"colno":4,"context_line":"throw new Error()","in_app":true}],
    "breadcrumbs":[{"timestamp":"2026-04-11T00:00:00Z","category":"ui.click","message":"smoke breadcrumb","level":"info","data":"{}"}]
  }'
```

Client proof:

```bash
npm --prefix mesher/client run dev
npm --prefix mesher/client run build
PORT=3001 npm --prefix mesher/client run start
```

Then verify in browser/Playwright that:
- the dashboard boots against the seeded default context
- the issues route performs real `/api/v1/*` reads without failed requests
- the list/detail flow reflects live backend issue/event data where it exists
- unsupported UI remains visually present instead of disappearing
- failures surface through the mounted toast UI
- console and network stay clean except for intentionally induced backend-failure checks

For automated proof, follow the existing runtime-signal pattern in `tests/e2e/dashboard-route-parity.spec.ts` and add a **separate** live-backend spec. Per `playwright-best-practices`, seed deterministic test data up front and assert explicit runtime cleanliness rather than depending on brittle mock IDs.

## Don't Hand-Roll

| Problem | Existing Solution | Why Use It |
|---------|------------------|------------|
| Toast-style failure visibility | `mesher/client/components/ui/toaster.tsx` + `mesher/client/hooks/use-toast.ts` | R158 already asks for minimal truthful toast feedback, and this stack is already in the package; only mounting/wiring is missing. |

## Constraints

- The truthful local context is the seeded default org/project/API-key path, not a real session or multi-project switcher: `default` slug resolution is supported, but there is no HTTP route to enumerate orgs/projects.
- The backend exposes no CORS surface; live browser reads must be same-origin via proxy/bridge rather than direct `:3000` → `:8080` cross-origin fetches.
- The user explicitly requires **no UI simplification** for missing backend fields; unsupported surfaces or fields must remain fake/present inside the existing shell.
- `mesher/client` route parity and shell behavior currently depend on provider-local state, not URL search params. Preserve that contract unless the user explicitly reopens URL semantics.
- `mesher/client/lib/mock-data.ts` still backs other routes; S01 should carve out the issues/dashboard read seam only.

## Common Pitfalls

- **Direct browser fetches to the Mesher backend** — `vite.config.ts` and `server.mjs` currently provide no `/api/v1` proxy, and the backend shows no CORS handling. Add the proxy first or every live client fetch will fail for the wrong reason.
- **Treating missing backend fields as a reason to simplify UI** — for this slice that is explicitly out of bounds. Use a merge adapter and keep the current UI/data shape intact.
- **Mapping backend status/level names implicitly** — the backend uses `unresolved/resolved/archived` and `fatal/error/warning/info/debug`, while the UI uses `open/in-progress/regressed/resolved/ignored` and `critical/high/medium/low`. The adapter needs explicit mapping rules.
- **Breaking the existing shell-parity proof accidentally** — `dashboard-route-parity.spec.ts` is intentionally tied to mock ids like `HPX-1039` and zero network calls. Keep that proof isolated from new live-backend verification.
- **Choosing Sonner by default** — `components/ui/sonner.tsx` expects `next-themes`, but no `ThemeProvider` is mounted. The Radix toast stack is the lower-risk path for S01.

## Open Risks

- The current issues shell has more product metadata than the backend can provide truthfully. The planner should assume the adapter will need explicit fallback policy for each field rather than expecting one clean 1:1 API model.
- Later milestone slices that need org/team truth may hit a harder seam: `GET /api/v1/orgs/:org_id/members` requires a UUID org id, and S01 found no HTTP route that discovers org ids from the seeded default context.

## Skills Discovered

| Technology | Skill | Status |
|------------|-------|--------|
| TanStack Router / TanStack Start | `tanstack-router-best-practices` | available |
| React | `react-best-practices` | available |
| Playwright | `playwright-best-practices` | available |
