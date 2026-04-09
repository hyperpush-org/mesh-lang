# S03 UAT: Daily-Driver Tooling Trust

## Purpose

Validate that `reference-backend/` can now be used as a real day-to-day backend project without the Mesh toolchain lying, panicking, or hiding behind toy-only proof. This UAT checks the concrete S03 trust claims: formatter safety on backend files, truthful `meshc test <path>` behavior, honest unsupported coverage, real stdio JSON-RPC LSP proof, and synced docs/editor instructions.

## Preconditions

- Run every command from the repo root.
- Rust/Cargo is installed.
- `rg` is installed.
- If the repo uses a local `.env`, load it before running commands that expect it:
  ```bash
  set -a
  source .env
  set +a
  ```
- No manual DB setup is required for the S03 tooling checks themselves.

## Test Case 1 — Formatter trust on the real backend tree

### Steps

1. Run the formatter crate regression suite:
   ```bash
   cargo test -p mesh-fmt -- --nocapture
   ```
2. Run the CLI formatter end-to-end suite:
   ```bash
   cargo test -p meshc --test e2e_fmt -- --nocapture
   ```
3. Run the real backend check-mode command:
   ```bash
   cargo run -p meshc -- fmt --check reference-backend
   ```
4. Verify the backend health file is still present and unchanged in-place behavior was respected:
   ```bash
   test -f reference-backend/api/health.mpl
   ```

### Expected outcome

- All formatter tests pass.
- `meshc fmt --check reference-backend` exits 0.
- The command reports already-formatted files instead of panicking or rewriting source.
- The verified backend formatter path is safe to reuse from the LSP document-formatting surface.

## Test Case 2 — Exported type aliases stay format-safe

### Steps

1. Create a temporary Mesh file containing an exported type alias:
   ```bash
   mkdir -p .gsd/tmp
   cat > .gsd/tmp/format-type-alias.mpl <<'EOF'
   pub type UserId = Int
   EOF
   ```
2. Format it:
   ```bash
   cargo run -p meshc -- fmt .gsd/tmp/format-type-alias.mpl
   ```
3. Inspect the file contents:
   ```bash
   cat .gsd/tmp/format-type-alias.mpl
   ```
4. Confirm the old broken token never appears:
   ```bash
   ! rg -n "pubtype" .gsd/tmp/format-type-alias.mpl
   ```

### Expected outcome

- Formatting succeeds.
- The file still contains `pub type UserId = Int`.
- `pubtype` never appears.

## Test Case 3 — `meshc test` works for project root, tests dir, and single file

### Steps

1. Run the backend project-root workflow:
   ```bash
   cargo run -p meshc -- test reference-backend
   ```
2. Run the nested test-directory workflow:
   ```bash
   cargo run -p meshc -- test reference-backend/tests
   ```
3. Run the specific backend test file:
   ```bash
   cargo run -p meshc -- test reference-backend/tests/config.test.mpl
   ```
4. Confirm the test file exists and targets backend config helpers:
   ```bash
   rg -n "database_url_key|port_key|job_poll_ms_key|missing_required_env|invalid_positive_int" reference-backend/tests/config.test.mpl
   ```

### Expected outcome

- All three invocations exit 0.
- The project-root command reports one passing test file and two passing test assertions inside `config.test.mpl`.
- The nested-directory and single-file commands also pass.
- Imports resolve correctly even when the requested path is below the project root.

## Test Case 4 — Coverage stays honest until it really exists

### Steps

1. Run the unsupported coverage command:
   ```bash
   cargo run -p meshc -- test --coverage reference-backend
   ```
2. Capture the shell exit status:
   ```bash
   echo $?
   ```
3. Re-run the tooling integration suite to confirm the contract is mechanically enforced:
   ```bash
   cargo test -p meshc --test tooling_e2e -- --nocapture
   ```

### Expected outcome

- `meshc test --coverage reference-backend` exits non-zero.
- Stderr includes:
  ```text
  coverage reporting is not implemented for `meshc test`; run the command without --coverage
  ```
- The command does **not** claim success, emit a placeholder coverage report, or print a “coming soon” success message.

## Test Case 5 — Real LSP transport proof passes on backend-shaped files

### Steps

1. Run the meshc-side JSON-RPC regression suite:
   ```bash
   cargo test -p meshc --test e2e_lsp -- --nocapture
   ```
2. Run the mesh-lsp crate tests:
   ```bash
   cargo test -p mesh-lsp -- --nocapture
   ```
3. (Optional spot check) Confirm the CLI language-server entrypoint exists:
   ```bash
   cargo run -p meshc -- lsp --help
   ```

### Expected outcome

- The JSON-RPC suite passes.
- The suite proves these phases against `reference-backend/api/health.mpl` and `reference-backend/api/jobs.mpl`: initialize, didOpen diagnostics, hover, definition, formatting, signature help, invalid-buffer diagnostics, and shutdown.
- The mesh-lsp crate tests pass, including the project-import-aware backend analysis coverage.
- `meshc lsp --help` exits 0.

## Test Case 6 — Public docs and editor instructions match the verified contract

### Steps

1. Re-run the stale-string sweep used by slice closure:
   ```bash
   ! rg -n "meshc new|mesh fmt|meshc test \.|mesh-lang-0\.1\.0\.vsix|Coverage reporting is available as a stub" README.md website/docs/docs/tooling/index.md website/docs/docs/testing/index.md website/docs/docs/cheatsheet/index.md tools/editors/vscode-mesh/README.md reference-backend/README.md
   ```
2. Confirm the top-level README points at the modern tooling commands:
   ```bash
   rg -n "meshc init|meshc fmt|meshc test <project-or-dir>|LSP" README.md
   ```
3. Confirm the backend README documents the verified S03 workflow:
   ```bash
   rg -n "fmt --check reference-backend|test reference-backend|test --coverage reference-backend|cargo test -p meshc --test e2e_lsp" reference-backend/README.md
   ```
4. Confirm the VS Code README documents the verified feature set and current local-install artifact:
   ```bash
   rg -n "Verified LSP Diagnostics|Verified Hover|Verified Go to Definition|Verified Document Formatting|Verified Signature Help|mesh-lang-0.3.0.vsix" tools/editors/vscode-mesh/README.md
   ```

### Expected outcome

- The negative `rg` sweep returns no matches.
- The README/docs surfaces reference `meshc init`, `meshc fmt`, project-directory `meshc test`, honest unsupported coverage wording, and the current VSIX artifact.
- The editor docs describe only the LSP capabilities that now have transport-level proof.

## Edge cases to watch

### 1. Formatter regression edge case

If `meshc fmt --check reference-backend` crashes, panics, or reports an overflow instead of an ordinary formatting result, S03 is no longer trustworthy.

### 2. Test-runner path-semantics edge case

If `meshc test reference-backend/tests` or `meshc test reference-backend/tests/config.test.mpl` starts failing with import-resolution errors while `meshc test reference-backend` still passes, the discovery-root/project-root split regressed.

### 3. Coverage-truth edge case

If `meshc test --coverage reference-backend` exits 0 without producing a real coverage artifact, treat that as a failure, not as progress.

### 4. LSP project-awareness edge case

If the LSP suite starts reporting import/module-not-found noise for canonical backend files, or if invalid-buffer diagnostics stop localizing to the edited file, the project-aware overlay analysis regressed.

## Cleanup

Remove the temporary exported-type-alias formatter probe if desired:

```bash
rm -f .gsd/tmp/format-type-alias.mpl
```
