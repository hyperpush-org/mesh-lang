---
id: T01
parent: S01
milestone: M033
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-rt/src/db/query.rs", "compiler/mesh-rt/src/db/repo.rs", "compiler/mesh-rt/src/lib.rs", "compiler/mesh-typeck/src/infer.rs", "compiler/mesh-codegen/src/mir/lower.rs", "compiler/mesh-codegen/src/codegen/intrinsics.rs", "compiler/meshc/tests/e2e_m033_s01.rs", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Represent structured SELECT items as internal `EXPR:` query slots plus ordered `select_params` so placeholder numbering stays stable without exposing `RAW:` to Mesh code.", "Expose `Expr.label(...)` as the Mesh-callable aliasing surface for now, while still lowering to `mesh_expr_alias`, because `Expr.alias(...)` currently collides with keyword parsing after module qualification.", "Make the M033/S01 compiler e2e helper rebuild `mesh-rt` once before temp-project compilation so new runtime ABI symbols are present in `target/debug/libmesh_rt.a` during Mesh binary linking."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task-level verification passed with the new focused proofs: `cargo test -p meshc --test e2e_m033_s01 expr_ -- --nocapture` passed with `e2e_m033_expr_select_executes`, `e2e_m033_expr_repo_executes`, and both `expr_error_*` checks green; `cargo test -p mesh-rt db::repo::tests::test_select_expr_sql_renumbers_select_params_before_where_params -- --nocapture` passed to prove SQL placeholder stability in the runtime builder; and `cargo run -q -p meshc -- build mesher` passed after the compiler/runtime wiring changes.

I also ran the broader slice acceptance target `cargo test -p meshc --test e2e_m033_s01 -- --nocapture` to assess slice status under recovery. That broader suite failed only in the later Mesher mutation/upsert acceptance tests because `/api/v1/events` returned HTTP 429 rate-limit responses; the expr-focused proofs and both `expr_error_*` checks inside the same run passed."
completed_at: 2026-03-25T06:30:01.049Z
blocker_discovered: false
---

# T01: Added Query.select_exprs with compiler/runtime wiring and expr select e2e coverage

> Added Query.select_exprs with compiler/runtime wiring and expr select e2e coverage

## What Happened
---
id: T01
parent: S01
milestone: M033
key_files:
  - compiler/mesh-rt/src/db/query.rs
  - compiler/mesh-rt/src/db/repo.rs
  - compiler/mesh-rt/src/lib.rs
  - compiler/mesh-typeck/src/infer.rs
  - compiler/mesh-codegen/src/mir/lower.rs
  - compiler/mesh-codegen/src/codegen/intrinsics.rs
  - compiler/meshc/tests/e2e_m033_s01.rs
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Represent structured SELECT items as internal `EXPR:` query slots plus ordered `select_params` so placeholder numbering stays stable without exposing `RAW:` to Mesh code.
  - Expose `Expr.label(...)` as the Mesh-callable aliasing surface for now, while still lowering to `mesh_expr_alias`, because `Expr.alias(...)` currently collides with keyword parsing after module qualification.
  - Make the M033/S01 compiler e2e helper rebuild `mesh-rt` once before temp-project compilation so new runtime ABI symbols are present in `target/debug/libmesh_rt.a` during Mesh binary linking.
duration: ""
verification_result: mixed
completed_at: 2026-03-25T06:30:01.051Z
blocker_discovered: false
---

# T01: Added Query.select_exprs with compiler/runtime wiring and expr select e2e coverage

**Added Query.select_exprs with compiler/runtime wiring and expr select e2e coverage**

## What Happened

I verified the existing M033/S01 runtime/compiler work first and found that the neutral expression builder plus expression-aware write paths were already present, but the portable SELECT side of the contract was still missing from the Query surface. I added a new `Query.select_exprs` runtime entrypoint in `compiler/mesh-rt/src/db/query.rs`, encoded structured SELECT items as internal `EXPR:` slots plus ordered `select_params`, and threaded those parameters through `compiler/mesh-rt/src/db/repo.rs` so SELECT placeholders are renumbered before WHERE placeholders without exposing `RAW:` to Mesh code. I kept the old pure SQL-builder helper shape for the existing repo tests and added a new select-placeholder-order unit proof.

I wired the new surface through the compiler by extending `compiler/mesh-typeck/src/infer.rs`, `compiler/mesh-codegen/src/mir/lower.rs`, `compiler/mesh-codegen/src/codegen/intrinsics.rs`, and the runtime exports in `compiler/mesh-rt/src/lib.rs`. While exercising the new API from Mesh source, I hit a real parser-facing issue: `Expr.alias(...)` is not callable after `Expr.` because `alias` is tokenized as a keyword in this path. Rather than widen scope into parser surgery, I exposed `Expr.label(...)` as a callable synonym that lowers to the same `mesh_expr_alias` runtime intrinsic and used that in the new e2e proof.

