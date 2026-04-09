---
estimated_steps: 4
estimated_files: 3
skills_used:
  - rust-testing
  - powershell-windows
  - debug-like-expert
---

# T03: Add the retained S03 acceptance rail for staged `meshc update` and installed `meshpkg update`

**Slice:** S03 — Toolchain self-update commands
**Milestone:** M048

## Description

The slice is not done when help text exists; it is done when the real update commands can refresh a staged or installed Mesh toolchain through the same installer path users already trust. This task adds the proof rail that exercises both commands against a staged local release server and fake Mesh home.

The rail should stay diagnosable like S01: retain the staged server tree, fake-home snapshots, command lines, stdout/stderr, exit statuses, version-file state, and credential-presence checks under `.tmp/m048-s03/...` so the first broken seam is inspectable without rerunning under manual instrumentation.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Local staged release server and hosted installer bytes | Fail the test with the staged URL and retained server tree instead of silently falling back to production URLs. | Surface the server/startup phase and stop before invoking update commands against a missing host. | Treat broken release metadata, missing archives, or wrong hosted installer content as contract failures. |
| Staged or installed CLI binaries under fake home state | Preserve command lines, stdout/stderr, and exit status so update failures are attributable to `meshc`, `meshpkg`, or installer bootstrap behavior. | Poll or wait with explicit phase names instead of hanging on bootstrap/self-overwrite paths. | Reject malformed fake-home state before assertions so false positives do not slip through. |
| Shared version/credentials contract under `~/.mesh` | Fail if the version file is not refreshed, the sibling binary stays broken, or credential presence disappears after update. | Surface the exact post-update check that timed out (version, sibling repair, credentials). | Never read or print credential contents; only assert preserved file presence and expected non-destructive behavior. |

## Load Profile

- **Shared resources**: local HTTP port, temp staged-release tree, fake home directory, copied binaries, and retained artifact directories.
- **Per-operation cost**: one staged metadata/installer host, two end-to-end update scenarios, and post-update binary/credential checks.
- **10x breakpoint**: port collisions, slow process startup, and artifact churn fail before CPU does, so the rail must keep bounded waits and per-phase logs.

## Negative Tests

- **Malformed inputs**: broken release metadata, missing sibling archive, stale version file, and corrupted installed sibling binary.
- **Error paths**: updater cannot reach the staged installer, installer exits non-zero, or the post-update sibling binary still fails to run.
- **Boundary conditions**: staged binary outside `~/.mesh/bin`, installed binary inside fake Mesh home, and host-specific bootstrap behavior that must still end in a repaired toolchain pair.

## Steps

1. Add `compiler/meshc/tests/e2e_m048_s03.rs` and any minimal support extensions in `compiler/meshc/tests/support/m046_route_free.rs` needed to find/build `meshpkg`, create retained artifact roots, and stage local release assets.
2. Build a local staged release server that serves hosted installer copies plus meshc/meshpkg archives, metadata JSON, and `SHA256SUMS`, all wired through `MESH_UPDATE_INSTALLER_URL` and the existing `MESH_INSTALL_*` overrides.
3. Add one staged `meshc update` scenario that starts from a copied binary outside `~/.mesh/bin` and proves both binaries land in the fake home with a refreshed shared version file.
4. Add one installed `meshpkg update` scenario that seeds fake-home credentials, corrupts the sibling binary and/or version file, runs update, then proves both binaries are healthy again and credentials were preserved without printing their contents.

## Must-Haves

- [ ] `compiler/meshc/tests/e2e_m048_s03.rs` proves staged `meshc update` and installed `meshpkg update` against a local staged release server.
- [ ] The rail asserts whole-toolchain repair: both binaries exist/run and the shared `~/.mesh/version` file is refreshed.
- [ ] The installed `meshpkg update` case proves `~/.mesh/credentials` survives self-update.
- [ ] Per-scenario artifacts under `.tmp/m048-s03/...` make the first failing phase inspectable without rerunning manually.

## Verification

- `cargo test -p meshc --test e2e_m048_s03 -- --nocapture`
- The retained rail leaves `.tmp/m048-s03/...` artifacts for both scenarios, including staged server files, command logs, version snapshots, and credential-presence checks.

## Observability Impact

- Signals added/changed: retained per-scenario project/home snapshots, installer stdout/stderr, version snapshots, credential-presence markers, and staged server layout.
- How a future agent inspects this: rerun `cargo test -p meshc --test e2e_m048_s03 -- --nocapture` and inspect `.tmp/m048-s03/<scenario>/`.
- Failure state exposed: the first failing phase is labeled (server, download, install, version, sibling repair, credentials) with command/output artifacts.

## Inputs

- `compiler/meshc/tests/e2e_m048_s01.rs` — retained-artifact acceptance-rail pattern worth reusing for per-scenario diagnostics.
- `compiler/meshc/tests/support/m046_route_free.rs` — repo-root, meshc-binary, artifact, and helper utilities the new rail can extend.
- `scripts/fixtures/m034-s03-installer-smoke` — existing staged-installer smoke project used to prove installed binaries still work after update.
- `scripts/verify-m034-s03.sh` — Unix staged-installer proof contract and release-layout reference.
- `scripts/verify-m034-s03.ps1` — Windows staged-installer proof contract and launcher/layout reference.
- `compiler/mesh-pkg/src/toolchain_update.rs` — updater implementation whose real behavior this rail must exercise.
- `compiler/meshc/src/main.rs` — `meshc update` command surface under test.
- `compiler/meshpkg/src/main.rs` — `meshpkg update` command surface under test.

## Expected Output

- `compiler/meshc/tests/e2e_m048_s03.rs` — dedicated retained acceptance rail for staged `meshc update` and installed `meshpkg update`.
- `compiler/meshc/tests/support/m046_route_free.rs` — minimal support helpers for artifact retention and locating/building staged binaries.
- `compiler/mesh-pkg/src/toolchain_update.rs` — any acceptance-driven diagnostics or host-specific wait/bootstrap refinements needed to make the rail truthful and debuggable.
