---
id: S02
parent: M049
milestone: M049
provides:
  - An explicit local-only SQLite `todo-api` scaffold that generates current Mesh-local files, package tests, README/Docker contract, and live runtime proof.
  - A fixture-backed compatibility seam for the old clustered SQLite Todo contract so the M047 historical rail stays green without teaching that story publicly.
  - An explicit starter split across README, website docs, CLI wording, and Mesh skills: SQLite stays local, Postgres is the fuller shared/deployable path, and `meshc init --clustered` remains the minimal clustered scaffold.
requires:
  - slice: S01
    provides: the typed `--db` init/scaffold seam and the already-green Postgres starter contract that S02 mirrored on the SQLite-local side
affects:
  - S03
  - S04
  - S05
key_files:
  - scripts/fixtures/m047-s05-clustered-todo/mesh.toml
  - scripts/fixtures/m047-s05-clustered-todo/main.mpl
  - scripts/fixtures/m047-s05-clustered-todo/api/router.mpl
  - compiler/meshc/tests/support/m047_todo_scaffold.rs
  - compiler/meshc/tests/e2e_m047_s05.rs
  - scripts/verify-m047-s05.sh
  - compiler/mesh-pkg/src/scaffold.rs
  - compiler/meshc/src/main.rs
  - compiler/meshc/tests/tooling_e2e.rs
  - compiler/meshc/tests/support/m049_todo_sqlite_scaffold.rs
  - compiler/meshc/tests/e2e_m049_s02.rs
  - README.md
  - website/docs/docs/tooling/index.md
  - website/docs/docs/getting-started/clustered-example/index.md
  - website/docs/docs/distributed/index.md
  - website/docs/docs/distributed-proof/index.md
  - tools/skill/mesh/SKILL.md
  - tools/skill/mesh/skills/clustering/SKILL.md
  - tools/skill/mesh/skills/http/SKILL.md
  - scripts/tests/verify-m048-s04-skill-contract.test.mjs
key_decisions:
  - Preserve the historical M047 clustered SQLite Todo contract as a committed fixture instead of preserving a hidden legacy starter mode in public `meshc init`.
  - Treat the SQLite `todo-api` starter as explicitly local/single-node, and teach clustered/deployable guidance through `meshc init --clustered` and `meshc init --template todo-api --db postgres`.
  - Keep generated SQLite package tests at compile-smoke/import-surface coverage and put real CRUD/restart/failure proof in `e2e_m049_s02` until the generated storage-test negative path stops hitting `expected (), found Int` compiler failures.
  - Make docs and Mesh skills fail closed on stale generic `meshc init --template todo-api` wording and clustered-SQLite claims.
  - Keep the retained `scripts/verify-m047-s05.sh` wrapper scoped to the cutover subrail, the fixture-backed historical e2e rail, docs build, and retained-bundle/provenance checks.
patterns_established:
  - Freeze retired public scaffold behavior behind a committed internal fixture when the public contract changes, rather than carrying a shadow legacy mode in `meshc init`.
  - Use generated package tests for scaffold shape and compile/import contract, and a dedicated Rust harness for live runtime behavior when Mesh-side generated tests are still compiler-unstable.
  - Pin public docs and assistant-facing skills with fail-closed wording tests whenever a starter story changes; runtime proof alone is not enough to keep teaching surfaces honest.
  - Retain generated-project snapshots, raw HTTP exchanges, stdout/stderr, and unreachable-health artifacts under `.tmp/<slice>/...` so downstream example-generation slices can consume the actual proved output instead of reconstructing it.
observability_surfaces:
  - Health signal: generated SQLite `/health` returns `status=ok`, `mode=local`, `db_backend=sqlite`, `storage_mode=single-node`, the active `db_path`, and rate-limit settings, while omitting clustered fields such as `clustered_handler`, `migration_strategy`, and `node_name`.
  - Failure signal: startup logs emit `[todo-api] Database init failed: ...` or `[todo-api] Config error: ...`, and the bad-path rail proves `/health` stays unreachable with a retained `*.connect-error.txt` artifact instead of hanging behind a partially started runtime.
  - Recovery: restart the binary with a valid `TODO_DB_PATH`; reusing the same `todo.sqlite3` path preserves data across process restart, which the live e2e harness proves via `restart-health.json` plus post-restart CRUD snapshots.
  - Monitoring gap: the SQLite starter is intentionally local-only and currently exposes only HTTP `/health` plus local stdout/stderr logs; it does not ship cluster/operator diagnostics or external metrics/alert hooks.
drill_down_paths:
  - .gsd/milestones/M049/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M049/slices/S02/tasks/T02-SUMMARY.md
  - .gsd/milestones/M049/slices/S02/tasks/T03-SUMMARY.md
  - .gsd/milestones/M049/slices/S02/tasks/T04-SUMMARY.md
  - .gsd/milestones/M049/slices/S02/tasks/T05-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-03T00:42:49.381Z
blocker_discovered: false
---

# S02: SQLite local starter contract

