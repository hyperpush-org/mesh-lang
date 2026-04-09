# S02: Clustered HTTP request correlation

**Goal:** Expose a runtime-owned request-correlation seam for clustered HTTP, prove it on the low-level route rail and the serious one-public-URL starter, and align the starter/verifier around that bounded operator-facing lookup flow.
**Demo:** After this: After this: a single clustered HTTP request can be traced directly to one continuity record through runtime-owned correlation output instead of before/after continuity diffing.

## Tasks
- [x] **T01: Added a runtime-owned clustered HTTP correlation header and switched the low-level route proof to direct continuity lookup.** — Land the single runtime seam this slice depends on. The clustered HTTP server already generates the continuity request key before dispatch; expose that key as an operator-facing response header on both successful and runtime-generated rejection responses, preserve any handler-supplied headers, and prove direct continuity lookup at the lower clustered-route rail without widening routing semantics.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| clustered HTTP response construction / handler header merge | Fail closed and keep the existing route response body/status; do not ship a path that drops app headers or hides rejection reasons. | Treat hung route execution or direct-record polling as a runtime regression, archive the last response/continuity artifacts, and stop. | Reject responses missing the correlation header or returning header values that do not match continuity lookup. |
| continuity direct lookup via existing `meshc cluster` surfaces | Stop the low-level rail if the returned header value cannot retrieve exactly one record on both nodes. | Bound lookup polling and archive the last continuity JSON rather than falling back to before/after diff. | Fail closed if the returned record is missing `request_key`, `declared_handler_runtime_name`, `phase`, or `result`. |

## Load Profile

- **Shared resources**: continuity registry, clustered-route request sequence, response-header map allocation, and the dual-node route proof harness.
- **Per-operation cost**: one extra response header and one direct continuity lookup per verified request; runtime work itself stays the same.
- **10x breakpoint**: continuity record churn and proof polling before the header injection overhead matters.

## Negative Tests

- **Malformed inputs**: empty runtime name/payload identity generation stays rejected, and malformed response/header parsing in the e2e fails closed.
- **Error paths**: unsupported replication count `503` still returns the correlation header plus a rejected continuity record.
- **Boundary conditions**: app-supplied headers survive alongside the new header, and repeated same-runtime requests get unique keys without exact numeric suffix assertions.

## Steps

1. Update `compiler/mesh-rt/src/http/server.rs` so `clustered_route_response_from_request(...)` attaches `X-Mesh-Continuity-Request-Key` after request-key generation on success and on runtime-generated rejection responses where a request key exists.
2. Add or extend `mesh-rt` unit coverage in `compiler/mesh-rt/src/http/server.rs` for header injection, handler-header preservation, and rejection-path correlation without asserting specific numeric request-id suffixes.
3. Update `compiler/meshc/tests/e2e_m047_s07.rs` to read the response header from raw HTTP, use it for direct `meshc cluster continuity <node> <request-key> --json` lookups on both nodes, and keep the old diff-helper unit coverage as separate guardrails.
4. Retain raw HTTP plus continuity artifacts so a missing or mismatched header is diagnosable without reopening the runtime manually.

## Must-Haves

- [ ] Clustered HTTP success responses include `X-Mesh-Continuity-Request-Key`.
- [ ] Runtime-generated `503` rejection responses for clustered routes still include the same correlation key when the runtime created a continuity record.
- [ ] Existing handler headers survive beside the new correlation header.
- [ ] The low-level clustered-route e2e uses the response header to fetch the single continuity record directly on both nodes.
  - Estimate: 1 context window
  - Files: compiler/mesh-rt/src/http/server.rs, compiler/meshc/tests/e2e_m047_s07.rs, compiler/meshc/src/cluster.rs
  - Verify: - `cargo test -p mesh-rt m054_s02_ -- --nocapture`
