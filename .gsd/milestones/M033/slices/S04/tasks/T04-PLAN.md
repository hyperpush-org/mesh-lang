---
estimated_steps: 4
estimated_files: 4
skills_used:
  - test
  - postgresql-database-engineering
  - rust-best-practices
---

# T04: Prove schema extras and partition lifecycle on live Postgres

Why: S04 is only complete once the new helper boundary is proven against real Postgres catalogs and the real Mesher startup path, not just string snapshots.

Steps:
1. Add `compiler/meshc/tests/e2e_m033_s04.rs`, reusing the S02/S03 Docker/Postgres and Mesher-spawn patterns, with named proofs for migration-time schema extras, startup partition creation from `mesher/main.mpl`, and runtime expired-partition list/drop behavior through the real storage helpers.
2. Assert catalog truth directly: `pg_extension` contains `pgcrypto`, `events` is a partitioned table in `pg_partitioned_table` / `pg_inherits`, the `tags` index uses `GIN` with `jsonb_path_ops`, startup-created future partitions exist, and dropped partitions disappear from `to_regclass(...)` / inheritance catalogs.
3. Add `scripts/verify-m033-s04.sh` to run the full S04 suite, Mesher fmt/build, and a mechanical sweep that bans S04 raw DDL/query regressions in the migration and runtime partition files.
4. Update `scripts/verify-m033-s03.sh` so the old verifier no longer silently excludes S04 partition/catalog keep-sites.

Must-Haves:
- [ ] `compiler/meshc/tests/e2e_m033_s04.rs` proves migration apply, startup partition creation, and list/drop cleanup on live Postgres with named `e2e_m033_s04_*` tests.
- [ ] `scripts/verify-m033-s04.sh` is the stable slice-level acceptance command and names the offending proof family or raw keep-site when it fails.
- [ ] `scripts/verify-m033-s03.sh` stops exempting the S04 partition/catalog helpers once the new verifier exists.

## Inputs

- `mesher/migrations/20260216120000_create_initial_schema.mpl`
- `mesher/storage/schema.mpl`
- `mesher/storage/queries.mpl`
- `mesher/services/retention.mpl`
- `mesher/main.mpl`
- `compiler/meshc/tests/e2e_m033_s02.rs`
- `compiler/meshc/tests/e2e_m033_s03.rs`
- `scripts/verify-m033-s03.sh`

## Expected Output

- `compiler/meshc/tests/e2e_m033_s04.rs`
- `scripts/verify-m033-s04.sh`
- `scripts/verify-m033-s03.sh`

## Verification

cargo test -p meshc --test e2e_m033_s04 -- --nocapture
cargo run -q -p meshc -- fmt --check mesher
cargo run -q -p meshc -- build mesher
bash scripts/verify-m033-s04.sh

## Observability Impact

- Signals added/changed: named `e2e_m033_s04_*` failures and verifier sweep errors should distinguish migration-extra drift, startup partition bootstrap drift, and runtime cleanup drift.
- How a future agent inspects this: run `bash scripts/verify-m033-s04.sh`, inspect the failing Rust test name, then query the same catalogs the harness uses.
- Failure state exposed: catalog mismatches, missing future partitions, or raw-keep-site regressions should be explicit without manual SQL archaeology.
