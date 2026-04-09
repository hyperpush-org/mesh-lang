---
id: S05
parent: M047
milestone: M047
provides:
  - Zero-ceremony declared-work wrappers for ordinary `@cluster` functions.
  - Route-free `Work.add` cutover across `tiny-cluster/`, `cluster-proof/`, and `meshc init --clustered`.
  - `meshc init --template todo-api` with native CRUD/rate-limit/restart proof, public docs, and Docker packaging.
requires:
  - slice: S03
    provides: The explicit milestone boundary that clustered HTTP route wrappers are still unshipped, so the Todo scaffold keeps ordinary local HTTP handlers and does not pretend `HTTP.clustered(...)` exists.
  - slice: S04
    provides: The repo-wide source-first `@cluster` cutover and route-free clustered public contract that the Todo starter, examples, and docs now build on.
affects:
  - S06
key_files:
  - compiler/mesh-codegen/src/declared.rs
  - compiler/mesh-pkg/src/scaffold.rs
  - compiler/meshc/src/main.rs
  - compiler/meshc/tests/tooling_e2e.rs
  - compiler/meshc/tests/support/m047_todo_scaffold.rs
  - compiler/meshc/tests/e2e_m047_s05.rs
  - scripts/verify-m047-s05.sh
  - README.md
  - website/docs/docs/tooling/index.md
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/getting-started/clustered-example/index.md
  - tiny-cluster/work.mpl
  - cluster-proof/work.mpl
key_decisions:
  - Keep declared-work compatibility narrow: zero-arg `@cluster` functions are now the public contract, while legacy two-string handlers stay compatibility-only behind compiler-generated wrappers.
  - Do not expose raw `Int ! String` success payloads at the live HTTP delete boundary; return the deleted todo id string instead so the generated route stays stable and user-facing.
  - Package the Todo Docker image from the local `meshc build .` output binary instead of reinstalling Mesh inside the image while the public installer release lags the current `@cluster` scaffold contract.
patterns_established:
  - Keep continuity metadata runtime-owned and compiler-injected; repo examples, scaffolds, and docs should only show ordinary `@cluster` function names.
  - When a live HTTP route only needs success/failure from a mutation, prefer string/domain payloads or `_` bindings over exposing raw `Int ! String` success payloads at the handler boundary.
  - For generated app packaging, keep the Docker path honest to the currently provable build surface: package the built binary instead of pretending a lagging installer-in-container flow already supports the scaffold.
observability_surfaces:
  - `cargo test -p meshc --test e2e_m047_s05 -- --nocapture`
  - `bash scripts/verify-m047-s05.sh`
  - `bash scripts/verify-m047-s04.sh`
  - `cargo test -p mesh-pkg m047_s05 -- --nocapture`
  - `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture`
  - `GET /health` on the generated Todo app returns `status`, `clustered_handler`, `db_path`, and rate-limit configuration.
  - Retained `.tmp/m047-s05/todo-scaffold-runtime-truth-*` bundles now include health snapshots, first/second run logs, persisted SQLite artifacts, and Docker build logs.
drill_down_paths:
  - .gsd/milestones/M047/slices/S05/tasks/T01-SUMMARY.md
  - .gsd/milestones/M047/slices/S05/tasks/T02-SUMMARY.md
  - .gsd/milestones/M047/slices/S05/tasks/T03-SUMMARY.md
  - .gsd/milestones/M047/slices/S05/tasks/T04-SUMMARY.md
  - .gsd/milestones/M047/slices/S05/tasks/T05-SUMMARY.md
  - .gsd/milestones/M047/slices/S05/tasks/T06-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-01T20:00:08.643Z
blocker_discovered: false
---

# S05: Simple clustered Todo scaffold

**S05 completed the source-first clustered scaffold cutover and shipped a proved SQLite Todo starter with native CRUD/rate-limit/persistence proof, ordinary `@cluster` naming, refreshed docs, and Docker packaging.**

## What Happened

S05 started as a blocker-discovery slice: the public clustered contract still leaked `request_key` / `attempt_id`, route-free examples still taught `execute_declared_work(...)`, and the planned Todo scaffold filters were not proving anything real. That upstream seam is now closed. The declared-work wrapper/codegen path was repaired so ordinary zero-arg `@cluster` functions are the public contract, and the route-free public surfaces (`meshc init --clustered`, `tiny-cluster/`, `cluster-proof/`, README/docs, and the historical route-free rails) were rebaselined onto ordinary names like `add()` / `Work.add` instead of wrapper-era public ceremony.

On that base, S05 added a real `meshc init --template todo-api` starter. The generated project now includes a route-free clustered `work.mpl` with `@cluster pub fn sync_todos()`, SQLite-backed storage, HTTP health/list/get/create/toggle/delete routes, a runtime registry, an actor-backed write limiter, README guidance, `.dockerignore`, and a Dockerfile. The generated README and repo docs stay explicit that `HTTP.clustered(...)` is still unshipped; the ordinary clustered function is the visible boundary, while the HTTP handlers remain ordinary local handlers.

The final closeout work retired two product-path failures exposed by the live S05 rail. First, the DELETE route was crashing after a successful mutation because the live handler bound a raw `Int ! String` success payload; the scaffold now returns the deleted todo id string instead of exposing a boxed integer success payload at the HTTP boundary. Second, the Docker packaging path no longer pretends the current public installer release can build the new source-first scaffold inside the image. Instead, the generated Dockerfile packages the `./output` binary produced by `meshc build .`, and the verifier now builds that output in-place before the Docker image build. With those fixes in place, the native Todo CRUD/rate-limit/restart-persistence rail, the assembled S05 verifier, and the docs build all pass together.

