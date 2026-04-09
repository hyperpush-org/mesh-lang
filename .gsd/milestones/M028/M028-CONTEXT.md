# M028: Language Baseline Audit & Hardening

**Gathered:** 2026-03-23
**Status:** Ready for planning

## Project Description

Mesh is being planned as a production-trustworthy general-purpose language with a lean toward server/backend code. The user does not want a narrow feature sprint. They want Mesh to keep improving with features, fixes, and more tests, but only after taking a deep look into what it already offers and making sure the basics exist first.

The target is broad: “all types of backend code,” with the first deep proof shape being **API + DB + migrations + background jobs**. The comparison target is Elixir because it is Mesh’s closest relative in spirit, but the early goal is not ecosystem parity. The early goal is to make Mesh as good or better than Elixir first on **easier deployment, raw performance, and better DX**.

## Why This Milestone

Mesh already has a broad backend-looking surface in the repo: compiler, runtime, actors, supervision, HTTP, WebSocket, DB access, migrations, formatter, LSP, test runner, package tooling, registry, and dogfooded applications. The risk is that this surface can read as feature-complete while still leaving a trust gap between what already exists and what someone would trust for “a real production app backend in any capacity.”

This milestone exists to make “everything a production ready language needs to have” concrete enough to plan, then prove the first serious backend path honestly. The user explicitly said two failure modes would be unacceptable: **“concurrency exists but isn’t trustworthy”** and **“docs/examples don’t prove real use.”** M028 is where those two objections start getting retired.

## User-Visible Outcome

### When this milestone is complete, the user can:

- point to one real Mesh backend path — API + DB + migrations + background jobs — and say it works end-to-end with honest proof
- build and ship that backend as a native binary with a boring documented workflow closer to a Go app than to a fragile language stack

### Entry point / environment

- Entry point: `meshc build <reference-backend>` plus the reference backend’s run command and documentation path
- Environment: local dev, CI, and production-like native-binary deployment flow
- Live dependencies involved: HTTP server, database, migration runner, background jobs/services, native OS process lifecycle

## Completion Class

- Contract complete means: the production-baseline capability contract exists, the milestone slices are delivered, and the proof surfaces are documented and checkable
- Integration complete means: one reference Mesh backend actually exercises compiler → runtime → HTTP → DB → migrations → background jobs together
- Operational complete means: build, startup, migration apply, job supervision, failure visibility, and binary deployment smoke verification all work as a coherent story

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- a reference Mesh backend with API + DB + migrations + background jobs can be built and run from the repo as a native binary
- the reference backend survives meaningful failure/recovery scenarios strongly enough that concurrency does not merely exist — it is trustworthy
- the docs/examples and deployment path are strong enough that an outside backend engineer can see real use rather than toy-only evidence

## Risks and Unknowns

- Broad claimed capability may hide unwired or weakly-proven paths — the milestone could drift into abstract cleanup unless it stays anchored to one canonical backend app
- Concurrency may “exist but isn’t trustworthy” under crash/restart conditions — that would undercut the comparison to Elixir immediately
- Tooling gaps may be deep enough that DX still feels fragile even if the backend runtime works — that would weaken one of Mesh’s intended wins
- Docs/examples may lag reality — if they still do not prove real use, the milestone will not actually improve external credibility

## Existing Codebase / Prior Art

- `compiler/meshc/src/main.rs` — best high-level map of the compiler CLI, build pipeline, test runner, formatter, REPL, LSP, and migration entrypoints
- `compiler/mesh-rt/src/lib.rs` — backend-heavy runtime surface including actors, supervision, HTTP, DB, migrations, JSON, files, env, and collections
- `compiler/meshc/src/test_runner.rs` — current Mesh test runner, including the still-stubbed `--coverage` path
- `compiler/mesh-fmt/src/lib.rs` — formatter implementation with explicitly documented known limitations
- `compiler/meshc/tests/tooling_e2e.rs` — current tooling E2E coverage, which is still shallow in places such as LSP proof
- `compiler/meshc/tests/e2e_stdlib.rs` — broad stdlib/runtime proof with some ignored/manual backend cases
- `mesher/main.mpl` — real repo-local Mesh backend application proving the language is already being dogfooded in non-trivial backend code
- `README.md` — public-facing claims about Mesh’s backend/runtime/tooling story and performance positioning

> See `.gsd/DECISIONS.md` for all architectural and pattern decisions — it is an append-only register; read it during planning, append to it during execution.

## Relevant Requirements

- R001 — makes the production baseline explicit instead of vague
- R002 — proves the canonical backend path end-to-end
- R003 — hardens HTTP/DB/migrations under real verification
- R004 — proves concurrency and supervision are trustworthy under failure
- R005 — pushes the native deployment story toward “boring and reliable”
- R006 — raises the daily-driver tooling bar for backend work
- R008 — turns docs/examples into honest proof surfaces
- R009 — ensures Mesh is dogfooded through a real backend, not only fixtures

## Scope

### In Scope

- defining the production-readiness baseline for Mesh in a way that can be checked
- proving one canonical backend path deeply: API + DB + migrations + background jobs
- fixing the most credibility-damaging gaps exposed by that backend path
- improving the deployment, tooling, and documentation surfaces needed to make the proof believable
- using real repo code and patterns rather than inventing a disconnected greenfield story

### Out of Scope / Non-Goals

- frontend-first language planning
- broad new syntax/feature expansion ahead of backend trust proof
- trying to beat Elixir on ecosystem breadth during M028
- calling Mesh production-ready based only on claims, benchmarks, or toy examples

## Technical Constraints

- Work from the existing brownfield codebase and established compiler/runtime/tooling crates rather than assuming a greenfield rewrite
- If the canonical backend path exposes a blocking Mesh limitation, fix the limitation in Mesh rather than papering over it in docs or examples
- Keep the first deployment story native-binary and boring; do not depend on a heavyweight runtime-manager narrative to explain basic deployability
- Tie proof to real commands, real runtime behavior, and real docs surfaces, not only static artifact existence

## Integration Points

- `meshc` CLI — build, test, fmt, repl, lsp, migrate, and diagnostic entrypoints that define the developer workflow
- Mesh runtime — actors, supervision, HTTP, DB, migrations, JSON, files, env, and background job behavior
- Reference backend app — the concrete backend used to prove the golden path and expose real friction
- Docs/examples surface — README/docs/reference content that must become an honest proof surface for external evaluators

## Open Questions

- Should the canonical reference backend for M028 narrow onto a focused new backend app, or should it deliberately sharpen an existing dogfooded app such as `mesher/`? — Current thinking: decide during planning based on which route gives the strongest honest proof with the least incidental complexity.
- Which currently ignored/manual backend proof paths should be pulled into M028 versus left for later milestones? — Current thinking: include whichever ones directly block trust in the canonical backend path.
- How much package/LSP/coverage depth can fit into M028 without starving the golden path itself? — Current thinking: include enough to make daily backend work credible, then leave broader tooling/package maturity for M030.
