# S01 Research — Runtime-Owned Cluster Bootstrap

## Summary

This slice primarily owns **R077**, **R079**, and **R080**, and directly supports **R078** while preserving the already-active clustered continuity/operator requirements (**R049 / R050 / R052**).

The runtime/public inspection side is already in decent shape:

- declared clustered work is runtime-owned (`Continuity.submit_declared_work`, `Continuity.status`, `Continuity.authority_status`) in `compiler/mesh-rt/src/dist/continuity.rs`
- `meshc cluster status|continuity|diagnostics` is already a built-in read-only operator surface in `compiler/meshc/src/cluster.rs`
- those CLI commands use the transient operator transport in `compiler/mesh-rt/src/dist/operator.rs` + `compiler/mesh-rt/src/dist/node.rs::execute_transient_operator_query`, so the CLI does **not** join the cluster or add a fake peer
- DNS discovery already starts automatically after `Node.start(...)` via `compiler/mesh-rt/src/dist/node.rs::mesh_node_start()` calling `compiler/mesh-rt/src/dist/discovery.rs::start_from_env()`
- runtime authority is already env-owned in `compiler/mesh-rt/src/dist/continuity.rs` (`MESH_CONTINUITY_ROLE`, `MESH_CONTINUITY_PROMOTION_EPOCH`)

The remaining bootstrap problem is narrower and more concrete than the milestone brief makes it sound:

1. **Generated clustered apps still own cluster-mode detection and startup.**
   `compiler/mesh-pkg/src/scaffold.rs` generates a `main.mpl` that reads `MESH_CLUSTER_COOKIE`, `MESH_DISCOVERY_SEED`, `MESH_NODE_NAME`, `MESH_CLUSTER_PORT`, branches between standalone/cluster mode, and calls `Node.start(...)` directly.

2. **`cluster-proof` still duplicates bootstrap logic heavily.**
   The package still owns:
   - `cluster-proof/main.mpl` (startup orchestration)
   - `cluster-proof/config.mpl` (env parsing / identity / topology validation)
   - `cluster-proof/docker-entrypoint.sh` (a second shell-level copy of the same validation)

3. **The repo’s current regression rails lock that manual bootstrap shape in place.**
   Existing tests and verifiers explicitly assert that the scaffolded `main.mpl` contains the `MESH_*` env reads. Any runtime-owned bootstrap change will need those rails updated in the same task.

The highest-value seam for S01 is therefore **a new public runtime-owned bootstrap API exposed to Mesh code**, then migrating the scaffold to it. If that API is good, `cluster-proof` becomes the natural second consumer. If it is not good enough for `cluster-proof`, it is not yet the right public surface.

## Requirements Targeted

### Primary
- **R077** — primary clustered docs example must become tiny and language-first
- **R079** — example apps must contain no app-owned clustering, failover, routing-choice, load-balancing, or status-truth logic
- **R080** — `meshc init --clustered` must become the primary docs-grade clustered example surface

### Supported / protected
- **R078** — one local example must still prove cluster formation, remote execution, and failover end to end
- **R049 / R050 / R052** — existing clustered continuity/operator truth must stay honest while bootstrap moves downward

### Not primary in this slice
- **R081** (docs-teach-simple-example-first) matters, but the low-level docs rewrite can wait until the bootstrap surface is stable. S01 should not spend its risk budget on docs-first cleanup before the runtime API exists.

## Skills Discovered

### Loaded
- **rust-best-practices**
  - Relevant rules for this slice:
    - prefer explicit `Result<T, E>` error paths over panics for runtime/bootstrap validation
    - public API changes should be documented clearly at the API boundary, not via scattered inline comments
    - add focused tests for invalid env/topology matrices instead of only one broad golden-path rail

### Additional skill discovery
- Ran `npx skills find "mesh language runtime"`
- No directly relevant additional skill surfaced; nothing worth installing for this slice. The work is repo-specific compiler/runtime/bootstrap plumbing.

