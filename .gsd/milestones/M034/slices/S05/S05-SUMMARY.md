---
id: S05
parent: M034
milestone: M034
provides:
  - A canonical S05 release-assembly verifier (`scripts/verify-m034-s05.sh`) that composes the existing S01-S04 proof surfaces around one candidate identity.
  - An exact public release contract across deploy workflows, installer/docs wording, packages-site URLs, registry scoped-search, and hosted workflow/candidate-tag naming.
  - Auditable local and hosted evidence artifacts under `.tmp/m034-s05/` that isolate whether a failure is local contract drift, hosted rollout drift, or stale deployed public content.
requires:
  - slice: S01
    provides: The live real-registry publish/install proof and scoped package truth surface that S05 reuses unchanged as the only authoritative publish/install phase.
  - slice: S02
    provides: The authoritative workflow-ownership pattern and local workflow-contract verification approach that S05 extends to deploy/docs/release assembly.
  - slice: S03
    provides: The canonical installer/release-asset proof, canonical installer sources, and staged binary verification surfaces that S05 replays rather than re-implementing.
  - slice: S04
    provides: The deterministic VSIX proof, extension-release workflow ownership model, and exact-artifact handoff that S05 composes into the public release story.
affects:
  []
key_files:
  - scripts/verify-m034-s05.sh
  - scripts/verify-m034-s05-workflows.sh
  - scripts/tests/verify-m034-s05-contract.test.mjs
  - .github/workflows/deploy.yml
  - .github/workflows/deploy-services.yml
  - README.md
  - website/docs/docs/getting-started/index.md
  - website/docs/docs/tooling/index.md
  - tools/editors/vscode-mesh/package.json
  - tools/editors/vscode-mesh/README.md
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
  - .gsd/PROJECT.md
key_decisions:
  - Use `scripts/verify-m034-s05.sh` as the single assembly entrypoint and compose S01-S04 behind paired candidate tags `v<Cargo version>` and `ext-v<extension version>` rather than inventing one shared tag.
  - Fail the release assembly on exact public HTTP drift with installer body diffs, normalized docs text checks, packages-site body markers, and registry scoped-search JSON markers instead of homepage-only reachability.
  - Run candidate-tag derivation and hosted-run evidence before public-http / S01 so external GitHub rollout gaps still leave auditable `candidate-tags.json` and `remote-runs.json` artifacts.
patterns_established:
  - Keep assembly proof in repo-local verifier scripts and make GitHub Actions thin callers or queried evidence sources rather than reimplementing proof logic in YAML.
  - Treat public docs/installers as exact truth surfaces; for VitePress output, normalize rendered HTML text before asserting command markers because code blocks split URLs and flags across spans.
  - Separate local contract drift from external rollout/public freshness drift with phase-scoped artifacts under `.tmp/m034-s05/workflows/` and `.tmp/m034-s05/verify/`.
  - Derive binary and extension candidate identities mechanically from version sources and keep those tags independent across workflows, runbooks, and hosted evidence.
observability_surfaces:
  - `.tmp/m034-s05/workflows/phase-report.txt`, `docs.log`, `services.log`, and `full-contract.log` for local deploy-workflow drift.
  - `.tmp/m034-s05/verify/current-phase.txt`, `failed-phase.txt`, `status.txt`, `phase-report.txt`, `candidate-tags.json`, `remote-runs.json`, and `remote-*.log` for assembled verifier state and hosted rollout evidence.
  - `public-*.body`, `public-*.headers`, `public-*.diff`, and `public-http.log` when the public-http phase executes, plus the standalone S01 proof artifacts under `.tmp/m034-s01/verify/`.
drill_down_paths:
  - .gsd/milestones/M034/slices/S05/tasks/T01-SUMMARY.md
  - .gsd/milestones/M034/slices/S05/tasks/T02-SUMMARY.md
  - .gsd/milestones/M034/slices/S05/tasks/T03-SUMMARY.md
  - .gsd/milestones/M034/slices/S05/tasks/T04-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-27T03:27:40.325Z
