# S03 Research — Built-in Operator Surfaces & Clustered Scaffold

## Requirement Focus

Primary slice ownership:
- **R065** — runtime API first, CLI second, HTTP optional for operator surfaces.
- **R066** — `meshc init --clustered` scaffolds the public clustered-app path.

Strong supporting pressure from earlier M044 requirements:
- **R052** — do not extend the proof-app env/config dialect into the public clustered story.
- **R061/R062/R063/R064** — the scaffold and operator surfaces must sit on top of the real manifest-declared/runtime-owned clustered boundary from S01/S02, not reintroduce app-owned placement, JSON shims, or undeclared handler leakage.

## Skills Discovered

- Loaded the existing **`rust-best-practices`** skill. Relevant guidance here: prefer safe typed Rust APIs over raw pointer/FFI-style calls when `meshc` talks to `mesh-rt`, keep fallible surfaces on `Result`, and verify behavior with focused named tests.
- Ran `npx skills find` for **clap** / **TOML config**. No additional directly relevant skill was installed; the work is mainly repo-local Rust/compiler/runtime design rather than framework integration.

## Summary

S03 is a **targeted but high-risk integration slice**. The declared clustered execution substrate is already real from S02, but the public operator/bootstrap story is still proof-app-shaped:

- `meshc init` still scaffolds a 2-file hello-world app.
- The only operator-style cluster status surface in-tree is still `cluster-proof`’s own `/membership`, `/work/:request_key`, and `/promote` wiring.
- The runtime exposes typed continuity values, but it does **not** yet expose a built-in structured operator bundle for membership + authority + continuity lookup/listing + failover diagnostics.
- The current clustered startup/config path is still app-owned (`cluster-proof/config.mpl`, `Node.start(...)`, `CLUSTER_PROOF_*` env), so a truthful `meshc init --clustered` cannot just copy current proof-app code without freezing the folklore path into the public product.

The main planner question is **not** “how to add a flag to `meshc init`.” It is: **what is the first-class runtime operator query surface that the scaffold and CLI are allowed to depend on?**

## Implementation Landscape

### 1. Current scaffold path is minimal and low-blast-radius

**Files:**
- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/src/main.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `README.md`
- `website/docs/docs/getting-started/index.md`
- `website/docs/docs/tooling/index.md`

**What exists now:**
- `mesh_pkg::scaffold_project(name, dir)` writes only:
  - `mesh.toml`
  - `main.mpl`
- `meshc init <name>` just calls `mesh_pkg::scaffold_project(&name, &cwd)` from `compiler/meshc/src/main.rs`.
- Tests only prove file creation / manifest contents, not a clustered scaffold contract.
- Docs everywhere still describe the hello-world 2-file shape.

**Useful constraint:**
- `scaffold_project(...)` is only called from `meshc` main and its own tests, so evolving it to support a clustered template is low-risk.

### 2. Manifest clustered config is intentionally narrow and fail-closed

**Files:**
- `compiler/mesh-pkg/src/manifest.rs`
- `cluster-proof/mesh.toml`
- `compiler/meshc/tests/e2e_m044_s01.rs`

**What exists now:**
- `[cluster]` supports only:
  - `enabled = true`
  - `declarations = [...]`
- `ClusterConfig` uses `#[serde(deny_unknown_fields)]`.
- Empty declaration arrays are rejected.
- S01/S02 validation expects a real exported target with compiler-known executable metadata.

**Planner implication:**
- A clustered scaffold cannot truthfully emit a `[cluster]` block unless it also emits at least one valid declared target and matching source file.
- The **smallest valid scaffold target is a public work function**. Service call/cast declarations are a wider seam because they rely on generated `__declared_service_*` wrappers from S02.

### 3. Runtime continuity APIs are typed, but operator APIs are incomplete

