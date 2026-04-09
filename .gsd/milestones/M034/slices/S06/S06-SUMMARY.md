---
id: S06
parent: M034
milestone: M034
provides:
  - A non-destructive hosted-evidence capture path that downstream slices can rerun without losing the previous snapshot
  - Durable, labeled remote-evidence bundles that show exactly why S05 still stops at `remote-evidence`
  - Transport-recovery artifacts proving the blocker is the HTTPS upload path rather than local workflow drift
  - Truthful extension-proof polling semantics for reusable GitHub Actions workflows
requires:
  - slice: S05
    provides: The canonical assembled verifier (`scripts/verify-m034-s05.sh`) and the S01-S04 local proof surfaces that S06 wraps and archives during `remote-evidence`.
affects:
  - S07
key_files:
  - scripts/verify-m034-s05.sh
  - scripts/verify-m034-s06-remote-evidence.sh
  - scripts/tests/verify-m034-s06-contract.test.mjs
  - .tmp/m034-s06/push-main.stderr
  - .tmp/m034-s06/transport-recovery/attempts.log
  - .tmp/m034-s06/evidence/preflight/manifest.json
  - .tmp/m034-s06/evidence/v0.1.0/manifest.json
  - .tmp/m034-s06/evidence/closeout-20260326-1525/manifest.json
key_decisions:
  - D093: add a stop-after remote-evidence/archive path around `scripts/verify-m034-s05.sh` and preserve deterministic S06 evidence labels
  - D094: do not fabricate `v0.1.0` hosted proof until remote `main` actually contains the rollout workflow graph
  - D095: derive reusable extension proof evidence from the `publish-extension.yml` caller run instead of querying `extension-release-proof.yml` as a standalone push workflow
patterns_established:
  - Treat hosted rollout as a snapshot problem: run `scripts/verify-m034-s05.sh` only through `remote-evidence`, then archive the entire bundle under a unique S06 label before any later rerun can delete it.
  - Treat a green hosted run with missing required jobs/steps as stale rollout evidence, not as success; the deploy workflow now has an explicit required-step contract (`build: Verify public docs contract`).
  - When a required branch/tag has no matching hosted run, preserve both `remote-<workflow>-list.*` and `remote-<workflow>-latest-available.*`; do not assume a success-path `remote-<workflow>-view.*` file exists.
  - For reusable workflows such as extension proof, query the caller workflow surface (`publish-extension.yml`) and require the named proof job, rather than inventing a top-level run surface GitHub does not expose.
observability_surfaces:
  - `.tmp/m034-s05/verify/phase-report.txt`, `failed-phase.txt`, and `remote-runs.json` for current S05 boundary diagnosis
  - `.tmp/m034-s06/evidence/<label>/{manifest.json,remote-runs.json,phase-report.txt}` for durable hosted rollout snapshots
  - `.tmp/m034-s06/transport-recovery/{attempts.log,*.stdout,*.stderr}` for bounded push-recovery evidence and timing
  - The contract test `scripts/tests/verify-m034-s06-contract.test.mjs` for stop-after semantics, archive layout, label reuse, and extension-proof polling behavior
drill_down_paths:
  - .gsd/milestones/M034/slices/S06/tasks/T01-SUMMARY.md
  - .gsd/milestones/M034/slices/S06/tasks/T02-SUMMARY.md
  - .gsd/milestones/M034/slices/S06/tasks/T03-SUMMARY.md
  - .gsd/milestones/M034/slices/S06/tasks/T04-SUMMARY.md
  - .gsd/milestones/M034/slices/S06/tasks/T05-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-27T06:05:39.860Z
blocker_discovered: false
---

# S06: Hosted rollout evidence capture

**S06 turned hosted rollout from ephemeral failure into durable evidence: the S05 verifier now supports stop-after remote polling, labeled S06 archives preserve truthful GitHub state, and the retained blocker is the HTTPS push path that still leaves remote `main` stale.**

## What Happened

S06 converted hosted rollout investigation into an auditable, replayable proof surface instead of one-off CLI output. The slice added a `--stop-after remote-evidence` boundary to `scripts/verify-m034-s05.sh`, wrapped it with `scripts/verify-m034-s06-remote-evidence.sh`, and pinned that operator contract with `scripts/tests/verify-m034-s06-contract.test.mjs`. That gives downstream work deterministic archive roots under `.tmp/m034-s06/evidence/<label>/`, fail-closed label reuse, and truthful manifests that retain the exact hosted state even when `scripts/verify-m034-s05.sh` would normally wipe `.tmp/m034-s05/verify/` on the next run.

