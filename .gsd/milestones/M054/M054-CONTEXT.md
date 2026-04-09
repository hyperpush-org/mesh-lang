# M054: Load Balancing Truth & Follow-through — Context Draft

**Gathered:** 2026-04-05
**Status:** Ready for planning

## Project Description

This milestone does a deep dive into how load balancing actually works today for clustered Mesh apps, then carries real follow-through if the current story is not good enough. The target public model is still server-side first: a frontend or browser client should usually point at one public app URL, and the balancing or routing story should not require the client to know nodes. Replay is acceptable inside that one-URL contract. Fly can remain the current proving environment, but the contract has to stay platform-agnostic instead of collapsing into Fly-specific behavior.

This milestone is for both audiences equally: evaluators deciding whether the load-balancing claim is real, and adopters trying to understand what networking contract they can actually rely on.

## Why This Milestone

The repo already makes strong public claims — including `website/docs/index.md` describing Mesh as having "Built-in failover, load balancing, and exactly-once semantics" — while the current public materials mostly explain clustered apps, deploy truth, retained proof rails, and Fly as a proving environment. They do not yet answer the practical question cleanly enough: if a frontend points at one public server URL, where does balancing really happen?

M053 closed the serious starter deploy truth for the generated Postgres starter. That makes the remaining balancing claim sharper, not safer. Mesh now needs to explain ingress versus runtime-owned routing honestly, and if the current server-side story is weak, this milestone should not stop at explanation alone. It should carry real follow-through instead of leaving a fake-green public claim in place.

## User-Visible Outcome

### When this milestone is complete, the user can:

- point a frontend or browser client at one public app URL for the serious clustered starter, let the server-side stack handle routing, and understand what happened without learning node topology
- read the Mesh docs and deploy guidance and tell where ingress balancing ends, where Mesh runtime placement or failover begins, and what Fly proves versus what stays portable

### Entry point / environment

- Entry point: one public clustered app URL, the docs home page, `Clustered Example`, `Distributed Proof`, generated starter READMEs, and `meshc cluster ...` inspection surfaces
- Environment: browser-facing app traffic, local proof, CI, and production-like clustered deployment, with Fly as the current proving environment
- Live dependencies involved: frontend or browser clients, Fly Proxy / `fly-replay`, the generated Postgres starter, PostgreSQL, and Mesh runtime/operator surfaces

## Completion Class

- Contract complete means: Mesh can state the one-public-URL, server-side-first balancing story honestly in docs and proof surfaces; the wording maps to real verifier or artifact evidence; and any gap between the public claim and actual behavior is either repaired or explicitly narrowed
- Integration complete means: the docs home page, clustered docs, generated starter guidance, deploy proof, and runtime/operator truth all describe the same balancing model instead of mixing ingress behavior, runtime placement, and Fly-specific evidence into one blurry claim
- Operational complete means: a production-like clustered starter run can take real requests through one public app URL, route them correctly with server-side behavior that does not require client node awareness, and expose truthful evidence about what the platform/proxy handled versus what Mesh runtime handled

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- a real clustered starter can accept browser or frontend traffic through one public app URL, complete real requests successfully, and explain the resulting server-side routing path honestly without requiring the client to know nodes
- the public docs and retained proof rails show Fly as the current proving environment while clearly separating Fly Proxy ingress behavior from Mesh runtime-owned placement, failover, and operator truth
- if the current story is not sufficient, the milestone includes the runtime/platform/deploy follow-through needed to close the gap; a docs-only rewrite is not enough if it leaves the public claim materially stronger than the shipped behavior

## Risks and Unknowns

- The current public claim may already be stronger than the actual ingress-plus-runtime behavior — that matters because the home page already promises load balancing as a built-in capability
- Fly proof could accidentally turn into the product contract — that matters because the user wants Fly as evidence, not as the definition of Mesh distribution
- Different request classes may not all fit the same simple story — that matters because "one public URL" can still be misleading if reads, writes, replay, and failover each behave differently and the docs flatten that difference
- Frontend-aware adapters may turn out to be necessary after all — that matters because they are explicitly deferred today and should only be introduced if the server-side/runtime story truly is not enough

## Existing Codebase / Prior Art

