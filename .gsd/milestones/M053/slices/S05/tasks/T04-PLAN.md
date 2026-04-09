---
estimated_steps: 1
estimated_files: 10
skills_used: []
---

# T04: Repair the starter-proof path on main and re-close hosted mainline evidence.

Fix the T03 root cause in the workflow/script/test path without weakening the M053 contract. If the failure is diagnostic opacity, harden log retention so nested Rust output survives future hosted failures; if it is timeout or product drift, change the responsible workflow, scripts, or tests so the starter proof stays truthful on clean GitHub runners. Re-run the local starter-proof rails to green, push only the repair commit(s) to remote main, wait for fresh authoritative-verification.yml and deploy-services.yml push runs on the new shipped SHA, and replay bash scripts/verify-m053-s03.sh so the retained hosted bundle shows main is closed again before touching the tag.

## Inputs

- `.tmp/m053-s05/starter-proof-repro/root-cause.md`
- `.tmp/m053-s05/starter-proof-repro/ci-failure-classification.json`
- `.github/workflows/authoritative-starter-failover-proof.yml`
- `.github/workflows/authoritative-verification.yml`
- `scripts/verify-m053-s02.sh`
- `scripts/verify-m053-s01.sh`
- `scripts/verify-m053-s03.sh`

## Expected Output

- `.tmp/m053-s05/rollout/main-shipped-sha.txt`
- `.tmp/m053-s05/rollout/main-workflows.json`
- `.tmp/m053-s03/verify/remote-runs.json`
- `.tmp/m053-s05/rollout/starter-proof-fix-summary.md`

## Verification

bash scripts/verify-m034-s02-workflows.sh && node --test scripts/tests/verify-m053-s03-contract.test.mjs && DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash scripts/verify-m053-s02.sh && test -s .tmp/m053-s05/rollout/main-shipped-sha.txt && python3 - <<'PY'
import json, pathlib
ship = pathlib.Path('.tmp/m053-s05/rollout/main-shipped-sha.txt').read_text().strip()
workflows = {w['workflowFile']: w for w in json.loads(pathlib.Path('.tmp/m053-s03/verify/remote-runs.json').read_text())['workflows']}
auth = workflows['authoritative-verification.yml']
deploy = workflows['deploy-services.yml']
assert auth['status'] == 'ok'
assert auth['observedHeadSha'] == ship
assert deploy['status'] == 'ok'
assert deploy['observedHeadSha'] == ship
PY
