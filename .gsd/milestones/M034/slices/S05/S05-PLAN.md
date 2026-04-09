# S05: Full public release assembly proof

**Goal:** Turn M034's release-truth surfaces into one honest public-ready acceptance flow by adding deploy/docs contract verification, exact public content checks, and a canonical S05 verifier that composes the existing S01-S04 proof surfaces around one release candidate.
**Demo:** After this: One release candidate is proven across binaries, installer, docs deployment, registry/packages-site health, and extension release checks as a single public-ready flow.

## Tasks
- [x] **T01: Added an S05 deploy-workflow verifier and exact Pages/Fly public-surface checks for docs, installers, and registry proof URLs.** — Close the remaining local workflow coverage gap before composing the final release proof. S02 and S04 already verify release and extension workflows, but `deploy.yml` and `deploy-services.yml` still only have basic YAML/runtime coverage and root curls. This task adds an S05-owned workflow verifier and tightens the deploy workflows so docs deployment, Fly service deployment, and post-deploy checks are mechanically provable rather than assumed.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| GitHub Actions YAML / contract parser | Fail the task immediately and keep the first drifting invariant in a named `.tmp/m034-s05/workflows/*.log` artifact. | N/A | Treat missing triggers, permissions, needs edges, or required URL probes as contract drift. |
| Fly deploy post-deploy health checks | Keep the deploy jobs fail-closed and surface which exact URL/content assertion drifted instead of falling back to root-level curls. | Treat retries/timeouts as deploy-health failure, not as a soft warning. | Treat HTML/JSON bodies missing the exact docs or package proof markers as failure. |
| Docs deploy workflow | Block the task if Pages deploy still proves only build/upload shape without wiring the exact public files that S05 claims. | Treat a stalled Pages deploy step as workflow drift that must be fixed locally first. | Treat missing build/upload steps or wrong artifact paths as contract failure. |

## Load Profile

- **Shared resources**: GitHub runner minutes, Fly deploy jobs, and the public docs/packages hosts checked after deploy.
- **Per-operation cost**: one local workflow-contract parse sweep plus a small number of exact URL/content checks in the deploy workflows.
- **10x breakpoint**: hosted runner time and slow public retries fail first, so the contract must stay exact and avoid duplicate curl probes.

## Negative Tests

- **Malformed inputs**: missing `workflow_dispatch`, wrong tag filters, missing `needs`, or post-deploy checks that only hit `/`.
- **Error paths**: stale installer/docs content, empty package search results, or wrong registry search query must each keep the workflow red.
- **Boundary conditions**: the deploy-services checks must use the full scoped package query and exact public docs/install URLs, not slug-only or homepage-only probes.

## Steps

1. Add `scripts/verify-m034-s05-workflows.sh` as the S05 local contract verifier for `.github/workflows/deploy.yml` and `.github/workflows/deploy-services.yml`, following the S02/S04 parser-backed pattern and writing phase logs under `.tmp/m034-s05/workflows/`.
2. Tighten `.github/workflows/deploy.yml` so the docs build/deploy lane preserves the exact VitePress/public contract S05 will later verify live.
3. Tighten `.github/workflows/deploy-services.yml` so post-deploy checks prove the exact docs/install/package surfaces instead of only root reachability.
4. Rerun the new workflow verifier and YAML parse checks until deploy-lane drift fails locally before any remote rollout.

## Must-Haves

- [ ] `scripts/verify-m034-s05-workflows.sh` mechanically rejects drift in `deploy.yml` and `deploy-services.yml`
- [ ] Deploy workflow checks prove exact public installer/docs/package surfaces, not just `200 OK`
- [ ] The deploy-services package search probe uses the full scoped package query from S01
- [ ] Workflow diagnostics land under `.tmp/m034-s05/workflows/` so future failures localize cleanly

## Observability Impact

