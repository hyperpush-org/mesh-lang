# M046/S02 Research — Runtime-owned startup trigger and route-free status contract

## Requirements targeted

- **Primary:** R086 — move clustered-work triggering/control semantics fully into runtime/tooling ownership.
- **Primary:** R087 — remove app-owned HTTP and explicit continuity submission from the proof flow.
- **Primary:** R091 — make runtime/tooling inspection sufficient for route-free proofs.
- **Primary:** R092 — remove HTTP-route dependence from the public clustered proof story.
- **Supports:** R093 — keep the proof workload trivial so the remaining complexity is visibly Mesh-owned.
- **Not this slice unless the plan deliberately broadens scope:** R088 (`tiny-cluster/` local proof), R089 (`cluster-proof/` rebuild), and R090 (equal-surface scaffold alignment). Those are explicitly owned by later M046 slices.

## Skills Discovered

- Loaded installed skill: **`rust-best-practices`**.
  - Relevant rules here:
    - keep new runtime/codegen failure paths as explicit `Result`-style diagnostics instead of panics or silent fallbacks;
    - keep tests narrow and behavior-specific so each runtime seam failure stays legible;
    - use comments only for *why* (for example, any bounded convergence wait or keepalive workaround), not as long-lived pseudo-docs.
- Ran `npx skills find "compiler"` and `npx skills find "distributed runtime"`.
  - Results were generic and less relevant than the already-installed Rust guidance.
  - **No additional skills installed.**

## Summary

- S01 already delivers the declaration side of the feature. The build path is real and reusable:
  - `compiler/meshc/src/main.rs::prepare_project_build(...)` produces `PreparedBuild.clustered_execution_plan`
  - `compiler/meshc/src/main.rs::prepare_declared_handler_plan(...)`
  - `compiler/mesh-codegen/src/declared.rs::prepare_declared_runtime_handlers(...)`
  - codegen startup registration through `mesh_register_declared_handler(...)`
  - runtime execution through `compiler/mesh-rt/src/dist/node.rs::submit_declared_work(...)`
- There is **no runtime-owned startup trigger** today. Every public clustered example still triggers work in app code with `Continuity.submit_declared_work(...)`:
  - `compiler/mesh-pkg/src/scaffold.rs`
  - `website/docs/docs/getting-started/clustered-example/index.md`
  - `cluster-proof/work_continuity.mpl`
- Route-free inspection is already mostly present in built-in tooling:
  - `compiler/meshc/src/cluster.rs` exposes `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics`
  - `compiler/mesh-rt/src/dist/operator.rs` already supports transient remote queries for status, one-key continuity lookup, list mode, and diagnostics without registering the CLI as a visible cluster peer
- The main hidden gap on the status side: `ContinuityRecord` already stores `declared_handler_runtime_name` in `compiler/mesh-rt/src/dist/continuity.rs`, but `compiler/meshc/src/cluster.rs::continuity_record_json(...)` omits it. For route-free startup work, that is the simplest existing place to expose work identity without inventing an app status route.
- The main runtime risk is **timing**:
  - `compiler/mesh-rt/src/dist/node.rs::declared_work_membership()` derives membership from `node_state()` plus current sessions
  - `compiler/mesh-rt/src/dist/node.rs::declared_work_placement(...)` hashes over that live membership
  - a startup submit before peer convergence will truthfully choose single-member/local-only placement or `replica_required_unavailable`
- The second risk is **lifetime**:
  - current scaffold and `cluster-proof` stay alive because `HTTP.serve(...)` blocks in app code
  - a route-free app whose `main` only boots the node and returns no longer has that accidental keepalive
  - S02 needs an explicit runtime-owned process-lifetime story, not just a trigger hook
- Auto-trigger must be **work-only**. Current build/codegen plumbing drops the declaration kind too early for that:
  - `ClusteredExecutionMetadata.kind` still knows `Work` / `ServiceCall` / `ServiceCast`
  - `DeclaredRuntimeRegistration` only keeps runtime name + executable symbol, so by the time `compiler/mesh-codegen/src/codegen/mod.rs::generate_main_wrapper(...)` runs, the kind is gone

## Recommendation

### Recommended ownership boundary

Implement startup triggering as a **codegen-emitted runtime hook**, not as a new Mesh-visible API that app authors must call. A public `Continuity.trigger_startup_work(...)` style API would put orchestration back into app code and violate the stated milestone goal.

### Recommended execution shape

