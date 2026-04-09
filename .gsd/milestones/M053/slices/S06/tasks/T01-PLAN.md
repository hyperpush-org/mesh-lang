---
estimated_steps: 4
estimated_files: 3
skills_used:
  - debug-like-expert
  - rust-best-practices
  - test
---

# T01: Restore the runtime-owned startup dispatch window override

- Why: Hosted red currently depends on a runtime bug: the staged Postgres starter harness exports `MESH_STARTUP_WORK_DELAY_MS`, but `startup_dispatch_window_ms(...)` ignores it and closes the promotable window at 2500ms on hosted Ubuntu.
- Do: Restore the runtime-owned env override inside `compiler/mesh-rt/src/dist/node.rs`, keep 2500ms as the absent-env default, preserve zero-delay behavior for non-startup or replica-free requests, and extend the nearby runtime tests instead of pushing the fix back into starter source or wrapper scripts.
- Done when: `startup_dispatch_window_ms(...)` honors a positive `MESH_STARTUP_WORK_DELAY_MS` for runtime-owned clustered startup work, invalid or missing env falls back to the safe default, and targeted runtime tests cover override plus default behavior.

## Steps

1. Add a small helper next to `startup_dispatch_window_ms(...)` that reads/parses `MESH_STARTUP_WORK_DELAY_MS` once and returns either the configured positive delay or the existing `STARTUP_CLUSTERED_PENDING_WINDOW_MS` fallback.
2. Extend the nearby runtime tests in `compiler/mesh-rt/src/dist/node.rs` to prove override behavior, default behavior when the env is absent, and zero-delay behavior for non-startup requests or `required_replica_count == 0`.
3. Keep the change runtime-owned only: do not add app-owned `Timer.sleep(...)`, README knobs, or wrapper-script-only workarounds.

## Must-Haves

- [ ] A positive `MESH_STARTUP_WORK_DELAY_MS` changes the runtime-owned clustered startup window without affecting non-startup requests.
- [ ] Missing, zero, negative, or malformed env values fall back to the safe default instead of widening the contract or crashing startup work.
- [ ] The proof lives in runtime tests near `startup_dispatch_window_ms(...)`, so later agents can diagnose regressions without reopening starter code or hosted workflows.

## Inputs

- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs`
- `compiler/meshc/tests/e2e_m053_s02.rs`

## Expected Output

- `compiler/mesh-rt/src/dist/node.rs`

## Verification

cargo test -p mesh-rt startup_work_dispatch_window_ -- --nocapture

## Observability Impact

- Signals added/changed: `startup_dispatch_window` diagnostics and unit assertions now distinguish the configured `pending_window_ms` from the default 2500ms fallback.
- How a future agent inspects this: run `cargo test -p mesh-rt startup_work_dispatch_window_ -- --nocapture` and compare the nearby startup-window tests with later staged failover logs.
- Failure state exposed: whether runtime-owned startup requests used the override, the default fallback, or an unintended zero-delay path.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `MESH_STARTUP_WORK_DELAY_MS` parsing in `compiler/mesh-rt/src/dist/node.rs` | Fall back to `STARTUP_CLUSTERED_PENDING_WINDOW_MS` and keep startup work runtime-owned instead of panicking or widening app code. | N/A — env parsing is synchronous; a hung unit test means surrounding runtime drift, not a retryable timeout. | Treat non-integer, zero, or negative values as invalid and fall back to the default window. |
| `startup_dispatch_window_ms(...)` gating rules | Preserve zero-delay behavior for non-`startup::...` requests and `required_replica_count == 0`. | Stop on any hanging runtime test and inspect the nearby startup tests before changing the contract again. | Fail tests closed if non-startup requests receive a delay or clustered startup requests ignore the configured override. |

## Load Profile

- **Shared resources**: process env and the runtime-owned startup timer in `compiler/mesh-rt/src/dist/node.rs`.
- **Per-operation cost**: one env lookup/parse plus one timer decision per startup submission; trivial relative to cluster I/O.
- **10x breakpoint**: silently honoring bad values or repeatedly re-parsing the env would distort the pending window and make hosted failover timing non-deterministic across runners.

## Negative Tests

- **Malformed inputs**: unset env, non-integer text, zero, and negative numbers.
- **Error paths**: env set on a non-startup request, env set with `required_replica_count == 0`, and env absent on a clustered startup request.
- **Boundary conditions**: positive override `1`, large override `20000`, and the default 2500ms fallback when the env is missing.
