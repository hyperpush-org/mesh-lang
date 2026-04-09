---
estimated_steps: 10
estimated_files: 9
skills_used: []
---

# T05: Retarget the candidate tags on the repaired rollout commit and refresh the hosted run snapshots.

After T03 and T04 land, the old candidate-tag runs are still tied to the broken rollout commit, so they are not acceptable evidence. This task first confirms the repaired rollout SHA on local `HEAD` and `origin/main`, then asks for explicit approval before any outward mutation. After approval, retarget or recreate `v0.1.0` and `ext-v0.3.0` on the repaired rollout commit, monitor the hosted runs, and persist durable snapshots that distinguish the new runs from the stale red ones by ref name and `headSha`.

Steps:
1. Confirm the intended rollout SHA from local `HEAD`, `origin/main`, and the version files that derive `v0.1.0` / `ext-v0.3.0`.
2. Present the exact outward action needed to retarget or recreate the two existing remote tags, and wait for explicit user approval before mutating any remote ref.
3. Update the remote tags on the repaired rollout commit, then use the rollout monitor to capture `release.yml`, `deploy-services.yml`, and `publish-extension.yml` run payloads plus any failed job logs under `.tmp/m034-s08/tag-rollout/`.
4. Stop only when the monitored runs are completed green on the expected refs and `headSha`, or when a new concrete hosted blocker is captured.

Must-haves:
- No remote tag mutation happens before explicit user approval.
- The saved status distinguishes stale earlier runs from the repaired reroll by `headSha` and ref.
- `release.yml`, `deploy-services.yml`, and `publish-extension.yml` all settle green on the expected candidate tags before T06 starts.

## Inputs

- `compiler/meshc/Cargo.toml`
- `compiler/meshpkg/Cargo.toml`
- `tools/editors/vscode-mesh/package.json`
- `.tmp/m034-s08/tag-rollout/workflow-status.json`
- `.tmp/m034-s08/tag-rollout/tag-refs.txt`
- `.tmp/m034-s08/tag-rollout/rollout_monitor.py`

## Expected Output

- `.tmp/m034-s08/tag-rollout/tag-refs.txt`
- `.tmp/m034-s08/tag-rollout/workflow-status.json`
- `.tmp/m034-s08/tag-rollout/release-v0.1.0-view.json`
- `.tmp/m034-s08/tag-rollout/deploy-services-v0.1.0-view.json`
- `.tmp/m034-s08/tag-rollout/publish-extension-ext-v0.3.0-view.json`

## Verification

bash -c 'set -euo pipefail; test -s .tmp/m034-s08/tag-rollout/tag-refs.txt; grep -F "refs/tags/v0.1.0" .tmp/m034-s08/tag-rollout/tag-refs.txt; grep -F "refs/tags/ext-v0.3.0" .tmp/m034-s08/tag-rollout/tag-refs.txt'
python3 - <<'PY'
import json
from pathlib import Path
summary = json.loads(Path('.tmp/m034-s08/tag-rollout/workflow-status.json').read_text())
expected = {
    'release.yml': 'v0.1.0',
    'deploy-services.yml': 'v0.1.0',
    'publish-extension.yml': 'ext-v0.3.0',
}
for workflow, ref_name in expected.items():
    entry = summary[workflow]
    assert entry['headBranch'] == ref_name, (workflow, entry)
    assert entry['status'] == 'completed', (workflow, entry)
    assert entry['conclusion'] == 'success', (workflow, entry)
    assert entry['headSha'], (workflow, entry)
PY
