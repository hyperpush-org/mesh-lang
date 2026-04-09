# M053 Research — Deploy Truth for Scaffolds & Packages Surface

**Date:** 2026-04-04
**Status:** Ready for roadmap planning

## Summary

M053 is not starting from zero. The repo already has three strong foundations:

1. **The public starter split already exists and is explicit.**
   - `meshc init --template todo-api --db sqlite` is intentionally local-only.
   - `meshc init --template todo-api --db postgres` is already described as the serious shared/deployable starter.
   - That split is enforced in generator code, CLI tests, example parity, docs, and public contracts.

2. **The packages surface is already partly inside the deploy/public proof chain.**
   - `.github/workflows/deploy-services.yml` already deploys `packages-website/`.
   - `scripts/lib/m034_public_surface_contract.py public-http` already probes package detail/search plus registry search.
   - `scripts/verify-m034-s05.sh` already expects hosted `deploy-services.yml` evidence.

3. **The retained backend fixture already demonstrates the deploy-proof shape worth reusing.**
   - `scripts/fixtures/backend/reference-backend/scripts/stage-deploy.sh` stages a deploy bundle from source without mutating the source tree.
   - `deploy-smoke.sh` waits for truthful health, exercises live endpoints, and fails closed.
   - `scripts/verify-m051-s02.sh` already assembles that into a retained deploy/runtime proof rail.

The real gap is narrower than the milestone title suggests: **the generated Postgres starter still does not own a generated-starter-first deploy bundle, production-like clustered deploy replay, or starter-native failover/operator proof.** Existing Postgres proof is local runtime CRUD plus public wording. The older M047 clustered/Docker proof exists, but it is historical, partly fixture-backed, and not the current generated Postgres starter contract.

## Skills Discovered

Installed skills already relevant to this milestone:

- `github-workflows` — directly relevant for hosted deploy/evidence-chain work
- `flyio-cli-public` — directly relevant for Fly proof/deploy inspection boundaries
- `postgresql-database-engineering` — relevant for migration/deploy/database truth
- `multi-stage-dockerfile` — relevant for starter deploy assets and container packaging

No additional skill installs were needed.

## Current Codebase Reality

### 1. Starter generation already draws the correct honesty boundary

Primary files:
- `compiler/meshc/src/main.rs`
- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `compiler/meshc/tests/e2e_m049_s01.rs`
- `compiler/meshc/tests/e2e_m049_s02.rs`
- `scripts/tests/verify-m049-s03-materialize-examples.mjs`
- `examples/todo-postgres/README.md`
- `examples/todo-sqlite/README.md`

What is already true:
- CLI resolution in `compiler/meshc/src/main.rs` rejects `--db` outside `--template todo-api` and rejects `--clustered` mixed with `todo-api`.
- `compiler/mesh-pkg/src/scaffold.rs` has a typed `TodoApiDatabase` split and two separate generator branches.
- SQLite starter has:
  - no `work.mpl`
  - no `HTTP.clustered(...)`
  - no `meshc cluster` story
  - direct local startup and schema creation
- Postgres starter has:
  - `work.mpl` with `@cluster pub fn sync_todos()`
  - `Node.start_from_env()` bootstrap
  - migrations-owned schema creation
  - `HTTP.clustered(1, ...)` only on `GET /todos` and `GET /todos/:id`
  - local `/health` and local mutating routes

Important constraint: `compiler/mesh-pkg/src/scaffold.rs` tests explicitly pin the Postgres README to **omit** `failover` and **omit** `Fly.io`. That means M053 cannot honestly solve its public-contract problem by turning the generated starter README into a Fly runbook.

### 2. Existing Postgres starter proof is still below the M053 bar

