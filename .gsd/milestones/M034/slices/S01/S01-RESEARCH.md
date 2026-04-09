# S01 Research — Real registry publish/install proof

## Summary
- Direct target: **R007**. This slice also advances the package-manager truth part of **R045/R046** from the milestone context, and lightly supports the public package visibility side of **R047**.
- The repo already has real production code for publish/install, registry metadata/download, and the packages website, but it does **not** have a named end-to-end verifier for the real registry path.
- Live control surfaces are public right now: the registry list/metadata/download/search APIs work, and the packages website reflects registry data immediately on request.
- The real end-to-end install/build path is currently blocked by a concrete bug: **scoped package installs do not compile after install**.

## Skills Discovered
- Loaded existing skill: **github-workflows**.
  - Relevant rule carried forward into planning: **“No errors is not validation. Prove observable change.”**
- Already-installed relevant skills: **github-workflows**, **rust-best-practices**.
- Searched for missing directly-relevant skills:
  - `Fly.io` → `thinkfleetai/thinkfleet-engine@flyio-cli-public` (14 installs)
  - `SvelteKit` → highest-install match `spences10/svelte-skills-kit@sveltekit-structure` (334 installs)
- No extra skills installed. S01 is centered on existing Rust CLI/registry code and live HTTP proof surfaces, not Fly CLI automation or SvelteKit internals.

## Recommendation
1. **Fix scoped install/build truth first.** Without that, S01 cannot honestly claim that a real scoped package can be installed and consumed from the real registry path.
2. **Create one canonical repo-local verifier before touching workflow YAML.** Use `meshpkg --json` plus direct HTTP checks against the real registry and packages website. S02 can wire that verifier into CI later.
3. **Decide explicitly whether README/page truth is in scope for S01.** The registry and website support README metadata, but `meshpkg publish` currently never packs `README.md`, so a README-bearing proof package will still show `readme: null`.
4. **Use one dedicated proof package name plus unique versions per release/run.** Registry versions are immutable, there is no delete route, and publish tokens are owner-scoped.

## Implementation Landscape
- `compiler/meshpkg/src/main.rs` — CLI entrypoints, JSON mode, default production registry URL.
- `compiler/meshpkg/src/publish.rs` — tarball creation, SHA-256 calculation, authenticated upload, JSON publish output.
- `compiler/meshpkg/src/install.rs` — named install, install-all from `mesh.toml`, checksum verification, extract path, lockfile write.
- `compiler/mesh-pkg/src/manifest.rs` — dependency syntax; scoped registry dependency keys must be valid TOML keys.
- `compiler/mesh-pkg/src/lockfile.rs` — lockfile schema; registry entries carry `version`, `source`, `revision`, and `sha256`.
- `compiler/meshc/src/discovery.rs` — scans `.mesh/packages/*` and currently assumes each immediate child is a package root.
- `registry/src/routes/publish.rs` — auth, owner namespace enforcement, SHA verification, DB insert, R2 upload, README extraction.
- `registry/src/routes/metadata.rs` — latest/version metadata surface used by `meshpkg install` and package pages.
- `registry/src/routes/download.rs` — download stream and download counter increment.
- `registry/src/routes/auth.rs` — GitHub OAuth flow plus dashboard token creation; there is no headless token-mint path.
- `packages-website/src/routes/+page.server.js` and `packages-website/src/routes/packages/[name]/+page.server.js` — live SSR fetches from the registry API on each request.
- `.github/workflows/release.yml` — builds `meshc` and `meshpkg`, publishes release assets, but does not run the real registry path.
- `.github/workflows/deploy-services.yml` — deploys registry/packages website and only curls the list/homepage after deploy.

## Findings

### 1. There is no current S01 verifier surface
- No existing script or test exercises **publish → metadata → download → install → consumer build** against the real registry.
- Current workflow truth stops at artifact builds and homepage/list cURL checks.
- S02 should wire a verifier into CI; S01 first needs the verifier to exist.

### 2. Live registry control surfaces are already public and directly scriptable
Control package observed during research: `snowdamiz/mesh-slug@1.0.0`

What works live right now:
- `GET https://api.packages.meshlang.dev/api/v1/packages` returns the live package list.
- `GET /api/v1/packages/snowdamiz/mesh-slug` returns latest version + sha + download count.
- `GET /api/v1/packages/snowdamiz/mesh-slug/1.0.0/download` returns a tarball whose actual SHA-256 matches metadata (`1405b356932535959314418c9347e7e7b21997d6bd6356904abeb02d5695557e` during research).
- `GET /api/v1/packages?search=slug` returns the package via search.
- `https://packages.meshlang.dev` and `/packages/snowdamiz/mesh-slug` reflect registry data on the next request; there is no separate indexing or deploy lag in the current code path.

### 3. Real install/build currently fails for scoped package names
Repro shape:
- temp consumer `mesh.toml` with:
  - `[package] name = "research/consumer"`
  - `[dependencies] "snowdamiz/mesh-slug" = "1.0.0"`
