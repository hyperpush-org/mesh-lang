---
estimated_steps: 4
estimated_files: 8
skills_used:
  - rust-best-practices
  - test
---

# T01: Teach typecheck and diagnostics the `HTTP.clustered(...)` route wrapper

**Slice:** S07 — Clustered HTTP route wrapper completion
**Milestone:** M047

## Description

Make `HTTP.clustered` a compiler-known surface instead of an undefined stdlib lookup. This task should accept only bare route-handler references, validate both direct and pipe-form registrations, preserve imported handler origin for runtime names, and turn misuse into source-local diagnostics instead of generic `undefined variable` fallout.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| typecheck stdlib/builtin HTTP surface | fail with wrapper-specific diagnostics at the call range; do not fall back to generic `NoSuchField` / `UnboundVariable` noise | N/A | reject mismatched wrapper shapes instead of inferring a fake helper type |
| import-context handler resolution | preserve the defining module for bare imported handlers or fail the focused tests; do not collapse `Api.Todos.handle_list_todos` to the local module name | N/A | treat missing or ambiguous origin metadata as a contract failure, not as a guessed runtime name |
| LSP diagnostic projection | surface the same wrapper misuse spans through editor diagnostics or fail the focused tests; do not leave stale clustering diagnostics pointing at unrelated tokens | N/A | malformed diagnostic ranges are test failures, not acceptable best-effort output |

## Load Profile

- **Shared resources**: import-context maps, route-wrapper metadata tables, and LSP diagnostic rendering.
- **Per-operation cost**: one parse/typecheck pass plus focused unit/integration assertions.
- **10x breakpoint**: conflicting wrapper metadata and imported-name origin drift fail long before throughput matters.

## Negative Tests

- **Malformed inputs**: closure/anonymous-fn arguments, call expressions, non-route-position use, and private handlers.
- **Error paths**: conflicting duplicate counts for the same handler, wrapper use under direct vs piped route registration, and imported bare handlers with missing origin metadata.
- **Boundary conditions**: default vs explicit counts, module-qualified handlers, and imported bare handlers all produce stable runtime identities.

## Steps

1. Add compiler-known `HTTP.clustered` typing and a sibling metadata map on `InferCtx` / `TypeckResult` that records wrapper callsite info plus defining-module origin for bare imported handlers.
2. Validate both direct `HTTP.on_get(router, "/x", HTTP.clustered(handle))` and piped `router |> HTTP.on_get("/x", HTTP.clustered(handle))` forms, accepting only bare handler refs or module-qualified refs and rejecting non-route-position use.
3. Thread new error variants and renderers through `compiler/mesh-typeck/src/error.rs`, `compiler/mesh-typeck/src/diagnostics.rs`, and `compiler/mesh-lsp/src/analysis.rs` so misuse localizes to the wrapper call instead of unrelated route tokens.
4. Add focused `mesh-typeck` / `mesh-lsp` tests covering imported bare-handler origin, default vs explicit counts, direct and piped forms, and the closed-failure cases.

## Must-Haves

- [ ] `HTTP.clustered(handler)` and `HTTP.clustered(N, handler)` type-check only in route-handler position.
- [ ] Imported bare handlers keep their defining-module runtime identity for later lowering.
- [ ] Misuse produces named, source-local diagnostics rather than generic undefined-symbol fallout.

## Verification

- `cargo test -p mesh-typeck m047_s07 -- --nocapture`
- `cargo test -p mesh-lsp m047_s07 -- --nocapture`

## Observability Impact

- Signals added/changed: wrapper-specific error codes and call-range diagnostics instead of generic undefined-symbol fallout.
- How a future agent inspects this: rerun the `m047_s07` typecheck/LSP tests and inspect the rendered diagnostic text and spans.
- Failure state exposed: route-position misuse, imported-origin drift, and conflicting count declarations become explicit, named failures.

## Inputs

- `compiler/mesh-typeck/src/infer.rs` — current stdlib HTTP surface and call inference.
- `compiler/mesh-typeck/src/builtins.rs` — builtin function registry that must learn `HTTP.clustered`.
- `compiler/mesh-typeck/src/unify.rs` — current `InferCtx` metadata surfaces, including imported functions and overloaded call targets.
- `compiler/mesh-typeck/src/lib.rs` — `TypeckResult` handoff to lowering.
- `compiler/mesh-typeck/src/error.rs` — existing type-error catalog for source-local diagnostics.
- `compiler/mesh-typeck/src/diagnostics.rs` — diagnostic rendering and error-code mapping.
- `compiler/mesh-lsp/src/analysis.rs` — LSP projection of typecheck diagnostics.
- `reference-backend/api/router.mpl` — real imported bare-handler pipe-form route shape to preserve.

## Expected Output

- `compiler/mesh-typeck/src/infer.rs` — compiler-known `HTTP.clustered` route-wrapper inference and metadata capture.
- `compiler/mesh-typeck/src/builtins.rs` — builtin registry entries for the wrapper surface.
- `compiler/mesh-typeck/src/unify.rs` — enriched inference metadata for route-wrapper lowering.
- `compiler/mesh-typeck/src/lib.rs` — `TypeckResult` fields that preserve wrapper metadata and imported handler origin.
- `compiler/mesh-typeck/src/error.rs` — wrapper-specific closed-failure errors.
- `compiler/mesh-typeck/src/diagnostics.rs` — rendered diagnostics for wrapper misuse.
- `compiler/mesh-lsp/src/analysis.rs` — aligned editor diagnostics for the new errors.
- `compiler/mesh-typeck/tests/http_clustered_routes.rs` — focused direct, piped, imported-handler, and rejection assertions.