Primary files:
- `compiler/meshc/tests/e2e_m049_s01.rs`
- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs`

What it proves today:
- `meshc init --template todo-api --db postgres`
- `meshc migrate <project> up`
- `meshc test <project>`
- `meshc build <project>`
- local boot
- `/health`
- CRUD behavior
- fail-closed behavior for missing `DATABASE_URL`
- explicit error on unmigrated database

What it does **not** prove today:
- staged deploy artifact generation
- reusable deploy bundle/runbook owned by the generated starter
- multi-node clustered deployment
- starter-native failover or node-loss survival
- hosted deploy replay
- Fly deployment/reference assets
- integration into the normal hosted release/deploy chain beyond existing public wording

Important constraint: `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs` only has a **single-node** runtime helper shape. It can inject clustered env, but there is no existing reusable two-node/generated-starter harness here yet.

### 3. There is historical clustered starter proof to reuse — but not to claim as the current surface

Primary files:
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `scripts/fixtures/m047-s05-clustered-todo/`
- `scripts/verify-m047-s05.sh`

What exists:
- native + Docker clustered route truth
- single-node cluster-mode operator/continuity checks
- retained historical fixture-backed rails
- operator command markers in README/docs contracts

What matters strategically:
- This is **reusable proof technique**, not the current public/generated-starter contract.
- M049 deliberately moved public onboarding away from proof-app-shaped or historical retained surfaces.
- M053 should borrow its mechanics, not revive its public status.

### 4. The packages site is already more integrated than the milestone context implies

Primary files:
- `.github/workflows/deploy-services.yml`
- `scripts/verify-m034-s05-workflows.sh`
- `scripts/lib/m034_public_surface_contract.py`
- `packages-website/Dockerfile`
- `packages-website/fly.toml`
- `README.md`

What is already true:
- `deploy-services.yml` deploys `registry/`, `packages-website/`, and `mesher/landing/` on `main` and `v*` tags.
- `health-check` already runs the shared public-surface helper.
- `m034_public_surface_contract.py public-http` already checks:
  - docs URLs
  - installer URLs
  - package detail page
  - package search page
  - registry search API
- `scripts/verify-m034-s05.sh` already expects hosted evidence for `deploy-services.yml`, including `Deploy mesh-packages website` and `Verify public surface contract`.
- `packages-website/Dockerfile` already uses the proven safe pattern: `npm ci -> npm run build -> npm prune --omit=dev` in builder, then copy pruned runtime artifacts forward.

So the packages-site problem is **not** “missing deployment or missing health checks.” The real gap is:
- it still feels like a parallel services workflow rather than part of the mainline starter/deploy truth story
- the canonical hosted evidence chain is still split across workflows rather than expressed as one obvious starter-to-public-surface contract

### 5. The strongest reusable deploy-proof pattern already exists in the retained backend fixture

Primary files:
- `scripts/fixtures/backend/reference-backend/scripts/stage-deploy.sh`
- `scripts/fixtures/backend/reference-backend/scripts/deploy-smoke.sh`
- `scripts/fixtures/backend/reference-backend/scripts/smoke.sh`
- `scripts/verify-m051-s02.sh`

Reusable pattern:
- build from source into a deploy bundle outside the source tree
- stage executable + migrations + smoke scripts together
- fail if the source tree gets polluted by in-place outputs
- wait for a truthful health signal before traffic
- exercise real endpoints against the running deployment
- retain artifacts and make bundle shape part of the contract

This is the clearest existing model for M053’s starter deploy proof.

### 6. Fly is already deliberately bounded as proof environment, not public contract

Primary files:
- `scripts/verify-m043-s04-fly.sh`
- `scripts/fixtures/clustered/cluster-proof/fly.toml`
- `examples/todo-postgres/README.md`

What is already true:
- The read-only Fly verifier is intentionally **non-destructive**.
- It validates config, logs, membership, and optional status probes only.
- It explicitly does **not** deploy, mutate, or prove destructive failover.
- The generated Postgres starter README currently avoids Fly-specific wording altogether.

This gives M053 a clean boundary:
- Fly-specific assets or proof can exist
- but they must be clearly secondary/reference surfaces
- the starter’s public contract still needs a portable deploy/deploy-artifact story first

## Strategic Answers

### What should be proven first?

**First prove that the generated Postgres starter can produce a truthful deploy bundle and survive a local production-like replay before touching hosted/Fly proof.**

Reason:
- It is the highest-risk unknown.
- It is the real substance behind R122.
- Without a generated-starter-first deploy bundle, any hosted or Fly proof will either be hand-curated or maintainer-only.

Concrete first proof target:
- generated Postgres starter
- migrations applied externally
- built artifact staged outside source tree
- deploy smoke waits for truthful health
- live CRUD succeeds against the staged runtime
- operator CLI truth is exercised against that same running starter

### What existing patterns should be reused?

1. **Retained backend deploy bundle pattern**
   - `stage-deploy.sh` + `deploy-smoke.sh` shape from `scripts/fixtures/backend/reference-backend/`
   - likely adapted into generator-owned starter assets or starter-focused test harness helpers

2. **Existing starter parity/materializer pattern**
   - `/examples` stay generator-owned via `scripts/tests/verify-m049-s03-materialize-examples.mjs`
   - any starter deploy asset checked into `examples/todo-postgres` should come from generation, not hand edits

3. **Shared public-surface helper**
   - do not add new ad hoc curl checks for packages site or docs
   - keep using `scripts/lib/m034_public_surface_contract.py`

4. **Current operator CLI sequence**
   - `meshc cluster status`
   - continuity list
   - continuity record
   - diagnostics
   This sequence is already the canonical runtime-owned inspection story.

5. **Packages-site Docker packaging pattern**
   - keep the current `npm ci -> build -> prune` builder pattern
   - do not regress to runtime-stage reinstall

### What boundary contracts matter?

1. **SQLite boundary is non-negotiable**
   - no clustered durability claim
   - no `work.mpl`
   - no `HTTP.clustered(...)`
   - no operator/Fly/deploy-grade shared-story creep

2. **Postgres starter boundary is currently narrow and explicit**
   - clustered reads only
   - local writes/health
   - migration-first startup
   - no schema-on-boot
   - no secret leakage in `/health`
   - no Fly-specific contract in generated README

3. **Packages site stays separately deployed**
   - D336 already settled this: unify evidence chain, do not collapse architecture

4. **Public docs remain scaffold/examples-first**
   - deeper backend/failover/deploy proof must not displace the starter ladder
   - Mesher and retained backend fixtures remain maintainer-facing

### What constraints does the existing codebase impose?

- `compiler/mesh-pkg/src/scaffold.rs` tests already fail closed on Fly/failover wording in generated Postgres README.
- `/examples` are generator-owned and parity-checked; any M053 surface added there must come from scaffold generation.
- `deploy-services.yml` shape is tightly pinned by `scripts/verify-m034-s05-workflows.sh`.
- `scripts/verify-m034-s05.sh` currently validates hosted `deploy-services.yml` against the **binary tag**, not mainline branch evidence.
- current starter runtime helpers are local/single-node oriented; multi-node starter proof needs a new harness layer.
- existing Fly verifier is read-only; destructive or deploy-time Fly starter proof is a new contract.

### Known failure modes that should shape slice ordering

1. **Fake-green deploy proof via hand-curated fixture instead of generated starter**
   - biggest milestone risk
   - avoid by proving directly from `meshc init --template todo-api --db postgres`

2. **Fly-only truth leakage into the public starter contract**
   - already guarded against in scaffold tests
   - suggests Fly assets belong in secondary/reference surfaces or generated optional kit files, not main README contract

3. **Packages-site regressions from Docker packaging drift**
   - known sensitive seam
   - do not reintroduce runtime `npm install --omit=dev`

4. **Hosted evidence split-brain**
   - current canonical local proof wrapper (`verify-m034-s05.sh`) spans multiple workflows, but `authoritative-verification.yml` itself does not own the deploy-services job graph
   - this is a workflow-chain design risk, not a missing-health-check problem

5. **Over-scoping into M054**
   - if M053 tries to “fix load balancing” instead of proving starter deploy truth, it will sprawl into the next milestone’s work

## Slice Recommendations

### Suggested Slice 1 — Generated Postgres starter deploy bundle + local deploy smoke

**Goal:** make the generated Postgres starter produce a truthful deploy artifact and deploy-smoke story without any Fly dependency.

Own these seams:
- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/e2e_m049_s01.rs`
- new starter deploy helper/harness under `compiler/meshc/tests/support/`
- generated `examples/todo-postgres`

