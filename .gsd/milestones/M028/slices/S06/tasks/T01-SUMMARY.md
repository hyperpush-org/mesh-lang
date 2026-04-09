---
id: T01
parent: S06
milestone: M028
provides:
  - Cooperative worker-exit recovery path groundwork and a documented resume point for the blocked proof gates
key_files:
  - reference-backend/jobs/worker.mpl
  - compiler/mesh-fmt/src/walker.rs
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Replaced the partial-function crash trick with a cooperative worker-loop exit so supervisor restarts no longer depend on `E0012`-emitting non-exhaustive matches
  - Investigated the `meshc fmt --check reference-backend` failure far enough to localize it to `CHILD_SPEC_DEF` formatting in `compiler/mesh-fmt/src/walker.rs`
patterns_established:
  - Record `NoteCrashSoon`, return `false` from the worker tick, and let the permanent supervisor restart the actor instead of forcing a runtime panic
  - Reproduce formatter corruption with `.tmp_worker_probe/fmt_worker/main.mpl` before touching `reference-backend/jobs/worker.mpl` again
observability_surfaces:
  - GET /health
  - GET /jobs/:id
  - jobs table reads
  - compiler/meshc/tests/e2e_reference_backend.rs
  - runtime worker logs
  - .tmp_worker_probe/fmt_worker/main.mpl
duration: partial
verification_result: failed
completed_at: 2026-03-23T01:25:29-04:00
blocker_discovered: false
---

# T01: Re-green the `reference-backend` proof baseline and recovery gates

**Switched the reference backend worker to cooperative supervisor exits and isolated a formatter bug that still blocks the recovery proof gates.**

## What Happened

I resumed from the S05 carry-forward state in `reference-backend/jobs/worker.mpl` and fixed the immediate parser break in `process_claimed_job(...)` by extracting the multi-statement `case` arms into helpers.

From there I reproduced the next build failure and verified that the old crash simulation (`fn force_worker_crash(0) = 0`) was the source of the noisy `E0012 non-exhaustive match on Int` diagnostic. I removed that trick and changed the worker loop to exit cooperatively: `crash_after_claim(...)` now records `NoteCrashSoon(...)` and returns `false`, `process_next_job(...)` propagates that signal, and `job_worker_loop(...)` stops recursing so the permanent supervisor can restart the actor normally.

That got the honest build/test path clean again for `meshc build`, `meshc test`, and the build-only Rust harness.

When I tried to clear the remaining `fmt --check` gate, `meshc fmt` rewrote the supervisor child spec in `reference-backend/jobs/worker.mpl` into parse-invalid text (`childworker do`, then later `childworkerdo`). I restored the worker supervisor block manually and then narrowed the corruption to `compiler/mesh-fmt/src/walker.rs` handling of `CHILD_SPEC_DEF`. I attempted a local walker fix, but it is not finished: formatter output for child specs is still invalid, so `cargo run -p meshc -- fmt --check reference-backend` remains red and I stopped here because of the context-budget warning.

Because the first two ignored recovery tests were run while the worker file was in that formatter-corrupted state, they failed at the inner `meshc build reference-backend` step before exercising runtime recovery. I did not rerun them after restoring `reference-backend/jobs/worker.mpl`, and I did not start the missing `e2e_reference_backend_process_restart_recovers_inflight_job` work.

## Verification

I ran the backend truth gates incrementally while fixing the worker path.

Verified passing after the cooperative-exit change:
- `cargo run -p meshc -- build reference-backend`
- `cargo run -p meshc -- test reference-backend`
- `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_builds -- --nocapture`

Still failing / not yet complete:
- `cargo run -p meshc -- fmt --check reference-backend` still fails because `meshc fmt` corrupts supervisor child specs
- `e2e_reference_backend_worker_crash_recovers_job` and `e2e_reference_backend_worker_restart_is_visible_in_health` failed during their inner build step while `reference-backend/jobs/worker.mpl` was formatter-corrupted; they need to be rerun after the formatter fix
- `e2e_reference_backend_process_restart_recovers_inflight_job` was not implemented or run in this unit

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -p meshc -- build reference-backend` | 0 | ✅ pass | 8.66s |
| 2 | `cargo run -p meshc -- fmt --check reference-backend` | 1 | ❌ fail | 7.01s |
| 3 | `cargo run -p meshc -- test reference-backend` | 0 | ✅ pass | 9.39s |
| 4 | `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_builds -- --nocapture` | 0 | ✅ pass | 8.64s |
| 5 | `set -a; source .env; set +a; cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_worker_crash_recovers_job -- --ignored --nocapture` | 101 | ❌ fail | 6.41s |
| 6 | `set -a; source .env; set +a; cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_worker_restart_is_visible_in_health -- --ignored --nocapture` | 101 | ❌ fail | 6.20s |

## Diagnostics

- The runtime behavior change is in `reference-backend/jobs/worker.mpl`: `crash_after_claim(...)` now signals loop exit instead of triggering a partial-function crash.
- The formatter investigation point is `compiler/mesh-fmt/src/walker.rs`, especially `CHILD_SPEC_DEF` handling.
- Reproduce the formatter bug with `.tmp_worker_probe/fmt_worker/main.mpl`.
- Once formatter output is valid again, rerun the three authoritative runtime proofs from the slice plan and inspect `/health`, `GET /jobs/:id`, and direct `jobs` table rows together.
- The env-backed ignored tests in this worktree require `set -a; source .env; set +a; ...` because non-interactive shell runs do not inherit the repo `.env` automatically.

## Deviations

- I touched `compiler/mesh-fmt/src/walker.rs` and `.gsd/KNOWLEDGE.md`, even though the task plan focused on backend/test files, because the required `fmt --check` gate is currently blocked by a compiler formatter bug rather than by the backend logic itself.

## Known Issues

- `meshc fmt` still produces parse-invalid output for supervisor child specs; `fmt --check` is the current blocker.
- The ignored recovery tests were not rerun after restoring the worker file, so runtime recovery remains unverified in this summary.
- `compiler/meshc/tests/e2e_reference_backend.rs` still does not include the planned `e2e_reference_backend_process_restart_recovers_inflight_job` proof.

## Files Created/Modified

- `reference-backend/jobs/worker.mpl` — fixed the parser break and changed the worker from a partial-function crash trick to cooperative loop exit for supervisor restart.
- `compiler/mesh-fmt/src/walker.rs` — started a targeted `CHILD_SPEC_DEF` formatter repair to unblock `fmt --check`.
- `.gsd/KNOWLEDGE.md` — recorded the supervisor-child formatter corruption and the cooperative-exit crash-simulation constraint.
- `.gsd/milestones/M028/slices/S06/tasks/T01-SUMMARY.md` — recorded this partial execution state and resume notes.
