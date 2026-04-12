# S04: Full backend-backed shell assembly

**Goal:** Prove the full Mesher dashboard shell works end to end in one seeded local environment by adding a single composed walkthrough across every current route and repairing only the exact blockers that proof exposes.
**Demo:** In one seeded local environment, the full backend-backed shell walkthrough succeeds across every currently existing Mesher dashboard route, with only minimal backend seam repairs and no redesign drift.

## Must-Haves

- One seeded Playwright walkthrough traverses every dashboard route in a single browser session, sourcing the canonical route inventory from `mesher/client/components/dashboard/dashboard-route-map.ts` instead of duplicating route lists. (`R160`)
- Live-backed routes (`/`, `/alerts`, `/settings`) stay truthful inside the assembled shell: they read and perform the already-supported same-origin actions against `/api/v1`, then show refreshed `data-state` / `data-source` truth after navigation and writes. (`R153`, `R154`, `R160`)
- Mock-only routes and controls remain visibly present, reachable, and shell-stable without being relabeled as live or removed to make the walkthrough pass. (`R156`, `R157`)
- Any break uncovered by the composed walkthrough is fixed only at the exact client, selector, or backend seam required to restore the existing live-backed flow; no new backend surface or shell redesign is introduced. (`R159`)
- `mesher/client/README.md` documents the seeded full-shell verification rail so maintainers can reproduce the assembled proof in dev and prod.

## Threat Surface

- **Abuse**: replayed issue/alert/settings actions, route tampering, or helper drift could make the walkthrough mutate the wrong seeded record or mask a broken path unless selectors stay deterministic and only the known hidden-Issues abort noise is filtered.
- **Data exposure**: seeded member emails, alert text, and one-time API-key creation surfaces appear during the assembled proof; tests, logs, and docs must keep secrets redacted and must not print raw key material or backend bodies.
- **Input trust**: route pathnames, seeded ids, query state, and every `/api/v1` payload remain untrusted; the walkthrough must fail on malformed responses or unexpected direct-backend traffic instead of guessing fallback state.

## Requirement Impact

- **Requirements touched**: `R153`, `R154`, `R156`, `R157`, `R159`, `R160`
- **Re-verify**: same-origin issue/alert/settings reads and writes, route-to-route shell continuity, mock-only route stability, filtered hidden-Issues abort handling, and seeded dev/prod parity.
- **Decisions revisited**: `D510`, `D515`, `D518`, `D519`

## Proof Level

- This slice proves: final-assembly
- Real runtime required: yes
- Human/UAT required: no

## Verification

- `mesher/client/tests/e2e/seeded-walkthrough.spec.ts` contains explicit route-to-route assertions for live Issues, Alerts, and Settings behavior plus mock-only route stability and filtered same-origin runtime diagnostics.
- `bash mesher/scripts/seed-live-issue.sh`
- `bash mesher/scripts/seed-live-admin-ops.sh`
- `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live|admin and ops live|seeded walkthrough"`
- `npm --prefix mesher/client run test:e2e:prod -- --grep "issues live|admin and ops live|seeded walkthrough"`

## Observability / Diagnostics

- Runtime signals: `dashboard-shell[data-route-key]`, `issues-shell`, `alerts-shell`, `settings-shell`, subsection `data-state` / `data-source` markers, Playwright-tracked same-origin API paths, and the mounted Radix toast region.
- Inspection surfaces: `mesher/client/tests/e2e/seeded-walkthrough.spec.ts`, the shared E2E helper under `mesher/client/tests/e2e/`, `bash mesher/scripts/seed-live-issue.sh`, `bash mesher/scripts/seed-live-admin-ops.sh`, and browser console/network output during dev/prod runs.
- Failure visibility: the first failing route/action, unexpected 4xx/5xx or direct-backend browser call, the last visible shell state/source marker, and any destructive toast shown after write or refresh failure.
- Redaction constraints: never print full API keys, raw backend bodies, or unrelated seeded credentials in tests, docs, or debug output.

## Integration Closure

