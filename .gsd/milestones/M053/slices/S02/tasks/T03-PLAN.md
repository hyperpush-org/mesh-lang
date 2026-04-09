---
estimated_steps: 4
estimated_files: 3
skills_used:
  - bash-scripting
  - test
---

# T03: Publish the retained S02 starter failover verifier surface

**Slice:** S02 — Generated Postgres starter proves clustered failover truth
**Milestone:** M053

## Description

Wrap the new S02 proof into one fail-closed retained verifier surface that later hosted-chain work can call without reconstructing the failover setup. The executor should replay the S01 wrapper first, run the authoritative S02 Rust rail, copy only the fresh `.tmp/m053-s02/...` proof bundle into `.tmp/m053-s02/verify/`, and validate the retained bundle shape and redaction contract.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| prerequisite S01/S02 proof commands | Fail the wrapper immediately and preserve the command log instead of publishing stale evidence. | Kill the phase, record timeout context, and stop before copying old bundles. | Fail closed if a named test filter runs 0 tests or if the prerequisite wrapper/target emits malformed output. |
| retained bundle copy and manifest publication | Stop if the source bundle is missing, empty, or points at an old run; do not silently reuse prior evidence. | Treat a hung copy/archive step as artifact corruption and abort the wrapper. | Fail closed if bundle pointers, manifests, or required retained files do not match the copied directory. |
| secret-redaction and bundle-shape checks | Stop on the first leaked secret marker or missing required artifact path. | Treat a slow redaction scan as an artifact problem and abort. | Fail closed if required JSON/log paths are malformed or absent. |

## Load Profile

- **Shared resources**: `.tmp/m053-s02/verify/`, copied retained proof bundles, and prerequisite S01/S02 logs.
- **Per-operation cost**: one ordered replay of prerequisite commands plus one bundle-copy/shape/redaction audit.
- **10x breakpoint**: artifact-copy size and repeated full replays, not runtime throughput.

## Negative Tests

- **Malformed inputs**: missing `DATABASE_URL`, missing fresh `.tmp/m053-s02` bundle, malformed phase report, or stale bundle pointer.
- **Error paths**: S01 wrapper failure, S02 target failure, 0-test filter, missing copied artifact, or redaction drift.
- **Boundary conditions**: rerun with an existing `.tmp/m053-s02/verify/` tree, multiple fresh candidate bundles, and stale `latest-proof-bundle.txt` from a previous run.

## Steps

1. Create `scripts/verify-m053-s02.sh` to replay `bash scripts/verify-m053-s01.sh` before `cargo test -p meshc --test e2e_m053_s02 -- --nocapture`, with explicit phase logging, timeout handling, and named-test-count guards.
2. Snapshot the pre-run `.tmp/m053-s02` tree, copy only the fresh proof bundle(s) created by the S02 replay into `.tmp/m053-s02/verify/`, and publish `status.txt`, `current-phase.txt`, `phase-report.txt`, and `latest-proof-bundle.txt`.
3. Add retained-bundle checks for the specific starter-owned failover artifacts: per-node logs, before/after HTTP snapshots, cluster status/continuity/diagnostics JSON, scenario metadata, and redaction markers.
4. Keep the wrapper scoped to generated Postgres starter failover truth; do not pull packages-site or public-doc/Fly work forward from S03/S04.

## Must-Haves

- [ ] `bash scripts/verify-m053-s02.sh` is the single retained S02 failover proof surface and it replays S01 first.
- [ ] The verifier fail-closes on prerequisite failures, 0-test filters, stale/malformed bundle pointers, missing retained artifacts, or secret leakage.
- [ ] `.tmp/m053-s02/verify/` contains stable phase/status/pointer files and a copied retained proof bundle that downstream hosted-chain work can consume directly.

## Inputs

- `scripts/verify-m053-s01.sh`
- `compiler/meshc/tests/e2e_m053_s01.rs`
- `compiler/meshc/tests/e2e_m053_s02.rs`

## Expected Output

- `scripts/verify-m053-s02.sh`

## Verification

DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash scripts/verify-m053-s02.sh

## Observability Impact

- Signals added/changed: phase-report entries, status/current-phase markers, copied proof-bundle manifest, redaction-scan output, and a retained bundle pointer under `.tmp/m053-s02/verify/`.
- How a future agent inspects this: read `.tmp/m053-s02/verify/phase-report.txt`, then follow `latest-proof-bundle.txt` into the copied failover bundle.
- Failure state exposed: failing phase log, stale-pointer mismatch, missing copied artifact path, 0-test filter evidence, and redaction drift.
