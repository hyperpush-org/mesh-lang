# M039: Auto-Discovery & Native Cluster Balancing

**Gathered:** 2026-03-28
**Status:** Ready for planning

## Project Description

This milestone is about proving Mesh's horizontal scaling story in a way that feels real, not partial. The target is a narrow distributed proof app with one endpoint, multiple running instances, automatic awareness of peers, and runtime-native work movement across nodes. The user explicitly wants the cluster behavior to feel "just like Elixir" in the important ways: instances come up, discover each other automatically, know who else is alive, and can spread work across the cluster without relying on manual peer lists.

The proof must be general. Fly.io is only one place to deploy on for proof; it is not the architecture. The intended first discovery provider is DNS-based discovery through a general discovery seam. The operator path should feel simple: run one Docker image with a small env-driven surface and let the runtime do the clustering work.

## Why This Milestone

Mesh already has real distributed runtime primitives and public docs that talk about distributed nodes, remote spawn, global registry, and cluster-wide actor communication. `mesher/` also uses pieces of this today. What is missing is a narrow, undeniable proof surface that shows the language can actually auto-form a cluster, maintain truthful membership, and redistribute work internally at runtime instead of relying on a front-door proxy to do the real balancing.

M039 exists to close that gap first, before the later milestones claim replicated continuity and disaster survival. The repo should not move into in-cluster replicated durability or cross-cluster failover while the base cluster formation and balancing story is still only partially proven.

## User-Visible Outcome

### When this milestone is complete, the user can:

- run the same Docker image as multiple nodes locally or on Fly, have those nodes discover each other automatically, and inspect truthful cluster membership without hand-entering peer lists
- hit one proof endpoint repeatedly and see requests enter on one node, execute on another node when the runtime chooses to rebalance internally, and keep working when an individual node dies or rejoins

### Entry point / environment

- Entry point: one narrow proof app container image plus its single HTTP endpoint and verification commands
- Environment: local multi-node dev and production-like Fly multi-region deployment
- Live dependencies involved: DNS-based discovery, TLS/cookie-authenticated node transport, Fly only as one real proof environment

## Completion Class

- Contract complete means: shell verifiers, tests, and artifact checks prove DNS-driven auto-discovery, truthful membership, ingress-vs-execution visibility, and one-image operator setup
- Integration complete means: a real cluster formed from multiple running instances can move work internally across nodes and recover from single-node failure/rejoin without manual repair
- Operational complete means: node join, loss, and rejoin are exercised under real lifecycle conditions locally and on Fly, with public docs and claims reconciled to the verified path

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- the same proof app image forms a cluster automatically in local multi-node execution and on Fly without manual peer lists
- the proof endpoint demonstrates runtime-native internal balancing by distinguishing ingress node from execution node on real requests
- killing one running node degrades safely, preserves truthful membership, keeps the cluster serving new work, and lets the node rejoin cleanly without operator repair steps

## Risks and Unknowns

- DNS-based discovery may be clean as a first provider but still leave tricky edge cases around stale or partial membership views — if membership truth is soft, every higher-level claim becomes suspect
- The runtime already has connect/list/registry primitives, but the repo has not yet proven that those pieces assemble into an honest operator path with one image and no peer-list handholding — phantom capability is the main risk here
- Internal balancing can be faked accidentally by front-door request spread unless the proof app makes ingress-node truth and execution-node truth visibly different
- Node loss and rejoin across real environments can expose lag, monitor holes, or split-brain-adjacent behavior that does not show up in unit tests

## Existing Codebase / Prior Art

- `compiler/mesh-rt/src/dist/node.rs` — current node transport, peer-list exchange, monitoring, and remote spawn runtime surface
- `compiler/mesh-rt/src/dist/global.rs` — replicated global registry used for cluster-wide process lookup
- `website/docs/docs/distributed/index.md` — current public distributed-language claims that M039 must reconcile to the actual proof path
- `mesher/main.mpl` — current env-driven node startup and manual peer connection path via `MESHER_NODE_NAME`, `MESHER_COOKIE`, and `MESHER_PEERS`
- `mesher/ingestion/pipeline.mpl` — current dogfood usage of `Node.list`, `Node.spawn`, `Node.monitor`, and `Global.register`
- `mesher/api/helpers.mpl` — cluster-aware `Global.whereis("mesher_registry")` lookup pattern already used on the app side

> See `.gsd/DECISIONS.md` for all architectural and pattern decisions — it is an append-only register; read it during planning, append to it during execution.

## Relevant Requirements

- R045 — establish general auto-discovery with DNS as the first canonical provider
- R046 — make membership truth visible and reliable on join/loss/rejoin
- R047 — prove runtime-native internal balancing rather than front-door-only spread
- R048 — prove single-cluster failure and rejoin without manual repair
- R052 — keep the operator surface to one image and small env-driven config
- R053 — reconcile docs and public claims to the canonical verified proof path

## Scope

### In Scope

- a new narrow proof app rather than repurposing `mesher/` as the primary proof surface
- one-image, env-driven local and Fly operator flow
- general discovery architecture with DNS as the first real provider
- truthful membership visibility and cluster-health introspection for the proof path
- runtime-native internal balancing with visible ingress-vs-execution truth
- safe single-cluster node loss and clean rejoin
- canonical verifier/docs reconciliation for distributed claims

### Out of Scope / Non-Goals

- in-cluster replicated request durability beyond what is needed to support M039's single-cluster failure/rejoin proof; full keyed continuity belongs in M040
- cross-cluster replication and standby-cluster disaster survival; that belongs in M041
- front-door load balancer spread counting as proof of Mesh runtime-native balancing
- Fly-specific clustering architecture or API-token-based control-plane discovery as the primary design
- exactly-once semantics, generic consensus for arbitrary app data, or impossible "durability with no surviving replica anywhere" claims

## Technical Constraints

- Discovery has to remain general; Fly is only one proof target
- Runtime-native balancing must not rely on an external orchestrator or durable store as the real balancing path
- Cluster admission should be explicit and safe; the likely first boundary is shared secret/cookie plus TLS transport and cluster identity
- The proof app must make ingress-node and execution-node truth separately visible so the runtime's behavior cannot be confused with proxy behavior
- The operator path should stay small enough that running the image feels like using a language/runtime capability, not hand-assembling a distributed system

## Integration Points

- `compiler/mesh-rt/src/dist/node.rs` — discovery hooks, node connect/list/monitor/spawn, and session lifecycle
- `compiler/mesh-rt/src/dist/global.rs` — cluster-wide replica of named-process lookup
- proof app runtime and HTTP surface — visible ingress/execution/membership proof endpoint
- DNS discovery source — first canonical peer-discovery provider
- Fly.io — real multi-region proof environment only, not the architecture definition
- local multi-node environment — first replayable development proof

## Open Questions

- How should the discovery seam be represented in Mesh/runtime config so DNS-first lands cleanly without overbuilding later providers? — current thinking: one generic discovery contract with a single canonical DNS provider in M039
- How should the proof app expose cluster truth without becoming a product app? — current thinking: one keyed proof endpoint plus lightweight membership/status visibility
- What is the cleanest node-loss/rejoin verifier path that stays honest across both local and Fly runs? — current thinking: one canonical shell verifier plus targeted runtime tests and doc-truth checks
