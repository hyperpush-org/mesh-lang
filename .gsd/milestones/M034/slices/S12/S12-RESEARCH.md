# S12 Research — Windows release-smoke remediation and final green closeout

## Summary

S12 starts from a narrow but still-opaque blocker. The slice directory exists, but `S12-PLAN.md` currently has an empty `## Tasks` section, so the planner needs to create the actual task split.

The remaining hosted failure is unchanged from S11: `release.yml` on `v0.1.0` reaches `Verify release assets (x86_64-pc-windows-msvc)`, then dies at `installed meshc.exe build installer smoke fixture`. The authoritative artifact is still `.tmp/m034-s11/t03/diag-download/windows/verify/run/07-hello-build.log`, which records `exit_code: -1073741819` with empty stdout/stderr.

What is already retired:
- The old missing-LLVM workflow setup bug is gone. `.github/workflows/release.yml` now installs LLVM 21 for the Windows smoke verifier, exports `LLVM_SYS_211_PREFIX`, builds `mesh-rt`, and runs `pwsh -NoProfile -File scripts/verify-m034-s03.ps1`.
- The PowerShell `$LASTEXITCODE`/strict-mode bug is also retired. `scripts/tests/verify-m034-s03-last-exitcode.ps1` only guards that helper behavior now.
- S10’s link-path refactor already made `mesh-codegen` target-aware for Windows MSVC and added `cargo test -p mesh-codegen link -- --nocapture` coverage.

What is still missing is a regression surface that exercises the same path the hosted runner is failing on: an **installed Windows compiler** building the trivial hello fixture. Right now local coverage stops short of that.

## Requirements Context

`REQUIREMENTS.md` was not preloaded in this unit, but the current slice clearly supports the M034 delivery-proof contract and continues the recovery-briefing advancement of **R007**: the milestone cannot claim delivery truth until the last hosted release-smoke lane is green and the final assembled replay is fresh.

This slice also supports the milestone-level release-proof goals described in M034 context: hosted release assets must be proven installable and runnable through the documented path, not inferred from artifact presence.

## Skills Discovered

No new skill installation is needed. Existing installed skills already match the work:

- `debug-like-expert` — relevant rule: verify actual behavior before proposing fixes; keep confirmed facts separate from live hypotheses.
- `github-workflows` — relevant rule: treat hosted workflow evidence as an observable surface, not as something to infer from YAML alone.
- `powershell-windows` — relevant because the remaining hosted failure is inside the PowerShell staged verifier, but the strict-mode/LASTEXITCODE pitfall is already fixed.
- `rust-best-practices` — relevant for any compiler-side repair in `mesh-codegen` / `meshc`.
- `llvm` — relevant because the failing path diverges from `meshc --version` exactly when `meshc build` enters LLVM/codegen/link territory.

## Recommendation

Plan S12 as a **diagnosis-first** remediation slice with four tasks:

1. **Create a truthful local regression/diagnostic seam for the installed Windows build path.**
   Do not start by editing workflow YAML again. The hosted failure is already on the correct path; the gap is that the repo has no local proof surface equivalent to hosted `07-hello-build`.

2. **Fix the actual installed-compiler failure based on that proof.**
   The most likely seams are:
   - runtime/static-library discovery in `compiler/mesh-codegen/src/link.rs`, especially how the installed compiler finds `mesh_rt.lib`
   - LLVM/codegen initialization in `compiler/mesh-codegen/src/codegen/mod.rs`, because `meshc --version` succeeds while `meshc build` crashes before emitting a normal Rust error

3. **After a local regression exists, rerun hosted `release.yml` on the approved `v0.1.0` ref and refresh remote evidence.**
   This is an external GitHub mutation and needs user confirmation at execution time.

4. **Only after hosted release smoke is green, spend `first-green` exactly once and run the fresh full `bash scripts/verify-m034-s05.sh` replay.**

## Implementation Landscape

### A. Current hosted failure surface

Relevant files and artifacts:
- `.github/workflows/release.yml`
- `scripts/verify-m034-s03.ps1`
- `scripts/fixtures/m034-s03-installer-smoke/{mesh.toml,main.mpl}`
- `.tmp/m034-s11/t03/diag-download/windows/verify/run/00-context.log`
- `.tmp/m034-s11/t03/diag-download/windows/verify/run/04-install-good.log`
- `.tmp/m034-s11/t03/diag-download/windows/verify/run/05-meshc-version.log`
- `.tmp/m034-s11/t03/diag-download/windows/verify/run/06-meshpkg-version.log`
- `.tmp/m034-s11/t03/diag-download/windows/verify/run/07-hello-build.log`

