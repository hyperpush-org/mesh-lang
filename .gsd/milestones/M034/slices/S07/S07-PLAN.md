# S07: Public surface freshness and final assembly replay

**Goal:** Make the public release surface current and replayable by aligning hosted deploy checks with the exact S05 public contract, landing the current workflow graph on remote `main` and the candidate tags, and rerunning `bash scripts/verify-m034-s05.sh` until `remote-evidence`, `public-http`, and `s01-live-proof` all pass.
**Demo:** After this: `meshlang.dev` installers/docs now match repo truth, and the canonical `bash scripts/verify-m034-s05.sh` replay finishes green through `remote-evidence`, `public-http`, and `s01-live-proof`.

## Tasks
- [x] **T01: Centralized the public-surface contract in a shared helper and rewired S05 plus hosted deploy workflows to consume it.** — The current hosted deploy lanes validate a weaker public contract than `run_public_http_truth()`, which is how the repo can have green deploy runs while `meshlang.dev` still serves stale installers and docs. This task closes that drift first by making the exact installer/docs/packages markers and the bounded freshness-wait behavior shared and testable across the repo-local S05 verifier and the hosted deploy workflows.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Shared public-contract helper / marker set | Fail closed in tests and workflow verifiers if any caller drops a required marker or step. | Treat exhausted freshness wait budget as a proof failure, not a soft warning. | Treat missing marker tables, missing retry budget, or drifted step names as contract breakage. |
| GitHub Pages / Fly deploy workflows | Keep workflow sweeps red until both workflows call the exact stronger contract instead of the current weaker grep set. | Fail with the last observed public mismatch and attempt count so later tasks can see whether the problem is propagation or permanent drift. | Treat presence-only checks, homepage-only curls, or missing runbook markers as malformed proof. |
| Exact public HTTP marker set | Stop the slice if installers/docs/packages markers diverge between S05, deploy.yml, deploy-services.yml, and the workflow verifier. | N/A | Treat wrong content-type expectations, missing diff paths, or missing tooling runbook markers as malformed contract data. |

## Load Profile

- **Shared resources**: live `meshlang.dev` / `packages.meshlang.dev` endpoints, CDN propagation window, workflow YAML, and `.tmp/m034-s05/verify/` diagnostics.
- **Per-operation cost**: repeated HTTP GETs plus exact diff/marker checks for four public `meshlang.dev` surfaces and three packages surfaces.
- **10x breakpoint**: stale-content polling and repeated endpoint fetches dominate first, so the helper must use one bounded retry budget with actionable diagnostics instead of many ad hoc curls.

## Negative Tests

- **Malformed inputs**: missing installer/docs markers, wrong content types, missing workflow steps, or mismatched retry-budget constants.
- **Error paths**: stale public bytes after deploy, missing shared helper call-sites, or workflow verifiers still accepting the weaker pre-S07 checks.
- **Boundary conditions**: exact installer body diffs, tooling page runbook markers, package search/detail/API markers, and the full GitHub Pages cache window are all pinned.

## Steps

1. Extract the exact installer/docs/packages marker set and bounded freshness-wait behavior into a shared helper or equivalent single-source contract that `scripts/verify-m034-s05.sh` can call for local/built/live checks without duplicating marker lists.
2. Rewire `.github/workflows/deploy.yml` and `.github/workflows/deploy-services.yml` to enforce that same stronger contract, including the full tooling runbook markers and propagation-aware failure output instead of the current weaker grep subset.
3. Update `scripts/verify-m034-s05-workflows.sh` so the workflow contract sweeps pin the stronger step bodies and fail if the workflows drift back to shallow checks.
4. Add or extend Node contract coverage so the shared markers, workflow call-sites, and freshness retry budget are mechanically pinned before any hosted rollout attempt.

## Must-Haves

