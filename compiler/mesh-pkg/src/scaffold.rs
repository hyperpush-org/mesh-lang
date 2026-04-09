//! Project scaffolding for `meshc init`.
//!
//! Creates one of three standard Mesh project layouts:
//!
//! ```text
//! <name>/
//!   mesh.toml
//!   main.mpl
//! ```
//!
//! or, for `meshc init --clustered`:
//!
//! ```text
//! <name>/
//!   mesh.toml
//!   main.mpl
//!   work.mpl
//!   README.md
//! ```
//!
//! or, for `meshc init --template todo-api`:
//!
//! ```text
//! <name>/
//!   mesh.toml
//!   main.mpl
//!   config.mpl
//!   README.md
//!   Dockerfile
//!   .dockerignore
//!   api/
//!   runtime/
//!   services/
//!   storage/
//!   tests/
//!   types/
//! ```

use serde::Deserialize;
use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ScaffoldRepoIdentity {
    language_repo: ScaffoldRepoIdentityRepo,
    product_repo: ScaffoldRepoIdentityRepo,
    product_handoff: ScaffoldProductHandoff,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ScaffoldRepoIdentityRepo {
    repo_url: String,
    blob_base_url: String,
    docs_root: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ScaffoldProductHandoff {
    label: String,
    relative_runbook_path: String,
}

struct ScaffoldPublicLinks {
    todo_postgres_readme_url: String,
    todo_sqlite_readme_url: String,
    production_backend_proof_url: String,
    product_repo_url: String,
    product_handoff_label: String,
    product_runbook_path: String,
    product_runbook_url: String,
}

fn scaffold_public_links() -> &'static ScaffoldPublicLinks {
    static LINKS: OnceLock<ScaffoldPublicLinks> = OnceLock::new();
    LINKS.get_or_init(|| {
        let contract: ScaffoldRepoIdentity = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../scripts/lib/repo-identity.json"
        )))
        .expect("scripts/lib/repo-identity.json should stay valid JSON for scaffold links");
        let docs_root = contract
            .language_repo
            .docs_root
            .as_deref()
            .expect("language repo docsRoot should stay present for scaffold links")
            .trim_end_matches('/');
        let language_blob_base = contract.language_repo.blob_base_url;
        let product_repo_url = contract.product_repo.repo_url;
        let product_blob_base = contract.product_repo.blob_base_url;
        let product_handoff_label = contract.product_handoff.label;
        let product_runbook_path = contract.product_handoff.relative_runbook_path;
        let runbook_path = product_runbook_path.trim_start_matches('/').to_owned();

        ScaffoldPublicLinks {
            todo_postgres_readme_url: format!("{language_blob_base}examples/todo-postgres/README.md"),
            todo_sqlite_readme_url: format!("{language_blob_base}examples/todo-sqlite/README.md"),
            production_backend_proof_url: format!("{docs_root}/production-backend-proof/"),
            product_repo_url,
            product_handoff_label,
            product_runbook_path,
            product_runbook_url: format!("{}/{}", product_blob_base.trim_end_matches('/'), runbook_path),
        }
    })
}

fn create_project_dir(name: &str, dir: &Path) -> Result<PathBuf, String> {
    let project_dir = dir.join(name);

    if project_dir.exists() {
        return Err(format!("Directory '{}' already exists", name));
    }

    std::fs::create_dir_all(&project_dir)
        .map_err(|e| format!("Failed to create directory '{}': {}", name, e))?;
    Ok(project_dir)
}

fn write_project_file(path: &Path, contents: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory '{}': {}", parent.display(), e))?;
    }
    std::fs::write(path, contents).map_err(|e| format!("Failed to write {}: {}", path.display(), e))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TodoApiDatabase {
    Sqlite,
    Postgres,
}

/// Create a new hello-world Mesh project with the given name inside the given parent directory.
pub fn scaffold_project(name: &str, dir: &Path) -> Result<(), String> {
    let project_dir = create_project_dir(name, dir)?;

    let manifest = format!(
        r#"[package]
name = "{}"
version = "0.1.0"

[dependencies]
"#,
        name
    );
    write_project_file(&project_dir.join("mesh.toml"), &manifest)?;

    let main_mesh = r#"fn main() do
  IO.puts("Hello from Mesh!")
end
"#;
    write_project_file(&project_dir.join("main.mpl"), main_mesh)?;

    println!("Created project '{}'", name);
    Ok(())
}

/// Create a new clustered Mesh project that uses only the public clustered-app contract.
pub fn scaffold_clustered_project(name: &str, dir: &Path) -> Result<(), String> {
    let project_dir = create_project_dir(name, dir)?;

    let manifest = format!(
        r#"[package]
name = "{}"
version = "0.1.0"

[dependencies]
"#,
        name
    );
    write_project_file(&project_dir.join("mesh.toml"), &manifest)?;

    let main_mesh = r#"fn log_bootstrap(status :: BootstrapStatus) do
  println(
    "[clustered-app] runtime bootstrap mode=#{status.mode} node=#{status.node_name} cluster_port=#{status.cluster_port} discovery_seed=#{status.discovery_seed}"
  )
end

fn log_bootstrap_failure(reason :: String) do
  println("[clustered-app] runtime bootstrap failed reason=#{reason}")
end

fn main() do
  case Node.start_from_env() do
    Ok(status) -> log_bootstrap(status)
    Err(reason) -> log_bootstrap_failure(reason)
  end
end
"#;
    write_project_file(&project_dir.join("main.mpl"), main_mesh)?;

    let work_mesh = r#"@cluster pub fn add() -> Int do
  1 + 1
end
"#;
    write_project_file(&project_dir.join("work.mpl"), work_mesh)?;

    let public_links = scaffold_public_links();
    let readme = format!(
        r#"# {name}

This project was generated by `meshc init --clustered`.

The generated files keep the public route-free clustered-work contract small: `mesh.toml` stays package-only, `work.mpl` declares the clustered work in source with `@cluster`, and `main.mpl` only boots through `Node.start_from_env()` while the runtime owns startup, placement, continuity, and diagnostics.

When you want the public follow-on starters, branch to the generated Mesh repo examples instead of internal proof fixtures: [`examples/todo-postgres/README.md`]({todo_postgres_readme_url}) is the serious shared/deployable PostgreSQL starter, [`examples/todo-sqlite/README.md`]({todo_sqlite_readme_url}) is the honest local single-node SQLite starter, and [Production Backend Proof]({production_backend_proof_url}) is the repo-boundary handoff into the [{product_handoff_label}]({product_repo_url}) once those public starters stop being enough. Keep any product-owned runbook reading on the far side of that handoff at [{product_handoff_runbook}]({product_runbook_url}) instead of teaching local mesh-lang product paths here.

## Package contract

- `mesh.toml` is package-only and intentionally omits `[cluster]`
- `main.mpl` has one bootstrap path: `Node.start_from_env()`
- `work.mpl` defines `@cluster pub fn add()`
- the runtime-owned handler name is derived from the ordinary source function name as `Work.add`
- the visible work body stays `1 + 1`
- the project does not own HTTP routes, submit handlers, or work-delay seams

## Runtime contract

Set these environment variables when you want the app to participate in a cluster:

- `MESH_CLUSTER_COOKIE` — shared cluster cookie used for authenticated node traffic
- `MESH_NODE_NAME` — optional advertised node identity (`name@host:port`); defaults to `app@127.0.0.1:$MESH_CLUSTER_PORT`
- `MESH_DISCOVERY_SEED` — discovery seed used by the runtime DNS discovery loop
- `MESH_CLUSTER_PORT` — node listener port (default `4370`)
- `MESH_CONTINUITY_ROLE` — runtime continuity role (`primary` or `standby`)
- `MESH_CONTINUITY_PROMOTION_EPOCH` — bounded promotion epoch (`0` by default)

The runtime automatically starts the source-declared `@cluster` function and closes the continuity record when it returns.

## Runtime inspection

Once a built node is running in cluster mode, inspect it through Mesh-owned CLI surfaces instead of app-owned routes:

```bash
meshc cluster status <node-name@host:port> --json
meshc cluster continuity <node-name@host:port> --json
meshc cluster continuity <node-name@host:port> <request-key> --json
meshc cluster diagnostics <node-name@host:port> --json
```

Use the list form first to discover request keys and runtime-owned startup records, then inspect a single record when you want the per-request continuity detail.

## Local run example

```bash
meshc build .
MESH_CLUSTER_COOKIE=dev-cookie \
MESH_NODE_NAME=app@127.0.0.1:4370 \
MESH_DISCOVERY_SEED=localhost \
MESH_CLUSTER_PORT=4370 \
MESH_CONTINUITY_ROLE=primary \
MESH_CONTINUITY_PROMOTION_EPOCH=0 \
./{name}
```
"#,
        name = name,
        todo_postgres_readme_url = public_links.todo_postgres_readme_url,
        todo_sqlite_readme_url = public_links.todo_sqlite_readme_url,
        production_backend_proof_url = public_links.production_backend_proof_url,
        product_repo_url = public_links.product_repo_url,
        product_handoff_label = public_links.product_handoff_label,
        product_handoff_runbook = public_links.product_runbook_path,
        product_runbook_url = public_links.product_runbook_url
    );
    write_project_file(&project_dir.join("README.md"), &readme)?;

    println!("Created clustered project '{}'", name);
    Ok(())
}

fn todo_readme(name: &str) -> String {
    r#"# __NAME__

This project was generated by `meshc init --template todo-api --db sqlite`.

It is the honest local starter: a single-node SQLite Todo API, a process-backed write rate limiter, generated package tests, and a Dockerfile that packages the binary produced by `meshc build .`. It does not claim clustered placement, continuity, or operator surfaces.

## Project layout

```text
__NAME__/
  mesh.toml
  main.mpl
  config.mpl
  README.md
  Dockerfile
  .dockerignore
  api/
    health.mpl
    router.mpl
    todos.mpl
  runtime/
    registry.mpl
  services/
    rate_limiter.mpl
  storage/
    todos.mpl
  tests/
    config.test.mpl
    storage.test.mpl
  types/
    todo.mpl
```

## Local contract

- `mesh.toml` stays package-only and intentionally omits `[cluster]`
- `main.mpl` validates local config, creates the SQLite schema, and starts the HTTP server directly
- generated tests under `tests/` prove the local config and storage contract with `meshc test .`
- `/health` reports local SQLite state and rate-limit configuration
- all routes run locally; there is no `work.mpl`, `HTTP.clustered(...)`, or `meshc cluster` story in this starter

## HTTP routes

- `GET /health` — local SQLite + rate-limit configuration snapshot
- `GET /todos` — list todos locally
- `GET /todos/:id` — fetch one todo locally
- `POST /todos` — create from `{"title":"Buy milk"}`
- `PUT /todos/:id` — toggle completion
- `DELETE /todos/:id` — delete a todo

Mutating routes (`POST`, `PUT`, `DELETE`) stay local and go through the actor-backed limiter in `services/rate_limiter.mpl`.

## Environment

- `PORT` — HTTP port (`8080` by default)
- `TODO_DB_PATH` — SQLite database path (`todo.sqlite3` by default)
- `TODO_RATE_LIMIT_WINDOW_SECONDS` — limiter window length (`60` by default)
- `TODO_RATE_LIMIT_MAX_REQUESTS` — allowed mutating requests per window (`5` by default)

Use `:memory:` only for tests or intentionally ephemeral runs. If you set `TODO_DB_PATH`, it must be a non-empty path.

## Local verification

```bash
meshc test .
meshc build .
PORT=8080 \
TODO_DB_PATH=./todo.sqlite3 \
TODO_RATE_LIMIT_WINDOW_SECONDS=60 \
TODO_RATE_LIMIT_MAX_REQUESTS=5 \
./__NAME__
```

## Clustered and deployable paths

When you need clustered/runtime-owned execution or a shared-database starter, use one of the explicit paths instead of stretching this SQLite template past its contract:

```bash
meshc init --template todo-api --db postgres my-shared-todo
meshc init --clustered my-clustered-app
```

## Docker

Build the Linux binary you want to ship, then package that `./output` artifact into the image:

```bash
meshc build .
docker build -t __NAME__ .
```

Re-run `meshc build .` before rebuilding the image, and build on the Linux target you plan to run because the Dockerfile copies the already-compiled `./output` binary into the container. If you're driving Docker from macOS or Windows, emit `./output` from a Linux builder host, CI job, or container first.

Run it with a persisted SQLite volume:

```bash
docker run --rm \
  -p 8080:8080 \
  -e PORT=8080 \
  -e TODO_DB_PATH=/var/lib/todo/todo.sqlite3 \
  -v todo-data:/var/lib/todo \
  __NAME__
```
"#
    .replace("__NAME__", name)
}

fn todo_dockerfile(name: &str) -> String {
    r#"FROM ubuntu:24.04
RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY output /usr/local/bin/__NAME__
ENV PORT=8080
ENV TODO_DB_PATH=/var/lib/todo/todo.sqlite3
ENV TODO_RATE_LIMIT_WINDOW_SECONDS=60
ENV TODO_RATE_LIMIT_MAX_REQUESTS=5
VOLUME ["/var/lib/todo"]
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/__NAME__"]
"#
    .replace("__NAME__", name)
}

const TODO_POSTGRES_MIGRATION_FILENAME: &str = "20260402120000_create_todos.mpl";

fn todo_postgres_deploy_sql_filename(name: &str) -> String {
    format!("{name}.up.sql")
}

