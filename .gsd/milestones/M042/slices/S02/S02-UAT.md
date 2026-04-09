# S02: Replica-backed admission and fail-closed durability truth — UAT

**Milestone:** M042
**Written:** 2026-03-28T23:40:43.868Z

# S02: Replica-backed admission and fail-closed durability truth — UAT

**Milestone:** M042
**Written:** 2026-03-28

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: This slice is a runtime/distributed truth change. The honest acceptance surface is a mix of live-runtime execution and artifact inspection: the verifier must exercise the real two-node and one-node paths, then confirm the preserved HTTP JSON and per-node stderr logs match the claimed rejected/mirrored/degraded state.

## Preconditions

- Run from the repo root with the Rust toolchain available.
- No stale `cluster-proof` processes should be holding the ephemeral test ports.
- The repo should build locally; the verifier itself will replay `cargo build -q -p mesh-rt`, `cargo run -q -p meshc -- test cluster-proof/tests`, and `cargo run -q -p meshc -- build cluster-proof` before the slice-specific e2e cases.
- If you want the assembled evidence bundle, prefer `bash scripts/verify-m042-s02.sh` over running the tests individually.

## Smoke Test

1. Run `bash scripts/verify-m042-s02.sh`.
2. Confirm the command exits 0 and ends with `verify-m042-s02: ok`.
3. Open `.tmp/m042-s02/verify/status.txt` and `.tmp/m042-s02/verify/phase-report.txt`.
4. **Expected:** `status.txt` contains `ok`, and `phase-report.txt` shows every phase through `s02-degraded-status` as `passed`.

## Test Cases

### 1. Single-node replica-required submit is durably rejected and replayed truthfully

1. Run `cargo test -p meshc --test e2e_m042_s02 continuity_api_single_node_cluster_rejects_replica_required_submit_and_replays_status -- --nocapture`.
2. Inspect the copied bundle under `.tmp/m042-s02/verify/05-rejection-artifacts/` (or the latest matching `.tmp/m042-s02/continuity-api-single-node-rejection-*` directory if running the test directly).
3. Open `rejected-submit.json`, `rejected-status.json`, and `rejected-duplicate.json`.
4. Open `rejected-conflict.json`.
5. **Expected:**
   - `rejected-submit.json` shows `ok=false`, `phase="rejected"`, `result="rejected"`, `replica_status="rejected"`, and `error="replica_required_unavailable"`.
   - `rejected-status.json` replays the same stored rejected truth for the same `request_key`/`attempt_id`.
   - `rejected-duplicate.json` preserves the rejected record instead of dispatching new work.
   - `rejected-conflict.json` still reports the existing same-key conflict contract rather than silently overwriting the prior record.

### 2. Two-node local-owner admission mirrors pending truth on both sides before completion

1. Run `cargo test -p meshc --test e2e_m042_s02 continuity_api_two_node_local_owner_mirrors_status_between_owner_and_replica -- --nocapture`.
2. Inspect `.tmp/m042-s02/verify/06-mirrored-artifacts/` (or the latest matching `.tmp/m042-s02/continuity-api-two-node-mirrored-*` directory).
3. Open `pending-owner-status.json` and `pending-replica-status.json`.
4. Open `completed-owner-status.json`, `completed-replica-status.json`, and `completed-duplicate.json`.
5. **Expected:**
   - The pending owner and replica status files both show the same `request_key` / `attempt_id`, `result="pending"`, and `replica_status="mirrored"`.
   - `owner_node` and `replica_node` stay explicit and consistent between the two status reads.
   - After completion, the completed status files remain truthful on both nodes, and `completed-duplicate.json` replays the stored accepted result instead of resubmitting the work.

### 3. Replica loss downgrades mirrored pending work to `degraded_continuing`

1. Run `cargo test -p meshc --test e2e_m042_s02 continuity_api_replica_loss_degrades_pending_mirrored_status -- --nocapture`.
2. Inspect `.tmp/m042-s02/verify/07-degraded-artifacts/` (or the latest matching `.tmp/m042-s02/continuity-api-two-node-degraded-*` directory).
3. Open `pending-owner-status.json` and `degraded-owner-status.json`.
4. Open `node-a.stderr.log`.
5. **Expected:**
   - The pre-loss pending status shows `result="pending"` and `replica_status="mirrored"`.
   - After the replica is lost, `degraded-owner-status.json` still shows `ok=true` and `result="pending"`, but `replica_status` changes to `degraded_continuing` instead of staying mirrored.
   - `node-a.stderr.log` contains a continuity transition line with `transition=degraded` and `reason=replica_lost:<node>`.

## Edge Cases

### Malformed non-JSON HTTP response is archived as a contract failure surface

1. Run `cargo test -p meshc --test e2e_m042_s02 continuity_api_archives_non_json_http_response_as_contract_failure -- --nocapture`.
2. Inspect `.tmp/m042-s02/verify/04-malformed-artifacts/` (or the latest matching `.tmp/m042-s02/continuity-api-malformed-response-*` directory).
3. **Expected:** The bundle contains both `malformed-response.http` and `malformed-response.body.txt`, proving the harness archived the bad body instead of silently treating it as success or dropping the evidence.

### Wrapper fails closed on missing proof evidence

1. After a normal `bash scripts/verify-m042-s02.sh` run, inspect `.tmp/m042-s02/verify/*.test-count.log` and the three copied artifact directories under `05-rejection-artifacts/`, `06-mirrored-artifacts/`, and `07-degraded-artifacts/`.
2. **Expected:** Each named phase has a test-count log showing the intended tests actually ran, and each copied artifact directory contains the JSON/log files referenced by the phase-specific artifact check logs. A green wrapper run without those files would be a verifier bug.

## Failure Signals

- `bash scripts/verify-m042-s02.sh` exits non-zero, `status.txt` is not `ok`, or `phase-report.txt` stops before `s02-degraded-status`.
- The rejection artifact shows `ok=true`, missing `replica_required_unavailable`, or a new `attempt_id` on duplicate replay.
- The mirrored artifact never reaches `replica_status="mirrored"` on both owner and replica status reads.
- The degraded artifact stays `mirrored` after peer loss, or the status surface flips to a hard failure instead of truthful `degraded_continuing` pending state.
- The degraded stderr log lacks the continuity transition line or the copied artifact bundle is missing JSON/log files the verifier claims to have checked.

## Requirements Proved By This UAT

- R049 — Advances the runtime-owned keyed continuity contract by proving durable rejected replay and accepted duplicate replay through the ordinary submit/status rail.
- R050 — Advances replica-backed continuity by proving mirrored pending admission on two nodes, explicit rejection when replica safety is unavailable, and truthful downgrade to `degraded_continuing` after replica loss.

## Not Proven By This UAT

- Owner-loss recovery, rolled `attempt_id` retry, and stale-completion rejection after the active owner dies (S03 scope).
- Cross-cluster standby replication or full active-cluster disaster continuity (R051 / later milestone scope).
- The unrelated remote-owner completion path that still hits the existing `Node.spawn` string-argument/runtime crash after mirrored submission.

## Notes for Tester

This slice intentionally proves runtime-owned durability truth on the stable local-owner rail. Do not treat the absence of remote-owner completion proof here as an S02 failure; that remains a separate runtime blocker for S03. Also note that replica-loss reason is currently a stderr-only diagnostic (`transition=degraded ... reason=replica_lost:<node>`); the HTTP status JSON only promises truthful `replica_status="degraded_continuing"`, not the textual reason.
