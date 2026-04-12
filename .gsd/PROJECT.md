# Project

## What This Is

Mesh is a programming language and backend application platform repository focused on truthful proof for real backend and distributed-systems work. This repo owns the compiler, runtime, formatter, LSP, REPL, package tooling, docs site, registry, packages website, retained proof fixtures, and split-boundary tooling used to hand off into the maintained Hyperpush product repo.

This checkout is not a monorepo. `mesh-lang` owns language/toolchain/docs/installers/registry/packages/public-site surfaces. The sibling product repo (`../hyperpush-mono`) owns `mesher/`, `mesher/client/`, and `mesher/landing/`. Local `mesh-lang/mesher` is compatibility-only and should not be treated as owned source.

## Core Value

If Mesh claims it can cluster, route work, survive node loss, and report truthful runtime status, those claims must be proven through small docs-grade examples and real maintained product surfaces where the language/runtime owns the behavior instead of the app reimplementing it.

## Current State

Mesh already ships a broad backend-oriented stack across the Rust workspace under `compiler/`, native compilation, runtime support for HTTP/WebSocket/DB/migrations/files/env/crypto/datetime/collections, clustered runtime surfaces, retained proof fixtures, the registry and packages website, public docs, and editor integrations.

Completed recent milestones:
- M048 through M055, M057, M059, and M060 are closed.
- M060 closed the Mesher client live-backend wiring milestone with proven same-origin `/api/v1` reads/writes, live/mock overlay adapters, state-provider-owned mutations, route-map-driven proof, and documented dev/prod verification rails.

M061 is in progress.
- S01 is complete: `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` is the canonical maintainer-facing route inventory, backed by parser/test/wrapper proof.
- S02 is complete: Issues/Alerts/Settings mixed-surface truth is encoded as fail-closed stable tables with runtime proof.
- S03 is complete: the canonical inventory now includes a backend gap map from client promise to backend seam or missing route family.
- S04 implementation is largely assembled but not yet stably closed. The handoff docs, root wrapper, retained proof-bundle contract, isolated-by-default issue seeding, toast-store hardening, and some Playwright/runtime timing hardening landed. The remaining blocker is the assembled rerun rail: `bash ../hyperpush-mono/scripts/verify-m061-s04.sh` still flakes in the delegated combined replay, with current retained evidence showing the first actionable failures inside `route-inventory-dev.log` and later connection-refused fallout as secondary symptoms.

## Architecture / Key Patterns

- Rust workspace under `compiler/` with separate crates for parser, type checker, codegen, runtime, formatter, LSP, CLI, REPL, package tooling, and package manager code.
- Proof-first development: reproduce a real limitation, fix it at the right layer, then prove the repaired path end to end.
- Honest boundary management between `mesh-lang` and `../hyperpush-mono`.
- Assembled closeout verifiers own fresh `.tmp/<slice>/verify` bundles and retain delegated proof trees via copied artifacts plus `latest-proof-bundle.txt` pointers.
- `mesher/client` remains the canonical dashboard package in the sibling repo, with route/runtime verification anchored in real browser proof rather than prose.
- M061 established a maintainer-inventory pattern: keep the canonical route list in code, keep the human-readable inventory beside `mesher/client`, and guard it with fail-closed parser/test/wrapper rails.
- M061/S04 established the closeout pattern: product-root wrapper delegates to the package-owned verifier, retained proof inputs/bundles are explicit, and live-issue seeding is isolated by default unless reuse is explicitly requested.

## Current Risks / Active Blockers

- The M061/S04 assembled route-inventory replay is still flaky under the long combined Playwright run. Current retained evidence shows:
  - prod startup previously needed a longer webServer timeout;
  - the shared toast store needed lifecycle hardening for repeated mount/unmount runs;
  - the current remaining blocker is still the full delegated replay under `bash ../hyperpush-mono/scripts/verify-m061-s04.sh`, especially the combined dev route-inventory phase.
- Resume from:
  - `../hyperpush-mono/.tmp/m061-s04/verify/delegated-route-inventory.log`
  - `../hyperpush-mono/mesher/.tmp/m061-s01/verify-client-route-inventory/route-inventory-dev.log`
  - `../hyperpush-mono/mesher/.tmp/m061-s01/verify-client-route-inventory/route-inventory-prod.log`

## Capability Contract

See `.gsd/REQUIREMENTS.md` for the explicit requirement/status mapping.

## Milestone Sequence

- [x] M028 through M060 except deferred later milestones
- [ ] M061: Mesher Client Mock Truth & Backend Gap Map
- [ ] M035: Test Framework Hardening
- [ ] M037: Package Experience & Ecosystem Polish
