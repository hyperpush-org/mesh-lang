# S02: Entrypoint-aware LSP, editors, and package surfaces — UAT

**Milestone:** M048
**Written:** 2026-04-02T09:34:52.882Z

# S02 UAT — Entrypoint-aware LSP, editors, and package surfaces

## Preconditions
- Work from the repository root.
- Rust/Cargo toolchain available.
- `target/debug/meshc` is buildable by the workspace tests.
- `nvim` 0.11+ is installed for the Neovim smoke.
- Node dependencies for `tools/editors/vscode-mesh` are installed.

## Test Case 1 — Unit-level LSP analysis honors manifest-first override entrypoints
1. Run `cargo test -p mesh-lsp m048_s02_ -- --nocapture`.
2. Confirm the focused M048/S02 tests pass.

Expected outcomes:
- `find_project_root` prefers the nearest `mesh.toml` over a nearer legacy `main.mpl` nested below it.
- An override-only project with `mesh.toml` + `lib/start.mpl` analyzes cleanly without requiring a root `main.mpl`.
- When both root `main.mpl` and `lib/start.mpl` exist, the override entry stays executable while root `main.mpl` remains discovered but non-entry.
- Missing or escaping manifest entrypoints surface one explicit project diagnostic instead of bogus import noise.

## Test Case 2 — Live `meshc lsp` JSON-RPC flow stays clean for override-entry projects
1. Run `cargo test -p meshc --test e2e_lsp lsp_json_rpc_override_entry_flow -- --nocapture`.
2. Read the test log for the override-entry `didOpen` and hover phases.

Expected outcomes:
- Opening `lib/start.mpl` and `lib/support/message.mpl` publishes `diagnostics=0` for both files.
- Hover over `message()` in `lib/start.mpl` returns a non-empty Mesh signature containing `String`.
- The test shuts down cleanly after the override-entry proof.

## Test Case 3 — Reference-backend LSP transport still preserves honest diagnostics
1. Run `cargo test -p meshc --test e2e_lsp lsp_json_rpc_backend_reference_flow -- --nocapture`.
2. Inspect the log lines for `health` / `jobs` open, definition, and invalid-change diagnostics.

Expected outcomes:
- `reference-backend/api/health.mpl` and `reference-backend/api/jobs.mpl` open with zero diagnostics.
- Definition on `create_job_response(job, body)` resolves back to the function definition inside `jobs.mpl`.
- After the test sends an invalid `didChange`, the server publishes at least one real diagnostic rather than staying green.

## Test Case 4 — Neovim host roots on `mesh.toml` first and keeps standalone mode honest
1. Run `NEOVIM_BIN="${NEOVIM_BIN:-nvim}" bash scripts/verify-m036-s02.sh lsp`.
2. Inspect the emitted `phase=lsp` lines.

Expected outcomes:
- `reference-backend/api/health.mpl` and `reference-backend/api/jobs.mpl` attach to one client rooted at `reference-backend/mesh.toml`.
- The materialized override-entry fixture roots at its own `mesh.toml`, and `lib/start.mpl` plus `lib/support/message.mpl` reuse the same client.
- The standalone temp file attaches with `marker=single-file` and `root=<none>`.
- The phase ends with `result=pass checked_cases=4`.

## Test Case 5 — Neovim contract docs/tests stay synchronized with runtime behavior
1. Run `node --test scripts/tests/verify-m036-s02-contract.test.mjs`.

Expected outcomes:
- The README still documents `pack/*/start/mesh-nvim`, explicit `mesh_lsp_path` overrides, manifest-first root detection, and the verifier commands.
- The runtime and exported LSP config still share the same root-marker order.
- The smoke script assertions still mention the override-entry case and `checked_cases=4`.

## Test Case 6 — VS Code smoke proves override-entry editor-host behavior
1. Run `npm --prefix tools/editors/vscode-mesh run test:smoke`.
2. Inspect the smoke log in `.tmp/m036-s03/vscode-smoke/smoke.log` if needed.

Expected outcomes:
- The extension resolves `meshc` from the configured `mesh.lsp.path`.
- `reference-backend` files open with clean diagnostics.
- The materialized override-entry project opens both `lib/start.mpl` and `lib/support/message.mpl` with clean diagnostics.
- Hover on `message()` in the override-entry project returns a non-empty Mesh signature containing `String`.
- Definition on the reference-backend probe still resolves to the expected line in `jobs.mpl`.

## Test Case 7 — `meshpkg publish` archives nested project-root Mesh sources truthfully
1. Run `cargo test -p meshpkg publish_archive_members_ -- --nocapture`.
2. Review the three targeted archive-member regressions.

Expected outcomes:
- An override-entry package archives `mesh.toml`, `lib/start.mpl`, and nested support modules under their project-root-relative paths.
- Hidden files/directories and `*.test.mpl` files are excluded.
- If both root `main.mpl` and an override entry exist, both source files remain archived, with no duplicate-path errors.

## Edge Cases to Replay Before Release
- `cargo test -p mesh-lsp m048_s02_missing_configured_entry_reports_project_diagnostic -- --nocapture`
  - Expected: one project diagnostic naming the missing `lib/start.mpl` entrypoint.
- `cargo test -p mesh-lsp m048_s02_invalid_manifest_entrypoint_reports_project_diagnostic -- --nocapture`
  - Expected: one project diagnostic explaining that the configured entrypoint must stay within the project root.
- `cargo test -p mesh-lsp m048_s02_override_precedence_keeps_root_main_path_derived_but_not_executable -- --nocapture`
  - Expected: root `main.mpl` stays discovered as `Main`, but the executable entry remains `lib/start.mpl` / `Lib.Start`.

