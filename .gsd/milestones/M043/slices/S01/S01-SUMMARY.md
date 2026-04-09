---
id: S01
parent: M043
milestone: M043
provides:
  - Runtime-owned primary/standby authority metadata on continuity records and merge precedence.
  - Operator-visible role, promotion-epoch, and replication-health truth on `cluster-proof` membership and keyed continuity status surfaces.
  - A fail-closed local M043 proof rail with retained malformed-contract and primary→standby artifact bundles under `.tmp/m043-s01/verify/`.
requires:
  []
affects:
  - S02
  - S03
  - S04
key_files:
  - compiler/mesh-rt/src/dist/continuity.rs
  - compiler/mesh-rt/src/dist/node.rs
  - cluster-proof/config.mpl
  - cluster-proof/main.mpl
  - cluster-proof/work.mpl
  - cluster-proof/work_continuity.mpl
  - compiler/meshc/tests/e2e_m043_s01.rs
  - scripts/lib/m043_cluster_proof.sh
  - scripts/verify-m043-s01.sh
  - .gsd/PROJECT.md
key_decisions:
  - D172: project replicated continuity records into the local cluster role/epoch on merge and degrades standby replication health on upstream loss instead of reusing owner-loss recovery before promotion exists.
  - Keep `replica_status` precedence ahead of `replication_health` in runtime merge preference so stale mirrored health updates cannot overwrite real owner-loss or degraded-continuing truth.
  - D174: require explicit `MESH_CONTINUITY_ROLE` and `MESH_CONTINUITY_PROMOTION_EPOCH` in cluster mode, validated in `cluster-proof/config.mpl`, instead of inheriting the runtime's permissive primary default.
  - D173: prove S01 on one connected mesh node network with explicit role/epoch truth per node and retained JSON/log artifacts instead of inventing a second replication transport.
patterns_established:
  - Push authority metadata through the runtime-owned continuity record and transport seam first, then project it outward on proof-app HTTP surfaces; do not recompute role or epoch in Mesh code.
  - Before promotion exists, standby-side upstream loss must degrade `replication_health` while preserving mirrored authority truth rather than reusing the single-cluster `owner_lost` recovery path.
  - Destructive continuity verifiers must fail closed on both non-zero named test counts and retained raw HTTP/log artifacts so future drift cannot hide behind passing exit codes.
observability_surfaces:
  - `/membership` now exposes `cluster_role`, `promotion_epoch`, and `replication_health` for each node's local authority view.
  - `/work/:request_key` status payloads now expose `cluster_role`, `promotion_epoch`, `replication_health`, `replica_status`, and the mirrored owner/replica nodes on both primary and standby.
  - `mesh-rt` continuity transition logs now include authority metadata on submit, replica-ack, degraded, owner-loss, and replication-degraded paths.
  - `.tmp/m043-s01/verify/` retains raw HTTP bodies, parsed JSON, phase-report logs, and per-node stdout/stderr bundles for both the malformed-authority and primary→standby scenarios.
drill_down_paths:
  - .gsd/milestones/M043/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M043/slices/S01/tasks/T02-SUMMARY.md
  - .gsd/milestones/M043/slices/S01/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-29T07:25:24.332Z
blocker_discovered: false
---

# S01: Primary→Standby Runtime Replication and Role Truth

**Mesh now mirrors runtime-owned keyed continuity truth from a primary node to a standby node and exposes explicit role, promotion epoch, and replication health on membership and keyed status surfaces without app-authored disaster-recovery logic.**

## What Happened

S01 closed the first cross-cluster disaster-continuity seam at the runtime layer instead of growing app-authored DR logic in `cluster-proof`. T01 extended `mesh-rt` continuity records with runtime-owned `cluster_role`, `promotion_epoch`, and `replication_health`, projected mirrored records into the local standby authority during merge, and made standby disconnect handling degrade replication health rather than incorrectly entering the single-cluster `owner_lost` recovery path. T02 kept `cluster-proof` thin but surfaced that new truth to operators: cluster mode now requires explicit primary/standby topology env, `/membership` exposes role/epoch/health, and keyed continuity payloads expose the mirrored standby view without any Mesh-side promotion logic. T03 then added the destructive M043 rail — `e2e_m043_s01` plus `scripts/verify-m043-s01.sh` — which boots primary and standby nodes, submits keyed work on the primary, waits for mirrored truth on the standby, and archives raw JSON plus per-node logs under `.tmp/m043-s01/verify/` so failures stay debuggable.

