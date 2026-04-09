# M049/S02 Research — SQLite local starter contract

## Summary

- `R115` still depends on the SQLite half of the Todo starter, but the current SQLite branch in `compiler/mesh-pkg/src/scaffold.rs` is still the old clustered starter in public clothing: `main.mpl` calls `Node.start_from_env()`, the scaffold emits `work.mpl`, `GET /todos` and `GET /todos/:id` use `HTTP.clustered(1, ...)`, `/health` reports `clustered_handler`, and the generated README teaches `meshc cluster` plus optional `MESH_*` env.
- The honest SQLite seam already proven in this repo is **not** the Postgres `Pool`/`Migration` story. The repo-local Mesh database skill (`tools/skill/mesh/skills/database/SKILL.md`) still treats SQLite as `Sqlite.open` / `Sqlite.execute` / `Sqlite.query` / `Sqlite.close`, with typed row conversion through `deriving(Row)`. That means S02 should modernize SQLite as a good local starter, not pretend it now shares the serious clustered/deployable contract.
- Changing the public SQLite scaffold will immediately break historical M047 surfaces that still generate `meshc init --template todo-api` and expect the old clustered Todo app: `compiler/meshc/tests/support/m047_todo_scaffold.rs`, `compiler/meshc/tests/e2e_m047_s05.rs`, `scripts/verify-m047-s05.sh`, and the M047 docs contract in `compiler/meshc/tests/e2e_m047_s06.rs`.
- The clean escape hatch is an **internal fixture-backed compatibility seam**, not another hidden public scaffold mode. The repo already has fixture-copy precedents (`compiler/meshc/tests/e2e_m034_s12.rs::copy_smoke_fixture`, the M048 override-entry fixture writers), so S02 can move the old clustered Todo proof off the public scaffold without inventing a second shadow template.
- S02 also has bounded public wording fallout. Root docs and Mesh skill surfaces currently describe `meshc init --template todo-api` as a clustered starter. Those statements need to split into **SQLite local** vs **Postgres serious clustered** wording now, even though full proof-app retirement still belongs to S04.

## Requirements Targeted

- **R115** — supporting owner for S02. S01 added the typed `--db` seam and Postgres branch; S02 has to make the SQLite/default branch truthful and current instead of preserving the old clustered SQLite contract.
- **R122** — supporting owner for S02. This slice owns the “SQLite stays explicitly local/single-node” half of the requirement. The current public SQLite starter violates that in code, README text, docs, and skill guidance.
- **R116** — enabler for S03. `/examples/todo-sqlite` cannot be generated honestly until the SQLite scaffold output stops moving and its public contract is explicit.
- **R112 / R113 / R114** — non-regression guardrails only. S02 will likely touch Mesh skill/docs wording, so it must preserve the already-landed entrypoint/update/editor truths while changing the Todo starter story.
- **R127** — boundary only, not owned here. S02 should not try to fully retire `tiny-cluster/`, `cluster-proof/`, or `reference-backend` as public surfaces yet; it only needs to stop claiming that the SQLite Todo starter is itself one of the clustered canonical surfaces.

## Skills Discovered

- Loaded: **SQLite Database Expert**
  - Relevant rules for this slice: parameterized queries only, test-first verification, transaction discipline where multi-step writes need it, and migration/security honesty instead of pretending local SQLite has a shared deployment story.
- Loaded repo-local skill: **tools/skill/mesh/skills/database/SKILL.md**
  - Relevant rules for this slice: SQLite’s existing truthful Mesh API is `Sqlite.open` / `execute` / `query` / `close`; typed row conversion should use `deriving(Row)` instead of bespoke `Map.get(...)` parsing when the row shape is stable.
- No additional skill installs were needed. The only directly relevant external technology for S02 is SQLite, and it already has installed skill coverage.

## Do Not Hand-Roll

- Do **not** keep the public SQLite starter on `Node.start_from_env()` / `work.mpl` / `HTTP.clustered(...)` just to keep M047 history green. That preserves the exact false clustered durability story this slice is supposed to remove.
- Do **not** force the Postgres migration/pool contract onto SQLite. There is no proved neutral SQLite `PoolHandle` + `meshc migrate` story in the current repo; the honest local path is still direct `Sqlite.*` plus local file init.
- Do **not** preserve historical M047 clustered proof by adding another public starter flag/template. If the old clustered Todo app still matters for historical verifier coverage, move it to an internal fixture that tests can copy.
- Do **not** mix S03 `/examples` generation or S04 proof-app retirement into the core scaffold rewrite. S02 should stabilize the SQLite output and its wording; later slices can snapshot and replace broader onboarding surfaces.

