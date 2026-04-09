# S01: Dual clustered-work declaration

**Goal:** Allow a Mesh package to declare clustered work either in `mesh.toml` or with a narrow source-level `clustered(work)` function marker, with both forms converging on the existing runtime-owned declared-handler boundary and without touching the later route-free startup/status work.
**Demo:** After this: After this: a tiny Mesh package can mark clustered work through `mesh.toml` or a source decorator, and both forms compile to the same declared runtime boundary.

## Tasks
- [x] **T01: Added fail-closed `clustered(work)` parser intake and seeded real compiler/LSP proof rails for T02.** — Introduce the narrow source declaration form without widening Mesh into a general annotation system. Keep the accepted syntax to a contextual `clustered(work)` item prefix immediately before `fn|def`, preserve the existing `@` lexer rejection, and expose the marker through `FnDef` so later planning code can consume it deterministically.

### Failure Modes
| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/mesh-parser/src/parser/mod.rs` item dispatch | emit a targeted parse error and stop before the following function body is swallowed | N/A | reject the prefix as invalid item syntax instead of silently treating it as a plain expression statement |
| `compiler/mesh-parser/src/parser/items.rs` function parser | keep undecorated functions on the existing path and recover to the next item boundary | N/A | leave the decorator accessor empty rather than fabricating clustered metadata |
| `compiler/mesh-parser/src/ast/item.rs` `FnDef` accessor | return `None` when the prefix is absent or malformed | N/A | keep malformed nodes observable through parser tests instead of hiding them |

### Load Profile
- **Shared resources**: parser event stream and snapshot tree output.
- **Per-operation cost**: one linear contextual-prefix parse before the existing function-definition parse.
- **10x breakpoint**: the real risk is cascade errors from losing item sync after malformed decorator input, not raw throughput.

### Negative Tests
- **Malformed inputs**: missing `(` or `)`, missing `work`, wrong payload like `clustered(service_call)`, and decorator prefixes not followed by `fn|def`.
- **Error paths**: malformed decorator forms surface parser errors instead of falling through as stray expressions or corrupting the following function body.
- **Boundary conditions**: decorated `pub fn`, decorated private `fn`, decorated `def`, undecorated `fn`, and mixed files containing decorated and undecorated functions.

### Steps
1. Add the minimal composite syntax node(s) and item-dispatch branch needed to recognize contextual `clustered(work)` before `fn|def`, without introducing `@` or reserving `clustered` globally.
2. Extend `parse_fn_def` and `FnDef` AST accessors so the marker is represented as optional metadata on an otherwise ordinary function definition.
3. Add parser snapshots and AST-focused tests that lock the valid syntax and fail-closed recovery behavior.

### Must-Haves
- [ ] Only the narrow `clustered(work)` item prefix is accepted in S01.
- [ ] Undecorated functions continue to parse exactly as before.
- [ ] Malformed decorator forms fail closed with targeted parser errors.
- [ ] `FnDef` exposes a stable accessor the compiler/LSP task can consume.
  - Estimate: 2h
  - Files: compiler/mesh-parser/src/parser/mod.rs, compiler/mesh-parser/src/parser/items.rs, compiler/mesh-parser/src/ast/item.rs, compiler/mesh-parser/src/syntax_kind.rs, compiler/mesh-parser/tests/parser_tests.rs, compiler/mesh-parser/tests/snapshots/parser_tests__m046_s01_parser_clustered_work_fn_def.snap, compiler/mesh-parser/tests/snapshots/parser_tests__m046_s01_parser_clustered_work_invalid_prefix.snap
  - Verify: cargo test -p mesh-parser --test parser_tests m046_s01_parser_ -- --nocapture
- [x] **T02: Merged source `clustered(work)` declarations into the shared clustered planner for meshc and mesh-lsp.** — Feed decorated work functions into the existing declared-handler path without adding a second runtime boundary. The compiler and LSP should collect source-declared work, convert it into the same logical declaration records used by `mesh.toml`, reject same-target source+manifest duplicates explicitly, and keep the M044 manifest rails green.

### Failure Modes
| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/mesh-pkg/src/manifest.rs` clustered validation helpers | surface a deterministic duplicate or invalid-target diagnostic and abort planning before codegen | N/A | reject the declaration instead of emitting partial `ClusteredExecutionMetadata` |
| `compiler/meshc/src/main.rs` build merge path | emit build diagnostics and stop before declared-handler preparation | N/A | avoid registering mismatched runtime names or executable symbols |
| `compiler/mesh-lsp/src/analysis.rs` project analysis path | report the same duplicate or invalid-target reason as `meshc` | N/A | avoid false-clean editor output for source-only decorated projects |

### Load Profile
- **Shared resources**: full-project parse/typecheck/export scans reused by both `meshc` and `mesh-lsp`.
- **Per-operation cost**: one additional scan of function items to collect source declarations before existing clustered validation.
- **10x breakpoint**: compiler/LSP drift in declaration collection or duplicate handling will show up first as inconsistent diagnostics, not raw CPU cost.

### Negative Tests
- **Malformed inputs**: decorated private work functions, ambiguous overloaded public work names, same-target manifest/source declarations, and wrong-boundary targets.
- **Error paths**: source-only and mixed declaration projects fail before declared-handler generation when a decorated target cannot resolve to a valid public work function or when the same target is declared twice.
- **Boundary conditions**: manifest-only projects stay green, source-only projects emit the same registration surface, and source-declared work can coexist with manifest-declared service handlers without forking runtime behavior.

### Steps
1. Collect decorated work functions from parsed `FnDef`s and convert them into the same logical declaration shape used by manifest declarations, with an explicit same-target duplicate policy between source and manifest.
2. Wire the merged declarations into `prepare_project_build(...)` and LSP analysis so source-only decorated projects get the same `ClusteredExecutionMetadata` validation and declared-handler planning path as manifest projects.
3. Add compiler and LSP proof rails that cover source-only success, duplicate failure, private/ambiguous rejection, emitted declared-handler registration, and M044 manifest regressions.

### Must-Haves
- [ ] Source-only clustered work reaches the existing declared-handler pipeline with no runtime-path fork.
- [ ] Manifest-only clustered work remains green.
- [ ] Same-target manifest/source duplicates fail closed with an explicit diagnostic.
- [ ] LSP diagnostics match compiler behavior for source-only and invalid decorated work.
- [ ] Proof rails live in real tests under `compiler/meshc/tests/e2e_m046_s01.rs` and `compiler/mesh-lsp/src/analysis.rs`.
  - Estimate: 3h
  - Files: compiler/mesh-pkg/src/manifest.rs, compiler/meshc/src/main.rs, compiler/mesh-lsp/src/analysis.rs, compiler/meshc/tests/e2e_m046_s01.rs
  - Verify: cargo test -p meshc --test e2e_m046_s01 m046_s01_ -- --nocapture && cargo test -p mesh-lsp m046_s01_ -- --nocapture && cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture && cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture
