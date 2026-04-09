# S03: Hosted evidence chain fails on starter deploy or packages drift

**Goal:** Make packages-website and serious-starter deploy truth first-class hosted release/deploy gates instead of parallel side surfaces.
**Demo:** After this: Run the normal hosted release/deploy chain and show it fails when the serious starter deploy proof breaks or when packages-website deploy/public-surface checks drift, with workflow evidence that makes packages part of the same public contract.

## Tasks
- [x] **T01: Added a reusable hosted starter failover proof workflow and wired it into authoritative main/tag gates.** — Add a dedicated GitHub Actions reusable workflow for `bash scripts/verify-m053-s02.sh` so the serious generated Postgres starter proof runs on hosted CI with a runner-local Postgres service instead of being squeezed into the M034 publish proof lane. Then wire that reusable workflow into both authoritative mainline verification and tag release gating, and lock the topology with a local workflow contract test.

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
  - Estimate: 1h
  - Files: .github/workflows/authoritative-starter-failover-proof.yml, .github/workflows/authoritative-verification.yml, .github/workflows/release.yml, scripts/verify-m034-s02-workflows.sh, scripts/tests/verify-m053-s03-contract.test.mjs
  - Verify: bash scripts/verify-m034-s02-workflows.sh && node --test scripts/tests/verify-m053-s03-contract.test.mjs
- [x] **T02: Added a freshness-aware hosted verifier that couples starter proof with deploy-services packages/public-surface truth.** — Create the slice-owned hosted evidence verifier that turns starter failover proof and packages deploy/public-surface health into one explicit hosted contract. Reuse the proven `git ls-remote` + `gh run list/view` freshness pattern instead of scraping logs, and fail closed when deploy-services proof is satisfied only by a stale or tag-only run.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| GitHub CLI auth and remote workflow discovery | Fail the verifier immediately and preserve query logs; do not infer success from missing workflows or anonymous rate limits. | Treat `gh` timeouts and rate-limit stalls as incomplete hosted proof and stop with a failed phase. | Fail closed on malformed JSON or missing workflow fields instead of normalizing partial results. |
| `git ls-remote` expected-ref resolution | Stop when `main` or the current binary tag cannot resolve to a fresh remote SHA; do not fall back to local HEAD. | Time-box remote ref resolution and stop before comparing hosted runs against guessed SHAs. | Fail closed if peeled tags or head-SHA fields are absent or inconsistent. |
| deploy-services / caller-run job-step evidence | Fail when the latest `deploy-services.yml` run on `main` is red, stale, or missing `Verify public surface contract`, and when caller runs are missing `Hosted starter failover proof`. | Treat missing `main` runs as hosted drift even if a tag run is green. | Fail closed if run-view data omits required jobs, required steps, or freshness fields. |

## Load Profile

- **Shared resources**: GitHub API rate budget, remote run history, local `.tmp/m053-s03/verify/` artifacts, and the retained S02 proof contract surfaced through workflow diagnostics.
- **Per-operation cost**: one remote ref sweep plus one `gh run list` and one `gh run view` per required workflow.
- **10x breakpoint**: GitHub API rate limits and remote log volume, not application throughput.

## Negative Tests

- **Malformed inputs**: missing `GH_TOKEN`, workflow-not-found on remote `main`, malformed `gh run list/view` JSON, or missing phase/status files.
- **Error paths**: stale green workflow run, missing `Hosted starter failover proof` job, missing `Verify public surface contract` step, or packages proof satisfied only by a tag run.
- **Boundary conditions**: `deploy-services.yml` green on a release tag but stale on `main`, latest caller run on an older SHA, and missing release-tag run for the current binary version.

## Steps

1. Create `scripts/verify-m053-s03.sh` with S02-style phase/status artifacts under `.tmp/m053-s03/verify/`; reuse the M034 remote-evidence pattern (`git ls-remote`, `gh run list`, `gh run view`) rather than scraping Actions pages.
2. Require `authoritative-verification.yml` on `main`, `deploy-services.yml` on `main`, and `release.yml` on the current binary tag to resolve fresh expected SHAs; verify the required jobs/steps include `Hosted starter failover proof` and `Verify public surface contract`.
3. Record stable hosted-evidence artifacts like `candidate-refs.json`, `remote-runs.json`, `status.txt`, `current-phase.txt`, and `phase-report.txt`, and fail closed when deploy-services is satisfied only by a stale or tag-only run.
4. Extend `scripts/tests/verify-m053-s03-contract.test.mjs` with stubbed remote-evidence cases that lock the freshness fields, mainline packages semantics, and verifier artifact contract.

## Must-Haves

- [ ] `scripts/verify-m053-s03.sh` assembles starter proof plus packages deploy/public-surface into one hosted evidence contract.
- [ ] The verifier enforces fresh `main` evidence for `deploy-services.yml` and fresh expected refs for the authoritative main/tag workflows instead of accepting stale green runs.
- [ ] Contract tests cover success and fail-closed paths for missing workflows, stale head SHAs, missing required jobs/steps, and missing verifier artifacts.

## Verification

- `node --test scripts/tests/verify-m053-s03-contract.test.mjs`
- `GH_TOKEN=${GH_TOKEN:?set GH_TOKEN} bash scripts/verify-m053-s03.sh`

## Observability Impact

- Signals added/changed: `.tmp/m053-s03/verify/status.txt`, `current-phase.txt`, `phase-report.txt`, `candidate-refs.json`, `remote-runs.json`, and per-query `gh` logs.
- How a future agent inspects this: open `.tmp/m053-s03/verify/remote-runs.json` and compare `expectedHeadSha`, `observedHeadSha`, `requiredJobs`, and `requiredSteps` for each hosted workflow.
- Failure state exposed: missing remote workflow on `main`, stale green run URL, missing starter-proof or public-surface step, `gh` auth error, and ref-resolution mismatch.
  - Estimate: 1h
  - Files: scripts/verify-m053-s03.sh, scripts/tests/verify-m053-s03-contract.test.mjs, scripts/verify-m034-s05.sh
  - Verify: node --test scripts/tests/verify-m053-s03-contract.test.mjs && GH_TOKEN=${GH_TOKEN:?set GH_TOKEN} bash scripts/verify-m053-s03.sh
