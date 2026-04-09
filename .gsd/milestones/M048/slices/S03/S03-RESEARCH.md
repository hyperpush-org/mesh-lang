# S03 Research — Toolchain self-update commands

## Summary

- **Primary requirement:** `R113` — explicit binary self-update for the public Mesh toolchain.
- **Governing decision:** `D311` — `meshc update` and `meshpkg update` must mean **binary self-update through the existing release/install path**, not a second updater story.
- **Current gap:** neither CLI exposes `update` today. Repro:
  - `cargo run -q -p meshc -- update` → `error: unrecognized subcommand 'update'`
  - `cargo run -q -p meshpkg -- update` → `error: unrecognized subcommand 'update'`
- **Existing truth already exists:** the installer pair in `tools/install/install.sh` and `tools/install/install.ps1` installs **both** binaries, writes a shared `~/.mesh/version` / `~\.mesh\version`, and is already proven by the release workflow plus `scripts/verify-m034-s03.sh` / `scripts/verify-m034-s03.ps1`.
- The slice is **targeted research**, not open-ended architecture work. The safest implementation is to wire both CLIs into the existing installer contract instead of re-implementing release metadata, archive naming, checksum verification, install location, or PATH mutation in Rust.

## Skills Discovered

Existing installed skills already cover the core tech for this slice; no extra skill install is needed.

- **`rust-best-practices`**
  - Relevant rule: keep the updater path `Result`-based and avoid `unwrap()` / `expect()` in production code.
  - Relevant rule: prefer a small shared helper over duplicating logic across both binaries.
- **`powershell-windows`**
  - Relevant rule: any Windows launcher/bootstrap script should stay ASCII-only, use `Join-Path`, and capture `$LASTEXITCODE` explicitly.
  - Relevant rule: when logical operators combine cmdlets, wrap each cmdlet call in parentheses.

## Requirement Focus

### Primary
- **R113** — explicit self-update commands for the installed/staged toolchain.

### Supporting constraints
- **D311** requires reuse of the same release/install path users already trust.
- The release contract is already anchored by `.github/workflows/release.yml` and the M034 staged-installer verifiers.
- The public installer contract is pairwise/toolchain-wide, not per-binary:
  - both installers always install `meshc` **and** `meshpkg`
  - both installers use a single shared version file
  - release workflow enforces version alignment between `compiler/meshc/Cargo.toml` and `compiler/meshpkg/Cargo.toml`

## Implementation Landscape

### CLI entrypoints
- `compiler/meshc/src/main.rs`
  - Clap enum/dispatch for compiler subcommands.
  - No `Update` variant today.
  - Top-of-file command list/help text will need updating alongside the enum.
- `compiler/meshpkg/src/main.rs`
  - Clap enum/dispatch for registry/package commands.
  - No `Update` variant today.
  - Has a **global `--json` flag**, so update behavior must make an explicit decision about machine-readable mode.

### Shared library seam
- `compiler/mesh-pkg/src/lib.rs`
  - Good place to export a small shared updater helper because **both** binaries already depend on `mesh-pkg`.
  - `mesh-pkg` already has `ureq`; that is enough to fetch installer bytes without introducing a shell dependency on `curl`/`wget`.
  - There is **no current shared updater module**.

### Canonical installer behavior
- `tools/install/install.sh`
  - Canonical Unix installer.
  - Key behaviors already implemented:
    - default release metadata URL + release base URL
    - env overrides: `MESH_INSTALL_RELEASE_API_URL`, `MESH_INSTALL_RELEASE_BASE_URL`, `MESH_INSTALL_DOWNLOAD_TIMEOUT_SEC`, `MESH_INSTALL_STRICT_PROOF`
    - single `~/.mesh/version`
    - installs `meshc` and `meshpkg` together
    - `check_update_needed()` is keyed off the version file, not the currently running binary path
- `tools/install/install.ps1`
  - Canonical Windows installer.
  - Same pairwise contract as the shell installer.
  - Also uses the shared version file and installs both binaries together.
  - Important current truth: it is written to be invoked **externally**; there is no built-in installed-self-overwrite trampoline.

### Public installer mirror
- `website/docs/public/install.sh`
- `website/docs/public/install.ps1`
  - Exact public copies of the canonical installers.
  - M034 public-contract tests fail if these drift from `tools/install/*`.
  - If S03 can keep all special handling in Rust-side launch/orchestration, it avoids widening blast radius into these mirrored files.

