---
estimated_steps: 4
estimated_files: 3
skills_used:
  - rust-best-practices
  - debug-like-expert
---

# T01: Add runtime primary/standby continuity metadata and live replication merge rules

**Slice:** S01 — Primary→Standby Runtime Replication and Role Truth
**Milestone:** M043

## Description

Close the runtime authority seam before touching `cluster-proof`. This task extends `mesh-rt`'s continuity record, replication transport, and merge precedence so a standby cluster can hold mirrored request truth that is explicitly marked as standby-owned replicated state rather than looking like an ordinary same-cluster replica.

The task should stop short of promotion behavior. S01 only needs truthful mirrored standby state plus operator-visible role/epoch/health metadata. Promotion, stale-primary fencing on rejoin, and explicit failover action belong to S02.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Continuity record and merge logic in `compiler/mesh-rt/src/dist/continuity.rs` | Keep the fresher authoritative record and reject stale or impossible authority combinations instead of silently overwriting them. | Leave the last truthful local record in place; do not block status on missing remote confirmation. | Ignore malformed sync/upsert payloads and preserve local truth rather than synthesizing default authority state. |
| Continuity transport in `compiler/mesh-rt/src/dist/node.rs` | Fail closed on replication/setup errors and surface them in logs or record health fields. | Mark replication health accordingly instead of hanging a submit/status path. | Reject malformed wire payloads before they can poison the registry. |
| Discovery/topology assumptions in `compiler/mesh-rt/src/dist/discovery.rs` | Keep discovery as peer-finding only; never let it become implicit promotion authority. | Preserve current cluster truth and report degraded replication state. | Reject invalid topology metadata rather than deriving role from broken input. |

## Load Profile

- **Shared resources**: continuity registry lock, attempt-token counter, inter-node sync/upsert traffic, and any role/health snapshot state.
- **Per-operation cost**: one registry lookup/update plus continuity payload encode/decode and authority-metadata comparison.
- **10x breakpoint**: hot-key submit/status traffic with extra replication chatter will stress registry contention and stale-message ordering first.

## Negative Tests

- **Malformed inputs**: missing or invalid authority metadata, malformed continuity upsert payloads, and impossible role/epoch combinations.
- **Error paths**: standby receives an older authority snapshot after a newer one, replication is unavailable during a mirror attempt, and stale primary-shaped data arrives after standby truth is already fresher.
- **Boundary conditions**: epoch `0` initial mirrored state, same-request merges with identical epoch but different health, and healthy primary→standby replication without any promotion signal.

## Steps

1. Extend the runtime continuity record and any encoded sync/upsert payloads with the minimal authority metadata S01 needs: cluster role, promotion epoch, and replication health.
2. Rework merge precedence so fresher authority metadata and mirrored standby truth beat stale primary-shaped updates without introducing promotion or stale-primary fencing yet.
3. Keep replication runtime-owned inside `mesh-rt` transport/discovery seams; do not move failover logic into Mesh application code.
4. Add runtime continuity tests proving mirrored standby replication, stale-authority rejection, and truthful health/epoch propagation through snapshot and upsert flows.

## Must-Haves

- [ ] `mesh-rt` continuity records can represent primary vs standby mirrored truth without changing the public keyed-work semantics.
- [ ] Runtime merge precedence rejects stale authority updates instead of letting older primary-shaped records overwrite fresher mirrored standby state.
- [ ] Healthy-path primary→standby replication remains runtime-owned rather than app-authored.
- [ ] Runtime tests prove role/epoch/health propagation through the replicated continuity seam.

## Verification

- `cargo test -p mesh-rt continuity -- --nocapture`

## Observability Impact

- Signals added/changed: runtime continuity logs and/or record fields for cluster role, promotion epoch, replication health, and stale-authority rejection.
- How a future agent inspects this: run the continuity test target and inspect `compiler/mesh-rt/src/dist/continuity.rs` / `node.rs` logs or assertions for the mirrored standby cases.
- Failure state exposed: missing replication, stale-authority rejection, and malformed continuity sync become attributable at the runtime seam itself.

## Inputs

- `compiler/mesh-rt/src/dist/continuity.rs` — current keyed continuity record, merge precedence, and snapshot/upsert logic.
- `compiler/mesh-rt/src/dist/node.rs` — current continuity wire transport and connect/disconnect sync behavior.
- `compiler/mesh-rt/src/dist/discovery.rs` — current peer-discovery seam that must stay discovery-only.
- `.gsd/milestones/M043/M043-RESEARCH.md` — runtime-first design constraints and the authority/fencing boundaries for this milestone.

## Expected Output

- `compiler/mesh-rt/src/dist/continuity.rs` — continuity record, merge rules, and runtime tests updated for primary/standby authority metadata.
- `compiler/mesh-rt/src/dist/node.rs` — transport/sync logic updated to move the new mirrored-state metadata between clusters.
- `compiler/mesh-rt/src/dist/discovery.rs` — any narrowly-scoped topology/discovery support needed to keep replication truthful without turning discovery into a control plane.
