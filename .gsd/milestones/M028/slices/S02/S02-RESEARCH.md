# M028/S02 — Research

**Date:** 2026-03-23

## Summary

S02 owns **R003** directly and should strengthen the existing `reference-backend/` proof instead of creating any new backend surface. The current golden-path proof is real but still shallow: `compiler/meshc/tests/e2e_reference_backend.rs` has only three tests, two are ignored, and the only Postgres-backed correctness proof mostly delegates to `reference-backend/scripts/smoke.sh`, which exercises one linear happy path. I verified the fast build-only proof still passes; the trust gap is not “can it build,” but “does the runtime path stay correct when the backend does more than one idealized thing?”

The biggest correctness risk in the current code is the job-claim path. `reference-backend/storage/jobs.mpl` does `SELECT oldest pending job` and then a separate `UPDATE ... WHERE id = ? AND status = 'pending'`. That is fine for a single lucky worker, but under two backend instances or two claimers pointed at the same database it can turn ordinary contention into `update_where: no rows matched`, which `reference-backend/jobs/worker.mpl` currently counts as a worker failure. That is exactly the kind of “implemented but not trustworthy” behavior S02 is supposed to retire.

## Recommendation

Follow the **debug-like-expert** rule _“VERIFY, DON’T ASSUME”_ and the **test** skill rule _“MATCH EXISTING PATTERNS.”_ Concretely: extend the existing Rust integration harness in `compiler/meshc/tests/e2e_reference_backend.rs` until it can reproduce the missing proof, then harden the storage claim path in `reference-backend/storage/jobs.mpl` so those tests pass. Do **not** invent a second proof harness, and do **not** pull supervision work from S05 forward unless the new tests prove the single-worker golden path cannot be trusted without it.

The best implementation shape is:
1. keep using `reference-backend/` as the only app under test
2. grow `e2e_reference_backend.rs` from build/start/smoke into a real correctness harness with JSON assertions, direct DB assertions, and multi-instance execution
3. make job claiming atomic at the storage layer, likely via `Repo.query_raw(...)` with one SQL statement that both claims and returns the row (`FOR UPDATE SKIP LOCKED` / single `UPDATE ... RETURNING` shape)
4. only drop into `compiler/mesh-rt/src/db/*` or `compiler/meshc/src/migrate.rs` if the stronger tests expose a real runtime primitive bug rather than an app-level race

## Implementation Landscape

### Key Files

- `compiler/meshc/tests/e2e_reference_backend.rs` — existing compiler-facing proof harness. Right now it only proves build-only, startup reachability, and shell-script smoke. This is the primary place to add richer S02 verification helpers: configurable ports, multi-instance spawning, JSON parsing with `serde_json`, migration assertions, and direct DB truth checks.
- `reference-backend/storage/jobs.mpl` — highest-probability correctness fix. `claim_next_pending_job()` currently does a non-atomic read-then-update sequence and inherits `Repo.update_where()`’s zero-row error path.
- `reference-backend/jobs/worker.mpl` — current error classification surface. Any `Err(...)` from `claim_next_pending_job()` except `"no pending jobs"` increments `failed_jobs`, so claim contention currently looks like runtime failure.
- `reference-backend/api/health.mpl` — existing observability surface for worker state (`status`, `poll_ms`, timestamps, `processed_jobs`, `failed_jobs`, `last_job_id`, `last_error`). S02 should assert against this instead of inventing new status endpoints.
- `reference-backend/api/jobs.mpl` — HTTP contract for `POST /jobs` and `GET /jobs/:id`; useful for verifying persisted row shape, 404 behavior, and post-worker transitions.
- `reference-backend/migrations/20260323010000_create_jobs.mpl` — canonical durable schema. If S02 adds stronger migration assertions or atomic-claim SQL, keep them consistent with this schema and index (`idx_jobs_pending_scan`).
- `compiler/meshc/src/migrate.rs` — existing migration runner already uses `mesh_rt::db::pg::{native_pg_connect, native_pg_execute, native_pg_query}`. That makes it the best pattern to follow for DB truth checks from Rust tests instead of shelling out to `psql`.
- `compiler/mesh-rt/src/db/repo.rs` — fallback seam if app-level storage changes are blocked by runtime behavior. `mesh_repo_update_where()` currently turns a zero-row update into `"update_where: no rows matched"`, which is exactly what makes claim contention surface as failure today.
- `compiler/mesh-rt/src/db/pool.rs` — runtime pool primitive already has one ignored Postgres round-trip test. Only touch this if richer S02 tests expose a real checkout/execute/query defect rather than app logic.
- `reference-backend/scripts/smoke.sh` — still useful as operator smoke, but it should remain a coarse package-local smoke path. S02’s stronger proof should move into Rust integration tests, not into more shell parsing.

### Build Order

1. **Expand the proof harness first.**
   - Add the missing assertions to `compiler/meshc/tests/e2e_reference_backend.rs` before changing runtime/app code.
   - This follows the debug-like-expert discipline: reproduce the trust gap, then fix it.
   - Likely new helpers: unique-port spawning, reusable HTTP/JSON parsing, direct DB queries, multi-instance startup/cleanup.

2. **Add the first failing S02 correctness proofs.**
   - Single-instance richer proof: migration state, health counters, multiple jobs, explicit DB row assertions.
   - Multi-instance proof: two backend processes against one database, asserting jobs are processed exactly once and ordinary contention does not increment `failed_jobs`.

