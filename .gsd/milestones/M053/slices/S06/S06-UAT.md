# S06: Hosted failover promotion truth and annotated tag reroll — UAT

**Milestone:** M053
**Written:** 2026-04-06

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: S06 only closes when both the live local failover rail and the hosted workflow freshness chain agree on the same shipped SHA.

## Preconditions

- Docker is running locally.
- `cargo` can build the workspace.
- `gh` is authenticated and `GH_TOKEN` is available for read-only hosted evidence queries.
- Port `55432` is free for a disposable Postgres container.
- The repo is at the S06 closeout state.

## Smoke Test

1. Run `cargo test -p mesh-rt startup_work_dispatch_window_ -- --nocapture`.
2. **Expected:** 4 tests pass, including the positive override, default fallback, and zero-delay non-startup behavior.

## Test Cases

### 1. Runtime-owned startup pending window respects the env override

1. Run `cargo test -p mesh-rt startup_work_dispatch_window_ -- --nocapture`.
2. Confirm the passing test names include:
   - `startup_work_dispatch_window_uses_positive_env_override_for_clustered_startup_requests`
   - `startup_work_dispatch_window_falls_back_to_default_for_zero_negative_or_malformed_env`
3. **Expected:** The runtime, not starter code, owns the override behavior and rejects malformed widening.

### 2. Local staged Postgres failover proof stays green only with the configured window

1. Start a disposable Postgres:
   `docker run --rm --name m053_s06_pg -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=postgres -p 55432:5432 postgres:16-alpine`
2. Run `DATABASE_URL=postgres://postgres:postgres@127.0.0.1:55432/postgres cargo test -p meshc --test e2e_m053_s02 m053_s02_staged_postgres_failover_proves_clustered_http_and_runtime_recovery -- --nocapture`.
3. **Expected:** The test passes and retains a bundle showing pre-kill `startup_dispatch_window.pending_window_ms=20000`, owner-loss promotion/recovery, stale-primary fencing, and successful standby CRUD after failover.

### 3. Assembled local failover verifier retains the starter-owned proof bundle

1. With the same disposable Postgres running, execute `DATABASE_URL=postgres://postgres:postgres@127.0.0.1:55432/postgres bash scripts/verify-m053-s02.sh`.
2. Open `.tmp/m053-s02/verify/status.txt` and `.tmp/m053-s02/verify/phase-report.txt`.
3. **Expected:** `status.txt` is `ok`, every phase in `phase-report.txt` passed, and the retained bundle includes pre-kill diagnostics, post-kill status/continuity, rejoin artifacts, and staged deploy logs.

### 4. Hosted starter/packages/release evidence closes on one shipped SHA

1. Run `bash scripts/verify-m034-s02-workflows.sh`.
2. Run `node --test scripts/tests/verify-m053-s03-contract.test.mjs`.
3. Run `GH_TOKEN=<redacted> bash scripts/verify-m053-s03.sh`.
4. Inspect `.tmp/m053-s03/verify/status.txt`, `.tmp/m053-s03/verify/current-phase.txt`, and `.tmp/m053-s03/verify/remote-runs.json`.
5. **Expected:** `status.txt` is `ok`, `current-phase.txt` is `complete`, and `remote-runs.json` shows fresh successful `authoritative-verification.yml`, `deploy-services.yml`, and `release.yml` runs all aligned on `e5fb36a6fe7e9e56f3a608a608abbaaab6764167`, with `release.yml` querying the annotated `refs/tags/v0.1.0^{}` target.

## Edge Cases

### Invalid or missing startup window env falls back safely

1. Rely on the focused unit rail instead of editing starter code: `cargo test -p mesh-rt startup_work_dispatch_window_ -- --nocapture`.
2. **Expected:** Missing, zero, negative, and malformed `MESH_STARTUP_WORK_DELAY_MS` inputs fall back to the 2500ms default; non-startup or replica-free requests still use zero delay.

### Hosted freshness checks fail closed when main/tag evidence drifts

1. Run `node --test scripts/tests/verify-m053-s03-contract.test.mjs`.
2. **Expected:** The contract tests prove the hosted verifier rejects stale main evidence, missing starter jobs, missing packages-step coverage, malformed GH JSON, or a tag without the peeled `^{}` release target.

## Failure Signals

- `startup_work_dispatch_window_` unit tests fail or rename unexpectedly.
- The S02 e2e/verifier bundles show `pending_window_ms` falling back to 2500 or `startup_completed` landing before the forced owner stop.
- Standby diagnostics show `automatic_promotion_rejected:no_mirrored_state` without the corrected startup-window evidence.
- `.tmp/m053-s03/verify/status.txt` is not `ok`, or `remote-runs.json` points main/tag workflows at different SHAs or a non-annotated tag freshness result.

## Requirements Proved By This UAT

- R121 — The packages site is now part of the normal hosted CI/deploy contract, proven by the green hosted verifier and fresh `deploy-services.yml` evidence on the shipped SHA.
- R122 — The generated Postgres starter remains the truthful clustered deploy/failover path, carried from local staged replay into fresh hosted main/tag evidence.

## Not Proven By This UAT

- R123 load-balancing follow-through beyond the current starter failover and hosted-contract scope.
- Continuous monitoring/alerting outside the retained verifier and workflow evidence surfaces.

## Notes for Tester

- Use a disposable Docker Postgres for local S02 proof; the tests create isolated databases per run, so no shared external database is needed.
- Do not `source .env` for hosted verification. If you need a local `GH_TOKEN`, export just that key because this repo’s `.env` can be dotenv-valid but not shell-safe.
