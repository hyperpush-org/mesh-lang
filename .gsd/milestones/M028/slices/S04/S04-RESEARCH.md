# S04: Boring Native Deployment — Research

## Summary

S04 directly owns **R005** and should also strengthen **R008** and **R010** by replacing vague “single binary” language with one concrete deployable reference-backend story.

The good news: the **runtime binary itself is already close to the target**.

- `meshc build ./reference-backend` succeeds and emits `./reference-backend/reference-backend`.
- `compiler/meshc/tests/e2e.rs` already has `e2e_self_contained_binary`, which proves Mesh binaries do **not** dynamically link `mesh_rt`.
- I verified the built `reference-backend` binary can be **copied to a temp directory outside the repo** and still run.
- I also verified a copied temp-dir artifact can be started against a real Postgres database and process a job successfully after migrations are applied.

The real deployment blocker is **not** HTTP/job/runtime behavior anymore. It is the **migration/deploy workflow**:

- `meshc migrate` is still a **compile-at-apply** path.
- Using `meshc` on a deploy target is not boring today: it needs the compiler binary, `libmesh_rt.a`, a working `cc`, and meshc’s own runtime/shared-library environment.
- The benchmark Dockerfile shows the amount of system setup currently needed when a machine/container must compile Mesh code itself.

So S04 should be scoped around this question:

> How do we deploy `reference-backend` as a boring native artifact **without** requiring the production host to behave like a Mesh compiler workstation?

This matches the `debug-like-expert` rule **“VERIFY, DON’T ASSUME”**: the research should trust the copied-binary smoke, not the README claim. It also matches the `test` skill rule to **extend existing test patterns instead of inventing a new harness**.

## Recommendation

Make S04 **artifact-first, not platform-first**.

The canonical deliverable should be a deployable `reference-backend` artifact/workflow where the **runtime host** needs only:

- the compiled `reference-backend` binary
- environment variables (`DATABASE_URL`, `PORT`, `JOB_POLL_MS`)
- a reachable Postgres database

And **does not** need:

- `meshc`
- `libmesh_rt.a`
- LLVM/toolchain setup
- project source tree checkout

### Recommended shape

1. **Keep the runtime proof centered on the compiled app binary.**
   - The binary already runs correctly outside the repo root.
   - That is the closest thing Mesh currently has to a Go-like deployment story.

2. **Do not make `meshc migrate` the runtime-host migration story.**
   - It is too heavy for “boring deploy”.
   - It is acceptable for developer workstations and CI/build stages, but not as the core operator story.

3. **Choose a deployment migration strategy before touching docs or smoke harnesses.**
   This is the main architectural fork for S04.

   Lowest-risk options, in order:

   - **Option A: deploy-time migration artifact separate from meshc**
     - e.g. a SQL artifact or a prebuilt one-off migrator artifact that preserves `_mesh_migrations`
     - best fit for “boring runtime host”
     - keeps the app runtime thin
   - **Option B: builder/release-stage migration execution**
     - acceptable for a platform wrapper (container/release command)
     - still keeps the runtime machine free of meshc
   - **Option C: startup-time auto-migrate inside `reference-backend`**
     - most self-contained operationally
     - but most invasive to the already-proved startup path, and duplicates migration/tracking logic unless designed carefully

4. **If S04 adds a platform example, keep it thin and secondary.**
   - A multi-stage Docker/Fly example is reasonable because the repo already has donor patterns.
   - But the canonical proof should still be: “here is the staged native artifact, here is how to run it, here is how to smoke-check it.”

This aligns with the `best-practices` skill: **keep build toolchains in builder stages, not in the production runtime surface**.

## What exists now

### `compiler/meshc/src/main.rs`

Why it matters:
- authoritative `meshc build` behavior
- already supports `--output`, `--opt-level`, and `--target`
- default output path is `dir/<project_name>`

Key fact:
- `build()` determines output as `dir.join(project_name)` unless `--output` is passed.

