# M046/S01 Research — Dual clustered-work declaration

## Requirements targeted

- **Primary:** R085 — add both manifest and source decorator clustered-work declaration.
- **Supports:** R090 — manifest and source forms need to converge on the same runtime-owned declared boundary.
- **Not this slice:** R086/R087/R088/R089/R091/R092/R093. The current scaffold, docs, and `cluster-proof/` still intentionally own HTTP submit routes and call `Continuity.submit_declared_work(...)`; that cleanup belongs to later slices.

## Skills Discovered

- Loaded installed skill: **`rust-best-practices`**.
  - Relevant guidance here: keep new validation/error paths fallible via `Result` instead of panicking; keep tests narrow and behavior-specific; avoid widening abstraction surface unless there is a stable seam.
- Ran `npx skills find "Rust compiler parser codegen"`.
  - Results were generic Rust skills only; nothing was more relevant than the already-installed Rust skill.
  - **No additional skills installed.**

## Summary

- The downstream declared-handler path is already real and reusable. Once a target becomes a `ClusteredExecutionMetadata`, the compiler/runtime path is already generic:
  - `compiler/meshc/src/main.rs::prepare_project_build(...)` builds `clustered_execution_plan`
  - `compiler/meshc/src/main.rs::prepare_declared_handler_plan(...)`
  - `compiler/mesh-codegen/src/declared.rs::prepare_declared_runtime_handlers(...)`
  - startup registration via `mesh_register_declared_handler`
  - runtime execution via `compiler/mesh-rt/src/dist/node.rs::submit_declared_work(...)`
- The missing piece for S01 is **declaration intake**, not runtime behavior.
- Current declaration intake is **manifest-only**:
  - `compiler/mesh-pkg/src/manifest.rs` owns `[cluster].declarations`
  - `compiler/meshc/src/main.rs` only builds a plan when `manifest.cluster` exists
  - `compiler/mesh-lsp/src/analysis.rs` only diagnoses clustered declarations when `manifest.cluster` exists
- There is **no existing source-level decorator/annotation syntax** for this. Important constraints:
  - `@` is currently an outright lexer error (`compiler/mesh-common/src/error.rs` has a test for `UnexpectedCharacter('@')`)
  - `compiler/mesh-common/src/token.rs` / `compiler/mesh-parser/src/syntax_kind.rs` have no attribute/decorator token family
  - top-level item dispatch in `compiler/mesh-parser/src/parser/mod.rs::parse_item_or_stmt(...)` only knows `pub fn`, `fn`, `module`, `struct`, etc., plus contextual `from`
- Current public clustered examples are still contractually explicit-submit and route-owned:
  - `compiler/mesh-pkg/src/scaffold.rs` generates `Continuity.submit_declared_work(...)` and HTTP routes
  - `compiler/meshc/tests/tooling_e2e.rs::test_init_clustered_creates_project` asserts those strings exist
  - `website/docs/docs/getting-started/clustered-example/index.md` teaches the same route-owned submit flow
  - `cluster-proof/main.mpl` + `cluster-proof/work_continuity.mpl` still own `/work` and status routes
- S01 should therefore stay narrow: **accept source declaration, merge it into the existing declared-handler plan, and prove equivalence with manifest declarations.** Do not spend this slice rewriting scaffold/docs/example runtime surfaces.

## Recommendation

### Recommended source syntax

Use a **narrow contextual decorator**, not a general annotation system and not `@...` syntax.

Recommended shape:

```mesh
clustered(work)
pub fn execute_declared_work(request_key :: String, attempt_id :: String) -> Int do
  ...
end
```

Why this is the cheapest honest fit:

- It matches the milestone’s “source decorator” intent without forcing a full annotation feature.
- It can be parsed as a **contextual identifier** (`clustered`) plus `(` `work` `)` in item position, similar in spirit to existing contextual forms like `from` and trailing `deriving(...)`.
- It avoids turning `clustered` into a globally reserved keyword.
- It avoids any lexer/token work for `@`, which would widen surface area immediately.
- It stays purpose-built for **clustered work only**, which is what R085 asks for. Do not expand S01 into service-call/cast decorators.

### Recommended internal shape

Keep a decorated function as an ordinary `FN_DEF` with one extra child node / accessor, **not** as a new wrapper item kind.

That keeps the rest of the compiler simple:

- parser still yields a normal function item
- export-surface collection can inspect `FnDef` for `clustered(work)` metadata
- existing function/name/visibility logic stays local
- later codegen/runtime still consume the same `ClusteredExecutionMetadata`

### Recommended merge model

Treat source-decorated work as a second producer of the same logical declaration records.

Practical flow:

1. parse AST with optional source decorator metadata on `FnDef`
2. collect source-declared work targets as `<ModulePath>.<function>`
3. convert them into the same logical declaration shape as manifest work declarations
4. merge manifest + source declarations **before** `prepare_declared_handler_plan(...)`
5. keep the rest of codegen/runtime unchanged

This keeps the slice honest: one runtime boundary, two front doors.

## Implementation Landscape

### 1. Existing manifest declaration path

**Files:**
- `compiler/mesh-pkg/src/manifest.rs`
- `compiler/meshc/src/main.rs`
- `compiler/mesh-lsp/src/analysis.rs`

What exists now:

- `Manifest` optionally parses `[cluster]` into `ClusterConfig`.
- `validate_cluster_declarations(...)` checks manifest declarations against `ClusteredExportSurface` and returns `Vec<ClusteredExecutionMetadata>`.
- `ClusteredDeclarationKind` already exists and includes `Work`, `ServiceCall`, and `ServiceCast`, but S01 only needs the `Work` path.
- `meshc` and `mesh-lsp` each build their own `ClusteredExportSurface` by scanning parsed public functions and service export metadata.

Important limits of the current path:

- it is stringly: manifest targets are validated after parse/typecheck
- it is manifest-gated: no manifest cluster section means no execution plan and no LSP clustered diagnostics
- identical logic is duplicated in `meshc` and `mesh-lsp`

### 2. Existing downstream declared-handler path (already enough)

