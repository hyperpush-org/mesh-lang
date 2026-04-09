# M044 — Research

**Date:** 2026-03-28

## Summary

S01 most directly advances **R061**, **R062**, and **R063**, and it sets up the later runtime-owned execution work for **R064** while supporting the continuity/productization requirements behind **R049**, **R050**, and **R052**.

The main findings are sharper than the roadmap wording suggests:

1. **`mesh.toml` is not part of the compiler build contract today.**
   - `compiler/mesh-pkg/src/manifest.rs` defines the only shared manifest schema, but it only models `[package]` and `[dependencies]`.
   - `compiler/meshc/src/main.rs::build(...)` only requires `main.mpl`; it never reads `mesh.toml`.
   - `cluster-proof/` does not have a `mesh.toml` at all and still builds because `meshc build cluster-proof` is manifest-agnostic.

2. **The compiler already has a real static-validation seam for clustered declarations — but only for some handler kinds.**
   - `compiler/mesh-parser/src/ast/item.rs` exposes `ServiceDef`, `call_handlers()`, and `cast_handlers()`.
   - `compiler/mesh-typeck/src/infer.rs::infer_service_def(...)` already resolves service helper types and method names.
   - `compiler/mesh-typeck/src/lib.rs` exports `ServiceExportInfo` with `(method_name, generated_fn_name)` mappings.
   - That means service/call/cast declarations can be validated against existing compiler knowledge.
   - **But there is no first-class compiler concept named “work handler” today.** Generic public functions and actors are exported, but only service handlers have a dedicated handler/method model.

3. **The runtime already owns typed continuity state; only the Mesh-facing ABI is stringly.**
   - `compiler/mesh-rt/src/dist/continuity.rs` already has typed Rust structs and enums: `ContinuityRecord`, `ContinuityAuthorityStatus`, `SubmitDecision`, `SubmitRequest`, `SubmitOutcome`, `ReplicaStatus`, etc.
   - The FFI boundary currently serializes those structs to JSON strings and returns `MeshResult<String, String>`.
   - `compiler/mesh-typeck/src/infer.rs` hard-codes the Mesh `Continuity` module as `String ! String` everywhere.
   - `cluster-proof/work_continuity.mpl` is therefore full of adapter code: `parse_authority_status_json(...)`, `parse_continuity_submit_response(...)`, `parse_continuity_record(...)`, and the `Continuity.*` wrappers that decode JSON back into Mesh structs.

4. **There is already a proven pattern for typed builtin struct results.**
   - `Http.send(...) -> Result<HttpResponse, String>` is the existing precedent.
   - `compiler/mesh-typeck/src/infer.rs` pre-registers builtin struct field access for `HttpResponse`.
   - `compiler/mesh-codegen/src/mir/lower.rs` pre-seeds a matching `MirStructDef` for `HttpResponse`.
   - `compiler/mesh-rt/src/http/client.rs` heap-allocates the response struct and returns it via `alloc_result(...)`.
   - `compiler/mesh-codegen/src/codegen/pattern.rs` already handles `Result<Struct, String>` payload boxing/unboxing correctly.
   - So S01 does **not** need a new compiler capability to return typed continuity structs — it needs the continuity surface migrated onto an existing capability.

5. **There is a compatibility trap: do not make manifests mandatory for all builds.**
   - Many compiler e2e helpers intentionally build temp projects with only `main.mpl`; e.g. `compiler/meshc/tests/e2e_m043_s02.rs::build_only_mesh(...)` writes `main.mpl` and calls `meshc build` without a manifest.
   - If S01 makes `mesh.toml` required globally, it will break unrelated compiler proof rails.
   - The honest contract is: **manifest-aware when present, clustered opt-in when declared, non-manifest builds still valid**.

