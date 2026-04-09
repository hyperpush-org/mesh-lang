---
estimated_steps: 1
estimated_files: 8
skills_used: []
---

# T05: Reroll the annotated binary tag on the repaired shipped SHA and close the hosted verifier.

After T04 makes main green again, recreate and push v0.1.0 as an annotated tag on the repaired shipped SHA, verify both raw and peeled refs resolve, wait for the fresh tag-triggered release.yml run to finish with Hosted starter failover proof plus Create Release, and rerun bash scripts/verify-m053-s03.sh to green. If the final hosted replay still finds drift, record the exact blocker in .tmp/m053-s05/rollout/final-blocker.md before changing code or verifier expectations.

## Inputs

- `.tmp/m053-s05/rollout/main-shipped-sha.txt`
- `.github/workflows/release.yml`
- `scripts/verify-m053-s03.sh`
- `.tmp/m053-s03/verify/remote-runs.json`

## Expected Output

- `.tmp/m053-s03/verify/status.txt`
- `.tmp/m053-s03/verify/current-phase.txt`
- `.tmp/m053-s03/verify/remote-runs.json`
- `.tmp/m053-s05/rollout/release-workflow.json`
- `.tmp/m053-s05/rollout/final-blocker.md`

## Verification

GH_TOKEN=${GH_TOKEN:?set GH_TOKEN} bash scripts/verify-m053-s03.sh && python3 - <<'PY'
import json, pathlib
verify = pathlib.Path('.tmp/m053-s03/verify')
assert verify.joinpath('status.txt').read_text().strip() == 'ok'
assert verify.joinpath('current-phase.txt').read_text().strip() == 'complete'
workflows = json.loads(verify.joinpath('remote-runs.json').read_text())['workflows']
assert len(workflows) == 3 and all(w['status'] == 'ok' for w in workflows)
PY

git ls-remote --quiet origin refs/tags/v0.1.0 'refs/tags/v0.1.0^{}'
