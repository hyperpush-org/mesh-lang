# S03: Admin and ops surfaces live

**Goal:** Make the existing Alerts route and the backend-backed portions of Settings truthful by wiring real Mesher reads and writes for alerts, settings/storage, team members, API keys, and alert rules through the same-origin client seam, while keeping unsupported controls visibly present and honest inside the current shell.
**Demo:** Alerts, settings/storage, team, and API-key areas use real backend reads and writes wherever the backend already has a route, while the broader shell stays visually intact.

## Must-Haves

- The standalone Alerts route is backend-backed through same-origin `/api/v1` reads and fired-alert lifecycle actions, with unsupported silence/channel affordances still visually present but not mislabeled as live. (`R154`, `R157`)
- The existing Settings shell exposes live retention/sample-rate + storage metrics, live API key list/create/revoke, and live alert-rule list/create/toggle/delete without keeping the current fake page-wide Save behavior for unsupported controls. (`R154`, `R157`)
- The Team tab becomes truthful through a narrow org-slug seam and supports real member list/add/role/remove behavior without hardcoding the seeded org UUID or faking an email invite flow the backend does not support. (`R154`)
- Backend-backed admin/ops read and write failures surface through the mounted Radix toaster plus explicit page state markers instead of silent fallback. (`R158`, advances `R154`)

## Threat Surface

- **Abuse**: replayed alert/key/member/rule mutations, parameter tampering against ids/slugs, and fake-save drift could mislead operators unless writes stay on same-origin `/api/v1`, supported action sets are explicit, and read-after-write refresh is authoritative.
- **Data exposure**: API keys, member emails/display names, and backend error text flow through this slice; new key values must be revealed only at creation time, and toasts/tests must not echo full secrets or raw backend bodies.
- **Input trust**: retention/sample-rate form values, raw `user_id` add-member input, alert-rule payloads, and every route payload are untrusted inputs that must be parsed and mapped before reaching shell state.

## Requirement Impact

- **Requirements touched**: `R154`, `R157`, `R158`
- **Re-verify**: same-origin admin/ops reads and writes, shell continuity for still-mocked areas, one-time API-key reveal behavior, Team slug resolution, and destructive-toast failure feedback in both dev and prod.
- **Decisions revisited**: `D508`, `D510`, `D511`, `D513`

## Proof Level

- This slice proves: integration
- Real runtime required: yes
- Human/UAT required: no

## Verification

- `mesher/client/tests/e2e/admin-ops-live.spec.ts` contains real assertions for alerts, settings/storage, API keys, alert rules, Team flows, same-origin request tracking, and destructive-toast failure paths.
- `bash mesher/scripts/seed-live-admin-ops.sh`
- `npm --prefix mesher/client run test:e2e:dev -- --grep "admin and ops live"`
- `npm --prefix mesher/client run test:e2e:prod -- --grep "admin and ops live"`

## Observability / Diagnostics

- Runtime signals: `alerts-shell` / `settings-shell` `data-*` state markers, visible subsection source badges, same-origin request tracking in Playwright, and the mounted Radix toast region.
- Inspection surfaces: `bash mesher/scripts/seed-live-admin-ops.sh`, `mesher/client/tests/e2e/admin-ops-live.spec.ts`, browser network/console logs, and the visible shell state banners/markers.
- Failure visibility: last failed read/write per surface, selected alert/member/action phase, and destructive toast text instead of silent fallback or fake save success.
- Redaction constraints: never print full API key secrets, raw backend error bodies, or unrelated seeded credentials in logs, assertions, or UI copy.

## Integration Closure

- Upstream surfaces consumed: `mesher/client/lib/mesher-api.ts`, `mesher/client/src/routes/_dashboard.alerts.tsx`, `mesher/client/src/routes/_dashboard.settings.tsx`, `mesher/client/src/routes/__root.tsx`, `mesher/api/alerts.mpl`, `mesher/api/team.mpl`, `mesher/api/settings.mpl`, and the seeded default-org migration/runtime.
- New wiring introduced in this slice: alerts/settings state owners, admin/ops payload adapters, same-origin Team slug resolution through the existing routes, and a deterministic admin/ops seed/proof rail.
- What remains before the milestone is truly usable end-to-end: S04 still needs the full assembled seeded-shell walkthrough and any narrow seam repairs discovered while composing every live surface together.

## Tasks

