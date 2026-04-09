# S01: Neutral expression core on real write paths — UAT

**Milestone:** M033
**Written:** 2026-03-25T16:12:59.858Z

# S01 UAT — Neutral expression core on real write paths

## Preconditions
- Run from the repo root with Docker available locally.
- `postgres:16` is available or pullable.
- No other process is holding port `5432` while the temporary Postgres container is running.
- Rust/Cargo tooling is installed and able to build `meshc` and `mesher`.

## Test Case 1 — Portable expression core compiles and executes
1. Run `cargo test -p meshc --test e2e_m033_s01 expr_ -- --nocapture`.
   - **Expected:** All expr-focused tests pass, including expression-valued SELECT, computed UPDATE, conflict-update UPSERT, UUID assignment/clear, and the negative guard tests.
2. Confirm the test output includes the named successes for `e2e_m033_expr_select_executes`, `e2e_m033_expr_repo_executes`, and `e2e_m033_expr_uuid_update_executes`.
   - **Expected:** The neutral core proves expression-valued select/update/upsert work without `RAW:` escape hatches.

## Test Case 2 — Fresh Mesher boot accepts the first event and rate limits honestly
1. Run `cargo test -p meshc --test e2e_m033_s01 mesher_ingest_first_event -- --nocapture`.
   - **Expected:** The first seeded-key `/api/v1/events` request returns `202 Accepted`, a second under-threshold request is still accepted, and the configured threshold is enforced only when exceeded.
2. Review the test assertions/logs.
   - **Expected:** The harness sees `RateLimiter started (60s window, 2 max)` and the DB-side `issues.event_count` stays unchanged on the rejected request.

## Test Case 3 — Live mutation routes drive the neutral write surface
1. Run `cargo test -p meshc --test e2e_m033_s01 mesher_mutations -- --nocapture`.
   - **Expected:** The test passes end to end against a real Postgres-backed Mesher instance.
2. Verify the assertions exercised by the test.
   - **Expected:**
     - assigning and unassigning an issue changes `issues.assigned_to` between a UUID and `NULL`
     - revoking an API key sets `api_keys.revoked_at`
     - acknowledging and resolving an alert set `alerts.acknowledged_at` and `alerts.resolved_at`
     - partial settings updates change `projects.retention_days` and `projects.sample_rate`

## Test Case 4 — Repeated ingest upserts the same issue and persists events
1. Run `cargo test -p meshc --test e2e_m033_s01 mesher_issue_upsert -- --nocapture`.
   - **Expected:** The test passes against a real Postgres-backed Mesher instance.
2. Confirm the behavior asserted by the harness.
   - **Expected:**
     - first ingest creates one issue with `status = unresolved` and `event_count = 1`
     - second ingest updates the same issue row, increments `event_count` to `2`, and advances `last_seen`
     - resolving the issue flips `status` to `resolved`
     - a later repeated event reopens the same issue, increments `event_count` to `3`, advances `last_seen`, and persists `3` rows in `events` for that `issue_id`
3. Edge condition to watch.
   - **Expected:** Low-volume event persistence still succeeds even though the batch size is not reached, because the timer flush path is active.

## Test Case 5 — Slice acceptance bundle and raw keep-list stay honest
1. Run `bash scripts/verify-m033-s01.sh`.
   - **Expected:** The script finishes with `verify-m033-s01: ok`.
2. Confirm what the script enforces.
   - **Expected:**
     - full `e2e_m033_s01` suite passes
     - `cargo run -q -p meshc -- fmt --check mesher` passes
     - `cargo run -q -p meshc -- build mesher` passes
     - the raw keep-list sweep only allows the explicit S02-owned PG sites (`create_alert_rule`, `fire_alert`, `insert_event`)
3. Negative boundary check.
   - **Expected:** The portable S01-owned write families (`revoke_api_key`, `upsert_issue`, `assign_issue`, `acknowledge_alert`, `resolve_fired_alert`, `update_project_settings`) do not trip the raw SQL keep-list gate.
