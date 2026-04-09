# S03 Research — Release assets and installer truth

## Summary

- S03 primarily advances **R045** (release/CI truth for shipped surfaces). It also supports **R046** insofar as the shipped `meshpkg` binary must still be runnable after the documented install path, but it should **reuse** S01/S02’s live registry proof rather than re-run publish/install registry logic here.
- The current public install path is split and only partially honest:
  - `website/docs/public/install.sh` is the real docs-served Unix installer and already works on the current host against the latest public release.
  - `tools/install/install.sh` is a drifted repo-local copy that prints success but exits non-zero because of an `EXIT` trap / `set -u` bug.
  - `install.ps1` points at a nonexistent GitHub repo and only installs `meshc`.
  - `release.yml` does not publish any Windows `meshpkg` asset, so the Windows path cannot ever install both binaries today.
- The safest S03 shape is **reversible**: prove staged release artifacts plus the exact public installer scripts locally/in-CI first, then leave the irreversible public-tag replay for S05.

## Requirement focus

- **R045** — primary requirement for this slice. The release flow still uploads artifacts without proving the installer path and post-install binaries.
- **R046** — supporting requirement. The real package-manager path is already proven by S01/S02; S03 should only prove the released `meshpkg` artifact still runs after installer/release packaging.
- **R007** — already validated by S01. Reuse that proof surface; do not rebuild a second registry verifier here.

## Skills Discovered

- **Existing relevant skill:** `github-workflows`
- **Installed during research for downstream units:** `sickn33/antigravity-awesome-skills@powershell-windows` (`powershell-windows`)
- **Applied guidance:** from the `github-workflows` skill, **“No errors is not validation. Prove observable change.”** That applies directly here: `bash -n`, YAML parse checks, and “artifact uploaded” are insufficient. S03 needs observable installer smoke plus runnable-binary proof.

## Implementation Landscape

### 1) Release asset producer

**File:** `.github/workflows/release.yml`

Current shape:
- `build` packages `meshc` for:
  - `x86_64-apple-darwin`
  - `aarch64-apple-darwin`
  - `x86_64-unknown-linux-gnu`
  - `aarch64-unknown-linux-gnu`
  - `x86_64-unknown-linux-musl`
  - `x86_64-pc-windows-msvc`
- `build-meshpkg` packages `meshpkg` for only:
  - `x86_64-apple-darwin`
  - `aarch64-apple-darwin`
  - `x86_64-unknown-linux-gnu`
  - `aarch64-unknown-linux-gnu`
- `release` only downloads artifacts, generates `SHA256SUMS`, and publishes the GitHub release.

What is missing:
- no release-asset smoke step
- no installer smoke step
- no Windows `meshpkg` asset path at all

Important constraint from S02:
- `scripts/verify-m034-s02-workflows.sh:456` hard-codes the release workflow job set to exactly `build`, `build-meshpkg`, `authoritative-live-proof`, and `release`.
- `scripts/verify-m034-s02-workflows.sh:515` hard-codes `release.needs` to only those three upstream jobs.
- If S03 adds a `verify-release-assets` job, that verifier must be updated in the same task or the old contract check will fail immediately.

### 2) Installer surfaces

#### Public Unix installer

**File:** `website/docs/public/install.sh`

What it does now:
- hardcodes `REPO="snowdamiz/mesh-lang"`
- queries `https://api.github.com/repos/${REPO}/releases/latest`
- downloads release archives from `https://github.com/${REPO}/releases/download/...`
- downloads and verifies `SHA256SUMS`
- installs **both** `meshc` and `meshpkg`
- writes `~/.mesh/version`
- wires PATH via `~/.mesh/env`

Observed behavior from research:
- current-host smoke against latest public release succeeded
- installed `meshc` and `meshpkg` both ran
- a known-good minimal Mesh program using `println("hello")` built and ran with the installed `meshc`

#### Drifted repo-local Unix copy

**File:** `tools/install/install.sh`

This is not byte-identical to the public copy.

Observed behavior from research:
- `sh tools/install/install.sh --version 14.3` into a temp HOME printed success, installed both binaries, then exited non-zero with:
  - `tools/install/install.sh: line 430: _tmpdir: unbound variable`
- Root cause: the repo-local copy uses `trap 'rm -rf "$ _tmpdir"' EXIT`-style cleanup inside `install_binary()` while also running under `set -u`; the trap outlives the function scope and dereferences an unset local on shell exit.
- The public copy does **not** have this bug because it uses explicit cleanup instead of the lingering trap.

Planner implication:
- any repo-local verifier that targets `tools/install/install.sh` will fail today even though the public file works
- S03 needs either one canonical source plus sync enforcement, or explicit proof that both copies stay identical

#### Windows installer

