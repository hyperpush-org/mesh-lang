# M053 / S05 Research — Hosted workflow evidence closes the starter/packages contract

**Date:** 2026-04-05  
**Status:** Ready for planning

## Summary

S05 is an **operational rollout / hosted-evidence** slice, not a new local workflow-design slice.

What is already true locally:

- `bash scripts/verify-m034-s02-workflows.sh` passes.
- `node --test scripts/tests/verify-m053-s03-contract.test.mjs` passes (`14/14`).
- The local workflow graph and hosted-verifier contract are already wired correctly.

What is still red:

- `GH_TOKEN=<from .env> bash scripts/verify-m053-s03.sh` still fails in `remote-evidence`.
- The failure is **entirely remote-state-driven**, not a local code/contract failure.

Current live blocker state from `.tmp/m053-s03/verify/remote-runs.json`:

- `authoritative-verification.yml` latest green push run on `main` is **23993141627** at SHA **`9d70de9d232f1b210b44b02504826a512a96b475`** and is missing the required `Hosted starter failover proof` job.
- `deploy-services.yml` latest green push run on `main` is also fresh at the same SHA and is already good: it contains `Deploy mesh-packages website` plus `Post-deploy health checks -> Verify public surface contract`.
- `release.yml` fails even earlier: `git ls-remote --quiet origin refs/tags/v0.1.0 'refs/tags/v0.1.0^{}'` returns only `refs/tags/v0.1.0`, so the verifier stops on **missing peeled tag data**.

Two additional operational facts matter:

- `origin/main` is **19 commits behind** local `HEAD`.
- Those 19 commits include both the needed **M053 S01-S04** work **and unrelated M056 `/pitch` work**. A blind `git push origin main` would ship more than this slice.

So the planner should treat S05 as:

1. roll remote `main` forward to a commit containing the full M053 S01-S04 tree,
2. get a fresh mainline `authoritative-verification.yml` run with the starter proof job,
3. reroll the binary tag as an **annotated** tag on the shipped M053 commit so `refs/tags/v0.1.0^{}` exists,
4. wait for a fresh `release.yml` run whose jobs include both `Hosted starter failover proof` and `Create Release`,
5. rerun `bash scripts/verify-m053-s03.sh` to green.

## Requirements Focus

Primary requirements this slice closes in live hosted state:

- **R121** — packages website must be part of the normal CI/deploy contract.
- **R122** — generated Postgres starter must have truthful hosted clustered deploy/failover proof, while SQLite remains local-only.

Supporting constraints still in force:

- **R115** — keep the dual-db starter split honest.
- **R116** — do not re-promote retained proof apps over generated examples.
- **R117** — keep public/docs evaluator-facing, not proof-maze-first.
- **R120** — landing/docs/packages still need one coherent public story, but S05 itself is about hosted proof being green, not rewriting landing.

Concrete S05 completion bar:

- `.tmp/m053-s03/verify/status.txt = ok`
- `.tmp/m053-s03/verify/current-phase.txt = complete`
- `.tmp/m053-s03/verify/remote-runs.json` contains exactly three workflow entries and all are `status: ok`
- `authoritative-verification.yml` on `main` has a green `Hosted starter failover proof` job
- `deploy-services.yml` on `main` has a green `Verify public surface contract` step
- `release.yml` on the current binary tag has a green `Hosted starter failover proof` job and `Create Release`
- the release tag resolves through `refs/tags/<tag>^{}` rather than only a lightweight raw tag ref

## Skills Discovered

Relevant installed skills already present:

- **github-workflows**
  - Relevant rule: workflow work must be treated as an **observable contract**; “YAML parses” is not proof.
  - Planning implication: S05 must prove **before/after remote run state** (run IDs, head SHAs, required jobs/steps), not just finish with a rerun command.
- **gh**
  - Relevant rule: authenticate through `GH_TOKEN` and pass `-R` on every `gh` command.
  - Planning implication: any manual hosted-run inspection or ref mutation should stay repo-explicit and token-backed.

No extra skill installs are needed.

One note from the skills layer: `github-workflows` points to `ci_monitor.cjs`, but this repo does **not** have `scripts/ci_monitor.cjs`. For this slice, the truthful monitoring surface is the repo-owned `scripts/verify-m053-s03.sh` plus explicit `gh run list/view -R hyperpush-org/hyperpush-mono` commands.

