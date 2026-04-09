---
depends_on: [M039]
---

# M042: Runtime-Native Distributed Continuity Core

**Gathered:** 2026-03-28
**Status:** Ready for planning

## Project Description

This milestone changes the implementation boundary for Mesh's distributed story. M039 proved automatic discovery, truthful membership, runtime-native work routing, degrade/rejoin continuity, and the one-image operator path through `cluster-proof`. M040's first keyed slice proved a narrow standalone request-key contract, but the follow-on continuity plan exposed the wrong ownership boundary: distribution, replication, failover, and continuity should be runtime-native capabilities in `mesh-rt`, with a small ergonomic API exposed to Mesh programs.

M042 makes single-cluster continuity a first-class language/runtime feature rather than app-authored distributed orchestration in `cluster-proof/work.mpl`. The runtime should own the hard parts — replica-backed request state, fail-closed durability admission, owner-loss recovery semantics, and the safe retry/idempotence substrate — while Mesh code consumes those capabilities through a narrow API and surfaces status truth through ordinary app code. The existing Mesh-authored keyed submit/status/retry behavior is not throwaway: it becomes the semantic seed to absorb into the language/runtime surface so users get the same capability as a first-class feature instead of re-implementing it.

## Why This Milestone

The current app-level continuity plan is pulling core language claims into one proof app's Mesh implementation. The recent cluster-mode crashes and the earlier transport limitations both point at the same root issue: if Mesh is going to claim built-in distribution and replication, the hard distributed machinery belongs in Rust/runtime code, not in user-authored Mesh workflow logic.

This needs to happen before more single-cluster or cross-cluster milestones pile product-level logic on top of a shaky boundary. The next honest step is to move the substrate down a layer, keep the API simple for Mesh users, and let proof apps demonstrate the capability instead of implementing it. Just as importantly, the working Mesh-authored keyed contract already explored in `cluster-proof` should be preserved and promoted into that first-class API rather than discarded.

## User-Visible Outcome

### When this milestone is complete, the user can:

- write Mesh code against a small runtime-native distributed continuity API instead of hand-authoring owner/replica orchestration logic in Mesh application code
- run `cluster-proof` on the same Docker/Fly rail and see keyed continuity, replica truth, and fail-closed durability behavior backed by runtime-owned machinery

### Entry point / environment

- Entry point: runtime-owned distributed continuity API consumed by `cluster-proof` and exercised by repo verifiers/tests
- Environment: local Docker first, then the same Fly/operator rail already established by M039
- Live dependencies involved: DNS-based discovery, Mesh node transport/session lifecycle, cluster membership, Docker, Fly; explicitly not an external durable database or orchestrator

## Completion Class

- Contract complete means: the runtime exposes a narrow Mesh-facing API for keyed distributed continuity, safe retry/idempotence, replica-backed admission, and observable owner/replica status without requiring app-authored replication logic
- Integration complete means: `cluster-proof` is reduced to a thin consumer/proof surface over the runtime capability, and the same one-image operator path still works end to end
- Operational complete means: owner loss, replica loss, same-key retry, degrade/rejoin, and fail-closed admission are proven under real clustered lifecycle conditions rather than only unit tests

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- a Mesh app can submit keyed work through the runtime-native API, inspect truthful status/owner/replica state, and survive individual node loss without implementing replica logic in Mesh code
- the runtime rejects new durable work when the configured replica safety is unavailable instead of silently downgrading the durability claim
- `cluster-proof` and the existing Docker/Fly operator rail prove the capability as a consumer of the API rather than as the owner of the distributed algorithm

## Risks and Unknowns

- the runtime API could become too low-level or too magical — it needs to expose enough control/status truth for operators without forcing app authors back into hand-rolled coordination
- moving logic into `mesh-rt` can sprawl into a fake distributed database or consensus product if the scope stops being tightly anchored to keyed request continuity
- if the API leaks unstable transport/payload assumptions, Mesh users will still end up coding around runtime seams instead of trusting a first-class feature

## Existing Codebase / Prior Art

- `compiler/mesh-rt/src/dist/node.rs` — current discovery/session/remote-spawn/runtime lifecycle seam that the new substrate must build on
- `compiler/mesh-rt/src/dist/global.rs` — existing replicated runtime-owned state machinery and cleanup behavior that can inform replica-backed request truth
- `cluster-proof/work.mpl` — current proof-app logic that should shrink into a thin consumer of the runtime capability instead of remaining the primary implementation site
- `compiler/meshc/tests/e2e_m039_s03.rs` — existing continuity-style proof harness patterns to preserve while the implementation boundary moves down-stack
- `scripts/verify-m039-s04.sh` and `scripts/verify-m039-s04-fly.sh` — operator rails the new substrate must keep honest rather than replace

> See `.gsd/DECISIONS.md` for all architectural and pattern decisions — it is an append-only register; read it during planning, append to it during execution.

## Relevant Requirements

- R049 — advances keyed at-least-once idempotent continuity as a language/runtime capability instead of an app-authored pattern
- R050 — advances replica-backed continuity and two-node safety on a runtime-owned substrate
- R052 — preserves the one-image small-env operator path while moving the implementation boundary into `mesh-rt`
- R053 — requires the proof/docs surface to stay honest as the runtime API replaces app-level continuity logic

## Scope

### In Scope

- runtime-native keyed continuity primitives in `mesh-rt`
- promoting the existing Mesh-authored keyed submit/status/retry semantics into a first-class language/runtime API instead of replacing them with a different contract
- a small Mesh-facing API for submit/status/retry and observable durability state
- runtime-owned owner/replica tracking, fail-closed admission, and owner-loss recovery substrate for the single-cluster case
- reducing `cluster-proof` to a proof surface and API consumer
- preserving or extending the current verifier/doc rail so the public claim stays truthful

### Out of Scope / Non-Goals

- full cross-cluster disaster failover
- exactly-once semantics
- arbitrary replicated application state beyond keyed continuity needs
- leaving `cluster-proof` as the real owner of distribution/replication algorithms

## Technical Constraints

- M039 stays the validated baseline and should not be rewritten; M042 supersedes its successor direction rather than editing history
- the first runtime-owned continuity wave must stay single-cluster and keyed, with restart-by-key as the honest resume model
- the Mesh-facing API must be easy to use, but it must still expose enough status truth that failures are observable and testable
- the same Docker/Fly operator path must remain the public proof rail

## Integration Points

- `mesh-rt` distributed runtime — owns the new continuity substrate
- `cluster-proof` — consumes the new API and remains the narrow proof app
- `compiler/meshc/tests/` and `scripts/verify-m039-s04*.sh` — prove the runtime-owned path end to end
- `website/docs/docs/distributed-proof/` and related runbooks — must describe the runtime API truthfully once it exists

## Open Questions

- What exact Mesh-facing API shape is small enough to feel first-class but explicit enough to keep owner/replica/durability truth visible? — current thinking: a small request-keyed submit/status/retry surface with runtime-owned policy and observable status records
- What minimum replica state is required for honest automatic continuation versus restart-by-key? — current thinking: keep the first wave narrow and only persist the minimum runtime-owned continuity record needed for truthful retry/failover behavior
- Which pieces of the current `cluster-proof` keyed contract should remain app-facing versus becoming direct runtime API semantics? — current thinking: preserve the useful request-key/status contract but move replica placement, admission, and failover behavior behind the runtime boundary
