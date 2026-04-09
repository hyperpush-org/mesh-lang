---
id: M043
title: "Runtime-Native Cross-Cluster Disaster Continuity"
status: complete
completed_at: 2026-03-29T12:36:18.827Z
key_decisions:
  - Keep continuity authority, promotion, merge precedence, and stale-primary fencing runtime-owned in `mesh-rt`, with `cluster-proof` consuming only narrow runtime APIs and runtime-authored truth.
  - Use explicit startup role/epoch env only as topology input; all post-start operator-visible authority truth comes from runtime-backed continuity status.
  - Expose explicit failover through narrow `Continuity.promote()` and `Continuity.authority_status()` APIs instead of adding Mesh-side disaster-recovery orchestration.
  - Package the failover proof on one `cluster-proof` image with hostname-derived node identity and early entrypoint validation for contradictory continuity env.
  - Make destructive continuity verifiers replay prior authoritative rails, fail closed on named test counts, and validate copied JSON/log artifact bundles instead of trusting exit codes alone.
  - Keep Fly read-only and environment-dependent: the destructive failover authority remains the local same-image rail, while the Fly helper only inspects an already-deployed app.
key_files:
  - compiler/mesh-rt/src/dist/continuity.rs
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/mesh-rt/src/lib.rs
  - cluster-proof/config.mpl
  - cluster-proof/main.mpl
  - cluster-proof/work.mpl
  - cluster-proof/work_continuity.mpl
  - cluster-proof/docker-entrypoint.sh
  - cluster-proof/README.md
  - cluster-proof/fly.toml
  - compiler/meshc/tests/e2e_m043_s01.rs
  - compiler/meshc/tests/e2e_m043_s02.rs
  - compiler/meshc/tests/e2e_m043_s03.rs
  - scripts/lib/m043_cluster_proof.sh
  - scripts/verify-m043-s01.sh
  - scripts/verify-m043-s02.sh
  - scripts/verify-m043-s03.sh
  - scripts/verify-m043-s04-proof-surface.sh
  - scripts/verify-m043-s04-fly.sh
  - README.md
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/distributed/index.md
lessons_learned:
  - Cross-cluster continuity stayed honest only once authority metadata, promotion, and fencing lived in the runtime; pushing that logic into Mesh app code would have split the contract.
  - For packaged continuity proofs, selecting artifact bundles by manifest/required-file shape is more reliable than matching by loose directory prefixes because the same e2e target can emit both positive and negative-test bundles.
  - `cluster_role`/`promotion_epoch` startup env is configuration input, not durable authority truth; operator surfaces must read live runtime status after promotion or fencing occurs.
  - A read-only hosted verifier can fail closed on missing deployment input and still be the correct proof surface when the milestone scope is local destructive proof plus bounded public/operator documentation truth.
---

# M043: Runtime-Native Cross-Cluster Disaster Continuity

**M043 extended Mesh’s runtime-native continuity model across primary/standby clusters, proving mirrored standby truth, explicit promotion, recovery rollover, stale-primary fencing, a same-image operator rail, and aligned public proof surfaces.**

## What Happened

M043 took the single-cluster runtime continuity work from M042 and extended it across a bounded primary/standby disaster-recovery contract without moving failover logic into Mesh application code. S01 added runtime-owned authority metadata (`cluster_role`, `promotion_epoch`, `replication_health`) to continuity records, projected mirrored request truth into standby authority, and surfaced that truth on `cluster-proof`’s `/membership` and `/work/:request_key` surfaces. S02 made authority live and mutable inside `mesh-rt`, exposed narrow `Continuity.promote()` and `Continuity.authority_status()` APIs to Mesh code, and proved the real failover path: mirrored standby truth before loss, explicit promotion after primary loss, runtime-owned same-key attempt rollover on the promoted standby, successful completion there, and fenced/deposed old-primary rejoin. S03 packaged that same contract into a one-image two-cluster Docker/operator rail with hostname-derived identities, early continuity-env validation in the image entrypoint, retained inspect/log/JSON artifacts, and a fail-closed verifier that replays the prior rails before checking the packaged proof bundle. S04 aligned the public/operator contract to what actually shipped: README, distributed-proof docs, proof-surface verifier, and read-only Fly helper all describe the same bounded failover story—explicit `/promote`, runtime-owned authority truth, fenced stale-primary rejoin, same-image local authority proof, and non-destructive Fly scope. Verification passed because the milestone has real non-`.gsd` code changes in runtime, proof app, compiler e2es, shell verifiers, packaging, and docs, all four slices are complete with rendered summaries/UAT artifacts, the retained proof bundles for S01–S04 are present in `.tmp/m043-s0{1,2,3,4}/`, and the existing milestone validation found no cross-slice boundary drift.

