# M047/S01 Research — Source decorator reset for clustered functions

## Summary

S01 directly targets **R097** and supports **R098**, **R099**, and **R106**.

The current clustered declaration path is already shared in one useful place: **parser/AST -> `mesh-pkg` source declaration collection -> shared validation -> meshc build plan / mesh-lsp diagnostics**. That is the right seam for the slice.

What is missing is not runtime registration. It is the source model:

- the lexer/parser only understand `clustered(work)`
- source declarations lose file/span provenance before validation
- `ClusteredExecutionMetadata` has **no replication-count field**
- `meshc` and `mesh-lsp` duplicate the clustered export-surface builder
- LSP clustered diagnostics are currently anchored at **(0,0)** instead of the source declaration

The safest S01 cut is therefore:

1. add `@cluster` / `@cluster(N)` source syntax
2. lower it onto the existing clustered work metadata path
3. preserve source provenance for diagnostics
4. extract the duplicated clustered export-surface builder into shared code
5. keep runtime registration names and executable-symbol lowering stable

Do **not** push `@cluster(3)` into runtime behavior in S01. The current runtime continuity submit contract still rejects `required_replica_count > 1`, so S01 can only make counts truthful in parser/compiler/LSP metadata. S02 has to own the runtime semantics.

## Requirements Targeted

- **R097** — replace `clustered(work)` with `@cluster` / `@cluster(N)`
- **R098** — make counts mean replication counts with default `2`
- **R099** — keep clustering as a general function capability, not an HTTP-only feature
- **R106** — teach one coherent source-first clustered model

For this slice, the honest deliverable is **source-first function declaration + compiler/LSP metadata/diagnostic truth**. Route wrappers, scaffold migration, and docs/example cutover belong to later slices.

## Skills Discovered

- Reviewed installed skills. None were directly relevant enough to load for this repo-internal compiler/LSP slice.
- Ran `npx skills find "language server protocol"`.
- Results were generic setup/integration skills only; none justified installation for this work.

## Recommendation

Following the repo rule to prefer boring standard abstractions over clever frameworks, do **not** invent a generic decorator system for S01. There is no other decorator machinery in Mesh today, and the lexer does not even have an `@` token yet.

Use a **dedicated `@cluster` path** first, keep the internal clustered execution plan shape as stable as possible, and sequence the work like this:

1. **Lexer/parser/AST compatibility bridge**
   - Recognize `@cluster` and `@cluster(N)` on ordinary `fn` / `def` items.
   - Preserve the existing `clustered(work)` path temporarily so later slices can cut over examples/scaffolds/docs without a 78-reference churn in S01.
   - Expose one AST accessor that gives the source declaration plus optional count.

2. **Shared source declaration model in `mesh-pkg`**
   - Replace the current lossy collector with a richer source declaration record that retains module target, optional count, file/path/span provenance, and source origin.
   - Add replication count to the validated metadata now so S02 does not have to reopen the same seam.

3. **Shared clustered export-surface helper**
   - Move the duplicated `build_clustered_export_surface(...)` out of both `meshc` and `mesh-lsp`.
   - Keep validation centralized in `mesh-pkg`.

4. **meshc and mesh-lsp consumers**
   - Make both consume the same shared source-declaration + export-surface helpers.
   - Upgrade clustered diagnostics to point at the actual source declaration when origin is source.
   - Leave manifest-origin diagnostics as project/file-level diagnostics for now.

5. **Verification**
   - Add new M047-named parser / manifest / LSP / meshc tests instead of overloading all M046 rails.
   - Keep old M046 compatibility tests green until the hard cutover slice removes the old public syntax.

## Implementation Landscape

### 1. Lexer + parser + AST are currently hardcoded to `clustered(work)`

**Files:**
- `compiler/mesh-common/src/token.rs`
- `compiler/mesh-lexer/src/lib.rs`
- `compiler/mesh-parser/src/parser/mod.rs`
- `compiler/mesh-parser/src/parser/items.rs`
- `compiler/mesh-parser/src/ast/item.rs`
- `compiler/mesh-parser/src/syntax_kind.rs`
- `compiler/mesh-parser/tests/parser_tests.rs`

