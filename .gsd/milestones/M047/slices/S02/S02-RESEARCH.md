# S02 Research — Replication-count semantics for clustered functions

## Summary

- **Primary requirement:** R098. This slice is where S01's `replication_count` metadata stops being compiler-only and starts meaning something at runtime.
- **Supporting requirements:** R097 and R099, because the new proof surface should use source-first `@cluster` on ordinary non-HTTP functions instead of falling back to `clustered(work)` or route wrappers. R106 only matters here insofar as runtime/CLI truth should stay source-first and runtime-name-specific.
- S01 already did the parser/pkg/compiler/LSP half. The important handoff is that `ClusteredExecutionMetadata.replication_count` exists in `compiler/mesh-pkg/src/manifest.rs`, but **nothing past mesh-pkg/meshc currently consumes it**.
- The cleanest implementation seam is **declared-handler metadata**, not a second startup-only count registry. Thread the count from `ClusteredExecutionMetadata` into the declared-handler registration path, then let startup trigger / direct submit / recovery all reuse that one source of truth.
- The current runtime is still structurally **single-replica**: `SubmitRequest::validate` rejects `required_replica_count > 1`, `DeclaredWorkPlacement` stores only one `replica_node`, and `ContinuityRecord` stores only one `replica_status`. That means full `N`-fanout is not already present.
- The hardcoded `Work.execute_declared_work` story is **not** in the core runtime registry anymore; it mainly survives in `tiny-cluster/`, `cluster-proof/`, scaffold output, and old route-free proofs. Since roadmap S04 owns dogfood migration, S02 can add new M047-targeted proofs without rewriting those packages yet.

## Requirements Focus

- **R098 (primary):** make default `@cluster` mean runtime replication count `2`, and preserve explicit `@cluster(N)` counts through runtime truth.
- **R099 (support):** prove this on ordinary clustered functions, not HTTP routes.
- **R097 (support):** new S02 proofs should use `@cluster` / `@cluster(N)` rather than legacy `clustered(work)`.
- **R106 (minor support):** if new runtime truth is surfaced through `meshc cluster continuity`, it should describe the source-first runtime name/count rather than a manifest-shaped fallback.

## Skills Discovered

- Used installed skill **`rust-best-practices`** for the implementation recommendation to keep one metadata seam and explicit `Result`-based failure surfaces instead of parallel registries or silent fallback.
- Used installed skill **`llvm`** for verification guidance: emitted LLVM marker checks are the right proof for registration/lowering changes before runtime rails.
- No additional directly relevant skills were missing, so no new skill installs were needed.

## Implementation Landscape

### 1. S01 count metadata exists, but stops in mesh-pkg / meshc

**Files:**
- `compiler/mesh-pkg/src/manifest.rs`
- `compiler/meshc/src/main.rs`

**What exists now:**
- `compiler/mesh-pkg/src/manifest.rs` defines `DEFAULT_CLUSTER_REPLICATION_COUNT = 2`, `ClusteredReplicationCount`, and threads that into `ClusteredExecutionMetadata`.
- `collect_source_cluster_declarations(...)` already resolves `@cluster` to **defaulted 2** and `@cluster(N)` to **explicit N**.
- `compiler/meshc/src/main.rs` stores the validated list on `PreparedBuild.clustered_execution_plan`.

**What is missing:**
- The next compiler hop, `prepare_declared_handler_plan(...)` in `compiler/meshc/src/main.rs`, drops everything except:
  - `kind`
  - `runtime_registration_name`
  - `executable_symbol`
- Repo-wide search confirms `replication_count` is currently only used in `compiler/mesh-pkg/src/manifest.rs` tests and diagnostics; it never reaches codegen or runtime.

**Planner implication:**
- S02 does **not** need new parser/pkg discovery logic.
- The first code seam is `ClusteredExecutionMetadata -> DeclaredHandlerPlanEntry`.

### 2. Best propagation seam: enrich declared-handler metadata, not startup registry duplication

**Files:**
- `compiler/mesh-codegen/src/declared.rs`
- `compiler/mesh-codegen/src/codegen/mod.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-rt/src/dist/node.rs`

