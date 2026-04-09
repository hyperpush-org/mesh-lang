---
estimated_steps: 4
estimated_files: 5
skills_used:
  - rust-best-practices
  - rust-testing
---

# T02: Add a real-package startup/status proof rail in `e2e_m046_s03.rs`

**Slice:** S03 — `tiny-cluster/` local no-HTTP proof
**Milestone:** M046

## Description

Build the real `tiny-cluster/` package through a new Rust e2e rail that proves the source/manifest contract, boots two nodes without HTTP routes, and discovers the deterministic startup record entirely through runtime-owned CLI surfaces.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Real `tiny-cluster/` package build path | Fail before launching nodes and archive the build output instead of silently swapping back to a temp fixture. | N/A | Treat missing package files or malformed source assertions as a contract failure. |
| `compiler/meshc/tests/e2e_m046_s02.rs` helper port | Archive build/stdout/stderr and fail with the retained last observation instead of collapsing the proof to one panic line. | Bound each wait and record which CLI surface failed to converge. | Treat malformed JSON or missing fields as proof failures with archived raw output. |
| `meshc cluster status|continuity|diagnostics` queries | Fail on `target_not_connected` or missing runtime name rather than probing an app route. | Archive the last CLI output and logs for the failed node. | Reject malformed CLI JSON as a tooling proof failure. |

## Load Profile

- **Shared resources**: Two long-running local node processes, one continuity registry, bounded CLI polling, and retained artifact directories under `.tmp/m046-s03/...`.
- **Per-operation cost**: Two process boots plus repeated `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics` queries until one logical startup record completes.
- **10x breakpoint**: Slow convergence, duplicate startup records, or artifact churn will fail the rail before test-runner CPU becomes interesting.

## Negative Tests

- **Malformed inputs**: Package drift that reintroduces `[cluster]`, `HTTP.serve(...)`, `/work`, `/status`, `/health`, or explicit continuity calls.
- **Error paths**: Missing runtime-name discovery, duplicate startup execution, or missing diagnostics must fail with retained CLI evidence.
- **Boundary conditions**: Two-node simultaneous boot still dedupes to one logical startup record, and the work body remains trivial (`1 + 1`) even though the runtime owns the orchestration.

## Steps

1. Add `compiler/meshc/tests/e2e_m046_s03.rs` that builds the real repo package at `tiny-cluster/` (not temp source strings) and copies the core package files into `.tmp/m046-s03/...` artifacts for contract debugging.
2. Add file-content assertions that fail if `tiny-cluster/` reintroduces `[cluster]`, `HTTP.serve(...)`, `/work`, `/status`, `/health`, or explicit continuity calls, and confirm the single `Node.start_from_env()` and visible `1 + 1` work body.
3. Reuse the S02 dual-node route-free harness pattern to boot two nodes from the built `tiny-cluster` binary and discover the startup record by `declared_handler_runtime_name == "Work.execute_declared_work"` via list mode on both nodes.
4. Assert one logical record completes on both nodes, completion stays Mesh-owned, and diagnostics show startup trigger/completion without any app-owned control flow.

## Must-Haves

- [ ] The rail builds and runs the real repo package, not a copied temp fixture.
- [ ] `meshc cluster continuity` list/single-record output is sufficient to discover and inspect the startup work for `tiny-cluster/`.
- [ ] Two nodes converge on one logical startup record and complete without app-owned submit/status routes.
- [ ] Failures retain build logs, CLI JSON, and node stdout/stderr under `.tmp/m046-s03/...`.

## Verification

- `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_package_ -- --nocapture`
- `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_startup_ -- --nocapture`

## Observability Impact

- Signals added/changed: `.tmp/m046-s03/...` build/status/continuity/diagnostics snapshots for the real `tiny-cluster` package.
- How a future agent inspects this: rerun the focused test filters and inspect the archived JSON/logs in the failing bundle.
- Failure state exposed: the last membership, continuity, and diagnostics observation plus per-node stdout/stderr stay attached to the failure.

## Inputs

- `tiny-cluster/mesh.toml` — real package manifest that must stay source-first and declaration-free.
- `tiny-cluster/main.mpl` — route-free bootstrap entrypoint whose content must stay minimal.
- `tiny-cluster/work.mpl` — declared work source whose runtime name and trivial arithmetic must stay intact.
- `compiler/meshc/tests/e2e_m046_s02.rs` — reusable dual-node route-free startup harness and artifact-retention patterns.
- `compiler/meshc/src/cluster.rs` — CLI JSON surface that must be sufficient for runtime-name discovery and inspection.

## Expected Output

- `compiler/meshc/tests/e2e_m046_s03.rs` — real-package startup contract/build/completion rails with retained artifacts.
