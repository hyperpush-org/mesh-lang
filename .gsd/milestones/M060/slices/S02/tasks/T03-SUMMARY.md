---
id: T03
parent: S02
milestone: M060
key_files:
  - mesher/scripts/seed-live-issue.sh
  - mesher/client/tests/e2e/issues-live-actions.spec.ts
  - mesher/client/tests/e2e/issues-live-read.spec.ts
  - mesher/client/README.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Use separate deterministic seeded issues for the read seam and the action seam so the combined dev/prod `issues live` Playwright runs stay truthful under parallel workers while still using the exact slice verification commands.
duration: 
verification_result: passed
completed_at: 2026-04-11T22:44:09.867Z
blocker_discovered: false
---

# T03: Made the Issues live proof replay-safe across dev and prod by seeding separate read/action issues and documenting the supported same-origin seam.

**Made the Issues live proof replay-safe across dev and prod by seeding separate read/action issues and documenting the supported same-origin seam.**

## What Happened

I rewrote `mesher/scripts/seed-live-issue.sh` into a replay-safe two-fixture seed helper for S02: it now seeds a dedicated read-proof issue and a dedicated action-proof issue, verifies both through real detail/timeline readbacks, and force-resets the action issue back to `unresolved` so repeated resolve/reopen/archive proofs start from a truthful open state instead of inheriting stale status from the previous run. I then refactored `mesher/client/tests/e2e/issues-live-actions.spec.ts` to stop posting fresh events inside every test, look up the deterministic action-proof issue, reset it to open through the same backend seam when needed, and prove the real maintainer loop end to end by replaying `Resolve`, `Reopen`, and `Ignore` against the same same-origin issue while still asserting busy state, destructive mutation toasts, overview-refresh failure handling, unsupported-action validation, and same-origin request capture. In `mesher/client/tests/e2e/issues-live-read.spec.ts` I made the read-proof issue lookup follow the new replay-safe seed contract, filtered expected bootstrap abort noise from the request tracker, and replaced one brittle sparse-fallback assertion with a truthful check that the fallback stack/breadcrumb shell stays mounted even when the live detail payload is sparse. Finally, I updated `mesher/client/README.md` so maintainers see the real S02 contract: supported live actions are Resolve/Reopen/Ignore, shell-only controls stay visibly unsupported, and the canonical verification commands are the exact seeded `issues live` dev/prod runs. I also added a knowledge entry noting that the combined `issues live` suites must use separate deterministic read/action issues because the harness runs the two spec files in parallel workers and a shared seeded issue lets action mutations race the read-side proofs.

## Verification

Verified the exact slice command set, not a weaker smoke check. `bash mesher/scripts/seed-live-issue.sh` now seeds both the read and action proof issues, validates their latest event detail plus timeline surfaces, and leaves the action proof issue reopened for deterministic maintainer-loop replay. `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live"` passed with the combined read/action suite proving same-origin issue reads, Resolve/Reopen/Ignore happy paths, disabled-state handling during writes, destructive mutation and refresh-failure toasts, truthful list/detail/source markers, and validation-only proof-rail failures. `npm --prefix mesher/client run test:e2e:prod -- --grep "issues live"` passed with the same assertions against the built production server, confirming the supported live seam behaves the same in both runtimes.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash mesher/scripts/seed-live-issue.sh` | 0 | ✅ pass | 1800ms |
| 2 | `npm --prefix mesher/client run test:e2e:dev -- --grep "issues live"` | 0 | ✅ pass | 33800ms |
| 3 | `npm --prefix mesher/client run test:e2e:prod -- --grep "issues live"` | 0 | ✅ pass | 26600ms |

## Deviations

Used two deterministic seeded issues (`read seam` and `action seam`) instead of one shared seeded issue. The combined `issues live` grep runs the read and action spec files in parallel workers, so a single shared issue let action mutations race the read-side sparse/failure assertions. Splitting the fixtures kept the exact verification commands deterministic without weakening coverage.

## Known Issues

None.

## Files Created/Modified

- `mesher/scripts/seed-live-issue.sh`
- `mesher/client/tests/e2e/issues-live-actions.spec.ts`
- `mesher/client/tests/e2e/issues-live-read.spec.ts`
- `mesher/client/README.md`
- `.gsd/KNOWLEDGE.md`