## Verification

Executed the full S05 matrix and final assembled proof bundle:
- `cargo test -p meshc --test e2e_m047_s01 -- --nocapture`
- `cargo test -p meshc --test e2e_m047_s02 -- --nocapture`
- `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture`
- `cargo test -p mesh-pkg m047_s05 -- --nocapture`
- `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`
- `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture`
- `cargo run -q -p meshc -- test tiny-cluster/tests`
- `cargo run -q -p meshc -- test cluster-proof/tests`
- `cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture`
- `bash scripts/verify-m047-s04.sh`
- `cargo test -p meshc --test e2e_m047_s05 -- --nocapture`
- `bash scripts/verify-m047-s05.sh`
- `npm --prefix website run build`

The authoritative S05 runtime bundle now retains native health/CRUD/rate-limit/restart/Docker evidence under `.tmp/m047-s05/todo-scaffold-runtime-truth-*`, and the assembled verifier replays the full cutover + Todo + docs stack successfully.

## Requirements Advanced

- R106 — S05 updated the scaffold README, root README, tooling docs, route-free example wording, and the assembled verifier so the public clustered story consistently teaches ordinary `@cluster` function names and an honest Docker packaging path, while leaving the final migration/assembled closeout to S06.

## Requirements Validated

- R104 — `cargo test -p mesh-pkg m047_s05 -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture`, `cargo test -p meshc --test e2e_m047_s05 -- --nocapture`, `bash scripts/verify-m047-s05.sh`, and `npm --prefix website run build` now prove the Todo scaffold generates a SQLite-backed API with real routes, actor-backed rate limiting, restart-persistent state, and a Docker build path.
- R105 — `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture`, `cargo run -q -p meshc -- test tiny-cluster/tests`, `cargo run -q -p meshc -- test cluster-proof/tests`, `cargo test -p meshc --test e2e_m047_s05 -- --nocapture`, and `bash scripts/verify-m047-s05.sh` now prove the public clustered surfaces and Todo starter use ordinary `@cluster` names instead of `execute_declared_work(...)` and read like ordinary starting points rather than wrapper-shaped proof apps.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

The generated Docker packaging story changed during closeout. The original draft installed Mesh inside the image and built the app there, but that was not honest for the current source-first clustered scaffold because the public installer release does not yet build the new `@cluster` template contract. S05 therefore closes on a Dockerfile that packages the already-built `./output` binary from `meshc build .`, which is the smallest honest build path for the current scaffold.

## Known Limitations

`HTTP.clustered(...)` remains unshipped, so the generated Todo API keeps ordinary local HTTP handlers and uses route-free `@cluster` work only.

The generated Dockerfile packages the prebuilt `./output` binary. Rebuild that binary with `meshc build .` on the Linux target platform you intend to ship before running `docker build`.

## Follow-ups

- Fold the now-green Todo starter, Docker packaging story, and public wording into the final M047/S06 migration and assembled closeout surfaces.
- When a public Mesh binary release supports the current source-first clustered scaffold contract end to end, reconsider whether the Dockerfile should switch back to an in-image compiler/install path or keep the package-built-binary model.

## Files Created/Modified

- `compiler/mesh-codegen/src/declared.rs` — Repaired declared-work wrapper generation so ordinary zero-arg `@cluster` functions are the public contract and legacy two-string handlers are compatibility-only.
- `compiler/meshc/tests/e2e_m047_s01.rs` — Updated the source-first clustered-function proof rail to the zero-ceremony public contract.
- `compiler/meshc/tests/e2e_m047_s02.rs` — Updated the replication-count proof rail to ordinary `@cluster` function surfaces.
- `compiler/mesh-pkg/src/scaffold.rs` — Rebased the clustered scaffold to `Work.add`, generated the Todo template, fixed the delete-route return shape, and switched Docker packaging to copy the `meshc build .` output binary.
- `compiler/meshc/src/main.rs` — Added the `meshc init --template todo-api` CLI path.
- `compiler/meshc/tests/tooling_e2e.rs` — Added real Todo template contract rails and updated clustered scaffold/Docker expectations to ordinary `@cluster` naming and output-binary packaging.
- `compiler/meshc/tests/support/m047_todo_scaffold.rs` — Added the Todo scaffold native runtime + Docker packaging harness and in-place `meshc build .` step before Docker build.
- `compiler/meshc/tests/e2e_m047_s05.rs` — Added and then closed the end-to-end Todo scaffold regression rail covering generation, native runtime truth, public surface contract, and Docker build.
- `scripts/verify-m047-s05.sh` — Added the assembled S05 verifier that replays cutover proof, Todo rails, and docs build.
- `tiny-cluster/work.mpl` — Moved the route-free local proof package to `@cluster pub fn add()` / `Work.add`.
- `cluster-proof/work.mpl` — Moved the packaged route-free proof app to `@cluster pub fn add()` / `Work.add`.
- `README.md` — Updated the public clustered starter wording to ordinary `@cluster` names and the honest Todo Docker packaging story.
- `website/docs/docs/tooling/index.md` — Updated Todo template docs to the ordinary `@cluster` contract and the output-binary Docker packaging path.