**Files:**
- `compiler/meshc/src/main.rs`
- `compiler/mesh-codegen/src/declared.rs`
- `compiler/mesh-codegen/src/codegen/mod.rs`
- `compiler/mesh-codegen/src/codegen/expr.rs`
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/dist/continuity.rs`

What exists now:

- `prepare_declared_runtime_handlers(...)` generates actor-style work wrappers and service wrappers.
- codegen registers those wrappers via `mesh_register_declared_handler(...)`.
- runtime `submit_declared_work(...)` looks up the registered runtime name, computes placement, creates the continuity record, and dispatches local/remote execution.
- declared work auto-completes through the wrapper path (`mesh_continuity_complete_declared_work`).

Implication for S01:

- **No new runtime or codegen semantics are required** if the source decorator ends up producing the same `runtime_registration_name` + `executable_symbol` pair.
- Keep any codegen/runtime edits minimal and only if a test proves the merged plan is not rooting or registering the decorated function correctly.

### 3. Parser constraints

**Files:**
- `compiler/mesh-parser/src/parser/mod.rs`
- `compiler/mesh-parser/src/parser/items.rs`
- `compiler/mesh-parser/src/ast/item.rs`
- `compiler/mesh-parser/src/syntax_kind.rs`
- `compiler/mesh-common/src/token.rs`
- `compiler/mesh-common/src/error.rs`

What matters:

- `parse_item_or_stmt(...)` already dispatches contextual item syntax from an `IDENT` (`from`). That makes a contextual `clustered(...)` prefix plausible.
- `parse_fn_def(...)` currently accepts only optional visibility before `fn|def`.
- There is no existing AST accessor for item-level decorators.
- `@` is not available without lexer work.

Recommendation for scope discipline:

- Prefer **contextual `clustered(work)` prefix** over introducing new token kinds or a generic annotation framework.
- Avoid making `clustered` a reserved keyword unless the contextual form proves impossible.
- Avoid trailing syntax like `end clustered(work)`; it reads more like deriving/traits than a decorator and will complicate body parsing.

### 4. LSP and diagnostics seam

**Files:**
- `compiler/mesh-lsp/src/analysis.rs`

What exists now:

- LSP only surfaces clustered diagnostics from the manifest path.
- It duplicates the same `build_clustered_export_surface(...)` implementation as `meshc`.

Implication:

- Source-only clustered work will be invisible to the editor unless S01 adds a second diagnostic/plan path here.
- The planner should assume **both `meshc` and `mesh-lsp` must change in the same slice**.

### 5. Current example/docs surfaces to leave alone in S01

**Files:**
- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `cluster-proof/main.mpl`
- `cluster-proof/work_continuity.mpl`

Why leave them alone now:

- They currently assert and teach explicit `Continuity.submit_declared_work(...)` and HTTP submit routes.
- Route-free startup-triggered work is the next slice’s contract, not this one.
- Rewriting these now will spill into S02/S04/S05 and blur the acceptance boundary.

## Natural Task Seams

### Seam A — Parser/AST: source decorator syntax

Likely file set:

- `compiler/mesh-parser/src/syntax_kind.rs`
- `compiler/mesh-parser/src/parser/mod.rs`
- `compiler/mesh-parser/src/parser/items.rs`
- `compiler/mesh-parser/src/ast/item.rs`
- `compiler/mesh-parser/tests/parser_tests.rs`
- new parser snapshots under `compiler/mesh-parser/tests/snapshots/`

Deliverable:

- parser accepts the chosen source decorator form on work functions
- AST exposes a narrow accessor on `FnDef`
- bad forms fail closed (wrong location, missing `(work)`, malformed target syntax)

### Seam B — Build/LSP planning: merge manifest + source declarations

Likely file set:

- `compiler/mesh-pkg/src/manifest.rs` (types / merge helper / validation reuse)
- `compiler/meshc/src/main.rs`
- `compiler/mesh-lsp/src/analysis.rs`

Deliverable:

- source-decorated functions produce the same logical declaration records as manifest declarations
- merged plan feeds the existing declared-handler pipeline
- LSP recognizes source-only clustered work and reports any clustered-specific diagnostics

Note: keep this merge logic small and explicit. Per the Rust skill guidance, use ordinary `Result`-returning helpers and avoid clever abstraction here.

### Seam C — Proof rails: parser + compiler + editor equivalence

Likely file set:

- `compiler/meshc/tests/e2e_m046_s01.rs` (new)
- `compiler/mesh-lsp/src/analysis.rs` tests
- existing `compiler/meshc/tests/e2e_m044_s01.rs` and `compiler/meshc/tests/e2e_m044_s02.rs` stay as regressions

Deliverable:

- manifest-only regression still green
- source-only declaration compiles to the same declared runtime registration path
- if both forms are allowed together, the merged result is deduped before codegen; if not, the error is explicit and tested

## Risks / Unknowns

### 1. Duplicate declaration semantics need an explicit choice

If the same target is declared in both `mesh.toml` and source, current code has no duplicate-handling step before wrapper generation/registration.

S01 needs a deliberate rule:

- **either** fail closed on manifest+source duplicates
- **or** identical-target dedupe before `prepare_declared_handler_plan(...)`

Do not let duplicate entries flow through implicitly.

### 2. Multi-clause / overloaded public functions are already treated as ambiguous

`build_clustered_export_surface(...)` currently marks repeated public function names in one module as ambiguous work targets.

That means source-decorating a multi-clause public function is not free. The cheapest S01 rule is:

- keep the same ambiguity rule
- reject/fail-close on decorated overloaded work functions
- leave smarter grouped-clause handling for later only if real dogfood needs it

### 3. The `meshc` / `mesh-lsp` duplicated surface builder is a drift risk

Both copies will have to learn source decorations.

Recommendation: **do not** widen `mesh-pkg` into a parser/typechecker-aware crate just to dedupe this in S01. That is a bigger architectural move than the slice needs. Update both copies deliberately and cover both with tests.

### 4. S01 can accidentally sprawl into S02 if the planner is not strict

Current scaffold/docs/tests explicitly assert:

- HTTP submit route exists
- `Continuity.submit_declared_work(...)` exists in app code
- `cluster-proof/` still owns `/work` and status surfaces

Those are not bugs for S01. They are later-slice targets.

## Verification

### Existing regressions to keep green

- `cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture`
- `cargo test -p mesh-lsp m044_s01_ -- --nocapture`

These prove the current manifest-only path still works.

### New S01 proof surfaces to add

1. **Parser proof**
   - targeted parser tests/snapshots for the chosen decorator syntax
   - malformed decorator forms fail closed

2. **Compiler equivalence proof**
   - new `compiler/meshc/tests/e2e_m046_s01.rs`
   - source-only clustered work build succeeds
   - emitted LLVM / registration surface still contains `mesh_register_declared_handler`
   - source-only and manifest-only versions converge on the same runtime registration name / wrapper symbol pattern

3. **LSP proof**
   - source-decorated clustered work analyzes without false diagnostics
   - any clustered-specific invalid source form gets a deterministic diagnostic

### What not to require yet

- no scaffold rewrite
- no route removal
- no startup-triggered work
- no `tiny-cluster/`
- no `cluster-proof/` rebuild

Those belong to S02–S05.

## Planner Notes

- The slice is **targeted**, not broad architectural exploration.
- The riskiest choice is syntax. Once that is fixed, the rest is mostly plumbing into the already-working declared-handler path.
- The simplest safe plan is:
  1. add narrow source decorator syntax on `FnDef`
  2. collect/merge source declarations into the existing clustered execution plan in `meshc` and `mesh-lsp`
  3. prove manifest/source equivalence with new tests while keeping M044 manifest rails green
- Do not schedule runtime startup-trigger work or example/doc rewrites in this slice; those are separate milestones in the roadmap.