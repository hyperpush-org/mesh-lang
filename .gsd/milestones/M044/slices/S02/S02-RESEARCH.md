# S02 Research — Runtime-Owned Declared Handler Execution

## Summary

S02 is the first slice where the M044 contract becomes operational instead of declarative. S01 proved that `mesh.toml` can name a narrow clustered boundary, but today that boundary is validation-only: `meshc` loads `mesh.toml`, validates declarations against compiler exports, and then drops the manifest before MIR/codegen. The runtime already owns the continuity state machine, replica prepare/ack flow, promotion state, and remote spawn transport, but `cluster-proof` still owns three critical pieces in Mesh code:

1. **membership + placement** (`cluster-proof/cluster.mpl`, `cluster-proof/work.mpl`)
2. **submit/status orchestration** (`cluster-proof/work_continuity.mpl`)
3. **actor-context remote dispatch bridging** (`dispatch_remote_work` / `Node.spawn` indirection)

The highest-risk design question is not the continuity registry — that already exists. The risk is the **execution ABI for declared handlers**. Today `work` declarations can name any public function, and `cluster-proof/mesh.toml` currently declares HTTP-facing handlers (`WorkContinuity.handle_work_submit`, `handle_work_status`, `handle_promote`, `WorkLegacy.handle_work_probe`). Those are not a truthful long-term runtime-execution boundary. The current remote-spawn path is string-name based, actor-context only, and explicitly limited by remote argument support. If S02 tries to “execute declared handlers remotely” without tightening or thunking this ABI, it will overclaim.

## Requirements Targeted

### Primary
- **R064** — S02 owns this requirement. The runtime must become the owner of placement, continuity replication, attempt fencing, authority state, and failover behavior for declared handlers.

### Guardrails / supporting requirements
- **R063** — S01 already locked the declared boundary. S02 must preserve that exact boundary and only attach runtime semantics to declared targets.
- **R069** — S02 advances this by removing more explicit clustering logic from `cluster-proof`, but final removal of the old path still belongs to S05.

### Explicitly not this slice
- **R065** built-in operator surfaces are S03.
- **R068** bounded automatic promotion is S04.

## Skills Discovered

- **Loaded existing skill:** `rust-best-practices`
  - Relevant rules used here: prefer `Result` over panic for fallible runtime seams; keep tests descriptive and narrow so verifier filters stay stable.
- **Installed during research:** `distributed-systems` (`yonatangross/orchestkit@distributed-systems`)
  - Relevant rule used here: keep idempotency/dedup centralized in one runtime-owned coordination layer; do not split coordination between app-local logic and runtime state.

## Key Findings

### 1. Manifest declarations are still compile-time validation only.

**Files:**
- `compiler/meshc/src/main.rs`
- `compiler/mesh-pkg/src/manifest.rs`

`meshc` loads `mesh.toml`, typechecks all modules, builds a `ClusteredExportSurface`, validates `[cluster].declarations`, and then immediately lowers to MIR without carrying any clustered metadata forward.

Planner implication:
- S02 needs a **new metadata plumbing seam** after validation and before/through MIR/codegen/runtime registration.
- `ClusteredExportSurface` is sufficient for fail-closed validation, but **not** sufficient for runtime execution. It only stores names, not execution metadata.

### 2. The runtime continuity substrate is already real and fairly complete.

**Files:**
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/dist/node.rs`

What already exists in Rust/runtime:
- typed continuity authority/record/submit decision payloads
- dedup/conflict/rejected/owner-loss recovery in `ContinuityRegistry::submit`
- replica prepare/ack transport over node sessions
- promotion state and re-projection of mirrored records
- remote spawn by registered function name
- continuity/logging surfaces with explicit transition reasons

Planner implication:
- S02 should **reuse** the continuity registry and node-side replica hooks, not rebuild any of that in Mesh code.
- Per `rust-best-practices`, new runtime-owned seams should keep returning explicit `Result`-style reject reasons instead of panicking or hiding failures.

### 3. `cluster-proof` still owns placement and dispatch in app code.

**Files:**
- `cluster-proof/cluster.mpl`
- `cluster-proof/work.mpl`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/work_legacy.mpl`
- `cluster-proof/main.mpl` (wiring confirmed via search output)

