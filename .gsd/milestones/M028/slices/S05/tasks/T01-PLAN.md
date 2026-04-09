---
estimated_steps: 4
estimated_files: 6
skills_used:
  - debug-like-expert
  - test
  - review
---

# T01: Repair and prove Mesh source-level supervisor child lifecycle

**Slice:** S05 — Supervision, Recovery, and Failure Visibility
**Milestone:** M028

## Description

Make the Mesh-language supervisor path trustworthy before `reference-backend/` depends on it. The current runtime donor implementation is stronger than the source-level e2e coverage, and research already found evidence that compiled supervisors may not even start children correctly. This task should align the compiler/runtime child-spec contract and replace banner-only proof with assertions that compiled Mesh supervisors actually start children, restart crashes, and surface restart-limit exhaustion.

## Steps

1. Trace the current supervisor child-spec path from `compiler/mesh-codegen/src/mir/lower.rs` through `compiler/mesh-codegen/src/codegen/expr.rs` into `compiler/mesh-rt/src/actor/mod.rs`, then fix the serialization/parsing mismatch so compiled supervisors hand the runtime a coherent child config.
2. Strengthen `compiler/meshc/tests/e2e_supervisors.rs` so it stops treating a printed banner as success and instead asserts visible child boot/restart/restart-limit behavior from compiled Mesh programs.
3. Update the `tests/e2e/` supervisor fixtures so they emit child-start, crash, and restart-limit signals that the Rust harness can assert deterministically.
4. Keep `cargo test -p mesh-rt supervisor::tests:: --lib` green so S05 still relies on the existing runtime supervisor donor behavior rather than drifting into a second implementation.

## Must-Haves

- [ ] The compiled Mesh supervisor path uses a child-spec encoding that matches what `parse_supervisor_config(...)` expects at runtime.
- [ ] `compiler/meshc/tests/e2e_supervisors.rs` asserts real child lifecycle behavior, not only supervisor startup banners.
- [ ] The source-level e2e suite proves at least child boot, child restart after crash, and restart-limit visibility.
- [ ] Runtime supervisor donor tests stay green after the compiler/runtime bridge fix.

## Verification

- `cargo test -p mesh-rt supervisor::tests:: --lib -- --nocapture`
- `cargo test -p meshc --test e2e_supervisors -- --nocapture`

## Observability Impact

- Signals added/changed: supervisor fixtures should print child boot/crash/restart markers that the Rust e2e harness asserts explicitly.
- How a future agent inspects this: run `cargo test -p meshc --test e2e_supervisors -- --nocapture` and inspect the named lifecycle assertions instead of inferring behavior from one startup line.
- Failure state exposed: compiler/runtime supervisor regressions become visible as missing child-start/restart signals or restart-limit assertion failures.

## Inputs

- `compiler/mesh-codegen/src/mir/lower.rs` — current supervisor lowering logic with the simplified child-start extraction
- `compiler/mesh-codegen/src/codegen/expr.rs` — current supervisor config serializer used by compiled Mesh programs
- `compiler/mesh-rt/src/actor/mod.rs` — runtime parser and supervisor-start entrypoint the compiler must match
- `compiler/meshc/tests/e2e_supervisors.rs` — shallow source-level proof surface that needs stronger assertions
- `tests/e2e/supervisor_basic.mpl` — existing child-start fixture to harden into a real lifecycle proof
- `tests/e2e/supervisor_restart_limit.mpl` — existing restart-limit fixture to harden into a real restart proof

## Expected Output

- `compiler/mesh-codegen/src/mir/lower.rs` — corrected supervisor lowering assumptions for child start metadata
- `compiler/mesh-codegen/src/codegen/expr.rs` — supervisor child-spec encoding aligned with runtime parsing
- `compiler/mesh-rt/src/actor/mod.rs` — runtime parser/entrypoint adjustments needed to consume the compiled config honestly
- `compiler/meshc/tests/e2e_supervisors.rs` — source-level e2e assertions for child start/restart/restart-limit behavior
- `tests/e2e/supervisor_basic.mpl` — fixture that proves a compiled supervisor actually starts a child
- `tests/e2e/supervisor_restart_limit.mpl` — fixture that proves restart-limit behavior from compiled Mesh source
