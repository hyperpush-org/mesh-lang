---
estimated_steps: 8
estimated_files: 6
skills_used: []
---

# T02: Recorded the exact synthetic rollout target SHA and approval payload for the S09 hosted reroll.

Why: S08 proved that local `HEAD` contains more than the two original rollout-fix commits, so S09 needs one deliberate target SHA before any outward GitHub action.

Files: `packages-website/Dockerfile`, `.github/workflows/release.yml`, `scripts/verify-m034-s05.sh`, `.tmp/m034-s09/rollout/target-sha.txt`, `.tmp/m034-s09/rollout/remote-refs.before.txt`, `.tmp/m034-s09/rollout/plan.md`

Do:
- Compare `origin/main..HEAD` and isolate the exact commit set that must be shipped for the hosted `release.yml`, `deploy-services.yml`, and freshness-gated `remote-evidence` path.
- Record the current remote refs and the proposed target SHA / tag moves under `.tmp/m034-s09/rollout/`.
- Write the exact outward-action summary the executor will show the user for approval, including which refs move and why, then stop before mutating GitHub.

Verify: `test -s .tmp/m034-s09/rollout/target-sha.txt && test -s .tmp/m034-s09/rollout/remote-refs.before.txt && test -s .tmp/m034-s09/rollout/plan.md`

Done when: the executor has one concrete rollout SHA, a recorded before-state for remote refs, and an unambiguous approval payload that says exactly what will be shipped.

## Inputs

- `packages-website/Dockerfile`
- `.github/workflows/release.yml`
- `scripts/verify-m034-s05.sh`

## Expected Output

- `.tmp/m034-s09/rollout/target-sha.txt`
- `.tmp/m034-s09/rollout/remote-refs.before.txt`
- `.tmp/m034-s09/rollout/plan.md`

## Verification

bash -c 'set -euo pipefail; test -s .tmp/m034-s09/rollout/target-sha.txt; test -s .tmp/m034-s09/rollout/remote-refs.before.txt; test -s .tmp/m034-s09/rollout/plan.md'
