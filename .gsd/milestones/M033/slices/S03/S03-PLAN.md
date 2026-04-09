# S03: Hard read-side coverage and honest raw-tail collapse

**Goal:** Collapse Mesher’s read-side raw SQL honestly by rewriting the mechanically expressible read helpers and the recurring hard whole-query families onto the existing `Query` / `Expr` / `Pg` surfaces or small Mesh-side decompositions, while leaving only a short named raw keep-list instead of a fake universal SQL abstraction.
**Demo:** After this: After this: Mesher’s recurring scalar-subquery, derived-table, parameterized select, and expression-heavy read paths use the new builders wherever honest, and the remaining raw query keep-list is short and named.

## Tasks
- [x] **T01: Seeded the S03 harness and rewrote the basic read helpers, but the new Mesh probes still need quote cleanup** — Start S03 with the lowest-risk raw-tail collapse and the permanent proof harness. This task should create the first real `compiler/meshc/tests/e2e_m033_s03.rs` file instead of deferring all proof work to the end, then use the current S01/S02 `Expr` / `Query` / `Pg` surface to eliminate the simplest projection/count/cast read helpers in `mesher/storage/queries.mpl`. The key constraint is caller stability: preserve every row key the existing API and ingestion callers read today.

## Steps

1. Copy the Docker/Postgres harness pattern from `compiler/meshc/tests/e2e_m033_s02.rs` into a new `compiler/meshc/tests/e2e_m033_s03.rs` target and add the first named `e2e_m033_s03_basic_reads_*` proofs for the easy helper families.
2. Rewrite the plain projection/count/cast helpers in `mesher/storage/queries.mpl` — `count_unresolved_issues`, `get_issue_project_id`, `validate_session`, `list_api_keys`, `list_alert_rules`, `get_all_project_retention`, `get_project_storage`, and `get_project_settings` — to use `Query.select_expr{s}`, `Query.where_expr`, `Expr.label`, `Expr.coalesce`, and explicit `Pg.*` casts where the current surface already expresses the query honestly.
3. Keep the caller-visible map keys stable for `mesher/ingestion/routes.mpl`, `mesher/api/dashboard.mpl`, `mesher/api/settings.mpl`, and `mesher/api/alerts.mpl`; only touch a caller if a field name would otherwise drift.
4. Leave the hard whole-query raw families and the named S03 leftovers for later tasks instead of sneaking in dishonest abstractions during the easy cleanup pass.

## Must-Haves

- [ ] `compiler/meshc/tests/e2e_m033_s03.rs` exists with named `e2e_m033_s03_basic_reads_*` coverage for the T01 helper families
- [ ] The T01 helper families no longer depend on raw projection strings or trivial raw whole-query SQL where the existing builder surface is already honest
- [ ] Caller-visible row keys such as `cnt`, `project_id`, `token`, `revoked_at`, `retention_days`, `sample_rate`, `event_count`, and `estimated_bytes` remain unchanged
  - Estimate: 2h
  - Files: compiler/meshc/tests/e2e_m033_s03.rs, mesher/storage/queries.mpl, mesher/ingestion/routes.mpl, mesher/api/dashboard.mpl, mesher/api/settings.mpl, mesher/api/alerts.mpl
  - Verify: - `cargo test -p meshc --test e2e_m033_s03 basic_reads -- --nocapture`
- `cargo run -q -p meshc -- build mesher`
- [x] **T02: Attempted the T02 composed-read proof expansion, fixed the probe-compatible boolean helpers, and recorded a storage-probe blocker for the remaining read families.** — Keep pushing the read-side cleanup on the families that already fit the current ORM surface but still lean on raw SELECT, ORDER BY, or GROUP BY fragments. This task is still Mesher-only work: use the current builder and explicit `Pg.*` seam rather than widening the neutral core. The important constraint is that the dashboard/detail/search/team/alerts callers must see the same row keys and ordering semantics they consume today.

## Steps

1. Extend `compiler/meshc/tests/e2e_m033_s03.rs` with named `e2e_m033_s03_composed_reads_*` coverage for the joined, aggregate, and list families this task owns.
2. Rewrite the joined and aggregate read helpers in `mesher/storage/queries.mpl` — `get_project_by_api_key`, `list_issues_by_status`, `event_volume_hourly`, `error_breakdown_by_level`, `top_issues_by_frequency`, `event_breakdown_by_tag`, `get_event_detail`, and `get_members_with_users` — onto `Query.select_expr{s}`, ordinary `group_by` / `order_by`, `Expr.label`, `Expr.coalesce`, and explicit `Pg.*` casts wherever those surfaces already tell the truth.
3. Rewrite the remaining current-surface list helpers that only need conditional query assembly or projection cleanup — `list_events_for_issue`, `list_alerts`, `check_new_issue`, and `should_fire_by_cooldown` — without promoting them back to `Repo.query_raw(...)` whole-query strings.
4. Keep the map keys, sort order, and null/default handling stable for `mesher/api/search.mpl`, `mesher/api/dashboard.mpl`, `mesher/api/detail.mpl`, `mesher/api/alerts.mpl`, and `mesher/api/team.mpl`.

## Must-Haves

