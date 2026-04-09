---
id: S03
parent: M053
milestone: M053
provides:
  - A reusable hosted starter failover proof lane that mainline and release workflows can call without new secrets.
  - One slice-owned hosted verifier that binds fresh starter deploy evidence and fresh packages/public-surface evidence into the same contract.
  - A retained remote-evidence bundle under `.tmp/m053-s03/verify/` that downstream slices can inspect instead of re-deriving hosted drift manually.
requires:
  - slice: S02
    provides: `bash scripts/verify-m053-s02.sh` and the retained `.tmp/m053-s02/**` diagnostics contract for the serious generated Postgres starter failover proof.
affects:
  - M053/S04
key_files:
  - .github/workflows/authoritative-starter-failover-proof.yml
  - .github/workflows/authoritative-verification.yml
  - .github/workflows/release.yml
  - scripts/verify-m034-s02-workflows.sh
  - scripts/verify-m053-s03.sh
  - scripts/tests/verify-m053-s03-contract.test.mjs
  - .gsd/KNOWLEDGE.md
  - .gsd/PROJECT.md
key_decisions:
  - D407: Host the serious starter failover proof in its own secret-free reusable workflow with runner-local Postgres, call it from authoritative-verification after whitespace-guard, and require it in tag release gating alongside authoritative-live-proof.
  - D408: Derive hosted-verifier GitHub repo slugs from `git remote get-url origin` by default and keep an explicit override for fixtures/unusual remotes.
patterns_established:
  - Use a dedicated reusable workflow with runner-local Postgres and failure artifact upload for long-running serious starter proof, rather than squeezing it into an unrelated live publish lane.
  - Treat workflow topology as a local contract with a fail-closed verifier plus fixture-backed tests before trusting remote GitHub state.
  - Model hosted proof freshness as `git ls-remote` expected refs plus `gh run list/view` job-step evidence, and fail closed on stale main/tag state or missing required jobs/steps.
observability_surfaces:
  - Hosted reusable workflow artifact upload: `.tmp/m053-s02/**` as `authoritative-starter-failover-proof-diagnostics` on failure.
  - Hosted verifier artifacts under `.tmp/m053-s03/verify/`: `status.txt`, `current-phase.txt`, `phase-report.txt`, `candidate-refs.json`, `remote-runs.json`, `full-contract.log`, and per-query `gh`/`git` logs.
  - Explicit hosted contract markers: `Hosted starter failover proof` job presence in authoritative/release evidence and `Verify public surface contract` in `deploy-services.yml` mainline evidence.
drill_down_paths:
  - .gsd/milestones/M053/slices/S03/tasks/T01-SUMMARY.md
  - .gsd/milestones/M053/slices/S03/tasks/T02-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-05T21:07:54.277Z
blocker_discovered: false
---

# S03: Hosted evidence chain fails on starter deploy or packages drift

**S03 made the generated Postgres starter failover proof and the packages-website public-surface proof part of one fail-closed hosted contract, then proved the live hosted state now goes red when either side drifts.**

## What Happened

S03 closed the gap between the serious starter deploy proof and the hosted release/deploy story.

## What actually shipped

- Added `.github/workflows/authoritative-starter-failover-proof.yml` as a dedicated secret-free reusable workflow for `bash scripts/verify-m053-s02.sh`.
- Gave that hosted lane its own runner-local `postgres:16` service, masked `DATABASE_URL` export, startup wait, and failure artifact upload for `.tmp/m053-s02/**`.
- Wired the new reusable workflow into `.github/workflows/authoritative-verification.yml` as a whitespace-guarded mainline lane and into `.github/workflows/release.yml` as a tag-gated prerequisite alongside the existing authoritative live proof.
- Extended `scripts/verify-m034-s02-workflows.sh` so the local contract sweep now fails closed if the reusable workflow, caller wiring, release prerequisites, service shape, timeouts, permissions, or diagnostics upload surface drift.
- Extended `scripts/tests/verify-m053-s03-contract.test.mjs` into the slice-owned topology + hosted-evidence suite. It now covers the fresh-green hosted path plus negative cases for missing GH auth, stale SHAs, tag-only deploy-services evidence, missing starter-proof jobs, missing `Verify public surface contract`, malformed `gh` JSON, and missing final artifacts.
- Added `scripts/verify-m053-s03.sh` as the hosted evidence verifier that derives the GitHub repo slug from `origin`, resolves fresh `main` and current binary-tag refs with `git ls-remote`, queries `gh run list/view`, and records `candidate-refs.json`, `remote-runs.json`, phase markers, and per-query logs under `.tmp/m053-s03/verify/`.

## What the live replay proved

The slice goal was not just to add green-path plumbing; it was to make the hosted chain fail when the serious starter lane or packages/public-surface truth drifts. The live replay did that truthfully.

- `deploy-services.yml` on `main` is currently fresh and still exposes both `Deploy mesh-packages website` and the `Post-deploy health checks -> Verify public surface contract` step.
- The latest green `authoritative-verification.yml` run on `main` is fresh by SHA, but it predates the new hosted starter lane and is therefore missing the required `Hosted starter failover proof` job.
- The remote `v0.1.0` tag currently does not expose the peeled `refs/tags/v0.1.0^{}` ref that the hosted verifier requires for release freshness, so the release branch of the contract also fails closed.

That means the hosted evidence chain is now doing the right thing: it no longer lets packages/public-surface proof or starter deploy proof live as parallel side surfaces.

## Patterns established

