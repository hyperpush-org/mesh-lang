# S06: Hosted rollout evidence capture

**Goal:** Capture first green hosted rollout evidence for the current M034 release candidate by making remote-evidence polling non-destructive, rolling the new workflow graph to `origin/main`, running the binary and extension candidate tags, and archiving the first all-green remote bundle that moves `scripts/verify-m034-s05.sh` past `remote-evidence`.
**Demo:** After this: The remote default branch and current candidate tags have first green hosted runs for authoritative verification, release-smoke, deploy, services deploy, extension proof, and extension publish, with evidence captured for S05 consumption.

## Tasks
- [x] **T01: Added a stop-after remote-evidence mode to the S05 verifier, shipped the S06 archive helper, and captured the hosted-red preflight bundle.** — 1. Add a non-destructive remote-evidence-only path for `scripts/verify-m034-s05.sh` plus a slice-owned wrapper `scripts/verify-m034-s06-remote-evidence.sh` that copies the current verify bundle into deterministic `.tmp/m034-s06/evidence/<label>/` directories.
2. Add `scripts/tests/verify-m034-s06-contract.test.mjs` to pin the new operator contract, archive layout, and allowed stop-after phase behavior.
3. Capture the current red hosted state into `.tmp/m034-s06/evidence/preflight/` so later rollout tasks can diff against a known baseline instead of relying on ephemeral `.tmp/m034-s05/verify/`.
  - Estimate: 2h
  - Files: scripts/verify-m034-s05.sh, scripts/verify-m034-s06-remote-evidence.sh, scripts/tests/verify-m034-s06-contract.test.mjs, .tmp/m034-s05/verify/remote-runs.json, .tmp/m034-s06/evidence/preflight/manifest.json
  - Verify: bash -n scripts/verify-m034-s05.sh
node --test scripts/tests/verify-m034-s06-contract.test.mjs
bash scripts/verify-m034-s06-remote-evidence.sh preflight || true
test -f .tmp/m034-s06/evidence/preflight/remote-runs.json
test -f .tmp/m034-s06/evidence/preflight/manifest.json
- [x] **T02: Verified the local M034 workflow graph, captured the `git push` HTTP 408 blocker, and left truthful remote-main recovery evidence.** — 1. Re-run the safe preflight gate (`bash scripts/verify-m034-s05-workflows.sh`, `bash scripts/verify-m034-s02-workflows.sh`, `bash scripts/verify-m034-s04-workflows.sh`, `bash -n scripts/verify-m034-s05.sh`) before any push.
2. Push the rollout commit already on local `main` to `origin/main`, then wait for new `deploy.yml` and `authoritative-verification.yml` push runs on `main` to complete successfully.
3. Run `bash scripts/verify-m034-s06-remote-evidence.sh main || true` to archive the hosted state, and assert that `deploy.yml` and `authoritative-verification.yml` are `ok` in `.tmp/m034-s06/evidence/main/remote-runs.json`.
  - Estimate: 90m
  - Files: scripts/verify-m034-s06-remote-evidence.sh, .github/workflows/deploy.yml, .github/workflows/authoritative-verification.yml, .tmp/m034-s06/evidence/preflight/remote-runs.json, .tmp/m034-s06/evidence/main/remote-runs.json
  - Verify: bash scripts/verify-m034-s05-workflows.sh
bash scripts/verify-m034-s02-workflows.sh
bash scripts/verify-m034-s04-workflows.sh
bash -n scripts/verify-m034-s05.sh
gh run list -R snowdamiz/mesh-lang --workflow deploy.yml --event push --branch main --limit 1 --json databaseId,status,conclusion,headSha,url
gh run list -R snowdamiz/mesh-lang --workflow authoritative-verification.yml --event push --branch main --limit 1 --json databaseId,status,conclusion,headSha,url
bash scripts/verify-m034-s06-remote-evidence.sh main || true
  - Blocker: The required rollout push is blocked by a transport failure from this host/environment: `git push` locally generates a long-running pack and then dies with `error: RPC failed; HTTP 408 curl 22 The requested URL returned error: 408`, `send-pack: unexpected disconnect while reading sideband packet`, and `fatal: the remote end hung up unexpectedly`. Until that transport issue is resolved, T02 cannot produce any legitimate hosted `main` evidence, and downstream tag tasks T03/T04 must not proceed.
