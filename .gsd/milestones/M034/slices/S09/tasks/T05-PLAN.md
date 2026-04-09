---
estimated_steps: 6
estimated_files: 9
skills_used: []
---

# T05: Roll the repaired SHA onto the approved refs and capture all-green hosted evidence

Once the local reroll blockers are repaired, produce one new rollout target and rerun the hosted evidence set on that exact commit.
- Recompute the minimal repaired rollout commit relative to `origin/main`, record the exact target SHA plus before-state ref map under `.tmp/m034-s09/rollout/`, and update the approval payload with the new diff and ref moves.
- Show the recorded summary and get explicit user confirmation before any outward GitHub action.
- Move `main`, `v0.1.0`, and `ext-v0.3.0` onto the repaired SHA using the least-destructive path the remote state allows, then record the resulting after-state ref map.
- Monitor `deploy.yml`, `authoritative-verification.yml`, `release.yml`, `deploy-services.yml`, `extension-release-proof.yml`, and `publish-extension.yml` until they are completed/success on the expected refs and `headSha`.
- If any lane still fails, persist the failing job URLs plus `gh run view --log-failed` output under `.tmp/m034-s09/rollout/failed-jobs/` before stopping so the next blocker is self-explanatory rather than just 'workflow red'.

## Inputs

- `.github/workflows/release.yml`
- `.github/workflows/publish-extension.yml`
- `packages-website/Dockerfile`
- `scripts/verify-m034-s05.sh`
- `scripts/verify-m034-s06-remote-evidence.sh`
- `.tmp/m034-s09/rollout/plan.md`
- `.tmp/m034-s09/rollout/apply_rollout.py`
- `.tmp/m034-s09/rollout/monitor_workflows.py`

## Expected Output

- `.tmp/m034-s09/rollout/target-sha.txt`
- `.tmp/m034-s09/rollout/remote-refs.before.txt`
- `.tmp/m034-s09/rollout/plan.md`
- `.tmp/m034-s09/rollout/remote-refs.after.txt`
- `.tmp/m034-s09/rollout/workflow-status.json`
- `.tmp/m034-s09/rollout/workflow-urls.txt`
- `.tmp/m034-s09/rollout/failed-jobs/`

## Verification

bash -c 'set -euo pipefail; test -s .tmp/m034-s09/rollout/target-sha.txt; test -s .tmp/m034-s09/rollout/remote-refs.before.txt; test -s .tmp/m034-s09/rollout/remote-refs.after.txt; test -s .tmp/m034-s09/rollout/workflow-status.json; test -s .tmp/m034-s09/rollout/workflow-urls.txt'
python3 - <<'PY'
from pathlib import Path
import json
required = [
    'deploy.yml',
    'deploy-services.yml',
    'authoritative-verification.yml',
    'release.yml',
    'extension-release-proof.yml',
    'publish-extension.yml',
]
target = Path('.tmp/m034-s09/rollout/target-sha.txt').read_text().strip()
status = json.loads(Path('.tmp/m034-s09/rollout/workflow-status.json').read_text())
for name in required:
    entry = status[name]
    assert entry['headSha'] == target, (name, entry)
    assert entry['status'] == 'completed', (name, entry)
    assert entry['conclusion'] == 'success', (name, entry)
print('workflow-status.json matches repaired rollout target and all-green hosted evidence')
PY
