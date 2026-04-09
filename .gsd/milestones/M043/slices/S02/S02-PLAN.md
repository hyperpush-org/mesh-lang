# S02: Standby Promotion and Stale-Primary Fencing

**Goal:** Deliver the actual disaster-continuity feature promised by M043: preserve mirrored continuity truth through an explicit standby promotion, complete surviving keyed work from the promoted authority, and fence the old primary on rejoin so higher-epoch standby truth remains authoritative.
**Demo:** After this: After mirrored standby state is live, kill the primary cluster, perform the explicit promotion action, complete surviving keyed work through the promoted standby, then bring the old primary back and observe that it stays fenced/deposed instead of resuming authority.

## Tasks
- [x] **T01: Moved continuity authority into the runtime registry and fenced stale lower-epoch primaries before projection.** — Close the highest-risk seam first inside `mesh-rt`. Replace the process-static authority model with mutable runtime state that can survive explicit promotion without discarding the mirrored in-memory registry, then rework merge and rejoin precedence so higher-epoch truth deposes stale primaries instead of projecting incoming records into the local role before comparison.

This task should keep the failover model runtime-owned. Reuse the existing owner-loss retry-rollover path for promoted pending records where possible, but do not invent a second Mesh-side failover state machine in `cluster-proof`.
  - Estimate: 3h
  - Files: compiler/mesh-rt/src/dist/continuity.rs, compiler/mesh-rt/src/dist/node.rs
  - Verify: cargo test -p mesh-rt continuity -- --nocapture
- [x] **T02: Added runtime-backed Continuity promotion and authority-status APIs with compiler e2e coverage.** — Add the narrow Mesh-visible continuity surface that S02 needs: an explicit promote action and a read-only authority-status call backed directly by `mesh-rt`. The goal is to let Mesh code trigger operator-approved promotion and read current authority truth without re-deriving role/epoch/health from env.

Keep the API boring and runtime-owned. The compiler/typechecker/codegen seam should only forward the minimal new intrinsics needed by `cluster-proof`, and the task should add focused compiler-facing proof coverage so future slices can reuse the same surface.
  - Estimate: 2h
  - Files: compiler/mesh-rt/src/dist/continuity.rs, compiler/mesh-rt/src/lib.rs, compiler/mesh-typeck/src/infer.rs, compiler/mesh-codegen/src/mir/lower.rs, compiler/mesh-codegen/src/codegen/intrinsics.rs, compiler/meshc/tests/e2e_m043_s02.rs
  - Verify: cargo test -p meshc --test e2e_m043_s02 continuity_api_ -- --nocapture
- [x] **T03: Switched cluster-proof to runtime-backed authority status and added the explicit /promote operator route.** — Keep `cluster-proof` a thin consumer while making the new failover boundary visible. Replace the startup-env-derived “current” role/epoch/health helpers with runtime-backed authority reads, add the explicit promotion operator surface, and ensure keyed status/error payloads reflect post-promotion truth from `mesh-rt` instead of stale config.

This task should not grow a Mesh-side disaster-recovery control plane. Config remains a startup topology contract only; live authority truth comes from the runtime API added in T02.
  - Estimate: 2h
  - Files: cluster-proof/main.mpl, cluster-proof/config.mpl, cluster-proof/work.mpl, cluster-proof/work_continuity.mpl, cluster-proof/tests/config.test.mpl, cluster-proof/tests/work.test.mpl
  - Verify: cargo run -q -p meshc -- test cluster-proof/tests && cargo run -q -p meshc -- build cluster-proof
- [x] **T04: Extended `e2e_m043_s02.rs` into a destructive failover harness that reaches standby promotion, surviving-work recovery, and fenced rejoin artifacts, but the shell verifier work remains unfinished.** — Close the slice with real failover evidence. Extend the M043 harness and verifier helpers so the repo proves the full sequence: mirrored standby state exists, the primary dies, the standby is explicitly promoted, surviving keyed work completes through the promoted authority, and the restarted old primary stays fenced/deposed at the newer epoch.

The verifier must fail closed on named test counts, copied-artifact manifests, and stale proof drift. It should replay the S01 verifier and the targeted M042 rejoin regression before asserting the new S02 contract.
  - Estimate: 3h
  - Files: compiler/meshc/tests/e2e_m043_s02.rs, scripts/lib/m043_cluster_proof.sh, scripts/verify-m043-s02.sh
  - Verify: bash scripts/verify-m043-s02.sh
