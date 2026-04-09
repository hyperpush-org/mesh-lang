# S08 Research — Hosted rollout completion and first-green evidence

## Summary

S08 is now an execution slice, not a broad code-change slice.

Fresh stop-after evidence at `.tmp/m034-s06/evidence/s08-scout-20260327/` shows the old `main` rollout blocker from S07 is gone:

- `deploy.yml` is now green on remote `main` at `6979a4a17221af8e39200b574aa2209ad54bc983`.
  - run: `23639371864`
  - required step present: `build / Verify public docs contract`
- `authoritative-verification.yml` is now green on remote `main` at the same SHA.
  - run: `23639371918`

The remaining blocker is narrower and operational:

- `deploy-services.yml` has **no hosted push run** on `v0.1.0`
- `release.yml` has **no hosted push run** on `v0.1.0`
- `extension-release-proof.yml` has **no hosted push run** on `ext-v0.3.0`
- `publish-extension.yml` has **no hosted push run** on `ext-v0.3.0`

The direct ref check matches that result:

- `git tag -l v0.1.0 ext-v0.3.0` → `local_tags=<none>`
- `git ls-remote --tags origin v0.1.0 ext-v0.3.0` → `remote_tags=<none>`

So the current slice is not blocked by workflow drift on `main`. It is blocked because the candidate tags that S05/S06 expect have not been created and pushed yet.

One local trap also matters: `.tmp/m034-s06/evidence/v0.1.0/` already exists, but it is **incomplete** (logs only; no `manifest.json`, `status.txt`, or `current-phase.txt`). The archive wrapper still refuses to reuse that label, so S08 needs explicit evidence-label hygiene before capturing a final green bundle.

## Skills Discovered

Relevant installed skills already present:

- `github-workflows`
- `gh`

No new skills were installed.

Notes from those skills that matter here:

- The `github-workflows` skill says the authoritative path should be one repo-owned wrapper instead of ad hoc CLI checks. In this repo, that wrapper is `scripts/verify-m034-s05.sh` plus `scripts/verify-m034-s06-remote-evidence.sh`.
- The skill references `scripts/ci_monitor.cjs`, but this repo does **not** contain that file. Do not design S08 around a missing helper.
- The `gh` skill rule to always pass `-R` is already encoded in the verifier. Keep using the repo-owned scripts instead of improvising raw `gh` calls.

## Implementation Landscape

### Primary verifier ownership

#### `scripts/verify-m034-s05.sh`
Canonical assembled verifier.

What matters for S08:

- `run_candidate_tags()` derives the required candidate refs directly from repo truth:
  - binary tag: `v0.1.0`
  - extension tag: `ext-v0.3.0`
- `run_remote_evidence()` is the actual hosted-proof gate.
- `--stop-after remote-evidence` is a first-class public interface, not a hack.
- `remote-evidence` runs **after** all local verifier reuse (`s02`, `s03`, `s04`, docs checks) and **before** `public-http` / `s01-live-proof`.

That means S08 should reuse this entrypoint, not fork the logic.

#### `scripts/verify-m034-s06-remote-evidence.sh`
Archive wrapper for S08-style evidence capture.

Important behavior:

- runs `scripts/verify-m034-s05.sh` with `VERIFY_M034_S05_STOP_AFTER=remote-evidence`
- requires these artifacts to exist before archiving:
  - `candidate-tags.json`
  - `remote-runs.json`
  - `phase-report.txt`
  - `status.txt`
  - `current-phase.txt`
- copies the verifier output into `.tmp/m034-s06/evidence/<label>/`
- generates `manifest.json`
- **fails closed** if the label already exists
- preserves red bundles too, so it can be used before final green

This is the correct S08 evidence entrypoint.

#### `scripts/tests/verify-m034-s06-contract.test.mjs`
Best concise statement of the archive contract.

It locks in:

- explicit stop-after boundary
- reusable proof derivation through `publish-extension.yml`
- red-bundle archiving semantics
- fail-closed behavior when required artifacts are missing
- fail-closed behavior when the label already exists

If S08 changes archive behavior, update this test in the same task.

### Workflow-owned truth surfaces

#### `.github/workflows/deploy.yml`
Triggers on:

- `push` to `main`
- `push` tags `v*`
- `workflow_dispatch`

Hosted proof requirement in S05:

- run on `main`
- jobs `build`, `deploy`
- `build` job must contain `Verify public docs contract`

