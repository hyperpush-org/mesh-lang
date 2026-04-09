---
estimated_steps: 4
estimated_files: 7
skills_used:
  - github-workflows
---

# T03: Run the full S05 assembly replay and seal the final closeout evidence

**Slice:** S11 — First-green archive and final assembly closeout
**Milestone:** M034

## Description

Use the freshly captured `first-green` archive as the hosted-proof anchor, then run the full blocking S05 replay with `.env` loaded in-process so the milestone closes on a fresh `remote-evidence` + `public-http` + `s01-live-proof` pass rather than on stale partial artifacts.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `.env`-backed live proof environment | Stop before the full replay, collect the missing-key signal without echoing secret values, and keep the task red until the real environment is present. | Preserve the failing phase and verify-root artifacts; do not switch to a weaker dry-run path. | Treat malformed env-dependent responses from the live registry/package-site proof as real regressions, not as noise. |
| `scripts/verify-m034-s05.sh` full replay | Fail closed on the first red phase and preserve the fresh `.tmp/m034-s05/verify/` tree plus `.tmp/m034-s11/t03/` summary context. | Keep the run blocking and authoritative; do not move to async polling or partial success claims. | Treat malformed phase/state artifacts as verification drift that invalidates milestone closeout. |
| `first-green` manifest linkage | Stop red if the archive manifest is missing, stale, or points at the wrong refs before trusting the full replay as milestone evidence. | Preserve the mismatch in `.tmp/m034-s11/t03/` and do not claim closeout. | Treat missing `remoteRunsSummary`, `s05Status`, or `currentPhase` fields as archive drift that must be fixed before milestone revalidation. |

## Load Profile

- **Shared resources**: real registry/package-site surfaces behind `.env`, the final `.tmp/m034-s05/verify/` tree, and the `first-green` evidence archive.
- **Per-operation cost**: one full assembled S05 replay plus small artifact assertions and summary generation.
- **10x breakpoint**: re-running the full public/live proof without fresh artifact checks can make a stale prior green look current, or hide a regression behind an old verify root.

## Negative Tests

- **Malformed inputs**: missing `.env`, missing `first-green` manifest, malformed manifest JSON, and missing final phase files.
- **Error paths**: `public-http` fails, `s01-live-proof` fails, or the full replay exits non-zero after a clean hosted `first-green` capture.
- **Boundary conditions**: fresh `first-green` plus a green final replay, fresh `first-green` plus a late public/live regression, and verify-root cleanup between runs.

## Steps

1. Confirm `.tmp/m034-s06/evidence/first-green/manifest.json` exists and still matches the expected binary and extension refs before trusting the hosted-proof archive.
2. Source `.env` in-process and run the full blocking `bash scripts/verify-m034-s05.sh` path from a fresh verify root.
3. Assert `.tmp/m034-s05/verify/status.txt = ok`, `.tmp/m034-s05/verify/current-phase.txt = complete`, and passed `remote-evidence`, `public-http`, and `s01-live-proof` entries in `.tmp/m034-s05/verify/phase-report.txt`.
4. Write a compact `.tmp/m034-s11/t03/final-assembly-summary.json` that links the final replay back to the `first-green` manifest and stop red if any phase regresses.

## Must-Haves

- [ ] The full S05 replay runs with `.env` loaded in-process and no secrets echoed.
- [ ] The final verify root is fresh and records `status.txt = ok` plus `current-phase.txt = complete`.
- [ ] `remote-evidence`, `public-http`, and `s01-live-proof` all pass in `.tmp/m034-s05/verify/phase-report.txt`.
- [ ] The final closeout summary ties the fresh green replay back to the one-shot `first-green` archive.

## Verification

- `set -euo pipefail; test -f .env; set -a; source .env; set +a; bash scripts/verify-m034-s05.sh`
- `grep -Fxq 'ok' .tmp/m034-s05/verify/status.txt`
- `grep -Fxq 'complete' .tmp/m034-s05/verify/current-phase.txt`
- `grep -Fxq $'remote-evidence	passed' .tmp/m034-s05/verify/phase-report.txt`
- `grep -Fxq $'public-http	passed' .tmp/m034-s05/verify/phase-report.txt`
- `grep -Fxq $'s01-live-proof	passed' .tmp/m034-s05/verify/phase-report.txt`

## Observability Impact

- Signals added/changed: a fresh `.tmp/m034-s05/verify/` tree tied to the final full replay, plus `.tmp/m034-s11/t03/final-assembly-summary.json`.
- How a future agent inspects this: read `.tmp/m034-s05/verify/status.txt`, `.tmp/m034-s05/verify/current-phase.txt`, `.tmp/m034-s05/verify/phase-report.txt`, and `.tmp/m034-s11/t03/final-assembly-summary.json`.
- Failure state exposed: late public-surface or live-proof regressions stay attributable after a green `first-green` capture.

## Inputs

- `scripts/verify-m034-s05.sh` — canonical full assembly verifier.
- `.env` — local live-proof environment required by the final assembled replay.
- `.tmp/m034-s06/evidence/first-green/manifest.json` — one-shot hosted-proof archive that T03 must link to the final replay.
- `.tmp/m034-s05/verify/remote-runs.json` — fresh hosted green summary from T02.
- `.tmp/m034-s11/t02/hosted-rollout-summary.json` — local hosted-rollout context for the final closeout summary.

## Expected Output

- `.tmp/m034-s05/verify/status.txt` — final replay status marker with `ok`.
- `.tmp/m034-s05/verify/current-phase.txt` — final replay phase marker with `complete`.
- `.tmp/m034-s05/verify/phase-report.txt` — final replay phase ledger including passed `remote-evidence`, `public-http`, and `s01-live-proof` entries.
- `.tmp/m034-s11/t03/final-assembly-summary.json` — closeout summary linking `first-green` to the final S05 replay.
