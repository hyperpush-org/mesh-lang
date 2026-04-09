---
estimated_steps: 34
estimated_files: 7
skills_used: []
---

# T04: Stopped before a dishonest `cluster-proof` rewrite and recorded the missing S02 declared-execution seams.

---
estimated_steps: 4
estimated_files: 7
skills_used:
  - test
---

# T04: Rewrite cluster-proof onto runtime-owned declared execution

**Slice:** S02 — Runtime-Owned Declared Handler Execution
**Milestone:** M044

## Description

`cluster-proof` is the dogfood consumer for this slice, so it must stop declaring route handlers as clustered work and stop computing placement/dispatch on the new submit/status path. This task retargets the manifest to the real business handlers or generated wrappers from T02/T03, keeps HTTP parsing/JSON shaping local, and shrinks the proof app’s clustering logic to the legacy surfaces that still belong to S05.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Retargeted `cluster-proof/mesh.toml` declarations | Fail the build if the proof app still declares route wrappers or undeclared helpers. | N/A — manifest validation is synchronous. | Reject stale targets rather than broadening the clustered boundary to keep the app green. |
| Runtime-owned submit/status path | Return truthful HTTP errors from runtime authority/continuity failures; do not recreate app-local placement or `Node.spawn(...)` on error. | Preserve continuity timeout/error surfaces at HTTP level. | Reject malformed runtime payloads at the app boundary instead of reintroducing JSON shims or fake local fallback. |
| Legacy probe coexistence | Keep the existing legacy proof path honest without letting it become the clustered hot path again. | Preserve existing timeout/error reporting for the legacy probe. | Scope any remaining legacy helpers to the old probe only. |

## Load Profile

- **Shared resources**: HTTP routes, runtime continuity/status reads, cluster-proof package tests, and proof-app logs.
- **Per-operation cost**: one HTTP parse/encode plus one runtime clustered submit/status operation per request.
- **10x breakpoint**: proof-app HTTP/log volume should become the first bottleneck, not app-owned placement math.

## Negative Tests

- **Malformed inputs**: invalid JSON, blank request keys, and malformed status lookups.
- **Error paths**: same-key duplicate/conflict at HTTP level, runtime authority unavailable, and declared target mismatch in the manifest.
- **Boundary conditions**: standalone local execution, two-node remote-owner execution, and coexistence with the legacy probe path.

## Steps

1. Retarget `cluster-proof/mesh.toml` declarations from `handle_work_*` ingress handlers to the real runtime-safe declared work/service targets introduced in T02/T03.
2. Rewrite `cluster-proof/work_continuity.mpl` so submit/status/promotion call the runtime-owned declared execution/status surfaces and no longer compute `current_target_selection`, `canonical_placement`, or actor-context `Node.spawn(...)` on the new path.
3. Shrink `cluster-proof/work.mpl` and `cluster-proof/cluster.mpl` to the surfaces still needed for status/legacy proof behavior; keep HTTP request parsing and JSON response shaping local.
4. Update `cluster-proof/tests/work.test.mpl` and proof-app wiring in `cluster-proof/main.mpl` / `work_legacy.mpl` to assert the new declared-runtime boundary and the remaining legacy boundary honestly.

## Must-Haves

- [ ] `cluster-proof` no longer declares HTTP ingress handlers as clustered work.
- [ ] The new submit/status hot path does not compute placement or call `Node.spawn(...)` in Mesh code.
- [ ] HTTP behavior stays the same externally while runtime-owned execution truth drives the result.

## Inputs

- ``cluster-proof/mesh.toml` — current route-shaped clustered declarations from S01.`
- ``cluster-proof/work_continuity.mpl` — current app-owned placement, continuity submit, and remote dispatch hot path.`
- ``cluster-proof/work.mpl` — request-key validation and target-selection helpers, including the runtime-owned placement seam to delete.`
- ``cluster-proof/cluster.mpl` — current canonical membership/placement logic that should stop owning the new path.`
- ``cluster-proof/main.mpl` — HTTP ingress wiring that must remain local while clustered execution moves behind runtime-owned handlers.`
- ``cluster-proof/tests/work.test.mpl` — proof-app contract tests that must move with the rewrite.`

## Expected Output

- ``cluster-proof/mesh.toml` — retargeted declarations that name real runtime-safe work/service handlers instead of route wrappers.`
- ``cluster-proof/work_continuity.mpl` — thin HTTP-to-runtime adapter for the new declared-handler path.`
- ``cluster-proof/tests/work.test.mpl` — proof-app assertions covering the runtime-owned declared execution path and the remaining legacy boundary.`

## Verification

`cargo test -p meshc --test e2e_m044_s02 m044_s02_cluster_proof_ -- --nocapture`
`cargo run -q -p meshc -- build cluster-proof`
`cargo run -q -p meshc -- test cluster-proof/tests`

## Observability Impact

- Signals added/changed: proof-app submit/status logs reflect runtime-owned owner/replica/execution data instead of app-computed placement decisions.
- How a future agent inspects this: `cluster-proof` build/test output plus the named `m044_s02_cluster_proof_` e2e artifacts.
- Failure state exposed: request key, declared target, owner/replica, cluster role, promotion epoch, and runtime reject reason at the HTTP boundary.
