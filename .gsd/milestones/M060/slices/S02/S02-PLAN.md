# S02: Core maintainer loop live

**Goal:** Make the existing Issues maintainer loop truthful by wiring supported issue actions and the existing Issues summary chrome to the Mesher backend through the provider-owned same-origin seam, while keeping unsupported controls and fallback-only fields visibly honest.
**Demo:** The dashboard summaries and issue actions are live: operators can inspect real issues, perform existing issue actions, and see backend-backed summary data instead of broad mock stats.

## Must-Haves

- Wire the supported maintainer action set (`resolve`, `unresolve`, `archive` as ignore) through same-origin `/api/v1/issues/:id/...` mutations owned by `DashboardIssuesStateProvider`, without moving the Issues route into route loaders or inventing a second state seam. (`R153`, advances `R154`)
- Refresh list, detail, filter, and stats/chart state truthfully after every successful mutation by invalidating selected snapshots and refetching provider-owned overview/detail data instead of trusting thin `{"status":"ok"}` responses. (`R153`, `R156`)
- Replace broad mock-only Issues summary signals with backend-backed summary data derived from existing dashboard/issue routes while keeping unsupported fields visibly fallback-backed rather than pretending they are live. (`R153`, `R156`)
- Surface selected-issue action failures through the mounted Radix toaster and preserve operator context; unsupported actions (`assign`, `discard`, `delete`) must not pretend to be live in S02. (`R156`, `R158`, advances `R154`)

## Threat Surface

- **Abuse**: arbitrary issue-id mutation attempts, repeated resolve/archive replay, or stale cached detail after a successful write could mislead operators if the client patches state optimistically; keep all writes on same-origin `/api/v1`, normalize the supported action set explicitly, and re-read provider state after mutation.
- **Data exposure**: issue ids, event metadata, and backend error text are visible to the operator, but secrets/API keys are not part of this surface; toasts and logs must avoid echoing raw backend bodies or unrelated seeded credentials.
- **Input trust**: selected issue ids, active filters, route payloads, and mutation success/error bodies are all untrusted inputs that must be validated and mapped before they affect UI state.

## Requirement Impact

- **Requirements touched**: `R153`, `R154`, `R156`, `R158`
- **Re-verify**: same-origin Issues boot/read seam from S01, live issue status changes under active filters, provider-backed summary chrome, and destructive toast visibility for induced mutation or refresh failures in both dev and prod Playwright runs.
- **Decisions revisited**: `D508`, `D510`, `D511`, `D513`, `D515`

## Proof Level

- This slice proves: integration
- Real runtime required: yes
- Human/UAT required: no

## Verification

- `bash mesher/scripts/seed-live-issue.sh`
- `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live"`
- `npm --prefix mesher/client run test:e2e:prod -- --grep "issues live"`

## Observability / Diagnostics

- Runtime signals: same-origin issue-mutation requests, `issues-shell` / detail-panel `data-*` attributes for action pending/error/source state, live status badges in the list/detail panel, and the mounted Radix toast region.
- Inspection surfaces: `bash mesher/scripts/seed-live-issue.sh`, `mesher/client/tests/e2e/issues-live-actions.spec.ts`, browser network logs, the visible toaster surface, and the existing issues-shell attributes already used by Playwright.
- Failure visibility: last mutation action/error, post-mutation refresh failure, stale selected-issue cache mismatches, and destructive toast text instead of silent fallback.
- Redaction constraints: do not expose API keys, raw `user_context`, or backend error bodies in browser assertions, toasts, or docs.

## Integration Closure

- Upstream surfaces consumed: `mesher/client/lib/mesher-api.ts`, `mesher/client/lib/issues-live-adapter.ts`, `mesher/client/components/dashboard/dashboard-issues-state.tsx`, `mesher/client/components/dashboard/issue-detail.tsx`, `mesher/client/components/dashboard/issue-list.tsx`, `mesher/client/components/dashboard/stats-bar.tsx`, `mesher/ingestion/routes.mpl`, `mesher/api/dashboard.mpl`, and the existing seeded Playwright harness.
- New wiring introduced in this slice: same-origin issue mutation helpers, provider-owned refresh/invalidation for post-mutation truth, maintainer controls in the existing issue-detail action row, and action-focused live browser verification.
- What remains before the milestone is truly usable end-to-end: S03 still needs admin/ops surfaces wired live, and S04 still needs the full seeded local walkthrough plus any narrow backend seam repairs discovered during integration.

