---
id: S12
parent: M034
milestone: M034
provides:
  - A local regression and diagnostic seam for Windows installed-compiler failures (`MESH_BUILD_TRACE_PATH`, e2e coverage, and PowerShell classifier coverage).
  - A repaired installed-compiler preflight handshake that resolves runtime and Windows clang prerequisites before object emission and exports repo-root `CARGO_TARGET_DIR` in the staged verifier.
  - Fresh hosted blocker evidence for `release.yml` on `v0.1.0` plus fail-closed protection for the reserved `first-green` archive.
requires:
  - slice: S11
    provides: The approved `v0.1.0` rerun target, prior hosted Windows diagnostics baseline, and the known red `release.yml` blocker surface that S12 had to refresh and tighten.
affects:
  []
key_files:
  - compiler/mesh-codegen/src/link.rs
  - compiler/mesh-codegen/src/lib.rs
  - compiler/meshc/src/main.rs
  - compiler/meshc/tests/e2e_m034_s12.rs
  - scripts/verify-m034-s03.ps1
  - scripts/tests/verify-m034-s03-installed-build.ps1
  - scripts/verify-m034-s06-remote-evidence.sh
  - scripts/tests/verify-m034-s06-contract.test.mjs
  - .tmp/m034-s12/t03/hosted-rollout-summary.json
  - .tmp/m034-s12/t04/final-closeout-summary.json
  - .gsd/PROJECT.md
key_decisions:
  - Use `MESH_BUILD_TRACE_PATH` plus verifier-side JSON classification instead of exit-code inference for Windows staged-smoke diagnosis.
  - Preflight runtime discovery and Windows clang resolution before LLVM object emission, while the staged verifier exports repo-root `CARGO_TARGET_DIR` and preserves `MESH_RT_LIB_PATH` as an explicit override.
  - Allow red diagnostic archives under nonreserved labels, but refuse the reserved `first-green` label unless the stop-after remote-evidence bundle is genuinely green.
patterns_established:
  - Classify Windows installed-compiler failures from persisted build traces and summary JSON rather than from opaque process exits.
  - Move runtime/toolchain prerequisite discovery ahead of object emission so installed-compiler failures become actionable prerequisite errors instead of crash bundles.
  - Protect one-shot evidence labels with fail-closed semantics: diagnostic labels may archive red bundles, but reserved milestone-truth labels must require a green source bundle.
observability_surfaces:
  - `.tmp/m034-s12/t01/diagnostic-summary.json` classifies the hosted or local Windows smoke failure bucket.
  - `.tmp/m034-s12/t03/hosted-rollout-summary.json` records the approved rerun target, resulting run state, and failing hosted job URL.
  - `.tmp/m034-s12/t03/diag-download/windows/verify/run/07-hello-build.log` is the authoritative hosted Windows smoke blocker artifact.
  - `.tmp/m034-s05/verify/remote-runs.json`, `status.txt`, `current-phase.txt`, and `phase-report.txt` show whether closeout is blocked at hosted rollout or has moved downstream.
  - `.tmp/m034-s12/t04/final-closeout-summary.json` ties the fresh hosted blocker to the fresh full replay state and the reserved-label contract outcome.
drill_down_paths:
  - .gsd/milestones/M034/slices/S12/tasks/T01-SUMMARY.md
  - .gsd/milestones/M034/slices/S12/tasks/T02-SUMMARY.md
  - .gsd/milestones/M034/slices/S12/tasks/T03-SUMMARY.md
  - .gsd/milestones/M034/slices/S12/tasks/T04-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-28T00:49:49.581Z
blocker_discovered: false
---

# S12: Windows release-smoke remediation and final green closeout

**Local Windows release-smoke diagnostics, installed-compiler preflight repairs, and fail-closed `first-green` archive protection landed, but hosted `release.yml` on `v0.1.0` remains the final blocker to milestone closeout.**

## What Happened

S12 repaired the remaining local blind spots around the Windows staged installer smoke path and then refreshed the hosted evidence honestly. T01 added an opt-in `MESH_BUILD_TRACE_PATH` build trace and verifier-side classification so installed `meshc.exe build` failures can be separated into pre-object, runtime-lookup, and link-time buckets instead of collapsing into opaque exit codes. T02 moved runtime/toolchain discovery ahead of LLVM object emission, added an explicit `MESH_RT_LIB_PATH` override for regressions, exported repo-root `CARGO_TARGET_DIR` from the staged PowerShell verifier, and expanded Rust plus PowerShell regressions around the installed-compiler contract. Those local proofs now pass cleanly.

After the local repair surface went green, T03 reused the approved hosted mutation and reran `release.yml` on `refs/tags/v0.1.0` / SHA `1e83ea930fdfd346b9e56659dc50d2f759ec5da2`. The rerun stayed red only at `Verify release assets (x86_64-pc-windows-msvc)`, so the slice refreshed `.tmp/m034-s05/verify/remote-runs.json`, preserved a fresh Windows diagnostics bundle under `.tmp/m034-s12/t03/diag-download/windows/verify/run/07-hello-build.log`, and confirmed that the canonical stop-after replay still fails at `remote-evidence` for the same hosted run.

T04 then found and fixed a local truth-surface bug: the reserved `first-green` archive label could still be spent on a red stop-after bundle. The helper now allows red diagnostic archives under nonreserved labels but refuses to archive `first-green` unless the stop-after verifier exits 0 with `status.txt=ok` and `current-phase.txt=stopped-after-remote-evidence`. The bogus red `first-green` archive was removed, the contract tests were expanded, and a fresh full `.env`-backed `bash scripts/verify-m034-s05.sh` replay confirmed the milestone is still blocked at `remote-evidence` because hosted `release.yml` remains failure on the expected SHA.