6. **S01 should stay out of later-slice scope.**
   - `compiler/mesh-pkg/src/scaffold.rs` and `compiler/meshc/tests/tooling_e2e.rs` currently cover only plain `meshc init`; that is a real seam, but **R066 / `meshc init --clustered` belongs to S03, not S01**.
   - Likewise, **D184** says runtime API first, CLI second, HTTP optional; S01 should not turn HTTP payloads into the primary abstraction.
   - **D185** says auto-promotion only, no manual operator override. That means typing `Continuity.promote()` may be necessary for compatibility with current M043 proof rails, but it should not define the long-term public clustered-app model.

Two loaded skill rules matter directly here:
- From **`rust-best-practices`**: encode valid public states in the type system, and keep public fallible APIs as `Result` rather than panic surfaces. That argues for replacing stringly continuity JSON with typed result values.
- From the installed **`distributed-systems`** skill, the fencing-token rule maps directly onto `promotion_epoch`: it is not incidental metadata; it is part of the safety contract and should stay explicit in the typed public surface.

## Recommendation

1. **Keep S01 scoped to manifest opt-in, declaration validation, and typed continuity values.**
   Do **not** pull `meshc init --clustered`, CLI operator surfaces, or the final auto-promotion operator story into this slice. Those belong to S03/S04.

2. **Add an optional top-level clustered-app section to the shared manifest schema, but keep non-cluster builds working.**
   `mesh_pkg::Manifest` is the right schema owner, but `meshc build` must treat the manifest as optional input, not a universal prerequisite. A clustered app opts in through `mesh.toml`; ordinary ad hoc compiler tests and minimal Mesh projects still build with only `main.mpl`.

3. **Validate declared clustered targets against compiler exports after discovery/typecheck, not against app-authored strings.**
   The current build pipeline already:
   - discovers modules
   - type-checks in topological order
   - collects `ExportedSymbols`

   That means S01 can validate declarations against:
   - `service_defs` / `ServiceExportInfo.methods` for service call/cast handlers
   - `functions` for declared public function-style work entrypoints
   - `actor_defs` if actors are part of the clustered-handler boundary

   This is the narrowest honest path for **R063**.

4. **Move the `Continuity` Mesh API to typed struct results using the `HttpResponse` precedent.**
   Likely public types:
   - `ContinuityAuthorityStatus`
   - `ContinuityRecord`
   - `ContinuitySubmitDecision`

   Likely function shape:
   - `authority_status() -> ContinuityAuthorityStatus ! String`
   - `status(String) -> ContinuityRecord ! String`
   - `submit(...) -> ContinuitySubmitDecision ! String`
   - `mark_completed(...) -> ContinuityRecord ! String`
   - `acknowledge_replica(...) -> ContinuityRecord ! String`

   `promote()` can be typed too if current M043 compatibility requires it, but S01 should treat that as transitional surface area because **D185** says manual promotion is not the M044 end state.

5. **Do not solve builtin JSON traits in S01 unless a real proof requires them.**
   `cluster-proof/work_continuity.mpl` already has local payload structs like `WorkStatusPayload` and `ContinuityAuthorityPayload` for HTTP encoding. That means the slice can remove runtime JSON parsing without also teaching builtin continuity structs to derive/implement `Json` immediately.

6. **Mirror any new manifest-aware project semantics into LSP/discovery before calling the slice done.**
   `compiler/mesh-lsp/src/analysis.rs` duplicates project/package discovery logic. If clustered declarations become part of project semantics, editor analysis will drift unless the same root/manifest contract is reused there.

## Implementation Landscape

### Key Files

#### Manifest / project metadata seam

- `compiler/mesh-pkg/src/manifest.rs`
  - Shared `mesh.toml` schema and parser.
  - Today only models `package` and `dependencies`.
  - Best first place to add optional clustered-app metadata.

- `compiler/meshc/src/main.rs`
  - `build(...)` currently ignores `mesh.toml` completely.
  - Natural place to load optional manifest metadata and thread it into later validation.

- `compiler/meshc/src/discovery.rs`
  - Project/package discovery.
  - Still structural today; does not expose app metadata.

