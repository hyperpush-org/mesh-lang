# S07: Public surface freshness and final assembly replay — UAT

**Milestone:** M034
**Written:** 2026-03-27T08:28:00.273Z

# S07 UAT — Public surface freshness and final assembly replay

**Milestone:** M034
**Written:** 2026-03-27

## Preconditions
- Work from the repo root with GitHub CLI authenticated for `snowdamiz/mesh-lang`.
- `.env` must exist locally so the canonical S05 replay can source the publish credentials without printing them.
- `curl`, `python3`, `node`, and `npm` must be available on the host.

## Test Case 1 — Local public-surface contract guards stay green
1. Run `python3 -m py_compile scripts/lib/m034_public_surface_contract.py`.
   - Expected: exit code `0`.
2. Run `bash -n scripts/verify-m034-s05.sh`.
   - Expected: exit code `0`.
3. Run `node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s07-public-contract.test.mjs`.
   - Expected: all tests pass.
4. Run `bash scripts/verify-m034-s05-workflows.sh`.
   - Expected: `verify-m034-s05-workflows: ok (all)`.

## Test Case 2 — Canonical S05 replay still fail-closes at the hosted boundary
1. Run `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh`.
   - Expected: the command exits non-zero after all local phases through `s04-workflows` pass and `remote-evidence` fails.
2. Open `.tmp/m034-s05/verify/status.txt`.
   - Expected: exactly `failed`.
3. Open `.tmp/m034-s05/verify/current-phase.txt`.
   - Expected: exactly `remote-evidence`.
4. Open `.tmp/m034-s05/verify/phase-report.txt`.
   - Expected: every phase through `s04-workflows` is `passed`, `remote-evidence` is `failed`, and there are **no** `public-http` or `s01-live-proof` entries.

## Test Case 3 — Hosted rollout state is still stale on GitHub
1. Run `gh api repos/snowdamiz/mesh-lang/branches/main --jq '.commit.sha'`.
   - Expected: `8d3e76a65986e7bd5e00d107f9e11c1923ecd0ab`.
2. Run `gh run list -R snowdamiz/mesh-lang --workflow deploy.yml --event push --branch main --limit 1 --json databaseId,status,conclusion,headSha,url`.
   - Expected: one green run on `8d3e76a65986e7bd5e00d107f9e11c1923ecd0ab`.
3. Inspect `.tmp/m034-s05/verify/remote-runs.json` or run `gh run view` on that deploy run.
   - Expected: the `build` job is still missing `Verify public docs contract`.
4. Run `gh run list -R snowdamiz/mesh-lang --workflow authoritative-verification.yml --event push --branch main --limit 1 --json databaseId,status,conclusion,headSha,url`.
   - Expected: GitHub returns `workflow authoritative-verification.yml not found on the default branch`.
5. Run the same `gh run list` command for `release.yml` and `deploy-services.yml` on branch `v0.1.0`, and `publish-extension.yml` on branch `ext-v0.3.0`.
   - Expected: each command returns `[]` because the candidate tags still do not have truthful push runs.

## Test Case 4 — Direct live public-surface contract reaches the network and shows real stale content
1. Run `python3 scripts/lib/m034_public_surface_contract.py public-http --root . --artifact-dir .tmp/m034-s07-public-http-uat`.
   - Expected: the command exits non-zero **after reaching the live HTTPS surfaces** (no local certificate-validation failure) and leaves `.tmp/m034-s07-public-http-uat/public-http.log` plus per-surface diff/check artifacts.
2. Open `.tmp/m034-s07-public-http-uat/public-http.log`.
   - Expected:
     - `public-package-detail`, `public-package-search`, and `public-registry-search` are `passed`.
     - `public-install-sh`, `public-install-ps1`, `public-getting-started`, and `public-tooling` are `failed`.
3. Open `.tmp/m034-s07-public-http-uat/public-install-sh.diff` and `.tmp/m034-s07-public-http-uat/public-install-ps1.diff`.
   - Expected: the live installers are still the older meshc-only/publicly stale versions rather than the repo’s current meshc+meshpkg installers.
4. Open `.tmp/m034-s07-public-http-uat/public-getting-started-check.log` and `.tmp/m034-s07-public-http-uat/public-tooling-check.log`.
   - Expected: they list the missing S07 markers (`install.sh`, `install.ps1`, `meshpkg --version`, workflow/runbook markers, candidate-tag/remote-runs artifacts, etc.).

## Test Case 5 — Do not mistake stale S01 artifacts for a current green replay
1. Run `find .tmp/m034-s01/verify -mindepth 2 -maxdepth 2 -name package-version.txt | grep -q .`.
   - Expected: this may still succeed from an older S01 run.
2. Cross-check that result with `.tmp/m034-s05/verify/status.txt`, `.tmp/m034-s05/verify/current-phase.txt`, and `.tmp/m034-s05/verify/phase-report.txt` from Test Case 2.
   - Expected: the current replay is still red at `remote-evidence`, so any existing `package-version.txt` must **not** be treated as proof that `s01-live-proof` passed in the current run.

## Edge Cases
- If the direct `public-http` command fails with a local TLS/certificate error instead of stale-body / missing-marker diagnostics, the helper transport regressed; it should use curl-backed fetches on this host.
- If a future rerun makes `remote-evidence` pass, the next honest check is whether `public-http` also turns green from the same replay; do not skip directly to `s01-live-proof`.
- A green `deploy.yml` run without `Verify public docs contract` is stale hosted evidence, not success; compare required steps/jobs before treating remote rollout as current.

