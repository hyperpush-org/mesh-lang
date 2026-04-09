---
id: T01
parent: S01
milestone: M028
provides:
  - reference-backend package scaffold plus an initial compiler-facing e2e target
key_files:
  - reference-backend/mesh.toml
  - reference-backend/main.mpl
  - reference-backend/config.mpl
  - reference-backend/api/router.mpl
  - reference-backend/api/health.mpl
  - compiler/meshc/tests/e2e_reference_backend.rs
key_decisions:
  - Seed the `e2e_reference_backend` Rust test target in T01 because slice-level verification already depends on it.
  - Keep `config.mpl` as the centralized startup-contract surface, but move integer parsing back into `main.mpl` after the imported parsing path proved crash-prone at runtime.
patterns_established:
  - Top-level Mesh package layout with modular API handler/router files instead of a single-file demo
  - Compiler e2e coverage for a directory-based Mesh package
observability_surfaces:
  - `cargo run -p meshc -- build reference-backend`
  - `compiler/meshc/tests/e2e_reference_backend.rs`
  - missing-env startup error from `./reference-backend/reference-backend`
  - lldb backtrace for the non-empty `DATABASE_URL` crash
  - .gsd/KNOWLEDGE.md
duration: over-budget recovery
verification_result: partial
completed_at: 2026-03-23
blocker_discovered: true
---

# T01: Scaffold the `reference-backend/` package and startup contract

**Scaffolded the `reference-backend/` package and compiler-facing build proof, then isolated a runtime segfault on the non-empty `DATABASE_URL` startup path.**

## What Happened

I created the new top-level `reference-backend/` Mesh package with its own `mesh.toml`, `main.mpl`, `config.mpl`, `api/router.mpl`, and `api/health.mpl`, and wired a real `GET /health` route through package-local modules instead of defining routes inline.

I also applied the pre-flight fix to the slice plan by adding an explicit missing-config failure-path verification command, and I seeded `compiler/meshc/tests/e2e_reference_backend.rs` in this first task because the slice verification contract already names that test target.

Build-side proof is now real: `meshc build reference-backend` succeeds, and the dedicated Rust test target passes for the build-only case.

To finish the runtime verification, I provisioned a disposable local Postgres in Docker and wrote a local `.env` for the worktree. That exposed a deeper blocker: any startup path that reaches a non-empty `DATABASE_URL` crashes the compiled binary with `EXC_BAD_ACCESS` before HTTP bind. A direct lldb run points at `parse_required_positive_int` in the generated binary, which means this is no longer a simple package-scaffolding issue. Because S01 depends on a live Postgres-backed runtime, I marked `blocker_discovered: true`.

## Verification

I verified the package compiles as a directory-based Mesh project and that the new compiler-facing test target is wired correctly. I also verified the explicit missing-env failure path now reports `DATABASE_URL` clearly.

I attempted the DB-backed smoke path twice: once via the ignored Rust test after provisioning a local Docker Postgres, and once by launching the built binary directly. Both failed because the runtime segfaults when `DATABASE_URL` is non-empty, so `/health` never binds.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -p meshc -- build reference-backend` | 0 | ✅ pass | 5.7s |
| 2 | `cargo build -p mesh-rt && cargo test -p meshc e2e_reference_backend_builds --test e2e_reference_backend -- --nocapture` | 0 | ✅ pass | 5.8s |
| 3 | `env -u DATABASE_URL PORT=18080 JOB_POLL_MS=500 ./reference-backend/reference-backend 2>&1 | rg "DATABASE_URL"` | 0 | ✅ pass | n/a |
| 4 | `set -a && source .env && set +a && cargo test -p meshc e2e_reference_backend_postgres_smoke --test e2e_reference_backend -- --ignored --nocapture` | 101 | ❌ fail | 10.1s |
| 5 | `DATABASE_URL=x PORT=18080 JOB_POLL_MS=1000 ./reference-backend/reference-backend` | 139 | ❌ fail | n/a |

## Diagnostics

- Build inspection: `cargo run -p meshc -- build reference-backend`
- Missing-env inspection: `env -u DATABASE_URL PORT=18080 JOB_POLL_MS=500 ./reference-backend/reference-backend`
- Crash inspection: `env DATABASE_URL=x PORT=18080 JOB_POLL_MS=1000 lldb --batch -o run -o bt -- ./reference-backend/reference-backend`
- The lldb backtrace currently stops in `parse_required_positive_int` inside the generated `reference-backend` binary.
- A disposable local Docker Postgres container was created during verification to test the DB-backed startup path.

## Deviations

- I created `compiler/meshc/tests/e2e_reference_backend.rs` in T01 instead of waiting for T04 because the slice verification contract already depended on that test target existing.
- `config.mpl` ended this task as the centralized startup-contract surface rather than the full runtime parser implementation because the imported parse path was part of the crash investigation. The actual positive-int parsing currently lives in `main.mpl`.

## Known Issues

- `reference-backend/reference-backend` segfaults when startup reaches a non-empty `DATABASE_URL` path, so the task’s `/health` runtime proof is still blocked.
- Because of that crash, downstream DB-backed slice work (`migrate`, jobs persistence, worker processing, smoke script) is not currently safe to continue without addressing the runtime issue first.
- `reference-backend/scripts/smoke.sh` is still absent; that was planned for T03 and was not part of this partial T01 recovery.

## Files Created/Modified

- `reference-backend/mesh.toml` — added the new top-level Mesh package manifest for the reference backend
- `reference-backend/main.mpl` — added the env-driven startup entrypoint, pool-open sequence, and modular router wiring
- `reference-backend/config.mpl` — added the centralized startup-contract surface for `DATABASE_URL`, `PORT`, and `JOB_POLL_MS`
- `reference-backend/api/router.mpl` — added package-local router assembly for `GET /health`
- `reference-backend/api/health.mpl` — added the initial health handler
- `compiler/meshc/tests/e2e_reference_backend.rs` — added initial build-only and ignored Postgres smoke coverage for the new package
- `.gsd/milestones/M028/slices/S01/S01-PLAN.md` — added the missing-config verification command and marked T01 complete for recovery bookkeeping
- `.gsd/KNOWLEDGE.md` — recorded the non-obvious runtime crash site to avoid repeating the same investigation