## Implementation Landscape

### 1. Local contract surfaces are already green

**Files:**

- `scripts/verify-m053-s03.sh`
- `scripts/tests/verify-m053-s03-contract.test.mjs`
- `.github/workflows/authoritative-starter-failover-proof.yml`
- `.github/workflows/authoritative-verification.yml`
- `.github/workflows/release.yml`
- `.github/workflows/deploy-services.yml`
- `scripts/verify-m034-s02-workflows.sh`

What they already prove locally:

- the new reusable starter-failover workflow exists and is shaped correctly
- `authoritative-verification.yml` calls it on mainline after `whitespace-guard`
- `release.yml` calls it on the binary tag and requires it before `Create Release`
- `deploy-services.yml` still carries the packages/public-surface step
- the hosted verifier expects exactly the right workflow files, required jobs, required steps, freshness markers, and artifact outputs

This means S05 does **not** need more local YAML or verifier design unless remote rollout exposes a real hosted-only failure.

### 2. The live hosted baseline is red for two specific reasons

**Observed via:** `GH_TOKEN=<parsed from .env> bash scripts/verify-m053-s03.sh`

#### Mainline authoritative blocker

`authoritative-verification.yml` currently resolves to:

- run id: **23993141627**
- ref: `refs/heads/main`
- expected sha: **`9d70de9d232f1b210b44b02504826a512a96b475`**
- observed sha: same
- verdict: **failed** because required job `Hosted starter failover proof` is missing

The stored `gh run view` output shows only:

- `Whitespace guard`
- `Authoritative live proof / Authoritative live proof`

So the remote default branch is still on a pre-S03 workflow graph even though the run itself is fresh for the current remote `main` SHA.

#### Release-tag blocker

`release.yml` currently fails before any `gh run view` job inspection because:

- binary tag derived from local Cargo versions is still **`v0.1.0`**
- `git ls-remote --quiet origin refs/tags/v0.1.0 'refs/tags/v0.1.0^{}'` returns only:
  - `74f2d8558b9fe7cd4cf03548e93a101308244db6 refs/tags/v0.1.0`
- there is **no peeled `refs/tags/v0.1.0^{}` ref**

Locally, `git cat-file -t v0.1.0` returns `commit`, which confirms the current tag is lightweight rather than annotated.

There is also already a historical green hosted release run on that tag:

- `gh run list -R hyperpush-org/hyperpush-mono --workflow release.yml --event push --branch v0.1.0 ...`
- latest success run is **23966460116** at SHA **`74f2d8558b9fe7cd4cf03548e93a101308244db6`**

Filtered `gh run view` output for that run shows:

- `Authoritative live proof / Authoritative live proof`
- `Verify release assets (...)`
- `Create Release`

and **does not show** `Authoritative starter failover proof`.

So even after fixing the peeled-ref problem, the current tag still points at an older release workflow graph.

### 3. `deploy-services.yml` is already the green part of the contract

**Current good run:** **23993141615** on SHA `9d70de9d232f1b210b44b02504826a512a96b475`

It already has:

- `Deploy mesh-packages website`
- `Post-deploy health checks`
- required step `Verify public surface contract`

This means S05 does not need to redesign packages deploy verification. It only needs a **fresh rerun on the post-rollout `main` SHA** so the hosted verifier sees the packages proof on the same shipped ref as the starter-proof lane.

### 4. `origin/main` is behind local `HEAD`, but local `HEAD` is broader than S05

`git fetch --quiet origin && git rev-list --left-right --count origin/main...HEAD` returned:

- `0 19`

So local `HEAD` is 19 commits ahead of `origin/main`.

Those 19 commits are mixed:

- unrelated **M056 `/pitch`** work and milestone auto-commits
- all needed **M053 S01-S04** changes, including starter deploy/failover scripts, runtime fixes, workflows, docs, and hosted verifier work

Relevant ahead commit groups:

- M056 `/pitch`: `f57140da` through `232bac57` plus milestone auto-commit wrappers
- M053 S01-S04: `cb4826dc` through `79a030c8`

Planning implication:

- **Pushing local `HEAD` directly would ship unrelated M056 work.**
- If S05 should ship only M053 rollout, create a **minimal rollout branch/commit** from `origin/main` containing the M053 S01-S04 tree instead of all 19 ahead commits.

