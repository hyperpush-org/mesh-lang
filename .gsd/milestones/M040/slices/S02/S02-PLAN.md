# S02: Replica-Backed Admission, Fail-Closed Policy, and Owner-Loss Convergence

**Goal:** Make the existing `cluster-proof` rail prove honest two-node continuity: deterministic owner/replica placement, replica-backed durable admission, fail-closed rejection when replica safety disappears, and same-key convergence after owner loss without widening into a generic distributed-state platform.
**Demo:** After this: On a real two-node cluster, keyed work is accepted only after replica-backed durability is confirmed, status shows owner/replica truth, new durable work is rejected when replica safety disappears, and a request still converges after owner loss through surviving continuity and same-key retry/continuation.

## Tasks
- [x] **T01: Fixed the clustered startup shadowing crash and staged legacy /work compatibility, but final M039/S03 verification still needs rerun.** — Reproduce the exact `e2e_m039_s03_rejoins_and_routes_to_peer_again_without_manual_repair` failure, isolate the cluster-only startup path that now aborts before `work services ready`, and make the keyed `cluster-proof` app start reliably on two nodes again. Keep any `mesh-rt` fix narrow, safety-commented, and tied to the startup path rather than broad runtime refactors.
  - Estimate: 2h
  - Files: cluster-proof/main.mpl, cluster-proof/work.mpl, compiler/mesh-rt/src/string.rs, compiler/mesh-rt/src/dist/global.rs, compiler/meshc/tests/e2e_m039_s03.rs
  - Verify: cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_rejoins_and_routes_to_peer_again_without_manual_repair -- --nocapture
- [x] **T02: Partially wired canonical placement and durability-policy env handling for cluster-proof, but verification is still failing and needs a fresh follow-up unit.** — Replace the current membership-order routing rule with a canonical owner/replica placement rule that every node derives the same way, then expose the minimal durability policy on the existing config/bootstrap rail. Rewrite the tests that currently encode `Node.list()` order as truth, and keep policy/config fail-closed on `cluster-proof/config.mpl`, `docker-entrypoint.sh`, and `fly.toml` instead of creating a second deployment path.
  - Estimate: 2h
  - Files: cluster-proof/work.mpl, cluster-proof/cluster.mpl, cluster-proof/config.mpl, cluster-proof/tests/work.test.mpl, cluster-proof/tests/config.test.mpl, cluster-proof/docker-entrypoint.sh, cluster-proof/fly.toml
  - Verify: cargo run -q -p meshc -- test cluster-proof/tests && cargo run -q -p meshc -- build cluster-proof
- [x] **T03: Require replica-backed admission and mirrored continuity status** — Extend the app-owned continuity record so submit prepares mirrored state on both owner and replica, only returns success after replica acknowledgment, and answers keyed status from surviving continuity truth instead of ingress-local state only. Reuse the global registry for service discovery only; continuity state itself stays inside `cluster-proof/work.mpl`.
  - Estimate: 3h
  - Files: cluster-proof/main.mpl, cluster-proof/work.mpl, cluster-proof/tests/work.test.mpl
  - Verify: cargo run -q -p meshc -- test cluster-proof/tests && bash scripts/verify-m040-s01.sh
- [ ] **T04: Add monitor-driven owner-loss continuation and a Rust e2e proof for convergence** — Spawn a dedicated continuity monitor actor from `start_work_services()` that owns `Node.monitor(...)`, reacts to owner/replica loss, and only rolls the active attempt forward when surviving replicated state makes continuation truthful. Add `compiler/meshc/tests/e2e_m040_s02.rs` to prove owner loss, degraded new-submit rejection, and same-key convergence without duplicate completion leakage.
  - Estimate: 3h
  - Files: cluster-proof/main.mpl, cluster-proof/work.mpl, cluster-proof/tests/work.test.mpl, compiler/meshc/tests/e2e_m039_s03.rs, compiler/meshc/tests/e2e_m040_s02.rs
  - Verify: cargo test -p meshc --test e2e_m040_s02 -- --nocapture
- [ ] **T05: Ship the slice-local verifier and evidence bundle for replica-backed continuity** — Add a repo-root verifier that replays the keyed baseline, runs the new two-node replica-backed flow, and archives pre-loss, degraded, and post-owner-loss JSON plus per-node logs under `.tmp/m040-s02/verify/`. Keep the broader M039 operator/docs/Fly truth surfaces untouched except for additive helper reuse; this task’s job is to leave truthful new S02 evidence, not to perform S03’s migration.
  - Estimate: 2h
  - Files: compiler/meshc/tests/e2e_m040_s02.rs, scripts/lib/m039_cluster_proof.sh, scripts/lib/m040_cluster_proof.sh, scripts/verify-m040-s01.sh, scripts/verify-m040-s02.sh
  - Verify: bash scripts/verify-m040-s02.sh
