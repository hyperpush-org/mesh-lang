# M044: First-Class Clustered Apps & Bounded Auto-Promotion

**Gathered:** 2026-03-28
**Status:** Ready for planning

## Project Description

M044 productizes the M042/M043 runtime-native continuity and failover substrate into a real language/platform feature for ordinary Mesh apps. A clustered app should opt in through `mesh.toml` / app metadata, declare which service/message/work handlers are clustered, and then let Mesh own the clustering behavior behind the scenes. The app author provides business logic for those handlers and ordinary local code everywhere else. They do **not** write placement, promotion, continuity plumbing, or app-specific operator wiring.

The honest line for this milestone is:
- clustered where declared
- ordinary everywhere else

That means M044 is **not** “replicate all server work,” **not** “every function is clustered,” and **not** “routes only.” Routes are one ingress path, but the clustered contract itself belongs at the declared service/message/work-handler boundary.

## Why This Milestone

The substrate is now real, but the product story is still proof-app shaped. `mesh-rt` already owns the lower-level continuity record, authority, promotion, and stale-primary fencing, but ordinary app authors still come in through `cluster-proof`-style seams: Mesh app code still sees stringly `Continuity.* -> Result<String, String>` APIs, `cluster-proof` still owns placement/config/operator translation glue, and `meshc init` still scaffolds only a hello-world app.

This milestone solves the gap between “the runtime can do it” and “a normal Mesh app can use it without becoming a distributed systems project.” It is the right next step because the bounded failover substrate exists now, and the remaining work is to move the user-facing boundary from proof-app folklore to a first-class clustered-app model.

## User-Visible Outcome

### When this milestone is complete, the user can:

- `meshc init --clustered` a new Mesh app, declare clustered service/message/work handlers in `mesh.toml`, run the same binary on two nodes, and submit clustered work without writing explicit clustering logic.
- inspect built-in cluster status through runtime APIs and CLI, kill the primary, and watch the standby auto-promote and continue declared clustered work safely when the runtime can prove the transition is safe.

### Entry point / environment

- Entry point: `meshc init --clustered`, `mesh.toml` clustered opt-in, public clustered-app APIs, built-in runtime/CLI operator surfaces, and the rewritten `cluster-proof` proof app
- Environment: local dev, same-binary two-node proof, CI verifiers, and optional HTTP exposure on top of the runtime truth
- Live dependencies involved: Mesh node transport, discovery, continuity runtime, compiler/typechecker/codegen seams, CLI tooling, optional HTTP helpers, same-image operator proof

## Completion Class

- Contract complete means: clustered mode, declared clustered handlers, typed continuity/authority values, and built-in runtime/CLI operator surfaces all compile and are proven by targeted tests/verifiers instead of proof-app JSON shims.
- Integration complete means: a scaffolded clustered app and `cluster-proof` both run the same binary on multiple nodes using only the public clustered-app model, with runtime-owned placement, continuity, authority, and failover.
- Operational complete means: primary loss, bounded automatic promotion, stale-primary fencing on rejoin, and fail-closed ambiguity handling are exercised under a real destructive verifier path.

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- a freshly scaffolded clustered Mesh app can declare clustered handlers in `mesh.toml`, run the same binary on two nodes, submit clustered work, and inspect built-in cluster status without app-authored clustering logic.
- killing the active primary causes bounded automatic promotion when the runtime’s safety conditions are satisfied, declared clustered work continues on the standby, and the stale primary rejoin path stays fenced/deposed.
- `cluster-proof` has been rewritten onto the same public clustered-app standard with the old explicit clustering path removed, so the proof app is a consumer of the feature instead of the template for it.

## Risks and Unknowns

- The declaration model could leak explicit clustering logic back into app code or overfit HTTP routes instead of the service/message/work-handler boundary — that would fail the “ordinary Mesh app” goal.
- General service/message handling is a wider seam than the current keyed proof path — the compiler/runtime/app-model contract may need more surgery than the existing route-shaped continuity surface suggests.
- Bounded automatic promotion in a two-node primary/standby topology can overclaim safety if the ambiguity rules are not explicit and verifier-backed — “two nodes are running” is not itself a sufficient promotion condition.
- Rewriting `cluster-proof` onto the new standard may expose hidden dependence on stringly APIs, proof-app-specific config, or old operator surfaces that the new public model has to absorb or remove.

## Existing Codebase / Prior Art

