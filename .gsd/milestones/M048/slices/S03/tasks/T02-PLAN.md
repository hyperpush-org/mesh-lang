---
estimated_steps: 4
estimated_files: 4
skills_used:
  - rust-best-practices
  - rust-testing
---

# T02: Wire `meshc update` and `meshpkg update` into the shared helper with honest help and JSON behavior

**Slice:** S03 — Toolchain self-update commands
**Milestone:** M048

## Description

Once the shared helper exists, the public CLI surface still needs to expose it explicitly and truthfully. This task makes the new command discoverable, keeps both binaries on the same updater seam, and closes the `meshpkg --json` ambiguity instead of letting installer prose masquerade as structured output.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Shared updater helper in `compiler/mesh-pkg/src/toolchain_update.rs` | Preserve the helper's phase-rich error text at the CLI boundary instead of wrapping it into a vague generic failure. | Bubble the helper timeout/launcher context unchanged. | Do not post-process installer output into a fake success shape. |
| Clap subcommand/help wiring in `compiler/meshc/src/main.rs` and `compiler/meshpkg/src/main.rs` | Fail tests if `update` is missing from help or dispatches to the wrong code path. | N/A for local CLI parsing. | Reject unsupported flag combinations before trying to launch the installer. |
| Global `--json` mode in `meshpkg` | Fail closed with one explicit machine-readable error before the installer launches. | N/A for local formatting. | Do not emit mixed human installer prose inside JSON mode. |

## Load Profile

- **Shared resources**: none beyond one updater call per invocation.
- **Per-operation cost**: clap parse + one helper dispatch; trivial relative to the installer work itself.
- **10x breakpoint**: operator confusion, not resource saturation, is the first failure mode here, so help text and guard rails must stay explicit.

## Negative Tests

- **Malformed inputs**: `meshpkg --json update`, unknown flags with `update`, and missing helper wiring.
- **Error paths**: helper returns a launcher/download error and the CLI must still print one clear failure.
- **Boundary conditions**: `meshc update --help`, `meshpkg update --help`, and `meshpkg --json update` all stay truthful.

## Steps

1. Add `Update` subcommands and dispatch in `compiler/meshc/src/main.rs` and `compiler/meshpkg/src/main.rs`, keeping top-of-file help text and command docs aligned with the new surface.
2. Route both commands through the shared `mesh_pkg` updater seam so the pair is refreshed through one code path instead of two bespoke implementations.
3. Make `meshpkg --json update` fail closed before the installer launches, with one clear machine-readable error that explains why JSON mode is unsupported for installer-backed self-update.
4. Add light CLI smoke coverage in `compiler/meshc/tests/tooling_e2e.rs` and `compiler/meshpkg/tests/update_cli.rs` for help/discovery plus the JSON-mode guard.

## Must-Haves

- [ ] `meshc --help` and `meshpkg --help` both expose an explicit `update` command.
- [ ] Both CLIs dispatch through the same shared updater helper.
- [ ] `meshpkg --json update` fails closed with a clear machine-readable error before any installer process starts.
- [ ] Light CLI tests pin help text, dispatch presence, and the JSON-mode guard so future refactors cannot silently remove the surface.

## Verification

- `cargo test -p meshc --test tooling_e2e test_update -- --nocapture`
- `cargo test -p meshpkg --test update_cli -- --nocapture`

## Inputs

- `compiler/meshc/src/main.rs` — current compiler CLI subcommand enum and dispatch table without `update`.
- `compiler/meshpkg/src/main.rs` — current package-manager CLI surface, including the global `--json` flag.
- `compiler/mesh-pkg/src/toolchain_update.rs` — shared updater seam introduced by T01 that both CLIs must call.
- `compiler/meshc/tests/tooling_e2e.rs` — existing light CLI smoke rail for `meshc` subcommand discovery/help behavior.

## Expected Output

- `compiler/meshc/src/main.rs` — `meshc update` subcommand docs and dispatch wired to the shared updater helper.
- `compiler/meshpkg/src/main.rs` — `meshpkg update` subcommand docs, dispatch, and fail-closed JSON-mode guard.
- `compiler/meshc/tests/tooling_e2e.rs` — smoke coverage that proves `meshc update` is discoverable and truthful in help output.
- `compiler/meshpkg/tests/update_cli.rs` — targeted CLI tests for `meshpkg update` help/discovery and the JSON-mode guard.