**What exists now:**
- `DeclaredHandlerPlanEntry` carries only `kind`, `runtime_registration_name`, and `executable_symbol`.
- `DeclaredRuntimeRegistration` carries only `runtime_registration_name` and `executable_symbol`.
- Runtime `DeclaredHandlerEntry` stores only `executable_name` and `fn_ptr`.
- `mesh_register_declared_handler(...)` registers runtime name + executable + fn pointer.
- `mesh_register_startup_work(...)` stores only a runtime name in `STARTUP_WORK_REGISTRY`.

**Why this matters:**
- Startup work currently registers only names, then later resolves work again through the declared-handler registry.
- Direct declared submit and automatic recovery also start from a runtime name.
- That makes the **declared-handler registry** the obvious place to keep replication-count truth.

**Recommended shape:**
- Add replication-count metadata to:
  - `DeclaredHandlerPlanEntry`
  - `DeclaredRuntimeRegistration`
  - runtime `DeclaredHandlerEntry`
  - `mesh_register_declared_handler(...)` / corresponding LLVM intrinsic
- Keep startup registry name-only if possible.
- When startup/recovery needs count semantics, look them up from the declared-handler registry entry for that runtime name.

**Why this is the clean seam:**
- One source of truth.
- No extra name->count map to keep in sync.
- Matches the `rust-best-practices` preference for explicit, narrow data flow over parallel registries.

### 3. Runtime semantics are still structurally single-replica

**Files:**
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/dist/node.rs`

**Current hard limits:**
- `SubmitRequest::validate(...)` in `compiler/mesh-rt/src/dist/continuity.rs` rejects `required_replica_count > 1` with `invalid_required_replica_count`.
- `DeclaredWorkPlacement` in `compiler/mesh-rt/src/dist/node.rs` has only:
  - `owner_node`
  - `replica_node`
- `ContinuityRecord` in `compiler/mesh-rt/src/dist/continuity.rs` stores one `replica_node` and one `replica_status`.
- `wait_for_startup_convergence_with(...)` returns a hardcoded `required_replica_count: 1` when a peer stabilizes, otherwise `0`.
- `automatic_recovery_submit_entry(...)` hardcodes `submit_declared_work(..., 0)`.

**What that means:**
- S02 must decide early whether public `@cluster(N)` means:
  1. **total copies / replication factor**, with internal `required_replica_count = N - 1`, or
  2. some other interpretation.
- The cleanest mapping is **public replication factor = total copies**:
  - `@cluster` -> `2` total copies -> internal `required_replica_count = 1`
  - `@cluster(3)` -> `3` total copies -> internal `required_replica_count = 2`
- But the current runtime cannot honestly place or track two replica nodes. So **full `N > 2` fanout would require a larger redesign** of placement, record state, replication prepare/ack, diagnostics, and operator surfaces.

**Planner implication:**
- Treat this as the first architectural fork.
- The slice can still land honestly if it:
  - makes default `@cluster` mean replication factor `2`
  - preserves explicit counts in runtime truth
  - **fails closed** when a requested replication factor cannot be satisfied by the current runtime/topology
- What it must not do is silently clip `@cluster(3)` down to the existing single-replica model while pretending count `3` was honored.

### 4. Public runtime truth surfaces do not preserve count yet

**Files:**
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/dist/operator.rs`
- `compiler/meshc/src/cluster.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`

**What exists now:**
- `ContinuityRecord` stores runtime name, routing, owner/replica, etc., but **not replication count**.
- `meshc cluster continuity` JSON/human output in `compiler/meshc/src/cluster.rs` prints no count.
- The Mesh-language `ContinuityRecord` shape in:
  - `compiler/mesh-typeck/src/infer.rs`
  - `compiler/mesh-codegen/src/mir/lower.rs`
  - `compiler/mesh-rt/src/dist/continuity.rs` (`MeshContinuityRecord`)
  also has no count field.
- Diagnostics already have a foothold: `log_submit(...)` in `compiler/mesh-rt/src/dist/continuity.rs` records `required_replicas`, and startup diagnostics already include `required_replicas` metadata.