- Signals added/changed: deploy-workflow contract logs under `.tmp/m034-s05/workflows/` and named failing invariants for docs vs services deploy drift.
- How a future agent inspects this: rerun `bash scripts/verify-m034-s05-workflows.sh` and read `.tmp/m034-s05/workflows/*.log`.
- Failure state exposed: the first drifting workflow, step, trigger, or URL/content assertion.
  - Estimate: 2h
  - Files: .github/workflows/deploy.yml, .github/workflows/deploy-services.yml, scripts/verify-m034-s05-workflows.sh
  - Verify: - `bash scripts/verify-m034-s05-workflows.sh`
- `ruby -e 'require "yaml"; %w[.github/workflows/deploy.yml .github/workflows/deploy-services.yml].each { |f| YAML.load_file(f) }'`
- `rg -n 'install\.sh|install\.ps1|packages/snowdamiz/mesh-registry-proof|api/v1/packages\?search=snowdamiz%2Fmesh-registry-proof' .github/workflows/deploy.yml .github/workflows/deploy-services.yml`
- [x] **T02: Aligned README/docs install-proof wording and VS Code extension metadata with the S05 public release contract.** — Make every public-facing local source file say exactly what S05 will verify live. The deployed site is stale today, and the extension manifest still points at the old repository. This task updates docs, installer sources, and extension metadata so the local public contract matches the release-candidate story and gives the S05 verifier stable exact-string truth surfaces.

## Steps

1. Update `README.md`, `website/docs/docs/getting-started/index.md`, and `website/docs/docs/tooling/index.md` so the verified installer path, package-manager story, and assembled release-proof references all agree.
2. Reconfirm that `website/docs/public/install.sh` and `website/docs/public/install.ps1` carry the exact repo/install contract S05 expects to verify live: `snowdamiz/mesh-lang`, both binaries, and the public install commands.
3. Fix `tools/editors/vscode-mesh/package.json` repository/bugs URLs and `tools/editors/vscode-mesh/README.md` so the extension’s public metadata matches the current repo and verified install story.
4. Add or preserve grep-friendly strings so S05 can prove this public contract mechanically instead of relying on prose interpretation.

## Must-Haves

- [ ] Public docs no longer present source builds as the only verified install path
- [ ] Both installer sources name `snowdamiz/mesh-lang` and `meshpkg` consistently
- [ ] Extension repository and bugs URLs point at the current `snowdamiz/mesh-lang` repo
- [ ] README, docs, and extension README describe the same release-proof/install contract S05 will verify live
  - Estimate: 90m
  - Files: README.md, website/docs/docs/getting-started/index.md, website/docs/docs/tooling/index.md, website/docs/public/install.sh, website/docs/public/install.ps1, tools/editors/vscode-mesh/package.json, tools/editors/vscode-mesh/README.md
  - Verify: - `rg -n 'meshlang.dev/install.sh|meshlang.dev/install.ps1|meshpkg --version' README.md website/docs/docs/getting-started/index.md website/docs/docs/tooling/index.md tools/editors/vscode-mesh/README.md`
- `! rg -n 'Today the verified install path is building \`meshc\` from source|mesh-lang/mesh' README.md website/docs/docs/getting-started/index.md website/docs/docs/tooling/index.md website/docs/public/install.ps1 tools/editors/vscode-mesh/package.json tools/editors/vscode-mesh/README.md`
- `node -p "require('./tools/editors/vscode-mesh/package.json').repository.url"`
- `node -p "require('./tools/editors/vscode-mesh/package.json').bugs.url"`
- [x] **T03: Added the canonical S05 release-assembly verifier and made it fail on the first stale public install surface.** — Create the one acceptance command that turns subsystem green lights into one auditable release proof. The verifier should stay serial, reuse existing slice-owned verifiers unchanged, add the S05-owned docs-truth and public-HTTP phases, and preserve first-failing-phase logs under `.tmp/m034-s05/verify/`.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Reused slice verifiers (`S01`–`S04`) | Stop at the first failing phase and keep the upstream verifier log path visible; never continue into later phases after a red prerequisite. | Treat the phase as failed and report which verifier stalled. | Treat missing status/artifact files from reused verifiers as proof failure. |
| Docs build / truth sweep | Fail before any live HTTP or publish work if the local public contract or VitePress build is already red. | Treat VitePress stalls as a hard failure and leave the build log under `.tmp/m034-s05/verify/`. | Treat missing or drifted exact strings in docs/installers/README/extension metadata as docs-truth failure. |
| Public HTTP and S01 live proof | Fail on the first mismatched URL/body/header or live publish/install drift; do not silently downgrade to local-only success. | Keep retries bounded and surface the exact URL or live proof phase that timed out. | Treat slug-only search hits, stale installer scripts, or wrong docs text as public-release drift. |

