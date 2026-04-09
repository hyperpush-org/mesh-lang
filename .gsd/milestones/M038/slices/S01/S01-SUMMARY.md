---
id: S01
parent: M038
milestone: M038
provides:
  - Green release.yml Windows build and verify jobs without /FORCE:MULTIPLE
  - Working installed meshc.exe build on Windows MSVC with proper system library linking
requires:
  []
affects:
  []
key_files:
  - compiler/mesh-codegen/src/lib.rs
  - compiler/mesh-codegen/src/link.rs
  - .github/workflows/release.yml
key_decisions:
  - Removed vcpkg from global LIB path; instead copy libxml2 .lib into LLVM prefix dir to avoid duplicate symbols while satisfying llvm-config --system-libs
  - Strip rpmalloc from LLVM tarball's LLVMSupport.lib via llvm-ar extract/rebuild rather than using /FORCE:MULTIPLE
  - Add Windows system libraries (ws2_32, userenv, advapi32, bcrypt, ntdll, kernel32, msvcrt, synchronization) to the MSVC clang link invocation for installed meshc.exe build
patterns_established:
  - LLVM tarball post-processing: extract, strip, rebuild with llvm-ar to remove unwanted objects from prebuilt static libraries
  - Windows MSVC system library forwarding via -Wl, prefix through clang to link.exe
observability_surfaces:
  - build_trace::set_stage('pre-llvm-init') before Context::create() in both codegen entry points
  - clang -v flag on Windows MSVC link invocations for diagnostic visibility
drill_down_paths:
  - .gsd/milestones/M038/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M038/slices/S01/tasks/T02-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-28T03:41:31.841Z
blocker_discovered: false
---

# S01: Fix Windows MSVC Build and Verify Release Lane

**Fixed three distinct Windows MSVC link failures — libxml2 duplicate symbols, rpmalloc/ucrt heap collisions, and missing system libraries in the installed compiler's link invocation — making the full release.yml workflow green without /FORCE:MULTIPLE.**

## What Happened

The Windows MSVC release build had been failing with LNK1169 (multiply defined symbols) requiring the /FORCE:MULTIPLE linker hack. Investigation revealed three layered problems:

**Layer 1: libxml2 duplicate symbols.** The original workflow installed vcpkg's libxml2 and added its lib directory to the global LIB path. LLVM's own lib directory also appeared on the linker path via llvm-sys. When both contained libxml2-related symbols, the MSVC linker reported duplicates. T01 initially removed vcpkg entirely, but CI proved that llvm-config --system-libs still demands libxml2s.lib which the LLVM tarball doesn't ship. The fix: reinstall vcpkg libxml2 but copy the .lib into the LLVM prefix lib directory instead of adding vcpkg to the global LIB path. This satisfies llvm-config without introducing a second lib search path.

**Layer 2: rpmalloc/ucrt heap symbol collision.** With the libxml2 fix in place, a second duplicate-symbol error surfaced: LLVM 21's prebuilt tarball is built with LLVM_ENABLE_RPMALLOC=ON, which embeds rpmalloc's malloc/free/realloc/calloc/_msize overrides into LLVMSupport.lib. These collide with ucrt.lib's heap symbols. The fix: after extracting the tarball, use llvm-ar to extract all objects from LLVMSupport.lib, delete the rpmalloc object files, compile a no-op stub for rpmalloc_linker_reference (referenced by InitLLVM.cpp.obj), and rebuild the library. The LLVM cache key was bumped to v4 to invalidate old cached copies.

**Layer 3: Missing Windows system libraries in installed meshc.exe build.** Even after the build job went green, the verify job's installed meshc.exe build test failed. The installed compiler uses clang.exe to link Mesh programs against mesh_rt.lib, but mesh_rt.lib is a Rust staticlib whose transitive dependencies (ureq for TLS, bundled SQLite, crossbeam, rand, Rust std) need Windows system libraries at final link time. The fix: add ws2_32, userenv, advapi32, bcrypt, ntdll, kernel32, msvcrt, and synchronization via -Wl, prefix so clang forwards them to link.exe. Also added -v for diagnostic visibility on future link failures.

Additionally, T01 added build_trace::set_stage("pre-llvm-init") before Context::create() in both compile_to_binary and compile_mir_to_binary entry points, providing diagnostic breadcrumbs for any future Windows crash during LLVM initialization.

## Verification

1. cargo test -p mesh-codegen link -- --nocapture: 8 passed
2. rg 'FORCE:MULTIPLE' .github/workflows/release.yml: no matches (absent)
3. YAML validation: valid
4. pre-llvm-init stage present in both codegen entry points
5. Hosted release.yml run 23676428260 on main: all 16 non-skipped jobs green, including Build (x86_64-pc-windows-msvc) and Verify release assets (x86_64-pc-windows-msvc). Two skipped jobs (Authoritative live proof, Create Release) are tag-gated by design.
6. Full workflow conclusion: success

## Requirements Advanced

None.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

T01 initially removed vcpkg libxml2 entirely, but CI proved it was still needed (llvm-config --system-libs demands libxml2s.lib). T02 was marked complete while CI run 23675078751 was still in progress; the slice closer discovered it failed with the rpmalloc collision and fixed it. Two additional issues emerged beyond the original plan: the llvm-lib /REMOVE approach didn't work (switched to llvm-ar extract/rebuild), and the installed meshc.exe link step needed explicit Windows system libraries.

## Known Limitations

The Node.js 20 deprecation warning from actions/checkout@v4 and actions/download-artifact@v4 will need attention before September 2026 when Node 20 is removed from runners. The 'Authoritative live proof' and 'Create Release' jobs are skipped by design on main pushes — they only run on release tags (refs/tags/v*).

## Follow-ups

None.

## Files Created/Modified

- `compiler/mesh-codegen/src/lib.rs` — Added build_trace::set_stage('pre-llvm-init') before Context::create() in both compile_to_binary and compile_mir_to_binary
- `compiler/mesh-codegen/src/link.rs` — Added Windows system libraries via -Wl, prefix and -v verbose flag to MSVC link invocation
- `.github/workflows/release.yml` — Removed /FORCE:MULTIPLE, added vcpkg libxml2 copy-to-LLVM-prefix, added rpmalloc stripping via llvm-ar extract/rebuild, bumped LLVM cache key to v4