### 5. Release reroll must carry full M053 content, not just workflow files

This is the most important operational seam.

The release lane does not just read workflow YAML. It runs repo scripts such as:

- `scripts/verify-m053-s02.sh`
- the staged starter/runtime support from S01/S02
- the new reusable hosted starter-proof entrypoint wiring

So a workflow-only synthetic reroll is not sufficient. The tag commit must include the **full M053 S01-S04 code/content**, not only `.github/workflows/`.

Relevant full-tree M053 files that the rollout commit/tag needs to carry include:

- starter proof/runtime files:
  - `compiler/mesh-rt/src/dist/node.rs`
  - `compiler/mesh-rt/src/http/server.rs`
  - `compiler/meshc/tests/e2e_m053_s01.rs`
  - `compiler/meshc/tests/e2e_m053_s02.rs`
  - `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs`
  - `scripts/verify-m053-s01.sh`
  - `scripts/verify-m053-s02.sh`
  - `examples/todo-postgres/deploy/todo-postgres.up.sql`
  - `examples/todo-postgres/scripts/*`
- hosted workflow/verifier files:
  - `.github/workflows/authoritative-starter-failover-proof.yml`
  - `.github/workflows/authoritative-verification.yml`
  - `.github/workflows/release.yml`
  - `scripts/verify-m053-s03.sh`
  - `scripts/tests/verify-m053-s03-contract.test.mjs`
  - `scripts/verify-m034-s02-workflows.sh`
- public-contract files from S04 (if the rollout should reflect full shipped M053 state):
  - `README.md`
  - `website/docs/docs/getting-started/*`
  - `website/docs/docs/distributed*`
  - `website/docs/docs/tooling/index.md`
  - `scripts/verify-m053-s04.sh`
  - `scripts/tests/verify-m053-s04-contract.test.mjs`

## Natural Seams for Planning

### Seam 1 — Reconfirm local readiness before any remote mutation

Use this as preflight only:

- `bash scripts/verify-m034-s02-workflows.sh`
- `node --test scripts/tests/verify-m053-s03-contract.test.mjs`
- `GH_TOKEN=<parsed key> bash scripts/verify-m053-s03.sh` (baseline red capture)

Why first:

- local surfaces are already green and should stay green before remote mutation
- gives one fresh baseline `.tmp/m053-s03/verify/` bundle naming the exact current hosted blockers

### Seam 2 — Create an M053-only rollout target on GitHub

Goal:

- move remote `main` to a commit that contains **all M053 S01-S04 content** but not unrelated M056 `/pitch` work unless intentionally approved

Why separate:

- local `HEAD` is broader than S05
- remote `main` currently lacks all S03/S04 hosted/doc changes and likely the upstream S01/S02 starter proof content those workflows call

Good planner options:

- temp branch from `origin/main` + cherry-pick or replay only the M053 commits/files
- GitHub-created synthetic rollout commit/tree if the repo wants to avoid directly pushing local history

Historical prior-art constraint from earlier hosted-rollout work:

- do **not** retarget a tag to a commit that only exists locally
- do **not** ship only workflow-file deltas and assume the tagged release proof is valid

### Seam 3 — Mainline hosted rerun and verification

After remote `main` moves:

- wait for fresh `authoritative-verification.yml` push run on the new main SHA
- wait for fresh `deploy-services.yml` push run on the same SHA
- inspect with `gh run list/view -R hyperpush-org/hyperpush-mono`

Success condition for this seam:

- `authoritative-verification.yml` includes the `Authoritative starter failover proof` reusable job and it is green
- `deploy-services.yml` is green on the same ref and still includes `Verify public surface contract`

At this point, rerunning `bash scripts/verify-m053-s03.sh` should leave only the release-tag side red.

### Seam 4 — Binary tag reroll as an annotated tag

Goal:

- satisfy both release freshness requirements:
  - fresh hosted `release.yml` run on the shipped M053 commit
  - existence of `refs/tags/v0.1.0^{}` in `git ls-remote`

Because the current local/remote binary version is still `0.1.0`, the planner must explicitly choose one of two paths:

1. **retag/reroll `v0.1.0`** to the shipped M053 commit as an **annotated** tag, or
2. change Cargo versions and create a new binary tag

