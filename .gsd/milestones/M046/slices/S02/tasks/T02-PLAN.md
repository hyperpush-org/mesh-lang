---
estimated_steps: 4
estimated_files: 5
skills_used:
  - rust-best-practices
---

# T02: Implement runtime-owned startup submission, convergence waiting, and route-free keepalive

**Slice:** S02 — Runtime-owned startup trigger and route-free status contract
**Milestone:** M046

## Description

Make the runtime consume startup-work registrations autonomously: register stable startup identities, wait boundedly for peer convergence, submit through `submit_declared_work(...)`, and keep cluster-mode route-free binaries alive long enough to inspect without any app glue.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/mesh-rt/src/dist/node.rs` startup registry and trigger path | Reject the startup work with an explicit diagnostic instead of panicking or silently skipping it. | Record a convergence-timeout diagnostic and fail closed if the replica requirement is still unmet. | Never submit startup work with a blank runtime name, blank request key, or missing handler metadata. |
| `compiler/mesh-rt/src/dist/continuity.rs` submit/state machine | Keep the continuity record rejected with an explicit reason instead of inventing success. | Preserve pending-or-rejected truth in continuity/diagnostics instead of dropping the record. | Reject duplicate or stale startup identities explicitly. |
| `compiler/mesh-rt/src/actor/mod.rs` scheduler lifetime | Keep cluster-mode route-free apps alive via runtime-owned work/keepalive actors instead of exiting immediately after `mesh_main` returns. | N/A | Do not strand the scheduler with zero active runtime-owned actors before tooling can inspect the node. |

## Load Profile

- **Shared resources**: Continuity registry, node session map, operator diagnostics buffer, and scheduler active-process count.
- **Per-operation cost**: One bounded membership polling loop and one continuity submit per startup work item per process boot.
- **10x breakpoint**: Slow peer convergence or many startup work items will show up first as pending actor/diagnostic churn, not raw CPU saturation.

## Negative Tests

- **Malformed inputs**: Blank runtime names, duplicate startup registrations, missing registered handlers, and standalone boot with no cluster cookie.
- **Error paths**: Convergence timeout, `replica_required_unavailable`, and remote spawn rejection after reconnect.
- **Boundary conditions**: Standalone mode, single-node cluster mode, two-node cluster mode with late peer arrival, and simultaneous boot on both nodes using the same startup identity.

## Steps

1. Add a runtime registry for startup work keyed by declared runtime registration name and derive a deterministic startup request key from that runtime name.
2. Spawn runtime-owned startup actors after registration that wait boundedly for peer convergence, derive required replicas from observed membership, and submit through the existing declared-work continuity path.
3. Add runtime-owned diagnostics for startup registration, trigger, timeout, rejection, and completion/fencing, and keep cluster-mode route-free binaries alive without any app `HTTP.serve(...)` or `Continuity.submit_declared_work(...)` glue.
4. Add focused runtime tests for registration dedupe, deterministic request identity, bounded timeout behavior, and keepalive-trigger interaction.

## Must-Haves

- [ ] Startup work uses a deterministic runtime-owned identity so multiple boots converge on one logical startup run.
- [ ] Startup submission waits boundedly for peers and fails closed with diagnostics when replication cannot be satisfied.
- [ ] Route-free cluster-mode processes stay alive long enough for `meshc cluster ...` inspection without app keepalive glue.
- [ ] No app-owned `Continuity.submit_declared_work(...)` or `Continuity.mark_completed(...)` call is required for startup execution.

## Verification

- `cargo test -p mesh-rt startup_work_ -- --nocapture`

## Observability Impact

- Signals added/changed: Startup-trigger, convergence-timeout, startup-rejected, and startup-completed diagnostics plus route-free keepalive state.
- How a future agent inspects this: `meshc cluster diagnostics --json`, continuity records, and the focused `cargo test -p mesh-rt startup_work_ -- --nocapture` rail.
- Failure state exposed: Runtime name, request key, attempt id, owner/replica nodes, and explicit startup rejection/timeout reason.

## Inputs

- `compiler/mesh-rt/src/dist/node.rs` — current declared-work path exposes `submit_declared_work(...)` but has no startup registry or keepalive.
- `compiler/mesh-rt/src/dist/continuity.rs` — continuity records already carry runtime-name metadata and need stable startup identity handling.
- `compiler/mesh-rt/src/dist/operator.rs` — diagnostics buffer used to make startup trigger failures inspectable.
- `compiler/mesh-rt/src/actor/mod.rs` — scheduler shutdown/run behavior that currently lets route-free cluster-mode binaries exit immediately.
- `compiler/mesh-rt/src/lib.rs` — runtime export surface that must publish any new startup-work helpers.
- `compiler/mesh-codegen/src/codegen/mod.rs` — codegen hook from T01 that will invoke the runtime startup path.

## Expected Output

- `compiler/mesh-rt/src/dist/node.rs` — startup-work registration, bounded trigger, deterministic request identity, and keepalive behavior exist in the runtime.
- `compiler/mesh-rt/src/dist/continuity.rs` — continuity submit/record handling preserves stable startup identity and honest rejection state.
- `compiler/mesh-rt/src/dist/operator.rs` — diagnostics surface startup registration/timeout/rejection/completion transitions.
- `compiler/mesh-rt/src/actor/mod.rs` — scheduler/runtime interaction keeps route-free cluster-mode apps inspectable instead of exiting immediately.
- `compiler/mesh-rt/src/lib.rs` — startup-work helpers are exported consistently for codegen and tests.
