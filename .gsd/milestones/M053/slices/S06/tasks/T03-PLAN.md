---
estimated_steps: 4
estimated_files: 6
skills_used:
  - github-workflows
  - debug-like-expert
---

# T03: Ship the repair and close hosted/tag freshness on one SHA

- Why: The slice does not close locally; it closes only when the repaired `main` SHA is green in both hosted starter proof and packages proof, and the binary tag is rerolled as annotated so `refs/tags/v0.1.0^{}` resolves for release freshness.
- Do: Refresh hosted evidence with `bash scripts/verify-m053-s03.sh`, keep `deploy-services.yml` green on the same shipped SHA, and then push/reroll only after explicit user confirmation for the outward-facing GitHub mutations. Do not mutate `main` or `v0.1.0` speculatively.
- Done when: after explicit approval for the remote mutations, fresh `authoritative-verification.yml`, `deploy-services.yml`, and `release.yml` runs are green on the expected refs, `bash scripts/verify-m053-s03.sh` turns green, and `git ls-remote` resolves `refs/tags/v0.1.0^{}`. Without approval, stop blocked with the retained mutation plan instead of pretending the slice is complete.

## Steps

1. Re-run the local workflow/verifier preflights and refresh `.tmp/m053-s03/verify/` in read-only mode so Task 2’s green starter bundle and the current shipped SHA are captured before any remote mutation.
2. After explicit user confirmation, push the S06 repair commit to `main`, wait for fresh green `authoritative-verification.yml` and already-green `deploy-services.yml` runs on that exact SHA, and record the alignment in `.tmp/m053-s06/rollout/remote-mutation-plan.md` plus the refreshed hosted bundle.
3. After explicit user confirmation for the tag mutation, reroll `v0.1.0` as an annotated tag on the same green SHA, wait for `release.yml`, rerun `bash scripts/verify-m053-s03.sh`, and write the final closeout or exact remaining blocker to `.tmp/m053-s06/rollout/final-hosted-closeout.md`.

## Must-Haves

- [ ] The repaired S06 commit is pushed to `main` only after Task 2 is green locally and the user explicitly approves the remote mutation.
- [ ] Fresh `authoritative-verification.yml` and `deploy-services.yml` runs are green on the same shipped SHA before the annotated tag reroll begins.
- [ ] After explicit approval for the tag mutation, `v0.1.0` is rerolled as annotated on that SHA, `refs/tags/v0.1.0^{}` resolves, and `bash scripts/verify-m053-s03.sh` turns green; otherwise the task stops blocked with a retained mutation plan.

## Inputs

- `.tmp/m053-s02/verify/status.txt`
- `.tmp/m053-s02/verify/latest-proof-bundle.txt`
- `.tmp/m053-s05/rollout/main-shipped-sha.txt`
- `.tmp/m053-s05/rollout/final-blocker.md`
- `scripts/verify-m053-s03.sh`
- `scripts/tests/verify-m053-s03-contract.test.mjs`
- `.github/workflows/authoritative-verification.yml`
- `.github/workflows/deploy-services.yml`
- `.github/workflows/release.yml`

## Expected Output

- `.tmp/m053-s03/verify/status.txt`
- `.tmp/m053-s03/verify/current-phase.txt`
- `.tmp/m053-s03/verify/remote-runs.json`
- `.tmp/m053-s06/rollout/remote-mutation-plan.md`
- `.tmp/m053-s06/rollout/release-workflow.json`
- `.tmp/m053-s06/rollout/final-hosted-closeout.md`

## Verification

bash scripts/verify-m034-s02-workflows.sh && node --test scripts/tests/verify-m053-s03-contract.test.mjs && GH_TOKEN=${GH_TOKEN:?set GH_TOKEN} bash scripts/verify-m053-s03.sh && git ls-remote --quiet origin refs/tags/v0.1.0 'refs/tags/v0.1.0^{}'

## Observability Impact

- Signals added/changed: refreshed `.tmp/m053-s03/verify/status.txt`, `current-phase.txt`, `phase-report.txt`, and `remote-runs.json`, plus rollout notes under `.tmp/m053-s06/rollout/`.
- How a future agent inspects this: open `.tmp/m053-s03/verify/remote-runs.json` for fresh workflow SHA alignment, then read `.tmp/m053-s06/rollout/remote-mutation-plan.md` / `final-hosted-closeout.md` for the exact shipped SHA, approval state, and tag outcome.
- Failure state exposed: stale or mismatched hosted SHA, missing required job/step, absent peeled tag, failed remote mutation, or withheld user approval.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `GH_TOKEN`, `git ls-remote`, and `bash scripts/verify-m053-s03.sh` | Stop on auth or ref-resolution failures and retain `.tmp/m053-s03/verify/` plus `.tmp/m053-s06/rollout/remote-mutation-plan.md`; do not guess at hosted freshness. | Treat queued or slow workflows as unresolved hosted evidence and keep polling artifacts instead of concluding green. | Fail closed if `remote-runs.json` or expected-ref data omit the required SHA/job/tag fields. |
| Remote `main` push and annotated `v0.1.0` reroll | Do not perform either mutation without explicit user confirmation; if the push/tag command fails, retain the exact stderr and stop before any second mutation. | Re-check remote refs before retrying a slow push or tag publication; stale local assumptions are worse than waiting. | Fail closed if the rerolled tag is still lightweight or resolves to a different SHA than the green `main` run. |
| Hosted workflow graph (`authoritative-verification.yml`, `deploy-services.yml`, `release.yml`) | Stop on the first red or stale run instead of reusing older green history. | Treat missing completion on the shipped SHA as “not done”; keep the watch logs and current `remote-runs.json`. | Fail closed if required jobs/steps disappear or if `deploy-services.yml` is only green on an older or tag-only SHA. |

## Load Profile

- **Shared resources**: GitHub API rate budget, workflow queue capacity, remote `main` and `v0.1.0` refs, `.tmp/m053-s03/verify/`, and `.tmp/m053-s06/rollout/`.
- **Per-operation cost**: one hosted verifier replay before mutation, one push to `main`, one annotated tag reroll, and one release rerun watch.
- **10x breakpoint**: workflow queue wait time and remote mutation latency, not local compute.

## Negative Tests

- **Malformed inputs**: missing approval, missing `GH_TOKEN`, missing shipped-SHA file, absent peeled tag, and malformed workflow JSON.
- **Error paths**: `authoritative-verification.yml` green on the wrong SHA, `deploy-services.yml` regresses on `main`, the tag reroll stays lightweight, or `release.yml` fails after tag publication.
- **Boundary conditions**: remote `main` advances between local verification and push, the tag already exists, and a release run appears fresh but `refs/tags/v0.1.0^{}` is still absent.