**What exists now:**
- `mesh-common` has no token for `@`.
- `mesh-lexer` has no `@` branch, so `@cluster` currently lexes as an error.
- `parser/mod.rs` treats the identifier text `clustered` as a contextual item prefix.
- `parser/items.rs::parse_optional_clustered_work_decl(...)` only accepts `clustered(work)` and emits `CLUSTERED_WORK_DECL`.
- `FnDef::clustered_work_decl()` and `ClusteredWorkDecl::target()` only expose the inner target token (`"work"`).

**Implication for planning:**
- S01 is not a text rename. It must start in **tokenization + parser + AST**.
- The minimal honest AST is a dedicated clustered declaration node or accessor with:
  - present/absent
  - optional explicit count
  - source span for diagnostics
- Avoid a generic annotation/decorator framework unless another slice proves a second user.

### 2. `mesh-pkg` is already the shared declaration contract

**Files:**
- `compiler/mesh-pkg/src/manifest.rs`

**What exists now:**
- `ClusteredDeclaration { kind, target }`
- `ClusteredExecutionMetadata { kind, manifest_target, runtime_registration_name, executable_symbol }`
- `collect_source_cluster_declarations(...)` walks parsed modules and turns source markers into manifest-shaped declarations
- `validate_cluster_declarations_with_source(...)` merges manifest + source declarations, validates them against a `ClusteredExportSurface`, and returns validated execution metadata

**Current limitation:**
`collect_source_cluster_declarations(...)` drops source provenance immediately. It returns plain `ClusteredDeclaration` values with only kind + qualified target.

That causes both downstream problems:
- `meshc` emits source-origin diagnostics without a real file/span
- `mesh-lsp` can only attach clustered diagnostics to `(0,0)`

**Implication for planning:**
The natural S01 seam is to introduce a richer **source declaration record** here instead of trying to patch diagnostics only in LSP.

Likely shape:
- qualified target
- declaration kind (still `Work` for S01)
- optional explicit replica count
- origin metadata (file/path + text range/span)

### 3. Count semantics are not represented in compiler metadata yet

**Files:**
- `compiler/mesh-pkg/src/manifest.rs`
- `compiler/meshc/src/main.rs`
- `compiler/mesh-codegen/src/codegen/mod.rs`

**What exists now:**
- `ClusteredExecutionMetadata` carries no count/default-count information.
- `prepare_declared_handler_plan(...)` only forwards kind + runtime name + executable symbol.

**Implication for planning:**
Add the count/default-count field to validated metadata in S01 even if runtime behavior still ignores it. Otherwise S02 has to reopen the same validation and plan plumbing.

Recommended S01 contract:
- `@cluster` -> validated metadata carries default replication count `2`
- `@cluster(N)` -> validated metadata carries explicit replication count `N`
- compiler and LSP diagnostics can report invalid shapes/counts from this metadata
- runtime submission still stays unchanged until S02

### 4. `meshc` and `mesh-lsp` duplicate the export-surface builder

**Files:**
- `compiler/meshc/src/main.rs`
- `compiler/mesh-lsp/src/analysis.rs`
- `compiler/mesh-typeck/src/lib.rs`
- `compiler/mesh-typeck/src/infer.rs`

**What exists now:**
Both `meshc` and `mesh-lsp` locally implement near-identical `build_clustered_export_surface(...)` functions.

That duplicated logic walks:
- public/private work functions
- ambiguous work functions
- service call/cast/start helpers via `ServiceExportInfo.method_exports`

**Implication for planning:**
This duplication should be extracted as its own task.

It is the cleanest shared seam in the slice:
- parser changes feed source declarations
- one shared export-surface builder feeds both compiler and LSP
- one shared validator already exists in `mesh-pkg`

If S01 does not extract this, compiler/editor drift will be easy to reintroduce when counts or source-origin rules change.

### 5. Current clustered diagnostics are structurally weak

**Files:**
- `compiler/meshc/src/main.rs`
- `compiler/mesh-lsp/src/analysis.rs`

**What exists now:**
- `meshc::emit_clustered_declaration_diagnostics(...)` only knows manifest-path diagnostics; source-origin JSON output has empty file/spans.
- `mesh-lsp::clustered_declaration_diagnostic(...)` wraps every clustered issue in `project_diagnostic(...)`, which anchors it at line 0, column 0.

