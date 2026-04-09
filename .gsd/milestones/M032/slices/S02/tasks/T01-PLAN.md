---
estimated_steps: 5
estimated_files: 7
skills_used:
  - debug-like-expert
  - rust-best-practices
  - test
---

# T01: Repair inferred-export lowering and freeze regression coverage

**Slice:** S02 — Cross-module and inferred-export blocker retirement
**Milestone:** M032

## Description

Fix the real compiler bug first, with proofs attached. This task replaces the old `xmod_identity` failure-only story with passing regression coverage for the repaired behavior, then threads the minimal extra type information needed from the `meshc` driver into MIR lowering so unconstrained exported functions stop degrading to a unit-shaped ABI. The fix has to cover both the local single-file path and the imported cross-module path, because research already proved they share one root cause.

## Steps

1. Replace the old `xmod_identity` failure-only proof in `compiler/meshc/tests/e2e.rs` with passing `m032_inferred_*` coverage that exercises local inferred identity and imported inferred identity through real build-and-run assertions, including both `Int` and `String` outputs.
2. Keep the slice scoped by leaving `e2e_cross_module_polymorphic` and `e2e_cross_module_service` as adjacency controls; do not widen this into a broader generics or module-system rewrite.
3. Extend the `meshc` multi-module compile path in `compiler/meshc/src/main.rs` and `compiler/mesh-codegen/src/lib.rs` so lowering receives concrete function-usage evidence for exported functions, not just same-module usage.
4. Update `compiler/mesh-codegen/src/mir/lower.rs` to recover unresolved return types as well as parameter types from that usage evidence, and touch `compiler/mesh-codegen/src/mir/types.rs` only if needed to keep unresolved-type behavior explicit instead of silently papered over.
5. Run the targeted e2e filters until the repaired local/imported identity paths pass and the already-green cross-module controls still hold.

## Must-Haves

- [ ] `compiler/meshc/tests/e2e.rs` contains passing `m032_inferred_*` coverage for local and imported inferred identity behavior
- [ ] Imported function usage reaches MIR lowering through `compiler/meshc/src/main.rs` and `compiler/mesh-codegen/src/lib.rs`
- [ ] `compiler/mesh-codegen/src/mir/lower.rs` recovers unresolved inferred returns as well as parameters without broad generic-monomorphization work

## Verification

- `cargo test -p meshc --test e2e m032_inferred -- --nocapture`
- `cargo test -p meshc --test e2e e2e_cross_module_polymorphic -- --nocapture`
- `cargo test -p meshc --test e2e e2e_cross_module_service -- --nocapture`

## Observability Impact

- Signals added/changed: named `m032_inferred_*` test failures now distinguish local inferred-return regressions from imported inferred-export regressions
- How a future agent inspects this: `cargo test -p meshc --test e2e m032_inferred -- --nocapture` plus the existing `e2e_cross_module_polymorphic` and `e2e_cross_module_service` controls
- Failure state exposed: exact failing fixture/test name and the wrong stdout or LLVM verifier symptom for the repaired path

## Inputs

- `compiler/meshc/tests/e2e.rs` — existing CLI e2e harness, `xmod_identity` proof, and cross-module controls
- `compiler/meshc/src/main.rs` — multi-module typecheck and lowering loop
- `compiler/mesh-codegen/src/lib.rs` — lowering entrypoints between the CLI driver and MIR
- `compiler/mesh-codegen/src/mir/lower.rs` — current same-module usage recovery and function lowering
- `compiler/mesh-codegen/src/mir/types.rs` — current `Ty::Var` fallback behavior during MIR type resolution
- `.tmp/m032-s01/xmod_identity/main.mpl` — existing cross-module inferred-identity caller fixture to keep as the canonical repaired program
- `.tmp/m032-s01/xmod_identity/utils.mpl` — existing cross-module inferred-identity callee fixture to keep as the canonical repaired program

## Expected Output

- `compiler/meshc/tests/e2e.rs` — passing `m032_inferred_*` regression coverage for local and imported inferred identity
- `compiler/meshc/src/main.rs` — multi-module lowering path that passes imported usage evidence downstream
- `compiler/mesh-codegen/src/lib.rs` — lowered-module API updated to accept the extra usage evidence
- `compiler/mesh-codegen/src/mir/lower.rs` — inferred parameter/return recovery that works for local and imported definitions
