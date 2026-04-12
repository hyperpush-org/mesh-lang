---
id: T03
parent: S03
milestone: M060
key_files:
  - mesher/storage/queries.mpl
  - mesher/api/helpers.mpl
  - mesher/api/team.mpl
  - mesher/client/lib/admin-ops-live-adapter.ts
  - mesher/client/components/dashboard/settings/settings-live-state.tsx
  - mesher/client/components/dashboard/settings/settings-page.tsx
  - mesher/scripts/seed-live-admin-ops.sh
  - mesher/client/tests/e2e/admin-ops-live.spec.ts
key_decisions:
  - (none)
duration: 
verification_result: mixed
completed_at: 2026-04-12T00:11:42.742Z
blocker_discovered: false
---

# T03: Partially wired Team live-state and deterministic admin/ops seeding scaffolding; backend build and full Playwright proof still need completion.

**Partially wired Team live-state and deterministic admin/ops seeding scaffolding; backend build and full Playwright proof still need completion.**

## What Happened

I updated the Mesher backend seam so organization slugs can be resolved alongside project slugs by adding `get_org_id_by_slug` in `mesher/storage/queries.mpl` and `resolve_org_id` in `mesher/api/helpers.mpl`, then switched the Team handlers in `mesher/api/team.mpl` to resolve `/api/v1/orgs/default/*` through that helper instead of assuming a UUID. I also made team mutation queries/reporting more truthful by returning affected row counts for member role and remove operations rather than always returning success.

On the client, I extended `mesher/client/lib/admin-ops-live-adapter.ts` with Team snapshot adapters and updated `mesher/client/components/dashboard/settings/settings-live-state.tsx` to bootstrap Team state, track Team mutation phases/errors, and expose same-origin add/update/remove actions. In `mesher/client/components/dashboard/settings/settings-page.tsx`, I replaced the old Team mock-only panel with a live Team UI that exposes real members, a raw-`user_id` add-member affordance, inline role changes, and owner-lock/remove semantics.

I added a new deterministic seed helper at `mesher/scripts/seed-live-admin-ops.sh`. It reuses or boots Mesher, seeds fixed Team fixtures with stable UUIDs, resets settings to the admin/ops baseline, ensures a seeded API key and alert rule/alert exist, and prints only redacted readback. I also began extending `mesher/client/tests/e2e/admin-ops-live.spec.ts` so the main settings happy-path includes Team add/role/remove coverage and the malformed-input test no longer expects Team to be mock-only.

The context-budget wrap-up event arrived before I could finish the remaining Team test updates, rerun the Mesher/backend build, or execute the full seed + Playwright dev/prod verification loop. I am recording the exact partial state here so the next unit can resume cleanly from the modified files instead of re-investigating.

## Verification

Completed only minimal closing verification before wrap-up: `bash -n mesher/scripts/seed-live-admin-ops.sh` passed, and `npm --prefix mesher/client run build` passed after the Team hook/page/adapter changes. I did not rerun the Mesher build, the new seed script end-to-end, or the `admin and ops live` Playwright suites in dev/prod before the context cap forced wrap-up.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash -n mesher/scripts/seed-live-admin-ops.sh` | 0 | ✅ pass | 100ms |
| 2 | `npm --prefix mesher/client run build` | 0 | ✅ pass | 5900ms |
| 3 | `Not run before wrap-up: backend Mesher build, `bash mesher/scripts/seed-live-admin-ops.sh`, `npm --prefix mesher/client run test:e2e:dev -- --grep "admin and ops live"`, and `npm --prefix mesher/client run test:e2e:prod -- --grep "admin and ops live"`.` | -1 | unknown (coerced from string) | 0ms |

## Deviations

Wrapped the unit early because of the automated context-budget warning. The task plan’s full verification bar was not reached in this unit, and `mesher/client/tests/e2e/admin-ops-live.spec.ts` still needs the remaining Team failure/empty-state proof updates before the slice can be considered done.

## Known Issues

Backend Mesher compilation was not rerun after the `helpers.mpl` / `team.mpl` / `queries.mpl` edits, so the org-slug route changes are not yet build-verified. The new seed script only received a shell syntax check, not a live run. `mesher/client/tests/e2e/admin-ops-live.spec.ts` is only partially updated for Team: the happy-path and malformed-input sections were started, but the Team failure-path and empty-state proof updates were not finished, and the dev/prod Playwright commands were not rerun.

## Files Created/Modified

- `mesher/storage/queries.mpl`
- `mesher/api/helpers.mpl`
- `mesher/api/team.mpl`
- `mesher/client/lib/admin-ops-live-adapter.ts`
- `mesher/client/components/dashboard/settings/settings-live-state.tsx`
- `mesher/client/components/dashboard/settings/settings-page.tsx`
- `mesher/scripts/seed-live-admin-ops.sh`
- `mesher/client/tests/e2e/admin-ops-live.spec.ts`
