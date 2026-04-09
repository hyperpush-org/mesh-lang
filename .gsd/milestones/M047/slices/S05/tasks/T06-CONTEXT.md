# T06 handoff

## Landed

- `compiler/mesh-codegen/src/declared.rs::generate_declared_work_wrapper(...)` now supports ordinary zero-arg `@cluster` work by accepting hidden `(request_key, attempt_id)` metadata in the generated wrapper body while calling the user function with no public args.
- Legacy two-string declared-work handlers still lower as a compatibility path.
- `compiler/meshc/tests/e2e_m047_s01.rs` and `compiler/meshc/tests/e2e_m047_s02.rs` were updated to the zero-arg contract and both pass.
- `tiny-cluster/` and `cluster-proof/` now use `@cluster pub fn add() -> Int do 1 + 1 end`, their READMEs describe the derived runtime name `Work.add`, and both package smoke suites pass.
- `compiler/meshc/tests/support/m046_route_free.rs` now treats `Work.add` / `@cluster pub fn add()` as the route-free equal-surface contract.

## Verified green rails

1. `cargo test -p meshc --test e2e_m047_s01 -- --nocapture`
2. `cargo test -p meshc --test e2e_m047_s02 -- --nocapture`
3. `cargo run -q -p meshc -- test tiny-cluster/tests`
4. `cargo run -q -p meshc -- test cluster-proof/tests`

## Still missing

- Todo scaffold generator in `compiler/mesh-pkg/src/scaffold.rs`
- `meshc init --template todo-api` CLI path in `compiler/meshc/src/main.rs`
- real `m047_s05` mesh-pkg test filter and `test_init_clustered_todo_...` tooling test
- `compiler/meshc/tests/support/m047_todo_scaffold.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `scripts/verify-m047-s05.sh`
- final README/docs refresh for the Todo template and ordinary `@cluster` names

## Resume order

1. Finish `compiler/mesh-pkg/src/scaffold.rs` with the Todo template and its unit tests.
2. Re-export the helper from `compiler/mesh-pkg/src/lib.rs`.
3. Add the CLI template flag/path in `compiler/meshc/src/main.rs`.
4. Add tooling/e2e/verifier rails.
5. Refresh README + clustered example docs.
