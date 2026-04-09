---
estimated_steps: 4
estimated_files: 6
skills_used: []
---

# T04: Retire the remaining app/user-owned timing seam from tiny-cluster failover observability

1. Audit `tiny-cluster/`, the S03 e2e rail, the local verifier, and any slice-owned runbook text for surviving `Env.get_int(...)`, `Timer.sleep(...)`, `TINY_CLUSTER_*DELAY*`, or user-directed `MESH_STARTUP_WORK_DELAY_MS` guidance.
2. Replace any remaining app/package code or user-facing setup requirement with a Mesh-owned runtime/proof seam that stays invisible to example code while still giving the failover rail an observable pending window.
3. Extend negative assertions so the package smoke rail, e2e contract checks, and verifier fail closed if tiny-cluster source or slice-owned guidance reintroduce package-owned timing helpers.
4. Re-run the focused runtime, package, failover, and direct verifier commands and retain a fresh `.tmp/m046-s03/...` evidence bundle proving the route-free failover story still works without app/user-owned timing seams.

## Inputs

- `.gsd/OVERRIDES.md`
- `.gsd/milestones/M046/slices/S03/S03-PLAN.md`
- `tiny-cluster/work.mpl`
- `tiny-cluster/tests/work.test.mpl`
- `tiny-cluster/README.md`
- `compiler/meshc/tests/e2e_m046_s03.rs`
- `scripts/verify-m046-s03.sh`

## Expected Output

- Updated `.gsd/milestones/M046/slices/S03/S03-PLAN.md` with a follow-up override task
- Fresh verification commands and evidence proving `tiny-cluster/` no longer depends on app/user-owned timing helpers
- Retained `.tmp/m046-s03/...` bundle showing route-free failover truth still works

## Verification

cargo test -p mesh-rt startup_work_ -- --nocapture && cargo run -q -p meshc -- build tiny-cluster && cargo run -q -p meshc -- test tiny-cluster/tests && cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_failover_ -- --nocapture && bash scripts/verify-m046-s03.sh
