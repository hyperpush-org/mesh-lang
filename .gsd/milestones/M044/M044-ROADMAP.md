# M044: 

## Vision
Take the M042/M043 continuity and failover substrate and make it a real language/platform feature for ordinary Mesh apps by moving from proof-app-specific continuity plumbing to a declared clustered-handler model: apps opt into clustered mode in `mesh.toml`, declare which service/message/work handlers are clustered, the runtime owns placement/continuity/authority/failover behind that boundary, undeclared code stays local, and bounded automatic promotion becomes the default failover path when the runtime can prove it is safe.

## Slice Overview
| ID | Slice | Risk | Depends | Done | After this |
|----|-------|------|---------|------|------------|
| S01 | Clustered Declarations & Typed Public Surface | high | — | ✅ | After this: a Mesh app can opt into clustered mode in `mesh.toml`, declare clustered handlers, and compile against typed continuity/authority values without continuity JSON parsing in app code. |
| S02 | Runtime-Owned Declared Handler Execution | high | S01 | ✅ | After this: the same binary can run on two nodes and execute declared clustered handlers with runtime-owned placement and continuity, while undeclared code stays ordinary local Mesh code. |
| S03 | Built-in Operator Surfaces & Clustered Scaffold | high | S02 | ✅ | After this: `meshc init --clustered` scaffolds a clustered app, and built-in runtime/CLI surfaces can inspect membership, authority, continuity status, and failover diagnostics without app-defined operator wiring. |
| S04 | Bounded Automatic Promotion | high | S02, S03 | ✅ | After this: killing the active primary causes safe auto-promotion for declared clustered work when the runtime can prove safety; ambiguous cases fail closed, and stale-primary rejoin stays fenced. |
| S05 | Cluster-Proof Rewrite, Docs, and Final Closeout | medium | S01, S02, S03, S04 | ✅ | After this: `cluster-proof` is a dogfood consumer of the new clustered-app model, the old explicit clustering path is gone from its code, and the docs/verifiers teach “build a clustered Mesh app” as the primary story. |
