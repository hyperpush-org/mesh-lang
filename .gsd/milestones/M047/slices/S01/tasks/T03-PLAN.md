---
estimated_steps: 4
estimated_files: 3
skills_used: []
---

# T03: Wire meshc to the shared clustered declaration model and source-ranged diagnostics

**Slice:** S01 — Source decorator reset for clustered functions
**Milestone:** M047

## Description

Make the compiler path truthful for the new source-first model. meshc should build a source-only `@cluster` package without depending on `[cluster]`, preserve the stable runtime registration/executable-symbol seam, and emit clustered validation diagnostics at the actual declaration site instead of falling back to manifest-only error reporting.

## Negative Tests

- **Malformed inputs**: duplicate manifest/source declarations, private decorated functions, and invalid decorator counts fail before codegen.
- **Error paths**: JSON diagnostics for source-origin failures include a real file/range instead of an empty file with no spans.
- **Boundary conditions**: bare `@cluster` and explicit `@cluster(3)` both compile through planning, and runtime registration names/executable symbols stay stable even though runtime semantics are unchanged in this slice.

## Steps

1. Replace meshc’s local clustered declaration/export-surface plumbing with the shared mesh-pkg helpers.
2. Thread the new replication-count metadata through prepared-build planning without changing runtime submission semantics yet.
3. Update plain and JSON clustered diagnostics to use source provenance for source-origin errors.
4. Add `compiler/meshc/tests/e2e_m047_s01.rs` covering source-only success, explicit-count planning, duplicate/private failures, and stable registration naming.

## Must-Haves

- [ ] Source-only `@cluster` projects build without `[cluster]` manifest declarations.
- [ ] Compiler diagnostics for source-origin clustered failures include the declaration file/range.
- [ ] Prepared build planning preserves the stable runtime registration/executable-symbol boundary while carrying count metadata forward.

## Verification

- `cargo test -p meshc --test e2e_m047_s01 -- --nocapture`
- Confirm the M047 e2e rail asserts both the clean source-only build path and the source-ranged error path.

## Observability Impact

- Signals added/changed: human and JSON compiler diagnostics expose declaration origin, declaration range, and count-context for clustered validation failures.
- How a future agent inspects this: run `meshc build` on the temporary M047 fixtures or replay `cargo test -p meshc --test e2e_m047_s01 -- --nocapture`.
- Failure state exposed: Empty-file JSON clustered errors, missing source spans, and registration-name drift become directly visible in the e2e rail.

## Inputs

- `compiler/meshc/src/main.rs` — current build planning still duplicates clustered export-surface logic and drops source-ranged diagnostics.
- `compiler/mesh-pkg/src/manifest.rs` — this task consumes the new shared declaration/count/provenance API from T02.
- `compiler/meshc/tests/e2e_m046_s01.rs` — existing source-declared compiler e2e coverage is the reference starting point for the M047 rail.
- `compiler/mesh-codegen/src/codegen/mod.rs` — codegen-facing plan structures may need small plumbing updates to preserve count metadata without changing runtime behavior.

## Expected Output

- `compiler/meshc/src/main.rs` — meshc consumes the shared clustered declaration/export-surface seam and emits source-ranged clustered diagnostics.
- `compiler/meshc/tests/e2e_m047_s01.rs` — an M047 e2e rail proves source-only compile success, count metadata retention, and diagnostic truth.
- `compiler/mesh-codegen/src/codegen/mod.rs` — codegen-facing planning stays compatible with the existing runtime registration seam while accepting the richer metadata.
