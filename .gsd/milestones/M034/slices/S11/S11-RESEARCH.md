# S11 Research — First-green archive and final assembly closeout

## Summary

S11 is **not** a pure evidence-copy slice yet. The repo-owned closeout wrappers are healthy locally, but the hosted release story is still blocked by fixes that exist only in the local tree.

Current state from the repo and saved hosted artifacts:

- `.tmp/m034-s05/verify/current-phase.txt` is still `remote-evidence` and `status.txt` is `failed`.
- `.tmp/m034-s06/evidence/first-green/` is still **absent**, which is correct — that label remains unspent.
- `origin/main` is `6979a4a17221af8e39200b574aa2209ad54bc983`, while local `HEAD` is `6378afa87c5a5b8f2f1a8d9333cbd77069b2c31c` (`git rev-list --left-right --count origin/main...HEAD` = `0 15`). The closeout-relevant fixes are not fully rolled out.
- Local commits ahead of `origin/main` include:
  - `ccb106cf` — registry latest-version/search monotonicity repair
  - `363f815b` — target-aware Windows linker/runtime selection
  - `5e457f3c` — release smoke workflow repair work
- There is still an **uncommitted** local diff in `.github/workflows/release.yml` and `scripts/verify-m034-s02-workflows.sh` adding Windows LLVM setup for the staged smoke verifier.
- The canonical remote-evidence artifact `.tmp/m034-s05/verify/remote-runs.json` currently says:
  - `deploy.yml`: ok
  - `deploy-services.yml`: ok
  - `authoritative-verification.yml`: ok
  - `release.yml`: failed on `v0.1.0` / `e59f18203a30951af5288791bf9aed5b53a24a2a`
  - `extension-release-proof.yml`: ok on `ext-v0.3.0` / `8e6d49dacc4f4cd64824b032078ae45aabfe9635`
  - `publish-extension.yml`: ok on `ext-v0.3.0` / `8e6d49dacc4f4cd64824b032078ae45aabfe9635`
- The saved failed hosted release run (`23667370566`) is more specific than the slice summary alone: it has **two** red proof jobs:
  - `Authoritative live proof / Run authoritative live proof` failed with `search results did not expose the published version`
  - `Verify release assets (x86_64-pc-windows-msvc) / Verify staged installer assets (Windows)` failed with `installed meshc.exe build installer smoke fixture failed`

That combination matches the code that is fixed locally but not fully represented in the current hosted ref graph.

## Requirement Focus

- **R007** is the direct requirement surface for S11. The final assembled replay must prove the real publish/install/search/download path and preserve its hosted evidence in one truthful archive.
- This slice also supports the milestone-level CI/release-closeout bar from M034 context (`R045` / `R046`), but the concrete blocker remains the R007 publish/search/release assembly path.

## Skills Discovered

Relevant installed skills already exist; no new skill install was needed.

- **`github-workflows`** — relevant for the hosted Actions surfaces and the “prove observable before/after change” rule. Caveat: the skill expects `scripts/ci_monitor.cjs`, but this repo does **not** contain that file, so the practical path here is the repo-local verifier scripts plus read-only `gh` inspection.
- **`powershell-windows`** — relevant for `scripts/verify-m034-s03.ps1`. The existing strict-mode-safe `$LASTEXITCODE` handling is already the right pattern and should be preserved if the Windows smoke path is touched again.

## Local Proof State

The repo-owned contract surfaces are currently healthy locally:

- `node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s06-contract.test.mjs` → pass
- `bash scripts/verify-m034-s02-workflows.sh` → pass
- `bash scripts/verify-m034-s05-workflows.sh` → pass
- `bash -n scripts/verify-m034-s05.sh scripts/verify-m034-s06-remote-evidence.sh scripts/verify-m034-s03.sh` → pass

That means S11 is blocked by rollout/hosted truth, not by a broken local closeout wrapper.

## Implementation Landscape