## Success Criteria Results

- ✅ **Code-change verification passed.** `git diff --name-only $(git merge-base HEAD origin/main) HEAD -- ':!.gsd/'` shows non-`.gsd` milestone changes in `compiler/mesh-rt/src/dist/node.rs`, `compiler/mesh-rt/src/lib.rs`, `compiler/meshc/tests/e2e_m043_s01.rs`, `compiler/meshc/tests/e2e_m043_s02.rs`, `compiler/meshc/tests/e2e_m043_s03.rs`, `cluster-proof/README.md`, `cluster-proof/docker-entrypoint.sh`, `cluster-proof/fly.toml`, `scripts/verify-m043-s0{1,2,3,4}*.sh`, `scripts/lib/m043_cluster_proof.sh`, `README.md`, and `website/docs/docs/distributed{,-proof}/index.md`.
- ✅ **Runtime-owned primary→standby replication plus truthful role/epoch/health surfaces.** S01 summary/UAT and `.tmp/m043-s01/verify/phase-report.txt` show `runtime-continuity`, `cluster-proof-tests`, `build-cluster-proof`, `m043-e2e`, `malformed-contract`, and `primary-to-standby` all passed. The retained S01 artifact bundle contains `membership-primary.json` with `cluster_role=primary`, `promotion_epoch=0`, `replication_health=healthy`, plus standby-side pending/completed status files with mirrored request truth and explicit standby metadata.
- ✅ **Standby can be explicitly promoted after full primary loss and continue surviving keyed work.** `.tmp/m043-s02/verify/phase-report.txt` shows `runtime-continuity`, `cluster-proof-tests`, `build-cluster-proof`, `s01-contract`, `m042-rejoin`, `m043-api`, `failover-contract`, and `failover-artifacts` all passed. The retained failover bundle includes `promote-standby.json` with `promotion_epoch: 1` and `failover-completed-standby.json` showing the promoted standby as `owner_node`/`execution_node` for the surviving completion.
- ✅ **Returning old primary stays fenced/deposed and cannot silently resume authority.** S02 summary and `.tmp/m043-s02/verify/07-failover-artifacts/.../post-rejoin-primary-status.json` plus `stale-guard-primary.json` show the rejoined old primary reporting `cluster_role: standby`, `promotion_epoch: 1`, and the promoted standby as authoritative. The same fence markers are preserved on the packaged same-image rail under `.tmp/m043-s03/verify/05-same-image-artifacts/...`.
- ✅ **One-image operator rail, cluster-proof surface, and public docs/verifiers describe the same bounded failover contract.** `.tmp/m043-s03/verify/phase-report.txt` shows `same-image-contract`, `entrypoint-misconfig`, and `same-image-artifacts` all passed after replaying prior rails; retained S03 artifacts include `scenario-meta.json` and inspect/log bundles for the same image on both roles. `.tmp/m043-s04/proof-surface/phase-report.txt` is fully green, and `bash scripts/verify-m043-s04-fly.sh --help` exposes the same read-only Fly contract and non-goals. The local Fly bundle still fails closed on missing `CLUSTER_PROOF_FLY_APP`, which matches the milestone’s bounded read-only/environment-dependent scope rather than contradicting the shipped contract.

## Definition of Done Results

