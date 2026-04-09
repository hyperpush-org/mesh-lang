---
estimated_steps: 3
estimated_files: 4
skills_used:
  - test
  - debug-like-expert
  - rust-best-practices
---

# T01: Add the scaffold-first failover e2e with runtime-only truth and retained artifacts

Finish the core R078 proof on the scaffold-first example by creating a dedicated destructive failover e2e that keeps cluster truth runtime-owned. The harness should use the scaffolded binary, not `cluster-proof`, and it should confirm actual placement from `meshc cluster continuity --json` before taking destructive action so local placement prediction cannot make the rail go green for the wrong reason.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Two-node scaffold runtime plus `meshc cluster status|continuity|diagnostics` surfaces | Stop with retained HTTP, CLI, and node-log artifacts; do not infer failover success from startup logs alone. | Bound membership/pending-state/promotion/rejoin polling and fail with the last observed payload. | Treat malformed CLI JSON as a hard proof failure, not a retryable warning. |
| Pending mirrored owner-loss timing window in the scaffold work path | Keep the app surface unchanged and fail with retained pre-kill artifacts showing whether records completed too early. | Use bounded candidate search and, if needed, bounded batching before declaring the harness nondeterministic. | Treat missing or contradictory pre-kill record fields as a proof failure rather than assuming the right placement. |

## Load Profile

- **Shared resources**: temporary scaffold dirs, local ports, spawned node processes, CLI subprocesses, and `.tmp/m045-s03` artifact roots.
- **Per-operation cost**: one scaffold init/build, two node boots, repeated CLI polls, one destructive kill/rejoin cycle, and one retained artifact bundle.
- **10x breakpoint**: process cleanup, pending-window timing, and artifact churn fail before runtime throughput; the harness must capture enough state to explain which stage drifted.

## Negative Tests

- **Malformed inputs**: request keys that never produce the required pre-kill owner/replica shape and malformed JSON from runtime CLI surfaces.
- **Error paths**: primary dies after the record already completed, standby never promotes, automatic recovery never rolls the attempt, or the stale primary executes after rejoin.
- **Boundary conditions**: `replica_status=preparing` vs `mirrored` before kill, single-submit vs bounded candidate batching, and same-identity rejoin after standby promotion.

## Steps

1. Create `compiler/meshc/tests/e2e_m045_s03.rs` with scaffold init/build/spawn helpers adapted from `compiler/meshc/tests/e2e_m045_s02.rs`, artifact retention under `.tmp/m045-s03/...`, and a two-node cluster setup that runs one scaffold binary as primary and one as standby.
2. Drive request-key candidates until runtime CLI truth shows a pre-kill record with `owner_node=primary`, `replica_node=standby`, `phase=submitted`, `result=pending`, and `replica_status` still `mirrored` or `preparing`; if a single submit races completion, use bounded candidate batching while keeping the app surface unchanged.
3. Kill the primary, prove standby promotion/recovery and same-identity rejoin fencing through `meshc cluster status|continuity|diagnostics --json`, and retain pre-kill/post-kill/post-rejoin JSON plus node logs and `scenario-meta.json`.

## Must-Haves

- [ ] The new scaffold failover rail proves the owner-loss shape before the destructive step instead of trusting a local heuristic.
- [ ] Promotion, recovery, and fenced rejoin are all asserted from runtime-owned CLI truth plus node logs.
- [ ] The retained bundle is sufficient to debug whether drift happened before kill, during promotion/recovery, or during rejoin.

## Verification

- `cargo test -p meshc --test e2e_m045_s03 m045_s03_failover_ -- --nocapture`

## Inputs

- `compiler/meshc/tests/e2e_m045_s02.rs`
- `compiler/meshc/tests/e2e_m044_s04.rs`
- `compiler/meshc/src/cluster.rs`
- `compiler/mesh-pkg/src/scaffold.rs`

## Expected Output

- `compiler/meshc/tests/e2e_m045_s03.rs`

## Verification

cargo test -p meshc --test e2e_m045_s03 m045_s03_failover_ -- --nocapture

## Observability Impact

- Signals added/changed: retained pre-kill/post-kill/post-rejoin CLI JSON, `scenario-meta.json`, and per-node stdout/stderr for the scaffold failover rail.
- How a future agent inspects this: rerun `cargo test -p meshc --test e2e_m045_s03 m045_s03_failover_ -- --nocapture` and inspect `.tmp/m045-s03/...`.
- Failure state exposed: pending-window loss, promotion/recovery drift, malformed CLI truth, and stale-primary execution after rejoin are separated in artifacts.