## Load Profile

- **Shared resources**: VitePress build temp files, local Cargo/Node caches, public docs/packages hosts, and the real registry publish path used by S01.
- **Per-operation cost**: one serial docs build, one local docs truth sweep, five reused verifier invocations, several public HTTP fetches, and one real registry publish/install proof.
- **10x breakpoint**: the live registry proof and public HTTP retries fail first, so the wrapper must stay strictly serial and avoid duplicate publish/package checks.

## Negative Tests

- **Malformed inputs**: missing `.env` vars, stale docs strings, missing verifier scripts, or a public search query that drops the owner prefix.
- **Error paths**: any reused verifier failure, exact-content HTTP mismatch, or live publish/install regression must stop the wrapper on a named phase.
- **Boundary conditions**: keep one owner of the live publish/install proof (`scripts/verify-m034-s01.sh`) and one owner of the extension proof (`scripts/verify-m034-s04-extension.sh`) rather than duplicating those assertions in S05.

## Steps

1. Create `scripts/verify-m034-s05.sh` with deterministic `.tmp/m034-s05/verify/` state, named phases, first-failure reporting, and repo-root-relative artifact paths.
2. Add serial docs build and exact-string local truth phases over the README/docs/installers/extension metadata that T02 aligned.
3. Reuse `scripts/verify-m034-s05-workflows.sh`, `scripts/verify-m034-s02-workflows.sh`, `scripts/verify-m034-s03.sh`, `scripts/verify-m034-s04-extension.sh`, and `scripts/verify-m034-s04-workflows.sh` unchanged from the wrapper.
4. Add exact public HTTP checks for the deployed installers, getting-started/tooling pages, packages-site detail/search pages, and registry scoped search API, then reuse `set -a && source .env && set +a && bash scripts/verify-m034-s01.sh` as the only live registry publish/install proof phase.

## Must-Haves

- [ ] `scripts/verify-m034-s05.sh` is the single assembled acceptance command for S05
- [ ] The wrapper stays serial and reuses S01-S04 verifiers rather than re-implementing their assertions
- [ ] Local docs truth, public HTTP truth, and the live registry publish/install proof all participate in one artifacted run
- [ ] `.tmp/m034-s05/verify/` exposes current phase, failure status, and per-phase logs for every stage
  - Estimate: 150m
  - Files: scripts/verify-m034-s05.sh, scripts/verify-m034-s05-workflows.sh, README.md, website/docs/docs/getting-started/index.md, website/docs/docs/tooling/index.md, website/docs/public/install.sh, website/docs/public/install.ps1, tools/editors/vscode-mesh/package.json
  - Verify: - `bash -n scripts/verify-m034-s05.sh`
- `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh`
- `test -f .tmp/m034-s05/verify/current-phase.txt`
- `test -f .tmp/m034-s05/verify/status.txt && rg -n '^ok$' .tmp/m034-s05/verify/status.txt`
- [x] **T04: Added candidate-tag derivation plus hosted-run evidence to the S05 release verifier and documented the canonical public release runbook.** — Finish the assembly slice by making the release candidate explicit and checking the hosted rollout surfaces honestly. This task codifies the binary-vs-extension tag policy, adds GitHub Actions evidence checks to the S05 verifier, and publishes one canonical runbook entry so future agents know exactly which tags, workflows, URLs, and proof artifacts define a public-ready Mesh release.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| GitHub CLI / workflow evidence queries | Fail the remote-evidence phase with a named rollout gap and keep the exact `gh run list/view` command output under `.tmp/m034-s05/verify/`; never synthesize success. | Treat GitHub API timeouts as remote-evidence failure and preserve the partial query output. | Treat missing workflows, wrong events, or missing jobs as rollout drift rather than ignoring them. |
| Candidate tag derivation | Fail if Cargo versions and extension version do not map cleanly to `v<Cargo version>` and `ext-v<extension version>`. | N/A | Treat mismatched or stale tag examples in docs/runbooks as contract drift. |
| Runbook documentation | Fail if the canonical runbook references commands, tags, or workflows that the verifier does not actually use. | N/A | Treat placeholder-only or outdated release instructions as incomplete proof. |

