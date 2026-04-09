---
id: T03
parent: S04
milestone: M028
provides:
  - operator-facing package-local docs for the verified staged native deployment path, with the runtime env contract kept aligned to the shipped bundle scripts
key_files:
  - reference-backend/README.md
  - reference-backend/.env.example
  - .gsd/milestones/M028/slices/S04/tasks/T03-PLAN.md
  - .gsd/milestones/M028/slices/S04/S04-PLAN.md
key_decisions:
  - kept the runtime contract narrow in docs: the runtime host gets the staged bundle plus DATABASE_URL/PORT/JOB_POLL_MS, while meshc stays on the build host and psql/curl/python3 remain operator tools around the staged artifact
patterns_established:
  - document the staged deploy path in the same build/apply/run/smoke order proven by the package scripts and ignored e2e, and separate build-host prerequisites from runtime-host prerequisites explicitly
observability_surfaces:
  - reference-backend/README.md boring native deployment runbook
  - reference-backend/.env.example runtime contract comments
  - reference-backend/scripts/stage-deploy.sh named bundle-phase output
  - reference-backend/scripts/apply-deploy-migrations.sh named apply-phase output
  - reference-backend/scripts/deploy-smoke.sh named health/create/poll/processed output
  - compiler/meshc/tests/e2e_reference_backend.rs ignored deploy-artifact proof
duration: 1h 10m
verification_result: passed
completed_at: 2026-03-23T17:46:29-0400
blocker_discovered: false
---

# T03: Document the operator-facing boring deployment workflow

**Documented the verified staged native deploy runbook for `reference-backend` and aligned the runtime env example.**

## What Happened

I added a `Boring native deployment` section to `reference-backend/README.md` that now tells the exact package-local build/apply/run/smoke story proven in S04.

The README now separates build host requirements from runtime host requirements, shows the staged bundle contents by name, documents the runtime-side `psql` apply step with `apply-deploy-migrations.sh`, shows how to start the staged binary from the staged location, and points operators at the probe-only `deploy-smoke.sh` command.

I kept the deployment language concrete instead of marketing-heavy: the runtime host does not need `meshc` after staging, but it still needs the staged bundle, `DATABASE_URL`/`PORT`/`JOB_POLL_MS`, reachable Postgres, and the small operator tools around that path (`psql` for apply, `curl`/`python3` for smoke).

I updated `reference-backend/.env.example` to keep the staged runtime contract explicit and to clarify that `BASE_URL` is only an optional `deploy-smoke.sh` override, not part of the staged binary’s required runtime env.

As required by the task pre-flight note, I also added `## Observability Impact` to `T03-PLAN.md` so future agents know which package-local signals and failure stages this task is responsible for.

## Verification

I cross-checked the docs against the shipped deploy scripts and the ignored deploy-artifact e2e, then ran the exact slice-level verification commands in the foreground.

The README/env grep check passed, the staged bundle and missing-artifact failure-path checks passed, and the backend proof commands passed, including the ignored `e2e_reference_backend_deploy_artifact_smoke` run with `.env` sourced for `DATABASE_URL`.

I also reran the slice’s supporting non-doc checks on this final task because S04 requires the full boring-deployment story to stay green at slice close.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_builds -- --nocapture` | 0 | ✅ pass | 9.70s |
| 2 | `tmp_dir="$(mktemp -d)" && bash reference-backend/scripts/stage-deploy.sh "$tmp_dir" && test -x "$tmp_dir/reference-backend" && test -f "$tmp_dir/reference-backend.up.sql" && test -x "$tmp_dir/deploy-smoke.sh"` | 0 | ✅ pass | 8.75s |
| 3 | `cargo test -p meshc e2e_self_contained_binary -- --nocapture` | 0 | ✅ pass | 22.93s |
| 4 | `set -a && source ./.env && set +a && cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_deploy_artifact_smoke -- --ignored --nocapture` | 0 | ✅ pass | 67.74s |
| 5 | `tmp_dir="$(mktemp -d)" && if bash reference-backend/scripts/apply-deploy-migrations.sh "$tmp_dir/missing-reference-backend.up.sql" >"$tmp_dir/apply-missing.log" 2>&1; then echo "expected apply-deploy-migrations.sh to fail for a missing artifact" >&2; exit 1; else rg -n "\[deploy-apply\] missing deploy SQL artifact" "$tmp_dir/apply-missing.log"; fi` | 0 | ✅ pass | 0.20s |
| 6 | `rg -n "Boring native deployment|stage-deploy\.sh|apply-deploy-migrations\.sh|deploy-smoke\.sh|runtime host" reference-backend/README.md && rg -n "^DATABASE_URL=|^PORT=|^JOB_POLL_MS=" reference-backend/.env.example` | 0 | ✅ pass | 0.06s |

## Diagnostics

Future agents should inspect the package-local boring deploy story in `reference-backend/README.md`, then verify behavior against:

- `reference-backend/scripts/stage-deploy.sh`
- `reference-backend/scripts/apply-deploy-migrations.sh`
- `reference-backend/scripts/deploy-smoke.sh`
- `compiler/meshc/tests/e2e_reference_backend.rs`

For Postgres-backed compiler proofs in this worktree, source `./.env` into the subprocess before running the ignored backend e2e so `DATABASE_URL` is present without echoing it.

If cargo target discovery looks wrong under the background runner, use foreground `bash` for authoritative verification; the foreground runs here matched the actual repo state.

## Deviations

I made one required pre-flight change outside the original task body: I added the missing `## Observability Impact` section to `T03-PLAN.md` before proceeding.

## Known Issues

None in the shipped `reference-backend` docs or deploy path. The only mismatch I hit was tooling-side: the background runner intermittently reported false path/target failures, while direct foreground verification passed.

## Files Created/Modified

- `reference-backend/README.md` — added the operator-facing `Boring native deployment` runbook with the verified build/apply/run/smoke sequence and explicit build-host vs runtime-host requirements.
- `reference-backend/.env.example` — clarified the staged runtime env contract and the optional `deploy-smoke.sh` `BASE_URL` override.
- `.gsd/milestones/M028/slices/S04/tasks/T03-PLAN.md` — added the required `Observability Impact` section before task execution.
- `.gsd/milestones/M028/slices/S04/S04-PLAN.md` — marked T03 complete.
- `.gsd/milestones/M028/slices/S04/tasks/T03-SUMMARY.md` — recorded the execution and verification artifact for this task.
