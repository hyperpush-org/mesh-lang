---
estimated_steps: 5
estimated_files: 8
skills_used:
  - react-best-practices
  - bash-scripting
  - playwright-best-practices
---

# T03: Resolve org-scoped team context and close the seeded admin/ops proof in dev and prod

**Slice:** S03 — Admin and ops surfaces live
**Milestone:** M060

## Description

Finish the remaining blocked surface by making org-scoped Team actions reachable without hardcoding UUIDs. The default organization already has a stable slug, so the narrow seam repair is to resolve org identifiers the same way project identifiers are resolved instead of inventing a brand-new context route. Then wire live member list/add/role/remove behavior into the Team tab with a truthful raw-`user_id` add-member affordance, create a deterministic admin/ops seed helper, and close the browser proof across dev and prod.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Org identifier resolution in `mesher/api/helpers.mpl` and team handlers | Fail the Team bootstrap/action with an explicit error and leave the shell visible; do not hardcode the seeded org UUID in the client. | N/A | Reject unknown slug/id lookup failures and surface them as Team read/write errors. |
| `GET/POST /api/v1/orgs/default/members` and role/remove mutations | Keep the Team tab mounted, preserve the currently rendered list, and show a destructive toast without pretending the member change applied. | Clear pending state, keep the selected tab active, and expose timeout diagnostics in Team state markers. | Reject malformed member rows or missing IDs and stop the action instead of mutating local state heuristically. |

## Load Profile

- **Shared resources**: org-membership reads/writes, seeded helper state, and the combined admin/ops Playwright suite.
- **Per-operation cost**: one team bootstrap read plus one POST and a list refresh per member add/role/remove action.
- **10x breakpoint**: repeated member mutations will reveal stale list refreshes and user-id validation gaps before backend cost becomes meaningful, so read-after-write truth must stay explicit.

## Negative Tests

- **Malformed inputs**: empty `user_id`, unknown `user_id`, invalid role, or missing membership id on update/remove.
- **Error paths**: team bootstrap 500, add-member 400/500, role/remove failures, and failure-to-toast coverage in the browser proof.
- **Boundary conditions**: seeded default org resolved by slug, owner rows that cannot be removed through the current shell, and an empty member delta after a failed mutation.

## Steps

1. Add `get_org_id_by_slug` in `mesher/storage/queries.mpl`, expose `resolve_org_id` from `mesher/api/helpers.mpl`, and update `mesher/api/team.mpl` so `/api/v1/orgs/default/members` and related mutations accept the seeded org slug without a new routed context endpoint.
2. Extend `mesher/client/lib/mesher-api.ts` and `mesher/client/components/dashboard/settings/settings-live-state.tsx` with Team list/add/role/remove helpers that target `/api/v1/orgs/default/members` and preserve the same-origin timeout/error semantics already used elsewhere in the dashboard.
3. Refactor the Team section in `mesher/client/components/dashboard/settings/settings-page.tsx` to show live members, truthful role/remove actions, and a minimal additive raw-`user_id` add-member affordance instead of a fake email invite flow the backend cannot honor.
4. Create `mesher/scripts/seed-live-admin-ops.sh` so verification can seed replay-safe alerts, rules, keys, and team fixtures against the running Mesher backend without printing secrets.
5. Finish `mesher/client/tests/e2e/admin-ops-live.spec.ts` with full seeded coverage for alerts, settings/storage, API keys, alert rules, and Team flows in dev and prod, including same-origin request assertions and destructive-toast failure proof.

## Must-Haves

- [ ] Team member routes accept the stable seeded org slug through a narrow helper/query seam; no client hardcodes a generated org UUID.
- [ ] The Team tab lists real members and supports truthful add/role/remove actions that match the existing backend contract.
- [ ] The add-member affordance is honest about the backend requirement (`user_id`), not a fake email invite experience.
- [ ] `mesher/scripts/seed-live-admin-ops.sh` plus `mesher/client/tests/e2e/admin-ops-live.spec.ts` prove the full admin/ops slice in both dev and prod runtimes.

## Verification

- `bash mesher/scripts/seed-live-admin-ops.sh && npm --prefix mesher/client run test:e2e:dev -- --grep "admin and ops live" && npm --prefix mesher/client run test:e2e:prod -- --grep "admin and ops live"`
- The Team tab works through `/api/v1/orgs/default/members`, not a hardcoded UUID or fake invite path, and the full admin/ops proof stays same-origin in both runtimes.

## Observability Impact

- Signals added/changed: `settings-shell` Team state markers, last team action/error metadata, and deterministic seed helper readback output for alerts/settings/team fixtures.
- How a future agent inspects this: run `bash mesher/scripts/seed-live-admin-ops.sh`, replay the combined `admin and ops live` Playwright grep in dev/prod, and inspect the visible Team tab plus toast region.
- Failure state exposed: slug-resolution failures, rejected `user_id` adds, and stale team-list refreshes instead of silent fake invites.

## Inputs

- `mesher/storage/queries.mpl` — existing org/project lookup queries that currently resolve only project slugs.
- `mesher/api/helpers.mpl` — current helper module that needs the org-slug resolution seam.
- `mesher/api/team.mpl` — existing team/API-key handlers whose org-member paths currently require raw UUIDs.
- `mesher/client/lib/mesher-api.ts` — admin/ops client boundary that must absorb Team reads and writes.
- `mesher/client/components/dashboard/settings/settings-live-state.tsx` — live Settings state owner from T02 that needs Team orchestration.
- `mesher/client/components/dashboard/settings/settings-page.tsx` — Team tab shell that currently implies a fake invite flow.
- `mesher/client/tests/e2e/admin-ops-live.spec.ts` — proof file that should close the full slice.
- `mesher/migrations/20260226000000_seed_default_org.mpl` — source of truth that the seeded default organization slug is stable while the UUID is not.

## Expected Output

- `mesher/storage/queries.mpl` — org-slug lookup helper used by the API boundary.
- `mesher/api/helpers.mpl` — shared org identifier resolution helper alongside project resolution.
- `mesher/api/team.mpl` — Team handlers that accept the seeded default org slug for list/add/role/remove flows.
- `mesher/client/lib/mesher-api.ts` — Team request helpers and parsers using same-origin `/api/v1/orgs/default/members` calls.
- `mesher/client/components/dashboard/settings/settings-live-state.tsx` — Team bootstrap/mutation orchestration with toast-backed failures.
- `mesher/client/components/dashboard/settings/settings-page.tsx` — truthful Team tab UI with live list/role/remove and raw-`user_id` add-member affordance.
- `mesher/scripts/seed-live-admin-ops.sh` — deterministic seed/readback helper for admin/ops verification.
- `mesher/client/tests/e2e/admin-ops-live.spec.ts` — full dev/prod proof for the live admin/ops seam.