What the artifact proves today:
- install succeeds
- installed `meshc.exe --version` succeeds
- installed `meshpkg.exe --version` succeeds
- the first `meshc.exe build ...` invocation crashes hard enough to bypass normal stderr reporting

That means the failure boundary is already tight: it is not installer download, not archive shape, not version reporting, and not the older missing-LLVM workflow bug.

### B. The installed-compiler path still depends on auto-detection

Relevant code:
- `compiler/meshc/src/main.rs::build`
- `compiler/mesh-codegen/src/lib.rs::compile_mir_to_binary`
- `compiler/mesh-codegen/src/link.rs::{find_mesh_rt, find_workspace_target_dir, windows_clang_path}`

Important current behavior:
- `meshc build` calls `mesh_codegen::compile_mir_to_binary(..., target=None, rt_lib_path=None)`.
- `link.rs` only finds the runtime static library via:
  1. `CARGO_TARGET_DIR`, or
  2. walking up from `std::env::current_exe()` until it finds `target/`.
- In the Windows verifier, `mesh-rt` is built in the repo root, but the installed compiler runs from `%USERPROFILE%\.mesh\bin\meshc.exe` after `scripts/verify-m034-s03.ps1` rewrites `$env:USERPROFILE` to the sandbox home.
- No workflow step or PowerShell verifier step currently exports `CARGO_TARGET_DIR` before invoking installed `meshc.exe`.

That is a **real mismatch**: the verifier builds the runtime in repo `target/`, but the installed compiler’s runtime discovery logic never looks there unless `CARGO_TARGET_DIR` is set.

This does **not** fully explain the current hosted artifact by itself, because a clean runtime-missing path should ideally surface as a normal error, not an empty-stderr access violation. But it is still the first low-risk seam to test, because the current verifier path is relying on auto-discovery that is clearly not shaped for an installed executable.

### C. The crash may be earlier than linker/runtime lookup

Relevant code:
- `compiler/mesh-codegen/src/codegen/mod.rs::CodeGen::new`
- `compiler/mesh-codegen/src/lib.rs::{compile_to_object, compile_mir_to_binary}`

Why this remains a live hypothesis:
- `meshc --version` never touches LLVM/codegen.
- `meshc build` enters `CodeGen::new`, which calls `Target::initialize_native`, builds a target machine, emits an object file, and only then reaches `link::link`.
- The hosted artifact has empty stdout/stderr, which is more consistent with a native crash below normal Rust error reporting than with a clean `Err(String)` bubbling out of `link.rs`.

Planner implication:
- treat **both** of these as live hypotheses until disproved:
  1. runtime lookup is still broken in the installed verifier path
  2. the crash occurs earlier in LLVM/codegen init or object emission

A good S12 task does not guess between them first; it adds enough local proof/diagnostics to separate them.

### D. Current test and observability gap

What exists now:
- `cargo test -p mesh-codegen link -- --nocapture` covers target-aware linker/runtime helper behavior.
- `pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1` covers PowerShell log-combining behavior.

What does **not** exist:
- any regression that exercises installed Windows `meshc.exe` building the hello fixture
- any proof that records whether the failure happens before object emission, during runtime lookup, or during linker invocation

This is the main slice seam. Without filling it, another link-only or workflow-only patch can still leave hosted reruns opaque.

## Suggested Task Split

### T01 — Installed Windows build-path diagnosis + local regression seam

**Goal:** create a local/controlled surface that matches hosted `07-hello-build` closely enough to prove where the crash happens.

**Likely files**
- `scripts/verify-m034-s03.ps1`
- `scripts/tests/*` or a new focused verifier helper
- possibly `compiler/meshc/tests/*` if a Windows-only integration harness is a better fit

**What to prove first**
- whether explicitly setting `CARGO_TARGET_DIR=$RootDir/target` for the installed compiler changes the failure shape
- whether the build gets far enough to emit an object file before crashing
- what resolved values the failing path actually uses for `LLVM_SYS_211_PREFIX`, `CARGO_TARGET_DIR`, linker path, and expected runtime path

**Why first**
- This is the cheapest way to distinguish “broken runtime lookup” from “earlier LLVM/codegen crash” without another blind hosted rerun.

### T02 — Installed Windows compiler repair

**Goal:** make the installed compiler build the staged hello fixture successfully, or at minimum fail with a deterministic actionable error when verifier preconditions are missing.

