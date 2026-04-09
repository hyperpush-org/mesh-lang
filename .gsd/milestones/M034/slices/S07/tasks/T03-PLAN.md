---
estimated_steps: 3
estimated_files: 7
skills_used:
  - debug-like-expert
  - test
---

# T03: Replay the canonical S05 assembly and prove live public freshness end to end

Once the hosted graph is genuinely green, S07 is only done when the canonical acceptance entrypoint finishes. This task uses the stronger public-surface contract and the archived `first-green` bundle to rerun `bash scripts/verify-m034-s05.sh`, letting the replay itself prove `remote-evidence`, `public-http`, and the real S01 live publish/install path in one continuous run.

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

## Inputs

- `scripts/verify-m034-s05.sh`
- `scripts/verify-m034-s01.sh`
- `.env`
- `.tmp/m034-s06/evidence/first-green/manifest.json`
- `.tmp/m034-s06/evidence/first-green/remote-runs.json`

## Expected Output

- `.tmp/m034-s05/verify/status.txt`
- `.tmp/m034-s05/verify/current-phase.txt`
- `.tmp/m034-s05/verify/phase-report.txt`
- `.tmp/m034-s05/verify/public-http.log`
- `.tmp/m034-s05/verify/remote-runs.json`
- `.tmp/m034-s05/verify/s01-live-proof.log`

## Verification

set -a && source .env && set +a && bash scripts/verify-m034-s05.sh
grep -Fx 'ok' .tmp/m034-s05/verify/status.txt
grep -Fx 'complete' .tmp/m034-s05/verify/current-phase.txt
grep -Fx 'remote-evidence	passed' .tmp/m034-s05/verify/phase-report.txt
grep -Fx 'public-http	passed' .tmp/m034-s05/verify/phase-report.txt
grep -Fx 's01-live-proof	passed' .tmp/m034-s05/verify/phase-report.txt
test -s .tmp/m034-s05/verify/public-http.log
find .tmp/m034-s01/verify -mindepth 2 -maxdepth 2 -name package-version.txt | grep -q .

## Observability Impact

- Signals added/changed: the canonical replay must leave `status.txt`, `current-phase.txt`, `phase-report.txt`, `public-http.log`, `remote-runs.json`, and S01 live-proof logs/artifacts populated.
- How a future agent inspects this: rerun `bash scripts/verify-m034-s05.sh`, then inspect `.tmp/m034-s05/verify/` and `.tmp/m034-s01/verify/` without needing separate ad hoc curls.
- Failure state exposed: a future agent can distinguish hosted-rollout regression, public-surface drift, and S01 publish/install failure from the final replay artifacts alone.