**Planner implication:**
- If S02 wants “explicit counts are preserved through runtime truth,” count must be surfaced in more than diagnostic metadata.
- The honest place is the continuity record itself, then update CLI/operator/type surfaces together.
- This is one of those multi-file “update the struct everywhere” seams. If a field is added, update the whole quartet together:
  - `compiler/mesh-rt/src/dist/continuity.rs` (`ContinuityRecord`, `MeshContinuityRecord`, encode/decode)
  - `compiler/mesh-typeck/src/infer.rs`
  - `compiler/mesh-codegen/src/mir/lower.rs`
  - `compiler/meshc/src/cluster.rs`
  and any operator payload code that serializes continuity records.

### 5. Generic runtime-name proof infrastructure already exists

**Files:**
- `compiler/meshc/tests/e2e_m046_s02.rs`
- `compiler/meshc/tests/support/m046_route_free.rs`

**Important existing proof:**
- `compiler/meshc/tests/e2e_m046_s02.rs::m046_s02_codegen_source_and_manifest_work_share_startup_identity` already proves source-declared and manifest-declared work can share a generic runtime name like `Work.handle_submit`.
- `support/m046_route_free.rs` is generic over runtime name and continuity record lookup; it is not tied to `Work.execute_declared_work`.

**What is still hardcoded:**
- The older route-free runtime project fixtures in `e2e_m046_s02.rs` and `e2e_m046_s03.rs` still use `Work.execute_declared_work` / `Work.execute_aux_work` because they are exercising the M046 public proof packages.

**Planner implication:**
- Reuse `support/m046_route_free.rs`.
- Add a fresh **M047-targeted** e2e target instead of rewriting M046 rails.
- That keeps M046 historical acceptance intact while letting S02 prove the new runtime-name/count semantics on ordinary `@cluster` functions.

### 6. Hardcoded `Work.execute_declared_work` survives in dogfood/scaffold surfaces, but S04 owns the public migration

**Files:**
- `tiny-cluster/work.mpl`
- `cluster-proof/work.mpl`
- `compiler/mesh-pkg/src/scaffold.rs`
- `tiny-cluster/README.md`
- `cluster-proof/README.md`
- `tiny-cluster/tests/work.test.mpl`
- `cluster-proof/tests/work.test.mpl`

**Current state:**
- `tiny-cluster/` and `cluster-proof/` still expose:
  - `declared_work_runtime_name() -> "Work.execute_declared_work"`
  - a single `clustered(work)` declaration
- `compiler/mesh-pkg/src/scaffold.rs` still generates the same legacy story.
- Repo tests assert on those exact strings.

**Planner implication:**
- Do **not** use these files as the primary S02 implementation surface unless a core runtime change forces it.
- Dogfood/scaffold migration belongs to S04.
- S02 can deliver its contract with new temp-project runtime proofs and core compiler/runtime changes only.

## Recommendation

1. **Resolve the public->internal count mapping first.**
   - Recommended: public `replication_count` means total copies.
   - Internal continuity semantics can derive `required_replica_count = replication_count - 1`.
   - That makes default `@cluster` cleanly map to one required replica.

2. **Thread count through declared-handler registration, not a second registry.**
   - Start at `ClusteredExecutionMetadata` in `meshc`.
   - Extend `DeclaredHandlerPlanEntry` / `DeclaredRuntimeRegistration` / runtime `DeclaredHandlerEntry`.
   - Let startup/direct submit/recovery look up count from declared-handler metadata.

3. **Make runtime truth durable, not just transient diagnostic metadata.**
   - Add count to `ContinuityRecord` and its CLI/operator/type surfaces.
   - Diagnostics can remain a secondary proof surface, but they are not enough by themselves.

4. **Fail closed on unsupported multi-replica requests if full fanout is not in scope for S02.**
   - Given the single `replica_node` model, S02 should not silently pretend `@cluster(3)` has three live copies.
   - If the slice does not widen placement/record shape, reject unsupported counts or topologies explicitly.

5. **Add a new M047-specific proof target instead of mutating M046 dogfood rails.**
   - Use `@cluster` ordinary functions with runtime names like `Work.handle_submit` / `Work.handle_retry`.
   - Keep `tiny-cluster/`, `cluster-proof/`, and scaffold output for S04.

## Natural Task Split

### T01 — Compiler/codegen count threading

**Likely files:**
- `compiler/meshc/src/main.rs`
- `compiler/mesh-codegen/src/declared.rs`
- `compiler/mesh-codegen/src/codegen/mod.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`

**Goal:**
- Carry replication count from `ClusteredExecutionMetadata` into the declared-handler registration/runtime boundary.

