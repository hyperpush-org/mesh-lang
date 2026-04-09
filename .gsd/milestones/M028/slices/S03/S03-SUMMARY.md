---
id: S03
parent: M028
milestone: M028
provides:
  - overflow-safe formatter behavior on backend-shaped documents plus a passing `meshc fmt --check reference-backend` proof path
  - truthful `meshc test <path>` semantics for project roots, test directories, and specific `*.test.mpl` files, backed by a real `reference-backend/tests/config.test.mpl` Mesh test
  - transport-level `meshc lsp` JSON-RPC proof on `reference-backend/` covering diagnostics, hover, definition, formatting, and signature help
  - synced README/website/editor/backend docs tied to verified S03 commands and an honest unsupported coverage contract
requires:
  - S01
affects:
  - R006
  - R008
key_files:
  - compiler/mesh-fmt/src/printer.rs
  - compiler/mesh-fmt/src/lib.rs
  - compiler/mesh-fmt/src/walker.rs
  - compiler/meshc/src/main.rs
  - compiler/meshc/src/test_runner.rs
  - compiler/meshc/tests/e2e_fmt.rs
  - compiler/meshc/tests/tooling_e2e.rs
  - compiler/meshc/tests/e2e_lsp.rs
  - compiler/mesh-lsp/src/server.rs
  - compiler/mesh-lsp/src/analysis.rs
  - reference-backend/tests/config.test.mpl
  - reference-backend/README.md
  - README.md
  - website/docs/docs/tooling/index.md
  - website/docs/docs/testing/index.md
  - website/docs/docs/cheatsheet/index.md
  - tools/editors/vscode-mesh/README.md
key_decisions:
  - Treat formatter groups with `measure_flat(...) == usize::MAX` as forced-broken output instead of risking overflow, and keep the backend formatter path covered at unit, file, and CLI layers.
  - Define `meshc test <path>` as discovery-root semantics with project import scope anchored at the nearest ancestor containing `main.mpl`, and keep `--coverage` as an explicit unsupported non-zero contract until real reporting exists.
  - Make `meshc lsp` project-aware for backend-shaped files by resolving imports from the nearest `main.mpl` root and overlaying open-document contents during analysis.
  - Gate doc/editor truth with live command reruns plus a targeted negative stale-string sweep so drift fails mechanically instead of relying on manual reading.
patterns_established:
  - Tooling trust should be proved against the same `reference-backend/` package used elsewhere in M028, not against tiny standalone fixtures alone.
  - Directory-target developer tools often need split scopes: a user-facing discovery root and a compiler-facing project/module root.
  - Unsupported tooling features should fail explicitly and non-zero; green placeholders erode trust faster than a missing feature with an honest contract.
  - Editor claims should be backed by stdio JSON-RPC transport tests against canonical source files, not just unit tests or `--help` output.
  - Doc-truth work should close with both command reruns and a negative stale-string sweep over the user-facing surfaces.
observability_surfaces:
  - `cargo run -p meshc -- fmt --check reference-backend`
  - `cargo run -p meshc -- test reference-backend`
  - `cargo run -p meshc -- test --coverage reference-backend`
  - `cargo test -p meshc --test tooling_e2e -- --nocapture`
  - `cargo test -p meshc --test e2e_lsp -- --nocapture`
  - `cargo test -p mesh-lsp -- --nocapture`
  - `rg -n "meshc new|mesh fmt|meshc test \.|mesh-lang-0\.1\.0\.vsix|Coverage reporting is available as a stub" README.md website/docs/docs/tooling/index.md website/docs/docs/testing/index.md website/docs/docs/cheatsheet/index.md tools/editors/vscode-mesh/README.md reference-backend/README.md`
drill_down_paths:
  - .gsd/KNOWLEDGE.md
  - .gsd/DECISIONS.md
  - .gsd/milestones/M028/slices/S03/tasks/T01-SUMMARY.md
  - .gsd/milestones/M028/slices/S03/tasks/T02-SUMMARY.md
  - .gsd/milestones/M028/slices/S03/tasks/T03-SUMMARY.md
  - .gsd/milestones/M028/slices/S03/tasks/T04-SUMMARY.md
duration: slice closure verification + summary
verification_result: passed
completed_at: 2026-03-23
blocker_discovered: false
---
# S03: Daily-Driver Tooling Trust

## Outcome