### Release / proof surfaces
- `.github/workflows/release.yml`
  - Builds `meshc` and `meshpkg` separately but keeps versions aligned.
  - Packages archives with the names the installer expects:
    - `meshc-v<version>-<target>.(tar.gz|zip)`
    - `meshpkg-v<version>-<target>.(tar.gz|zip)`
  - Generates `SHA256SUMS`.
  - Runs staged installer smoke via:
    - `bash scripts/verify-m034-s03.sh`
    - `pwsh -NoProfile -File scripts/verify-m034-s03.ps1`
- `scripts/verify-m034-s03.sh`
- `scripts/verify-m034-s03.ps1`
  - Best existing reference for:
    - fake-home setup
    - staged local HTTP release server layout
    - release metadata JSON shape
    - archive naming expectations
    - post-install verification of both binaries

### Test harness patterns
- `compiler/meshc/tests/tooling_e2e.rs`
  - Good venue only for **light CLI smoke** such as “subcommand exists/help text is wired”.
  - Not a good home for staged release orchestration.
- `compiler/meshc/tests/e2e_m048_s01.rs`
- `compiler/meshc/tests/support/m046_route_free.rs`
  - Good pattern for a **dedicated slice acceptance rail** with retained artifacts under `.tmp/m048-s03/...`.
  - Useful helpers already exist for `repo_root()`, `meshc_bin()`, artifact dirs, and archive/log retention.
  - There is no existing `meshpkg_bin()` helper in `meshc` tests; new S03 test code will need to build/find it explicitly.

## Recommendation

### 1. Reuse the installer path, do not reimplement it
The safest shape is:

- add a shared Rust helper such as `mesh_pkg::toolchain_update::run(...)`
- have it download the **same public installer script** the docs already serve
- execute that installer with `--yes`
- pass through the existing `MESH_INSTALL_*` env overrides for staged testing

This preserves D311. It also means the new commands inherit:
- release metadata contract
- archive naming contract
- checksum behavior
- install destination
- PATH configuration
- pairwise versioning

### 2. Add one updater-specific env override for the installer script URL itself
Existing `MESH_INSTALL_*` env vars only redirect the installer **after it starts**. They do **not** help the new CLI locate a local installer copy for acceptance tests.

Recommended helper-level override:
- `MESH_UPDATE_INSTALLER_URL`

Recommended defaults:
- Unix: `https://meshlang.dev/install.sh`
- Windows: `https://meshlang.dev/install.ps1`

That gives S03 a deterministic local acceptance path without touching the public installer semantics.

### 3. Keep semantics toolchain-wide
Both `meshc update` and `meshpkg update` should refresh the **pair**, not only the invoking binary. That matches:
- the existing installers
- the shared version file
- the release workflow’s aligned versions
- D311’s “binary self-update through the existing release/install path” wording

### 4. Prefer stock Windows PowerShell for the real launcher path
The public installer usage string is `powershell -ExecutionPolicy ByPass ...`, not `pwsh`.

For Windows orchestration, prefer:
- `powershell.exe` / `powershell`
- only fall back if necessary

That keeps the command aligned with the already-documented public path.

### 5. Decide `meshpkg --json update` explicitly
`meshpkg` has a global JSON mode; raw installer prose would violate that contract if streamed directly.

Recommended smallest safe choice for S03:
- **fail closed** for `meshpkg --json update` with a clear message that update delegates to the public installer and does not currently support JSON mode.

That is safer than inventing half-structured async installer output in the same slice.

## Natural Seams / Task Shape

### Seam A — shared updater helper
Likely files:
- `compiler/mesh-pkg/src/lib.rs`
- `compiler/mesh-pkg/src/<new updater module>.rs`

Responsibilities:
- choose default installer URL by platform
- honor `MESH_UPDATE_INSTALLER_URL`
- fetch installer bytes with `ureq`
- Unix path: run `/bin/sh` with installer on stdin and `--yes`
- Windows path: choose launcher/orchestration strategy
- pass through `MESH_INSTALL_RELEASE_API_URL`, `MESH_INSTALL_RELEASE_BASE_URL`, `MESH_INSTALL_DOWNLOAD_TIMEOUT_SEC`, `MESH_INSTALL_STRICT_PROOF`
- return structured `Result<(), String>` errors (per `rust-best-practices`)

### Seam B — CLI wiring
Likely files:
- `compiler/meshc/src/main.rs`
- `compiler/meshpkg/src/main.rs`