### `scripts/verify-m034-s05.sh`
Canonical assembled verifier. Important behavior:

- Resets `.tmp/m034-s05/verify` on every run.
- Derives split candidate tags from live source files:
  - binary tag from `compiler/meshc/Cargo.toml` and `compiler/meshpkg/Cargo.toml`
  - extension tag from `tools/editors/vscode-mesh/package.json`
- Reuses prior slice verifiers in order:
  - `scripts/verify-m034-s05-workflows.sh`
  - docs build + local/built doc truth
  - `scripts/verify-m034-s02-workflows.sh`
  - `scripts/verify-m034-s03.sh`
  - `scripts/verify-m034-s04-extension.sh`
  - `scripts/verify-m034-s04-workflows.sh`
  - remote evidence
  - public HTTP truth
  - `.env`-backed S01 live proof
- Supports `VERIFY_M034_S05_STOP_AFTER=remote-evidence` (or `--stop-after remote-evidence`) and exits cleanly before `public-http` / `s01-live-proof`.

This is the **authoritative** first-green gate for S11.

### `scripts/verify-m034-s06-remote-evidence.sh`
Archive wrapper around the S05 stop-after mode.

- Runs `scripts/verify-m034-s05.sh` with `VERIFY_M034_S05_STOP_AFTER=remote-evidence`
- Requires `candidate-tags.json`, `remote-runs.json`, `status.txt`, `current-phase.txt`, and `phase-report.txt`
- Copies the whole verify root into `.tmp/m034-s06/evidence/<label>/`
- Writes `manifest.json` with a condensed `remoteRunsSummary`
- **Fails closed on label reuse** before invoking S05

For S11, `first-green` must be used **exactly once** and only after stop-after `remote-evidence` is already green.

### `scripts/verify-m034-s02-workflows.sh`
Repo-local contract gate for `authoritative-verification.yml` and `release.yml`.

Current local patch extends the release workflow contract to require:

- `Install LLVM 21 for Windows smoke verifier`
- `Set LLVM prefix for Windows smoke verifier`

This file must stay in sync with `.github/workflows/release.yml`. Right now both are locally modified together.

### `.github/workflows/release.yml`
Still the riskiest file for S11.

Relevant shape:

- `build` matrix builds `meshc`
- `build-meshpkg` matrix builds `meshpkg`
- `authoritative-live-proof` reuses `authoritative-live-proof.yml`
- `verify-release-assets` runs the staged installer proof on every release artifact target
- `release` depends on `build`, `build-meshpkg`, `authoritative-live-proof`, and `verify-release-assets`

Current local uncommitted diff adds the Windows LLVM install/prefix steps before:

- `cargo build -q -p mesh-rt`
- `pwsh -NoProfile -File scripts/verify-m034-s03.ps1`

That diff is not reflected in the current hosted `v0.1.0` run.

### `scripts/verify-m034-s03.ps1`
Windows staged installer smoke verifier.

Important details to preserve:

- `Set-StrictMode -Version Latest`
- strict-mode-safe `Get-Variable LASTEXITCODE -ErrorAction SilentlyContinue` instead of direct `$LASTEXITCODE` reads
- staged release layout under `.tmp/m034-s03/windows/verify`
- final build step is `installed meshc.exe build installer smoke fixture`

The hosted failure is exactly at that last build step, so if S11 still needs a Windows fix after rollout, this is the diagnostic entrypoint.

### `registry/src/db/packages.rs`, `registry/src/routes/search.rs`, `registry/src/routes/metadata.rs`
These are the local code fixes that explain the remote authoritative-live-proof failure.

- `packages.rs` now recomputes `latest_version` from committed `versions` rows under a row lock instead of last-writer-wins update order
- `search.rs` and `metadata.rs` now fail closed when the latest-version join is missing, and tests pin the monotonic latest behavior

That lines up directly with the saved hosted failure string from the release run: `search results did not expose the published version`.

