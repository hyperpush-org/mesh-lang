# M060 — Research

**Date:** 2026-04-11

## Summary

M060 should stay a client-wiring milestone, not a redesign or backend-expansion wave. S01 already proved the durable pattern: keep browser traffic on same-origin `/api/v1`, centralize transport and payload normalization in `mesher/client/lib/mesher-api.ts` plus focused adapters, let provider-owned client state orchestrate reads/writes, and surface backend failures through the mounted Radix toaster instead of inventing new UX.

That same pattern should drive the remaining slices. S02 can make the maintainer loop truthful without widening into S03 by keeping orchestration inside `DashboardIssuesStateProvider`, wiring only the supported issue actions that the current shell can represent honestly, and tightening the existing Issues summary chrome so it depends on backend-backed data instead of broad mock stats. The low-risk S02 action set is `resolve`, `unresolve`, and `archive` (rendered as ignore). `assign` needs org-member context the current bootstrap does not expose cheaply, and `discard` introduces backend status `discarded`, which the current issue-shell type system does not represent.

S03 is similar in shape but broader in scope: alerts, settings/storage, team, and API-key surfaces already have backend routes, but many payloads are leaner than the current UI. The main blocker there is truthful context, especially the org-scoped team-member seam. S04 should stay focused on final seeded-local walkthrough proof plus only the narrow backend repairs required to make already-existing routes usable from the client.

## Recommendation

1. Reuse the S01 seam everywhere possible.
   - Same-origin `/api/v1` transport only.
   - One typed fetch/error layer in `mesher/client/lib/mesher-api.ts`.
   - Provider-owned orchestration and cache invalidation above leaf components.
   - Existing Radix toast surface for visible failure feedback.

2. Keep mixed live/mock shell behavior explicit but quiet.
   - Make backend-backed fields and actions truthful.
   - Keep unsupported shell sections visible with stable fallback/mock data.
   - Do not widen scope into new auth UX, route-loader rewrites, or new backend surfaces.

3. Treat action/status vocabulary mismatches as first-class planning constraints.
   - S02 should support only `resolve`, `unresolve`, and `archive` unless execution proves assignment can be added cheaply.
   - Defer `discard` until the client has a truthful `discarded` mapping.
   - Defer assignment until org/member context is available without pulling S03 forward.

4. Keep verification runtime-backed and browser-first.
   - Reuse the seeded-local Mesher helper path.
   - Extend Playwright with explicit same-origin request assertions, visible-toast failure checks, and filter/status refresh assertions.
   - Prove both dev and built-prod runtimes before calling the milestone integrated.

## Cross-slice Constraints

- The current dashboard shell is richer than the Mesher backend contracts; adapters must preserve unsupported fields visibly instead of weakening the shell contract.
- Reads and writes are split across backend modules (`api/search.mpl` / `api/dashboard.mpl` for reads, `ingestion/routes.mpl` for issue mutations, separate admin/ops modules for S03).
- The client does not consume websocket broadcasts today, so post-mutation truth must come from provider-owned refetch/invalidation, not optimistic assumptions.
- The seeded default org UUID is runtime-generated and not safe to hardcode; S03 must solve org-context discovery explicitly.
- Verification and UI feedback must avoid exposing secrets, API keys, or raw event `user_context`/`extra` payloads.

## Expected Slice Shape

- **S02** — provider-owned issue mutation orchestration, minimal maintainer controls in the existing detail action row, live-backed issue summary chrome, and dev/prod Playwright proof for same-origin issue actions plus failure toasts.
- **S03** — provider/hook-owned admin and ops wiring for alerts, settings/storage, team, and API keys, with truthful live/mock boundaries and explicit handling of the org-context gap.
- **S04** — assembled seeded-local walkthrough proof for the full backend-backed shell, plus only the backend seam repairs directly required to make existing routes usable.

## Verification Direction

- `bash mesher/scripts/seed-live-issue.sh`
- `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live|admin and ops live|seeded walkthrough"`
- `npm --prefix mesher/client run test:e2e:prod -- --grep "issues live|admin and ops live|seeded walkthrough"`

The milestone should only complete once the full backend-backed shell walkthrough is proven in a seeded local environment without direct browser calls to Mesher backend ports and without silent failure fallback.