# S03 — Research

**Date:** 2026-04-11

## Summary

S03 is targeted integration work, not a new architecture wave. The existing S01 pattern is the right base: keep browser traffic on same-origin `/api/v1`, extend the typed fetch/error layer in `mesher/client/lib/mesher-api.ts`, normalize lean Mesher payloads into the richer shell contract with explicit adapters, and surface failures through the already-mounted Radix toast stack. Today the admin/ops client surfaces are still fully mock-driven: the standalone Alerts route reads `MOCK_ALERTS`, and the Settings route is a single large mock page with inline Team, API Keys, Alerts, and General tabs.

The real risk is contract asymmetry, not transport. Backend routes already exist for project settings/storage, alert rules, fired alerts, team members, and API keys, but their payloads are materially leaner than the current UI. `GET /projects/:project_id/settings` only returns `retention_days` and `sample_rate`; `GET /projects/:project_id/storage` returns only `event_count` and `estimated_bytes`; API keys expose `label`, `key_value`, timestamps, and revocation state but no scopes/last-used; fired alerts expose `active|acknowledged|resolved`, not the client’s current `firing|silenced|resolved`; and team membership is org-scoped with raw `org_id` + `user_id` mutations. So S03 should make only the actually-backed fields/actions live, keep the rest of the shell visually present but non-live, and solve org-context discovery before promising a truthful Team write path.

## Recommendation

Extend the S01 seam instead of inventing a second one. Put all new admin/ops HTTP reads and writes into `mesher/client/lib/mesher-api.ts`, add one small provider/hook layer for admin/ops state modeled on `DashboardIssuesStateProvider`, and use lightweight adapters to feed the existing leaf components. Per the loaded React guidance, keep independent bootstrap reads parallel (`async-parallel`) and keep orchestration above the leaf widgets instead of scattering fetches across `alerts-page.tsx`, `alert-detail.tsx`, and the inline settings tabs.

Treat the Settings route as a mixed live/mock page on purpose. Wire the backed subsections only: retention/sample-rate + storage in General, members in Team, key list/create/revoke in API Keys, and rule list/create/toggle/delete plus fired-alert lifecycle where the current UI can honestly represent it. Do **not** let unsupported controls keep looking writable after this slice: the current global fake Save flow would be dishonest if untouched. For verification, follow the Playwright guidance already used in S01: explicit locators/assertions, runtime-signal tracking, and no sleep-driven tests. Add a targeted live admin/ops spec rather than bloating the existing route-parity suite.

## Implementation Landscape

### Key Files

