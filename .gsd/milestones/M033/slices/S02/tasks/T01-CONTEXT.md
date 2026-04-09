# T01 resume context

## Status
- Implementation not started.
- No source files were modified.
- No verification commands were run.
- No blocker was discovered; this pause was due to context budget only.

## What was confirmed

### Runtime/expression layer
- `compiler/mesh-rt/src/db/expr.rs` already has the neutral `SqlExpr` tree and serialization used by S01.
- It does **not** yet have cast-capable serialization. A new expression node is needed for shapes like `...::jsonb` / `...::int`.
- Existing neutral surface already supports generic calls (`Expr.call` / `Expr.fn_call`) and aliases, so the vendor-specific public API can be added as thin `Pg.*` constructors over the expression internals without polluting `Expr`.

### Query / repo layer
- `compiler/mesh-rt/src/db/query.rs` already has slot 13 `select_params` and existing `mesh_query_select_exprs` support.
- `compiler/mesh-rt/src/db/repo.rs` already consumes `select_params` in `build_select_sql_from_parts_with_select_params(...)` and has a unit test named `test_select_expr_sql_renumbers_select_params_before_where_params` proving SELECT params are ordered before WHERE params.
- `repo.rs` already has expression-valued update/upsert helpers (`build_update_where_expr_sql_pure`, `build_insert_or_update_expr_sql_pure`, `mesh_repo_update_where_expr`, `mesh_repo_insert_or_update_expr`).
- There is **no** expression-valued insert helper yet (`Repo.insert_expr`).
- There is **no** query-side `where_expr` helper yet.
- Public task plan asks for `Query.select_expr`, but local reality currently exposes plural runtime/type names (`select_exprs`). Next executor should decide whether to:
  - add singular public surface as an alias over existing plural runtime plumbing, or
  - add a new singular runtime symbol and keep plural for compatibility.
  The task contract favors an explicit `select_expr` public API.

### Compiler/type/lowering registration sites that must agree
- Module-qualified surfaces are declared in `compiler/mesh-typeck/src/infer.rs` inside `stdlib_modules()`.
- Bare builtin env names are mirrored in `compiler/mesh-typeck/src/builtins.rs`.
- MIR known runtime symbol types are registered in `compiler/mesh-codegen/src/mir/lower.rs`.
- Builtin-to-runtime symbol mapping is in `map_builtin_name(...)` in `compiler/mesh-codegen/src/mir/lower.rs`.
- LLVM extern declarations are in `compiler/mesh-codegen/src/codegen/intrinsics.rs`.
- Runtime exports are re-exported from `compiler/mesh-rt/src/lib.rs`.

### Important lowering detail
- Module-qualified calls like `Pg.foo(...)`, `Query.foo(...)`, `Repo.foo(...)` are lowered by `lower_field_access(...)` in `compiler/mesh-codegen/src/mir/lower.rs` using:
  - lowercase module prefix + method name (for example `Query.from` -> `query_from`), then
  - `map_builtin_name(...)` to resolve the runtime symbol.
- Because of that, any new `Pg.*`, `Query.*`, or `Repo.*` method must be added consistently to:
  - `infer.rs` module table,
  - `builtins.rs` bare env names (recommended for consistency with existing pattern),
  - `lower.rs` known function signatures,
  - `lower.rs` `map_builtin_name(...)`,
  - `intrinsics.rs`,
  - `mesh-rt/src/lib.rs` exports.

## Current Mesher auth path
- `mesher/storage/queries.mpl` currently uses raw pgcrypto SQL in exactly the places T01 targets:
  - `create_user` uses `Repo.query_raw(pool, "SELECT crypt($1, gen_salt('bf', 12)) AS hash", [password])` and then `Repo.insert(...)`.
  - `authenticate_user` uses `Query.where_raw("password_hash = crypt(?, password_hash)", [password])`.
- This is the smallest real runtime slice to move first.

## Likely implementation plan for the next executor
1. `expr.rs`
   - Add a cast node to `SqlExpr` plus serializer support.
   - Add vendor-specific Pg expression constructors (most likely at least `Pg.cast`, `Pg.crypt`, `Pg.gen_salt`).
   - Keep JSONB/search/pgcrypto names out of `Expr`.
2. `query.rs`
   - Add `mesh_query_where_expr(...)` that serializes an expression to SQL, appends it as a raw WHERE fragment, and appends the resulting params in order.
   - Decide how to present `Query.select_expr` publicly given current `mesh_query_select_exprs(...)` internals.
3. `repo.rs`
   - Add `mesh_repo_insert_expr(...)` using a `Map<String, Ptr>` and expression serialization for VALUES.
   - Reuse the existing parameter renumbering discipline from update/upsert helpers.
   - Add focused unit tests for insert-expression parameter ordering.
4. Compiler plumbing
   - Wire all new names through `infer.rs`, `builtins.rs`, `lower.rs`, `intrinsics.rs`, and `mesh-rt/src/lib.rs`.
5. `mesher/storage/queries.mpl`
   - Rewrite `create_user` to compute `password_hash` via `Repo.insert_expr(...)` using explicit `Pg.*` helpers instead of `Repo.query_raw(...)`.
   - Rewrite `authenticate_user` to use `Query.where_expr(...)` with explicit `Pg.crypt(...)` instead of `Query.where_raw(...)`.

## Naming notes to resolve before coding
- The task plan explicitly says the public API should be under `Pg`, `Query.select_expr`, `Query.where_expr`, and `Repo.insert_expr`.
- Local code currently uses `select_exprs` internally, so the next executor should prefer **public singular naming** and adapt runtime naming as needed, rather than preserving the plural by inertia.

## Tests already present that are useful anchors
- `compiler/mesh-rt/src/repo.rs`
  - `test_select_expr_sql_renumbers_select_params_before_where_params`
  - `test_update_where_expr_sql_renumbers_set_and_where_params`
  - `test_insert_or_update_expr_sql_preserves_insert_then_update_param_order`
- Extend this pattern instead of inventing a separate testing style.

## Resume point
Start directly at implementation. No further broad repo mapping should be needed before editing the files listed in the task plan.