Responsibilities:
- add `Update` clap subcommand variants
- dispatch into shared updater helper
- update top-of-file command documentation/help text
- decide/guard JSON behavior on `meshpkg`

### Seam C — acceptance rail
Recommended file:
- `compiler/meshc/tests/e2e_m048_s03.rs`

Why this shape:
- dedicated slice rail matches current M048 testing style better than bloating `tooling_e2e.rs`
- can retain scenario artifacts under `.tmp/m048-s03/...`
- can reuse `route_free` artifact helpers

Recommended scenarios:
1. **staged `meshc update`**
   - run a copied/extracted `meshc` binary from outside `~/.mesh/bin`
   - empty fake HOME / USERPROFILE
   - local HTTP server serves installer + staged release assets
   - assert install of both binaries into fake home
2. **installed `meshpkg update`**
   - seed fake home with installed binaries + credentials file
   - corrupt the sibling binary and/or stale version file
   - run installed `meshpkg update`
   - assert both binaries are healthy afterward
   - assert credentials survived

This gives the planner a small, honest proof bundle:
- both commands are exercised
- both staged and installed contexts are exercised
- update is proven to repair the toolchain pair, not just print help text

## Key Risks / Constraints

### 1. Windows installed-self-update is the sharp edge
The current PowerShell installer copies `meshc.exe` / `meshpkg.exe` into the install dir. A direct installed-binary flow like:
- `meshc.exe update`
- spawn `powershell install.ps1`
- installer overwrites `meshc.exe`

is the likely trap on Windows because the running executable is still locked.

Implication for planning:
- do **not** assume the Unix synchronous approach is portable to installed Windows binaries
- if full installed-Windows support is in-scope for S03, plan a trampoline/bootstrap path that lets the parent process exit before replacement happens
- keep that trampoline logic in Rust-side orchestration if possible so the mirrored installer copies do not need to change

### 2. The installer contract is mirrored and externally proven
If the implementation changes `tools/install/install.sh` or `tools/install/install.ps1`, the planner must remember:
- update the matching `website/docs/public/*` copy
- expect `scripts/verify-m034-s03.sh` / `.ps1` and public-contract tests to join the blast radius

### 3. The installer is version-file driven, not binary-path driven
Because the installers consult the shared version file, the new command should be framed as:
- “refresh the official Mesh toolchain install”

not:
- “patch this exact executable in place wherever it lives”

That distinction matters for:
- staged binaries
- source-built binaries
- fake-home acceptance tests

### 4. Credentials/state preservation is worth checking
`meshpkg` stores auth at `~/.mesh/credentials`, inside the same top-level home directory the installer writes into.

The installer does **not** remove `~/.mesh` during install/update, so S03 should cheaply verify that running self-update does not destroy auth state.

## Don’t Hand-Roll

Do **not** build a second Rust-native updater that duplicates:
- GitHub release metadata fetch logic
- archive naming rules
- SHA256SUMS parsing
- checksum verification behavior
- install destination logic
- PATH mutation
- pairwise version alignment

Those rules already live in the canonical installers plus `release.yml` and M034 staged-installer proof. D311 points straight at reuse.

## Verification

### Reproduce the current gap
- `cargo run -q -p meshc -- update`
- `cargo run -q -p meshpkg -- update`

Both currently fail with `unrecognized subcommand 'update'`.

### Recommended slice-local proof
- `cargo test -p meshc --test e2e_m048_s03 -- --nocapture`

### If installer bytes or behavior change
Also rerun the existing installer proof rails:
- `bash scripts/verify-m034-s03.sh`
- `pwsh -NoProfile -File scripts/verify-m034-s03.ps1`

### Optional light smoke
If the planner wants a fast subcommand-presence check in addition to the dedicated rail:
- add/update a targeted help test in `compiler/meshc/tests/tooling_e2e.rs`

## Sources

- `compiler/meshc/src/main.rs`
- `compiler/meshpkg/src/main.rs`
- `compiler/mesh-pkg/src/lib.rs`
- `tools/install/install.sh`
- `tools/install/install.ps1`
- `website/docs/public/install.sh`
- `website/docs/public/install.ps1`
- `.github/workflows/release.yml`
- `scripts/verify-m034-s03.sh`
- `scripts/verify-m034-s03.ps1`
- `compiler/meshc/tests/tooling_e2e.rs`
- `compiler/meshc/tests/e2e_m048_s01.rs`