## Current Implementation Landscape

### 1. What already belongs to the runtime/public surface

#### Built-in operator inspection is already the right boundary
- `compiler/meshc/src/cluster.rs`
  - user-facing `meshc cluster status|continuity|diagnostics`
  - emits either human-readable or JSON output
- `compiler/mesh-rt/src/dist/operator.rs`
  - `query_operator_status_remote`
  - `query_operator_continuity_status_remote`
  - `query_operator_diagnostics_remote`
  - builds runtime-backed status snapshots from membership + continuity authority
- `compiler/mesh-rt/src/dist/node.rs`
  - `execute_transient_operator_query(...)` performs a one-shot authenticated query transport
  - the target node treats these transient clients specially and does not register them as peers

This is already the correct answer for the “inspection path should be runtime/public-surface owned” part of the milestone. S01 should reuse it, not replace it.

#### Declared clustered work is already runtime-owned
- `compiler/mesh-rt/src/dist/continuity.rs`
  - `mesh_continuity_submit_declared_work`
  - `mesh_continuity_authority_status`
  - `ContinuityRegistry::authority_status()`
- `compiler/meshc/src/main.rs`
  - manifest declarations are validated into `clustered_execution_plan`
  - declared handler symbols are rooted and registered during codegen
- `compiler/mesh-codegen/src/codegen/mod.rs`
  - runtime registration via `mesh_register_declared_handler`

This means S01 does **not** need to reopen the declared-handler or operator-query architecture. The ownership gap is startup/bootstrap.

#### Discovery is already partially lower than the app
- `compiler/mesh-rt/src/dist/discovery.rs`
  - `DiscoveryConfig::from_env(...)`
  - `start_from_env()`
- `compiler/mesh-rt/src/dist/node.rs::mesh_node_start(...)`
  - after listener init, it calls `start_discovery_from_env()`

So the runtime already owns: “once a node exists, inspect `MESH_DISCOVERY_SEED` and begin reconciling peers.”

That narrows the missing piece to: **who decides whether to start a node at all, how to derive the node identity, and how to fail closed on malformed cluster env.**

### 2. What is still app-owned today

#### Scaffolded clustered app still performs manual bootstrap
- `compiler/mesh-pkg/src/scaffold.rs`
  - generated `main.mpl` contains:
    - `current_http_port()`
    - `current_cluster_port()`
    - `current_discovery_seed()`
    - `current_node_name(...)`
    - `start_cluster(...)`
    - cluster/standalone branching based on `MESH_CLUSTER_COOKIE`
    - direct `Node.start(advertised_node_name, cluster_cookie)`

The generated scaffold is already smaller than `cluster-proof`, but it still reads like a tiny hand-built operator.

#### Existing scaffold assertions explicitly pin the old shape
These will fail the moment bootstrap moves lower unless updated together:

- `compiler/mesh-pkg/src/scaffold.rs::scaffold_clustered_project_writes_public_cluster_contract`
- `compiler/meshc/tests/tooling_e2e.rs::test_init_clustered_creates_project`
- `scripts/verify-m044-s03.sh::assert_scaffold_contract`
  - currently requires scaffold `main.mpl` to contain `MESH_CLUSTER_COOKIE`, `MESH_NODE_NAME`, `MESH_DISCOVERY_SEED`, `MESH_CLUSTER_PORT`

Those are not future-proof tests of the public contract; they are tests of the current implementation shape.

#### `cluster-proof` still duplicates bootstrap in three layers
File map:

- `cluster-proof/main.mpl` — 189 lines
- `cluster-proof/config.mpl` — 722 lines
- `cluster-proof/docker-entrypoint.sh` — 372 lines

That is **1,283 lines** of startup/config/validation surface before touching the continuity route logic.

What those files currently own:

