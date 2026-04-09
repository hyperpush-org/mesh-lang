---
estimated_steps: 4
estimated_files: 7
skills_used:
  - rust-best-practices
  - llvm
  - powershell-windows
  - debug-like-expert
---

# T02: Repair the installed Windows compiler path and prove the staged hello build

**Slice:** S12 — Windows release-smoke remediation and final green closeout
**Milestone:** M034

## Description

Repair the installed Windows compiler/runtime handshake and keep the staged verifier honest so the hello fixture either builds successfully through the documented path or fails with a deterministic actionable error. This task owns the actual fix. It should start from T01's classified boundary, keep Unix/macOS behavior intact, and only touch workflow contract text if the truthful verifier shape changes.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/mesh-codegen` runtime/linker discovery | Return an explicit target-aware error naming the expected runtime path or linker/toolchain prerequisite. | Fail the build immediately; do not keep waiting on a path that will never resolve. | Reject unsupported or contradictory Windows target/runtime inputs instead of falling back to Unix assumptions. |
| `scripts/verify-m034-s03.ps1` staged install-build proof | Keep the verifier red if the repaired path is not actually exercised through installed `meshc.exe build`. | Preserve phase logs and summary artifacts rather than retrying silently. | Treat missing enriched diagnostics or missing hello binary output as proof drift. |
| Release workflow contract | Re-run the contract checker if the truthful verifier shape changes. | Fail the task on contract drift; do not defer it to hosted reruns. | Treat workflow/verifier mismatch as a blocker even if local compiler tests pass. |

## Load Profile

- **Shared resources**: `compiler/mesh-codegen` linker/runtime discovery, `compiler/meshc` build entrypoint, staged Windows verifier tree, and hosted Windows runner assumptions.
- **Per-operation cost**: one installed-build smoke attempt plus targeted Rust/PowerShell regressions.
- **10x breakpoint**: a wrong runtime-path handshake or target-specific linker assumption fans out to every hosted Windows release smoke run immediately, so the task must keep the regression surface narrow and target-aware.

## Negative Tests

- **Malformed inputs**: missing `mesh_rt.lib`, missing or bad `LLVM_SYS_211_PREFIX`, bad target triple assumptions, and staged verifier paths pointing outside the release asset tree.
- **Error paths**: installed build reaches a deterministic runtime/toolchain error, workflow contract checker fails after verifier changes, or the hello fixture still crashes instead of surfacing a Rust error.
- **Boundary conditions**: debug vs release runtime preference, installed compiler outside the repo tree, and unchanged Unix/macOS behavior after the Windows repair.

## Steps

1. Start from `.tmp/m034-s12/t01/diagnostic-summary.json` and confirm whether the repair belongs in runtime discovery, CLI plumbing, verifier env export, or earlier codegen/object-emission logic.
2. Fix the Windows MSVC compiler/verifier handshake in `compiler/mesh-codegen/src/link.rs`, `compiler/mesh-codegen/src/lib.rs`, `compiler/meshc/src/main.rs`, and `scripts/verify-m034-s03.ps1` as needed, while preserving actionable normal errors for missing prerequisites.
3. Extend `compiler/meshc/tests/e2e_m034_s12.rs`, `cargo test -p mesh-codegen link`, and the staged PowerShell regression so the installed hello-build path is proven rather than inferred.
4. Re-run the workflow contract checker only if the truthful verifier shape changed, then write `.tmp/m034-s12/t02/local-repair-summary.json` with the exact commands and artifacts that now pass.

## Must-Haves

- [ ] Installed `meshc.exe build` no longer collapses into an opaque access-violation bundle on the staged hello fixture.
- [ ] Missing runtime/toolchain prerequisites surface as deterministic actionable errors when the happy path is not available.
- [ ] The repaired local proof includes both compiler-side and staged-verifier regressions.
- [ ] Any workflow/verifier contract change is re-verified locally before the hosted rerun.

## Verification

- `cargo test -p mesh-codegen link -- --nocapture`
- `cargo test -p meshc --test e2e_m034_s12 -- --nocapture`
- `pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1`
- `pwsh -NoProfile -File scripts/tests/verify-m034-s03-installed-build.ps1`
- `bash scripts/verify-m034-s02-workflows.sh`

## Observability Impact

- Signals added/changed: the installed compiler path must emit target/runtime/toolchain mismatches as normal logged errors instead of empty crash bundles.
- How a future agent inspects this: read `.tmp/m034-s12/t02/local-repair-summary.json`, the staged verifier logs, and the targeted Rust/PowerShell test outputs.
- Failure state exposed: future regressions stay attributable to runtime discovery, verifier env export, or earlier codegen rather than to a generic hosted failure.

## Inputs

- `compiler/mesh-codegen/src/link.rs` — current target-aware runtime discovery and linker behavior.
- `compiler/mesh-codegen/src/lib.rs` — compiler entrypoints that thread target/runtime paths into linking.
- `compiler/meshc/src/main.rs` — CLI build path that invokes `mesh_codegen::compile_mir_to_binary(...)`.
- `scripts/verify-m034-s03.ps1` — staged Windows installer/build verifier.
- `scripts/tests/verify-m034-s03-installed-build.ps1` — installed-build regression from T01.
- `compiler/meshc/tests/e2e_m034_s12.rs` — focused compiler-side regression from T01.
- `.github/workflows/release.yml` — hosted release-smoke workflow that must stay truthful if the verifier contract shifts.
- `scripts/verify-m034-s02-workflows.sh` — contract checker for any workflow/verifier shape change.
- `.tmp/m034-s12/t01/diagnostic-summary.json` — T01 classification of the current failure phase.

## Expected Output

- `compiler/mesh-codegen/src/link.rs` — repaired Windows runtime/linker discovery or clearer failure reporting.
- `compiler/mesh-codegen/src/lib.rs` — updated compiler plumbing if the repair must thread runtime-path context explicitly.
- `compiler/meshc/src/main.rs` — updated CLI build path if the repair belongs at the entrypoint.
- `scripts/verify-m034-s03.ps1` — truthful staged verifier that exercises and explains the repaired installed-build path.
- `compiler/meshc/tests/e2e_m034_s12.rs` — expanded regression coverage for the installed compiler boundary.
- `scripts/tests/verify-m034-s03-installed-build.ps1` — staged installed-build regression that now passes or fails with deterministic diagnostics.
- `.tmp/m034-s12/t02/local-repair-summary.json` — compact proof bundle for the repaired local boundary.