fn postgres_todo_readme(name: &str) -> String {
    r#"# __NAME__

This project was generated by `meshc init --template todo-api --db postgres`.

It keeps the clustered-function contract source-first and route-free while moving the starter onto the serious PostgreSQL path: `DATABASE_URL` configuration, pool-backed startup, migration-owned schema creation, a staged deploy bundle, helper-backed CRUD, and a Dockerfile that packages the binary produced by `meshc build .`. One public app URL may front multiple starter nodes, but Mesh runtime ingress/owner/replica/execution truth is still inspected through `meshc cluster` instead of frontend-aware node selection.

## Project layout

```text
__NAME__/
  mesh.toml
  main.mpl
  work.mpl
  config.mpl
  README.md
  Dockerfile
  .dockerignore
  .env.example
  api/
    health.mpl
    router.mpl
    todos.mpl
  deploy/
    __DEPLOY_SQL__
  migrations/
    __MIGRATION__
  runtime/
    registry.mpl
  scripts/
    apply-deploy-migrations.sh
    deploy-smoke.sh
    stage-deploy.sh
  services/
    rate_limiter.mpl
  storage/
    todos.mpl
  tests/
    config.test.mpl
  types/
    todo.mpl
```

## Clustered contract

- `mesh.toml` stays package-only and intentionally omits `[cluster]`
- `main.mpl` validates config, opens the local PostgreSQL pool, then boots through `Node.start_from_env()` before starting the local HTTP server
- `work.mpl` declares `@cluster pub fn sync_todos()`
- the runtime-owned handler name is derived from the ordinary source function name as `Work.sync_todos`
- `GET /todos` and `GET /todos/:id` use `HTTP.clustered(1, ...)` so the starter truthfully dogfoods the shipped wrapper while one public `BASE_URL` can sit behind a proxy/platform ingress in front of multiple nodes; `GET /health` and mutating routes stay local

## HTTP routes

- `GET /health` — local runtime + PostgreSQL/starter configuration snapshot without leaking `DATABASE_URL`
- `GET /todos` — list todos through `HTTP.clustered(1, ...)`
- `GET /todos/:id` — fetch one todo through `HTTP.clustered(1, ...)`
- `POST /todos` — create from `{"title":"Buy milk"}`
- `PUT /todos/:id` — toggle completion
- `DELETE /todos/:id` — delete a todo

Mutating routes (`POST`, `PUT`, `DELETE`) stay local and go through the actor-backed limiter in `services/rate_limiter.mpl`.

## One public URL, runtime-owned placement

- `BASE_URL` in `deploy-smoke.sh` is the single public app URL for this starter. Locally it can point straight at `http://127.0.0.1:8080`; behind a proxy/platform ingress it can front multiple starter nodes.
- Clustered `GET /todos` and `GET /todos/:id` responses include a runtime-owned `X-Mesh-Continuity-Request-Key` response header.
- Treat that header as an operator/debug seam: take the returned request key and run `meshc cluster continuity <node-name@host:port> <request-key> --json` against a node when you want the same request's continuity record directly.
- Use `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics` against node addresses when you need the same request's ingress, owner, replica, or execution truth.
- The response header is a runtime-owned operator/debug seam, not a frontend-aware routing signal.
- The starter does not promise frontend-aware node selection, sticky-session semantics, or a Fly-specific product contract.
- If you only need a local-only single-node path, stay on `meshc init --template todo-api --db sqlite`.

## Schema and migrations

`migrations/` owns the source-tree schema changes. Apply those migrations before booting the API directly from source:

```bash
meshc migrate . status
meshc migrate . up
```

`deploy/__DEPLOY_SQL__` is the staged bundle artifact for production-like replays. The runtime opens the pool and serves HTTP only after configuration succeeds. It does not create tables at startup.

## Environment

- `DATABASE_URL` — required PostgreSQL connection string
- `PORT` — HTTP port (`8080` by default)
- `TODO_RATE_LIMIT_WINDOW_SECONDS` — limiter window length (`60` by default)
- `TODO_RATE_LIMIT_MAX_REQUESTS` — allowed mutating requests per window (`5` by default)
- `.env.example` — starter copy of the same local-development keys
- `MESH_CLUSTER_COOKIE`, `MESH_NODE_NAME`, `MESH_DISCOVERY_SEED`, `MESH_CLUSTER_PORT`, `MESH_CONTINUITY_ROLE`, `MESH_CONTINUITY_PROMOTION_EPOCH` — optional clustered-runtime env when you want the runtime-owned operator flow

## Local run

```bash
meshc migrate . up
meshc build .
DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/__NAME__ \
PORT=8080 \
TODO_RATE_LIMIT_WINDOW_SECONDS=60 \
TODO_RATE_LIMIT_MAX_REQUESTS=5 \
./__NAME__
```

## Staged deploy bundle

Stage a production-like bundle outside the source tree:

```bash
bash scripts/stage-deploy.sh ./.bundle/__NAME__
```

The staged directory contains:

- `./.bundle/__NAME__/__NAME__` — built starter binary
- `./.bundle/__NAME__/__DEPLOY_SQL__` — deploy SQL artifact copied from `deploy/`
- `./.bundle/__NAME__/apply-deploy-migrations.sh` — `psql`-based schema apply helper
- `./.bundle/__NAME__/deploy-smoke.sh` — health + CRUD smoke helper

Each script emits phase-tagged logs: `[stage-deploy]`, `[deploy-apply]`, and `[deploy-smoke]`.

## Production-like replay

Apply the staged schema, boot the staged binary, then smoke the running starter through one public `BASE_URL`:

```bash
DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/__NAME__ \
bash ./.bundle/__NAME__/apply-deploy-migrations.sh ./.bundle/__NAME__/__DEPLOY_SQL__

DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/__NAME__ \
PORT=8080 \
TODO_RATE_LIMIT_WINDOW_SECONDS=60 \
TODO_RATE_LIMIT_MAX_REQUESTS=5 \
MESH_CLUSTER_COOKIE=dev-cookie \
MESH_NODE_NAME=__NAME__@127.0.0.1:4370 \
MESH_DISCOVERY_SEED=localhost \
MESH_CLUSTER_PORT=4370 \
MESH_CONTINUITY_ROLE=primary \
MESH_CONTINUITY_PROMOTION_EPOCH=0 \
./.bundle/__NAME__/__NAME__
```

The smoke helper treats `BASE_URL` as the public app URL. In local replay it can point directly at the starter port; behind a proxy/platform ingress it can front multiple nodes without changing the public contract.

```bash
DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/__NAME__ \
PORT=8080 \
BASE_URL=http://127.0.0.1:8080 \
bash ./.bundle/__NAME__/deploy-smoke.sh
```

The staged bundle is the public deploy contract. Hosted runtimes or managed PostgreSQL providers can sit behind the same bundle later, and a proxy/platform ingress may expose one public app URL in front of multiple nodes, but Mesh runtime ownership still stays on the `meshc cluster` inspection path and the starter does not promise frontend-aware node selection or a Fly-specific product contract.

## Runtime inspection

When you boot the app in clustered mode, inspect the runtime-owned state through Mesh CLI surfaces instead of package-owned control routes:

```bash
meshc cluster status <node-name@host:port> --json
meshc cluster continuity <node-name@host:port> --json
meshc cluster continuity <node-name@host:port> <request-key> --json
meshc cluster diagnostics <node-name@host:port> --json
```

Use the continuity list form first to discover runtime-owned startup records or for general manual investigation.

## Docker

Build the Linux binary you want to ship, then package that `./output` artifact into the image:

```bash
meshc build .
docker build -t __NAME__ .
```

The container expects a migrated database and a runtime `DATABASE_URL`; it does not run migrations or create schema at startup. The same staged SQL artifact is the contract for applying schema before the container boots. Re-run `meshc build .` before rebuilding the image, and build on the Linux target you plan to run because the Dockerfile copies the already-compiled `./output` binary into the container.

```bash
docker run --rm \
  -p 8080:8080 \
  -e PORT=8080 \
  -e DATABASE_URL=postgres://postgres:postgres@host.docker.internal:5432/__NAME__ \
  __NAME__
```
"#
    .replace("__NAME__", name)
    .replace("__MIGRATION__", TODO_POSTGRES_MIGRATION_FILENAME)
    .replace("__DEPLOY_SQL__", &todo_postgres_deploy_sql_filename(name))
}

fn postgres_todo_dockerfile(name: &str) -> String {
    r#"FROM ubuntu:24.04
RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY output /usr/local/bin/__NAME__
ENV PORT=8080
ENV TODO_RATE_LIMIT_WINDOW_SECONDS=60
ENV TODO_RATE_LIMIT_MAX_REQUESTS=5
ENV MESH_CLUSTER_PORT=4370
EXPOSE 8080 4370
ENTRYPOINT ["/usr/local/bin/__NAME__"]
"#
    .replace("__NAME__", name)
}

fn postgres_todo_config_mesh() -> &'static str {
    r#"pub fn database_url_key() -> String do
  "DATABASE_URL"
end

pub fn port_key() -> String do
  "PORT"
end

pub fn todo_rate_limit_window_seconds_key() -> String do
  "TODO_RATE_LIMIT_WINDOW_SECONDS"
end

pub fn todo_rate_limit_max_requests_key() -> String do
  "TODO_RATE_LIMIT_MAX_REQUESTS"
end

pub fn missing_required_env(name :: String) -> String do
  "Missing required environment variable #{name}"
end

pub fn invalid_positive_int(name :: String) -> String do
  "Invalid #{name}: expected a positive integer"
end
"#
}

fn postgres_todo_config_test() -> &'static str {
    r#"from Config import database_url_key, port_key, todo_rate_limit_window_seconds_key, todo_rate_limit_max_requests_key, missing_required_env, invalid_positive_int

describe("Config helpers") do
  test("exposes the canonical environment variable keys") do
    assert_eq(database_url_key(), "DATABASE_URL")
    assert_eq(port_key(), "PORT")
    assert_eq(todo_rate_limit_window_seconds_key(), "TODO_RATE_LIMIT_WINDOW_SECONDS")
    assert_eq(todo_rate_limit_max_requests_key(), "TODO_RATE_LIMIT_MAX_REQUESTS")
  end

  test("formats missing-env and invalid-int messages") do
    assert_eq(missing_required_env(database_url_key()), "Missing required environment variable DATABASE_URL")
    assert_eq(invalid_positive_int(todo_rate_limit_window_seconds_key()), "Invalid TODO_RATE_LIMIT_WINDOW_SECONDS: expected a positive integer")
    assert_eq(invalid_positive_int(todo_rate_limit_max_requests_key()), "Invalid TODO_RATE_LIMIT_MAX_REQUESTS: expected a positive integer")
  end
end
"#
}

fn postgres_todo_env_example(name: &str) -> String {
    r#"DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/__NAME__
PORT=8080
TODO_RATE_LIMIT_WINDOW_SECONDS=60
TODO_RATE_LIMIT_MAX_REQUESTS=5
MESH_CLUSTER_COOKIE=dev-cookie
MESH_NODE_NAME=__NAME__@127.0.0.1:4370
MESH_DISCOVERY_SEED=localhost
MESH_CLUSTER_PORT=4370
MESH_CONTINUITY_ROLE=primary
MESH_CONTINUITY_PROMOTION_EPOCH=0
"#
    .replace("__NAME__", name)
}

