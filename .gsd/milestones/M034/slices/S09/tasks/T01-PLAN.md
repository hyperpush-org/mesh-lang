---
estimated_steps: 8
estimated_files: 4
skills_used: []
---

# T01: Enforced remote-evidence headSha freshness and preserved the new SHA contract in archived manifests.

Why: `R045` is still weak if `scripts/verify-m034-s05.sh` accepts a green run that only matches the branch/tag name while the hosted `headSha` is stale.

Files: `scripts/verify-m034-s05.sh`, `scripts/verify-m034-s06-remote-evidence.sh`, `scripts/tests/verify-m034-s05-contract.test.mjs`, `scripts/tests/verify-m034-s06-contract.test.mjs`

Do:
- Extend remote-evidence so each required workflow resolves the expected ref SHA for `main`, `v0.1.0`, and `ext-v0.3.0` and compares it against the hosted run's `headSha`.
- Persist expected SHA, observed SHA, mismatch reason, and latest-available run context into `remote-runs.json` and the S06 archive manifest so stale-green failures are self-explanatory.
- Update the Node contract tests so the verifier and archive helper both cover stale-sha failure, reusable workflow naming, and preserved artifact shape.

Verify: `node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s06-contract.test.mjs`

Done when: stop-after `remote-evidence` can fail closed on stale hosted runs even when workflow/job names are otherwise green, and the archive contract preserves the extra freshness context.

## Inputs

- `scripts/verify-m034-s05.sh`
- `scripts/verify-m034-s06-remote-evidence.sh`
- `scripts/tests/verify-m034-s05-contract.test.mjs`
- `scripts/tests/verify-m034-s06-contract.test.mjs`

## Expected Output

- `scripts/verify-m034-s05.sh`
- `scripts/verify-m034-s06-remote-evidence.sh`
- `scripts/tests/verify-m034-s05-contract.test.mjs`
- `scripts/tests/verify-m034-s06-contract.test.mjs`

## Verification

node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s06-contract.test.mjs

## Observability Impact

- Signals added/changed: `.tmp/m034-s05/verify/remote-runs.json` and archived `manifest.json` now record expected ref SHAs, observed run SHAs, and freshness-specific failure reasons.
- How a future agent inspects this: rerun stop-after `remote-evidence` and inspect `.tmp/m034-s05/verify/remote-runs.json` or `.tmp/m034-s06/evidence/<label>/manifest.json`.
- Failure state exposed: stale-run acceptance becomes an explicit mismatch instead of a silent green.
