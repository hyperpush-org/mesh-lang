# S03: Hosted evidence chain fails on starter deploy or packages drift — UAT

**Milestone:** M053
**Written:** 2026-04-05T21:07:54.277Z

# S03 UAT — Hosted evidence chain fails on starter deploy or packages drift

## Preconditions

- Worktree contains the S03 workflow/verifier changes.
- `gh`, `git`, `python3`, `ruby`, and `node` are installed.
- `GH_TOKEN` is available either in the environment or in repo-root `.env`.
- Network access to GitHub is available for the live hosted replay.

## Test Case 1 — Local workflow topology stays fail-closed

1. Run `bash scripts/verify-m034-s02-workflows.sh`.
   - **Expected:** Exit code `0`.
   - **Expected:** The verifier confirms:
     - `.github/workflows/authoritative-starter-failover-proof.yml` exists.
     - The reusable workflow uses runner-local `postgres:16`, exports a masked `DATABASE_URL`, runs `bash scripts/verify-m053-s02.sh`, and uploads `.tmp/m053-s02/**` on failure.
     - `.github/workflows/authoritative-verification.yml` references the reusable starter workflow exactly once as a secret-free whitespace-guarded lane.
     - `.github/workflows/release.yml` references the reusable starter workflow exactly once and requires it in `release.needs`.

2. Intentionally inspect the generated verifier logs under `.tmp/m034-s02/verify/`.
   - **Expected:** `full-contract.log` shows reusable, starter-reusable, caller, and release contract sweeps all passing.

## Test Case 2 — Contract tests cover both green and fail-closed hosted scenarios

1. Run `node --test scripts/tests/verify-m053-s03-contract.test.mjs`.
   - **Expected:** Exit code `0` with `14` passing tests.

2. Confirm the suite includes the following negative cases in its output names:
   - missing `GH_TOKEN`
   - stale authoritative `main` SHA
   - `deploy-services.yml` evidence only on the tag and not on `main`
   - missing `Hosted starter failover proof` job
   - missing `Verify public surface contract` step
   - missing remote workflow / malformed `gh` JSON / missing `remote-runs.json`
   - **Expected:** Each of those cases is modeled as a fail-closed contract test rather than ignored drift.

## Test Case 3 — Live hosted replay exposes real drift with retained evidence

1. Run the live verifier with GitHub auth, for example:
   - `GH_TOKEN=<token> bash scripts/verify-m053-s03.sh`
   - or parse only `GH_TOKEN` from `.env` and pass it to the command.

2. Observe the command result.
   - **Expected:** Current remote state exits non-zero in `remote-evidence`.
   - **Expected:** This is treated as a successful UAT outcome for S03 because the slice promise is to fail when starter deploy proof or packages/public-surface truth drifts.

3. Inspect `.tmp/m053-s03/verify/status.txt`, `.tmp/m053-s03/verify/current-phase.txt`, and `.tmp/m053-s03/verify/phase-report.txt`.
   - **Expected:** `status.txt=failed`.
   - **Expected:** `current-phase.txt=remote-evidence`.
   - **Expected:** `phase-report.txt` shows `gh-preflight` and `candidate-refs` passed before `remote-evidence` failed.

4. Inspect `.tmp/m053-s03/verify/remote-runs.json`.
   - **Expected:** `deploy-services.yml` has `status: ok`, a fresh `main` SHA, matched `Deploy mesh-packages website`, and matched `Post-deploy health checks` with `Verify public surface contract` present.
   - **Expected:** `authoritative-verification.yml` has `status: failed` with a reason that the latest green hosted run is missing `Hosted starter failover proof`.
   - **Expected:** `release.yml` has `status: failed` with a reason that `refs/tags/v0.1.0^{}` could not be resolved for release freshness.

5. Inspect the retained query logs.
   - **Expected:** `.tmp/m053-s03/verify/authoritative-verification-view.log`, `.tmp/m053-s03/verify/deploy-services-view.log`, and `.tmp/m053-s03/verify/release-expected-ref.log` exist and line up with the JSON failure reasons.

## Edge Cases

- If `GH_TOKEN` is missing, the verifier must stop in `gh-preflight` before creating `candidate-refs.json`.
- If remote `main` is stale or packages proof exists only on the tag, `remote-runs.json` must mark freshness as failed rather than accepting the stale green run.
- If local workflow wiring drifts (missing reusable workflow, wrong diagnostics path, dropped release dependency), `bash scripts/verify-m034-s02-workflows.sh` must fail before any hosted query is trusted.
