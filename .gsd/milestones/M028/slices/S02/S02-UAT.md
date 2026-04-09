# S02 UAT: Runtime Correctness on the Golden Path

## Purpose

Validate that `reference-backend/` is no longer just a working smoke demo. This UAT checks the concrete trust claims S02 added: migration truth, HTTP/DB/health agreement for one job lifecycle, atomic shared-DB claiming, and two-instance exact-once processing without benign contention showing up as worker failure.

## Preconditions

- Run every command from the repo root.
- Load the repo-root `.env` into the shell before running Postgres-backed commands:
  ```bash
  set -a
  source .env
  set +a
  ```
- `DATABASE_URL` points at a reachable Postgres instance you are allowed to mutate for test purposes.
- `reference-backend/reference-backend` exists; if not, build it first:
  ```bash
  cargo run -p meshc -- build reference-backend
  ```
- `python3`, `curl`, `rg`, and `psql` are installed.
- Ports `18080`, `18081`, and `18082` are free before the manual runtime cases.
- If a stale `reference-backend` listener exists on those ports, stop it first.

## Test Case 1 — Authoritative Rust harness stays green

### Steps

1. Run the build-only gate:
   ```bash
   cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_builds -- --nocapture
   ```
2. Run the runtime-start gate:
   ```bash
   DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_runtime_starts -- --ignored --nocapture
   ```
3. Run the migration-truth gate:
   ```bash
   DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_migration_status_and_apply -- --ignored --nocapture
   ```
4. Run the single-job lifecycle gate:
   ```bash
   DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_job_flow_updates_health_and_db -- --ignored --nocapture
   ```
5. Run the multi-instance exact-once gate:
   ```bash
   DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_multi_instance_claims_once -- --ignored --nocapture
   ```

### Expected outcome

- All five commands pass.
- The last command runs **1 test**, not 0 tests.
- No command fails early on an unset `DATABASE_URL`.

## Test Case 2 — Migration truth is visible in both CLI and Postgres

### Steps

1. Re-run the dedicated migration proof for a clean assertion path:
   ```bash
   DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_migration_status_and_apply -- --ignored --nocapture
   ```
2. After it passes, inspect the migration table directly:
   ```bash
   psql "$DATABASE_URL" -P pager=off -x -c "SELECT version, applied_at IS NOT NULL AS applied FROM _mesh_migrations ORDER BY version;"
   ```
3. Confirm the expected version exists:
   ```bash
   psql "$DATABASE_URL" -Atqc "SELECT COUNT(*) FROM _mesh_migrations WHERE version = '20260323010000_create_jobs';"
   ```

### Expected outcome

- The test passes.
- `_mesh_migrations` contains `20260323010000_create_jobs`.
- `applied_at` is non-null for that version.
- There is no mismatch between the CLI proof and the database truth.

## Test Case 3 — One job’s API state, DB row, and `/health` stay in sync

### Steps

1. Start one backend instance and capture logs:
   ```bash
   mkdir -p .gsd/tmp
   rm -f .gsd/tmp/s02-single.log .gsd/tmp/s02-single.pid
   DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} PORT=18080 JOB_POLL_MS=200 ./reference-backend/reference-backend >.gsd/tmp/s02-single.log 2>&1 & echo $! > .gsd/tmp/s02-single.pid
   ```
2. Wait for `/health` to come up:
   ```bash
   python3 - <<'PY'
import json, time, urllib.request
url = 'http://127.0.0.1:18080/health'
for _ in range(40):
    try:
        data = json.loads(urllib.request.urlopen(url, timeout=2).read().decode())
        print(json.dumps(data))
        raise SystemExit(0)
    except Exception:
        time.sleep(0.25)
raise SystemExit('health never became ready')
PY
   ```
3. Create a job:
   ```bash
   create_json=$(curl -sf -X POST http://127.0.0.1:18080/jobs -H 'content-type: application/json' --data '{"kind":"uat","source":"S02-single","n":1}')
   echo "$create_json"
   ```