- temp `main.mpl` with `from Slug import slugify`
- install with:
  - `(cd <tmp> && cargo run --manifest-path ../../compiler/meshpkg/Cargo.toml --quiet -- install --registry https://api.packages.meshlang.dev --json)`
- then build with:
  - `cargo run -q -p meshc -- build <tmp>`

Observed result today:
- install succeeds and writes `mesh.lock`
- build fails with `module 'Slug' not found`
- diagnostics also show the package’s own `main.mpl` being compiled as a normal module

Root cause:
- `compiler/meshpkg/src/install.rs` extracts scoped packages to `.mesh/packages/<owner>/<package>@<version>`.
- `compiler/meshc/src/discovery.rs` iterates one level under `.mesh/packages` and treats each immediate child as a package root.
- For scoped names, the immediate child is the **owner directory**, so relative paths become `mesh-slug@1.0.0/main.mpl` and `mesh-slug@1.0.0/slug.mpl`.
- That means package-root `main.mpl` is no longer recognized as a root entry, and imports like `from Slug ...` stop resolving.

Control proving the root cause:
- After manually moving the installed directory to a **flat immediate child** under `.mesh/packages` (`.mesh/packages/snowdamiz__mesh-slug@1.0.0`) and rerunning `meshc build`, the consumer builds successfully.
- Only an upstream package warning (`W0001 redundant match arm`) remains.

Planner implication:
- Natural fix seam is either:
  1. teach `meshc` discovery to treat **leaf directories containing `mesh.toml`** as package roots under `.mesh/packages`, or
  2. flatten/sanitize the install directory name so each installed package is an immediate child of `.mesh/packages`.
- Option 1 is more robust to raw scoped names; option 2 is a smaller change. The control shows either should unblock S01.

### 4. Scoped dependencies need quoted TOML keys
- Bare TOML like `snowdamiz/mesh-slug = "1.0.0"` fails parse before install.
- Quoted key `"snowdamiz/mesh-slug" = "1.0.0"` works.
- Current docs and website examples do not call out this scoped-key requirement.
- Any S01 verifier that installs from manifest must use quoted keys, and docs should stop implying otherwise.

### 5. `meshpkg install <name>` does not modify `mesh.toml`
- `install_named` is commented as “Install a single named package and add it to mesh.toml.”
- Actual implementation downloads, extracts, and updates `mesh.lock`; it does **not** edit the manifest.
- This matters for R007/reproducibility: named install alone is not a full declared-dependency workflow unless S01 either changes behavior or makes the contract explicit.

### 6. README/package-page truth is partially wired but not publishable
- `registry/src/routes/publish.rs` extracts `README.md` from the uploaded tarball.
- `registry/src/routes/metadata.rs` returns `readme`.
- `packages-website/src/routes/packages/[name]/+page.svelte` renders `readme` or “No README provided.”
- `compiler/meshpkg/src/publish.rs` never adds `README.md` to the tarball; it currently packs `mesh.toml`, root `*.mpl` (excluding `*.test.mpl`), and `src/`.
- Result: current package pages can only show a README if some other publish path uploaded it. The live control package shows `readme: null` and the website renders “No README provided.”
- If S01’s public proof should include README truth, this is a real code task, not just a docs tweak.

### 7. Registry state ordering still allows “metadata green, artifact red”
- `registry/src/routes/publish.rs` commits DB metadata **before** uploading the blob to R2. If the upload fails after the insert, the version row can exist with no downloadable artifact.
- `registry/src/routes/download.rs` increments download counters **before** fetching the blob. A failed object fetch can still increment counts.
- Because S01 is supposed to prove real truth, the verifier should never trust counters alone. It needs actual download hash/install/build checks.
- Decide in planning whether to harden these route orderings inside S01 or at least capture them as explicit failure modes in the verifier.

### 8. Auth / secret model is CI-friendly on consume, not on mint
- Good: `meshpkg login --token <token>` is noninteractive and already works for automation.
- Constraint: the only token creation flow in-tree is GitHub OAuth/session-based (`/auth/github`, `/dashboard/tokens`). There is no machine-to-machine token mint path.
- That means the S01 verifier will need a **pre-provisioned publish token secret** tied to the owner namespace. It cannot self-bootstrap a token inside CI.

### 9. Live website proof is cheap once publish works
- The packages website is just server-side fetches against the live registry API on each request.
- S01 can verify public discoverability with ordinary HTTP requests:
  - registry list/search endpoints
  - packages homepage
  - package detail page
- No website build/deploy step is needed for this slice’s public visibility check.

## Natural Seams / Taskable Units
1. **Scoped-package correctness**
   - Files: `compiler/meshpkg/src/install.rs`, `compiler/meshc/src/discovery.rs`
   - Goal: make real scoped installs importable/buildable without manual filesystem surgery.
   - First proof to rerun: install real `snowdamiz/mesh-slug`, then `meshc build` a consumer.

