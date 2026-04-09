# S06: Hosted rollout evidence capture — UAT

**Milestone:** M034
**Written:** 2026-03-27T06:05:39.861Z

# S06 UAT — Hosted rollout evidence capture

## Preconditions
- Work from the repo root with GitHub CLI authenticated for `snowdamiz/mesh-lang`.
- Use a **new** archive label that does not already exist under `.tmp/m034-s06/evidence/`.
- Do **not** use the reserved `first-green` label unless the hosted workflows are actually all green.

## Test Case 1 — Contract and local workflow guards stay green
1. Run `bash -n scripts/verify-m034-s05.sh`.
   - Expected: exit code `0`.
2. Run `node --test scripts/tests/verify-m034-s06-contract.test.mjs`.
   - Expected: all 5 tests pass.
3. Run `bash scripts/verify-m034-s05-workflows.sh`.
   - Expected: `verify-m034-s05-workflows: ok (all)`.
4. Run `bash scripts/verify-m034-s02-workflows.sh`.
   - Expected: `verify-m034-s02-workflows: ok (all)`.
5. Run `bash scripts/verify-m034-s04-workflows.sh`.
   - Expected: `verify-m034-s04-workflows: ok (all)`.

## Test Case 2 — Capture a fresh hosted blocker snapshot without destroying prior evidence
1. Pick a new label, for example `uat-remote-check`.
2. Run `bash scripts/verify-m034-s06-remote-evidence.sh uat-remote-check || true`.
   - Expected: the command prints `archive: .tmp/m034-s06/evidence/uat-remote-check` and exits non-zero because hosted rollout is still blocked.
3. Open `.tmp/m034-s06/evidence/uat-remote-check/manifest.json`.
   - Expected:
     - `stopAfterPhase` is `remote-evidence`.
     - `failedPhase` is `remote-evidence`.
     - `gitRefs.binaryTag` is `v0.1.0`.
     - `gitRefs.extensionTag` is `ext-v0.3.0`.
4. Open `.tmp/m034-s06/evidence/uat-remote-check/remote-runs.json`.
   - Expected:
     - `deploy.yml` is `failed` because the latest remote `main` run is missing `build: Verify public docs contract`.
     - `authoritative-verification.yml` is `failed` because the workflow is missing on the remote default branch.
     - `release.yml` / `deploy-services.yml` have no `push` runs on `v0.1.0`.
     - `extension-release-proof.yml` / `publish-extension.yml` have no `push` runs on `ext-v0.3.0`.

## Test Case 3 — Confirm the remote branch and candidate tags are still stale
1. Run `gh api repos/snowdamiz/mesh-lang/branches/main --jq '.commit.sha'`.
   - Expected: `5ddf3b2dce17abe08e1188d9b46e575d83525b50`.
2. Run `gh run list -R snowdamiz/mesh-lang --workflow deploy.yml --event push --branch main --limit 1 --json databaseId,status,conclusion,headSha,url`.
   - Expected: one green run on `5ddf3b2dce17abe08e1188d9b46e575d83525b50`.
3. Run `gh run list -R snowdamiz/mesh-lang --workflow authoritative-verification.yml --event push --branch main --limit 1 --json databaseId,status,conclusion,headSha,url`.
   - Expected: GitHub returns `workflow authoritative-verification.yml not found on the default branch`.
4. Run `gh run list -R snowdamiz/mesh-lang --workflow release.yml --event push --branch v0.1.0 --limit 1 --json databaseId,status,conclusion,headSha,url`.
   - Expected: `[]`.
5. Run `gh run list -R snowdamiz/mesh-lang --workflow deploy-services.yml --event push --branch v0.1.0 --limit 1 --json databaseId,status,conclusion,headSha,url`.
   - Expected: `[]`.
6. Run `gh run list -R snowdamiz/mesh-lang --workflow publish-extension.yml --event push --branch ext-v0.3.0 --limit 1 --json databaseId,status,conclusion,headSha,url`.
   - Expected: `[]`.

## Test Case 4 — Full S05 still stops at the hosted-rollout boundary
1. Run `bash scripts/verify-m034-s05.sh || test "$(cat .tmp/m034-s05/verify/failed-phase.txt)" = "remote-evidence"`.
   - Expected: the compound command succeeds because the verifier fail-closes at `remote-evidence`.
2. Open `.tmp/m034-s05/verify/phase-report.txt`.
   - Expected:
     - phases through `s04-workflows` are `passed`.
     - `remote-evidence` is `failed`.
     - there are **no** `public-http` or `s01-live-proof` entries, proving the hosted blocker still happens first.

## Test Case 5 — Transport blocker evidence is still actionable
1. Open `.tmp/m034-s06/push-main.stderr`.
   - Expected: it shows a chunked `git-receive-pack` attempt ending in `HTTP 408`.
2. Open `.tmp/m034-s06/transport-recovery/attempts.log`.
   - Expected: it records the bounded HTTP/1.1 retry attempts, durations, target SHA, and stdout/stderr paths.
3. Open `.tmp/m034-s06/transport-recovery/03-http11-postbuffer-1g.stderr`.
   - Expected: it shows `POST git-receive-pack (564496785 bytes)` followed by the same `HTTP 408`, proving that increasing `http.postBuffer` did not fix the rollout path.

## Edge Cases
- **Label reuse fails closed:** `bash scripts/verify-m034-s06-remote-evidence.sh preflight` should fail immediately with `archive label already exists` and must not overwrite the preserved baseline bundle.
- **Missing hosted runs still archive truthfully:** when a branch/tag has no matching run, the bundle should contain `remote-<workflow>-list.*` plus `remote-<workflow>-latest-available.*`; a missing `remote-<workflow>-view.*` file is expected in that case, not a bug.
- **Reusable extension proof uses the caller run:** if you inspect extension hosted evidence manually, use `publish-extension.yml` on the tag and require the `Verify extension release proof` job. Direct standalone polling of `extension-release-proof.yml` is not the authoritative surface because the workflow is `workflow_call`-only.