Implication for S04:
- no compiler change is strictly required to stage a deployment artifact; S04 can already build into a temp/staging path with `--output` if needed

### `compiler/mesh-codegen/src/link.rs`

Why it matters:
- tells the truth about what the compiler needs at link time

Key facts:
- Mesh programs link via system `cc`
- they require `libmesh_rt.a` at build time
- the linker emits a native executable and removes the intermediate `.o`

Implication for S04:
- building Mesh programs on a production host is still toolchain-dependent even if running them is not

### `compiler/meshc/src/migrate.rs`

Why it matters:
- this is the core S04 friction point

Key facts:
- `meshc migrate` discovers `.mpl` migration files
- copies each migration into a temp synthetic project
- generates a synthetic `main.mpl`
- calls `crate::build(...)`
- executes the compiled temp binary
- records versions in `_mesh_migrations`

Implication for S04:
- current migration apply is a **compiler workflow**, not a lightweight operator workflow
- if S04 keeps this as the production migration path, the runtime host stops being “boring”

### `compiler/meshc/tests/e2e_reference_backend.rs`

Why it matters:
- already the authoritative backend harness from S01/S02
- best place to add deployment verification

What it already has:
- build helper
- spawn/stop helpers
- migration command helpers
- HTTP helpers
- direct Postgres helpers via `native_pg_query` / `native_pg_execute`
- single-instance and two-instance runtime proofs

Implication for S04:
- do **not** invent a second verification harness
- extend this file with a staged-artifact deployment test or helper layer

### `compiler/meshc/tests/e2e.rs`

Why it matters:
- already contains the generic `e2e_self_contained_binary` proof

Key fact:
- existing test asserts Mesh binaries do not dynamically link `mesh_rt`

Implication for S04:
- this is strong prior proof for the language-level claim
- S04 should add the reference-backend-specific operational version of that claim

### `reference-backend/main.mpl`

Why it matters:
- defines the runtime startup contract S04 must preserve

Key facts:
- only required env vars are `DATABASE_URL`, `PORT`, `JOB_POLL_MS`
- startup logs are explicit and readable
- the runtime path does not read repo-relative files at startup

Implication for S04:
- the app binary is already suitable for “copy to host, set env, run”
- any change that broadens startup behavior should be treated carefully because S02 already proved the current path

### `reference-backend/config.mpl`

Why it matters:
- stable source of env var names and config error messages

Implication for S04:
- deployment docs, service units, env examples, and smoke scripts should reuse this exact contract

### `reference-backend/scripts/smoke.sh`

Why it matters:
- current operator smoke surface

What it does now:
- requires `DATABASE_URL`
- checks the `jobs` table exists already
- rebuilds `reference-backend`
- starts the app locally
- creates a job and polls until processed

Why it is not sufficient for S04 by itself:
- it is a **developer/local smoke** path, not a deployed-artifact smoke path
- it always rebuilds and re-spawns locally
- it assumes migrations already ran

Implication for S04:
- either refactor it into reusable pieces or add a sibling “probe an already deployed instance” script

### `reference-backend/README.md` and `reference-backend/.env.example`

Why they matter:
- these are the primary operator-facing docs surfaces already attached to the canonical backend package

Current gap:
- they document build/run/migrate/smoke for a dev workflow, but not a deployment/staged-artifact workflow

Implication for S04:
- package-local docs are the right place to land the first boring deployment story
- broader README/site promotion can stay mostly for S06

### `reference-backend/migrations/20260323010000_create_jobs.mpl`

Why it matters:
- current schema truth for the reference backend

Key facts:
- mostly raw SQL already (`CREATE EXTENSION`, `CREATE TABLE`, `CREATE INDEX`)
- `down` uses `Migration.drop_table`

Implication for S04:
- because the migration is already almost entirely raw SQL, a deploy-time SQL artifact is lower-risk than generalizing the migration runtime in this slice