### `.tmp/m034-s09/rollout/monitor_workflows.py` and `workflow-status.json`
Useful, but **not authoritative** for S11 closeout.

This monitor assumes one `TARGET_SHA` for `main`, `v0.1.0`, and `ext-v0.3.0`. That is stricter than the split-tag S05 contract. Today it reports the extension workflows as pending because `ext-v0.3.0` still points at `8e6d...` while `target-sha.txt` is `e59f...`, even though the canonical S05 `remote-runs.json` correctly marks the extension workflows as green on their own tag.

Use this monitor for rollout diagnostics, not for deciding whether `first-green` may be claimed.

## Natural Seams for Planning

### 1. Rollout blocker retirement
Highest-risk seam. This is where the slice can still turn into real implementation work.

Scope:

- finish the dirty `release.yml` / `verify-m034-s02-workflows.sh` patch
- ensure the local registry/search fix and Windows smoke fixes are represented on the remote refs that S05 actually checks
- do not assume old hosted red runs are evidence against the current local tree

### 2. Hosted proof refresh
Once rollout truth is corrected, rerun the canonical stop-after gate:

- `VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh`

Success condition: all six workflows in `.tmp/m034-s05/verify/remote-runs.json` have `status: ok` on their expected refs.

### 3. One-shot archive capture
After the stop-after gate is green:

- `bash scripts/verify-m034-s06-remote-evidence.sh first-green`

Success condition:

- `.tmp/m034-s06/evidence/first-green/manifest.json` exists
- `s05ExitCode == 0`
- `s05Status == ok`
- every `remoteRunsSummary[*].status == ok`

### 4. Full final assembly replay
After `first-green` is captured:

- `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh`

Success condition:

- `.tmp/m034-s05/verify/status.txt` = `ok`
- `.tmp/m034-s05/verify/current-phase.txt` = `complete`
- S05 gets past `remote-evidence` and `public-http` and finishes with `s01-live-proof`

## Risks and Constraints

- **External mutation approval is required.** Any push, tag retarget, tag recreation, workflow rerun that changes GitHub state, or similar outward action needs explicit user confirmation.
- **`first-green` is one-shot.** The wrapper refuses overwrite. Use disposable labels for red preflights only; do not spend `first-green` until the stop-after gate is green.
- **S05 deletes its own working artifact root.** `scripts/verify-m034-s05.sh` wipes `.tmp/m034-s05/verify` at start. Anything worth preserving must be archived before the next run.
- **Do not use `workflow-status.json` as the final `first-green` gate.** It still encodes a single-SHA rollout assumption that conflicts with the documented split binary/extension tag model in `README.md` and `website/docs/docs/tooling/index.md`.
- **The current best Windows failure evidence is hosted.** Local `.tmp/m034-s03/windows/verify/run/07-hello-build.log` is absent in this checkout, so the freshest specific failure text is still from the GitHub job log.
- **PowerShell script hygiene still matters.** If `scripts/verify-m034-s03.ps1` changes again, keep the existing strict-mode-safe patterns from the `powershell-windows` skill: no implicit `$LASTEXITCODE` assumptions, preserve ASCII-only output, and keep variable extraction explicit.

## What to Prove First

1. **Cheap local contract truth**
   - `node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s06-contract.test.mjs`
   - `bash scripts/verify-m034-s02-workflows.sh`
   - `bash scripts/verify-m034-s05-workflows.sh`

2. **Hosted blocker moved**
   - `VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh`

3. **One-shot archive only after green stop-after**
   - `bash scripts/verify-m034-s06-remote-evidence.sh first-green`

4. **Full closeout replay**
   - `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh`

## Recommendation

Plan S11 as a **blocker-retirement + closeout** slice, not as a pure archive task.

Recommended execution order:

1. Finish the remaining local release-workflow patch and keep the S02 workflow verifier aligned.
2. Roll the local committed fixes that matter for hosted truth onto the remote refs with explicit approval:
   - registry latest-version/search repair
   - Windows linker/runtime repair
   - release workflow Windows smoke-toolchain patch
3. Re-run the canonical stop-after `remote-evidence` gate until `release.yml` turns green in `.tmp/m034-s05/verify/remote-runs.json`.
4. Claim `.tmp/m034-s06/evidence/first-green/` exactly once with the archive wrapper.
5. Run the full `.env`-backed S05 replay and use its `status.txt` / `current-phase.txt` plus the `first-green` manifest as milestone-closeout evidence.

One important planning detail: **do not force the extension tag to match the binary/main rollout SHA unless the extension surface itself changed.** The authoritative S05 verifier already treats binary and extension tags independently, and the docs explicitly describe that split identity. The stricter single-SHA monitor under `.tmp/m034-s09/rollout/` is still useful for rollout diagnostics, but it should not be the gate for spending `first-green`.

## Verification Commands

Local contract and syntax gates:

```bash
node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s06-contract.test.mjs
bash scripts/verify-m034-s02-workflows.sh
bash scripts/verify-m034-s05-workflows.sh
bash -n scripts/verify-m034-s05.sh scripts/verify-m034-s06-remote-evidence.sh scripts/verify-m034-s03.sh
```

Hosted preflight / blocker refresh:

```bash
VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh
```

Authoritative archive capture once green:

```bash
bash scripts/verify-m034-s06-remote-evidence.sh first-green
```

Final assembled replay:

```bash
set -a && source .env && set +a && bash scripts/verify-m034-s05.sh
```

Useful inspection snippets:

```bash
python3 - <<'PY'
import json
from pathlib import Path
artifact = json.loads(Path('.tmp/m034-s05/verify/remote-runs.json').read_text())
bad = {entry['workflowFile']: entry['status'] for entry in artifact['workflows'] if entry['status'] != 'ok'}
print(bad)
PY
```

```bash
python3 - <<'PY'
import json
from pathlib import Path
manifest = json.loads(Path('.tmp/m034-s06/evidence/first-green/manifest.json').read_text())
print(manifest['s05Status'], manifest['currentPhase'])
for entry in manifest['remoteRunsSummary']:
    print(entry['workflowFile'], entry['status'])
PY
```

## Sources

Files and artifacts read during research:

- `scripts/verify-m034-s05.sh`
- `scripts/verify-m034-s06-remote-evidence.sh`
- `scripts/verify-m034-s05-workflows.sh`
- `scripts/tests/verify-m034-s05-contract.test.mjs`
- `scripts/tests/verify-m034-s06-contract.test.mjs`
- `.github/workflows/release.yml`
- `.github/workflows/authoritative-verification.yml`
- `scripts/verify-m034-s03.ps1`
- `scripts/tests/verify-m034-s03-last-exitcode.ps1`
- `scripts/verify-m034-s01.sh`
- `registry/src/db/packages.rs`
- `registry/src/routes/search.rs`
- `registry/src/routes/metadata.rs`
- `.tmp/m034-s05/verify/remote-runs.json`
- `.tmp/m034-s05/verify/remote-evidence.log`
- `.tmp/m034-s09/rollout/monitor_workflows.py`
- `.tmp/m034-s09/rollout/workflow-status.json`
- `.tmp/m034-s09/rollout/target-sha.txt`
- `README.md`
- `website/docs/docs/tooling/index.md`

Read-only hosted inspection used during research:

- `gh run view 23667370566 --job 68953890968 --log-failed -R snowdamiz/mesh-lang`
- `gh run view 23667370566 --job 68952587340 --log-failed -R snowdamiz/mesh-lang`
- `git ls-remote --tags origin refs/tags/v0.1.0 refs/tags/v0.1.0^{} refs/tags/ext-v0.3.0 refs/tags/ext-v0.3.0^{}`
