---
id: T01
parent: S02
milestone: M032
provides:
  - Named `m032_inferred_*` regressions that reproduce the local-runtime and imported-LLVM inferred-identity failures.
key_files:
  - compiler/meshc/tests/e2e.rs
  - .gsd/milestones/M032/slices/S02/S02-PLAN.md
key_decisions:
  - Reverted the half-wired `mesh-codegen` signature edits when the context cutoff hit, so the tree stays coherent for the next unit.
patterns_established:
  - Use `cargo test -p meshc --test e2e m032_inferred -- --nocapture` as the primary diagnostic surface for this lowering bug.
observability_surfaces:
  - cargo test -p meshc --test e2e m032_inferred -- --nocapture
duration: partial handoff at context cutoff
verification_result: failed
completed_at: 2026-03-24
blocker_discovered: false
---

# T01: Repair inferred-export lowering and freeze regression coverage

**Added `m032_inferred_*` regressions and stopped at a clean handoff before the lowering fix was implemented.**

## What Happened

I replaced the old `e2e_m032_limit_xmod_identity` failure-only proof in `compiler/meshc/tests/e2e.rs` with two named regressions:

- `m032_inferred_local_identity`
- `m032_inferred_cross_module_identity`

I also fixed the pre-flight observability gap in `.gsd/milestones/M032/slices/S02/S02-PLAN.md` by adding an exact named verification step for `m032_inferred_cross_module_identity`.

Those new tests immediately reproduced the two real failure modes the slice is meant to fix:

- **local inferred identity** still compiles and then returns garbage / crashes at runtime
- **imported inferred identity** still dies at LLVM verification with the call-signature mismatch

I started threading importer-side usage evidence into the lowering seam, but the context-budget warning arrived mid-edit. Rather than leave `mesh-codegen` half-wired, I reverted the unfinished signature changes and stopped with only the finished regression surface preserved.

## Verification

What passed:

- existing adjacency controls still pass: `e2e_cross_module_polymorphic` and `e2e_cross_module_service`
- the historical failure proof for `e2e_m032_limit_xmod_identity` still passed before I replaced it

What failed:

- the new `m032_inferred_*` regression filter fails exactly where expected on the current compiler

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e e2e_m032_limit_xmod_identity -- --nocapture` | 0 | ✅ pass | 2.91s |
| 2 | `cargo test -p meshc --test e2e e2e_cross_module_polymorphic -- --nocapture` | 0 | ✅ pass | 7.23s |
| 3 | `cargo test -p meshc --test e2e e2e_cross_module_service -- --nocapture` | 0 | ✅ pass | 3.58s |
| 4 | `cargo test -p meshc --test e2e m032_inferred -- --nocapture` | 101 | ❌ fail | 5.90s |

## Diagnostics

Primary inspection commands:

- `cargo test -p meshc --test e2e m032_inferred -- --nocapture`
- `cargo run -q -p meshc -- build .tmp/m032-s01/xmod_identity`

The current failure split is already useful:

- `m032_inferred_local_identity` shows the **runtime-corruption** side of the bug
- `m032_inferred_cross_module_identity` shows the **LLVM verifier** side of the bug

Resume target for the next unit:

1. `compiler/meshc/src/main.rs` — aggregate importer-side function-usage evidence per exporting module before lowering
2. `compiler/mesh-codegen/src/lib.rs` — thread that evidence into `lower_to_mir_raw(...)`
3. `compiler/mesh-codegen/src/mir/lower.rs` — recover unresolved return types alongside parameter types, and handle mixed concrete usage without “first observed type wins” ABI drift

## Deviations

- Stopped early because of the context-budget cutoff.
- Intentionally reverted the incomplete `mesh-codegen` wiring instead of leaving the compiler in a half-updated state.
- Left T01 open in `S02-PLAN.md` because the required verification still fails.

## Known Issues

- `m032_inferred_local_identity` currently fails at runtime after printing the wrong value.
- `m032_inferred_cross_module_identity` currently fails during LLVM module verification with `Call parameter type does not match function signature!`.
- No lowering fix shipped in this unit; only the regression surface and plan observability fix are durable.

## Files Created/Modified

- `compiler/meshc/tests/e2e.rs` — replaced the old `xmod_identity` failure-only proof with named `m032_inferred_*` regressions.
- `.gsd/milestones/M032/slices/S02/S02-PLAN.md` — added an exact named diagnostic verification step for the imported inferred-identity surface.
- `.gsd/milestones/M032/slices/S02/tasks/T01-SUMMARY.md` — recorded the partial handoff, failing verification evidence, and resume notes.