### `benchmarks/fly/Dockerfile.servers`

Why it matters:
- best current donor pattern for “build Mesh in Linux container”

What it proves:
- compiling Mesh programs inside a Linux image currently needs a large builder setup (LLVM dev, Rust toolchain, meshc, mesh-rt)
- runtime images can be slimmer than builder images

Implication for S04:
- if a platform example is added, use a multi-stage container pattern
- but do **not** put meshc/toolchain into the final runtime image unless there is no better migration story

### Historical gotcha: `.gsd/milestones/M021/slices/S09/tasks/T01-SUMMARY.md`

Why it matters:
- this file records a prior real failure mode: installed `meshc` could not locate `libmesh_rt.a`

Implication for S04:
- do not design the boring deployment story around “just install meshc on the box and run it there”

## Research evidence gathered

These were the most useful direct checks:

### 1. Reference-backend build works now

```bash
cargo run -p meshc -- build ./reference-backend
```

Observed:
- succeeded
- emitted `./reference-backend/reference-backend`

### 2. The built app binary is thin at runtime

```bash
file reference-backend/reference-backend
otool -L reference-backend/reference-backend
```

Observed on this machine:
- native Mach-O executable
- dynamic deps were only system libraries/frameworks (`libSystem`, `Security`)
- no dynamic `mesh_rt` dependency

### 3. The binary runs outside the repo root

I copied the compiled binary into a temp directory and ran it there.

Observed:
- missing-env path worked immediately from the temp dir
- no repo-relative assets were required just to launch the binary

### 4. Copied temp-dir artifact can still do the real backend job flow

I sourced the existing local `.env`, applied migrations, copied the binary to a temp directory, started it from there, and exercised:

- `GET /health`
- `POST /jobs`
- `GET /jobs/:id`

Observed:
- the copied artifact processed a job successfully
- this is the strongest evidence that the app binary itself is already deployable

### 5. The compiler is much heavier than the app binary

```bash
otool -L target/debug/meshc
find target -maxdepth 2 -name 'libmesh_rt.a'
```

Observed:
- `meshc` depends on more host libraries/tooling context than the compiled app binary
- the repo must also have `libmesh_rt.a` available for build/link flows

This is the core reason S04 should avoid runtime-host dependence on `meshc`.

## Natural seams for planning

### Seam 1: migration strategy decision (first, highest leverage)

This is the blocker that determines everything else.

Planner should decide first whether S04 will:

- produce a SQL/deploy migration artifact
- produce a separate migrator artifact
- or teach the app binary to auto-migrate on startup

Without this decision, docs and smoke verification will drift.

### Seam 2: deployment artifact staging

Likely files:
- `reference-backend/README.md`
- `reference-backend/.env.example`
- possibly new `reference-backend/deploy/` or `reference-backend/scripts/` files
- maybe no compiler changes needed if `meshc build --output ...` is sufficient

Goal:
- define what gets copied/shipped
- define the boring runtime command exactly

### Seam 3: probe-only deployment smoke

Likely files:
- `reference-backend/scripts/smoke.sh` refactor, or
- new sibling script for deployed/staged instances

Goal:
- smoke-check a running artifact without rebuilding it locally
- verify health + create/read/process flow against a real running deployment target

### Seam 4: automated deployment verification

Best file:
- `compiler/meshc/tests/e2e_reference_backend.rs`

Goal:
- add an ignored deploy-artifact test that stages the binary outside the repo root and proves the deployed-path contract end to end

### Seam 5: optional platform wrapper

Only after the artifact/migration story is clear.

Likely files if chosen:
- new container/service files under `reference-backend/`
- possibly a Fly/systemd example

Goal:
- keep the wrapper thin and obviously derivative of the artifact-first workflow

## What to build or prove first

1. **Decide and implement the migration story.**
   - This is the only part that can still force a non-boring runtime host.
   - Everything else is documentation and proof around an already-good binary.

