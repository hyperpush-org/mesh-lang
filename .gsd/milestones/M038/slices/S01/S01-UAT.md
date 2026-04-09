# S01: Fix Windows MSVC Build and Verify Release Lane — UAT

**Milestone:** M038
**Written:** 2026-03-28T03:41:31.841Z

## UAT: Fix Windows MSVC Build and Verify Release Lane

### Preconditions
- Access to the GitHub Actions CI for the mesh-lang repository
- Local checkout of the repo at the slice commit or later

### Test 1: No /FORCE:MULTIPLE in workflow
**Steps:**
1. Run `rg 'FORCE:MULTIPLE' .github/workflows/release.yml`
**Expected:** No matches. The linker hack is gone.

### Test 2: Local link tests pass
**Steps:**
1. Run `cargo test -p mesh-codegen link -- --nocapture`
**Expected:** 8 tests pass.

### Test 3: YAML validity
**Steps:**
1. Run `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"`
**Expected:** No error.

### Test 4: Pre-LLVM-init build trace stage present
**Steps:**
1. Run `rg 'pre-llvm-init' compiler/mesh-codegen/src/lib.rs`
**Expected:** Two matches — one in compile_to_binary, one in compile_mir_to_binary.

### Test 5: Windows system libraries in MSVC link path
**Steps:**
1. Run `rg 'Wl,ws2_32' compiler/mesh-codegen/src/link.rs`
**Expected:** Match found — system libs are forwarded via -Wl, prefix.

### Test 6: Hosted release.yml all green
**Steps:**
1. Push to main and wait for release.yml to complete
2. Check all Build and Verify jobs
**Expected:** All non-skipped jobs are green, including Build (x86_64-pc-windows-msvc) and Verify release assets (x86_64-pc-windows-msvc). The two skipped jobs (Authoritative live proof, Create Release) are tag-gated and expected to skip on main pushes.

### Test 7: rpmalloc stripping in both LLVM extraction points
**Steps:**
1. Run `rg -c 'rpmalloc_strip' .github/workflows/release.yml`
**Expected:** 2 occurrences — one in Build job, one in Verify job.

### Edge Cases
- **LLVM cache hit:** The cache key was bumped to v4 so old copies with rpmalloc are invalidated. Verify the key with `rg 'llvm-21.1.8-v4' .github/workflows/release.yml`.
- **clang -v on link failure:** If a future link fails, stderr will contain the full link.exe invocation because -v is always passed on Windows MSVC targets.