- ✅ **All roadmap slices complete.** The milestone directory contains `S01`–`S04`, and each slice is marked done in the roadmap with matching rendered slice summaries and UAT artifacts.
- ✅ **All slice summaries exist.** `find .gsd/milestones/M043/slices -maxdepth 2 -name 'S*-SUMMARY.md'` returns `S01-SUMMARY.md`, `S02-SUMMARY.md`, `S03-SUMMARY.md`, and `S04-SUMMARY.md`.
- ✅ **Task summaries exist for executed work.** `find .gsd/milestones/M043/slices -maxdepth 3 -name 'T*-SUMMARY.md'` returns all expected task summaries across S01–S04.
- ✅ **Cross-slice integration works correctly.** `M043-VALIDATION.md` records PASS for every slice delivery audit row and states no material S01→S02, S02→S03, or S03→S04 boundary mismatch; the operator-visible surfaces remain the same (`/membership`, `/work/:request_key`, explicit `/promote`) while authority/promotion/fencing stay runtime-owned.
- ✅ **Assembled proof surfaces present and current.** The retained verifier roots exist at `.tmp/m043-s01/verify/`, `.tmp/m043-s02/verify/`, `.tmp/m043-s03/verify/`, and `.tmp/m043-s04/{proof-surface,fly}/`, with green phase reports for S01–S03 plus S04 proof-surface and a bounded fail-closed Fly input-validation log.
- ℹ️ **Horizontal checklist.** No separate horizontal checklist was present in the milestone validation surface beyond the assembled proof/docs/operator rails above.

## Decision Re-evaluation

| Decision | Re-evaluation | Status | Next-milestone action |
|---|---|---|---|
| Keep continuity authority, promotion, merge precedence, and stale-primary fencing runtime-owned in `mesh-rt`. | Confirmed by S01–S03: the same runtime-owned truth surfaces power mirrored standby status, promotion, rollover, fenced rejoin, and the packaged same-image rail without Mesh-side failover orchestration drift. | Keep | Continue extending disaster continuity at the runtime seam rather than in `cluster-proof`. |
| Use startup role/epoch env only as topology input; derive live authority truth from runtime status after startup. | Confirmed by S02 and S03: promoted/fenced state changed after startup and remained truthful only because `/membership` and `/work/:request_key` read runtime authority, not boot env. | Keep | Preserve this split for any later quorum or hosted failover work. |
| Expose explicit failover through narrow `Continuity.promote()` / `Continuity.authority_status()` APIs. | Confirmed by the shipped operator contract: `/promote` stayed the one explicit authority boundary and the public/docs rail now describes that exact shape. | Keep | Reuse the same narrow API pattern if future failover controls are added. |
| Package the failover proof on one `cluster-proof` image with hostname-derived node identity and early entrypoint validation. | Confirmed by S03: the same image served both roles and the contradictory-env path now fails before ambiguous runtime startup. | Keep | Preserve the one-image/operator rail as the authoritative destructive local proof surface. |
| Make destructive verifiers replay prior rails, fail closed on named test counts, and validate copied artifact bundles instead of trusting exit codes alone. | Confirmed by S02–S04: the assembled milestone evidence depends on retained phase reports, manifests, JSON, and logs rather than green command exits alone. | Keep | Apply the same verifier pattern to future distributed/operator milestones. |
| Keep Fly read-only and environment-dependent, with destructive failover staying local. | Still valid for the shipped M043 scope. The help/proof-surface contract is green and the missing-app path fails closed, but there is still no live hosted destructive failover proof. | Keep, revisit only if hosted failover becomes scope | If a future milestone targets hosted failover evidence, add it as a new scope item rather than broadening M043 retroactively. |

## Requirement Outcomes

- **R051** — **Active → Validated.** S01 proved the prerequisite half of R051 by mirroring continuity truth from primary to standby and surfacing runtime-owned `cluster_role`, `promotion_epoch`, and `replication_health` through `/membership` and `/work/:request_key`. S02 proved the failover half with `bash scripts/verify-m043-s02.sh`, whose retained `.tmp/m043-s02/verify/07-failover-artifacts/` bundle shows explicit promotion to epoch 1, runtime-owned attempt rollover on the promoted standby, successful completion there, and fenced/deposed old-primary rejoin. S03 packaged the same contract into the same-image operator rail without changing the underlying requirement semantics.
- No other requirement status transition was established in the milestone closeout evidence.

## Deviations

No plan-invalidating deviation. The only closeout caveat was the expected Fly evidence boundary: local S04 validation could prove the help/input-validation contract and docs alignment, but without a deployed app the Fly bundle remained a fail-closed environment check rather than a live hosted-status sample.

## Follow-ups

Automatic/quorum-backed promotion, active-active intake, and destructive hosted-environment failover remain out of scope for M043. Future distributed work can build on the shipped runtime-owned authority model and same-image verifier without reopening the local/public contract proved here.
