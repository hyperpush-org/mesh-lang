---
id: T01
parent: S06
milestone: M047
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-codegen/src/codegen/expr.rs", "compiler/mesh-codegen/src/codegen/mod.rs", "compiler/meshc/tests/e2e_sqlite_built_package.rs", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Box non-pointer payloads before storing them in generic {i8, ptr} Result/Option variant slots so helper-shaped Ok(...) reconstruction stays ABI-safe.", "Use a dedicated manifest-backed built-package SQLite regression with retained .tmp artifacts instead of relying on broader Todo coverage to catch this seam."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the root-cause fix with cargo test -p mesh-codegen test_int_in_result_pointer_boxing -- --nocapture, then ran cargo test -p meshc --test e2e_sqlite_built_package -- --nocapture and confirmed the manifest-backed built package now handles helper-shaped CREATE TABLE, parameterized INSERT, helper-shaped read-back, and placeholder mismatch reporting without crashing. I also ran the existing slice rail cargo test -p meshc --test e2e_m047_s05 -- --nocapture to confirm the Todo scaffold remains green. As expected for an intermediate slice task, the later S06 surfaces remain red because cargo test -p meshc --test e2e_m047_s06 -- --nocapture has no target yet and bash scripts/verify-m047-s06.sh does not exist yet."
completed_at: 2026-04-01T20:47:38.134Z
blocker_discovered: false
---

# T01: Fixed generic Result scalar payload boxing for SQLite helper rewraps and added a dedicated built-package regression rail.

> Fixed generic Result scalar payload boxing for SQLite helper rewraps and added a dedicated built-package regression rail.

## What Happened
---
id: T01
parent: S06
milestone: M047
key_files:
  - compiler/mesh-codegen/src/codegen/expr.rs
  - compiler/mesh-codegen/src/codegen/mod.rs
  - compiler/meshc/tests/e2e_sqlite_built_package.rs
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Box non-pointer payloads before storing them in generic {i8, ptr} Result/Option variant slots so helper-shaped Ok(...) reconstruction stays ABI-safe.
  - Use a dedicated manifest-backed built-package SQLite regression with retained .tmp artifacts instead of relying on broader Todo coverage to catch this seam.
duration: ""
verification_result: mixed
completed_at: 2026-04-01T20:47:38.136Z
blocker_discovered: false
---

# T01: Fixed generic Result scalar payload boxing for SQLite helper rewraps and added a dedicated built-package regression rail.

**Fixed generic Result scalar payload boxing for SQLite helper rewraps and added a dedicated built-package regression rail.**

## What Happened

I reproduced the failing seam with real built packages under .tmp and narrowed it to helper functions that unwrap SQLite Results into locals and then re-wrap them with Ok(...). LLVM IR for the minimal ensure_schema repro showed the root cause: the helper was returning the generic { i8, ptr } Result layout while storing a raw i64 into that pointer slot, so callers dereferenced garbage and crashed. I fixed this in compiler/mesh-codegen/src/codegen/expr.rs::codegen_construct_variant by boxing non-pointer payloads before storing them into pointer-slot generic sum variants. I added a focused IR unit test in compiler/mesh-codegen/src/codegen/mod.rs and a new manifest-backed built-package regression in compiler/meshc/tests/e2e_sqlite_built_package.rs that proves helper-shaped SQLite execute/query success plus placeholder-mismatch failure visibility with retained .tmp artifacts. I also updated .gsd/KNOWLEDGE.md so current-state guidance reflects the fix instead of the old boxed-int workaround.

## Verification

Verified the root-cause fix with cargo test -p mesh-codegen test_int_in_result_pointer_boxing -- --nocapture, then ran cargo test -p meshc --test e2e_sqlite_built_package -- --nocapture and confirmed the manifest-backed built package now handles helper-shaped CREATE TABLE, parameterized INSERT, helper-shaped read-back, and placeholder mismatch reporting without crashing. I also ran the existing slice rail cargo test -p meshc --test e2e_m047_s05 -- --nocapture to confirm the Todo scaffold remains green. As expected for an intermediate slice task, the later S06 surfaces remain red because cargo test -p meshc --test e2e_m047_s06 -- --nocapture has no target yet and bash scripts/verify-m047-s06.sh does not exist yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-codegen test_int_in_result_pointer_boxing -- --nocapture` | 0 | ✅ pass | 27500ms |
| 2 | `cargo test -p meshc --test e2e_sqlite_built_package -- --nocapture` | 0 | ✅ pass | 12300ms |
| 3 | `cargo test -p meshc --test e2e_m047_s05 -- --nocapture` | 0 | ✅ pass | 27300ms |
| 4 | `cargo test -p meshc --test e2e_m047_s06 -- --nocapture` | 101 | ❌ fail | 926ms |
| 5 | `bash scripts/verify-m047-s06.sh` | 127 | ❌ fail | 16ms |


## Deviations

The task plan expected the repair to land in compiler/mesh-codegen/src/mir/lower.rs and/or compiler/mesh-rt/src/db/sqlite.rs, but the reproduced local fault was lower in compiler/mesh-codegen/src/codegen/expr.rs: generic Result construction stored raw scalar payloads into a pointer slot. I fixed that root cause directly and added a codegen unit test alongside the requested built-package regression instead of changing MIR lowering or the SQLite runtime wrapper.

## Known Issues

Later S06 verification surfaces are still not landed: cargo test -p meshc --test e2e_m047_s06 -- --nocapture fails because the target does not exist yet, and bash scripts/verify-m047-s06.sh fails because the script does not exist yet. No plan-invalidating blocker was discovered.

## Files Created/Modified

- `compiler/mesh-codegen/src/codegen/expr.rs`
- `compiler/mesh-codegen/src/codegen/mod.rs`
- `compiler/meshc/tests/e2e_sqlite_built_package.rs`
- `.gsd/KNOWLEDGE.md`


## Deviations
The task plan expected the repair to land in compiler/mesh-codegen/src/mir/lower.rs and/or compiler/mesh-rt/src/db/sqlite.rs, but the reproduced local fault was lower in compiler/mesh-codegen/src/codegen/expr.rs: generic Result construction stored raw scalar payloads into a pointer slot. I fixed that root cause directly and added a codegen unit test alongside the requested built-package regression instead of changing MIR lowering or the SQLite runtime wrapper.

## Known Issues
Later S06 verification surfaces are still not landed: cargo test -p meshc --test e2e_m047_s06 -- --nocapture fails because the target does not exist yet, and bash scripts/verify-m047-s06.sh fails because the script does not exist yet. No plan-invalidating blocker was discovered.