- `compiler/mesh-rt/src/dist/continuity.rs` — already contains the typed Rust-side continuity, authority, promotion, and fencing substrate that M044 needs to expose as a first-class Mesh surface.
- `compiler/mesh-typeck/src/infer.rs` — the current `Continuity` Mesh module still returns `Result<String, String>` values, which is the public API seam M044 has to replace.
- `cluster-proof/work_continuity.mpl` — currently parses runtime-owned continuity JSON and translates it into Mesh structs, which is exactly the proof-app glue M044 should eliminate.
- `cluster-proof/work.mpl` — currently computes route selection and owner/replica placement in Mesh code, which is the app-authored placement seam M044 should move into the runtime.
- `cluster-proof/config.mpl` — still carries proof-app-specific env conventions alongside `MESH_*`, which blocks a shared clustered-app config story.
- `compiler/mesh-pkg/src/scaffold.rs` — `meshc init` currently produces only a hello-world project, so M044 needs a real clustered scaffold path.

> See `.gsd/DECISIONS.md` for all architectural and pattern decisions — it is an append-only register; read it during planning, append to it during execution.

## Relevant Requirements

- R049 — productizes the at-least-once keyed continuity contract behind declared clustered handlers.
- R050 — carries replica-backed continuity into the first-class clustered app model.
- R052 — replaces the proof-app env dialect with a standard clustered-app operator/config contract while preserving the same-binary story.
- R061 — makes clustered mode a `mesh.toml` opt-in instead of proof-app glue.
- R062 — removes app-level continuity JSON parsing from the public Mesh surface.
- R063 — locks the boundary that only declared clustered handlers get continuity/failover semantics.
- R064 — moves placement, replication, fencing, authority, and failover fully into the runtime for declared clustered handlers.
- R065 — establishes runtime API first, CLI second, HTTP optional as the built-in operator model.
- R066 — requires `meshc init --clustered` to scaffold the public path.
- R067 — defines bounded automatic promotion as auto-only and fail-closed on ambiguity.
- R068 — requires real primary-loss survival on the declared clustered-handler path.
- R069 — requires a full `cluster-proof` rewrite onto the new standard with the old path removed.
- R070 — moves the primary docs/proof story to “build a clustered Mesh app.”

## Scope

### In Scope

- `mesh.toml` / app-metadata clustered opt-in and declaration of clustered service/message/work handlers
- typed public Mesh-facing continuity, authority, and promotion surfaces
- runtime-owned placement, continuity replication, attempt fencing, authority, and failover for declared clustered handlers
- built-in runtime/CLI operator surfaces with HTTP optional
- `meshc init --clustered` scaffold and a public sample app path
- bounded automatic promotion for the one-primary/one-standby topology
- full `cluster-proof` dogfood rewrite onto the new declared-handler standard
- public docs/verifier closeout that teaches clustered apps above low-level proof folklore

### Out of Scope / Non-Goals

- replicating all server work or every function automatically
- route-only clustering as the core abstraction
- active-active writes or general replicated application state
- consensus-backed global control plane or arbitrary distributed transactions
- manual promotion or operator override in M044
- exactly-once claims
- broader failover topologies than one active primary plus one standby

## Technical Constraints

- clustered execution must stay explicit at the declaration boundary: declared clustered handlers get the distributed contract; undeclared code stays local
- the runtime owns placement, continuity, authority, promotion, and fail-closed behavior; app code must not grow explicit clustering logic back in
- operator truth is runtime API first, CLI second, HTTP optional
- bounded automatic promotion must be auto-only and fail closed on ambiguity
- M044 must preserve the existing honesty boundary: if the runtime cannot prove promotion is safe, it must not promote
- `cluster-proof` must dogfood the new standard fully enough that the old explicit clustering path can be removed from its code

## Integration Points

- `mesh-rt` distributed continuity and node runtime — the execution layer that must own placement, replication, fencing, authority, and failover
- compiler/typechecker/codegen/intrinsics — the public Mesh-facing clustered declarations and typed `Continuity` API have to align across all compile/runtime seams
- `mesh.toml` and `meshc init` — clustered mode activation, declaration model, and scaffold path
- CLI/operator tooling — standard inspection surfaces over the runtime truth
- `cluster-proof` — full dogfood consumer of the new declared clustered-handler model
- docs and verifier scripts — the public clustered-app story and the proof rails must agree mechanically

## Open Questions

- What is the exact declaration syntax for clustered service/message/work handlers in `mesh.toml` and Mesh code? — current thinking: config declares what is clustered and business Mesh code supplies the handler logic, but the exact syntax still needs planning.
- How should the CLI package the built-in operator surfaces? — current thinking: CLI should wrap the runtime truth directly instead of inventing a separate parallel status model.
- How should optional HTTP exposure be provided without making HTTP the real abstraction? — current thinking: HTTP should sit on top of the built-in runtime/CLI truth as an optional helper surface, not define the clustered contract itself.
