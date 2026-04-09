---
estimated_steps: 25
estimated_files: 8
skills_used: []
---

# T02: Created the candidate tags on the rolled-out SHA and captured the hosted blockers preventing a truthful first-green archive.

This is the only outward-facing step in the slice. It must not happen speculatively. The task first proves the intended commit and tag names locally, asks the user for explicit approval to create and push `v0.1.0` and `ext-v0.3.0`, then watches the resulting hosted `push` runs until the candidate-tag workflows either go green or produce concrete failure evidence for the next iteration.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `git` tag/push to `origin` | Stop immediately, keep the exact stderr, and do not claim any hosted evidence progress. | Treat a stalled push as an external transport blocker and preserve the last attempted ref state. | Treat an unexpected remote SHA or missing remote tag as failure, even if a local tag exists. |
| GitHub Actions hosted runs | Poll until the expected run exists on the expected tag or a bounded wait expires, then keep the run metadata for inspection. | Preserve the last observed run status and URL rather than looping forever. | Treat wrong `headBranch`, wrong `headSha`, or missing required jobs as failure, not as eventual-consistency success. |
| User confirmation gate | Do not create or push tags until the user explicitly approves the exact outward action. | N/A | Treat ambiguous confirmation as "no" and stop cleanly. |

## Load Profile

- **Shared resources**: remote git refs, GitHub Actions queues, and the `.tmp/m034-s08/tag-rollout/` monitoring files.
- **Per-operation cost**: two tag pushes plus repeated `gh run list/view` polling for `release.yml`, `deploy-services.yml`, and `publish-extension.yml`.
- **10x breakpoint**: hosted polling dominates first, so the task should use bounded waits and durable JSON snapshots instead of ad hoc terminal-only checks.

## Negative Tests

- **Malformed inputs**: wrong target SHA, missing local version/tag derivation, or absent user approval.
- **Error paths**: push rejected, tag already exists remotely with the wrong SHA, hosted run fails, or hosted run is green on the wrong ref.
- **Boundary conditions**: the task must prove both remote tags exist and each candidate workflow run points at the same intended rollout commit before T03 can archive `first-green`.

## Steps

1. Confirm local `HEAD`, remote `main`, and the derived tags from `compiler/meshc/Cargo.toml` plus `tools/editors/vscode-mesh/package.json` all line up with the intended rollout commit.
2. Present the exact outward action (`git tag` / `git push origin v0.1.0 ext-v0.3.0`) and wait for explicit user confirmation before mutating any remote state.
3. Create and push the tags, then poll `gh run list/view` for `release.yml`, `deploy-services.yml`, and `publish-extension.yml`, persisting the latest successful or failing run payloads under `.tmp/m034-s08/tag-rollout/`.
4. Stop only when the remote tags exist and the monitored run payloads show completed green runs on the expected branch/tag and `headSha`, or when a concrete hosted blocker is captured for follow-up.

## Must-Haves

- [ ] No remote tag creation or push happens before explicit user confirmation.
- [ ] `origin` ends the task with both `v0.1.0` and `ext-v0.3.0` present on the intended rollout commit.
- [ ] Durable run snapshots exist for `release.yml`, `deploy-services.yml`, and `publish-extension.yml`.
- [ ] The task distinguishes wrong-ref/stale-run failures from genuinely green candidate-tag runs.

## Inputs

- ``compiler/meshc/Cargo.toml``
- ``compiler/meshpkg/Cargo.toml``
- ``tools/editors/vscode-mesh/package.json``
- ``.tmp/m034-s06/evidence/s08-prepush/remote-runs.json``

## Expected Output

- ``.tmp/m034-s08/tag-rollout/tag-refs.txt``
- ``.tmp/m034-s08/tag-rollout/workflow-status.json``
- ``.tmp/m034-s08/tag-rollout/release-v0.1.0-view.json``
- ``.tmp/m034-s08/tag-rollout/deploy-services-v0.1.0-view.json``
- ``.tmp/m034-s08/tag-rollout/publish-extension-ext-v0.3.0-view.json``

## Verification

bash -c 'set -euo pipefail; test -s .tmp/m034-s08/tag-rollout/tag-refs.txt; grep -F "refs/tags/v0.1.0" .tmp/m034-s08/tag-rollout/tag-refs.txt; grep -F "refs/tags/ext-v0.3.0" .tmp/m034-s08/tag-rollout/tag-refs.txt'
python3 - <<'PY'
import json
from pathlib import Path
summary = json.loads(Path('.tmp/m034-s08/tag-rollout/workflow-status.json').read_text())
expected = {
    'release.yml': 'v0.1.0',
    'deploy-services.yml': 'v0.1.0',
    'publish-extension.yml': 'ext-v0.3.0',
}
for workflow, ref_name in expected.items():
    entry = summary[workflow]
    assert entry['headBranch'] == ref_name, (workflow, entry)
    assert entry['status'] == 'completed', (workflow, entry)
    assert entry['conclusion'] == 'success', (workflow, entry)
    assert entry['headSha'], (workflow, entry)
PY

## Observability Impact

- Signals added/changed: per-workflow run JSON plus a compact tag-ref snapshot under `.tmp/m034-s08/tag-rollout/`.
- How a future agent inspects this: read the saved `gh run view` payloads and `workflow-status.json` instead of re-polling from scratch.
- Failure state exposed: push rejection, missing tag, wrong `headSha`, or non-green hosted run stays visible as structured data.