**Shipped an explicit local-only SQLite `todo-api` starter with live CRUD/restart/failure proof, moved the old clustered SQLite Todo contract behind fixture-backed M047 rails, and rewired docs plus Mesh skills to teach SQLite-local vs Postgres-clustered honestly.**

## What Happened

S02 split the old single public Todo story into two truthful contracts. First, it froze the historical M047 clustered SQLite Todo starter under `scripts/fixtures/m047-s05-clustered-todo/` and rewired `compiler/meshc/tests/support/m047_todo_scaffold.rs`, `compiler/meshc/tests/e2e_m047_s05.rs`, and `scripts/verify-m047-s05.sh` so the old native/Docker/runtime proof now copies a committed fixture, records `source=fixture-copy` provenance in `init.log`, and fails closed on missing or partial fixture trees instead of silently depending on the evolving public scaffold.

Then it rewrote the public SQLite branch in `compiler/mesh-pkg/src/scaffold.rs` into an explicitly local single-node starter. The generated project now emits `config.mpl`, a local-only `main.mpl`, local `/health`, typed SQLite storage via `deriving(Json, Row)` / `Todo.from_row(...)`, generated package-test files, a SQLite-specific README/Docker contract, and no `work.mpl`, `Node.start_from_env()`, `HTTP.clustered(...)`, `meshc cluster`, or `MESH_*` runtime story. `compiler/meshc/src/main.rs` and `compiler/meshc/tests/tooling_e2e.rs` were updated so CLI/init guidance teaches the explicit split: `--db sqlite` is the honest local starter, `--db postgres` is the fuller shared/deployable starter, and `--clustered` stays the minimal clustered scaffold.

S02 also added a live acceptance harness in `compiler/meshc/tests/support/m049_todo_sqlite_scaffold.rs` and `compiler/meshc/tests/e2e_m049_s02.rs`. That harness generates the SQLite starter into a temp workspace, runs `meshc test` and `meshc build`, boots the binary without any cluster env, proves local `/health`, empty-list, create/fetch/toggle, malformed-id and blank-title rails, rate limiting, and restart persistence against the same `todo.sqlite3`, and retains rich `.tmp/m049-s02/...` artifacts including generated-project snapshots, build/test logs, raw HTTP exchanges, stdout/stderr, restart snapshots, and unreachable-health evidence for the bad-`TODO_DB_PATH` path.

On the public-surface side, S02 rewrote `README.md`, the M047-facing website pages, the root Mesh skill, the clustering skill, and the HTTP skill so they all spell the starter split explicitly. The docs and skill contract rails now fail closed on stale generic `meshc init --template todo-api` wording or clustered-SQLite claims. During slice closeout, the retained `scripts/verify-m047-s05.sh` wrapper was narrowed to the truthful post-S02 contract: replay `scripts/verify-m047-s04.sh`, replay `cargo test -p meshc --test e2e_m047_s05 -- --nocapture`, rebuild the website docs, and then validate retained fixture provenance and bundle shape. That removed stale dependence on unrelated public scaffold filters once the historical clustered Todo story became fixture-backed.

## Verification

Verified the slice with the full named closeout set:

- `cargo test -p meshc --test e2e_m047_s05 -- --nocapture` — passed after updating the retained public-surface assertions and wrapper contract to the explicit SQLite/Postgres starter split.
- `bash scripts/verify-m047-s05.sh` — passed after narrowing the historical wrapper to the delegated M047 cutover rail, the fixture-backed `e2e_m047_s05` replay, docs build, and retained provenance/bundle-shape checks.
- `cargo test -p mesh-pkg m049_s02_sqlite_scaffold_ -- --nocapture` — passed (3 tests), proving the local-only SQLite file set and generated package-test files.
- `cargo test -p meshc --test tooling_e2e test_init_todo_template_db_sqlite_ -- --nocapture` — passed (5 tests), proving the CLI/init wording and generated-project contract for the SQLite local starter.
- `cargo test -p meshc --test e2e_m049_s02 -- --nocapture` — passed (2 tests), proving live local `/health`, CRUD, restart persistence, rate limiting, and bad-`TODO_DB_PATH` failure truth with retained `.tmp/m049-s02/...` artifacts.
- `cargo test -p meshc --test e2e_m047_s06 -- --nocapture` — passed (3 tests), proving the docs contract now requires the explicit SQLite-local/Postgres-clustered split.
- `npm --prefix website run build` — passed, confirming the edited VitePress docs still render.
- `node --test scripts/tests/verify-m048-s04-skill-contract.test.mjs` — passed (4 tests), proving the Mesh skill bundle publishes the new starter split and fails closed on stale clustered-SQLite wording.
- `node --test scripts/tests/verify-m048-s05-contract.test.mjs` — passed (4 tests), proving the broader retained M048 public-contract rail stayed green.

