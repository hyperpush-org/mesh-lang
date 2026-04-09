---
id: S04
parent: M033
milestone: M033
provides:
  - A live Postgres proof bundle for helper-driven migration extras, runtime partition lifecycle, and Mesher startup partition bootstrap/logging.
  - A rewritten Mesher initial migration and runtime retention/startup partition path that now run through first-class `Migration.*`, `Pg.*`, and `Storage.Schema` helpers instead of the slice-owned raw DDL/query sites.
  - Stable acceptance commands (`compiler/meshc/tests/e2e_m033_s04.rs`, `bash scripts/verify-m033-s04.sh`) and a tightened S03 verifier boundary that downstream slices can treat as the authoritative schema/partition contract.
requires:
  - slice: S01
    provides: the neutral expression/rendering/building contract and the honest baseline rule that only truly portable behavior belongs on the neutral `Migration` / `Query` / `Repo` surface.
  - slice: S02
    provides: the explicit `Pg` namespacing pattern and the established rule that PostgreSQL-only data-layer behavior should stay explicit rather than leak back into the neutral API.
affects:
  - S05
key_files:
  - compiler/mesh-rt/src/db/migration.rs
  - compiler/mesh-rt/src/db/pg_schema.rs
  - mesher/migrations/20260216120000_create_initial_schema.mpl
  - mesher/storage/schema.mpl
  - mesher/services/retention.mpl
  - mesher/main.mpl
  - compiler/meshc/tests/e2e_m033_s04.rs
  - scripts/verify-m033-s04.sh
  - scripts/verify-m033-s03.sh
  - .gsd/REQUIREMENTS.md
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
  - .gsd/PROJECT.md
key_decisions:
  - Keep `Migration.create_index(...)` strictly neutral: exact names, unique flag, trailing partial predicate, and per-column `ASC`/`DESC` stay on the baseline path, while PostgreSQL-only index method/opclass behavior remains explicit under `Pg.*`.
  - Keep runtime partition create/list/drop ownership in `mesher/storage/schema.mpl` as thin wrappers over explicit `Pg.*` helpers instead of widening `Storage.Queries` or the generic query API.
  - Allow `Pg.create_range_partitioned_table(...)` to accept table-level constraint entries so helper-driven partitioned parents can keep composite declarations like `PRIMARY KEY (id, received_at)` without falling back to raw DDL.
  - Use `compiler/meshc/tests/e2e_m033_s04.rs` plus `scripts/verify-m033-s04.sh` as the authoritative S04 acceptance surface, and remove the old S03 partition/catalog exemption so regressions fail mechanically.
patterns_established:
  - Keep the neutral data-layer surface small and honest, and route PostgreSQL-only schema behavior through explicit `Pg.*` helpers instead of smuggling it through a fake portable API.
  - Treat runtime schema/catalog lifecycle as `Storage.Schema` work, not generic query work, so startup and retention code consume one authoritative helper layer.
  - Prove schema-helper work against live PostgreSQL catalogs (`pg_extension`, `pg_partitioned_table`, `pg_inherits`, `pg_indexes` / `pg_am` / `pg_opclass`, and `to_regclass(...)`) rather than trusting SQL-string snapshots alone.
  - Use verifier scripts that ban raw-boundary tokens and assert expected helper/log surfaces so future regressions fail mechanically instead of becoming folklore.
  - When a helper-driven migration still needs table-level constraints on a partitioned parent, extend the helper deliberately rather than backing out to raw DDL.
observability_surfaces:
  - `compiler/meshc/tests/e2e_m033_s04.rs` with named `e2e_m033_s04_migrations_render_pg_catalog_state`, `e2e_m033_s04_storage_schema_helpers_manage_runtime_partitions`, and `e2e_m033_s04_mesher_startup_bootstraps_partitions_and_logs` failures that isolate catalog drift vs runtime helper drift vs startup/bootstrap/logging drift.
  - Mesher startup and retention log strings in `mesher/main.mpl` and `mesher/services/retention.mpl`, especially `Partition bootstrap succeeded (7 days ahead)`, `Partition bootstrap failed: ...`, `Retention partition listing failed ...`, and `Retention partition drop failed ...`.
  - `scripts/verify-m033-s04.sh` and its Python raw-boundary sweep, which names the offending token/file when migration/runtime partition work drifts back to raw SQL or loses the expected helper/log surfaces.
  - `scripts/verify-m033-s03.sh`, which now treats reintroduced partition/catalog raw sites in `mesher/storage/queries.mpl` as immediate regressions instead of silently exempting them.
drill_down_paths:
  - .gsd/milestones/M033/slices/S04/tasks/T01-SUMMARY.md
  - .gsd/milestones/M033/slices/S04/tasks/T02-SUMMARY.md
  - .gsd/milestones/M033/slices/S04/tasks/T03-SUMMARY.md
  - .gsd/milestones/M033/slices/S04/tasks/T04-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-25T23:59:27.969Z
blocker_discovered: false
---

