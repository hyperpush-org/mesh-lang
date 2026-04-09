---
estimated_steps: 4
estimated_files: 5
skills_used:
  - github-workflows
  - gh
  - test
---

# T04: Add release-candidate identity, hosted-run evidence checks, and the final runbook

**Slice:** S05 — Full public release assembly proof
**Milestone:** M034

## Description

Finish the assembly slice by making the release candidate explicit and checking the hosted rollout surfaces honestly. This task codifies the binary-vs-extension tag policy, adds GitHub Actions evidence checks to the S05 verifier, and publishes one canonical runbook entry so future agents know exactly which tags, workflows, URLs, and proof artifacts define a public-ready Mesh release.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| GitHub CLI / workflow evidence queries | Fail the remote-evidence phase with a named rollout gap and keep the exact `gh run list/view` command output under `.tmp/m034-s05/verify/`; never synthesize success. | Treat GitHub API timeouts as remote-evidence failure and preserve the partial query output. | Treat missing workflows, wrong events, or missing jobs as rollout drift rather than ignoring them. |
| Candidate tag derivation | Fail if Cargo versions and extension version do not map cleanly to `v<Cargo version>` and `ext-v<extension version>`. | N/A | Treat mismatched or stale tag examples in docs/runbooks as contract drift. |
| Runbook documentation | Fail if the canonical runbook references commands, tags, or workflows that the verifier does not actually use. | N/A | Treat placeholder-only or outdated release instructions as incomplete proof. |

## Load Profile

- **Shared resources**: GitHub Actions run history, GitHub API rate limits/auth, and the same remote workflow surfaces that release/deploy/publication depend on.
- **Per-operation cost**: a few `gh run list/view` queries plus grep-level runbook truth checks.
- **10x breakpoint**: GitHub API/rate-limit pressure fails first, so the verifier should request only the most recent required runs/jobs and persist the results locally.

## Negative Tests

- **Malformed inputs**: missing default-branch workflows, stale historical runs that predate the new proof jobs, or docs that claim a single unified tag version.
- **Error paths**: 404 workflow-not-found, empty run lists, missing required jobs, or mismatched candidate tags must each keep S05 red.
- **Boundary conditions**: the binary release and extension release remain independently versioned; S05 must compose them honestly instead of inventing one shared tag.

## Steps

1. Extend `scripts/verify-m034-s05.sh` with a remote-evidence phase that records the most recent required `gh run list/view` output for `deploy.yml`, `deploy-services.yml`, `authoritative-verification.yml`, `release.yml`, `extension-release-proof.yml`, and `publish-extension.yml`.
2. Make the verifier derive and report the expected binary and extension candidate tags from `compiler/meshc/Cargo.toml`, `compiler/meshpkg/Cargo.toml`, and `tools/editors/vscode-mesh/package.json`, and fail if the docs/runbook or remote evidence disagree.
3. Update the canonical operator-facing runbook entry in `README.md` and the public tooling docs so the assembled proof command, candidate-tag policy, and required hosted workflows are explicit.
4. Rerun the full S05 verifier and confirm it leaves `remote-runs.json` and `candidate-tags.json` artifacts instead of hand-waved rollout notes.

## Must-Haves

- [ ] S05 derives and documents `v<Cargo version>` for binaries and `ext-v<extension version>` for the extension instead of inventing a fake unified tag.
- [ ] Hosted workflow evidence is part of the canonical verifier and fails honestly on rollout gaps.
- [ ] The README/tooling runbook names the exact command, workflows, and public URLs that define a public-ready release.
- [ ] `.tmp/m034-s05/verify/remote-runs.json` and `.tmp/m034-s05/verify/candidate-tags.json` make hosted evidence auditable after the run.

## Verification

- `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh`
- `test -f .tmp/m034-s05/verify/remote-runs.json`
- `test -f .tmp/m034-s05/verify/candidate-tags.json`
- `rg -n 'verify-m034-s05|v<Cargo version>|ext-v<extension version>|deploy\.yml|deploy-services\.yml|authoritative-verification\.yml|extension-release-proof\.yml|publish-extension\.yml' README.md website/docs/docs/tooling/index.md`

## Observability Impact

- Signals added/changed: persisted GitHub run metadata plus derived candidate-tag artifacts under `.tmp/m034-s05/verify/`.
- How a future agent inspects this: rerun `bash scripts/verify-m034-s05.sh` or read `remote-runs.json` / `candidate-tags.json` after a failure.
- Failure state exposed: missing workflow rollout, wrong event/job graph, or candidate-tag drift between repo state and hosted evidence.

## Inputs

- `scripts/verify-m034-s05.sh` — canonical S05 verifier that this task extends with remote evidence and candidate-tag logic.
- `README.md` — operator-facing runbook surface that must match the verifier exactly.
- `website/docs/docs/tooling/index.md` — public tooling runbook surface that must name the same workflows and proof command.
- `.github/workflows/release.yml` — binary release workflow whose hosted runs must satisfy the candidate story.
- `.github/workflows/publish-extension.yml` — extension publish workflow whose hosted runs must satisfy the candidate story.
- `compiler/meshc/Cargo.toml` — authoritative binary version source for `v<Cargo version>`.
- `compiler/meshpkg/Cargo.toml` — second binary version source that must stay aligned with `meshc`.
- `tools/editors/vscode-mesh/package.json` — authoritative extension version source for `ext-v<extension version>`.

## Expected Output

- `scripts/verify-m034-s05.sh` — canonical verifier extended with hosted-run and candidate-tag phases.
- `README.md` — runbook entry updated to the same candidate/workflow truth the verifier enforces.
- `website/docs/docs/tooling/index.md` — tooling docs updated to the same assembled proof command and workflow set.
- `.tmp/m034-s05/verify/remote-runs.json` — persisted hosted workflow evidence for the latest required runs/jobs.
- `.tmp/m034-s05/verify/candidate-tags.json` — persisted binary and extension candidate-tag derivation.
