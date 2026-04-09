# S05: Hosted workflow evidence closes the starter/packages contract — UAT

**Milestone:** M053
**Written:** 2026-04-06T01:16:54.172Z

# S05 UAT — Hosted workflow evidence closer

## Preconditions
- Repo checkout contains the S05 closer changes (`scripts/verify-m053-s01.sh`, `scripts/verify-m053-s02.sh`, `compiler/meshc/tests/e2e_m053_s01.rs`, `compiler/meshc/tests/e2e_m053_s02.rs`).
- `GH_TOKEN` is available for hosted read checks.
- A disposable Postgres admin URL is reachable at `postgres://postgres:postgres@127.0.0.1:5432/postgres` for the local failover rail.

## Test Case 1 — Local workflow and hosted-contract rails still enforce the S03/S05 contract
1. Run `bash scripts/verify-m034-s02-workflows.sh`.
   - Expected: exits 0 and reports all hosted workflows still wire the starter failover proof into authoritative main/tag gates.
2. Run `node --test scripts/tests/verify-m053-s03-contract.test.mjs`.
   - Expected: exits 0 with all tests passing.

## Test Case 2 — Local staged failover rail is green after the wrapper hardening
1. Run `DATABASE_URL='postgres://postgres:postgres@127.0.0.1:5432/postgres' bash scripts/verify-m053-s02.sh`.
   - Expected: exits 0.
2. Inspect `.tmp/m053-s02/verify/status.txt` and `.tmp/m053-s02/verify/current-phase.txt`.
   - Expected: `status.txt` contains `ok`; `current-phase.txt` contains `complete`.
3. Inspect `.tmp/m053-s02/verify/phase-report.txt`.
   - Expected: both `m053-s01-contract` and `m053-s02-failover-e2e` are marked `passed`.
4. Inspect `.tmp/m053-s02/verify/upstream-m053-s01-verify/`.
   - Expected: retained nested S01 verifier logs exist so a future hosted red can be diagnosed from this bundle.

## Test Case 3 — Hosted verifier now refreshes the exact remaining blocker instead of stale or opaque evidence
1. Run `bash scripts/verify-m053-s03.sh`.
   - Expected: exits non-zero.
2. Inspect `.tmp/m053-s03/verify/status.txt` and `.tmp/m053-s03/verify/current-phase.txt`.
   - Expected: `status.txt` is `failed`; `current-phase.txt` is `remote-evidence`.
3. Inspect `.tmp/m053-s03/verify/remote-runs.json`.
   - Expected:
     - `deploy-services.yml` has `status: ok` and `observedHeadSha: 314bbac88b171388b04072a97f22be0bca4882aa`.
     - `authoritative-verification.yml` has `status: failed` on the same SHA.
     - `release.yml` fails freshness because `refs/tags/v0.1.0^{}` is still missing.

## Test Case 4 — Hosted failure diagnostics preserve the real failover blocker
1. Inspect `.tmp/m053-s05/remote-auth-24014506220/artifacts/authoritative-starter-failover-proof-diagnostics/verify/m053-s01-contract.log`.
   - Expected: it ends with `verify-m053-s01: ok`.
2. Inspect `.tmp/m053-s05/remote-auth-24014506220/artifacts/authoritative-starter-failover-proof-diagnostics/verify/m053-s02-failover-e2e.log`.
   - Expected: the failing test is `m053_s02_staged_postgres_failover_proves_clustered_http_and_runtime_recovery`, not the S01 contract.
3. Inspect `.tmp/m053-s05/remote-auth-24014506220/artifacts/authoritative-starter-failover-proof-diagnostics/staged-postgres-failover-runtime-truth-1775437858534365102/post-kill-status-standby.timeout.txt`.
   - Expected: the archived standby status still shows `cluster_role=standby`, `promotion_epoch=0`, `replication_health=healthy`.
4. Inspect `.tmp/m053-s05/remote-auth-24014506220/log-failed.txt`.
   - Expected: the hosted log contains `automatic_promotion_rejected:no_mirrored_state`.

## Test Case 5 — Rollout artifacts name the exact remaining recovery work
1. Inspect `.tmp/m053-s05/rollout/main-shipped-sha.txt`.
   - Expected: it contains `314bbac88b171388b04072a97f22be0bca4882aa`.
2. Inspect `.tmp/m053-s05/rollout/main-workflows.json`.
   - Expected: it records `deploy-services.yml` green and `authoritative-verification.yml` red on that same SHA.
3. Inspect `.tmp/m053-s05/rollout/release-workflow.json` and `.tmp/m053-s05/rollout/final-blocker.md`.
   - Expected: both files call out the missing peeled tag ref and the need to fix authoritative starter failover proof before rerolling `v0.1.0`.

## Edge Cases
- If `GH_TOKEN` is absent, `bash scripts/verify-m053-s03.sh` must fail in `gh-preflight` before remote queries start.
- If `deploy-services.yml` turns green but `authoritative-verification.yml` stays red, the hosted contract remains open and `scripts/verify-m053-s03.sh` must keep failing closed.
- If `authoritative-verification.yml` turns green but `refs/tags/v0.1.0^{}` is still absent, `release.yml` freshness must still block the final closeout.
