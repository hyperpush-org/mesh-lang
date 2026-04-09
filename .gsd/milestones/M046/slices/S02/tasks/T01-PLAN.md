---
estimated_steps: 4
estimated_files: 6
skills_used:
  - rust-best-practices
---

# T01: Thread startup-work registrations through build planning and main-wrapper codegen

**Slice:** S02 — Runtime-owned startup trigger and route-free status contract
**Milestone:** M046

## Description

Carry S01's clustered execution plan into a dedicated startup-work registration surface so codegen can emit runtime-owned startup hooks for `kind == Work` declarations and keep declared service handlers off the startup path.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/meshc/src/main.rs` clustered plan threading | Fail the build with an explicit missing-startup-metadata error instead of silently dropping startup work. | N/A | Never synthesize startup registrations from service call/cast entries. |
| `compiler/mesh-codegen/src/codegen/mod.rs` main-wrapper emission | Fail the focused codegen rail on missing or misordered runtime hook calls. | N/A | Keep emitted IR free of startup registration calls when no work declarations exist. |
| `compiler/mesh-codegen/src/codegen/intrinsics.rs` runtime declarations | Fail fast on IR or link drift rather than emitting undeclared symbol names. | N/A | Reject signature mismatch between codegen declarations and runtime exports. |

## Load Profile

- **Shared resources**: Merged MIR function table plus the generated startup-registration vector.
- **Per-operation cost**: One linear pass over declared handlers plus one emitted registration call per startup work item.
- **10x breakpoint**: Duplicate or missing startup registrations in generated IR will fail long before compile-time throughput becomes interesting.

## Negative Tests

- **Malformed inputs**: Clustered service call/cast declarations and binaries with no clustered work.
- **Error paths**: Missing runtime hook declarations or missing lowered wrapper symbols fail the compiler rail instead of compiling a partial startup path.
- **Boundary conditions**: Source-declared and manifest-declared work both emit the same startup registration name, while service call/cast handlers remain declared-handler-only.

## Steps

1. Extend build/codegen planning with an explicit startup-work registration list derived only from `ClusteredDeclarationKind::Work`.
2. Thread that list through `compile_mir_to_binary(...)`, `compile_mir_to_llvm_ir(...)`, and `CodeGen`, leaving existing declared-handler registration behavior untouched.
3. Emit runtime startup registration and post-`mesh_main` trigger calls in `generate_main_wrapper(...)`, ordered after declared-handler registration and before scheduler handoff.
4. Add focused compiler/codegen proof rails that assert emitted LLVM contains the startup hook for work handlers and omits it for service call/cast handlers.

## Must-Haves

- [ ] Only clustered work declarations reach the startup-work runtime hook.
- [ ] Source and manifest declarations converge on the same startup registration identity.
- [ ] Declared service call/cast handlers remain available for their existing runtime path but never auto-trigger at startup.
- [ ] Emitted LLVM/main-wrapper ordering proves registration happens before the startup trigger runs.

## Verification

- `cargo test -p meshc --test e2e_m046_s02 m046_s02_codegen_ -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s02 m044_s02_metadata_ -- --nocapture`

## Observability Impact

- Signals added/changed: Emitted startup-registration symbols and trigger-call ordering in the generated LLVM/main wrapper.
- How a future agent inspects this: Rerun `cargo test -p meshc --test e2e_m046_s02 m046_s02_codegen_ -- --nocapture` or inspect `meshc build --emit-llvm` output for the startup hook.
- Failure state exposed: Missing, duplicate, or misordered startup registrations fail the compiler rail before runtime debugging begins.

## Inputs

- `compiler/meshc/src/main.rs` — current clustered build planning drops startup intent after declared-handler preparation.
- `compiler/mesh-codegen/src/declared.rs` — existing declared-handler plan/registration structs that need a parallel startup-work shape.
- `compiler/mesh-codegen/src/lib.rs` — compile entrypoints that currently only pass declared-handler registrations into codegen.
- `compiler/mesh-codegen/src/codegen/mod.rs` — `generate_main_wrapper(...)` currently registers handlers and runs `mesh_main` without a startup-work hook.
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — runtime declaration table that must match any new startup-work exports.
- `compiler/meshc/tests/e2e_m044_s02.rs` — current declared-handler regression rail that must stay green.

## Expected Output

- `compiler/meshc/src/main.rs` — build planning preserves a startup-work registration list alongside the declared-handler plan.
- `compiler/mesh-codegen/src/declared.rs` — startup-work registration metadata exists in a form codegen can consume.
- `compiler/mesh-codegen/src/lib.rs` — binary and LLVM compile entrypoints pass startup-work registrations through.
- `compiler/mesh-codegen/src/codegen/mod.rs` — main-wrapper codegen emits runtime startup registration and trigger calls in the right order.
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — startup-work runtime exports are declared with the right signatures.
- `compiler/meshc/tests/e2e_m046_s02.rs` — focused codegen/LLVM assertions cover work-only startup registration behavior.
