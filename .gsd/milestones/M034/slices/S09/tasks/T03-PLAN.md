---
estimated_steps: 8
estimated_files: 5
skills_used: []
---

# T03: Retargeted `main`, `v0.1.0`, and `ext-v0.3.0` to the approved rollout SHA and captured hosted workflow evidence up to the red `publish-extension.yml` blocker.

Why: the verifier hardening in T01 only matters if `main`, `v0.1.0`, and `ext-v0.3.0` are actually rerun on the intended rollout SHA, and `R047` still depends on the extension lane staying inside that hosted evidence set.

Files: `scripts/verify-m034-s05.sh`, `.tmp/m034-s09/rollout/plan.md`, `.tmp/m034-s09/rollout/remote-refs.after.txt`, `.tmp/m034-s09/rollout/workflow-status.json`, `.tmp/m034-s09/rollout/workflow-urls.txt`

Do:
- Show the recorded rollout summary and get explicit user confirmation before any remote mutation.
- Push or retarget `main`, `v0.1.0`, and `ext-v0.3.0` onto the approved SHA using the least-destructive path allowed by the current remote state, then record the resulting ref map.
- Monitor `deploy.yml`, `authoritative-verification.yml`, `release.yml`, `deploy-services.yml`, `extension-release-proof.yml`, and `publish-extension.yml` until they are green on the expected refs and `headSha`, persisting the final URLs and status payloads.

Verify: `python3 - <<'PY' ... workflow-status.json ... PY` plus `git ls-remote` checks for the updated refs.

Done when: the remote refs and the saved hosted-workflow status payloads all agree on the intended SHA, and every required workflow is completed/success on the correct ref.

## Inputs

- `scripts/verify-m034-s05.sh`
- `.tmp/m034-s09/rollout/target-sha.txt`
- `.tmp/m034-s09/rollout/remote-refs.before.txt`
- `.tmp/m034-s09/rollout/plan.md`

## Expected Output

- `.tmp/m034-s09/rollout/remote-refs.after.txt`
- `.tmp/m034-s09/rollout/workflow-status.json`
- `.tmp/m034-s09/rollout/workflow-urls.txt`

## Verification

bash -c 'set -euo pipefail; test -s .tmp/m034-s09/rollout/remote-refs.after.txt; test -s .tmp/m034-s09/rollout/workflow-status.json; test -s .tmp/m034-s09/rollout/workflow-urls.txt'

## Observability Impact

- Signals added/changed: `.tmp/m034-s09/rollout/workflow-status.json` and `.tmp/m034-s09/rollout/workflow-urls.txt` record per-workflow ref, `headSha`, status, conclusion, and URLs.
- How a future agent inspects this: compare `.tmp/m034-s09/rollout/remote-refs.before.txt` to `remote-refs.after.txt`, then inspect `workflow-status.json` for any non-green lane.
- Failure state exposed: wrong-ref pushes, stale reruns, and still-red hosted workflows are preserved as durable rollout artifacts instead of ephemeral terminal output.