- `compiler/mesh-lsp/src/analysis.rs`
  - Mirrors project/package discovery logic for editor analysis.
  - Secondary seam if clustered declarations become project-semantic.

- `compiler/meshpkg/src/publish.rs`
- `compiler/meshpkg/src/install.rs`
- `compiler/mesh-pkg/src/resolver.rs`
  - All read the shared manifest.
  - Important planner note: these consumers only care about package/dependency data today; adding optional clustered-app metadata is comparatively low-risk here.

- `mesher/mesh.toml`, `reference-backend/mesh.toml`, `mesh-slug/mesh.toml`
  - Real manifest examples.
  - All currently minimal.

- `cluster-proof/`
  - **No `mesh.toml` exists yet.**

#### Declaration validation seam

- `compiler/mesh-parser/src/ast/item.rs`
  - `ServiceDef::call_handlers()` / `cast_handlers()` expose handler names directly.

- `compiler/mesh-typeck/src/infer.rs::infer_service_def(...)`
  - Registers helper functions and method mappings for services.

- `compiler/mesh-typeck/src/lib.rs`
  - `ModuleExports`, `ExportedSymbols`, and `ServiceExportInfo` already contain the data needed to validate clustered declaration targets.
  - `actor_defs` and public `functions` exist too, but they are not handler-specialized the way services are.

- `compiler/meshc/src/main.rs`
  - Current build pipeline type-checks all modules, collects exports, then lowers MIR.
  - The most natural place for manifest declaration validation is **after exports exist, before MIR/codegen**.

#### Typed continuity public surface seam

- `compiler/mesh-typeck/src/infer.rs`
  - Hard-codes the `Continuity` stdlib module.
  - Currently returns `String ! String` for every continuity API.

- `compiler/mesh-typeck/src/infer.rs` (builtin struct registration)
  - Pre-registers builtin `HttpResponse` for field access.
  - This is the typechecker precedent continuity should follow.

- `compiler/mesh-codegen/src/mir/lower.rs`
  - Already maps continuity builtin names to runtime symbols.
  - Also pre-seeds `MirStructDef` for `HttpResponse`; continuity builtin structs need the same treatment.

- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
  - Declares the continuity externs today as `MeshResult<String, String>` style pointers.
  - If ABI return payloads change to typed structs, this declaration surface must move with the typechecker/runtime.

- `compiler/mesh-codegen/src/codegen/pattern.rs`
  - Already supports `Result<Struct, String>` payload dereferencing and boxing.
  - This is the proof that S01 is plumbing, not a new representation problem.

- `compiler/mesh-rt/src/dist/continuity.rs`
  - Already owns typed continuity structs and serializes them to JSON for Mesh-facing APIs.
  - The S01 change seam is here: return typed payloads instead of JSON strings.

- `compiler/mesh-rt/src/lib.rs`
  - Re-exports the continuity ABI and typed Rust structs.

#### Current proof-app consumer seam

- `cluster-proof/work.mpl`
  - Defines app-local `WorkRequestRecord`, `WorkStatusPayload`, `TargetSelection`, and routing glue.

- `cluster-proof/work_continuity.mpl`
  - Thin continuity adapter layer.
  - This is the highest-value consumer file for S01 because it shows every current JSON parse and every cluster fact still passed as raw strings.

- `cluster-proof/cluster.mpl`
  - App-owned canonical membership and placement logic.
  - Important for later slices, but S01 only needs to understand that placement is still outside the runtime.

- `cluster-proof/main.mpl`
  - Wires the HTTP routes and authority/membership payloads.

- `compiler/meshc/tests/e2e_m043_s02.rs`
  - Current compiler-owned proof of the stringly continuity API.
  - Contains compile-only helpers that build temp projects without a manifest.
  - Will need either adaptation or retirement once the typed continuity API lands.

### Natural Seams