## Implementation Landscape

### 1. The current SQLite scaffold is still a clustered starter

**Primary file:** `compiler/mesh-pkg/src/scaffold.rs`

**Current public SQLite output (from `scaffold_sqlite_todo_api_project(...)`):**
- `main.mpl` logs runtime bootstrap and always calls `Node.start_from_env()` before starting HTTP.
- `work.mpl` contains `@cluster pub fn sync_todos() -> Int do 1 + 1 end`.
- `api/router.mpl` wraps `GET /todos` and `GET /todos/:id` in `HTTP.clustered(1, ...)`.
- `api/health.mpl` reports `clustered_handler : "Work.sync_todos"`.
- `README.md` teaches optional `MESH_CLUSTER_COOKIE`, `MESH_NODE_NAME`, `MESH_DISCOVERY_SEED`, `MESH_CLUSTER_PORT`, `MESH_CONTINUITY_ROLE`, and `MESH_CONTINUITY_PROMOTION_EPOCH`, plus `meshc cluster status|continuity|diagnostics`.
- `Dockerfile` exposes `4370` and sets `MESH_CLUSTER_PORT=4370`.

**Implication:**
- S02 is not a light wording pass. The public SQLite scaffold contract needs a real shape change: local-only runtime, local-only README, local-only health surface, and local-only Docker story.

### 2. The CLI still teaches the old SQLite story in its conflict text

**Primary file:** `compiler/meshc/src/main.rs`

**Current SQLite-specific wording:**
- `resolve_init_target(...)` currently rejects `meshc init --clustered --template todo-api ...` with:
  - ``use `meshc init --template todo-api <name>` for the current SQLite starter.``

**Implication:**
- Once SQLite becomes explicitly local, this wording becomes dishonest immediately.
- The clean split is:
  - `meshc init --clustered <name>` = minimal route-free clustered scaffold
  - `meshc init --template todo-api --db postgres <name>` = serious clustered/deployable starter
  - `meshc init --template todo-api [--db sqlite] <name>` = local starter

### 3. The honest SQLite modernization path is `Sqlite.*` plus typed row conversion, not fake Postgres parity

**Files / proofs:**
- `tools/skill/mesh/skills/database/SKILL.md`
- `tests/e2e/deriving_row_basic.mpl`
- `mesher/types/event.mpl`
- `compiler/mesh-rt/src/db/row.rs`

**What is already proven:**
- SQLite is still exposed as direct `Sqlite.open` / `execute` / `query` / `close`.
- `deriving(Row)` is live and supports `Int`, `Float`, `Bool`, `String`, and `Option<T>`.
- `compiler/mesh-rt/src/db/row.rs` explicitly accepts both true and false string variants (`"true"`, `"t"`, `"1"`, `"yes"`, and `"false"`, `"f"`, `"0"`, `"no"`).
- Multiple derives compose (`mesher/types/event.mpl` uses `end deriving(Schema, Json, Row)`).

**Implication:**
- S02 can modernize the SQLite starter without pretending it has the Postgres ORM/pool story by making `Todo` derive `Json, Row` and replacing manual `row_to_todo(...)` parsing with `Todo.from_row(...)`.
- That is a real “current Mesh pattern” improvement and keeps the local path honest.

### 4. The current SQLite starter has no generated package-test seam

**Files:**
- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs`

**Observed gap:**
- The SQLite generator does **not** emit `tests/` or any package-level `.test.mpl` files.
- The current explicit SQLite tooling test (`test_init_todo_template_db_sqlite_explicit_flag_preserves_current_starter`) only checks for `TODO_DB_PATH`, `ensure_schema`, and `Sqlite.open`.
- The Postgres starter now emits `tests/config.test.mpl`, and the live S01 harness expects `meshc test <project>` to run real generated package tests.

**Implication:**
- S02 should add a small generated test surface for SQLite too. It does not need Postgres-sized migration/config tests, but it should stop making `meshc test <project>` a zero-proof run.
- Low-risk candidates:
  - local config helper tests (port/rate-limit/db-path defaults/validation)
  - local storage roundtrip tests against `":memory:"` or a temp DB path
  - an injection-shaped title regression if the starter continues to use raw SQL strings with parameters

### 5. Historical M047 clustered-Todo proof is tightly coupled to the public scaffold

**Files:**
- `compiler/meshc/tests/support/m047_todo_scaffold.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `scripts/verify-m047-s05.sh`
- `compiler/meshc/tests/e2e_m047_s06.rs`

