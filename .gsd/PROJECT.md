# Project

## What This Is

Mesh is a programming language and backend application platform repository aimed at being trustworthy for real backend and distributed-systems work, not just toy examples. The repo contains the compiler, runtime, formatter, LSP, REPL, package tooling, docs site, package registry, packages website, retained proof fixtures, and split-boundary tooling used to pressure-test the language and hand off into the maintained Hyperpush product repo.

The repo works against a real two-repo sibling workspace: `mesh-lang` keeps the language/toolchain/docs/installers/registry/packages/public-site surfaces, and the sibling product repo (`hyperpush-org/hyperpush`, surfaced locally through `../hyperpush-mono`) owns `mesher/`, `mesher/landing/`, and `mesher/client/`. This checkout does not own product source; any local `mesh-lang/mesher` path is compatibility-only and comes from the workspace helper instead of a tracked tree. `WORKSPACE.md` is the maintainer-facing contract for the blessed sibling layout, and repo-local `.gsd` remains authoritative instead of yielding to one umbrella workspace tree.

The split-boundary and public-surface reset milestones are complete: M048 through M055, M057, M059, and M060 are closed with retained proof and repo-boundary handoff intact.

## Core Value

If Mesh claims it can cluster, route work, survive node loss, and report truthful runtime status, those claims must be proven through small docs-grade examples where the language/runtime owns the magic instead of the example app reimplementing distributed behavior — including the syntax users actually write.

The public Mesh story should stay honest: Mesh is a general-purpose language, but its strongest proof surface and clearest value are fault-tolerant distributed systems and backend workloads.

## Current State

Mesh already ships a broad backend-oriented stack:
- Rust workspace crates under `compiler/` for lexing, parsing, type checking, code generation, runtime, formatter, LSP, REPL, package tooling, and CLI commands
- native compilation to standalone binaries
- runtime support for actors, supervision, HTTP, WebSocket, JSON, database access, migrations, files, env, crypto, datetime, and collections
- a distributed runtime surface with node start/connect/list/monitor, remote spawn/send, continuity, authority, and clustered-app tooling
- retained proof surfaces: the backend-only fixture under `scripts/fixtures/backend/reference-backend/`, the clustered fixtures under `scripts/fixtures/clustered/tiny-cluster` plus `scripts/fixtures/clustered/cluster-proof`, and the split-boundary wrappers that hand off into the maintained Hyperpush product repo
- a real package registry service in `registry/`, a public packages website in `packages-website/`, a docs site in `website/`, and repo-owned split-boundary docs/install/release helpers for the sibling product repo
- editor surfaces including the VS Code extension and repo-owned Neovim pack
- the frontend migration wave is complete in the sibling product repo: `../hyperpush-mono/mesher/client/` is the canonical TanStack Start/Vite dashboard package, the parity suite covers route/runtime behavior in dev and prod, and direct maintainer guidance now points at `mesher/client`

M060 is complete and verified.

The canonical `mesher/client` dashboard now has a proven assembled backend-backed shell:
- S01 made the Issues route real through same-origin `/api/v1` reads, a typed live-overlay adapter, explicit live/derived/fallback source markers, and destructive toast feedback for failure paths.
- S02 made dashboard summaries and existing issue actions real, with provider-owned mutation/refetch orchestration and deterministic seeded proof for live issue workflows.
- S03 made the admin and ops surfaces truthful where the backend already has routes: Alerts acknowledge/resolve flows, Settings general/storage reads and writes, API key list/create/revoke, alert-rule list/create/toggle/delete, Team list/add/role/remove through org-slug resolution, and explicit `data-*` state markers plus mounted Radix toasts for read/write failures.
- S04 closed the milestone with the canonical assembled-shell proof: a route-map-driven seeded walkthrough spanning every current dashboard route, shared same-origin runtime diagnostics in `mesher/client/tests/e2e/live-runtime-helpers.ts`, explicit sparse issue-detail state markers, serial Playwright execution for the shared seeded runtime, and README-documented dev/prod verification rails.