Current truth: **already green on remote `main`**.

#### `.github/workflows/authoritative-verification.yml`
Triggers on:

- `push` to `main`
- `pull_request`
- `workflow_dispatch`
- `schedule`

Hosted proof requirement in S05:

- run on `main`
- required job `Authoritative live proof`

Current truth: **already green on remote `main`**.

#### `.github/workflows/release.yml`
Triggers on:

- `push` to `main`
- `push` tags `v*`
- `pull_request`

Hosted proof requirement in S05 for the candidate tag:

- event `push`
- head branch/tag `v0.1.0`
- required jobs:
  - `Authoritative live proof`
  - `Create Release`
- required job prefixes:
  - `Build (`
  - `Build meshpkg (`
  - `Verify release assets (`

Current truth: workflow exists and runs on `main`, but there is **no hosted push run on `v0.1.0`**.

#### `.github/workflows/deploy-services.yml`
Triggers on:

- `push` tags `v*`
- `workflow_dispatch`

Hosted proof requirement in S05 for the candidate tag:

- event `push`
- head branch/tag `v0.1.0`
- required jobs:
  - `Deploy mesh-registry`
  - `Deploy mesh-packages website`
  - `Post-deploy health checks`
- required step:
  - `Post-deploy health checks / Verify public surface contract`

Current truth: workflow exists, but there is **no hosted push run on `v0.1.0`**.

#### `.github/workflows/publish-extension.yml`
Triggers on:

- `push` tags `ext-v*`

Hosted proof requirement in S05 for the candidate tag:

- event `push`
- head branch/tag `ext-v0.3.0`
- required jobs:
  - `Verify extension release proof`
  - `Publish verified extension`

Current truth: workflow exists, but there is **no hosted push run on `ext-v0.3.0`**.

#### `.github/workflows/extension-release-proof.yml`
Reusable `workflow_call`-only proof workflow.

Important nuance from the verifier/test:

- S05 does **not** query this workflow file directly as an independently triggered run.
- It queries `publish-extension.yml`, then derives proof from the caller run and tolerates reusable-workflow job names like:
  - `Publish Extension / Verify extension release proof`

Current truth: the reusable proof lane exists, but there is **no caller run on `ext-v0.3.0`** yet.

### Shared public-surface helper

#### `scripts/lib/m034_public_surface_contract.py`
Still important for S05, but **not the current S08 blocker**.

Relevant only insofar as:

- `deploy.yml` and `deploy-services.yml` are already wired to the shared helper
- local docs/build/public contract drift is no longer what is blocking hosted evidence

## Current Hosted Truth

Freshest source of truth:

- `.tmp/m034-s06/evidence/s08-scout-20260327/manifest.json`
- `.tmp/m034-s06/evidence/s08-scout-20260327/remote-runs.json`

### Green now

- `deploy.yml` on `main` → **ok**
  - run URL: `https://github.com/snowdamiz/mesh-lang/actions/runs/23639371864`
  - head SHA: `6979a4a17221af8e39200b574aa2209ad54bc983`
- `authoritative-verification.yml` on `main` → **ok**
  - run URL: `https://github.com/snowdamiz/mesh-lang/actions/runs/23639371918`
  - head SHA: `6979a4a17221af8e39200b574aa2209ad54bc983`

### Still missing

- `deploy-services.yml` on `v0.1.0` → no hosted run
  - latest available tag run is stale: `v15.0`
- `release.yml` on `v0.1.0` → no hosted run
  - latest available run is on `main`, not the candidate tag
- `extension-release-proof.yml` / `publish-extension.yml` on `ext-v0.3.0` → no hosted run
  - latest available extension-tag run is stale: `ext-v0.2.0`

### Ref truth

Candidate tags are derived locally but not materialized anywhere yet:

- local tags: none
- remote tags: none

That is the simplest explanation for the missing hosted runs.

## Recommendation

1. **Do not start by editing verifier or workflow code.**
   The local verifier stack is already in the right shape, and remote `main` now proves the rollout landed.

2. **Treat S08 as an execution/evidence slice first.**
   The first work should be:
   - confirm the intended target commit for tag creation (currently remote `main` is at local `HEAD` `6979a4a17221af8e39200b574aa2209ad54bc983`)
   - resolve the evidence-label collision risk
   - create/push the missing candidate tags
   - wait for hosted runs
   - archive the first green bundle

