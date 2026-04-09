# M043: 

## Vision
Extend Mesh continuity from single-cluster survival to runtime-owned primary→standby disaster continuity, with explicit failover authority, live replicated standby truth, stale-primary fencing, and an honest same-image operator/proof surface.

## Slice Overview
| ID | Slice | Risk | Depends | Done | After this |
|----|-------|------|---------|------|------------|
| S01 | Primary→Standby Runtime Replication and Role Truth | high | — | ✅ | Start a primary cluster and standby cluster with cluster-proof, submit keyed work on the primary, and observe standby-side continuity/status surfaces showing mirrored request truth plus explicit primary/standby role, promotion epoch, and replication health without app-authored DR logic. |
| S02 | Standby Promotion and Stale-Primary Fencing | high | S01 | ✅ | After mirrored standby state is live, kill the primary cluster, perform the explicit promotion action, complete surviving keyed work through the promoted standby, then bring the old primary back and observe that it stays fenced/deposed instead of resuming authority. |
| S03 | Same-Image Two-Cluster Operator Rail | medium | S02 | ✅ | Using the same cluster-proof image and a small env surface, an operator can launch a primary cluster and standby cluster locally, run the packaged destructive failover verifier, and get retained artifacts that show replication, promotion, and fenced rejoin truth. |
| S04 | Public Proof Surface and Operator Contract Truth | medium | S03 | ✅ | The cluster-proof README, distributed-proof docs, proof-surface verifiers, and read-only Fly status checks all show the same failover contract: primary/standby roles, explicit promotion boundary, fenced old-primary behavior, supported topology, and non-goals. |
