---
estimated_steps: 1
estimated_files: 4
skills_used: []
---

# T05: Add the opt-in SQLite Todo starter template

Extend `meshc init` with an explicit Todo template selector (for example `--template todo-api`) without changing the corrected route-free `--clustered` default. Generate a small multi-file starter with SQLite persistence, several HTTP routes, actor-backed rate limiting, clustered work on the no-ceremony `@cluster` contract, a README, and a standalone Dockerfile/.dockerignore that use the public Mesh install/build story rather than repo-only assumptions.

## Inputs

- `compiler/meshc/src/main.rs`
- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/mesh-pkg/src/lib.rs`
- `Route-free clustered scaffold output from T04`

## Expected Output

- `Opt-in generated Todo starter template with SQLite, HTTP routes, rate limiting, README, Dockerfile, and .dockerignore`
- `CLI/tooling coverage proving template selection and generated files are correct`

## Verification

cargo test -p mesh-pkg m047_s05 -- --nocapture && cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture
