---
estimated_steps: 4
estimated_files: 3
skills_used: []
---

# T02: Capture clustered source declarations with counts, provenance, and one shared surface helper

**Slice:** S01 — Source decorator reset for clustered functions
**Milestone:** M047

## Description

Move clustered declaration truth into one shared mesh-pkg seam so meshc and mesh-lsp stop reconstructing different answers. This task should produce a richer source declaration record, validated metadata that carries replication counts, and one shared export-surface helper that understands both ordinary work functions and service-generated clustered handlers.

## Negative Tests

- **Malformed inputs**: duplicate source declarations, duplicate manifest/source declarations, blank targets, and malformed qualified names still fail closed.
- **Error paths**: private functions, ambiguous overloaded work functions, and service helper mismatches return explicit validation errors instead of partial metadata.
- **Boundary conditions**: bare `@cluster` yields the default replication count `2`, `@cluster(N)` preserves the explicit count, and source provenance survives validation for later diagnostic mapping.

## Steps

1. Replace the lossy source collector with a richer source declaration model that stores qualified target, declaration kind, replication count, and declaration provenance.
2. Extend validated clustered execution metadata so later compiler/LSP tasks can read default-versus-explicit replication counts without reopening this seam.
3. Extract one shared clustered export-surface helper in mesh-pkg for public work functions and service-generated call/cast handlers.
4. Add mesh-pkg tests for source-only success, duplicate manifest/source declarations, private targets, and count/provenance preservation.

## Must-Haves

- [ ] Source declarations retain enough provenance to map validation failures back to a source file and range.
- [ ] Validated clustered execution metadata carries the default replication count `2` or the explicit `N` from source.
- [ ] One shared helper can build the clustered export surface for both meshc and mesh-lsp.

## Verification

- `cargo test -p mesh-pkg m047_s01 -- --nocapture`
- Confirm the M047 mesh-pkg tests assert on default count `2`, explicit count preservation, and source-origin duplicate/private failures.

## Observability Impact

- Signals added/changed: Clustered validation failures now carry declaration origin, target, range/provenance, and default-vs-explicit replication-count context.
- How a future agent inspects this: read the M047 mesh-pkg test assertions in `compiler/mesh-pkg/src/manifest.rs` and the downstream compiler/LSP diagnostics that consume this metadata.
- Failure state exposed: Duplicate declaration surfaces, private targets, ambiguous targets, and wrong count handling become distinguishable without reverse-engineering bare target strings.

## Inputs

- `compiler/mesh-pkg/src/manifest.rs` — current clustered declaration types, source collection, and validation live here but drop source provenance.
- `compiler/mesh-pkg/src/lib.rs` — public exports need to expose the new shared declaration/helper API to downstream crates.
- `compiler/mesh-typeck/src/lib.rs` — service export metadata shapes define what the shared export-surface helper can consume.

## Expected Output

- `compiler/mesh-pkg/src/manifest.rs` — clustered source declaration records, validation metadata, and M047 tests carry counts and provenance.
- `compiler/mesh-pkg/src/lib.rs` — downstream crates can import the shared clustered declaration/export-surface helpers from one place.
