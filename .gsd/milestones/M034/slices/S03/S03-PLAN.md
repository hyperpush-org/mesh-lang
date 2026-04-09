# S03: Release assets and installer truth

**Goal:** Prove staged Mesh release assets and the documented install scripts actually yield runnable `meshc` and `meshpkg` binaries, so `release.yml` stops equating artifact upload with installer truth.
**Demo:** After this: Released `meshc` and `meshpkg` artifacts are proven installable and runnable through the documented installer path instead of only being uploaded.

## Tasks
- [x] **T01: Canonicalized public installers, added staged release proof hooks, and landed staged installer smoke verifiers.** — Make the docs-served installer copies the real source of truth before touching CI. The public Unix script already works while the repo-local copy drifted, and the Windows script still points at the wrong repo and only installs `meshc`. Canonicalize on `website/docs/public/install.{sh,ps1}`, keep `tools/install/*` byte-identical, add test-only release URL override hooks that default to GitHub, and land staged-release verifiers plus a known-good hello fixture so S03 can prove installer behavior without waiting for a live tag.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Local staged release server / GitHub release endpoints | Fail the verifier at the exact download or checksum phase and leave the staged root plus requested URL in logs. | Stop without retry loops and report which asset URL stalled. | Treat missing `tag_name`, missing archive entries, or missing asset files as proof failure. |
| Installer checksum contract | Abort install and keep the mismatched archive plus expected hash visible; never continue with an unchecked binary. | N/A | Treat malformed `SHA256SUMS` or missing archive lines as verifier failures instead of warnings in staged-proof mode. |
| Public vs repo installer copies | Refuse proof if `tools/install/*` drifts from `website/docs/public/*` so the docs-served contract stays canonical. | N/A | Treat the wrong repo slug or a Windows installer that still omits `meshpkg` as contract drift. |

## Load Profile

- **Shared resources**: temp staging directories under `.tmp/m034-s03/`, downloaded archives, checksum manifests, and isolated HOME/PATH rewrites for smoke runs.
- **Per-operation cost**: release-style archive staging, two installer downloads/extracts per platform, and one hello build/run with the installed `meshc`.
- **10x breakpoint**: repeated archive staging and checksum scans would dominate first, so the verifier should prepare the staged release root once per run and reuse it across checks.

## Negative Tests

- **Malformed inputs**: missing `meshpkg` archive, wrong repo metadata, malformed `SHA256SUMS`, or drift between public and repo-local installer copies.
- **Error paths**: checksum mismatch, missing binary inside an archive, or installer failure after PATH/HOME setup must stop the proof and keep the failing phase log.
- **Boundary conditions**: Unix and Windows both install `meshc` plus `meshpkg`, and the hello smoke uses a known-good `println("hello")` fixture instead of the currently broken `meshc init` scaffold.

## Steps

1. Make `website/docs/public/install.sh` and `website/docs/public/install.ps1` the canonical installer sources, sync `tools/install/install.sh` and `tools/install/install.ps1` to them, and keep both pairs byte-identical.
2. Add test-only override hooks for the release API and asset base URLs that default to the current public GitHub endpoints so ordinary user installs stay unchanged while verifiers can point at staged local assets.
3. Extend the PowerShell installer to use the real `snowdamiz/mesh-lang` repo and install/checksum-verify both `meshc.exe` and `meshpkg.exe`; preserve Unix installation of both binaries and fix any remaining cleanup or drift bugs only in the canonical public source.
4. Add `scripts/verify-m034-s03.sh`, `scripts/verify-m034-s03.ps1`, and a checked-in hello fixture under `scripts/fixtures/m034-s03-installer-smoke/` that stage release-style archives plus `SHA256SUMS`, run the documented installer scripts against that staged source, verify `meshc --version` plus `meshpkg --version`, and build/run the known-good hello program.

## Must-Haves

- [ ] The public installer copies are canonical and the repo-local copies are byte-identical.
- [ ] Both installers accept test-only staged-release overrides without changing default public behavior.
- [ ] The Windows installer points at `snowdamiz/mesh-lang` and installs both `meshc` and `meshpkg`.
- [ ] Repo-local staged verifiers prove checksum, install, runnable binaries, and hello build/run truth.

  - Estimate: 2.5h
  - Files: website/docs/public/install.sh, tools/install/install.sh, website/docs/public/install.ps1, tools/install/install.ps1, scripts/verify-m034-s03.sh, scripts/verify-m034-s03.ps1, scripts/fixtures/m034-s03-installer-smoke/mesh.toml, scripts/fixtures/m034-s03-installer-smoke/main.mpl
  - Verify: - `bash -n tools/install/install.sh`
- `diff -u tools/install/install.sh website/docs/public/install.sh`
- `diff -u tools/install/install.ps1 website/docs/public/install.ps1`
- `bash scripts/verify-m034-s03.sh`
- [x] **T02: Gated release publication on staged installer smoke and shipped Windows meshpkg asset coverage.** — Once the installer and staged verifier seam exists, make `release.yml` ship every asset those installers expect and prove them before publication. Extend `build-meshpkg` to cover Windows, add platform-native release-asset smoke that reuses the repo-local S03 verifier scripts instead of inline YAML assertions, and update the S02 workflow contract verifier so the release graph remains mechanically guarded after the new job and dependency edges land.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Release artifact build matrix | Fail the platform build job and block smoke/publication; do not allow partial asset sets to proceed. | Let the job fail visibly with the target name and missing artifact path in logs. | Treat a missing Windows `meshpkg` archive or wrong archive extension as contract drift. |
| Installer smoke jobs | Keep `Create Release` blocked if any platform verifier fails and surface the failing verifier phase plus artifact pair in job logs. | Stop the smoke job and leave the staged asset directory plus verifier output visible. | Treat missing verifier scripts or inlined installer assertions as drift; the workflow must call the repo-local proof scripts. |
| S02 release contract verifier | Reject job-name, needs-graph, permission, and asset-matrix drift locally before CI. | N/A | Treat missing `verify-release-assets` wiring or stale expected jobs as release-contract failure. |

## Load Profile

- **Shared resources**: GitHub runner minutes, cross-platform build matrices, artifact upload/download storage, and staged smoke temp directories.
- **Per-operation cost**: one extra Windows `meshpkg` build/package plus one installer smoke run per platform before release publication.
- **10x breakpoint**: artifact fan-out and queued release smoke would bottleneck first, so the workflow should reuse built artifacts and avoid recompiling inside the smoke jobs.

## Negative Tests

- **Malformed inputs**: missing Windows `meshpkg` artifact, smoke job missing one binary archive, or a release workflow that still publishes without the smoke gate.
- **Error paths**: checksum mismatch, installer-verifier failure, or a broken `needs` graph must keep `Create Release` blocked.
- **Boundary conditions**: the smoke jobs reuse `scripts/verify-m034-s03.sh` and `scripts/verify-m034-s03.ps1` instead of duplicating installer assertions inside YAML.

## Steps

1. Extend `.github/workflows/release.yml` so `build-meshpkg` emits a Windows `meshpkg-v<version>-x86_64-pc-windows-msvc.zip` artifact alongside the existing Unix archives, using platform-appropriate packaging for Windows.
2. Add a dedicated `verify-release-assets` job, or an equivalent platform-native smoke graph, that downloads the built artifacts, stages a local release directory plus `SHA256SUMS`, and runs `bash scripts/verify-m034-s03.sh` on Unix runners and `pwsh -File scripts/verify-m034-s03.ps1` on Windows.
3. Make `Create Release` depend on that installer-smoke job in addition to `build`, `build-meshpkg`, and `authoritative-live-proof`, so uploaded artifacts are already proven installable and runnable.
4. Extend `scripts/verify-m034-s02-workflows.sh` release-mode checks to require the new smoke job, the updated `release.needs` graph, Windows `meshpkg` asset production, and reuse of the repo-local S03 verifier scripts.

## Must-Haves

- [ ] The release workflow produces every asset the documented installers expect, including Windows `meshpkg`.
- [ ] Installer smoke runs from staged release assets before `Create Release` publishes anything.
- [ ] Repo-local S03 verifier scripts, not inline YAML logic, own installer truth in the workflow.
- [ ] The S02 workflow contract verifier stays authoritative after the new release job graph lands.

  - Estimate: 2h
  - Files: .github/workflows/release.yml, scripts/verify-m034-s02-workflows.sh, scripts/verify-m034-s03.sh, scripts/verify-m034-s03.ps1
  - Verify: - `bash scripts/verify-m034-s02-workflows.sh release`
- `ruby -e 'require "yaml"; YAML.load_file(".github/workflows/release.yml")'`
- `rg -n 'verify-release-assets|x86_64-pc-windows-msvc' .github/workflows/release.yml`
- [x] **T03: Rewrote public install docs to use the verified installer path for meshc and meshpkg, with source builds kept as an explicit alternative.** — Only after the staged verifiers and release workflow smoke are in place should the public story stop telling users that building from source is the only verified path. Rewrite the top-level quick start, getting-started guide, tooling docs, and editor README around the documented installer path, and phrase platform/binary coverage precisely so the docs claim exactly what S03 now proves.

## Steps

1. Update `README.md` so the quick start uses `https://meshlang.dev/install.sh` and `https://meshlang.dev/install.ps1` as the verified install path, while keeping source-build instructions only as an explicit alternative or contributor path.
2. Update `website/docs/docs/getting-started/index.md` to replace the old source-build truth claim with installer-based instructions and verification commands for both `meshc` and `meshpkg`, using wording that matches the actual platform coverage proven by T01 and T02.
3. Update `website/docs/docs/tooling/index.md` and `tools/editors/vscode-mesh/README.md` so tooling/editor docs reference the same installer truth and do not over- or under-claim what the installer provides.

## Must-Haves

- [ ] No top-level or getting-started docs still claim that building `meshc` from source is the only verified install path.
- [ ] Public docs say plainly that the installer path installs `meshc` and `meshpkg` on the platforms S03 proves.
- [ ] Any retained source-build instructions are clearly labeled as an alternative or contributor workflow, not the authoritative public install proof.

  - Estimate: 1h
  - Files: README.md, website/docs/docs/getting-started/index.md, website/docs/docs/tooling/index.md, tools/editors/vscode-mesh/README.md
  - Verify: - `rg -n 'meshlang.dev/install.sh|meshlang.dev/install.ps1' README.md website/docs/docs/getting-started/index.md website/docs/docs/tooling/index.md tools/editors/vscode-mesh/README.md`
- `! rg -n 'Today the verified install path is building `meshc` from source' website/docs/docs/getting-started/index.md`
- `rg -n 'meshc --version|meshpkg --version' README.md website/docs/docs/getting-started/index.md website/docs/docs/tooling/index.md`
