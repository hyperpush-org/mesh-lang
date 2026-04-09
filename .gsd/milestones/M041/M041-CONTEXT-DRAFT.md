---
depends_on: [M040]
draft: true
---

# M041: Cross-Cluster Disaster Continuity — Context Draft

**Gathered:** 2026-03-28
**Status:** Draft — needs dedicated discussion before planning

## Seed From Current Discussion

This milestone exists because the user's actual end goal is bigger than single-cluster resilience. They explicitly said full cluster loss is in scope and that, if necessary, the work should use more than one milestone because that is "the whole point of the language."

The accepted first disaster-recovery shape is:
- active-primary cluster
- live replication to a standby cluster
- not active-active request intake in the first wave
- Fly.io as one real environment to prove the behavior, but not the architectural definition

The system still must not rely on an external durable store or orchestrator as the real truth source.

## What This Milestone Likely Covers

- cross-cluster replication of continuity state from primary to standby
- failover after full primary-cluster loss
- request-truth survival when one entire cluster is gone but standby replicas still exist
- re-entry / resumption path after disaster without pretending lost-all-replicas cases are recoverable
- one-image operator workflow extended to cluster role / failover configuration

## Why This Needs Its Own Discussion

The current conversation was enough to set direction, but not enough to finalize the disaster model. The biggest unresolved areas are:
- what failover automation vs operator action is honest for the first wave
- how primary/standby role is established and observed
- what the replication contract between clusters actually is
- how to reason about partitions between clusters without quietly inventing a consensus product
- what counts as successful failover proof on Fly in a way that is repeatable and not theater

A dedicated M041 discussion should set those boundaries explicitly before planning.

## Existing Codebase / Prior Art To Revisit

- M039 narrow proof app and M040 continuity model — M041 should extend those rather than fork them
- `compiler/mesh-rt/src/dist/node.rs` and related runtime surfaces — cross-cluster continuity will build on the same node/session/monitoring foundation
- Fly deployment surfaces already present in `registry/` and `packages-website/` — useful only as operational prior art, not as the distributed architecture itself

## Technical Findings Already Established

- the user wants general architecture, with Fly used as one real proof environment rather than as the only supported shape
- the first disaster-continuity shape is active-primary with live replication to standby, not active-active
- the design must stay honest about surviving replicas: no external store, but also no claim of recovery when no replica survived anywhere

## Likely Risks / Unknowns

- cross-cluster work can easily drift into a vague DR story with no crisp failover contract
- if failover is too manual, the language claim weakens; if it is over-automated too early, the system may overclaim safety it cannot yet prove
- standby replication can accidentally become external-orchestrator truth if the boundary is not made explicit

## Likely Outcome When Done

Mesh can prove that active keyed work survives full loss of the primary cluster because a standby cluster already has enough live replicated continuity state to take over honestly.

## Open Questions For The Dedicated Discussion

- What should the first honest failover trigger look like: explicit operator action, bounded automatic promotion, or something in between?
- What continuity state must be replicated cross-cluster to make failover truthful without overbuilding?
- How should primary/standby role and failover state be surfaced to operators and the proof endpoint?
- What exact Fly-based proof sequence would make full-cluster-loss survival undeniable?
