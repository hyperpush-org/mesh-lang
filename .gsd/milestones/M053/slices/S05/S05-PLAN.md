# S05: Hosted workflow evidence closes the starter/packages contract

**Goal:** Converge live GitHub state onto the shipped M053 contract so remote `main` and the current binary tag both carry the generated Postgres starter failover proof plus the packages/public-surface proof, and `bash scripts/verify-m053-s03.sh` turns green without weakening the verifier.
**Demo:** After this: Run `bash scripts/verify-m053-s03.sh` to green so `.tmp/m053-s03/verify/status.txt` becomes `ok`, `remote-runs.json` shows fresh successful `authoritative-verification.yml`, `deploy-services.yml`, and `release.yml` runs on the expected refs, and the hosted chain honestly carries starter failover plus packages/public-surface proof.

## Tasks
- [x] **T01: Captured a fresh hosted blocker baseline and built a current-origin/main M053-only rollout commit for T02.** — Reconfirm the local workflow/verifier contract, capture one fresh hosted-evidence baseline under `.tmp/m053-s03/verify/`, and assemble a minimal rollout commit from `origin/main` that carries the M053 S01-S04 tree without unrelated local-ahead work. Later tasks should push a known commit SHA, not local `HEAD`.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Local contract rails (`bash scripts/verify-m034-s02-workflows.sh` and `node --test scripts/tests/verify-m053-s03-contract.test.mjs`) | Stop before any remote mutation and fix local workflow/contract drift first. | Treat slow local rails as real regression signal; do not skip them to unblock rollout. | Fail closed if the local workflow or hosted-contract assertions stop producing the expected markers. |
| Hosted baseline capture (`GH_TOKEN` + `bash scripts/verify-m053-s03.sh`) | Preserve the retained `.tmp/m053-s03/verify/` bundle and stop on auth/repo-slug failures instead of guessing. | Treat `gh` or network timeouts as incomplete hosted evidence and retain the failing logs. | Fail closed if `remote-runs.json`, `candidate-refs.json`, or phase markers are missing or unparsable. |
| Local git history / rollout assembly | Stop if the candidate still contains M056 `/pitch`, omits required M053 files, or cannot be rebased cleanly onto `origin/main`. | Treat fetch/cherry-pick conflicts as rollout-assembly failures that must be resolved before pushing. | Fail closed if the retained rollout manifest cannot prove which SHA/files will ship. |

## Load Profile

- **Shared resources**: local git refs/branches, GitHub API rate budget for baseline capture, and retained artifacts under `.tmp/m053-s03/verify/` plus `.tmp/m053-s05/rollout/`.
- **Per-operation cost**: one local workflow sweep, one hosted verifier baseline replay, one `git fetch`, and one minimal rollout-branch/commit assembly.
- **10x breakpoint**: GitHub API waiting and git conflict resolution, not application throughput.

## Negative Tests

- **Malformed inputs**: missing `GH_TOKEN`, missing/renamed origin remote, absent required M053 files, or a commit selection that still includes unrelated M056 surfaces.
- **Error paths**: local workflow contract turns red, hosted baseline artifacts are missing, cherry-pick/conflict resolution fails, or the rollout tree drops S04 docs/runtime evidence.
- **Boundary conditions**: `origin/main` has advanced since research, the hosted baseline is already greener than expected, or the current binary tag is still lightweight-only.

## Steps

1. Re-run `bash scripts/verify-m034-s02-workflows.sh` and `node --test scripts/tests/verify-m053-s03-contract.test.mjs`, then capture a fresh hosted baseline with `GH_TOKEN` parsed from `.env` (do **not** `source .env`) so `.tmp/m053-s03/verify/` reflects current live blockers before any push.
2. Starting from `origin/main`, assemble a minimal local rollout branch/commit that carries the full M053 S01-S04 tree needed by starter proof, packages verification, and public docs while excluding unrelated M056 `/pitch` work.
3. Record the candidate commit SHA, included files/commits, and explicit exclusion of unrelated work in `.tmp/m053-s05/rollout/main-rollout-plan.md` plus `.tmp/m053-s05/rollout/main-rollout-commit.txt` for downstream tasks.

## Must-Haves

