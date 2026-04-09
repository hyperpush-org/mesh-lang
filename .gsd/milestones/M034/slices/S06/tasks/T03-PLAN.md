---
estimated_steps: 3
estimated_files: 7
skills_used: []
---

# T03: Retire the rollout push transport blocker and archive truthful remote-main evidence

1. Start from `.tmp/m034-s06/push-main.stdout` / `.tmp/m034-s06/push-main.stderr`, the rollout SHA already validated locally, and the stale remote-`main` SHA captured by T02; reproduce the blocked push in a bounded way and test transport-safe recovery options that do not rewrite history or fabricate hosted proof, recording each attempt under `.tmp/m034-s06/transport-recovery/`.
2. Stop only when `origin/main` advances to the intended rollout SHA, then wait for fresh `deploy.yml` and `authoritative-verification.yml` `push` runs on `main` to complete successfully for that SHA.
3. Run `bash scripts/verify-m034-s06-remote-evidence.sh main || true`, archive the first truthful `main` bundle, and mechanically assert that `deploy.yml` and `authoritative-verification.yml` are `ok` in `.tmp/m034-s06/evidence/main/remote-runs.json`.

## Inputs

- `.tmp/m034-s06/push-main.stdout`
- `.tmp/m034-s06/push-main.stderr`
- `scripts/verify-m034-s06-remote-evidence.sh`
- `.github/workflows/deploy.yml`
- `.github/workflows/authoritative-verification.yml`

## Expected Output

- `.tmp/m034-s06/transport-recovery/attempts.log`
- `.tmp/m034-s06/evidence/main/manifest.json`
- `.tmp/m034-s06/evidence/main/remote-runs.json`
- `.tmp/m034-s06/evidence/main/remote-deploy-view.log`
- `.tmp/m034-s06/evidence/main/remote-authoritative-verification-view.log`

## Verification

gh api repos/snowdamiz/mesh-lang/branches/main --jq '.commit.sha'
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

## Observability Impact

Captures the transport-recovery attempt log plus the first truthful `main` archive, which is the authoritative proof that the remote default branch actually advanced to the rollout SHA before any tag-triggered hosted evidence is claimed.