4. Extract the id:
   ```bash
   job_id=$(CREATE_JSON="$create_json" python3 - <<'PY'
import json, os
print(json.loads(os.environ['CREATE_JSON'])['id'])
PY
)
   echo "$job_id"
   ```
5. Poll `GET /jobs/:id` until it becomes `processed`:
   ```bash
   JOB_ID="$job_id" python3 - <<'PY'
import json, os, time, urllib.request
job_id = os.environ['JOB_ID']
url = f'http://127.0.0.1:18080/jobs/{job_id}'
for _ in range(60):
    data = json.loads(urllib.request.urlopen(url, timeout=2).read().decode())
    if data['status'] == 'processed':
        print(json.dumps(data))
        raise SystemExit(0)
    time.sleep(0.2)
raise SystemExit('job never reached processed')
PY
   ```
6. Read the same row directly from Postgres:
   ```bash
   psql "$DATABASE_URL" -P pager=off -x -c "SELECT id::text, status, attempts, last_error, processed_at IS NOT NULL AS processed, payload::text FROM jobs WHERE id = '$job_id';"
   ```
7. Check `/health` after the worker finishes:
   ```bash
   curl -sf http://127.0.0.1:18080/health
   ```
8. Check the worker log for a processed-job line:
   ```bash
   rg "Job worker processed id=$job_id" .gsd/tmp/s02-single.log
   ```

### Expected outcome

- `/health` returns HTTP 200 with `status: "ok"`.
- The created job reaches `processed` without manual DB edits.
- `GET /jobs/:id` and the `jobs` row agree on `status = processed`, `attempts = 1`, and `processed_at` being present.
- The DB row has empty/clear `last_error`.
- `/health.worker.failed_jobs` remains `0` and `/health.worker.last_error` remains `null`.
- The worker log shows `Job worker processed id=<job_id>`.

## Test Case 4 — Two instances share one DB and still process each job exactly once

### Steps

1. Stop the single-instance process from Test Case 3:
   ```bash
   kill "$(cat .gsd/tmp/s02-single.pid)" 2>/dev/null || true
   rm -f .gsd/tmp/s02-single.pid
   ```
2. Start two backend instances against the same `DATABASE_URL`:
   ```bash
   rm -f .gsd/tmp/s02-a.log .gsd/tmp/s02-a.pid .gsd/tmp/s02-b.log .gsd/tmp/s02-b.pid
   DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} PORT=18081 JOB_POLL_MS=150 ./reference-backend/reference-backend >.gsd/tmp/s02-a.log 2>&1 & echo $! > .gsd/tmp/s02-a.pid
   DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} PORT=18082 JOB_POLL_MS=150 ./reference-backend/reference-backend >.gsd/tmp/s02-b.log 2>&1 & echo $! > .gsd/tmp/s02-b.pid
   ```
3. Wait for both health endpoints:
   ```bash
   python3 - <<'PY'
import json, time, urllib.request
for port in (18081, 18082):
    url = f'http://127.0.0.1:{port}/health'
    for _ in range(40):
        try:
            data = json.loads(urllib.request.urlopen(url, timeout=2).read().decode())
            print(port, json.dumps(data))
            break
        except Exception:
            time.sleep(0.25)
    else:
        raise SystemExit(f'health never became ready on {port}')
PY
   ```
4. Create 6 jobs, alternating between both instances:
   ```bash
   python3 - <<'PY'
import json, urllib.request
ports = [18081, 18082, 18081, 18082, 18081, 18082]
ids = []
for index, port in enumerate(ports, start=1):
    req = urllib.request.Request(
        f'http://127.0.0.1:{port}/jobs',
        data=json.dumps({'kind':'uat','source':'S02-multi','n':index}).encode(),
        headers={'content-type':'application/json'},
        method='POST',
    )
    with urllib.request.urlopen(req, timeout=3) as resp:
        payload = json.loads(resp.read().decode())
    ids.append(payload['id'])
print('\n'.join(ids))
PY
   ```