1. Keep S01’s `ClusteredExecutionMetadata` as the single source of truth.
2. Derive a **startup work plan** from `kind == Work` entries during build.
3. Emit a runtime hook from the generated main wrapper **after** declared-handler registration and **after** `mesh_main` returns, so a route-free app can still call `Node.start_from_env()` in ordinary Mesh code first.
4. Have the runtime hook spawn a small actor per startup work item instead of submitting synchronously. That actor can:
   - wait bounded time for membership to converge,
   - derive required replicas from observed membership,
   - call existing `submit_declared_work(...)`,
   - preserve fail-closed diagnostics on rejection/timeout.
5. Keep inspection on the existing `meshc cluster status|continuity|diagnostics` surfaces. Prefer exposing `declared_handler_runtime_name` in continuity JSON/list over adding a second status transport.

### Request identity recommendation

A deterministic startup request key derived from the runtime registration name is viable here because continuity state is **in-memory/mirrored**, not durable across a full cluster restart. That means a stable key can preserve “the one logical startup run” across failover and rejoin inside a live cluster without preventing a fresh run on the next clean cluster boot. Even if that convention is used, the CLI should still expose runtime name so route-free inspection does not become guesswork.

### Scope discipline recommendation

Do **not** rewrite the current scaffold/docs/`cluster-proof/` routeful story inside S02 unless the plan explicitly broadens the slice. S03/S04/S05 already own `tiny-cluster/`, the `cluster-proof/` rebuild, and equal-surface scaffold alignment. S02 can and should prove the runtime seam first with a dedicated route-free fixture.

## Implementation Landscape

### 1. Shared clustered-work planning already exists

**Files:**
- `compiler/mesh-pkg/src/manifest.rs`
- `compiler/meshc/src/main.rs`
- `compiler/mesh-codegen/src/declared.rs`

**What exists now:**
- `PreparedBuild.clustered_execution_plan` survives build preparation.
- `ClusteredExecutionMetadata.kind` still distinguishes `Work` from service call/cast declarations.
- `prepare_declared_runtime_handlers(...)` already generates the runtime wrapper surface used by S01/M044.

**What is missing:**
- no startup-work plan survives into codegen main-wrapper generation today.

### 2. Main-wrapper generation is the cleanest existing insertion point

**Files:**
- `compiler/mesh-codegen/src/codegen/mod.rs`
- `compiler/mesh-codegen/src/lib.rs`

**What exists now:**
- generated `main` already calls `mesh_rt_init()` and `mesh_rt_init_actor()`
- registers normal remote-spawn functions and declared handlers before entry
- calls `mesh_main`
- then calls `mesh_rt_run_scheduler()`

**Implications:**
- a post-`mesh_main` runtime hook fits the current control flow for route-free apps
- the current codegen API only receives `DeclaredRuntimeRegistration`, which loses handler kind; either widen that type or pass a separate startup-work plan alongside the declared registrations

### 3. Runtime submit/status surfaces are already generic enough

