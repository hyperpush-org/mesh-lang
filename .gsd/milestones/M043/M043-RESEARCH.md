# M043 — Research

**Date:** 2026-03-28

## Summary

M043 should extend the existing **runtime-owned** continuity seam in `mesh-rt`, not add a second disaster-recovery system in Mesh application code. The codebase already has the right single-cluster substrate: `compiler/mesh-rt/src/dist/continuity.rs` owns keyed request truth, `compiler/mesh-rt/src/dist/node.rs` owns replica-prepare transport plus disconnect-triggered continuity state changes, and `cluster-proof/` is already mostly a thin consumer over `Continuity.submit`, `Continuity.status`, and `Continuity.mark_completed`. The main architectural gap is that all of this is still **single-cluster-shaped**: continuity merge precedence has no cluster/site or failover epoch, discovery is one DNS-seed full mesh, and disconnect handling is eager enough for node loss but too weak for cross-cluster promotion.

The first honest M043 shape is **active-primary / live-standby with explicit promotion and fencing**, not active-active and not automatic promotion by simple peer loss. The repo does not currently have a quorum-bearing control plane or external durable authority, and the current runtime would overclaim safety if standby promotion happened purely because one side stopped answering. The right first milestone is: add runtime-owned primary/standby role + failover epoch truth, replicate continuity records across clusters, prove full primary-cluster loss locally, prove stale-primary fencing on return, then update `cluster-proof`, packaged verifiers, and docs around that runtime contract.

## Recommendation

Treat M043 as a **runtime state/transport milestone first, proof-surface milestone second**.

Recommended first-wave design:

- keep the Mesh-facing API thin and continuity-centric
- preserve `request_key` and `attempt_id` semantics from M042
- add a **runtime-owned failover epoch / promotion fence** instead of replacing `attempt_id`
- start with **operator-triggered promotion** or a tightly bounded promotion action, not open-ended automatic failover
- keep `cluster-proof` as the proof app and operator surface, but move any new DR decision logic into `mesh-rt`

Why this is the safest honest path:

1. `attempt_id` already behaves like a per-request fence token; M043 needs the same idea at the **cluster authority** level.
2. Current disconnect semantics (`owner_lost`, `degraded_continuing`) are acceptable inside one cluster but too eager for WAN/cross-cluster loss.
3. The repo already has a strong verification pattern: runtime/unit tests -> local destructive verifier -> packaged Docker rail -> read-only Fly rail -> proof-surface docs gate. Reuse that instead of inventing a new deployment story.
4. If M043 grows Mesh-side actors, polling loops, or `Node.spawn` orchestration as the real DR brain, it breaks the runtime-native direction set by M042 and runs into known transport/codegen limitations.

## Implementation Landscape

### Key Files

- `compiler/mesh-rt/src/dist/continuity.rs` — current continuity truth owner. Holds `ContinuityRecord`, submit/completion/rejection transitions, merge precedence, snapshot/upsert sync, and `attempt_id` issuance. **First file to change** for cross-cluster role/epoch/failover truth.
- `compiler/mesh-rt/src/dist/node.rs` — continuity wire tags (`DIST_CONTINUITY_*`), targeted replica prepare/ack RPC, connect-time snapshot sync, and disconnect hooks that currently mark `owner_lost` / `degraded_continuing`. Likely home for new cross-cluster replication/promotion transport.
- `compiler/mesh-rt/src/dist/discovery.rs` — current one-seed DNS reconcile loop. Good for intra-cluster discovery; not sufficient as cross-cluster failover authority.
- `compiler/mesh-typeck/src/infer.rs` — hard-coded `Continuity` module signatures exposed to Mesh.
- `compiler/mesh-codegen/src/mir/lower.rs` — lowers `Continuity.*` calls to runtime intrinsics.
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — LLVM declarations for continuity intrinsics.
- `cluster-proof/work.mpl` — current Mesh-side request/status structs and deterministic target-selection wrapper around local membership.
- `cluster-proof/cluster.mpl` — canonical membership normalization and deterministic owner/replica placement. Reuse the semantics if placement logic moves into runtime; do not invent new hash rules casually.
- `cluster-proof/work_continuity.mpl` — thin consumer over `Continuity.*`, but still contains a policy leak in `submit_required_replica_count(...)` based on `owner_lost`. This should shrink, not grow, in M043.
- `cluster-proof/config.mpl` — current small-env operator contract (`local-only` vs `replica-backed`, explicit vs Fly identity). Likely place for any primary/standby env additions, but keep them minimal.
- `cluster-proof/main.mpl` — mounts `/membership`, legacy `GET /work`, keyed `POST /work`, and `GET /work/:request_key`.
- `cluster-proof/docker-entrypoint.sh` — fail-closed bootstrap for the one-image operator rail. Important for R052 if new role/failover env is introduced.
- `cluster-proof/fly.toml` — current Fly contract. M043 can extend it, but Fly remains a proof environment, not the architecture definition.
- `scripts/lib/m042_cluster_proof.sh` — keyed payload/status assertion helpers. Best starting point for M043 proof helpers.
- `scripts/verify-m042-s03.sh` — current destructive local continuity authority. M043 should layer on top of this pattern.
- `scripts/verify-m042-s04.sh` — packaged local Docker/operator proof rail. Natural parent for a packaged M043 failover verifier.
- `scripts/verify-m042-s04-fly.sh` — read-only live Fly sanity rail. Keep read-only unless the external-state policy changes explicitly.
- `scripts/verify-m042-s04-proof-surface.sh` — docs/public-truth gate. Any status/contract change in M043 must update this rail in the same slice.
- `website/docs/docs/distributed-proof/index.md` — public proof map for runtime-owned continuity.
- `cluster-proof/README.md` — deepest operator runbook and canonical packaging commands.

