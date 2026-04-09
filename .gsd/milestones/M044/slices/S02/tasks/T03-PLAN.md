---
estimated_steps: 35
estimated_files: 6
skills_used: []
---

# T03: Stopped at the investigation handoff and recorded the exact clustered service-wrapper seams before implementation.

---
estimated_steps: 4
estimated_files: 6
skills_used:
  - rust-best-practices
  - test
---

# T03: Generate clustered service-call and service-cast execution wrappers

**Slice:** S02 — Runtime-Owned Declared Handler Execution
**Milestone:** M044

## Description

Declared service handlers cannot reuse the raw remote-spawn path as-is because the real handler bodies are compiler-internal `__service_*` functions. This task turns S01’s service declarations into runtime-executable clustered wrapper/thunk symbols, keeps the internal service loop private, and proves declared-vs-undeclared behavior stays honest.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `ServiceExportInfo` method mapping | Fail compilation if a declared service target cannot be mapped to a clustered wrapper. | N/A — compile-time only. | Do not expose raw `__service_*` internals as a fallback. |
| Runtime registration/dispatch for service wrappers | Reject undeclared or stale wrapper ids explicitly. | Surface reply timeout or missing remote result as an error, not as a silent cast. | Reject malformed reply payloads instead of corrupting service state. |
| Existing local service start/call/cast lowering | Preserve ordinary local behavior for undeclared services and `start` helpers. | N/A — local path is synchronous except existing service scheduler behavior. | Treat wrapper/local-path mismatches as lowering regressions. |

## Load Profile

- **Shared resources**: actor scheduler, service mailboxes, node sessions, and declared-handler registry state.
- **Per-operation cost**: one wrapper dispatch plus service message encode/decode per clustered call or cast.
- **10x breakpoint**: remote service mailbox pressure and reply waiters fail before codegen complexity matters.

## Negative Tests

- **Malformed inputs**: wrong arity, wrong service/method kind, and undeclared method targets.
- **Error paths**: remote reply timeout, missing reply, and wrapper-to-handler symbol mismatch.
- **Boundary conditions**: declared call vs declared cast, undeclared local service methods, and `start` helpers staying non-clustered.

## Steps

1. Generate declared clustered wrapper/thunk symbols for service call/cast handlers from `ServiceExportInfo.methods` instead of exposing compiler-internal `__service_*` bodies directly.
2. Register only declared service wrappers with the runtime declared-handler registry and lower clustered service calls/casts through that path.
3. Preserve ordinary local service start/call/cast behavior for undeclared services and start helpers.
4. Extend `compiler/meshc/tests/e2e_m044_s02.rs` with `m044_s02_service_` coverage for declared remote call/cast behavior and undeclared local service behavior.

## Must-Haves

- [ ] Declared `service_call` and `service_cast` targets map to runtime-executable wrappers, not raw `__service_*` internals.
- [ ] Undeclared service methods and `start` helpers stay on the ordinary local path.
- [ ] Named tests prove declared-vs-undeclared service behavior in a clustered runtime.

## Inputs

- ``compiler/mesh-typeck/src/lib.rs` — `ServiceExportInfo` source of truth for exported methods and generated names.`
- ``compiler/mesh-codegen/src/mir/lower.rs` — service helper generation and current `__service_*` lowering.`
- ``compiler/mesh-codegen/src/codegen/mod.rs` — top-level function registration path that currently skips compiler-internal service handlers.`
- ``compiler/meshc/tests/e2e_m044_s02.rs` — task-owned S02 e2e file that will prove declared service execution.`

## Expected Output

- ``compiler/mesh-codegen/src/mir/lower.rs` — generated clustered service wrapper/thunk symbols tied to declared handlers only.`
- ``compiler/mesh-codegen/src/codegen/mod.rs` — registration logic that exposes declared service wrappers without widening undeclared behavior.`
- ``compiler/meshc/tests/e2e_m044_s02.rs` — named `m044_s02_service_` coverage for declared remote service execution and undeclared local behavior.`

## Verification

`cargo test -p meshc --test e2e_m044_s02 m044_s02_service_ -- --nocapture`

## Observability Impact

- Signals added/changed: runtime/service logs surface declared service target, wrapper symbol, dispatch kind, and remote/local execution node.
- How a future agent inspects this: run the named `m044_s02_service_` filter and inspect the retained e2e artifact bundle.
- Failure state exposed: service target, wrapper symbol, call-vs-cast path, and reply timeout/reject reason.