M061 is in progress.
- S01 is complete as a deliverable slice: `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` is now the canonical maintainer-facing top-level route inventory, backed by `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs`, `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`, and the retained wrapper `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh`.
- The structural route-inventory contract passes and the retained verifier emits phase/status artifacts under `../hyperpush-mono/mesher/.tmp/m061-s01/verify-client-route-inventory/`.
- The remaining known limitation is in the retained prod admin/ops alert lifecycle rail: the wrapper can still fail because the just-created seeded alert is not always observed quickly enough in `/api/v1/projects/default/alerts`. The failure is named and retained rather than silent.

## Architecture / Key Patterns

- Rust workspace under `compiler/` with separate crates for parser, type checker, codegen, runtime, formatter, LSP, CLI, REPL, package tooling, and package manager code
- backend-first proof surfaces through narrow reference apps and shell verifiers, not marketing-only examples
- proof-first dogfooding: reproduce a real runtime/platform limitation, fix it at the correct layer, then prove the repaired path end to end
- explicit honesty boundaries when behavior is genuinely environment-specific; avoid claiming portability or automation that the runtime does not really own
- assembled closeout verifiers own a fresh `.tmp/<slice>/verify` bundle and retain delegated subrails by copying their verify trees plus bundle pointers, rather than sharing or mutating lower-level `.tmp/.../verify` directories directly
- current clustered runtime surface lives primarily in `compiler/mesh-rt/src/dist/`, `compiler/mesh-codegen/`, `compiler/mesh-typeck/`, and `compiler/meshc/`, with user-facing docs in `website/docs/docs/distributed/` and scaffold generation in `compiler/mesh-pkg/src/scaffold.rs`
- for public evaluator-facing surfaces, keep the simple path simpler than retained proof rails: scaffold/examples first, Mesher as the deeper real app, and verifier detail out of the primary docs story
- M059 established and closed the frontend migration pattern: keep one visible dashboard shell mounted through TanStack Start route files, decompose top-level sections into real file routes while preserving shell-owned transient state, keep the external command contract even if the internal production runner changes, move the canonical package only after parity proof passes, and keep repo-owned dev/prod parity rails plus a root harness
- M060 established and closed the live-shell wiring pattern for product-backed dashboard work: keep browser traffic on same-origin `/api/v1`, adapt Mesher payloads through typed live/mock overlays instead of weakening the existing UI contract, keep mutation orchestration and post-write refetch inside slice-owned state providers, expose source/state through stable `data-*` markers for proof, leave unsupported controls visibly present but explicitly non-live, surface backend read/write failures through the mounted toast path instead of inventing a new error UX, and close the milestone with one route-map-driven seeded walkthrough plus shared runtime diagnostics instead of disconnected route proofs
- M061/S01 established the maintainer-inventory pattern for client truth work: keep the canonical top-level route list in `dashboard-route-map.ts`, place the human-readable inventory beside `mesher/client`, guard it with a fail-closed parser/test instead of a second registry, and expose one retained wrapper that reuses existing seeded runtime rails with explicit phase logs
- for deterministic admin/ops proof, seed canonical Postgres rows directly first and let the Playwright harness boot its own backend against that DB rather than trusting whichever Mesher might already be listening on `:18180`
- for split-workspace Playwright execution from `mesh-lang`, pass the sibling package's explicit `--config` path; relying on cwd inference causes config/suite resolution drift

## Capability Contract

See `.gsd/REQUIREMENTS.md` for the explicit capability contract, requirement status, and coverage mapping.

## Milestone Sequence

