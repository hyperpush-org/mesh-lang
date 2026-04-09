---
estimated_steps: 1
estimated_files: 9
skills_used: []
---

# T06: Prove the Todo starter end to end, remove the stale `execute_declared_work(...)` example shape, and refresh public wording

Add the generated-project helper, named `e2e_m047_s05` rail, and assembled verifier for the Todo starter. Generate the template, build and boot it with retained logs/SQLite artifacts, exercise CRUD + rate-limit + restart-persistence behavior through real HTTP requests, build the Docker image, and rewrite the remaining public clustered surfaces so `tiny-cluster/`, `cluster-proof/`, scaffold output, and targeted docs/help all dogfood ordinary `@cluster` function names like `add()` or domain-specific verbs instead of `execute_declared_work(...)`, while still staying honest that `HTTP.clustered(...)` is only planned future work.

## Inputs

- `Generated Todo starter from T05`
- `compiler/meshc/tests/support/m046_route_free.rs`
- `tiny-cluster/work.mpl`
- `cluster-proof/work.mpl`
- `README.md`
- `website/docs/docs/getting-started/clustered-example/index.md`

## Expected Output

- `Named e2e + assembled verifier bundle for the generated Todo starter`
- `Updated route-free example/scaffold/public wording that replaces the public `execute_declared_work(...)` shape with ordinary `@cluster` function names`

## Verification

bash scripts/verify-m047-s04.sh && cargo test -p meshc --test e2e_m047_s05 -- --nocapture && bash scripts/verify-m047-s05.sh && npm --prefix website run build
