# M046: 

## Vision
Completely delete the current `cluster-proof/`, rebuild it as a tiny route-free clustered proof, add a new local `tiny-cluster/`, and push the remaining clustered-work trigger/control/status seams down into Mesh so the only thing app authors do is denote what work gets replicated while the language/runtime handles everything else.

## Slice Overview
| ID | Slice | Risk | Depends | Done | After this |
|----|-------|------|---------|------|------------|
| S01 | Dual clustered-work declaration | high | — | ✅ | After this: a tiny Mesh package can mark clustered work through `mesh.toml` or a source decorator, and both forms compile to the same declared runtime boundary. |
| S02 | Runtime-owned startup trigger and route-free status contract | high | S01 | ✅ | After this: a route-free clustered app can auto-run its clustered work on startup and be inspected entirely through built-in `meshc cluster ...` surfaces, with no app-owned submit/status routes. |
| S03 | `tiny-cluster/` local no-HTTP proof | medium | S01, S02 | ✅ | After this: `tiny-cluster/` proves the local clustered story with no HTTP routes, trivial `1 + 1` work, and live runtime-owned placement/failover/status truth. |
| S04 | Rebuild `cluster-proof/` as tiny packaged proof | high | S01, S02 | ✅ | After this: `cluster-proof/` is a packaged route-free proof app on the same tiny `1 + 1` clustered contract instead of a legacy proof app with its own trigger/status layers. |
| S05 | Equal-surface scaffold alignment | medium | S03, S04 | ✅ | After this: the scaffold, `tiny-cluster/`, and `cluster-proof/` all show the same clustered-work story, and docs/verifiers fail closed if one drifts. |
| S06 | Assembled verification and docs closeout | medium | S03, S04, S05 | ✅ | After this: one assembled verifier replays the local and packaged route-free proofs and re-checks startup-triggered work, failover, and status truth end to end. |
