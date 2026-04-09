# S02: Clustered HTTP request correlation — UAT

**Milestone:** M054
**Written:** 2026-04-06T15:55:45.085Z

# S02: Clustered HTTP request correlation — UAT

**Milestone:** M054
**Written:** 2026-04-05

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: This slice shipped a runtime-owned observability seam plus a live staged-starter proof and a retained assembled verifier. A truthful UAT therefore needs both command-level runtime replay and artifact inspection for the exact request key, direct continuity record, and request-scoped diagnostics.

## Preconditions

- Docker is running.
- A disposable PostgreSQL admin URL is available in `DATABASE_URL` (the local closeout used a throwaway Docker Postgres instance).
- The repo can build local Rust targets from the current worktree.
- No one is relying on the retained `.tmp/m054-s02/` directory while the verifier is replaying.

## Smoke Test

1. Export a disposable `DATABASE_URL`.
2. Run `cargo test -p meshc --test e2e_m054_s02 -- --nocapture`.
3. **Expected:** `running 3 tests` appears, `m054_s02_staged_postgres_public_ingress_directly_correlates_selected_get_todos_request ... ok` passes, and a fresh `.tmp/m054-s02/staged-postgres-public-ingress-direct-correlation-*/public-selected-list.request-key.json` artifact exists.

## Test Cases

### 1. Low-level clustered route responses expose a direct continuity lookup key

1. Run `cargo test -p mesh-rt m054_s02_ -- --nocapture`.
2. Run `cargo test -p meshc --test e2e_m047_s07 -- --nocapture`.
3. **Expected:** the `mesh-rt` unit rails prove successful clustered responses preserve app headers while adding `X-Mesh-Continuity-Request-Key`, runtime-generated clustered 503s keep the same header when a continuity record exists, and the authoritative clustered-route e2e finishes green with direct continuity lookup by the returned header on both nodes.

### 2. One public standby-routed `GET /todos` maps directly to one mirrored continuity record pair

1. Export a disposable `DATABASE_URL`.
2. Run `cargo test -p meshc --test e2e_m054_s02 -- --nocapture`.
3. Open the newest `.tmp/m054-s02/staged-postgres-public-ingress-direct-correlation-*/public-selected-list.request-summary.json`.
4. Open the sibling `selected-route.summary.json`.
5. **Expected:** the selected public request targets `standby`, the raw HTTP response contains `X-Mesh-Continuity-Request-Key`, and `selected-route.summary.json` reports the same `request_key` with `phase=completed`, `result=succeeded`, `ingress_node` on standby, and `owner_node`/`execution_node` on primary.

### 3. Primary and standby both resolve the same request key directly and retain matching diagnostics

1. In the same fresh `.tmp/m054-s02/staged-postgres-public-ingress-direct-correlation-*` directory, open `public-selected-list.request-key.json`.
2. Open `selected-route-direct-primary-record.json` and `selected-route-direct-standby-record.json`.
3. Open `selected-route-direct-primary-diagnostics.json` and `selected-route-direct-standby-diagnostics.json`.
4. **Expected:** the request-key artifact names `X-Mesh-Continuity-Request-Key` and the same request key found in the raw response; both direct record files agree on `request_key`, `declared_handler_runtime_name`, `phase`, and `result`; and both diagnostics files retain request-scoped entries for that same key instead of forcing a before/after continuity-list diff.

### 4. The assembled verifier republishes one self-contained proof bundle and delegates S01 unchanged

1. Export a disposable `DATABASE_URL`.
2. Run `bash scripts/verify-m054-s02.sh`.
3. Read `.tmp/m054-s02/verify/status.txt`, `.tmp/m054-s02/verify/current-phase.txt`, and `.tmp/m054-s02/verify/latest-proof-bundle.txt`.
4. Inspect the retained bundle path from `latest-proof-bundle.txt`.
5. **Expected:** `status.txt` is `ok`, `current-phase.txt` is `complete`, the retained bundle contains `retained-m054-s01-verify/` unchanged, includes `retained-m054-s02-artifacts.manifest.txt`, copies the staged starter bundle into `retained-staged-bundle/`, and includes `verify-m054-s02.sh` plus `verify-m054-s02-contract.test.mjs` for replay/debugging.

## Edge Cases

### Missing, empty, duplicate, or malformed response headers fail closed

1. Run `cargo test -p meshc --test e2e_m054_s02 m054_s02_response_header_helper_fails_closed_on_malformed_missing_empty_and_duplicate_headers -- --nocapture`.
2. **Expected:** the test passes because the helper rejects all malformed raw-response cases with explicit failure messages instead of silently falling back to body parsing or continuity-list diffing.

### Primary/standby lookup drift fails closed

1. Run `cargo test -p meshc --test e2e_m054_s02 m054_s02_route_continuity_summary_fails_closed_on_primary_standby_drift -- --nocapture`.
2. **Expected:** the test passes because the route-correlation summary helper aborts on `request_key` drift between primary and standby records.

## Failure Signals

- The selected clustered HTTP response is missing `X-Mesh-Continuity-Request-Key`, repeats it, or returns it empty.
- `selected-route-direct-primary-record.json` and `selected-route-direct-standby-record.json` disagree on `request_key`, `declared_handler_runtime_name`, `phase`, or `result`.
- Request-scoped diagnostics do not retain the same request key on both nodes.
- `scripts/verify-m054-s02.sh` stops before `complete`, loses the retained S01 verify tree, or publishes a bundle without `public-selected-list.request-key.{txt,json}` and `selected-route-direct-{primary,standby}-{record,diagnostics}.json`.

## Requirements Proved By This UAT

- R123 — Advances the load-balancing operability story by proving one public clustered HTTP request can be correlated directly to continuity and diagnostics truth on both nodes through a runtime-owned header-to-CLI seam.

## Not Proven By This UAT

- Sticky-session guarantees, frontend-aware routing, or Fly as the product contract.
- A response-header seam for startup work; startup/manual discovery still uses continuity lists and diagnostics.
- Final public-docs/hard-guard alignment for the broader load-balancing story; that remains S03 work.

## Notes for Tester

Use the retained S02 artifacts as the first debug surface. `public-selected-list.http` plus `public-selected-list.request-key.{txt,json}` prove the handoff from HTTP response to request key, and `selected-route-direct-{primary,standby}-{record,diagnostics}.json` are the authoritative lookup surfaces for the same request. If those drift, do not reintroduce continuity-list diffing as a substitute proof.