- mode detection (`standalone` vs `cluster`)
- cluster cookie presence rules
- `MESH_NODE_NAME` validation
- Fly identity fallback (`FLY_APP_NAME`, `FLY_REGION`, `FLY_MACHINE_ID`, `FLY_PRIVATE_IP`)
- topology validation for `MESH_CONTINUITY_ROLE` / `MESH_CONTINUITY_PROMOTION_EPOCH`
- direct `Node.start(...)`
- shell-level duplication of the same contract before the Mesh binary starts

This is the strongest evidence that clustered bootstrap is still too example-shaped.

#### `mesher` is a real second consumer of the same smell
- `mesher/main.mpl`
  - reads `MESHER_NODE_NAME`, `MESHER_COOKIE`
  - calls `Node.start(...)`
  - owns “standalone mode vs distributed mode” branching

`mesher` is not the primary S01 acceptance surface, but it matters as a reuse signal. If S01 extracts a real runtime bootstrap seam, `mesher` is an obvious future adopter.

### 3. What is adjacent but should not dominate S01

#### `cluster-proof/work_continuity.mpl` is still large, but it is not primarily bootstrap
- `cluster-proof/work_continuity.mpl` — 738 lines
- owns submit/status HTTP parsing, response shaping, log translation, and work completion
- uses runtime-owned `Continuity.*` APIs already

This file is still a cleanup target later in M045, but it is not the first seam for S01.

#### `cluster-proof/cluster.mpl` still shapes membership JSON
- `cluster-proof/cluster.mpl` — 363 lines
- still computes payload fields and even string-replaces `"node":` to `"self":` after JSON encode

That is inspection payload ownership, not bootstrap ownership. It is real cleanup work, but not the first blocker for the scaffold getting smaller.

#### Low-level distributed docs still teach raw `Node.start(...)`
- `website/docs/docs/distributed/index.md`
  - multiple `Node.start(...)` examples

That is compatible with the current split: low-level docs teach primitives, scaffold/docs-grade clustered app teaches the higher-level public path. It does **not** need to block S01.

## Recommendation

### Recommended API direction
Add a **new public runtime-owned Mesh bootstrap surface** instead of forcing scaffolded apps to hand-roll cluster startup.

The safest shape is:

- keep low-level `Node.start(name, cookie)` for primitive/distribution docs and explicit apps
- add a higher-level convenience bootstrap call for clustered apps, e.g. `Node.start_from_env()` or equivalent
- expose it as a typed `Result<BootstrapStatus, String>` rather than another stringly/int-only contract

A typed status should carry at least:
- `mode` (`standalone` / `cluster`)
- `node_name` (empty in standalone is acceptable)
- `cluster_port`
- `discovery_seed`

Reasoning:
- a pure `Int` return only moves the `Node.start(...)` call; it does **not** remove app-owned env parsing/logging
- a typed status lets scaffold and `cluster-proof` shrink without immediately reintroducing side-channel env reads
- the repo already has the pattern for runtime-exported typed payloads in the `Continuity*` surfaces

If scope becomes tight, the fallback is a lighter `Result<String, String>` shape (actual node name or empty string), but that is a compromise, not the best public contract.

### Ownership boundary to enforce
The new runtime/bootstrap surface should own:
- standalone vs cluster mode detection
- cookie/seed/identity validation
- node identity resolution (`MESH_NODE_NAME`, and Fly identity only if the slice chooses to migrate `cluster-proof` now)
- calling existing `mesh_node_start`
- fail-closed errors

The app should still own:
- its HTTP routes
- its application-specific `PORT`
- its business logic

### What **not** to hand-roll again
Do **not**:
- create another Mesh-side `Config` helper module in the scaffold
- move the same contract into another shell wrapper
- add scaffold-specific `/membership` or operator routes instead of using `meshc cluster ...`

Per the Rust skill’s guidance, the runtime parser/validator should be a pure Rust helper returning `Result`, with focused unit tests around env matrices. Do not bury the rules inside listener setup or process startup side effects.

## Natural Task Seams

### Seam 1 — Runtime bootstrap core (highest risk, do first)
**Goal:** one Rust-owned parser/start helper for the public clustered-app env contract.

Likely files:
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/dist/discovery.rs`
- `compiler/mesh-rt/src/dist/continuity.rs` (reuse/reference only; authority env already lives here)
- possibly a new `compiler/mesh-rt/src/dist/bootstrap.rs`
- `compiler/mesh-rt/src/lib.rs`

What to build first:
- pure env parsing + validation helpers
- typed bootstrap status object
- runtime export that calls existing `mesh_node_start`

Why first:
- every downstream code path depends on the final helper shape
- this is where the real ownership boundary changes

### Seam 2 — Compiler/public API exposure
**Goal:** make the new runtime helper callable from Mesh code.

Likely files:
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-typeck/src/builtins.rs` if new builtin names/types need module registration there
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-codegen/src/codegen/expr.rs`

Notes:
- this is the standard stdlib/runtime seam already used by `Node.*` and `Continuity.*`
- if the helper introduces a new public typed payload, follow the same pattern the continuity payloads use

### Seam 3 — Scaffold migration and regression updates
**Goal:** make `meshc init --clustered` visibly smaller and runtime-owned.

Likely files:
- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/mesh-pkg/src/scaffold.rs` tests
- `compiler/meshc/tests/tooling_e2e.rs`
- `compiler/meshc/tests/e2e_m044_s03.rs`
- `scripts/verify-m044-s03.sh`

Important:
- the behavior rail in `m044_s03_scaffold_generated_project_builds_and_reports_runtime_truth` should mostly survive
- the source-shape assertions must change, because they currently require manual env parsing in scaffold `main.mpl`

### Seam 4 — Secondary consumer migration (optional in S01, but natural next step)
**Goal:** prove the helper is real by removing duplicate bootstrap from `cluster-proof`.

Likely files:
- `cluster-proof/main.mpl`
- bootstrap-only parts of `cluster-proof/config.mpl`
- `cluster-proof/docker-entrypoint.sh`
- `cluster-proof/tests/config.test.mpl`

Scope warning:
- `config.mpl` is mixed-use today. Some of it is bootstrap, some of it still feeds continuity submit policy (`durability_policy`, `required_replica_count`).
- Do not assume the whole file can disappear in one move.

## What To Build / Prove First

1. **Choose and freeze the public bootstrap API shape.**
   Everything else depends on this. Do not start with scaffold edits.

2. **Prove the helper in Rust before wiring it into Mesh.**
   Focus on malformed env matrices, not only the happy path.

3. **Expose it to Mesh and switch the scaffold.**
   This is the user-visible win and the primary acceptance surface for S01.

4. **Only then decide whether `cluster-proof` should migrate in this slice or later.**
   If the helper cannot replace `cluster-proof/main.mpl` cleanly, that is useful evidence that the public surface is still too weak.

## Verification Plan

### Must-have regressions after scaffold migration
- `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture`
- `bash scripts/verify-m044-s03.sh` (or the M045 replacement, if the planner decides to fork rather than mutate)

### Public contract rails worth preserving
If the helper preserves the existing `MESH_*` public contract, replay:
- `cargo test -p meshc --test e2e_m044_s05 m044_s05_public_contract_ -- --nocapture`

At minimum, the two fail-closed cases remain relevant:
- `m044_s05_public_contract_old_bootstrap_env_names_fail_closed`
- `m044_s05_public_contract_malformed_inputs_fail_closed`

### If `cluster-proof` bootstrap migrates in this slice
Replay:
- `cargo run -q -p meshc -- build cluster-proof`
- `cargo run -q -p meshc -- test cluster-proof/tests`

