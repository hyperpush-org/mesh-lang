---
id: T01
parent: S03
milestone: M048
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-pkg/src/toolchain_update.rs", "compiler/mesh-pkg/src/lib.rs", "compiler/mesh-pkg/tests/toolchain_update.rs"]
key_decisions: ["Use the public install scripts as the only source of truth for update semantics; the Rust seam only downloads, validates, forwards env, and launches them.", "On Windows, spawn a PowerShell bootstrap and return so the invoking .exe can exit before replacement instead of assuming in-place self-overwrite is safe."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "`cargo test -p mesh-pkg --test toolchain_update -- --nocapture` passed with 15 focused updater tests covering default URL selection, installer override handling, env passthrough, Unix launcher construction, Windows bootstrap/PowerShell construction, and negative failure rails. Slice-level verification was also sampled: `cargo test -p meshpkg --test update_cli -- --nocapture` fails because the target does not exist yet (T02), `cargo test -p meshc --test tooling_e2e test_update -- --nocapture` exits 0 but matches 0 tests so update coverage is still missing until T02, and `cargo test -p meshc --test e2e_m048_s03 -- --nocapture` fails because the acceptance rail target does not exist yet (T03)."
completed_at: 2026-04-02T10:14:14.601Z
blocker_discovered: false
---

# T01: Added a shared mesh-pkg toolchain updater that downloads the public installer, forwards staged-proof env overrides, and uses a PowerShell bootstrap on Windows.

> Added a shared mesh-pkg toolchain updater that downloads the public installer, forwards staged-proof env overrides, and uses a PowerShell bootstrap on Windows.

## What Happened
---
id: T01
parent: S03
milestone: M048
key_files:
  - compiler/mesh-pkg/src/toolchain_update.rs
  - compiler/mesh-pkg/src/lib.rs
  - compiler/mesh-pkg/tests/toolchain_update.rs
key_decisions:
  - Use the public install scripts as the only source of truth for update semantics; the Rust seam only downloads, validates, forwards env, and launches them.
  - On Windows, spawn a PowerShell bootstrap and return so the invoking .exe can exit before replacement instead of assuming in-place self-overwrite is safe.
duration: ""
verification_result: mixed
completed_at: 2026-04-02T10:14:14.602Z
blocker_discovered: false
---

# T01: Added a shared mesh-pkg toolchain updater that downloads the public installer, forwards staged-proof env overrides, and uses a PowerShell bootstrap on Windows.

**Added a shared mesh-pkg toolchain updater that downloads the public installer, forwards staged-proof env overrides, and uses a PowerShell bootstrap on Windows.**

## What Happened

Added `compiler/mesh-pkg/src/toolchain_update.rs` as the shared installer-backed updater seam for downstream CLI work and exported it from `compiler/mesh-pkg/src/lib.rs`. The helper now selects the public Unix or Windows installer URL unless `MESH_UPDATE_INSTALLER_URL` is set, downloads the script with `ureq`, rejects empty or non-text responses, and forwards only the existing `MESH_INSTALL_RELEASE_API_URL`, `MESH_INSTALL_RELEASE_BASE_URL`, `MESH_INSTALL_DOWNLOAD_TIMEOUT_SEC`, and `MESH_INSTALL_STRICT_PROOF` overrides unchanged to the installer child. Unix execution pipes the installer text into `/bin/sh -s -- --yes` and waits for completion; Windows execution writes `install.ps1` plus an explicit bootstrap PowerShell script into a temp directory, launches `powershell.exe -NoProfile -ExecutionPolicy Bypass -File <bootstrap>`, and returns after the bootstrap is started so the invoking `.exe` can exit before replacement. Added focused tests in `compiler/mesh-pkg/tests/toolchain_update.rs` to pin default URL selection, override handling, env passthrough, launcher construction, and malformed-download/write/spawn failure rails without re-implementing installer semantics.

## Verification

`cargo test -p mesh-pkg --test toolchain_update -- --nocapture` passed with 15 focused updater tests covering default URL selection, installer override handling, env passthrough, Unix launcher construction, Windows bootstrap/PowerShell construction, and negative failure rails. Slice-level verification was also sampled: `cargo test -p meshpkg --test update_cli -- --nocapture` fails because the target does not exist yet (T02), `cargo test -p meshc --test tooling_e2e test_update -- --nocapture` exits 0 but matches 0 tests so update coverage is still missing until T02, and `cargo test -p meshc --test e2e_m048_s03 -- --nocapture` fails because the acceptance rail target does not exist yet (T03).

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-pkg --test toolchain_update -- --nocapture` | 0 | ✅ pass | 5303ms |
| 2 | `cargo test -p meshpkg --test update_cli -- --nocapture` | 101 | ❌ fail | 720ms |
| 3 | `cargo test -p meshc --test tooling_e2e test_update -- --nocapture` | 0 | ❌ fail | 61652ms |
| 4 | `cargo test -p meshc --test e2e_m048_s03 -- --nocapture` | 101 | ❌ fail | 1029ms |


## Deviations

Recorded decision D321 because the Unix pipe-vs-Windows bootstrap launch split is now part of the slice contract. Otherwise none.

## Known Issues

The remaining slice verification targets are still incomplete by design at T01 scope: `meshpkg` does not yet have `tests/update_cli.rs`, `meshc` does not yet have `tests/e2e_m048_s03.rs`, and the `tooling_e2e` update filter currently matches zero tests until T02 adds the update case.

## Files Created/Modified

- `compiler/mesh-pkg/src/toolchain_update.rs`
- `compiler/mesh-pkg/src/lib.rs`
- `compiler/mesh-pkg/tests/toolchain_update.rs`


## Deviations
Recorded decision D321 because the Unix pipe-vs-Windows bootstrap launch split is now part of the slice contract. Otherwise none.

## Known Issues
The remaining slice verification targets are still incomplete by design at T01 scope: `meshpkg` does not yet have `tests/update_cli.rs`, `meshc` does not yet have `tests/e2e_m048_s03.rs`, and the `tooling_e2e` update filter currently matches zero tests until T02 adds the update case.
