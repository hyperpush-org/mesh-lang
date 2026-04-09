---
id: M038
title: "Windows Release Smoke Fix — Context"
status: complete
completed_at: 2026-03-28T03:51:27.872Z
key_decisions:
  - D112 — remove `/FORCE:MULTIPLE` and eliminate duplicate-symbol sources rather than keeping an unsafe linker override
  - D113 — strip rpmalloc objects from `LLVMSupport.lib` via `llvm-ar` extract/rebuild plus a stubbed `rpmalloc_linker_reference`
  - D114 — pass required Windows system libraries to the installed MSVC compiler path via `-Wl,` forwarding through `clang.exe`
key_files:
  - .github/workflows/release.yml
  - compiler/mesh-codegen/src/lib.rs
  - compiler/mesh-codegen/src/link.rs
lessons_learned:
  - LLVM's Windows prebuilt tarball is not fully self-contained for `llvm-config --system-libs`; providing the missing libxml2 archive from the LLVM prefix avoids duplicate-search-path regressions better than exporting vcpkg globally.
  - For the Windows installed compiler path, fixing the workflow alone is not enough: the final `meshc.exe build` smoke test also depends on forwarding Rust staticlib transitive MSVC system libraries to `link.exe`.
  - Milestone closeout on a local integration branch can make `git diff HEAD $(git merge-base HEAD main)` look empty even when the milestone changed real code; the truthful non-`.gsd` baseline is `origin/main` or the pre-milestone integration commit.
---

# M038: Windows Release Smoke Fix — Context

**M038 removed the Windows MSVC release smoke crash path by fixing duplicate-symbol and link-environment issues, leaving the hosted release workflow green without `/FORCE:MULTIPLE`.**

## What Happened

M038 was a single-slice tactical closeout milestone focused on the Windows hosted release-smoke failure. S01 repaired three separate causes behind the broken `meshc.exe build` path: it stopped introducing a second libxml2 search path by copying the needed static lib into the LLVM prefix instead of exporting vcpkg globally, stripped LLVM 21's bundled rpmalloc objects out of `LLVMSupport.lib` with an `llvm-ar` extract/rebuild flow plus a stubbed `rpmalloc_linker_reference`, and taught the installed Windows compiler path to forward the required MSVC system libraries through `clang.exe` to `link.exe`. The slice also added `build_trace::set_stage("pre-llvm-init")` breadcrumbs before `Context::create()` so any future Windows crash around LLVM initialization is easier to bucket. Verification stayed proof-first: the local code delta exists in `.github/workflows/release.yml`, `compiler/mesh-codegen/src/lib.rs`, and `compiler/mesh-codegen/src/link.rs`; `cargo test -p mesh-codegen link -- --nocapture` passed; `/FORCE:MULTIPLE` is absent from the release workflow; and the hosted `release.yml` run 23676428260 finished green across all non-skipped jobs, including the Windows build and verify-release-assets jobs. With only one slice in the milestone, there were no cross-slice integration seams beyond confirming the workflow and codegen/linker changes worked together end to end.

## Success Criteria Results

- **Windows `meshc.exe build` access violation fixed** — Met. S01 resolved the three layered Windows MSVC failures called out in the slice summary: libxml2 duplicate-symbol conflicts, LLVM 21 rpmalloc/ucrt heap collisions, and missing final-link system libraries for the installed compiler path. Evidence: S01 summary narrative plus hosted `release.yml` run `23676428260` concluding `success`.
- **All six hosted workflow lanes green** — Met. `M038-VALIDATION.md` records this criterion as passed with the evidence that run `23676428260` had "all 16 non-skipped jobs green," including `Build (x86_64-pc-windows-msvc)` and `Verify release assets (x86_64-pc-windows-msvc)`. The two skipped jobs are tag-gated by design and are not hosted-lane regressions.
- **No `/FORCE:MULTIPLE` linker hack remains** — Met. S01 verification explicitly recorded `rg 'FORCE:MULTIPLE' .github/workflows/release.yml` with no matches, and the workflow diff for the milestone shows the Windows release lane was repaired by removing the root causes instead of retaining the linker override.
- **Code-change verification** — Met. The direct `git diff --stat HEAD $(git merge-base HEAD main) -- ':!.gsd/'` check was empty because local `main` already contains the milestone commits; using the closeout-equivalent pre-M038 integration range `git diff --stat 5f649152..HEAD -- ':!.gsd/'` shows real non-`.gsd` changes in `.github/workflows/release.yml`, `compiler/mesh-codegen/src/lib.rs`, and `compiler/mesh-codegen/src/link.rs`.

## Definition of Done Results

- **All roadmap slices complete** — Met. The only planned slice, `S01`, is marked complete in the roadmap context and has a rendered summary/UAT pair on disk.
- **All slice/task summary artifacts exist** — Met. Files present under `.gsd/milestones/M038/` include `slices/S01/S01-SUMMARY.md`, `slices/S01/S01-UAT.md`, `slices/S01/tasks/T01-SUMMARY.md`, and `slices/S01/tasks/T02-SUMMARY.md`.
- **Cross-slice integration works** — Met / N/A. This is a single-slice milestone. `M038-VALIDATION.md` records cross-slice integration as not applicable beyond verifying the workflow and codegen/linker changes together in hosted CI.
- **Horizontal checklist** — No horizontal checklist was present in the milestone context provided, so there were no additional checklist items to audit.
- **Milestone validation verdict** — Met. `.gsd/milestones/M038/M038-VALIDATION.md` is present with verdict `pass`, remediation round `0`, and a checklist marking all success criteria complete.

## Requirement Outcomes

No requirement statuses changed during M038.

This milestone was intentionally a tactical CI/build repair with no mapped requirement owner changes. `M038-VALIDATION.md` explicitly records requirement coverage as complete with no gaps because the milestone fixed an infrastructure regression rather than advancing or re-scoping a product capability requirement. As a result, there are no `gsd_requirement_update` calls to apply for this closeout.

## Deviations

T01 initially removed vcpkg libxml2 entirely, but CI proved LLVM still needed a `libxml2s.lib` artifact. The milestone also uncovered an additional LLVM 21 rpmalloc/ucrt collision and the installed `meshc.exe` system-library forwarding issue before the final hosted green run.

## Follow-ups

- Update Actions dependencies that still warn about Node.js 20 deprecation before runner support is removed in September 2026.
- Keep the Windows system-library list in `compiler/mesh-codegen/src/link.rs` under review if future runtime dependencies introduce additional Win32 link requirements.
