---
estimated_steps: 4
estimated_files: 9
skills_used: []
---

# T05: Push the extension candidate tag, preserve the first all-green hosted bundle, and rerun S05

1. Derive the extension candidate tag from `tools/editors/vscode-mesh/package.json`, confirm it is still `ext-v0.3.0`, and create/push that tag from the same rollout commit already proven on remote `main`.
2. Wait for fresh `extension-release-proof.yml` and `publish-extension.yml` `push` runs on `ext-v0.3.0` to complete successfully.
3. If GitHub exposes the reusable extension proof through a different filename/query surface than `gh run list --workflow extension-release-proof.yml`, update `scripts/verify-m034-s05.sh` and `scripts/tests/verify-m034-s06-contract.test.mjs` so the verifier records the real hosted proof truthfully before sign-off.
4. Run `bash scripts/verify-m034-s06-remote-evidence.sh first-green`, preserve the first all-green bundle, then rerun `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh` and confirm the failure boundary has moved past `remote-evidence`.

## Inputs

- `scripts/verify-m034-s06-remote-evidence.sh`
- `scripts/verify-m034-s05.sh`
- `scripts/tests/verify-m034-s06-contract.test.mjs`
- `tools/editors/vscode-mesh/package.json`
- `.github/workflows/extension-release-proof.yml`
- `.github/workflows/publish-extension.yml`
- `.tmp/m034-s06/evidence/v0.1.0/remote-runs.json`

## Expected Output

- `.tmp/m034-s06/evidence/first-green/manifest.json`
- `.tmp/m034-s06/evidence/first-green/remote-runs.json`
- `.tmp/m034-s06/evidence/first-green/remote-extension-release-proof-view.log`
- `.tmp/m034-s06/evidence/first-green/remote-publish-extension-view.log`
- `.tmp/m034-s05/verify/phase-report.txt`
- `.tmp/m034-s05/verify/failed-phase.txt`

## Verification

bash -n scripts/verify-m034-s05.sh
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
