---
estimated_steps: 4
estimated_files: 4
skills_used:
  - postgresql-database-engineering
  - rust-best-practices
---

# T03: Move runtime partition lifecycle into Storage.Schema

Why: S04 owns the remaining runtime partition/catalog keep-sites, and they should collapse onto the new explicit helper family instead of staying split between storage modules.

Steps:
1. Expand `mesher/storage/schema.mpl` so it owns partition create-ahead, expired-partition listing, and quoted drop behavior through the new `Pg.create_daily_partitions_ahead(...)`, `Pg.list_daily_partitions_before(...)`, and `Pg.drop_partition(...)` helpers.
2. Remove `get_expired_partitions(...)` / `drop_partition(...)` from `mesher/storage/queries.mpl`, update `mesher/services/retention.mpl` imports and call sites to use `Storage.Schema`, and keep per-project row deletion logic in `Storage.Queries` untouched.
3. Keep partition naming/date math aligned to PostgreSQL’s clock, not host time, and preserve or improve startup/retention logging in `mesher/main.mpl` / `mesher/services/retention.mpl` so failures localize cleanly.
4. Do not widen the generic query API here; all remaining schema/catalog behavior should stay explicitly PG-shaped.

Must-Haves:
- [ ] `mesher/storage/schema.mpl` becomes the sole Mesher module that owns partition create/list/drop helpers.
- [ ] `mesher/storage/queries.mpl` no longer exports the S04 partition/catalog keep-sites.
- [ ] Mesher startup and retention flows still call real partition lifecycle code on the live runtime path, but without `Repo.query_raw(...)` / `Repo.execute_raw(...)` in the owned functions.

## Inputs

- `compiler/mesh-rt/src/db/pg_schema.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `mesher/storage/schema.mpl`
- `mesher/storage/queries.mpl`
- `mesher/services/retention.mpl`
- `mesher/main.mpl`

## Expected Output

- `mesher/storage/schema.mpl`
- `mesher/storage/queries.mpl`
- `mesher/services/retention.mpl`
- `mesher/main.mpl`

## Verification

cargo run -q -p meshc -- fmt --check mesher
cargo run -q -p meshc -- build mesher
rg -n "pub fn (create_partitions_ahead|get_expired_partitions|drop_partition)" mesher/storage/schema.mpl
! rg -n "pub fn get_expired_partitions|pub fn drop_partition" mesher/storage/queries.mpl

## Observability Impact

- Signals added/changed: Mesher startup and retention cleanup logs should distinguish partition bootstrap/list/drop failures from unrelated cleanup work.
- How a future agent inspects this: read `mesher/storage/schema.mpl`, run `cargo run -q -p meshc -- build mesher`, and use the later S04 verifier or startup logs to confirm helper ownership.
- Failure state exposed: partition helper regressions should be visible in one module (`Storage.Schema`) instead of being split across runtime queries and raw DDL strings.