2. **Canonical real-registry verifier**
   - Likely files: new `scripts/verify-m034-s01.sh` or similar verifier surface, plus any supporting docs/reference notes.
   - Goal: publish a unique proof version to the real registry, recheck metadata SHA, actual download SHA, install outcome, lockfile SHA, consumer build, duplicate publish rejection, and public website visibility.
   - Use `meshpkg --json` and fixed repo-local temp dirs. Avoid parsing spinner text.

3. **Public package metadata truth**
   - Files: `compiler/meshpkg/src/publish.rs`, `registry/src/routes/metadata.rs`, possibly package website docs/pages.
   - Goal: decide whether README truth is in-scope for S01. If yes, pack `README.md` and verify it appears on the live package page after publish.

4. **Registry state-order hardening**
   - Files: `registry/src/routes/publish.rs`, `registry/src/routes/download.rs`
   - Goal: eliminate or at least surface partial-state failures where DB rows/counters advance before artifact truth is confirmed.

5. **Docs / operator contract**
   - Files: `website/docs/docs/tooling/index.md`, maybe `packages-website/src/routes/publish/+page.svelte`
   - Goal: document quoted scoped dependency keys, actual named-install behavior, and the release-scoped proof package contract.

## What to Build / Prove First
1. **Fix scoped install/build truth first.** It is a hard blocker: the real registry install path currently cannot satisfy the slice claim for the required scoped naming contract.
2. **Then build the live verifier script.** Once install/build works, use the real registry/token to create the named proof surface S02 can later call.
3. **Only then decide whether to expand to README/page truth.** That is valuable public-surface honesty, but it is secondary to making install/build actually work.
4. **Workflow YAML comes later.** Per the loaded `github-workflows` skill: green jobs are not validation; first create a verifier that proves observable registry change, then wire it in during S02.

## Verification Targets
Authoritative repros/control commands from research:

- **Live registry list**
  - `curl -fsSL https://api.packages.meshlang.dev/api/v1/packages`

- **Live metadata and version list**
  - `curl -fsSL https://api.packages.meshlang.dev/api/v1/packages/snowdamiz/mesh-slug`
  - `curl -fsSL https://api.packages.meshlang.dev/api/v1/packages/snowdamiz/mesh-slug/versions`

- **Live tarball integrity**
  - `curl -fsSL https://api.packages.meshlang.dev/api/v1/packages/snowdamiz/mesh-slug/1.0.0/download -o /tmp/mesh-slug-s01.tar.gz`
  - `shasum -a 256 /tmp/mesh-slug-s01.tar.gz`
  - `tar tzf /tmp/mesh-slug-s01.tar.gz`

- **Current failure: scoped install/build**
  - temp `mesh.toml` with quoted dependency key
  - `(cd <tmp> && cargo run --manifest-path ../../compiler/meshpkg/Cargo.toml --quiet -- install --registry https://api.packages.meshlang.dev --json)`
  - `cargo run -q -p meshc -- build <tmp>`
  - expected today: install succeeds, build fails with `module not found`

- **Root-cause control**
  - move installed package to a flat immediate child under `.mesh/packages`
  - rerun `cargo run -q -p meshc -- build <tmp>`
  - expected today: build succeeds (only upstream warning remains)

Recommended final S01 acceptance bundle after implementation:
- `meshpkg login --token ...`
- `meshpkg publish --json`
- `curl` registry metadata/version/search endpoints
- `curl` actual download + `shasum`
- `meshpkg install --json` into a temp consumer
- inspect `mesh.lock`
- `cargo run -q -p meshc -- build <tmp-consumer>`
- duplicate publish attempt expecting HTTP 409 / immutable-version rejection
- `curl` packages website exact package detail page (and optionally search) for public visibility

## Risks / Open Questions
- **Naming contract:** use a fixed proof package name plus unique version per release/run, or another bounded scheme. Avoid many one-off package names; there is no delete route.
- **Version format:** the registry currently treats version as an opaque string. If the proof wants tag/run metadata in the version, define that contract explicitly instead of assuming semver validation exists.
- **README scope:** should S01 require a live page with rendered README, or is name/version/description/download truth enough for this slice?
- **Manifest contract:** should named install start editing `mesh.toml`, or should docs be corrected to say named install is fetch+lockfile only?
- **Registry hardening scope:** are publish/download side-effect ordering fixes part of S01, or acceptable as documented failure modes until later?

## Forward Intelligence
- The real publish/install proof is blocked by a **concrete code bug today**, not by missing infrastructure.
- Do **not** start with workflow automation. Fix the scoped install/discovery boundary first, then build the verifier.
- Do **not** use homepage package counts as the main proof. They will drift over time. Prefer exact package search/detail checks.
- Do **not** treat website visibility as a deploy problem. The current packages site is a thin live view over the registry API.
- If the planner wants README/page truth, it must schedule tarball-content work explicitly; the current publish path cannot satisfy that contract.
- Use a verifier script with JSON outputs and direct HTTP checks. That keeps S02’s later workflow wiring honest and small.
