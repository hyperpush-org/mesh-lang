---
estimated_steps: 4
estimated_files: 5
skills_used:
  - debug-like-expert
  - powershell-windows
---

# T01: Create a truthful Windows installed-build regression and diagnostic seam

**Slice:** S12 — Windows release-smoke remediation and final green closeout
**Milestone:** M034

## Description

Create the first local proof surface that exercises the same boundary the hosted release lane is failing on: installed Windows `meshc.exe build` against the staged hello fixture. The task is diagnostic-first. It must keep the installed compiler path real, enrich the verifier with enough resolved-path context to separate codegen/object-emission failure from runtime lookup or linker failure, and add at least one regression file that makes this boundary executable instead of anecdotal.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m034-s03.ps1` staged verifier | Stop on the first failing phase and keep `.tmp/m034-s03/windows/verify/run/` as the authoritative diagnostic surface. | Keep the run blocking; do not replace the installed-build step with a weaker helper-only proof. | Treat missing phase logs, missing env markers, or missing emitted-object evidence as verifier drift. |
| Installed `meshc.exe build` path | Preserve resolved LLVM/runtime/linker inputs and record whether object emission completed before failure. | Fail with the last completed phase and keep the diagnostic summary red. | Treat empty stdout/stderr plus missing enriched diagnostics as an incomplete seam, not as a passing repro. |
| S11 hosted crash artifact | Use the artifact only as a boundary anchor; do not claim the local seam matches it unless the new summary shows the same failing phase. | Keep the artifact comparison local and deterministic. | Treat missing or truncated `07-hello-build.log` content as an evidence gap that must be refetched or replaced. |

## Load Profile

- **Shared resources**: `.tmp/m034-s03/windows/verify/`, staged release archives, LLVM/runtime env vars, and the installed-build fixture.
- **Per-operation cost**: one staged install plus one installed `meshc.exe build` attempt and a small diagnostic summary write.
- **10x breakpoint**: repeated runs can overwrite the same `.tmp` logs and make the crash phase ambiguous unless the task writes a dedicated summary under `.tmp/m034-s12/t01/`.

## Negative Tests

- **Malformed inputs**: unset or wrong `LLVM_SYS_211_PREFIX`, unset `CARGO_TARGET_DIR`, missing staged archive files, and truncated hosted `07-hello-build.log` input.
- **Error paths**: installed build still exits non-zero, object emission never happens, runtime lookup cannot find `mesh_rt.lib`, or linker invocation fails after object creation.
- **Boundary conditions**: diagnostic surface when the build succeeds locally, when it fails before object emission, and when it reaches linker/runtime lookup with an actionable error.

## Steps

1. Reproduce the staged installed-build path as closely as possible to hosted `07-hello-build`, keeping installed `meshc.exe build` as the real boundary rather than dropping to a unit-only helper test.
2. Extend `scripts/verify-m034-s03.ps1` to record resolved LLVM/runtime/linker inputs, emitted-object progress, and installed-binary paths without echoing secrets.
3. Add a focused regression file in `scripts/tests/verify-m034-s03-installed-build.ps1` and/or `compiler/meshc/tests/e2e_m034_s12.rs` that turns the installed-build boundary into a mechanically executable proof surface.
4. Write `.tmp/m034-s12/t01/diagnostic-summary.json` that classifies the current failure as pre-object, runtime lookup, or link-time.

## Must-Haves

- [ ] The installed Windows hello-build path is exercised through the staged installer flow, not a synthetic shortcut.
- [ ] The verifier logs resolved LLVM/runtime/linker state and whether object emission completed.
- [ ] At least one new regression file with real assertions exists for this boundary.
- [ ] `.tmp/m034-s12/t01/diagnostic-summary.json` captures the current failing phase and the evidence paths behind it.

## Verification

- `pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1`
- `pwsh -NoProfile -File scripts/tests/verify-m034-s03-installed-build.ps1`
- `cargo test -p meshc --test e2e_m034_s12 -- --nocapture`

## Observability Impact

- Signals added/changed: enriched staged-verifier logs for resolved LLVM/runtime/linker inputs and emitted-object progress.
- How a future agent inspects this: read `.tmp/m034-s03/windows/verify/run/` plus `.tmp/m034-s12/t01/diagnostic-summary.json` before touching compiler code.
- Failure state exposed: the hosted-style crash boundary becomes attributable to a concrete local phase instead of an empty `exit_code` bundle.

## Inputs

- `scripts/verify-m034-s03.ps1` — current staged Windows installer/build verifier.
- `scripts/tests/verify-m034-s03-last-exitcode.ps1` — existing PowerShell logging regression that must keep passing.
- `compiler/meshc/src/main.rs` — CLI build entrypoint for the installed compiler path.
- `compiler/mesh-codegen/src/lib.rs` — compiler pipeline entrypoints that call runtime lookup and linking.
- `compiler/mesh-codegen/src/link.rs` — target-aware runtime discovery and linker invocation.
- `compiler/mesh-codegen/src/codegen/mod.rs` — earlier LLVM/object-emission boundary if the crash happens before link.
- `scripts/fixtures/m034-s03-installer-smoke/main.mpl` — hello fixture the installed compiler must build.
- `.tmp/m034-s11/t03/diag-download/windows/verify/run/07-hello-build.log` — authoritative hosted crash boundary from S11.

## Expected Output

- `scripts/verify-m034-s03.ps1` — enriched staged verifier diagnostics for the installed-build boundary.
- `scripts/tests/verify-m034-s03-installed-build.ps1` — new installed-build regression with real assertions.
- `compiler/meshc/tests/e2e_m034_s12.rs` — focused compiler-side regression covering the same boundary.
- `.tmp/m034-s12/t01/diagnostic-summary.json` — compact classification of the current failure phase and evidence paths.
