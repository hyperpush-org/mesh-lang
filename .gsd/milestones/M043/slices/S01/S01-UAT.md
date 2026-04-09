# S01: Primary→Standby Runtime Replication and Role Truth — UAT

**Milestone:** M043
**Written:** 2026-03-29T07:25:24.333Z

# S01: Primary→Standby Runtime Replication and Role Truth — UAT

**Milestone:** M043
**Written:** 2026-03-28

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: S01 is a runtime + proof-surface slice. The honest acceptance bar is a mix of direct runtime tests, live destructive e2e on the real `cluster-proof` path, and retained JSON/log artifact inspection.

## Preconditions

- Run from the repo root with the Rust toolchain available.
- Ensure no stale `cluster-proof` processes are still listening on previous test ports.
- Keep the working tree on the current M043/S01 code; these commands assert the shipped runtime/proof-app contract, not a mock harness.

## Smoke Test

1. Run `bash scripts/verify-m043-s01.sh`.
2. Open `.tmp/m043-s01/verify/phase-report.txt`.
3. **Expected:** every phase is `passed` (`runtime-continuity`, `cluster-proof-tests`, `build-cluster-proof`, `m043-e2e`, `malformed-contract`, and `primary-to-standby`), and the script prints `verify-m043-s01: ok`.

## Test Cases

### 1. Runtime authority metadata and standby-safe merge rules

1. Run `cargo test -p mesh-rt continuity -- --nocapture`.
2. Confirm the output includes the new authority-specific tests, especially `continuity_merge_projects_remote_truth_into_standby_role`, `continuity_standby_truth_degrades_replication_health_without_owner_loss`, and `continuity_merge_prefers_healthier_state_at_same_epoch`.
3. **Expected:** 28 continuity tests pass, and the log stream shows continuity transitions carrying `cluster_role`, `promotion_epoch`, and `replication_health`.

### 2. Proof-app topology and status surfaces stay fail-closed

1. Run `cargo run -q -p meshc -- test cluster-proof/tests`.
2. Confirm the config test output includes `cluster mode requires an explicit continuity role` and the keyed work test output includes `membership payload surfaces runtime topology truth without app-authored promotion` plus `status payload preserves mirrored standby role, epoch, and replication health`.
3. Run `cargo run -q -p meshc -- build cluster-proof`.
4. **Expected:** the package tests pass, the named topology/status cases are present, and `cluster-proof` builds successfully.

### 3. Live primary submit mirrors request truth to standby without promotion

1. Run `cargo test -p meshc --test e2e_m043_s01 -- --nocapture`.
2. Confirm the suite runs exactly 2 tests and both pass.
3. Open `.tmp/m043-s01/verify/05-primary-to-standby-artifacts/continuity-api-m043-primary-to-standby-1774768888345644000/membership-primary.json`.
4. Verify it contains `"cluster_role": "primary"`, `"promotion_epoch": 0`, and `"replication_health": "healthy"`.
5. Open `.tmp/m043-s01/verify/05-primary-to-standby-artifacts/continuity-api-m043-primary-to-standby-1774768888345644000/pending-standby.json`.
6. Verify it contains `"cluster_role": "standby"`, `"promotion_epoch": 0`, `"replica_status": "mirrored"`, `"replication_health": "healthy"`, and that `"execution_node"` is still empty while the request is pending.
7. Open `.tmp/m043-s01/verify/05-primary-to-standby-artifacts/continuity-api-m043-primary-to-standby-1774768888345644000/completed-standby.json`.
8. Verify it contains `"cluster_role": "standby"`, `"phase": "completed"`, `"result": "succeeded"`, and `"execution_node": "primary@127.0.0.1:52033"`.
9. **Expected:** standby mirrors the request truth with explicit standby authority metadata, but execution still belongs to the primary and promotion remains at epoch 0.

### 4. Missing authority fields fail closed instead of silently degrading the contract

1. Open `.tmp/m043-s01/verify/04-malformed-artifacts/continuity-api-m043-malformed-contract-1774768862109978000/missing-membership-authority.json`.
2. Confirm the JSON omits `cluster_role`, `promotion_epoch`, and `replication_health`.
3. Re-run the focused negative path with `cargo test -p meshc --test e2e_m043_s01 e2e_m043_s01_missing_authority_fields_fail_closed -- --nocapture`.
4. **Expected:** the test passes by catching the missing-field failure; the panic text may appear in output under `--nocapture`, but the test verdict remains green because the contract fails closed instead of accepting incomplete authority data.

## Edge Cases

### Standby stays non-authoritative before promotion exists

1. Inspect `pending-standby.json` and `completed-standby.json` from the retained artifact bundle.
2. Confirm both files keep `"cluster_role": "standby"` and `"promotion_epoch": 0` even after the request completes.
3. **Expected:** standby mirrors request truth but does not implicitly promote itself, claim execution ownership, or advertise a higher epoch.

### Artifact retention remains part of the acceptance contract

1. Run `find .tmp/m043-s01/verify/05-primary-to-standby-artifacts -maxdepth 2 -type f | sort`.
2. Confirm the bundle still contains `submit-primary.http`, both membership JSON files, both pending/completed standby JSON files, and the per-node stdout/stderr logs.
3. **Expected:** the verifier would fail if any of these raw artifacts disappeared; passing proof requires retained evidence, not just a green exit code.

## Failure Signals

- `cargo test -p mesh-rt continuity -- --nocapture` loses one of the authority-specific tests or reports fewer than 28 passing continuity cases.
- `cluster-proof/tests` stops mentioning explicit continuity role enforcement or mirrored standby status truth.
- `cargo test -p meshc --test e2e_m043_s01 -- --nocapture` runs 0 tests, or the positive scenario passes without retained artifact files.
- Any retained membership/status JSON is missing `cluster_role`, `promotion_epoch`, or `replication_health`.
- Standby completed status shows an execution node other than the primary or a role/epoch that implies promotion happened automatically.

## Requirements Proved By This UAT

- R051 — proves the live-replication half of the disaster-continuity contract: primary request truth is mirrored onto standby with explicit role/epoch/health metadata and without app-authored DR logic.

## Not Proven By This UAT

- Explicit standby promotion after primary loss.
- Stale-primary fencing or deposed-primary rejoin behavior.
- The packaged same-image two-cluster operator rail or live Fly evidence.

## Notes for Tester

- The malformed-authority negative test intentionally prints panic text under `--nocapture`; that is expected Rust panic-hook noise, not a failed assertion.
- The retained artifact directory name includes a timestamped scenario suffix; if the verifier is re-run, use the newest `continuity-api-m043-primary-to-standby-*` bundle rather than assuming this exact timestamp persists.
- Treat `.tmp/m043-s01/verify/phase-report.txt` plus the copied JSON/log bundle as the authoritative slice evidence, not the console output alone.