- [ ] One exact public-surface contract owns the required installers/docs/packages markers and bounded freshness wait semantics.
- [ ] `scripts/verify-m034-s05.sh`, `deploy.yml`, and `deploy-services.yml` all consume that stronger contract instead of weaker subsets.
- [ ] `scripts/verify-m034-s05-workflows.sh` and Node contract tests fail if required markers, steps, or retry boundaries drift.
- [ ] The task leaves high-signal diagnostics for stale bytes vs missing markers vs wrong workflow wiring.
  - Estimate: 3h
  - Files: scripts/lib/m034_public_surface_contract.py, scripts/verify-m034-s05.sh, scripts/verify-m034-s05-workflows.sh, .github/workflows/deploy.yml, .github/workflows/deploy-services.yml, scripts/tests/verify-m034-s05-contract.test.mjs, scripts/tests/verify-m034-s07-public-contract.test.mjs
  - Verify: bash -n scripts/verify-m034-s05.sh
node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s07-public-contract.test.mjs
bash scripts/verify-m034-s05-workflows.sh
- [x] **T02: Advanced remote main incrementally to 8d3e76a65986e7bd5e00d107f9e11c1923ecd0ab, proved the next large fast-forward still 408s, and left a truthful staged-rollout resume point.** — No local code change can move S07 forward if remote `main` stays stale or if the candidate-tag workflows never run. This task turns the S06 transport blocker into either a resolved rollout or a fresh truthful blocker bundle: it must land the current rollout graph on `origin/main` without rewriting history, then advance `v0.1.0` and `ext-v0.3.0`, and archive `main`, `v0.1.0`, and `first-green` evidence bundles in order.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `git push` / remote receive-pack path | Record the exact attempt, stderr, and target SHA under `.tmp/m034-s06/transport-recovery/` and stop claiming rollout progress until remote SHA truly advances. | Treat `HTTP 408` or equivalent stalled upload as the active blocker and keep the last bounded attempt visible. | Treat partial remote advancement or unknown target SHA as failure; do not fabricate `main` or tag evidence bundles. |
| GitHub Actions hosted runs | Wait for fresh push runs on the expected branch/tag and reject stale green runs with the wrong `headSha`. | Fail with the last seen run metadata rather than assuming eventual consistency. | Treat missing workflows, missing required jobs/steps, or reusable-proof misqueries as malformed hosted truth. |
| S06 archive helper labels | Refuse label reuse and archive only truthful bundles in order `main` -> `v0.1.0` -> `first-green`. | N/A | Treat missing `remote-runs.json`, incomplete phase reports, or label collisions as archive failures. |

## Load Profile

- **Shared resources**: one large Git receive-pack upload, GitHub Actions polling, and the `.tmp/m034-s06/evidence/` archive tree.
- **Per-operation cost**: at least one remote branch push, up to two tag pushes, repeated `gh run list/view` polling, and three archive-helper executions.
- **10x breakpoint**: the receive-pack upload and hosted polling cadence dominate first; retries must stay bounded and evidence-oriented instead of spinning indefinitely.

## Negative Tests

- **Malformed inputs**: stale remote `main` SHA, wrong candidate tag, missing workflow file on the remote default branch, or remote runs tied to the wrong branch/tag.
- **Error paths**: transport failure, green-but-stale hosted runs, missing push runs for `v0.1.0` / `ext-v0.3.0`, or archive-label collisions.
- **Boundary conditions**: remote `main` must advance before tag pushes are trusted, and `first-green` is only valid once every required workflow entry is `ok` in one bundle.

## Steps

1. Starting from the retained S06 transport artifacts, land the current local rollout graph on `origin/main` through a transport path that preserves history, logging every bounded recovery attempt under `.tmp/m034-s06/transport-recovery/` until the remote `main` SHA matches the intended local commit.
2. Wait for fresh `deploy.yml` and `authoritative-verification.yml` push runs on `main`, then archive the truthful `main` bundle with `bash scripts/verify-m034-s06-remote-evidence.sh main || true` and confirm those workflow entries are `ok` on the new head SHA.
3. Push or confirm `v0.1.0` and `ext-v0.3.0` from the rolled-out graph, wait for `release.yml`, `deploy-services.yml`, and `publish-extension.yml` push runs, archive `v0.1.0` and `first-green` in order, and prove the final `first-green/remote-runs.json` has no red workflow entries.