- [x] **T01: Wire the Alerts route to live Mesher reads/actions and seed the admin/ops proof file** `est:2h30m`
  - Why: The standalone Alerts route is the cleanest user-visible admin/ops seam and gives the slice its first truthful live surface plus the first real browser-proof file.
  - Files: `mesher/client/lib/mesher-api.ts`, `mesher/client/lib/admin-ops-live-adapter.ts`, `mesher/client/components/dashboard/alerts-live-state.tsx`, `mesher/client/components/dashboard/alerts-page.tsx`, `mesher/client/components/dashboard/alert-detail.tsx`, `mesher/client/components/dashboard/alert-stats.tsx`, `mesher/client/lib/mock-data.ts`, `mesher/client/tests/e2e/admin-ops-live.spec.ts`
  - Do: Extend the shared same-origin Mesher client for admin/ops payloads, add an alerts-owned live state seam, refactor the alerts shell to use live reads and supported fired-alert lifecycle actions, and seed `admin-ops-live.spec.ts` with same-origin happy/failure coverage for Alerts.
  - Verify: `npm --prefix mesher/client run test:e2e:dev -- --grep "admin and ops live alerts"`
  - Done when: the Alerts route reads and acts on real backend data, unsupported alert controls remain visibly non-live, and the new browser proof exercises an Alerts happy path plus a destructive-toast failure path.
- [ ] **T02: Make Settings general, API keys, and alert rules truthful without breaking the shell** `est:2h30m`
  - Why: S03 is not honest until the existing Settings shell stops pretending the whole page is writable and backs the real subsections with actual reads and writes.
  - Files: `mesher/client/components/dashboard/settings/settings-live-state.tsx`, `mesher/client/components/dashboard/settings/settings-page.tsx`, `mesher/client/lib/admin-ops-live-adapter.ts`, `mesher/client/tests/e2e/admin-ops-live.spec.ts`
  - Do: Add a settings-owned state seam, wire live General/storage, API key, and alert-rule subsections through the shared adapters, remove fake global save behavior for unsupported controls, and expand the browser proof for same-origin settings mutations and failure toasts.
  - Verify: `npm --prefix mesher/client run test:e2e:dev -- --grep "admin and ops live settings"`
  - Done when: Settings General/storage, API Keys, and Alert Rules are truthful in the current shell, unsupported tabs stay visibly stable but non-live, and the proof file covers the backed write paths.
- [ ] **T03: Resolve org-scoped team context and close the seeded admin/ops proof in dev and prod** `est:2h`
  - Why: Team work is still blocked on org-context lookup, and the slice is not closed until the combined admin/ops seam is reproducible in seeded dev and prod runs.
  - Files: `mesher/storage/queries.mpl`, `mesher/api/helpers.mpl`, `mesher/api/team.mpl`, `mesher/client/lib/mesher-api.ts`, `mesher/client/components/dashboard/settings/settings-live-state.tsx`, `mesher/client/components/dashboard/settings/settings-page.tsx`, `mesher/scripts/seed-live-admin-ops.sh`, `mesher/client/tests/e2e/admin-ops-live.spec.ts`
  - Do: Add an org-slug resolver, update Team handlers to accept the seeded default slug, wire live Team list/add/role/remove behavior with a truthful raw-`user_id` affordance, create a deterministic admin/ops seed helper, and finish the combined dev/prod browser proof.
  - Verify: `bash mesher/scripts/seed-live-admin-ops.sh && npm --prefix mesher/client run test:e2e:dev -- --grep "admin and ops live" && npm --prefix mesher/client run test:e2e:prod -- --grep "admin and ops live"`
  - Done when: `/api/v1/orgs/default/members` works through the same-origin seam, the Team tab is truthful without hardcoded UUIDs or fake invites, and the full admin/ops proof passes in both runtimes.

## Files Likely Touched

- `mesher/client/lib/mesher-api.ts`
- `mesher/client/lib/admin-ops-live-adapter.ts`
- `mesher/client/components/dashboard/alerts-live-state.tsx`
- `mesher/client/components/dashboard/alerts-page.tsx`
- `mesher/client/components/dashboard/alert-detail.tsx`
- `mesher/client/components/dashboard/alert-stats.tsx`
- `mesher/client/lib/mock-data.ts`
- `mesher/client/components/dashboard/settings/settings-live-state.tsx`
- `mesher/client/components/dashboard/settings/settings-page.tsx`
- `mesher/storage/queries.mpl`
- `mesher/api/helpers.mpl`
- `mesher/api/team.mpl`
- `mesher/scripts/seed-live-admin-ops.sh`
- `mesher/client/tests/e2e/admin-ops-live.spec.ts`
