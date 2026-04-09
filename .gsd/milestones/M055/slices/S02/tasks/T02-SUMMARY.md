---
id: T02
parent: S02
milestone: M055
provides: []
requires: []
affects: []
key_files: ["mesher/scripts/verify-maintainer-surface.sh", "scripts/verify-m051-s01.sh", "compiler/meshc/tests/support/m051_mesher.rs", "compiler/meshc/tests/e2e_m051_s01.rs", "mesher/scripts/smoke.sh", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Use `mesher/scripts/verify-maintainer-surface.sh` as the only authoritative Mesher proof replay and keep `scripts/verify-m051-s01.sh` as a compatibility wrapper that only delegates and validates the delegated markers.", "Have the product-owned verifier assign a unique `MESH_CLUSTER_PORT`/`MESH_NODE_NAME` pair for smoke replays so local Mesh processes cannot make the maintainer rail fail for the wrong reason."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed the required task/slice verification commands: `cargo test -p meshc --test e2e_m051_s01 -- --nocapture`, `bash mesher/scripts/verify-maintainer-surface.sh`, and `bash scripts/verify-m051-s01.sh`. These confirm the Rust helper/e2e contract, the product-owned verifier replay, and the compatibility wrapper’s delegated-marker checks all succeed."
completed_at: 2026-04-06T19:41:24.454Z
blocker_discovered: false
---

# T02: Moved the deeper Mesher proof rail into a package-owned verifier and reduced the repo-root M051 rail to a delegation-checked compatibility wrapper.

> Moved the deeper Mesher proof rail into a package-owned verifier and reduced the repo-root M051 rail to a delegation-checked compatibility wrapper.

## What Happened
---
id: T02
parent: S02
milestone: M055
key_files:
  - mesher/scripts/verify-maintainer-surface.sh
  - scripts/verify-m051-s01.sh
  - compiler/meshc/tests/support/m051_mesher.rs
  - compiler/meshc/tests/e2e_m051_s01.rs
  - mesher/scripts/smoke.sh
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Use `mesher/scripts/verify-maintainer-surface.sh` as the only authoritative Mesher proof replay and keep `scripts/verify-m051-s01.sh` as a compatibility wrapper that only delegates and validates the delegated markers.
  - Have the product-owned verifier assign a unique `MESH_CLUSTER_PORT`/`MESH_NODE_NAME` pair for smoke replays so local Mesh processes cannot make the maintainer rail fail for the wrong reason.
duration: ""
verification_result: passed
completed_at: 2026-04-06T19:41:24.455Z
blocker_discovered: false
---

# T02: Moved the deeper Mesher proof rail into a package-owned verifier and reduced the repo-root M051 rail to a delegation-checked compatibility wrapper.

**Moved the deeper Mesher proof rail into a package-owned verifier and reduced the repo-root M051 rail to a delegation-checked compatibility wrapper.**

## What Happened

Added `mesher/scripts/verify-maintainer-surface.sh` as the authoritative Mesher maintainer replay. It now owns phase markers, temporary Postgres bootstrap, package-local test/build/migrate/smoke phases, and the retained proof bundle under `.tmp/m051-s01/verify/`. Refactored `compiler/meshc/tests/support/m051_mesher.rs` so the runtime rail builds and migrates Mesher through the package-owned scripts instead of repo-root `meshc build mesher` / `meshc migrate mesher` assumptions, and updated its build metadata to package-root semantics. Replaced the old README-focused source assertions in `compiler/meshc/tests/e2e_m051_s01.rs` with package-verifier/wrapper/helper contract assertions. Rewrote `scripts/verify-m051-s01.sh` into a thin compatibility wrapper that delegates to the package-owned verifier and fail-closes if the delegated markers or proof-bundle pointer are missing. While exercising the new verifier, I fixed `mesher/scripts/smoke.sh` so its JSON helper can actually parse piped `curl` output, it retains the last unreadable settings response, and verifier-driven smoke runs on a unique cluster port instead of the default `4370`.

## Verification

Passed the required task/slice verification commands: `cargo test -p meshc --test e2e_m051_s01 -- --nocapture`, `bash mesher/scripts/verify-maintainer-surface.sh`, and `bash scripts/verify-m051-s01.sh`. These confirm the Rust helper/e2e contract, the product-owned verifier replay, and the compatibility wrapper’s delegated-marker checks all succeed.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m051_s01 -- --nocapture` | 0 | ✅ pass | 44528ms |
| 2 | `bash mesher/scripts/verify-maintainer-surface.sh` | 0 | ✅ pass | 75194ms |
| 3 | `bash scripts/verify-m051-s01.sh` | 0 | ✅ pass | 82334ms |


## Deviations

Updated `mesher/scripts/smoke.sh` as a local execution adaptation after the new package-owned verifier exposed a real JSON-parser/stdin bug and cluster-port collision risk in the smoke phase. The plan’s primary four files were still the main contract surface.

## Known Issues

None.

## Files Created/Modified

- `mesher/scripts/verify-maintainer-surface.sh`
- `scripts/verify-m051-s01.sh`
- `compiler/meshc/tests/support/m051_mesher.rs`
- `compiler/meshc/tests/e2e_m051_s01.rs`
- `mesher/scripts/smoke.sh`
- `.gsd/KNOWLEDGE.md`


## Deviations
Updated `mesher/scripts/smoke.sh` as a local execution adaptation after the new package-owned verifier exposed a real JSON-parser/stdin bug and cluster-port collision risk in the smoke phase. The plan’s primary four files were still the main contract surface.

## Known Issues
None.
