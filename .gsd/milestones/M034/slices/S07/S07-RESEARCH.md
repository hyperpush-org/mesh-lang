# S07 Research — Public surface freshness and final assembly replay

## Summary

- **The canonical acceptance entrypoint is already `scripts/verify-m034-s05.sh`.** It runs a fixed phase chain: `prereq-sweep -> candidate-tags -> s05-workflows -> docs-build -> docs-truth-local -> docs-truth-built -> s02-workflows -> s03-installer -> s04-extension -> s04-workflows -> remote-evidence -> public-http -> s01-live-proof`.
- **Current local code is ahead of hosted truth.** `git status --short --branch` shows `main...origin/main [ahead 134]`; the best retained hosted snapshot (`.tmp/m034-s06/evidence/closeout-20260326-1525/manifest.json`) still reports `remote-evidence` red because remote `main` is stale and candidate tag push runs do not exist.
- **Current live public state is split:** `packages.meshlang.dev` detail/search/API already satisfy the S05 markers, but `meshlang.dev` installer/docs surfaces are stale. Live `install.sh` / `install.ps1` return 200 with the expected content types, but both bodies differ from `website/docs/public/install.*`; the docs pages return 200 but still expose older meshc-only/source-build text and omit the newer S05 runbook markers.
- **The root cause of not reaching `public-http` from the canonical replay is still hosted rollout, not local docs source.** S06 transport evidence shows the HTTPS push path times out on `git-receive-pack` (`HTTP 408`) even after forcing HTTP/1.1 and `http.postBuffer=1073741824`.
- **Important structural gap:** the hosted workflows still validate a weaker public contract than `run_public_http_truth()` does. If S07 wants “public surface freshness” to be included in CI/CD rather than only in the repo-local S05 replay, the deploy workflows need stronger checks.

## Requirements / Traceability

- `REQUIREMENTS.md` currently has **no active M034 requirement rows**. The milestone context still references **R045 / R046 / R047**, but those IDs are not present in the live requirements registry yet.
- Practically, S07 is consuming and re-proving the already-validated **R007** package truth path and advancing the deferred package/release maturity story described by **R021**, but there is a traceability gap the planner should keep in mind.
- Recommendation: do not burn executor time re-exploring requirements during planning; treat this slice as a release-proof/public-freshness closeout slice and surface the requirements gap explicitly if any closeout artifact updates it.

## Skills Discovered

- **Used:** `github-workflows`
  - Relevant rule: **“No errors is not validation. Prove observable change.”** That matches this slice exactly: a green workflow run is insufficient unless `remote-runs.json`, live HTTP bodies, and final replay artifacts prove the public surface changed.
- **Used:** `vitepress`
  - Relevant rule: **`public/` assets are served as-is.** In this repo that means `website/docs/public/install.sh` and `website/docs/public/install.ps1` are the canonical bytes that GitHub Pages should publish.
- **Installed for downstream units:** `thinkfleetai/thinkfleet-engine@flyio-cli-public`
  - Directly relevant because `deploy-services.yml` deploys the registry and packages site to Fly.io.

## Implementation Landscape

### Canonical files and what they own

- `scripts/verify-m034-s05.sh`
  - Canonical S05/S07 assembly verifier.
  - Owns candidate tag derivation, hosted GitHub Actions evidence capture, exact public HTTP assertions, and final handoff into the real S01 live publish/install proof.
  - `public-http` is implemented here; there is no separate public-surface verifier.
- `scripts/verify-m034-s06-remote-evidence.sh`
  - Wrapper that runs S05 with `VERIFY_M034_S05_STOP_AFTER=remote-evidence` and archives the full `.tmp/m034-s05/verify/` bundle under `.tmp/m034-s06/evidence/<label>/`.
  - This is the fastest truthful way to refresh hosted evidence without rerunning the live publish/install phase.
- `scripts/tests/verify-m034-s05-contract.test.mjs`
  - Thin contract coverage for candidate-tag independence, README/tooling runbook strings, and tag trigger separation.
  - **Does not cover the detailed `public-http` marker set.**
- `scripts/tests/verify-m034-s06-contract.test.mjs`
  - Pins the stop-after boundary, archive helper semantics, and reusable extension-proof polling via `publish-extension.yml`.
- `.github/workflows/deploy.yml`
  - GitHub Pages build/deploy workflow.
  - Verifies built docs/installers before upload, but only a subset of the full S05 public contract.
- `.github/workflows/deploy-services.yml`
  - Fly.io deploy for `registry/` and `packages-website/` plus post-deploy health checks.
  - Its live checks are also weaker than S05 `public-http`.
- `.github/workflows/authoritative-verification.yml`
  - Hosted wrapper over the reusable S01 live proof on same-repo PRs, `main`, manual, and schedule.
