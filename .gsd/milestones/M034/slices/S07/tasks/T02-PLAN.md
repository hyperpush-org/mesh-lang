---
estimated_steps: 3
estimated_files: 7
skills_used:
  - github-workflows
  - gh
  - debug-like-expert
---

# T02: Land the rollout graph on remote main and candidate tags, then archive first-green hosted evidence

No local code change can move S07 forward if remote `main` stays stale or if the candidate-tag workflows never run. This task turns the S06 transport blocker into either a resolved rollout or a fresh truthful blocker bundle: it must land the current rollout graph on `origin/main` without rewriting history, then advance `v0.1.0` and `ext-v0.3.0`, and archive `main`, `v0.1.0`, and `first-green` evidence bundles in order.

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

## Inputs

- `.tmp/m034-s06/push-main.stderr`
- `.tmp/m034-s06/transport-recovery/attempts.log`
- `scripts/verify-m034-s06-remote-evidence.sh`
- `.tmp/m034-s06/evidence/closeout-20260326-1525/remote-runs.json`
- `compiler/meshc/Cargo.toml`
- `compiler/meshpkg/Cargo.toml`
- `tools/editors/vscode-mesh/package.json`

## Expected Output

- `.tmp/m034-s06/transport-recovery/attempts.log`
- `.tmp/m034-s06/evidence/main/manifest.json`
- `.tmp/m034-s06/evidence/main/remote-runs.json`
- `.tmp/m034-s06/evidence/v0.1.0/manifest.json`
- `.tmp/m034-s06/evidence/v0.1.0/remote-runs.json`
- `.tmp/m034-s06/evidence/first-green/manifest.json`
- `.tmp/m034-s06/evidence/first-green/remote-runs.json`

## Verification

gh api repos/snowdamiz/mesh-lang/branches/main --jq '.commit.sha'
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

## Observability Impact

- Signals added/changed: bounded transport-recovery logs tied to target SHA plus labeled `main`, `v0.1.0`, and `first-green` manifests.
- How a future agent inspects this: read `.tmp/m034-s06/transport-recovery/attempts.log`, inspect `.tmp/m034-s06/evidence/<label>/manifest.json`, and follow the run URLs/head SHAs inside `remote-runs.json`.
- Failure state exposed: the task makes transport failure, stale hosted SHA, missing push runs, and archive-label misuse visible as separate states.