**Current coupling:**
- `m047_todo_scaffold::init_todo_project(...)` literally runs `meshc init --template todo-api`.
- `e2e_m047_s05.rs` asserts the generated project contains `work.mpl`, `@cluster pub fn sync_todos()`, `HTTP.clustered(1, ...)`, `meshc cluster` guidance, and containerized SQLite persistence.
- `scripts/verify-m047-s05.sh` still reruns `cargo test -p mesh-pkg m047_s05 -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture`, and `cargo test -p meshc --test e2e_m047_s05 -- --nocapture`.
- `e2e_m047_s06.rs` still enforces public docs text that calls `meshc init --template todo-api` the clustered “fuller starter.”

**Implication:**
- Rewriting the public SQLite scaffold without first or simultaneously moving these rails will create wide unrelated breakage.
- `e2e_m047_s07.rs` is **not** part of this blast radius; it materializes its own temporary clustered HTTP project and does not rely on the scaffold.

### 6. The repo already has fixture-copy precedents for compatibility surfaces

**Files:**
- `compiler/meshc/tests/e2e_m034_s12.rs`
- `compiler/meshc/tests/e2e_m048_s01.rs`

**What exists now:**
- `e2e_m034_s12.rs::copy_smoke_fixture(...)` copies a committed package fixture into a temp workspace before build/run.
- The M048 override-entry tests write controlled temp fixtures instead of relying on repo CWD or public scaffolds.

**Implication:**
- If S02 needs to preserve the old clustered Todo app for historical M047 rails, the cleanest seam is a committed internal fixture directory plus a helper that copies it into temp workspaces.
- That is cheaper and cleaner than keeping legacy clustered Todo logic in `mesh-pkg` or adding a hidden second public init path.

### 7. Public docs and Mesh skills still claim the Todo starter is clustered

**Files:**
- `README.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `tools/skill/mesh/SKILL.md`
- `tools/skill/mesh/skills/clustering/SKILL.md`
- `tools/skill/mesh/skills/http/SKILL.md`
- `scripts/tests/verify-m048-s04-skill-contract.test.mjs`
- `compiler/meshc/tests/e2e_m047_s06.rs`

**Current pinned story:**
- `README.md` says `meshc init --template todo-api` is the “fuller starter” that keeps the same source-first clustered contract.
- Tooling and clustered-example docs say the SQLite Todo template keeps `@cluster pub fn sync_todos()` and selected `HTTP.clustered(1, ...)` routes.
- The Mesh root skill and clustering skill route clustered questions to `meshc init --template todo-api` as part of the clustered runtime story.
- `verify-m048-s04-skill-contract.test.mjs` explicitly asserts the clustering skill includes `meshc init --template todo-api`, `GET /todos`, `GET /health`, and “mutating routes stay local.”

**Implication:**
- S02 must update the wording split at the same time as the scaffold split, or the repo’s public/assistant-facing contract becomes wrong immediately.
- This does **not** require full S04 proof-app retirement yet. It only requires changing statements about what the Todo starter means:
  - `--db sqlite` = local starter
  - `--db postgres` = clustered/deployable starter
  - `meshc init --clustered` = canonical minimal clustered scaffold

### 8. S03 depends on S02 stabilizing file layout and wording

**Observed repo state:**
- There is still no top-level `examples/` directory.
- `tiny-cluster/` and `cluster-proof/` still exist at repo root.

**Implication:**
- S03 should not snapshot `/examples/todo-sqlite` until S02 decides the real public SQLite shape:
  - whether `work.mpl` disappears
  - whether `/health` keeps `db_path` or changes to `db_backend/storage_mode`
  - whether Docker still exposes only `8080`
  - how README distinguishes local SQLite from serious Postgres

## Recommendation

1. **Make the public SQLite starter explicitly local.** Keep bare `meshc init --template todo-api <name>` defaulting to SQLite if desired, but treat it as the local starter and show `--db sqlite` explicitly in docs so the database choice stays visible.
2. **Remove clustered runtime surfaces from the public SQLite output.** The honest SQLite starter should not emit or teach `work.mpl`, `Node.start_from_env()`, `HTTP.clustered(...)`, `MESH_*` env, or `meshc cluster` inspection guidance.
3. **Modernize within the real SQLite seam.** Stay on `Sqlite.*`, parameterized queries, and add typed row conversion (`deriving(Json, Row)`) plus generated package tests rather than pretending SQLite now has Postgres-style pools/migrations.
4. **Split public wording immediately.** Clustered docs/skills should move clustered Todo guidance to `meshc init --template todo-api --db postgres`; local starter guidance should use `--db sqlite`. Do this as bounded truth maintenance, not as the full S04 onboarding rewrite.
5. **Preserve historical M047 clustered proof via internal fixtures.** Before or alongside the public scaffold rewrite, move the old clustered Todo contract behind a committed fixture copied by tests/helpers. That keeps historical M047 rails green without forcing the public starter to lie.

## Natural Seams

### Seam A — Public SQLite scaffold rewrite

**Files:**
- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/mesh-pkg/src/lib.rs` (likely unchanged export surface, but comments/tests may shift)

