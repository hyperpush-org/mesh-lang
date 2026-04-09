# S03: Release assets and installer truth — UAT

**Milestone:** M034
**Written:** 2026-03-27T00:08:04.965Z

# S03 UAT — Release assets and installer truth

## Preconditions
- Worktree contains the landed S03 changes.
- Local tooling: `bash`, `cargo`, `ruby`, and `rg` are installed.
- No manual cleanup is required under `.tmp/m034-s03/`; the verifier recreates its own staging directories.
- For remote CI parity, GitHub Actions runners provide the target-specific release archives that `verify-release-assets` downloads.

## Test Case 1 — Canonical installer copies stay identical
1. Run `bash -n tools/install/install.sh`.
   - **Expected:** shell syntax check passes.
2. Run `diff -u tools/install/install.sh website/docs/public/install.sh`.
   - **Expected:** exit 0 with no diff output.
3. Run `diff -u tools/install/install.ps1 website/docs/public/install.ps1`.
   - **Expected:** exit 0 with no diff output.

## Test Case 2 — Unix staged installer smoke proves install + runtime truth
1. Run `bash scripts/verify-m034-s03.sh`.
   - **Expected:** the script ends with `verify-m034-s03: ok`.
2. Inspect `.tmp/m034-s03/verify/run/07-meshc-version.log` and `.tmp/m034-s03/verify/run/08-meshpkg-version.log`.
   - **Expected:** the logs contain the staged version for both `meshc` and `meshpkg`.
3. Inspect `.tmp/m034-s03/verify/run/09-hello-build.log` and `.tmp/m034-s03/verify/run/10-hello-run.log`.
   - **Expected:** the installed `meshc` built the checked-in hello fixture successfully and the built binary printed `hello`.
4. Inspect `.tmp/m034-s03/verify/run/00-context.log` and `.tmp/m034-s03/verify/run/staged-layout.txt`.
   - **Expected:** the staged release root includes `meshc`, `meshpkg`, and `SHA256SUMS` for the detected host target.

## Test Case 3 — Strict proof mode fails on the right installer edge cases
1. Reuse the output from `bash scripts/verify-m034-s03.sh`.
2. Inspect these logs under `.tmp/m034-s03/verify/run/`:
   - `11-missing-tag.log`
   - `12-bad-sha.log`
   - `13-missing-meshpkg.log`
   - `14-missing-binary.log`
3. Confirm each log records a hard verifier failure at the exact expected phase.
   - **Expected:**
     - missing `tag_name` fails metadata resolution,
     - malformed `SHA256SUMS` fails checksum proof,
     - missing `meshpkg` archive fails download/install proof,
     - archive missing `meshpkg` binary fails extraction proof.
   - **Expected:** none of these conditions downgrade to warning-only success when strict-proof mode is enabled.

## Test Case 4 — Release workflow publishes the full asset set and blocks publication on installer smoke
1. Run `bash scripts/verify-m034-s02-workflows.sh release`.
   - **Expected:** exits 0 and reports `verify-m034-s02-workflows: ok (release)`.
2. Run `ruby -e 'require "yaml"; YAML.load_file(".github/workflows/release.yml")'`.
   - **Expected:** YAML parses successfully.
3. Run `rg -n 'verify-release-assets|x86_64-pc-windows-msvc' .github/workflows/release.yml`.
   - **Expected:** output shows the Windows target in the build/build-meshpkg/verify-release-assets matrices and the `verify-release-assets` job in the release graph.
4. Open `.github/workflows/release.yml` and confirm `release.needs` includes `verify-release-assets`.
   - **Expected:** `Create Release` is blocked on installer smoke in addition to build/build-meshpkg/authoritative-live-proof.

## Test Case 5 — Public docs tell the verified installer story
1. Run `rg -n 'meshlang.dev/install.sh|meshlang.dev/install.ps1' README.md website/docs/docs/getting-started/index.md website/docs/docs/tooling/index.md tools/editors/vscode-mesh/README.md`.
   - **Expected:** all four documents reference the public installer URLs.
2. Run `! rg -n 'Today the verified install path is building \`meshc\` from source' website/docs/docs/getting-started/index.md`.
   - **Expected:** the stale source-only truth claim is absent.
3. Run `rg -n 'meshc --version|meshpkg --version' README.md website/docs/docs/getting-started/index.md website/docs/docs/tooling/index.md`.
   - **Expected:** the docs explicitly instruct users to verify both installed binaries.
4. Read the surrounding install sections in those docs.
   - **Expected:** source builds are presented as an alternative for contributors or unsupported targets, not as the primary public install path.

## Edge-case follow-up checks
- If `bash scripts/verify-m034-s03.sh` fails, use the phase name in `.tmp/m034-s03/verify/run/*.log` to decide whether the regression is metadata, checksum, archive assembly, installer logic, build output, or runtime behavior.
- If future CI fails only on Windows, compare `scripts/verify-m034-s03.ps1` with the asset names and `SHA256SUMS` generation in `release.yml` before adding any inline YAML workaround; the workflow contract is to reuse the repo-local PowerShell verifier unchanged.
- After the next push, the first live `verify-release-assets` GitHub Actions run should be captured as supporting evidence that the Windows/macOS/Linux smoke matrix works on real hosted runners, not just through local contract verification.
