---
estimated_steps: 13
estimated_files: 4
skills_used: []
---

# T02: Confirmed that the current runtime operator query path would make `meshc cluster` join the target cluster and pollute the membership snapshot it is supposed to inspect.

**Slice:** S03 — Built-in Operator Surfaces & Clustered Scaffold
**Milestone:** M044

## Description

S03 only becomes public once ordinary operators can inspect a live node without `cluster-proof` routes. This task adds read-only `meshc cluster` commands on top of the safe `mesh-rt` operator API and proves them against a live two-node declared-handler runtime.

## Steps

1. Add a dedicated `compiler/meshc/src/cluster.rs` command module and wire read-only subcommands for cluster status (membership + authority), continuity lookup, and recent diagnostics with default human output plus `--json`.
2. Make the CLI use the runtime operator client/query seam directly, authenticated via the standard clustered-app cookie/env contract, and fail closed on unreachable or malformed responses.
3. Add `compiler/meshc/tests/e2e_m044_s03.rs` live-node proofs that launch a real two-node declared-handler app, assert zero-record status truth, assert per-key continuity lookup after declared submit, and assert diagnostics surface degraded or owner-loss transitions after a controlled fault.
4. Keep the scope read-only: no manual promotion, mutation, or app HTTP fallback in the new CLI surface.

## Must-Haves

- [ ] `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics` report runtime-owned truth instead of app-authored HTTP payloads.
- [ ] Every command supports `--json` and returns non-zero on timeout, auth, or malformed-response failures.
- [ ] The named `m044_s03_operator_` rail proves zero-record inspection and post-fault diagnostics against a live two-node runtime.

## Inputs

- `compiler/mesh-rt/src/dist/operator.rs`
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/meshc/src/main.rs`

## Expected Output

- `compiler/meshc/src/main.rs`
- `compiler/meshc/src/cluster.rs`
- `compiler/meshc/Cargo.toml`
- `compiler/meshc/tests/e2e_m044_s03.rs`

## Verification

`cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture`

## Observability Impact

- Signals added/changed: operator CLI error messages include query kind, target, and timeout/decode/auth context; JSON mode surfaces structured runtime snapshots for retained artifacts.
- How a future agent inspects this: rerun the named `m044_s03_operator_` filter or invoke `meshc cluster ... --json` against a retained two-node test target.
- Failure state exposed: unreachable target, auth mismatch, malformed query reply, missing request key, and empty/truncated diagnostics.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Runtime operator client in `compiler/meshc/src/cluster.rs` | Exit non-zero with query kind and target; do not silently print stale data. | Exit non-zero with explicit timeout context. | Fail closed and print a decode/auth error instead of partially rendering JSON. |
| Live target node started by `compiler/meshc/tests/e2e_m044_s03.rs` | Surface connect/auth failure and keep the e2e artifact bundle. | Bound the CLI wait so live tests do not hang. | Treat malformed operator replies as a runtime regression, not a CLI formatting issue. |
| Human/JSON output formatting in `compiler/meshc/src/main.rs` | Preserve the raw structured error and return non-zero. | N/A — local formatting. | Reject inconsistent fields instead of guessing defaults. |

## Load Profile

- **Shared resources**: target node sessions, live test binaries, and operator snapshot serialization.
- **Per-operation cost**: one operator query plus local human/JSON rendering.
- **10x breakpoint**: large continuity or diagnostics payloads make serialization and stdout artifacts expensive before the node runtime itself becomes CPU-bound.

## Negative Tests

- **Malformed inputs**: invalid `--target`, unsupported subcommand flags, and malformed request keys.
- **Error paths**: auth mismatch, unreachable target, and malformed operator replies.
- **Boundary conditions**: zero-record status on a healthy cluster, missing request-key lookup, and bounded diagnostics after a controlled owner-loss or degraded transition.