- [x] **T03: Captured repeated HTTPS rollout push failures and left durable transport-recovery evidence for `main`.** — 1. Start from `.tmp/m034-s06/push-main.stdout` / `.tmp/m034-s06/push-main.stderr`, the rollout SHA already validated locally, and the stale remote-`main` SHA captured by T02; reproduce the blocked push in a bounded way and test transport-safe recovery options that do not rewrite history or fabricate hosted proof, recording each attempt under `.tmp/m034-s06/transport-recovery/`.
2. Stop only when `origin/main` advances to the intended rollout SHA, then wait for fresh `deploy.yml` and `authoritative-verification.yml` `push` runs on `main` to complete successfully for that SHA.
3. Run `bash scripts/verify-m034-s06-remote-evidence.sh main || true`, archive the first truthful `main` bundle, and mechanically assert that `deploy.yml` and `authoritative-verification.yml` are `ok` in `.tmp/m034-s06/evidence/main/remote-runs.json`.
  - Estimate: 2h
  - Files: .tmp/m034-s06/push-main.stdout, .tmp/m034-s06/push-main.stderr, .tmp/m034-s06/transport-recovery/, scripts/verify-m034-s06-remote-evidence.sh, .github/workflows/deploy.yml, .github/workflows/authoritative-verification.yml, .tmp/m034-s06/evidence/main/remote-runs.json
  - Verify: gh api repos/snowdamiz/mesh-lang/branches/main --jq '.commit.sha'
gh run list -R snowdamiz/mesh-lang --workflow deploy.yml --event push --branch main --limit 1 --json databaseId,status,conclusion,headSha,url
gh run list -R snowdamiz/mesh-lang --workflow authoritative-verification.yml --event push --branch main --limit 1 --json databaseId,status,conclusion,headSha,url
bash scripts/verify-m034-s06-remote-evidence.sh main || true
python3 - <<'PY'
import json
from pathlib import Path
artifact = json.loads(Path('.tmp/m034-s06/evidence/main/remote-runs.json').read_text())
wanted = {'deploy.yml', 'authoritative-verification.yml'}
bad = {entry['workflowFile']: entry['status'] for entry in artifact['workflows'] if entry['workflowFile'] in wanted and entry['status'] != 'ok'}
if bad:
    raise SystemExit(f'main rollout still red: {bad}')
PY
  - Blocker: This host still cannot land the rollout over HTTPS: both the default chunked push and a 1 GiB buffered non-chunked push fail with `HTTP 408` after roughly twelve minutes, leaving `origin/main` unchanged at `5ddf3b2dce17abe08e1188d9b46e575d83525b50`. SSH push is unavailable, neither the T02 rollout commit nor current `HEAD` exists remotely yet, `authoritative-verification.yml` is still absent from the remote default branch, and downstream hosted-evidence tasks T04/T05 remain blocked until a transport path that can upload the ~565 MB receive-pack payload is available.
- [x] **T04: Archived blocked `v0.1.0` hosted evidence and proved the binary-tag rollout is still blocked behind stale remote `main`.** — 1. Derive the binary candidate tag mechanically from `compiler/meshc/Cargo.toml` / `compiler/meshpkg/Cargo.toml`, confirm it is still `v0.1.0`, and create/push that tag from the exact rollout commit already proven on remote `main` by T03.
2. Wait for fresh `release.yml` and `deploy-services.yml` `push` runs on `v0.1.0` to complete successfully, remembering that release-smoke is represented by the `Verify release assets (*)` jobs inside `release.yml`, not by a separate workflow.
3. Run `bash scripts/verify-m034-s06-remote-evidence.sh v0.1.0 || true` and mechanically assert that `release.yml` and `deploy-services.yml` are `ok` in `.tmp/m034-s06/evidence/v0.1.0/remote-runs.json`.
  - Estimate: 90m
  - Files: scripts/verify-m034-s06-remote-evidence.sh, compiler/meshc/Cargo.toml, compiler/meshpkg/Cargo.toml, .github/workflows/release.yml, .github/workflows/deploy-services.yml, .tmp/m034-s06/evidence/main/remote-runs.json, .tmp/m034-s06/evidence/v0.1.0/remote-runs.json
  - Verify: gh run list -R snowdamiz/mesh-lang --workflow release.yml --event push --branch v0.1.0 --limit 1 --json databaseId,status,conclusion,headSha,url
