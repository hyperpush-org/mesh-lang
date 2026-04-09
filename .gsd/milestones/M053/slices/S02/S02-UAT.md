# S02: Generated Postgres starter proves clustered failover truth — UAT

**Milestone:** M053
**Written:** 2026-04-05T20:15:41.467Z

# S02: Generated Postgres starter proves clustered failover truth — UAT

**Milestone:** M053
**Written:** 2026-04-05

## Preconditions
- Working tree rooted at `/Users/sn0w/Documents/dev/mesh-lang`
- Rust toolchain available
- Docker available for a disposable local PostgreSQL instance
- Start a disposable local Postgres and export `DATABASE_URL` before running the rails. One concrete setup that matches the closeout replay is:
  1. `docker run -d --name mesh-m053-s02-uat -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=postgres -p 127.0.0.1::5432 postgres:16-alpine`
  2. `docker port mesh-m053-s02-uat 5432/tcp`
  3. `export DATABASE_URL=postgres://postgres:postgres@127.0.0.1:<published-port>/postgres`
- Optional cleanup after UAT: `docker rm -f mesh-m053-s02-uat`

## Primary Acceptance Flow

### 1. Prove the two-node staged helper seam
1. Run `cargo test -p meshc --test e2e_m053_s02 m053_s02_staged_postgres_helper_ -- --nocapture`.
2. Expected:
   - the command exits 0
   - Cargo reports `3 passed`
3. Set `HELPER_DIR="$(find .tmp/m053-s02 -maxdepth 1 -type d -name 'staged-postgres-helper-dual-node-truth-*' | sort | tail -n 1)"`.
4. Inspect `"$HELPER_DIR"`.
5. Expected retained artifacts include:
   - `health-primary-health.json` and `health-standby-health.json`
   - `cluster-status-primary-status.json` and `cluster-status-standby-status.json`
   - `continuity-before-route-*.json`
   - `cluster-diagnostics-primary.json` and `cluster-diagnostics-standby.json`
   - `route-request-key-primary.json`, `route-record-primary.json`, and `route-record-standby.json`
   - `runtime-primary.combined.log` and `runtime-standby.combined.log`
6. Expected outcome:
   - the helper booted one staged bundle as a primary/standby pair against one shared Postgres database
   - operator and HTTP evidence exists for both nodes
   - no starter-source failover glue was required

### 2. Prove authoritative generated-starter failover truth
1. Run `cargo test -p meshc --test e2e_m053_s02 -- --nocapture`.
2. Expected:
   - the command exits 0
   - Cargo reports `4 passed`
3. Set `FAILOVER_DIR="$(find .tmp/m053-s02 -maxdepth 1 -type d -name 'staged-postgres-failover-runtime-truth-*' | sort | tail -n 1)"`.
4. Inspect these retained files in `"$FAILOVER_DIR"`:
   - `scenario-meta.json`
   - `create-todo-primary.json`
   - `todos-before-failover-primary.json`
   - `post-kill-status-standby.json`
   - `post-kill-diagnostics-standby.json`
   - `post-kill-continuity-standby-completed.json`
   - `get-todo-after-failover-standby.json`
   - `toggle-todo-after-failover-standby.json`
   - `delete-todo-after-failover-standby.json`
   - `missing-todo-after-delete-standby.json`
   - `post-rejoin-status-primary.json`
   - `post-rejoin-status-standby.json`
   - `post-rejoin-diagnostics-primary.json`
5. Expected outcomes:
   - `scenario-meta.json` records a real `request_key`, `initial_attempt_id`, `failover_attempt_id`, `list_route_request_key`, and `todo_id`
   - `create-todo-primary.json` shows the created todo and `todos-before-failover-primary.json` shows the same row through real starter routes
   - `post-kill-status-standby.json` shows `cluster_role=primary`, `promotion_epoch=1`, only the standby node in membership, and a truthful promoted health value (`unavailable`, `local_only`, or `healthy`)
   - `post-kill-diagnostics-standby.json` contains `automatic_promotion`, `automatic_recovery`, and `recovery_rollover`
   - `post-kill-continuity-standby-completed.json` shows the failover attempt completed on the promoted standby
   - the post-failover GET/PUT/DELETE snapshots prove reads, toggle, delete, and 404-after-delete from the promoted node
   - `post-rejoin-status-primary.json` shows the restarted original primary as `standby` with `promotion_epoch=1`
   - `post-rejoin-status-standby.json` shows the promoted node still as `primary` with `promotion_epoch=1`
   - `post-rejoin-diagnostics-primary.json` contains `fenced_rejoin`