Current app-owned responsibilities:
- `cluster.mpl` computes canonical membership and deterministic owner/replica placement.
- `work.mpl` turns that into `TargetSelection` and exposes `current_target_selection(...)`.
- `work_continuity.mpl` still:
  - looks up target selection
  - picks owner/replica locally
  - calls `Continuity.submit(...)`
  - decides whether to dispatch
  - bridges into actor context for remote `Node.spawn`
  - polls status until completion for the legacy probe path
- `work_legacy.mpl` still owns the old probe routing path and legacy target selection.

Planner implication:
- The real S02 rewrite seam is **cluster-proof hot-path deletion**, not more typed wrappers.
- Placement logic in `cluster.mpl` / `work.mpl` is the cleanest candidate to move into Rust/runtime.

### 4. The current remote-spawn path is useful, but it is not a generic declared-handler executor.

**Files:**
- `compiler/mesh-codegen/src/codegen/mod.rs`
- `compiler/mesh-codegen/src/codegen/expr.rs`
- `compiler/mesh-rt/src/dist/node.rs`

Important runtime/codegen facts:
- Codegen registers all top-level non-closure functions at program startup via `mesh_register_function(...)`.
- The function registry is then used by `mesh_node_spawn(...)` to look up a function by string name on the remote node.
- `mesh_node_spawn(...)` must run from actor context.
- `cluster-proof/work_continuity.mpl` already works around that by spawning `dispatch_remote_work`, which then calls `Node.spawn(...)`.
- Codegen explicitly skips compiler-internal functions whose names start with `__`.

Planner implication:
- Declared **work** handlers can plausibly reuse the current function registry path.
- Declared **service_call** / **service_cast** handlers cannot reuse it unchanged, because the actual generated service handler bodies are `__service_*` internals and are intentionally not registered.
- S02 needs either:
  - compiler-generated public wrapper/thunk functions for declared service handlers, or
  - a separate runtime registration path for service handler entrypoints.

### 5. The current `work` declaration kind is broader than the runtime execution ABI.

**Files:**
- `compiler/mesh-pkg/src/manifest.rs`
- `cluster-proof/mesh.toml`
- `cluster-proof/work_continuity.mpl`

`work` currently means “public function target” at validation time. That was fine for S01. It is **not** enough for S02 execution.

Why:
- `cluster-proof/mesh.toml` currently declares HTTP handlers as clustered `work` targets.
- The remote-spawn path transports raw/tagged arguments and requires actor-context dispatch.
- The current proof app does **not** remotely execute those HTTP handlers directly; it executes a much smaller actor (`execute_work(request_key, attempt_id)`) after doing placement and continuity work locally.

Planner implication:
- S02 must choose one honest path:
  1. **Narrow the executable declared-work ABI** to transport-safe/runtime-owned handler shapes, or
  2. **Generate wrapper/thunk functions** that convert from declared boundary -> runtime-safe execution shape.
- Do **not** assume that “declared public function” is already a remote-executable unit.
- This is the first planner decision. Everything else depends on it.

### 6. Service declarations already have the type/export information needed for codegen work.

**Files:**
- `compiler/mesh-typeck/src/lib.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/expr.rs`

Useful existing compiler seams:
- `ServiceExportInfo.methods` already maps public method names to generated function names.
- MIR lowering already generates service helper functions and service dispatch tables.
- Service call/cast helpers already package arguments/replies in a compiler-owned way.

Planner implication:
- S02 does **not** need parser work for service support.
- The likely seam is compiler/codegen metadata and wrapper generation, not syntax.

### 7. `cluster-proof`’s current manifest is good evidence for S01, but it is not obviously the right dogfood boundary for S02.

**Files:**
- `cluster-proof/mesh.toml`

Current declarations:
- `WorkContinuity.handle_work_submit`
- `WorkContinuity.handle_work_status`
- `WorkContinuity.handle_promote`
- `WorkLegacy.handle_work_probe`