Closeout verification confirmed that the local workflow graph is healthy while the hosted rollout is still red. During closeout, `bash -n scripts/verify-m034-s05.sh`, the contract test, `bash scripts/verify-m034-s05-workflows.sh`, `bash scripts/verify-m034-s02-workflows.sh`, and `bash scripts/verify-m034-s04-workflows.sh` all passed. A fresh archive (`.tmp/m034-s06/evidence/closeout-20260326-1525/`) shows every prereq phase through `s04-workflows` passing before the first hosted failure, so the remaining issue is not local YAML/script drift.

The slice did **not** achieve the original first-green hosted-rollout goal. The latest closeout evidence still reports remote `main` at `5ddf3b2dce17abe08e1188d9b46e575d83525b50`, a green but stale `deploy.yml` run (`23506361663`) whose `build` job is missing the required `Verify public docs contract` step, a 404 when querying `authoritative-verification.yml` on the default branch, and no `push` runs for `release.yml`, `deploy-services.yml`, or `publish-extension.yml` on `v0.1.0` / `ext-v0.3.0`. The archived `preflight/`, `v0.1.0/`, and closeout bundles all agree on that blocker surface. No truthful `.tmp/m034-s06/evidence/main/` or `.tmp/m034-s06/evidence/first-green/` bundle exists because remote rollout never reached those states.

The transport-recovery artifacts are the main thing S06 contributes to roadmap reassessment. `.tmp/m034-s06/push-main.stderr` records the original chunked HTTPS failure, and `.tmp/m034-s06/transport-recovery/attempts.log` plus `03-http11-postbuffer-1g.stderr` show that forcing HTTP/1.1 and raising `http.postBuffer` to 1 GiB still produced `HTTP 408` after roughly twelve minutes. That narrows the remaining rollout blocker to the host's HTTPS receive-pack path rather than to chunked transfer encoding, missing local proofs, or bad hosted polling logic.

## Operational Readiness (Q8)
- **Health signal:** a fresh `bash scripts/verify-m034-s06-remote-evidence.sh <new-label> || true` run should leave `candidate-tags` passed, `remote-evidence` terminal, and a `manifest.json` / `remote-runs.json` pair whose workflow statuses match GitHub truth.
- **Failure signal:** `.tmp/m034-s05/verify/failed-phase.txt` equals `remote-evidence`, `authoritative-verification.yml` returns 404 on the remote default branch, `deploy.yml` reports a missing `Verify public docs contract` build step, and the push stderr artifacts contain `RPC failed; HTTP 408`.
- **Recovery procedure:** land the current local rollout graph on remote `main` through a transport that can upload the full receive-pack payload, wait for fresh `main` push runs, then re-run the archive helper in order `main` -> `v0.1.0` -> `first-green` before replaying full `bash scripts/verify-m034-s05.sh`.
- **Monitoring gaps:** there is still no automatic alert for “workflow file exists locally but not on remote default branch,” no automated first-green claimant, and no early warning that the current HTTPS push path will time out on this payload size.

## Verification

- `bash -n scripts/verify-m034-s05.sh` ✅
- `node --test scripts/tests/verify-m034-s06-contract.test.mjs` ✅ (`5` tests passed)
- `bash scripts/verify-m034-s05-workflows.sh` ✅ (`verify-m034-s05-workflows: ok (all)`)
- `bash scripts/verify-m034-s02-workflows.sh` ✅ (`verify-m034-s02-workflows: ok (all)`)
- `bash scripts/verify-m034-s04-workflows.sh` ✅ (`verify-m034-s04-workflows: ok (all)`)
- `gh api repos/snowdamiz/mesh-lang/branches/main --jq '.commit.sha'` ✅ -> `5ddf3b2dce17abe08e1188d9b46e575d83525b50`
- `gh run list -R snowdamiz/mesh-lang --workflow deploy.yml --event push --branch main --limit 1 --json databaseId,status,conclusion,headSha,url` ✅ -> latest run `23506361663` is green on stale SHA `5ddf3b2dce17abe08e1188d9b46e575d83525b50`
- `gh run list -R snowdamiz/mesh-lang --workflow authoritative-verification.yml --event push --branch main --limit 1 --json ...` ✅ (truthful failure) -> `HTTP 404: workflow authoritative-verification.yml not found on the default branch`
- `gh run list -R snowdamiz/mesh-lang --workflow release.yml --event push --branch v0.1.0 --limit 1 --json ...` ✅ -> `[]`
- `gh run list -R snowdamiz/mesh-lang --workflow deploy-services.yml --event push --branch v0.1.0 --limit 1 --json ...` ✅ -> `[]`
- `gh run list -R snowdamiz/mesh-lang --workflow publish-extension.yml --event push --branch ext-v0.3.0 --limit 1 --json ...` ✅ -> `[]`
- `bash scripts/verify-m034-s06-remote-evidence.sh closeout-20260326-1525 || true` ✅ -> archived `.tmp/m034-s06/evidence/closeout-20260326-1525/` with `failedPhase=remote-evidence` and the expected stale/missing hosted workflow statuses
- `bash scripts/verify-m034-s05.sh || test "$(cat .tmp/m034-s05/verify/failed-phase.txt)" = "remote-evidence"` ✅ -> full assembly still fail-closes at `remote-evidence`, not a later phase

