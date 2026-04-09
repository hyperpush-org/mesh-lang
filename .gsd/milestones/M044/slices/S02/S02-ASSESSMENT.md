# S02 closeout assessment — incomplete

## Status
- Slice **not complete**.
- I ran the authoritative slice rail: `bash scripts/verify-m044-s02.sh`.
- It failed at the first missing S02 execution surface:
  - phase: `s02-declared-work`
  - reason: named test filter ran 0 tests
  - evidence: `.tmp/m044-s02/verify/03-s02-declared-work.test-count.log`

## Verified failure
```text
==> cargo test -p meshc --test e2e_m044_s02 m044_s02_declared_work_ -- --nocapture
verification drift: named test filter ran 0 tests or produced malformed output
03-s02-declared-work: test filter ran 0 tests
```

## Current repo truth at stop point

### 1. The verifier failure is real, not harness drift
- `compiler/meshc/tests/e2e_m044_s02.rs` currently contains only the `m044_s02_metadata_*` tests.
- There are **no** `m044_s02_declared_work_*`, `m044_s02_service_*`, or `m044_s02_cluster_proof_*` tests yet.

### 2. `meshc` still computes clustered execution metadata and drops it
- `compiler/meshc/src/main.rs`:
  - `PreparedBuild` includes `clustered_execution_plan`.
  - `prepare_project_build(...)` populates it.
  - `build(...)` immediately discards it with:
    - `let _clustered_execution_plan = &prepared.clustered_execution_plan;`
- So the S01 manifest/executable metadata exists, but codegen/runtime still do not consume it.

### 3. `cluster-proof` still uses the pre-S02 app-owned hot path
- `cluster-proof/mesh.toml` still declares HTTP ingress handlers as clustered work:
  - `WorkContinuity.handle_work_submit`
  - `WorkContinuity.handle_work_status`
  - `WorkContinuity.handle_promote`
  - `WorkLegacy.handle_work_probe`
- `cluster-proof/work_continuity.mpl` still contains the old submit/status seams the verifier expects to disappear:
  - `current_target_selection(...)`
  - `submit_from_selection(...)`
  - `dispatch_work(...)`
  - `spawn_remote_work(...)`
  - `spawn_local_work(...)`
  - `Node.spawn(...)`
- That means even after adding the missing named tests, the later absence phases in `scripts/verify-m044-s02.sh` will still fail until the hot path is rewritten.

### 4. The runtime/compiler seam is only partially there
- `mesh-pkg` + `meshc` + `mesh-lsp` already carry validated clustered executable metadata.
- The runtime/codegen still need the actual execution path:
  - declared work registration/dispatch
  - declared service wrapper registration/lowering
  - `cluster-proof` rewrite onto the runtime-owned path

## Best resume point for the next agent
1. Implement real consumption of `PreparedBuild.clustered_execution_plan` in compiler/codegen/runtime instead of dropping it in `build(...)`.
2. Add the missing named tests in `compiler/meshc/tests/e2e_m044_s02.rs`:
   - `m044_s02_declared_work_*`
   - `m044_s02_service_*`
   - `m044_s02_cluster_proof_*`
3. Retarget `cluster-proof/mesh.toml` away from HTTP ingress handlers.
4. Remove the old app-owned submit/status routing/dispatch flow from `cluster-proof/work_continuity.mpl` so the three absence checks in `scripts/verify-m044-s02.sh` can pass.
5. Re-run `bash scripts/verify-m044-s02.sh` as the authoritative slice rail.

## Files inspected during this closeout attempt
- `scripts/verify-m044-s02.sh`
- `compiler/meshc/tests/e2e_m044_s02.rs`
- `compiler/meshc/src/main.rs`
- `cluster-proof/mesh.toml`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/work.mpl`
- `cluster-proof/work_legacy.mpl`
- `compiler/mesh-typeck/src/lib.rs`
- `compiler/mesh-codegen/src/codegen/mod.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-rt/src/lib.rs`
- `compiler/mesh-rt/src/dist/node.rs`

## Local state
- No source files were modified in this closeout attempt.
- No slice summary/UAT was written because the slice acceptance rail is still red.
