# M042: 

## Vision
Move keyed distributed continuity from app-authored Mesh orchestration into a runtime-native mesh-rt subsystem with a small Mesh-facing API, while keeping cluster-proof as the thin proof consumer and preserving the existing one-image Docker/Fly operator rail.

## Slice Overview
| ID | Slice | Risk | Depends | Done | After this |
|----|-------|------|---------|------|------------|
| S01 | Runtime-native keyed continuity API on the healthy path | high | — | ✅ | After this slice, cluster-proof submits keyed work and reads keyed status through a runtime-native continuity API on standalone and healthy two-node clusters, while preserving request_key vs attempt_id, duplicate dedupe, conflict rejection, and explicit owner/replica status fields. |
| S02 | Replica-backed admission and fail-closed durability truth | high | S01 | ✅ | After this slice, the same cluster-proof keyed submit path either accepts work with mirrored replica truth or rejects it explicitly when replica safety is unavailable; operators can inspect that mirrored/degraded/rejected state through the ordinary status surface. |
| S03 | Owner-loss recovery, same-key retry, and stale-completion safety | high | S02 | ✅ | After this slice, a two-node cluster can lose the active owner and still serve truthful keyed continuity status from surviving replicated state; same-key retry converges through a rolled attempt_id, stale completions are rejected, and rejoin is observable through the same runtime-owned status model. |
| S04 | Thin cluster-proof consumer and truthful operator/docs rail | medium | S03 | ✅ | After this slice, cluster-proof is visibly just a thin consumer over the runtime-native continuity API, and the repo’s Docker/Fly/operator/docs surfaces truthfully show the runtime-owned capability instead of app-authored continuity machinery. |
