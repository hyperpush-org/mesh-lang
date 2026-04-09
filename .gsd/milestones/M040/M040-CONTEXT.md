# M040: Replicated Continuity & In-Cluster Request Durability

**Gathered:** 2026-03-28
**Status:** Ready for planning

## Project Description

This milestone extends the already-real `cluster-proof` rail from M039. Today, `cluster-proof` already proves automatic cluster formation, truthful `/membership`, remote `/work` routing, safe degrade after node loss, same-identity rejoin, one-image Docker/Fly operation, and docs truth around that proof surface.

M040 pushes that proof from clustering and balancing into replicated continuity. The target is a request-keyed proof path where a caller submits keyed work, can inspect request status plus owner/replica visibility, and can prove that keyed work survives individual node loss through cluster-internal replicated continuity.

The semantic target for the first honest durability wave is intentionally narrow and explicit:

- at-least-once execution
- idempotent completion
- safe retry by request key
- automatic continuation when possible
- continuity replicated across live nodes in the cluster
- primary + replica as the first ownership model
- two-node safety as the default proof bar, with replica count configurable upward later
- no external durable store or orchestrator as the real source of truth
- no exactly-once claim
- no deep checkpoint-replay requirement in the first wave; "resume" means restart by key

The proof surface should move from today's unkeyed read-only `/work` probe toward a keyed **submit + status** contract while staying on the same `cluster-proof` rail.

## Why This Milestone

The user does not want Mesh's distributed story to stop at auto-discovery, membership, and internal balancing. M039 proved the cluster can form, route work internally, degrade safely, and rejoin cleanly. The next credibility bar is higher: a keyed unit of work should survive individual node loss without outsourcing truth to a database or external coordinator.

This needs to exist now because the repo should not drift into broader disaster-recovery claims or distributed-runtime rhetoric while the first honest in-cluster durability wave is still only implied. M040 is the milestone that turns "the cluster can route and recover" into "the cluster can preserve continuity for keyed work in a way that survives node loss and remains retry-safe."

## User-Visible Outcome

### When this milestone is complete, the user can:

- submit keyed work to `cluster-proof`, inspect request status plus owner/replica visibility, kill the active node, and still see the request converge correctly without an external durable store becoming the real authority
- retry the same request key against a surviving node after failure and see idempotent completion converge without duplicate completion leaking through retries

### Entry point / environment

- Entry point: the existing `cluster-proof` proof app, extended with a keyed submit + status contract and verifier commands
- Environment: local Docker and Fly, on the same operator path M039 already established
- Live dependencies involved: DNS-based discovery, TLS/cookie-authenticated Mesh node transport, cluster membership, Docker, Fly; explicitly not an external durable database or orchestrator

## Completion Class

- Contract complete means: tests, shell verifiers, and preserved artifacts prove keyed submit + status behavior, replicated ownership/completion truth, primary-to-replica failover behavior, automatic continuation when possible, safe same-key retry convergence, and visible rejection of new durable work when the configured replica safety is missing
- Integration complete means: the same `cluster-proof` image/operator path works across real clustered nodes, continuity state is replicated inside the live cluster, primary loss is handled honestly through the primary + replica model, and the docs/proof surface is updated when the contract changes
- Operational complete means: node join, node loss, same-identity rejoin, replica-loss rejection, and continuity recovery are exercised under real local-Docker and Fly lifecycle conditions rather than only in unit tests or mocked harnesses

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- a keyed request submitted into `cluster-proof` shows owner/replica status, loses its active node, and still converges correctly through cluster-internal continuity without an external durable store becoming the real authority
- a keyed request can be retried against a surviving node after failure and converges idempotently without duplicate completion leaking through retries, while automatic continuation also happens when the surviving continuity state makes that possible
- if the replica path required by the durability policy is lost, the system visibly stops accepting new durable work instead of pretending continuity still exists; this must be proven on the same Docker + Fly operator path, not only through local mocks or isolated unit tests

## Risks and Unknowns

- The continuity design can sprawl into a fake distributed database or consensus platform if it stops being tightly scoped to keyed request durability — that would solve the wrong problem and distort the milestone
- Partition behavior is a truth risk — if the system keeps claiming durable continuity after it loses the replica path that makes the claim real, the milestone becomes dishonest
- M039 already exposed a runtime boundary where rich cross-node payload transport is shaky through restart/rejoin — if M040 assumes deep distributed payload movement too early, the proof could depend on a seam that is not yet trustworthy
- "Automatic continuation when possible" is stronger than retry-only semantics, so planning has to be explicit about what replicas actually store and when failover is allowed to continue versus restart by key
- The docs/proof surface can drift if the runbook, verifier chain, and public distributed-proof page are not updated as the continuity contract evolves

## Existing Codebase / Prior Art

- `cluster-proof/main.mpl` — current proof-app entrypoint with `/membership` and `/work`, plus the M039 operator-mode startup contract
- `cluster-proof/work.mpl` — current routing proof surface, ingress-owned request correlation, and the narrow place M040 should evolve rather than creating a second proof app
- `compiler/meshc/tests/e2e_m039_s03.rs` — existing local degrade/rejoin continuity harness that already preserves the pre-loss/degraded/post-rejoin evidence pattern M040 should build on
- `scripts/verify-m039-s04.sh` — authoritative local one-image Docker proof wrapper that M040 should extend instead of replacing
- `scripts/verify-m039-s04-fly.sh` — existing read-only Fly verifier contract; M040 should keep the same real-environment proof bar rather than falling back to local-only proof
- `compiler/mesh-rt/src/dist/node.rs` — node identity, discovery bootstrap, session lifecycle, remote spawn, and the distributed runtime seams that continuity work will stress
- `compiler/mesh-rt/src/dist/global.rs` — example of cluster-internal replicated state and cleanup behavior, useful prior art for how live-node replication currently behaves
- `website/docs/docs/distributed-proof/index.md` — public proof page that must stay synchronized as the operator-visible continuity contract changes
- `.gsd/KNOWLEDGE.md` — records the M039 truth that cross-node rich payload transport through restart/rejoin is still not a seam to trust casually

> See `.gsd/DECISIONS.md` for all architectural and pattern decisions — it is an append-only register; read it during planning, append to it during execution.

## Relevant Requirements

- R049 — advances the core request-keyed continuity contract so retries are safe and visible completion converges correctly without an external durable store
- R050 — advances replica-backed continuity by proving that in-flight ownership/progress truth survives individual node loss with two-node safety as the default proof bar and configurable replica count upward later
- R052 — preserves the one-image, small env-driven operator surface instead of introducing a bespoke durability-only deployment path
- R053 — requires the public distributed/docs proof surface to stay honest as continuity claims change; docs/docs site updates are part of truth maintenance, not optional cleanup

## Scope

### In Scope

- extending the existing `cluster-proof` rail rather than inventing a second proof surface
- introducing a keyed submit + status contract for the proof app
- primary + replica as the first ownership/replication model
- replicated continuity state for active keyed work inside the live cluster
- automatic continuation when possible
- safe same-key retry after failure with idempotent completion
- status/owner/replica visibility in the proof surface so failures are mechanically obvious
- rejecting new durable work when the configured replica safety is not available
- proving the continuity story on the same Docker + Fly operator path M039 established
- updating the docs/proof surface whenever the operator-visible continuity contract changes

### Out of Scope / Non-Goals

- exactly-once semantics
- using a database or external orchestrator as the real durability authority
- turning M040 into a generic consensus or arbitrary app-state replication platform
- deep checkpoint replay or rich mid-flight workflow resumability as the first-wave requirement; the first honest resume model is restart-by-key
- cross-cluster replication or standby-cluster disaster continuity; that belongs to M041
- inventing a new showcase app or broader product surface outside `cluster-proof`

## Technical Constraints

- M040 must stay on the existing `cluster-proof` operator and verifier rail instead of fragmenting proof into separate packages
- The first ownership model should be primary + replica, with two-node safety as the default proof bar and configurable replica count as a future-safe seam
- The first honest resume model is restart-by-key, not general checkpoint replay
- The system must reject new durable work when the replica path required by policy is unhealthy; it must not keep making fake durability claims for availability theater
- The implementation has to remain disciplined about what crosses the distributed seam because M039 already showed that rich cross-node payload transport through restart/rejoin is not yet a trustworthy foundation
- The same Docker + Fly operator path from M039 remains the real environment bar for completion
- Public docs, runbooks, and the distributed proof page need to be updated when the continuity contract changes so the docs/docs site stays truthful

## Integration Points

- `cluster-proof` HTTP contract — evolves from the current `/work` probe into keyed submit + status while preserving a narrow proof-app shape
- `scripts/verify-m039-s03.sh` and `scripts/verify-m039-s04.sh` — existing continuity/operator wrappers that M040 should extend instead of bypassing
- `scripts/verify-m039-s04-fly.sh` — real-environment Fly proof contract that should stay part of the milestone bar
- `compiler/mesh-rt/src/dist/node.rs` — discovery/session/remote-spawn lifecycle that continuity replication and failover will stress
- `compiler/mesh-rt/src/dist/global.rs` — current example of live-node replicated state, cleanup, and merge semantics
- DNS discovery and node membership — the live cluster substrate continuity still depends on
- Docker and Fly packaging/runbooks — the operator path that must remain honest as durability is added
- `website/docs/docs/distributed-proof/index.md` and related proof docs — public truth surface that has to move in lockstep with the new verifier and contract

## Open Questions

- What exact continuity record must replicas store beyond ownership/completion truth to support "automatic continuation when possible" without drifting into checkpoint-platform territory? — current thinking: keep the first wave narrow, preserve the restart-by-key baseline, and only store the minimum replica state needed to make automatic continuation honest when it really is possible
- What precise failover trigger and partition rule flips the system from durable mode into "reject new durable work"? — current thinking: if the replica path required by the configured durability policy is not confirmed healthy, the system must stop accepting new durable work rather than downgrade silently
- What exact keyed endpoint names and verifier sequence best fit `cluster-proof` while preserving the current runbook/docs rail? — current thinking: keyed submit + keyed status on the existing proof surface, with the verifier chain and docs/proof page updated in lockstep