- [x] M028: Language Baseline Audit & Hardening — prove the first honest API + DB + migrations + jobs backend path
- [x] M029: Mesher & Reference-Backend Dogfood Completion — fix formatter corruption and complete the dogfood cleanup wave
- [x] M031: Language DX Audit & Rough Edge Fixes — retire real dogfood rough edges through compiler and parser fixes
- [x] M032: Mesher Limitation Truth & Mesh Dogfood Retirement — audit workaround folklore, fix real blockers in Mesh, and dogfood those repairs back into `mesher/`
- [x] M033: ORM Expressiveness & Schema Extras — strengthen the neutral data layer, add PG-first extras now, and leave a clean path for SQLite extras later
- [x] M034: Delivery Truth & Public Release Confidence — harden CI/CD, prove the package manager end to end, and make the public release path trustworthy instead of artifact-only
- [x] M036: Editor Parity & Multi-Editor Support — make editor support match real Mesh syntax and give at least one non-VSCode editor a first-class path
- [x] M038: Fix Windows MSVC Build — repair the hosted Windows release lane so the shipped compiler path is trustworthy
- [x] M039: Auto-Discovery & Native Cluster Balancing — prove discovery, truthful membership, runtime-native internal balancing, and single-cluster failure/rejoin on a narrow proof app
- [x] M042: Runtime-Native Distributed Continuity Core — move single-cluster distribution, replication, and keyed continuity into `mesh-rt` behind a simple Mesh-facing API
- [x] M043: Runtime-Native Cross-Cluster Disaster Continuity — extend the same runtime-owned continuity model across primary/standby clusters
- [x] M044: First-Class Clustered Apps & Bounded Auto-Promotion — turn runtime continuity/failover into the default productized clustered-app model for ordinary Mesh services
- [x] M045: Language-Owned Clustered Example Simplification — make the primary clustered example tiny, docs-grade, and fully language/runtime-owned instead of proof-app-shaped
- [x] M046: Language-Owned Tiny Cluster Proofs — make clustered work auto-triggered, decorator-declarable, route-free, and equally proven through `meshc init --clustered`, `tiny-cluster/`, and rebuilt `cluster-proof/`
- [x] M047: Cluster Declaration Reset & Clustered Route Ergonomics — replace `clustered(work)` with `@cluster`, reset canonical examples/scaffolds to ordinary `@cluster` function names, continue the clustered-route wrapper work honestly, and ship a clear SQLite Todo scaffold with a complete Dockerfile that makes clustering obvious without looking like a proof app
- [x] M048: Entrypoint Flexibility & Tooling Truth Reset — make entrypoints configurable, add toolchain self-update, and align editors plus init-time skills with the current language/runtime contract
- [x] M049: Scaffold & Example Reset — support SQLite-local and Postgres-clustered scaffolds, generate checked-in examples, and replace proof-app-shaped public teaching surfaces
- [x] M050: Public Docs Truth Reset — make docs evaluator-facing, remove proof-maze public material, and re-test commands and code samples one by one
- [x] M051: Mesher as the Living Reference App — complete; `reference-backend/` is retired, Mesher is the maintained deeper reference app, and `bash scripts/verify-m051-s05.sh` is the post-deletion acceptance rail
- [x] M053: Deploy Truth for Scaffolds & Packages Surface — prove the Postgres starter and packages surfaces through CI and real deployment evidence while keeping the public contract platform-agnostic
- [x] M054: Load Balancing Deep Dive & Runtime Follow-through — explain current balancing honestly and implement follow-through if the existing server-side story is not sufficient
- [x] M055: Multi-Repo Split & GSD Workflow Continuity — split language and product ownership cleanly without breaking the truthful handoff chain
- [x] M056: Interactive Pitch Deck Page — ship the evaluator-facing `/pitch` route in the product repo and retain the repo-boundary handoff in `mesh-lang`
- [x] M057: Cross-Repo Tracker Reconciliation — align `mesh-lang`, `hyperpush`, and org project #1 to the actual code and ownership state
- [x] M059: Frontend Framework Migration to TanStack Start — replaced the active Next.js dashboard runtime with TanStack Start, moved the canonical package to `mesher/client`, and closed the migration with passing parity proof plus updated operational guidance
- [x] M060: Mesher Client Live Backend Wiring — connected the canonical dashboard shell to the seeded Mesher backend through same-origin reads/writes and closed the work with passing seeded dev/prod full-shell proof across every current dashboard route
- [ ] M061: Mesher Client Mock Truth & Backend Gap Map — document exactly what in `mesher/client` is still mocked, mixed, or live so backend expansion can follow the client’s current promises without re-auditing the shell
- [ ] M035: Test Framework Hardening — get Mesh's testing story ready to test `mesher` thoroughly during development
- [ ] M037: Package Experience & Ecosystem Polish — improve the package manager experience, website-first, once the underlying trust path is proven
