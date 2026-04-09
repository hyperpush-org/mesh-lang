# M049: Scaffold & Example Reset

**Gathered:** 2026-04-02
**Status:** Ready for planning

## Project Description

M049 resets Mesh's first real app surface. It upgrades `meshc init --template todo-api` from one SQLite-only starter into a dual-database story, generates checked-in examples from the scaffold output, and removes `tiny-cluster/` / `cluster-proof/` as top-level onboarding surfaces so the public path becomes scaffold/examples first instead of a proof maze. Existing M048 work around manifest entrypoints, self-update commands, and editor grammar truth stays in scope as non-regression guardrails, not as the main implementation target.

## Why This Milestone

The repo currently tells three different first-contact stories: the scaffolded Todo app, the tiny route-free proof apps, and proof-heavy public docs. That sprawl makes Mesh look like a language with verifier artifacts instead of approachable starting points. M049 fixes the starting point first so later docs, landing, deploy, and Mesher work can point at one truthful public app surface.

## User-Visible Outcome

### When this milestone is complete, the user can:

- run `meshc init --template todo-api --db postgres <name>` and get a modern Mesh starter that uses tests, ORM surfaces, pipes, and the current `@cluster` contract where it fits honestly.
- run `meshc init --template todo-api --db sqlite <name>` and get the matching local-first starter without implying shared clustered durability.
- browse checked-in `/examples` generated from those scaffold outputs instead of being sent into `tiny-cluster/` or `cluster-proof/` as onboarding.

### Entry point / environment

- Entry point: `meshc init --template todo-api --db <sqlite|postgres> <name>` and checked-in `/examples/*`
- Environment: local dev / repo source tree / CLI scaffold generation
- Live dependencies involved: none for the milestone acceptance path; real clustered deploy proof is deferred to the later Postgres deploy milestone

## Completion Class

- Contract complete means: scaffold generation, generated example parity, example build/test/README truth, and public-surface replacement are proven by tests, shell verifiers, and retained artifacts.
- Integration complete means: `meshc init`, generated examples, repo references, and verifier surfaces agree on one dual-database starter story.
- Operational complete means: the scaffold/examples generation path is repeatable from a clean tree and old proof-app onboarding surfaces are removed without leaving broken repo-owned references.

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- a user can generate both the Postgres and SQLite Todo starters, build them, run their tests, and see distinct truthful README guidance for each database mode.
- `/examples` contains checked-in generated examples that stay mechanically aligned with scaffold output rather than drifting into hand-maintained variants.
- `tiny-cluster/` and `cluster-proof/` no longer survive as top-level onboarding surfaces, while any proof mechanics they still justify live lower in tests, fixtures, or support code instead of public example directories.
- the assembled verifier replays the new dual-starter/example contract and keeps the already-landed M048 entrypoint/update/editor truths green.

## Risks and Unknowns

- Hard repo removal of `tiny-cluster/` and `cluster-proof` may expose hidden test/verifier dependencies — if those surfaces are still referenced deeply, naive deletion will leave the repo red.
- The SQLite and Postgres starters can drift into two different product stories instead of one shared scaffold contract with honest database-specific differences.
- Generated `/examples` can silently stop matching `meshc init` output if the generation/update path is not made mechanical.

## Existing Codebase / Prior Art

- `compiler/mesh-pkg/src/scaffold.rs` — current `todo-api` scaffold is SQLite-only and already encodes the old starter shape that M049 must replace.
- `compiler/meshc/src/main.rs` — exposes `meshc init --template todo-api` and the CLI surface that will need a database choice.
- `compiler/mesh-pkg/src/manifest.rs` and `compiler/mesh-pkg/src/toolchain_update.rs` — M048's entrypoint override and self-update surfaces are already landed and should stay green as non-regression truth.
- `tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json` and `tools/editors/neovim-mesh/syntax/mesh.vim` — current editor grammar surfaces already carry `@cluster` and both interpolation forms, so M049 should treat them as guardrails, not as the main feature target.
- `compiler/meshc/tests/e2e_m047_s05.rs`, `compiler/meshc/tests/e2e_m046_s05.rs`, and `compiler/meshc/tests/tooling_e2e.rs` — existing scaffold/proof-app assertions will need migration as the old top-level example surfaces are removed.
- `website/docs/docs/getting-started/clustered-example/index.md`, `website/docs/docs/distributed/index.md`, and `website/docs/docs/distributed-proof/index.md` — current public docs still point readers at the proof-app surfaces M049 intends to replace.

> See `.gsd/DECISIONS.md` for all architectural and pattern decisions — it is an append-only register; read it during planning, append to it during execution.

## Relevant Requirements

- R115 — dual-database Todo scaffold with current Mesh patterns
- R116 — checked-in generated examples replace proof-app-shaped teaching surfaces
- R122 — Postgres gets the clustered deploy proof later while SQLite stays explicitly local
- R127 — `tiny-cluster`, `cluster-proof`, and `reference-backend` stop being coequal public onboarding surfaces
- R112, R113, and R114 — already-landed entrypoint, update, and editor truths remain non-regression guardrails while M049 reshapes the public starter story

## Scope

### In Scope

- add explicit SQLite/Postgres choice to the Todo scaffold
- modernize generated starter code toward tests, ORM surfaces, pipes, and current `@cluster` usage where honest
- generate checked-in `/examples` from scaffold output
- migrate top-level onboarding away from `tiny-cluster/` and `cluster-proof`
- add one assembled verifier for the new scaffold/example contract

### Out of Scope / Non-Goals

- real clustered deployment proof for the serious starter (belongs to the later deploy milestone)
- full docs-site rewrite
- landing-page rewrite
- retiring `reference-backend/` and modernizing `mesher/`
- frontend-aware load-balancing adapters

## Technical Constraints

- SQLite must stay an explicitly local/single-node starter and must not imply shared clustered durability.
- The public contract stays platform-agnostic even if later deployment proof uses Fly as the current proving ground.
- `/examples` should be generated from scaffold output rather than hand-maintained lookalikes.
- Removing the old top-level proof apps cannot leave repo-owned tests/verifiers orphaned; any surviving proof value must move into lower-level fixtures or support code.

## Integration Points

- `meshc init` CLI — database-mode selection and scaffold routing
- `compiler/mesh-pkg/src/scaffold.rs` — starter generation source of truth
- repo example directories — generated outputs committed under `/examples`
- existing scaffold/proof e2e and verifier surfaces — migration path away from top-level proof apps
- later docs milestone — will consume the new scaffold/examples as the primary evaluator path

## Open Questions

- How much of the existing `tiny-cluster/` / `cluster-proof/` verifier value can be absorbed into tests/support without reintroducing a second shadow example surface? — current thinking: keep proof mechanics only at test/fixture level, not as top-level packages.
- Should the Postgres starter already encode clustered route adoption in M049, or stay simpler and let the later deploy proof milestone add the deeper production story? — current thinking: keep M049 honest and starter-shaped; let the later deploy milestone own the real clustered/deploy proof bar.
- What is the cleanest regeneration/update path for committed `/examples` so drift is mechanically detectable? — current thinking: one repo-owned generation/verification command rather than manual edits.