# S04: Schema extras and live partition lifecycle proof

**Shipped helper-driven Mesher schema extras and live partition lifecycle proof, replacing the slice-owned raw DDL/query sites with honest neutral `Migration.*` growth, explicit `Pg.*` helpers, and catalog-backed acceptance tests.**

## What Happened

S04 finished the schema/DDL side of M033 by extending the shared Mesh runtime/compiler boundary first, then collapsing Mesher's owned raw schema and partition sites onto those helpers, and finally proving the result on live Postgres. The runtime side now exposes honest neutral migration index support (`Migration.create_index(...)` with exact names, ordered columns, and partial predicates only) plus explicit PostgreSQL schema helpers for `pgcrypto`, range-partitioned parent tables, GIN/opclass indexes, and database-clock-driven partition create/list/drop work. Those helpers were wired through mesh-rt, type checking, MIR lowering, LLVM intrinsics, and REPL/JIT resolution.

With that boundary in place, Mesher's initial migration was rewritten to use `Migration.*` for the truthful portable cases and `Pg.*` only for the genuinely PostgreSQL-specific families. The remaining runtime partition lifecycle moved out of `mesher/storage/queries.mpl` into `mesher/storage/schema.mpl`, and the retention/startup code now consumes that one authoritative schema helper layer. Startup logs now report partition bootstrap success/failure explicitly, and retention cleanup logs partition-list and partition-drop failures separately so the failing stage is obvious.

The live proof work then closed the slice for real. `compiler/meshc/tests/e2e_m033_s04.rs` now proves, against a real Postgres container, that the helper-driven migration installs `pgcrypto`, keeps `events` partitioned by `received_at`, preserves the `idx_events_tags` GIN/jsonb_path_ops index, lets `Storage.Schema` create/list/drop daily partitions correctly, removes dropped partitions from both `to_regclass(...)` and `pg_inherits`, and bootstraps seven future partitions when the real Mesher binary starts. The first live run exposed a real gap: `Pg.create_range_partitioned_table(...)` still rejected the helper-driven `PRIMARY KEY (id, received_at)` table constraint. S04 fixed that root cause in the runtime helper instead of backing out to raw DDL.

The slice closed by hardening the verifier story. `scripts/verify-m033-s04.sh` is now the stable acceptance command: it reruns the live S04 test target, Mesher fmt/build, and a Python raw-boundary sweep that bans raw DDL/query tokens in the owned migration/runtime files while also checking for the expected helper calls and operational log strings. `scripts/verify-m033-s03.sh` no longer exempts the old S04 partition/catalog helpers, so the short named raw keep-list is now truthful across both read-side and schema-side boundaries.

## Verification

Ran every slice-level acceptance command from the plan and all passed from the working tree. `cargo test -p meshc --test e2e_m033_s04 -- --nocapture` passed 3/3 named live Postgres proofs, covering migration catalog state, runtime `Storage.Schema` partition create/list/drop behavior, and real Mesher startup partition bootstrap/logging. `cargo run -q -p meshc -- fmt --check mesher` passed. `cargo run -q -p meshc -- build mesher` passed. `bash scripts/verify-m033-s04.sh` passed and reran the S04 test target plus fmt/build and a Python raw-boundary/observability sweep. Across the task-level evidence used to assemble the slice, `bash scripts/verify-m033-s03.sh` also passed after its temporary S04 exemption was removed. Observability surfaces were confirmed through the named `e2e_m033_s04_*` failures, catalog assertions against `pg_extension` / `pg_partitioned_table` / `pg_inherits` / `pg_indexes` / `pg_am` / `pg_opclass` / `to_regclass(...)`, and Mesher startup/retention log string checks that also verified secret-bearing DSNs and `DATABASE_URL` never appear in logs.

## Requirements Advanced

- R038 — Closed the DDL-side gap behind the honest raw-tail-collapse goal by moving the S04-owned migration/runtime partition sites onto helper surfaces and enforcing that boundary with `scripts/verify-m033-s03.sh` plus `scripts/verify-m033-s04.sh`, while leaving only the integrated S05 replay to validate the full milestone keep-list end to end.
- R040 — Strengthened the future SQLite seam by keeping S04's only neutral migration growth to honest index name/order support and forcing extension/index/partition behavior to remain explicit under `Pg.*`, but still without claiming SQLite runtime proof.

## Requirements Validated

- R036 — Validated by the assembled M033 neutral-plus-explicit-extra proof path, culminating in S04's live Postgres catalog/startup verification through `cargo test -p meshc --test e2e_m033_s04 -- --nocapture` and `bash scripts/verify-m033-s04.sh` on top of the earlier S01/S02 neutral/PG helper proofs.
- R037 — Validated by the combined S02+S04 PG-extra proof set: `cargo test -p meshc --test e2e_m033_s02 -- --nocapture`, `cargo test -p meshc --test e2e_m033_s04 -- --nocapture`, `bash scripts/verify-m033-s02.sh`, and `bash scripts/verify-m033-s04.sh` now cover JSONB/search/crypto plus partition-related migration/runtime helpers on the real Mesher path.
- R039 — Validated directly by `cargo test -p meshc --test e2e_m033_s04 -- --nocapture`, `cargo run -q -p meshc -- fmt --check mesher`, `cargo run -q -p meshc -- build mesher`, and `bash scripts/verify-m033-s04.sh`, which together prove helper-driven migration apply, runtime partition lifecycle, and the absence of the old raw DDL/query ownership sites.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