fn postgres_todo_stage_deploy_script(name: &str) -> String {
    r#"#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
PACKAGE_NAME="__NAME__"
DEPLOY_SQL_NAME="__DEPLOY_SQL__"
BUNDLE_DIR="${1:-}"
SOURCE_SQL="$PROJECT_DIR/deploy/$DEPLOY_SQL_NAME"
SOURCE_APPLY_SCRIPT="$PROJECT_DIR/scripts/apply-deploy-migrations.sh"
SOURCE_SMOKE_SCRIPT="$PROJECT_DIR/scripts/deploy-smoke.sh"
ACCIDENTAL_BINARY="$PROJECT_DIR/$PACKAGE_NAME"
TARGET_BINARY=""
TARGET_SQL=""
TARGET_APPLY_SCRIPT=""
TARGET_SMOKE_SCRIPT=""

usage() {
  echo "usage: bash scripts/stage-deploy.sh <bundle-dir>" >&2
}

fail() {
  echo "[stage-deploy] $1" >&2
  exit 1
}

require_command() {
  local command_name="$1"
  if ! command -v "$command_name" >/dev/null 2>&1; then
    fail "required command missing from PATH: $command_name"
  fi
}

require_file() {
  local label="$1"
  local path="$2"
  if [[ ! -f "$path" ]]; then
    fail "missing required ${label}: $path"
  fi
}

ensure_source_tree_clean() {
  if [[ -e "$ACCIDENTAL_BINARY" ]]; then
    fail "source tree contains an in-place binary: $ACCIDENTAL_BINARY"
  fi
}

if [[ $# -ne 1 || -z "$BUNDLE_DIR" ]]; then
  usage
  exit 1
fi

if [[ -e "$BUNDLE_DIR" && ! -d "$BUNDLE_DIR" ]]; then
  fail "bundle path exists but is not a directory: $BUNDLE_DIR"
fi

require_command meshc
require_file "project manifest" "$PROJECT_DIR/mesh.toml"
require_file "deploy SQL artifact" "$SOURCE_SQL"
require_file "deploy migration script" "$SOURCE_APPLY_SCRIPT"
require_file "deploy smoke script" "$SOURCE_SMOKE_SCRIPT"
ensure_source_tree_clean

mkdir -p "$BUNDLE_DIR"
TARGET_BINARY="$BUNDLE_DIR/$PACKAGE_NAME"
TARGET_SQL="$BUNDLE_DIR/$DEPLOY_SQL_NAME"
TARGET_APPLY_SCRIPT="$BUNDLE_DIR/apply-deploy-migrations.sh"
TARGET_SMOKE_SCRIPT="$BUNDLE_DIR/deploy-smoke.sh"
rm -f "$TARGET_BINARY" "$TARGET_SQL" "$TARGET_APPLY_SCRIPT" "$TARGET_SMOKE_SCRIPT"

printf '[stage-deploy] building starter binary project=%s\n' "$PROJECT_DIR"
(
  cd "$PROJECT_DIR"
  meshc build . --output "$TARGET_BINARY"
)

ensure_source_tree_clean

if [[ ! -f "$TARGET_BINARY" ]]; then
  fail "meshc build reported success but the staged binary is missing: $TARGET_BINARY"
fi

if [[ ! -x "$TARGET_BINARY" ]]; then
  fail "staged binary is not executable: $TARGET_BINARY"
fi

printf '[stage-deploy] staging bundle dir=%s\n' "$BUNDLE_DIR"
cp "$SOURCE_SQL" "$TARGET_SQL"
cp "$SOURCE_APPLY_SCRIPT" "$TARGET_APPLY_SCRIPT"
cp "$SOURCE_SMOKE_SCRIPT" "$TARGET_SMOKE_SCRIPT"
chmod 755 "$TARGET_BINARY" "$TARGET_APPLY_SCRIPT" "$TARGET_SMOKE_SCRIPT"

printf '[stage-deploy] staged layout\n'
for staged_path in \
  "$TARGET_BINARY" \
  "$TARGET_SQL" \
  "$TARGET_APPLY_SCRIPT" \
  "$TARGET_SMOKE_SCRIPT"
do
  printf '[stage-deploy] - %s\n' "$staged_path"
done

printf '[stage-deploy] bundle ready dir=%s\n' "$BUNDLE_DIR"
"#
    .replace("__NAME__", name)
    .replace("__DEPLOY_SQL__", &todo_postgres_deploy_sql_filename(name))
}

fn postgres_todo_apply_deploy_migrations_script() -> &'static str {
    r#"#!/usr/bin/env bash
set -euo pipefail

SQL_PATH="${1:-}"

usage() {
  echo "usage: bash apply-deploy-migrations.sh <deploy-sql-path>" >&2
}

fail() {
  echo "[deploy-apply] $1" >&2
  exit 1
}

if [[ $# -ne 1 || -z "$SQL_PATH" ]]; then
  usage
  exit 1
fi

if ! command -v psql >/dev/null 2>&1; then
  fail "psql is required but was not found on PATH"
fi

if [[ ! -f "$SQL_PATH" ]]; then
  fail "missing deploy SQL artifact: $SQL_PATH"
fi

if [[ -z "${DATABASE_URL:-}" ]]; then
  fail "DATABASE_URL must be set"
fi

printf '[deploy-apply] sql artifact=%s\n' "$SQL_PATH"
printf '[deploy-apply] applying starter schema via psql\n'
psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -f "$SQL_PATH" >/dev/null

todos_table_present="$(psql "$DATABASE_URL" -Atqc "SELECT to_regclass('public.todos') IS NOT NULL")"
if [[ "$todos_table_present" != "t" ]]; then
  fail "todos table missing after apply"
fi

todos_index_present="$(psql "$DATABASE_URL" -Atqc "SELECT to_regclass('public.idx_todos_created_at') IS NOT NULL")"
if [[ "$todos_index_present" != "t" ]]; then
  fail "idx_todos_created_at missing after apply"
fi

printf '[deploy-apply] schema ready table=todos index=idx_todos_created_at\n'
"#
}

fn postgres_todo_deploy_smoke_script() -> &'static str {
    r#"#!/usr/bin/env bash
set -euo pipefail

DEFAULT_PORT="8080"
PORT_VALUE="${PORT:-$DEFAULT_PORT}"
BASE_URL="${BASE_URL:-http://127.0.0.1:${PORT_VALUE}}"
LAST_HEALTH_RESPONSE=""
LAST_RESPONSE=""

usage() {
  echo "usage: bash deploy-smoke.sh" >&2
}

fail() {
  echo "[deploy-smoke] $1" >&2
  exit 1
}

require_command() {
  local command_name="$1"
  if ! command -v "$command_name" >/dev/null 2>&1; then
    fail "required command missing from PATH: $command_name"
  fi
}

json_field() {
  local field="$1"
  python3 -c '
import json
import sys

field = sys.argv[1]
data = json.load(sys.stdin)
value = data
for key in field.split("."):
    if not isinstance(value, dict):
        sys.exit(1)
    value = value.get(key)
    if value is None:
        sys.exit(1)
if isinstance(value, bool):
    print("true" if value else "false")
elif isinstance(value, (dict, list)):
    print(json.dumps(value, separators=(",", ":")))
else:
    print(value)
' "$field"
}

list_length() {
  python3 -c '
import json
import sys

data = json.load(sys.stdin)
if not isinstance(data, list):
    sys.exit(1)
print(len(data))
'
}

list_contains_id() {
  local expected_id="$1"
  python3 -c '
import json
import sys

expected_id = sys.argv[1]
data = json.load(sys.stdin)
if not isinstance(data, list):
    sys.exit(1)
print("true" if any(isinstance(item, dict) and item.get("id") == expected_id for item in data) else "false")
' "$expected_id"
}

if [[ $# -ne 0 ]]; then
  usage
  exit 1
fi

for required_command in curl python3; do
  require_command "$required_command"
done

if [[ ! "$PORT_VALUE" =~ ^[1-9][0-9]*$ ]]; then
  fail "PORT must be a positive integer, got: $PORT_VALUE"
fi

case "$BASE_URL" in
  http://*|https://*) ;;
  *) fail "BASE_URL must start with http:// or https://, got: $BASE_URL" ;;
esac

printf '[deploy-smoke] waiting for health base_url=%s\n' "$BASE_URL"
for attempt in $(seq 1 80); do
  if health_response="$(curl -fsS "$BASE_URL/health" 2>/dev/null)"; then
    LAST_HEALTH_RESPONSE="$health_response"
    health_status="$(printf '%s' "$health_response" | json_field status || true)"
    db_backend="$(printf '%s' "$health_response" | json_field db_backend || true)"
    clustered_handler="$(printf '%s' "$health_response" | json_field clustered_handler || true)"
    printf '[deploy-smoke] health poll=%s status=%s db_backend=%s clustered_handler=%s\n' \
      "$attempt" "${health_status:-missing}" "${db_backend:-missing}" "${clustered_handler:-missing}"
    if [[ "$health_status" == "ok" && "$db_backend" == "postgres" && "$clustered_handler" == "Work.sync_todos" ]]; then
      printf '[deploy-smoke] health ready body=%s\n' "$health_response"
      break
    fi
  fi
  sleep 0.25
  if [[ "$attempt" == "80" ]]; then
    fail "/health never became ready at $BASE_URL; last_body=${LAST_HEALTH_RESPONSE:-unavailable}"
  fi
done

payload='{"title":"deploy smoke todo"}'
printf '[deploy-smoke] creating todo via POST %s/todos\n' "$BASE_URL"
create_response="$(curl -fsS -X POST "$BASE_URL/todos" -H 'content-type: application/json' -d "$payload")"
printf '[deploy-smoke] created todo body=%s\n' "$create_response"
TODO_ID="$(printf '%s' "$create_response" | json_field id || true)"
TITLE="$(printf '%s' "$create_response" | json_field title || true)"
if [[ -z "$TODO_ID" ]]; then
  fail "created todo response did not include id"
fi
if [[ "$TITLE" != "deploy smoke todo" ]]; then
  fail "created todo response title drifted: $create_response"
fi

printf '[deploy-smoke] fetching todo id=%s\n' "$TODO_ID"
get_response="$(curl -fsS "$BASE_URL/todos/$TODO_ID")"
get_title="$(printf '%s' "$get_response" | json_field title || true)"
if [[ "$get_title" != "deploy smoke todo" ]]; then
  fail "GET /todos/$TODO_ID returned unexpected body: $get_response"
fi

printf '[deploy-smoke] toggling todo id=%s\n' "$TODO_ID"
toggle_response="$(curl -fsS -X PUT "$BASE_URL/todos/$TODO_ID")"
completed="$(printf '%s' "$toggle_response" | json_field completed || true)"
if [[ "$completed" != "true" ]]; then
  fail "toggle response did not mark the todo completed: $toggle_response"
fi

printf '[deploy-smoke] listing todos\n'
list_response="$(curl -fsS "$BASE_URL/todos")"
list_count="$(printf '%s' "$list_response" | list_length || true)"
list_has_id="$(printf '%s' "$list_response" | list_contains_id "$TODO_ID" || true)"
if [[ -z "$list_count" || "$list_count" == "0" ]]; then
  fail "expected GET /todos to return at least one todo, got: $list_response"
fi
if [[ "$list_has_id" != "true" ]]; then
  fail "expected GET /todos to include id=$TODO_ID, got: $list_response"
fi

printf '[deploy-smoke] deleting todo id=%s\n' "$TODO_ID"
delete_response="$(curl -fsS -X DELETE "$BASE_URL/todos/$TODO_ID")"
deleted_id="$(printf '%s' "$delete_response" | json_field id || true)"
if [[ "$deleted_id" != "$TODO_ID" ]]; then
  fail "delete response did not report the created todo id: $delete_response"
fi

final_list="$(curl -fsS "$BASE_URL/todos")"
final_has_id="$(printf '%s' "$final_list" | list_contains_id "$TODO_ID" || true)"
if [[ "$final_has_id" != "false" ]]; then
  fail "expected deleted todo id=$TODO_ID to disappear from GET /todos, got: $final_list"
fi

printf '[deploy-smoke] CRUD smoke passed id=%s\n' "$TODO_ID"
printf '%s\n' "$final_list"
"#
}

fn postgres_todo_deploy_sql() -> &'static str {
    r#"-- Deploy artifact for the Postgres todo-api starter.
-- Apply this with scripts/apply-deploy-migrations.sh or directly through psql.

BEGIN;

CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS todos (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  title TEXT NOT NULL,
  completed BOOLEAN NOT NULL DEFAULT false,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_todos_created_at ON todos (created_at DESC);

COMMIT;
"#
}

fn postgres_todo_main_mesh() -> &'static str {
    r#"from Config import database_url_key, port_key, todo_rate_limit_max_requests_key, todo_rate_limit_window_seconds_key, invalid_positive_int, missing_required_env
from Api.Router import build_router
from Runtime.Registry import start_registry
from Services.RateLimiter import start_rate_limiter

fn log_bootstrap(status :: BootstrapStatus) do
  println(
    "[todo-api] runtime bootstrap mode=#{status.mode} node=#{status.node_name} cluster_port=#{status.cluster_port} discovery_seed=#{status.discovery_seed}"
  )
end

fn log_bootstrap_failure(reason :: String) do
  println("[todo-api] runtime bootstrap failed reason=#{reason}")
end

fn log_config_error(message :: String) do
  println("[todo-api] Config error: #{message}")
end

fn optional_positive_env_int(name :: String, default_value :: Int) -> Int ! String do
  let raw = Env.get(name, "")
  if raw == "" do
    Ok(default_value)
  else
    let value = Env.get_int(name, -1)
    if value > 0 do
      Ok(value)
    else
      Err(invalid_positive_int(name))
    end
  end
end

fn start_runtime(port :: Int, window_seconds :: Int, max_requests :: Int) do
  println(
    "[todo-api] Runtime ready port=#{port} db_backend=postgres write_limit_window_seconds=#{window_seconds} write_limit_max=#{max_requests}"
  )
  let router = build_router()
  println("[todo-api] HTTP server starting on :#{port}")
  HTTP.serve(router, port)
end

fn on_pool_ready(port :: Int, window_seconds :: Int, max_requests :: Int, pool :: PoolHandle) do
  println("[todo-api] PostgreSQL pool ready")
  let limiter_pid = start_rate_limiter(window_seconds, max_requests)
  start_registry(pool, limiter_pid, window_seconds, max_requests)
  println("[todo-api] Runtime registry ready")
  start_runtime(port, window_seconds, max_requests)
end

fn maybe_boot_with_pool(port :: Int, window_seconds :: Int, max_requests :: Int, pool :: PoolHandle) do
  case Node.start_from_env() do
    Ok( status) -> do
      log_bootstrap(status)
      on_pool_ready(port, window_seconds, max_requests, pool)
    end
    Err( reason) -> log_bootstrap_failure(reason)
  end
end

fn start_with_values(database_url :: String, port :: Int, window_seconds :: Int, max_requests :: Int) do
  println(
    "[todo-api] Config loaded port=#{port} write_limit_window_seconds=#{window_seconds} write_limit_max=#{max_requests}"
  )
  println("[todo-api] Connecting to PostgreSQL pool...")
  let pool_result = Pool.open(database_url, 1, 4, 5000)
  case pool_result do
    Ok( pool) -> maybe_boot_with_pool(port, window_seconds, max_requests, pool)
    Err( e) -> println("[todo-api] PostgreSQL connect failed: #{e}")
  end
end

fn maybe_start_with_max_requests(database_url :: String, port :: Int, window_seconds :: Int) do
  let max_requests_env = todo_rate_limit_max_requests_key()
  case optional_positive_env_int(max_requests_env, 5) do
    Ok( max_requests) -> start_with_values(database_url, port, window_seconds, max_requests)
    Err( message) -> log_config_error(message)
  end
end

fn maybe_start_with_window_seconds(database_url :: String, port :: Int) do
  let window_seconds_env = todo_rate_limit_window_seconds_key()
  case optional_positive_env_int(window_seconds_env, 60) do
    Ok( window_seconds) -> maybe_start_with_max_requests(database_url, port, window_seconds)
    Err( message) -> log_config_error(message)
  end
end

fn maybe_start_with_port(database_url :: String) do
  let port_env = port_key()
  case optional_positive_env_int(port_env, 8080) do
    Ok( port) -> maybe_start_with_window_seconds(database_url, port)
    Err( message) -> log_config_error(message)
  end
end

fn main() do
  let database_url_env = database_url_key()
  let database_url = Env.get(database_url_env, "")
  if database_url == "" do
    log_config_error(missing_required_env(database_url_env))
  else
    maybe_start_with_port(database_url)
  end
end
"#
}

fn postgres_todo_health_mesh() -> &'static str {
    r#"from Runtime.Registry import get_max_requests, get_window_seconds

pub fn handle_health(_request) do
  HTTP.response(200,
  json {
    status : "ok",
    db_backend : "postgres",
    migration_strategy : "meshc migrate",
    clustered_handler : "Work.sync_todos",
    rate_limit_window_seconds : get_window_seconds(),
    rate_limit_max_requests : get_max_requests()
  })
end
"#
}

fn postgres_todo_todos_api_mesh() -> &'static str {
    r#"from Runtime.Registry import get_pool, get_rate_limiter
from Services.RateLimiter import allow_write
from Storage.Todos import create_todo, delete_todo, get_todo, list_todos, toggle_todo
from Types.Todo import Todo

fn todo_to_json(todo :: Todo) -> String do
  Json.encode(todo)
end

fn require_param(request, name :: String) -> String do
  let value = Request.param(request, name)
  case value do
    Some( param) -> param
    None -> ""
  end
end

fn title_from_body(body :: String) -> String do
  String.trim(Json.get(body, "title"))
end

fn not_found_response() do
  HTTP.response(404, json { error : "todo not found" })
end

fn rate_limited_response() do
  HTTP.response(429, json { error : "rate limited" })
end

fn internal_error_response(reason :: String) do
  HTTP.response(500, json { error : reason })
end

fn todo_error_response(reason :: String) do
  if reason == "todo not found" do
    not_found_response()
  else
    internal_error_response(reason)
  end
end

fn allow_mutation() -> Bool do
  let limiter_pid = get_rate_limiter()
  allow_write(limiter_pid, "todo-write")
end

fn create_todo_with_title(pool :: PoolHandle, title :: String) do
  let result = create_todo(pool, title)
  case result do
    Ok( todo) -> HTTP.response(201, todo_to_json(todo))
    Err( reason) -> internal_error_response(reason)
  end
end

fn create_todo_with_body(pool :: PoolHandle, body :: String) do
  let title = title_from_body(body)
  if String.length(title) == 0 do
    HTTP.response(400, json { error : "title is required" })
  else
    create_todo_with_title(pool, title)
  end
end

fn get_todo_response(pool :: PoolHandle, id :: String) do
  let result = get_todo(pool, id)
  case result do
    Ok( todo) -> HTTP.response(200, todo_to_json(todo))
    Err( reason) -> todo_error_response(reason)
  end
end

fn toggle_todo_response(pool :: PoolHandle, id :: String) do
  let result = toggle_todo(pool, id)
  case result do
    Ok( todo) -> HTTP.response(200, todo_to_json(todo))
    Err( reason) -> todo_error_response(reason)
  end
end

fn delete_todo_response(pool :: PoolHandle, id :: String) do
  let result = delete_todo(pool, id)
  case result do
    Ok( deleted_id) -> HTTP.response(200, json { status : "deleted", id : deleted_id })
    Err( reason) -> todo_error_response(reason)
  end
end

pub fn handle_list_todos(_request :: Request) -> Response do
  let pool = get_pool()
  let result = list_todos(pool)
  case result do
    Ok( todos_json) -> HTTP.response(200, todos_json)
    Err( reason) -> internal_error_response(reason)
  end
end

pub fn handle_get_todo(request :: Request) -> Response do
  let pool = get_pool()
  let id = require_param(request, "id")
  get_todo_response(pool, id)
end

pub fn handle_create_todo(request) do
  if allow_mutation() do
    let pool = get_pool()
    create_todo_with_body(pool, Request.body(request))
  else
    rate_limited_response()
  end
end

pub fn handle_toggle_todo(request) do
  if allow_mutation() do
    let pool = get_pool()
    let id = require_param(request, "id")
    toggle_todo_response(pool, id)
  else
    rate_limited_response()
  end
end

pub fn handle_delete_todo(request) do
  if allow_mutation() do
    let pool = get_pool()
    let id = require_param(request, "id")
    delete_todo_response(pool, id)
  else
    rate_limited_response()
  end
end
"#
}

