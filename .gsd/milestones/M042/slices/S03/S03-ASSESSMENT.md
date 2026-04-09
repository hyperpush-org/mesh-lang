# S03 closeout assessment — blocked

## Verdict
S03 is **not complete yet**. Do **not** call `gsd_complete_slice` from this handoff state.

## What is green
- `cargo test -p mesh-rt continuity -- --nocapture`
- `cargo run -q -p meshc -- test cluster-proof/tests`
- `cargo test -p meshc --test e2e_m042_s03 -- --nocapture`
- `bash scripts/verify-m042-s03.sh` passed once after the wrapper was changed to replay the full `e2e_m042_s03` target once and then copy the owner-loss/rejoin artifacts from that shared run.

## What blocked the closeout
A final clean evidence rerun exposed a **real prerequisite regression** inside `bash scripts/verify-m042-s03.sh`:
- The wrapper replays `bash scripts/verify-m042-s02.sh`.
- That S02 replay then failed in `continuity_api_two_node_local_owner_mirrors_status_between_owner_and_replica`.
- Failure shape: the healthy mirrored replica status unexpectedly flipped to `owner_lost` on the replica even though the owner was still up, so the replica never satisfied the expected pending mirrored status.
- Evidence bundle/logs:
  - `.tmp/m042-s03/verify/03-s02-contract.log`
  - `.tmp/m042-s02/verify/06-s02-mirrored-admission.log`
  - failing runtime symptoms in those logs show `transition=owner_lost` on the replica during a supposedly healthy mirrored path.

## Most likely root cause to resume from
The strongest current hypothesis is in `compiler/mesh-rt/src/dist/node.rs`:
- `cleanup_session(remote_name)` removes sessions **by remote name only**.
- Duplicate-connection tiebreaker paths (`register_session(...)` / handshake rejection) can leave a losing duplicate connection still calling `cleanup_session(remote_name)`.
- That cleanup can tear down the healthy winning session for the same node name and then fire `handle_node_disconnect(...)`, which now marks continuity records `owner_lost`.
- This fits the failing S02 logs: duplicate/handshake churn, then false `owner_lost` on a healthy mirrored path.

## Code already changed in this unit
- `scripts/verify-m042-s03.sh` now replays the shared `cargo test -p meshc --test e2e_m042_s03 -- --nocapture` target once, asserts both named S03 scenarios passed in that shared run, and copies the owner-loss/rejoin artifact bundles from the same replay. That wrapper change is still worth keeping.
- `cluster-proof/work.mpl`, `compiler/mesh-rt/src/dist/continuity.rs`, `compiler/mesh-rt/src/dist/node.rs`, and `compiler/meshc/tests/e2e_m042_s03.rs` remain in the state that made the direct S03 target green.

## Resume steps
1. Start in `compiler/mesh-rt/src/dist/node.rs` around `cleanup_session(...)`, `handle_node_disconnect(...)`, and the duplicate-connection path in `register_session(...)`.
2. Fix session cleanup so a losing duplicate connection cannot remove the healthy winning session or fire false node-loss continuity transitions.
3. Re-run, in this order:
   - `cargo test -p meshc --test e2e_m042_s02 continuity_api_two_node_local_owner_mirrors_status_between_owner_and_replica -- --nocapture`
   - `bash scripts/verify-m042-s02.sh`
   - `cargo test -p meshc --test e2e_m042_s03 -- --nocapture`
   - `bash scripts/verify-m042-s03.sh`
4. Only if all of those pass again should the next unit write `S03-SUMMARY.md`, `S03-UAT.md`, and call `gsd_complete_slice`.

## Truthful current status
- Slice checkbox should remain **open**.
- Requirement validation should remain **unclosed** until the S02 prerequisite regression is fixed and the full S03 wrapper is green on a fresh replay.
