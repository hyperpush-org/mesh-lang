---
estimated_steps: 4
estimated_files: 8
skills_used:
  - rust-testing
  - rust-best-practices
  - test
---

# T03: Add the packaged route-free proof rail and shared M046 CLI/runtime test support

**Slice:** S04 — Rebuild `cluster-proof/` as tiny packaged proof
**Milestone:** M046

## Description

Prove the rebuilt package end to end with a dedicated M046/S04 route-free rail, extracting only the shared M046 helper layer needed to keep `tiny-cluster/` and `cluster-proof/` on the same CLI/runtime proof surfaces.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Temp-path `meshc build ... --output` flow | Fail before launch if the output parent directory was not created or the temp binary is missing. | N/A | Treat missing temp output metadata as a proof-harness failure instead of silently building in-place and churning tracked binaries. |
| Shared helper extraction from `e2e_m046_s03.rs` | Fail the S03 regression rail and archive the last artifacts instead of forking two near-duplicate helper stacks. | Bound the reused waits and record the last observed CLI state from both packages. | Reject malformed CLI JSON or missing required fields in the shared parser helpers. |
| `meshc cluster status|continuity|diagnostics` package proof queries | Fail on `target_not_connected`, missing runtime-name discovery, or duplicate records rather than probing app routes. | Archive the last CLI JSON and node logs for the failed wait. | Treat malformed JSON and missing `declared_handler_runtime_name` as proof failures. |

## Load Profile

- **Shared resources**: Two long-running node processes, repeated CLI polling, copied proof bundles under `.tmp/m046-s04/...`, and one shared Rust helper module used by S03 and S04.
- **Per-operation cost**: Temp build of the package binary plus repeated `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics` queries until one logical startup record completes.
- **10x breakpoint**: Artifact churn, slow convergence, or duplicate startup records will fail the rail before test-runner CPU becomes interesting.

## Negative Tests

- **Malformed inputs**: Missing temp output parent directories, package drift that reintroduces routes or manifest declarations, or malformed CLI JSON.
- **Error paths**: Missing runtime-name discovery, duplicate startup execution, or absent diagnostics must fail with retained raw artifacts.
- **Boundary conditions**: The package proof must stay route-free, use `Work.execute_declared_work`, and keep tracked binaries untouched by building to a temp output path.

## Steps

1. Extract only the reusable M046 route-free build/spawn/CLI JSON wait helpers from `compiler/meshc/tests/e2e_m046_s03.rs` into `compiler/meshc/tests/support/` and update S03 to import them.
2. Add `compiler/meshc/tests/e2e_m046_s04.rs` that archives the `cluster-proof/` source/package files, pre-creates a temp output parent directory, builds `cluster-proof` to a temp binary, boots two nodes, and discovers the startup record by `declared_handler_runtime_name == "Work.execute_declared_work"` through `meshc cluster continuity` list mode.
3. Assert the packaged route-free proof completes and is inspectable only through `meshc cluster status|continuity|diagnostics`, retaining build logs plus per-node stdout/stderr and CLI JSON under `.tmp/m046-s04/...`.
4. Add `scripts/verify-m046-s04.sh` as the direct slice verifier and rerun the focused S03 route-free rail so helper reuse does not regress `tiny-cluster/`.

## Must-Haves

- [ ] Reused helpers live under `compiler/meshc/tests/support/` and do not create a standalone integration-test target.
- [ ] `compiler/meshc/tests/e2e_m046_s04.rs` builds `cluster-proof` to a temp output path whose parent directory is created up front.
- [ ] The packaged proof uses only Mesh-owned CLI/runtime surfaces; no `/membership`, `/work`, `/status`, or Fly HTTP probe is added back.
- [ ] Failures retain build logs, CLI JSON, and node stdout/stderr under `.tmp/m046-s04/...`, and the S03 route-free rail still passes after helper extraction.

## Verification

- `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_ -- --nocapture && cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture && bash scripts/verify-m046-s04.sh`

## Observability Impact

- Signals added/changed: `.tmp/m046-s04/...` build/status/continuity/diagnostics snapshots, verifier phase/status/current-phase files, and copied node stdout/stderr.
- How a future agent inspects this: rerun the focused S04 test filters or `bash scripts/verify-m046-s04.sh`, then inspect the retained JSON/log bundle.
- Failure state exposed: the last CLI observation, missing runtime-name discovery, temp-output build drift, and per-node process logs remain attached to the failure.

## Inputs

- `compiler/meshc/tests/e2e_m046_s03.rs` — existing route-free local proof rail whose helpers should be the extraction source.
- `scripts/verify-m046-s03.sh` — direct verifier pattern for route-free proof bundles and phase files.
- `compiler/meshc/tests/e2e_m046_s02.rs` — dual-node startup/status reference for runtime-owned CLI discovery.
- `cluster-proof/mesh.toml` — rebuilt package manifest that the new e2e rail must assert on disk.
- `cluster-proof/main.mpl` — route-free bootstrap source that must stay free of HTTP routes.
- `cluster-proof/work.mpl` — source-declared clustered work whose runtime name and trivial arithmetic must stay stable.
- `cluster-proof/tests/work.test.mpl` — package smoke contract that the e2e/verifier rail must build on.

## Expected Output

- `compiler/meshc/tests/support/mod.rs` — shared test-support module entrypoint for M046 route-free helpers.
- `compiler/meshc/tests/support/m046_route_free.rs` — extracted route-free package build/spawn/CLI helper layer.
- `compiler/meshc/tests/e2e_m046_s03.rs` — updated tiny-cluster rail that imports the shared helpers.
- `compiler/meshc/tests/e2e_m046_s04.rs` — packaged route-free proof rail with retained `.tmp/m046-s04/...` artifacts.
- `scripts/verify-m046-s04.sh` — direct packaged-proof verifier that replays build/test/e2e and retains evidence.