- Upstream surfaces consumed: `mesher/client/components/dashboard/dashboard-route-map.ts`, `mesher/client/components/dashboard/dashboard-shell.tsx`, `mesher/client/components/dashboard/sidebar.tsx`, `mesher/client/tests/e2e/dashboard-route-parity.spec.ts`, `mesher/client/tests/e2e/issues-live-read.spec.ts`, `mesher/client/tests/e2e/issues-live-actions.spec.ts`, `mesher/client/tests/e2e/admin-ops-live.spec.ts`, `mesher/scripts/seed-live-issue.sh`, `mesher/scripts/seed-live-admin-ops.sh`, and `mesher/client/playwright.config.ts`.
- New wiring introduced in this slice: a single seeded full-shell walkthrough spec, a shared E2E runtime-diagnostics helper, and only the minimal selector or backend-seam patching required to keep the assembled route flow truthful.
- What remains before the milestone is truly usable end-to-end: nothing in M060 once the seeded dev/prod verification rail passes; any future scoping of the globally mounted Issues provider is optional follow-up outside this milestone.

## Tasks

- [x] **T01: Add the seeded full-shell walkthrough proof rail** `est:2h`
  - Why: R160 is still open because proof is split across route-parity, Issues, and admin/ops suites instead of one assembled shell walkthrough.
  - Files: `mesher/client/components/dashboard/dashboard-route-map.ts`, `mesher/client/tests/e2e/dashboard-route-parity.spec.ts`, `mesher/client/tests/e2e/issues-live-read.spec.ts`, `mesher/client/tests/e2e/issues-live-actions.spec.ts`, `mesher/client/tests/e2e/admin-ops-live.spec.ts`, `mesher/client/tests/e2e/live-runtime-helpers.ts`, `mesher/client/tests/e2e/seeded-walkthrough.spec.ts`
  - Do: Extract a small shared E2E runtime helper, then add `seeded-walkthrough.spec.ts` that derives its route inventory from `DASHBOARD_ROUTE_MAP`, walks a single browser session across all dashboard routes, proves live-backed truth on Issues/Alerts/Settings, and asserts that mock-only routes remain reachable and shell-stable while filtering only the already-known hidden-Issues abort noise.
  - Verify: `bash mesher/scripts/seed-live-issue.sh && bash mesher/scripts/seed-live-admin-ops.sh && npm --prefix mesher/client run test:e2e:dev -- --grep "seeded walkthrough|dashboard route parity"`
  - Done when: the repo has a deterministic seeded walkthrough spec plus helper, the spec fails on unexpected console/request regressions, and it proves both live-backed and mock-only route truth in dev.
- [ ] **T02: Close assembly blockers and document the canonical full-shell rail** `est:1h30m`
  - Why: R159 and R160 are only closed once the new walkthrough survives both runtimes and any exposed blocker is fixed at the smallest truthful seam.
  - Files: `mesher/client/tests/e2e/seeded-walkthrough.spec.ts`, `mesher/client/tests/e2e/live-runtime-helpers.ts`, `mesher/client/components/dashboard/dashboard-shell.tsx`, `mesher/client/components/dashboard/alerts-page.tsx`, `mesher/client/components/dashboard/settings/settings-page.tsx`, `mesher/client/README.md`, `mesher/api/helpers.mpl`, `mesher/storage/queries.mpl`
  - Do: Run the new walkthrough with the existing live suites in dev and prod, patch only the exact client selector/state or backend route/helper/query seam the walkthrough proves is blocking, preserve the existing shell continuity promises, and update the README with the assembled seed plus dev/prod verification rail.
  - Verify: `bash mesher/scripts/seed-live-issue.sh && bash mesher/scripts/seed-live-admin-ops.sh && npm --prefix mesher/client run test:e2e:dev -- --grep "issues live|admin and ops live|seeded walkthrough" && npm --prefix mesher/client run test:e2e:prod -- --grep "issues live|admin and ops live|seeded walkthrough"`
  - Done when: the combined seeded dev/prod greps pass, no unsupported route/control was made dishonest to satisfy the walkthrough, and maintainers can reproduce the final assembly rail from `mesher/client/README.md`.

## Files Likely Touched

- `mesher/client/components/dashboard/dashboard-route-map.ts`
- `mesher/client/tests/e2e/dashboard-route-parity.spec.ts`
- `mesher/client/tests/e2e/issues-live-read.spec.ts`
- `mesher/client/tests/e2e/issues-live-actions.spec.ts`
- `mesher/client/tests/e2e/admin-ops-live.spec.ts`
- `mesher/client/tests/e2e/live-runtime-helpers.ts`
- `mesher/client/tests/e2e/seeded-walkthrough.spec.ts`
- `mesher/client/components/dashboard/dashboard-shell.tsx`
- `mesher/client/components/dashboard/alerts-page.tsx`
- `mesher/client/components/dashboard/settings/settings-page.tsx`
- `mesher/client/README.md`
- `mesher/api/helpers.mpl`
- `mesher/storage/queries.mpl`