- `cargo test -p meshc --test e2e_m047_s07 -- --nocapture`
- [x] **T02: Added a serious-starter public-ingress rail that follows one runtime header straight to one continuity record pair and one diagnostics pair.** — Add the serious-starter rail that consumes the runtime header instead of continuity diffing. Keep S01 intact as the historical diff-based proof, but add a new S02 e2e and small support helpers that extract the selected public response header, jump straight into primary/standby direct continuity and diagnostics lookups, and retain a self-contained direct-correlation bundle.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| public ingress harness + staged two-node Postgres starter | Stop on the first staged boot, health, or ingress-forwarding failure and preserve runtime config, node logs, and the last ingress snapshot. | Time-box cluster convergence and selected-request waits, then archive the last public request transcript and status/diagnostics output instead of adding sleeps. | Fail closed if the selected public response omits the correlation header or if the retained raw HTTP transcript cannot be parsed deterministically. |
| direct continuity + diagnostics lookup on both nodes | Abort if the primary/standby direct lookups disagree about the request key or completed record. | Treat polling exhaustion as a proof-surface regression and retain the final lookup JSON on both nodes. | Fail closed if the record or diagnostics entries for the returned key omit required fields or disappear on one node. |

## Load Profile

- **Shared resources**: public ingress harness, two-node staged starter, disposable PostgreSQL database, and retained bundles under `.tmp/m054-s02/`.
- **Per-operation cost**: one staged deploy/apply flow, one selected public `GET /todos`, and paired direct continuity plus diagnostics polls on both nodes.
- **10x breakpoint**: cluster convergence, diagnostics polling, and database startup time before any response-header parsing cost matters.

## Negative Tests

- **Malformed inputs**: public responses missing `X-Mesh-Continuity-Request-Key`, empty header values, or malformed raw HTTP transcripts.
- **Error paths**: primary/standby direct lookup disagreement, diagnostics missing entries for the returned key, and staged runtime readiness failures.
- **Boundary conditions**: the first public standby-targeted `GET /todos` after startup, empty-list response before create, and one public request mapping to exactly one completed continuity record on both nodes.

## Steps

1. Extend `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs` or `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs` with a small fail-closed HTTP response-header helper so the staged/public-ingress rails can read `X-Mesh-Continuity-Request-Key` from saved raw responses without inventing a second HTTP abstraction.
2. Add `compiler/meshc/tests/e2e_m054_s02.rs` as a new rail that stages the serious Postgres starter behind the existing public ingress harness, extracts the selected public response header, and goes directly to `wait_for_continuity_record_completed_pair(...)` plus `wait_for_request_diagnostics_pair(...)`.
3. Retain S02-specific artifacts that show the raw public response, extracted request-key JSON, direct primary/standby continuity record JSON, and request-scoped diagnostics entries, while leaving S01's diff-based retained bundle shape untouched.
4. Fail closed on missing or mismatched headers or lookup drift instead of silently diffing continuity lists again.

## Must-Haves

- [ ] The serious starter gets a dedicated S02 e2e rail instead of mutating S01's retained diff-based proof.
- [ ] The S02 rail extracts one request key from the selected public response and uses it for direct continuity lookup on both nodes.
- [ ] The retained S02 bundle includes raw public HTTP, extracted correlation data, direct record JSON, and request-scoped diagnostics JSON for the same request.
- [ ] Missing or malformed response headers or primary/standby lookup drift fail closed.
  - Estimate: 1 context window
  - Files: compiler/meshc/tests/e2e_m054_s02.rs, compiler/meshc/tests/support/m053_todo_postgres_deploy.rs, compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs, compiler/meshc/tests/support/m054_public_ingress.rs
  - Verify: - `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_m054_s02 -- --nocapture`
