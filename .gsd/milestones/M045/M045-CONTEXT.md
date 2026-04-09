# M045: Clean Language-Owned Clustered Example

**Gathered:** 2026-03-28
**Status:** Ready for planning

## Project Description

M045 finishes the cleanup M044 did not fully complete. The clustered example should be a clean simple example of clustering, distribution, and failover all working from the language side. The current repo is close to that story, but the primary example surfaces still carry too much proof-app-shaped machinery: custom bootstrap logic, example-owned config/env handling, custom status/operator payload shaping, and leftover placement or cluster-translation seams.

The user’s standard for this milestone is explicit: **everything related to clustering, fail over, distribution, load balancing, cluster state, routing choice, authority/failover, and status truth should be part of the language and auto-magic**. The example should stop helping the runtime do distributed work.

## Why This Milestone

M044 productized clustered apps, but the docs-grade example is still not small or language-owned enough. If the primary example still looks manual, Mesh still looks more manual than it should, even if the runtime is doing more underneath.

This milestone matters now because the next credibility step is not another big distributed feature. It is proving that the current clustered-app model is truly first-class by making the primary example tiny, readable, and obviously language-owned. If the example still needs proof-app-sized seams, that means the language/runtime boundary is still incomplete.

## User-Visible Outcome

### When this milestone is complete, the user can:

- run one small clustered Mesh example on two local nodes, submit work, and see runtime-owned remote execution without app-owned routing or placement logic
- kill the primary in that same small example and see failover/status truth survive from the language/runtime side, not from example-owned failover choreography

### Entry point / environment

- Entry point: `meshc init --clustered`, the generated example app, built-in runtime/CLI cluster inspection, and the docs pages that teach that path
- Environment: local dev, same-binary two-node proof, CI verifiers, and docs examples
- Live dependencies involved: Mesh node transport, clustered runtime/continuity/authority surfaces, compiler/codegen/tooling seams, and the scaffold/docs path

## Completion Class

- Contract complete means: the public clustered-app example surface compiles and runs with distributed behavior owned by the language/runtime rather than example-specific cluster helpers
- Integration complete means: the scaffolded example, runtime cluster inspection, remote execution path, and failover path all work together on one tiny local example
- Operational complete means: the same tiny example survives primary loss and continues to expose truthful runtime-owned authority/status without switching to a different proof app

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- `meshc init --clustered` produces the primary docs-grade clustered example, and that example can run on two local nodes and show runtime-chosen remote execution with no app-owned cluster logic
- the same small example survives primary loss and reports truthful runtime-owned failover/status state end to end
- the old proof-app-shaped cluster glue is gone or collapsed deeply enough that the example reads as business logic plus minimal ingress/declaration code instead of a distributed systems tutorial

## Risks and Unknowns

- The current scaffold and `cluster-proof` still own more bootstrap/status/config/translation work than the user wants — if that logic cannot move lower, the example cannot honestly become tiny
- Some remaining seams may only look like cleanup but actually require new runtime or public API surface — that could make M045 more than a deletion pass
- Docs simplification can overclaim if the example becomes smaller by hiding deeper proof obligations instead of truly moving behavior into the language/runtime

## Existing Codebase / Prior Art

- `compiler/mesh-pkg/src/scaffold.rs` — current `meshc init --clustered` output; smaller than `cluster-proof`, but still owns startup/bootstrap logic that may need to move lower
- `cluster-proof/work_continuity.mpl` — still a large example-owned continuity/status translation layer and the main sign that the clustered example is not yet small enough
- `cluster-proof/config.mpl` — current example-owned config/env parsing seam that may still be too proof-app-shaped for the desired docs story
- `cluster-proof/cluster.mpl` — still contains membership/placement-style helper code, including leftover logic that no longer appears to be the rightful app boundary
- `compiler/meshc/src/cluster.rs` — built-in runtime/CLI inspection surface that should stay the source of truth rather than example-owned operator routes
- `website/docs/docs/distributed-proof/index.md` and `website/docs/docs/getting-started/index.md` — current public clustered-app teaching surfaces that need to become more obviously scaffold-first and language-owned

> See `.gsd/DECISIONS.md` for all architectural and pattern decisions — it is an append-only register; read it during planning, append to it during execution.

## Relevant Requirements

- R077 — the primary clustered docs example must become tiny and language-first
- R078 — one local example must prove cluster formation, remote execution, and failover end to end
- R079 — example apps must contain no app-owned clustering, failover, routing-choice, load-balancing, or status-truth logic
- R080 — `meshc init --clustered` must become the primary docs-grade clustered example surface
- R081 — public docs must teach the simple example first and keep deeper proof rails secondary
- R049 / R050 / R052 — the already-active clustered continuity/operator requirements must stay honest while the example surface is simplified

## Scope

### In Scope

- simplifying the primary clustered example until it is a clean simple example of it all working from the language side
- moving remaining example-owned clustering/failover/distribution/status mechanics into the language/runtime or built-in public surfaces where needed
- making `meshc init --clustered` the main clustered-app teaching path
- keeping one local example that shows clustering, runtime-chosen remote execution, and failover end to end
- deleting or collapsing old proof-app-shaped cluster glue that only exists because earlier milestones needed it

### Out of Scope / Non-Goals

- expanding the clustered model into a new active-active balancing or consensus feature wave
- preserving old proof-app structure for compatibility when it conflicts with the language-owned example goal
- making Docker or Fly the primary teaching surface for this milestone
- claiming new distributed guarantees beyond the current runtime-owned clustered-app contract just to make the example look more magical

## Technical Constraints

- the example must stay truthful: if the language/runtime does not own a behavior yet, M045 should move that behavior down or keep the docs honest instead of faking simplicity
- all cluster state, routing choice, authority/failover, and status truth for the primary example must come from the language/runtime
- the example should read as business logic plus minimal ingress/declaration code, not a hand-built operator or placement layer
- the simple example is local-first and docs-first, but it still must prove the full clustered story end to end
- deeper verifier rails may remain, but they should become secondary evidence rather than the primary onboarding abstraction

## Integration Points

- `mesh-rt` distributed runtime and continuity/authority surfaces — where remaining example-owned distributed behavior may need to move
- scaffold generation in `compiler/mesh-pkg` — the main teaching/example entrypoint
- compiler/typechecker/codegen seams for declared clustered handlers — the public contract the simple example must rely on
- built-in cluster inspection in `compiler/meshc/src/cluster.rs` — the operator/status truth surface the example should reuse instead of replacing
- `cluster-proof` — current deeper proof consumer and likely source of legacy example-shaped seams to remove or collapse
- docs and verifier scripts — must be rewritten to teach the simple clustered example first without losing proof honesty

## Open Questions

- Exactly which remaining bootstrap and operator seams can move fully into the language/runtime in this milestone, and which require a deliberately smaller public helper surface? — current thinking: the milestone should push as much as possible downward, because a tiny example is the explicit success bar
- How much of `cluster-proof` survives as a deeper proof rail once the simple scaffold-first example is truly good? — current thinking: keep only what still adds distinct proof value after the main example becomes small and language-owned
