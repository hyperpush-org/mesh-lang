---
estimated_steps: 4
estimated_files: 6
skills_used:
  - rust-best-practices
  - test
---

# T02: Convert scaffold-era regression rails to the route-free CLI-only contract

Bring the still-live scaffold regression suite onto the same equal-surface story as the scaffold and proof packages so old routeful tests stop blocking the slice and future drift fails closed in one place.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/meshc/tests/support/m046_route_free.rs` shared harness | Fail with retained build/log/CLI artifacts instead of forking another scaffold-specific runtime harness. | Bound waits for startup, continuity discovery, and diagnostics convergence. | Reject malformed `meshc cluster` JSON and missing runtime-name discovery as proof failures. |
| Historical scaffold rails (`e2e_m044_s03.rs`, `e2e_m045_s01.rs`, `e2e_m045_s02.rs`, `e2e_m045_s03.rs`) | Fail fast if they still require `/health`, `/work`, or app-owned submit/status helpers. | Stop on the reused harness timeout instead of retrying through deleted routes. | Treat stale routeful assertions as contract drift rather than carrying compatibility shims forward. |
| New `e2e_m046_s05.rs` equal-surface rail | Fail if the generated scaffold cannot be built, booted, and inspected through the same CLI-only surfaces as the proof packages. | Retain the last status/continuity/diagnostics snapshots and node logs in `.tmp/m046-s05/...`. | Treat malformed retained artifacts or missing continuity list/record linkage as proof failure. |

## Load Profile

- **Shared resources**: temp scaffold project generation, two runtime processes, repeated `meshc cluster` polling, and copied proof bundles under `.tmp/m046-s05/...`.
- **Per-operation cost**: one scaffold generation/build plus repeated `meshc cluster status`, continuity list, continuity record, and diagnostics queries until the startup record is discovered and completed.
- **10x breakpoint**: artifact churn, slow startup convergence, or duplicate startup records will fail the rail long before CPU or memory matter.

## Negative Tests

- **Malformed inputs**: stale routeful scaffold fixtures, missing temp output parents, malformed CLI JSON, or missing `declared_handler_runtime_name` / request-key discovery.
- **Error paths**: historical rails must fail on `/health`, `/work`, `[cluster]`, `Continuity.submit_declared_work(...)`, or request-key-only continuity assumptions instead of masking drift.
- **Boundary conditions**: the scaffold runtime proof must start from continuity list mode, then inspect the discovered record by request key, without inventing a second control plane.

## Steps

1. Extend the shared route-free harness only where needed so a generated scaffold project can be built to a temp output path, booted on two nodes, and inspected through `meshc cluster status|continuity|diagnostics` the same way `tiny-cluster/` and `cluster-proof/` are.
2. Add `compiler/meshc/tests/e2e_m046_s05.rs` to generate a temp clustered scaffold, assert on-disk parity against the proof packages, and prove startup inspection through continuity list mode followed by single-record inspection.
3. Rewrite or narrow `compiler/meshc/tests/e2e_m044_s03.rs`, `compiler/meshc/tests/e2e_m045_s01.rs`, `compiler/meshc/tests/e2e_m045_s02.rs`, and `compiler/meshc/tests/e2e_m045_s03.rs` so they no longer depend on deleted HTTP submit/health behavior and instead either assert the route-free contract directly or delegate to the new equal-surface rail.
4. Keep failures diagnosable by retaining generated scaffold source, build logs, status/continuity/diagnostics JSON, and per-node stdout/stderr in the S05 artifact bundle.

## Must-Haves

- [ ] `compiler/meshc/tests/e2e_m046_s05.rs` exists and proves generated scaffold parity plus CLI-only runtime inspection.
- [ ] Historical scaffold rails no longer require `/health`, `/work`, `Continuity.submit_declared_work(...)`, or `Timer.sleep(...)` in generated code.
- [ ] The shared route-free harness remains the single runtime/CLI proof seam instead of spawning a second bespoke scaffold harness.
- [ ] Failures retain enough `.tmp/m046-s05/...` evidence to localize whether the drift is in generation, startup, continuity discovery, or diagnostics.

## Done When

- [ ] The scaffold regression suite fails closed on routeful drift and passes against the new scaffold contract.
- [ ] The new S05 equal-surface rail proves the scaffold on the same CLI-only surfaces as `tiny-cluster/` and `cluster-proof`.

## Inputs

- `compiler/meshc/tests/support/m046_route_free.rs`
- `compiler/meshc/tests/e2e_m046_s03.rs`
- `compiler/meshc/tests/e2e_m046_s04.rs`
- `compiler/meshc/tests/e2e_m044_s03.rs`
- `compiler/meshc/tests/e2e_m045_s01.rs`
- `compiler/meshc/tests/e2e_m045_s02.rs`
- `compiler/meshc/tests/e2e_m045_s03.rs`
- `compiler/mesh-pkg/src/scaffold.rs`

## Expected Output

- `compiler/meshc/tests/support/m046_route_free.rs`
- `compiler/meshc/tests/e2e_m044_s03.rs`
- `compiler/meshc/tests/e2e_m045_s01.rs`
- `compiler/meshc/tests/e2e_m045_s02.rs`
- `compiler/meshc/tests/e2e_m045_s03.rs`
- `compiler/meshc/tests/e2e_m046_s05.rs`

## Verification

cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture && cargo test -p meshc --test e2e_m045_s01 m045_s01_ -- --nocapture && cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture && cargo test -p meshc --test e2e_m045_s03 m045_s03_ -- --nocapture && cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture

## Observability Impact

- Signals added/changed: `.tmp/m046-s05/...` scaffold build logs, retained generated source, status/continuity/diagnostics snapshots, and node stdout/stderr.
- How a future agent inspects this: rerun the focused historical/new scaffold test filters and inspect the retained S05 bundle.
- Failure state exposed: the last successful generated scaffold state, the last CLI observation, and the exact phase that drifted stay on disk.
