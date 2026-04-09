# S04: Schema extras and live partition lifecycle proof — UAT

**Milestone:** M033
**Written:** 2026-03-25T23:59:27.970Z

## UAT Type

- UAT mode: live-runtime
- Why this mode is sufficient: S04's contract is a live Postgres-backed schema/partition proof, so the authoritative acceptance surface is the Mesher-backed Rust harness plus the raw-boundary verifier rather than a manual UI flow.

## Preconditions

- Docker is available locally so `compiler/meshc/tests/e2e_m033_s04.rs` can start its temporary `postgres:16` container.
- The repo builds from the working tree with `cargo` available.
- The relevant Postgres environment is available to the command runner (in this checkout the verification commands source `.env` before running).
- `compiler/meshc/tests/e2e_m033_s04.rs`, `scripts/verify-m033-s04.sh`, and `scripts/verify-m033-s03.sh` are present in the workspace.

## Smoke Test

1. Run `bash scripts/verify-m033-s04.sh`.
2. **Expected:** the script prints `verify-m033-s04: ok` after rerunning the live S04 test target plus Mesher fmt/build and the raw-boundary/log sweep.

## Test Cases

### 1. Migration helpers render the intended PostgreSQL catalog state

1. Run `cargo test -p meshc --test e2e_m033_s04 e2e_m033_s04_migrations_render_pg_catalog_state -- --nocapture`.
2. **Expected:** the helper-driven Mesher migration applies successfully on a fresh Postgres container.
3. **Expected:** `pg_extension` contains `pgcrypto`.
4. **Expected:** `events` is still the partitioned parent in `pg_partitioned_table`, partitioned on `received_at`, and `to_regclass('public.events')` resolves.
5. **Expected:** `idx_events_tags` remains a `GIN` index using the `jsonb_path_ops` opclass.

### 2. `Storage.Schema` owns runtime partition create/list/drop behavior

1. Run `cargo test -p meshc --test e2e_m033_s04 e2e_m033_s04_storage_schema_helpers_manage_runtime_partitions -- --nocapture`.
2. **Expected:** `create_partitions_ahead(pool, 3)` succeeds and materializes exactly three current/future daily partitions based on the database clock.
3. **Expected:** `get_expired_partitions(pool, 90)` returns the one seeded expired partition name.
4. **Expected:** `drop_partition(pool, first)` drops that partition successfully.
5. **Expected:** the dropped partition disappears from both `to_regclass(...)` and `pg_inherits`.

### 3. Real Mesher startup bootstraps partitions and logs safely

1. Run `cargo test -p meshc --test e2e_m033_s04 e2e_m033_s04_mesher_startup_bootstraps_partitions_and_logs -- --nocapture`.
2. **Expected:** the Mesher binary reaches ready state and the settings endpoint returns numeric `retention_days` and `sample_rate` values.
3. **Expected:** startup creates seven daily partitions ahead and the furthest `events_YYYYMMDD` partition exists for the +6 day window.
4. **Expected:** the captured logs contain `[Mesher] Connecting to PostgreSQL...`, `[Mesher] Partition bootstrap succeeded (7 days ahead)`, and the HTTP port banner.
5. **Expected:** the captured logs do **not** contain `postgres://` or `DATABASE_URL`.

### 4. The acceptance verifier keeps the raw-boundary honest

1. Run `bash scripts/verify-m033-s04.sh`.
2. Run `bash scripts/verify-m033-s03.sh`.
3. **Expected:** the S04 verifier passes and its Python sweep confirms that the initial migration and runtime partition files do not contain `Pool.execute`, `Migration.execute`, `Repo.query_raw`, `Repo.execute_raw`, or `Query.select_raw` in the owned surfaces.
4. **Expected:** the S03 verifier also passes and no longer exempts `get_expired_partitions` or `drop_partition` in `mesher/storage/queries.mpl`.

## Edge Cases

### Composite partitioned-parent constraints stay helper-driven

1. Run `cargo test -p meshc --test e2e_m033_s04 e2e_m033_s04_migrations_render_pg_catalog_state -- --nocapture`.
2. **Expected:** the helper-driven `events` parent migration still succeeds even though it includes the table-level `PRIMARY KEY (id, received_at)` constraint.

### Startup and retention diagnostics stay localized and non-secret-bearing

1. Run `bash scripts/verify-m033-s04.sh`.
2. **Expected:** the verifier finds the expected startup and retention log strings (`Partition bootstrap succeeded`, `Partition bootstrap failed`, `Retention partition listing failed`, `Retention partition drop failed`) and does not accept a drift back to secret-bearing or stage-opaque logging.

## Failure Signals

- Any non-zero exit from `cargo test -p meshc --test e2e_m033_s04 -- --nocapture`, `cargo run -q -p meshc -- fmt --check mesher`, `cargo run -q -p meshc -- build mesher`, `bash scripts/verify-m033-s04.sh`, or `bash scripts/verify-m033-s03.sh`.
- A failing named `e2e_m033_s04_*` family in the test output.
- Missing `pgcrypto`, wrong partition column, missing `GIN` / `jsonb_path_ops`, missing future partitions, or a dropped partition that still appears in `to_regclass(...)` / `pg_inherits`.
- Verifier output naming a raw-boundary token or missing expected helper/log snippet in the migration/runtime partition files.
- Startup logs that print `postgres://` or `DATABASE_URL`.

## Requirements Proved By This UAT

- R036 — proves the neutral migration surface stayed honest while PostgreSQL-only schema behavior remained explicit under `Pg.*`.
- R037 — proves the missing partition-related PostgreSQL migration/runtime helpers on the real Mesher path.
- R039 — proves the recurring Mesher schema and partition-management families moved off the slice-owned raw DDL/query sites and onto helper surfaces.
- R038 — advances the final honest raw-tail-collapse contract by mechanically enforcing the S04-owned migration/runtime boundary.

## Not Proven By This UAT

- S05's public docs closeout.
- The full integrated M033 acceptance replay across S02+S03+S04 together.
- SQLite-specific runtime behavior for future vendor extras.

## Notes for Tester

The Rust harness is self-contained: it starts its own Postgres container, runs the Mesher migration, builds/spawns Mesher when needed, and names the drifting proof family. If the slice regresses, start with the first failing `e2e_m033_s04_*` test or the verifier's Python raw-boundary error before broadening the helper surface or reintroducing raw SQL.