#### 1. Manifest opt-in and clustered declaration schema
Primary files:
- `compiler/mesh-pkg/src/manifest.rs`
- `compiler/meshc/src/main.rs`
- possibly `compiler/meshc/src/discovery.rs`

This task owns:
- optional clustered-app manifest schema
- manifest loading during build when present
- compatibility with non-manifest builds
- build-time declaration validation entrypoint

#### 2. Declaration-to-compiler symbol resolution
Primary files:
- `compiler/mesh-typeck/src/lib.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-parser/src/ast/item.rs`
- `compiler/meshc/src/main.rs`

This task owns:
- what declaration strings/paths mean
- how service call/cast handlers are named in config
- how public function/actor-style “work” declarations resolve, if included in S01
- compile errors for unknown/private/mismatched clustered targets

#### 3. Typed continuity ABI and builtin struct registration
Primary files:
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/lib.rs`

This task owns:
- typed `Continuity.*` function signatures
- builtin continuity struct field access
- matching MIR layouts
- runtime FFI payload shape

#### 4. Thin consumer rewrite and proof migration
Primary files:
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/work.mpl`
- `cluster-proof/main.mpl`
- `compiler/meshc/tests/e2e_m043_s02.rs` or a new `e2e_m044_s01.rs`

This task owns:
- deleting continuity JSON decode wrappers
- rewriting the proof app onto typed values
- keeping the HTTP layer local and optional
- preserving current behavior while the public surface changes underneath it

#### 5. Project-analysis mirror
Primary files:
- `compiler/mesh-lsp/src/analysis.rs`

This is a secondary seam, but if manifest declarations become semantic, leaving LSP behind will create stale editor truth.

### What to Build First

1. **Lock the declaration contract and manifest compatibility story.**
   This shapes everything else. The riskiest product question in S01 is not JSON vs typed structs; it is what a clustered declaration names, and whether `mesh.toml` becomes semantic without breaking ad hoc compiler builds.

2. **Move the `Continuity` API onto typed structs using the existing builtin-struct pattern.**
   The runtime state machine already exists. The surface is the work.

3. **Rewrite `cluster-proof/work_continuity.mpl` onto typed values.**
   This is the slice’s most honest consumer proof for **R062**.

4. **Only then add/refresh verification rails.**
   The tests need the surface shape to settle first.

### Verification Approach

#### Manifest / declaration contract
Add targeted compiler tests for:
- manifest-present clustered opt-in build succeeds
- unknown clustered target fails cleanly
- private/non-exported target fails cleanly
- service call/cast target resolution works
- **non-manifest build still succeeds**

Likely commands:
- `cargo test -p mesh-pkg manifest -- --nocapture`
- `cargo test -p meshc --test tooling_e2e -- --nocapture` if any init/scaffold contract changes are intentionally pulled in
- more likely: a new `cargo test -p meshc --test e2e_m044_s01 -- --nocapture`

#### Typed continuity API
Add compile/build proofs for:
- `Continuity.authority_status()` field access without `from_json`
- `Continuity.status()` field access without `from_json`
- `Continuity.submit()` returning a typed decision with typed nested record
- wrong field/arity/result-shape usage fails at compile time

The best existing harness to copy is the compile-only helper style in `compiler/meshc/tests/e2e_m043_s02.rs`.

#### Consumer proof
- `cargo run -q -p meshc -- build cluster-proof`
- `cargo run -q -p meshc -- test cluster-proof/tests`

And add a fail-closed absence check that `cluster-proof/work_continuity.mpl` no longer decodes runtime continuity JSON via:
- `ContinuityAuthorityStatus.from_json(...)`
- `ContinuitySubmitDecision.from_json(...)`
- `WorkRequestRecord.from_json(...)`

Keep the negative grep narrowly targeted to continuity-runtime decode helpers so it does not punish legitimate request-body JSON parsing.

#### Editor/project-analysis mirror (if semantic in S01)
If clustered declarations are surfaced in project analysis, add a matching LSP regression test rather than leaving the compiler and editor with different project-root semantics.