**Implication for planning:**
If S01 claims LSP/diagnostic truth, it should upgrade source-origin clustered diagnostics to use real source ranges.

That work depends on the richer source declaration model; doing it first in LSP alone would be fake.

### 6. Runtime registration is already the right downstream boundary

**Files:**
- `compiler/meshc/src/main.rs`
- `compiler/mesh-codegen/src/codegen/mod.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-rt/src/dist/node.rs`

**What exists now:**
- `PreparedBuild.clustered_execution_plan` is converted into `DeclaredHandlerPlanEntry`
- codegen registers `runtime_registration_name + executable_symbol + fn_ptr`
- runtime keeps a declared-handler registry keyed by runtime name
- `submit_declared_work(...)` looks up the registered runtime name and dispatches work

**Implication for planning:**
S01 should preserve this runtime boundary if possible.

That means:
- keep `runtime_registration_name` derived from normal module/function identity
- keep `executable_symbol` plumbing stable
- do not make S01 depend on a runtime refactor

This matches the milestone context: the source syntax is wrong, but the runtime-owned clustered execution seam already exists.

### 7. `@cluster` prior art already exists, but only as a dormant sketch

**Files:**
- `tiny-cluster-prefered/add.mpl`
- `tiny-cluster-prefered/lib/subtract.mpl`
- `tiny-cluster-prefered/mesh.toml`

**What exists now:**
- `add.mpl` uses `@cluster(3)`
- `lib/subtract.mpl` uses `@cluster(2)`
- `tiny-cluster-prefered/mesh.toml` uses an old prototype `[cluster].declarations = [{ clusters: 2, target = ... }]`

**Important constraint:**
That manifest prototype does **not** match the current manifest schema (`kind`, `target` only). So `tiny-cluster-prefered/` is prior art, not a live truth fixture.

**Implication for planning:**
Use it as spelling guidance, not as a verification target.

### 8. Broad cutover is later-slice work, not S01 work

**Evidence:**
`rg -n "clustered\\(work\\)" . -g '!target'` finds **78** references. Top buckets:
- `compiler/meshc`: 26
- `compiler/mesh-parser`: 21
- `compiler/mesh-pkg`: 12
- `website/docs`: 4
- `compiler/mesh-lsp`: 4
- examples/tests/readmes/scripts: remainder

**Implication for planning:**
S01 should **not** try to migrate scaffold/examples/docs/readmes and every historical verifier. The roadmap already puts hard cutover/dogfood migration in **S04**.

The honest S01 blast radius is:
- parser/AST
- shared declaration collection/validation
- compiler/LSP consumers
- new M047 tests

## Risks / Unknowns

### Runtime still rejects replication counts above 1