**Files:**
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/lib.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-typeck/src/builtins.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`

**What exists now:**
- Built-in Mesh-facing continuity API is already typed:
  - `Continuity.submit_declared_work(...)`
  - `Continuity.status(request_key)`
  - `Continuity.authority_status()`
  - `Continuity.promote()`
  - `Continuity.mark_completed(...)`
  - `Continuity.acknowledge_replica(...)`
- Runtime structs exist in Rust:
  - `ContinuityAuthorityStatus`
  - `ContinuityRecord`
  - `SubmitDecision`
  - `ContinuitySnapshot`
- Node runtime already exposes:
  - `mesh_node_self()`
  - `mesh_node_list()`
  - declared-handler registry / declared-work submission

**What is missing for S03:**
- No built-in **membership status struct/surface**. `Node.list()` returns peers only; `cluster-proof` has to add `self` and canonicalize membership itself.
- No built-in **operator bundle** combining membership + authority + continuity.
- No built-in **continuity listing/query surface** beyond `status(request_key)` for a single key.
- No built-in **failover diagnostics surface**. All transition detail is currently stderr log lines from `continuity.rs` (`transition=submit`, `owner_lost`, `degraded`, `promote`, `fenced_rejoin`, `stale_epoch_rejected`, etc.).
- `ContinuitySnapshot` stores only `next_attempt_token + records`; it does **not** retain an authority-only snapshot or any transition history.

**Important repo-specific rule:**
- For any new stdlib-style Mesh module or builtin structured surface, the seam is wider than one file. New public module/type work must be reflected in:
  - `compiler/mesh-typeck/src/infer.rs`
  - `compiler/mesh-typeck/src/builtins.rs`
  - `compiler/mesh-codegen/src/mir/lower.rs`
  - `compiler/mesh-rt/src/lib.rs`
  - plus any new runtime ABI / intrinsic surfaces in `mesh-rt` / codegen

### 4. Current operator truth is still app-defined in `cluster-proof`

**Files:**
- `cluster-proof/main.mpl`
- `cluster-proof/cluster.mpl`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/work.mpl`
- `cluster-proof/config.mpl`
- `cluster-proof/README.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`

**What exists now:**
- `cluster-proof/main.mpl` wires:
  - `GET /membership`
  - `GET /work`
  - `POST /work`
  - `GET /work/:request_key`
  - `POST /promote`
- `cluster-proof/cluster.mpl` computes membership payloads from `Node.self()` + `Node.list()` and canonicalizes them.
- `cluster-proof/work_continuity.mpl` wraps runtime continuity values into HTTP payloads and status codes.
- `cluster-proof/config.mpl` still owns proof-app-specific env naming and startup validation (`CLUSTER_PROOF_COOKIE`, `CLUSTER_PROOF_NODE_BASENAME`, `CLUSTER_PROOF_ADVERTISE_HOST`, `CLUSTER_PROOF_DURABILITY`, etc.).

**Planner implication:**
- These files are the **baseline to replace**, not the template to copy.
- S03 should avoid hard-coding new public surfaces around `CLUSTER_PROOF_*` or around app-owned JSON/HTTP wrappers if the slice is supposed to make operator truth built-in.

### 5. Current live query path for a future CLI is unresolved

**What the code supports today:**
- `meshc` already depends on `mesh-rt` (`compiler/meshc/Cargo.toml`), so a CLI can use runtime code directly.
- `mesh-rt` exports safe Rust structs for continuity state.
- Node transport already syncs continuity records on connect.

**What it does not support yet:**
- No dedicated operator RPC/query protocol over node transport.
- No safe Rust operator client API in `mesh-rt` for `meshc` to call.
- No structured authority sync independent of continuity records.

**Non-obvious constraint:**
- A temporary CLI node that only learns state through continuity-record sync cannot reliably learn the authoritative `cluster_role` / `promotion_epoch` when there are **zero continuity records**, because authority is not synced as a first-class queryable object.

**Result:**
- S03 needs a real runtime query surface, not just a thin CLI wrapper.

## Key Findings / Constraints

1. **`meshc init --clustered` is blocked by the manifest contract unless the template includes a real declared handler.**
   - `[cluster]` cannot be present with an empty declaration list.
   - Minimal public scaffold should prefer a declared `work` function over a declared service handler.

2. **There is no structured failover diagnostic API today.**
   - The runtime emits diagnostics to stderr only.
   - Any CLI promise to inspect failover diagnostics requires runtime-side state retention or explicit query responses, not just string parsing in tests.

