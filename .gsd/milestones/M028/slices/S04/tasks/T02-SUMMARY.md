---
id: T02
parent: S04
milestone: M028
provides:
  - ignored staged-artifact deployment proof for reference-backend that stages outside the repo root, applies deploy SQL, starts the staged binary from the staged bundle, and cross-checks health, jobs, _mesh_migrations, and log redaction
key_files:
  - compiler/meshc/tests/e2e_reference_backend.rs
  - .gsd/KNOWLEDGE.md
key_decisions:
  - kept the deploy proof in the existing e2e_reference_backend harness and seeded follow-up jobs until the staged worker itself reported ownership in /health, which keeps the proof stable even if stray shared-DB workers exist during local verification
patterns_established:
  - stage the bundle into a temp dir, apply the staged SQL from that same temp dir, start the staged binary with bundle-dir cwd, run the staged deploy-smoke script, then assert worker-owned truth through /health, /jobs/:id, jobs, and _mesh_migrations
observability_surfaces:
  - compiler/meshc/tests/e2e_reference_backend.rs ignored deploy-artifact proof
  - reference-backend/scripts/stage-deploy.sh named bundle-phase output
  - reference-backend/scripts/apply-deploy-migrations.sh named apply-phase output
  - reference-backend/scripts/deploy-smoke.sh named health/create/poll/processed output
  - /health
  - /jobs/:id
  - _mesh_migrations
  - jobs
  - staged binary startup/job logs
duration: 3h 20m
verification_result: passed
completed_at: 2026-03-23T21:31:00-0400
blocker_discovered: false
---

# T02: Prove staged-artifact deployment in the backend e2e harness

**Added an ignored staged-deploy backend e2e that applies the checked-in SQL artifact, starts the staged binary from a temp bundle outside the repo root, and proves health, job, migration, and redaction truth end to end.**

## What Happened

I extended `compiler/meshc/tests/e2e_reference_backend.rs` with the missing staged-artifact operational proof required by S04.

The harness now has temp-dir helpers to:
- run `reference-backend/scripts/stage-deploy.sh` into a bundle outside the repo root,
- run the staged `apply-deploy-migrations.sh` against the staged `reference-backend.up.sql`,
- start the staged `reference-backend` binary with the staged bundle as its working directory,
- run the staged `deploy-smoke.sh`, and
- parse/assert the staged script output instead of trusting exit codes alone.

I added the ignored test `e2e_reference_backend_deploy_artifact_smoke`, which resets the DB, stages the deploy bundle, applies the boring SQL artifact, asserts `_mesh_migrations` truth, starts the staged binary, runs the staged smoke flow, and then cross-checks durable runtime truth through `/health`, `/jobs/:id`, `jobs`, and staged binary logs without echoing `DATABASE_URL`.

During verification I found a real local-environment gotcha: stray `reference-backend` workers on the same shared Postgres can steal the smoke-created job before the staged instance records ownership in `/health`. I fixed the proof at the harness level by keeping the staged smoke flow intact, then seeding follow-up jobs until the staged worker itself recorded a processed job in `/health`; that keeps the test authoritative about staged-worker ownership instead of relying on a clean local machine.

I also hardened the helper polling path so it retries transient HTTP read failures instead of panicking on the first socket hiccup.

## Verification

I ran the task-plan verification and the relevant slice-level checks against the current worktree.

The core T02 proof now passes:
- `cargo test -p meshc e2e_self_contained_binary -- --nocapture`
- `DATABASE_URL=... cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_deploy_artifact_smoke -- --ignored --nocapture`

I also reran the supporting S04 checks that this task materially depends on:
- `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_builds -- --nocapture`
- `tmp_dir="$(mktemp -d)" && bash reference-backend/scripts/stage-deploy.sh "$tmp_dir" && test -x "$tmp_dir/reference-backend" && test -f "$tmp_dir/reference-backend.up.sql" && test -x "$tmp_dir/deploy-smoke.sh"`
- `tmp_dir="$(mktemp -d)" && if bash reference-backend/scripts/apply-deploy-migrations.sh "$tmp_dir/missing-reference-backend.up.sql" ...; then ...; else rg ...; fi`

The README/env-doc verification remains owned by T03 and was not rerun here.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc e2e_self_contained_binary -- --nocapture` | 0 | ✅ pass | 11.0s |
| 2 | `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_builds -- --nocapture` | 0 | ✅ pass | 8.10s |
| 3 | `tmp_dir="$(mktemp -d)" && bash reference-backend/scripts/stage-deploy.sh "$tmp_dir" && test -x "$tmp_dir/reference-backend" && test -f "$tmp_dir/reference-backend.up.sql" && test -x "$tmp_dir/deploy-smoke.sh"` | 0 | ✅ pass | 91.8s |
| 4 | `DATABASE_URL=... cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_deploy_artifact_smoke -- --ignored --nocapture` | 0 | ✅ pass | 73.75s |
| 5 | `tmp_dir="$(mktemp -d)" && if bash reference-backend/scripts/apply-deploy-migrations.sh "$tmp_dir/missing-reference-backend.up.sql" >"$tmp_dir/apply-missing.log" 2>&1; then echo "expected apply-deploy-migrations.sh to fail for a missing artifact" >&2; exit 1; else rg -n "\[deploy-apply\] missing deploy SQL artifact" "$tmp_dir/apply-missing.log"; fi` | 0 | ✅ pass | 91.8s |

## Diagnostics

Rerun `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_deploy_artifact_smoke -- --ignored --nocapture` with `DATABASE_URL` loaded to exercise the full staged-artifact proof.

On failure, inspect the emitted bundle dir plus the named `stage_output`, `apply_output`, `smoke_output`, and staged binary stdout/stderr that the test includes in its panic message.

Use the staged scripts directly for manual inspection:
- `bash reference-backend/scripts/stage-deploy.sh <tmp-dir>`
- `DATABASE_URL=... bash <tmp-dir>/apply-deploy-migrations.sh <tmp-dir>/reference-backend.up.sql`
- `BASE_URL=http://127.0.0.1:<port> bash <tmp-dir>/deploy-smoke.sh`

The proof now exposes these concrete inspection surfaces: `_mesh_migrations`, `jobs`, `/health`, `/jobs/:id`, staged bundle paths, and readable staged-runtime logs that do not print `DATABASE_URL`.

## Deviations

I made one local harness adaptation beyond the original task text: after the staged smoke script proves the deploy path from the staged bundle, the test may seed a few follow-up jobs until the staged instance itself records ownership in `/health`. This was necessary to keep the proof stable in a dirty local environment where unrelated `reference-backend` workers may still be attached to the same Postgres database.

## Known Issues

- The broader slice docs verification (`reference-backend/README.md` and `.env.example`) still belongs to T03 and remains outside this task’s code changes.
- In this environment, `async_bash` intermittently misreported `cargo test -p meshc --test e2e_reference_backend ...` target discovery; blocking `bash` produced the authoritative verification evidence.

## Files Created/Modified

- `compiler/meshc/tests/e2e_reference_backend.rs` — added staged bundle/apply/start/smoke helpers, hardened polling, and implemented the ignored `e2e_reference_backend_deploy_artifact_smoke` proof.
- `.gsd/KNOWLEDGE.md` — recorded the local `.env` loading requirement and the async Cargo-target discovery gotcha that affected verification.
- `.gsd/milestones/M028/slices/S04/tasks/T02-SUMMARY.md` — recorded the durable execution and verification artifact for this task.
