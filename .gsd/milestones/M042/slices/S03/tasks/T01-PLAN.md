---
estimated_steps: 24
estimated_files: 1
skills_used:
  - rust-best-practices
  - debug-like-expert
---

# T01: Implement recovery-aware attempt rollover and stale-completion fencing in the continuity registry

Close the correctness core inside `mesh-rt` before touching node lifecycle or live harnesses. Same-key retry after owner loss only becomes honest if the registry can roll a new attempt and then fence older completions everywhere.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Existing continuity records in `compiler/mesh-rt/src/dist/continuity.rs` | Keep the stored record authoritative when retry is not eligible; never mutate conflicting payloads into recovery state. | Do not block retry on missing liveness evidence; leave the record unchanged and fail closed to duplicate/conflict truth. | Reject impossible record combinations instead of merging them into active state. |
| Continuity merge and snapshot upserts | Prefer the newer attempt token before terminal/non-terminal phase so stale completed truth cannot overwrite an active retry. | Preserve the local newer attempt even if a rejoin snapshot arrives late. | Ignore payload-hash mismatches and attempt regressions instead of poisoning the registry. |

## Load Profile

- **Shared resources**: Continuity registry lock, attempt-token counter, and cluster-wide continuity upsert traffic.
- **Per-operation cost**: One registry mutation plus attempt-token parsing and merge-precedence checks.
- **10x breakpoint**: Hot-key retry storms hit lock contention and stale-upsert races first; precedence must remain monotonic under extra message volume.

## Negative Tests

- **Malformed inputs**: Missing request key / payload hash / owner node, invalid attempt IDs, and mismatched payload hashes on merge.
- **Error paths**: Same-key retry while the old owner is still authoritative, late `mark_completed(old_attempt)` after rollover, and stale completed upserts arriving after a newer retry was created.
- **Boundary conditions**: `attempt-0` to `attempt-1` rollover, repeated retry after a prior rollover, and snapshot/upsert merges where both sides carry different replica states for the same request key.

## Steps

1. Add an explicit recovery-eligible retry transition in `compiler/mesh-rt/src/dist/continuity.rs` for same-key same-payload pending records that have lost their active owner and should roll to a new `attempt_id`.
2. Reorder `preferred_record(...)` and any related helpers so parsed `attempt_id` tokens fence stale older records before terminal/non-terminal precedence is considered.
3. Keep `mark_completed(...)` fenced to the active attempt and add coverage for stale completion after rollover plus stale rejoin/snapshot merges.
4. Expand the continuity unit tests so recovery retry, stale completion rejection, and stale completed merge rejection are proven at the runtime level before node-lifecycle work starts.

## Must-Haves

- [ ] Same-key retry after owner loss can return `created` with a new active `attempt_id` instead of unconditional `duplicate`.
- [ ] Older completed or rejected records cannot overwrite a newer submitted retry through `merge_remote_record(...)` or `merge_snapshot(...)`.
- [ ] `mark_completed(old_attempt_id)` fails with `attempt_id_mismatch` after rollover.
- [ ] `next_attempt_token` stays monotonic across retry rollover and merged records.

## Inputs

- ``compiler/mesh-rt/src/dist/continuity.rs``
- ``cluster-proof/work.mpl``

## Expected Output

- ``compiler/mesh-rt/src/dist/continuity.rs``

## Verification

cargo test -p mesh-rt continuity -- --nocapture

## Observability Impact

Keeps the continuity status and runtime log surface authoritative by making the active `attempt_id` the durable fencing token for both local completion and replicated merge precedence.
