---
estimated_steps: 4
estimated_files: 3
skills_used:
  - bash-scripting
  - test
---

# T03: Publish a retained starter deploy verifier surface for later hosted-chain wiring

**Slice:** S01 — Generated Postgres starter owns staged deploy truth
**Milestone:** M053

## Description

Wrap the starter deploy proof into one fail-closed verifier surface that later hosted-chain work can call without re-implementing starter logic. The executor should replay the generator/example parity rail and the staged deploy e2e, copy the retained proof bundle under `.tmp/m053-s01/verify/`, and publish pointer/status/phase markers so S03 can wire this starter-owned command into the normal release/deploy chain alongside packages-site checks.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| prerequisite test commands | Fail the wrapper immediately and preserve the command log instead of continuing with stale evidence. | Kill the phase, record timeout context, and leave the last successful artifact pointer intact for debugging. | Fail closed if a named test filter runs 0 tests or emits malformed output. |
| retained bundle copy/pointer publishing | Stop if the source bundle is missing or under the repo root and log the bundle-shape mismatch. | Treat a hung copy/archive step as artifact corruption and abort the wrapper. | Fail closed if pointer, manifest, or status files do not match the retained bundle directory. |

## Load Profile

- **Shared resources**: `.tmp/m053-s01/verify/`, copied retained bundle directories, and prerequisite test logs.
- **Per-operation cost**: one scripted replay of prerequisite commands plus one retained bundle copy/shape check.
- **10x breakpoint**: artifact copy size and repeated full replays, not runtime throughput.

## Negative Tests

- **Malformed inputs**: missing `DATABASE_URL`, missing retained bundle path, malformed phase report, or bundle pointer under the repo root.
- **Error paths**: prerequisite command failure, 0-test filter, missing copied artifact, or leaked secret in retained logs.
- **Boundary conditions**: rerun with an existing `.tmp/m053-s01/verify/` tree, stale `latest-proof-bundle.txt`, and partial retained bundle copies.

## Steps

1. Create `scripts/verify-m053-s01.sh` to run the Postgres scaffold rail, example-parity rail, and staged deploy e2e in a fixed order with explicit phase logging and 0-test guards where needed.
2. Copy the retained staged-deploy artifact bundle into `.tmp/m053-s01/verify/`, publish `status.txt`, `current-phase.txt`, `phase-report.txt`, and `latest-proof-bundle.txt`, and fail closed if the copied bundle shape or redaction markers drift.
3. Keep the wrapper scoped to the starter deploy truth; do not pull packages-site or Fly-public-docs work forward from later slices.
4. Leave the verify surface ready for later CI/deploy-chain wiring by making the script the single starter-owned command S03 can call.

## Must-Haves

- [ ] `bash scripts/verify-m053-s01.sh` is the single retained starter deploy proof surface for S01.
- [ ] The verifier fail-closes on prerequisite failures, 0-test filters, missing bundle pointers, or redaction drift.
- [ ] `.tmp/m053-s01/verify/` contains stable phase/status/pointer files and a copied retained proof bundle for downstream slices.

## Verification

- `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash scripts/verify-m053-s01.sh`
- Confirm `.tmp/m053-s01/verify/status.txt` is `ok`, `current-phase.txt` is `complete`, and `latest-proof-bundle.txt` points at the copied retained bundle.

## Observability Impact

- Signals added/changed: phase-report entries, status/current-phase markers, bundle manifest, and retained bundle pointer under `.tmp/m053-s01/verify/`.
- How a future agent inspects this: read `.tmp/m053-s01/verify/phase-report.txt`, then follow `latest-proof-bundle.txt` into the copied starter evidence bundle.
- Failure state exposed: failing phase log, stale pointer mismatch, missing copied artifact, and secret-redaction drift.

## Inputs

- `compiler/meshc/tests/e2e_m053_s01.rs` — staged deploy replay that the wrapper must treat as the authoritative runtime rail.
- `scripts/tests/verify-m049-s03-materialize-examples.mjs` — generator/example parity command the wrapper must replay before bundle publication.
- `examples/todo-postgres/README.md` — generated public starter contract that the wrapper should not broaden into packages or Fly-docs scope.

## Expected Output

- `scripts/verify-m053-s01.sh` — fail-closed retained verifier that replays starter deploy truth and publishes `.tmp/m053-s01/verify/` markers.