fn postgres_todo_registry_mesh() -> &'static str {
    r#"struct RegistryState do
  pool :: PoolHandle
  rate_limiter_pid :: Pid
  window_seconds :: Int
  max_requests :: Int
end

service TodoRegistry do
  fn init(pool :: PoolHandle, rate_limiter_pid :: Pid, window_seconds :: Int, max_requests :: Int) -> RegistryState do
    RegistryState {
      pool : pool,
      rate_limiter_pid : rate_limiter_pid,
      window_seconds : window_seconds,
      max_requests : max_requests
    }
  end

  call GetPool() :: PoolHandle do|state|
    (state, state.pool)
  end

  call GetRateLimiter() :: Pid do|state|
    (state, state.rate_limiter_pid)
  end

  call GetWindowSeconds() :: Int do|state|
    (state, state.window_seconds)
  end

  call GetMaxRequests() :: Int do|state|
    (state, state.max_requests)
  end
end

pub fn start_registry(pool :: PoolHandle, rate_limiter_pid :: Pid, window_seconds :: Int, max_requests :: Int) do
  let registry_pid = TodoRegistry.start(pool, rate_limiter_pid, window_seconds, max_requests)
  Process.register("todo_api_registry", registry_pid)
  registry_pid
end

pub fn get_pool() do
  let registry_pid = Process.whereis("todo_api_registry")
  TodoRegistry.get_pool(registry_pid)
end

pub fn get_rate_limiter() -> Pid do
  let registry_pid = Process.whereis("todo_api_registry")
  TodoRegistry.get_rate_limiter(registry_pid)
end

pub fn get_window_seconds() -> Int do
  let registry_pid = Process.whereis("todo_api_registry")
  TodoRegistry.get_window_seconds(registry_pid)
end

pub fn get_max_requests() -> Int do
  let registry_pid = Process.whereis("todo_api_registry")
  TodoRegistry.get_max_requests(registry_pid)
end
"#
}

fn postgres_todo_storage_mesh() -> &'static str {
    r#"from Types.Todo import Todo

fn todos_table() -> String do
  "todos"
end

fn row_to_todo(row) -> Todo do
  Todo {
    id : Map.get(row, "id"),
    title : Map.get(row, "title"),
    completed : Map.get(row, "completed") == "true",
    created_at : Map.get(row, "created_at")
  }
end

fn rows_to_json_loop(rows, index :: Int, total :: Int, acc :: List < String >) -> List < String > do
  if index >= total do
    acc
  else
    let encoded = Json.encode(row_to_todo(List.get(rows, index)))
    rows_to_json_loop(rows, index + 1, total, List.append(acc, encoded))
  end
end

fn todo_select_query() do
  Query.from(todos_table())
    |> Query.select_exprs([Expr.label(Pg.text(Expr.column("id")), "id"), Expr.label(Expr.column("title"), "title"), Expr.label(Pg.text(Expr.column("completed")), "completed"), Expr.label(Pg.text(Expr.column("created_at")), "created_at")])
end

fn todo_query_by_id(id :: String) do
  todo_select_query()
    |> Query.where_expr(Expr.eq(Expr.column("id"), Pg.uuid(Expr.value(id))))
end

fn find_single_todo(rows, missing_message :: String) -> Todo ! String do
  if List.length(rows) > 0 do
    Ok(row_to_todo(List.head(rows)))
  else
    Err(missing_message)
  end
end

fn bool_expr(value :: Bool) do
  if value do
    Pg.cast(Expr.value("true"), "boolean")
  else
    Pg.cast(Expr.value("false"), "boolean")
  end
end

fn update_completed_value(pool :: PoolHandle, id :: String, next_completed) -> Todo ! String do
  let q = Query.from(todos_table())
    |> Query.where_expr(Expr.eq(Expr.column("id"), Pg.uuid(Expr.value(id))))
  let updated_result = Repo.update_where_expr(pool, todos_table(), %{"completed" => next_completed}, q)
  case updated_result do
    Ok( _row) -> get_todo(pool, id)
    Err( reason) -> if String.contains(reason, "no rows matched") do
      Err("todo not found")
    else
      Err(reason)
    end
  end
end

fn continue_toggle_todo(pool :: PoolHandle, id :: String, current :: Todo) -> Todo ! String do
  let next_completed = if current.completed do
    bool_expr(false)
  else
    bool_expr(true)
  end
  update_completed_value(pool, id, next_completed)
end

fn delete_todo_with_current(pool :: PoolHandle, id :: String, current :: Todo) -> String ! String do
  let q = Query.from(todos_table())
    |> Query.where_expr(Expr.eq(Expr.column("id"), Pg.uuid(Expr.value(id))))
  let deleted = Repo.delete_where(pool, todos_table(), q) ?
  if deleted == 0 do
    Err("todo not found")
  else
    Ok(current.id)
  end
end

pub fn list_todos(pool :: PoolHandle) -> String ! String do
  let q = todo_select_query()
    |> Query.order_by(:created_at, :desc)
  let rows = Repo.all(pool, q) ?
  let encoded = rows_to_json_loop(rows, 0, List.length(rows), List.new())
  Ok("[#{String.join(encoded, ",")}]")
end

pub fn get_todo(pool :: PoolHandle, id :: String) -> Todo ! String do
  let rows = Repo.all(pool, todo_query_by_id(id)) ?
  find_single_todo(rows, "todo not found")
end

pub fn create_todo(pool :: PoolHandle, title :: String) -> Todo ! String do
  let row = Repo.insert_expr(pool,
  todos_table(),
  %{"title" => Expr.value(title), "completed" => bool_expr(false)}) ?
  let todo_id = Map.get(row, "id")
  get_todo(pool, todo_id)
end

pub fn toggle_todo(pool :: PoolHandle, id :: String) -> Todo ! String do
  let current_result = get_todo(pool, id)
  case current_result do
    Ok( current) -> continue_toggle_todo(pool, id, current)
    Err( reason) -> Err(reason)
  end
end

pub fn delete_todo(pool :: PoolHandle, id :: String) -> String ! String do
  let current_result = get_todo(pool, id)
  case current_result do
    Ok( current) -> delete_todo_with_current(pool, id, current)
    Err( reason) -> Err(reason)
  end
end
"#
}

fn postgres_todo_migration_mesh() -> &'static str {
    r#"# Migration: create_todos
# Initial todo schema for the Postgres todo-api starter.

pub fn up(pool :: PoolHandle) -> Int ! String do
  Pg.create_extension(pool, "pgcrypto") ?
  Migration.create_table(pool,
  "todos",
  ["id:UUID:PRIMARY KEY DEFAULT gen_random_uuid()", "title:TEXT:NOT NULL", "completed:BOOLEAN:NOT NULL DEFAULT false", "created_at:TIMESTAMPTZ:NOT NULL DEFAULT now()"]) ?
  Migration.create_index(pool, "todos", ["created_at:DESC"], "name:idx_todos_created_at") ?
  Ok(0)
end

pub fn down(pool :: PoolHandle) -> Int ! String do
  Migration.drop_table(pool, "todos") ?
  Ok(0)
end
"#
}

/// Create a new Todo API starter template using the default local SQLite backend.
pub fn scaffold_todo_api_project(name: &str, dir: &Path) -> Result<(), String> {
    scaffold_todo_api_project_with_db(name, dir, TodoApiDatabase::Sqlite)
}

/// Create a new Todo API starter template for the selected database backend.
pub fn scaffold_todo_api_project_with_db(
    name: &str,
    dir: &Path,
    database: TodoApiDatabase,
) -> Result<(), String> {
    match database {
        TodoApiDatabase::Sqlite => scaffold_sqlite_todo_api_project(name, dir),
        TodoApiDatabase::Postgres => scaffold_postgres_todo_api_project(name, dir),
    }
}

fn scaffold_postgres_todo_api_project(name: &str, dir: &Path) -> Result<(), String> {
    let project_dir = create_project_dir(name, dir)?;

    let manifest = format!(
        r#"[package]
name = "{}"
version = "0.1.0"

[dependencies]
"#,
        name
    );
    write_project_file(&project_dir.join("mesh.toml"), &manifest)?;
    write_project_file(&project_dir.join("main.mpl"), postgres_todo_main_mesh())?;

    let work_mesh = r#"@cluster pub fn sync_todos() -> Int do
  1 + 1
end
"#;
    write_project_file(&project_dir.join("work.mpl"), work_mesh)?;
    write_project_file(&project_dir.join("config.mpl"), postgres_todo_config_mesh())?;
    write_project_file(
        &project_dir.join("api/health.mpl"),
        postgres_todo_health_mesh(),
    )?;

    let router_mesh = r#"from Api.Health import handle_health
from Api.Todos import handle_create_todo, handle_delete_todo, handle_get_todo, handle_list_todos, handle_toggle_todo

pub fn build_router() do
  let router = HTTP.router()
    |> HTTP.on_get("/health", handle_health)
    |> HTTP.on_get("/todos", HTTP.clustered(1, handle_list_todos))
    |> HTTP.on_get("/todos/:id", HTTP.clustered(1, handle_get_todo))
    |> HTTP.on_post("/todos", handle_create_todo)
    |> HTTP.on_put("/todos/:id", handle_toggle_todo)
    |> HTTP.on_delete("/todos/:id", handle_delete_todo)
  router
end
"#;
    write_project_file(&project_dir.join("api/router.mpl"), router_mesh)?;
    write_project_file(
        &project_dir.join("api/todos.mpl"),
        postgres_todo_todos_api_mesh(),
    )?;
    write_project_file(
        &project_dir.join("runtime/registry.mpl"),
        postgres_todo_registry_mesh(),
    )?;

    let rate_limiter_mesh = r#"struct RateLimiterState do
  counts :: Map < String, Int >
  window_seconds :: Int
  max_requests :: Int
end

fn check_limit_impl(state :: RateLimiterState, key :: String) ->( RateLimiterState, Bool) do
  let count = Map.get(state.counts, key)
  let allowed = count < state.max_requests
  let next_counts = if allowed do
    Map.put(state.counts, key, count + 1)
  else
    state.counts
  end
  let next_state = RateLimiterState {
    counts : next_counts,
    window_seconds : state.window_seconds,
    max_requests : state.max_requests
  }
  (next_state, allowed)
end

fn reset_window_impl(state :: RateLimiterState) -> RateLimiterState do
  RateLimiterState {
    counts : Map.new(),
    window_seconds : state.window_seconds,
    max_requests : state.max_requests
  }
end

service TodoWriteRateLimiter do
  fn init(window_seconds :: Int, max_requests :: Int) -> RateLimiterState do
    RateLimiterState {
      counts : Map.new(),
      window_seconds : window_seconds,
      max_requests : max_requests
    }
  end

  call Check(key :: String) :: Bool do|state|
    check_limit_impl(state, key)
  end

  cast Reset() do|state|
    reset_window_impl(state)
  end
end

actor rate_window_ticker(limiter_pid, interval_ms :: Int) do
  Timer.sleep(interval_ms)
  TodoWriteRateLimiter.reset(limiter_pid)
  rate_window_ticker(limiter_pid, interval_ms)
end

pub fn start_rate_limiter(window_seconds :: Int, max_requests :: Int) do
  let limiter_pid = TodoWriteRateLimiter.start(window_seconds, max_requests)
  spawn(rate_window_ticker, limiter_pid, window_seconds * 1000)
  limiter_pid
end

pub fn allow_write(limiter_pid :: Pid, key :: String) -> Bool do
  TodoWriteRateLimiter.check(limiter_pid, key)
end
"#;
    write_project_file(
        &project_dir.join("services/rate_limiter.mpl"),
        rate_limiter_mesh,
    )?;
    write_project_file(
        &project_dir.join("storage/todos.mpl"),
        postgres_todo_storage_mesh(),
    )?;

    let todo_type_mesh = r#"pub struct Todo do
  id :: String
  title :: String
  completed :: Bool
  created_at :: String
end deriving(Json)
"#;
    write_project_file(&project_dir.join("types/todo.mpl"), todo_type_mesh)?;
    write_project_file(&project_dir.join("README.md"), &postgres_todo_readme(name))?;
    write_project_file(
        &project_dir.join("Dockerfile"),
        &postgres_todo_dockerfile(name),
    )?;
    write_project_file(
        &project_dir.join(".dockerignore"),
        ".git\n.gitignore\ntarget\n.mesh\n.tmp\n.env\n.env.local\n",
    )?;
    write_project_file(
        &project_dir.join(".env.example"),
        &postgres_todo_env_example(name),
    )?;
    write_project_file(
        &project_dir.join("scripts/stage-deploy.sh"),
        &postgres_todo_stage_deploy_script(name),
    )?;
    write_project_file(
        &project_dir.join("scripts/apply-deploy-migrations.sh"),
        postgres_todo_apply_deploy_migrations_script(),
    )?;
    write_project_file(
        &project_dir.join("scripts/deploy-smoke.sh"),
        postgres_todo_deploy_smoke_script(),
    )?;
    write_project_file(
        &project_dir
            .join("deploy")
            .join(todo_postgres_deploy_sql_filename(name)),
        postgres_todo_deploy_sql(),
    )?;
    write_project_file(
        &project_dir.join("tests/config.test.mpl"),
        postgres_todo_config_test(),
    )?;
    write_project_file(
        &project_dir
            .join("migrations")
            .join(TODO_POSTGRES_MIGRATION_FILENAME),
        postgres_todo_migration_mesh(),
    )?;

    println!("Created todo-api project '{}'", name);
    Ok(())
}

