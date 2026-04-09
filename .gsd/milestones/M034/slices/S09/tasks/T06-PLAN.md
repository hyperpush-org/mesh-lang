---
estimated_steps: 5
estimated_files: 6
skills_used: []
---

# T06: Archive first-green exactly once and rerun the full assembled verifier

With the repaired hosted evidence set green on the fresh rollout SHA, close the slice with the canonical assembled proof.
- Load `.env`, rerun the stop-after `remote-evidence` preflight, and confirm it is green on the repaired refs and `headSha`.
- If `.tmp/m034-s06/evidence/first-green/` is still absent, capture it exactly once through `scripts/verify-m034-s06-remote-evidence.sh` and validate the archived manifest plus remote-run summary.
- Run the full `bash scripts/verify-m034-s05.sh` replay with `.env` loaded and confirm `remote-evidence`, `public-http`, and `s01-live-proof` all pass on the repaired hosted state.
- Check the final proof bundle for `status.txt == ok`, a complete phase report, `public-http.log`, and the S01 package-version evidence so the slice demo is preserved in one truthful bundle.

## Inputs

- `scripts/verify-m034-s05.sh`
- `scripts/verify-m034-s06-remote-evidence.sh`
- `.tmp/m034-s09/rollout/workflow-status.json`
- `.tmp/m034-s09/rollout/target-sha.txt`
- `.env`

## Expected Output

- `.tmp/m034-s06/evidence/first-green/manifest.json`
- `.tmp/m034-s05/verify/status.txt`
- `.tmp/m034-s05/verify/phase-report.txt`
- `.tmp/m034-s05/verify/public-http.log`

## Verification

bash -c 'set -euo pipefail; test -f .env; set -a; source .env; set +a; VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh'
bash -c 'set -euo pipefail; test -f .env; set -a; source .env; set +a; bash scripts/verify-m034-s06-remote-evidence.sh first-green'
bash -c 'set -euo pipefail; test -f .env; set -a; source .env; set +a; bash scripts/verify-m034-s05.sh'
python3 - <<'PY'
from pathlib import Path
import json
root = Path('.tmp/m034-s06/evidence/first-green')
assert (root / 'manifest.json').exists()
assert Path('.tmp/m034-s05/verify/status.txt').read_text().strip() == 'ok'
manifest = json.loads((root / 'manifest.json').read_text())
assert manifest['s05ExitCode'] == 0, manifest
assert manifest['stopAfterPhase'] == 'remote-evidence', manifest
public_log = Path('.tmp/m034-s05/verify/public-http.log')
assert public_log.exists(), public_log
assert any(Path('.tmp/m034-s01/verify').rglob('package-version.txt'))
print('assembled proof bundle is complete')
PY