2. **Create a staged-artifact proof that runs outside the repo root.**
   - This is the central S04 claim.
   - I already proved manually that it works; S04 should turn that into a repeatable harness.

3. **Split local smoke from deploy smoke.**
   - Today’s smoke script is too dev-oriented for deployment proof.

4. **Write package-local operator docs only after the exact commands exist.**
   - S04 docs should be command-truth, not aspiration.

## Verification plan

Minimum slice gate should include:

```bash
cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_builds -- --nocapture
cargo test -p meshc e2e_self_contained_binary -- --nocapture
```

And S04 should add one authoritative deployment verification, likely ignored/Postgres-backed, shaped roughly like:

```bash
DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_deploy_artifact_smoke -- --ignored --nocapture
```

That test should prove all of these together:

- build/stage the artifact outside the repo root
- apply migrations through the chosen S04 deployment path
- start the staged artifact from that staged location
- `GET /health` succeeds
- `POST /jobs` succeeds
- `GET /jobs/:id` reaches `processed`
- logs do not echo `DATABASE_URL`
- runtime success does not require `meshc` or source files on the runtime side

If S04 adds a platform wrapper (container/Fly/systemd), add one more verification layer only after the artifact proof passes.

## Risks / gotchas

- **Do not broaden the runtime-host contract accidentally.**
  - Shipping `meshc` + `libmesh_rt.a` + compiler dependencies to production is exactly the thing this slice should avoid normalizing.

- **Do not treat the current smoke script as deployment proof.**
  - It rebuilds and spawns locally; it is not yet a deploy smoke harness.

- **Be careful with startup-path changes.**
  - `reference-backend/main.mpl` is already on a proved S02/S03 path. Auto-migrate-on-boot is attractive, but it widens the hot startup path.

- **If SQL artifacts are introduced, keep one source of truth clear.**
  - The planner should explicitly decide whether the Mesh migration file or generated SQL is canonical.

- **Prefer extending `e2e_reference_backend.rs` over creating a parallel deployment harness.**
  - The repo already has the right process/HTTP/DB helpers.

## Skill discovery

Relevant missing skills I checked but did **not** install:

- **Postgres:** `npx skills add supabase/agent-skills@supabase-postgres-best-practices`
  - highest install count from search
  - useful if S04 chooses a SQL artifact or migration-hardening path

- **Docker:** `npx skills add github/awesome-copilot@multi-stage-dockerfile`
  - useful if S04 adds a container wrapper around the staged binary

- **Systemd:** `npx skills add chaterm/terminal-skills@systemd`
  - only useful if the planner chooses a Linux VM/service-unit story instead of container/Fly

## Sources

Repo files inspected:

- `compiler/meshc/src/main.rs`
- `compiler/mesh-codegen/src/link.rs`
- `compiler/meshc/src/migrate.rs`
- `compiler/meshc/tests/e2e.rs`
- `compiler/meshc/tests/e2e_reference_backend.rs`
- `reference-backend/main.mpl`
- `reference-backend/config.mpl`
- `reference-backend/scripts/smoke.sh`
- `reference-backend/README.md`
- `reference-backend/.env.example`
- `reference-backend/migrations/20260323010000_create_jobs.mpl`
- `benchmarks/fly/Dockerfile.servers`
- `benchmarks/fly/README.md`
- `.gsd/milestones/M021/slices/S09/tasks/T01-SUMMARY.md`

Commands run during research:

```bash
cargo run -p meshc -- build ./reference-backend
cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_builds -- --nocapture
cargo test -p meshc e2e_self_contained_binary -- --nocapture
file reference-backend/reference-backend
otool -L reference-backend/reference-backend
otool -L target/debug/meshc
```

Additional manual proof performed:
- copied the built `reference-backend` binary to a temp directory
- started it there against a real Postgres database after migrations were applied
- verified health/job processing flow without running the app from the repo root