gh run list -R snowdamiz/mesh-lang --workflow deploy-services.yml --event push --branch v0.1.0 --limit 1 --json databaseId,status,conclusion,headSha,url
bash scripts/verify-m034-s06-remote-evidence.sh v0.1.0 || true
python3 - <<'PY'
import json
from pathlib import Path
artifact = json.loads(Path('.tmp/m034-s06/evidence/v0.1.0/remote-runs.json').read_text())
wanted = {'release.yml', 'deploy-services.yml'}
bad = {entry['workflowFile']: entry['status'] for entry in artifact['workflows'] if entry['workflowFile'] in wanted and entry['status'] != 'ok'}
if bad:
    raise SystemExit(f'binary-tag rollout still red: {bad}')
PY
  - Blocker: Remote `main` has not advanced to the rollout graph, `authoritative-verification.yml` is still missing on the default branch, `v0.1.0` is absent remotely, and there are no `release.yml` or `deploy-services.yml` `push` runs on `v0.1.0`. Until a transport path or equivalent remote recovery lands the rollout commit on `main`, T05 cannot truthfully capture the first all-green hosted bundle.
- [x] **T05: Retargeted S05 extension-proof polling to the publish workflow surface and confirmed the hosted rollout is still blocked before `remote-evidence`.** — 1. Derive the extension candidate tag from `tools/editors/vscode-mesh/package.json`, confirm it is still `ext-v0.3.0`, and create/push that tag from the same rollout commit already proven on remote `main`.
2. Wait for fresh `extension-release-proof.yml` and `publish-extension.yml` `push` runs on `ext-v0.3.0` to complete successfully.
3. If GitHub exposes the reusable extension proof through a different filename/query surface than `gh run list --workflow extension-release-proof.yml`, update `scripts/verify-m034-s05.sh` and `scripts/tests/verify-m034-s06-contract.test.mjs` so the verifier records the real hosted proof truthfully before sign-off.
4. Run `bash scripts/verify-m034-s06-remote-evidence.sh first-green`, preserve the first all-green bundle, then rerun `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh` and confirm the failure boundary has moved past `remote-evidence`.
  - Estimate: 2h
  - Files: scripts/verify-m034-s06-remote-evidence.sh, scripts/verify-m034-s05.sh, scripts/tests/verify-m034-s06-contract.test.mjs, tools/editors/vscode-mesh/package.json, .github/workflows/extension-release-proof.yml, .github/workflows/publish-extension.yml, .tmp/m034-s06/evidence/v0.1.0/remote-runs.json, .tmp/m034-s06/evidence/first-green/remote-runs.json, .tmp/m034-s05/verify/phase-report.txt
  - Verify: bash -n scripts/verify-m034-s05.sh
node --test scripts/tests/verify-m034-s06-contract.test.mjs
gh run list -R snowdamiz/mesh-lang --workflow extension-release-proof.yml --event push --branch ext-v0.3.0 --limit 1 --json databaseId,status,conclusion,headSha,url
gh run list -R snowdamiz/mesh-lang --workflow publish-extension.yml --event push --branch ext-v0.3.0 --limit 1 --json databaseId,status,conclusion,headSha,url
bash scripts/verify-m034-s06-remote-evidence.sh first-green
python3 - <<'PY'
import json
from pathlib import Path
artifact = json.loads(Path('.tmp/m034-s06/evidence/first-green/remote-runs.json').read_text())
bad = {entry['workflowFile']: entry['status'] for entry in artifact['workflows'] if entry['status'] != 'ok'}
if bad:
    raise SystemExit(f'first-green archive still has red workflows: {bad}')
PY
set -a && source .env && set +a && bash scripts/verify-m034-s05.sh || test "$(cat .tmp/m034-s05/verify/failed-phase.txt)" = "public-http"
grep -Fx 'remote-evidence	passed' .tmp/m034-s05/verify/phase-report.txt
  - Blocker: Remote `main` still resolves to `5ddf3b2dce17abe08e1188d9b46e575d83525b50`, `authoritative-verification.yml` is still absent from the remote default branch, there are still no hosted `push` runs for `v0.1.0` or `ext-v0.3.0`, and GitHub still does not know the local rollout commit `6428dca29064e4c0e8ab54d210d2fe475e0b9f68`.
