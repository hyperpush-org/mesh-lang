---
id: T03
parent: S04
milestone: M061
key_files:
  - ../hyperpush-mono/mesher/scripts/seed-live-issue.sh
  - ../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh
  - ../hyperpush-mono/mesher/scripts/seed-live-admin-ops.sh
  - ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs
  - ../hyperpush-mono/mesher/client/hooks/use-toast.ts
  - ../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts
  - ../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Make live issue seeding isolated-by-default with explicit backend reuse opt-in, and align the delegated verifier to stop pinning issue-seed WS/cluster ports.
  - Treat the combined dev proof flake as a runtime-hardening problem: stabilize the shared toast listener and extend alert-seed polling instead of weakening the Playwright assertions.
duration: 
verification_result: mixed
completed_at: 2026-04-12T18:27:01.027Z
blocker_discovered: true
---

# T03: Hardened isolated live-issue seeding, added regression coverage, and narrowed the remaining wrapper blocker to `seed-live-issue.sh` readiness against its own isolated backend.

**Hardened isolated live-issue seeding, added regression coverage, and narrowed the remaining wrapper blocker to `seed-live-issue.sh` readiness against its own isolated backend.**

## What Happened

I updated `../hyperpush-mono/mesher/scripts/seed-live-issue.sh` so backend reuse is opt-in via `MESHER_REUSE_RUNNING_BACKEND=true` instead of silently trusting any responder on the chosen port. The script now picks an isolated backend port by default, guards high-port overflow in its port picker, and uses a well-formed settings payload rather than a hard-coded `retention_days=90` value as the readiness shape check. I aligned `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` with that contract by removing the hard-pinned issue-seed WS/cluster ports that prevented isolated startup from binding.

I extended `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` with a runtime regression that proves default isolation versus explicit reuse, and I hardened the Mesher client proof helpers under load by (1) wrapping the client toast subscription listener in `../hyperpush-mono/mesher/client/hooks/use-toast.ts` so rapid mount/unmount cycles do not replay stale `setState` listeners, and (2) extending the alert-seeding polls plus failure diagnostics in `../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts` and `../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts`.

The structural contract and the delegated dev Playwright subset passed after those changes, but the final root wrapper still fails earlier in `seed-live-issue.sh`. The retained artifact shows the isolated Mesher process logging `Runtime ready` while `project-settings-last-response.txt` stays empty and the script times out waiting for `/api/v1/projects/default/settings`. A manual control run confirmed `seed-live-admin-ops.sh` still succeeds, so the remaining blocker is specific to the isolated issue-seed readiness path rather than the general Mesher build/runtime or the downstream Playwright phases.

Because the remaining wrapper failure is now concentrated in that readiness seam and needs fresh investigation, I recorded a durable knowledge note and decision so the next unit can resume from the retained `seed-live-issue` artifacts instead of redoing the earlier isolation, Playwright, and port-range work.

## Verification

Passed: `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` after the new runtime regression coverage; `npm --prefix ../hyperpush-mono/mesher/client exec -- playwright test --config ../hyperpush-mono/mesher/client/playwright.config.ts --project=dev --grep "dashboard route parity|issues live|admin and ops live|seeded walkthrough"` after the toast/poll hardening; and `env MESHER_SEED_ARTIFACT_DIR=../hyperpush-mono/.tmp/manual-seed-admin-ops bash ../hyperpush-mono/mesher/scripts/seed-live-admin-ops.sh` as a control proving isolated admin/ops seeding still works.

Failed: `bash ../hyperpush-mono/scripts/verify-m061-s04.sh` still exits through delegated phase `seed-live-issue`. The retained log at `../hyperpush-mono/.tmp/m061-s01/verify-client-route-inventory/seed-live-issue/mesher.log` shows the isolated backend reaching `Runtime ready`, but `../hyperpush-mono/.tmp/m061-s01/verify-client-route-inventory/seed-live-issue/project-settings-last-response.txt` is empty and the wrapper fails before the downstream dev/prod proofs run.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` | 0 | ✅ pass | 3631ms |
| 2 | `npm --prefix ../hyperpush-mono/mesher/client exec -- playwright test --config ../hyperpush-mono/mesher/client/playwright.config.ts --project=dev --grep "dashboard route parity|issues live|admin and ops live|seeded walkthrough"` | 0 | ✅ pass | 210000ms |
| 3 | `bash ../hyperpush-mono/scripts/verify-m061-s04.sh` | 1 | ❌ fail | 53900ms |

## Deviations

I made one additional consistency fix outside the original file list by patching `../hyperpush-mono/mesher/scripts/seed-live-admin-ops.sh` with the same high-port overflow guard used by the new isolated port picker. I also hardened the existing Playwright proof helpers (`client/hooks/use-toast.ts`, `admin-ops-live.spec.ts`, and `seeded-walkthrough.spec.ts`) after the delegated dev suite exposed timing-sensitive runtime noise under the combined verification load.

## Known Issues

`bash ../hyperpush-mono/scripts/verify-m061-s04.sh` still fails in delegated phase `seed-live-issue`. The isolated `seed-live-issue.sh` backend logs `Runtime ready` on its chosen port, but the script never records a successful `/api/v1/projects/default/settings` response and times out with an empty `project-settings-last-response.txt`. Resume from `../hyperpush-mono/.tmp/m061-s01/verify-client-route-inventory/seed-live-issue/mesher.log` plus the empty last-response artifact and compare that path against the working `seed-live-admin-ops.sh` control. This is the only remaining blocker observed after the structural contract and the delegated dev Playwright subset passed.

## Files Created/Modified

- `../hyperpush-mono/mesher/scripts/seed-live-issue.sh`
- `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh`
- `../hyperpush-mono/mesher/scripts/seed-live-admin-ops.sh`
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
- `../hyperpush-mono/mesher/client/hooks/use-toast.ts`
- `../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts`
- `../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts`
- `.gsd/KNOWLEDGE.md`
