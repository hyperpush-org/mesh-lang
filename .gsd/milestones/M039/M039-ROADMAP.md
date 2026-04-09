# M039: 

## Vision
Prove that Mesh can form a cluster automatically, balance work internally across nodes, and survive single-cluster node loss/rejoin through a narrow one-endpoint proof app that feels real rather than theoretical.

## Slice Overview
| ID | Slice | Risk | Depends | Done | After this |
|----|-------|------|---------|------|------------|
| S01 | General DNS Discovery & Membership Truth | high | — | ✅ | After this: multiple nodes started from the same image can auto-discover and report truthful membership locally and on Fly without manual peer lists. |
| S02 | Native Cluster Work Routing Proof App | high | S01 | ✅ | After this: one proof endpoint can show ingress node and execution node separately, proving that Mesh moved work internally across the cluster. |
| S03 | Single-Cluster Failure, Safe Degrade, and Rejoin | high | S01, S02 | ✅ | After this: killing and restarting a node shows safe degrade, truthful membership updates, continued service for new work, and clean rejoin without manual repair. |
| S04 | One-Image Operator Path, Local/Fly Verifiers, and Docs Truth | medium | S01, S02, S03 | ✅ | After this: one Docker image, one env-driven operator contract, and one canonical verifier/doc path prove the M039 cluster story locally and on Fly. |