5. Poll Postgres until all 6 rows are terminal and correct:
   ```bash
   python3 - <<'PY'
import os, subprocess, time
for _ in range(60):
    out = subprocess.check_output([
        'psql', os.environ['DATABASE_URL'], '-Atqc',
        "SELECT COUNT(*) FILTER (WHERE status = 'processed'), COUNT(*) FILTER (WHERE status = 'failed'), COUNT(*) FILTER (WHERE attempts = 1), COUNT(*) FROM jobs WHERE payload::text LIKE '%S02-multi%'"
    ], text=True).strip()
    processed, failed, attempts_one, total = map(int, out.split('|'))
    if total == 6 and processed == 6 and failed == 0 and attempts_one == 6:
        print(out)
        raise SystemExit(0)
    time.sleep(0.25)
raise SystemExit('jobs never reached exact-once processed state')
PY
   ```
6. Read the rows directly:
   ```bash
   psql "$DATABASE_URL" -P pager=off -x -c "SELECT id::text, status, attempts, last_error, processed_at IS NOT NULL AS processed, payload::text FROM jobs WHERE payload::text LIKE '%S02-multi%' ORDER BY created_at ASC, id ASC;"
   ```
7. Spot-check one job through the opposite instance’s API. First capture an id:
   ```bash
   sample_id=$(psql "$DATABASE_URL" -Atqc "SELECT id::text FROM jobs WHERE payload::text LIKE '%S02-multi%' ORDER BY created_at ASC, id ASC LIMIT 1")
   echo "$sample_id"
   ```
8. Read that job from both instances:
   ```bash
   curl -sf http://127.0.0.1:18081/jobs/$sample_id
   curl -sf http://127.0.0.1:18082/jobs/$sample_id
   ```
9. Check `/health` on both instances:
   ```bash
   curl -sf http://127.0.0.1:18081/health
   curl -sf http://127.0.0.1:18082/health
   ```
10. Check both logs for real participation and absence of the old race failure:
   ```bash
   rg "Job worker processed id=" .gsd/tmp/s02-a.log
   rg "Job worker processed id=" .gsd/tmp/s02-b.log
   ! rg "update_where: no rows matched" .gsd/tmp/s02-a.log .gsd/tmp/s02-b.log
   ```

### Expected outcome

- All 6 jobs end in `processed`.
- No row ends in `failed`.
- Every row has `attempts = 1`.
- Both instances can read the same shared-DB job through `GET /jobs/:id`.
- `/health.worker.failed_jobs` stays `0` on both instances.
- `/health.worker.last_error` is `null` on both instances.
- Both log files contain at least one `Job worker processed id=` line, proving both workers participated.
- Neither log contains `update_where: no rows matched`.

## Test Case 5 — Edge case: benign contention is not counted as failure

### Steps

1. Run the focused contention regression:
   ```bash
   DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_claim_contention_is_not_failure -- --ignored --nocapture
   ```
2. Read the result summary from stdout.

### Expected outcome

- The regression passes.
- It does not report worker failure inflation from ordinary claim contention.
- This remains true even though two backend instances are competing for the same `jobs` table.

## Cleanup

Stop any manually started backend processes:

```bash
kill "$(cat .gsd/tmp/s02-a.pid)" 2>/dev/null || true
kill "$(cat .gsd/tmp/s02-b.pid)" 2>/dev/null || true
rm -f .gsd/tmp/s02-a.pid .gsd/tmp/s02-b.pid .gsd/tmp/s02-single.pid
```

## Notes for the tester

- For this slice, `/health.failed_jobs` and `/health.last_error` are the trustworthy shared-DB contention signal.
- Do **not** use `/health.processed_jobs` or `/health.last_job_id` as the sole exact-once proof under multi-instance polling; use direct `jobs` reads, `GET /jobs/:id`, and per-instance processed-job logs.
- If a Postgres-backed command fails immediately with `${DATABASE_URL:?set DATABASE_URL}`, you forgot to load `.env` into the current shell.