## Tasks

- [x] **T01: Add same-origin issue mutation orchestration and the live action-proof harness** `est:2h`
  - Why: Live issue writes must use the same transport/error boundary as S01 and must not leave cached list/detail state stale.
  - Files: `mesher/client/lib/mesher-api.ts`, `mesher/client/components/dashboard/dashboard-issues-state.tsx`, `mesher/client/components/dashboard/issues-page.tsx`, `mesher/client/tests/e2e/issues-live-actions.spec.ts`
  - Do: Extend the shared Mesher client with typed `resolve` / `unresolve` / `archive` helpers, refactor the provider to own refresh and snapshot invalidation after mutations, expose action diagnostics through stable `data-*` attributes, and create the first action-focused Playwright proof file.
  - Verify: `bash mesher/scripts/seed-live-issue.sh && npm --prefix mesher/client run test:e2e:dev -- --grep "issues live actions"`
  - Done when: the provider owns supported issue mutations end to end, stale selected-detail caches are invalidated after writes, and the new action proof file can assert same-origin writes plus visible failure feedback.
- [ ] **T02: Wire supported maintainer controls and backend-backed Issues summary signals** `est:2h`
  - Why: The slice is not truthful until operators can actually trigger the supported actions and see the refreshed live status and summary signals in the existing shell.
  - Files: `mesher/client/components/dashboard/issue-detail.tsx`, `mesher/client/lib/issues-live-adapter.ts`, `mesher/client/components/dashboard/issue-list.tsx`, `mesher/client/components/dashboard/stats-bar.tsx`
  - Do: Add `Resolve`, `Reopen`, and `Ignore` controls to the existing detail action row with busy/disabled state, tighten list/status/summary derivations so refreshed live data drives the visible shell, and keep unsupported actions out of the S02 live surface.
  - Verify: `bash mesher/scripts/seed-live-issue.sh && npm --prefix mesher/client run test:e2e:dev -- --grep "issues live actions"`
  - Done when: the action row exposes only the supported live actions, filtered list/status badges update from refreshed provider data, and the summary chrome no longer relies on broad mock-only signals.
- [ ] **T03: Finish seeded dev/prod maintainer-loop proof and document the supported live seam** `est:1h30m`
  - Why: The slice only closes once the maintainer loop is deterministic in both runtimes and the supported live seam is documented for future maintainers.
  - Files: `mesher/scripts/seed-live-issue.sh`, `mesher/client/tests/e2e/issues-live-actions.spec.ts`, `mesher/client/tests/e2e/issues-live-read.spec.ts`, `mesher/client/README.md`
  - Do: Make the seeded issue replay-safe for repeated action assertions, finish dev/prod Playwright happy-path and failure-path coverage for the combined `issues live` suite, and document the supported S02 action set plus verification commands in the client README.
  - Verify: `bash mesher/scripts/seed-live-issue.sh && npm --prefix mesher/client run test:e2e:dev -- --grep "issues live" && npm --prefix mesher/client run test:e2e:prod -- --grep "issues live"`
  - Done when: the seed helper is replay-safe, both runtimes prove same-origin issue actions with destructive toast coverage, and the README documents the supported live seam without claiming unsupported actions are ready.

## Files Likely Touched

- `mesher/client/lib/mesher-api.ts`
- `mesher/client/components/dashboard/dashboard-issues-state.tsx`
- `mesher/client/components/dashboard/issues-page.tsx`
- `mesher/client/tests/e2e/issues-live-actions.spec.ts`
- `mesher/client/components/dashboard/issue-detail.tsx`
- `mesher/client/lib/issues-live-adapter.ts`
- `mesher/client/components/dashboard/issue-list.tsx`
- `mesher/client/components/dashboard/stats-bar.tsx`
- `mesher/scripts/seed-live-issue.sh`
- `mesher/client/tests/e2e/issues-live-read.spec.ts`
- `mesher/client/README.md`