Why first:
- highest risk
- prerequisite for everything else
- platform-agnostic public contract starts here

Expected output:
- generated starter-owned staging/deploy assets or equivalent harness contract
- migrated + staged + smoke-tested starter replay
- retained artifacts

### Suggested Slice 2 — Generated Postgres starter clustered deploy truth + failover/operator proof

**Goal:** prove the serious starter in a production-like clustered environment with runtime/operator truth and node-loss/failover behavior.

Own these seams:
- starter runtime harnesses/tests
- generated starter docs/examples wording
- possibly Docker-based two-node replay before any hosted/Fly layer

Why second:
- real unknown after deploy artifactization
- likely needs new harness work
- should stay generated-starter-first instead of jumping to Fly immediately

Important boundary:
- prove only the failover semantics the starter/runtime actually own today
- do not silently expand into general load-balancing follow-through

### Suggested Slice 3 — Packages-site evidence-chain hardening

**Goal:** move packages-site verification from “already deployed and checked” to “obviously part of the same normal hosted evidence chain.”

Own these seams:
- `.github/workflows/deploy-services.yml`
- `.github/workflows/authoritative-verification.yml` and/or related wrappers
- `scripts/verify-m034-s05.sh`
- `scripts/verify-m034-s05-workflows.sh`
- `scripts/lib/m034_public_surface_contract.py`