3. **Fix `reference-backend/storage/jobs.mpl` claim semantics.**
   - Replace the current `oldest_pending_job()` + `Repo.update_where(...)` race window with an atomic claim.
   - Keep the success/failure surface compatible with the worker: benign contention should map to `"no pending jobs"` or another non-failure branch, not `failed_jobs`.

4. **Only then touch runtime internals if needed.**
   - If the new harness exposes `Repo.query_raw`, migration runner, or pool-level bugs, the natural fallback files are `compiler/mesh-rt/src/db/repo.rs`, `compiler/meshc/src/migrate.rs`, and then `compiler/mesh-rt/src/db/pool.rs` / `pg.rs`.
   - Do not broaden into supervision or DX work; those are owned by S05 and S03.

### Verification Approach

Use the existing compiler-facing test target as the authoritative proof surface and make it richer.

**Baseline commands already worth keeping:**

```bash
cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_builds -- --nocapture
DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_runtime_starts -- --ignored --nocapture
DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_postgres_smoke -- --ignored --nocapture
```

**New S02 proof should add ignored tests that cover:**

```bash
DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_migration_status_and_apply -- --ignored --nocapture
DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_job_flow_updates_health_and_db -- --ignored --nocapture
DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_multi_instance_claims_once -- --ignored --nocapture
```

**What those tests should assert, not just run:**

- `meshc migrate reference-backend status` shows pending before `up` and applied after `up`
- `_mesh_migrations` contains the expected version after apply
- `POST /jobs` creates persisted rows with `pending` state
- `GET /jobs/:id` eventually returns `processed` with `attempts = 1`
- `GET /health` reflects worker activity (`processed_jobs`, `failed_jobs`, `last_status`, `last_job_id`)
- under two backend instances against one DB, every created job is processed once, no row lands in `failed`, and `failed_jobs` does not grow because of claim contention

**Best DB truth source:**
Use `mesh_rt::db::pg::{native_pg_connect, native_pg_query, native_pg_close}` inside `compiler/meshc/tests/e2e_reference_backend.rs` instead of `psql`. The code already exists and `compiler/meshc/src/migrate.rs` demonstrates the pattern.

## Don't Hand-Roll

| Problem | Existing Solution | Why Use It |
|---------|------------------|------------|
| Direct Postgres assertions from Rust tests | `mesh_rt::db::pg::{native_pg_connect, native_pg_query, native_pg_close}` | Keeps S02 proof inside the repo’s Rust harness, avoids shelling out to `psql`, and reuses the same native PG wire path the migration runner already trusts. |
| JSON response verification in Rust integration tests | `serde_json` already present in `compiler/meshc/Cargo.toml` | Avoids brittle string contains checks once S02 starts asserting concrete health/job payloads. |

## Constraints

- **R003 is the owned requirement.** S02 should harden the existing canonical backend path, not invent a second example app or broaden into supervision/tooling scope owned elsewhere.
- `reference-backend/jobs/worker.mpl` is intentionally a single unsupervised timer loop today; changing supervision strategy is S05 territory unless S02 proves the golden path is impossible without it.
- `compiler/meshc/tests/e2e_reference_backend.rs` and `reference-backend/scripts/smoke.sh` currently hardcode `:18080`; any multi-instance proof needs unique ports and careful cleanup.
- `Repo.update_where()` currently returns `"update_where: no rows matched"` on a zero-row update; with the current claim design, that is a correctness liability, not just an implementation detail.
- S01’s env-validation hot path is intentionally local to `reference-backend/main.mpl`; do not move startup parsing back through a broader config path without re-proving the non-empty `DATABASE_URL` startup path.

## Common Pitfalls

- **Treating smoke as correctness proof** — the current shell smoke proves one happy path, not trust. Per the loaded `test` skill, extend the existing Rust integration-test pattern instead of adding more shell-only checks.
- **Leaving the claim path as read-then-update** — with two backend instances, ordinary contention becomes `failed_jobs`, which looks like runtime instability even when the DB is behaving normally.
- **Diagnosing via logs first** — follow the loaded `debug-like-expert` rule and prove behavior with structured signals first: HTTP JSON, direct DB rows, then logs only when needed.
- **Using fixed port 18080 for concurrent proofs** — this creates false negatives unrelated to the runtime path. Multi-instance tests need parameterized ports.
- **Pulling S05 forward** — the worker is not supervised yet, but S02 should first retire the immediate golden-path correctness blockers before changing failure/restart architecture.

## Open Risks

- Atomic claim in Mesh may need a small runtime escape hatch if the exact SQL shape is awkward through current higher-level `Repo` helpers; `Repo.query_raw` is the first escape hatch to try.
- Multi-instance tests may be timing-sensitive on slower CI if they keep S01’s short poll intervals and wait windows. Borrow the more generous timeout posture already used in `compiler/meshc/tests/e2e_concurrency_stdlib.rs`.
- If a new S02 test trips `mesh_pool_checkout`, `String.to_int`, or result/option unwrapping on the live PG path, the real fix may still be in runtime/codegen payload boxing rather than in `reference-backend/` itself.

## Skills Discovered

| Technology | Skill | Status |
|------------|-------|--------|
| Rust | `apollographql/skills@rust-best-practices` | none installed; promising for runtime/test refactors. Install with `npx skills add apollographql/skills@rust-best-practices` |
| PostgreSQL | `github/awesome-copilot@postgresql-optimization` | none installed; promising if atomic-claim SQL or locking behavior gets tricky. Install with `npx skills add github/awesome-copilot@postgresql-optimization` |
