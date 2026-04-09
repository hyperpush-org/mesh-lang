---
id: S01
parent: M045
milestone: M045
provides:
  - A public runtime-owned clustered bootstrap API: `Node.start_from_env()` plus typed `BootstrapStatus`.
  - A smaller `meshc init --clustered` source contract that no longer hand-rolls bootstrap env parsing or direct node startup.
  - A `cluster-proof` startup path aligned to the public runtime bootstrap contract instead of proof-app-owned bootstrap code.
  - A terminal M045/S01 verifier that protects the new bootstrap surfaces and the retained M044 scaffold/public-contract rails.
requires:
  []
affects:
  - S02
  - S03
  - S04
  - S05
key_files:
  - compiler/mesh-rt/src/dist/bootstrap.rs
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/mesh-typeck/src/infer.rs
  - compiler/mesh-typeck/src/builtins.rs
  - compiler/mesh-codegen/src/mir/lower.rs
  - compiler/mesh-codegen/src/codegen/intrinsics.rs
  - compiler/mesh-pkg/src/scaffold.rs
  - compiler/meshc/tests/e2e_m045_s01.rs
  - compiler/meshc/tests/e2e_m044_s03.rs
  - compiler/meshc/tests/e2e_m044_s05.rs
  - cluster-proof/main.mpl
  - cluster-proof/config.mpl
  - cluster-proof/docker-entrypoint.sh
  - scripts/verify-m045-s01.sh
  - cluster-proof/README.md
key_decisions:
  - D212: keep the runtime bootstrap validator separate from the low-level bind parser so `start_from_env()` stays fail-closed while `mesh_node_start("name@host:0", ...)` still supports OS-assigned ports.
  - D213: expose `Node.start_from_env()` as a public typed Mesh surface returning `Result<BootstrapStatus, String>`, and make clustered scaffolds consume/log that status instead of reading bootstrap env or calling `Node.start(...)` directly.
  - D214: make `cluster-proof/main.mpl` delegate clustered bootstrap to `Node.start_from_env()`, while `cluster-proof/docker-entrypoint.sh` only preflights continuity topology env that the runtime bootstrap API does not yet validate.
patterns_established:
  - Clustered Mesh apps should call `Node.start_from_env()` and inspect/log `BootstrapStatus`; app code should keep only local HTTP/work logic instead of re-implementing cluster mode detection, identity parsing, or direct `Node.start(...)` orchestration.
  - Keep the low-level listener primitive available, but validate clustered startup at a higher runtime-owned bootstrap layer and fail closed before side effects on malformed env.
  - Use one terminal verifier script to replay the lower proof rails and fail on zero-test or stale-artifact drift instead of inventing a second docs-only acceptance story.
observability_surfaces:
  - Scaffolded clustered apps now emit a runtime-owned startup log (`[clustered-app] runtime bootstrap ...`) that includes the bootstrap mode and resolved node identity.
  - The scaffolded runtime surface still exposes `/health` for basic service readiness.
  - `meshc cluster status <node-name@host:port> --json` remains the read-only cluster truth surface and is explicitly protected by the scaffold rails.
  - `scripts/verify-m045-s01.sh` produces an assembled fail-closed proof replay for bootstrap/scaffold/proof-app drift.
drill_down_paths:
  - .gsd/milestones/M045/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M045/slices/S01/tasks/T02-SUMMARY.md
  - .gsd/milestones/M045/slices/S01/tasks/T03-SUMMARY.md
  - .gsd/milestones/M045/slices/S01/tasks/T04-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-30T19:22:01.870Z
blocker_discovered: false
---

# S01: Runtime-Owned Cluster Bootstrap

**Runtime-owned clustered bootstrap is now a public Mesh surface: `Node.start_from_env()` returns typed `BootstrapStatus`, and both scaffolded clustered apps and `cluster-proof` use it instead of hand-rolled env/bootstrap code.**

## What Happened

S01 moved clustered startup ownership onto the runtime instead of leaving it in generated/example Mesh code. T01 added `compiler/mesh-rt/src/dist/bootstrap.rs` plus the `start_from_env()` integration seam in `compiler/mesh-rt/src/dist/node.rs`, so standalone-vs-cluster detection, `MESH_*` validation, Fly identity fallback, typed bootstrap status, and fail-closed startup now live in Rust. The low-level `mesh_node_start(...)`/`Node.start(...)` primitive stayed intact, but the runtime now keeps the public bootstrap validator separate from a bind-only parser so `name@host:0` listener tests still work without weakening the clustered env contract.

The local tree was missing the planned T02 compiler/runtime exposure when T03 started, so that prerequisite was finished there before the scaffold rewrite. `Node.start_from_env() -> Result<BootstrapStatus, String>` is now a public builtin in typeck/codegen, the matching status type is pre-seeded through MIR/intrinsics, and `compiler/meshc/tests/e2e_m045_s01.rs` proves both happy-path typed access and compile-time misuse rails. With that seam available, `meshc init --clustered` was rewritten so generated `main.mpl` no longer reads bootstrap env or calls `Node.start(...)` directly. The scaffold now logs the returned runtime bootstrap status, keeps only HTTP/work logic local, and still exposes `/health` plus read-only `meshc cluster status` truth.

T04 finished the ownership move on the retained proof app. `cluster-proof/main.mpl` now calls `Node.start_from_env()` directly, `cluster-proof/config.mpl` only keeps continuity/durability concerns, and `cluster-proof/docker-entrypoint.sh` was reduced to continuity-topology preflight the runtime bootstrap API does not yet validate. The slice also added `scripts/verify-m045-s01.sh` as the terminal fail-closed acceptance rail. It replays the new M045 bootstrap rails, the `cluster-proof` build/test contract, and the protected M044 scaffold/public-contract rails so later slices inherit one authoritative closeout surface instead of a second parallel proof story.

