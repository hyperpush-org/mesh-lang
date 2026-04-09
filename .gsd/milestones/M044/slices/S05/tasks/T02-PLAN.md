---
estimated_steps: 24
estimated_files: 6
skills_used: []
---

# T02: Delete the legacy explicit clustering path from cluster-proof

Now that the public bootstrap contract is stable, finish the dogfood rewrite by removing the old explicit clustering probe path and the dead manual-era helpers from `cluster-proof`. After this task, the app should expose only the keyed runtime-owned submit/status path, with no `WorkLegacy` route or app-owned placement/dispatch logic left in code.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Keyed continuity submit/status path in `cluster-proof/work_continuity.mpl` | Fail the package/e2e rail and keep the keyed route red rather than falling back to the deleted legacy path. | Bound status polling in tests and surface whether submit or completion stalled. | Reject malformed continuity records or request payloads instead of inventing partial JSON. |
| Remaining keyed helper module in `cluster-proof/work.mpl` | Keep request/status models and validation truthful even as legacy placement helpers are removed. | N/A — local helper path. | Reject malformed request keys and payloads explicitly. |
| Package tests and new live `e2e_m044_s05` coverage | Fail closed on missing 404/absence checks, stale helper literals, or regressions in duplicate/conflict status behavior. | Bound live server waits and surface the exact rail that still depended on the legacy probe. | Reject malformed responses instead of letting the cleanup proof pass on happy-path only behavior. |

## Load Profile

- **Shared resources**: runtime continuity records, async declared-work execution, and package/e2e status polling.
- **Per-operation cost**: one keyed submit plus duplicate/conflict/status lookups; the cleanup risk is correctness drift, not throughput.
- **10x breakpoint**: repeated same-key submits and pending-status polling will stress the keyed path first if the legacy cleanup accidentally drops the runtime-owned behavior.

## Negative Tests

- **Malformed inputs**: invalid request keys, malformed submit JSON, and status lookups for missing request keys.
- **Error paths**: duplicate submit with a rejected record, conflicting same-key submit, and keyed status while authority is unavailable.
- **Boundary conditions**: `GET /work` returns 404 / is not mounted, `POST /work` still creates and reuses keyed records, and `cluster-proof/work_legacy.mpl` is gone from the tree.

## Steps

1. Remove `WorkLegacy` route wiring from `cluster-proof/main.mpl` and delete `cluster-proof/work_legacy.mpl`, keeping only the keyed `POST /work` / `GET /work/:request_key` surfaces mounted.
2. Shrink `cluster-proof/work.mpl` to keyed request/status models and validation helpers only, or extract the surviving keyed helpers into a smaller module, so `TargetSelection`, canonical placement, and legacy probe utilities disappear from app code.
3. Delete dead manual/legacy helpers from `cluster-proof/work_continuity.mpl` (`promotion_response_status_code`, `log_promotion*`, `dispatch_work`, `run_legacy_probe_record`, `submit_from_selection`) and update `cluster-proof/tests/work.test.mpl` plus `compiler/meshc/tests/e2e_m044_s05.rs` to assert the keyed runtime-owned path still works and the legacy route is absent.
4. Add fail-closed absence checks so the rewrite cannot quietly reintroduce `WorkLegacy`, `handle_work_probe`, or app-owned placement/dispatch helpers after the cleanup lands.

## Must-Haves

- [ ] `cluster-proof` exposes only the keyed runtime-owned submit/status surface; the legacy `GET /work` probe is gone.
- [ ] `WorkLegacy`, `TargetSelection`, and the manual-era promotion/dispatch helpers are removed from the proof app code, not merely left dead.
- [ ] Package tests and the S05 live e2e rail prove keyed submit/status behavior and the absence of the old explicit path together.

## Inputs

- `cluster-proof/main.mpl`
- `cluster-proof/work.mpl`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/work_legacy.mpl`
- `cluster-proof/tests/work.test.mpl`
- `compiler/meshc/tests/e2e_m044_s05.rs`

## Expected Output

- `cluster-proof/main.mpl`
- `cluster-proof/work.mpl`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/work_legacy.mpl`
- `cluster-proof/tests/work.test.mpl`
- `compiler/meshc/tests/e2e_m044_s05.rs`

## Verification

cargo run -q -p meshc -- build cluster-proof
cargo run -q -p meshc -- test cluster-proof/tests
cargo test -p meshc --test e2e_m044_s05 m044_s05_legacy_cleanup_ -- --nocapture
test ! -e cluster-proof/work_legacy.mpl

## Observability Impact

- Signals added/changed: keyed submit/status logs remain the only `cluster-proof` runtime work signals, and the new e2e rail should retain a concrete 404/absence artifact for `GET /work`.
- How a future agent inspects this: rerun the named `m044_s05_legacy_cleanup_` filter and package tests, then inspect the retained HTTP artifacts/logs for the missing legacy route and keyed-path behavior.
- Failure state exposed: leftover legacy route wiring, stale helper literals in source, or regressions in duplicate/conflict/status truth on the keyed path.
