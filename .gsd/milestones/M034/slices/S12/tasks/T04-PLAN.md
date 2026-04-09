---
estimated_steps: 4
estimated_files: 5
skills_used:
  - github-workflows
  - debug-like-expert
---

# T04: Capture first-green once and finish the final assembly replay

**Slice:** S12 — Windows release-smoke remediation and final green closeout
**Milestone:** M034

## Description

Seal the repaired hosted release lane into the one-shot `first-green` archive and then finish the full `.env`-backed S05 replay from a fresh verify root. This is the closeout task. It must fail closed on any freshness mismatch, refuse to spend `first-green` early or twice, and leave a final summary that ties the full replay back to the hosted run and head SHA that made the milestone claim true.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m034-s06-remote-evidence.sh first-green` | Do not spend the archive unless the fresh hosted release lane is green on the expected ref. | Keep the archive unspent and the task red. | Treat missing or malformed manifest fields as an invalid closeout artifact. |
| Full `.env`-backed `scripts/verify-m034-s05.sh` replay | Fail closed on the first red phase and preserve the fresh verify root. | Keep the run blocking and authoritative; do not fall back to partial replay. | Treat malformed phase/status artifacts as closeout failure, not as a warning. |
| `.env` live proof surface | Stop before the full replay if required keys are missing and report only key names, never values. | Keep the missing-env condition visible without weakening the proof path. | Treat malformed live responses as real public-surface regressions. |

## Load Profile

- **Shared resources**: one-shot `first-green` archive, live `.env`-backed public proof surfaces, and the final `.tmp/m034-s05/verify/` tree.
- **Per-operation cost**: one first-green capture plus one full assembled replay.
- **10x breakpoint**: repeating the full replay without a fresh verify root or re-spending `first-green` would destroy milestone-closeout attribution; the task must guard uniqueness and freshness explicitly.

## Negative Tests

- **Malformed inputs**: missing `.env`, missing `first-green` manifest, stale `remote-runs.json`, and malformed `phase-report.txt` or status/current-phase markers.
- **Error paths**: first-green capture rejected, full replay fails at `remote-evidence`, `public-http`, or `s01-live-proof`, or the final summary cannot tie the replay back to the hosted archive.
- **Boundary conditions**: first-green already exists, first-green is fresh but the full replay regresses later, and the full replay passes with fresh hosted proof.

## Steps

1. Confirm T03's hosted summary and `remote-runs.json` both show the approved `v0.1.0` release lane green on the expected head SHA before touching `first-green`.
2. Run `bash scripts/verify-m034-s06-remote-evidence.sh first-green` exactly once and confirm `.tmp/m034-s06/evidence/first-green/manifest.json` is written with the expected label/ref context.
3. Source `.env` in-process and run the full blocking `bash scripts/verify-m034-s05.sh` path from a fresh verify root.
4. Write `.tmp/m034-s12/t04/final-closeout-summary.json` that links the final replay back to the hosted run, `first-green` manifest, and phase/status files.

## Must-Haves

- [ ] `first-green` is not spent until the hosted release lane is freshly green on the expected ref/head SHA.
- [ ] `first-green` is captured exactly once and leaves a readable manifest on disk.
- [ ] The final `.env`-backed S05 replay is fresh and passes through `remote-evidence`, `public-http`, and `s01-live-proof`.
- [ ] A final summary ties the replay back to the hosted run and `first-green` manifest so milestone revalidation is auditably fresh.

## Verification

- `bash scripts/verify-m034-s06-remote-evidence.sh first-green`
- `set -euo pipefail; test -f .env; set -a; source .env; set +a; bash scripts/verify-m034-s05.sh`
- `grep -Fxq 'ok' .tmp/m034-s05/verify/status.txt`
- `grep -Fxq 'complete' .tmp/m034-s05/verify/current-phase.txt`
- `grep -Fxq $'remote-evidence\tpassed' .tmp/m034-s05/verify/phase-report.txt`
- `grep -Fxq $'public-http\tpassed' .tmp/m034-s05/verify/phase-report.txt`
- `grep -Fxq $'s01-live-proof\tpassed' .tmp/m034-s05/verify/phase-report.txt`

## Observability Impact

- Signals added/changed: one-shot `first-green` manifest, final verify status/current-phase/phase-report markers, and the closeout summary that ties them together.
- How a future agent inspects this: read `.tmp/m034-s06/evidence/first-green/manifest.json`, `.tmp/m034-s05/verify/{status.txt,current-phase.txt,phase-report.txt}`, and `.tmp/m034-s12/t04/final-closeout-summary.json`.
- Failure state exposed: late public-surface regressions or stale-archive mismatches stay attributable after hosted green proof exists.

## Inputs

- `scripts/verify-m034-s05.sh` — canonical full assembly verifier.
- `scripts/verify-m034-s06-remote-evidence.sh` — one-shot archive helper.
- `.env` — live-proof environment required by the final assembled replay.
- `.tmp/m034-s05/verify/remote-runs.json` — refreshed hosted run state from T03.
- `.tmp/m034-s12/t03/hosted-rollout-summary.json` — hosted rerun summary that gates `first-green` spending.

## Expected Output

- `.tmp/m034-s06/evidence/first-green/manifest.json` — one-shot hosted-proof archive for the final green lane.
- `.tmp/m034-s05/verify/status.txt` — final replay status marker with `ok`.
- `.tmp/m034-s05/verify/current-phase.txt` — final replay phase marker with `complete`.
- `.tmp/m034-s05/verify/phase-report.txt` — final replay phase ledger including passed `remote-evidence`, `public-http`, and `s01-live-proof` entries.
- `.tmp/m034-s12/t04/final-closeout-summary.json` — closeout summary linking the hosted run, `first-green`, and the final replay.