/// Create a new local SQLite Todo API starter template.
fn scaffold_sqlite_todo_api_project(name: &str, dir: &Path) -> Result<(), String> {
    let project_dir = create_project_dir(name, dir)?;

    let manifest = format!(
        r#"[package]
name = "{}"
version = "0.1.0"

[dependencies]
"#,
        name
    );
    write_project_file(&project_dir.join("mesh.toml"), &manifest)?;

    let config_mesh = r#"pub fn todo_db_path_key() -> String do
  "TODO_DB_PATH"
end

pub fn port_key() -> String do
  "PORT"
end

pub fn todo_rate_limit_window_seconds_key() -> String do
  "TODO_RATE_LIMIT_WINDOW_SECONDS"
end

pub fn todo_rate_limit_max_requests_key() -> String do
  "TODO_RATE_LIMIT_MAX_REQUESTS"
end

pub fn default_todo_db_path() -> String do
  "todo.sqlite3"
end

pub fn invalid_positive_int(name :: String) -> String do
  "Invalid #{name}: expected a positive integer"
end

pub fn invalid_db_path(name :: String) -> String do
  "Invalid #{name}: expected a non-empty path"
end

pub fn invalid_todo_id_message() -> String do
  "invalid todo id"
end

pub fn title_required_message() -> String do
  "title is required"
end

pub fn todo_not_found_message() -> String do
  "todo not found"
end
"#;
    write_project_file(&project_dir.join("config.mpl"), config_mesh)?;

    let main_mesh = r#"from Config import default_todo_db_path, invalid_db_path, invalid_positive_int, port_key, todo_db_path_key, todo_rate_limit_max_requests_key, todo_rate_limit_window_seconds_key
from Api.Router import build_router
from Runtime.Registry import start_registry
from Services.RateLimiter import start_rate_limiter
from Storage.Todos import ensure_schema

fn log_config_error(message :: String) do
  println("[todo-api] Config error: #{message}")
end

fn optional_positive_env_int(name :: String, default_value :: Int) -> Int ! String do
  let raw = Env.get(name, "")
  if raw == "" do
    Ok(default_value)
  else
    let value = Env.get_int(name, -1)
    if value > 0 do
      Ok(value)
    else
      Err(invalid_positive_int(name))
    end
  end
end

fn resolve_db_path() -> String ! String do
  let key = todo_db_path_key()
  let raw = Env.get(key, default_todo_db_path())
  let trimmed = String.trim(raw)
  if trimmed == "" do
    Err(invalid_db_path(key))
  else
    Ok(trimmed)
  end
end

fn start_runtime(port :: Int, db_path :: String, window_seconds :: Int, max_requests :: Int) do
  let limiter_pid = start_rate_limiter(window_seconds, max_requests)
  start_registry(db_path, limiter_pid, window_seconds, max_requests)
  println(
    "[todo-api] local runtime ready port=#{port} db_backend=sqlite storage_mode=single-node db_path=#{db_path} write_limit_window_seconds=#{window_seconds} write_limit_max=#{max_requests}"
  )
  let router = build_router()
  println("[todo-api] HTTP server starting on :#{port}")
  HTTP.serve(router, port)
end

fn start_with_values(port :: Int, db_path :: String, window_seconds :: Int, max_requests :: Int) do
  println(
    "[todo-api] local config loaded port=#{port} db_path=#{db_path} write_limit_window_seconds=#{window_seconds} write_limit_max=#{max_requests}"
  )
  case ensure_schema(db_path) do
    Ok( _) -> do
      println("[todo-api] SQLite schema ready path=#{db_path}")
      start_runtime(port, db_path, window_seconds, max_requests)
    end
    Err( reason) -> println("[todo-api] Database init failed: #{reason}")
  end
end

fn maybe_start_with_max_requests(port :: Int, db_path :: String, window_seconds :: Int) do
  let max_requests_env = todo_rate_limit_max_requests_key()
  case optional_positive_env_int(max_requests_env, 5) do
    Ok( max_requests) -> start_with_values(port, db_path, window_seconds, max_requests)
    Err( message) -> log_config_error(message)
  end
end

fn maybe_start_with_window_seconds(port :: Int, db_path :: String) do
  let window_seconds_env = todo_rate_limit_window_seconds_key()
  case optional_positive_env_int(window_seconds_env, 60) do
    Ok( window_seconds) -> maybe_start_with_max_requests(port, db_path, window_seconds)
    Err( message) -> log_config_error(message)
  end
end

fn maybe_start_with_port(db_path :: String) do
  let port_env = port_key()
  case optional_positive_env_int(port_env, 8080) do
    Ok( port) -> maybe_start_with_window_seconds(port, db_path)
    Err( message) -> log_config_error(message)
  end
end

fn main() do
  case resolve_db_path() do
    Ok( db_path) -> maybe_start_with_port(db_path)
    Err( message) -> log_config_error(message)
  end
end
"#;
    write_project_file(&project_dir.join("main.mpl"), main_mesh)?;

    let health_mesh = r#"from Runtime.Registry import get_db_path, get_max_requests, get_window_seconds

pub fn handle_health(_request) do
  HTTP.response(200,
  json {
    status : "ok",
    mode : "local",
    db_backend : "sqlite",
    db_path : get_db_path(),
    storage_mode : "single-node",
    rate_limit_window_seconds : get_window_seconds(),
    rate_limit_max_requests : get_max_requests()
  })
end
"#;
    write_project_file(&project_dir.join("api/health.mpl"), health_mesh)?;

    let router_mesh = r#"from Api.Health import handle_health
from Api.Todos import handle_create_todo, handle_delete_todo, handle_get_todo, handle_list_todos, handle_toggle_todo

pub fn build_router() do
  let router = HTTP.router()
    |> HTTP.on_get("/health", handle_health)
    |> HTTP.on_get("/todos", handle_list_todos)
    |> HTTP.on_get("/todos/:id", handle_get_todo)
    |> HTTP.on_post("/todos", handle_create_todo)
    |> HTTP.on_put("/todos/:id", handle_toggle_todo)
    |> HTTP.on_delete("/todos/:id", handle_delete_todo)
  router
end
"#;
    write_project_file(&project_dir.join("api/router.mpl"), router_mesh)?;

    let todos_api_mesh = r#"from Config import invalid_todo_id_message, title_required_message, todo_not_found_message
from Runtime.Registry import get_db_path, get_rate_limiter
from Services.RateLimiter import allow_write
from Storage.Todos import create_todo, delete_todo, get_todo, list_todos, toggle_todo
from Types.Todo import Todo

fn todo_to_json(todo :: Todo) -> String do
  Json.encode(todo)
end

fn require_param(request, name :: String) -> String do
  let value = Request.param(request, name)
  case value do
    Some( param) -> param
    None -> ""
  end
end

fn title_from_body(body :: String) -> String do
  String.trim(Json.get(body, "title"))
end

fn not_found_response() do
  HTTP.response(404, json { error : todo_not_found_message() })
end

fn bad_request_response(message :: String) do
  HTTP.response(400, json { error : message })
end

fn rate_limited_response() do
  HTTP.response(429, json { error : "rate limited" })
end

fn internal_error_response(reason :: String) do
  HTTP.response(500, json { error : reason })
end

fn todo_error_response(reason :: String) do
  if reason == todo_not_found_message() do
    not_found_response()
  else if reason == invalid_todo_id_message() do
    bad_request_response(reason)
  else if reason == title_required_message() do
    bad_request_response(reason)
  else
    internal_error_response(reason)
  end
end

fn allow_mutation() -> Bool do
  let limiter_pid = get_rate_limiter()
  allow_write(limiter_pid, "todo-write")
end

fn create_todo_with_title(db_path :: String, title :: String) do
  let result = create_todo(db_path, title)
  case result do
    Ok( todo) -> HTTP.response(201, todo_to_json(todo))
    Err( reason) -> todo_error_response(reason)
  end
end

fn create_todo_with_body(db_path :: String, body :: String) do
  let title = title_from_body(body)
  create_todo_with_title(db_path, title)
end

fn get_todo_response(db_path :: String, id :: String) do
  let result = get_todo(db_path, id)
  case result do
    Ok( todo) -> HTTP.response(200, todo_to_json(todo))
    Err( reason) -> todo_error_response(reason)
  end
end

fn toggle_todo_response(db_path :: String, id :: String) do
  let result = toggle_todo(db_path, id)
  case result do
    Ok( todo) -> HTTP.response(200, todo_to_json(todo))
    Err( reason) -> todo_error_response(reason)
  end
end

fn delete_todo_response(db_path :: String, id :: String) do
  let result = delete_todo(db_path, id)
  case result do
    Ok( deleted_id) -> HTTP.response(200, json { status : "deleted", id : deleted_id })
    Err( reason) -> todo_error_response(reason)
  end
end

pub fn handle_list_todos(_request :: Request) -> Response do
  case list_todos(get_db_path()) do
    Ok( todos_json) -> HTTP.response(200, todos_json)
    Err( reason) -> internal_error_response(reason)
  end
end

pub fn handle_get_todo(request :: Request) -> Response do
  let id = require_param(request, "id")
  get_todo_response(get_db_path(), id)
end

pub fn handle_create_todo(request) do
  if allow_mutation() do
    create_todo_with_body(get_db_path(), Request.body(request))
  else
    rate_limited_response()
  end
end

pub fn handle_toggle_todo(request) do
  if allow_mutation() do
    let id = require_param(request, "id")
    toggle_todo_response(get_db_path(), id)
  else
    rate_limited_response()
  end
end

pub fn handle_delete_todo(request) do
  if allow_mutation() do
    let id = require_param(request, "id")
    delete_todo_response(get_db_path(), id)
  else
    rate_limited_response()
  end
end
"#;
    write_project_file(&project_dir.join("api/todos.mpl"), todos_api_mesh)?;

    let registry_mesh = r#"struct RegistryState do
  db_path :: String
  rate_limiter_pid :: Pid
  window_seconds :: Int
  max_requests :: Int
end

service TodoRegistry do
  fn init(db_path :: String, rate_limiter_pid :: Pid, window_seconds :: Int, max_requests :: Int) -> RegistryState do
    RegistryState {
      db_path : db_path,
      rate_limiter_pid : rate_limiter_pid,
      window_seconds : window_seconds,
      max_requests : max_requests
    }
  end

  call GetDbPath() :: String do|state|
    (state, state.db_path)
  end

  call GetRateLimiter() :: Pid do|state|
    (state, state.rate_limiter_pid)
  end

  call GetWindowSeconds() :: Int do|state|
    (state, state.window_seconds)
  end

  call GetMaxRequests() :: Int do|state|
    (state, state.max_requests)
  end
end

pub fn start_registry(db_path :: String, rate_limiter_pid :: Pid, window_seconds :: Int, max_requests :: Int) do
  let registry_pid = TodoRegistry.start(db_path, rate_limiter_pid, window_seconds, max_requests)
  Process.register("todo_api_registry", registry_pid)
  registry_pid
end

pub fn get_db_path() -> String do
  let registry_pid = Process.whereis("todo_api_registry")
  TodoRegistry.get_db_path(registry_pid)
end

pub fn get_rate_limiter() -> Pid do
  let registry_pid = Process.whereis("todo_api_registry")
  TodoRegistry.get_rate_limiter(registry_pid)
end

pub fn get_window_seconds() -> Int do
  let registry_pid = Process.whereis("todo_api_registry")
  TodoRegistry.get_window_seconds(registry_pid)
end

pub fn get_max_requests() -> Int do
  let registry_pid = Process.whereis("todo_api_registry")
  TodoRegistry.get_max_requests(registry_pid)
end
"#;
    write_project_file(&project_dir.join("runtime/registry.mpl"), registry_mesh)?;

    let rate_limiter_mesh = r#"struct RateLimiterState do
  counts :: Map < String, Int >
  window_seconds :: Int
  max_requests :: Int
end

fn check_limit_impl(state :: RateLimiterState, key :: String) ->( RateLimiterState, Bool) do
  let count = Map.get(state.counts, key)
  let allowed = count < state.max_requests
  let next_counts = if allowed do
    Map.put(state.counts, key, count + 1)
  else
    state.counts
  end
  let next_state = RateLimiterState {
    counts : next_counts,
    window_seconds : state.window_seconds,
    max_requests : state.max_requests
  }
  (next_state, allowed)
end

fn reset_window_impl(state :: RateLimiterState) -> RateLimiterState do
  RateLimiterState {
    counts : Map.new(),
    window_seconds : state.window_seconds,
    max_requests : state.max_requests
  }
end

service TodoWriteRateLimiter do
  fn init(window_seconds :: Int, max_requests :: Int) -> RateLimiterState do
    RateLimiterState {
      counts : Map.new(),
      window_seconds : window_seconds,
      max_requests : max_requests
    }
  end

  call Check(key :: String) :: Bool do|state|
    check_limit_impl(state, key)
  end

  cast Reset() do|state|
    reset_window_impl(state)
  end
end

actor rate_window_ticker(limiter_pid, interval_ms :: Int) do
  Timer.sleep(interval_ms)
  TodoWriteRateLimiter.reset(limiter_pid)
  rate_window_ticker(limiter_pid, interval_ms)
end

pub fn start_rate_limiter(window_seconds :: Int, max_requests :: Int) do
  let limiter_pid = TodoWriteRateLimiter.start(window_seconds, max_requests)
  spawn(rate_window_ticker, limiter_pid, window_seconds * 1000)
  limiter_pid
end

pub fn allow_write(limiter_pid :: Pid, key :: String) -> Bool do
  TodoWriteRateLimiter.check(limiter_pid, key)
end
"#;
    write_project_file(
        &project_dir.join("services/rate_limiter.mpl"),
        rate_limiter_mesh,
    )?;

    let storage_mesh = r#"from Config import invalid_todo_id_message, title_required_message, todo_not_found_message
from Types.Todo import Todo

fn todo_from_row(row) -> Todo ! String do
  Todo.from_row(row)
end

fn rows_to_json_loop(rows, index :: Int, total :: Int, acc :: List < String >) -> List < String > ! String do
  if index >= total do
    Ok(acc)
  else
    let todo = todo_from_row(List.get(rows, index))?
    let encoded = Json.encode(todo)
    rows_to_json_loop(rows, index + 1, total, List.append(acc, encoded))
  end
end

fn normalized_title(title :: String) -> String ! String do
  let trimmed = String.trim(title)
  if String.length(trimmed) == 0 do
    Err(title_required_message())
  else
    Ok(trimmed)
  end
end

fn normalized_todo_id(id :: String) -> String ! String do
  let trimmed = String.trim(id)
  case String.to_int(trimmed) do
    Some( value) -> if value > 0 do
      Ok(String.from(value))
    else
      Err(invalid_todo_id_message())
    end
    None -> Err(invalid_todo_id_message())
  end
end

fn first_todo(rows) -> Todo ! String do
  if List.length(rows) == 0 do
    Err(todo_not_found_message())
  else
    todo_from_row(List.get(rows, 0))
  end
end

pub fn ensure_schema(db_path :: String) -> Int ! String do
  let db = Sqlite.open(db_path)?
  let applied = Sqlite.execute(
    db,
    "CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL, completed INTEGER NOT NULL DEFAULT 0, created_at TEXT NOT NULL)",
    []
  )?
  Sqlite.close(db)
  Ok(applied)
end

pub fn list_todos(db_path :: String) -> String ! String do
  let db = Sqlite.open(db_path)?
  let rows = Sqlite.query(db, "SELECT id, title, completed, created_at FROM todos ORDER BY id", [])?
  Sqlite.close(db)
  let encoded = rows_to_json_loop(rows, 0, List.length(rows), List.new())?
  Ok("[#{String.join(encoded, ",")}]")
end

pub fn get_todo(db_path :: String, id :: String) -> Todo ! String do
  let todo_id = normalized_todo_id(id)?
  let db = Sqlite.open(db_path)?
  let rows = Sqlite.query(db, "SELECT id, title, completed, created_at FROM todos WHERE id = ?", [todo_id])?
  Sqlite.close(db)
  first_todo(rows)
end

pub fn create_todo(db_path :: String, title :: String) -> Todo ! String do
  let normalized = normalized_title(title)?
  let db = Sqlite.open(db_path)?
  let created_at = DateTime.to_iso8601(DateTime.utc_now())
  let _ = Sqlite.execute(db, "INSERT INTO todos (title, completed, created_at) VALUES (?, ?, ?)", [normalized, "0", created_at])?
  let rows = Sqlite.query(db, "SELECT id, title, completed, created_at FROM todos WHERE id = last_insert_rowid()", [])?
  Sqlite.close(db)
  if List.length(rows) == 0 do
    Err("todo insert did not return a row")
  else
    todo_from_row(List.get(rows, 0))
  end
end

pub fn toggle_todo(db_path :: String, id :: String) -> Todo ! String do
  let current = get_todo(db_path, id)?
  let db = Sqlite.open(db_path)?
  let next_completed = if current.completed do
    "0"
  else
    "1"
  end
  let updated = Sqlite.execute(db, "UPDATE todos SET completed = ? WHERE id = ?", [next_completed, current.id])?
  let rows = Sqlite.query(db, "SELECT id, title, completed, created_at FROM todos WHERE id = ?", [current.id])?
  Sqlite.close(db)
  if updated == 0 do
    Err(todo_not_found_message())
  else
    first_todo(rows)
  end
end

pub fn delete_todo(db_path :: String, id :: String) -> String ! String do
  let current = get_todo(db_path, id)?
  let db = Sqlite.open(db_path)?
  let deleted = Sqlite.execute(db, "DELETE FROM todos WHERE id = ?", [current.id])?
  Sqlite.close(db)
  if deleted == 0 do
    Err(todo_not_found_message())
  else
    Ok(current.id)
  end
end
"#;
    write_project_file(&project_dir.join("storage/todos.mpl"), storage_mesh)?;

    let todo_type_mesh = r#"pub struct Todo do
  id :: String
  title :: String
  completed :: Bool
  created_at :: String
end deriving(Json, Row)
"#;
    write_project_file(&project_dir.join("types/todo.mpl"), todo_type_mesh)?;

    let config_test = r#"from Config import default_todo_db_path, invalid_db_path, invalid_positive_int, invalid_todo_id_message, port_key, title_required_message, todo_db_path_key, todo_not_found_message, todo_rate_limit_max_requests_key, todo_rate_limit_window_seconds_key

describe("SQLite todo-api config") do
  test("exposes local environment keys and defaults") do
    assert_eq(port_key(), "PORT")
    assert_eq(todo_db_path_key(), "TODO_DB_PATH")
    assert_eq(default_todo_db_path(), "todo.sqlite3")
    assert_eq(todo_rate_limit_window_seconds_key(), "TODO_RATE_LIMIT_WINDOW_SECONDS")
    assert_eq(todo_rate_limit_max_requests_key(), "TODO_RATE_LIMIT_MAX_REQUESTS")
  end

  test("formats local validation messages") do
    assert_eq(invalid_positive_int(port_key()), "Invalid PORT: expected a positive integer")
    assert_eq(invalid_positive_int(todo_rate_limit_window_seconds_key()), "Invalid TODO_RATE_LIMIT_WINDOW_SECONDS: expected a positive integer")
    assert_eq(invalid_positive_int(todo_rate_limit_max_requests_key()), "Invalid TODO_RATE_LIMIT_MAX_REQUESTS: expected a positive integer")
    assert_eq(invalid_db_path(todo_db_path_key()), "Invalid TODO_DB_PATH: expected a non-empty path")
    assert_eq(invalid_todo_id_message(), "invalid todo id")
    assert_eq(title_required_message(), "title is required")
    assert_eq(todo_not_found_message(), "todo not found")
  end
end
"#;
    write_project_file(&project_dir.join("tests/config.test.mpl"), config_test)?;

    let storage_test = r#"from Storage.Todos import create_todo, delete_todo, ensure_schema, get_todo, list_todos, toggle_todo
from Types.Todo import Todo

fn sample_todo() -> Todo do
  Todo {
    id : "1",
    title : "compile",
    completed : false,
    created_at : "now"
  }
end

describe("SQLite todo storage") do
  test("local storage module compiles for the generated starter") do
    let todo = sample_todo()
    assert(todo.title == "compile")
    assert(todo.completed == false)
  end

  test("storage helper imports stay available to the generated project") do
    let todo = sample_todo()
    assert(todo.id == "1")
    assert(todo.created_at == "now")
  end
end
"#;
    write_project_file(&project_dir.join("tests/storage.test.mpl"), storage_test)?;

    write_project_file(&project_dir.join("README.md"), &todo_readme(name))?;
    write_project_file(&project_dir.join("Dockerfile"), &todo_dockerfile(name))?;
    write_project_file(
        &project_dir.join(".dockerignore"),
        ".git\n.gitignore\ntarget\n.mesh\n.tmp\n*.sqlite3\n",
    )?;

    println!("Created todo-api project '{}'", name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::Manifest;
    use tempfile::TempDir;

    #[test]
    fn scaffold_creates_directory_structure() {
        let tmp = TempDir::new().unwrap();
        scaffold_project("my-app", tmp.path()).unwrap();

        let project_dir = tmp.path().join("my-app");
        assert!(project_dir.exists(), "Project directory should exist");
        assert!(project_dir.is_dir(), "Project path should be a directory");
        assert!(
            project_dir.join("mesh.toml").exists(),
            "mesh.toml should exist"
        );
        assert!(
            project_dir.join("main.mpl").exists(),
            "main.mpl should exist"
        );
    }

    #[test]
    fn scaffold_mesh_toml_is_valid() {
        let tmp = TempDir::new().unwrap();
        scaffold_project("test-project", tmp.path()).unwrap();

        let toml_path = tmp.path().join("test-project").join("mesh.toml");
        let content = std::fs::read_to_string(&toml_path).unwrap();
        let manifest = Manifest::from_str(&content).unwrap();

        assert_eq!(manifest.package.name, "test-project");
        assert_eq!(manifest.package.version, "0.1.0");
        assert!(manifest.dependencies.is_empty());
        assert!(manifest.cluster.is_none());
    }

    #[test]
    fn scaffold_main_mesh_content() {
        let tmp = TempDir::new().unwrap();
        scaffold_project("hello", tmp.path()).unwrap();

        let main_path = tmp.path().join("hello").join("main.mpl");
        let content = std::fs::read_to_string(&main_path).unwrap();
        assert!(content.contains("fn main()"), "Should have main function");
        assert!(content.contains("IO.puts"), "Should have IO.puts call");
    }

    #[test]
    fn scaffold_clustered_project_writes_public_cluster_contract() {
        let tmp = TempDir::new().unwrap();
        scaffold_clustered_project("clustered", tmp.path()).unwrap();

        let project_dir = tmp.path().join("clustered");
        let manifest_path = project_dir.join("mesh.toml");
        let main_path = project_dir.join("main.mpl");
        let work_path = project_dir.join("work.mpl");
        let readme_path = project_dir.join("README.md");

        assert!(manifest_path.exists(), "clustered mesh.toml should exist");
        assert!(main_path.exists(), "clustered main.mpl should exist");
        assert!(work_path.exists(), "clustered work.mpl should exist");
        assert!(readme_path.exists(), "clustered README.md should exist");

        let manifest = Manifest::from_str(&std::fs::read_to_string(&manifest_path).unwrap())
            .expect("clustered manifest should parse");
        assert!(
            manifest.cluster.is_none(),
            "clustered scaffold should keep mesh.toml package-only"
        );

        let main = std::fs::read_to_string(&main_path).unwrap();
        assert_eq!(main.matches("Node.start_from_env()").count(), 1);
        assert!(main.contains("BootstrapStatus"));
        assert!(main.contains("runtime bootstrap"));
        assert!(main.contains("runtime bootstrap failed"));
        assert!(!main.contains("HTTP.serve("));
        assert!(!main.contains("Continuity.submit_declared_work"));
        assert!(!main.contains("/health"));
        assert!(!main.contains("/work"));
        assert!(!main.contains("Env.get_int"));
        assert!(!main.contains("Node.start("));
        assert!(!main.contains("CLUSTER_PROOF_"));

        let work = std::fs::read_to_string(&work_path).unwrap();
        assert!(work.contains("@cluster pub fn add()"));
        assert!(!work.contains("declared_work_runtime_name"));
        assert!(!work.contains("clustered(work)"));
        assert!(work.contains("1 + 1"));
        assert!(!work.contains("declared_work_target"));
        assert!(!work.contains("Continuity.submit_declared_work"));
        assert!(!work.contains("Continuity.mark_completed"));
        assert!(!work.contains("Timer.sleep"));
        assert!(!work.contains("owner_node"));
        assert!(!work.contains("replica_node"));

        let readme = std::fs::read_to_string(&readme_path).unwrap();
        assert!(readme.contains("mesh.toml` is package-only and intentionally omits `[cluster]`"));
        assert!(readme.contains("Node.start_from_env()"));
        assert!(readme.contains("@cluster pub fn add()"));
        assert!(readme.contains("Work.add"));
        assert!(readme.contains("1 + 1"));
        assert!(readme.contains("examples/todo-postgres/README.md"));
        assert!(readme.contains("examples/todo-sqlite/README.md"));
        assert!(readme.contains("https://meshlang.dev/docs/production-backend-proof/"));
        assert!(readme.contains("https://github.com/hyperpush-org/hyperpush-mono"));
        assert!(readme.contains("Hyperpush product repo"));
        assert!(readme.contains(
            "https://github.com/hyperpush-org/hyperpush-mono/blob/main/mesher/README.md"
        ));
        assert!(!readme.contains("verify-m051-s01.sh"));
        assert!(!readme.contains("verify-m051-s02.sh"));
        assert!(readme.contains("serious shared/deployable PostgreSQL starter"));
        assert!(readme.contains("honest local single-node SQLite starter"));
        assert!(readme.contains("MESH_CONTINUITY_ROLE"));
        assert!(readme.contains("MESH_CONTINUITY_PROMOTION_EPOCH"));
        assert!(readme.contains("meshc cluster status"));
        assert!(readme.contains("meshc cluster continuity <node-name@host:port> --json"));
        assert!(
            readme.contains("meshc cluster continuity <node-name@host:port> <request-key> --json")
        );
        assert!(readme.contains("meshc cluster diagnostics"));
        assert!(readme.contains("automatically starts the source-declared `@cluster` function"));
        assert!(!readme.contains("tiny-cluster/README.md"));
        assert!(!readme.contains("cluster-proof/README.md"));
        assert!(!readme.contains("reference-backend/README.md"));
        assert!(!readme.contains("declared_work_runtime_name()"));
        assert!(!readme.contains("clustered(work)"));
        assert!(!readme.contains("Continuity.submit_declared_work"));
        assert!(!readme.contains("HTTP.serve("));
        assert!(!readme.contains("HTTP.clustered("));
        assert!(!readme.contains("/health"));
        assert!(!readme.contains("/work"));
        assert!(!readme.contains("Timer.sleep"));
        assert!(!readme.contains("CLUSTER_PROOF_"));
    }

    #[test]
    fn scaffold_clustered_error_when_directory_exists() {
        let tmp = TempDir::new().unwrap();
        let existing = tmp.path().join("existing-clustered");
        std::fs::create_dir_all(&existing).unwrap();

        let result = scaffold_clustered_project("existing-clustered", tmp.path());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.contains("already exists"),
            "Error should mention 'already exists', got: {}",
            err
        );
    }

    fn assert_local_sqlite_todo_scaffold(project_dir: &std::path::Path, name: &str) {
        let manifest_path = project_dir.join("mesh.toml");
        let main_path = project_dir.join("main.mpl");
        let config_path = project_dir.join("config.mpl");
        let work_path = project_dir.join("work.mpl");
        let readme_path = project_dir.join("README.md");
        let dockerfile_path = project_dir.join("Dockerfile");
        let dockerignore_path = project_dir.join(".dockerignore");
        let router_path = project_dir.join("api/router.mpl");
        let todos_api_path = project_dir.join("api/todos.mpl");
        let health_path = project_dir.join("api/health.mpl");
        let registry_path = project_dir.join("runtime/registry.mpl");
        let limiter_path = project_dir.join("services/rate_limiter.mpl");
        let storage_path = project_dir.join("storage/todos.mpl");
        let todo_type_path = project_dir.join("types/todo.mpl");
        let config_test_path = project_dir.join("tests/config.test.mpl");
        let storage_test_path = project_dir.join("tests/storage.test.mpl");

        for path in [
            &manifest_path,
            &main_path,
            &config_path,
            &readme_path,
            &dockerfile_path,
            &dockerignore_path,
            &router_path,
            &todos_api_path,
            &health_path,
            &registry_path,
            &limiter_path,
            &storage_path,
            &todo_type_path,
            &config_test_path,
            &storage_test_path,
        ] {
            assert!(path.exists(), "missing scaffolded file {}", path.display());
        }
        assert!(
            !work_path.exists(),
            "sqlite starter should not emit work.mpl at {}",
            work_path.display()
        );

        let manifest = Manifest::from_str(&std::fs::read_to_string(&manifest_path).unwrap())
            .expect("sqlite todo scaffold manifest should parse");
        assert_eq!(manifest.package.name, name);
        assert!(manifest.cluster.is_none());

        let main = std::fs::read_to_string(&main_path).unwrap();
        assert!(main.contains("todo_db_path_key()"));
        assert!(main.contains("start_rate_limiter"));
        assert!(main.contains("start_registry"));
        assert!(main.contains("ensure_schema"));
        assert!(main.contains("[todo-api] local config loaded"));
        assert!(main.contains("[todo-api] SQLite schema ready"));
        assert!(main.contains("[todo-api] local runtime ready"));
        assert!(main.contains("[todo-api] HTTP server starting on"));
        assert!(main.contains("resolve_db_path()"));
        assert!(main.contains("todo_rate_limit_window_seconds_key()"));
        assert!(main.contains("todo_rate_limit_max_requests_key()"));
        assert!(!main.contains("Node.start_from_env()"));
        assert!(!main.contains("BootstrapStatus"));
        assert!(!main.contains("runtime bootstrap"));
        assert!(!main.contains("HTTP.clustered("));
        assert!(!main.contains("MESH_CLUSTER_"));
        assert!(!main.contains("Work.sync_todos"));

        let config = std::fs::read_to_string(&config_path).unwrap();
        assert!(config.contains("todo_db_path_key"));
        assert!(config.contains("default_todo_db_path"));
        assert!(config.contains("invalid_positive_int"));
        assert!(config.contains("invalid_db_path"));
        assert!(config.contains("invalid_todo_id_message"));
        assert!(config.contains("title_required_message"));
        assert!(config.contains("todo_not_found_message"));

        let router = std::fs::read_to_string(&router_path).unwrap();
        assert!(router.contains("HTTP.on_get(\"/health\", handle_health)"));
        assert!(router.contains("HTTP.on_get(\"/todos\", handle_list_todos)"));
        assert!(router.contains("HTTP.on_get(\"/todos/:id\", handle_get_todo)"));
        assert!(router.contains("HTTP.on_post(\"/todos\", handle_create_todo)"));
        assert!(router.contains("HTTP.on_put(\"/todos/:id\", handle_toggle_todo)"));
        assert!(router.contains("HTTP.on_delete(\"/todos/:id\", handle_delete_todo)"));
        assert!(!router.contains("HTTP.clustered("));

        let todos_api = std::fs::read_to_string(&todos_api_path).unwrap();
        assert!(todos_api.contains("allow_write("));
        assert!(todos_api.contains("todo_error_response"));
        assert!(todos_api.contains("invalid_todo_id_message"));
        assert!(todos_api.contains("title_required_message"));
        assert!(todos_api.contains("pub fn handle_list_todos(_request :: Request) -> Response do"));
        assert!(todos_api.contains("pub fn handle_get_todo(request :: Request) -> Response do"));
        assert!(todos_api.contains("bad_request_response"));
        assert!(!todos_api.contains("HTTP.clustered("));
        assert!(!todos_api.contains("Work.sync_todos"));

        let health = std::fs::read_to_string(&health_path).unwrap();
        assert!(health.contains("mode : \"local\""));
        assert!(health.contains("db_backend : \"sqlite\""));
        assert!(health.contains("storage_mode : \"single-node\""));
        assert!(health.contains("db_path : get_db_path()"));
        assert!(!health.contains("clustered_handler"));

        let registry = std::fs::read_to_string(&registry_path).unwrap();
        assert!(registry.contains("Process.register(\"todo_api_registry\""));
        assert!(registry.contains("call GetRateLimiter() :: Pid"));
        assert!(registry.contains("call GetDbPath() :: String"));

        let limiter = std::fs::read_to_string(&limiter_path).unwrap();
        assert!(limiter.contains("service TodoWriteRateLimiter do"));
        assert!(limiter.contains("spawn(rate_window_ticker"));
        assert!(limiter.contains("call Check(key :: String) :: Bool"));

        let storage = std::fs::read_to_string(&storage_path).unwrap();
        assert!(storage.contains("Todo.from_row(row)"));
        assert!(storage.contains("String.to_int(trimmed)"));
        assert!(storage.contains("CREATE TABLE IF NOT EXISTS todos"));
        assert!(storage.contains("completed INTEGER NOT NULL DEFAULT 0"));
        assert!(storage.contains("title_required_message()"));
        assert!(storage.contains("invalid_todo_id_message()"));
        assert!(storage.contains("todo_not_found_message()"));
        assert!(!storage.contains("Map.get(row,"));
        assert!(!storage.contains("HTTP.clustered("));

        let todo_type = std::fs::read_to_string(&todo_type_path).unwrap();
        assert!(todo_type.contains("pub struct Todo do"));
        assert!(todo_type.contains("end deriving(Json, Row)"));

        let config_test = std::fs::read_to_string(&config_test_path).unwrap();
        assert!(config_test.contains("describe(\"SQLite todo-api config\")"));
        assert!(config_test.contains("TODO_DB_PATH"));
        assert!(config_test.contains("Invalid TODO_DB_PATH: expected a non-empty path"));

        let storage_test = std::fs::read_to_string(&storage_test_path).unwrap();
        assert!(storage_test.contains("describe(\"SQLite todo storage\")"));
        assert!(storage_test.contains("sample_todo"));
        assert!(storage_test.contains("create_todo"));
        assert!(storage_test.contains("list_todos"));
        assert!(storage_test.contains("ensure_schema"));

        let readme = std::fs::read_to_string(&readme_path).unwrap();
        assert!(readme.contains(
            "This project was generated by `meshc init --template todo-api --db sqlite`."
        ));
        assert!(!readme.contains("This project was generated by `meshc init --template todo-api`."));
        assert!(readme.contains("single-node SQLite Todo API"));
        assert!(readme.contains("meshc test ."));
        assert!(readme.contains("meshc init --template todo-api --db postgres my-shared-todo"));
        assert!(readme.contains("meshc init --clustered my-clustered-app"));
        assert!(readme.contains("GET /todos` — list todos locally"));
        assert!(readme.contains("there is no `work.mpl`, `HTTP.clustered(...)`, or `meshc cluster` story in this starter"));
        assert!(readme.contains("TODO_DB_PATH"));
        assert!(readme.contains(&format!("docker build -t {} .", name)));
        assert!(!readme.contains("Node.start_from_env()"));
        assert!(!readme.contains("Work.sync_todos"));
        assert!(!readme.contains("meshc cluster status"));
        assert!(!readme.contains("MESH_CLUSTER_"));

        let dockerfile = std::fs::read_to_string(&dockerfile_path).unwrap();
        assert!(dockerfile.contains("FROM ubuntu:24.04"));
        assert!(dockerfile.contains(&format!("COPY output /usr/local/bin/{}", name)));
        assert!(dockerfile.contains(&format!("ENTRYPOINT [\"/usr/local/bin/{}\"]", name)));
        assert!(dockerfile.contains("EXPOSE 8080"));
        assert!(!dockerfile.contains("4370"));
        assert!(!dockerfile.contains("MESH_CLUSTER_PORT"));

        let dockerignore = std::fs::read_to_string(&dockerignore_path).unwrap();
        assert!(dockerignore.contains("*.sqlite3"));
        assert!(dockerignore.contains("target"));
    }

    #[test]
    fn m049_s02_sqlite_scaffold_database_selector_routes_to_local_contract() {
        let tmp = TempDir::new().unwrap();
        scaffold_todo_api_project_with_db("todo-api", tmp.path(), TodoApiDatabase::Sqlite)
            .expect("sqlite todo-api scaffold should succeed");

        let project_dir = tmp.path().join("todo-api");
        assert_local_sqlite_todo_scaffold(&project_dir, "todo-api");
    }

    #[test]
    fn m049_s02_sqlite_scaffold_writes_local_only_template_and_package_tests() {
        let tmp = TempDir::new().unwrap();
        scaffold_todo_api_project("todo-api", tmp.path()).unwrap();

        let project_dir = tmp.path().join("todo-api");
        assert_local_sqlite_todo_scaffold(&project_dir, "todo-api");
    }

    #[test]
    fn m049_s01_postgres_scaffold_writes_migration_first_file_set() {
        let tmp = TempDir::new().unwrap();
        scaffold_todo_api_project_with_db("todo-api", tmp.path(), TodoApiDatabase::Postgres)
            .expect("postgres todo-api scaffold should succeed");

        let project_dir = tmp.path().join("todo-api");
        let migration_path = project_dir
            .join("migrations")
            .join(TODO_POSTGRES_MIGRATION_FILENAME);
        let deploy_sql_path = project_dir
            .join("deploy")
            .join(todo_postgres_deploy_sql_filename("todo-api"));
        let stage_script_path = project_dir.join("scripts/stage-deploy.sh");
        let apply_script_path = project_dir.join("scripts/apply-deploy-migrations.sh");
        let deploy_smoke_path = project_dir.join("scripts/deploy-smoke.sh");

        for path in [
            project_dir.join("mesh.toml"),
            project_dir.join("main.mpl"),
            project_dir.join("work.mpl"),
            project_dir.join("config.mpl"),
            project_dir.join("README.md"),
            project_dir.join("Dockerfile"),
            project_dir.join(".dockerignore"),
            project_dir.join(".env.example"),
            project_dir.join("api/health.mpl"),
            project_dir.join("api/router.mpl"),
            project_dir.join("api/todos.mpl"),
            project_dir.join("runtime/registry.mpl"),
            project_dir.join("scripts/stage-deploy.sh"),
            project_dir.join("scripts/apply-deploy-migrations.sh"),
            project_dir.join("scripts/deploy-smoke.sh"),
            project_dir.join("services/rate_limiter.mpl"),
            project_dir.join("storage/todos.mpl"),
            project_dir.join("tests/config.test.mpl"),
            project_dir.join("types/todo.mpl"),
            project_dir.join("deploy").join("todo-api.up.sql"),
            migration_path.clone(),
        ] {
            assert!(path.exists(), "missing scaffolded file {}", path.display());
        }

        let manifest =
            Manifest::from_str(&std::fs::read_to_string(project_dir.join("mesh.toml")).unwrap())
                .expect("postgres todo scaffold manifest should parse");
        assert_eq!(manifest.package.name, "todo-api");
        assert!(manifest.cluster.is_none());

        let main = std::fs::read_to_string(project_dir.join("main.mpl")).unwrap();
        assert!(main.contains("Node.start_from_env()"));
        assert!(main.contains("Pool.open(database_url, 1, 4, 5000)"));
        assert!(main.contains("database_url_key()"));
        assert!(main.contains("[todo-api] PostgreSQL pool ready"));
        assert!(main.contains("start_registry(pool, limiter_pid, window_seconds, max_requests)"));
        assert!(main.contains("HTTP.serve(router, port)"));
        assert!(!main.contains("TODO_DB_PATH"));
        assert!(!main.contains("ensure_schema"));
        assert!(!main.contains("CREATE TABLE IF NOT EXISTS todos"));

        let config = std::fs::read_to_string(project_dir.join("config.mpl")).unwrap();
        assert!(config.contains("database_url_key"));
        assert!(config.contains("todo_rate_limit_window_seconds_key"));
        assert!(config.contains("todo_rate_limit_max_requests_key"));
        assert!(config.contains("Missing required environment variable"));
        assert!(config.contains("expected a positive integer"));

        let storage = std::fs::read_to_string(project_dir.join("storage/todos.mpl")).unwrap();
        assert!(storage.contains("Query.from(todos_table())"));
        assert!(storage.contains("Repo.insert_expr"));
        assert!(storage.contains("Repo.update_where_expr"));
        assert!(storage.contains("Repo.delete_where"));
        assert!(storage.contains("Pg.uuid(Expr.value(id))"));
        assert!(storage.contains("Pg.cast(Expr.value(\"false\"), \"boolean\")"));
        assert!(!storage.contains("Sqlite.open"));
        assert!(!storage.contains("CREATE TABLE IF NOT EXISTS todos"));
        assert!(!storage.contains("TODO_DB_PATH"));

        let migration = std::fs::read_to_string(&migration_path).unwrap();
        assert!(migration.contains("# Migration: create_todos"));
        assert!(migration.contains("Pg.create_extension(pool, \"pgcrypto\")"));
        assert!(migration.contains("Migration.create_table(pool,"));
        assert!(migration.contains("Migration.create_index(pool, \"todos\", [\"created_at:DESC\"], \"name:idx_todos_created_at\")"));
        assert!(!migration.contains("CREATE TABLE IF NOT EXISTS todos"));

        let deploy_sql = std::fs::read_to_string(&deploy_sql_path).unwrap();
        assert!(deploy_sql.contains("CREATE EXTENSION IF NOT EXISTS pgcrypto"));
        assert!(deploy_sql.contains("CREATE TABLE IF NOT EXISTS todos"));
        assert!(deploy_sql.contains("CREATE INDEX IF NOT EXISTS idx_todos_created_at"));
        assert!(!deploy_sql.contains("DATABASE_URL"));

        let stage_script = std::fs::read_to_string(&stage_script_path).unwrap();
        assert!(stage_script.contains("usage: bash scripts/stage-deploy.sh <bundle-dir>"));
        assert!(stage_script.contains("[stage-deploy]"));
        assert!(stage_script.contains("meshc build . --output \"$TARGET_BINARY\""));
        assert!(stage_script.contains("DEPLOY_SQL_NAME=\"todo-api.up.sql\""));
        assert!(stage_script.contains("apply-deploy-migrations.sh"));
        assert!(stage_script.contains("deploy-smoke.sh"));
        assert!(stage_script.contains("bundle path exists but is not a directory"));

        let apply_script = std::fs::read_to_string(&apply_script_path).unwrap();
        assert!(apply_script.contains("usage: bash apply-deploy-migrations.sh <deploy-sql-path>"));
        assert!(apply_script.contains("[deploy-apply]"));
        assert!(apply_script.contains("psql"));
        assert!(apply_script.contains("DATABASE_URL must be set"));
        assert!(apply_script.contains("todos table missing after apply"));
        assert!(apply_script.contains("idx_todos_created_at missing after apply"));

        let deploy_smoke = std::fs::read_to_string(&deploy_smoke_path).unwrap();
        assert!(deploy_smoke.contains("usage: bash deploy-smoke.sh"));
        assert!(deploy_smoke.contains("[deploy-smoke]"));
        assert!(deploy_smoke.contains("/health"));
        assert!(deploy_smoke.contains("Work.sync_todos"));
        assert!(deploy_smoke.contains("POST \"$BASE_URL/todos\""));
        assert!(deploy_smoke.contains("PUT \"$BASE_URL/todos/$TODO_ID\""));
        assert!(deploy_smoke.contains("DELETE \"$BASE_URL/todos/$TODO_ID\""));

        let readme = std::fs::read_to_string(project_dir.join("README.md")).unwrap();
        assert!(readme.contains("meshc init --template todo-api --db postgres"));
        assert!(readme.contains("migrations/"));
        assert!(readme.contains("deploy/"));
        assert!(readme.contains("scripts/"));
        assert!(readme.contains("stage-deploy.sh"));
        assert!(readme.contains("apply-deploy-migrations.sh"));
        assert!(readme.contains("deploy-smoke.sh"));
        assert!(readme.contains("bash scripts/stage-deploy.sh ./.bundle/todo-api"));
        assert!(readme.contains("./.bundle/todo-api/todo-api.up.sql"));
        assert!(readme.contains("DATABASE_URL"));
        assert!(readme.contains(".env.example"));
        assert!(readme.contains("packages the binary produced by `meshc build .`"));
        assert!(readme.contains("One public app URL may front multiple starter nodes"));
        assert!(readme.contains("proxy/platform ingress"));
        assert!(readme.contains("X-Mesh-Continuity-Request-Key"));
        assert!(readme.contains(
            "meshc cluster continuity <node-name@host:port> <request-key> --json"
        ));
        assert!(readme.contains(
            "The response header is a runtime-owned operator/debug seam, not a frontend-aware routing signal."
        ));
        assert!(readme.contains(
            "Use the continuity list form first to discover runtime-owned startup records or for general manual investigation."
        ));
        assert!(readme.contains("frontend-aware node selection"));
        assert!(readme.contains("Fly-specific product contract"));
        assert!(readme.contains(
            "meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics`"
        ));
        assert!(readme.contains("meshc init --template todo-api --db sqlite"));
        assert!(readme.contains("staged bundle is the public deploy contract"));
        assert!(readme.contains("does not run migrations or create schema at startup"));
    }

    #[test]
    fn m049_s01_postgres_scaffold_omits_sqlite_markers_and_secret_leaks() {
        let tmp = TempDir::new().unwrap();
        scaffold_todo_api_project_with_db("todo-api", tmp.path(), TodoApiDatabase::Postgres)
            .expect("postgres todo-api scaffold should succeed");

        let project_dir = tmp.path().join("todo-api");
        let readme = std::fs::read_to_string(project_dir.join("README.md")).unwrap();
        let dockerfile = std::fs::read_to_string(project_dir.join("Dockerfile")).unwrap();
        let dockerignore = std::fs::read_to_string(project_dir.join(".dockerignore")).unwrap();
        let env_example = std::fs::read_to_string(project_dir.join(".env.example")).unwrap();
        let health = std::fs::read_to_string(project_dir.join("api/health.mpl")).unwrap();
        let registry = std::fs::read_to_string(project_dir.join("runtime/registry.mpl")).unwrap();
        let config_test =
            std::fs::read_to_string(project_dir.join("tests/config.test.mpl")).unwrap();
        let stage_script =
            std::fs::read_to_string(project_dir.join("scripts/stage-deploy.sh")).unwrap();
        let apply_script =
            std::fs::read_to_string(project_dir.join("scripts/apply-deploy-migrations.sh"))
                .unwrap();
        let deploy_smoke =
            std::fs::read_to_string(project_dir.join("scripts/deploy-smoke.sh")).unwrap();
        let deploy_sql = std::fs::read_to_string(
            project_dir
                .join("deploy")
                .join(todo_postgres_deploy_sql_filename("todo-api")),
        )
        .unwrap();

        assert!(!readme.contains("TODO_DB_PATH"));
        assert!(!readme.contains("todo.sqlite3"));
        assert!(!readme.contains("ensure_schema"));
        assert!(!readme.contains("failover"));
        assert!(!readme.contains("Fly.io"));
        assert!(!readme.contains("local single-node SQLite starter"));
        assert!(readme.contains("One public app URL may front multiple starter nodes"));
        assert!(readme.contains("X-Mesh-Continuity-Request-Key"));
        assert!(readme.contains(
            "The response header is a runtime-owned operator/debug seam, not a frontend-aware routing signal."
        ));
        assert!(readme.contains(
            "Use the continuity list form first to discover runtime-owned startup records or for general manual investigation."
        ));
        assert!(readme.contains("The starter does not promise frontend-aware node selection, sticky-session semantics, or a Fly-specific product contract."));
        assert!(!readme.contains("before/after continuity diff"));

        assert!(!dockerfile.contains("TODO_DB_PATH"));
        assert!(!dockerfile.contains("sqlite3"));
        assert!(!dockerfile.contains("VOLUME"));
        assert!(dockerfile.contains("COPY output /usr/local/bin/todo-api"));
        assert!(dockerfile.contains("ENTRYPOINT [\"/usr/local/bin/todo-api\"]"));

        assert!(!dockerignore.contains("*.sqlite3"));
        assert!(dockerignore.contains(".env"));
        assert!(dockerignore.contains("target"));

        assert!(env_example.contains("DATABASE_URL="));
        assert!(env_example.contains("TODO_RATE_LIMIT_WINDOW_SECONDS=60"));
        assert!(env_example.contains("TODO_RATE_LIMIT_MAX_REQUESTS=5"));
        assert!(!env_example.contains("TODO_DB_PATH"));
        assert!(!env_example.contains("sqlite3"));

        assert!(health.contains("db_backend : \"postgres\""));
        assert!(health.contains("migration_strategy : \"meshc migrate\""));
        assert!(!health.contains("DATABASE_URL"));
        assert!(!health.contains("todo.sqlite3"));

        assert!(registry.contains("pool :: PoolHandle"));
        assert!(!registry.contains("database_url :: String"));
        assert!(!registry.contains("GetDatabaseUrl"));

        assert!(config_test.contains("DATABASE_URL"));
        assert!(config_test.contains("TODO_RATE_LIMIT_WINDOW_SECONDS"));
        assert!(config_test.contains("TODO_RATE_LIMIT_MAX_REQUESTS"));
        assert!(!config_test.contains("TODO_DB_PATH"));

        assert!(!stage_script.contains("cargo run -q -p meshc"));
        assert!(!stage_script.contains("todo.sqlite3"));
        assert!(!stage_script.contains("Fly.io"));

        assert!(!apply_script.contains("todo.sqlite3"));
        assert!(!apply_script.contains("Fly.io"));

        assert!(!deploy_smoke.contains("todo.sqlite3"));
        assert!(!deploy_smoke.contains("Fly.io"));
        assert!(!deploy_smoke.contains("TODO_DB_PATH"));

        assert!(!deploy_sql.contains("sqlite"));
        assert!(!deploy_sql.contains("TODO_DB_PATH"));
        assert!(!deploy_sql.contains("DATABASE_URL"));
    }

    #[test]
    fn m049_s02_sqlite_scaffold_rejects_existing_directory() {
        let tmp = TempDir::new().unwrap();
        let existing = tmp.path().join("todo-api");
        std::fs::create_dir_all(&existing).unwrap();

        let result = scaffold_todo_api_project("todo-api", tmp.path());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.contains("already exists"),
            "Error should mention 'already exists', got: {}",
            err
        );
    }
}