S03 turns `reference-backend/` into a tooling surface a backend engineer can actually lean on. The formatter no longer panics on backend-shaped files, `meshc test` now tells the truth for real project-directory workflows, `meshc lsp` is mechanically proved over stdio JSON-RPC against backend code instead of toy snippets, and the public/editor docs now describe the commands and features that actually passed in this repo.

This slice did not add broad new tooling surface area. It made the existing surface credible on the same backend package S01/S02 already proved at runtime.

## What this slice actually delivered

### 1. Formatter trust on the real backend tree

S03 fixed the formatter failure mode that mattered most to daily work: `meshc fmt --check reference-backend` now exits cleanly instead of panicking on backend-shaped source.

The core formatter repair was in the printer’s group-fit logic:

- `measure_flat(...) == usize::MAX` is now treated as “must break” instead of participating in a width calculation that can overflow.
- grouped documents containing `Hardline` now choose broken rendering deterministically.
- the shared `mesh_fmt::format_source(...)` path used by CLI formatting and LSP formatting stays covered by regressions.

The slice also fixed one real source-safety regression in `compiler/mesh-fmt/src/walker.rs`: exported type aliases must keep a separator after `pub`, or formatting can turn valid `pub type Foo = Bar` source into parse-invalid `pubtype Foo = Bar`.

Formatter trust is now covered at three levels:

- printer unit tests (`grouped_hardline_breaks_instead_of_overflowing`)
- formatter regression tests on backend-shaped source (`reference_backend_health_file_formats_canonically`)
- CLI end-to-end checks on the real directory (`fmt_check_reference_backend_directory_succeeds`)

### 2. A truthful `meshc test` workflow for backend-shaped projects

Before S03, the documented project-directory test workflow was misleading. After S03:

- `meshc test reference-backend` works
- `meshc test reference-backend/tests` works
- `meshc test reference-backend/tests/config.test.mpl` works

The key semantic change is that the requested path is the test-discovery root, while project import resolution stays anchored at the nearest ancestor containing `main.mpl`. That split lets nested test-directory and single-file invocation work without breaking project-local imports.

S03 also added a real Mesh-native backend test file: `reference-backend/tests/config.test.mpl`. It verifies the canonical env-variable keys and the exact user-facing error strings for missing env / invalid positive-int failures.

Coverage behavior is now honest instead of aspirational. `meshc test --coverage reference-backend` exits non-zero with:

```text
coverage reporting is not implemented for `meshc test`; run the command without --coverage
```

That is intentionally a red path until real coverage artifacts exist.

### 3. Real JSON-RPC LSP proof on backend files

S03 adds the missing transport-level evidence that a real editor session works on backend-shaped code.

`compiler/meshc/tests/e2e_lsp.rs` now spawns `meshc lsp`, speaks stdio JSON-RPC directly, and asserts named phases against `reference-backend/api/health.mpl` and `reference-backend/api/jobs.mpl`:

- `initialize` / capability advertisement
- `didOpen` diagnostics on canonical backend files
- `hover`
- `definition`
- `signatureHelp`
- `textDocument/formatting`
- `didChange` diagnostics on intentionally broken backend text
- `shutdown`

To make that proof credible, the LSP analysis layer now behaves like a real project instead of a single-file sandbox:

- it discovers the nearest ancestor containing `main.mpl`
- it analyzes imports in project order
- it overlays open-document contents so diagnostics and editor requests reflect live buffers, not just on-disk files
- it reuses source↔tree offset helpers so diagnostics localize to actual backend source coordinates

The result is that backend diagnostics no longer degrade into bogus import noise when the editor buffer changes, and formatting/navigation/editor-assist claims are tied to a real transport proof.

### 4. Docs and editor guidance now match the verified command surface

S03 updated the top-level README, website tooling/testing/cheatsheet docs, `reference-backend/README.md`, and the VS Code README so they point at the commands and editor features that actually passed in this slice.

That includes:

- `meshc init` instead of stale scaffolding commands
- `meshc fmt` / `meshc fmt --check reference-backend`
- `meshc test reference-backend` and nested-path variants
- the honest unsupported `--coverage` contract
- the currently packaged VS Code install artifact (`mesh-lang-0.3.0.vsix`)
- the JSON-RPC-proven LSP feature set: diagnostics, hover, go-to-definition, document formatting, and signature help

The important pattern is not just that docs changed. Drift is now checked mechanically with a targeted negative `rg` sweep for known stale strings, so a backslide fails at closure time instead of silently shipping old claims.

## Verification that passed during slice closure

