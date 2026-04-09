# M045: 

## Vision
Remove all remaining old example-side clustering code so the primary clustered example is as simple as possible while still showing clustering, distribution, and failover from the language side, with all cluster state, routing choice, authority/failover, and status truth owned by the language/runtime.

## Slice Overview
| ID | Slice | Risk | Depends | Done | After this |
|----|-------|------|---------|------|------------|
| S01 | Runtime-Owned Cluster Bootstrap | high | — | ✅ | After this: `meshc init --clustered` produces a visibly smaller clustered app whose startup and inspection path are mostly runtime/public-surface owned instead of proof-app-shaped bootstrap code. |
| S02 | Tiny End-to-End Clustered Example | high | S01 | ✅ | After this: one small local clustered example runs on two nodes and proves runtime-chosen remote execution without app-owned routing or placement logic. |
| S03 | Tiny Example Failover Truth | high | S01, S02 | ✅ | After this: the same tiny example survives primary loss and reports failover/status truth from the runtime without app-owned authority or failover choreography. |
| S04 | Remove Legacy Example-Side Cluster Logic | medium | S01, S02, S03 | ✅ | After this: old `cluster-proof`-style placement/config/status glue is gone or deeply collapsed, and the repo no longer teaches example-owned distributed mechanics as the primary story. |
| S05 | Docs-First Example & Proof Closeout | medium | S02, S03, S04 | ✅ | After this: the docs teach the tiny clustered example first, deeper proof rails are secondary, and the verifier stack proves the same simple language-owned story end to end. |
