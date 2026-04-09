# S02: Runtime-Owned Declared Handler Execution — UAT

**Milestone:** M044
**Written:** 2026-03-29T23:27:16.614Z

# S02: Runtime-Owned Declared Handler Execution — UAT

## Preconditions
- Working tree rooted at `/Users/sn0w/Documents/dev/mesh-lang`
- Rust toolchain available
- No pre-existing assumption that `.tmp/m044-s02/verify/` is fresh; the UAT replay should recreate it

## Primary Acceptance Flow

### 1. Replay the assembled slice contract
1. Run `bash scripts/verify-m044-s02.sh`.
2. Open `.tmp/m044-s02/verify/status.txt`.
3. Open `.tmp/m044-s02/verify/current-phase.txt`.
4. Open `.tmp/m044-s02/verify/phase-report.txt`.
5. Expected:
   - the command exits 0
   - `status.txt` contains `ok`
   - `current-phase.txt` contains `complete`
   - `phase-report.txt` shows `passed` for `s01-contract`, `mesh-rt-staticlib`, `s02-metadata`, `s02-declared-work`, `s02-service`, `s02-cluster-proof`, `cluster-proof-build`, `cluster-proof-tests`, `hot-submit-selection-absence`, `hot-submit-dispatch-absence`, and `hot-status-legacy-absence`

### 2. Prove declared work is the only clustered work path
1. Run `cargo test -p meshc --test e2e_m044_s02 m044_s02_declared_work_ -- --nocapture`.
2. Open `.tmp/m044-s02/verify/03-s02-declared-work.test-count.log` after the assembled verifier replay.
3. Expected:
   - the named filter exits 0
   - the test-count log records `running-counts=[1]`
   - the emitted proof confirms manifest-declared work is registered while undeclared local helpers stay out of the declared runtime registry

### 3. Prove declared service handlers lower to runtime-safe wrappers without widening manifestless builds
1. Run `cargo test -p meshc --test e2e_m044_s02 m044_s02_service_ -- --nocapture`.
2. Open `.tmp/m044-s02/verify/04-s02-service.test-count.log` after the assembled verifier replay.
3. Expected:
   - the named filter exits 0
   - the test-count log records `running-counts=[2]`
   - the proof rail shows generated `__declared_service_call_*` / `__declared_service_cast_*` wrappers for declared services
   - manifestless service builds do not grow declared wrapper symbols

### 4. Prove cluster-proof dogfoods the runtime-owned declared-work path
1. Run `cargo test -p meshc --test e2e_m044_s02 m044_s02_cluster_proof_ -- --nocapture`.
2. Open `.tmp/m044-s02/verify/05-s02-cluster-proof.test-count.log`.
3. Open `.tmp/m044-s02/verify/05-s02-cluster-proof-artifacts.txt`.
4. Expected:
   - the named filter exits 0
   - the test-count log records `running-counts=[2]`
   - the artifact manifest points at a retained `.tmp/m044-s02/cluster-proof-runtime-owned-submit-*` bundle containing `scenario-meta.json`, `cluster-proof-build.log`, `cluster-proof-tests.log`, and a `work_continuity.mpl` snapshot

### 5. Prove the app still builds and its package tests still pass on the shipped surface
1. Run `cargo run -q -p meshc -- build cluster-proof`.
2. Run `cargo run -q -p meshc -- test cluster-proof/tests`.
3. Expected:
   - both commands exit 0
   - the assembled verifier logs for phases `06-cluster-proof-build.log` and `07-cluster-proof-tests.log` show successful replay

## Edge Cases

### A. Undeclared code must stay local
1. Re-run `cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture`.
2. Expected:
   - the metadata rail still passes
   - undeclared public helpers stay absent from the execution plan
   - manifestless builds still succeed without any clustered registration surface

### B. The new submit/status hot path must not regress back to app-owned placement/dispatch
1. Re-run `bash scripts/verify-m044-s02.sh`.
2. Open `.tmp/m044-s02/verify/08-hot-submit-selection-absence.log`, `.tmp/m044-s02/verify/09-hot-submit-dispatch-absence.log`, and `.tmp/m044-s02/verify/10-hot-status-legacy-absence.log`.
3. Expected:
   - all three checks pass
   - the logs confirm the declared-runtime `handle_valid_submit`, `created_submit_response`, and `handle_valid_status` ranges in `cluster-proof/work_continuity.mpl` do not contain `current_target_selection(...)`, `submit_from_selection(...)`, `dispatch_work(...)`, or `Node.spawn(...)`

## Failure Signals
- `bash scripts/verify-m044-s02.sh` exits non-zero
- `.tmp/m044-s02/verify/status.txt` is not `ok`
- `.tmp/m044-s02/verify/current-phase.txt` is not `complete`
- any named test-count log is missing or reports zero tests
- `cluster-proof` build/package-test phases fail
- the hot-path absence logs report legacy placement/dispatch literals inside the declared-runtime submit/status ranges

## UAT Verdict
- Pass when the assembled verifier is green and the retained artifacts/logs match the expected declared-work, declared-service, and cluster-proof dogfood boundaries.
- Fail if any phase regresses to app-owned placement/dispatch, undeclared code enters the clustered path, or the named proof rails disappear behind a zero-test filter.