Repo evidence currently points to path 1 as the existing operational seam; there is no local version-bump work in S05 scope.

Important constraint:

- the current `v0.1.0` tag is already used by an older release run, so rerolling it is a **real ref mutation**, not a fresh tag create
- if the repo wants the verifier green without widening scope into versioning work, the reroll must be deliberate and should be treated as the slice’s main operational action

### Seam 5 — Final green replay and closeout

Once remote main and the annotated binary tag are green:

- rerun `GH_TOKEN=<parsed key> bash scripts/verify-m053-s03.sh`
- confirm:
  - `status.txt = ok`
  - `current-phase.txt = complete`
  - `phase-report.txt` includes `remote-evidence	passed` and `artifact-contract	passed`
  - `remote-runs.json` shows all three workflows as `status: ok`

Then update slice closeout/state files rather than changing the verifier contract.

## Verification

### Commands already run during research

- `bash scripts/verify-m034-s02-workflows.sh` ✅
- `node --test scripts/tests/verify-m053-s03-contract.test.mjs` ✅
- `GH_TOKEN=<parsed from .env> bash scripts/verify-m053-s03.sh` ❌ expected hosted-red baseline
- `git fetch --quiet origin && git rev-list --left-right --count origin/main...HEAD` → `0 19`
- `git ls-remote --quiet origin refs/tags/v0.1.0 'refs/tags/v0.1.0^{}'` → only raw tag ref, no peeled ref
- `gh run list -R hyperpush-org/hyperpush-mono --workflow release.yml --event push --branch v0.1.0 ...` → latest success run `23966460116` on old SHA `74f2d855...`

### Recommended final verification sequence for executors

1. Local preflight
   - `bash scripts/verify-m034-s02-workflows.sh`
   - `node --test scripts/tests/verify-m053-s03-contract.test.mjs`

2. Remote baseline capture
   - `GH_TOKEN=<parsed key> bash scripts/verify-m053-s03.sh`
   - inspect `.tmp/m053-s03/verify/remote-runs.json`

3. After main rollout
   - use `gh run list/view -R hyperpush-org/hyperpush-mono` to confirm fresh mainline run IDs and head SHAs
   - rerun `bash scripts/verify-m053-s03.sh`
   - expect release-tag side only to remain red

4. After annotated tag reroll
   - `git ls-remote --quiet origin refs/tags/v0.1.0 'refs/tags/v0.1.0^{}'`
   - confirm both raw and peeled refs exist
   - inspect fresh `release.yml` hosted run for starter proof + create release
   - rerun `GH_TOKEN=<parsed key> bash scripts/verify-m053-s03.sh`

5. Final artifact checks
   - `.tmp/m053-s03/verify/status.txt` is `ok`
   - `.tmp/m053-s03/verify/current-phase.txt` is `complete`
   - `.tmp/m053-s03/verify/remote-runs.json` contains three `status: ok` entries

## Risks and Watchouts

- **Biggest scope trap:** pushing local `HEAD` directly ships unrelated M056 `/pitch` work along with M053.
- **Biggest operational trap:** fixing only the workflows on remote GitHub is not enough. The tagged commit must carry the full M053 starter proof scripts/runtime changes.
- **Release-tag trap:** the verifier specifically requires a peeled `refs/tags/<tag>^{}` ref. Recreating another lightweight tag will keep S05 red.
- **Hidden release trap:** after adding the peeled ref, the release lane can still fail if the tag still points at the old pre-S03 commit. The current green run `23966460116` already proves that.
- **Observability trap:** do not infer success from GitHub UI summaries alone. Follow the `github-workflows` skill rule and prove run IDs, head SHAs, required jobs, and required step names explicitly.

## Planner Recommendation

Treat S05 as a **two-stage hosted rollout** rather than as a coding slice:

1. **Create and ship an M053-only rollout target to remote `main`.**  
   Keep unrelated M056 work out unless intentionally approved.

2. **Reroll the binary tag as an annotated tag on that shipped M053 commit.**  
   This is what turns both the mainline starter-proof job and the release peeled-ref check green.

Only after those two hosted mutations succeed should the executor spend time on any local file edits. Right now the local verifier/tests already say the repo-side contract is ready; the remaining work is remote state convergence.