---
estimated_steps: 31
estimated_files: 4
skills_used: []
---

# T01: Added optional clustered manifest declarations with shared compiler/LSP validation and the first M044 contract tests.

---
estimated_steps: 4
estimated_files: 4
skills_used:
  - rust-best-practices
  - test
---

Define the public clustered-app declaration contract at the manifest/compiler boundary first so later runtime and proof-app work do not guess at handler naming or accidentally make `mesh.toml` mandatory for all builds.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Shared `mesh.toml` parsing in `mesh-pkg` | Fail the build with the manifest path and parser reason; do not silently ignore an invalid clustered config. | N/A — parsing is synchronous. | Reject partial clustered config instead of defaulting to a broader or looser clustered boundary. |
| Export collection in `meshc` | Stop before MIR lowering and report the unresolved/private target explicitly. | N/A — export collection is synchronous. | Treat unknown handler kinds or bad target strings as contract errors, not as local-only fallbacks. |
| Project-aware LSP analysis | Keep editor diagnostics aligned with `meshc build` and surface drift as a test failure. | Fail the LSP test instead of returning stale single-file truth. | Reject manifest-backed clustered apps that analyze differently than the compiler path. |

## Load Profile

- **Shared resources**: manifest parsing, project-wide export discovery, and project-aware LSP analysis.
- **Per-operation cost**: one optional manifest parse plus one export-validation pass per build/analyze request.
- **10x breakpoint**: broad project analysis and repeated validation work would drift first if the clustered declaration resolver is duplicated instead of shared.

## Negative Tests

- **Malformed inputs**: missing `[cluster]` fields, unknown handler kinds, bad target strings, and empty declaration arrays.
- **Error paths**: unknown targets, private targets, or service-method mismatches must fail closed.
- **Boundary conditions**: manifest absent, clustered section absent, and manifest present on a project that still has to build locally without clustered declarations.

## Steps

1. Extend `compiler/mesh-pkg/src/manifest.rs` with an optional clustered-app section that keeps existing package/dependency parsing intact and leaves non-manifest builds valid.
2. Teach `compiler/meshc/src/main.rs` to load the optional manifest when present, validate clustered declarations after export collection and before MIR lowering, and emit explicit diagnostics for unknown/private/mismatched targets.
3. Mirror the same manifest-aware clustered validation in `compiler/mesh-lsp/src/analysis.rs` so editor truth stays aligned with the compiler path.
4. Create `compiler/meshc/tests/e2e_m044_s01.rs` with `manifest_`/`clustered_manifest_`-prefixed coverage and add matching parser/LSP tests so the later verifier can target them fail-closed.

## Must-Haves

- [ ] Clustered mode is an app-level opt-in in `mesh.toml`, not a mandatory compiler prerequisite.
- [ ] The declaration boundary is explicit and narrow: service calls, service casts, and public work functions only.
- [ ] Invalid declarations fail before codegen and show the exact bad target/reason in both compiler and LSP proof rails.

## Inputs

- ``compiler/mesh-pkg/src/manifest.rs` — current shared manifest schema with only package/dependency data.`
- ``compiler/meshc/src/main.rs` — current build pipeline that discovers, type-checks, and lowers without manifest validation.`
- ``compiler/mesh-lsp/src/analysis.rs` — project-aware editor analysis that must stay aligned with compiler semantics.`
- ``compiler/meshc/tests/e2e_m043_s02.rs` — existing compile-only continuity harness pattern to reuse for new M044 tests.`

## Expected Output

- ``compiler/mesh-pkg/src/manifest.rs` — optional clustered-app manifest schema and parser tests.`
- ``compiler/meshc/src/main.rs` — fail-closed clustered declaration validation in the build pipeline.`
- ``compiler/mesh-lsp/src/analysis.rs` — manifest-aware clustered analysis/test coverage.`
- ``compiler/meshc/tests/e2e_m044_s01.rs` — manifest/declaration contract tests with stable prefixes for the verifier.`

## Verification

`cargo test -p mesh-pkg clustered_manifest_ -- --nocapture`
`cargo test -p meshc --test e2e_m044_s01 manifest_ -- --nocapture`
`cargo test -p mesh-lsp clustered_manifest_ -- --nocapture`

## Observability Impact

- Signals added/changed: compiler and LSP diagnostics identify the exact clustered target and why validation rejected it.
- How a future agent inspects this: run the named `clustered_manifest_` / `manifest_` tests and compare `meshc build` vs LSP diagnostics.
- Failure state exposed: whether the drift is manifest parse failure, target-resolution failure, or compiler/LSP contract mismatch.