**Files:**
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/dist/operator.rs`
- `compiler/meshc/src/cluster.rs`

**What exists now:**
- `node.rs::submit_declared_work(...)` already does runtime-owned placement, continuity submit, and local/remote dispatch
- `ContinuityRecord` already stores `declared_handler_runtime_name`
- operator queries already support:
  - one-key lookup
  - recent continuity list
  - diagnostics
  - transient remote query transport
- `meshc cluster continuity` currently hides `declared_handler_runtime_name` in both JSON and human-readable output

**Implications:**
- S02 does not need an app-owned status route
- the smallest truthful status improvement is probably in `compiler/meshc/src/cluster.rs`, not a new operator transport

### 4. Startup convergence is a real runtime problem, not a docs problem

**Files:**
- `compiler/mesh-rt/src/dist/node.rs::declared_work_membership`
- `compiler/mesh-rt/src/dist/node.rs::declared_work_placement`
- `compiler/mesh-rt/src/dist/continuity.rs`

**What exists now:**
- placement hashes over current `node_state + sessions`
- early submit before discovery/session convergence will legitimately produce single-member placement and `local_only` / `replica_required_unavailable` truth
- current routeful tests avoid this by waiting for membership before calling `/work`

**Implications:**
- a runtime-owned startup actor needs a bounded convergence wait or retry policy
- otherwise S02 will land an auto-trigger that works, but only proves local-only startup rather than the replicated/failover story later slices need

### 5. Current public clustered surfaces are still intentionally routeful

**Files:**
- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `compiler/meshc/tests/e2e_m045_s02.rs`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `cluster-proof/main.mpl`
- `cluster-proof/work_continuity.mpl`

**What exists now:**
- scaffold, docs, and `cluster-proof` all assert or teach `Continuity.submit_declared_work(...)` plus `/work`
- `cluster-proof` additionally owns `/membership` and `/work/:request_key`
- the current M045 scaffold/runtime rails hard-code those strings and route expectations

**Implications:**
- rewriting these now broadens S02 into S04/S05 territory
- a dedicated route-free test fixture is the safer first proof for the runtime-owned seam

## Natural Task Seams

### Seam A — Build/codegen startup-work plumbing

**Likely files:**
- `compiler/meshc/src/main.rs`
- `compiler/mesh-codegen/src/lib.rs`
- `compiler/mesh-codegen/src/codegen/mod.rs`
- possibly `compiler/mesh-codegen/src/declared.rs`

**Deliverable:**
- preserve `Work` kind (or a separate startup plan) through codegen
- emit a runtime hook from the generated main wrapper without changing app code

### Seam B — Runtime-owned startup actor

**Likely files:**
- `compiler/mesh-rt/src/dist/node.rs`
- possibly `compiler/mesh-rt/src/dist/continuity.rs`

**Deliverable:**
- runtime hook that schedules startup work using existing `submit_declared_work(...)`
- bounded convergence wait / replica derivation
- durable diagnostics on rejection or timeout
- explicit process-lifetime/keepalive plan for route-free apps

### Seam C — Route-free inspection contract

**Likely files:**
- `compiler/meshc/src/cluster.rs`
- `compiler/mesh-rt/src/dist/operator.rs` only if wire changes are truly needed

**Deliverable:**
- continuity JSON/list exposes enough identity to find startup-run work without app routes
- preferably expose `declared_handler_runtime_name` rather than inventing a new operator path

### Seam D — Focused proof rail

**Likely files:**
- new `compiler/meshc/tests/e2e_m046_s02.rs`
- possibly a small temp-project fixture created inside that test
- later verifier/docs changes can wait for S03–S05

**Deliverable:**
- route-free clustered app boots
- runtime auto-triggers declared work
- built-in `meshc cluster ...` surfaces prove status/diagnostics
- no app-owned submit/status routes in the fixture

## Risks / Unknowns

1. **Kind metadata loss**
   - auto-trigger must not start service call/cast handlers
   - current codegen registration path drops kind information too early

2. **Early-submit drift**
   - without a convergence wait, startup trigger will often prove only local-only placement

3. **Route-free lifetime**
   - removing HTTP routes also removes the accidental `HTTP.serve` keepalive; S02 needs an explicit runtime-owned lifetime story, not just a trigger

4. **Status discoverability**
   - if startup request keys are runtime-generated and CLI output still hides runtime name, route-free inspection becomes guesswork

5. **Scope creep into scaffold/docs/package rails**
   - current routeful scaffold and `cluster-proof` rails are still owned by later slices; S02 should prove the runtime boundary first, then let S03/S04/S05 align the public surfaces

## Don’t Hand-Roll

- Do not add another app-owned `/work` or `/status` route just to prove the runtime hook.
- Do not add a new public Mesh API that app authors must call to start clustered work if a codegen-emitted runtime hook can own it.
- Do not auto-trigger every declared handler blindly; only `kind == Work` is valid.
- Do not duplicate placement logic in the test harness; use `meshc cluster status|continuity|diagnostics` as the proof surface.

## Verification Plan

Keep the runtime/codegen regression bar and add a focused S02 rail:

- `cargo test -p meshc --test e2e_m046_s01 m046_s01_ -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture`
- new focused rail: `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture`

Only add scaffold/docs/verifier replays in S02 if the implementation intentionally broadens into those surfaces; otherwise keep that work for S03–S05 as planned.

## Resume Notes

- `tiny-cluster/` does not exist yet.
- Current route-free proof should start from the runtime/codegen seam, not from `cluster-proof/` or scaffold rewrites.
- The most leverage-heavy files are:
  - `compiler/mesh-codegen/src/codegen/mod.rs::generate_main_wrapper`
  - `compiler/meshc/src/main.rs::PreparedBuild` / `prepare_declared_handler_plan`
  - `compiler/mesh-rt/src/dist/node.rs::submit_declared_work`, `declared_work_membership`, `declared_work_placement`
  - `compiler/meshc/src/cluster.rs::continuity_record_json`
