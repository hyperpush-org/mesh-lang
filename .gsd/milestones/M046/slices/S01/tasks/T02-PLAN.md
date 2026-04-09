---
estimated_steps: 3
estimated_files: 4
skills_used:
  - rust-best-practices
---

# T02: Merge source declarations into compiler and LSP planning

**Slice:** S01 — Dual clustered-work declaration
**Milestone:** M046

## Description

Feed decorated work functions into the existing declared-handler path without adding a second runtime boundary. The compiler and LSP should collect source-declared work, convert it into the same logical declaration records used by `mesh.toml`, reject same-target source+manifest duplicates explicitly, and keep the M044 manifest rails green.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/mesh-pkg/src/manifest.rs` clustered validation helpers | Surface a deterministic duplicate or invalid-target diagnostic and abort planning before codegen. | N/A | Reject the declaration instead of emitting partial `ClusteredExecutionMetadata`. |
| `compiler/meshc/src/main.rs` build merge path | Emit build diagnostics and stop before declared-handler preparation. | N/A | Avoid registering mismatched runtime names or executable symbols. |
| `compiler/mesh-lsp/src/analysis.rs` project analysis path | Report the same duplicate or invalid-target reason as `meshc`. | N/A | Avoid false-clean editor output for source-only decorated projects. |

## Load Profile

- **Shared resources**: full-project parse/typecheck/export scans reused by both `meshc` and `mesh-lsp`.
- **Per-operation cost**: one additional scan of function items to collect source declarations before existing clustered validation.
- **10x breakpoint**: compiler/LSP drift in declaration collection or duplicate handling will show up first as inconsistent diagnostics, not raw CPU cost.

## Negative Tests

- **Malformed inputs**: decorated private work functions, ambiguous overloaded public work names, same-target manifest/source declarations, and wrong-boundary targets.
- **Error paths**: source-only and mixed declaration projects fail before declared-handler generation when a decorated target cannot resolve to a valid public work function or when the same target is declared twice.
- **Boundary conditions**: manifest-only projects stay green, source-only projects emit the same registration surface, and source-declared work can coexist with manifest-declared service handlers without forking runtime behavior.

## Steps

1. Collect decorated work functions from parsed `FnDef`s and convert them into the same logical declaration shape used by manifest declarations, with an explicit same-target duplicate policy between source and manifest.
2. Wire the merged declarations into `prepare_project_build(...)` and LSP analysis so source-only decorated projects get the same `ClusteredExecutionMetadata` validation and declared-handler planning path as manifest projects.
3. Add compiler and LSP proof rails that cover source-only success, duplicate failure, private/ambiguous rejection, emitted declared-handler registration, and M044 manifest regressions.

## Must-Haves

- [ ] Source-only clustered work reaches the existing declared-handler pipeline with no runtime-path fork.
- [ ] Manifest-only clustered work remains green.
- [ ] Same-target manifest/source duplicates fail closed with an explicit diagnostic.
- [ ] LSP diagnostics match compiler behavior for source-only and invalid decorated work.
- [ ] Proof rails live in real tests under `compiler/meshc/tests/e2e_m046_s01.rs` and `compiler/mesh-lsp/src/analysis.rs`.

## Verification

- `cargo test -p meshc --test e2e_m046_s01 m046_s01_ -- --nocapture`
- `cargo test -p mesh-lsp m046_s01_ -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture`

## Observability Impact

- Signals added/changed: compiler and LSP diagnostics now name duplicate or invalid source declarations explicitly, and the compiler proof rail still inspects emitted `mesh_register_declared_handler` registration text.
- How a future agent inspects this: rerun the new `e2e_m046_s01` target, the `mesh-lsp` `m046_s01_` tests, and the retained M044 regression filters.
- Failure state exposed: source/manifest drift becomes an explicit diagnostic or missing-registration assertion instead of a silent runtime mismatch.

## Inputs

- `compiler/mesh-parser/src/ast/item.rs` — `FnDef` clustered-work accessor added in T01.
- `compiler/mesh-pkg/src/manifest.rs` — existing clustered declaration types and validation helpers.
- `compiler/meshc/src/main.rs` — `prepare_project_build(...)` and clustered execution planning for declared handlers.
- `compiler/mesh-lsp/src/analysis.rs` — manifest-only clustered diagnostics and export-surface collection.
- `compiler/meshc/tests/e2e_m044_s01.rs` — manifest-only declared-work regression rail.
- `compiler/meshc/tests/e2e_m044_s02.rs` — declared-handler metadata and LLVM registration regression rail.

## Expected Output

- `compiler/mesh-pkg/src/manifest.rs` — merged source/manifest declaration validation with explicit duplicate handling.
- `compiler/meshc/src/main.rs` — source-declared work feeds the same clustered execution plan used by manifest declarations.
- `compiler/mesh-lsp/src/analysis.rs` — source-declared projects analyze with the same clustered diagnostics contract as `meshc`.
- `compiler/meshc/tests/e2e_m046_s01.rs` — compiler proof rail for source-only success, duplicate failure, and declared-handler registration equivalence.
