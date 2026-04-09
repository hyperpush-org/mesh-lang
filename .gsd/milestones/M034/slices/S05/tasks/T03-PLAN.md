---
estimated_steps: 4
estimated_files: 8
skills_used:
  - debug-like-expert
  - test
---

# T03: Build the canonical S05 assembly verifier over S01-S04 and public HTTP truth

**Slice:** S05 — Full public release assembly proof
**Milestone:** M034

## Description

Create the one acceptance command that turns subsystem green lights into one auditable release proof. The verifier should stay serial, reuse existing slice-owned verifiers unchanged, add the S05-owned docs-truth and public-HTTP phases, and preserve first-failing-phase logs under `.tmp/m034-s05/verify/`.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Reused slice verifiers (`S01`–`S04`) | Stop at the first failing phase and keep the upstream verifier log path visible; never continue into later phases after a red prerequisite. | Treat the phase as failed and report which verifier stalled. | Treat missing status or artifact files from reused verifiers as proof failure. |
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

- [ ] `scripts/verify-m034-s05.sh` is the single assembled acceptance command for S05.
- [ ] The wrapper stays serial and reuses S01-S04 verifiers rather than re-implementing their assertions.
- [ ] Local docs truth, public HTTP truth, and the live registry publish/install proof all participate in one artifacted run.
- [ ] `.tmp/m034-s05/verify/` exposes current phase, failure status, and per-phase logs for every stage.

## Verification

- `bash -n scripts/verify-m034-s05.sh`
- `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh`
- `test -f .tmp/m034-s05/verify/current-phase.txt`
- `test -f .tmp/m034-s05/verify/status.txt && rg -n '^ok$' .tmp/m034-s05/verify/status.txt`

## Observability Impact

- Signals added/changed: the canonical `.tmp/m034-s05/verify/` artifact tree with named phases, public HTTP bodies/headers, and composed S01-S04 verifier logs.
- How a future agent inspects this: rerun `bash scripts/verify-m034-s05.sh` and inspect `.tmp/m034-s05/verify/` in phase order.
- Failure state exposed: the first failing phase, the exact reused verifier or public URL that drifted, and the retained upstream log path.

## Inputs

- `scripts/verify-m034-s05-workflows.sh` — new deploy-workflow verifier that T03 must call rather than duplicate.
- `.github/workflows/deploy.yml` — deploy contract consumed by the S05 workflow verifier.
- `.github/workflows/deploy-services.yml` — services deploy contract consumed by the S05 workflow verifier.
- `scripts/verify-m034-s02-workflows.sh` — authoritative release/workflow verifier reused unchanged.
- `scripts/verify-m034-s03.sh` — installer/release-asset verifier reused unchanged.
- `scripts/verify-m034-s04-extension.sh` — extension prepublish proof reused unchanged.
- `scripts/verify-m034-s04-workflows.sh` — extension workflow verifier reused unchanged.
- `scripts/verify-m034-s01.sh` — canonical live registry publish/install proof reused as-is.
- `README.md` — local docs-truth surface for the assembled release claim.
- `website/docs/docs/getting-started/index.md` — local docs-truth surface for deployed getting-started guidance.
- `website/docs/docs/tooling/index.md` — local docs-truth surface for package-manager and extension guidance.
- `website/docs/public/install.sh` — exact installer content that public HTTP checks must match.
- `website/docs/public/install.ps1` — exact Windows installer content that public HTTP checks must match.
- `tools/editors/vscode-mesh/package.json` — extension metadata truth surface included in the local docs sweep.

## Expected Output

- `scripts/verify-m034-s05.sh` — canonical serial S05 assembly verifier.
- `.tmp/m034-s05/verify/current-phase.txt` — current phase marker for the running verifier.
- `.tmp/m034-s05/verify/status.txt` — final verifier status file used by slice-level acceptance.
- `.tmp/m034-s05/verify/public-http.log` — persisted exact-content HTTP check output for deployed docs/packages surfaces.
