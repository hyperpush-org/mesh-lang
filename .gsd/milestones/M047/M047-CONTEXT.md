# M047: Cluster Declaration Reset & Clustered Route Ergonomics

**Gathered:** 2026-03-31
**Status:** Ready for planning

## Project Description

M047 resets Mesh's clustered authoring model so clustering reads like a normal language feature instead of a narrow proof-only mechanism. The current repo is technically capable enough to prove route-free clustered work, startup triggering, failover, and runtime-owned status truth, but the public syntax still feels wrong: parser, validation, scaffold, docs, examples, and proof rails are still shaped around `clustered(work)`, hardcoded declared-work names, and a specialized clustered-work story instead of a cleaner source-first `@cluster` model.

The user's direction should be preserved literally where it matters: the current syntax is not liked, clustering should stay a general function capability, clustered routes should be ergonomic in router chains, and the new scaffold should not feel like a proof app. They explicitly called out the failure modes to avoid: **clustering is technically present but not obvious**, **too much boilerplate**, and **it looks like a proof app instead of a starting point**.

## Why This Milestone

M046 proved the runtime-owned clustered execution story strongly enough that the next honest weakness is the programming model itself. If clustering still reads like a special declaration for one narrow proof path, Mesh has not actually made distribution feel first-class even if the runtime underneath is solid.

This milestone matters now because the user wants the syntax and the examples to line up with what Mesh already claims to own. The repo already has proof that clustered work, startup triggering, and route-free failover can be language/runtime-owned. The next move is to make that ownership obvious in the source language, in HTTP routing, in a container-ready starting-point scaffold users can actually run, and in the examples users actually copy.

## User-Visible Outcome

### When this milestone is complete, the user can:

- write `@cluster` or `@cluster(3)` on a function and have Mesh treat it as the clustered execution boundary without the old `clustered(work)` ceremony
- write clustered HTTP routes with wrapper syntax like `HTTP.on_get("/todos", HTTP.clustered(handle_list_todos))` or `HTTP.on_post("/todos", HTTP.clustered(3, handle_create_todo))`, then scaffold a simple SQLite Todo API with a complete Dockerfile that demonstrates the same model with low boilerplate

### Entry point / environment

- Entry point: ordinary Mesh source files, `meshc init` scaffolding, `meshc build`, Docker build/run, built-in HTTP routing, and existing `meshc cluster ...` inspection surfaces
- Environment: local dev, compiler/LSP tests, generated scaffold projects, route-free proof packages, real HTTP route wiring in repo-owned examples, and containerized scaffold smoke paths
- Live dependencies involved: parser/typechecker/codegen/runtime clustering surfaces, HTTP router/runtime lowering, existing clustered proof packages, SQLite-backed scaffolded app paths, Docker packaging, and public docs/verifier rails

## Completion Class

- Contract complete means: `@cluster` / `@cluster(N)` is the public clustered function syntax, counts mean replication counts with default `2`, route wrapper clustering lowers onto the same general clustered function model, and the scaffold includes a complete Dockerfile
- Integration complete means: clustered functions, clustered routes, route-free proof apps, and the new scaffold all share one coherent clustered execution story instead of separate special-case models
- Operational complete means: the scaffolded Todo API and repo-owned clustered examples can actually run/build on the new syntax and remain inspectable through the existing runtime/tooling surfaces without reintroducing old proof-app seams, including container build success for the scaffolded app

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- one ordinary clustered function can be declared with `@cluster` or `@cluster(3)`, defaults and explicit counts are visible in execution planning/runtime truth, and the old `clustered(work)` public surface is gone from canonical examples/docs/tests
- one HTTP router chain can opt into clustering through `HTTP.clustered(...)` wrapper syntax and execute the route handler as the clustered boundary while downstream function calls run naturally inside that execution
- `meshc` can scaffold a simple SQLite Todo API with several routes, actors, rate limiting, and obvious clustered route syntax, and the generated project includes a complete Dockerfile that builds and runs without reading like a proof harness

## Risks and Unknowns

- The current parser, AST, manifest/planning logic, scaffold, docs, examples, and proof rails are all shaped around the narrow `clustered(work)` marker — this is a real cross-repo cutover, not a local syntax alias.
- The current clustered proof path still leans on hardcoded declared-handler names like `Work.execute_declared_work`; M047 needs a cleaner clustered function naming/registration story that works for both ordinary functions and route handlers.
- HTTP routing today accepts plain handler functions through `HTTP.on_get` / `HTTP.on_post` and runtime lowering paths that were not designed around clustered-route wrappers. The wrapper form is product-right, but the compiler/runtime seam for it still has to be made real.
- The scaffold can still fail even if the runtime works if it ends up visually noisy, clustered in ways that are technically present but not obvious, missing a real container story, or shaped too much like a proof app instead of a starting point.

## Existing Codebase / Prior Art

