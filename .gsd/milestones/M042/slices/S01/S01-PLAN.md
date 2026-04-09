# S01: Runtime-native keyed continuity API on the healthy path

**Goal:** Move keyed continuity ownership from `cluster-proof` into `mesh-rt` behind a small Mesh-facing `Continuity` API, while preserving the M040 request-key contract and proving the healthy-path behavior on standalone and healthy two-node clusters.
**Demo:** After this: After this slice, cluster-proof submits keyed work and reads keyed status through a runtime-native continuity API on standalone and healthy two-node clusters, while preserving request_key vs attempt_id, duplicate dedupe, conflict rejection, and explicit owner/replica status fields.

## Tasks
- [x] **T01: Added a runtime-owned continuity registry in mesh-rt with keyed transitions and healthy-path node sync hooks.** — Introduce a dedicated `mesh-rt` continuity subsystem that owns keyed request records, attempt tokens, duplicate/conflict decisions, completion transitions, and healthy-path owner/replica state on top of the existing distributed runtime substrate. Keep the state model explicit enough for later fail-closed durability and owner-loss recovery slices without trying to solve those paths here.
  - Estimate: 2h
  - Files: compiler/mesh-rt/src/dist/continuity.rs, compiler/mesh-rt/src/dist/mod.rs, compiler/mesh-rt/src/dist/node.rs, compiler/mesh-rt/src/lib.rs
  - Verify: cargo test -p mesh-rt continuity -- --nocapture
- [x] **T02: Stopped after tracing the full Continuity compiler/runtime seam so the next unit can implement it without re-research.** — Add a dedicated `Continuity` Mesh module backed by new runtime intrinsics so Mesh code can submit keyed work, read truthful keyed status, and mark healthy-path completions without talking directly to runtime internals. Keep the API small and continuity-specific rather than overloading `Node` or `Global` with workload semantics.
  - Estimate: 90m
  - Files: compiler/mesh-typeck/src/infer.rs, compiler/mesh-codegen/src/codegen/intrinsics.rs, compiler/mesh-codegen/src/codegen/expr.rs, compiler/mesh-rt/src/lib.rs, compiler/meshc/tests/e2e_m042_s01.rs
  - Verify: cargo test -p meshc --test e2e_m042_s01 continuity_api -- --nocapture
- [x] **T03: Stopped after reproducing the absent M042 Continuity proof target and confirming that the compiler/runtime Continuity surface is still unwired; no code shipped.** — Replace the app-authored request registry, replica-prepare logic, and keyed status ownership inside `cluster-proof` with calls into the runtime-native `Continuity` API while preserving the existing `/work` HTTP contract. Update the proof surfaces so standalone and healthy two-node cluster runs prove the new ownership boundary and keep the M040 semantic guarantees intact.
  - Estimate: 2h
  - Files: cluster-proof/work.mpl, cluster-proof/main.mpl, cluster-proof/tests/work.test.mpl, compiler/meshc/tests/e2e_m042_s01.rs, scripts/verify-m042-s01.sh
  - Verify: cargo run -q -p meshc -- test cluster-proof/tests && cargo test -p meshc --test e2e_m042_s01 -- --nocapture && bash scripts/verify-m042-s01.sh