All slice-level verification gates passed from the repo root.

### Passed commands

```bash
cargo test -p mesh-fmt -- --nocapture
cargo test -p meshc --test e2e_fmt -- --nocapture
cargo run -p meshc -- fmt --check reference-backend
cargo run -p meshc -- test reference-backend
! cargo run -p meshc -- test --coverage reference-backend
cargo test -p meshc --test tooling_e2e -- --nocapture
cargo test -p meshc --test e2e_lsp -- --nocapture
cargo test -p mesh-lsp -- --nocapture
! rg -n "meshc new|mesh fmt|meshc test \.|mesh-lang-0\.1\.0\.vsix|Coverage reporting is available as a stub" README.md website/docs/docs/tooling/index.md website/docs/docs/testing/index.md website/docs/docs/cheatsheet/index.md tools/editors/vscode-mesh/README.md reference-backend/README.md
```

### Observability spot checks confirmed

Closure verification also confirmed the slice’s intended diagnostic surfaces behave usefully:

- `meshc fmt --check reference-backend` reported `11 file(s) already formatted` and exited cleanly instead of panicking.
- `meshc test reference-backend` ran the real backend Mesh test file and reported `2 passed` inside the file and `1 passed` at the file-summary level.
- `meshc test --coverage reference-backend` exited non-zero with the explicit unsupported message instead of a green placeholder.
- `e2e_lsp` proved hover, definition, formatting, signature help, and invalid-buffer diagnostics over real backend files at the JSON-RPC boundary.
- the stale-string sweep passed, which means the known false doc/editor phrases are gone from the user-facing surfaces this slice targeted.

## What changed in Mesh itself, not just in docs/tests

S03 raised trust by changing Mesh behavior at the source, not only by adding harnesses:

- the formatter printer now breaks instead of overflowing on groups that can never fit flat
- exported type aliases stay parse-valid under formatting
- `meshc test` now accepts project roots, nested test directories, and specific test files with stable import behavior
- `meshc test --coverage` now fails honestly instead of pretending coverage succeeded
- the LSP now performs project-aware backend analysis with live-buffer overlays instead of shallow single-file analysis

That matters for later slices because S03 did not merely document a better workflow; it repaired the underlying CLI/LSP/formatter behavior so the workflow is real.

## Forward intelligence

### What later slices can rely on

- `reference-backend/` now has a trustworthy tooling baseline for formatter, tests, LSP, and doc/editor instructions.
- The `meshc test <path>` contract is now stable enough for later backend examples and docs to reference directly.
- The JSON-RPC LSP harness is now the right place to extend proof for future editor features instead of adding more `--help`-level smoke checks.
- Coverage remains intentionally unsupported; future work can add real reporting later without inheriting a misleading success contract.

### Authoritative diagnostics

Use these first when S03 surfaces look broken:

1. `cargo run -p meshc -- fmt --check reference-backend`
   - Fastest signal for formatter regressions on the real backend tree.
2. `cargo run -p meshc -- test reference-backend`
   - Fastest signal for test-discovery/import-scope regressions on the canonical backend package.
3. `cargo run -p meshc -- test --coverage reference-backend`
   - Best truth check for the current coverage contract. Success is a regression until real coverage exists.
4. `cargo test -p meshc --test e2e_lsp -- --nocapture`
   - Best transport-level signal for editor behavior regressions.
5. the stale-string `rg` sweep over README/website/VS Code/reference-backend docs
   - Best fast check for documentation drift back to disproven commands or placeholder claims.

### Gotchas worth preserving

- `meshc test <path>` now has split semantics: discovery root from the requested path, project import root from the nearest ancestor containing `main.mpl`.
- Backend-shaped editor work needs project-aware LSP analysis plus open-document overlays. Falling back to single-file analysis causes misleading import failures and stale diagnostics.
- `walk_type_alias_def(...)` must keep a separator after `VISIBILITY`, or the formatter can create invalid `pubtype` output.
- Treat any green `--coverage` placeholder as a trust regression, not as progress.

### What S03 does not prove yet

S03 proves daily-driver tooling credibility on the canonical backend path. It does **not** yet prove:

- real coverage artifact generation
- boring native deployment and smoke verification outside the dev workflow
- crash recovery / supervision behavior under failure
- final production-style docs/examples promotion across the whole project surface

Those remain the jobs of S04-S06, but they now inherit a backend/tooling path that is much harder to dismiss as toy-only.