### Build Order

1. **Prove the runtime record/fence model first.**
   - Extend `ContinuityRecord` and merge precedence with whatever site/role/epoch information is necessary to keep a promoted standby authoritative over a stale primary.
   - The first proof is not “failover worked”; it is **“a stale primary cannot win after promotion.”**

2. **Prove cross-cluster replication without promotion.**
   - Add a runtime-owned replication path from primary cluster to standby cluster.
   - Verify that standby truth is live and observable before any destructive failover claim is made.

3. **Prove promotion under full primary loss.**
   - Use a local multi-cluster harness to kill the active primary cluster and promote standby.
   - Verify that existing mirrored keyed work survives honestly and new status reflects promoted authority.

4. **Prove old-primary rejoin is fenced.**
   - Reintroduce the old primary and verify it cannot overwrite promoted standby truth.
   - This is the key split-brain guardrail and should be explicit in the plan.

5. **Only then update `cluster-proof`, packaged operator rail, Fly sanity rail, and docs.**
   - Keep the public story downstream of the runtime contract, not vice versa.

### Verification Approach

Preserve the existing proof layering and extend it rather than replacing it:

- **Runtime/unit authority**
  - continue to use `cargo test -p mesh-rt continuity -- --nocapture` as the base seam
  - add dedicated M043 runtime tests for role/epoch merge precedence, primary->standby replication, promotion, and stale-primary fencing

- **Compiler / consumer smoke**
  - `cargo run -q -p meshc -- test cluster-proof/tests`
  - `cargo run -q -p meshc -- build cluster-proof`

- **Local destructive authority**
  - new `scripts/verify-m043-s0x.sh` wrappers should follow the M042 style: explicit phase reports, fail-closed test-count checks, and copied artifact bundles
  - local proof should remain the destructive authority for full primary-loss failover and promotion behavior

- **Packaged/operator rail**
  - extend the one-image Docker pattern from `scripts/verify-m042-s04.sh`
  - likely requires a new topology: two clusters rather than two peers in one cluster
  - preserve retained artifacts for pre-failover replication, promotion, post-failover status, and old-primary rejoin

- **Fly rail**
  - keep `scripts/verify-m042-s04-fly.sh`’s read-only philosophy unless explicitly changed
  - useful for deployed role/config/status truth, not for destructive failover authority

- **Docs/public truth**
  - extend `scripts/verify-m042-s04-proof-surface.sh` or add an M043 equivalent so status wording, commands, and non-goals stay mechanically aligned

## Don't Hand-Roll

| Problem | Existing Solution | Why Use It |
|---------|------------------|------------|
| Keyed request identity and retry fencing | `request_key` + runtime-issued `attempt_id` in `compiler/mesh-rt/src/dist/continuity.rs` | This is already the public continuity contract. M043 should extend it, not replace it. |
| Healthy-path continuity sync | `DIST_CONTINUITY_UPSERT`, `DIST_CONTINUITY_SYNC`, snapshot/upsert helpers in `compiler/mesh-rt/src/dist/continuity.rs` and `node.rs` | Cross-cluster replication should build on the existing runtime-owned record transport, not reintroduce Mesh-side RPC/state machines. |
| Deterministic placement semantics | `cluster-proof/cluster.mpl` (`canonical_membership`, `canonical_placement`) | If placement moves into runtime, preserve these semantics. A new hashing scheme would create avoidable drift in the public proof surface. |
| Keyed proof assertions | `scripts/lib/m042_cluster_proof.sh` | Reusing the keyed payload/status assertion helpers will keep M043’s proof rail consistent with M042. |
| Operator/docs truth rail | `scripts/verify-m042-s04.sh`, `scripts/verify-m042-s04-fly.sh`, `scripts/verify-m042-s04-proof-surface.sh` | R052/R053 are already enforced through this layered rail. Extending it is safer than inventing a new public contract gate. |

## Constraints