### New coverage S01 probably needs
Existing rails prove behavior, but not the new bootstrap boundary. Add focused tests for:
- standalone mode with no cluster env
- cluster mode with valid `MESH_NODE_NAME`
- cluster mode with missing cookie / blank seed / malformed node name
- if in-scope: Fly identity fallback
- typed bootstrap return shape in Mesh code (not only Rust unit tests)

## Key Risks / Constraints

### 1. Existing M044 rails pin implementation details, not just behavior
Changing the scaffold without updating:
- `compiler/mesh-pkg/src/scaffold.rs` tests
- `compiler/meshc/tests/tooling_e2e.rs`
- `scripts/verify-m044-s03.sh`
will create false regressions.

### 2. `cluster-proof/config.mpl` is not purely bootstrap
A blunt delete will break submit-policy logic and package tests. Split bootstrap ownership from continuity-policy ownership.

### 3. Fly support is the tricky edge
If S01 migrates `cluster-proof` to the new helper, the helper must either:
- preserve Fly identity composition now, or
- explicitly defer `cluster-proof` migration

Because the existing read-only Fly evidence path still depends on that identity surface.

### 4. Do not regress to app-owned inspection
The CLI/operator path is already correct and more truthful than app-owned `/membership` routes. S01 should reduce app bootstrap, not add new app-level operator surfaces.

## Concrete File Map

### Primary runtime/compiler seam
- `compiler/mesh-rt/src/dist/node.rs` — current low-level start/connect entry points; discovery start happens here
- `compiler/mesh-rt/src/dist/discovery.rs` — env-backed discovery config already exists
- `compiler/mesh-rt/src/dist/continuity.rs` — authority env parsing already lives here; good reference pattern
- `compiler/meshc/src/cluster.rs` — built-in inspection CLI; already the right boundary
- `compiler/meshc/src/main.rs` — build path roots declared handlers; probably not central unless new stdlib surface needs wiring
- `compiler/mesh-typeck/src/infer.rs` — `Node` module function signatures live here
- `compiler/mesh-codegen/src/mir/lower.rs` — builtin name -> runtime intrinsic mapping
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — runtime intrinsic declarations
- `compiler/mesh-codegen/src/codegen/expr.rs` — custom codegen for `Node.start(...)`

### Scaffold/user-facing surface
- `compiler/mesh-pkg/src/scaffold.rs` — current clustered scaffold generation and unit tests
- `compiler/meshc/tests/tooling_e2e.rs` — current init smoke rail
- `compiler/meshc/tests/e2e_m044_s03.rs` — scaffold behavior + runtime truth rail
- `scripts/verify-m044-s03.sh` — assembled public clustered-app/operator rail

### Secondary proof consumer
- `cluster-proof/main.mpl` — startup orchestration using app-owned config
- `cluster-proof/config.mpl` — mixed bootstrap + continuity-policy helpers
- `cluster-proof/docker-entrypoint.sh` — duplicated shell-level bootstrap validation
- `cluster-proof/tests/config.test.mpl` — current config helper truth
- `cluster-proof/tests/work.test.mpl` — mostly continuity/payload behavior, not the first S01 seam

### Later-docs / not first slice
- `website/docs/docs/distributed/index.md` — low-level primitive docs still teach `Node.start(...)`
- `README.md` — already points users to `meshc init --clustered` + `meshc cluster ...`

## Planner Recommendation

Default decomposition:

1. **Runtime bootstrap helper + Rust unit tests**
2. **Compiler exposure for Mesh code**
3. **Scaffold migration + update old source-shape assertions**
4. **Optional `cluster-proof` migration only if helper is already strong enough**

If time/risk is tight, stop after step 3. That still delivers the slice’s explicit user-visible outcome: `meshc init --clustered` becomes visibly smaller and more runtime-owned.

If the helper lands cleanly and still leaves `cluster-proof` on `config.mpl` + shell validation, that is acceptable evidence to carry into S02/S04 — but the planner should treat that as **deferred cleanup**, not as proof that bootstrap is fully language-owned yet.