blocker_discovered: false
---

# S05: Full public release assembly proof

**S05 assembled the canonical public-release verifier and runbook, but the first full replay remains fail-closed on unshipped GitHub workflow evidence and stale meshlang.dev installer/docs content.**

## What Happened

S05 closed the remaining local composition work for M034 and turned the public release story into one explicit verifier-owned contract instead of a pile of separate assumptions. T01 added `scripts/verify-m034-s05-workflows.sh` plus parser-backed contract checks for `.github/workflows/deploy.yml` and `.github/workflows/deploy-services.yml`, and tightened those workflows so they prove the exact docs/install/package surfaces S05 claims instead of homepage-only reachability. T02 aligned `README.md`, the getting-started/tooling docs, and the VS Code extension metadata/README around the same public installer pair, both-binary install story, and current `snowdamiz/mesh-lang` repository identity.

T03 created `scripts/verify-m034-s05.sh` as the canonical serial assembly verifier. It owns `.tmp/m034-s05/verify/`, records current/failing phase state, reuses the existing S01-S04 verifiers unchanged, builds VitePress, proves local source-truth plus built-doc truth, and defines exact `public-http` checks for installers, docs pages, packages-site detail/search pages, and the registry scoped-search API. T04 extended that wrapper with candidate-tag derivation and hosted-run evidence: the verifier now derives `v<Cargo version>` from `compiler/meshc/Cargo.toml` / `compiler/meshpkg/Cargo.toml`, derives `ext-v<extension version>` from `tools/editors/vscode-mesh/package.json`, persists that data to `.tmp/m034-s05/verify/candidate-tags.json`, queries `gh run list/view` for `deploy.yml`, `deploy-services.yml`, `authoritative-verification.yml`, `release.yml`, `extension-release-proof.yml`, and `publish-extension.yml`, and writes the aggregate hosted-rollout report to `.tmp/m034-s05/verify/remote-runs.json`. `README.md` and `website/docs/docs/tooling/index.md` now publish the same command, candidate-tag policy, hosted-workflow set, public URLs, and proof-artifact paths that the verifier enforces mechanically.

Closeout verification showed that the local assembly is wired correctly but the public-ready story is still blocked by external rollout freshness. The local workflow-contract gate, docs-truth/build checks, S02 workflow proof, S03 installer proof, S04 extension proof, S04 workflow proof, and standalone S01 live publish/install proof all passed. The canonical assembled run still stops at `remote-evidence`, and the resulting `remote-runs.json` shows the real hosted gaps: the latest remote `deploy.yml` run predates the new `Verify public docs contract` step; `deploy-services.yml` and `release.yml` have no hosted `push` runs for binary tag `v0.1.0`; `authoritative-verification.yml` and `extension-release-proof.yml` are not yet present on the remote default branch queried by GitHub; and `publish-extension.yml` has no hosted `push` run for extension tag `ext-v0.3.0`. Independent direct public-surface checks also show that `https://meshlang.dev/install.sh`, `https://meshlang.dev/install.ps1`, and the deployed getting-started/tooling pages are still stale relative to the repo contract, even though the packages-site detail/search pages and registry scoped-search API still match the S01 proof package exactly.

