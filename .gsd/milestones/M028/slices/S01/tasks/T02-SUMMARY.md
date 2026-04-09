---
id: T02
parent: S01
milestone: M028
provides:
  - stable non-empty-DATABASE_URL startup for reference-backend plus a compiler-facing runtime-start regression proof
key_files:
  - reference-backend/main.mpl
  - reference-backend/config.mpl
  - compiler/meshc/tests/e2e_reference_backend.rs
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Keep `config.mpl` as the env-contract/messages surface and keep runtime env validation local to `reference-backend/main.mpl`.
  - Use `Env.get` plus `Env.get_int` for startup validation and rely on the ignored Rust e2e test as the mechanical `/health` proof for the non-empty `DATABASE_URL` path.
patterns_established:
  - Mesh startup validation is safest when missing/invalid env handling stays in the entrypoint and emits explicit log lines before touching the next runtime boundary.
observability_surfaces:
  - `GET /health`
  - `cargo test -p meshc e2e_reference_backend_runtime_starts --test e2e_reference_backend -- --ignored --nocapture`
  - `env -u DATABASE_URL PORT=18080 JOB_POLL_MS=500 ./reference-backend/reference-backend 2>&1 | rg "DATABASE_URL"`
  - `env DATABASE_URL=x PORT=18080 JOB_POLL_MS=1000 ./reference-backend/reference-backend`
duration: 2h
verification_result: passed
completed_at: 2026-03-23
blocker_discovered: false
---

# T02: Repair the non-empty `DATABASE_URL` startup path and land regression proof

**Hardened `reference-backend` startup against non-empty `DATABASE_URL` crashes and added a Postgres-backed `/health` regression test.**

## What Happened

I started by reproducing the crash on the smallest DB-backed startup path and rechecking the lldb backtrace against the rebuilt binary. The first hypothesis was that `String.to_int` in the startup path was still the direct trigger, so I replaced that parsing path with the built-in env helpers and reran the repro instead of layering on more code.

The stable fix ended up being narrower and more conservative than the first draft: `reference-backend/main.mpl` now keeps env validation local, uses `Env.get` plus `Env.get_int` to validate `PORT` and `JOB_POLL_MS`, preserves the explicit `DATABASE_URL` missing-config failure, and logs distinct config-load, DB-connect, pool-ready, and HTTP-bind messages without logging the connection string.

I left `reference-backend/config.mpl` as the startup-contract/messages surface instead of moving runtime parsing into it. That matches the safer working pattern proven during this task and avoids reintroducing the crash-prone shape that T01 had already flagged.

For the live proof target, I started the rebuilt backend against the worktree’s local `.env`, confirmed it bound `:18080`, and hit `GET /health` successfully against a real Postgres-backed startup path. The `reference-backend/api/health.mpl` handler itself did not need code changes; the task was to get startup to it reliably.

Finally, I extended `compiler/meshc/tests/e2e_reference_backend.rs` with an ignored `e2e_reference_backend_runtime_starts` test that builds `reference-backend`, launches it with `DATABASE_URL`, probes `/health`, and asserts the key startup log lines are present while also asserting the logs do not echo the database URL. I kept the existing ignored Postgres smoke target and routed it through the same helper so downstream slice work can keep building from the same runtime proof surface.

I also replaced the stale crash note in `.gsd/KNOWLEDGE.md` with the current rule that future agents should reuse.

## Verification

I ran the task’s build-only verification, the explicit missing-env verification, and the ignored Postgres-backed runtime-start regression test. All three passed once the verification commands were run sequentially.

I also ran the direct non-empty `DATABASE_URL` startup path with an intentionally invalid URL and confirmed it now logs a clean PostgreSQL connect failure instead of crashing, which directly proves the original segfault is gone.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo build -p mesh-rt && cargo test -p meshc e2e_reference_backend_builds --test e2e_reference_backend -- --nocapture` | 0 | ✅ pass | 6.42s |
| 2 | `env -u DATABASE_URL PORT=18080 JOB_POLL_MS=500 ./reference-backend/reference-backend 2>&1 | rg "DATABASE_URL"` | 0 | ✅ pass | 0.71s |
| 3 | `set -a && source .env && set +a && DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc e2e_reference_backend_runtime_starts --test e2e_reference_backend -- --ignored --nocapture` | 0 | ✅ pass | 6.58s |
| 4 | `env DATABASE_URL=x PORT=18080 JOB_POLL_MS=1000 ./reference-backend/reference-backend` | 0 | ✅ pass | 0.03s |

## Diagnostics

- Live runtime proof: start `reference-backend` with the local `.env`, then hit `GET /health`.
- Missing-config proof: `env -u DATABASE_URL PORT=18080 JOB_POLL_MS=500 ./reference-backend/reference-backend 2>&1 | rg "DATABASE_URL"`.
- Non-empty startup regression proof: `cargo test -p meshc e2e_reference_backend_runtime_starts --test e2e_reference_backend -- --ignored --nocapture` with `DATABASE_URL` set.
- Graceful invalid-URL proof: `env DATABASE_URL=x PORT=18080 JOB_POLL_MS=1000 ./reference-backend/reference-backend`.
- Expected startup signals now include config load, PostgreSQL connect attempt, pool ready, and HTTP bind, and the Rust e2e test asserts those lines directly.

## Deviations

- I did not modify `reference-backend/api/health.mpl`; the existing handler already matched the slice contract, so T02 verified it live instead of changing it.
- My first verification pass ran Cargo-based checks in parallel and produced a transient false negative on the build-only test. I reran the verification sequentially and recorded the stable results above.

## Known Issues

- A bad non-empty `DATABASE_URL` now fails visibly and without a segfault, but it still exits after logging the connect failure rather than surfacing a distinct nonzero process status. That is not a T02 blocker, but later hardening may want to tighten startup failure semantics.

## Files Created/Modified

- `reference-backend/main.mpl` — replaced the crash-prone startup validation path with local `Env.get`/`Env.get_int` handling and explicit startup logging.
- `reference-backend/config.mpl` — kept the startup contract surface focused on env keys and user-facing config error strings.
- `compiler/meshc/tests/e2e_reference_backend.rs` — added the ignored `e2e_reference_backend_runtime_starts` regression test and shared runtime-start helper assertions.
- `.gsd/KNOWLEDGE.md` — replaced the stale crash note with the current startup-validation rule and regression command.