The checkout did not yet have the planned S04 closeout fully wired: the first live gate exposed a pre-existing `Pg.create_range_partitioned_table(...)` limitation around table-level constraints, and the slice had to finish/tighten the dedicated S04 acceptance artifacts (`compiler/meshc/tests/e2e_m033_s04.rs` and `scripts/verify-m033-s04.sh`) while fixing that runtime helper regression. Otherwise the work stayed within the written slice goal.

## Known Limitations

SQLite-specific extras remain a later design seam rather than a runtime-proven surface, and S05 still has to publish the neutral-vs-PG contract and rerun the full integrated M033 acceptance replay. Catalog inspection and truly dynamic DDL can still remain explicit escape hatches where a dedicated helper would be dishonest or overly specific.

## Follow-ups

S05 should use `compiler/meshc/tests/e2e_m033_s04.rs` plus `bash scripts/verify-m033-s04.sh` as the canonical schema/partition proof surface when documenting the neutral-vs-PG boundary and replaying the full Mesher data-layer acceptance suite. S05 should also keep the new S03/S04 verifier split intact rather than broadening the raw keep-list or reintroducing partition/catalog exemptions.

## Files Created/Modified

- `compiler/mesh-rt/src/db/migration.rs` — Extended the neutral migration index builder to preserve exact names, ordered columns, and partial predicates while rejecting PostgreSQL-only index syntax.
- `compiler/mesh-rt/src/db/pg_schema.rs` — Added explicit PostgreSQL schema helpers for extensions, partitioned parents, GIN/opclass indexes, and runtime partition lifecycle, and taught partitioned-parent rendering to accept table-level constraints.
- `compiler/mesh-rt/src/db/mod.rs` — Exported the new pg_schema runtime module.
- `compiler/mesh-rt/src/lib.rs` — Re-exported the new pg_schema runtime ABI symbols.
- `compiler/mesh-typeck/src/infer.rs` — Registered `Pg.*` schema helper signatures in the typechecker.
- `compiler/mesh-codegen/src/mir/lower.rs` — Lowered the new pg schema helper builtins through MIR.
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — Declared LLVM intrinsics for the new pg schema helpers.
- `compiler/mesh-repl/src/jit.rs` — Registered the new pg schema helper runtime symbols for REPL/JIT resolution.
- `mesher/migrations/20260216120000_create_initial_schema.mpl` — Rewrote Mesher's initial schema migration onto neutral `Migration.*` helpers plus explicit `Pg.*` helpers and removed raw pool execution sites.
- `compiler/meshc/src/migrate.rs` — Updated generated migration scaffolds to teach explicit `Pg.*` schema helpers instead of raw `Migration.execute(...)` examples.
- `compiler/meshc/tests/e2e.rs` — Added compile coverage for named/ordered index helpers and Pg schema-helper migration examples.
- `mesher/storage/schema.mpl` — Made `Storage.Schema` the sole Mesher owner of runtime partition create/list/drop wrappers over `Pg.*` helpers.
- `mesher/storage/queries.mpl` — Removed the old exported partition lifecycle helpers from the generic query module.
- `mesher/services/retention.mpl` — Rewired retention cleanup onto `Storage.Schema` partition helpers and improved localized failure logging for project cleanup, partition listing, and partition drops.
- `mesher/main.mpl` — Bootstrapped seven days of partitions through `Storage.Schema` at startup and added explicit success/failure logging.
- `compiler/meshc/tests/e2e_m033_s04.rs` — Added the live Postgres S04 proof bundle for migration catalog state, runtime partition helper lifecycle, and Mesher startup bootstrap/logging.
- `scripts/verify-m033-s04.sh` — Added the stable S04 verifier that reruns the slice acceptance commands and mechanically enforces the migration/runtime raw-boundary and observability contract.
- `scripts/verify-m033-s03.sh` — Removed the temporary S04 partition/catalog exemption from the S03 raw keep-list verifier.
- `.gsd/REQUIREMENTS.md` — Updated M033 requirement status after the S04 proof closed the remaining schema/partition gap.
- `.gsd/DECISIONS.md` — Recorded the S04 neutral-vs-PG boundary, partition-ownership, and verification decisions.
- `.gsd/KNOWLEDGE.md` — Captured the partition-helper and verifier gotchas future slices should preserve.
- `.gsd/PROJECT.md` — Refreshed project state to mark S04 complete and narrow the remaining M033 work to S05 docs and integrated replay.
