# S02: Authoritative CI verification lane — UAT

**Milestone:** M034
**Written:** 2026-03-26T22:48:47.638Z

# S02: Authoritative CI verification lane — UAT

**Milestone:** M034

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: S02 changes GitHub Actions policy and proof wiring, so acceptance needs both deterministic local contract verification and a concrete post-push GitHub Actions replay once the new workflow files are available remotely.

## Preconditions

- Run from the repo root.
- Ruby is available for local YAML parsing.
- If performing the rollout checks, GitHub repository secrets `MESH_PUBLISH_OWNER` and `MESH_PUBLISH_TOKEN` are configured.
- If performing the rollout checks, the updated workflow files have been pushed to the remote branch GitHub is querying and `gh` is authenticated for this repository.

## Smoke Test

1. Run `bash scripts/verify-m034-s02-workflows.sh`.
2. Run `ruby -e 'require "yaml"; %w[.github/workflows/authoritative-live-proof.yml .github/workflows/authoritative-verification.yml .github/workflows/release.yml].each { |f| YAML.load_file(f) }'`.
3. **Expected:** both commands exit 0, and `.tmp/m034-s02/verify/` contains `reusable.log`, `caller.log`, `release.log`, and `full-contract.log`.

## Test Cases

### 1. Reusable workflow remains the only direct owner of the live proof

1. Run `bash scripts/verify-m034-s02-workflows.sh reusable`.
2. **Expected:** the verifier confirms `.github/workflows/authoritative-live-proof.yml` is `workflow_call` only, requires `MESH_PUBLISH_OWNER` and `MESH_PUBLISH_TOKEN`, runs on `ubuntu-24.04`, bootstraps LLVM 21 and Rust for `x86_64-unknown-linux-gnu`, invokes `bash scripts/verify-m034-s01.sh` exactly once, and uploads `.tmp/m034-s01/verify/**` as failure diagnostics.

### 2. Trusted-event caller lane fails closed for forks and covers the intended trigger set

1. Run `bash scripts/verify-m034-s02-workflows.sh caller`.
2. **Expected:** the verifier confirms `.github/workflows/authoritative-verification.yml` triggers on `pull_request`, `push` to `main`, `workflow_dispatch`, and one weekly `schedule`, uses read-only permissions, sets concurrency to `${{ github.workflow }}-${{ github.ref }}`, explicitly maps both publish secrets, and guards the reusable proof with `github.event.pull_request.head.repo.full_name == github.repository` for PRs.

### 3. Tag releases cannot publish without the authoritative proof

1. Run `bash scripts/verify-m034-s02-workflows.sh release`.
2. **Expected:** the verifier confirms `.github/workflows/release.yml` keeps workflow-wide permissions read-only, calls the reusable authoritative proof only for `refs/tags/v*`, and requires `Create Release` to depend on `build`, `build-meshpkg`, and `authoritative-live-proof` while keeping `contents: write` scoped only to the release job.

### 4. Full local CI/release contract sweep stays green

1. Run `bash scripts/verify-m034-s02-workflows.sh`.
2. **Expected:** the command exits 0 and `full-contract.log` shows the `reusable`, `caller`, and `release` contract sweeps running in order with no drift.

### 5. Post-push trusted-event workflow proves the live package-manager path

1. Push the updated workflow files to the remote branch GitHub Actions will run from.
2. Trigger `Authoritative verification` via `workflow_dispatch` or a same-repo push / PR.
3. Inspect the run.
4. **Expected:** the run includes an `Authoritative live proof` job, that job reaches `verify-m034-s01: ok` in its logs, and a failing proof uploads the `authoritative-live-proof-diagnostics` artifact containing `.tmp/m034-s01/verify/**`.

### 6. Post-push tag release stays blocked on the same proof

1. Push a `v*` tag after the new workflows are live on GitHub.
2. Inspect the `Release` workflow graph.
3. **Expected:** the run contains an `Authoritative live proof` job before `Create Release`, and `Create Release` remains pending/blocked until the proof job passes.

## Edge Cases

### Fork PRs must skip the live proof instead of receiving secrets

1. Open a pull request from a fork after the caller workflow is live on GitHub.
2. Inspect the `Authoritative verification` run.
3. **Expected:** the workflow does not run the reusable live proof for the forked head, and no one widens the trust boundary by switching to `pull_request_target`.

### Missing secret mapping must fail closed

1. Remove one of the publish-secret mappings in a local test edit or simulate the drift and rerun `bash scripts/verify-m034-s02-workflows.sh caller` / `release`.
2. **Expected:** the verifier exits non-zero and identifies the exact missing secret mapping rather than silently allowing a green build-only path.

### Release permission drift must be caught before CI

1. Move `contents: write` back to workflow scope in a local test edit and rerun `bash scripts/verify-m034-s02-workflows.sh release`.
2. **Expected:** the verifier exits non-zero and reports that the release job must be the only job requesting `contents: write`.

### Unshipped workflow files are a rollout gap, not local YAML drift

1. Run `gh run list --workflow authoritative-verification.yml --limit 1` before the workflow file exists on the remote default branch.
2. **Expected:** GitHub returns 404 for workflow discovery. Treat that as evidence that rollout has not happened yet, not as a failure of the local workflow files if the local verifier still passes.

## Failure Signals

- `bash scripts/verify-m034-s02-workflows.sh` exits non-zero or any phase log reports `verification drift`.
- The reusable workflow stops being the only workflow that directly runs `bash scripts/verify-m034-s01.sh`.
- The caller workflow adds `pull_request_target`, loses the weekly drift monitor, drops concurrency, or stops fail-closing for fork PRs.
- `release.yml` regains workflow-wide write permissions or `Create Release` no longer depends on `authoritative-live-proof` for tag runs.
- A post-push trusted-event run does not reach `verify-m034-s01: ok` or a tag run can publish without the proof job finishing first.

## Requirements Proved By This UAT

- No requirement status changes are finalized by this slice alone; it promotes the already-real S01 package-manager proof into the CI/release contract and prepares later M034 slices to validate broader release truth on top of it.

## Not Proven By This UAT

- Installer and released-binary truth (`S03`).
- VS Code extension release hardening (`S04`).
- The full assembled public release candidate across binaries, installer, docs deployment, registry/packages-site health, and extension checks (`S05`).

## Notes for Tester

Use `bash scripts/verify-m034-s02-workflows.sh` as the canonical local gate before pushing workflow edits. If the GitHub-side checks fail after rollout, inspect both the relevant `.tmp/m034-s02/verify/*.log` file and the reusable proof job's uploaded `.tmp/m034-s01/verify/**` artifact before changing workflow policy.
