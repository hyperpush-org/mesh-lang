---
estimated_steps: 4
estimated_files: 5
skills_used:
  - github-workflows
  - test
---

# T01: Wire a reusable hosted starter failover proof into authoritative main/tag workflows

Add a dedicated GitHub Actions reusable workflow for `bash scripts/verify-m053-s02.sh` so the serious generated Postgres starter proof runs on hosted CI with a runner-local Postgres service instead of being squeezed into the M034 publish proof lane. Then wire that reusable workflow into both authoritative mainline verification and tag release gating, and lock the topology with a local workflow contract test.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| reusable workflow + runner-local Postgres service | Fail the reusable workflow immediately and upload `.tmp/m053-s02/**`; do not fall back to SQLite or a missing DB. | Treat service-start or S02 replay overruns as contract drift and stop before later jobs consume false-green output. | Fail closed if the composed `DATABASE_URL`, artifact paths, or job wiring are malformed. |
| authoritative-verification / release caller graph | Fail the local contract sweep when callers skip the new reusable workflow, widen permissions, or change release needs. | Treat missing or reordered `needs` edges as workflow drift rather than letting release proceed partially gated. | Fail closed if YAML changes rename the required jobs or steps without intentionally updating the verifier. |
| existing authoritative live-proof contract | Keep the M034 live proof lane intact; if the new starter lane breaks those expectations, stop and update the verifier deliberately instead of weakening it. | Use the timeout mismatch as justification for a separate workflow, not for stretching the publish proof budget blindly. | Fail closed if caller/release job sets no longer match the explicit contract after the new lane is added. |

## Load Profile

- **Shared resources**: GitHub-hosted Ubuntu runner, Postgres service container, Cargo/LLVM caches, and `.tmp/m053-s02/**` diagnostics artifacts.
- **Per-operation cost**: one full `bash scripts/verify-m053-s02.sh` replay plus workflow YAML parsing in the local contract sweep.
- **10x breakpoint**: runner time budget and service-container startup, not starter data volume.

## Negative Tests

- **Malformed inputs**: missing reusable-workflow entrypoint, bad service env, renamed job ids, or missing diagnostics path.
- **Error paths**: Postgres service never becomes ready, the S02 verifier fails, diagnostics upload path is wrong, or the release job loses the new prerequisite.
- **Boundary conditions**: fork PR path vs trusted push/tag path, main push vs tag invocation, and reusable workflow step-name drift.

## Steps

1. Add `.github/workflows/authoritative-starter-failover-proof.yml` as a dedicated reusable workflow that provisions a Postgres service container, exports a runner-local `DATABASE_URL`, runs `bash scripts/verify-m053-s02.sh`, and uploads `.tmp/m053-s02/**` diagnostics on failure.
2. Wire the new reusable workflow into `.github/workflows/authoritative-verification.yml` and `.github/workflows/release.yml` so mainline and tag pipelines both require the hosted starter failover proof without widening the existing authoritative live proof contract or making Fly part of the product path.
3. Update `scripts/verify-m034-s02-workflows.sh` to pin the new reusable workflow file, required caller/release job sets, step names, timeout/permissions shape, and release dependencies fail-closed.
4. Create the initial `scripts/tests/verify-m053-s03-contract.test.mjs` coverage for the workflow topology: new reusable workflow file, Postgres-service shape, `bash scripts/verify-m053-s02.sh` entrypoint, diagnostics upload, and caller/release references.

## Must-Haves

- [ ] Hosted CI has a dedicated reusable starter-proof lane that runs `bash scripts/verify-m053-s02.sh` with runner-local Postgres and failure artifact upload.
- [ ] `.github/workflows/authoritative-verification.yml` and `.github/workflows/release.yml` both require the new hosted starter-proof lane while keeping the existing M034 live proof intact.
- [ ] Local workflow contracts fail when the reusable workflow, required job names, or diagnostics/artifact surfaces drift.

## Verification

- `bash scripts/verify-m034-s02-workflows.sh`
- `node --test scripts/tests/verify-m053-s03-contract.test.mjs`

## Observability Impact

- Signals added/changed: new hosted starter-proof job conclusion, uploaded `.tmp/m053-s02/**` diagnostics, and explicit caller/release job names for the hosted lane.
- How a future agent inspects this: run `bash scripts/verify-m034-s02-workflows.sh`, then inspect the hosted workflow YAML and failure artifact upload name/path.
- Failure state exposed: missing reusable workflow reference, missing Postgres service, starter-proof timeout, or dropped release prerequisite.

## Inputs

- `.github/workflows/authoritative-live-proof.yml`
- `.github/workflows/authoritative-verification.yml`
- `.github/workflows/release.yml`
- `scripts/verify-m034-s02-workflows.sh`
- `scripts/verify-m053-s02.sh`

## Expected Output

- `.github/workflows/authoritative-starter-failover-proof.yml`
- `.github/workflows/authoritative-verification.yml`
- `.github/workflows/release.yml`
- `scripts/verify-m034-s02-workflows.sh`
- `scripts/tests/verify-m053-s03-contract.test.mjs`

## Verification

bash scripts/verify-m034-s02-workflows.sh && node --test scripts/tests/verify-m053-s03-contract.test.mjs

## Observability Impact

- Signals added/changed: hosted starter-proof workflow conclusion, diagnostics upload artifacts, and explicit job/step names in the main/tag workflow graph.
- How a future agent inspects this: run `bash scripts/verify-m034-s02-workflows.sh` and inspect the reusable workflow file plus uploaded-artifact settings.
- Failure state exposed: missing Postgres service, missing artifact upload, dropped caller reference, or release graph drift.
