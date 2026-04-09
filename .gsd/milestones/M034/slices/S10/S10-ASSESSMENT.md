# S10 blocker assessment — 2026-03-28

## Status
S10 is **not complete**. Local remediation progressed, but the slice-level hosted verification contract still fails.

## What changed in this unit
- Confirmed all local T01/T02 verification surfaces still pass:
  - `DATABASE_URL='postgres://postgres:postgres@127.0.0.1:55433/postgres' cargo test --manifest-path registry/Cargo.toml latest -- --nocapture`
  - `bash scripts/tests/verify-m034-s01-fetch-retry.sh`
  - `cargo test -p mesh-codegen link -- --nocapture`
  - `pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1`
  - `bash scripts/verify-m034-s02-workflows.sh`
- Reconfirmed the only failing slice gate was hosted `release.yml`.
- Downloaded the failed Windows release-smoke diagnostics from the old tag run and identified that the hosted `verify-release-assets` Windows job had **no LLVM toolchain installed** before asking installed `meshc.exe` to compile the smoke fixture.
- Patched the local workflow surface:
  - `.github/workflows/release.yml` now installs LLVM 21 for the Windows smoke verifier and exports `LLVM_SYS_211_PREFIX` before `scripts/verify-m034-s03.ps1`.
  - `scripts/verify-m034-s02-workflows.sh` now makes those Windows LLVM steps part of the release-workflow contract.
- After explicit user approval, rolled out a **workflow-only synthetic remote commit** via GitHub API and retargeted `v0.1.0` to it.

## Current remote truth
- New remote rollout SHA: `e59f18203a30951af5288791bf9aed5b53a24a2a`
- Local durable target updated: `.tmp/m034-s09/rollout/target-sha.txt`
- Fresh hosted runs on that SHA:
  - `authoritative-verification.yml` main run `23667365836` — **success**
  - `release.yml` tag run `23667370566` — **failure**

## Why S10 is still blocked
The remote rollout used in this unit only changed `.github/workflows/release.yml`. It did **not** carry the rest of the local S10 slice changes onto GitHub.

That means the new tag run still executes an older repo tree for the verifier/code surfaces. The failure set on `23667370566` proves this:
- `Verify release assets (x86_64-pc-windows-msvc)` still failed.
- `Authoritative live proof / Authoritative live proof` also failed inside `release.yml` on the same SHA.

This is the key finding: **rolling out only the workflow file is insufficient**. The next unit must roll out the full local S10 content, not just the workflow YAML, before hosted truth can converge.

## Evidence captured
- Old Windows diagnostics proving missing-LLVM hosted setup: `.tmp/m034-s10/release-artifact/windows/verify/run/07-hello-build.log` and downloaded rerun logs under `.tmp/m034-s10/release-artifact/`
- New hosted rerun artifacts for the synthetic SHA: `.tmp/m034-s10/release-artifact-rerun/`
- Fresh failing tag run log: GitHub Actions run `23667370566`

## Resume instructions for the next unit
1. Start from the current local tree, which already contains the intended S10 code/workflow fixes.
2. Do **not** assume the current remote SHA contains those local changes; it only carries the workflow-only synthetic commit.
3. Roll out the **full** local S10 content to GitHub (not just `.github/workflows/release.yml`).
4. After rollout, refresh hosted evidence for:
   - `authoritative-verification.yml`
   - `release.yml` on `v0.1.0`
5. Rebuild these durable artifacts from the refreshed hosted state:
   - `.tmp/m034-s05/verify/remote-runs.json`
   - `.tmp/m034-s09/rollout/workflow-status.json`
6. If `release.yml` still fails after a full rollout, inspect the new downloaded diagnostics before changing code again. The current `release-artifact-rerun` directory is from a workflow-only remote commit and should not be treated as proof against the full local slice.
