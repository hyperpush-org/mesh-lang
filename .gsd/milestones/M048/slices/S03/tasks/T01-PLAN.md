---
estimated_steps: 4
estimated_files: 3
skills_used:
  - rust-best-practices
  - rust-testing
  - powershell-windows
---

# T01: Build a shared installer-backed updater seam in `mesh-pkg` with explicit cross-platform launcher coverage

**Slice:** S03 — Toolchain self-update commands
**Milestone:** M048

## Description

There is no shared self-update seam today, and the biggest risk in S03 is accidentally re-implementing release metadata, archive naming, checksum parsing, or install-location logic a second time inside Rust. This task creates the one reusable updater boundary both CLIs will call.

The helper should preserve D311 and D320 by downloading the existing public installer script, forwarding the staged-proof env overrides the installer already understands, and making the sharp Windows self-overwrite edge explicit in code and tests instead of assuming a running `.exe` can replace itself in place.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Installer script fetch via `ureq` and `MESH_UPDATE_INSTALLER_URL` / default public URL | Return a structured error that names the installer URL and stop before executing anything local. | Surface timeout context from the download phase instead of retrying invisibly or hanging. | Reject empty or obviously broken installer bytes before writing/executing a temp script. |
| Platform launcher selection (`/bin/sh` vs PowerShell bootstrap) | Fail closed with the attempted launcher path and platform in the error. | Surface the launcher/bootstrap phase instead of leaving the caller hung without context. | Reject impossible launcher plans (missing temp file, unsupported platform, bad script path) before spawn. |
| Existing `MESH_INSTALL_*` installer overrides | Pass them through unchanged and let the canonical installer own release/checksum/install failures. | N/A for local env reads. | Do not reinterpret release metadata, archive names, or checksum content inside Rust. |

## Load Profile

- **Shared resources**: one installer download, temp-script files, and one child installer/bootstrap process.
- **Per-operation cost**: one network fetch plus one shell/PowerShell launch; this is a rare operator command, not a hot path.
- **10x breakpoint**: repeated download/process-spawn overhead fails before CPU does, so the helper must stay stateless and avoid caching partial installer state under `~/.mesh`.

## Negative Tests

- **Malformed inputs**: invalid or unreachable `MESH_UPDATE_INSTALLER_URL`, empty installer body, and unsupported launcher/platform resolution.
- **Error paths**: download failure, temp-script write failure, and launcher/bootstrap spawn failure.
- **Boundary conditions**: default URL selection on Unix vs Windows, passthrough of staged-proof env vars, and staged binaries running from outside `~/.mesh/bin`.

## Steps

1. Add `compiler/mesh-pkg/src/toolchain_update.rs` and expose it from `compiler/mesh-pkg/src/lib.rs` as the one updater seam both CLIs will call.
2. Implement platform-aware installer download + launch planning: Unix should feed installer bytes to `/bin/sh` with `--yes`, while Windows should use an explicit PowerShell/bootstrap path that allows the invoking `.exe` to exit before replacement.
3. Forward `MESH_INSTALL_RELEASE_API_URL`, `MESH_INSTALL_RELEASE_BASE_URL`, `MESH_INSTALL_DOWNLOAD_TIMEOUT_SEC`, and `MESH_INSTALL_STRICT_PROOF` unchanged to the installer child, and keep error returns high-signal but secret-safe.
4. Add focused integration or unit coverage in `compiler/mesh-pkg/tests/toolchain_update.rs` for default URL selection, installer override handling, env passthrough, Unix launcher construction, and Windows bootstrap/PowerShell command construction.

## Must-Haves

- [ ] One shared `mesh_pkg` updater seam exists and both CLIs can call it instead of duplicating update logic.
- [ ] The helper downloads the platform installer script from the public default URL, with `MESH_UPDATE_INSTALLER_URL` as the only new staging override.
- [ ] Existing `MESH_INSTALL_*` overrides flow through unchanged to the installer child.
- [ ] Windows launcher planning is explicit and testable instead of assuming a running `.exe` can overwrite itself in place.

## Verification

- `cargo test -p mesh-pkg --test toolchain_update -- --nocapture`
- The helper tests cover default URL selection, installer URL override handling, env passthrough, and host/Windows launcher construction without re-implementing installer semantics.

## Observability Impact

- Signals added/changed: updater errors now name the installer URL, platform, and failing phase before the acceptance rail runs.
- How a future agent inspects this: rerun `cargo test -p mesh-pkg --test toolchain_update -- --nocapture` or point `MESH_UPDATE_INSTALLER_URL` at a staged server while invoking the CLI manually.
- Failure state exposed: download, write-installer, spawn-launcher, and wait/bootstrap failures are distinguishable.

## Inputs

- `compiler/mesh-pkg/src/lib.rs` — existing public exports that need to expose the shared updater seam.
- `tools/install/install.sh` — canonical Unix installer contract the helper must reuse instead of duplicating.
- `tools/install/install.ps1` — canonical Windows installer contract and self-overwrite constraint reference.
- `website/docs/public/install.sh` — hosted Unix installer copy the helper should conceptually target in production.
- `website/docs/public/install.ps1` — hosted Windows installer copy the helper should conceptually target in production.
- `scripts/verify-m034-s03.sh` — current staged-installer proof contract and local release layout reference.
- `scripts/verify-m034-s03.ps1` — current Windows staged-installer proof contract and launcher reference.

## Expected Output

- `compiler/mesh-pkg/src/toolchain_update.rs` — shared updater implementation with platform-aware installer download and launch behavior.
- `compiler/mesh-pkg/src/lib.rs` — exported updater seam for both CLIs.
- `compiler/mesh-pkg/tests/toolchain_update.rs` — focused tests for installer URL selection, override passthrough, and launcher planning.
