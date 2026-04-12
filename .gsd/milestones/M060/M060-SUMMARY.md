---
id: M060
title: "Mesher Client Live Backend Wiring"
status: complete
completed_at: 2026-04-12T02:03:09.378Z
key_decisions:
  - D512 — use one shared `MESHER_BACKEND_ORIGIN` resolver across Vite dev, built-prod bridge, and Playwright so browser traffic stays same-origin and runtime targets do not drift.
  - D513 — surface selected-issue live-read failures through the existing Radix toast path instead of inventing a new inline error UX.
  - D514 — make `mesher/scripts/seed-live-issue.sh` reuse a running backend when `DATABASE_URL` is absent so deterministic verification still works in split-workspace shells.
  - D515 — keep supported issue mutations inside `DashboardIssuesStateProvider`, expose only resolve/unresolve/archive as live S02 actions, and refetch provider-owned state after success.
  - D516 — expose per-card Issues summary source markers (`live`, `derived live`, `fallback`) instead of overclaiming one global live label.
  - D517 — resolve Team org context by slug through a narrow backend helper/query seam and target `/api/v1/orgs/default/*` routes rather than hardcoding generated UUIDs.
  - D518 — make Settings state subsection-scoped and remove the page-wide fake save affordance so only real backend-backed tabs claim live writes.
  - D519 — keep `mesher/scripts/seed-live-admin-ops.sh` database-first and deterministic rather than reusing whichever Mesher happens to be listening on the default port.
key_files:
  - ../hyperpush-mono/mesher/client/lib/mesher-api.ts
  - ../hyperpush-mono/mesher/client/lib/issues-live-adapter.ts
  - ../hyperpush-mono/mesher/client/lib/admin-ops-live-adapter.ts
  - ../hyperpush-mono/mesher/client/components/dashboard/dashboard-issues-state.tsx
  - ../hyperpush-mono/mesher/client/components/dashboard/alerts-live-state.tsx
  - ../hyperpush-mono/mesher/client/components/dashboard/settings/settings-live-state.tsx
  - ../hyperpush-mono/mesher/client/components/dashboard/settings/settings-page.tsx
  - ../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts
  - ../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts
  - ../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts
  - ../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts
  - ../hyperpush-mono/mesher/client/tests/e2e/live-runtime-helpers.ts
  - ../hyperpush-mono/mesher/client/playwright.config.ts
  - ../hyperpush-mono/mesher/scripts/seed-live-issue.sh
  - ../hyperpush-mono/mesher/scripts/seed-live-admin-ops.sh
  - ../hyperpush-mono/mesher/api/helpers.mpl
  - ../hyperpush-mono/mesher/api/team.mpl
  - ../hyperpush-mono/mesher/storage/queries.mpl
lessons_learned:
  - In this split workspace, milestone closeout must use the code-owning sibling repo as the non-`.gsd` diff truth; auto-mode on local `main` should compare against `origin/main`, not against `main` itself.
  - For live dashboard work, preserve the shell contract and adapt backend payloads through typed overlay seams rather than weakening the UI or silently replacing fallback-backed fields.
  - Provider-owned mutation/refetch orchestration is safer than optimistic local patching when backend mutation responses are thin and the surrounding shell mixes live and fallback state.
  - Assembled browser proof is materially stronger than isolated route tests: one seeded walkthrough plus shared same-origin runtime diagnostics caught shared-runtime and harness drift that slice-local tests alone would miss.
  - When multiple Playwright suites mutate one seeded backend runtime, serial execution (`workers=1`) is part of the product-proof contract, not just a test harness preference.
---

# M060: Mesher Client Live Backend Wiring

**Connected the canonical `mesher/client` dashboard shell to the seeded Mesher backend through same-origin `/api/v1` reads and supported writes, then closed the work with a seeded dev/prod full-shell proof across every current dashboard route.**

## What Happened

M060 turned the canonical TanStack Start dashboard package in the sibling product repo from a mostly shell-first mock surface into a truthful backend-backed client without redesigning the UI contract. S01 established the shared same-origin `/api/v1` transport seam, typed Mesher read helpers, live/mock overlay adapters, deterministic issue seeding, and mounted toast-backed failure visibility for the Issues route. S02 extended that seam into a real maintainer loop by wiring supported issue actions through provider-owned mutation/refetch orchestration and making the Issues summary chrome honest about which cards are live, derived-live, or fallback-backed. S03 applied the same pattern to admin and ops areas already supported by the backend: live Alerts acknowledge/resolve flows, Settings general/storage reads and writes, API key list/create/revoke, alert-rule list/create/toggle/delete, and Team list/add/role/remove through org-slug resolution, while leaving unsupported settings subsections visibly non-live. S04 then assembled the milestone into one deterministic closeout rail: a route-map-driven seeded walkthrough with shared same-origin runtime diagnostics, explicit sparse-detail state markers, serial Playwright execution for the shared seeded runtime, route parity coverage for every current dashboard route, and README-documented dev/prod verification commands.

