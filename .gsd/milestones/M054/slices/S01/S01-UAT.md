# S01: One-public-URL starter ingress truth — UAT

**Milestone:** M054
**Written:** 2026-04-06T06:32:10.547Z

# S01: One-public-URL starter ingress truth — UAT

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: this slice changes generator-owned starter wording, the staged two-node Postgres proof harness, a new public-ingress test surface, and the retained verifier contract. The truthful acceptance path therefore needs both live runtime replay and retained artifact inspection.

## Preconditions

1. Docker is available locally.
2. A disposable PostgreSQL container can be started locally, and `DATABASE_URL` is set to that admin URL (for example `postgres://<user>:<password>@127.0.0.1:<port>/postgres`).
3. The repo is built from the current working tree.
4. No manual cleanup inside `.tmp/m054-s01/` is required before the run; the slice wrapper manages its own verify tree.

## Smoke Test

1. Run `DATABASE_URL=<redacted-local-admin-url> bash scripts/verify-m054-s01.sh`.
2. Expected:
   - the command exits `0`
   - `.tmp/m054-s01/verify/status.txt` contains `ok`
   - `.tmp/m054-s01/verify/current-phase.txt` contains `complete`
   - `.tmp/m054-s01/verify/latest-proof-bundle.txt` points at a retained bundle under `.tmp/m054-s01/proof-bundles/`

## Test Cases

### 1. Starter wording and committed example stay on the bounded one-public-URL contract

1. Run `cargo build -q -p meshc`.
2. Run `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check`.
3. Run `node --test scripts/tests/verify-m054-s01-contract.test.mjs`.
4. Run `cargo test -p meshc --test e2e_m047_s04 m047_s04_example_readmes_define_the_public_postgres_vs_sqlite_split -- --nocapture`.
5. Expected:
   - the Postgres starter/example says one public app URL may front multiple nodes
   - `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics` are the inspection path
   - the Postgres README does not drift back to `Fly.io` or frontend-aware routing claims
   - the SQLite starter remains explicitly local-only and does not pick up a clustered/public-URL story

### 2. The one-public-URL runtime rail stays green

1. Run `DATABASE_URL=<redacted-local-admin-url> cargo test -p meshc --test e2e_m054_s01 -- --nocapture`.
2. Expected:
   - all 4 tests pass
   - a fresh `.tmp/m054-s01/staged-postgres-public-ingress-truth-*` directory exists
   - that artifact bucket contains `public-ingress.requests.json`, `public-ingress.snapshot.json`, `selected-route.summary.json`, `cluster-diagnostics-primary.json`, `cluster-diagnostics-standby.json`, and `public-deploy-smoke.stdout.log`

### 3. The selected standby-first request proves proxy ingress versus runtime placement

1. Open `public-selected-list.request-summary.json` inside the fresh `staged-postgres-public-ingress-truth-*` artifact bucket.
2. Open `selected-route.summary.json` from the same bucket.
3. Expected:
   - the selected public request is `request_id=1`, `GET /todos`, `status_code=200`, and `target_label=standby`
   - `selected-route.summary.json` shows:
     - `public_target_label=standby`
     - `ingress_node=<standby node>`
     - `owner_node=<primary node>`
     - `replica_node=<standby node>`
     - `execution_node=<primary node>`
     - `runtime_name=Api.Todos.handle_list_todos`
     - `phase=completed`
     - `result=succeeded`
4. This is the main S01 proof: the client/public URL hits one node, while Mesh runtime placement/execution truth is still visible separately.

### 4. CRUD still works through the same public URL after the selected request

1. Open `public-ingress.snapshot.json` from the same truth artifact bucket.
2. Expected:
   - `request_count` is at least `8`
   - the first request is the standby-first `GET /todos`
   - later requests show `/health`, `POST /todos`, `GET /todos/:id`, `PUT /todos/:id`, another `GET /todos`, `DELETE /todos/:id`, and a final `GET /todos` returning `[]`
3. Open `public-deploy-smoke.stdout.log`.
4. Expected:
   - the smoke log reports health readiness and CRUD success through the public URL
   - the log does not leak `DATABASE_URL`

### 5. The assembled retained bundle stays self-contained and fail-closed

1. Run `DATABASE_URL=<redacted-local-admin-url> bash scripts/verify-m054-s01.sh`.
2. Open `.tmp/m054-s01/verify/phase-report.txt`.
3. Open `.tmp/m054-s01/verify/latest-proof-bundle.txt`.
4. Open the retained bundle's `retained-m054-s01-artifacts.manifest.txt` and `retained-staged-bundle.manifest.json`.
5. Expected:
   - all wrapper phases pass, including `m054-s01-public-ingress-e2e`, `m054-s01-retain-artifacts`, `m054-s01-retain-staged-bundle`, `m054-s01-redaction-drift`, and `m054-s01-bundle-shape`
   - the retained bundle contains copied runtime artifacts plus a copied staged deploy bundle
   - `retained-staged-bundle/` contains exactly `todo-postgres`, `todo-postgres.up.sql`, `apply-deploy-migrations.sh`, and `deploy-smoke.sh`
   - the retained proof bundle stays under `.tmp/m054-s01/proof-bundles/`

## Edge Cases

### Invalid public-ingress config and truncated backend responses fail closed

1. Run `cargo test -p meshc --test e2e_m054_s01 m054_s01_public_ingress_fails_closed_on_invalid_target_config_and_truncated_backend_response -- --nocapture`.
2. Expected:
   - an empty target list fails with the explicit `public ingress requires at least one backend target` message
   - a truncated backend response is retained as a `502` with `truncated response body` in the error surface and ingress record

### Non-JSON backend output and malformed continuity summaries fail closed

1. Run `cargo test -p meshc --test e2e_m054_s01 m054_s01_public_json_and_continuity_summary_fail_closed_on_non_json_and_missing_route_fields -- --nocapture`.
2. Expected:
   - a non-JSON `GET /todos` response fails with the explicit JSON-body diagnostic and retains the raw HTTP/body artifacts
   - a synthetic continuity record missing `ingress_node` fails with the explicit missing-field panic instead of silently producing a partial summary

## Failure Signals

- `scripts/verify-m054-s01.sh` exits non-zero or stops before `m054-s01-bundle-shape`.
- `.tmp/m054-s01/verify/latest-proof-bundle.txt` is missing, empty, or points outside `.tmp/m054-s01/proof-bundles/`.
- `selected-route.summary.json` is missing any of `ingress_node`, `owner_node`, `replica_node`, or `execution_node`, or no longer shows the first request as standby-first.
- `public-ingress.requests.json` no longer records the first request hitting standby or no longer captures the follow-on CRUD flow.
- retained bundle scans detect a `DATABASE_URL` leak.

## Requirements Proved By This UAT

- R123 is advanced, but not fully validated: the serious starter now proves the one-public-URL server-side-first story on live runtime surfaces and aligned starter/verifier wording, while S02 still owns direct request correlation and S03 still owns broader public-claim closeout.

## Not Proven By This UAT

- a direct runtime-owned request-correlation surface that removes the before/after continuity diff
- broader homepage / distributed-proof / landing copy alignment
- frontend-aware node selection or any stronger client-topology-aware load-balancing claim
