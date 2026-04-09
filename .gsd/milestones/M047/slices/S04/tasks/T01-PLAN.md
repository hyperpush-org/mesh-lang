---
estimated_steps: 4
estimated_files: 6
skills_used:
  - test
---

# T01: Hard-cut legacy clustered declarations in parser/pkg/compiler surfaces

Finish the bridge D268 explicitly left in place for S04. Remove `clustered(work)` and manifest `[cluster]` as supported clustered-definition inputs, keep `@cluster` / `@cluster(N)` as the only accepted authoring surface, and turn legacy cases into explicit migration-oriented parser/pkg/compiler diagnostics instead of quiet compatibility.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| parser recovery + AST clustered-declaration extraction | emit a source-local cutover diagnostic and stop before building clustered metadata | N/A | reject mixed or partial legacy tokens instead of synthesizing a clustered declaration |
| mesh-pkg manifest loading / validation | fail manifest parsing with an explicit `[cluster]` migration error and do not generate clustered execution metadata | N/A | treat partial cluster config as invalid input, not as a fallback declaration source |
| compiler/LSP clustered diagnostics | keep errors anchored on the real source or manifest location instead of collapsing to unrelated project-level noise | N/A | prefer one explicit cutover message over cascading legacy-compat follow-on errors |

## Load Profile

- **Shared resources**: compiler parse/validation diagnostic buffers and clustered export-surface construction.
- **Per-operation cost**: one parse plus one manifest/source validation pass per package; no network or runtime work.
- **10x breakpoint**: large mixed-source packages will amplify cascading diagnostic spam first, so the cutover path should keep errors bounded and source-local.

## Negative Tests

- **Malformed inputs**: `clustered(work)` before `fn|def`, mixed `@cluster` + legacy declarations, stale `[cluster]` manifest sections, and malformed legacy target entries.
- **Error paths**: source-only builds using the old syntax fail before codegen, manifest-only legacy declarations fail with migration guidance, and compiler/LSP ranges still point at the right source after compatibility removal.
- **Boundary conditions**: valid `@cluster`, valid `@cluster(3)`, and existing private/decorated validation failures remain source-ranged and truthful after the hard cut.

## Steps

1. Remove the legacy parser/AST clustered declaration acceptance path and replace it with explicit cutover diagnostics that steer users toward `@cluster` / `@cluster(N)`.
2. Delete manifest `[cluster]` declaration support from mesh-pkg validation/export-surface construction and replace it with fail-closed migration guidance.
3. Update compiler/LSP rails that still depend on legacy provenance or wording so the only supported clustered-definition model is source-first.
4. Add parser/pkg/compiler regression cases named for `m047_s04` so the hard cut stays provable instead of being a one-off grep expectation.

## Must-Haves

- [ ] `clustered(work)` no longer produces a valid clustered declaration in parser/pkg/compiler flows.
- [ ] `[cluster]` manifest declarations fail closed with explicit migration guidance and no fallback clustered metadata.
- [ ] `@cluster` / `@cluster(N)` diagnostics remain source-ranged and truthful after the compatibility path is removed.

## Inputs

- ``compiler/mesh-parser/src/parser/items.rs``
- ``compiler/mesh-parser/src/ast/item.rs``
- ``compiler/mesh-parser/tests/parser_tests.rs``
- ``compiler/mesh-pkg/src/manifest.rs``
- ``compiler/meshc/tests/e2e_m047_s01.rs``
- ``compiler/mesh-lsp/src/analysis.rs``

## Expected Output

- ``compiler/mesh-parser/src/parser/items.rs``
- ``compiler/mesh-parser/src/ast/item.rs``
- ``compiler/mesh-parser/tests/parser_tests.rs``
- ``compiler/mesh-pkg/src/manifest.rs``
- ``compiler/meshc/tests/e2e_m047_s01.rs``
- ``compiler/mesh-lsp/src/analysis.rs``

## Verification

cargo test -p mesh-parser m047_s04 -- --nocapture && cargo test -p mesh-pkg m047_s04 -- --nocapture && cargo test -p meshc --test e2e_m047_s01 -- --nocapture

## Observability Impact

- Signals added/changed: cutover diagnostics should explicitly name the unsupported legacy source or manifest surface and point at the real location.
- How a future agent inspects this: `cargo test -p mesh-parser m047_s04 -- --nocapture`, `cargo test -p mesh-pkg m047_s04 -- --nocapture`, and compiler/LSP diagnostic output on a stale legacy fixture.
- Failure state exposed: silent compatibility fallback becomes an explicit parser/pkg/compiler failure instead of leaking into later runtime rails.