3. **Handle label hygiene before the final capture.**
   `.tmp/m034-s06/evidence/v0.1.0/` is incomplete but still occupies the label. Either:
   - remove that incomplete local temp directory after confirming it is junk, or
   - choose a different final archive label and document the mapping clearly in the slice summary

4. **External GitHub actions require explicit user confirmation.**
   Creating and pushing `v0.1.0` / `ext-v0.3.0` is outward-facing. Under the GSD contract, that cannot be done without a clear yes from the user.

5. **Only investigate code/workflow drift if the tag-triggered runs fail after push.**
   Right now the evidence does not justify more code churn.

## Natural Seams

### Seam 1 — Evidence-label hygiene
Local only.

Scope:

- inspect `.tmp/m034-s06/evidence/v0.1.0/`
- decide whether to clean it up or avoid the label
- preserve the ability to capture one definitive green bundle without archive-wrapper collision

Likely files:

- `.tmp/m034-s06/evidence/v0.1.0/`
- `scripts/verify-m034-s06-remote-evidence.sh` only if the current fail-closed contract proves too rigid (not currently indicated)

### Seam 2 — Hosted rollout execution
External-state step.

Scope:

- create/push `v0.1.0`
- create/push `ext-v0.3.0`
- wait for hosted runs to finish
- inspect run results if any fail

Likely surfaces:

- Git refs only
- GitHub Actions hosted runs for:
  - `release.yml`
  - `deploy-services.yml`
  - `publish-extension.yml`

No repo file edits are obviously required before trying this.

### Seam 3 — Green bundle capture and validation
Local artifact production after the external runs succeed.

Scope:

- rerun `scripts/verify-m034-s06-remote-evidence.sh <final-label>`
- confirm manifest and archived remote summaries are all green
- preserve the bundle for slice closeout / milestone validation

Likely files/artifacts:

- `.tmp/m034-s06/evidence/<label>/manifest.json`
- `.tmp/m034-s06/evidence/<label>/remote-runs.json`
- `.tmp/m034-s06/evidence/<label>/phase-report.txt`

## Risks and Gotchas

- **Archive labels are single-use.** `scripts/verify-m034-s06-remote-evidence.sh` will fail before rerunning the verifier if the destination label already exists.
- **There is already a misleading occupied label.** `.tmp/m034-s06/evidence/v0.1.0/` exists but is incomplete; that can confuse a naïve “use the tag as the label” approach.
- **Derived tags are not self-creating.** `candidate-tags.json` proves what the tags should be; it does not create them.
- **Remote evidence uses exact event/ref matching.** Green `main` runs do not satisfy the tag-owned workflows.
- **Extension proof is derived from the caller run.** If `publish-extension.yml` does not run on `ext-v0.3.0`, the reusable proof lane cannot pass S05/S06 even if the reusable workflow file exists.
- **Do not bypass the repo-owned verifier with ad hoc `gh` logic.** The scripts already encode the exact jobs, step names, reusable-workflow suffix handling, and failure messages that closeout expects.

## Verification

If S08 stays operational and does not edit code:

- authoritative hosted snapshot:
  - `bash -c 'set -euo pipefail; test -f .env; set -a; source .env; set +a; bash scripts/verify-m034-s06-remote-evidence.sh <label>'`

Green conditions for the archived bundle:

- `.tmp/m034-s06/evidence/<label>/status.txt` = `ok`
- `.tmp/m034-s06/evidence/<label>/current-phase.txt` = `stopped-after-remote-evidence`
- `.tmp/m034-s06/evidence/<label>/phase-report.txt` contains:
  - `candidate-tags	passed`
  - `remote-evidence	passed`
- `.tmp/m034-s06/evidence/<label>/manifest.json` has:
  - `s05ExitCode = 0`
  - `stopAfterPhase = "remote-evidence"`
  - `remoteRunsSummary[*].status = "ok"`

If archive-wrapper behavior changes:

- `node --test scripts/tests/verify-m034-s06-contract.test.mjs`

If S05 remote-evidence logic changes:

- `node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s06-contract.test.mjs`

## Planner Takeaway

Plan S08 as a short, high-consequence operational slice:

- **T01:** local artifact-hygiene / evidence-label decision
- **T02:** user-confirmed candidate tag creation + hosted run monitoring
- **T03:** final green evidence capture with `verify-m034-s06-remote-evidence.sh` and manifest validation

Do not spend context on speculative workflow edits until the missing tag-triggered runs have actually been attempted.