**Likely files**
- `compiler/mesh-codegen/src/link.rs`
- possibly `compiler/mesh-codegen/src/codegen/mod.rs`
- possibly `compiler/meshc/src/main.rs` if an explicit runtime path/env needs to be threaded from the CLI/verifier
- `scripts/verify-m034-s03.ps1` if the verifier must export an explicit runtime-discovery env to stay truthful

**Good first experiment**
- wire `CARGO_TARGET_DIR` into the Windows staged verifier and see whether the current hosted failure turns into a normal runtime/link result or goes green

**Do not do**
- do not reopen the old `$LASTEXITCODE` fix
- do not change the workflow to skip the hello build step

### T03 — Hosted rerun and evidence refresh

**Goal:** rerun the repaired Windows release smoke on the approved tag ref and refresh the remote-evidence bundle.

**Artifacts**
- `.tmp/m034-s05/verify/remote-runs.json`
- any freshly downloaded Windows diagnostics bundle if failure persists

**Constraint**
- requires explicit user confirmation before outward GitHub mutation

### T04 — First-green capture and full S05 replay

**Goal:** once hosted release smoke is green, capture `.tmp/m034-s06/evidence/first-green/` exactly once and then run the canonical full replay.

**Artifacts**
- `.tmp/m034-s06/evidence/first-green/manifest.json`
- fresh `bash scripts/verify-m034-s05.sh` output through `remote-evidence`, `public-http`, and `s01-live-proof`

## Verification Plan

### Local before any hosted rerun

Keep these existing checks:
- `cargo test -p mesh-codegen link -- --nocapture`
- `pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1`
- `bash scripts/verify-m034-s02-workflows.sh` if workflow/verifier contract text changes

Add a new regression or verifier harness that proves one of these outcomes explicitly:
- installed Windows `meshc build` succeeds on the hello fixture, or
- the failure is a clean actionable runtime/linker/configuration error with preserved resolved paths and env, not an opaque access violation bundle

### Hosted after local repair

- rerun `release.yml` on the approved `v0.1.0` ref
- rerun `VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh`
- confirm `.tmp/m034-s05/verify/remote-runs.json` shows `release.yml` green on the expected `headSha`

### Final closeout

Only after hosted `release.yml` is green:
- capture `.tmp/m034-s06/evidence/first-green/` exactly once
- run the full `.env`-backed `bash scripts/verify-m034-s05.sh` replay

## Risks / Notes for the Planner

- The S12 plan file currently has **no task entries**. The planner has to write the real breakdown; there is no prior task decomposition to inherit.
- Do not spend another unit rediscovering S10’s retired blockers. Missing-LLVM workflow setup and PowerShell strict-mode drift are already solved.
- The current verifier contract still depends on a repo-built `mesh-rt` artifact. If S12 keeps that contract, the runtime-path handshake between verifier and installed compiler must be explicit rather than assumed.
- Prefer diagnostic additions that survive hosted reruns. Empty `07-hello-build` bundles are not enough for another remediation loop.
- S12 is not done at “release lane green”. The slice also owns `first-green` capture and the fresh full S05 replay.

## Sources

Read during this unit:
- `.gsd/milestones/M034/slices/S12/S12-PLAN.md`
- `.gsd/milestones/M034/slices/S10/S10-RESEARCH.md`
- `.gsd/milestones/M034/slices/S10/S10-ASSESSMENT.md`
- `.gsd/milestones/M034/slices/S10/S10-SUMMARY.md`
- `.gsd/milestones/M034/slices/S10/tasks/T02-SUMMARY.md`
- `.gsd/milestones/M034/slices/S10/tasks/T02-PLAN.md`
- `.github/workflows/release.yml`
- `scripts/verify-m034-s03.ps1`
- `scripts/tests/verify-m034-s03-last-exitcode.ps1`
- `website/docs/public/install.ps1`
- `compiler/meshc/src/main.rs`
- `compiler/mesh-codegen/src/lib.rs`
- `compiler/mesh-codegen/src/link.rs`
- `compiler/mesh-codegen/src/codegen/mod.rs`
- `Cargo.toml`
- `.tmp/m034-s11/t03/diag-download/windows/verify/run/00-context.log`
- `.tmp/m034-s11/t03/diag-download/windows/verify/run/04-install-good.log`
- `.tmp/m034-s11/t03/diag-download/windows/verify/run/05-meshc-version.log`
- `.tmp/m034-s11/t03/diag-download/windows/verify/run/06-meshpkg-version.log`
- `.tmp/m034-s11/t03/diag-download/windows/verify/run/07-hello-build.log`