**Files:**
- `website/docs/public/install.ps1`
- `tools/install/install.ps1`

Current problems:
- both hardcode `$Repo = "mesh-lang/mesh"`
- the real public repo is `snowdamiz/mesh-lang`; GitHub API spot-check of `https://api.github.com/repos/mesh-lang/mesh/releases/latest` returns **404**
- both scripts only install `meshc.exe`
- both scripts verify only the `meshc` archive against `SHA256SUMS`
- all help/output strings still describe this as a `meshc`-only installer

Planner implication:
- if S03 means “documented installer path yields usable `meshc` and `meshpkg` binaries” on Windows too, then the slice must add Windows `meshpkg` asset production and extend PS1 to install both binaries
- if Windows is intentionally `meshc`-only, the acceptance text/docs need to narrow that explicitly, because the slice title and demo text currently read as both binaries

### 3) Public docs truth surfaces

**Files:**
- `website/docs/docs/getting-started/index.md`
- `README.md`
- `tools/editors/vscode-mesh/README.md`

Current split:
- `website/docs/docs/getting-started/index.md:24` still says: “Today the verified install path is building `meshc` from source”
- `README.md:52-57` still presents quick-start install as source build
- `tools/editors/vscode-mesh/README.md:44` already tells users to run `curl -sSf https://meshlang.dev/install.sh | sh`

Planner implication:
- the public install story is inconsistent today
- S03 should update docs **after** the installer/release proof is real, not before

### 4) Smoke-fixture choice: do not assume `meshc init` is usable

**File:** `compiler/mesh-pkg/src/scaffold.rs`

Key finding:
- `compiler/mesh-pkg/src/scaffold.rs:45` still scaffolds:

```mesh
fn main() do
  IO.puts("Hello from Mesh!")
end
```

Observed behavior from research:
- current `cargo run -q -p meshc -- init ... && cargo run -q -p meshc -- build ...` fails immediately with `undefined variable: IO`
- the latest public release `meshc` behaves the same way
- a manual known-good fixture using `println("hello")` builds and runs fine

Planner implication:
- do **not** use `meshc init` as the release-asset smoke unless you intentionally scope a scaffold fix into S03
- the lower-risk path is a checked-in “known good” fixture for installer smoke

### 5) External state spot-check

Using the GitHub Releases API for the current public repo state:
- latest public release observed during research: `v14.3`
- asset list includes Unix `meshpkg` archives but **no** `meshpkg-v14.3-x86_64-pc-windows-msvc.zip`
- installed public binaries report `meshc 0.1.0` / `meshpkg 0.1.0`

Planner implication:
- if S03 wants strict tag-to-binary version truth, there is additional version-stamping work
- if S03 only needs “installable and runnable,” verifier checks should key off successful execution plus expected smoke behavior, not exact tag equality, unless the slice explicitly expands scope

## Recommendation

### Build/prove this first

**First build the canonical S03 verifier**, not the docs changes.

Recommended shape:
- add `scripts/verify-m034-s03.sh`
- keep it as the single slice-owned proof surface, matching the S01/S02 pattern
- give it a deterministic `.tmp/m034-s03/...` artifact root with per-phase logs
- make it prove staged release assets + installer truth **without requiring a public release**

Why first:
- it retires the biggest ambiguity in the slice: how to prove installer behavior without irreversible publication
- it gives a stable acceptance target for workflow and docs work
- it prevents the team from “updating docs to installer truth” before the installer path is actually proven

### Recommended implementation order

#### Task A — Canonical installer source + verifier seam

Likely files:
- `tools/install/install.sh`
- `website/docs/public/install.sh`
- `tools/install/install.ps1`
- `website/docs/public/install.ps1`
- new `scripts/verify-m034-s03.sh`

What to do:
- choose a canonical installer source and enforce sync with the public copy
- remove the Unix trap/unbound-variable bug from the repo-local copy
- add **test-only URL override hooks** so the verifier can stage local/mock release assets instead of hitting real GitHub releases
  - keep defaults pointed at the public URLs so user behavior does not change
- make the verifier exercise the **public** installer files (or enforce byte-equality first, then exercise the canonical source)

Why this seam first:
- it isolates the core installer truth work from CI wiring
- it makes later workflow smoke reusable instead of bespoke YAML logic

#### Task B — Release asset coverage + workflow smoke

Likely files:
- `.github/workflows/release.yml`
- `scripts/verify-m034-s02-workflows.sh`
- maybe `scripts/verify-m034-s03.sh` if CI calls it directly

What to do:
- extend `release.yml` to emit every asset the installer expects
  - most likely: add Windows `meshpkg` packaging if Windows installer is in-scope for both binaries
- add asset smoke before `Create Release`
- add installer smoke on platform-native runners where needed
- update the S02 workflow-contract verifier so the release workflow is still mechanically guarded after the new job(s)/needs are added

Important design note:
- if possible, keep raw asset “does the packaged binary run?” smoke inside existing build jobs
- use a separate job only where installer smoke needs both `meshc` and `meshpkg` artifacts together
- if a separate verification job is added, update `scripts/verify-m034-s02-workflows.sh` in the same task

#### Task C — Docs truth after proof

Likely files:
- `website/docs/docs/getting-started/index.md`
- `README.md`
- `tools/editors/vscode-mesh/README.md`

What to do:
- switch docs from “verified install path is source build” to the installer path **only after the verifier and workflow proof are green**
- keep docs platform-specific if Windows remains narrower than Unix
- make wording precise about what is actually verified (`meshc`, `meshpkg`, platform coverage, checksum path)

#### Optional Task D — Only if chosen explicitly

Likely files:
- `compiler/mesh-pkg/src/scaffold.rs`
- its tests

Only do this if the planner wants `meshc init` itself to be part of the public release smoke. Otherwise, use a known-good fixture and keep S03 scoped to release assets + installer truth.

## Natural task seams

- **Task A: installer canonicalization + local verifier hooks**
  - mostly shell / PowerShell / proof-script work
- **Task B: release asset coverage + workflow proof**
  - mostly GitHub Actions / artifact-shape work
- **Task C: docs truth alignment**
  - mostly documentation updates after proof exists
- **Optional Task D: scaffold fix**
  - Rust change, adjacent but not required if smoke uses a fixture

These are genuinely separable. Task A unblocks B and C. Task D is optional and should not block the slice unless the planner explicitly wants `meshc init` as the smoke contract.

## Verification plan

### Minimum repo-local checks

- `diff -u tools/install/install.sh website/docs/public/install.sh`
- `diff -u tools/install/install.ps1 website/docs/public/install.ps1`
- `bash -n tools/install/install.sh`
- `ruby -e 'require "yaml"; YAML.load_file(".github/workflows/release.yml")'`

### Canonical slice proof to add

- `bash scripts/verify-m034-s03.sh`

That verifier should prove, at minimum:
- release-style archives plus `SHA256SUMS` exist in the expected names
- the documented installer path installs `meshc`
- the documented installer path installs `meshpkg`
- installed binaries are runnable
- a known-good Mesh hello fixture builds and runs with installed `meshc`
- checksum verification uses the same `SHA256SUMS` contract the release workflow publishes

### Workflow contract

- updated `bash scripts/verify-m034-s02-workflows.sh`

### Platform-specific note

- Unix installer smoke can run locally and in Actions
- Windows installer behavior needs a PowerShell-capable runner; the current scout environment does **not** have `pwsh`

## Concrete evidence from research

- `sh website/docs/public/install.sh --version 14.3` into a temp HOME succeeded.
- From that installed public path:
  - `meshc --version` ran
  - `meshpkg --version` ran
  - a manual `println("hello")` fixture built and ran successfully
- `sh tools/install/install.sh --version 14.3` into a temp HOME printed success but exited 1 with `_tmpdir: unbound variable`.
- `cargo run -q -p meshc -- init ... && cargo run -q -p meshc -- build ...` fails today because scaffolded `main.mpl` uses `IO.puts`.
- GitHub Releases API spot-check during research:
  - `snowdamiz/mesh-lang` latest release exists and currently lacks a Windows `meshpkg` asset
  - `mesh-lang/mesh` latest release endpoint returns 404, matching the PS1 hardcoded repo bug

## Don’t Hand-Roll

- Do **not** copy installer assertions into YAML steps. Keep one repo-local S03 verifier and have workflow call it.
- Do **not** trust syntax-only checks. The Unix repo-local installer passes parse/syntax checks but still exits non-zero at runtime.
- Do **not** use `meshc init` as the smoke fixture unless you intentionally fix `compiler/mesh-pkg/src/scaffold.rs` in-scope.
- Do **not** treat `tools/install/*` as “close enough” to the public installer path; the public docs site serves `website/docs/public/*`, and those copies are already drifting.

## Sources

### Local files
- `.github/workflows/release.yml`
- `scripts/verify-m034-s02-workflows.sh`
- `tools/install/install.sh`
- `website/docs/public/install.sh`
- `tools/install/install.ps1`
- `website/docs/public/install.ps1`
- `website/docs/docs/getting-started/index.md`
- `README.md`
- `tools/editors/vscode-mesh/README.md`
- `compiler/mesh-pkg/src/scaffold.rs`

### External spot-check
- `https://api.github.com/repos/snowdamiz/mesh-lang/releases/latest`
- `https://api.github.com/repos/mesh-lang/mesh/releases/latest`
