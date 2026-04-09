---
id: T04
parent: S04
milestone: M033
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/e2e_m033_s04.rs", "scripts/verify-m033-s04.sh", "scripts/verify-m033-s03.sh"]
key_decisions: ["Make the S04 verifier mechanically police both the initial migration and the runtime partition modules for raw-boundary regressions, so failures name the offending helper/token directly.", "Remove the S03 verifier’s temporary S04 keep-site exemption so reintroduced partition raw sites in `mesher/storage/queries.mpl` become immediate regressions."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the authoritative S04 acceptance commands from the slice plan: `cargo test -p meshc --test e2e_m033_s04 -- --nocapture` passed with the named `e2e_m033_s04_*` proofs for migration-time catalog state, runtime partition helper lifecycle, and real Mesher startup bootstrap/logging; `cargo run -q -p meshc -- fmt --check mesher` passed; `cargo run -q -p meshc -- build mesher` passed; and `bash scripts/verify-m033-s04.sh` passed with the tighter migration/runtime raw-boundary sweep. I also ran `bash scripts/verify-m033-s03.sh` to verify the old S03 keep-list no longer masks S04 partition helpers and still passes cleanly."
completed_at: 2026-03-25T23:37:32.438Z
blocker_discovered: false
---

# T04: Tightened S04 live Postgres proofs and verifier sweeps for migration/runtime partition boundaries

> Tightened S04 live Postgres proofs and verifier sweeps for migration/runtime partition boundaries

## What Happened
---
id: T04
parent: S04
milestone: M033
key_files:
  - compiler/meshc/tests/e2e_m033_s04.rs
  - scripts/verify-m033-s04.sh
  - scripts/verify-m033-s03.sh
key_decisions:
  - Make the S04 verifier mechanically police both the initial migration and the runtime partition modules for raw-boundary regressions, so failures name the offending helper/token directly.
  - Remove the S03 verifier’s temporary S04 keep-site exemption so reintroduced partition raw sites in `mesher/storage/queries.mpl` become immediate regressions.
duration: ""
verification_result: passed
completed_at: 2026-03-25T23:37:32.439Z
blocker_discovered: false
---

# T04: Tightened S04 live Postgres proofs and verifier sweeps for migration/runtime partition boundaries

**Tightened S04 live Postgres proofs and verifier sweeps for migration/runtime partition boundaries**

## What Happened

Validated local reality first: the live S04 proof target and verifier already existed in this checkout, so I adapted the task from creating them to tightening them where the contract was still incomplete. In `compiler/meshc/tests/e2e_m033_s04.rs`, I added an explicit `pg_inherits` assertion to the runtime partition cleanup proof so `Storage.Schema.drop_partition(...)` is now proven to remove expired partitions from the inheritance catalog as well as from `to_regclass(...)`. In `scripts/verify-m033-s04.sh`, I expanded the mechanical sweep to inspect `mesher/migrations/20260216120000_create_initial_schema.mpl` plus the runtime partition files and fail on raw-boundary regressions (`Pool.execute`, `Migration.execute`, `Repo.query_raw`, `Repo.execute_raw`, `Query.select_raw`) while still requiring the expected `Pg.*` and `Storage.Schema` helper call sites and startup/retention log strings. In `scripts/verify-m033-s03.sh`, I removed the old S04 exemption for `get_expired_partitions` and `drop_partition`, so any reintroduced raw keep-site in `mesher/storage/queries.mpl` now surfaces as an unexpected regression instead of being silently tolerated. After those changes, I reran the live Postgres proof target and both verifier scripts; all checks passed.

## Verification

Verified the authoritative S04 acceptance commands from the slice plan: `cargo test -p meshc --test e2e_m033_s04 -- --nocapture` passed with the named `e2e_m033_s04_*` proofs for migration-time catalog state, runtime partition helper lifecycle, and real Mesher startup bootstrap/logging; `cargo run -q -p meshc -- fmt --check mesher` passed; `cargo run -q -p meshc -- build mesher` passed; and `bash scripts/verify-m033-s04.sh` passed with the tighter migration/runtime raw-boundary sweep. I also ran `bash scripts/verify-m033-s03.sh` to verify the old S03 keep-list no longer masks S04 partition helpers and still passes cleanly.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m033_s04 -- --nocapture` | 0 | ✅ pass | 59154ms |
| 2 | `cargo run -q -p meshc -- fmt --check mesher` | 0 | ✅ pass | 7518ms |
| 3 | `cargo run -q -p meshc -- build mesher` | 0 | ✅ pass | 15554ms |
| 4 | `bash scripts/verify-m033-s04.sh` | 0 | ✅ pass | 80454ms |
| 5 | `bash scripts/verify-m033-s03.sh` | 0 | ✅ pass | 176204ms |


## Deviations

The checkout already contained `compiler/meshc/tests/e2e_m033_s04.rs` and `scripts/verify-m033-s04.sh`, so instead of creating those artifacts from scratch I tightened them to satisfy the remaining T04 contract gaps. Otherwise none.

## Known Issues

None.

## Files Created/Modified

- `compiler/meshc/tests/e2e_m033_s04.rs`
- `scripts/verify-m033-s04.sh`
- `scripts/verify-m033-s03.sh`


## Deviations
The checkout already contained `compiler/meshc/tests/e2e_m033_s04.rs` and `scripts/verify-m033-s04.sh`, so instead of creating those artifacts from scratch I tightened them to satisfy the remaining T04 contract gaps. Otherwise none.

## Known Issues
None.
