---
estimated_steps: 4
estimated_files: 2
skills_used:
  - rust-best-practices
  - debug-like-expert
---

# T01: Make continuity authority mutable and epoch-fence stale primaries

**Slice:** S02 — Standby Promotion and Stale-Primary Fencing
**Milestone:** M043

## Description

Close the highest-risk seam first inside `mesh-rt`. Replace the process-static authority model with mutable runtime state that can survive explicit promotion without discarding the mirrored in-memory registry, then rework merge and rejoin precedence so higher-epoch truth deposes stale primaries instead of projecting incoming records into the local role before comparison.

This task should keep the failover model runtime-owned. Reuse the existing owner-loss retry-rollover path for promoted pending records where possible, but do not invent a second Mesh-side failover state machine in `cluster-proof`.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Continuity authority and merge logic in `compiler/mesh-rt/src/dist/continuity.rs` | Keep the fresher authoritative record and return an explicit fencing or promotion error instead of silently overwriting state. | Preserve the last truthful local record; do not block status on a missing remote confirmation. | Reject impossible role, epoch, or attempt combinations before they can enter the registry. |
| Continuity wire handling in `compiler/mesh-rt/src/dist/node.rs` | Depose or ignore stale peers based on epoch instead of letting disconnect or reconnect races restore old authority. | Leave promoted truth intact and mark replication health accordingly rather than hanging. | Reject malformed sync or upsert payloads and preserve local truth rather than synthesizing defaults. |

## Load Profile

- **Shared resources**: continuity registry lock, attempt-token counter, authority snapshot state, and inter-node sync and upsert traffic.
- **Per-operation cost**: one registry lookup or update plus authority comparison, attempt-token parsing, and continuity payload encode or decode.
- **10x breakpoint**: hot-key submit and status traffic plus repeated reconnect churn will stress registry contention and message-ordering assumptions before raw CPU does.

## Negative Tests

- **Malformed inputs**: invalid role strings, impossible epoch transitions, stale completions carrying an older epoch, and malformed continuity sync payloads.
- **Error paths**: standby promotion without mirrored state, higher-epoch truth arriving after a stale primary already completed locally, and same-identity rejoin after promotion.
- **Boundary conditions**: epoch `0` mirrored standby state, first promotion to epoch `1`, repeated promotion on an already-promoted authority, and promoted pending records that should become recovery-eligible.

## Steps

1. Replace the `OnceLock` authority path with mutable runtime-owned authority state that survives explicit promotion without clearing the mirrored registry.
2. Rework merge and rejoin precedence so raw incoming role and epoch are compared before any local projection, and higher-epoch truth can depose a stale primary on sync or reconnect.
3. Promote mirrored pending records into the existing owner-loss retry-rollover seam where possible, and fence stale completions or lower-epoch upserts instead of inventing new Mesh-side recovery logic.
4. Add runtime unit coverage for promotion, stale-epoch rejection, promoted recovery eligibility, and fenced same-identity rejoin.

## Must-Haves

- [ ] Runtime authority state is mutable at runtime and preserved across explicit promotion without discarding mirrored request truth.
- [ ] Merge precedence and reconnect handling fence lower-epoch stale-primary truth instead of projecting it into local authority before comparison.
- [ ] Promoted pending mirrored records reuse the existing retry-rollover path instead of a new app-authored failover loop.
- [ ] Runtime tests prove promotion, stale completion rejection, and fenced rejoin behavior.

## Verification

- `cargo test -p mesh-rt continuity -- --nocapture`

## Observability Impact

- Signals added/changed: runtime continuity logs and unit assertions for promote, demote, stale-epoch rejection, retry-rollover, and fenced rejoin transitions.
- How a future agent inspects this: run the continuity test target and inspect `compiler/mesh-rt/src/dist/continuity.rs` and `compiler/mesh-rt/src/dist/node.rs` logs or assertions for the failing request key and epoch.
- Failure state exposed: promotion refusal, stale-epoch rejection, and fenced rejoin become attributable at the runtime seam instead of appearing as generic continuity drift.

## Inputs

- `compiler/mesh-rt/src/dist/continuity.rs` — current keyed continuity record, authority parsing, merge precedence, and retry-rollover logic.
- `compiler/mesh-rt/src/dist/node.rs` — current continuity wire transport, sync, and reconnect handling.
- `.gsd/milestones/M043/slices/S02/S02-RESEARCH.md` — failover-specific runtime constraints, promotion risks, and stale-primary fencing guidance.
- `.gsd/milestones/M043/slices/S01/S01-SUMMARY.md` — the shipped mirrored-standby contract that this task must preserve while adding promotion.

## Expected Output

- `compiler/mesh-rt/src/dist/continuity.rs` — mutable authority state, promotion logic, fencing precedence, and unit tests for promoted failover truth.
- `compiler/mesh-rt/src/dist/node.rs` — reconnect and transport handling updated to respect higher-epoch authority and fenced rejoin behavior.
