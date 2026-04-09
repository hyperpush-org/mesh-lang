---
estimated_steps: 3
estimated_files: 3
skills_used:
  - postgresql-database-engineering
  - rust-best-practices
---

# T02: Rewrite the initial Mesher migration onto the new helper boundary

Why: The densest S04-owned raw DDL cluster is the initial migration, so it should move first once the helper seam exists.

Steps:
1. Rewrite `mesher/migrations/20260216120000_create_initial_schema.mpl` so ordinary tables use `Migration.create_table(...)`, ordinary/partial/ordered indexes use the upgraded `Migration.create_index(...)` with exact `name:` options, and only the truly PG-only sites use the new `Pg` schema helpers.
2. Replace the raw PG-only families in that migration with explicit calls to `Pg.create_extension(...)`, `Pg.create_range_partitioned_table(...)`, and `Pg.create_gin_index(...)`, preserving the existing table/index names and predicates instead of introducing migration-name drift.
3. Update migration scaffolding/examples and compile-only coverage so new generated migrations teach the explicit PG helper path instead of `Migration.execute(...)` for schema extras.

Must-Haves:
- [ ] `mesher/migrations/20260216120000_create_initial_schema.mpl` no longer uses raw `Pool.execute(...)` for the S04-owned extension / partitioned-table / recurring-index families.
- [ ] Exact Mesher schema names (`idx_projects_slug`, `idx_issues_project_last_seen`, `idx_events_tags`, etc.) stay stable after the rewrite.
- [ ] `compiler/meshc/src/migrate.rs` and `compiler/meshc/tests/e2e.rs` stop teaching raw `Migration.execute(...)` as the default extension/schema-extra path.

## Inputs

- `compiler/mesh-rt/src/db/migration.rs`
- `compiler/mesh-rt/src/db/pg_schema.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `mesher/migrations/20260216120000_create_initial_schema.mpl`
- `compiler/meshc/src/migrate.rs`
- `compiler/meshc/tests/e2e.rs`

## Expected Output

- `mesher/migrations/20260216120000_create_initial_schema.mpl`
- `compiler/meshc/src/migrate.rs`
- `compiler/meshc/tests/e2e.rs`

## Verification

cargo test -p meshc --test e2e e2e_migration -- --nocapture
cargo run -q -p meshc -- build mesher
! rg -n "Pool\.execute\(pool" mesher/migrations/20260216120000_create_initial_schema.mpl
