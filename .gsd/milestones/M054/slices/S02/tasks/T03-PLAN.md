---
estimated_steps: 4
estimated_files: 4
skills_used:
  - test
---

# T03: Align the starter contract and assembled verifier around direct correlation

**Slice:** S02 — Clustered HTTP request correlation
**Milestone:** M054

## Description

Once the runtime seam and serious-starter rail are green, update the generated Postgres starter guidance and slice wrapper so operators can go from the clustered HTTP response header straight to `meshc cluster continuity <node> <request-key> --json` without overclaiming frontend-aware routing. Keep the public contract bounded, delegate S01 instead of rewriting it, and fail closed on retained bundle drift.

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

## Verification

- `node --test scripts/tests/verify-m054-s02-contract.test.mjs`
- `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash scripts/verify-m054-s02.sh`

## Observability Impact

- Signals added/changed: retained S02 verifier phase log, bundle pointer, and direct-correlation artifact manifest.
- How a future agent inspects this: start with `.tmp/m054-s02/verify/phase-report.txt` and `latest-proof-bundle.txt`, then open the retained S02 bundle and copied S01 pointer if delegation drift is suspected.
- Failure state exposed: stale README/verifier wording, missing retained bundle markers, S01 delegation breakage, and redaction drift.

## Inputs

- `compiler/mesh-pkg/src/scaffold.rs` — generator-owned Postgres starter README source and scaffold contract assertions.
- `examples/todo-postgres/README.md` — committed example README that must be re-materialized rather than hand-edited independently.
- `scripts/verify-m054-s01.sh` — existing assembled S01 wrapper that S02 should delegate instead of rewriting.
- `compiler/meshc/tests/e2e_m054_s02.rs` — new direct-correlation rail that the wrapper needs to replay.

## Expected Output

- `compiler/mesh-pkg/src/scaffold.rs` — updated starter guidance and generator-owned contract assertions.
- `examples/todo-postgres/README.md` — re-materialized committed Postgres starter README with direct-correlation guidance.
- `scripts/verify-m054-s02.sh` — assembled S02 verifier with retained bundle or pointer checks and S01 delegation.
- `scripts/tests/verify-m054-s02-contract.test.mjs` — cheap contract test for README/verifier wording and retained-bundle markers.