- `.github/workflows/authoritative-live-proof.yml`
  - Reusable workflow that actually runs `bash scripts/verify-m034-s01.sh` with publish secrets.
- `.github/workflows/release.yml`
  - Binary release workflow. Requires the authoritative live proof and release-asset smoke before `Create Release`.
- `.github/workflows/extension-release-proof.yml`
  - Reusable proof workflow for the VS Code extension; `remote-evidence` intentionally queries the caller workflow (`publish-extension.yml`) instead of pretending this workflow has a standalone push surface.
- `.github/workflows/publish-extension.yml`
  - Tag-triggered caller workflow for extension proof + publish.
- `website/docs/public/install.sh`
- `website/docs/public/install.ps1`
  - Canonical public installer sources.
  - These are copied verbatim into the VitePress dist output and should exactly match the public `meshlang.dev/install.*` endpoints.
- `tools/install/install.sh`
- `tools/install/install.ps1`
  - Repo-local copies. Currently still byte-identical to the `website/docs/public/*` sources.
- `website/docs/docs/getting-started/index.md`
- `website/docs/docs/tooling/index.md`
  - Canonical public docs pages that `public-http` checks against exact text markers.
- `.tmp/m034-s06/evidence/closeout-20260326-1525/manifest.json`
- `.tmp/m034-s06/evidence/closeout-20260326-1525/remote-runs.json`
  - Best current hosted-state snapshot.
- `.tmp/m034-s06/push-main.stderr`
- `.tmp/m034-s06/transport-recovery/attempts.log`
- `.tmp/m034-s06/transport-recovery/03-http11-postbuffer-1g.stderr`
  - Root-cause evidence for the current rollout blocker.

### Canonical candidate identity

From local source of truth and current contract tests:

- Binary candidate tag: **`v0.1.0`**
- Extension candidate tag: **`ext-v0.3.0`**

Those values come from:

- `compiler/meshc/Cargo.toml`
- `compiler/meshpkg/Cargo.toml`
- `tools/editors/vscode-mesh/package.json`

## Key Findings

### 1. `scripts/verify-m034-s05.sh` is already the exact S07 replay surface

This is not a “find the right entrypoint” slice. The repo already settled that.

Key mechanics inside `scripts/verify-m034-s05.sh`:

- `run_candidate_tags()` writes `.tmp/m034-s05/verify/candidate-tags.json`
- `run_remote_evidence()` writes `.tmp/m034-s05/verify/remote-runs.json`
- `run_public_http_truth()` does:
  - exact diff of live `https://meshlang.dev/install.sh` vs `website/docs/public/install.sh`
  - exact diff of live `https://meshlang.dev/install.ps1` vs `website/docs/public/install.ps1`
  - exact marker checks on live `docs/getting-started/` and `docs/tooling/`
  - exact package detail/search/registry checks for the S01 proof package
- `s01-live-proof` shells out to `bash scripts/verify-m034-s01.sh` with `.env` loaded

The planner should treat S07 as **making this existing replay finish green**, not inventing a second acceptance script.

### 2. The current hard stop is still `remote-evidence`

Current retained truth:

- `.tmp/m034-s05/verify/failed-phase.txt` -> `remote-evidence`
- `.tmp/m034-s05/verify/current-phase.txt` -> `remote-evidence`
- `.tmp/m034-s05/verify/public-http.log` is still **0 bytes** because the phase never started
- `.tmp/m034-s06/evidence/closeout-20260326-1525/manifest.json` shows every prereq phase through `s04-workflows` passing before `remote-evidence` failed

The latest archived hosted failures are:

- `deploy.yml` latest run on remote `main` is green **but stale** and missing the required `build: Verify public docs contract` step
- `authoritative-verification.yml` is still missing on the remote default branch
- `release.yml` has no push run on `v0.1.0`
- `deploy-services.yml` has no push run on `v0.1.0`
- `publish-extension.yml` / reusable extension proof have no push run on `ext-v0.3.0`

That means any S07 plan must start from the fact that **no local code change alone can move the canonical replay past `remote-evidence` until current local truth lands on the remote default branch and candidate tags**.

### 3. The public packages surfaces are already green; the `meshlang.dev` surfaces are not

Current live checks I ran against the public endpoints show:

#### Green now

- `https://packages.meshlang.dev/packages/snowdamiz/mesh-registry-proof`
  - contains `snowdamiz/mesh-registry-proof`
  - contains `Real registry publish/install proof fixture for M034 S01`
- `https://packages.meshlang.dev/search?q=snowdamiz%2Fmesh-registry-proof`
  - preserves the full scoped query result string
  - contains the package name + description
