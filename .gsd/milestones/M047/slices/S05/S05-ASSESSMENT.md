# S05 closeout assessment — blocked before slice completion

## Status
Slice S05 is **not complete**. I stopped on a real compiler/runtime blocker before touching the public scaffold/examples/docs layer or adding the Todo template.

## What I verified
- `cargo test -p meshc --test e2e_m047_s01 -- --nocapture` ✅
- `cargo test -p meshc --test e2e_m047_s02 -- --nocapture` ✅
- Those rails are **not sufficient** for S05 because they still exercise clustered functions that expose public continuity args.

## Reproduced blocker
Minimal repro:
- `mesh.toml` with a basic package
- `main.mpl` containing `fn main() do nil end`
- `work.mpl` containing `@cluster pub fn add() -> Int do 1 + 1 end`

Command:
- `cargo run -q -p meshc -- build .tmp/m047-s05-repro --emit-llvm`

Observed failure:
- `error: declared work wrapper '__declared_work_work_add' expected request_key and attempt_id arguments`

## Root seam
The current zero-ceremony failure is not in the scaffold layer.

Primary compile-time failure point:
- `compiler/mesh-codegen/src/codegen/expr.rs::codegen_actor_wrapper(...)`
  - declared-work wrappers still call `mesh_continuity_complete_declared_work(...)` by assuming the deserialized actor payload also supplies `request_key` and `attempt_id` as the first two **typed function arguments**.
  - For a no-arg `@cluster pub fn add() -> Int`, `body_param_types.len() == 0`, so wrapper codegen aborts before LLVM emission.

Related runtime seam to keep in view while fixing the wrapper:
- `compiler/mesh-rt/src/dist/node.rs::declared_work_arg_payload(...)`
  - runtime still serializes declared-work actor payloads as exactly two strings: `request_key` and `attempt_id`.
  - The next unit should decide deliberately whether the zero-ceremony fix is:
    1. wrapper-only for route-free startup work (read hidden metadata separately, call a zero-arg body), or
    2. a broader payload/adapter change for future clustered function shapes.

## Public-layer drift still present
These surfaces still teach the stale public contract and were intentionally **not** rewritten before the blocker was fixed:
- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `compiler/meshc/tests/support/m046_route_free.rs`
- `compiler/meshc/tests/e2e_m046_s05.rs`
- `tiny-cluster/work.mpl`
- `cluster-proof/work.mpl`
- `README.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `tiny-cluster/README.md`
- `cluster-proof/README.md`

They still refer to `execute_declared_work(...)` / `Work.execute_declared_work`.

## Missing S05 deliverables
Not started because the underlying compiler contract is still false:
- Todo scaffold selector in `meshc init`
- generated SQLite Todo API template
- Todo e2e/support rail (`compiler/meshc/tests/support/m047_todo_scaffold.rs`, `compiler/meshc/tests/e2e_m047_s05.rs`)
- assembled verifier `scripts/verify-m047-s05.sh`

## Resume order
1. Fix zero-ceremony declared-work wrapper generation first.
2. Add a focused regression proving `@cluster pub fn add() -> Int` builds and completes continuity.
3. Only then rebaseline scaffold/examples/docs from `execute_declared_work(...)` to ordinary names.
4. After the route-free cutover is truthful, add the Todo template and its e2e/verifier surface.

## Suggested first commands for the next unit
- Re-run the minimal repro: `cargo run -q -p meshc -- build .tmp/m047-s05-repro --emit-llvm`
- Inspect:
  - `compiler/mesh-codegen/src/codegen/expr.rs`
  - `compiler/mesh-codegen/src/declared.rs`
  - `compiler/mesh-rt/src/dist/node.rs`
- After a code fix, add/refresh named rails before touching docs or scaffold text.