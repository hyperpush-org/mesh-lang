---
id: T01
parent: S02
milestone: M055
provides: []
requires: []
affects: []
key_files: ["mesher/scripts/lib/mesh-toolchain.sh", "mesher/scripts/test.sh", "mesher/scripts/migrate.sh", "mesher/scripts/build.sh", "mesher/scripts/smoke.sh", "scripts/tests/verify-m055-s02-contract.test.mjs", ".gsd/milestones/M055/slices/S02/tasks/T01-SUMMARY.md"]
key_decisions: ["Mesher maintainer scripts now resolve `meshc` in source-first order (`enclosing-source` -> `sibling-workspace` -> `PATH`) and fail closed if a higher-priority source tier exists but its `target/debug/meshc` is missing.", "Product-owned maintainer wrappers stage build output into explicit bundle directories outside `mesher/` instead of writing in-place binaries."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed `node --test scripts/tests/verify-m055-s02-contract.test.mjs`, `bash -n mesher/scripts/lib/mesh-toolchain.sh mesher/scripts/test.sh mesher/scripts/migrate.sh mesher/scripts/build.sh mesher/scripts/smoke.sh`, `bash mesher/scripts/test.sh`, and `bash mesher/scripts/build.sh .tmp/m055-s02/mesher-build`. Also ran the slice-level checks: `cargo test -p meshc --test e2e_m051_s01 -- --nocapture`, `bash scripts/verify-m051-s01.sh`, `bash scripts/verify-production-proof-surface.sh`, and `bash scripts/verify-m055-s01.sh` all passed. The only expected red slice rail is `bash mesher/scripts/verify-maintainer-surface.sh`, which exits 127 because that package-owned verifier belongs to T02."
completed_at: 2026-04-06T19:14:42.485Z
blocker_discovered: false
---

# T01: Added Mesher-owned meshc resolution and package-local test/migrate/build/smoke scripts with outside-package staging.

> Added Mesher-owned meshc resolution and package-local test/migrate/build/smoke scripts with outside-package staging.

## What Happened
---
id: T01
parent: S02
milestone: M055
key_files:
  - mesher/scripts/lib/mesh-toolchain.sh
  - mesher/scripts/test.sh
  - mesher/scripts/migrate.sh
  - mesher/scripts/build.sh
  - mesher/scripts/smoke.sh
  - scripts/tests/verify-m055-s02-contract.test.mjs
  - .gsd/milestones/M055/slices/S02/tasks/T01-SUMMARY.md
key_decisions:
  - Mesher maintainer scripts now resolve `meshc` in source-first order (`enclosing-source` -> `sibling-workspace` -> `PATH`) and fail closed if a higher-priority source tier exists but its `target/debug/meshc` is missing.
  - Product-owned maintainer wrappers stage build output into explicit bundle directories outside `mesher/` instead of writing in-place binaries.
duration: ""
verification_result: mixed
completed_at: 2026-04-06T19:14:42.487Z
blocker_discovered: false
---

# T01: Added Mesher-owned meshc resolution and package-local test/migrate/build/smoke scripts with outside-package staging.

**Added Mesher-owned meshc resolution and package-local test/migrate/build/smoke scripts with outside-package staging.**

## What Happened

Added a shared `mesher/scripts/lib/mesh-toolchain.sh` helper that resolves `meshc` in source-first order (`enclosing-source` -> `sibling-workspace` -> `PATH`), logs the chosen toolchain tier, enforces bounded command execution, and rejects bundle paths inside `mesher/`. Built thin package-local `test.sh`, `migrate.sh`, `build.sh`, and `smoke.sh` wrappers on top of that helper so Mesher now runs as a normal package instead of a repo-root special case. Added `scripts/tests/verify-m055-s02-contract.test.mjs` to statically forbid repo-root `cargo run -q -p meshc -- ... mesher` fallbacks and dynamically prove the resolver tiers, unsupported migrate rejection, and outside-package build staging with temp-workspace stubs. Removed the stale in-place `mesher/mesher` and `mesher/output` binaries so the source tree reflects the new staged-build contract.

## Verification

Passed `node --test scripts/tests/verify-m055-s02-contract.test.mjs`, `bash -n mesher/scripts/lib/mesh-toolchain.sh mesher/scripts/test.sh mesher/scripts/migrate.sh mesher/scripts/build.sh mesher/scripts/smoke.sh`, `bash mesher/scripts/test.sh`, and `bash mesher/scripts/build.sh .tmp/m055-s02/mesher-build`. Also ran the slice-level checks: `cargo test -p meshc --test e2e_m051_s01 -- --nocapture`, `bash scripts/verify-m051-s01.sh`, `bash scripts/verify-production-proof-surface.sh`, and `bash scripts/verify-m055-s01.sh` all passed. The only expected red slice rail is `bash mesher/scripts/verify-maintainer-surface.sh`, which exits 127 because that package-owned verifier belongs to T02.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test scripts/tests/verify-m055-s02-contract.test.mjs` | 0 | ✅ pass | 4783ms |
| 2 | `bash -n mesher/scripts/lib/mesh-toolchain.sh mesher/scripts/test.sh mesher/scripts/migrate.sh mesher/scripts/build.sh mesher/scripts/smoke.sh` | 0 | ✅ pass | 8ms |
| 3 | `bash mesher/scripts/test.sh` | 0 | ✅ pass | 26500ms |
| 4 | `bash mesher/scripts/build.sh .tmp/m055-s02/mesher-build` | 0 | ✅ pass | 8100ms |
| 5 | `cargo test -p meshc --test e2e_m051_s01 -- --nocapture` | 0 | ✅ pass | 41800ms |
| 6 | `bash mesher/scripts/verify-maintainer-surface.sh` | 127 | ❌ fail | 7ms |
| 7 | `bash scripts/verify-m051-s01.sh` | 0 | ✅ pass | 69800ms |
| 8 | `bash scripts/verify-production-proof-surface.sh` | 0 | ✅ pass | 4800ms |
| 9 | `bash scripts/verify-m055-s01.sh` | 0 | ✅ pass | 89800ms |


## Deviations

Removed the stale in-place `mesher/mesher` and `mesher/output` binaries while landing the new staged-build guard so the package tree matches the new contract.

## Known Issues

`bash mesher/scripts/verify-maintainer-surface.sh` still fails with exit 127 because the package-owned verifier has not been added yet. That is expected follow-up work for T02.

## Files Created/Modified

- `mesher/scripts/lib/mesh-toolchain.sh`
- `mesher/scripts/test.sh`
- `mesher/scripts/migrate.sh`
- `mesher/scripts/build.sh`
- `mesher/scripts/smoke.sh`
- `scripts/tests/verify-m055-s02-contract.test.mjs`
- `.gsd/milestones/M055/slices/S02/tasks/T01-SUMMARY.md`


## Deviations
Removed the stale in-place `mesher/mesher` and `mesher/output` binaries while landing the new staged-build guard so the package tree matches the new contract.

## Known Issues
`bash mesher/scripts/verify-maintainer-surface.sh` still fails with exit 127 because the package-owned verifier has not been added yet. That is expected follow-up work for T02.
