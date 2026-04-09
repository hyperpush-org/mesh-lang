---
estimated_steps: 25
estimated_files: 6
skills_used: []
---

# T01: Captured a fresh s08-prepush red hosted-evidence bundle, kept first-green unused, and marked the stale v0.1.0 directory as incomplete noise.

The repo already has a misleading occupied `v0.1.0` evidence directory that is missing the files the archive helper requires. This task makes the final capture deterministic before any outward action: confirm the stale directory is incomplete, keep `first-green` unused, and archive one fresh red baseline under a dedicated non-final label so later tasks can distinguish real rollout progress from label-collision noise.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m034-s06-remote-evidence.sh` | Fail closed and inspect the wrapper contract instead of inventing manual archive state. | Treat a hung stop-after replay as a verifier regression and keep the last generated logs. | Treat missing `manifest.json`, `status.txt`, or `remote-runs.json` as archive drift. |
| Existing evidence directories under `.tmp/m034-s06/evidence/` | Preserve them as historical context unless they clearly block the planned label strategy. | N/A | Treat partial bundles like `v0.1.0/` as evidence hygiene problems, not as proof. |
| Hosted run query inside the stop-after replay | Keep the resulting red bundle and use it as the pre-push baseline instead of retrying blindly. | Capture the timeout in the archived logs and stop. | Treat missing workflow entries or wrong refs as baseline truth, not as a local script success. |

## Load Profile

- **Shared resources**: local `.tmp/m034-s06/evidence/` archive tree and one stop-after `verify-m034-s05.sh` replay.
- **Per-operation cost**: one archive-helper execution plus a bounded scan of the occupied evidence labels.
- **10x breakpoint**: repeated archive-helper reruns would mostly create label churn, so the task should claim exactly one disposable pre-push label and keep `first-green` untouched.

## Negative Tests

- **Malformed inputs**: occupied label missing `manifest.json`, missing `status.txt`, or a pre-existing `first-green` directory.
- **Error paths**: archive helper returns non-zero but fails to leave a red bundle, or the stop-after replay drifts into `public-http` / `s01-live-proof`.
- **Boundary conditions**: the new pre-push label must be unique, the stale `v0.1.0` directory must stay clearly non-authoritative, and the final `first-green` label must still be available when the task ends.

## Steps

1. Inspect `.tmp/m034-s06/evidence/v0.1.0/` and record which required archive files are missing so the stale directory is treated as noise, not truth.
2. Confirm `.tmp/m034-s06/evidence/first-green/` does not exist and reserve that label for the final green capture.
3. Run `bash scripts/verify-m034-s06-remote-evidence.sh s08-prepush` through the repo-root `.env` context, expect a red result, and keep the archived bundle as the authoritative pre-tag baseline.
4. If the wrapper or contract tests drift, repair them in this task before any outward tag creation is attempted.

## Must-Haves

- [ ] The task leaves one fresh non-final pre-push bundle at `.tmp/m034-s06/evidence/s08-prepush/`.
- [ ] `first-green` remains unused when the task finishes.
- [ ] The stale `v0.1.0` directory is explicitly recognized as incomplete and non-authoritative.
- [ ] Any wrapper/test drift discovered here is fixed before later tasks rely on the archive contract.

## Inputs

- ``.tmp/m034-s06/evidence/v0.1.0``
- ``scripts/verify-m034-s05.sh``
- ``scripts/verify-m034-s06-remote-evidence.sh``
- ``scripts/tests/verify-m034-s06-contract.test.mjs``

## Expected Output

- ``.tmp/m034-s06/evidence/s08-prepush/manifest.json``
- ``.tmp/m034-s06/evidence/s08-prepush/remote-runs.json``
- ``.tmp/m034-s06/evidence/s08-prepush/status.txt``

## Verification

node --test scripts/tests/verify-m034-s06-contract.test.mjs
bash -c 'set -euo pipefail; test -f .env; set -a; source .env; set +a; rm -rf .tmp/m034-s06/evidence/s08-prepush; if bash scripts/verify-m034-s06-remote-evidence.sh s08-prepush; then echo "expected pre-push bundle to stay red before tags exist" >&2; exit 1; fi'
python3 - <<'PY'
import json
from pathlib import Path
stale = Path('.tmp/m034-s06/evidence/v0.1.0')
assert not (stale / 'manifest.json').exists()
assert not (stale / 'status.txt').exists()
prepush = Path('.tmp/m034-s06/evidence/s08-prepush')
assert (prepush / 'manifest.json').exists()
assert (prepush / 'remote-runs.json').exists()
assert (prepush / 'status.txt').read_text().strip() == 'failed'
manifest = json.loads((prepush / 'manifest.json').read_text())
assert manifest['stopAfterPhase'] == 'remote-evidence'
assert manifest['s05ExitCode'] != 0
assert not Path('.tmp/m034-s06/evidence/first-green').exists()
PY

## Observability Impact

- Signals added/changed: one fresh red baseline bundle showing the exact pre-tag blocker set.
- How a future agent inspects this: read `.tmp/m034-s06/evidence/s08-prepush/{manifest.json,remote-runs.json,phase-report.txt}`.
- Failure state exposed: label collisions, wrapper drift, and missing hosted runs are separated from the older incomplete `v0.1.0` noise.
