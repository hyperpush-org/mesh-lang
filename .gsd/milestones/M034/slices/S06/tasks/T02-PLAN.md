---
estimated_steps: 3
estimated_files: 5
skills_used: []
---

# T02: Verified the local M034 workflow graph, captured the `git push` HTTP 408 blocker, and left truthful remote-main recovery evidence.

1. Re-run the safe preflight gate (`bash scripts/verify-m034-s05-workflows.sh`, `bash scripts/verify-m034-s02-workflows.sh`, `bash scripts/verify-m034-s04-workflows.sh`, `bash -n scripts/verify-m034-s05.sh`) before any push.
2. Push the rollout commit already on local `main` to `origin/main`, then wait for new `deploy.yml` and `authoritative-verification.yml` push runs on `main` to complete successfully.
3. Run `bash scripts/verify-m034-s06-remote-evidence.sh main || true` to archive the hosted state, and assert that `deploy.yml` and `authoritative-verification.yml` are `ok` in `.tmp/m034-s06/evidence/main/remote-runs.json`.

## Inputs

- `scripts/verify-m034-s06-remote-evidence.sh`
- `.github/workflows/deploy.yml`
- `.github/workflows/authoritative-verification.yml`
- `.tmp/m034-s06/evidence/preflight/remote-runs.json`

## Expected Output

- `.tmp/m034-s06/evidence/main/manifest.json`
- `.tmp/m034-s06/evidence/main/remote-runs.json`
- `.tmp/m034-s06/evidence/main/remote-deploy-view.log`
- `.tmp/m034-s06/evidence/main/remote-authoritative-verification-view.log`

## Verification

bash scripts/verify-m034-s05-workflows.sh
bash scripts/verify-m034-s02-workflows.sh
bash scripts/verify-m034-s04-workflows.sh
bash -n scripts/verify-m034-s05.sh
gh run list -R snowdamiz/mesh-lang --workflow deploy.yml --event push --branch main --limit 1 --json databaseId,status,conclusion,headSha,url
gh run list -R snowdamiz/mesh-lang --workflow authoritative-verification.yml --event push --branch main --limit 1 --json databaseId,status,conclusion,headSha,url
bash scripts/verify-m034-s06-remote-evidence.sh main || true

## Observability Impact

Archives the first main-branch hosted proofs with run URLs and raw `gh run list/view` logs so later tasks can prove the default-branch rollout actually landed.