- **Current continuity merge logic is single-cluster-only.** `preferred_record(...)` in `compiler/mesh-rt/src/dist/continuity.rs` ranks by attempt token, terminality, and replica-status rank only. That is not enough to distinguish a promoted standby from a stale returning primary.
- **Current disconnect semantics are node-loss-driven.** `compiler/mesh-rt/src/dist/node.rs` marks `owner_lost` and `degraded_continuing` on disconnect. That is too eager to be the whole cross-cluster failover contract.
- **Discovery is one-seed, full-mesh flavored.** `compiler/mesh-rt/src/dist/discovery.rs` reconciles one DNS seed into candidate peers. It has no notion of site role, standby cluster, or promotion authority.
- **R052 remains active.** The one-image, small-env operator path is still a milestone constraint. New configuration should stay narrow and operator-legible.
- **R053 is validated but still constraining.** The docs/proof surface already fail-closes on wording and command drift. M043 cannot change the contract piecemeal.
- **Application-side DR control planes are a bad fit for current transport limits.** Per project knowledge, cross-node actor args and mailbox payloads still have important limits; building the real DR brain in Mesh actors would be both directionally wrong and mechanically fragile.
- **Fly is evidence, not authority.** The architecture definition should stay runtime-native and local-proof-first; Fly remains one proof environment.

## Common Pitfalls

- **Implicit promotion from simple disconnects** — current `owner_lost` transitions are honest for single-cluster owner loss but not sufficient for cross-cluster standby promotion. Without an explicit failover epoch or similar runtime fence, a transient partition can become split-brain.
- **Letting the old primary win on return** — `attempt_id` only fences request attempts. M043 needs a cluster-authority fence so old-primary updates and completions cannot override a promoted standby.
- **Growing app-side DR logic again** — `cluster-proof/work_continuity.mpl` already leaks one policy decision through `submit_required_replica_count(...)`. M043 should move that sort of decision downward, not multiply it.
- **Abusing discovery as a control plane** — `MESH_DISCOVERY_SEED` is a peer-finding mechanism. Treating it as the sole source of primary/standby truth would blur membership discovery with failover authority.
- **Breaking the proof rail by partial updates** — keyed JSON assertions, proof-surface docs, and packaged verifiers are intentionally strict. Update runtime, consumer surface, verifier, and docs together.

## Open Risks

- **Promotion authority is still an open design question.** The safest first recommendation is explicit operator-triggered promotion with runtime-enforced fencing. Automatic promotion should remain out of scope until there is a stronger safety story.
- **Primary rejoin semantics are not defined yet.** M043 needs an explicit story for the old primary returning after promotion: follower/standby rejoin, explicit demotion, or isolation until operator action.
- **The current `Continuity` API may be too narrow for operator-visible failover truth.** Submit/status/mark_completed/acknowledge_replica are enough for M042; M043 may need a separate status or promotion surface. This is a candidate requirement, not an assumption.
- **The local packaged harness will get more complex.** The current Docker proof rail is two nodes in one cluster. M043 likely needs a two-cluster topology and clearer retained artifacts to stay debuggable.

## Requirements and Scope Observations

- **R051 is the table-stakes requirement.** The minimum honest bar is live primary->standby replication plus truthful failover when replicas still exist on standby.
- **R052 is still active and should remain narrow.** The operator story should stay “same image + small env surface,” not grow into bespoke orchestration scripts.
- **R053 is already validated, but it remains a guardrail.** M043 will break truth if it adds role/epoch/failover semantics without updating the proof/docs/verifier rail.

### Candidate Requirements (advisory, not auto-binding)

- **Candidate requirement:** explicit failover-authority contract.
  - The repo should state whether promotion is operator-triggered, bounded-automatic, or some explicit hybrid. This must be observable in status and docs, not implied.

- **Candidate requirement:** stale-primary fencing after promotion.
  - Once standby is promoted, old-primary writes/completions/replication must be rejected or ignored by a runtime fence.

- **Candidate requirement:** post-failover status truth.
  - The operator-facing proof surface should expose enough role/epoch/promotion truth to distinguish mirrored standby, promoted primary, and deposed old primary.

- **Candidate requirement:** primary rejoin contract.
  - A returning old primary must not silently resume authority. The runtime should define whether it rejoins as follower/standby or stays isolated until operator action.

### Probably Out of Scope for the First Wave

- active-active multi-cluster intake
- arbitrary application-state replication beyond runtime-owned continuity records
- automatic promotion based solely on peer disappearance with no stronger fence/authority model
- making live Fly destructive failover proof a milestone blocker

## Skills Discovered

| Technology | Skill | Status |
|------------|-------|--------|
| Rust runtime / compiler surfaces | `rust-best-practices` | available |
| Docker packaging / one-image operator rail | `multi-stage-dockerfile` | available |
| Fly.io operator workflow | `flyio-cli-public` | available |
| Distributed failover / fencing analysis | `wondelai/skills@ddia-systems` (installed locally as `distributed-systems`) | installed |