- The new `.tmp/m054-s02/...` bundle contains `public-selected-list.http`, an extracted request-key artifact, and direct primary/standby continuity-record artifacts for the same key.
- [x] **T03: Added direct-correlation starter guidance and an assembled M054 S02 verifier that delegates S01 and retains a self-contained proof bundle.** — Once the runtime seam and serious-starter rail are green, update the generated Postgres starter guidance and slice wrapper so operators can go from the clustered HTTP response header straight to `meshc cluster continuity <node> <request-key> --json` without overclaiming frontend-aware routing. Keep the public contract bounded, delegate S01 instead of rewriting it, and fail closed on retained bundle drift.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| scaffold materialization + committed example parity | Fail closed if the generated README and committed example diverge; re-materialize instead of hand-patching the committed example only. | Treat a hung materialization or stale example-parity rail as generator drift and stop before editing public copy in multiple places. | Reject wording that drops the response-header lookup, removes the startup-list caveat, or widens the contract back toward frontend-aware routing. |
| assembled verifier + retained S01/S02 bundles | Stop on the first missing delegated phase or bundle pointer and archive the last phase log; do not claim green proof from partial local state. | Bound wrapper waits and copy phases, then archive the last verifier phase log and bundle manifests instead of adding sleeps. | Fail closed if the retained bundle is missing the direct-correlation artifacts, copied S01 delegation markers, or redaction guards. |

## Load Profile

- **Shared resources**: scaffold generator, committed example README, and retained verifier bundles under `.tmp/m054-s01/` and `.tmp/m054-s02/`.
- **Per-operation cost**: one README materialization check, one contract test, and one assembled shell replay that copies retained bundles.
- **10x breakpoint**: repeated full wrapper replays and retained-bundle copying, not docs parsing itself.

## Negative Tests

- **Malformed inputs**: mutated README/verifier wording, missing retained bundle pointers, and missing response-header markers or startup-list caveats.
- **Error paths**: S01 delegation missing, S02 bundle-shape drift, and stale docs continuing to claim continuity-list diffing or frontend-aware routing.
- **Boundary conditions**: README teaches direct response-header correlation for clustered GETs while still reserving continuity-list discovery for startup records and general manual investigation.

## Steps

1. Update `compiler/mesh-pkg/src/scaffold.rs` so the generated Postgres starter README explains the direct clustered HTTP response-header -> `meshc cluster continuity <node> <request-key> --json` flow for operator/debug use, while keeping startup-record discovery on the list form and leaving broader docs-site cleanup to S03.
2. Re-materialize `examples/todo-postgres/README.md` from the scaffold template and adjust any generator-owned assertions in `compiler/mesh-pkg/src/scaffold.rs` that pin the old wording.
3. Add `scripts/verify-m054-s02.sh` and `scripts/tests/verify-m054-s02-contract.test.mjs` so the assembled rail delegates S01, replays the new S02 e2e, republishes a self-contained retained S02 bundle or pointer, and fails closed on docs, verifier, bundle, or redaction drift.
4. Keep the contract bounded: no client-aware routing promise, no sticky-session claim, no Fly-specific product contract, and no mutation of S01's retained diff-based bundle.

## Must-Haves

- [ ] Generated and committed Postgres starter README surfaces teach direct request-key correlation for clustered HTTP responses as an operator/debug seam.
- [ ] The README still reserves continuity-list discovery for startup records or general manual investigation and does not widen the product claim.
- [ ] `scripts/verify-m054-s02.sh` delegates S01, runs the new S02 rail, and republishes a retained S02 bundle or pointer fail-closed.
- [ ] Contract tests catch stale wording, missing bundle markers, or overclaiming docs/verifier copy.
  - Estimate: 1 context window
  - Files: compiler/mesh-pkg/src/scaffold.rs, examples/todo-postgres/README.md, scripts/verify-m054-s02.sh, scripts/tests/verify-m054-s02-contract.test.mjs
  - Verify: - `node --test scripts/tests/verify-m054-s02-contract.test.mjs`
- `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash scripts/verify-m054-s02.sh`