Operational surfaces were also confirmed from the retained SQLite runtime artifacts:
- `.tmp/m049-s02/todo-api-sqlite-runtime-truth-1775176542713626000/health.json` returned `status=ok`, `mode=local`, `db_backend=sqlite`, `storage_mode=single-node`, `db_path=.../todo.sqlite3`, and the configured rate-limit values, with no clustered metadata.
- `.tmp/m049-s02/todo-api-sqlite-runtime-truth-1775176542713626000/runtime.stdout.log` showed `[todo-api] local config loaded`, `[todo-api] SQLite schema ready`, `[todo-api] local runtime ready`, and `[todo-api] HTTP server starting`.
- `.tmp/m049-s02/todo-api-sqlite-bad-db-path-1775176542713626000/bad-db-path-runtime.stdout.log` showed `[todo-api] Database init failed: unable to open database file`, and `bad-db-path-health.connect-error.txt` confirmed `/health` stayed unreachable on the broken path.

## Requirements Advanced

- R122 — S02 proved the SQLite half of the long-term split explicitly: the starter now stays local/single-node, omits clustered/operator claims, and is backed by live runtime proof plus docs/skill wording guards.

## Requirements Validated

- R115 — With S01's Postgres starter already green, S02 completed the dual-db scaffold contract. The combined proof now includes `cargo test -p mesh-pkg m049_s02_sqlite_scaffold_ -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_init_todo_template_db_sqlite_ -- --nocapture`, and `cargo test -p meshc --test e2e_m049_s02 -- --nocapture`, which close the missing SQLite half of `meshc init --template todo-api` on current Mesh patterns.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

The written plan assumed the generated SQLite package tests would directly exercise storage behavior through `meshc test <generated-project>`. On this tree, the generated negative storage helper/assertion path still drifts into Mesh compiler `expected (), found Int` failures, so the slice narrowed generated package tests to compile-smoke/import-surface coverage and moved behavioral proof into the dedicated Rust e2e harness. During closeout, the retained `scripts/verify-m047-s05.sh` wrapper also had to drop stale `mesh-pkg` and `tooling_e2e` filters, because once the historical clustered Todo contract became fixture-backed those public scaffold filters were no longer part of the retained M047 proof surface and were producing unrelated red drift.

## Known Limitations

Direct generated package-test calls into the SQLite storage helper negative path are still compiler-unstable under `meshc test`, so generated package tests currently prove scaffold shape/importability while live behavior is owned by `compiler/meshc/tests/e2e_m049_s02.rs`. The public SQLite starter is intentionally single-node only and does not claim clustered/operator or shared durability semantics. Public proof-app surfaces such as `tiny-cluster/` and `cluster-proof/` are still present as bounded references until S04 retires them in favor of generated examples.

## Follow-ups

S03 should snapshot the proven scaffold outputs into checked-in `/examples/todo-sqlite` and `/examples/todo-postgres`, using the retained `.tmp/m049-s02/.../generated-project` bundle instead of reconstructing the SQLite contract by hand. S04 should remove the remaining top-level proof-app onboarding references now that docs and skills already teach the explicit SQLite-local/Postgres-clustered split. A later compiler/runtime follow-up can revisit the generated `tests/storage.test.mpl` negative helper path once the `expected (), found Int` `meshc test` instability is fixed.

## Files Created/Modified

- `scripts/fixtures/m047-s05-clustered-todo/mesh.toml` — Committed the historical clustered SQLite Todo fixture root so the old M047 contract no longer depends on public scaffold output.
- `compiler/meshc/tests/support/m047_todo_scaffold.rs` — Switched the historical M047 helper to fixture-copy mode, added fail-closed fixture validation, and recorded retained provenance markers.
- `compiler/meshc/tests/e2e_m047_s05.rs` — Updated the retained historical rail to validate fixture provenance, the narrowed wrapper contract, and the explicit SQLite/Postgres public wording split.
- `scripts/verify-m047-s05.sh` — Narrowed the retained M047 wrapper to the truthful post-S02 scope: delegated cutover replay, fixture-backed e2e replay, docs build, and retained provenance/bundle checks.
- `compiler/mesh-pkg/src/scaffold.rs` — Rewrote the SQLite `todo-api` generator into an explicit local-only starter with local config/health/storage/test/readme/docker surfaces.
- `compiler/meshc/src/main.rs` — Split `meshc init` guidance between SQLite-local, Postgres-shared, and minimal clustered scaffold flows.
- `compiler/meshc/tests/tooling_e2e.rs` — Updated CLI/init assertions so the generated SQLite starter and its `meshc test` path are checked as the current local contract.
- `compiler/meshc/tests/support/m049_todo_sqlite_scaffold.rs` — Added the live SQLite scaffold harness with retained generated-project, build/test, runtime log, health, restart, and bad-db-path artifacts.
- `compiler/meshc/tests/e2e_m049_s02.rs` — Added the slice-owned local SQLite CRUD/restart/failure truth rail.
- `README.md` — Reframed the public starter story to spell out SQLite-local, Postgres-clustered, and minimal `meshc init --clustered` guidance explicitly.
- `website/docs/docs/tooling/index.md` — Updated public CLI/scaffold docs to teach the explicit starter split and keep the historical M047 Todo rail bounded.
- `tools/skill/mesh/skills/clustering/SKILL.md` — Updated assistant-facing clustered guidance so SQLite stays local-only while clustered questions route to `--clustered` or the Postgres starter.