Net result: clustered bootstrap is now runtime-owned and typed across the public app surfaces. The scaffolded clustered app and `cluster-proof` are both visibly smaller at startup, and the remaining distributed logic is pushed into the runtime or left as explicit non-bootstrap continuity concerns for later slices to retire honestly.

## Verification

I reran every slice-plan verification command and they all passed:

- `cargo test -p mesh-rt bootstrap_ -- --nocapture`
- `cargo test -p meshc --test e2e_m045_s01 m045_s01_bootstrap_api_ -- --nocapture`
- `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture`
- `cargo run -q -p meshc -- build cluster-proof`
- `cargo run -q -p meshc -- test cluster-proof/tests`
- `bash scripts/verify-m045-s01.sh`

Those commands proved the runtime bootstrap matrix (standalone, explicit node name, Fly identity fallback, malformed env, bind failure), the public Mesh builtin/typed status contract including compile-fail misuse rails, the smaller scaffold source shape, live `/health` plus `meshc cluster status` truth on the scaffolded app, `cluster-proof` adoption of the same bootstrap boundary, and the assembled fail-closed verifier replay including protected M044 public-contract rails.

## Requirements Advanced

- R077 — The clustered example surfaces are visibly smaller because bootstrap ownership moved into `mesh-rt`; scaffolded `main.mpl` and `cluster-proof/main.mpl` now call `Node.start_from_env()` and inspect typed `BootstrapStatus` instead of carrying system-shaped startup glue.
- R079 — App-owned cluster-mode detection, bootstrap env parsing, and direct `Node.start(...)` orchestration were removed from the scaffolded clustered app and from `cluster-proof` Mesh startup, tightening the honesty boundary around runtime-owned clustering behavior.
- R080 — `meshc init --clustered` is now materially closer to the intended docs-grade entry surface because the generated app demonstrates the public runtime bootstrap API instead of teaching readers proof-app-style startup mechanics.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

The local snapshot did not actually contain the planned T02 public builtin/codegen seam when T03 began, so that prerequisite was completed during T03 before the scaffold rewrite could land. I also updated `cluster-proof/README.md` so the public runbook states the new bootstrap boundary explicitly instead of leaving the docs on the old example-owned story.

## Known Limitations

S01 does not yet deliver the final tiny two-node example that proves runtime-chosen remote execution and failover end to end; that remains S02/S03 work under R078. `Node.start_from_env()` now owns bootstrap env validation, but continuity-topology env still requires packaged preflight in `cluster-proof/docker-entrypoint.sh`, so the runtime bootstrap API is not yet the only startup validation seam for the retained proof app. `cluster-proof` also remains the deeper proof surface rather than the primary teaching surface; S04/S05 still need to collapse more example-side continuity/config residue and move the docs story onto the tiny scaffold-first example.

## Follow-ups

- S02 should build the tiny two-node example directly on the new `Node.start_from_env()` / `BootstrapStatus` surface and prove runtime-chosen remote execution without app-owned routing.
- S03 should prove failover/status truth on that same small example instead of switching back to a proof-app-only story.
- S04 should remove or deeply collapse the remaining non-bootstrap example-side continuity/config glue that survived this slice because it is still outside the runtime bootstrap API.
- S05 should make the scaffold-first example the main docs surface and keep `cluster-proof` as the deeper proof rail only.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/bootstrap.rs` — Added the runtime-owned bootstrap planner/validator and typed bootstrap status model.
- `compiler/mesh-rt/src/dist/node.rs` — Added `start_from_env()` integration, preserved the low-level node start primitive, and split bind-only parsing from the public bootstrap validator.
- `compiler/mesh-typeck/src/infer.rs` — Registered `Node.start_from_env()` and typed `BootstrapStatus` usage on the Mesh-facing compiler surface.
- `compiler/mesh-typeck/src/builtins.rs` — Added the builtin surface for the public runtime bootstrap API and status type.
- `compiler/mesh-codegen/src/mir/lower.rs` — Pre-seeded the bootstrap status type and lowering support so compiled Mesh code can inspect typed bootstrap fields.
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — Declared the new runtime intrinsic backing `Node.start_from_env()`.
- `compiler/mesh-pkg/src/scaffold.rs` — Rewrote the clustered scaffold so generated `main.mpl` delegates startup to `Node.start_from_env()` and logs `BootstrapStatus`.
- `compiler/meshc/tests/e2e_m045_s01.rs` — Added M045 bootstrap e2e coverage for typed happy paths, runtime errors, compile-fail misuse, and scaffold source shape.
- `compiler/meshc/tests/e2e_m044_s03.rs` — Updated the protected scaffold runtime rail to assert the runtime-owned bootstrap log, `/health`, and `meshc cluster status` truth.
- `compiler/meshc/tests/e2e_m044_s05.rs` — Added protected public-contract/source assertions so the retained M044 docs-grade story stays aligned with the new bootstrap boundary.
- `cluster-proof/main.mpl` — Removed example-owned bootstrap orchestration and switched the retained proof app onto `Node.start_from_env()`.
- `cluster-proof/config.mpl` — Reduced config ownership to continuity and durability concerns instead of bootstrap env parsing.
- `cluster-proof/docker-entrypoint.sh` — Trimmed packaged startup preflight down to continuity-topology env that the runtime bootstrap API does not yet validate.
- `scripts/verify-m045-s01.sh` — Added the assembled fail-closed slice acceptance rail that replays the M045 bootstrap/proof rails and protected M044 surfaces.
- `cluster-proof/README.md` — Updated the public runbook so the documented startup story matches the new runtime-owned bootstrap contract.
