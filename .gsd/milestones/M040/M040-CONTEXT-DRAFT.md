---
depends_on: [M039]
draft: true
---

# M040: Replicated Continuity & In-Cluster Request Durability — Context Draft

**Gathered:** 2026-03-28
**Status:** Draft — needs dedicated discussion before planning

## Seed From Current Discussion

This milestone exists because the user does not want Mesh's distributed story to stop at clustering and balancing. The explicit bar is higher: keyed work must survive individual node loss without relying on an external durable store or orchestrator as the real source of truth.

The accepted semantic target for the first honest durability wave is:
- at-least-once execution
- idempotent completion
- safe retry by request key
- continuity replicated across live nodes in the cluster
- two-node safety as the default proof bar, but configurable replication to more nodes later

The user explicitly did **not** want the first honest model to lean on a database or external durable coordinator, and did **not** ask for exactly-once semantics in the first wave.

## What This Milestone Likely Covers

- request-keyed continuity model for the proof app
- replicated in-flight ownership/progress state across live nodes
- safe retry/resume path after individual worker-node loss
- visible proof that duplicate completion does not leak through retries even though the execution model is at-least-once
- operator-visible durability policy / replica-count configuration without turning the proof app into a distributed database

## Why This Needs Its Own Discussion

The direction is clear, but the hard mechanics are still under-specified. There is no finalized decision yet on:
- how continuity state is replicated inside the cluster
- whether ownership transfer is leader-based, replica-based, or some other cluster-native pattern
- what partition behavior is acceptable before the system stops claiming continuity
- how much of the continuity state is exposed to operators or the proof endpoint
- how to keep the implementation honest without silently becoming a consensus platform for arbitrary app state

A dedicated M040 discussion should turn those into specific capability and verification decisions.

## Existing Codebase / Prior Art To Revisit

- `compiler/mesh-rt/src/dist/node.rs` — node monitoring, remote spawn, and session lifecycle
- `compiler/mesh-rt/src/dist/global.rs` — replicated registry and cleanup behavior
- M039 proof app and verifiers — the continuity work should extend the narrow proof path rather than start a second proof surface
- `mesher/ingestion/pipeline.mpl` — current examples of remote spawn and cluster-aware work movement, even if not yet durable

## Technical Findings Already Established

- the current distributed runtime already supports node identity, peer connections, monitoring, remote send/spawn, and replicated global names
- the current docs surface is ahead of the proof surface; M040 should continue the pattern of proving before broadening claims
- the user wants continuity replicated across live nodes, but does **not** want an external durable store or orchestrator doing the real work
- the currently accepted semantic bar is at-least-once with idempotent completion, not exactly-once

## Likely Risks / Unknowns

- the continuity design can sprawl into a generic consensus/data platform if the scope is not kept tied to keyed request durability
- partitions are dangerous here: continuity claims can become fake if the system keeps acting healthy after losing the replicas it depends on
- proving resume/ownership transfer honestly will likely expose runtime assumptions that do not show up in happy-path cluster balancing

## Likely Outcome When Done

Mesh can prove that a keyed unit of work survives individual node loss through cluster-internal replicated continuity, with retry-safe visible completion and no external durability dependency.

## Open Questions For The Dedicated Discussion

- What ownership/replication model best fits the first honest in-cluster continuity proof?
- What exact partition behavior is acceptable before continuity claims must degrade or stop?
- How much of the continuity state should be visible in the proof app and operator surface?
- What verifier sequence would make duplicate completion or lost continuity mechanically obvious?