## Load Profile

- **Shared resources**: GitHub Actions run history, GitHub API rate limits/auth, and the same remote workflow surfaces that release/deploy/publication depend on.
- **Per-operation cost**: a few `gh run list/view` queries plus grep-level runbook truth checks.
- **10x breakpoint**: GitHub API/rate-limit pressure fails first, so the verifier should request only the most recent required runs/jobs and persist the results locally.

## Negative Tests

- **Malformed inputs**: missing default-branch workflows, stale historical runs that predate the new proof jobs, or docs that claim a single unified tag version.
- **Error paths**: 404 workflow-not-found, empty run lists, missing required jobs, or mismatched candidate tags must each keep S05 red.
- **Boundary conditions**: the binary release and extension release remain independently versioned; S05 must compose them honestly instead of inventing one shared tag.

## Steps

1. Extend `scripts/verify-m034-s05.sh` with a remote-evidence phase that records the most recent required `gh run list/view` output for `deploy.yml`, `deploy-services.yml`, `authoritative-verification.yml`, `release.yml`, `extension-release-proof.yml`, and `publish-extension.yml`.
2. Make the verifier derive and report the expected binary and extension candidate tags from `compiler/meshc/Cargo.toml`, `compiler/meshpkg/Cargo.toml`, and `tools/editors/vscode-mesh/package.json`, and fail if the docs/runbook or remote evidence disagree.
3. Update the canonical operator-facing runbook entry in `README.md` and the public tooling docs so the assembled proof command, candidate-tag policy, and required hosted workflows are explicit.
4. Rerun the full S05 verifier and confirm it leaves `remote-runs.json` / `candidate-tags.json` artifacts instead of hand-waved rollout notes.

## Must-Haves

- [ ] S05 derives and documents `v<Cargo version>` for binaries and `ext-v<extension version>` for the extension instead of inventing a fake unified tag
- [ ] Hosted workflow evidence is part of the canonical verifier and fails honestly on rollout gaps
- [ ] The README/tooling runbook names the exact command, workflows, and public URLs that define a public-ready release
- [ ] `.tmp/m034-s05/verify/remote-runs.json` and `.tmp/m034-s05/verify/candidate-tags.json` make hosted evidence auditable after the run

## Observability Impact

- Signals added/changed: persisted GitHub run metadata plus derived candidate-tag artifacts under `.tmp/m034-s05/verify/`.
- How a future agent inspects this: rerun `bash scripts/verify-m034-s05.sh` or read `remote-runs.json` / `candidate-tags.json` after a failure.
- Failure state exposed: missing workflow rollout, wrong event/job graph, or candidate-tag drift between repo state and hosted evidence.
  - Estimate: 2h
  - Files: scripts/verify-m034-s05.sh, README.md, website/docs/docs/tooling/index.md, .github/workflows/release.yml, .github/workflows/publish-extension.yml, compiler/meshc/Cargo.toml, compiler/meshpkg/Cargo.toml, tools/editors/vscode-mesh/package.json
  - Verify: - `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh`
- `test -f .tmp/m034-s05/verify/remote-runs.json`
- `test -f .tmp/m034-s05/verify/candidate-tags.json`
- `rg -n 'verify-m034-s05|v<Cargo version>|ext-v<extension version>|deploy\.yml|deploy-services\.yml|authoritative-verification\.yml|extension-release-proof\.yml|publish-extension\.yml' README.md website/docs/docs/tooling/index.md`