1. Keep long-running serious starter deploy/failover proof in its own reusable workflow with runner-local dependencies and artifact upload instead of stretching an unrelated publish lane.
2. Lock reusable-workflow topology locally with an explicit contract verifier and fast fixture-backed tests before depending on remote GitHub state.
3. For hosted evidence, combine remote ref freshness (`git ls-remote`) with `gh run list/view` job-step assertions and fail closed on stale mainline runs, tag-only evidence, or missing required jobs/steps.

## Operational Readiness

- **Health signal:** local `bash scripts/verify-m034-s02-workflows.sh` stays green; hosted `.tmp/m053-s03/verify/remote-runs.json` shows matching `expectedHeadSha` / `observedHeadSha`, required jobs, and required steps.
- **Failure signal:** `.tmp/m053-s03/verify/status.txt=failed`, `current-phase.txt` names the stop point, `phase-report.txt` shows the last completed phase, and `remote-runs.json` names the missing job/step or stale ref. Hosted starter failures also upload `.tmp/m053-s02/**` through the reusable workflow artifact.
- **Recovery procedure:** rerun the hosted workflows after the remote `main` graph includes the starter lane and after the current release tag exposes the peeled ref; inspect the per-query logs in `.tmp/m053-s03/verify/*.log` before changing verifier expectations.
- **Monitoring gaps:** the live verifier stops before `artifact-contract` when `remote-evidence` fails, so only the contract tests cover that final phase in the red-state path; there is no automatic backfill for historical green runs that predate the new workflow graph.


## Verification

Ran all slice verification rails.

- `bash scripts/verify-m034-s02-workflows.sh` ✅ passed.
- `node --test scripts/tests/verify-m053-s03-contract.test.mjs` ✅ passed (`14/14` green).
- `GH_TOKEN=<parsed from .env> bash scripts/verify-m053-s03.sh` ✅ exercised the live hosted contract and intentionally failed closed in `remote-evidence`, which is the expected proof shape for this slice because current hosted state still drifts from the shipped contract.

Live hosted evidence retained under `.tmp/m053-s03/verify/` showed:

- `status.txt=failed`
- `current-phase.txt=remote-evidence`
- `phase-report.txt` recorded `gh-preflight` and `candidate-refs` as passed before stopping in `remote-evidence`
- `remote-runs.json` showed fresh `deploy-services.yml` proof on `main`, but flagged the missing `Hosted starter failover proof` job on the latest green `authoritative-verification.yml` run and the missing peeled `refs/tags/v0.1.0^{}` ref for release freshness
- per-query logs were retained for `gh` and `git ls-remote` inspection

That combination proves the assembled slice now fails on the exact starter/packages drift it was supposed to police.

## Requirements Advanced

- R121 — Made `deploy-services.yml` mainline packages-website deployment and `Verify public surface contract` evidence a required, freshness-checked input to the hosted verification chain instead of a parallel side surface.
- R122 — Made hosted CI/release fail closed when the generated Postgres starter failover proof lane is missing, stale, or detached from the authoritative workflow graph.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

No code-scope deviation from the slice plan. During live verification, the hosted verifier had to parse only `GH_TOKEN` from `.env` instead of `source .env`, and it now derives the repo slug from `git remote get-url origin` because this repository moved to `hyperpush-org/hyperpush-mono`.

## Known Limitations

The live hosted verifier is still red against real remote state until the remote `main` workflow graph includes the new starter-proof lane and the current release tag exposes the peeled ref required for freshness checks. Because `remote-evidence` failed first, the live replay did not reach the final `artifact-contract` phase; that end-state remains covered by the fixture-backed contract test suite.

## Follow-ups

1. Re-run `authoritative-verification.yml` on `main` after rollout so the latest green hosted run includes `Hosted starter failover proof`.
2. Ensure the current binary tag exposes the peeled ref needed by `git ls-remote ... refs/tags/<tag>^{}` or adjust the release publication process so that ref is always queryable.
3. Have S04 align public docs/reference assets with this hosted contract so the starter proof and packages surface are described as one release/deploy truth chain, not parallel stories.

## Files Created/Modified

- `.github/workflows/authoritative-starter-failover-proof.yml` — New reusable GitHub Actions workflow that provisions runner-local Postgres, exports masked `DATABASE_URL`, runs `bash scripts/verify-m053-s02.sh`, and uploads `.tmp/m053-s02/**` on failure.
- `.github/workflows/authoritative-verification.yml` — Now calls the new starter failover reusable workflow as a whitespace-guarded mainline lane.
- `.github/workflows/release.yml` — Now requires the new starter failover reusable workflow as a tag-gated release prerequisite alongside authoritative live proof.
- `scripts/verify-m034-s02-workflows.sh` — Expanded fail-closed workflow topology verifier to pin the new reusable workflow, caller wiring, release needs, permissions, service shape, and diagnostics artifact contract.
- `scripts/verify-m053-s03.sh` — New hosted evidence verifier that resolves expected refs, queries hosted workflow runs, records artifacts under `.tmp/m053-s03/verify/`, and fails closed on starter/packages drift.
- `scripts/tests/verify-m053-s03-contract.test.mjs` — Extended fixture-backed contract suite to cover the green hosted path plus negative cases for stale refs, tag-only deploy evidence, missing jobs/steps, malformed `gh` JSON, and missing artifacts.
- `.gsd/KNOWLEDGE.md` — Added M053/S03 notes about repo-slug derivation, single-key `.env` parsing, and peeled-tag release freshness failures.
- `.gsd/PROJECT.md` — Updated current project state to reflect that M053 S03 is complete and that live hosted drift is now visible through the new verifier bundle.
