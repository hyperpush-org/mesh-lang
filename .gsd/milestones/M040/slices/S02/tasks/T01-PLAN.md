---
estimated_steps: 4
estimated_files: 5
skills_used:
  - debug-like-expert
  - rust-best-practices
  - test
---

# T01: Retire the clustered keyed startup crash and prove two-node readiness again

**Slice:** S02 — Replica-Backed Admission, Fail-Closed Policy, and Owner-Loss Convergence
**Milestone:** M040

## Description

S02 cannot honestly claim replica-backed continuity until clustered keyed `cluster-proof` starts again. Right now the exact M039/S03 rejoin proof crashes before pre-loss membership is ready, with both nodes aborting at `compiler/mesh-rt/src/string.rs:171` (`mesh_string_length`) immediately after `Node started` and before `work services ready`.

Reproduce that exact red path first, then trace the clustered startup flow through `start_cluster()` and `start_work_services()`. Fix the narrowest real root cause so a two-node keyed cluster reaches `work services ready` again. If the repair lands in Rust runtime code, keep it tightly scoped, add a safety comment explaining why the fix is correct, and do not turn this task into a general string/runtime rewrite.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/meshc/tests/e2e_m039_s03.rs` rejoin repro | keep the exact failing artifact/log paths and use them to localize the crash instead of guessing | treat missing membership readiness as a startup failure, not as network flake | fail the task; a passing harness with missing logs or missing `work services ready` output is not trustworthy |
| `compiler/mesh-rt/src/string.rs` / clustered startup path | make a narrow fix at the actual crash boundary and document the safety reason | do not add retries that hide the abort; startup must become correct, not slower | reject any workaround that converts the crash into undefined behavior or silent data corruption |
| `cluster-proof` startup wiring | keep the standalone keyed S01 path intact while fixing clustered startup | keep startup bounded; if the process hangs before readiness, treat that as red | do not synthesize fake readiness before the work services are actually registered |

## Load Profile

- **Shared resources**: clustered startup threads, global name registration, per-node string allocations during service registration, and membership polling in the e2e harness.
- **Per-operation cost**: one cluster start per node, one work-service registration path, and repeated readiness polling during the regression test.
- **10x breakpoint**: repeated node joins/rejoins would amplify pointer misuse or startup ordering bugs before steady-state request load matters.

## Negative Tests

- **Malformed inputs**: cluster-mode startup with empty/invalid identity-derived strings must fail with explicit config/runtime errors instead of a pointer abort.
- **Error paths**: the exact rejoin test that currently crashes, plus the first two-node startup before any `/work` traffic.
- **Boundary conditions**: first cluster boot, second node join, and same-identity rejoin after restart.

## Steps

1. Reproduce `cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_rejoins_and_routes_to_peer_again_without_manual_repair -- --nocapture` and confirm the crash still occurs before `work services ready`.
2. Trace the startup path through `cluster-proof/main.mpl`, `cluster-proof/work.mpl`, and the clustered runtime string/global-registration seam to isolate the actual crash trigger.
3. Apply the narrowest fix that makes two-node keyed startup safe again, keeping any Rust runtime edit safety-commented and tightly scoped.
4. Re-run the exact rejoin proof and keep the failure surface diagnosable through startup logs and preserved artifact paths.

## Must-Haves

- [ ] The exact M039/S03 rejoin repro goes red before the fix and green after the fix.
- [ ] Both clustered nodes reach `work services ready` instead of aborting in `mesh_string_length`.
- [ ] The repair stays narrow and does not regress the standalone keyed S01 rail.

## Verification

- Re-run the exact failing clustered rejoin proof after the fix.
- `cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_rejoins_and_routes_to_peer_again_without_manual_repair -- --nocapture`
- `cargo run -q -p meshc -- build cluster-proof`

## Observability Impact

- Signals added/changed: startup should emit `work services ready` after the crash point instead of aborting with a misaligned-pointer panic.
- How a future agent inspects this: use the named M039/S03 e2e command and inspect the per-node stdout/stderr paths already emitted by the harness.
- Failure state exposed: clustered startup failures stay localized to the specific startup seam instead of appearing as a generic membership timeout.

## Inputs

- `cluster-proof/main.mpl` — clustered app startup and `start_work_services()` call site.
- `cluster-proof/work.mpl` — clustered work-service registration/startup logic.
- `compiler/mesh-rt/src/string.rs` — crash site currently aborting in `mesh_string_length`.
- `compiler/mesh-rt/src/dist/global.rs` — likely clustered registration/discovery neighbor of the failing path.
- `compiler/meshc/tests/e2e_m039_s03.rs` — exact red→green rejoin repro and artifact source.

## Expected Output

- `cluster-proof/main.mpl` — any startup-order or readiness fix needed on the app side.
- `cluster-proof/work.mpl` — narrowed clustered work-service startup logic if the fix lands above the runtime.
- `compiler/mesh-rt/src/string.rs` — narrow runtime safety fix if the root cause is the string boundary.
- `compiler/meshc/tests/e2e_m039_s03.rs` — tightened regression assertions only if the existing harness needs to pin the fixed failure mode.
