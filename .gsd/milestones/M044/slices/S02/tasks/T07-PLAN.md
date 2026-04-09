---
estimated_steps: 4
estimated_files: 8
skills_used: []
---

# T07: Recorded that T07 remains blocked by the missing declared-handler execution substrate; `cluster-proof` still cannot be rewritten honestly.

1. Retarget cluster-proof declarations from HTTP ingress handlers to the real declared work/service targets or generated wrappers introduced by T05/T06.
2. Replace the new submit/status hot path in cluster-proof/work_continuity.mpl so it calls the runtime-owned declared execution/status surfaces instead of computing keyed placement or direct Node.spawn(...) dispatch in Mesh code.
3. Keep HTTP parsing/JSON shaping local and confine any remaining current_target_selection(...) or Node.spawn(...) logic to explicitly legacy proof surfaces only.
4. Add m044_s02_cluster_proof_ coverage and update cluster-proof package tests/build expectations around the new boundary.

## Inputs

- `.gsd/milestones/M044/slices/S02/tasks/T04-SUMMARY.md`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/work.mpl`
- `cluster-proof/mesh.toml`

## Expected Output

- `cluster-proof no longer declares HTTP ingress handlers as clustered work on the new path`
- `The new submit/status hot path no longer computes current_target_selection(...) or direct Node.spawn(...) dispatch`
- `m044_s02_cluster_proof_ tests exist and pass alongside green cluster-proof build/package tests`

## Verification

cargo test -p meshc --test e2e_m044_s02 m044_s02_cluster_proof_ -- --nocapture
cargo run -q -p meshc -- build cluster-proof
cargo run -q -p meshc -- test cluster-proof/tests
