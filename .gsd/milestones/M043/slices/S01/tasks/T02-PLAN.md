---
estimated_steps: 4
estimated_files: 7
skills_used:
  - best-practices
  - test
---

# T02: Surface primary/standby role truth through cluster-proof and package tests

**Slice:** S01 — Primary→Standby Runtime Replication and Role Truth
**Milestone:** M043

## Description

Keep `cluster-proof` as a thin consumer while making the new runtime truth visible to operators. This task should expose the runtime-owned primary/standby role, promotion epoch, and replication-health state through the existing operator-visible surfaces instead of inventing a second DR control plane in Mesh code.

The env contract must stay narrow. The right outcome is a small, legible topology surface that tells the runtime whether a node belongs to the active primary cluster or standby cluster, then lets `cluster-proof` report whatever the runtime already knows.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Runtime continuity/status JSON consumed by `cluster-proof/work_continuity.mpl` | Fail closed and return an explicit contract error instead of inventing role or epoch fields in app code. | Status handlers should return the last truthful runtime state rather than waiting on background replication. | Reject malformed continuity JSON and preserve the existing invalid-record failure surface. |
| Cluster config/env parsing in `cluster-proof/config.mpl` | Refuse to start when topology/role inputs are incomplete or contradictory. | N/A — config parsing is synchronous. | Fail closed on blank/invalid role values or mismatched topology inputs instead of silently defaulting to primary. |
| Operator-visible HTTP surfaces in `cluster-proof/main.mpl` and `cluster-proof/work.mpl` | Keep `/membership` and keyed status truthful even when replication is unhealthy; do not hide degraded state behind 200-only happy-path prose. | Return explicit health/status truth promptly without hanging the route. | Reject malformed request/status bodies and preserve existing keyed-work validation behavior. |

## Load Profile

- **Shared resources**: runtime continuity status payloads, cluster membership snapshots, package-level config state, and keyed status polling.
- **Per-operation cost**: one config decode, one membership/status serialization, and one runtime continuity status parse per keyed lookup.
- **10x breakpoint**: repeated keyed polling plus wider mirrored-status payloads will stress JSON encode/decode and route serialization first.

## Negative Tests

- **Malformed inputs**: invalid topology role env, blank or contradictory cluster inputs, invalid keyed status JSON, and malformed submit/status request bodies.
- **Error paths**: replication unhealthy while `/membership` and `/work/:request_key` are polled, standby sees mirrored truth but should not claim promotion, and startup with missing role configuration fails closed.
- **Boundary conditions**: initial epoch `0`, standby mirror with no promotion, and the smallest valid primary/standby env surface.

## Steps

1. Extend `cluster-proof/config.mpl` with the minimal primary/standby topology inputs S01 needs while keeping the env contract small and fail-closed.
2. Thread the runtime-owned role, promotion epoch, and replication-health fields through `/membership` plus keyed continuity status payloads in `cluster-proof/main.mpl`, `cluster-proof/work.mpl`, and `cluster-proof/work_continuity.mpl`.
3. Keep `cluster-proof/cluster.mpl` and related helpers aligned with the runtime placement/identity story so the operator-visible surfaces stay consistent.
4. Add package tests proving config validation, mirrored standby status visibility, and the absence of app-authored promotion logic.

## Must-Haves

- [ ] `cluster-proof` exposes primary/standby role, promotion epoch, and replication health through the existing operator-visible membership and keyed status surfaces.
- [ ] The topology/env contract stays narrow and fail-closed instead of growing bespoke per-node orchestration rules.
- [ ] `cluster-proof` continues to consume runtime truth rather than implementing disaster-recovery logic itself.
- [ ] Package tests prove the new config and status surfaces before the end-to-end harness is added.

## Verification

- `cargo run -q -p meshc -- test cluster-proof/tests`
- `cargo run -q -p meshc -- build cluster-proof`

## Observability Impact

- Signals added/changed: `/membership` and keyed `/work/:request_key` payloads/logs gain cluster role, promotion epoch, and replication-health fields.
- How a future agent inspects this: run the package tests, then hit the two HTTP surfaces on a local cluster and compare the returned topology/continuity JSON.
- Failure state exposed: invalid topology env, malformed runtime continuity payloads, and degraded replication truth become visible through the same public proof surfaces.

## Inputs

- `compiler/mesh-rt/src/dist/continuity.rs` — new runtime-owned authority metadata and replication-health state from T01.
- `compiler/mesh-rt/src/dist/node.rs` — runtime sync/transport behavior that backs the visible status fields.
- `cluster-proof/config.mpl` — current small env contract for cluster identity and durability.
- `cluster-proof/main.mpl` — HTTP router and membership/status endpoints.
- `cluster-proof/cluster.mpl` — placement and membership helper surface that must stay consistent with runtime truth.
- `cluster-proof/work.mpl` — keyed-work structs and operator-visible payload shapes.
- `cluster-proof/work_continuity.mpl` — thin `Continuity.*` consumer and keyed status translation logic.

## Expected Output

- `cluster-proof/config.mpl` — minimal primary/standby topology config with fail-closed validation.
- `cluster-proof/main.mpl` — membership and keyed-status routes updated to report the new runtime truth.
- `cluster-proof/cluster.mpl` — helper alignment for any visible placement/topology fields.
- `cluster-proof/work.mpl` — payload structs updated for role/epoch/replication-health truth.
- `cluster-proof/work_continuity.mpl` — continuity-status parsing and surface mapping updated without adding app-authored DR policy.
- `cluster-proof/tests/config.test.mpl` — package tests covering topology/config validation.
- `cluster-proof/tests/work.test.mpl` — package tests covering mirrored standby status visibility and keyed-surface truth.