- `../hyperpush-mono/mesher/client/lib/mesher-api.ts` (lines 1-388) — existing typed same-origin fetch layer. It currently only covers issues/dashboard/event-detail/timeline reads plus shared `MesherApiError` handling. S03 should extend this file with admin/ops reads and POST helpers instead of creating a parallel client.
- `../hyperpush-mono/mesher/client/lib/issues-live-adapter.ts` (lines 1-611) — proven live/mock overlay pattern from S01. This is the reference shape for mapping Mesher payloads into richer shell models while preserving unsupported fallback fields.
- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-issues-state.tsx` (lines 1-474) — provider-owned bootstrap, parallel reads, selected-item hydration, and toast-backed failure handling. Reuse this state ownership pattern for S03 instead of fetching directly in leaf components.
- `../hyperpush-mono/mesher/client/components/dashboard/alerts-page.tsx` (lines 1-309) — standalone Alerts route; currently filters/sorts/selects only `MOCK_ALERTS`.
- `../hyperpush-mono/mesher/client/components/dashboard/alert-list.tsx` (lines 1-245) — presentational list/detail selector for alert rows. Safe to keep dumb if a new provider feeds adapted live rows.
- `../hyperpush-mono/mesher/client/components/dashboard/alert-detail.tsx` (lines 1-298) — detail view and footer actions. Current actions are `Resolve`, `Silence`, `Unsnooze`, and `Copy Link`; only `Resolve` maps cleanly to existing backend routes today.
- `../hyperpush-mono/mesher/client/components/dashboard/alert-stats.tsx` (lines 1-89) — stats bar derived from `MOCK_ALERT_STATS`. S03 can derive at least firing/resolved counts from live alerts instead of keeping this static.
- `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-page.tsx` (lines 41-123, 266-789) — monolithic Settings shell. Natural live seams are:
  - `41-123` mock nav/config constants
  - `266-372` page header + global fake save state
  - `374-431` General tab
  - `580-614` Team tab
  - `664-736` API Keys tab
  - `738-789` Alerts tab
  All other tabs below remain mock-only unless the planner explicitly decides otherwise.
- `../hyperpush-mono/mesher/client/lib/mock-data.ts` (lines 557-758) — current `Alert`, `AlertHistory`, `MOCK_ALERTS`, and `MOCK_ALERT_STATS` contract. Useful as a fallback/template reference, but many fields are not backed by Mesher.
- `../hyperpush-mono/mesher/client/src/routes/_dashboard.alerts.tsx` (lines 1-9) and `../hyperpush-mono/mesher/client/src/routes/_dashboard.settings.tsx` (lines 1-9) — route files are thin; route-level refactors are unnecessary unless a provider wrapper is introduced.
- `../hyperpush-mono/mesher/client/src/routes/__root.tsx` (import/mount at lines 18 and 91) — the shared toaster is already mounted. Reuse it; do not add a second notification surface.
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts` (lines 1-376; key tests at 99-304) — existing runtime-signal tracking and same-origin/failure assertions. Use this as the template for new S03 browser proof.
- `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts` (lines 23-89, 405-469) — route-shell parity spec. Keep it as shell coverage; add a separate live admin/ops spec for backend truth.
- `../hyperpush-mono/mesher/main.mpl` (lines 119-152) — authoritative route registration for this slice. Existing surfaced routes are already present for team members, API keys, alert rules, fired alerts, settings, and storage.
- `../hyperpush-mono/mesher/api/settings.mpl` (lines 1-97) — project settings/storage handlers. Contract is intentionally small: settings = `{ retention_days, sample_rate }`, storage = `{ event_count, estimated_bytes }`, update returns `{ status: "ok" }`.
- `../hyperpush-mono/mesher/api/team.mpl` (lines 1-307) — team + API-key handlers. Team reads/writes are org-scoped; API keys are project-scoped. Create key returns only `{ key_value }`.
- `../hyperpush-mono/mesher/api/alerts.mpl` (lines 1-181) — alert-rule CRUD and fired-alert lifecycle handlers. Fired alerts list returns `status` values from the backend (`active`, `acknowledged`, `resolved`).
- `../hyperpush-mono/mesher/api/helpers.mpl` (lines 1-65) — only project IDs get slug resolution via `resolve_project_id`. There is **no** equivalent org slug resolver or routed project-context lookup.
- `../hyperpush-mono/mesher/storage/queries.mpl` (key seams: 866-891, 898-937, 1027-1152) — backend DB truth for members, API keys, alert rules, alerts, settings, and storage. The raw `list_alerts` query is an intentional keep-site; do not “clean it up” during client wiring.
- `../hyperpush-mono/mesher/migrations/20260226000000_seed_default_org.mpl` (lines 1-58) — seeded default org/project/API-key migration. The default org UUID is looked up after insert and is not stable enough to hardcode in the client.
- `compiler/meshc/tests/e2e_m033_s01.rs` (lines ~1041-1179) — retained backend mutation proof for API-key revoke, alert acknowledge/resolve, and settings update.
- `compiler/meshc/tests/e2e_m033_s03.rs` (lines ~1749-1897 and ~2539-2609) — retained backend read proof for org members and fired alerts. These are the best truth references for client parsers when route payload details are ambiguous.

### Build Order

1. **Solve the context seam first** — determine how the client will obtain a truthful `org_id` for `/api/v1/orgs/:org_id/members`. Today the client only knows `project_id = default`, `Api.Helpers.resolve_project_id` only resolves project slugs, and the seed org UUID is runtime-generated. Team work is blocked until this is explicit.
2. **Extend the shared API layer next** — add typed reads/writes for settings, storage, members, API keys, alert rules, and fired alerts in `mesher-api.ts`, including a shared POST helper and parser coverage for the real route payloads.
3. **Wire the standalone Alerts route** — this is the cleanest live surface after transport because it has a dedicated route/page and existing list/detail components. Derive live stats from the fetched alerts list and map backend lifecycle actions before touching the monolithic Settings page.
4. **Wire Settings tab-by-tab, not as one rewrite** — General/Storage, Team, API Keys, and Alerts Rules are natural sub-seams inside `settings-page.tsx`. Keep unsupported tabs and unsupported controls visually intact but non-live.
5. **Add targeted browser proof last** — once the context and mutations exist, add a dedicated S03 Playwright spec in dev/prod that seeds/asserts live alerts/settings/team/api-key behavior without weakening the existing route-parity shell tests.

### Verification Approach

Use the same seeded-local workflow S01 established, plus a new targeted browser spec for admin/ops truth.

From `mesh-lang/` in this split workspace, first ensure Mesher boots with the sibling compiler binary if needed:

```bash
MESHER_MESHC_BIN=../hyperpush-mono/target/debug/meshc \
MESHER_MESHC_SOURCE=sibling \
  bash ../hyperpush-mono/mesher/scripts/migrate.sh up

MESHER_MESHC_BIN=../hyperpush-mono/target/debug/meshc \
MESHER_MESHC_SOURCE=sibling \
  bash ../hyperpush-mono/mesher/scripts/smoke.sh
```

Then verify the client package:

```bash
npm --prefix ../hyperpush-mono/mesher/client run build
npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev -- --grep "admin and ops live"
npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod -- --grep "admin and ops live"
```

The new S03 Playwright spec should follow the existing `issues-live-read.spec.ts` pattern:
- attach runtime-signal tracking and fail on unexpected console/network errors
- assert browser traffic stays on same-origin `/api/v1`
- seed/setup live backend state through request helpers or a narrow seed script, not through mock data
- assert visible outcomes with explicit locators per the Playwright guidance, not broad page text or sleeps
- prove at least one failure path with visible toast feedback for a backed mutation/read

API-level smoke worth covering before browser assertions:
- `GET/POST /api/v1/projects/default/settings`
- `GET /api/v1/projects/default/storage`
- `GET/POST /api/v1/projects/default/api-keys` and `POST /api/v1/api-keys/:key_id/revoke`
- `GET/POST /api/v1/projects/default/alert-rules`, `POST /api/v1/alert-rules/:rule_id/toggle`, `POST /api/v1/alert-rules/:rule_id/delete`
- `GET /api/v1/projects/default/alerts`, `POST /api/v1/alerts/:id/acknowledge`, `POST /api/v1/alerts/:id/resolve`
- `GET/POST /api/v1/orgs/:org_id/members` and membership role/remove once the org-id seam is solved

## Constraints

- `R154` and `R157` are the slice-owned requirements. The UI must keep mock-only areas present and stable while making existing backend-backed actions truthful.
- Team membership is org-scoped, not project-scoped. `../hyperpush-mono/mesher/api/helpers.mpl` resolves only project slugs, so the client has no current way to derive the seeded default org UUID from an existing routed project response.
- The seed migration in `../hyperpush-mono/mesher/migrations/20260226000000_seed_default_org.mpl` creates the default org/project idempotently but does **not** make the org UUID stable. Hardcoding it in the client is invalid.
- The Settings backend only supports `retention_days`, `sample_rate`, and storage metrics. Current controls for project name, description, default environment, public dashboard, anonymous issue submission, integrations, billing, security, notifications, and profile have no corresponding route.
- The Alerts backend exposes alert-rule CRUD and fired-alert acknowledge/resolve, but it does **not** expose silence/unsnooze or notification-channel configuration.
- API key creation returns only `{ key_value }`; list rows do not include scopes or last-used timestamps.
- `storage/queries.mpl::list_alerts(...)` is an intentional raw-SQL keep-site because a builder-backed version crashed the live route. Do not spend S03 on backend query cleanup there.

## Common Pitfalls

- **Global fake Save drift on Settings** — `settings-page.tsx` currently marks the whole page dirty and fakes save with a `setTimeout`. If S03 wires only some tabs/fields, unsupported controls must stop pretending to save or the page becomes dishonest.
- **Team invite is not actually email-backed** — `POST /api/v1/orgs/:org_id/members` requires `user_id`, and there is no routed user lookup/search surface. The current `Invite` button cannot become truthful without either a narrow discovery seam or a UI change that exposes raw IDs.
- **Alert status terminology does not line up** — backend statuses are `active|acknowledged|resolved`, while the client uses `firing|silenced|resolved`. Pick an explicit mapping and button label strategy before wiring actions.
- **Alert rule cards are richer than backend rule rows** — backend rules are `name + condition_json + action_json + enabled + cooldown + timestamps`. Severity/channel badges in the current settings shell are not guaranteed to be derivable from those JSON blobs.
- **Do not fork the transport/error path** — S01 already proved same-origin routing, typed parsing, and toast-backed failures. Reuse `mesher-api.ts`, the provider pattern, and the mounted toaster instead of building a second fetch stack.

## Open Risks

- The org-context gap may require a narrow backend seam repair before the Team tab can be wired honestly.
- The current General tab does not expose `sample_rate`, and the current shell has no storage section. A small additive UI change is likely required if S03 is supposed to use the existing settings/storage routes truthfully.
- Team add-member and alert-rule create flows may force a product decision if the existing backend contracts are technically present but too low-level for the current UI without small affordance changes.

## Skills Discovered

| Technology | Skill | Status |
|------------|-------|--------|
| React client wiring | `react-best-practices` | available |
| Playwright verification | `playwright-best-practices` | available |