- `https://api.packages.meshlang.dev/api/v1/packages?search=snowdamiz%2Fmesh-registry-proof`
  - returns an exact package match with the expected description

#### Red now

- `https://meshlang.dev/install.sh`
  - HTTP 200, `content-type: application/x-sh`
  - body does **not** match `website/docs/public/install.sh`
  - diff shows the live script is still from the older “meshc compiler only” era and is missing the newer staged-proof env hooks (`MESH_INSTALL_RELEASE_API_URL`, `MESH_INSTALL_RELEASE_BASE_URL`, `MESH_INSTALL_STRICT_PROOF`)
- `https://meshlang.dev/install.ps1`
  - HTTP 200, `content-type: application/octet-stream`
  - body does **not** match `website/docs/public/install.ps1`
  - live body still points at the old repo slug (`mesh-lang/mesh`), still lacks the meshpkg additions, and lacks the staged-proof hooks
- `https://meshlang.dev/docs/getting-started/`
  - still exposes the older source-build-only install story (`Today the verified install path is building meshc from source` in extracted text)
  - missing the documented installer URLs and `meshpkg --version` marker that the current local doc contains
- `https://meshlang.dev/docs/tooling/`
  - still exposes the older `meshc`-centric toolchain text
  - missing the S05 runbook markers, candidate-tag/proof-artifact paths, and `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh`

This isolates the current public-freshness problem to **GitHub Pages / `meshlang.dev` freshness**, not the registry or packages site.

### 4. Hosted workflow checks are weaker than the exact `public-http` contract

This is the most important code-level seam if S07 needs to harden CI/CD rather than only perform operator replay.

#### `deploy.yml`

`Verify public docs contract` currently checks:

- built `install.sh` and `install.ps1` exist in the dist output
- built docs pages exist
- dist installer files diff against the local `website/docs/public/*` files
- docs pages include installer URLs and a few version markers

It **does not** check the full S05 tooling runbook markers, workflow names, candidate-tag strings, or `.tmp/m034-s05/verify/{candidate-tags,remote-runs}.json` references that `run_local_docs_truth()` / `run_built_docs_truth()` and `run_public_http_truth()` require.

#### `deploy-services.yml`

Its live post-deploy checks are also weaker:

- installer endpoint check is grep-based and would not catch the current missing staged-proof hooks in the old shell script
- docs page check only looks for installer URLs / version markers / `packages.meshlang.dev`, not the full S05 release-assembly runbook content

So today there is still a **contract split**:

- **stronger contract:** local `scripts/verify-m034-s05.sh`
- **weaker contract:** hosted deploy workflows

If S07 is supposed to make public-freshness truth part of CI/CD rather than a later local replay only, this is the natural place to tighten.

### 5. There is no dedicated test coverage for `run_public_http_truth()` parity

Current automated coverage is useful but thin:

- `scripts/tests/verify-m034-s05-contract.test.mjs`
  - candidate tags
  - README/tooling runbook strings
  - release vs extension tag split
- `scripts/tests/verify-m034-s06-contract.test.mjs`
  - stop-after boundary
  - archive helper behavior
  - extension-proof polling semantics

There is **no contract test** that pins:

- the full docs/installers marker set used by `run_public_http_truth()`
- parity between those markers and the hosted workflow checks
- any wait/retry semantics around public freshness after rollout

If S07 edits `deploy.yml`, `deploy-services.yml`, or the `public-http` phase, add test coverage here instead of relying only on the expensive live replay.

### 6. The operational blocker is still transport-level HTTPS push failure

The retained S06 recovery logs are still the root-cause evidence:

- `.tmp/m034-s06/push-main.stderr` -> `POST git-receive-pack (chunked)` then `RPC failed; HTTP 408`
- `.tmp/m034-s06/transport-recovery/03-http11-postbuffer-1g.stderr` -> even a non-chunked `POST git-receive-pack (564496785 bytes)` still ends with `HTTP 408`
- `.tmp/m034-s06/transport-recovery/attempts.log` shows the retry took ~12–13 minutes and still failed

This matters for planning order:

- if S07 includes operational execution, the first risky task is still **finding a transport path that can land local main on origin/main**
- if S07 is limited to code changes, the planner should expect to end with a truthful blocker unless transport changes externally

## Natural Seams / Suggested Task Decomposition

### Seam A — Hosted rollout evidence refresh

Files/surfaces:

- `scripts/verify-m034-s06-remote-evidence.sh`
- `.tmp/m034-s06/evidence/*`
- `.tmp/m034-s06/transport-recovery/*`
- `.github/workflows/*.yml`

What to prove first:

- Remote `main` must expose the current workflow graph.
- Fresh hosted runs must exist for `main`, `v0.1.0`, and `ext-v0.3.0` before the full S05 replay can advance.

