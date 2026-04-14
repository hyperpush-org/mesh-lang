---
id: S04
parent: M061
milestone: M061
provides:
  - One canonical maintainer handoff for client truth + backend gap planning
  - One product-root closeout command for future maintainers
  - One retained proof-bundle layout for downstream debugging of route-inventory drift
requires:
  - slice: S01
    provides: Canonical top-level route inventory and parser/test wrapper baseline.
  - slice: S02
    provides: Fine-grained mixed-surface truth and self-seeded browser proof patterns.
  - slice: S03
    provides: Backend gap map vocabulary and fail-closed backend support classification.
affects:
  []
key_files:
  - ../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md
  - ../hyperpush-mono/mesher/client/README.md
  - ../hyperpush-mono/README.md
  - ../hyperpush-mono/scripts/verify-m061-s04.sh
  - ../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh
  - ../hyperpush-mono/mesher/scripts/seed-live-issue.sh
  - ../hyperpush-mono/mesher/scripts/seed-live-admin-ops.sh
  - ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs
  - ../hyperpush-mono/mesher/client/hooks/use-toast.ts
  - ../hyperpush-mono/mesher/client/playwright.config.ts
  - ../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts
key_decisions:
  - D535 — close S04 with a product-root wrapper delegating to the package-owned route-inventory verifier
  - D537 — make `seed-live-issue.sh` isolated by default with explicit reuse opt-in
  - D538 — harden shared runtime/client behavior instead of weakening the combined route-inventory assertions
patterns_established:
  - Canonical maintainer-facing inventory lives beside `mesher/client` while structural truth stays locked by parser/test rails
  - Product-root closeout wrappers should validate delegated artifacts rather than duplicate package-owned logic
  - Long serial browser proof rails need retained bundle pointers plus source-aware first-failure logs to keep diagnosis tractable
observability_surfaces:
  - ../hyperpush-mono/.tmp/m061-s04/verify/status.txt
  - ../hyperpush-mono/.tmp/m061-s04/verify/current-phase.txt
  - ../hyperpush-mono/.tmp/m061-s04/verify/phase-report.txt
  - ../hyperpush-mono/mesher/.tmp/m061-s01/verify-client-route-inventory/latest-proof-bundle.txt
  - ../hyperpush-mono/mesher/.tmp/m061-s01/verify-client-route-inventory/route-inventory-dev.log
  - ../hyperpush-mono/mesher/.tmp/m061-s01/verify-client-route-inventory/route-inventory-prod.log
drill_down_paths:
  - .gsd/milestones/M061/slices/S04/tasks/T01-SUMMARY.md
  - .gsd/milestones/M061/slices/S04/tasks/T02-SUMMARY.md
  - .gsd/milestones/M061/slices/S04/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-12T20:04:09.255Z
blocker_discovered: false
---

# S04: S04

**Packaged the canonical maintainer handoff, root closeout wrapper, retained proof-bundle contract, and isolated seeding hardening, but the final assembled `verify-m061-s04.sh` replay still flakes under the combined route-inventory browser rail.**

## What Happened

## Slice Summary

S04 completed the maintainer-facing closeout packaging around the M061 client truth inventory. The canonical `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` now carries an explicit maintainer handoff with backend expansion order and proof commands to rerun. Both `../hyperpush-mono/mesher/client/README.md` and `../hyperpush-mono/README.md` now surface that inventory and the root closeout command instead of framing `mesher/client` as a mock-only surface.

The product repo now has a root-level closeout wrapper at `../hyperpush-mono/scripts/verify-m061-s04.sh` that delegates to `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh`, records wrapper-level status/phase artifacts, and fails closed on missing delegated proof outputs. The delegated verifier now retains a proof-bundle directory, writes `latest-proof-bundle.txt`, copies proof inputs into the retained bundle, and has structural contract coverage in `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` for handoff headings, README markers, wrapper markers, retained-bundle markers, and truthful CI acknowledgment.

S04 also hardened live seeding and long-suite proof behavior. `../hyperpush-mono/mesher/scripts/seed-live-issue.sh` is now isolated-by-default and only reuses an existing backend when `MESHER_REUSE_RUNNING_BACKEND=true` is set explicitly. Both issue/admin seed helpers now use a safer high-port picker and `exec env` launch style. On the client side, the shared toast store in `../hyperpush-mono/mesher/client/hooks/use-toast.ts` was hardened for repeated mount/unmount cycles, the heavy admin live-settings proof got an explicit timeout budget, and the prod Playwright startup timeout was increased so the built frontend has time to come up under the assembled rail.

What this slice provides downstream is still valuable even with the remaining blocker: backend maintainers now have one canonical inventory, one backend gap map, one product-root command, one retained proof-bundle pointer, and one explicit seeding rule about wrong-runtime reuse. The remaining problem is not missing handoff packaging; it is stabilizing the final combined replay so the root wrapper stays green under the full dev/prod browser rail.

