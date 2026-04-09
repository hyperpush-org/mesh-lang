---
estimated_steps: 10
estimated_files: 9
skills_used: []
---

# T01: Added Query.select_exprs with compiler/runtime wiring and expr select e2e coverage

Land the neutral expression core end-to-end before touching Mesher storage code. This task adds the dedicated expression builder, the Query/Repo entrypoints needed for expression-valued `SELECT` / `SET` / `ON CONFLICT` work, the compiler/runtime wiring that makes those calls legal from Mesh code, and the first permanent `meshc` e2e proofs in `compiler/meshc/tests/e2e_m033_s01.rs`. The contract must stay portable: no JSONB, pgcrypto, search, or catalog-specific helpers belong in this layer.

Steps
1. Add a dedicated expression-builder surface under the runtime DB layer and expose only the portable nodes S01 needs: column refs, literal/parameter values, `NULL`, function calls, arithmetic/comparison, `CASE`, and `COALESCE`, plus the neutral conflict-update reference the upsert path will need later.
2. Extend `Query` / `Repo` so Mesh code can use those expression nodes for expression-valued `SELECT`, `SET`, and `ON CONFLICT` work without routing through `RAW:` strings.
3. Wire the new surface through `compiler/mesh-typeck/src/infer.rs`, `compiler/mesh-codegen/src/mir/lower.rs`, `compiler/mesh-codegen/src/codegen/intrinsics.rs`, and the runtime exports so the Mesh-side API is fully callable.
4. Add named `e2e_m033_expr_*` coverage in `compiler/meshc/tests/e2e_m033_s01.rs` that proves the contract compiles, executes, and keeps placeholder ordering / serializer output stable enough for later Mesher rewrites.

Must-Haves
- [ ] Mesh code can build neutral expression trees and pass them through Query/Repo without `RAW:` or `Repo.query_raw`
- [ ] `compiler/meshc/tests/e2e_m033_s01.rs` contains passing `e2e_m033_expr_*` proofs for expression-valued `SELECT`, `SET`, and conflict-update work
- [ ] The new core excludes PG-only JSONB/search/crypto helpers so the later vendor-specific slices still have an explicit seam

## Inputs

- `compiler/mesh-rt/src/db/query.rs`
- `compiler/mesh-rt/src/db/repo.rs`
- `compiler/mesh-rt/src/db/orm.rs`
- `compiler/mesh-rt/src/db/mod.rs`
- `compiler/mesh-rt/src/lib.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/meshc/tests/e2e.rs`

## Expected Output

- `compiler/mesh-rt/src/db/expr.rs`
- `compiler/mesh-rt/src/db/query.rs`
- `compiler/mesh-rt/src/db/repo.rs`
- `compiler/mesh-rt/src/db/mod.rs`
- `compiler/mesh-rt/src/lib.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/meshc/tests/e2e_m033_s01.rs`

## Verification

`cargo test -p meshc --test e2e_m033_s01 expr_ -- --nocapture`
`cargo run -q -p meshc -- build mesher`

## Observability Impact

- Signals added/changed: named `e2e_m033_expr_*` failures that distinguish serializer drift, placeholder-order drift, and unsupported expression-node bugs
- How a future agent inspects this: rerun the `expr_` filter in `compiler/meshc/tests/e2e_m033_s01.rs` and inspect the failing assertion/output tied to the exact expression family
- Failure state exposed: the first contract drift surfaces as a specific expression-proof name instead of a later Mesher route failure