- `website/docs/index.md` — already makes the strong public "Built-in failover, load balancing, and exactly-once semantics" claim that M054 must either justify or tighten
- `.gsd/milestones/M053/M053-CONTEXT.md` — the immediate predecessor milestone already flagged load-balancing expectations as the likely next seam after serious starter deploy truth
- `.gsd/milestones/M053/M053-ROADMAP.md` — records the staged deploy, failover, hosted-chain, and docs surfaces that M054 will likely build on rather than replace
- `website/docs/docs/distributed-proof/index.md` — the current public proof map already distinguishes the serious PostgreSQL starter from the retained Fly reference lane, but it does not yet make the balancing model explicit enough
- `examples/todo-postgres/README.md` — the serious clustered starter contract whose real ingress/routing behavior now needs an equally truthful explanation
- `scripts/verify-m053-s02.sh` — current serious clustered starter deploy/failover proof rail that can anchor any balancing follow-through to real artifacts instead of hand-wavy docs
- `.gsd/DECISIONS.md` (`D337`, `D314`) — existing direction already says the target model is platform-agnostic, server-side first, with real follow-through if the story is insufficient

> See `.gsd/DECISIONS.md` for all architectural and pattern decisions — it is an append-only register; read it during planning, append to it during execution.

## Relevant Requirements

- `R123` — the core requirement for this milestone: Mesh must explain current load balancing honestly and implement follow-through if the current server-side story is insufficient
- `R124` — keeps frontend-aware node-selection adapters deferred unless this milestone proves the server-side/runtime story is not enough
- `R060` — constrains the milestone to keep Fly as a proving environment rather than letting Fly-specific behavior become the architecture story

## Scope

### In Scope

- deep dive into how load balancing actually works today across public ingress, Fly proof, and Mesh runtime behavior
- make the one-public-URL, server-side-first contract explicit
- audit existing public load-balancing wording and tighten it if the current proof does not justify the claim
- implement real follow-through if the deep dive shows the current server-side story is not sufficient
- define when frontend-aware adapters would become justified instead of speculative

### Out of Scope / Non-Goals

- making clients, browsers, or frontend SDKs topology-aware by default
- turning the current planning wave into a frontend-first language push
- treating Fly-specific behavior as the Mesh product contract
- pretending shared SQLite durability or stronger clustered semantics than the current starter/proof surfaces actually prove

## Technical Constraints

- the public mental model should stay "one public app URL" unless the deep dive proves that model false or materially incomplete
- server-side replay is acceptable inside that model; the client still should not need node topology or per-node URLs
- Fly can remain the proving environment, but the public explanation must stay platform-agnostic
- any new public wording must map back to real runtime/deploy/verifier evidence rather than appealing to theory alone
- if follow-through work is required, it should land on the real clustered starter/runtime/deploy surfaces, not on proof-app-only seams

## Integration Points

- `website/docs/index.md` — public headline claim surface
- `website/docs/docs/getting-started/clustered-example/` and `website/docs/docs/distributed-proof/` — primary public explanation surfaces for the clustered story
- `examples/todo-postgres/README.md` and the M053 staged deploy/failover verifier chain — serious clustered starter path that must carry truthful balancing explanation or follow-through
- Fly Proxy / `fly-replay` — current proving-environment ingress layer whose behavior must be separated from Mesh runtime-owned behavior
- `meshc cluster status|continuity|diagnostics` — runtime/operator truth surfaces that should explain placement, failover, and execution truth independently of ingress balancing
- frontend or browser clients — external callers that should be able to rely on one public URL without learning node topology

## Open Questions

- Is the honest public claim simply "one public URL; server-side replay is fine," or does some part of the product story need a stronger direct-routing-first claim? — Current thinking: keep the one-public-URL, server-side-replay model as the default unless execution shows a stronger promise is already true and provable
- If the deep dive shows the current story is weak, what is the smallest real follow-through that closes the gap: docs/proof tightening, platform ingress changes, or runtime routing changes? — Current thinking: allow real follow-through and fail closed on docs-only fixes
- Does platform-agnostic truth need a second live proving environment now, or is Fly plus a clear portable model enough for this milestone? — Current thinking: Fly plus model is enough unless the deep dive shows the portable boundary still cannot be made concrete
