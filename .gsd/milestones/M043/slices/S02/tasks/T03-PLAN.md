---
estimated_steps: 4
estimated_files: 6
skills_used:
  - test
  - debug-like-expert
---

# T03: Wire cluster-proof to runtime-backed promotion and post-failover authority truth

**Slice:** S02 — Standby Promotion and Stale-Primary Fencing
**Milestone:** M043

## Description

Keep `cluster-proof` a thin consumer while making the new failover boundary visible. Replace the startup-env-derived “current” role, epoch, and health helpers with runtime-backed authority reads, add the explicit promotion operator surface, and ensure keyed status and error payloads reflect post-promotion truth from `mesh-rt` instead of stale config.

This task should not grow a Mesh-side disaster-recovery control plane. Config remains a startup topology contract only; live authority truth comes from the runtime API added in T02.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Runtime authority and promotion JSON consumed by `cluster-proof/work_continuity.mpl` | Fail closed and return an explicit promotion or authority-status error instead of inventing role or epoch in Mesh code. | Return the last truthful runtime status or an explicit timeout error instead of hanging the route. | Reject malformed runtime JSON and preserve the existing invalid-record failure surface. |
| Startup topology parsing in `cluster-proof/config.mpl` | Refuse to start when topology inputs are incomplete or contradictory, but never treat startup env as the source of live authority after promotion. | N/A — config parsing is synchronous. | Fail closed on blank or contradictory role and epoch inputs instead of silently defaulting to primary. |
| Operator-visible HTTP routes in `cluster-proof/main.mpl` and `cluster-proof/work.mpl` | Keep promotion, membership, and keyed status surfaces truthful even when the runtime is degraded; do not hide failover state behind generic 200-only responses. | Report explicit timeout or degraded-state truth promptly without hanging. | Reject malformed promote or status payloads and preserve existing keyed-work validation behavior. |

## Load Profile

- **Shared resources**: HTTP routes, authority-status polling, promotion result serialization, and keyed status serialization.
- **Per-operation cost**: one runtime promote or status call plus one JSON encode or decode for the response surface.
- **10x breakpoint**: repeated authority-status polling and failover retries will stress JSON encode or decode and route serialization first.

## Negative Tests

- **Malformed inputs**: invalid promote requests, malformed runtime authority JSON, stale config values after promotion, and malformed keyed status payloads.
- **Error paths**: promotion attempted from a primary node, promotion attempted twice, runtime status unavailable during missing-status rendering, and fenced old-primary status reporting after rejoin.
- **Boundary conditions**: first promotion to epoch `1`, missing request-key status after promotion, and startup env that is valid for topology but no longer authoritative for live state.

## Steps

1. Replace the `current_continuity_*` startup-env helpers with runtime-backed authority-status reads in `cluster-proof/work_continuity.mpl` and any surfaces that render live role, epoch, or health.
2. Add a single explicit operator promotion route in `cluster-proof/main.mpl` and keep its handler thin by delegating all authority changes to the new `Continuity` API.
3. Keep `cluster-proof/config.mpl` focused on startup topology validation only, and update keyed status and error payloads in `cluster-proof/work.mpl` and `cluster-proof/work_continuity.mpl` to reflect post-promotion runtime truth.
4. Extend `cluster-proof/tests/config.test.mpl` and `cluster-proof/tests/work.test.mpl` so promotion responses, missing-status payloads, and fenced old-primary reporting are mechanically covered.

## Must-Haves

- [ ] `cluster-proof` reads live authority truth from the runtime API rather than from startup env after promotion.
- [ ] The proof app exposes one explicit promotion route and keeps all authority mutation inside `mesh-rt`.
- [ ] Keyed status, missing-status, and error payloads stay truthful after promotion and fenced rejoin.
- [ ] Package tests prove the promotion boundary and runtime-backed post-failover status contract.

## Verification

- `cargo run -q -p meshc -- test cluster-proof/tests`
- `cargo run -q -p meshc -- build cluster-proof`

## Observability Impact

- Signals added/changed: promotion response bodies, runtime-backed role and epoch fields on membership and keyed status payloads, and proof-app logs that include promoted and fenced authority truth.
- How a future agent inspects this: run the `cluster-proof/tests` package suite, then inspect `cluster-proof/main.mpl` and `cluster-proof/work_continuity.mpl` logs or returned JSON on the promotion and status routes.
- Failure state exposed: stale-env truth, malformed runtime payloads, and invalid promotion attempts become visible at the proof-app boundary rather than looking like generic cluster drift.

## Inputs

- `cluster-proof/main.mpl` — current membership and keyed-work router surface.
- `cluster-proof/config.mpl` — startup topology validation that must remain narrow and fail-closed.
- `cluster-proof/work.mpl` — keyed status and error payload structures.
- `cluster-proof/work_continuity.mpl` — current `Continuity` consumer that still derives live role and epoch from config helpers.
- `cluster-proof/tests/config.test.mpl` — existing topology contract coverage.
- `cluster-proof/tests/work.test.mpl` — existing keyed-work contract coverage.
- `compiler/meshc/tests/e2e_m043_s02.rs` — compiler-facing API proof from T02 that defines the runtime contract this task consumes.
- `.gsd/milestones/M043/slices/S01/S01-SUMMARY.md` — mirrored-standby proof contract that must still hold before promotion.

## Expected Output

- `cluster-proof/main.mpl` — explicit promotion route and runtime-backed membership or status wiring.
- `cluster-proof/config.mpl` — startup topology validation kept narrow and clearly separated from live authority truth.
- `cluster-proof/work.mpl` — keyed payload structs or helpers updated for promoted and fenced authority reporting.
- `cluster-proof/work_continuity.mpl` — runtime-backed authority-status and promotion handling without app-authored failover logic.
- `cluster-proof/tests/config.test.mpl` — package tests covering startup topology constraints that still hold in the failover path.
- `cluster-proof/tests/work.test.mpl` — package tests covering promotion responses, missing-status truth, and fenced old-primary reporting.
