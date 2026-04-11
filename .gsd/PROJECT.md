# Project

## What This Is

Mesh is a programming language and backend application platform repository aimed at being trustworthy for real backend and distributed-systems work, not just toy examples. The repo contains the compiler, runtime, formatter, LSP, REPL, package tooling, docs site, package registry, packages website, retained proof fixtures, and split-boundary tooling used to pressure-test the language and hand off into the maintained Hyperpush product repo.

The repo works against a real two-repo sibling workspace: `mesh-lang` keeps the language/toolchain/docs/installers/registry/packages/public-site surfaces, and the sibling product repo (`hyperpush-org/hyperpush`, still surfaced locally through the `hyperpush-mono` workspace path) owns `mesher/`, `mesher/landing/`, and `mesher/client/`. This checkout no longer tracks product source; any local `mesh-lang/mesher` path is compatibility-only and comes from the workspace helper instead of a tracked tree. `WORKSPACE.md` is the maintainer-facing contract for the blessed sibling layout, repo-local `.gsd` remains authoritative instead of yielding to one umbrella workspace tree, and the evaluator-facing scaffold/docs/proof path now hands off through the repo-boundary Hyperpush contract instead of local product-source paths. M055 completed that split-boundary truth reset, and M057 is complete: the repo issues, sibling product issues, and org project board have been reconciled to the actual code and ownership reality that split left behind. The milestone closeout record lives under `.gsd/milestones/M057/` as `M057-VALIDATION.md` and `M057-SUMMARY.md`.

M048 is complete. The repo now ships a default-plus-override executable-entry contract across compiler build/test, `mesh-lsp`, editor hosts, and `meshpkg publish`; installer-backed `meshc update` / `meshpkg update`; truthful editor grammar and init-time Mesh skill guidance for `@cluster` plus both interpolation forms; and one retained closeout rail (`bash scripts/verify-m048-s05.sh`) plus bounded public docs that keep first-contact tooling claims honest.

M049 is complete. The repo no longer teaches `todo-api` onboarding through repo-root proof apps: `meshc init --template todo-api --db postgres <name>` emits the migration-first shared/deployable starter, `meshc init --template todo-api --db sqlite <name>` emits the explicit local single-node starter, generator-owned `examples/todo-postgres` and `examples/todo-sqlite` mirror scaffold output mechanically, repo-root `tiny-cluster/` and `cluster-proof/` have been retired in favor of fixture-backed retained proofs under `scripts/fixtures/clustered/`, and one assembled closeout rail (`bash scripts/verify-m049-s05.sh`) now replays the dual-db scaffold, example parity, public-surface retirement, and retained M039/M045/M047/M048 guardrails into a single retained bundle.

The evaluator-facing public-docs reset and reference-app retirement wave is complete: M050 and M051 closed with Mesher as the maintained deeper reference app, the repo-root `reference-backend/` compatibility tree gone, retained backend-only proof living under `scripts/fixtures/backend/reference-backend/`, `R119` validated, and `bash scripts/verify-m051-s05.sh` as the terminal post-deletion acceptance rail. The milestone closeout record lives under `.gsd/milestones/M051/` (`M051-VALIDATION.md` and `M051-SUMMARY.md`).

M053 is complete. The serious Postgres starter emits a staged deploy bundle, carries a truthful clustered failover rail, and is wired into hosted CI/release proof. `bash scripts/verify-m053-s01.sh`, `bash scripts/verify-m053-s02.sh`, `bash scripts/verify-m053-s03.sh`, and `bash scripts/verify-m053-s04.sh` remain the retained deploy/failover/docs truth chain, with the final milestone record under `.gsd/milestones/M053/`.

M054 is complete. The serious Postgres starter now has a truthful one-public-URL, server-side-first load-balancing story end to end: S01 proved standby-first public `GET /todos`, S02 added runtime-owned continuity request-key follow-through, and S03 aligned homepage metadata, Distributed Proof, starter guidance, OG output, and fail-closed verifiers. The closeout chain lives under `bash scripts/verify-m054-s01.sh`, `bash scripts/verify-m054-s02.sh`, and `bash scripts/verify-m054-s03.sh`, with the final record under `.gsd/milestones/M054/`.

M056's slice work lives in the sibling Hyperpush product repo rather than this checkout. The evaluator-facing `/pitch` route remains the maintained landing artifact there, but `mesh-lang` keeps only the repo-boundary handoff and split-contract verification that point into that product-owned surface.

The planning problem behind M057 was external rather than code-level: the local code and split-boundary contract had run ahead of the public GitHub tracker state. M057 is now complete, with canonical repo mutation results under `.gsd/milestones/M057/slices/S02/` and canonical project-board results under `.gsd/milestones/M057/slices/S03/` plus retained `.tmp/m057-s02/verify/` and `.tmp/m057-s03/verify/` replay bundles.

M059 is in flight and S01-S03 are now complete. The sibling product dashboard now runs as the canonical `../hyperpush-mono/mesher/client/` TanStack Start/Vite package instead of the old `frontend-exp` runtime path, keeps the visible dashboard shell and mock-data behavior, preserves the package-local `npm run dev`, `npm run build`, `npm run start`, `npm run test:e2e:dev`, and `npm run test:e2e:prod` contract, and has the surrounding machine-checked CI/README/Dependabot/root-Playwright surfaces rewired to `mesher/client`. The remaining milestone work is S04 equivalence proof and final direct operational cleanup.

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
- in-progress frontend migration work under the sibling product repo: `../hyperpush-mono/mesher/client/` is now the canonical TanStack Start/Vite dashboard package with a pathless `_dashboard` layout, real file routes for the current top-level sections, a package-local Node bridge server over built `dist/` output, and repo-owned dev/prod Playwright parity rails; only M059/S04 equivalence proof and final stale-reference cleanup remain

