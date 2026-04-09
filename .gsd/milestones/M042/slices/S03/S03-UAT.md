# S03: Owner-loss recovery, same-key retry, and stale-completion safety — UAT

**Milestone:** M042
**Written:** 2026-03-29T01:03:50.826Z

# S03: Owner-loss recovery, same-key retry, and stale-completion safety — UAT

**Milestone:** M042
**Written:** 2026-03-28

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: This slice is runtime-heavy and destructive. The honest acceptance surface is a live two-node replay plus retained HTTP/log artifacts that prove the status contract before owner loss, during owner loss, after retry rollover, and after same-identity rejoin.

## Preconditions

- Run from the repo root with the Rust workspace buildable.
- No stale `cluster-proof` processes should still be bound to old ephemeral ports.
- Do **not** run `cargo test -p meshc --test e2e_m042_s03 -- --nocapture` in parallel with `bash scripts/verify-m042-s03.sh`; serialize them.

## Smoke Test

1. Run `bash scripts/verify-m042-s03.sh`.
2. Open `.tmp/m042-s03/verify/status.txt` and `.tmp/m042-s03/verify/phase-report.txt`.
3. **Expected:** `status.txt` is exactly `ok`, `current-phase.txt` is `complete`, and `phase-report.txt` shows `passed` for `runtime-continuity`, `cluster-proof-tests`, `build-cluster-proof`, `s02-contract`, `s03-e2e`, `owner-loss-recovery`, and `rejoin-truth`.

## Test Cases

### 1. Owner loss is surfaced truthfully on the surviving replica

1. Run `bash scripts/verify-m042-s03.sh`.
2. Open the path named in `.tmp/m042-s03/verify/04-owner-loss-recovery-artifacts.txt`.
3. Inspect `owner-lost-status.json`.
4. **Expected:** the JSON still shows the original `attempt_id`, `phase` is `submitted`, `result` is `pending`, `replica_status` is `owner_lost`, `owner_node` is the lost node identity, `replica_node` is the surviving node identity, `execution_node` is empty, and `ok` is `true`.

### 2. Same-key retry rolls forward to a newer attempt on the survivor

1. In the same owner-loss artifact directory, inspect `retry-rollover.json`.
2. Compare `retry-rollover.json.attempt_id` against `owner-lost-status.json.attempt_id`.
3. Inspect `retry-pending-status.json` and `retry-completed-status.json`.
4. **Expected:** the retry `attempt_id` is newer than the owner-lost attempt, `owner_node` has moved to the survivor, `replica_node` is empty, `replica_status` is `unassigned`, the pending retry stays `submitted/pending`, and the completed retry ends as `completed/succeeded` without reviving the older attempt.

### 3. Stale completion from the superseded attempt cannot overwrite the newer retry

1. Open the path named in `.tmp/m042-s03/verify/05-rejoin-truth-artifacts.txt`.
2. Inspect `stale-completion-guard.json`.
3. Compare it with `retry-completed-status.json` from the same rejoin artifact directory.
4. **Expected:** `stale-completion-guard.json` still reports the newer recovered `attempt_id`, `phase` remains `completed`, `result` remains `succeeded`, `owner_node` stays on the survivor, and no stale old-attempt completion resurrects the superseded owner mapping.

### 4. Same-identity rejoin preserves the newer attempt as authoritative on both nodes

1. In the rejoin artifact directory, inspect `post-rejoin-node-a-status.json` and `post-rejoin-node-b-status.json`.
2. Compare both files’ `attempt_id`, `phase`, `result`, and `owner_node` fields.
3. Optionally inspect `membership-node-a-run2.json` and `membership-node-b-run2.json` to confirm the cluster has re-formed.
4. **Expected:** both post-rejoin status files report the same newer recovered `attempt_id`, both stay `completed/succeeded`, both point at the survivor-owned retry rather than the original owner attempt, and rejoin does not resurrect the stale record.

### 5. The direct S03 e2e target is green when run serially

1. Run `cargo test -p meshc --test e2e_m042_s03 -- --nocapture`.
2. **Expected:** exactly 2 tests run and both pass: `continuity_api_owner_loss_retry_rollover_survivor_status_is_truthful` and `continuity_api_same_identity_rejoin_preserves_newer_attempt_truth`.

## Edge Cases

### Serialized proof only

1. Run the direct S03 e2e target by itself.
2. After it completes, run `bash scripts/verify-m042-s03.sh`.
3. **Expected:** both commands pass. If they are run in parallel instead, a false red e2e failure is possible even when the serialized acceptance rail is green.

### S02 prerequisite stays green during S03 replay

1. After `bash scripts/verify-m042-s03.sh`, inspect `.tmp/m042-s03/verify/03-s02-status.txt` and `.tmp/m042-s03/verify/03-s02-phase-report.txt`.
2. **Expected:** S03’s prerequisite replay did not regress S02; `03-s02-status.txt` is `ok` and the S02 phase report still marks the degraded-status phase as passed.

## Failure Signals

- `scripts/verify-m042-s03.sh` exits non-zero or does not write `status.txt=ok`.
- `owner-lost-status.json` never reaches `replica_status="owner_lost"` after the owner process is killed.
- `retry-rollover.json` reuses the old `attempt_id` or returns a rejected durability error instead of converging through a newer attempt.
- `stale-completion-guard.json` or either post-rejoin status file falls back to the older attempt or owner mapping.
- The direct S03 e2e run reports fewer than 2 tests or any failure.

## Requirements Proved By This UAT

- R050 — The surviving node retains truthful replicated continuity state after owner loss, same-key retry rolls to a newer attempt, stale completion is fenced, and same-identity rejoin preserves the newer attempt as authoritative.

## Not Proven By This UAT

- Cross-cluster disaster failover to a standby cluster.
- Arbitrary process-state migration or exactly-once semantics.
- The older healthy two-node remote-owner execution path that is still blocked by the separate remote `Node.spawn` string-argument/runtime crash.

## Notes for Tester

- Treat `bash scripts/verify-m042-s03.sh` as the canonical acceptance rail for this slice.
- The retained artifact manifests under `.tmp/m042-s03/verify/04-owner-loss-recovery-artifacts.txt` and `05-rejoin-truth-artifacts.txt` are the fastest way to inspect the proof without rerunning the entire harness.
- The runtime-side diagnostic log signal for owner loss is `[mesh-rt continuity] transition=owner_lost ...`; use the copied per-node stderr logs in the artifact bundle if a phase fails.