## Must-Haves

- [ ] Remote `main` actually advances to the intended rollout SHA before any tag evidence is treated as truthful.
- [ ] `main`, `v0.1.0`, and `first-green` evidence bundles exist under `.tmp/m034-s06/evidence/` with current manifests and `remote-runs.json` payloads.
- [ ] Hosted runs are checked against expected branch/tag and `headSha`, not just green status.
- [ ] If rollout still fails, the slice preserves a fresh blocker bundle instead of inventing success.
  - Estimate: 3h
  - Files: .tmp/m034-s06/push-main.stderr, .tmp/m034-s06/transport-recovery/attempts.log, scripts/verify-m034-s06-remote-evidence.sh, compiler/meshc/Cargo.toml, tools/editors/vscode-mesh/package.json, .tmp/m034-s06/evidence/main/remote-runs.json, .tmp/m034-s06/evidence/first-green/remote-runs.json
  - Verify: gh api repos/snowdamiz/mesh-lang/branches/main --jq '.commit.sha'
gh run list -R snowdamiz/mesh-lang --workflow deploy.yml --event push --branch main --limit 1 --json databaseId,status,conclusion,headSha,url
gh run list -R snowdamiz/mesh-lang --workflow authoritative-verification.yml --event push --branch main --limit 1 --json databaseId,status,conclusion,headSha,url
gh run list -R snowdamiz/mesh-lang --workflow release.yml --event push --branch v0.1.0 --limit 1 --json databaseId,status,conclusion,headSha,url
gh run list -R snowdamiz/mesh-lang --workflow deploy-services.yml --event push --branch v0.1.0 --limit 1 --json databaseId,status,conclusion,headSha,url
gh run list -R snowdamiz/mesh-lang --workflow publish-extension.yml --event push --branch ext-v0.3.0 --limit 1 --json databaseId,status,conclusion,headSha,url
bash scripts/verify-m034-s06-remote-evidence.sh first-green
python3 - <<'PY'
import json
from pathlib import Path
artifact = json.loads(Path('.tmp/m034-s06/evidence/first-green/remote-runs.json').read_text())
bad = {entry['workflowFile']: entry['status'] for entry in artifact['workflows'] if entry['status'] != 'ok'}
if bad:
    raise SystemExit(f'first-green bundle still red: {bad}')
PY
- [x] **T03: Reproduced the canonical S05 replay blocker: `remote-evidence` is still red because remote `main` and the candidate tags are not fully rolled out.** — Once the hosted graph is genuinely green, S07 is only done when the canonical acceptance entrypoint finishes. This task uses the stronger public-surface contract and the archived `first-green` bundle to rerun `bash scripts/verify-m034-s05.sh`, letting the replay itself prove `remote-evidence`, `public-http`, and the real S01 live publish/install path in one continuous run.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `.env` / publish secrets | Fail immediately, collect only the missing key names if needed, and rerun without echoing any secret values. | N/A | Treat missing or invalid publish env as a blocking proof failure, not a skipped phase. |
| Public `meshlang.dev` freshness | Use the bounded wait semantics from T01 and fail with exact body/marker diffs when stale content never settles. | Stop after the configured freshness budget and keep the last mismatch artifacts. | Treat missing installer/docs markers or wrong content types as public drift. |
| Real S01 live publish/install proof | Stop on the first publish/install failure and keep `.tmp/m034-s01/verify/` plus `s01-live-proof.log` intact for inspection. | Treat long-running live proof timeout as failure with the named phase artifact. | Treat missing `package-version.txt`, missing registry truth, or malformed verifier artifacts as failed final assembly. |

## Load Profile

- **Shared resources**: real package registry/object storage, public docs/package sites, and the full `.tmp/m034-s05/verify/` / `.tmp/m034-s01/verify/` artifact trees.
- **Per-operation cost**: one complete S05 replay including hosted polling, public HTTP checks, and a live publish/install cycle.
- **10x breakpoint**: live publish/install and public CDN settlement dominate first, so reruns should happen only after inspecting the phase artifacts and forming a new hypothesis.

## Negative Tests

- **Malformed inputs**: missing `.env`, stale public installer/docs bodies, wrong content types, or missing `package-version.txt` under `.tmp/m034-s01/verify/`.
- **Error paths**: `remote-evidence` or `public-http` still red after `first-green`, publish auth failures, duplicate-package or registry search failures, or absent final logs/artifacts.
- **Boundary conditions**: the run must finish with `status.txt=ok`, `current-phase.txt=complete`, populated `public-http.log`, and all three late phases (`remote-evidence`, `public-http`, `s01-live-proof`) marked passed.

## Steps

1. Start from `.tmp/m034-s06/evidence/first-green/remote-runs.json`, source `.env` without printing it, and run `bash scripts/verify-m034-s05.sh` from repo root so the canonical entrypoint owns hosted polling, public freshness, and the live S01 proof.
2. If the replay fails, inspect `.tmp/m034-s05/verify/{phase-report.txt,failed-phase.txt,public-http.log,*-check.log,*diff}` and the S01 verify artifacts, fix the discovered issue within slice scope, and rerun rather than weakening or bypassing the acceptance script.
3. Stop only when `.tmp/m034-s05/verify/status.txt` says `ok`, `.tmp/m034-s05/verify/current-phase.txt` says `complete`, `phase-report.txt` marks `remote-evidence`, `public-http`, and `s01-live-proof` as passed, `public-http.log` is populated, and S01 emitted a `package-version.txt` under `.tmp/m034-s01/verify/`.

## Must-Haves

- [ ] The slice finishes on the unmodified canonical acceptance entrypoint `bash scripts/verify-m034-s05.sh`, not on manual spot checks.
- [ ] `remote-evidence`, `public-http`, and `s01-live-proof` all pass in one final run with durable artifacts left on disk.
- [ ] Live public installers/docs match repo truth strongly enough that `public-http` passes without ad hoc exceptions.
- [ ] The final proof leaves an inspectable S01 package-version artifact and populated public-http diagnostics.
  - Estimate: 2h
  - Files: scripts/verify-m034-s05.sh, scripts/verify-m034-s01.sh, .env, .tmp/m034-s06/evidence/first-green/manifest.json, .tmp/m034-s06/evidence/first-green/remote-runs.json, .tmp/m034-s05/verify/status.txt, .tmp/m034-s05/verify/phase-report.txt
  - Verify: set -a && source .env && set +a && bash scripts/verify-m034-s05.sh
grep -Fx 'ok' .tmp/m034-s05/verify/status.txt
grep -Fx 'complete' .tmp/m034-s05/verify/current-phase.txt
grep -Fx 'remote-evidence	passed' .tmp/m034-s05/verify/phase-report.txt
grep -Fx 'public-http	passed' .tmp/m034-s05/verify/phase-report.txt
grep -Fx 's01-live-proof	passed' .tmp/m034-s05/verify/phase-report.txt
test -s .tmp/m034-s05/verify/public-http.log
find .tmp/m034-s01/verify -mindepth 2 -maxdepth 2 -name package-version.txt | grep -q .
  - Blocker: Remote rollout is still incomplete: `origin/main` remains at `8d3e76a65986e7bd5e00d107f9e11c1923ecd0ab`; `deploy.yml` is green there but still reflects the older hosted step graph; `authoritative-verification.yml` is absent on the remote default branch; and there are no push runs yet for `release.yml`, `deploy-services.yml`, or `publish-extension.yml` on `v0.1.0` / `ext-v0.3.0`. Until those prerequisites are fixed, `remote-evidence` will stay red and the canonical replay cannot honestly reach `public-http` or `s01-live-proof`.
