---
estimated_steps: 3
estimated_files: 5
skills_used:
  - test
  - debug-like-expert
  - rust-best-practices
---

# T02: Move cluster-proof declared work into Work and remove wrapper-era completion glue

Align `cluster-proof` with the scaffold-first runtime-owned execution shape: the manifest-declared handler should live in `Work`, while `work_continuity` stays a thin HTTP/status translator over runtime `Continuity`.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Runtime-owned declared-work completion on the `cluster-proof` manifest target | Stop with build/test or failover-rail evidence; do not reintroduce `Continuity.mark_completed(...)` as a fallback. | Bound the destructive failover replay and retain the last continuity/log outputs. | Treat contradictory continuity/status payloads as real runtime drift, not as a reason to keep the wrapper seam. |
| Manifest/codegen registration for `Work.execute_declared_work` | Fail closed on missing-handler build/runtime errors; do not leave the manifest pointing at `WorkContinuity.execute_declared_work`. | N/A — registration drift should surface during build or targeted runtime proof. | Treat malformed handler/record truth as a contract failure, not a retryable warning. |

## Load Profile

- **Shared resources**: `cluster-proof` build output, local ports, node processes, existing same-image failover rails, and retained `.tmp` evidence.
- **Per-operation cost**: one package build/test replay plus one destructive runtime failover replay.
- **10x breakpoint**: runtime promotion/recovery timing and stale handler registration fail before throughput; the task must keep enough logs/continuity output to explain which seam drifted.

## Negative Tests

- **Malformed inputs**: missing declared-handler registration, malformed continuity/status JSON, and stale manifest target strings.
- **Error paths**: declared work returns but the record never completes through the runtime path, or automatic promotion/recovery regresses after the target move.
- **Boundary conditions**: duplicate submit, owner-loss recovery, and stale-primary rejoin still behave the same with the slimmer target shape.

## Steps

1. Move the manifest target from `WorkContinuity.execute_declared_work` to `Work.execute_declared_work`, implement the declared handler in `cluster-proof/work.mpl`, and keep only package-local execution behavior there (including any retained proof-only delay/logging the destructive rail still needs).
2. Remove manual completion and wrapper-era leftovers from `cluster-proof/work_continuity.mpl` (`Continuity.mark_completed(...)`, completion-failure logging, dead actor execution path, and the old target helper) so that file only owns keyed submit/status HTTP translation.
3. Update package/runtime assertions so the slimmer target shape still satisfies the existing same-image failover proof.

## Must-Haves

- [ ] `cluster-proof/mesh.toml` declares `Work.execute_declared_work`, not `WorkContinuity.execute_declared_work`.
- [ ] `cluster-proof/work_continuity.mpl` no longer manually closes continuity records or owns the actual declared-work handler.
- [ ] The existing same-image failover rail still passes on the runtime-owned completion path.

## Inputs

- `cluster-proof/mesh.toml`
- `cluster-proof/work.mpl`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/tests/work.test.mpl`
- `compiler/meshc/tests/e2e_m044_s04.rs`

## Expected Output

- `cluster-proof/mesh.toml`
- `cluster-proof/work.mpl`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/tests/work.test.mpl`
- `compiler/meshc/tests/e2e_m044_s04.rs`

## Verification

cargo run -q -p meshc -- build cluster-proof
cargo run -q -p meshc -- test cluster-proof/tests
cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture

## Observability Impact

- Signals added/changed: the declared-work execution log and continuity completion now come from `Work.execute_declared_work` plus runtime-owned completion rather than `work_continuity`'s manual completion helper.
- How a future agent inspects this: replay `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture` and inspect the retained continuity JSON and node logs under the existing failover artifact roots.
- Failure state exposed: missing declared-handler registration, lost completion, or owner-loss recovery drift remains visible through continuity/status payloads and node stderr logs.