Planner note:

- This is the highest-risk seam because it may be operationally blocked outside the repo.
- If a transport workaround is available, archive new evidence in the S06 label order (`main` -> `v0.1.0` -> `first-green`) before rerunning the full S05 replay.

### Seam B — Public surface contract parity

Files likely touched if code changes are needed:

- `.github/workflows/deploy.yml`
- `.github/workflows/deploy-services.yml`
- `scripts/verify-m034-s05.sh`
- `scripts/tests/verify-m034-s05-contract.test.mjs` (or a new dedicated S07 contract test)

What to prove:

- Hosted workflows check the same installer/docs truth that `public-http` expects, not a weaker subset.
- Public live endpoints match `website/docs/public/install.*` and the current docs markdown markers once rollout completes.

Planner note:

- This seam is the best place for any actual repo edits if the slice must “include everything important in CI/CD.”

### Seam C — Final assembled replay

Files/surfaces:

- `scripts/verify-m034-s05.sh`
- `scripts/verify-m034-s01.sh`
- `.tmp/m034-s05/verify/*`
- `.tmp/m034-s01/verify/*`

What to prove:

- `remote-evidence` passes
- `public-http` passes with fresh live bytes/bodies
- `s01-live-proof` passes and emits `package-version.txt` under `.tmp/m034-s01/verify/**/package-version.txt`

Planner note:

- Do not start here. This is the final integration replay after the hosted/public seams are green.

## Verification

### Lightweight verification I ran now

- `node --test scripts/tests/verify-m034-s05-contract.test.mjs` ✅
- `node --test scripts/tests/verify-m034-s06-contract.test.mjs` ✅
- `bash scripts/verify-m034-s05-workflows.sh` ✅
- direct live HTTP checks against:
  - `https://meshlang.dev/install.sh` -> 200 / `application/x-sh`, **body mismatch** vs local
  - `https://meshlang.dev/install.ps1` -> 200 / `application/octet-stream`, **body mismatch** vs local
  - `https://meshlang.dev/docs/getting-started/` -> 200, **missing current installer/meshpkg markers**
  - `https://meshlang.dev/docs/tooling/` -> 200, **missing current S05 runbook markers**
  - package detail/search/API endpoints -> all satisfied the expected S05 markers

### Best next verification sequence for executors

1. `bash scripts/verify-m034-s06-remote-evidence.sh <new-label> || true`
   - use only after a real hosted rollout attempt or remote fix
   - inspect:
     - `.tmp/m034-s06/evidence/<label>/manifest.json`
     - `.tmp/m034-s06/evidence/<label>/remote-runs.json`
2. If hosted evidence is green enough to continue, run the full replay:
   - `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh`
3. Confirm final artifacts:
   - `.tmp/m034-s05/verify/status.txt` -> `ok`
   - `.tmp/m034-s05/verify/current-phase.txt` -> `complete`
   - `.tmp/m034-s05/verify/phase-report.txt` includes `remote-evidence`, `public-http`, and `s01-live-proof` as `passed`
   - `.tmp/m034-s05/verify/public-http.log` is populated
   - `.tmp/m034-s01/verify/**/package-version.txt` exists

## Risks / Watchouts

- **Dirty working tree:** the repo has many unrelated `.gsd/`, `.tmp/`, and `mesher/landing/` changes. Avoid wide cleanup and keep S07 work scoped.
- **The public docs/installers are stale even though they are available.** This is a freshness/content problem, not an uptime problem.
- **GitHub Pages responses currently advertise `cache-control: max-age=600`.** Even after a good deploy, there may be short-lived propagation lag; if S07 changes timing semantics, do it intentionally and test it.
- **No active requirement row currently maps this slice.** If closeout needs requirement bookkeeping, that work is separate from the core code/verification path.
- **Do not weaken the canonical entrypoint.** The milestone context and prior decisions already established `scripts/verify-m034-s05.sh` as the one public-release acceptance surface.

## Recommendation

1. **Plan S07 around the real blocker order, not the nominal roadmap order:**
   - first hosted rollout truth (`remote-evidence`)
   - then live public freshness (`public-http`)
   - then final live publish/install replay (`s01-live-proof`)
2. **Assume at least one task is operational/blocker-handling, not purely code.** The retained S06 transport evidence is still current and should be treated as the first gate.
3. **If code changes are needed, spend them on hosted/public contract parity, not on new verification entrypoints.** The strongest likely repo edits are tightening `deploy.yml` / `deploy-services.yml` and adding contract tests around the exact `public-http` markers.
4. **Only after hosted truth and live freshness are green should executors spend time on the expensive full S05 replay.**
