# M049/S01 Research — Postgres starter contract

## Summary

- `R115` and `R122` point at a new Postgres-backed Todo starter, but the current `todo-api` scaffold is a single hardcoded SQLite template with no `--db` selector, no migrations, no package tests, and no Postgres-specific files.
- The current serious PostgreSQL package is `reference-backend/`, but it is not starter-shaped. It is the best source for env-validation + `Pool.open(...)` patterns, not a template to copy wholesale.
- The repo already has the right Postgres building blocks for a starter-sized app: `meshc migrate`, `Migration.*`, `Repo.*`, `Query.*`, and explicit `Pg.*` helpers. Those are proven in `compiler/meshc/src/migrate.rs`, `website/docs/docs/databases/index.md`, `mesher/migrations/20260216120000_create_initial_schema.mpl`, and `mesher/storage/queries.mpl`.
- The safest S01 path is additive: add `meshc init --template todo-api --db postgres <name>` plus new M049-specific tests/helpers while leaving the existing no-flag SQLite template and M047 public-doc rails intact until S02/S04 are ready to replace them.

## Requirements Targeted

- **R115** — primary owner for S01. This slice owns the Postgres half of the dual-database Todo scaffold.
- **R122** — primary owner for S01. Postgres is the serious clustered/deployable path; S01 should establish that contract honestly without pretending the later deploy/failover proof already exists.
- **R112 / R113 / R114** — support-only guardrails. If S01 touches `README.md`, `website/docs/docs/tooling/index.md`, or the `meshc` CLI surface, the M048 entrypoint/update/editor markers must remain intact.

## Skills Discovered

- Loaded: **postgresql-database-engineering**
  - Relevant rules for this slice: keep PostgreSQL-only behavior explicit, prefer migrations over hidden startup DDL, and use real PG primitives like `UUID DEFAULT gen_random_uuid()`, `TIMESTAMPTZ`, and named indexes rather than pseudo-portable wrappers.
- Loaded: **rust-best-practices**
  - Relevant rules for this slice: use a typed enum rather than stringly branching for DB mode, keep scaffold APIs on `Result` instead of panicking, and split focused tests between generator text contract and runtime behavior.
- No additional skill installs were needed. The core technologies in scope here (Rust, PostgreSQL, SQLite, Docker) already have either installed skill coverage or repo-local patterns.

## Do Not Hand-Roll

- Do **not** hand-roll PostgreSQL startup schema creation in `main.mpl`. The serious path should use `migrations/` + `meshc migrate`, not the SQLite-style `ensure_schema()` startup DDL.
- Do **not** default to raw SQL for starter CRUD or schema if `Migration.*`, `Repo.*`, `Query.*`, and `Pg.*` already cover the shape. The repo’s public database contract already teaches those helpers.
- Do **not** wedge Postgres into the current SQLite helper/test module by adding conditionals everywhere. `compiler/meshc/tests/support/m047_todo_scaffold.rs` is too filesystem- and `TODO_DB_PATH`-centric for that to stay clean.
- Do **not** broaden the public docs story in S01 unless necessary. The M047 docs/verifier stack still pins the old SQLite/layered-over-route-free story, and M048 still pins root README/tooling markers.

## Implementation Landscape

### 1. CLI seam: `meshc init` has no DB selector yet

**Files:**
- `compiler/meshc/src/main.rs`
- `compiler/mesh-pkg/src/lib.rs`
- `compiler/meshc/tests/tooling_e2e.rs`

**What exists now:**
- `Commands::Init` only has `clustered: bool`, `template: Option<String>`, and `name: String`.
- Dispatch in `compiler/meshc/src/main.rs` currently is:
  - `Some("todo-api") => mesh_pkg::scaffold_todo_api_project(...)`
  - `None if clustered => mesh_pkg::scaffold_clustered_project(...)`
  - else hello-world scaffold
- `--template` already silently wins over `--clustered`. Adding `--db` without explicit validation will make this ambiguity worse.

**Implication:**
- S01 needs a typed DB-mode seam at the CLI boundary.
- Recommended validation matrix:
  - `meshc init --template todo-api --db postgres <name>` -> success
  - `meshc init --db postgres <name>` -> explicit error
  - `meshc init --template todo-api --db unknown <name>` -> explicit error
  - `meshc init --clustered --template todo-api --db postgres <name>` -> reject explicitly rather than rely on silent precedence

### 2. Current Todo scaffold generator is one monolithic SQLite template

**File:** `compiler/mesh-pkg/src/scaffold.rs`

**Current generator surface:**
- `todo_readme(name)` — SQLite-specific README text
- `todo_dockerfile(name)` — SQLite-specific Dockerfile env and volume story
- `scaffold_todo_api_project(name, dir)` at `compiler/mesh-pkg/src/scaffold.rs:315`

**What it emits today:**
- `mesh.toml`
- `main.mpl`
- `work.mpl`
- `README.md`
- `Dockerfile`
- `.dockerignore`
- `api/health.mpl`
- `api/router.mpl`
- `api/todos.mpl`
- `runtime/registry.mpl`
- `services/rate_limiter.mpl`
- `storage/todos.mpl`
- `types/todo.mpl`

**Hardcoded SQLite contract:**
- `main.mpl` reads `TODO_DB_PATH` and calls `ensure_schema(db_path)` before starting HTTP.
- `storage/todos.mpl` uses `Sqlite.open`, `Sqlite.query`, `Sqlite.execute`, and `last_insert_rowid()`.
- `README.md` teaches SQLite file-path env, local volume persistence, and a SQLite Docker runbook.
- `.dockerignore` excludes `*.sqlite3`.
- There is no `migrations/`, no `tests/`, no `.env.example`, and no config helper module.

**Implication:**
- If S01 does not first split common vs DB-specific file emission, S02 will require either duplicating another 400+ lines of string builders or turning one function into a brittle pile of conditional text.

### 3. Existing scaffold tests duplicate the old SQLite story

**Files:**
- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`

**Current SQLite-specific tests:**
- `compiler/mesh-pkg/src/scaffold.rs`: `m047_s05_scaffold_todo_api_project_writes_clustered_http_template`
- `compiler/meshc/tests/tooling_e2e.rs`: `test_init_clustered_todo_template_creates_project`
- `compiler/meshc/tests/tooling_e2e.rs`: `test_init_clustered_todo_template_rejects_existing_directory`

**What they assert today:**
- `TODO_DB_PATH`
- `Sqlite.open`
- `CREATE TABLE IF NOT EXISTS todos`
- `last_insert_rowid()`
- `*.sqlite3`
- SQLite-specific Dockerfile and README strings
- narrow `HTTP.clustered(1, ...)` adoption on read routes only

**Implication:**
- Do not convert these into mixed two-db tests during S01.
- Keep them green for the legacy SQLite path and add new M049-specific PG generator/CLI tests beside them.

### 4. Existing runtime/e2e helper is SQLite-shaped

**Files:**
- `compiler/meshc/tests/support/m047_todo_scaffold.rs`
- `compiler/meshc/tests/support/mod.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `scripts/verify-m047-s05.sh`

**What exists now:**
- `TodoAppConfig` carries `db_path: PathBuf`.
- `spawn_todo_app*()` exports `TODO_DB_PATH`.
- Docker helpers mount a host directory and assert `todo.sqlite3` creation.
- The runtime truth rail proves local SQLite persistence and SQLite-in-container behavior.

**Implication:**
- This is the wrong seam for Postgres. Add a new support module for M049 Postgres starter runtime proof (or extract only generic HTTP/assertion pieces into shared helpers).
- Keep the M047 SQLite support intact until S02/S04 intentionally retire or replace it.

### 5. `reference-backend/` is the main in-repo PG precedent, but it is too heavy to copy directly

**Files:**
- `reference-backend/main.mpl`
- `reference-backend/config.mpl`
- `reference-backend/tests/config.test.mpl`
- `reference-backend/migrations/20260323010000_create_jobs.mpl`
- `reference-backend/storage/jobs.mpl`
- `reference-backend/README.md`

**Starter-relevant patterns worth reusing:**
- Env helper module + package test style (`config.mpl` + `tests/config.test.mpl`)
- `Pool.open(database_url, 1, 4, 5000)` startup pattern
- runtime logs with clear phase markers
- project-local `migrations/` + `meshc migrate <project> up/status` contract

**Why not copy it wholesale:**
- It is worker-, recovery-, and deploy-artifact-heavy.
- Its migration still uses raw `Pool.execute(...)` SQL by design.
- Its README/runbook is “production backend proof”, not a scaffold starter.

**Implication:**
- Reuse the small patterns (env helpers, package tests, migration directory shape, `Pool.open` startup) and leave the job-worker/deploy complexity out of the starter.

### 6. The proven PG helper surface already exists and is starter-sized enough

**Files:**
- `compiler/meshc/src/migrate.rs`
- `website/docs/docs/databases/index.md`
- `mesher/migrations/20260216120000_create_initial_schema.mpl`
- `mesher/storage/queries.mpl`

**What is already proven and should shape the starter:**
- `meshc migrate` expects `migrations/*.mpl` with `pub fn up(pool :: PoolHandle)` / `down(pool :: PoolHandle)`.
- The generated migration guidance in `compiler/meshc/src/migrate.rs` already teaches:
  - `Migration.create_table(...)`
  - `Migration.create_index(...)`
  - explicit PG-only helpers under `Pg.*`
- `mesher/migrations/...create_initial_schema.mpl` shows the intended schema style:
  - `Pg.create_extension(pool, "pgcrypto")`
  - `Migration.create_table(...)`
  - `Migration.create_index(...)`
  - explicit `Pg.create_gin_index(...)` for PG-only extras
- `mesher/storage/queries.mpl` shows the intended CRUD/query style:
  - `Repo.insert(...)`
  - `Repo.get(...)`
  - `Repo.get_by(...)`
  - `Repo.all(...)`
  - `Repo.update_where(...)`
  - `Repo.update_where_expr(...)`
  - `Query.from(...)`, `Query.where(...)`, `Query.where_expr(...)`, `Query.select_expr(s)`, `Query.order_by(...)`

**Implication:**
- S01 should use this helper stack for the Postgres starter instead of hand-authored raw SQL CRUD or hidden startup DDL.

### 7. Public docs/verifiers still pin the old SQLite/layered-over-route-free story

**Files:**
- `README.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `compiler/meshc/tests/e2e_m047_s06.rs`
- `scripts/verify-m047-s05.sh`
- `scripts/verify-m047-s06.sh`

**Current pinned story:**
- `meshc init --template todo-api <name>`
- SQLite-backed Todo
- `HTTP.clustered(1, ...)` on the two read routes only
- Todo as the “fuller starter” layered above `tiny-cluster` and `cluster-proof`

**Implication:**
- If S01 tries to replace the public docs story immediately, it will collide with M047 historical closeout rails before the PG starter is even proven.
- The cheapest S01 route is to keep broad repo docs unchanged or minimally additive, and let the generated PG README carry the new starter truth.

### 8. M048 doc/tooling guardrails are active if README or tooling docs move

**Files:**
- `scripts/tests/verify-m048-s05-contract.test.mjs`
- `scripts/verify-m048-s05.sh`

**Important current assertions:**
- `README.md` must keep:
  - `meshc update`
  - `meshpkg update`
  - `` `main.mpl` remains the default executable entrypoint ``
  - `` optional `[package].entrypoint = "lib/start.mpl"` ``
  - `bash scripts/verify-m048-s05.sh`
- `website/docs/docs/tooling/index.md` must keep:
  - the update section
  - override-entry wording
  - editor grammar markers
  - `bash scripts/verify-m048-s05.sh`

**Implication:**
- If S01 touches `README.md` or tooling docs, it must preserve these exact markers.
- For S01, it is cheaper to keep the public-doc edits minimal and postpone the broader onboarding rewrite.

### 9. Repo state relevant to later slices

**Observed state:**
- There is no top-level `examples/` directory yet.
- `tiny-cluster/`, `cluster-proof/`, and `reference-backend/` still exist as top-level onboarding-adjacent packages.
- Their removal/repointing is not an S01 task; it is later-slice work and would create unnecessary blast radius if mixed into the PG starter slice.

## Recommendation

1. **Keep S01 additive.** Add `--db postgres` and a PG scaffold path without breaking the existing no-flag SQLite template yet.
2. **Introduce a typed DB-mode seam now** in Rust (`TodoDatabaseKind` or equivalent) and split `scaffold_todo_api_project` into shared/common emitters plus DB-specific emitters. That makes S02 cheap instead of duplicative.
3. **Make the PG starter migration-first.** Use `migrations/` + `meshc migrate <project> up`, not startup `CREATE TABLE` logic inside `main.mpl`.
4. **Use the proven neutral/explicit helper split.** Starter CRUD/query code should lean on `Repo` / `Query` / `Migration`; PG-only behavior should stay under `Pg.*` or the explicit `DATABASE_URL`/pool contract.
5. **Keep the PG starter README honest about scope.** It can describe the serious path and runtime-owned cluster inspection, but it should not claim that M049 already proves the full later deploy/failover bar.
6. **Add new M049 tests/helpers instead of mutating M047’s SQLite rails.** S01 should prove the PG path on its own; S02/S04 can decide when the old SQLite/public-doc rails are retired or replaced.

## Natural Seams

### Seam A — CLI contract

**Files:**
- `compiler/meshc/src/main.rs`
- `compiler/mesh-pkg/src/lib.rs`
- `compiler/meshc/tests/tooling_e2e.rs`

**Work:**
- add `--db <postgres|sqlite>` parsing
- validate bad combinations explicitly
- route `todo-api` template selection through the new DB mode

**Proof:**
- new targeted `tooling_e2e` success/failure cases
- keep existing SQLite `test_init_clustered_todo_template_*` tests green until S02

### Seam B — shared scaffold refactor

**File:** `compiler/mesh-pkg/src/scaffold.rs`

**Work:**
- extract common Todo scaffold pieces from the monolith
- add PG-specific emitters without regressing current SQLite emitters
- keep file-writing and error handling centralized

**Proof:**
- existing generator unit tests stay green
- new M049 PG generator unit test proves the new contract

### Seam C — Postgres starter package contents

**Likely generated files:**
- `mesh.toml`
- `main.mpl`
- `work.mpl`
- `README.md`
- `Dockerfile` / `.dockerignore`
- `api/health.mpl`
- `api/router.mpl`
- `api/todos.mpl`
- `runtime/registry.mpl`
- `types/todo.mpl`
- **new** `config.mpl`
- **new** `storage/todos.mpl`
- **new** `migrations/<timestamp>_create_todos.mpl`
- **new** `tests/*.test.mpl`
- possibly **new** `.env.example`

**Work:**
- replace `TODO_DB_PATH` contract with `DATABASE_URL`
- open a PG pool, not SQLite handles
- move schema ownership into a migration file
- teach the correct `meshc migrate <project> up` / `status` order in the generated README
- add starter-sized package tests (config/source contract) rather than proof-package-sized smoke tests

**Proof:**
- generate scaffold
- inspect files / generated README
- run `meshc migrate <project> up`
- run `meshc build <project>`
- run `meshc test <project>`
- run a focused live PG HTTP smoke against the built starter

### Seam D — Postgres runtime/e2e harness

**Likely new files:**
- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs` (or a shared extracted helper + PG module)
- `compiler/meshc/tests/e2e_m049_s01.rs`
- optional S01 verifier script only if the slice really needs an assembled shell replay early

**Work:**
- scaffold project into temp dir
- create/isolate a PG database
- run migrations
- build binary
- boot starter with `DATABASE_URL`
- drive health + CRUD smoke
- assert generated README/runtime logs remain honest

**Proof:**
- one named milestone-specific Rust e2e target, not a mutation of the SQLite M047 helper/target

## What to Build or Prove First

1. **CLI + scaffold-mode split**
   - this is the seam that unblocks everything else and prevents S02 from becoming duplicate work
2. **Migration-first PG scaffold skeleton**
   - prove the package shape before building a runtime harness
3. **Live PG scaffold smoke**
   - prove `meshc migrate` + build + test + one booted runtime against Postgres
4. **Only then decide whether any public docs beyond the generated README need to move in S01**
   - current M047 and M048 doc tests make broad doc churn expensive

## Verification Surfaces

### Likely S01 acceptance commands

- new/updated generator rail:
  - `cargo test -p mesh-pkg <m049_s01 filter> -- --nocapture`
- new/updated CLI rail:
  - `cargo test -p meshc --test tooling_e2e <postgres todo filter> -- --nocapture`
- new PG runtime rail:
  - `cargo test -p meshc --test e2e_m049_s01 -- --nocapture`

### Existing rails worth keeping green during S01

- legacy SQLite scaffold still green while S02 is not done:
  - `cargo test -p mesh-pkg m047_s05 -- --nocapture`
  - `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture`
- if S01 leans on the PG helper surface heavily and something feels suspect:
  - `bash scripts/verify-m033-s05.sh`
- if S01 touches root README or tooling docs:
  - `node --test scripts/tests/verify-m048-s05-contract.test.mjs`
  - `bash scripts/verify-m048-s05.sh`

## Risks / Unknowns

- **Highest risk:** trying to replace the public SQLite story in S01 instead of adding the PG path alongside it. That drags in M047 docs/verifier rewrites before the PG scaffold is proven.
- **Second risk:** copying `reference-backend` too literally. It proves Mesh + PG, but it is worker/deploy/runbook heavy and still contains raw SQL keep-sites that are not appropriate starter defaults.
- **Third risk:** repeating SQLite’s `ensure_schema()` startup pattern for PG. That conflicts with the existing `meshc migrate` contract and weakens the “serious” starter story.
- **Fourth risk:** turning `m047_todo_scaffold.rs` into a multi-database god-helper instead of creating a clean PG seam.

## Planner Notes

- The cheapest path to S02/S03 is to make S01 establish **typed scaffold mode selection + shared/common file builders** now.
- Treat the generated PG `README.md` as the primary S01 truth surface. Leave broad repo-doc replacement for later slices unless it becomes strictly necessary.
- If the generated PG starter needs package tests, use the small `reference-backend/tests/config.test.mpl` style rather than importing the huge proof-package smoke pattern.
- Remember the current `meshc migrate` call order is project path first, command second (`meshc migrate <project> up` / `status`), not `meshc migrate up <project>`.
- Keep PG-only claims explicit. Both the loaded Postgres skill and the repo’s own database docs push the same rule: do not hide PG behavior behind fake portability.