Why third:
- lower technical uncertainty than starter failover
- existing helper and workflow shape already exist
- mostly chain-ownership/hosted-evidence work, not product/runtime work

### Suggested Slice 4 — Fly reference assets + public wording alignment

**Goal:** add Fly as the current proving environment for the serious starter without turning Fly into the product contract.

Own these seams:
- generated or example-level reference assets if needed
- public docs wording in README/docs/example README
- retained/proof-page wiring

Why last:
- depends on starter deploy contract already being truthful without Fly
- mostly wording + reference-asset integration after the real proof exists

## Requirements Assessment

### Table stakes from existing active requirements

- **R121** is core M053 work, not a side cleanup.
  - packages site must be part of the normal CI/deploy contract
- **R122** is the milestone’s main technical burden.
  - Postgres needs real clustered deploy proof
  - SQLite must remain explicitly local

### Upstream active requirements that constrain M053

- **R115**: keep the dual-db starter split honest
- **R116**: do not re-promote retained proof apps over generated examples
- **R117**: keep public docs evaluator-facing, not proof-maze-first
- **R120**: landing/docs/packages still need one coherent public story

### Candidate requirements to consider (advisory only)

1. **Generated Postgres starter owns a staged deploy bundle contract**
   - Reason: R122 currently says “truthful clustered deploy proof,” but not whether that proof must be starter-owned and reusable outside the test harness.
   - I recommend making this explicit.

2. **Hosted mainline evidence must fail when starter deploy proof or packages-site public surface breaks**
   - Reason: local assembled wrappers already exist, but hosted ownership is still split.
   - This sharpens R121/R122 into a concrete operational requirement.

3. **Public starter docs must distinguish portable contract from Fly reference proof**
   - Reason: current tests already forbid Fly-first wording in generated starter surfaces.
   - This is probably a docs/contract requirement, not just advice.

### Overbuilt risks / likely out of scope

- turning SQLite into a clustered/shared durability story
- collapsing packages-website into the docs deploy architecture
- making Fly the primary starter contract
- inventing a new proof app instead of using the generated Postgres starter
- broad load-balancing/platform work that belongs to M054

## Recommended Verification Surfaces

### Existing commands worth reusing

- `cargo test -p meshc --test e2e_m049_s01 -- --nocapture`
- `cargo test -p meshc --test e2e_m049_s02 -- --nocapture`
- `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check`
- `bash scripts/verify-m034-s05-workflows.sh`
- `bash scripts/verify-m034-s05.sh`
- `bash scripts/verify-production-proof-surface.sh`
- `bash scripts/verify-m051-s02.sh`
- `bash scripts/verify-m043-s04-fly.sh --help`

### New proof seams M053 likely needs

- starter-specific stage-deploy contract
- starter-specific deploy-smoke contract
- starter-specific clustered/failover replay
- hosted workflow evidence that clearly includes starter deploy truth alongside packages-site truth

## Planning Guidance

If the roadmap planner needs one sentence of direction:

**Treat M053 as a generated-Postgres-starter deploy-proof milestone first and a packages-site evidence-chain cleanup second; reuse the retained backend deploy-bundle pattern, preserve the SQLite/Postgres honesty split, and keep Fly as a secondary proof environment layered on top of a portable starter-owned deploy contract.**
