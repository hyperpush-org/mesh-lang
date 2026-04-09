---
estimated_steps: 23
estimated_files: 7
skills_used: []
---

# T02: Pushed the retained M053 rollout SHA to remote main, proved deploy-services green on that commit, and captured the authoritative starter-proof failure blocking tag reroll.

Push the T01 rollout commit to remote `main`, then verify that the fresh mainline hosted evidence closes the `main` side of R121/R122 on the shipped SHA before touching the release tag. This task should leave release-tag freshness as the only possible remaining blocker.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Push of the T01 rollout commit to remote `main` | Stop and retain the push rejection/branch-protection output; do not fall back to pushing local `HEAD`. | Treat a hung network push as incomplete rollout and recheck remote refs before retrying. | Fail closed if the remote ref moves to a SHA that does not match the retained rollout commit. |
| `authoritative-verification.yml` push run on `main` | Stop on the first red/failed required job and inspect the run instead of accepting stale green history. | Time-box workflow polling and retain the run URL/id rather than waiting forever. | Fail closed if `gh run view` omits `Hosted starter failover proof` or freshness fields. |
| `deploy-services.yml` push run on `main` | Stop if packages deploy/public-surface proof is missing or green only on an older SHA. | Treat slow mainline deploy completion as an unresolved hosted contract, not as good-enough evidence. | Fail closed if the `Post-deploy health checks` job or `Verify public surface contract` step is absent from the fresh run. |

## Load Profile

- **Shared resources**: remote `main`, GitHub Actions runner capacity, GitHub API rate budget, and retained rollout artifacts under `.tmp/m053-s05/rollout/`.
- **Per-operation cost**: one push to `main`, repeated `gh run list/view` polling for two workflows, and one hosted verifier replay after mainline workflows settle.
- **10x breakpoint**: workflow queue/wait time and GitHub API polling, not repository size.

## Negative Tests

- **Malformed inputs**: missing T01 commit SHA, stale `origin/main` after push, `gh` JSON without `headSha`, or workflow runs returned for the wrong branch/event.
- **Error paths**: `authoritative-verification.yml` lacks `Hosted starter failover proof`, `deploy-services.yml` lacks `Verify public surface contract`, or the fresh mainline runs stay pinned to the pre-rollout SHA.
- **Boundary conditions**: another actor moves `main` during rollout, the verifier reaches `release.yml` and stays red there, or GitHub queues both workflows but only one finishes green.

## Steps

1. Push the T01 rollout commit to remote `main` without merging unrelated local-ahead commits, then record the shipped SHA in `.tmp/m053-s05/rollout/main-shipped-sha.txt`.
2. Use `gh run list/view -R hyperpush-org/hyperpush-mono` to wait for fresh `authoritative-verification.yml` and `deploy-services.yml` push runs on that exact SHA; require `Hosted starter failover proof` plus `Post-deploy health checks -> Verify public surface contract` in the retained evidence.
3. Re-run `bash scripts/verify-m053-s03.sh` after those mainline workflows finish, retain the updated `.tmp/m053-s03/verify/` bundle, and write a focused summary to `.tmp/m053-s05/rollout/main-workflows.json`; at this point only release-tag freshness may remain red.

## Must-Haves

- [ ] Remote `main` moves to the retained T01 rollout commit SHA, not to local `HEAD`.
- [ ] Fresh `authoritative-verification.yml` and `deploy-services.yml` push runs are green on that exact shipped SHA.
- [ ] Retained artifacts make it obvious that the `main` side of the hosted contract is closed before the tag reroll begins.

## Inputs

- ``.tmp/m053-s05/rollout/main-rollout-plan.md` — retained T01 manifest for the exact M053-only ship set`
- ``.tmp/m053-s05/rollout/main-rollout-commit.txt` — exact commit SHA to push to `main``
- ``scripts/verify-m053-s03.sh` — hosted evidence verifier to rerun after mainline workflows settle`
- ``.github/workflows/authoritative-verification.yml` — expected required mainline starter-proof caller graph`
- ``.github/workflows/deploy-services.yml` — expected packages/public-surface job + step names on `main``
- ``.tmp/m053-s03/verify/remote-runs.json` — baseline hosted evidence to compare against fresh mainline runs`

## Expected Output

- ``.tmp/m053-s05/rollout/main-shipped-sha.txt` — exact SHA now live on remote `main``
- ``.tmp/m053-s05/rollout/main-workflows.json` — retained run-id/head-SHA summary for fresh mainline hosted evidence`
- ``.tmp/m053-s03/verify/remote-runs.json` — updated hosted verifier artifact showing `main` workflows green on the shipped SHA`
- ``.tmp/m053-s03/verify/current-phase.txt` — verifier phase pointer after the post-mainline replay`

## Verification

test -s .tmp/m053-s05/rollout/main-shipped-sha.txt && python3 -c 'import json, pathlib; ship=pathlib.Path(".tmp/m053-s05/rollout/main-shipped-sha.txt").read_text().strip(); workflows={w["workflowFile"]: w for w in json.loads(pathlib.Path(".tmp/m053-s03/verify/remote-runs.json").read_text())["workflows"]}; assert workflows["authoritative-verification.yml"]["status"] == "ok"; assert workflows["authoritative-verification.yml"]["observedHeadSha"] == ship; assert workflows["deploy-services.yml"]["status"] == "ok"; assert workflows["deploy-services.yml"]["observedHeadSha"] == ship'

## Observability Impact

- Signals added/changed: shipped-main SHA pointer, fresh GitHub run IDs for `authoritative-verification.yml` and `deploy-services.yml`, and updated `.tmp/m053-s03/verify/remote-runs.json`.
- How a future agent inspects this: compare `.tmp/m053-s05/rollout/main-shipped-sha.txt` to `observedHeadSha` values in `.tmp/m053-s03/verify/remote-runs.json`.
- Failure state exposed: branch-protection rejection, stale mainline workflow SHA, missing `Hosted starter failover proof`, or missing `Verify public surface contract`.