## Requirements Advanced

None.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

The slice plan targeted first green hosted runs on `main`, `v0.1.0`, and `ext-v0.3.0`, plus a preserved `first-green` bundle and an S05 replay that moved past `remote-evidence`. That did not happen. The truthful delivered state is archived red evidence (`preflight/`, `v0.1.0/`, and the closeout bundle) plus transport-recovery logs proving why `main` never advanced. There is still no legitimate `main/` or `first-green/` evidence directory because creating either would have implied a hosted state that never existed.

## Known Limitations

- Remote `main` is still stale at `5ddf3b2dce17abe08e1188d9b46e575d83525b50` and does not expose `authoritative-verification.yml` or the newer deploy step graph.
- There are still no hosted `push` runs for `release.yml` / `deploy-services.yml` on `v0.1.0` or for `publish-extension.yml` on `ext-v0.3.0`.
- The slice does not resolve the public freshness gap on `meshlang.dev`; S05 still cannot reach `public-http` or `s01-live-proof` because hosted rollout fails earlier.
- Transport recovery is documented but unresolved: the current host still cannot upload the rollout receive-pack over HTTPS without timing out.

## Follow-ups

Obtain a transport path or equivalent remote recovery that can land the current local rollout graph on `origin/main`; once remote `main` matches local truth, rerun `scripts/verify-m034-s06-remote-evidence.sh` with the reserved labels in order (`main`, `v0.1.0`, `first-green`) and then replay full `bash scripts/verify-m034-s05.sh` to determine whether S07 should proceed unchanged or be replanned around the unresolved public freshness gap.

## Files Created/Modified

- `scripts/verify-m034-s05.sh` — Added `--stop-after remote-evidence`, early-exit handling, and truthful remote-evidence polling for reusable extension proof via `publish-extension.yml`.
- `scripts/verify-m034-s06-remote-evidence.sh` — Added the slice-owned archive wrapper that snapshots `.tmp/m034-s05/verify/` into labeled `.tmp/m034-s06/evidence/<label>/` bundles with fail-closed label reuse and generated manifests.
- `scripts/tests/verify-m034-s06-contract.test.mjs` — Pinned the stop-after operator contract, archive layout, label-overwrite refusal, and reusable extension-proof polling semantics.
- `.tmp/m034-s06/transport-recovery/attempts.log` — Recorded bounded HTTPS push-recovery attempts, durations, target SHA, and stderr/stdout artifact paths.
- `.tmp/m034-s06/evidence/preflight/manifest.json` — Captured the first hosted-red baseline bundle before any rollout retries.
- `.tmp/m034-s06/evidence/v0.1.0/manifest.json` — Captured the blocked binary-tag remote-evidence state without fabricating a green tag rollout.
- `.tmp/m034-s06/evidence/closeout-20260326-1525/manifest.json` — Fresh closeout rerun proving the blocker surface is still current: stale `main`, missing authoritative workflow, and no candidate-tag push runs.
- `.gsd/KNOWLEDGE.md` — Added an S06 note explaining that a green `deploy.yml` run missing the `Verify public docs contract` step is a stale-default-branch workflow signal, not a docs outage.
- `.gsd/PROJECT.md` — Refreshed current-state text to describe S06’s archive/transport outputs and the remaining remote-rollout blocker.
