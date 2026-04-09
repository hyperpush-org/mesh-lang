# S02: Explicit PG extras for JSONB, search, and crypto — UAT

**Milestone:** M033
**Written:** 2026-03-25T17:55:50.756Z

# S02: Explicit PG extras for JSONB, search, and crypto — UAT

**Milestone:** M033
**Written:** 2026-03-25

## UAT Type

- UAT mode: live-runtime + artifact-driven
- Why this mode is sufficient: this slice is a storage/runtime boundary change, so the trustworthy acceptance surface is the real Postgres-backed Mesher helper path plus the keep-list verifier, not a UI walkthrough.

## Preconditions

- Docker is running and port `5432` is free for the temporary Postgres container used by `compiler/meshc/tests/e2e_m033_s02.rs`.
- Rust/Cargo toolchain is available in the repo.
- Run from the repo root: `/Users/sn0w/Documents/dev/mesh-lang`.

## Smoke Test

1. Run `bash scripts/verify-m033-s02.sh`.
2. **Expected:** the script completes with `verify-m033-s02: ok` after replaying the full `e2e_m033_s02` suite, `meshc` format/build checks, and the raw keep-list sweep.

## Test Cases

### 1. pgcrypto auth helper path

1. Run `cargo test -p meshc --test e2e_m033_s02 e2e_m033_s02_pgcrypto_auth_helpers -- --nocapture`.
2. **Expected:** the probe prints `created=` and `auth_ok=` markers, rejects the wrong password with `auth_wrong=not found`, and the harness confirms `users.password_hash = crypt(password, password_hash)` while never echoing the plaintext password.

### 2. Full-text search ranking and parameter ordering

1. Run `cargo test -p meshc --test e2e_m033_s02 e2e_m033_s02_search_fulltext_ranking_and_binding -- --nocapture`.
2. **Expected:** only the default-project events are returned, the hottest message ranks above the warmer match, and the DB row assertions prove project isolation plus stable `rank` ordering.

### 3. JSONB tag filtering, breakdown, and event defaulting

1. Run `cargo test -p meshc --test e2e_m033_s02 e2e_m033_s02_jsonb_tag_helpers -- --nocapture`.
2. Run `cargo test -p meshc --test e2e_m033_s02 e2e_m033_s02_event_ingest_defaulting -- --nocapture`.
3. **Expected:** tag filtering returns the two prod-tagged rows, tag breakdown returns `prod|2` then `staging|1`, untagged events default `tags`/`extra` to `{}`, and the ingest/defaulting proof confirms the stored issue/event shapes on the live `events` table.

### 4. Alert-rule create/fire helper path

1. Run `cargo test -p meshc --test e2e_m033_s02 e2e_m033_s02_alert_rule_and_fire_helpers -- --nocapture`.
2. **Expected:** the probe prints `event_rule_id=`, `threshold_rule_id=`, and `alert_id=` markers; the harness confirms default cooldown/action behavior for the event rule, threshold JSONB extraction for the threshold rule, `last_fired_at` is updated, and the stored `alerts.condition_snapshot` JSON matches the fired rule.

## Edge Cases

### Keep-list boundary stays honest

1. Run `python3` keep-list sweep indirectly through `bash scripts/verify-m033-s02.sh`.
2. **Expected:** the verifier allows only the named 24-hour `Query.where_raw(...)` clauses in the owned search helpers, requires `extract_event_fields` to keep its `Repo.query_raw(...)` call plus the in-function `Honest raw S03 keep-site` comment, and fails if any S02-owned helper regresses to `Repo.query_raw`, `Repo.execute_raw`, or `Query.select_raw`.

### Structured JSONB build arguments stay typed

1. Remove the `Pg.text(...)` casts around the `jsonb_build_object(...)` arguments in `fire_alert` and rerun `cargo test -p meshc --test e2e_m033_s02 e2e_m033_s02_alert_rule_and_fire_helpers -- --nocapture` only as a diagnostic experiment.
2. **Expected:** PostgreSQL raises `42P18 could not determine data type of parameter ...`; restoring the casts returns the proof to green.

## Failure Signals

- `meshc build failed for Mesher storage probe` indicates a probe program parse/typecheck regression in the temporary copied Mesher project.
- Missing `created=`, `auth_ok=`, `event_rule_id=`, `threshold_rule_id=`, or `alert_id=` markers indicates a helper-family runtime regression before DB assertions even begin.
- Raw keep-list drift from `scripts/verify-m033-s02.sh` indicates an owned helper slipped back to a raw query fragment or the named S03 raw-boundary comment drifted.
- Any mismatch in the direct `users`, `events`, `alert_rules`, `alerts`, or `issues` row assertions indicates a real behavior change on the live Postgres path.

## Requirements Proved By This UAT

- R037 — proves the shipped PG-specific helper usage for JSONB/search/crypto/alert storage paths on live Mesher runtime code.
- R036 — proves the slice keeps PG-only behavior explicit instead of leaking it into the neutral ORM API.
- R040 — proves the extension seam remains explicit because the verified helpers are namespaced PG behavior layered on top of the neutral expression core.

## Not Proven By This UAT

- Partition/schema helper coverage and partition lifecycle proof (S04).
- The final public docs and end-to-end assembled Mesher replay (S05).
- An honest expression-surface replacement for `extract_event_fields`; that remains the named S03 raw keep-site.

## Notes for Tester

- If a targeted proof fails with a Mesh parse error around `#{...}` interpolation, inspect the temporary probe template in `compiler/meshc/tests/e2e_m033_s02.rs` first; direct `Map.get(..., \"field\")` calls inside interpolation were a known source of false probe failures and should stay hoisted into locals.
- If the alert proof fails with PostgreSQL parameter-typing ambiguity, inspect `fire_alert` in `mesher/storage/queries.mpl` and confirm the `jsonb_build_object(...)` string arguments are still wrapped in `Pg.text(...)`.

