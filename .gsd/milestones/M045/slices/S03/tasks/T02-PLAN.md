---
estimated_steps: 3
estimated_files: 6
skills_used:
  - test
  - debug-like-expert
  - rust-best-practices
---

# T02: Harden the scaffold contract and keep any timing stabilization below the app surface

Keep the scaffold docs-grade and runtime-owned while S03 adds destructive failover proof. This task hardens the generated app and README rails against regression and reserves any timing stabilization to the smallest runtime or codegen seam if T01 proves harness-only observation is not deterministic.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Clustered scaffold generator and README contract in `compiler/mesh-pkg/src/scaffold.rs` | Fail source-contract tests loudly if the generated app grows app-owned failover, promotion, or status logic. | N/A — generation is synchronous and bounded. | Reject stale README/source assertions that would let proof-app-specific or manual failover text slip back in. |
| Optional runtime/codegen timing seam near declared-work completion | Keep the change below the app surface and fail tests if it leaks a scaffold-specific env knob or manual control path. | Bound any new wait/poll expectations in tests so timing drift produces evidence instead of hangs. | Treat mismatched attempt/phase truth as a runtime contract failure, not as a scaffold excuse. |

## Load Profile

- **Shared resources**: scaffold templates, temp-project init/build rails, and any runtime completion path touched for timing stabilization.
- **Per-operation cost**: one scaffold generation plus focused contract/e2e replays.
- **10x breakpoint**: stale source assertions and accidental app-surface growth fail before performance does; the task must pin the public surface tightly.

## Negative Tests

- **Malformed inputs**: generated `main.mpl` or `README.md` reintroducing `/promote`, `Continuity.promote`, `Continuity.mark_completed`, app-owned status helpers, `owner_node`/`replica_node` surfacing, or a scaffold-specific delay knob.
- **Error paths**: T01 shows the pending window is nondeterministic and the fallback stabilization leaks into the scaffold source shape or CLI contract.
- **Boundary conditions**: the scaffold still stays tiny after adding failover proof, and the README keeps `meshc cluster diagnostics` alongside the existing status/continuity contract.

## Steps

1. Tighten `compiler/mesh-pkg/src/scaffold.rs`, `compiler/meshc/tests/tooling_e2e.rs`, and scaffold contract assertions so generated clustered apps still expose only `Node.start_from_env()`, `/health`, the submit route, and runtime CLI surfaces including `meshc cluster diagnostics`.
2. Add explicit failover-absence assertions in the scaffold contract rails (no `/promote`, no `Continuity.promote`, no app-owned status/authority helpers, no `CLUSTER_PROOF_*`, and no app-specific delay knob), preferably with a named `m045_s03_scaffold_` assertion surface in `compiler/meshc/tests/e2e_m045_s03.rs`.
3. Only if T01's retained evidence shows harness-only timing is not deterministic, add the smallest runtime-owned pending-window seam beneath the app surface and extend the contract tests to prove the scaffold source shape does not grow.

## Must-Haves

- [ ] The generated scaffold remains visibly small and runtime-owned while S03 lands.
- [ ] README and contract rails explicitly protect `meshc cluster diagnostics` and the absence of manual failover surfaces.
- [ ] Any timing stabilization happens below the app surface and is proven not to leak example-owned distributed logic back into the scaffold.

## Verification

- `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`
- `cargo test -p meshc --test e2e_m045_s02 m045_s02_scaffold_runtime_completion_ -- --nocapture`
- `cargo test -p meshc --test e2e_m045_s03 m045_s03_scaffold_ -- --nocapture`

## Inputs

- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `compiler/meshc/tests/e2e_m045_s02.rs`
- `compiler/meshc/tests/e2e_m045_s03.rs`
- `compiler/mesh-codegen/src/codegen/expr.rs`
- `compiler/mesh-rt/src/dist/node.rs`

## Expected Output

- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `compiler/meshc/tests/e2e_m045_s02.rs`
- `compiler/meshc/tests/e2e_m045_s03.rs`
- `compiler/mesh-codegen/src/codegen/expr.rs`
- `compiler/mesh-rt/src/dist/node.rs`

## Verification

cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture
cargo test -p meshc --test e2e_m045_s02 m045_s02_scaffold_runtime_completion_ -- --nocapture
cargo test -p meshc --test e2e_m045_s03 m045_s03_scaffold_ -- --nocapture
