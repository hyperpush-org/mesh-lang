---
estimated_steps: 10
estimated_files: 7
skills_used: []
---

# T06: Capture the authoritative `first-green` hosted-evidence bundle and validate its manifest.

Once T05 proves the repaired candidate tags are green, preserve that truth through the repo-owned wrapper exactly once. Reconfirm that `.tmp/m034-s06/evidence/first-green/` is still unused, rerun the S05/S06 contract tests if any wrapper or workflow-contract code changed earlier in the slice, then run `scripts/verify-m034-s06-remote-evidence.sh first-green` exactly once from the authenticated repo root. Validate the archived manifest and copied verifier artifacts so milestone closeout can consume the bundle directly without another hosted query.

Steps:
1. Confirm `first-green` is absent and that T05's `workflow-status.json` shows all required workflows green on the expected refs and `headSha`.
2. Rerun the S05/S06 contract tests if earlier tasks touched the wrapper or workflow-contract logic.
3. Run the canonical wrapper once with the reserved label and validate `status.txt`, `current-phase.txt`, `phase-report.txt`, `manifest.json`, and `remote-runs.json`.
4. Leave the archive in place as the single authoritative first-green bundle for milestone closeout.

Must-haves:
- `first-green` is claimed exactly once.
- The wrapper stays the sole owner of the final hosted-evidence proof path.
- The archived manifest and remote-run summary both show all required workflows green.

## Inputs

- `scripts/verify-m034-s06-remote-evidence.sh`
- `scripts/tests/verify-m034-s05-contract.test.mjs`
- `scripts/tests/verify-m034-s06-contract.test.mjs`
- `.tmp/m034-s08/tag-rollout/workflow-status.json`
- `.env`

## Expected Output

- `.tmp/m034-s06/evidence/first-green/manifest.json`
- `.tmp/m034-s06/evidence/first-green/remote-runs.json`
- `.tmp/m034-s06/evidence/first-green/phase-report.txt`

## Verification

node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s06-contract.test.mjs
bash -c 'set -euo pipefail; test -f .env; set -a; source .env; set +a; bash scripts/verify-m034-s06-remote-evidence.sh first-green'
python3 - <<'PY'
import json
from pathlib import Path
root = Path('.tmp/m034-s06/evidence/first-green')
assert (root / 'status.txt').read_text().strip() == 'ok'
assert (root / 'current-phase.txt').read_text().strip() == 'stopped-after-remote-evidence'
phase_report = (root / 'phase-report.txt').read_text()
for needle in ['candidate-tags\tpassed', 'remote-evidence\tpassed']:
    assert needle in phase_report, needle
manifest = json.loads((root / 'manifest.json').read_text())
assert manifest['s05ExitCode'] == 0
assert manifest['stopAfterPhase'] == 'remote-evidence'
for entry in manifest['remoteRunsSummary']:
    assert entry['status'] == 'ok', entry
PY