Operational readiness:
- Health signal: `bash scripts/verify-m034-s05-workflows.sh`, `bash -n scripts/verify-m034-s05.sh`, `node --test scripts/tests/verify-m034-s05-contract.test.mjs`, and standalone `set -a && source .env && set +a && bash scripts/verify-m034-s01.sh` are green; `.tmp/m034-s05/verify/current-phase.txt`, `status.txt`, `phase-report.txt`, `candidate-tags.json`, and `remote-runs.json` are present and coherent; packages-site detail/search plus registry scoped search still match the S01 proof package.
- Failure signal: `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh` exits non-zero with `failed-phase.txt=remote-evidence`; `remote-runs.json` names missing workflow rollout; `public install.sh` / `public install.ps1` diffs are non-empty; normalized HTML checks on the deployed docs miss the new installer/runbook markers.
- Recovery procedure: land the local workflow/docs changes on the remote default branch, run the required `v0.1.0` and `ext-v0.3.0` hosted workflows, redeploy `meshlang.dev` so installers/docs match the repo contract, then rerun `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh` until it reaches `public-http` and `s01-live-proof` and finishes with `status.txt=ok`.
- Monitoring gap: there is still no hosted green evidence for the new workflow graph on the current candidate tags, and the public docs/installers are stale enough that the canonical wrapper cannot yet produce one honest end-to-end green run.

## Verification

Ran the slice-level verification bundle and closeout spot checks.

Passing checks:
- `bash scripts/verify-m034-s05-workflows.sh`
- `ruby -e 'require "yaml"; %w[.github/workflows/deploy.yml .github/workflows/deploy-services.yml].each { |f| YAML.load_file(f) }'`
- `rg -n 'install\.sh|install\.ps1|packages/snowdamiz/mesh-registry-proof|api/v1/packages\?search=snowdamiz%2Fmesh-registry-proof' .github/workflows/deploy.yml .github/workflows/deploy-services.yml`
- `rg -n 'meshlang.dev/install.sh|meshlang.dev/install.ps1|meshpkg --version' README.md website/docs/docs/getting-started/index.md website/docs/docs/tooling/index.md tools/editors/vscode-mesh/README.md`
- `! rg -n 'Today the verified install path is building \`meshc\` from source|mesh-lang/mesh' README.md website/docs/docs/getting-started/index.md website/docs/docs/tooling/index.md website/docs/public/install.ps1 tools/editors/vscode-mesh/package.json tools/editors/vscode-mesh/README.md`
- `node -p "require('./tools/editors/vscode-mesh/package.json').repository.url"`
- `node -p "require('./tools/editors/vscode-mesh/package.json').bugs.url"`
- `bash -n scripts/verify-m034-s05.sh`
- `node --test scripts/tests/verify-m034-s05-contract.test.mjs`
- `test -f .tmp/m034-s05/verify/current-phase.txt && test -f .tmp/m034-s05/verify/remote-runs.json && test -f .tmp/m034-s05/verify/candidate-tags.json && test -f .tmp/m034-s05/workflows/phase-report.txt`
- `rg -n 'verify-m034-s05|v<Cargo version>|ext-v<extension version>|deploy\.yml|deploy-services\.yml|authoritative-verification\.yml|extension-release-proof\.yml|publish-extension\.yml' README.md website/docs/docs/tooling/index.md`
- Standalone live registry proof: `set -a && source .env && set +a && bash scripts/verify-m034-s01.sh` -> `verify-m034-s01: ok`
- Independent public package surfaces: packages-site detail/search pages and the registry scoped-search API still return the exact `snowdamiz/mesh-registry-proof` name and `Real registry publish/install proof fixture for M034 S01` description.

Fail-closed checks that still surface the remaining rollout gap:
- `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh` stops at `remote-evidence` and writes `.tmp/m034-s05/verify/remote-runs.json` plus phase logs naming the missing remote workflow/tag evidence.
- Direct remote/public freshness checks still show drift: `https://meshlang.dev/install.sh` and `https://meshlang.dev/install.ps1` both differ from `website/docs/public/install.{sh,ps1}`, and normalized HTML checks over the deployed getting-started/tooling pages are missing the new installer/runbook markers.

Observability was confirmed: `.tmp/m034-s05/workflows/phase-report.txt` reports the deploy workflow contract phases as passed, and `.tmp/m034-s05/verify/current-phase.txt`, `status.txt`, `phase-report.txt`, `candidate-tags.json`, and `remote-runs.json` all updated truthfully during closeout.

