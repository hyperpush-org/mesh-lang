# S01: Source decorator reset for clustered functions — UAT

**Milestone:** M047
**Written:** 2026-04-01T06:06:56.809Z

# S01: Source decorator reset for clustered functions — UAT

**Milestone:** M047
**Written:** 2026-03-31T19:51:43-04:00

## UAT Type

- UAT mode: artifact-driven compiler/parser/editor verification
- Why this mode is sufficient: S01 changes language tooling seams rather than live runtime behavior. The honest acceptance surface is the four targeted parser, mesh-pkg, meshc, and mesh-lsp rails that prove syntax intake, shared validation metadata, compiler planning/diagnostics, and editor diagnostics together.

## Preconditions

- Run from the repo root with the Rust workspace toolchain available.
- `cargo` can build the workspace locally.
- No other process should be mutating the same Cargo test targets while the UAT commands run.

## Smoke Test

Run:

```bash
cargo test -p meshc --test e2e_m047_s01 -- --nocapture
```

**Expected:** 4 tests pass. The rail proves a source-only package can use `@cluster` / `@cluster(3)` without `[cluster]`, keeps stable runtime registration naming in emitted LLVM, and reports duplicate/private decorator failures against `work.mpl` instead of manifest-style fallback locations.

## Test Cases

### 1. Parser accepts source-first clustered decorators and keeps legacy compatibility

1. Run:
   ```bash
   cargo test -p mesh-parser m047_s01 -- --nocapture
   ```
2. Confirm the rail runs non-zero tests.
3. **Expected:** 14 parser tests pass, covering bare `@cluster`, counted `@cluster(3)`, malformed decorator shapes, stray `@`, missing `fn|def`, non-function attachment, missing `)`, and the compatibility-only `clustered(work)` snapshot.

### 2. mesh-pkg preserves clustered provenance, default counts, and shared export-surface truth

1. Run:
   ```bash
   cargo test -p mesh-pkg m047_s01 -- --nocapture
   ```
2. Confirm the rail runs non-zero tests.
3. **Expected:** 7 tests pass, proving source discovery records source provenance, bare `@cluster` resolves to replication count `2`, explicit `@cluster(N)` preserves the explicit count, duplicate manifest/source declarations fail closed, private-source declarations fail closed, malformed targets report count/source context, and one shared export-surface helper covers work functions plus service-generated handlers.

### 3. meshc builds source-only decorated clustered functions and emits source-ranged diagnostics

1. Run:
   ```bash
   cargo test -p meshc --test e2e_m047_s01 -- --nocapture
   ```
2. Confirm the rail runs 4 tests.
3. **Expected:**
   - a source-only package using bare `@cluster` and explicit `@cluster(3)` builds without `[cluster]` in `mesh.toml`
   - emitted LLVM still contains the stable declared-handler registration markers for `Work.handle_submit` and `Work.handle_retry`
   - a private `@cluster(3)` function fails before LLVM emission and the human diagnostic names `work.mpl:1:1`, the source decorator origin, and `replication count 3`
   - a duplicate manifest/source declaration fails in JSON mode with `file` pointing at `work.mpl` and a non-empty span array
   - malformed `@cluster` count syntax fails before codegen

### 4. mesh-lsp anchors clustered diagnostics on the decorated declaration line

1. Run:
   ```bash
   cargo test -p mesh-lsp m047_s01 -- --nocapture
   ```
2. Confirm the rail runs non-zero tests.
3. **Expected:** 3 tests pass, proving source-only `@cluster` analysis is diagnostics-clean, duplicate/private clustered failures are anchored to the decorated declaration range in the analyzed file, and clustered source-origin issues no longer surface as project-level `(0,0)` diagnostics.

## Edge Cases

### Malformed counted decorator fails closed

1. Use the existing parser/compiler rails above.
2. **Expected:** malformed forms like `@cluster(1, 2)` fail; parser recovery may still emit a follow-on recovery error, but the build must stop before codegen and must not silently coerce the count.

### Legacy syntax remains compatibility-only, not the new proof surface

1. Inspect the parser rail results from Test Case 1.
2. **Expected:** legacy `clustered(work)` coverage still passes so old rails stay green temporarily, but the real M047 proof surface is the new `@cluster` / `@cluster(N)` tests, not new feature work added to the old spelling.

## Failure Signals

- `cargo test -p mesh-parser m047_s01 -- --nocapture` runs 0 tests or loses decorator-specific coverage.
- `cargo test -p mesh-pkg m047_s01 -- --nocapture` stops proving default count `2`, explicit count preservation, or source provenance.
- `cargo test -p meshc --test e2e_m047_s01 -- --nocapture` emits LLVM for invalid decorator shapes, requires `[cluster]` for source-only builds, or reports duplicate/private errors without `work.mpl` source context.
- `cargo test -p mesh-lsp m047_s01 -- --nocapture` regresses back to project-level `(0,0)` clustered diagnostics or phantom errors on unrelated files.

## Requirements Proved By This UAT

- R097 — proves `@cluster` / `@cluster(N)` are real clustered declaration spellings for ordinary functions in parser, compiler, and LSP flows.
- R098 — proves bare decorators resolve to replication count `2` and explicit counts survive shared validation metadata.
- R099 — proves the new clustered declaration model applies to ordinary non-HTTP functions, not only future route wrappers.
- R106 — proves compiler and editor tooling now teach the source-first declaration model with real source ranges and count context instead of manifest-shaped fallback diagnostics.

## Not Proven By This UAT

- Runtime replication-count semantics above `1`; S01 only proves metadata/count truth, not runtime submit behavior.
- `HTTP.clustered(...)` route wrappers and clustered-route execution semantics; those belong to S03.
- Repo-wide hard cutover away from `clustered(work)` and manifest clustering in docs/examples/scaffolds; that belongs to S04 and S06.

## Notes for Tester

If the slice goes red, debug in this order: parser rail -> mesh-pkg rail -> meshc compiler rail -> mesh-lsp rail. The most trustworthy seams are `compiler/mesh-pkg/src/manifest.rs` for declaration/count/provenance truth, `compiler/meshc/tests/e2e_m047_s01.rs` for source-only build and source-ranged diagnostic expectations, and `compiler/mesh-lsp/src/analysis.rs` for the per-file source-origin diagnostic gate.