## Don't Hand-Roll

| Problem | Existing seam to reuse | Why it matters |
|---|---|---|
| Shared `mesh.toml` parsing | `compiler/mesh-pkg/src/manifest.rs` | Keep one schema owner instead of a compiler-only TOML parser. |
| Service handler name resolution | `ServiceExportInfo` + `ServiceDef::call_handlers()` / `cast_handlers()` | The compiler already knows service method names and generated targets. |
| Typed builtin struct results | `Http.send` / `HttpResponse` across `infer.rs`, `mir/lower.rs`, `http/client.rs`, and `codegen/pattern.rs` | This is the direct precedent for typed continuity structs. |
| HTTP JSON exposure | `cluster-proof/work_continuity.mpl` local payload structs (`WorkStatusPayload`, `ContinuityAuthorityPayload`) | S01 can keep HTTP as a thin layer without teaching builtin continuity structs to implement `Json`. |
| Compile-only surface tests | `compiler/meshc/tests/e2e_m043_s02.rs::build_only_mesh(...)` | Reuse the existing temp-project compiler proof pattern. |

## Constraints

- `meshc build` currently emits rich diagnostics only for Mesh source files. Manifest parsing/validation is string-error based today.
- `meshc build` currently accepts projects with only `main.mpl`; this compatibility matters for existing compiler tests.
- `cluster-proof` still duplicates cluster config rules between Mesh code (`config.mpl`) and shell packaging (`docker-entrypoint.sh`), but collapsing that duplication is not S01’s primary job.
- The current `Continuity` module exists only as builtin function signatures plus runtime FFI mapping. Adding typed builtin continuity structs requires **both** typechecker registration and MIR layout registration.
- **D184** still applies: runtime/API truth first, CLI second, HTTP optional.
- **D185** still applies: auto-promotion only. Do not let a nicer `promote()` binding become the product direction.
- **S03** owns scaffold/init and built-in operator surfaces; S01 should not absorb that work unless the planner consciously decides to steal scope.

## Common Pitfalls

- **Making `mesh.toml` mandatory for all builds.** This will break temp-project compiler tests and non-project Mesh builds.
- **Changing only `compiler/mesh-typeck/src/infer.rs`.** Typed builtin structs also need MIR-side registration and runtime payload changes.
- **Treating “typed continuity API” as “builtin continuity structs must derive `Json`.”** Field access is the first requirement; direct JSON encoding can stay local to the proof app for now.
- **Inventing a second declaration namespace in app code.** The compiler already exports service methods, public functions, and actors.
- **Over-scoping into `meshc init --clustered`.** That is real work, but it belongs to S03/R066, not the critical path for S01.
- **Treating manual promotion as a core public contract.** M044 explicitly wants auto-only promotion and fail-closed ambiguity handling.
- **Forgetting the editor mirror.** If the compiler makes manifest declarations semantic but LSP does not, users will get stale IDE truth.

## Open Risks

- **Exact declaration syntax is still unresolved for “work handlers.”** Service handlers have a clean compiler-visible model today; generic work entrypoints do not.
- **Manifest diagnostics may be less precise than Mesh-source diagnostics on the first pass.** The current manifest parser does not feed ariadne-style spans into `meshc build`.
- **Builtin continuity trait surface may expand later.** If later slices want direct `Json.encode(authority_status)` or docs that treat builtin continuity structs like ordinary derived Mesh structs, there is more trait-registration work beyond S01’s minimum.
- **`cluster-proof` still carries placement/config glue that S02/S05 are supposed to delete.** S01 should not mistake that for a typed-surface blocker.

## Skills Discovered

| Technology | Skill | Status |
|---|---|---|
| Rust compiler/runtime seams | `rust-best-practices` | available and used during research |
| Distributed failover/fencing patterns | `distributed-systems` | installed during this research (`yonatangross/orchestkit@distributed-systems`) |
| TOML-specific authoring | none relevant | searched; no directly relevant professional skill installed |