Verification confirms the work assembled correctly. The code-owning sibling repo `../hyperpush-mono` contains substantial non-`.gsd` changes under `mesher/client`, `mesher/api`, `mesher/storage`, and `mesher/scripts` when diffed against `origin/main`, which is the split-workspace closeout baseline recorded in knowledge for auto-mode on local `main`. `gsd_milestone_status` shows all four slices complete with every task done, slice summary files exist for S01-S04, and the retained proof closes both the route-specific seams and the assembled shell: S01 passed dev/prod `issues live read seam`, S02 passed dev/prod `issues live`, S03 passed seeded dev/prod `admin and ops live`, and S04 finished with 21/21 passing combined dev and prod rails for `issues live|admin and ops live|seeded walkthrough`. The roadmap file itself only contains Vision and Slice Overview sections, so milestone verification anchored on those planned slice outcomes plus the retained requirement validations and S04 UAT contract rather than on a separate success-criteria or horizontal-checklist section.

## Success Criteria Results

- **Seeded default-context boot through same-origin `/api/v1` reads** — Met. S01 passed `bash mesher/scripts/seed-live-issue.sh` and both `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live read seam"` / `npm --prefix mesher/client run test:e2e:prod -- --grep "issues live read seam"`, proving the dashboard boots against the seeded default project/org/API-key reality without adding auth UX and keeps browser traffic same-origin.
- **Existing Issues shell stays materially intact while live data overlays it truthfully** — Met. S01 and S02 preserved the shell while overlaying live list/stats/chart/detail data, explicit per-card live/derived/fallback markers, and supported issue actions. Sparse-detail and failure-path cases stayed truthful in the passing dev/prod suites.
- **Supported dashboard/admin operations use existing backend seams with visible failure feedback** — Met. S02 made resolve/reopen/archive live with provider-owned refetch and toast-backed failure handling. S03 extended the same pattern to Alerts, Settings/storage, API keys, alert rules, and Team with passing seeded dev/prod `admin and ops live` coverage.
- **Full assembled shell walkthrough succeeds across every current dashboard route with minimal seam repair and no redesign drift** — Met. S04's seeded walkthrough and route-parity coverage prove direct entry and shell continuity for Issues, Performance, Solana Programs, Releases, Alerts, Bounties, Treasury, and Settings, including unknown-path fallback. T02 records 21/21 passing dev and 21/21 passing prod combined rails for `issues live|admin and ops live|seeded walkthrough`.

## Definition of Done Results

- **All slices complete** — Met. `gsd_milestone_status` reports S01, S02, S03, and S04 all `complete`, with task counts fully done (2/2, 3/3, 3/3, 2/2).
- **All slice summaries exist** — Met. `.gsd/milestones/M060/slices/S01/S01-SUMMARY.md`, `S02-SUMMARY.md`, `S03-SUMMARY.md`, and `S04-SUMMARY.md` are present.
- **Cross-slice integration points work correctly** — Met. S04 assembled the S01 issue-read seam, S02 issue mutations/summary markers, and S03 alerts/settings/admin flows into one seeded browser session and proved the integrated behavior in dev and prod. The retained UAT also confirms route parity, unknown-path fallback to Issues, same-origin diagnostics, visible destructive toasts, and shell stability on still-mocked sections.
- **Roadmap checklist audit** — The roadmap file contains no separate `Horizontal Checklist` section, so there were no additional horizontal items to audit beyond the slice outcomes and retained UAT contract.

## Requirement Outcomes

- **R153 → validated** — Supported by S04 full-shell dev/prod rails and the assembled seeded walkthrough, which prove the existing backend-backed Issues, dashboard summary, Alerts, Settings/storage, Team, API key, and alert-rule surfaces all use same-origin `/api/v1` reads/writes inside the preserved shell.
- **R154 → validated** — Supported by S03 seeded admin/ops dev/prod rails proving end-to-end same-origin alerts acknowledge/resolve, settings retention/sample-rate writes, API key list/create/revoke, alert-rule list/create/toggle/delete, and Team list/add/role/remove behavior.
- **R155 → validated** — Supported by S01 seeded default-context boot/read proof through same-origin `/api/v1` reads and passing dev/prod `issues live read seam` coverage.
- **R156 → validated** — Supported by S01/S02 proving the Issues shell stays materially intact while live data overlays fallback-only fields truthfully, including sparse-detail behavior.
- **R157 → validated** — Supported by S03 admin/ops suites proving unsupported settings affordances remain visible, explicitly non-live, and shell-stable while live subsections use real backend reads and writes.
- **R158 → validated** — Supported by S01 and later slices via the mounted Radix toaster, visible destructive toasts for backend read/write failures, and passing dev/prod failure-path cases.

## Deviations

The shipped outcome stayed inside the milestone vision, but closeout required a few focused seam repairs discovered during verification rather than broader redesign: decoding JSON-encoded issue-detail fields before validation, making Settings state subsection-scoped instead of keeping a dishonest page-wide save affordance, adding explicit issue-detail tab markers for sparse live detail, narrowing known aborted request filtering in the runtime helper, and running the combined seeded Playwright rail with `workers: 1` to avoid false negatives against one shared seeded Mesher runtime.

## Follow-ups

Future dashboard work should reuse M060's established seams: same-origin `/api/v1` browser transport, typed live/mock adapters, subsection- or provider-owned state with read-after-write refresh, stable `data-*` observability markers, and mounted toast feedback for backend failures. Unsupported shell controls should only be wired live once the backend contract and shell vocabulary can represent them truthfully. If new assembled-route proof is added later, keep route coverage derived from `dashboard-route-map.ts` and maintain the shared runtime helper as the authoritative same-origin diagnostics surface.