Recent distributed-runtime state:
- M039 proved automatic cluster formation, truthful membership, runtime-native internal balancing, and single-cluster degrade/rejoin on a narrow proof app
- M042 moved single-cluster keyed continuity into `mesh-rt` behind a Mesh-facing `Continuity` API
- M043 proved cross-cluster primary/standby continuity, bounded promotion, and packaged same-image failover/operator rails
- M044 productized clustered apps: manifest opt-in, runtime-owned declared-handler execution, built-in read-only operator/CLI surfaces, `meshc init --clustered`, bounded automatic promotion/recovery, and a rewritten `cluster-proof` on the public clustered-app contract
- M045 simplified the clustered example story around runtime-owned bootstrap, runtime-chosen remote execution, automatic failover, and scaffold-first docs
- M046 closed the route-free clustered proof wave: `meshc init --clustered` plus the internal `scripts/fixtures/clustered/tiny-cluster` and `scripts/fixtures/clustered/cluster-proof` proofs now share one tiny `1 + 1` clustered-work contract, and the authoritative closeout rail is `bash scripts/verify-m046-s06.sh`
- M047 completed the public cutover to source-first `@cluster`, carried replication counts through runtime truth, shipped `HTTP.clustered(...)`, and updated the Todo scaffold, docs, and closeout rails around that shipped route wrapper

Public docs and repo teaching surfaces are now much more truthful after M048-M051, but later milestones still need to keep the simple public story ahead of retained proof detail:
- the default-plus-override `[package].entrypoint` contract now spans compiler build, test discovery, LSP, editor hosts, and `meshpkg publish`; first-contact docs also point at the retained `bash scripts/verify-m048-s05.sh` closeout rail and keep `main.mpl` as the simple default while documenting override entries such as `lib/start.mpl`
- the reset wave includes installer-backed `meshc update` / `meshpkg update`, bounded VS Code same-file-definition wording, manifest-first editor proof, and retained parity rails for `@cluster`, both interpolation forms, and clustered-runtime teaching truth
- M050 and M051 completed the docs-graph reset, first-contact pages, secondary proof surfaces, retained backend fixture, and post-deletion closeout rail so readers now enter through scaffold/examples-first material while Mesher stays maintainer-facing
- the broader Hyperpush landing story still has positioning cleanup left outside `/pitch`, but the language repo keeps only the evaluator-facing `/pitch` contract and the repo-boundary handoff into the sibling product repo
- M057 completed the tracker truth reset across the two repos and org project #1
- M059/S01-S03 completed the framework/path migration core: the product app no longer depends on Next.js on the active runtime path, the canonical dashboard path is now `mesher/client`, current top-level dashboard sections are real TanStack routes with URL-backed navigation and preserved shell-owned Issues state, and repo-owned Playwright plus root-harness checks confirm dev and built-production deep-link parity without backend-integration drift

## Architecture / Key Patterns

- Rust workspace under `compiler/` with separate crates for parser, type checker, codegen, runtime, formatter, LSP, CLI, REPL, package tooling, and package manager code
- backend-first proof surfaces through narrow reference apps and shell verifiers, not marketing-only examples
- proof-first dogfooding: reproduce a real runtime/platform limitation, fix it at the correct layer, then prove the repaired path end to end
- explicit honesty boundaries when behavior is genuinely environment-specific; avoid claiming portability or automation that the runtime does not really own
- assembled closeout verifiers own a fresh `.tmp/<slice>/verify` bundle and retain delegated subrails by copying their verify trees plus bundle pointers, rather than sharing or mutating lower-level `.tmp/.../verify` directories directly
- current clustered runtime surface lives primarily in `compiler/mesh-rt/src/dist/`, `compiler/mesh-codegen/`, `compiler/mesh-typeck/`, and `compiler/meshc/`, with user-facing docs in `website/docs/docs/distributed/` and scaffold generation in `compiler/mesh-pkg/src/scaffold.rs`
- clustered HTTP routes now reuse the same declared-handler seam as ordinary clustered work: compiler lowering rewrites `HTTP.clustered(...)` to deterministic `__declared_route_<runtime_name>` bare shims, router registration reverse-maps those shims onto declared-handler runtime metadata, and continuity/operator views stay keyed by the real handler runtime name rather than the shim symbol
- for public evaluator-facing surfaces, keep the simple path simpler than retained proof rails: scaffold/examples first, Mesher as the deeper real app, and verifier detail out of the primary docs story
- tracker-reconciliation work now follows a fail-closed artifact chain: S01 produced canonical snapshots/evidence/ledger truth, S02 turned that ledger into checked repo mutation plan/results artifacts with canonical issue mapping, and S03 completed the org-project realignment from those persisted mappings plus live repo truth instead of re-deriving state from stale board text
- M059 established the frontend migration pattern: keep one visible dashboard shell mounted through TanStack Start route files, decompose top-level sections into real file routes under a pathless layout while preserving shell-owned transient and Issues state, preserve the external command contract even if the internal production runner changes, move the canonical package wholesale when the route/state work is already proven, and keep repo-owned dev/prod parity rails plus a root harness when npm CLI forwarding from `mesh-lang` is not truthful enough

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
- [ ] M059: Frontend Framework Migration to TanStack Start — TanStack route migration and canonical path cutover are complete through S03; only S04 equivalence proof and final direct operational cleanup remain
- [ ] M035: Test Framework Hardening — get Mesh's testing story ready to test `mesher` thoroughly during development
- [ ] M037: Package Experience & Ecosystem Polish — improve the package manager experience, website-first, once the underlying trust path is proven
