---
estimated_steps: 6
estimated_files: 4
skills_used: []
---

# T04: Ship read-only `meshc cluster` commands on the transient operator channel

With the runtime-side transient query path in place, add the public CLI surface on top of that truthful transport instead of the session-joining peer path. This task wires `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics` to the runtime-owned operator API, keeps the scope read-only, and proves live two-node inspection without the CLI becoming a visible peer.

Steps
1. Add a dedicated `compiler/meshc/src/cluster.rs` command module and wire read-only subcommands for cluster status, per-key continuity lookup, and recent diagnostics with default human output plus `--json`.
2. Make the CLI use the transient authenticated operator client/query seam directly, and fail closed on timeout, auth failure, malformed reply, or unreachable target.
3. Add live `compiler/meshc/tests/e2e_m044_s03.rs` proofs that assert zero-record membership/authority truth, per-key continuity lookup after declared work submit, and post-fault diagnostics after controlled owner-loss or degrade events.
4. Prove the key blocker outcome explicitly: querying a live node through `meshc cluster status` must not cause the inspected runtime to report the CLI as a cluster peer.

## Inputs

- `compiler/mesh-rt/src/dist/operator.rs`
- `compiler/meshc/src/main.rs`
- `compiler/meshc/Cargo.toml`

## Expected Output

- `compiler/meshc/src/main.rs`
- `compiler/meshc/src/cluster.rs`
- `compiler/meshc/Cargo.toml`
- `compiler/meshc/tests/e2e_m044_s03.rs`

## Verification

`cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture`

## Observability Impact

- Signals added/changed: `.tmp/m044-s03/verify/phase-report.txt`, `status.txt`, `current-phase.txt`, named test-count logs, and copied operator/scaffold artifact manifests.
- How a future agent inspects this: run the assembled verifier and open `.tmp/m044-s03/verify/` before debugging individual commands.
- Failure state exposed: the failing phase, named test-count drift, missing retained artifacts, docs/public-command mismatch, and scaffold contract leakage.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m044-s03.sh` phase runner | Fail the slice immediately with the failing phase recorded in `.tmp/m044-s03/verify/`. | Kill the stuck command, record timeout, and stop the rail. | Treat missing `running N test` lines or malformed artifact manifests as proof drift. |
| README and VitePress docs surfaces | Keep the previous truthful text until the new command surface is actually verified. | N/A — static files. | Fail the docs/build check rather than publishing stale scaffold/operator instructions. |
| Retained artifact copy logic | Stop with a manifest error instead of pretending operator or scaffold proof exists. | N/A — local file copies. | Reject incomplete bundles and name the missing artifact. |

## Load Profile

- **Shared resources**: slice verifier temp artifacts, named test logs, and the VitePress build cache.
- **Per-operation cost**: one full S02 replay, the S03 named filters, artifact copying, and one docs build.
- **10x breakpoint**: retained artifact growth and docs build time dominate before runtime operator cost matters.

## Negative Tests

- **Malformed inputs**: `running 0 tests`, missing copied artifact manifests, scaffold outputs still containing `CLUSTER_PROOF_*`, and docs that mention auto-promotion or a finished `cluster-proof` rewrite.
- **Error paths**: S02 replay failure, named S03 test failure, docs build failure, and missing phase/status files.
- **Boundary conditions**: green behavior with zero-record status truth, green scaffold smoke, and retained operator/scaffold bundles under `.tmp/m044-s03/verify/`.
