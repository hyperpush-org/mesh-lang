# M040:

## Vision
Extend the existing `cluster-proof` rail from cluster formation and remote routing into honest replicated continuity: callers submit keyed work, inspect durable status with owner/replica visibility, and prove that keyed work converges through individual node loss via cluster-internal continuity rather than an external durable store.

## Slice Overview
| ID | Slice | Risk | Depends | Done | After this |
|----|-------|------|---------|------|------------|
| S01 | Keyed Submit/Status Contract on the Existing Proof Rail | high — this establishes the real request-key contract, visible state model, and idempotent semantics every later continuity claim depends on. | — | ✅ | An operator submits keyed work to `cluster-proof`, polls keyed status, sees stable request-key vs attempt identity information plus owner/replica placeholders or initial assignment data, and retries the same key on a healthy cluster without duplicate completion leakage. |
| S02 | Replica-Backed Admission, Fail-Closed Policy, and Owner-Loss Convergence | highest — this is the core durability seam where deterministic placement, replica acknowledgment, node monitoring, and truthful failover behavior must work on the existing runtime substrate. | S01 | ⬜ | On a real two-node cluster, keyed work is accepted only after replica-backed durability is confirmed, status shows owner/replica truth, new durable work is rejected when replica safety disappears, and a request still converges after owner loss through surviving continuity and same-key retry/continuation. |
| S03 | Operator Verifier and Docs Truth Alignment for Keyed Continuity | medium — broad blast radius across scripts, e2e harnesses, runbooks, and public docs, with truth drift risk if the shipped contract and proof artifacts diverge. | S01, S02 | ⬜ | The existing local Docker and Fly verification rail runs against keyed submit/status behavior, preserves pre-loss/degraded/post-rejoin evidence, and the README/public distributed-proof page describe exactly the continuity guarantees and non-goals the verifiers exercise. |
