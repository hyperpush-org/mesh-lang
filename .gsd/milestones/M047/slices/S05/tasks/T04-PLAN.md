---
estimated_steps: 1
estimated_files: 6
skills_used: []
---

# T04: Rebaseline route-free clustered scaffold and examples on the corrected contract

Once T03 lands, rewrite the existing route-free clustered public surfaces to dogfood the corrected no-ceremony `@cluster` model. Update `meshc init --clustered`, `tiny-cluster/`, `cluster-proof/`, and the shared route-free harnesses/tests so they stop teaching `execute_declared_work(request_key, attempt_id)` or `Work.execute_declared_work` as the public source contract while preserving runtime-owned `meshc cluster status|continuity|diagnostics` inspection and retained-artifact behavior.

## Inputs

- `.gsd/milestones/M047/slices/S05/tasks/T02-SUMMARY.md`
- `compiler/mesh-pkg/src/scaffold.rs`
- `tiny-cluster/`
- `cluster-proof/`

## Expected Output

- `Route-free scaffold/examples rewritten to the no-ceremony `@cluster` contract`
- `Updated route-free harnesses and exact-string rails that still localize failures honestly`

## Verification

cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture && cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture && cargo run -q -p meshc -- test tiny-cluster/tests && cargo run -q -p meshc -- test cluster-proof/tests && cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture

## Observability Impact

- Signals added/changed: retained `.tmp/m047-s05/verify/*` markers, generated-app stdout/stderr archives, HTTP response captures, SQLite file artifacts, and Docker build logs.
- How a future agent inspects this: start with `.tmp/m047-s05/verify/phase-report.txt` and `latest-proof-bundle.txt`, then drill into the generated-project e2e artifact directory.
- Failure state exposed: server readiness, schema init, HTTP status/body mismatches, Docker build failures, and wording drift all localize to a named phase and retained log path.