Planner implication:
- These are proof-app ingress/operator wrappers, not the milestone’s long-term “service/message/work business boundary.”
- S02 should be willing to **change the declared targets** in `cluster-proof/mesh.toml` if that is what makes the runtime-owned execution story honest.
- Do not freeze this manifest just because S01 validated it.

## Implementation Landscape

### Compiler / manifest boundary
- `compiler/mesh-pkg/src/manifest.rs`
  - Cluster manifest schema and fail-closed validation.
  - Good place to keep declaration parsing, but not enough metadata for execution.
- `compiler/meshc/src/main.rs`
  - Current validation choke point.
  - Best existing seam to derive richer clustered execution metadata after exports/typecheck.
- `compiler/mesh-lsp/src/analysis.rs`
  - S01 parity path only. Probably unaffected in S02 unless declaration semantics change.

### Runtime execution / continuity substrate
- `compiler/mesh-rt/src/dist/continuity.rs`
  - Already owns submit/dedup/reject/recovery/promotion state.
  - Strong reuse candidate for any runtime-owned declared-handler path.
- `compiler/mesh-rt/src/dist/node.rs`
  - Remote function registry, remote spawn, replica prepare, node-session continuity hooks.
  - Key constraint: actor-context requirement + registered-function-name model.

### Existing proof-app logic to delete or hollow out
- `cluster-proof/cluster.mpl`
  - Current canonical membership + placement algorithm in Mesh code.
  - Best candidate to migrate into runtime.
- `cluster-proof/work.mpl`
  - `TargetSelection`, request-key validation, target selection helpers.
  - Likely shrinks heavily once runtime owns placement.
- `cluster-proof/work_continuity.mpl`
  - Current cluster-aware orchestration layer; this is the main dogfood rewrite target.
- `cluster-proof/work_legacy.mpl`
  - Old probe path and legacy target-node logic; should not grow.

### Existing compiler/runtime features to leverage
- `compiler/mesh-typeck/src/lib.rs` `ServiceExportInfo`
- `compiler/mesh-codegen/src/mir/lower.rs` service helper / dispatch generation
- `compiler/mesh-codegen/src/codegen/mod.rs` startup function registration
- `compiler/mesh-codegen/src/codegen/expr.rs` remote spawn argument encoding

## Recommendation

### Recommended slice shape

#### 1. Start by defining a **richer clustered execution metadata model** in the compiler.
Do **not** try to reuse `ClusteredExportSurface` for execution. Keep it as the static validator. Add a second, richer representation after successful validation that includes:
- declaration kind
- manifest target
- execution symbol / generated symbol to invoke
- enough signature/ABI information to know whether the target is runtime-executable directly or needs a wrapper

This is the smallest honest seam between S01 and S02.

#### 2. Make **runtime-owned declared work** the first concrete proof path.
The runtime already has the most of the needed primitives for this:
- continuity registry
- node session transport
- remote spawn registry
- replica prepare/ack

But do **not** claim arbitrary public function execution unless the planner first chooses how declared work becomes transport-safe.

#### 3. Treat declared service handlers as a **separate compiler/codegen subproblem** inside S02.
They are not parser work; they are registration/wrapper work. The planner should isolate them from declared work execution because:
- current runtime remote spawn only knows named registered top-level functions
- generated `__service_*` handler bodies are intentionally not registered

#### 4. Rewrite `cluster-proof` to become a thin transport boundary, not a placement engine.
`cluster-proof` should keep only:
- HTTP request parsing/response shaping
- proof-specific logs
- maybe thin calls into runtime-owned declared-handler execution

It should stop computing owner/replica placement and stop bridging `Node.spawn` itself on the new path.

#### 5. Preserve the explicit declared boundary.
Per R063 and the S01 contract, only manifest-declared targets should go through the new runtime-owned path. Undeclared functions/services must stay ordinary local Mesh code.

## Natural Task Boundaries

### T1 — Compiler metadata plumbing
**Primary files:**
- `compiler/meshc/src/main.rs`
- `compiler/mesh-pkg/src/manifest.rs`
- likely `compiler/mesh-codegen/src/lib.rs` / `compiler/mesh-codegen/src/mir/*`

