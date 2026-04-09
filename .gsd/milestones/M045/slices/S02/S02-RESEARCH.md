# S02 Research — Tiny End-to-End Clustered Example

## Summary

S02 is **not** just a scaffold cleanup pass. The repo already has the runtime-owned bootstrap surface from S01, and the runtime already owns the important cluster truth surfaces (`meshc cluster status|continuity|diagnostics`, `ContinuityRecord`, runtime placement, authority, and continuity state). But the current scaffolded clustered example still fails the actual end-to-end story in two concrete ways:

1. **Local-owner submits never complete** in the generated example. The scaffolded `work.mpl` handler returns an `Int` and nothing in the generated app or current declared-work wrapper path calls `Continuity.mark_completed(...)`, so continuity stays `phase=submitted`, `result=pending`, `execution_node=""`.
2. **Remote-owner submits are rejected** in the generated example. A real two-node probe showed remote-owner keys fail with `declared_work_remote_spawn_failed:node-b@[::1]:...:__declared_work_work_execute_declared_work`, and the owner node logs `mesh node spawn rejected ... function not found __declared_work_work_execute_declared_work`.

That makes S02 a **runtime/codegen seam first**, then a scaffold/example simplification task.

## Requirements Targeted

Primary:
- **R078** — one local example must prove cluster formation and runtime-chosen remote execution end to end.

Directly supported:
- **R077** — the primary clustered docs example must become tiny and language-first.
- **R079** — example apps must not reintroduce app-owned routing-choice or status-truth logic.
- **R080** — `meshc init --clustered` must become the primary docs-grade clustered example surface.

## Skills Discovered

Installed during research:
- **distributed-systems** (`yonatangross/orchestkit@distributed-systems`) — directly relevant to clustered request identity and proof design.

Loaded and relevant:
- **rust-best-practices** — useful for the runtime/codegen seam. The most relevant rules here are:
  - prefer explicit `Result`-based error surfaces over panic/unwrap at runtime boundaries;
  - keep public/runtime API behavior observable through tests, not inferred from happy-path logs.

Implementation guidance taken from skills:
- From **rust-best-practices / Chapter 4**: keep the runtime/codegen seam fail-closed with explicit `Result` errors. The existing `declared_work_remote_spawn_failed:...` surface is the right kind of diagnostic; S02 should preserve and test that style rather than hiding failures behind fallback behavior.
- From **distributed-systems / idempotency-keys**: keep `request_key` as the single deterministic idempotency key. Do **not** invent a second example-side retry token, placement hint, or client-generated randomness just to make the tiny example easier to demo.

## Implementation Landscape

### 1. Current scaffold shape is small, but not end-to-end truthful yet

**File:** `compiler/mesh-pkg/src/scaffold.rs`

Current generated clustered project:
- `main.mpl`
  - calls `Node.start_from_env()`;
  - serves `GET /health`;
  - serves only `POST /work/:request_key`;
  - submits through `Continuity.submit_declared_work(...)`;
  - returns only `request_key`, `attempt_id`, and `outcome` in the submit response.
- `work.mpl`
  - exposes `declared_work_target()`;
  - defines `execute_declared_work(request_key, attempt_id) -> Int` as a pure length calculation.

Important consequence:
- the generated handler does **not** call `Continuity.mark_completed(...)` or emit any execution log;
- there is **no** scaffold-owned status route today;
- the current scaffold test surface only proves bootstrap + `/health` + `meshc cluster status`, not actual work completion or remote execution.

### 2. Runtime continuity and operator truth already exist and are sufficient for the tiny example

**Files:**
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/meshc/src/cluster.rs`

Relevant surfaces already available:
- `ContinuityRecord` already carries:
  - `owner_node`
  - `replica_node`
  - `execution_node`
  - `routed_remotely`
  - `fell_back_locally`
  - `cluster_role`
  - `promotion_epoch`
  - `replication_health`
  - `error`
- `meshc cluster continuity <node> <request_key> --json` already exposes that record directly.
- `meshc cluster status <node> --json` already exposes runtime-owned membership + authority.

Planner implication:
- S02 does **not** need to port `cluster-proof`’s full JSON/status shaping into the scaffold.
- The smallest truthful public example can rely on the built-in CLI for cluster/status truth.

### 3. `cluster-proof` is no longer the owner of placement/authority truth, but it still owns the only end-to-end completion wrapper

**Files:**
- `cluster-proof/main.mpl`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/work.mpl`
- `cluster-proof/cluster.mpl`

Current state:
- `cluster-proof/main.mpl` is already aligned with S01: startup via `Node.start_from_env()`.
- `cluster-proof/work_continuity.mpl` is still the only shipped Mesh example that:
  - submits via `Continuity.submit_declared_work(...)`;
  - marks completion via `Continuity.mark_completed(...)` inside `complete_work_execution(...)`;
  - exposes status through `GET /work/:request_key`.
