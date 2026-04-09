# M046/S02 closeout assessment

## Status
- Slice closeout is **not complete yet**.
- Runtime fix landed locally in `compiler/mesh-rt/src/dist/node.rs`.
- Most S02 verification rails are now green, including the previously failing two-node route-free startup proof.
- One retained prerequisite rail is still red during the final assembled replay: `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture`.

## What changed during closeout
- Fixed the real S02 runtime race by keeping the route-free keepalive actor on standby nodes but skipping startup-work actor spawn when local continuity authority is `standby`.
- Added a runtime diagnostic `startup_skipped` for that standby path.
- Added a new runtime unit test covering the standby-skip contract.
- Serialized the startup-work registry tests with a local mutex because they share global runtime registries and were racing under parallel unit-test execution.

## Current verification state
### Green
- `cargo test -p meshc --test e2e_m046_s02 m046_s02_codegen_ -- --nocapture && cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture`
- `cargo test -p mesh-rt startup_work_ -- --nocapture`
- `cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_ -- --nocapture`
- `cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_tiny_route_free_startup_dedupes_on_two_nodes -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_status_json_reports_runtime_truth_and_auth_failures_fail_closed -- --nocapture` was implicitly green inside the broader `m044_s03_operator_` runs.
- `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture`

### Still red / unresolved
- Final replay failed in the retained prerequisite rail:
  - `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture`
- Failure symptom from the last run:
  - `m044_s03_operator_continuity_and_diagnostics_report_runtime_truth`
  - panic text: `diagnostic transition degraded for m044-s03-key-35 did not appear within 20s; last diagnostics: None`

## Why this blocks slice completion
The slice contract explicitly replays retained M044 operator rails as part of the acceptance surface. Until that retained rail is green in the same closeout run, it is not honest to call `gsd_complete_slice`.

## Resume here
1. Start from the failing retained command only:
   - `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture`
2. Compare the failed run against the retained artifact directories under `.tmp/m044-s03/operator-continuity-diagnostics-*` and the matching `cluster-diagnostics*` / `primary.combined.log` / `standby.combined.log` files.
3. Decide whether the remaining red is:
   - a harness race in `compiler/meshc/tests/e2e_m044_s03.rs::wait_for_diagnostic_transition(...)`, or
   - a real regression in retained operator diagnostics after the new standby-startup behavior.
4. If the retained rail goes green, rerun the final slice replay command and only then write `S02-SUMMARY.md`, `S02-UAT.md`, update PROJECT/KNOWLEDGE if needed, and call `gsd_complete_slice`.

## Touched files in this closeout attempt
- `compiler/mesh-rt/src/dist/node.rs`

## Notes for next agent
- Do **not** reopen the S02 route-free startup fix first; the focused failing S02 rail is already green after the standby-skip change.
- The unresolved blocker is the retained M044 operator replay, not the new startup-work path itself.
- Because `startup_work_` tests mutate global registries, keep the new local test mutex in place when extending that test cluster.