**Files:**
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/dist/node.rs`

`SubmitRequest::validate()` currently rejects `required_replica_count > 1`, and `submit_declared_work(...)` forwards that field into continuity submission.

**Planning consequence:**
- `@cluster(3)` cannot drive real runtime behavior yet.
- S01 should only preserve/validate/report counts.
- S02 must own the runtime semantics and any submit-path changes.

### Source/manifest dual-surface tension

The milestone wants source-first declarations and eventually a hard cutover, but the repo still has many M044–M046 rails shaped around the old syntax.

**Planning consequence:**
Use a compatibility bridge in S01:
- make `@cluster` the new proof surface
- keep old `clustered(work)` compatibility temporarily
- defer repo-wide public cutover to S04

### Diagnostic provenance is a structural change

Because source declarations are currently lowered into plain `ClusteredDeclaration` values, better diagnostics are not a one-file LSP tweak.

**Planning consequence:**
Budget a dedicated task for source-origin metadata in `mesh-pkg` before touching mesh-lsp diagnostics.

## Suggested Task Cuts

### T1 — Parser/AST compatibility bridge
**Files:**
- `compiler/mesh-common/src/token.rs`
- `compiler/mesh-lexer/src/lib.rs`
- `compiler/mesh-parser/src/parser/mod.rs`
- `compiler/mesh-parser/src/parser/items.rs`
- `compiler/mesh-parser/src/ast/item.rs`
- `compiler/mesh-parser/src/syntax_kind.rs`
- `compiler/mesh-parser/tests/parser_tests.rs`

**Goal:** Parse `@cluster` / `@cluster(N)` on ordinary functions, keep temporary compatibility with `clustered(work)`.

### T2 — Shared source declaration model + count/provenance
**Files:**
- `compiler/mesh-pkg/src/manifest.rs`
- related tests in the same file

**Goal:** Replace the lossy collector with a richer source declaration model; add optional replication count and source origin/spans; keep validated metadata stable except for the new count field.

### T3 — Shared clustered export-surface helper
**Files:**
- extract from `compiler/meshc/src/main.rs`
- extract from `compiler/mesh-lsp/src/analysis.rs`
- likely new helper in `compiler/mesh-pkg/src/manifest.rs` or nearby shared module

**Goal:** Remove duplicated surface-building logic before meshc/LSP diverge on the new declaration model.

### T4 — meshc build validation + diagnostics
**Files:**
- `compiler/meshc/src/main.rs`
- `compiler/meshc/tests/e2e_m046_s01.rs` or new `compiler/meshc/tests/e2e_m047_s01.rs`

**Goal:** Consume the richer metadata, validate new syntax without manifest dependency, keep codegen handoff stable, improve source-origin diagnostics.

### T5 — mesh-lsp diagnostics
**Files:**
- `compiler/mesh-lsp/src/analysis.rs`

**Goal:** Consume the same shared helpers and attach clustered diagnostics to real source locations.

## Verification

Recommended post-implementation rails for S01:

```bash
cargo test -p mesh-parser m047_s01 -- --nocapture
cargo test -p mesh-pkg m047_s01 -- --nocapture
cargo test -p mesh-lsp m047_s01 -- --nocapture
cargo test -p meshc --test e2e_m047_s01 -- --nocapture
```

If the slice extends existing M046 rails instead of creating M047-named ones, still keep the verification split across:

- parser regression tests
- `mesh-pkg` source declaration / validation tests
- `mesh-lsp` clustered diagnostic tests
- one meshc e2e that proves:
  - source-only package (no `[cluster]` dependency)
  - `@cluster` compiles
  - `@cluster(3)` preserves count metadata
  - duplicate/private-source diagnostics remain explicit

Keep the verification commands fail-closed on **non-zero test count** if a wrapper script is added.

## File Inventory

- `compiler/mesh-common/src/token.rs` — token vocabulary; no `@` token today
- `compiler/mesh-lexer/src/lib.rs` — lexer; no `@cluster` path today
- `compiler/mesh-parser/src/parser/mod.rs` — item dispatch; contextual `clustered(...)` hook
- `compiler/mesh-parser/src/parser/items.rs` — hardcoded `clustered(work)` parser
- `compiler/mesh-parser/src/ast/item.rs` — current clustered AST accessor
- `compiler/mesh-parser/src/syntax_kind.rs` — `CLUSTERED_WORK_DECL` CST kind
- `compiler/mesh-parser/tests/parser_tests.rs` — current parser coverage for `clustered(work)`
- `compiler/mesh-pkg/src/manifest.rs` — shared declaration types, source collector, validator, clustered export surface
- `compiler/mesh-typeck/src/lib.rs` — service export metadata types used by clustered export-surface building
- `compiler/mesh-typeck/src/infer.rs` — populates `ServiceExportInfo.method_exports`
- `compiler/meshc/src/main.rs` — build path, clustered validation orchestration, duplicated export-surface builder, diagnostic emission
- `compiler/mesh-lsp/src/analysis.rs` — project analysis, clustered diagnostics, duplicated export-surface builder
- `compiler/mesh-codegen/src/codegen/mod.rs` — downstream registration sink for validated clustered execution metadata
- `compiler/mesh-rt/src/dist/node.rs` — declared-handler registry and runtime dispatch by runtime name
- `compiler/mesh-rt/src/dist/continuity.rs` — current `required_replica_count` validation constraint
- `compiler/meshc/tests/e2e_m046_s01.rs` — current source-declared compiler e2e coverage
- `tiny-cluster-prefered/add.mpl` — dormant prior art for `@cluster(3)`
- `tiny-cluster-prefered/lib/subtract.mpl` — dormant prior art for `@cluster(2)`
- `tiny-cluster-prefered/mesh.toml` — prototype-only manifest shape; not a live fixture