### 3. Replay the retained slice verifier
1. Run `bash scripts/verify-m053-s02.sh`.
2. Open:
   - `.tmp/m053-s02/verify/status.txt`
   - `.tmp/m053-s02/verify/current-phase.txt`
   - `.tmp/m053-s02/verify/phase-report.txt`
   - `.tmp/m053-s02/verify/latest-proof-bundle.txt`
3. Set `BUNDLE_DIR="$(cat .tmp/m053-s02/verify/latest-proof-bundle.txt)"`.
4. Inspect `"$BUNDLE_DIR"`.
5. Expected:
   - the command exits 0
   - `status.txt` contains `ok`
   - `current-phase.txt` contains `complete`
   - `phase-report.txt` shows `passed` for `m053-s02-db-env-preflight`, `m053-s01-contract`, `m053-s02-failover-e2e`, `m053-s02-retain-artifacts`, `m053-s02-retain-staged-bundle`, `m053-s02-redaction-drift`, and `m053-s02-bundle-shape`
   - `latest-proof-bundle.txt` points at a retained proof bundle containing:
     - `verify-m053-s02.sh` and `verify-m053-s01.sh`
     - `todo-postgres.README.md` and `todo-postgres.work.mpl`
     - `retained-m053-s02-artifacts/` with exactly one helper, one fail-closed, and one failover artifact directory
     - `retained-staged-bundle/` with `todo-postgres`, `todo-postgres.up.sql`, `apply-deploy-migrations.sh`, and `deploy-smoke.sh`

## Edge Cases

### A. Fail-closed helper artifacts are retained instead of normalized away
1. Set `FAIL_CLOSED_DIR="$(find "$BUNDLE_DIR/retained-m053-s02-artifacts" -maxdepth 1 -type d -name 'staged-postgres-helper-fail-closed-*' | sort | tail -n 1)"`.
2. Confirm `"$FAIL_CLOSED_DIR/missing-bundle.path.txt"` and `"$FAIL_CLOSED_DIR/cluster-status-not-ready.log"` exist.
3. Expected:
   - the malformed staged-bundle pointer case is retained verbatim
   - the non-ready `meshc cluster status` failure is retained verbatim
   - the verifier did not silently reuse stale happy-path artifacts

### B. Promoted-node health may be `unavailable` and still be truthful
1. Open `"$FAILOVER_DIR/post-kill-status-standby.json"` and `"$FAILOVER_DIR/post-rejoin-status-standby.json"`.
2. Expected:
   - if either file reports `replication_health: "unavailable"`, the run is still a **pass** as long as role/epoch/membership are correct and the matching diagnostics plus continuity files prove `automatic_promotion`, `automatic_recovery`, and `fenced_rejoin`
   - do not downgrade the result just because the promoted node is not yet back to `local_only` or `healthy`

### C. The starter README and work source remain bounded
1. Run `cargo test -p meshc --test e2e_m053_s02 m053_s02_staged_postgres_helper_keeps_readme_bounded_and_work_source_clean -- --nocapture`.
2. Expected:
   - the command exits 0
   - `examples/todo-postgres/README.md` still includes `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics`, but omits failover prose like `automatic_recovery`, `fenced_rejoin`, `verify-m053-s02`, and `MESH_STARTUP_WORK_DELAY_MS`
   - `examples/todo-postgres/work.mpl` still omits `Timer.sleep`, `Env.get_int`, `MESH_STARTUP_WORK_DELAY_MS`, `owner_node`, and other app-owned failover glue

## Failure Signals
- any of the three slice-level verification commands exit non-zero
- Cargo runs 0 tests for a named rail/filter
- `.tmp/m053-s02/verify/status.txt` is not `ok`
- `.tmp/m053-s02/verify/current-phase.txt` is not `complete`
- the retained proof bundle is missing the helper, fail-closed, or failover artifact directories
- the retained staged bundle copy is missing `todo-postgres`, `todo-postgres.up.sql`, `apply-deploy-migrations.sh`, or `deploy-smoke.sh`
- `post-kill-diagnostics-standby.json` is missing `automatic_promotion` or `automatic_recovery`
- `post-rejoin-diagnostics-primary.json` is missing `fenced_rejoin`
- any retained artifact leaks the raw `DATABASE_URL`

## UAT Verdict
- **Pass** when the helper rail, the full failover rail, and the assembled verifier are all green and the retained bundle preserves the helper/fail-closed/failover proof surfaces plus the staged deploy bundle snapshot.
- **Fail** if the generated starter needs app-owned failover glue, if operator truth and HTTP truth diverge, if stale-primary fencing or fenced rejoin disappears, or if the retained verifier stops replaying S01 before S02.