**Why first:**
- Everything else depends on count existing past meshc.

### T02 — Runtime continuity semantics + truth surfaces

**Likely files:**
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/dist/operator.rs`
- `compiler/meshc/src/cluster.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`

**Goal:**
- Interpret count honestly at runtime.
- Surface it through continuity records/CLI.
- Update startup/recovery paths that currently hardcode `0` / `1`.

**Why second:**
- This is the risky behavior change and the slice’s real contract.

### T03 — New M047 runtime proofs

**Likely files:**
- `compiler/meshc/tests/e2e_m047_s02.rs` (new)
- `compiler/meshc/tests/support/m046_route_free.rs` (reuse/extend only if needed)
- `compiler/mesh-rt/src/dist/continuity.rs` tests or `compiler/mesh-rt/src/dist/node.rs` tests for unit-level count semantics

**Goal:**
- Prove default `@cluster` -> runtime replication factor `2`.
- Prove explicit count is preserved in runtime truth.
- Prove the runtime-name truth surface is generic (`Work.handle_submit`, etc.), not hardcoded to `Work.execute_declared_work`.

## Risks and Unknowns

- **Biggest risk:** current runtime only has one replica slot. If the slice promises real `N`-replica execution for `N > 2`, that is a deeper redesign than the roadmap summary implies.
- **Record ABI drift risk:** adding a field to `ContinuityRecord` without updating the typeck/MIR/runtime struct surfaces together will reproduce the same kind of downstream drift this repo has hit before.
- **Startup/recovery mismatch risk:** even if direct submit honors count, `wait_for_startup_convergence_with(...)` and `automatic_recovery_submit_entry(...)` currently hardcode count-like behavior and will silently erase the new semantics unless updated in the same slice.
- **Historical-rail churn risk:** changing `tiny-cluster/`, `cluster-proof/`, or scaffold outputs here would drag S04 work into S02 and create unnecessary diff noise.

## Verification

Recommended acceptance stack for this slice:

1. **Codegen / lowering proof**
   - `cargo test -p mesh-codegen m047_s02 -- --nocapture`
   - Use emitted LLVM markers, per the `llvm` skill, to confirm runtime registration/startup registration now carries the new count metadata where intended.

2. **Runtime semantics proof**
   - `cargo test -p mesh-rt m047_s02 -- --nocapture`
   - Cover:
     - default `@cluster` mapping to runtime replication factor `2`
     - explicit count preservation
     - fail-closed behavior for unsupported counts/topologies if full multi-replica fanout is deferred

3. **End-to-end compiler/runtime proof**
   - `cargo test -p meshc --test e2e_m047_s02 -- --nocapture`
   - Fresh temp-project rails should assert:
     - source-first `@cluster` / `@cluster(N)` on ordinary functions
     - emitted LLVM registration markers use generic runtime names like `Work.handle_submit`
     - `meshc cluster continuity --json` and human output preserve runtime name + replication count
     - diagnostics still report truthful startup/submit transitions

4. **Regression guard if shared startup code is touched**
   - `cargo test -p meshc --test e2e_m046_s02 -- --nocapture`
   - Only as a follow-on guard; it should not become the primary S02 proof surface.

## Useful Starting Files

If the planner wants the shortest read path before tasking:

1. `compiler/mesh-pkg/src/manifest.rs` — where `replication_count` exists now
2. `compiler/meshc/src/main.rs` — where it gets dropped before codegen
3. `compiler/mesh-codegen/src/declared.rs` — declared-handler/startup registration structs
4. `compiler/mesh-rt/src/dist/node.rs` — declared-handler registry, startup registry, placement, startup trigger, recovery
5. `compiler/mesh-rt/src/dist/continuity.rs` — submit validation, record shape, submit logs, record encode/decode
6. `compiler/meshc/src/cluster.rs` — public human/JSON continuity truth
7. `compiler/meshc/tests/e2e_m046_s02.rs` + `compiler/meshc/tests/support/m046_route_free.rs` — reusable generic runtime-name proof scaffolding
8. `tiny-cluster/work.mpl`, `cluster-proof/work.mpl`, `compiler/mesh-pkg/src/scaffold.rs` — examples of the hardcoded story that should stay out of S02 unless absolutely necessary
