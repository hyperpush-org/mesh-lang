---
estimated_steps: 5
estimated_files: 8
skills_used:
  - github-workflows
  - debug-like-expert
---

# T02: Roll the approved release fixes onto the hosted refs and capture `first-green` exactly once

**Slice:** S11 — First-green archive and final assembly closeout
**Milestone:** M034

## Description

Advance the repaired release-lane surface onto the actual refs that S05 gates on, refresh the canonical `remote-evidence` proof, and spend the one-shot `first-green` label only after the stop-after replay is truly green on the expected split binary/extension refs.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| GitHub ref mutation / workflow rerun path | Stop before mutation until explicit user confirmation is granted, then preserve the exact outward action and failure output under `.tmp/m034-s11/t02/`. | Keep polling bounded, record the last observed run URL and `headSha`, and fail with the stale/unfinished state instead of guessing success. | Treat missing `headSha`, run URL, or expected-ref metadata as evidence drift and keep the task red. |
| `scripts/verify-m034-s05.sh` stop-after `remote-evidence` replay | Fail closed, keep the fresh `.tmp/m034-s05/verify/` tree, and use it to localize which workflow or freshness check stayed red. | Preserve the partial verify root and stop before `first-green` capture. | Treat malformed `remote-runs.json` or missing `candidate-tags.json` as proof-surface regressions. |
| `scripts/verify-m034-s06-remote-evidence.sh` | Do not retry `first-green` blindly; inspect the label-use failure or manifest drift and stop before spending the one-shot archive incorrectly. | Preserve the wrapper output and keep the archive label unused if the stop-after gate never reached green. | Treat a manifest missing `s05Status`, `currentPhase`, or `remoteRunsSummary` fields as archive drift that invalidates closeout. |

## Load Profile

- **Shared resources**: remote `main` / binary tag refs, hosted workflow queues, `.tmp/m034-s05/verify/`, and `.tmp/m034-s06/evidence/`.
- **Per-operation cost**: one approved rollout mutation path, one canonical stop-after replay, one archive capture, and read-only GitHub inspection.
- **10x breakpoint**: repeated reruns or ref mutations without fresh artifact checks can spend the one-shot archive or blur which hosted state is authoritative.

## Negative Tests

- **Malformed inputs**: missing rollout target SHA, wrong expected ref for a workflow, stale extension tag assumptions, and absent hosted run metadata.
- **Error paths**: user withholds confirmation, ref mutation fails, hosted rerun stays red, stop-after replay exits non-zero, or archive capture rejects the label.
- **Boundary conditions**: binary refs move while extension refs stay put, hosted workflows already green on the correct refs, and the first replay where every workflow is green but `first-green` is still absent.

## Steps

1. Inspect local/remote ref drift, the current hosted failure artifacts, and the expected split binary/extension refs, then prepare the exact outward-action summary for user approval.
2. After explicit confirmation, advance only the refs S05 actually gates on, preserving the split binary vs extension tag model unless extension files changed.
3. Replay `VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh` until `.tmp/m034-s05/verify/remote-runs.json` shows every workflow at `status: ok` on its expected ref/head SHA.
4. Run `bash scripts/verify-m034-s06-remote-evidence.sh first-green` exactly once and verify the resulting manifest fields before moving on.
5. Preserve a compact hosted rollout summary under `.tmp/m034-s11/t02/` so T03 can trust the archive without re-reading GitHub manually.

## Must-Haves

- [ ] No outward GitHub action happens without explicit user confirmation recorded in the task narrative.
- [ ] The release-lane fixes reach the refs that `scripts/verify-m034-s05.sh` actually checks.
- [ ] `.tmp/m034-s05/verify/remote-runs.json` records every workflow at `status: ok` on its expected ref/head SHA.
- [ ] `.tmp/m034-s06/evidence/first-green/manifest.json` exists exactly once and records `s05Status: ok` plus `currentPhase: stopped-after-remote-evidence`.

## Verification

- `VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh`
- `bash scripts/verify-m034-s06-remote-evidence.sh first-green`
- `python3 -c "import json; from pathlib import Path; data = json.loads(Path('.tmp/m034-s05/verify/remote-runs.json').read_text()); assert all(entry['status'] == 'ok' for entry in data['workflows'])"`
- `python3 -c "import json; from pathlib import Path; manifest = json.loads(Path('.tmp/m034-s06/evidence/first-green/manifest.json').read_text()); assert manifest['s05Status'] == 'ok'; assert manifest['currentPhase'] == 'stopped-after-remote-evidence'; assert all(entry['status'] == 'ok' for entry in manifest['remoteRunsSummary'])"`

## Observability Impact

- Signals added/changed: refreshed `.tmp/m034-s05/verify/remote-runs.json`, `.tmp/m034-s05/verify/candidate-tags.json`, and `.tmp/m034-s06/evidence/first-green/manifest.json` tied to the approved rollout state.
- How a future agent inspects this: read `.tmp/m034-s05/verify/remote-runs.json`, `.tmp/m034-s06/evidence/first-green/manifest.json`, and `.tmp/m034-s11/t02/hosted-rollout-summary.json`.
- Failure state exposed: wrong-ref runs, stale green evidence, and one-shot archive misuse stay attributable instead of being overwritten.

## Inputs

- `.github/workflows/release.yml` — repaired release workflow from T01 that must reach the hosted release lane.
- `scripts/verify-m034-s05.sh` — canonical stop-after replay used as the authoritative green gate.
- `scripts/verify-m034-s06-remote-evidence.sh` — one-shot archive wrapper that must only run after the stop-after gate is green.
- `.tmp/m034-s05/verify/remote-runs.json` — previous hosted evidence to replace with fresh green proof.
- `.tmp/m034-s09/rollout/target-sha.txt` — latest approved rollout target reference from the prior slice.
- `.tmp/m034-s09/rollout/workflow-status.json` — current hosted-status snapshot to compare against the refreshed proof.
- `README.md` — public documentation for the split binary vs extension tag model.
- `website/docs/docs/tooling/index.md` — docs surface that must stay aligned with the split-tag rollout model.

## Expected Output

- `.tmp/m034-s05/verify/remote-runs.json` — refreshed canonical remote-evidence summary with every workflow green.
- `.tmp/m034-s05/verify/candidate-tags.json` — fresh expected-ref payload for the green stop-after replay.
- `.tmp/m034-s06/evidence/first-green/manifest.json` — one-shot archive manifest proving the green hosted closeout state.
- `.tmp/m034-s11/t02/hosted-rollout-summary.json` — compact local summary of approved mutations, hosted run URLs, head SHAs, and archive status.