The assembled result is the first usable primary→standby continuity story in this repo: the runtime now owns the authority metadata, the proof app shows the same truth on existing operator-visible surfaces, and the verifier confirms both the positive mirrored-standby path and the fail-closed missing-authority path. The retained artifacts demonstrate the exact operator contract: `membership-primary.json` shows `cluster_role=primary`, `promotion_epoch=0`, and `replication_health=healthy`; `pending-standby.json` and `completed-standby.json` show the same request mirrored on the standby with `cluster_role=standby`, `replica_status=mirrored`, `promotion_epoch=0`, and `execution_node` still on the primary. That gives S02 a concrete promoted-authority substrate without overclaiming that standby failover already exists.

## Verification

Replayed the full slice acceptance rail in order and it stayed green:

- `cargo test -p mesh-rt continuity -- --nocapture` passed all 28 continuity tests, including the new standby projection, replication-health degradation, and merge-precedence cases.
- `cargo run -q -p meshc -- test cluster-proof/tests && cargo run -q -p meshc -- build cluster-proof` passed the new topology-validation and mirrored-standby payload tests and rebuilt the proof app.
- `cargo test -p meshc --test e2e_m043_s01 -- --nocapture` passed both the malformed-authority negative case and the live primary→standby mirrored-truth scenario.
- `bash scripts/verify-m043-s01.sh` replayed the whole rail again, checked the copied malformed-contract and primary-to-standby bundles, and wrote `.tmp/m043-s01/verify/phase-report.txt` with every phase marked `passed`.

I also spot-checked the retained evidence: `membership-primary.json` shows `cluster_role=primary`, `promotion_epoch=0`, and `replication_health=healthy`; `pending-standby.json` and `completed-standby.json` show `cluster_role=standby`, `replica_status=mirrored`, `promotion_epoch=0`, and `execution_node` still on the primary; and the malformed bundle omits authority fields exactly so the negative test can fail closed.

## Requirements Advanced

- R051 — S01 proves the prerequisite half of R051: continuity state now mirrors live from the primary to a standby authority view, and operators can observe that mirrored request truth plus role/epoch/health directly through `/membership` and `/work/:request_key` before any promotion occurs.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

No plan-invalidating deviation. Two implementation choices mattered: the local proof stayed on one connected mesh node network with explicit primary/standby role metadata instead of inventing a second replication transport, and `cluster-proof/config.mpl` kept promotion-epoch parsing on an error-string + plain getter seam after a `Result<Int, String>` helper reproduced the known boxed-primitive crash. The shipped contract stayed the same.

## Known Limitations

This slice does not ship promotion, failover execution on standby, or stale-primary fencing. The standby node remains explicitly non-authoritative: it mirrors pending/completed request truth with `cluster_role=standby`, `promotion_epoch=0`, and `replica_status=mirrored`, but it does not execute the work or roll recovery locally. The local proof seam is also the current runtime transport truth — one connected mesh network with explicit roles — not yet the packaged two-cluster same-image operator rail or live Fly evidence.

## Follow-ups

S02 should add the explicit promotion action, move surviving mirrored requests from standby truth to promoted authority without inventing implicit failover, and fence stale primary completions or syncs on rejoin. S03 should package the same proof on the operator rail without weakening the retained-artifact contract. S04 should only update docs/help/Fly truth after the S02/S03 behavior is mechanically proven.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/continuity.rs` — Extended runtime-native continuity records, merge precedence, wire codecs, disconnect handling, and unit coverage for cluster role, promotion epoch, and replication health.
- `compiler/mesh-rt/src/dist/node.rs` — Threaded authority metadata and standby-safe replication/degradation behavior through the node transport layer.
- `cluster-proof/config.mpl` — Added fail-closed primary/standby topology parsing and explicit role/epoch requirements for cluster mode.
- `cluster-proof/main.mpl` — Surfaced runtime-owned role, promotion epoch, and replication health on the `/membership` HTTP payload.
- `cluster-proof/work.mpl` — Threaded authority metadata through keyed submit/status payloads while keeping cluster-proof a thin Continuity consumer.
- `cluster-proof/work_continuity.mpl` — Mapped runtime continuity records into operator-visible keyed status JSON, including mirrored standby truth and malformed-payload rejection.
- `cluster-proof/tests/config.test.mpl` — Added package coverage for topology validation, mirrored standby status truth, and malformed-authority failure closure.
- `compiler/meshc/tests/e2e_m043_s01.rs` — Added the destructive primary→standby e2e harness, including malformed-authority negative coverage and retained artifacts.
- `scripts/lib/m043_cluster_proof.sh` — Added reusable M043 artifact/assertion helpers and the fail-closed local verifier that checks retained JSON and per-node logs.
- `.gsd/PROJECT.md` — Refreshed current project state to reflect that M043/S01 is complete and that promotion/fencing remain the next disaster-continuity seam.
