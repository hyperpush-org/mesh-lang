---
depends_on: [M042]
draft: true
---

# M043: Runtime-Native Cross-Cluster Disaster Continuity

**Gathered:** 2026-03-28
**Status:** Draft — needs dedicated discussion before planning

## Project Description

This milestone carries the distributed continuity story past single-cluster replica survival into full primary-cluster loss, but it does so on top of the runtime-native substrate established in M042. The goal is not to make `cluster-proof` or another Mesh app implement disaster recovery logic itself. The runtime should own cross-cluster continuity replication, cluster role/state truth, and failover boundaries, while Mesh programs use the same simple API and surface the resulting status honestly. The single-cluster semantics already explored in Mesh code should carry forward into this runtime-owned cross-cluster path rather than being replaced with a different user-facing contract.

The first disaster-recovery shape remains intentionally narrow:

- active-primary cluster
- live replication to a standby cluster
- no active-active request intake in the first wave
- no external durable store or orchestrator as the real authority
- no claim of survivability when no replica survived anywhere

## Why This Milestone

The user’s end goal is bigger than single-cluster resilience. If Mesh is going to claim serious distributed continuity, it must eventually prove that a whole-cluster failure can be survived without quietly outsourcing truth to a database or control plane. But after the milestone reorganization, that proof has to build on runtime-native distribution/replication instead of on app-authored disaster logic.

This remains a separate milestone because the disaster model still needs sharper boundaries than the single-cluster case: role assignment, failover triggers, cross-cluster partition behavior, and what counts as honest automation versus operator action are still open design questions.

## User-Visible Outcome

### When this milestone is complete, the user can:

- keep using the same high-level distributed continuity API while the runtime replicates continuity state from a primary cluster to a standby cluster behind the scenes
- prove that keyed work survives full loss of the active cluster because a standby cluster already has enough live replicated state to take over honestly

### Entry point / environment

- Entry point: the runtime-native distributed continuity API plus `cluster-proof` as the public proof surface
- Environment: local multi-cluster proof first, then Fly as one real environment on the same operator rail
- Live dependencies involved: Mesh node transport, discovery, cluster role/failover state, Docker, Fly; explicitly not an external durable database or orchestrator

## Completion Class

- Contract complete means: the runtime can replicate continuity state from an active primary cluster to a standby cluster and expose truthful failover state through the Mesh-facing API
- Integration complete means: `cluster-proof` proves full primary-cluster-loss survival on top of the runtime API without re-implementing DR logic in Mesh code
- Operational complete means: role assignment, failover, standby promotion, and post-failover status truth are exercised under real lifecycle conditions rather than only simulated tests

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- keyed continuity state is replicated from a primary cluster to a standby cluster by runtime-owned machinery, not by app-authored Mesh orchestration
- full loss of the primary cluster still allows a standby cluster to converge the surviving keyed work honestly when replicas still exist there
- the operator path and public proof/docs surface describe the failover contract, non-goals, and manual-vs-automatic boundaries exactly as the verifiers exercise them

## Risks and Unknowns

- cross-cluster continuity can easily sprawl into a vague DR story or implicit consensus system if the failover contract is not made explicit
- if failover is too manual, the language claim weakens; if it is over-automated too early, Mesh may overclaim safety it cannot yet prove
- the cross-cluster replication model must stay clearly runtime-owned so standby truth does not quietly become a disguised external control plane

## Existing Codebase / Prior Art

- `compiler/mesh-rt/src/dist/` — runtime substrate that must own cross-cluster continuity rather than pushing it into app code
- `cluster-proof/` — narrow proof surface that should demonstrate the runtime capability instead of implementing it
- `scripts/verify-m039-s04-fly.sh` — existing real-environment operator verifier rail to extend truthfully
- M041 draft context — prior disaster-recovery intent that now needs to be re-expressed on top of a runtime-native substrate instead of the old app-level continuity plan

> See `.gsd/DECISIONS.md` for all architectural and pattern decisions — it is an append-only register; read it during planning, append to it during execution.

## Relevant Requirements

- R051 — advances full active-cluster-loss survivability through live replication to a standby cluster
- R052 — preserves the one-image env-driven operator path while extending it to cluster role/failover truth
- R053 — requires the public proof and docs surface to stay honest as disaster-continuity claims become runtime-native

## Scope

### In Scope

- runtime-native cross-cluster continuity replication
- truthful primary/standby role and failover state
- proving disaster continuity through `cluster-proof` as a consumer of the runtime API
- extending the operator/doc/proof rail to the cross-cluster case

### Out of Scope / Non-Goals

- active-active multi-cluster request intake in the first wave
- external durable stores or orchestrators as the real continuity authority
- claiming recovery when no surviving replica exists in any cluster
- moving disaster-recovery orchestration back into Mesh application code

## Technical Constraints

- M043 depends on M042; cross-cluster work should not start before the single-cluster substrate is runtime-native
- the Mesh-facing API should remain simple and mostly stable across single-cluster and cross-cluster continuity modes
- Fly remains a proof environment, not the architecture definition
- public docs and verifiers must state exactly what is automatic, what is operator-driven, and what is not supported

## Integration Points

- `mesh-rt` distributed continuity substrate — owns cross-cluster replication and failover boundaries
- `cluster-proof` — public proof app consuming the runtime API
- Docker/Fly operator workflows — prove the real environment path
- distributed-proof docs/runbooks — publish the truthful failover contract

## Open Questions

- What is the first honest failover trigger: explicit operator action, bounded automatic promotion, or something in between? — current thinking: keep this open until M042 proves the substrate and observability seams
- What continuity state must replicate cross-cluster to make failover truthful without overbuilding? — current thinking: only the runtime-owned keyed continuity record needed for honest takeover, not arbitrary replicated application state
- How should primary/standby role and promotion state appear in the Mesh-facing API and proof surface? — current thinking: preserve simple app APIs while making role/failover status explicit in operator-visible status records
