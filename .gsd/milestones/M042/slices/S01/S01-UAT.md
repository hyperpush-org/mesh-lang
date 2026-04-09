# S01: Runtime-native keyed continuity API on the healthy path — UAT

**Milestone:** M042
**Written:** 2026-03-28T22:14:31.668Z

# S01: Runtime-native keyed continuity API on the healthy path — UAT

**Milestone:** M042
**Written:** 2026-03-28

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: this slice changes compiler/runtime ABI, the runtime registry, and a live HTTP proof app. The truthful acceptance surface is a mix of unit tests, live runtime e2e, and preserved logs/artifacts from the same runs.

## Preconditions

- Run from the repo root.
- `cargo` and the Mesh workspace build toolchain are available.
- No stale `cluster-proof` process is already listening on the ephemeral test ports.

## Smoke Test

1. Run `cargo test -p mesh-rt continuity -- --nocapture`.
2. Confirm all six continuity-focused runtime tests pass.
3. **Expected:** the runtime continuity registry proves created/duplicate/conflict submit behavior, completion guard behavior, replica prepare/ack/reject transitions, snapshot merge preference, and continuity wire-format roundtrips.

## Test Cases

### 1. Standalone runtime-native keyed submit/status/retry contract

1. Run `cargo test -p meshc --test e2e_m042_s01 continuity_api_standalone_keyed_submit_status_and_retry_contract -- --nocapture`.
2. Wait for the test harness to build `mesh-rt`, build `cluster-proof`, spawn the standalone proof app, submit keyed work, poll status, retry the same key with the same payload, retry with a conflicting payload, and query a missing key.
3. **Expected:** the test passes; the standalone response keeps `request_key` stable, assigns a runtime-generated `attempt_id`, reports `owner_node=standalone@local`, `replica_status=unassigned`, transitions to `completed/succeeded`, returns the same attempt on duplicate retry, rejects conflicting reuse with HTTP 409 and `conflict_reason=request_key_conflict`, and returns 404 with `request_key_not_found` for a missing key.

### 2. Cluster-proof keyed route-selection and request parsing surface

1. Run `cargo run -q -p meshc -- test cluster-proof/tests`.
2. Confirm the config tests and keyed work contract tests execute.
3. **Expected:** the suite passes, proving deterministic placement across peer-order/ingress variations, single-node local fallback, malformed membership rejection, request-key validation, and keyed JSON submit parsing.

### 3. Healthy two-node runtime-native submit/status mirror (current blocker reproduction)

1. Run `cargo test -p meshc --test e2e_m042_s01 continuity_api_two_node_cluster_syncs_status_between_ingress_and_owner -- --nocapture`.
2. Wait for the harness to start the two cluster-proof nodes and search for a request key that routes to a remote owner.
3. **Expected today:** the test fails closed, not green. The ingress node reports a truthful runtime-native `submitted` record with `owner_node` set to the remote peer, `replica_node` set to ingress, and `replica_status=mirrored`, but the request never reaches `completed` because the owner side aborts in `compiler/mesh-rt/src/string.rs:104` after the remote spawn/execution path is exercised.

## Edge Cases

### Same-key duplicate after completion

1. Re-run the standalone test or inspect its assertions.
2. **Expected:** a duplicate submit with the same payload returns the original `attempt_id` and completed status instead of minting a second attempt.

### Same-key conflicting payload reuse

1. Re-run the standalone test or inspect its assertions.
2. **Expected:** the same `request_key` with a different payload returns HTTP 409 and `conflict_reason=request_key_conflict` while preserving the original attempt/status record.

### Missing keyed status

1. Re-run the standalone test or query `/work/missing-key` in the spawned standalone app.
2. **Expected:** the status surface returns HTTP 404 with `phase=missing`, `result=unknown`, and `error=request_key_not_found`.

## Failure Signals

- `meshc build cluster-proof` fails with missing `mesh_continuity_*` symbols -> stale `mesh-rt` staticlib was linked instead of a rebuilt runtime.
- The standalone e2e crashes in `compiler/mesh-rt/src/collections/list.rs:23` -> the route-selection seam regressed back to the fragile imported membership-list path.
- The two-node e2e stalls on `phase=submitted` with `replica_status=mirrored` and owner logs end in `compiler/mesh-rt/src/string.rs:104` -> the current remote execution/string transport blocker is still present.

## Requirements Proved By This UAT

- none — the slice establishes real progress and a concrete blocker, but it does not yet retire a tracked requirement to validated status because the healthy two-node completion proof is still red.

## Not Proven By This UAT

- Healthy two-node remote owner completion on the runtime-native path.
- Any fail-closed durability rejection path when replica safety is unavailable.
- Owner-loss recovery, same-key retry after owner loss, or stale-completion rejection after failover.

## Notes for Tester

Use `bash scripts/verify-m042-s01.sh` as the single replay command for the current slice state. Today it is intentionally fail-closed: it proves the standalone/runtime-native and unit-level surfaces, then stops on the two-node remote execution crash instead of hiding that blocker behind a partial green result.
