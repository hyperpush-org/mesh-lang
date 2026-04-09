# S06 Research — Hosted rollout evidence capture

## Summary

S06 is mostly an operational rollout-and-evidence slice, not a new subsystem-building slice.

The core hosted-evidence contract already exists in `scripts/verify-m034-s05.sh`:
- `run_candidate_tags` derives the current release candidate tags and writes `.tmp/m034-s05/verify/candidate-tags.json`
- `run_remote_evidence` queries GitHub Actions with `gh run list/view` and writes `.tmp/m034-s05/verify/remote-runs.json` plus granular `remote-*.{stdout,stderr,log}` files

The current failure is not local verifier drift. It is hosted rollout drift:
- local `main` is ahead of `origin/main` by 129 commits
- `origin/main` is still at `5ddf3b2`
- local `HEAD` is `8d30ff85`
- the remote default branch still does not have `authoritative-verification.yml` or `extension-release-proof.yml`
- candidate tags `v0.1.0` and `ext-v0.3.0` do not exist locally or remotely
- the latest hosted runs are structurally old and do not satisfy the S05/S06 contract

This makes S06 a clean boundary slice:
- **S06 owns getting `remote-evidence` green and preserving the hosted proof artifacts**
- **S07 owns public surface freshness and the final all-green S05 replay**

That split is already encoded in `scripts/verify-m034-s05.sh`, because `remote-evidence` runs before `public-http` and `s01-live-proof`.

## Relevant Requirements / Scope

Milestone context ties this slice to the hosted-proof part of:
- **R045** — CI/CD and release flows prove shipped Mesh surfaces instead of only building artifacts
- **R046** — the package manager end-to-end path is exercised on the real path
- **R047** — extension release hardening is proven through the hosted release lane

S06 also reinforces already-validated **R007** by ensuring the authoritative live proof is green on hosted GitHub Actions, not only locally.

Important planning note: repo-wide `.gsd/REQUIREMENTS.md` currently does **not** contain R045/R046/R047, so treat them as milestone-local requirement IDs from the M034 context unless requirement-registry maintenance is explicitly added later.

## Skills Discovered

Core technology for this slice is GitHub Actions / GitHub CLI.

Installed relevant skills already exist:
- `github-workflows`
- `gh`

Rules from those skills that matter here:
- `github-workflows`: **“No errors” is not validation. Prove observable BEFORE / AFTER / EVIDENCE.**
- `gh`: keep using explicit `-R snowdamiz/mesh-lang` in hosted-run queries; that is already the local pattern in `scripts/verify-m034-s05.sh`.

No new skills were installed.

One caveat: the `github-workflows` skill prefers routing CI operations through `scripts/ci_monitor.cjs`, but this repo does not contain that file. The established local pattern here is the verifier-owned `gh` usage already embedded in `scripts/verify-m034-s05.sh`.

## Implementation Landscape

### `scripts/verify-m034-s05.sh`

This is the canonical assembly verifier and the key file for S06.

Relevant behavior:
- `prepare_verify_root` does `rm -rf "$VERIFY_ROOT"` before every run, so each rerun deletes prior evidence.
- `run_candidate_tags` derives:
  - binary tag from `compiler/meshc/Cargo.toml` + `compiler/meshpkg/Cargo.toml`
  - extension tag from `tools/editors/vscode-mesh/package.json`
- current derived values are already visible in `.tmp/m034-s05/verify/candidate-tags.json`:
  - `binaryTag = v0.1.0`
  - `extensionTag = ext-v0.3.0`
- `run_remote_evidence` checks six exact workflow files:
  - `deploy.yml`
  - `deploy-services.yml`
  - `authoritative-verification.yml`
  - `release.yml`
  - `extension-release-proof.yml`
  - `publish-extension.yml`
- it writes a machine-readable artifact at `.tmp/m034-s05/verify/remote-runs.json`
- it also preserves raw per-query diagnostics under `.tmp/m034-s05/verify/remote-*.{stdout,stderr,log}`

Critical seam: there is **no remote-evidence-only mode**. Phase order is fixed:
1. prereq
2. candidate-tags
3. local workflow/docs/verifier replay
4. `remote-evidence`
5. `public-http`
6. `s01-live-proof`

That means once hosted rollout is green, rerunning the canonical S05 wrapper will continue into `public-http` and then the live S01 publish/install proof.

### `.github/workflows/deploy.yml`

Remote-evidence expects the latest hosted `push` run on `main` to contain:
- jobs: `build`, `deploy`
- build step: `Verify public docs contract`

Current remote state does not satisfy that. The latest hosted main run is `23506361663`, and the failure artifact shows that this run predates the new build step.

### `.github/workflows/authoritative-verification.yml`

This is the new top-level main-branch hosted proof workflow. It calls the reusable `authoritative-live-proof.yml` and is the hosted version of the S02/S05 authoritative live-proof pattern.

Current remote state:
- `gh run list -R snowdamiz/mesh-lang --workflow authoritative-verification.yml ...` returns HTTP 404
- so the workflow file is not yet on the remote default branch

### `.github/workflows/release.yml`

This is the binary tag workflow (`v*`). For S06 purposes it proves two things at once:
- hosted release lane
- hosted release-smoke lane

There is **no separate `release-smoke.yml`**. The S05/S06 contract treats release-smoke as the `Verify release assets (*)` matrix jobs inside `release.yml`, enforced via required job prefixes:
- `Build (`
- `Build meshpkg (`
- `Verify release assets (`

Current remote state:
- there is no hosted `push` run on `v0.1.0`
- the latest available `release.yml` run is on `main`, not the candidate tag, so it cannot satisfy the tag-scoped hosted contract

### `.github/workflows/deploy-services.yml`

This is the Fly deploy tag workflow (`v*`). Remote-evidence expects a hosted `push` run on `v0.1.0` with jobs:
- `Deploy mesh-registry`
- `Deploy mesh-packages website`
- `Post-deploy health checks`

Current remote state:
- there is no hosted `push` run on `v0.1.0`
- latest available hosted run is an older tag rollout (`v15.0`), so history exists, but not for the current candidate

### `.github/workflows/extension-release-proof.yml`

This is reusable-only (`workflow_call`) and is supposed to produce hosted proof evidence for the extension candidate tag.

Current remote state:
- `gh run list -R snowdamiz/mesh-lang --workflow extension-release-proof.yml ...` returns HTTP 404
- so the file is not yet on the remote default branch

This is an important live unknown retired by S06: once the file is on remote default branch and `ext-v0.3.0` is pushed, GitHub must expose a hosted run addressable by this workflow filename for the current `run_remote_evidence` logic to pass.

### `.github/workflows/publish-extension.yml`

This is the extension publish tag workflow (`ext-v*`). Current local workflow shape is:
- `Verify extension release proof`
- `Publish verified extension`

Current remote history is older than that split. The latest available hosted run is `22021584959` on `ext-v0.2.0`, and `gh run view` shows the obsolete one-job shape (`publish` only). So historical green extension publish evidence cannot satisfy the current contract even if it was successful.

### `README.md`, `website/docs/docs/tooling/index.md`, `scripts/tests/verify-m034-s05-contract.test.mjs`

These already lock the public runbook strings, workflow list, and artifact paths. S06 likely should not change them unless the hosted-evidence contract itself changes.

## Current Hosted State

### Git state
- local `main` is ahead of `origin/main` by **129 commits**
- `origin/main` currently points at `5ddf3b2`
- local `HEAD` is `8d30ff85`

This means the local M034 workflow graph is not yet the public default-branch workflow graph.

### Candidate tags
Derived candidate tags are already stable in local code:
- binary candidate tag: `v0.1.0`
- extension candidate tag: `ext-v0.3.0`

But they do **not** exist yet:
- `refs/tags/v0.1.0` absent locally
- `refs/tags/ext-v0.3.0` absent locally
- `git ls-remote --tags origin 'v0.1.0' 'ext-v0.3.0'` returns no matches

### Remote workflow availability
Present on remote default branch:
- `deploy.yml`
- `deploy-services.yml`
- `release.yml`
- `publish-extension.yml`

Missing on remote default branch:
- `authoritative-verification.yml`
- `extension-release-proof.yml`

### Current remote-evidence failures from `.tmp/m034-s05/verify/remote-runs.json`
- `deploy.yml`: latest main run is missing step `Verify public docs contract`
- `deploy-services.yml`: no hosted `push` run on `v0.1.0`
- `authoritative-verification.yml`: missing on remote default branch
- `release.yml`: no hosted `push` run on `v0.1.0`
- `extension-release-proof.yml`: missing on remote default branch
- `publish-extension.yml`: no hosted `push` run on `ext-v0.3.0`

## Natural Seams / Task Cuts

### 1. Rollout preflight (safe, reversible)
Goal: prove the local workflow/verifier graph is still green before any irreversible public action.

Files / commands:
- `scripts/verify-m034-s05-workflows.sh`
- `scripts/verify-m034-s02-workflows.sh`
- `scripts/verify-m034-s04-workflows.sh`
- `scripts/verify-m034-s05.sh` (syntax / contract only if needed)

Why first:
- this is the last safe stop before tag pushes, release publication, Fly deploys, and extension publication

### 2. Default-branch hosted rollout
Goal: get the remote default branch onto the local M034 workflow graph.

Required outcome:
- `authoritative-verification.yml` exists on remote default branch and gets its first green `push` run on `main`
- `deploy.yml` gets a new green `push` run on `main` that includes `Verify public docs contract`

Why second:
- S05/S06 remote-evidence hardcodes `requiredEvent=push` / `requiredHeadBranch=main` for these workflows
- rerunning old hosted runs is insufficient because the workflow definitions are baked into the commit that triggered them

### 3. Binary-tag hosted rollout
Goal: create and push `v0.1.0` from the rollout commit, then capture green hosted runs for:
- `release.yml`
- `deploy-services.yml`

Important detail:
- release-smoke is not a separate workflow; it is represented by the `Verify release assets (*)` matrix jobs inside `release.yml`