3. **`Node.list()` is not a complete membership API.**
   - It returns connected peers only.
   - `cluster-proof` compensates by injecting `self` and canonicalizing. A built-in operator surface should own that normalization.

4. **Current cluster bootstrap remains app-owned.**
   - Running the same binary on two nodes still depends on proof-app config + `Node.start(...)` glue.
   - A truthful clustered scaffold needs either a standard bootstrap helper or a new public generic env/config wrapper — otherwise it just rebrands `cluster-proof` internals.

5. **S03 should stay read-only on the operator side.**
   - The roadmap/demo for S03 is inspection (`membership`, `authority`, `continuity status`, `failover diagnostics`), not mutation.
   - `Continuity.promote()` and `/promote` still exist in the proof rail, but D185 + S04 make clear that the public milestone direction is bounded automatic promotion, not new manual operator commands.
   - Do not make a new `meshc cluster promote` surface in S03.

6. **If `meshc` needs runtime data, prefer new safe Rust APIs in `mesh-rt` over calling raw `mesh_*` FFI entrypoints directly.**
   - This matches the loaded Rust guidance and keeps CLI code away from GC-allocated pointer decoding.

## Natural Seams

### Seam A — Runtime operator data model (highest risk, should land first)

Likely files:
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/lib.rs`

What belongs here:
- Membership snapshot / normalized membership truth.
- Authority snapshot as a first-class queryable runtime value.
- Continuity listing/query surface beyond single-key status if the CLI needs it.
- Retained failover diagnostic entries or another structured diagnostic bundle.
- Safe Rust-facing query helpers for `meshc`.

Why first:
- The CLI cannot be honest until the runtime exposes what it needs.
- The scaffold should target whatever this public operator contract becomes.

### Seam B — Mesh builtin/public API exposure

Likely files:
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-typeck/src/builtins.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- possibly `compiler/mesh-codegen/src/codegen/intrinsics.rs`

What belongs here:
- If S03 exposes a new Mesh-facing builtin `Cluster` / `Operator` module or new structured types, this is the registration/lowering seam.

Why separate:
- Planner can keep the Rust runtime shape stable first, then wire Mesh-facing public surface second.

### Seam C — CLI operator commands

Likely files:
- `compiler/meshc/src/main.rs`
- probably a new module such as `compiler/meshc/src/cluster.rs` or `compiler/meshc/src/operator.rs`
- `compiler/meshc/Cargo.toml` only if extra serialization/output helpers are needed

What belongs here:
- New read-only subcommand(s) for cluster inspection.
- Human-readable and/or JSON output.
- Safe calls into the new `mesh-rt` operator client/query API.

Why separate:
- `main.rs` is already large; this logic should not be piled into the existing match block if the command grows beyond a flag or two.

### Seam D — Clustered scaffold template

Likely files:
- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/mesh-pkg/src/lib.rs` (if signature/type export changes)
- `compiler/meshc/src/main.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- docs: `README.md`, `website/docs/docs/getting-started/index.md`, `website/docs/docs/tooling/index.md`

What belongs here:
- Add `--clustered` to `meshc init`.
- Emit a truthful clustered template with:
  - valid `[cluster]` declarations
  - at least one declared public work function
  - no proof-app-prefixed env/config names
  - preferably the smallest possible clustered runtime bootstrap surface

Why last:
- The scaffold should point at the real public operator/bootstrap contract, not guess it and force later rewrites.

## Recommendation

### Recommended order

1. **Define the runtime operator query surface first.**
   - This is the slice’s riskiest unknown and it governs both CLI and scaffold truth.
   - At minimum it must answer: full membership, current authority, single-key continuity status, and structured failover diagnostics.

2. **Choose the live-query transport explicitly before coding the CLI.**
   - The codebase does not already have a general-purpose live operator query path.
   - The real fork is:
     - **Option A:** new runtime/node query RPC + CLI client (more aligned with D184: runtime API first, CLI second, HTTP optional)
     - **Option B:** built-in runtime-owned HTTP operator exposure that the CLI wraps (lower implementation risk, but only acceptable if clearly layered on top of the same runtime truth and not made the primary abstraction)

3. **Only after the operator contract is real, add `meshc init --clustered`.**
   - Otherwise the scaffold will either copy `cluster-proof`’s proof-app config dialect or point at a nonexistent public operator/bootstrap surface.

### Recommended scaffold minimum

Use a **declared work handler** as the scaffold’s clustered boundary.

Reason:
- it satisfies manifest validation with the narrowest surface,
- it builds directly on S02’s declared-work path,
- it avoids the extra service-wrapper complexity unless the slice specifically wants to teach services in the template.

## Don’t Hand-Roll

- **Do not make `meshc` call raw `mesh_*` pointer-returning functions directly** if a safe Rust `mesh-rt` API can be added instead.
- **Do not copy `cluster-proof/config.mpl` into the scaffold** under a new name unless the env contract has been made public and generic.
- **Do not make new manual promotion CLI/operator surfaces in S03.** This slice’s public operator story should be read-only.
- **Do not add a new builtin Mesh module in only one place.** New stdlib-style module/type work must span typeck + builtins + MIR lowering + runtime export surfaces.

## Verification Strategy

Follow the M044 pattern: one named Rust e2e file + one fail-closed slice verifier that replays S02 first.

### Likely verification rail

Authoritative wrapper to add:
- `scripts/verify-m044-s03.sh`

Recommended phases for that wrapper:
1. `bash scripts/verify-m044-s02.sh`
2. `cargo build -q -p mesh-rt`
3. targeted scaffold/unit tests
4. targeted operator e2e tests
5. build the scaffolded clustered app
6. replay any package tests / retained artifacts needed for live operator inspection proof

### Likely test locations

- **Scaffold/template unit tests**
  - `compiler/mesh-pkg/src/scaffold.rs`
  - possibly `compiler/meshc/tests/tooling_e2e.rs`
- **New slice e2e**
  - `compiler/meshc/tests/e2e_m044_s03.rs`

### Specific proof surfaces to require

For scaffold:
- `meshc init --clustered <name>` creates the expected multi-file clustered layout.
- Generated `mesh.toml` includes a valid `[cluster]` section with at least one real declaration.
- Generated project builds successfully with `meshc build`.
- Generated project does **not** contain `CLUSTER_PROOF_*` names.

For operator/runtime truth:
- A live two-node proof can inspect full membership, authority, and continuity status without app-authored `/membership` / `/promote` wiring.
- Failover diagnostics are available through the new built-in runtime/CLI surface, not only stderr log greps.
- Named test filters must run **non-zero** tests, following the S01/S02 verifier pattern.

### Existing files/patterns to mirror

- `scripts/verify-m044-s01.sh`
- `scripts/verify-m044-s02.sh`
- `compiler/meshc/tests/e2e_m044_s01.rs`
- `compiler/meshc/tests/e2e_m044_s02.rs`

These already establish the slice-local pattern of:
- replay previous slice verifier first,
- use named test filters,
- fail closed on `running 0 tests`,
- keep retained artifacts under `.tmp/m044-s03/...` for later debugging.

## Risks / Open Questions

1. **How should the CLI query a running node?**
   - New node transport RPC/query path is more faithful to D184.
   - Built-in HTTP may be cheaper, but it must stay secondary and runtime-owned.

2. **What is the minimum public cluster bootstrap/config contract for a scaffolded app?**
   - Current repo only has proof-app-prefixed cookie/identity env names.
   - Without a generic replacement, `meshc init --clustered` risks freezing the wrong public contract.

3. **What exact shape should failover diagnostics take?**
   - Current truth is event-like transitions in stderr.
   - The runtime needs a retained/queryable representation if the CLI is supposed to inspect diagnostics after the fact.

4. **Should S03 expose continuity listing/history or only single-key status + recent diagnostics?**
   - The current builtin API only supports per-key status.
   - The planner should scope this carefully so the slice does not grow into a full control plane.

5. **Docs churn is real once the scaffold lands.**
   - `README.md`, `website/docs/docs/getting-started/index.md`, and `website/docs/docs/tooling/index.md` all currently teach the hello-world scaffold only.
   - If the public clustered scaffold is in-slice, these paths need to stay aligned with the command/help text in the same unit.