Deliverable:
- validated declarations survive past `meshc` validation and reach codegen/runtime registration in a richer form than name-only sets.

### T2 — Runtime placement + declared-work execution substrate
**Primary files:**
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/dist/node.rs`
- possibly a new runtime module for clustered placement/declared-handler registry

Deliverable:
- runtime can choose placement and execute the declared-work path without app-owned placement code.

### T3 — Service declaration execution wiring
**Primary files:**
- `compiler/mesh-typeck/src/lib.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/mod.rs`
- `compiler/mesh-codegen/src/codegen/expr.rs`

Deliverable:
- declared `service_call` / `service_cast` targets map to runtime-executable symbols or wrappers.

### T4 — `cluster-proof` dogfood rewrite for S02
**Primary files:**
- `cluster-proof/mesh.toml`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/work.mpl`
- `cluster-proof/cluster.mpl`
- `cluster-proof/work_legacy.mpl`
- `cluster-proof/main.mpl`

Deliverable:
- the proof app no longer computes clustered placement/dispatch on the new path.

### T5 — Verification rail
**Primary files:**
- `compiler/meshc/tests/e2e_m044_s02.rs` (new)
- `scripts/verify-m044-s02.sh` (new)
- maybe `cluster-proof/tests/work.test.mpl`

Deliverable:
- one fail-closed slice verifier with retained artifacts and non-zero test-count checks.

## Risks / Unknowns

### 1. Declared-work ABI is still unresolved.
If the planner does not settle this first, implementation will drift into ad hoc thunks.

### 2. Service declarations may be materially wider than work declarations.
The codebase already has the type info, but the runtime execution path is not symmetric today.

### 3. `cluster-proof`’s current declared targets may be the wrong dogfood proof for S02.
If they stay as HTTP handlers, the slice can accidentally re-entrench route-shaped clustering.

### 4. App-local idempotency/coordination must not grow back.
Per the distributed-systems skill, request dedup/ownership/fencing should stay centralized in the runtime continuity layer. `WorkLegacyCounter` is fine as a probe counter; it is **not** a coordination primitive.

## Verification Strategy

### New compiler/runtime rails to add
- `cargo test -p meshc --test e2e_m044_s02 -- --nocapture`
- named filters for:
  - manifest-to-execution metadata plumbing
  - declared work runtime execution
  - undeclared code stays local
  - declared service handler execution (if included in slice scope)

Per `rust-best-practices` testing guidance, keep these tests narrowly named and single-behavior so verifier filters remain stable and readable.

### Existing rails to keep replaying
- `bash scripts/verify-m044-s01.sh`
- `cargo run -q -p meshc -- build cluster-proof`
- `cargo run -q -p meshc -- test cluster-proof/tests`

### New assembled verifier shape
Create `bash scripts/verify-m044-s02.sh` that:
1. replays `bash scripts/verify-m044-s01.sh`
2. refreshes `mesh-rt`
3. runs named `e2e_m044_s02` filters and fails closed on `running 0 tests`
4. rebuilds/retests `cluster-proof`
5. retains `.tmp/m044-s02/verify/` artifacts and a phase report

### Good negative/absence checks for the verifier
If the new runtime-owned path lands, the verifier should assert that the new proof path no longer depends on app-owned placement/dispatch hot-path helpers. Candidate greps:
- no new uses of `current_target_selection(` in the declared-handler submit/status hot path
- no new direct `Node.spawn(` in the declared-handler hot path
- no new growth in `cluster-proof/cluster.mpl` placement logic

I would keep these checks narrow and path-specific so they do not block the eventual S05 rewrite on unrelated proof-only files.

## Planner Notes

- **First decision:** what exact declared-handler shapes are executable by the runtime in S02?
- **Do not over-scope into S03/S04:** built-in CLI/operator surfaces and bounded auto-promotion are separate slices.
- **Do not preserve route-shaped clustering as the public abstraction:** the current `cluster-proof` manifest is transitional.
- **Do not split coordination:** placement/dedup/fencing belong in runtime state, not half in `cluster-proof` and half in `mesh-rt`.
