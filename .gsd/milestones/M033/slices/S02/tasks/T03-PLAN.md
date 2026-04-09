---
estimated_steps: 4
estimated_files: 2
skills_used:
  - test
  - postgresql-database-engineering
---

# T03: Prove PG helper boundaries with Postgres-backed Mesher storage tests

**Slice:** S02 — Explicit PG extras for JSONB, search, and crypto
**Milestone:** M033

## Description

Close the slice with a proof bundle that does not depend on the still-fragile S01 Mesher HTTP readiness harness. This task should exercise the rewritten storage functions directly against live Postgres so the slice can prove auth, search, JSONB insert/defaulting, and keep-list boundaries on the real runtime path. The verifier script is part of the deliverable: future agents need one stable command bundle that says whether S02 still holds.

## Steps

1. Reuse the Docker/Postgres helper pattern from `compiler/meshc/tests/e2e_m033_s01.rs` to create `compiler/meshc/tests/e2e_m033_s02.rs` with named proofs for pgcrypto auth, full-text search ranking and parameter ordering, JSONB tag filtering/breakdown, alert-rule create/fire behavior, and event ingest/defaulting.
2. Make the Rust harness assert on live row contents from `users`, `events`, `alert_rules`, and `alerts`, not just that functions return success, and keep secret-bearing inputs redacted in failure messages.
3. Add `scripts/verify-m033-s02.sh` to run the new test target, `meshc` format/build checks, and a Python keep-list sweep that confirms the owned S02 families no longer use raw SQL while only the named leftovers remain.
4. Keep the keep-list sweep aligned with the actual S02 boundary: allow only the deliberately retained raw sites, and fail loudly if a rewritten owned function drifts back to `Repo.query_raw(...)`, `Repo.execute_raw(...)`, `Query.where_raw(...)`, or `Query.select_raw(...)` for PG-only behavior.

## Must-Haves

- [ ] `compiler/meshc/tests/e2e_m033_s02.rs` contains named live-Postgres proofs for pgcrypto auth, FTS ranking/query binding, JSONB insert/defaulting, and alert/event helper behavior
- [ ] `scripts/verify-m033-s02.sh` runs the slice proof bundle plus `meshc` build/fmt checks and an owned keep-list sweep
- [ ] Failure output stays actionable and redacted: no raw passwords, tokens, or connection strings appear in logs

## Verification

- `cargo test -p meshc --test e2e_m033_s02 -- --nocapture`
- `bash scripts/verify-m033-s02.sh`

## Observability Impact

- Signals added/changed: named `e2e_m033_s02_*` failures and verifier-script keep-list errors localize auth/search/jsonb drift immediately
- How a future agent inspects this: rerun the test target or the verifier script and inspect the first failing proof or keep-list assertion
- Failure state exposed: row-level mismatches, placeholder-order drift, and raw-boundary regressions become explicit without relying on the S01 HTTP harness

## Inputs

- `compiler/meshc/tests/e2e_m033_s01.rs` — reusable Docker/Postgres harness and direct-row assertion patterns
- `scripts/verify-m033-s01.sh` — existing slice verifier structure to mirror and adapt for S02
- `compiler/mesh-rt/src/db/query.rs` — expression-valued SELECT/WHERE parameter ordering that the proof bundle must exercise
- `compiler/mesh-rt/src/db/repo.rs` — expression-valued insert plumbing that the proof bundle must exercise
- `mesher/storage/queries.mpl` — rewritten auth/search/alert JSONB helper families to exercise directly
- `mesher/storage/writer.mpl` — rewritten event ingest path to exercise directly

## Expected Output

- `compiler/meshc/tests/e2e_m033_s02.rs` — permanent direct Postgres-backed S02 proof bundle for PG helpers on the real Mesh runtime path
- `scripts/verify-m033-s02.sh` — stable closeout command bundle and raw keep-list gate for the slice