**Work:**
- redefine the SQLite generator as local-only
- remove clustered files/guidance from generated output
- add generated tests/config helpers as needed
- optionally convert `Todo` to `deriving(Json, Row)` and simplify storage mapping

**Proof:**
- `cargo test -p mesh-pkg <m049_s02 filter> -- --nocapture`

### Seam B — CLI + tooling contract

**Files:**
- `compiler/meshc/src/main.rs`
- `compiler/meshc/tests/tooling_e2e.rs`

**Work:**
- update `resolve_init_target(...)` messaging to stop teaching the old SQLite-clustered story
- change tooling tests from “preserves current starter” to explicit local-start contract
- decide how much the bare no-`--db` command should still be used in docs vs tests

**Proof:**
- `cargo test -p meshc --test tooling_e2e <sqlite/local filters> -- --nocapture`

### Seam C — New SQLite live acceptance harness

**Likely new files:**
- `compiler/meshc/tests/support/m049_todo_sqlite_scaffold.rs`
- `compiler/meshc/tests/e2e_m049_s02.rs`
- `compiler/meshc/tests/support/mod.rs`

**Work:**
- scaffold a SQLite todo app into a temp workspace with `meshc init --template todo-api --db sqlite`
- run `meshc test <project>`
- build the binary
- boot it locally without `MESH_*` env
- prove `/health` + CRUD + persistence across restart + at least one explicit failure signal (`TODO_DB_PATH` bad path, malformed ID, etc.)
- if the scaffold keeps raw SQL, include one injection-shaped title roundtrip to verify parameters stay literal

**Proof:**
- `cargo test -p meshc --test e2e_m049_s02 -- --nocapture`

### Seam D — Historical M047 compatibility extraction

**Files:**
- `compiler/meshc/tests/support/m047_todo_scaffold.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `scripts/verify-m047-s05.sh`
- possibly a new committed fixture directory (for example under `scripts/fixtures/` or `compiler/meshc/tests/fixtures/`)

**Work:**
- snapshot the old clustered Todo starter into an internal fixture
- update the M047 helper to copy that fixture instead of calling public `meshc init --template todo-api`
- keep M047 clustered proof green without tying it to the new SQLite public contract

**Proof:**
- `cargo test -p meshc --test e2e_m047_s05 -- --nocapture`
- `bash scripts/verify-m047-s05.sh`

### Seam E — Public docs / Mesh skill split

**Files:**
- `README.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `tools/skill/mesh/SKILL.md`
- `tools/skill/mesh/skills/clustering/SKILL.md`
- `tools/skill/mesh/skills/http/SKILL.md`
- `scripts/tests/verify-m048-s04-skill-contract.test.mjs`
- `compiler/meshc/tests/e2e_m047_s06.rs`

