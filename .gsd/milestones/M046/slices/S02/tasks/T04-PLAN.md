---
estimated_steps: 4
estimated_files: 3
skills_used:
  - rust-best-practices
---

# T04: Add the tiny route-free S02 proof rail

**Slice:** S02 — Runtime-owned startup trigger and route-free status contract
**Milestone:** M046

## Description

Add the tiny route-free S02 proof fixture that boots two nodes, auto-runs startup work, and proves completion and diagnostics entirely through `meshc cluster ...` with no `/work`, `/status`, or explicit continuity-submit code in the fixture.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/meshc/tests/e2e_m046_s02.rs` temp-project harness | Archive build/stdout/stderr and fail with the retained last observation instead of collapsing the proof to one panic line. | Bound each wait and record which CLI surface failed to converge. | Treat malformed JSON or missing fields as proof failures with archived raw output. |
| Runtime startup trigger path | Fail the proof on missing continuity records, duplicate startup execution, or missing diagnostics instead of falling back to app routes. | Archive the last `status`, `continuity`, and `diagnostics` observations before failing. | Assert explicit reject reasons instead of swallowing startup-trigger failures. |
| `meshc cluster ...` surfaces | Fail on `target_not_connected` or missing runtime name rather than probing an app route. | Archive the last CLI output and logs for the failed node. | Reject malformed CLI JSON as a tooling proof failure. |

## Load Profile

- **Shared resources**: Dual-node temp binaries, continuity registry, operator query polling, and retained artifact directories.
- **Per-operation cost**: Two long-running processes plus bounded polling of `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics`.
- **10x breakpoint**: Slow convergence, port contention, or diagnostics-buffer truncation will fail the proof before test-runner CPU becomes relevant.

## Negative Tests

- **Malformed inputs**: Fixture sources that reintroduce `HTTP.serve(...)`, `/work`, `/status`, or explicit `Continuity.submit_declared_work(...)`.
- **Error paths**: Startup rejection or convergence timeout must be visible through CLI diagnostics with archived evidence.
- **Boundary conditions**: Simultaneous two-node boot dedupes to one logical startup record, and the declared work body stays trivial (`1 + 1`) so orchestration ownership is obvious.

## Steps

1. Build a temp-project fixture in `compiler/meshc/tests/e2e_m046_s02.rs` that uses source-level `clustered(work)` plus `Node.start_from_env()` only, with trivial `1 + 1` work and no app routes.
2. Run two nodes, wait for runtime-owned membership and authority truth, and discover the startup record entirely through `meshc cluster status|continuity|diagnostics`.
3. Assert the fixture source never calls `Continuity.submit_declared_work(...)`, never adds `/work` or `/status` routes, and never teaches app-owned owner/replica shaping.
4. Close the slice by replaying the retained M044 declared-handler and operator rails alongside the new route-free proof.

## Must-Haves

- [ ] The S02 proof fixture is route-free and contains no app-owned startup or status control flow.
- [ ] The clustered work body stays trivial enough that the remaining complexity is visibly Mesh-owned.
- [ ] The proof uses only `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics` to inspect startup truth.
- [ ] The new proof rail and retained M044 rails together cover R086, R087, R091, R092, and R093.

## Verification

- `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture`

## Observability Impact

- Signals added/changed: Retained CLI snapshots and node logs for startup status, continuity, and diagnostics.
- How a future agent inspects this: Rerun `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture` and open the archived `.tmp/m046-s02/...` bundle named by the failing test.
- Failure state exposed: The last status, continuity, and diagnostics reply plus per-node stdout/stderr stay attached to the proof failure.

## Inputs

- `compiler/meshc/tests/e2e_m046_s02.rs` — new S02 proof rail file that will own the temp-project fixture and archived evidence pattern.
- `compiler/meshc/tests/e2e_m045_s02.rs` — existing clustered-app harness patterns for dual-node bring-up and CLI polling.
- `compiler/meshc/tests/e2e_m044_s03.rs` — existing operator/CLI proof helpers and artifact-retention style.
- `compiler/meshc/src/cluster.rs` — updated CLI surface that the route-free proof must consume directly.
- `compiler/mesh-rt/src/dist/node.rs` — runtime startup trigger contract that the proof will exercise.

## Expected Output

- `compiler/meshc/tests/e2e_m046_s02.rs` — end-to-end route-free startup proof with retained artifacts and CLI-only inspection.