- [ ] A fresh hosted baseline bundle exists under `.tmp/m053-s03/verify/` before any remote mutation.
- [ ] The local rollout candidate carries the required M053 S01-S04 tree and explicitly excludes unrelated M056 work.
- [ ] A retained manifest names the exact rollout commit SHA and the proof-critical files it will ship.
  - Estimate: 75m
  - Files: scripts/verify-m034-s02-workflows.sh, scripts/tests/verify-m053-s03-contract.test.mjs, scripts/verify-m053-s03.sh, .github/workflows/authoritative-verification.yml, .github/workflows/deploy-services.yml, .github/workflows/release.yml, .tmp/m053-s05/rollout/main-rollout-plan.md, .tmp/m053-s05/rollout/main-rollout-commit.txt
  - Verify: bash scripts/verify-m034-s02-workflows.sh && node --test scripts/tests/verify-m053-s03-contract.test.mjs && test -s .tmp/m053-s03/verify/remote-runs.json && test -s .tmp/m053-s05/rollout/main-rollout-commit.txt
- [x] **T02: Pushed the retained M053 rollout SHA to remote main, proved deploy-services green on that commit, and captured the authoritative starter-proof failure blocking tag reroll.** — Push the T01 rollout commit to remote `main`, then verify that the fresh mainline hosted evidence closes the `main` side of R121/R122 on the shipped SHA before touching the release tag. This task should leave release-tag freshness as the only possible remaining blocker.

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
  - Estimate: 90m
  - Files: .github/workflows/authoritative-verification.yml, .github/workflows/deploy-services.yml, scripts/verify-m053-s03.sh, .tmp/m053-s03/verify/remote-runs.json, .tmp/m053-s05/rollout/main-rollout-commit.txt, .tmp/m053-s05/rollout/main-shipped-sha.txt, .tmp/m053-s05/rollout/main-workflows.json
  - Verify: test -s .tmp/m053-s05/rollout/main-shipped-sha.txt && python3 -c 'import json, pathlib; ship=pathlib.Path(".tmp/m053-s05/rollout/main-shipped-sha.txt").read_text().strip(); workflows={w["workflowFile"]: w for w in json.loads(pathlib.Path(".tmp/m053-s03/verify/remote-runs.json").read_text())["workflows"]}; assert workflows["authoritative-verification.yml"]["status"] == "ok"; assert workflows["authoritative-verification.yml"]["observedHeadSha"] == ship; assert workflows["deploy-services.yml"]["status"] == "ok"; assert workflows["deploy-services.yml"]["observedHeadSha"] == ship'
  - Blocker: Fresh authoritative-verification.yml run 24012277578 is red on shipped SHA c6d31bf495fd43a19e96a8becebbd3f7426c4bd7 because the required starter-proof job failed. The downloaded starter-proof diagnostics prove the failure passed through scripts/verify-m053-s01.sh and compiler/meshc/tests/e2e_m049_s03.rs, but the uploaded artifact does not retain the full nested Rust test log, so the exact inner assertion line is not preserved locally. bash scripts/verify-m053-s03.sh remains red in remote-evidence because authoritative-verification.yml failed on main and release.yml still lacks peeled tag data for refs/tags/v0.1.0. Because main is not yet green, T03’s planned tag-reroll sequence is no longer valid without replanning or a follow-up repair on the starter-proof path.
- [x] **T03: Reproduced the authoritative starter-proof failure as a missing mesh-rt prebuild in cold target dirs and retained CI-grade inner logs.** — Use the retained authoritative-starter-failover-proof diagnostics from T02 plus the shipped main SHA to reproduce the failing starter-proof path in a clean, CI-like environment. Drive the failure down to an explicit class (timeout/compile-budget, product assertion, or environment drift) by rerunning the nested S01/S02 entrypoints or the targeted cargo test with cold caches as needed, and preserve the full inner Rust/test logs under .tmp/m053-s05/starter-proof-repro/ instead of relying on the truncated workflow artifact. Write a short root-cause note that names the exact failing command, evidence log, and any diagnostic-retention gaps that must be fixed before rerunning hosted workflows.
  - Estimate: 90m
  - Files: .github/workflows/authoritative-starter-failover-proof.yml, scripts/verify-m053-s02.sh, scripts/verify-m053-s01.sh, compiler/meshc/tests/e2e_m049_s03.rs, .tmp/m053-s05/rollout/authoritative-starter-failover-proof-diagnostics/verify/m053-s01-contract.log, .tmp/m053-s05/starter-proof-repro/root-cause.md, .tmp/m053-s05/starter-proof-repro/ci-failure-classification.json
  - Verify: test -s .tmp/m053-s05/starter-proof-repro/root-cause.md && python3 - <<'PY'