- [ ] The T02 joined, aggregate, and list helpers use the current builder surface wherever it is already honest instead of recurring raw SELECT / ORDER / GROUP fragments
- [ ] `compiler/meshc/tests/e2e_m033_s03.rs` contains named `e2e_m033_s03_composed_reads_*` proofs for the T02 families
- [ ] Caller-visible row keys, ordering, and null/default semantics stay unchanged for the dashboard/detail/search/team/alerts surfaces
  - Estimate: 2.5h
  - Files: compiler/meshc/tests/e2e_m033_s03.rs, mesher/storage/queries.mpl, mesher/api/search.mpl, mesher/api/dashboard.mpl, mesher/api/detail.mpl, mesher/api/alerts.mpl, mesher/api/team.mpl
  - Verify: - `cargo test -p meshc --test e2e_m033_s03 composed_reads -- --nocapture`
- `cargo run -q -p meshc -- build mesher`
  - Blocker: `compiler/meshc/tests/e2e_m033_s03.rs` now contains partial `composed_reads` work, but the remaining T02 proof path is blocked by current compiler/runtime behavior in temporary Mesher storage probes: imported `List<Issue>` / similar struct-list results can fail with LLVM verifier or runtime switch crashes, and some aggregate map rows stringify as raw pointer addresses when staged through helper bindings. The Mesher app itself still builds, but the storage-only proof strategy assumed by the task plan is not sufficient to finish T02 as written.
- [x] **T03: Rewrite the S03 composed-read harness onto live Mesher routes and capture the remaining caller-contract regressions as the blocker** — Why: T02 showed the copied storage-only probe cannot safely consume the remaining struct-list and aggregate read shapes, so S03 needs a higher-level proof boundary before more read-side work is credible.

Do: Keep the passing `basic_reads` family, then move the partial `composed_reads` coverage off the direct storage-probe staging path and onto a Mesher-backed surface that exercises the same `search` / `dashboard` / `detail` / `alerts` / `team` caller contracts. Prove the already-rewritten joined, list, aggregate, and boolean helper families there (`get_project_by_api_key`, `list_issues_by_status`, `event_volume_hourly`, `error_breakdown_by_level`, `top_issues_by_frequency`, `event_breakdown_by_tag`, `get_event_detail`, `get_members_with_users`, `list_events_for_issue`, `list_alerts`, `check_new_issue`, `should_fire_by_cooldown`). If the new proof surface still trips the same staging bug, limit any compiler/runtime-side changes to the smallest test-enabler needed for honest read assertions rather than widening Mesh product scope.
  - Estimate: 3h
  - Files: compiler/meshc/tests/e2e_m033_s03.rs, mesher/storage/queries.mpl, mesher/api/search.mpl, mesher/api/dashboard.mpl, mesher/api/detail.mpl, mesher/api/alerts.mpl, mesher/api/team.mpl, .gsd/KNOWLEDGE.md
  - Verify: cargo test -p meshc --test e2e_m033_s03 composed_reads -- --nocapture
cargo run -q -p meshc -- build mesher
  - Blocker: The Mesher-backed composed-read gate still fails. `/api/v1/projects/:project_id/issues` currently returns blank `id`/`title`/`level`/`status` fields while counts/timestamps survive, `/api/v1/projects/:project_id/dashboard/volume` currently drops `bucket` values while keeping counts, `/api/v1/issues/:issue_id/events` currently crashes in `_handle_list_issue_events` with `non-exhaustive match in switch`, and the new-issue ingest path did not create the expected fresh alert for the live alert proof.
- [x] **T04: Stabilized a partial T04 pass with builder-backed issue/event read rewrites and precise resume notes for the remaining S03 regressions** — Why: Once the proof surface is honest again, S03 still has to retire the slice-owned whole-query raw families rather than leaving the main raw tail untouched.

Do: Rewrite `list_issues_filtered`, `project_health_summary`, `get_event_neighbors`, and `evaluate_threshold_rule` to use conditional builder-backed reads plus small Mesh-side composition, then add named `hard_reads` proofs on the Mesher-backed harness. Re-evaluate `extract_event_fields`, `check_volume_spikes`, and `check_sample_rate` after the rewrite pass; retire any that become honest, and keep only the genuinely dishonest leftovers in an explicit named keep-list with justification instead of hiding them behind a fake universal query abstraction.
  - Estimate: 3h
  - Files: compiler/meshc/tests/e2e_m033_s03.rs, mesher/storage/queries.mpl, mesher/api/search.mpl, mesher/api/dashboard.mpl, mesher/api/detail.mpl, mesher/api/alerts.mpl, mesher/ingestion/pipeline.mpl, mesher/ingestion/routes.mpl
  - Verify: cargo test -p meshc --test e2e_m033_s03 hard_reads -- --nocapture
cargo run -q -p meshc -- build mesher
- [x] **T05: Close S03 with a passing live Postgres verifier, hard-read coverage, and a named raw keep-list gate** — Why: After the proof-surface pivot and hard-family rewrites, the slice still needs one stable rerunnable acceptance path that proves both behavior and the raw-boundary contract.

Do: Finish the full live-Postgres `e2e_m033_s03.rs` suite on the new harness, then add or update `scripts/verify-m033-s03.sh` so it runs the full S03 test target, Mesher fmt/build checks, and a keep-list sweep naming the only allowed S03 leftovers while excluding the S04-owned partition/catalog raw sites. Make failures point at the drifting proof family or offending function block so future agents do not need to rediscover the boundary by hand.
  - Estimate: 2h
  - Files: compiler/meshc/tests/e2e_m033_s03.rs, scripts/verify-m033-s03.sh, mesher/storage/queries.mpl, compiler/meshc/tests/e2e_m033_s02.rs, scripts/verify-m033-s02.sh
  - Verify: cargo test -p meshc --test e2e_m033_s03 -- --nocapture
cargo run -q -p meshc -- fmt --check mesher
cargo run -q -p meshc -- build mesher
bash scripts/verify-m033-s03.sh