I then extended `compiler/meshc/tests/e2e_m033_s01.rs` with a dedicated `e2e_m033_expr_select_executes` proof covering expression-valued SELECT, aliasing, coalesce/case serialization, and SELECT-vs-WHERE placeholder ordering. While getting that proof green, the temp-project linker exposed another harness-level issue: newly added runtime ABI symbols can link against a stale `target/debug/libmesh_rt.a`. I fixed that by having the file-local `compile_and_run_mesh(...)` helper rebuild `mesh-rt` once before invoking `meshc`, which keeps the compiler e2e contract truthful when S01 adds new runtime exports.

The focused T01 contract is now implemented and green: Mesh code can build neutral expression trees for SELECT, SET, and ON CONFLICT update work without `RAW:` or `Repo.query_raw`, and the expr-specific e2e coverage now proves all three families. The later slice-wide Mesher acceptance checks still surfaced HTTP 429 rate-limiter behavior on `/api/v1/events`, which is outside T01’s focused expression-core work and is recorded below as a known issue rather than as a plan-invalidating blocker.

## Verification

Task-level verification passed with the new focused proofs: `cargo test -p meshc --test e2e_m033_s01 expr_ -- --nocapture` passed with `e2e_m033_expr_select_executes`, `e2e_m033_expr_repo_executes`, and both `expr_error_*` checks green; `cargo test -p mesh-rt db::repo::tests::test_select_expr_sql_renumbers_select_params_before_where_params -- --nocapture` passed to prove SQL placeholder stability in the runtime builder; and `cargo run -q -p meshc -- build mesher` passed after the compiler/runtime wiring changes.

I also ran the broader slice acceptance target `cargo test -p meshc --test e2e_m033_s01 -- --nocapture` to assess slice status under recovery. That broader suite failed only in the later Mesher mutation/upsert acceptance tests because `/api/v1/events` returned HTTP 429 rate-limit responses; the expr-focused proofs and both `expr_error_*` checks inside the same run passed.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m033_s01 expr_ -- --nocapture` | 0 | ✅ pass | 128510ms |
| 2 | `cargo test -p mesh-rt db::repo::tests::test_select_expr_sql_renumbers_select_params_before_where_params -- --nocapture` | 0 | ✅ pass | 1370ms |
| 3 | `cargo run -q -p meshc -- build mesher` | 0 | ✅ pass | 3000ms |
| 4 | `cargo test -p meshc --test e2e_m033_s01 -- --nocapture` | 101 | ❌ fail | 178030ms |


## Deviations

Added a Mesh-callable `Expr.label(...)` synonym that maps to the same runtime alias intrinsic because `Expr.alias(...)` currently collides with keyword parsing in Mesh source. Also updated the `e2e_m033_s01` temp-project compile helper to rebuild `mesh-rt` once before invoking `meshc`, because new runtime ABI symbols can otherwise link against a stale `libmesh_rt.a` during compiler e2e runs.

## Known Issues

The broader slice acceptance target `cargo test -p meshc --test e2e_m033_s01 -- --nocapture` is still red in `e2e_m033_mesher_mutations` and `e2e_m033_mesher_issue_upsert`: both fail because `/api/v1/events` returns HTTP 429 from Mesher’s rate limiter. That drift is outside the focused T01 expression-core contract, but it means the full slice gate and `scripts/verify-m033-s01.sh` were not green at task close.

## Files Created/Modified

- `compiler/mesh-rt/src/db/query.rs`
- `compiler/mesh-rt/src/db/repo.rs`
- `compiler/mesh-rt/src/lib.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/meshc/tests/e2e_m033_s01.rs`
- `.gsd/KNOWLEDGE.md`


## Deviations
Added a Mesh-callable `Expr.label(...)` synonym that maps to the same runtime alias intrinsic because `Expr.alias(...)` currently collides with keyword parsing in Mesh source. Also updated the `e2e_m033_s01` temp-project compile helper to rebuild `mesh-rt` once before invoking `meshc`, because new runtime ABI symbols can otherwise link against a stale `libmesh_rt.a` during compiler e2e runs.

## Known Issues
The broader slice acceptance target `cargo test -p meshc --test e2e_m033_s01 -- --nocapture` is still red in `e2e_m033_mesher_mutations` and `e2e_m033_mesher_issue_upsert`: both fail because `/api/v1/events` returns HTTP 429 from Mesher’s rate limiter. That drift is outside the focused T01 expression-core contract, but it means the full slice gate and `scripts/verify-m033-s01.sh` were not green at task close.