- `cluster-proof/cluster.mpl`’s old placement helpers are **not** on the current submit hot path anymore. It is now mostly membership payload shaping + `Node.self()`/`Node.list()` helpers.

Planner implication:
- S02 probably does **not** need to touch `cluster-proof/cluster.mpl` unless the planner decides the tiny example must expose a membership HTTP route.
- The real reusable reference in `cluster-proof` is the **minimal completion wrapper pattern**, not the old placement logic.

### 4. The declared-work remote path currently looks broken for scaffolded apps

**Files:**
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-codegen/src/codegen/mod.rs`
- `compiler/mesh-codegen/src/declared.rs`

Current runtime/codegen chain:
- `submit_declared_work(...)` in `node.rs` computes runtime-owned placement and dispatches:
  - local owner -> `spawn_declared_work_local(...)`
  - remote owner -> `spawn_declared_work_remote(...)`
- `spawn_declared_work_remote(...)` calls `mesh_node_spawn(...)` with `entry.executable_name`.
- The declared-work wrapper names generated in `declared.rs` are `__declared_work_*`.
- In `codegen/mod.rs::generate_main_wrapper(...)`, generic remote function registration skips every function whose name starts with `__`.
- Declared handlers are separately registered via `mesh_register_declared_handler(...)`, but remote `mesh_node_spawn(...)` still expects the target function to be present in the generic remote function registry.

This matches the probe failure exactly:
- remote owner selected by runtime;
- owner node rejects the remote spawn with `function not found __declared_work_work_execute_declared_work`.

Planner implication:
- highest-risk task is a **runtime/codegen registration repair**.
- The next agent should start in:
  - `compiler/mesh-codegen/src/codegen/mod.rs::generate_main_wrapper`
  - `compiler/mesh-codegen/src/declared.rs::generate_declared_work_wrapper`
  - `compiler/mesh-rt/src/dist/node.rs::{spawn_declared_work_remote, submit_declared_work}`

### 5. Completion ownership is still unresolved for the tiny public example

**Files:**
- `compiler/mesh-pkg/src/scaffold.rs`
- `cluster-proof/work_continuity.mpl`
- `compiler/mesh-codegen/src/declared.rs`

Even if the remote registry seam is fixed, the scaffold still lacks a completion path.

Current options:

**Option A — runtime/codegen-owned completion**
- extend the declared-work wrapper/runtime path so a successful declared-work run auto-updates continuity completion.
- best match for the milestone’s language-owned example goal.
- likely touches codegen/runtime, not just scaffold.

**Option B — tiny example-side completion shim**
- generate a minimal work wrapper in scaffolded `work.mpl` that logs execution and calls `Continuity.mark_completed(...)`, similar to `cluster-proof` but much smaller.
- faster and lower-risk for S02 specifically.
- but it keeps some status-truth glue in app code, which pushes against the full M045 direction and may need removal/collapse in S04.

My read: S02 should **repair the remote registry seam first**, then choose between A and B explicitly. If the planner wants the smallest risk path for this slice only, B is viable; if it wants to stay truest to M045’s end-state, A is better.

## Reproduced Evidence

### Manual two-node scaffold probe

Probe project:
- `.tmp/m045-s02-probe-11bzzngj/scaffolded`

Commands used:
- `cargo run -q -p meshc -- init --clustered scaffolded`
- `cargo run -q -p meshc -- build scaffolded`

Two-node run (same-host IPv4/IPv6 split, same cluster port):
- `node-a@127.0.0.1:54648` on HTTP `54646`
- `node-b@[::1]:54648` on HTTP `54647`

Confirmed good:
- `meshc cluster status node-a@127.0.0.1:54648 --json` showed both nodes in membership.
- submit requests from the scaffolded app were accepted and produced continuity records.

Confirmed broken:
- `req-0` stayed:
  - `phase: submitted`
  - `result: pending`
  - `execution_node: ""`
  - even after waiting 3 seconds and re-querying with `meshc cluster continuity ... --json`.
- Remote-owner records (`remote-1`, `remote-11`, `remote-14`, etc.) were rejected with:
  - `error: declared_work_remote_spawn_failed:node-b@[::1]:54648:__declared_work_work_execute_declared_work`
  - `routed_remotely: true`
- Owner node log reported:
  - `mesh node spawn rejected from node-a@127.0.0.1:54648: function not found __declared_work_work_execute_declared_work`

This is the current truthful reproduction surface for S02.

## Don’t Hand-Roll

1. **Do not copy `cluster-proof/work_continuity.mpl`’s full status payload logic into the scaffold.**
   The tiny example should consume runtime-owned truth through `meshc cluster continuity` unless a tiny HTTP status route is strictly necessary.

2. **Do not use the M044 failover harness’s local placement reimplementation for the public tiny-example test.**
   `compiler/meshc/tests/e2e_m044_s04.rs::find_submit_matching_placement(...)` duplicates the runtime hash. For S02, prefer the more honest M042 pattern: retry submits until the runtime chooses a remote owner, then trust the returned/runtime-reported `owner_node`.

3. **Do not add example-side owner/replica selection hints.**
   Runtime placement already lives in `compiler/mesh-rt/src/dist/node.rs::declared_work_placement(...)`; the tiny example should prove it, not teach it.

## Recommendation

### Recommended build order

1. **Fix or prove the remote declared-work dispatch seam**
   - unblock remote-owner execution for scaffolded apps.
   - do this before any example/docs cleanup, because S02 cannot honestly land without it.

2. **Resolve completion ownership for declared work**
   - preferred: runtime/codegen-owned completion;
   - acceptable fallback for S02 only: a very small scaffolded completion shim modeled on `cluster-proof`’s `complete_work_execution(...)`.

3. **Shrink the scaffolded example to the smallest truthful surface**
   - keep `Node.start_from_env()` + minimal HTTP submit route + minimal work logic;
   - rely on `meshc cluster status` / `meshc cluster continuity` for operator truth.

4. **Add a new two-node scaffolded-app e2e + verifier**
   - this should become the S02 authoritative proof surface.

### Natural task seams

#### Seam 1 — runtime/codegen declared-work dispatch repair
Likely files:
- `compiler/mesh-codegen/src/codegen/mod.rs`
- `compiler/mesh-codegen/src/declared.rs`
- `compiler/mesh-rt/src/dist/node.rs`
- possibly `compiler/meshc/tests/e2e_m044_s02.rs` if the registration contract needs new coverage

Goal:
- remote declared-work owner can actually spawn the generated wrapper on the owner node.

#### Seam 2 — completion ownership + tiny scaffold shape
Likely files:
- `compiler/mesh-pkg/src/scaffold.rs`
- maybe `compiler/mesh-codegen/src/declared.rs` / runtime if completion becomes automatic
- maybe a tiny shared example helper if the planner prefers not to repeat completion glue

Goal:
- generated clustered app can submit work and eventually surface `execution_node`/completed truth through runtime continuity.

#### Seam 3 — two-node public proof rail
Likely files:
- **new** `compiler/meshc/tests/e2e_m045_s02.rs`
- **new** `scripts/verify-m045-s02.sh`
- maybe `compiler/meshc/tests/tooling_e2e.rs` or `compiler/meshc/tests/e2e_m045_s01.rs` for source-shape expectations if scaffold output changes

Goal:
- scaffold-first example, two nodes, runtime-chosen remote execution, no app-owned placement logic, fail-closed test filter behavior.

## Verification Plan

Minimum truthful S02 verification should include:

1. **Existing bootstrap/source rails still green**
- `cargo test -p meshc --test e2e_m045_s01 m045_s01_ -- --nocapture`
- `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`

2. **New two-node scaffold rail**
- likely: `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture`
- must fail closed on `running N test` / zero-test drift.

3. **New assembled verifier**
- likely: `bash scripts/verify-m045-s02.sh`
- should probably replay S01 first, then the new S02 e2e, and retain copied artifacts.

4. **What the new e2e must prove explicitly**
- scaffolded app builds from `meshc init --clustered` output;
- two nodes converge in `meshc cluster status --json`;
- at least one submit becomes a **remote-owner** record chosen by runtime (no local placement reimplementation);
- continuity eventually reports:
  - `phase=completed`
  - `result=succeeded`
  - `execution_node == owner_node`
  - `routed_remotely == true` for the remote-owner case;
- source contract does not reintroduce app-owned bootstrap logic or placement helpers.

## Resume Notes

Start the next unit here, in order:
1. `compiler/mesh-codegen/src/codegen/mod.rs::generate_main_wrapper(...)`
   - confirm whether declared-work wrappers need generic remote registration despite the `__*` skip.
2. `compiler/mesh-rt/src/dist/node.rs::{spawn_declared_work_remote, submit_declared_work}`
   - verify the runtime expects `entry.executable_name` to be remote-spawnable through the generic registry.
3. `compiler/mesh-codegen/src/declared.rs::generate_declared_work_wrapper(...)`
   - decide whether completion should be auto-owned here/runtime-side or left to scaffold code for S02.
4. `compiler/mesh-pkg/src/scaffold.rs`
   - only after the runtime/codegen path is decided.

Useful existing patterns:
- **Use** `compiler/meshc/tests/e2e_m042_s01.rs::wait_for_remote_owner_submit(...)` as the honest remote-owner selection pattern.
- **Do not use** `compiler/meshc/tests/e2e_m044_s04.rs::find_submit_matching_placement(...)` for the public tiny-example rail unless you intentionally want a lower-level runtime-placement test.

Probe artifacts remain on disk under:
- `.tmp/m045-s02-probe-11bzzngj/scaffolded`

Probe background processes were killed before wrap-up.