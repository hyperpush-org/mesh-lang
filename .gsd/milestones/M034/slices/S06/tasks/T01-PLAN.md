---
estimated_steps: 3
estimated_files: 5
skills_used: []
---

# T01: Added a stop-after remote-evidence mode to the S05 verifier, shipped the S06 archive helper, and captured the hosted-red preflight bundle.

1. Add a non-destructive remote-evidence-only path for `scripts/verify-m034-s05.sh` plus a slice-owned wrapper `scripts/verify-m034-s06-remote-evidence.sh` that copies the current verify bundle into deterministic `.tmp/m034-s06/evidence/<label>/` directories.
2. Add `scripts/tests/verify-m034-s06-contract.test.mjs` to pin the new operator contract, archive layout, and allowed stop-after phase behavior.
3. Capture the current red hosted state into `.tmp/m034-s06/evidence/preflight/` so later rollout tasks can diff against a known baseline instead of relying on ephemeral `.tmp/m034-s05/verify/`.

## Inputs

- `scripts/verify-m034-s05.sh`
- `scripts/tests/verify-m034-s05-contract.test.mjs`
- `.tmp/m034-s05/verify/remote-runs.json`
- `.tmp/m034-s05/verify/candidate-tags.json`

## Expected Output

- `scripts/verify-m034-s05.sh`
- `scripts/verify-m034-s06-remote-evidence.sh`
- `scripts/tests/verify-m034-s06-contract.test.mjs`
- `.tmp/m034-s06/evidence/preflight/manifest.json`
- `.tmp/m034-s06/evidence/preflight/remote-runs.json`

## Verification

bash -n scripts/verify-m034-s05.sh
node --test scripts/tests/verify-m034-s06-contract.test.mjs
bash scripts/verify-m034-s06-remote-evidence.sh preflight || true
test -f .tmp/m034-s06/evidence/preflight/remote-runs.json
test -f .tmp/m034-s06/evidence/preflight/manifest.json

## Observability Impact

Adds deterministic archived evidence bundles under `.tmp/m034-s06/evidence/<label>/` so future agents can inspect hosted rollout state without rerunning destructive public phases.