- `tiny-cluster-prefered/mesh.toml`, `tiny-cluster-prefered/add.mpl`, `tiny-cluster-prefered/lib/subtract.mpl` — the preferred syntax direction captured in-repo: `@cluster`, `@cluster(3)`, and module/function style targets
- `compiler/mesh-parser/src/parser/items.rs` and `compiler/mesh-parser/src/ast/item.rs` — current parser/AST shape, explicitly hardcoded to `clustered(work)`
- `compiler/mesh-pkg/src/manifest.rs` — current clustered declaration planning/validation, still centered on manifest entries plus the narrow source marker
- `compiler/mesh-pkg/src/scaffold.rs` — current clustered scaffold, which still teaches the old shape and hardcoded declared-work naming
- `compiler/mesh-typeck/src/builtins.rs`, `compiler/mesh-codegen/src/mir/lower.rs`, `compiler/mesh-rt/src/http/router.rs` — current HTTP routing/type/lowering/runtime seams that route-local clustering will have to pass through honestly
- `reference-backend/api/router.mpl` and `mesher/main.mpl` — real existing router-chain usage that shows why route-local clustering needs an elegant wrapper instead of awkward handler indirection
- `tiny-cluster/` and `cluster-proof/` — current repo-owned clustered examples that prove the runtime but still encode the old public syntax

> See `.gsd/DECISIONS.md` for all architectural and pattern decisions — it is an append-only register; read it during planning, append to it during execution.

## Relevant Requirements

- R097 — replace `clustered(work)` with `@cluster` / `@cluster(N)`
- R098 — make counts mean replication counts with default `2`
- R099 — keep clustering as a general function capability, not an HTTP-only feature
- R100 — support route-local clustering through `HTTP.clustered(...)` wrappers
- R101 — make the route handler the clustered boundary while keeping downstream calls natural
- R102 — hard-cut the old public syntax instead of keeping both models alive
- R103 — dogfood the new clustered model across repo-owned examples and proof rails
- R104 — scaffold a simple SQLite Todo API with clustered routes, actors, rate limiting, several routes, and a complete Dockerfile
- R105 — keep clustering obvious and boilerplate low so the scaffold feels like a starting point
- R106 — teach one coherent source-first clustered model to both new and existing Mesh users

## Scope

### In Scope

- replacing the public clustered function syntax with `@cluster` / `@cluster(N)`
- making replication count semantics explicit and defaulting omitted counts to `2`
- preserving clustering as a general function capability beyond HTTP
- adding route-local clustered wrapper syntax for router chains
- migrating repo-owned clustered examples, scaffold output, docs, and proof rails onto the new source-first model
- generating a simple SQLite Todo API scaffold with actors, rate limiting, several routes, and a complete Dockerfile

### Out of Scope / Non-Goals

- preserving `.toml` as a second clustered declaration surface
- keeping `clustered(work)` as a long-term coequal public syntax
- making clustering fully implicit for arbitrary code paths without an explicit clustered function or route wrapper boundary
- turning the new scaffold into a heavier mini-platform with auth, external integrations, or product-scale complexity just to make clustering look useful

## Technical Constraints

- clustering should stay source-first: one general clustered function model plus route-local wrapper sugar
- route-level clustering should use wrapper style (`HTTP.clustered(...)`) rather than a separate clustered verb API for every HTTP method
- replication count is the public meaning of the numeric argument; omitted count means `2`
- the route handler is the clustered boundary for clustered HTTP; ordinary downstream function calls simply run inside that execution
- the scaffold should use SQLite in a simple way and prioritize visible syntax over maximal infrastructure depth
- the scaffold must include a complete Dockerfile so the generated project has a believable container-ready starting point
- repo-owned examples and proof rails must stop teaching the old public syntax if the new model is to be believable

## Integration Points

- parser / AST — to recognize `@cluster` and remove the old `clustered(work)` surface from the public grammar
- clustered declaration planning — to map source decorators onto execution planning without a parallel manifest contract
- codegen/runtime clustering — to derive runtime registration/execution behavior for general clustered functions and route-wrapper lowering
- HTTP router surface — to support `HTTP.clustered(...)` wrappers honestly through typeck/lowering/runtime paths
- scaffold generation — to create the new Todo starting point, Dockerfile, and migrate away from the old clustered scaffold story
- proof packages and docs — to migrate `tiny-cluster/`, `cluster-proof/`, README/docs snippets, and verifier rails onto the new model

## Open Questions

- What is the cleanest runtime-registration story once `@cluster` is no longer tied to the old `Work.execute_declared_work` public shape? — current thinking: derive from normal module/function identity instead of carrying a bespoke source helper like `declared_work_runtime_name()`.
- Should the route-wrapper implementation stay as one generic `HTTP.clustered(...)` primitive or grow method-specific helpers later? — current thinking: prove the wrapper primitive first and defer broader route ergonomics until real usage demands them.
- How small can the SQLite Todo scaffold stay while still making clustering obvious, shipping a believable Dockerfile, and not feeling toy-like? — current thinking: several real routes, actor-backed work, simple rate limiting, straightforward persistence, and one complete container path are enough; do not let the app expand into a proof harness or mini-platform.