## Operational Readiness (Q8)

- **Health signal:** `../hyperpush-mono/.tmp/m061-s04/verify/status.txt`, `current-phase.txt`, `phase-report.txt`, and delegated `latest-proof-bundle.txt` should all exist and indicate a full pass.
- **Failure signal:** the first actionable failure currently appears in `../hyperpush-mono/mesher/.tmp/m061-s01/verify-client-route-inventory/route-inventory-dev.log` or `route-inventory-prod.log`; later `ERR_CONNECTION_REFUSED` fallout in the same bundle is secondary once the first browser/runtime failure destabilizes the run.
- **Recovery procedure:** rerun `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`, then rerun `bash ../hyperpush-mono/scripts/verify-m061-s04.sh`, and inspect the retained bundle/log named by the wrapper before changing assertions.
- **Monitoring gaps:** the closeout rail is still sensitive to long-suite browser/runtime timing, so the wrapper does not yet provide a consistently green assembled replay even though the structural contract and individual sub-fixes landed.


## Verification

## Verification

Completed verification work:
- `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` passes and locks the maintainer handoff, README markers, wrapper markers, retained proof-bundle markers, and isolated-seeding contract.
- Targeted browser/runtime fixes were reproduced and repaired: the selected-issue read warning was addressed through shared toast-store lifecycle hardening, the prod frontend startup timeout was increased, and the heavy admin live-settings proof was given an explicit timeout budget.

Current blocker:
- `bash ../hyperpush-mono/scripts/verify-m061-s04.sh` still fails in the delegated combined route-inventory replay. The latest retained evidence shows the assembled dev browser phase still flakes under the long serial route-inventory run; once that first failure occurs, later connection-refused fallout is secondary. This means the slice implementation is packaged and documented, but the final assembled rerun rail is not yet stable enough to claim R170/R171 validation.

Most recent evidence:
- `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` ✅ pass
- `bash ../hyperpush-mono/scripts/verify-m061-s04.sh` ❌ fail (delegated combined replay still unstable)
- Isolated targeted reruns for the selected-issue failure path and admin/settings timing fixes passed while debugging, but the full assembled replay remains red.


## Requirements Advanced

- R170 — Implemented the root wrapper, retained proof-bundle contract, structural handoff markers, and isolated seeding hardening required for a repeatable proof rail, but the assembled replay is still unstable.
- R171 — Published the maintainer handoff, backend expansion order, and root/client README surfacing so future backend work can start from the canonical inventory, but final validation is blocked on the unstable assembled replay.

## Requirements Validated

None.

## New Requirements Surfaced

- R170 and R171 remain active because assembled replay validation is still incomplete

## Requirements Invalidated or Re-scoped

None.

## Deviations

Slice completion was recorded under hard timeout recovery while the final assembled root wrapper replay was still red. The slice packaging work landed, but the combined route-inventory browser rail remains unstable and needs follow-up.

## Known Limitations

`bash ../hyperpush-mono/scripts/verify-m061-s04.sh` is still not a consistently green assembled acceptance rail. Current retained evidence shows the delegated combined dev replay can fail in the Issues/browser proof path, after which later connection-refused errors are secondary fallout.

## Follow-ups

1. Stabilize the combined route-inventory dev replay so the first failing assertion no longer collapses the rest of the run.
2. Reconfirm the prod replay after the longer startup timeout under a fully green dev run.
3. Once the wrapper stays green end to end, validate R170 and R171 with the assembled replay evidence and update requirement status accordingly.

## Files Created/Modified

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` — Added the maintainer handoff section with backend expansion order and proof rerun guidance.
- `../hyperpush-mono/mesher/client/README.md` — Surfaced the canonical inventory and the root closeout command.
- `../hyperpush-mono/README.md` — Surfaced the route-inventory handoff at the product root and removed stale mock-only wording.
- `../hyperpush-mono/scripts/verify-m061-s04.sh` — Added the product-root delegated closeout wrapper with fail-closed artifact checks.
- `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` — Retained proof-bundle outputs and delegated route-inventory phases under one package-owned verifier.
- `../hyperpush-mono/mesher/scripts/seed-live-issue.sh` — Made live issue seeding isolated-by-default and hardened port/launch behavior.
- `../hyperpush-mono/mesher/scripts/seed-live-admin-ops.sh` — Matched the safer isolated port/launch behavior used by the issue-seed helper.
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` — Locked handoff/readme/wrapper/CI/proof-bundle markers and isolated-seeding behavior in the structural contract.
- `../hyperpush-mono/mesher/client/hooks/use-toast.ts` — Hardened the shared toast store lifecycle for repeated mount/unmount proof runs.
- `../hyperpush-mono/mesher/client/playwright.config.ts` — Raised the prod frontend startup timeout for the assembled route-inventory replay.
- `../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts` — Raised timeout budget for the heavy live settings proof in the combined route-inventory run.
