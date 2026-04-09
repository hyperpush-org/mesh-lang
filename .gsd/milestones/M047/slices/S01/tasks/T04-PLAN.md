---
estimated_steps: 4
estimated_files: 2
skills_used: []
---

# T04: Anchor mesh-lsp clustered diagnostics on the decorated source declaration

**Slice:** S01 — Source decorator reset for clustered functions
**Milestone:** M047

## Description

Finish the source-first reset on the editor side. mesh-lsp should consume the same clustered declaration/export-surface helpers as meshc and translate clustered validation failures into declaration-range diagnostics so valid source-only `@cluster` code stays clean while invalid declarations point at the right line.

## Negative Tests

- **Malformed inputs**: duplicate manifest/source declarations and private decorated functions still produce clustered diagnostics in project analysis.
- **Error paths**: source-origin clustered diagnostics no longer collapse to `(0,0)` or project-wide placeholders.
- **Boundary conditions**: valid source-only `@cluster` code remains diagnostics-clean, and the decorated declaration line is the anchor for clustered failures.

## Steps

1. Replace mesh-lsp’s local clustered export-surface builder with the shared mesh-pkg helper.
2. Translate clustered validation errors into LSP ranges using the recorded source provenance from T02.
3. Add M047 analysis tests for source-only `@cluster` success plus duplicate/private failures landing on the decorated function line.
4. Replay the M047 mesh-lsp rail and keep the diagnostics wording aligned with meshc where the shared validator already defines the message.

## Must-Haves

- [ ] mesh-lsp consumes the same clustered declaration/export-surface path as meshc.
- [ ] Source-origin clustered diagnostics land on the decorated declaration range instead of `(0,0)`.
- [ ] Valid source-only `@cluster` code stays diagnostics-clean in project analysis.

## Verification

- `cargo test -p mesh-lsp m047_s01 -- --nocapture`
- Confirm the M047 LSP tests assert both the diagnostics-clean success case and the range-anchored failure case.

## Observability Impact

- Signals added/changed: clustered LSP diagnostics now expose the declaration range instead of a project-level fallback.
- How a future agent inspects this: replay `cargo test -p mesh-lsp m047_s01 -- --nocapture` or inspect the clustered diagnostic assertions in `compiler/mesh-lsp/src/analysis.rs`.
- Failure state exposed: mis-anchored `(0,0)` clustered diagnostics and message drift between meshc and mesh-lsp become explicit test failures.

## Inputs

- `compiler/mesh-lsp/src/analysis.rs` — project analysis currently duplicates clustered export-surface logic and wraps clustered failures in `(0,0)` diagnostics.
- `compiler/mesh-pkg/src/manifest.rs` — this task consumes the shared declaration/count/provenance API from T02.

## Expected Output

- `compiler/mesh-lsp/src/analysis.rs` — project analysis uses the shared clustered declaration/export-surface path and emits range-anchored clustered diagnostics.