import json, pathlib
root = pathlib.Path('.tmp/m053-s05/starter-proof-repro')
data = json.loads(root.joinpath('ci-failure-classification.json').read_text())
assert data['failure_class'] in {'timeout', 'assertion', 'environment'}
assert data['failing_command']
log_path = pathlib.Path(data['primary_log'])
assert log_path.exists()
assert log_path.stat().st_size > 0
PY
- [x] **T04: Patched the starter-proof wrappers to normalize cargo paths and retain nested S01 logs, and proved the staged deploy rail with an absolute cold target path.** — Fix the T03 root cause in the workflow/script/test path without weakening the M053 contract. If the failure is diagnostic opacity, harden log retention so nested Rust output survives future hosted failures; if it is timeout or product drift, change the responsible workflow, scripts, or tests so the starter proof stays truthful on clean GitHub runners. Re-run the local starter-proof rails to green, push only the repair commit(s) to remote main, wait for fresh authoritative-verification.yml and deploy-services.yml push runs on the new shipped SHA, and replay bash scripts/verify-m053-s03.sh so the retained hosted bundle shows main is closed again before touching the tag.
  - Estimate: 120m
  - Files: .github/workflows/authoritative-starter-failover-proof.yml, .github/workflows/authoritative-verification.yml, scripts/verify-m053-s02.sh, scripts/verify-m053-s01.sh, compiler/meshc/tests/e2e_m049_s03.rs, scripts/verify-m053-s03.sh, .tmp/m053-s03/verify/remote-runs.json, .tmp/m053-s05/rollout/main-shipped-sha.txt, .tmp/m053-s05/rollout/main-workflows.json, .tmp/m053-s05/rollout/starter-proof-fix-summary.md
  - Verify: bash scripts/verify-m034-s02-workflows.sh && node --test scripts/tests/verify-m053-s03-contract.test.mjs && DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash scripts/verify-m053-s02.sh && test -s .tmp/m053-s05/rollout/main-shipped-sha.txt && python3 - <<'PY'
import json, pathlib
ship = pathlib.Path('.tmp/m053-s05/rollout/main-shipped-sha.txt').read_text().strip()
workflows = {w['workflowFile']: w for w in json.loads(pathlib.Path('.tmp/m053-s03/verify/remote-runs.json').read_text())['workflows']}
auth = workflows['authoritative-verification.yml']
deploy = workflows['deploy-services.yml']
assert auth['status'] == 'ok'
assert auth['observedHeadSha'] == ship
assert deploy['status'] == 'ok'
assert deploy['observedHeadSha'] == ship
PY
- [x] **T05: Shipped the repaired starter-proof commit to remote main and captured the exact remaining tag/release blocker for recovery.** — After T04 makes main green again, recreate and push v0.1.0 as an annotated tag on the repaired shipped SHA, verify both raw and peeled refs resolve, wait for the fresh tag-triggered release.yml run to finish with Hosted starter failover proof plus Create Release, and rerun bash scripts/verify-m053-s03.sh to green. If the final hosted replay still finds drift, record the exact blocker in .tmp/m053-s05/rollout/final-blocker.md before changing code or verifier expectations.
  - Estimate: 90m
  - Files: .github/workflows/release.yml, scripts/verify-m053-s03.sh, .tmp/m053-s03/verify/status.txt, .tmp/m053-s03/verify/current-phase.txt, .tmp/m053-s03/verify/remote-runs.json, .tmp/m053-s05/rollout/main-shipped-sha.txt, .tmp/m053-s05/rollout/release-workflow.json, .tmp/m053-s05/rollout/final-blocker.md
  - Verify: GH_TOKEN=${GH_TOKEN:?set GH_TOKEN} bash scripts/verify-m053-s03.sh && python3 - <<'PY'
import json, pathlib
verify = pathlib.Path('.tmp/m053-s03/verify')
assert verify.joinpath('status.txt').read_text().strip() == 'ok'
assert verify.joinpath('current-phase.txt').read_text().strip() == 'complete'
workflows = json.loads(verify.joinpath('remote-runs.json').read_text())['workflows']
assert len(workflows) == 3 and all(w['status'] == 'ok' for w in workflows)
PY

git ls-remote --quiet origin refs/tags/v0.1.0 'refs/tags/v0.1.0^{}'