So S12 did not achieve the original 'final green closeout' demo. What it actually delivered is a truthful, replayable boundary: the local Windows compiler/verifier repairs are in place, the hosted blocker is freshly captured rather than inferred from stale artifacts, and the one-shot `first-green` evidence cannot be spent dishonestly while the release lane is still red.

## Verification

Passed the repaired local proof surface: `pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1`, `pwsh -NoProfile -File scripts/tests/verify-m034-s03-installed-build.ps1`, `cargo test -p meshc --test e2e_m034_s12 -- --nocapture`, `cargo test -p mesh-codegen link -- --nocapture`, and `bash scripts/verify-m034-s02-workflows.sh`.

Confirmed the hosted blocker is still live with fresh evidence: `VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh` exits 1 at `remote-evidence`, `bash scripts/verify-m034-s06-remote-evidence.sh first-green` exits 1 and refuses the reserved label, and the full `.env`-backed `bash scripts/verify-m034-s05.sh` replay also exits 1 at `remote-evidence`.

Fresh output after the replay is truthful and consistent: `.tmp/m034-s05/verify/status.txt` contains `failed`, `.tmp/m034-s05/verify/current-phase.txt` contains `remote-evidence`, `.tmp/m034-s05/verify/phase-report.txt` shows earlier phases passed and `remote-evidence` failed, `.tmp/m034-s05/verify/remote-runs.json` points at hosted `release.yml` run `23669185030` on SHA `1e83ea930fdfd346b9e56659dc50d2f759ec5da2`, and `.tmp/m034-s06/evidence/first-green/` remains absent.

## Requirements Advanced

None.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

The slice plan assumed the local repairs would clear the hosted rerun and allow one-shot `first-green` capture plus a green full S05 replay. In reality, the approved hosted rerun remained red on the same Windows release-smoke job, so the slice pivoted from milestone closeout to truthful blocker capture. T04 therefore spent its effort on fixing the reserved-label contract and documenting the live hosted blocker instead of claiming final closeout.

## Known Limitations

Hosted `release.yml` run `23669185030` is still `completed/failure` on `refs/tags/v0.1.0` / SHA `1e83ea930fdfd346b9e56659dc50d2f759ec5da2`, specifically at `Verify release assets (x86_64-pc-windows-msvc)`. Because `remote-evidence` fails closed on that hosted run, the reserved `first-green` archive cannot be created and the full `.env`-backed `bash scripts/verify-m034-s05.sh` replay never reaches `public-http` or `s01-live-proof`. Local macOS verification can prove the trace/preflight contract, but it still cannot execute the actual staged Windows binaries directly; the remaining truth source is the hosted diagnostics bundle.

## Follow-ups

Resume from `.tmp/m034-s12/t03/diag-download/windows/verify/run/07-hello-build.log`, `.tmp/m034-s12/t01/diagnostic-summary.json`, and the new preflight trace surfaces in `compiler/meshc/tests/e2e_m034_s12.rs` / `scripts/tests/verify-m034-s03-installed-build.ps1`. Fix or otherwise retire the hosted installed `meshc.exe build` crash, obtain explicit approval for the next GitHub mutation, rerun hosted `release.yml` on the approved tag/SHA, rerun `VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh`, capture `first-green` exactly once, and only then rerun the full `.env`-backed `bash scripts/verify-m034-s05.sh` closeout replay.

## Files Created/Modified

- `compiler/mesh-codegen/src/link.rs` — Moved runtime/library and Windows clang discovery into a pre-object-emission preflight and made link planning record target-aware trace context.
- `compiler/mesh-codegen/src/lib.rs` — Threaded the new trace and preflight data through codegen so installed builds emit actionable failure context.
- `compiler/meshc/src/main.rs` — Plumbed compiler-side build tracing support for `MESH_BUILD_TRACE_PATH` into the CLI build path.
- `compiler/meshc/tests/e2e_m034_s12.rs` — Added focused regressions for native build tracing, missing runtime lookup, and bad Windows LLVM-prefix preflight behavior.
- `scripts/verify-m034-s03.ps1` — Exported repo-root `CARGO_TARGET_DIR`, preserved installed-build context, and classified staged Windows smoke failures from trace data.
- `scripts/tests/verify-m034-s03-installed-build.ps1` — Added PowerShell coverage for hosted-log parsing, trace classification buckets, and verifier environment shaping.
- `scripts/verify-m034-s06-remote-evidence.sh` — Changed reserved-label behavior so `first-green` refuses red stop-after bundles while diagnostic labels remain usable for blocker capture.
- `scripts/tests/verify-m034-s06-contract.test.mjs` — Added contract coverage for red-label refusal and green-label acceptance on the reserved `first-green` path.
- `.tmp/m034-s12/t03/hosted-rollout-summary.json` — Recorded the approved hosted rerun target, resulting run status, failing job, and refreshed remote-evidence linkage.
- `.tmp/m034-s12/t04/final-closeout-summary.json` — Captured the final blocked closeout state tying hosted `release.yml` failure to the full replay stopping at `remote-evidence`.
- `.gsd/PROJECT.md` — Updated the repo snapshot from the older S11 blocker note to the current S12 local-repair plus hosted-blocker state.
