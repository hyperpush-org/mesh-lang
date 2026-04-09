---
id: T03
parent: S04
milestone: M044
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-typeck/src/error.rs", "compiler/mesh-typeck/src/infer.rs", "compiler/mesh-typeck/src/builtins.rs", "compiler/mesh-codegen/src/mir/lower.rs", "compiler/mesh-codegen/src/codegen/intrinsics.rs", "compiler/mesh-rt/src/dist/continuity.rs", "compiler/mesh-rt/src/lib.rs", "compiler/mesh-lsp/src/analysis.rs", "compiler/meshc/tests/e2e_m044_s01.rs", "compiler/meshc/tests/e2e_m043_s02.rs", "cluster-proof/main.mpl", "cluster-proof/work_continuity.mpl", ".gsd/KNOWLEDGE.md", ".gsd/DECISIONS.md"]
key_decisions: ["D204 — reject `Continuity.promote()` in type checking with an explicit automatic-only diagnostic while keeping the Rust promotion helper internal-only."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the task-level contract with `cargo test -p meshc --test e2e_m044_s01 m044_s01_typed_continuity_ -- --nocapture` and `cargo test -p meshc --test e2e_m043_s02 continuity_api_ -- --nocapture`, both of which passed with the new read-only authority and disabled-manual-promotion expectations. Also smoke-verified that `cluster-proof` still builds and its package tests still pass after removing the dead `/promote` route via `cargo run -q -p meshc -- build cluster-proof && cargo run -q -p meshc -- test cluster-proof/tests`. During execution I also reran the full `e2e_m044_s01` and `e2e_m043_s02` targets; both were green, with the stale manual failover rail in `e2e_m043_s02` now explicitly ignored."
completed_at: 2026-03-30T03:39:48.753Z
blocker_discovered: false
---

# T03: Removed the public `Continuity.promote()` Mesh surface and replaced it with an explicit auto-only compiler diagnostic.

> Removed the public `Continuity.promote()` Mesh surface and replaced it with an explicit auto-only compiler diagnostic.

## What Happened
---
id: T03
parent: S04
milestone: M044
key_files:
  - compiler/mesh-typeck/src/error.rs
  - compiler/mesh-typeck/src/infer.rs
  - compiler/mesh-typeck/src/builtins.rs
  - compiler/mesh-codegen/src/mir/lower.rs
  - compiler/mesh-codegen/src/codegen/intrinsics.rs
  - compiler/mesh-rt/src/dist/continuity.rs
  - compiler/mesh-rt/src/lib.rs
  - compiler/mesh-lsp/src/analysis.rs
  - compiler/meshc/tests/e2e_m044_s01.rs
  - compiler/meshc/tests/e2e_m043_s02.rs
  - cluster-proof/main.mpl
  - cluster-proof/work_continuity.mpl
  - .gsd/KNOWLEDGE.md
  - .gsd/DECISIONS.md
key_decisions:
  - D204 — reject `Continuity.promote()` in type checking with an explicit automatic-only diagnostic while keeping the Rust promotion helper internal-only.
duration: ""
verification_result: passed
completed_at: 2026-03-30T03:39:48.755Z
blocker_discovered: false
---

# T03: Removed the public `Continuity.promote()` Mesh surface and replaced it with an explicit auto-only compiler diagnostic.

**Removed the public `Continuity.promote()` Mesh surface and replaced it with an explicit auto-only compiler diagnostic.**

## What Happened

Removed the Mesh-visible `Continuity.promote()` surface from builtin registration, stdlib typing, MIR lowering, LLVM intrinsic declarations, and the runtime export list, while keeping the internal Rust `promote_authority()` helper for bounded automatic promotion. Added an explicit typechecker diagnostic for `Continuity.promote()` so stale Mesh code now fails with an automatic-only failover message instead of a generic missing-member or dead-symbol error, and wired the new error through `compiler/mesh-lsp/src/analysis.rs` so editor diagnostics stay aligned. Retargeted `e2e_m044_s01` and `e2e_m043_s02` to prove the read-only `Continuity.authority_status()` surface still reports runtime truth and that any manual promotion attempt fails closed. As a local adaptation to keep the repo truthful after the compiler change, removed the dead `/promote` route and `Continuity.promote()` wrapper from `cluster-proof` so the proof app still builds and its package tests still pass.

## Verification

Verified the task-level contract with `cargo test -p meshc --test e2e_m044_s01 m044_s01_typed_continuity_ -- --nocapture` and `cargo test -p meshc --test e2e_m043_s02 continuity_api_ -- --nocapture`, both of which passed with the new read-only authority and disabled-manual-promotion expectations. Also smoke-verified that `cluster-proof` still builds and its package tests still pass after removing the dead `/promote` route via `cargo run -q -p meshc -- build cluster-proof && cargo run -q -p meshc -- test cluster-proof/tests`. During execution I also reran the full `e2e_m044_s01` and `e2e_m043_s02` targets; both were green, with the stale manual failover rail in `e2e_m043_s02` now explicitly ignored.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m044_s01 m044_s01_typed_continuity_ -- --nocapture` | 0 | ✅ pass | 14690ms |
| 2 | `cargo test -p meshc --test e2e_m043_s02 continuity_api_ -- --nocapture` | 0 | ✅ pass | 16120ms |
| 3 | `cargo run -q -p meshc -- build cluster-proof && cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 34610ms |


## Deviations

Pulled a small proof-app cleanup slice forward from the later cluster-proof work: removed the dead `/promote` route and stale manifest expectation now so the repo stays buildable and truthful after the compiler surface removal.

## Known Issues

`compiler/meshc/tests/e2e_m043_s02.rs::e2e_m043_s02_failover_promotion_and_fenced_rejoin_keep_newer_truth` is now explicitly ignored because it proves the removed manual failover contract and still needs the planned S04 auto-only replacement rail. The earlier S04 auto-resume proof surface from T02 is still missing: `compiler/meshc/tests/e2e_m044_s04.rs` does not exist yet, so the older gate command `cargo test -p meshc --test e2e_m044_s04 ...` remains unresolved outside this task.

## Files Created/Modified

- `compiler/mesh-typeck/src/error.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-typeck/src/builtins.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/lib.rs`
- `compiler/mesh-lsp/src/analysis.rs`
- `compiler/meshc/tests/e2e_m044_s01.rs`
- `compiler/meshc/tests/e2e_m043_s02.rs`
- `cluster-proof/main.mpl`
- `cluster-proof/work_continuity.mpl`
- `.gsd/KNOWLEDGE.md`
- `.gsd/DECISIONS.md`


## Deviations
Pulled a small proof-app cleanup slice forward from the later cluster-proof work: removed the dead `/promote` route and stale manifest expectation now so the repo stays buildable and truthful after the compiler surface removal.

## Known Issues
`compiler/meshc/tests/e2e_m043_s02.rs::e2e_m043_s02_failover_promotion_and_fenced_rejoin_keep_newer_truth` is now explicitly ignored because it proves the removed manual failover contract and still needs the planned S04 auto-only replacement rail. The earlier S04 auto-resume proof surface from T02 is still missing: `compiler/meshc/tests/e2e_m044_s04.rs` does not exist yet, so the older gate command `cargo test -p meshc --test e2e_m044_s04 ...` remains unresolved outside this task.
