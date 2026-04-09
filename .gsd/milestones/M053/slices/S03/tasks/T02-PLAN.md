---
estimated_steps: 4
estimated_files: 3
skills_used:
  - bash-scripting
  - github-workflows
  - test
---

# T02: Add a freshness-aware hosted verifier that couples starter proof with packages deploy truth

Create the slice-owned hosted evidence verifier that turns starter failover proof and packages deploy/public-surface health into one explicit hosted contract. Reuse the proven `git ls-remote` + `gh run list/view` freshness pattern instead of scraping logs, and fail closed when deploy-services proof is satisfied only by a stale or tag-only run.

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

## Inputs

- `scripts/verify-m053-s02.sh`
- `scripts/verify-m034-s05.sh`
- `scripts/verify-m034-s06-remote-evidence.sh`
- `scripts/tests/verify-m034-s06-contract.test.mjs`
- `.github/workflows/authoritative-verification.yml`
- `.github/workflows/deploy-services.yml`
- `.github/workflows/release.yml`
- `scripts/tests/verify-m053-s03-contract.test.mjs`

## Expected Output

- `scripts/verify-m053-s03.sh`
- `scripts/tests/verify-m053-s03-contract.test.mjs`

## Verification

node --test scripts/tests/verify-m053-s03-contract.test.mjs && GH_TOKEN=${GH_TOKEN:?set GH_TOKEN} bash scripts/verify-m053-s03.sh

## Observability Impact

- Signals added/changed: `.tmp/m053-s03/verify/` phase/status markers, candidate/ref artifacts, remote workflow summaries, and per-query GitHub logs.
- How a future agent inspects this: read `.tmp/m053-s03/verify/phase-report.txt`, then inspect `candidate-refs.json` and `remote-runs.json`.
- Failure state exposed: stale SHA mismatches, missing required jobs/steps, workflow-not-found failures, and `gh` auth/query errors.
