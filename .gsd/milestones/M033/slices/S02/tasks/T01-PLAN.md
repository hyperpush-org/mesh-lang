---
estimated_steps: 4
estimated_files: 8
skills_used:
  - rust-best-practices
  - postgresql-database-engineering
---

# T01: Add explicit Pg helper plumbing and rewrite the auth path

**Slice:** S02 — Explicit PG extras for JSONB, search, and crypto
**Milestone:** M033

## Description

Extend the existing S01 expression/runtime seam instead of replacing it. This task adds the minimum new mechanics S02 actually needs — cast-capable internals, explicit `Pg.*` constructors, expression-valued query/filter/select support, and expression-valued inserts — then proves the boundary on the smallest real Mesher path by rewriting `create_user` and `authenticate_user`. The public API must stay honest: portable mechanics live under `Expr`, vendor-specific behavior lives under `Pg`.

## Steps

1. Add the internal expression/rendering support needed for PG-only helpers, including cast-capable serialization that can represent shapes like `...::jsonb` and `...::int` without adding a fake universal SQL AST.
2. Add `Pg.*` helper entrypoints plus `Query.select_expr`, `Query.where_expr`, and `Repo.insert_expr`, and make `compiler/mesh-typeck/src/infer.rs`, `compiler/mesh-codegen/src/mir/lower.rs`, `compiler/mesh-codegen/src/codegen/intrinsics.rs`, and `compiler/mesh-rt/src/lib.rs` agree on the same callable surface.
3. Teach the runtime query builder to consume `select_params` in the correct order so expression-valued `SELECT` and `WHERE` clauses compose safely with existing filters and fragments.
4. Rewrite `create_user` and `authenticate_user` in `mesher/storage/queries.mpl` to use the new PG helper surface instead of `Repo.query_raw(...)` and `Query.where_raw(...)` pgcrypto fragments.

## Must-Haves

- [ ] The new public vendor-specific surface is explicit under `Pg`, while the neutral `Expr` API remains portable and does not absorb JSONB/search/pgcrypto names
- [ ] `Query.select_expr`, `Query.where_expr`, and `Repo.insert_expr` are wired end-to-end through runtime, type inference, codegen, and exports
- [ ] `create_user` and `authenticate_user` no longer depend on raw `crypt(...)` SQL fragments

## Verification

- `cargo run -q -p meshc -- build mesher`
- `cargo run -q -p meshc -- fmt --check mesher`

## Observability Impact

- Signals added/changed: compiler/runtime build failures should separate missing intrinsic wiring from placeholder-order drift in expression-valued SELECT/WHERE handling
- How a future agent inspects this: rebuild `mesher`, then inspect the compiler/runtime files and the auth functions in `mesher/storage/queries.mpl`
- Failure state exposed: the first broken vertical slice should surface as `create_user` / `authenticate_user` drift instead of a later route-level failure

## Inputs

- `compiler/mesh-rt/src/db/expr.rs` — existing neutral expression tree and serializer from S01
- `compiler/mesh-rt/src/db/query.rs` — reserved `select_params` slot and current raw-query fallbacks
- `compiler/mesh-rt/src/db/repo.rs` — expression-aware update/upsert plumbing that needs query/select/insert expansion
- `compiler/mesh-rt/src/lib.rs` — top-level runtime symbol exports consumed by compiled Mesh code
- `compiler/mesh-typeck/src/infer.rs` — Mesh-side module signatures for `Expr`, `Query`, and `Repo`
- `compiler/mesh-codegen/src/mir/lower.rs` — intrinsic lookup and builtin lowering table
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — LLVM-visible extern declarations for runtime linkage
- `mesher/storage/queries.mpl` — current auth functions still using raw pgcrypto fragments

## Expected Output

- `compiler/mesh-rt/src/db/expr.rs` — cast-capable expression internals that explicit PG helpers can compose with
- `compiler/mesh-rt/src/db/query.rs` — expression-valued SELECT/WHERE entrypoints and correct `select_params` handling
- `compiler/mesh-rt/src/db/repo.rs` — expression-valued insert support that Mesher PG write families can reuse
- `compiler/mesh-rt/src/lib.rs` — exported runtime symbols for the new PG/query/repo entrypoints
- `compiler/mesh-typeck/src/infer.rs` — Mesh-visible signatures for `Pg.*`, `Query.*_expr`, and `Repo.insert_expr`
- `compiler/mesh-codegen/src/mir/lower.rs` — builtin lowering entries for the new helper calls
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — runtime extern declarations matching the new helper surface
- `mesher/storage/queries.mpl` — auth storage functions rewritten onto explicit PG helpers
