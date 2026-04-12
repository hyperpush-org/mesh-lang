---
id: T02
parent: S04
milestone: M060
key_files:
  - mesher/client/components/dashboard/issue-detail.tsx
  - mesher/client/tests/e2e/issues-live-read.spec.ts
  - mesher/client/tests/e2e/admin-ops-live.spec.ts
  - mesher/client/tests/e2e/live-runtime-helpers.ts
  - mesher/client/playwright.config.ts
  - mesher/client/README.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Run the Mesher client Playwright E2E suite with `workers: 1` by default because the live route suites share and mutate one seeded backend runtime.
  - Expose explicit issue-detail stack/breadcrumb state markers so sparse live-detail proofs can assert truthful shell continuity without guessing from whole-panel text.
duration: 
verification_result: passed
completed_at: 2026-04-12T01:39:47.693Z
blocker_discovered: false
---

# T02: Stabilized the seeded Mesher dashboard proof rail, added explicit issue-detail tab markers, and documented the canonical dev/prod shell walkthrough commands.

**Stabilized the seeded Mesher dashboard proof rail, added explicit issue-detail tab markers, and documented the canonical dev/prod shell walkthrough commands.**

## What Happened

I used the new seeded walkthrough as the truth source and reproduced the combined `issues live|admin and ops live|seeded walkthrough` rail in dev before changing code. The first blocker was a stale sparse-detail assertion in `issues-live-read.spec.ts`, so I added minimal truthful observability in `mesher/client/components/dashboard/issue-detail.tsx` (`issue-detail-stack-*` and `issue-detail-breadcrumbs-*` markers) and updated the sparse-detail proof to assert the real mixed-overlay contract: sparse live event data may retain either shell-backed list content or an explicit shell-backed empty state, but the tab surface must stay mounted and truthful. The broader combined rail then exposed harness-level false negatives rather than route bugs: cross-file races against one seeded Mesher runtime and aborted local font asset fetches being treated like backend failures. I fixed only those exact seams by filtering known aborted Fontsource asset requests in `live-runtime-helpers.ts`, running the Playwright E2E suite with `workers: 1` by default in `playwright.config.ts`, and widening timeout budget only for the two alert tests that create live alert state. I also updated `mesher/client/README.md` to advertise the canonical seed + dev/prod full-shell rail and point maintainers at `tests/e2e/seeded-walkthrough.spec.ts` plus the shared runtime helper. I recorded the single-worker harness choice in GSD decisions and added a matching knowledge note so future agents do not reintroduce shared-runtime flakes.

## Verification

Passed the canonical seed/reset command (`bash mesher/scripts/seed-live-issue.sh && bash mesher/scripts/seed-live-admin-ops.sh`) and then passed the full combined dev and prod verification rails exactly as specified by the slice plan: `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live|admin and ops live|seeded walkthrough"` and `npm --prefix mesher/client run test:e2e:prod -- --grep "issues live|admin and ops live|seeded walkthrough"`. Both rails completed with 21/21 passing, covering the live Issues, Alerts, Settings, and seeded walkthrough surfaces with same-origin runtime diagnostics intact.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash mesher/scripts/seed-live-issue.sh && bash mesher/scripts/seed-live-admin-ops.sh` | 0 | ✅ pass | 5023ms |
| 2 | `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live|admin and ops live|seeded walkthrough"` | 0 | ✅ pass | 127800ms |
| 3 | `npm --prefix mesher/client run test:e2e:prod -- --grep "issues live|admin and ops live|seeded walkthrough"` | 0 | ✅ pass | 103800ms |

## Deviations

I broadened the fix slightly beyond the walkthrough spec itself by stabilizing the shared Playwright harness (`workers: 1`, known aborted font-asset filtering, and alert-seeding timeout budget). The combined seeded rail proved those were the actual failing seams, and changing them was necessary to make the exact slice verification commands truthful rather than intermittently red.

## Known Issues

None.

## Files Created/Modified

- `mesher/client/components/dashboard/issue-detail.tsx`
- `mesher/client/tests/e2e/issues-live-read.spec.ts`
- `mesher/client/tests/e2e/admin-ops-live.spec.ts`
- `mesher/client/tests/e2e/live-runtime-helpers.ts`
- `mesher/client/playwright.config.ts`
- `mesher/client/README.md`
- `.gsd/KNOWLEDGE.md`