## Requirements Advanced

None.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

No code-scope deviation from the slice plan. For closeout, I added two direct verification passes outside the canonical wrapper because the assembled verifier now stops earlier at `remote-evidence` when hosted rollout is red: a standalone replay of `scripts/verify-m034-s01.sh` to confirm the live registry publish/install proof still holds, and direct curl/text-normalization checks to measure current public installer/docs freshness.

## Known Limitations

The slice is still not honestly public-ready on hosted surfaces. `remote-runs.json` shows that the remote default branch/tag history has not yet caught up to the local workflow graph: the latest hosted `deploy.yml` run predates the `Verify public docs contract` step, `deploy-services.yml` and `release.yml` have no `push` runs for `v0.1.0`, `authoritative-verification.yml` and `extension-release-proof.yml` are missing on the remote default branch, and `publish-extension.yml` has no `push` run for `ext-v0.3.0`. Direct public-surface checks also show stale `meshlang.dev` installers and stale getting-started/tooling pages, so the canonical S05 wrapper cannot yet reach a truthful green `public-http` or full assembled success path.

## Follow-ups

1. Land the local S05 workflow/docs changes on the remote default branch and capture the first hosted green `deploy.yml`, `authoritative-verification.yml`, `release.yml`, `deploy-services.yml`, `extension-release-proof.yml`, and `publish-extension.yml` runs for `v0.1.0` / `ext-v0.3.0`.
2. Redeploy `meshlang.dev` so `install.sh`, `install.ps1`, and the getting-started/tooling pages match the repo-local contract that S05 now verifies.
3. Rerun `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh` after hosted rollout and public redeploy, and preserve the first all-green `.tmp/m034-s05/verify/` artifact bundle as milestone-closeout evidence.

## Files Created/Modified

- `scripts/verify-m034-s05.sh` — Added the canonical serial assembly verifier with phase tracking, candidate-tag derivation, hosted-run evidence capture, local docs truth checks, and exact public HTTP proof phases.
- `scripts/verify-m034-s05-workflows.sh` — Added the S05-owned parser-backed deploy workflow verifier and named workflow-drift logs under `.tmp/m034-s05/workflows/`.
- `scripts/tests/verify-m034-s05-contract.test.mjs` — Added a fast contract test that pins the published S05 runbook strings, candidate-tag policy, and workflow-trigger split.
- `.github/workflows/deploy.yml` — Tightened the Pages workflow so the built VitePress artifact preserves the exact installer/docs contract before upload.
- `.github/workflows/deploy-services.yml` — Tightened post-deploy health checks to prove the exact registry search, packages detail/search, installer, and docs URLs instead of homepage-only reachability.
- `README.md` — Documented the canonical S05 command, split binary-vs-extension candidate tags, required hosted workflows, and public proof URLs/artifacts.
- `website/docs/docs/getting-started/index.md` — Aligned first-run install guidance with the verified public installer pair and both-binary install story.
- `website/docs/docs/tooling/index.md` — Published the assembled S05 runbook, candidate-tag policy, required workflows, public URLs, and proof artifact paths.
- `tools/editors/vscode-mesh/package.json` — Corrected extension repository and bugs URLs to the current `snowdamiz/mesh-lang` repo so S05 can verify public metadata exactly.
- `tools/editors/vscode-mesh/README.md` — Aligned extension installation wording with the same public installer pair and proof story used in the repo docs.
- `.gsd/DECISIONS.md` — Recorded S05 decisions for the canonical assembly verifier, exact public HTTP proof, and hosted-evidence artifact ordering.
- `.gsd/KNOWLEDGE.md` — Recorded the VitePress HTML text-extraction gotcha and the S05 remote-evidence artifact-ordering lesson for future agents.
- `.gsd/PROJECT.md` — Refreshed current project state to describe the new S05 assembly verifier and the remaining hosted-rollout/public-freshness gap.