### 4. Extension-tag hosted rollout
Goal: create and push `ext-v0.3.0`, then capture green hosted runs for:
- `extension-release-proof.yml`
- `publish-extension.yml`

Why last:
- this is the most irreversible public surface because it publishes the extension to both registries

### 5. Evidence preservation / handoff
Goal: preserve the first hosted-green evidence bundle for S05/S07 consumption.

Important constraint:
- every `scripts/verify-m034-s05.sh` run deletes `.tmp/m034-s05/verify/` before rebuilding it
- the first hosted-green bundle must be copied or summarized immediately

Useful existing artifacts to preserve:
- `.tmp/m034-s05/verify/candidate-tags.json`
- `.tmp/m034-s05/verify/remote-runs.json`
- `.tmp/m034-s05/verify/remote-evidence.log`
- `.tmp/m034-s05/verify/remote-*.log`
- `.tmp/m034-s05/verify/phase-report.txt`
- `.tmp/m034-s05/verify/status.txt`
- `.tmp/m034-s05/verify/failed-phase.txt` (if the next blocker is `public-http`, which is the expected S06→S07 handoff)

## Recommendation

Treat S06 as an **operational rollout + evidence archival slice**.

Do **not** invent a new hosted-proof contract. Reuse the existing S05 contract in `scripts/verify-m034-s05.sh` and make the slice success criteria about moving the failure boundary:
- **before S06:** `failed-phase.txt = remote-evidence`
- **after S06:** `remote-evidence` passes, and the next truthful blocker (if any) is `public-http`

That gives a clean handoff into S07, which already owns public freshness and final replay.

Only add code if execution exposes a real gap in the current capture surface. The two likely triggers would be:
1. needing a dedicated `remote-evidence-only` entrypoint because repeated polling through the full S05 wrapper is too destructive or too slow
2. discovering that GitHub’s reusable-workflow run visibility does not match the current `gh run list --workflow extension-release-proof.yml` assumption

## Risks / Unknowns

- **Latest-vs-first green tension:** `run_remote_evidence` only checks the latest matching hosted run, not “any successful run.” If a later push regresses a workflow before evidence is archived, the verifier goes red again even though a first green existed.
- **Reusable workflow visibility:** `extension-release-proof.yml` is `workflow_call`-only, but the verifier assumes GitHub exposes it as a hosted run addressable by filename with event `push` and headBranch `ext-v0.3.0`. S06 is the first live proof of that assumption.
- **No partial mode:** there is no standalone hosted-evidence command; once `remote-evidence` passes, the canonical S05 wrapper continues into `public-http` and then live S01 work.
- **Real state changes:** `v0.1.0` and `ext-v0.3.0` pushes will create public release/deploy/publish side effects. The order above matters.

## Verification Plan

### Safe preflight
Run before any push/tag action:

```bash
bash scripts/verify-m034-s05-workflows.sh
bash scripts/verify-m034-s02-workflows.sh
bash scripts/verify-m034-s04-workflows.sh
bash -n scripts/verify-m034-s05.sh
```

### Hosted rollout spot checks
After the relevant push/tag actions, use the same repo the verifier is hardcoded against:

```bash
gh run list -R snowdamiz/mesh-lang --workflow deploy.yml --event push --branch main --limit 1 --json databaseId,createdAt,displayTitle,event,headBranch,headSha,status,conclusion,url
gh run list -R snowdamiz/mesh-lang --workflow authoritative-verification.yml --event push --branch main --limit 1 --json databaseId,createdAt,displayTitle,event,headBranch,headSha,status,conclusion,url
gh run list -R snowdamiz/mesh-lang --workflow deploy-services.yml --event push --branch v0.1.0 --limit 1 --json databaseId,createdAt,displayTitle,event,headBranch,headSha,status,conclusion,url
gh run list -R snowdamiz/mesh-lang --workflow release.yml --event push --branch v0.1.0 --limit 1 --json databaseId,createdAt,displayTitle,event,headBranch,headSha,status,conclusion,url
gh run list -R snowdamiz/mesh-lang --workflow extension-release-proof.yml --event push --branch ext-v0.3.0 --limit 1 --json databaseId,createdAt,displayTitle,event,headBranch,headSha,status,conclusion,url
gh run list -R snowdamiz/mesh-lang --workflow publish-extension.yml --event push --branch ext-v0.3.0 --limit 1 --json databaseId,createdAt,displayTitle,event,headBranch,headSha,status,conclusion,url
```

### Canonical slice proof
Use the existing assembled verifier and inspect where the failure boundary moves:

```bash
set -a && source .env && set +a && bash scripts/verify-m034-s05.sh
```

S06 is complete when all of the following are true:
- `.tmp/m034-s05/verify/remote-runs.json` shows every workflow entry with `"status": "ok"`
- `.tmp/m034-s05/verify/phase-report.txt` contains `remote-evidence\tpassed`
- if the full script still exits non-zero, `.tmp/m034-s05/verify/failed-phase.txt` has advanced to `public-http` rather than remaining `remote-evidence`

That gives S07 a truthful starting point for the public freshness and final all-green replay work.