**Work:**
- make clustered guidance point at `meshc init --clustered` and `meshc init --template todo-api --db postgres`
- make local-starter guidance point at `meshc init --template todo-api --db sqlite`
- preserve M048 entrypoint/update/editor truth while changing only the Todo starter wording
- do **not** fully remove proof-app references yet; that is S04

**Proof:**
- `node --test scripts/tests/verify-m048-s04-skill-contract.test.mjs`
- `node --test scripts/tests/verify-m048-s05-contract.test.mjs`
- `cargo test -p meshc --test e2e_m047_s06 -- --nocapture`
- `npm --prefix website run build`

## What to Build or Prove First

1. **Lock the compatibility strategy for the old clustered Todo proof.** If S02 changes the public scaffold before deciding this, M047 history explodes across tests, scripts, and docs contracts.
2. **Rewrite the public SQLite scaffold + CLI wording together.** The public artifact and the CLI explanations need to change in one pass or the repo will teach two contradictory starter stories.
3. **Add the new SQLite generated-test and live-runtime rail.** That proves the new local contract independently instead of inheriting confidence from the old clustered SQLite rail.
4. **Only then update docs and Mesh skill wording.** Once the new public SQLite shape is real and the old clustered proof is internalized, the doc split becomes mechanical rather than speculative.
5. **Leave `/examples` generation for S03.** Once the SQLite contract is stable, S03 can snapshot it without re-litigating local-vs-clustered semantics.

## Verification Surfaces

### Likely S02 acceptance commands

- new/updated SQLite generator rail:
  - `cargo test -p mesh-pkg <m049_s02 sqlite filter> -- --nocapture`
- new/updated CLI rail:
  - `cargo test -p meshc --test tooling_e2e <sqlite/local todo filters> -- --nocapture`
- new SQLite live runtime rail:
  - `cargo test -p meshc --test e2e_m049_s02 -- --nocapture`

### Regression rails that matter if S02 touches history/docs

- historical clustered-Todo compatibility:
  - `cargo test -p meshc --test e2e_m047_s05 -- --nocapture`
  - `bash scripts/verify-m047-s05.sh`
- doc / skill truth:
  - `cargo test -p meshc --test e2e_m047_s06 -- --nocapture`
  - `node --test scripts/tests/verify-m048-s04-skill-contract.test.mjs`
  - `node --test scripts/tests/verify-m048-s05-contract.test.mjs`
  - `npm --prefix website run build`

## Risks / Unknowns

- **Highest risk:** trying to preserve the old clustered Todo behavior inside the public SQLite scaffold instead of moving the historical proof internal. That keeps the public starter dishonest and still leaves S04 with the same cleanup burden.
- **Second risk:** swinging too hard the other direction and deleting clustered-Todo proof without replacing it. The current M047 verifier stack still expects that history to exist somewhere.
- **Third risk:** forcing SQLite into a fake Postgres parity story (pool, migrations, deploy semantics). The repo does not currently prove that, and the loaded SQLite + Mesh database skills both say not to fake it.
- **Fourth risk:** changing only code and not docs/skill surfaces. Right now the public repo and the assistant-facing Mesh skills explicitly teach the wrong story for `meshc init --template todo-api`.
- **Open implementation choice:** whether historical M047 compatibility should live under `scripts/fixtures/` or a test-local fixture directory. Either is fine; the important property is that it is **not** another public scaffold branch in `mesh-pkg`.

## Planner Notes

- `compiler/meshc/tests/e2e_m047_s07.rs` is already custom-project-based and is **not** tied to the public scaffold rewrite. Do not over-touch it.
- `scripts/tests/verify-m048-s05-contract.test.mjs` does not pin Todo-starter clustered wording today; the sharper guardrail is `scripts/tests/verify-m048-s04-skill-contract.test.mjs` plus `compiler/meshc/tests/e2e_m047_s06.rs`.
- If S02 adopts `deriving(Json, Row)` for `Todo`, the runtime already supports both truthy and falsey SQLite text variants, so storing `"true"` / `"false"` remains compatible.
- Root/website docs can keep proof-app references until S04; S02 only needs to stop calling the SQLite Todo starter part of the clustered contract.
- `/examples` does not exist yet. S03 should snapshot from the final S02/S01 scaffold outputs, not from any transitional compatibility fixture.
