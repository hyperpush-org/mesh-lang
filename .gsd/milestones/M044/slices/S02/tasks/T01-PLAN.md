---
estimated_steps: 35
estimated_files: 5
skills_used: []
---

# T01: Threaded declared clustered execution metadata through shared manifest validation and into meshc build preparation.

---
estimated_steps: 4
estimated_files: 5
skills_used:
  - rust-best-practices
  - test
---

# T01: Carry declared-handler execution metadata past manifest validation

**Slice:** S02 — Runtime-Owned Declared Handler Execution
**Milestone:** M044

## Description

S01 proved the declaration boundary, but `meshc` still throws the manifest away before MIR/codegen. This task creates the compiler-owned execution metadata that later runtime/codegen work can consume without reopening manifest parsing or widening the declared boundary past what `mesh.toml` explicitly names.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Shared manifest/export validation in `mesh-pkg` + `meshc` | Fail compilation with the declaration kind, target, and execution-planning reason; do not silently drop clustered metadata. | N/A — compile-time only. | Reject ambiguous/private/non-executable targets instead of coercing them into local-only behavior. |
| Project-aware `mesh-lsp` parity | Keep editor diagnostics aligned with compiler truth; treat drift as a regression. | N/A — analysis is synchronous. | Reject declarations that validate in one surface but not the other. |
| MIR/codegen handoff | Stop before lowering if a declared handler has no runtime-executable symbol or wrapper plan. | N/A — compile-time only. | Treat missing symbol metadata as a contract failure, not as “undeclared by accident.” |

## Load Profile

- **Shared resources**: manifest parsing, export discovery, project-aware analysis, and compiler metadata threading.
- **Per-operation cost**: one clustered execution-plan derivation per build or analysis.
- **10x breakpoint**: duplicated target-resolution logic drifts first; large-project builds/LSP reanalysis get slower before runtime cost matters.

## Negative Tests

- **Malformed inputs**: malformed service target paths, blank or route-shaped work targets, and ambiguous overloaded public work names.
- **Error paths**: a declared target validates as exported but cannot be mapped to an executable symbol/wrapper; undeclared targets stay absent from the execution plan.
- **Boundary conditions**: manifest absent, cluster section absent, and manifests mixing work plus service declarations.

## Steps

1. Extend the clustered manifest/compiler seam so each validated declaration becomes richer execution metadata: declaration kind, manifest target, executable symbol/wrapper identifier, and enough info for runtime registration.
2. Thread that metadata through `meshc` to the lowering/codegen boundary and mirror any semantic narrowing in `mesh-lsp` so S01’s explicit boundary remains the only clustered boundary.
3. Create `compiler/meshc/tests/e2e_m044_s02.rs` with `m044_s02_metadata_` coverage for manifestless builds, invalid executable targets, and undeclared-target absence.
4. Keep raw HTTP route handlers and undeclared helpers out of this execution plan entirely.

## Must-Haves

- [ ] Declared clustered targets survive past validation as compiler-owned execution metadata.
- [ ] Invalid executable targets fail before codegen with explicit target/reason output.
- [ ] Manifestless and undeclared code paths stay ordinary local behavior.

## Inputs

- ``compiler/mesh-pkg/src/manifest.rs` — current clustered declaration schema and validator from S01.`
- ``compiler/meshc/src/main.rs` — build path that validates declarations and then drops the manifest before lowering.`
- ``compiler/mesh-lsp/src/analysis.rs` — mirrored clustered declaration validation that must stay compiler-aligned.`
- ``compiler/mesh-typeck/src/lib.rs` — existing `ServiceExportInfo` mapping that later tasks will use for service wrapper planning.`

## Expected Output

- ``compiler/mesh-pkg/src/manifest.rs` — richer execution-metadata shape derived from validated declarations.`
- ``compiler/meshc/src/main.rs` — compiler plumbing that carries clustered execution metadata to the lowering/codegen boundary.`
- ``compiler/mesh-lsp/src/analysis.rs` — editor-side parity for any narrowed executable declaration semantics.`
- ``compiler/meshc/tests/e2e_m044_s02.rs` — named `m044_s02_metadata_` coverage for the new metadata seam.`

## Verification

`cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture`
`cargo test -p meshc --test e2e_m044_s01 m044_s01_manifest_ -- --nocapture`

## Observability Impact

- Signals added/changed: compiler diagnostics distinguish declaration-parse failure from execution-planning failure.
- How a future agent inspects this: run the named `m044_s02_metadata_` filter and compare emitted clustered-target diagnostics against `mesh.toml`.
- Failure state exposed: declaration kind, target, and missing/invalid executable-symbol reason.
