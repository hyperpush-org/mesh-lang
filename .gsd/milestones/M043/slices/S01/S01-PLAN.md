# S01: Primary→Standby Runtime Replication and Role Truth

**Goal:** Ship the first usable cross-cluster continuity mode: runtime-owned live replication from a primary cluster to a standby cluster with explicit role, promotion-epoch, and replication-health truth exposed through the existing Mesh-facing continuity surface.
**Demo:** After this: Start a primary cluster and standby cluster with cluster-proof, submit keyed work on the primary, and observe standby-side continuity/status surfaces showing mirrored request truth plus explicit primary/standby role, promotion epoch, and replication health without app-authored DR logic.

## Tasks
- [x] **T01: Added runtime continuity authority metadata and standby-safe merge rules to mesh-rt.** — Close the runtime authority seam before touching cluster-proof. Extend `mesh-rt` continuity records and cross-node sync so mirrored records carry cluster role, promotion epoch, and replication-health truth, and ensure merge precedence prefers fresher authority metadata over stale primary/standby updates without claiming promotion yet. Add runtime continuity tests for primary→standby upsert/sync truth, stale-role merge rejection, and live mirrored standby snapshots.
  - Estimate: 2h
  - Files: compiler/mesh-rt/src/dist/continuity.rs, compiler/mesh-rt/src/dist/node.rs, compiler/mesh-rt/src/dist/discovery.rs
  - Verify: cargo test -p mesh-rt continuity -- --nocapture
- [x] **T02: Surfaced runtime-owned primary/standby role truth through cluster-proof membership and keyed status surfaces, with fail-closed topology validation and package coverage.** — Keep `cluster-proof` a thin consumer while extending the operator-visible surfaces. Add the minimal env/config needed to declare primary vs standby topology, thread runtime role/epoch/replication-health fields through `/membership` and keyed continuity status JSON, and update package tests so mirrored standby truth is observable without app-authored DR logic or bespoke peer lists.
  - Estimate: 2h
  - Files: cluster-proof/config.mpl, cluster-proof/main.mpl, cluster-proof/cluster.mpl, cluster-proof/work.mpl, cluster-proof/work_continuity.mpl, cluster-proof/tests/config.test.mpl, cluster-proof/tests/work.test.mpl
  - Verify: cargo run -q -p meshc -- test cluster-proof/tests && cargo run -q -p meshc -- build cluster-proof
- [x] **T03: Added the M043 primary→standby continuity proof harness, retained-artifact shell helpers, and a fail-closed local verifier.** — Prove the slice on a real multi-cluster path. Add an M043-specific e2e harness that boots primary and standby clusters, submits keyed work on the primary, waits for standby-side mirrored continuity/status truth, and archives raw JSON/log artifacts. Wrap it in a repo-root verifier that replays the runtime/package prerequisites, fails closed on zero-test filters or missing artifacts, and leaves `.tmp/m043-s01/verify/` debuggable.
  - Estimate: 2h
  - Files: compiler/meshc/tests/e2e_m043_s01.rs, scripts/lib/m043_cluster_proof.sh, scripts/verify-m043-s01.sh
  - Verify: cargo test -p meshc --test e2e_m043_s01 -- --nocapture && bash scripts/verify-m043-s01.sh
