---
estimated_steps: 3
estimated_files: 7
skills_used: []
---

# T04: Push the binary candidate tag after remote-main recovery and capture release/services hosted greens

1. Derive the binary candidate tag mechanically from `compiler/meshc/Cargo.toml` / `compiler/meshpkg/Cargo.toml`, confirm it is still `v0.1.0`, and create/push that tag from the exact rollout commit already proven on remote `main` by T03.
2. Wait for fresh `release.yml` and `deploy-services.yml` `push` runs on `v0.1.0` to complete successfully, remembering that release-smoke is represented by the `Verify release assets (*)` jobs inside `release.yml`, not by a separate workflow.
3. Run `bash scripts/verify-m034-s06-remote-evidence.sh v0.1.0 || true` and mechanically assert that `release.yml` and `deploy-services.yml` are `ok` in `.tmp/m034-s06/evidence/v0.1.0/remote-runs.json`.

## Inputs

- `scripts/verify-m034-s06-remote-evidence.sh`
- `compiler/meshc/Cargo.toml`
- `compiler/meshpkg/Cargo.toml`
- `.github/workflows/release.yml`
- `.github/workflows/deploy-services.yml`
- `.tmp/m034-s06/evidence/main/remote-runs.json`

## Expected Output

- `.tmp/m034-s06/evidence/v0.1.0/manifest.json`
- `.tmp/m034-s06/evidence/v0.1.0/remote-runs.json`
- `.tmp/m034-s06/evidence/v0.1.0/remote-release-view.log`
- `.tmp/m034-s06/evidence/v0.1.0/remote-deploy-services-view.log`

## Verification

gh run list -R snowdamiz/mesh-lang --workflow release.yml --event push --branch v0.1.0 --limit 1 --json databaseId,status,conclusion,headSha,url
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

## Observability Impact

Preserves the first binary-tag hosted evidence with release and services run metadata, which is the authoritative proof that the public binary candidate actually exercised the hosted release and services lanes after remote-main recovery.
