---
estimated_steps: 4
estimated_files: 3
skills_used:
  - github-workflows
  - test
---

# T03: Wire fail-closed workflows around the verified VSIX

**Slice:** S04 — Extension release path hardening
**Milestone:** M034

## Description

Finish the slice by making GitHub Actions a thin caller around the repo-local verifier instead of another place where release truth can drift. Following the S02 pattern, add a reusable extension-proof workflow, make the tag-triggered publish lane consume the exact verified VSIX, and back the workflow contract with a repo-local verifier so future edits cannot quietly restore globbing or partial-success publication.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Reusable workflow ↔ publish workflow handoff | Block publication and preserve the missing output/artifact name in logs; never repackage ad hoc in the publish job. | Fail the caller job and keep the proof job/artifact status visible. | Treat missing workflow outputs, wrong artifact names, or a publish job that repackages independently as contract drift. |
| Marketplace publish actions | Fail closed if either Open VSX or VS Marketplace publish fails; do not mark the lane green after partial success. | Let the publish step timeout fail the job and keep the already-verified VSIX path visible for reruns. | Treat missing tokens, wrong `extensionFile`, or registry-specific config drift as publish-lane failure. |
| Local workflow contract verifier | Reject trigger, permission, reusable-call, diagnostics-upload, or `extensionFile` drift before CI. | N/A | Treat `continue-on-error`, `ls *.vsix`, or direct verifier execution from the publish workflow as broken contract. |

## Load Profile

- **Shared resources**: GitHub runner minutes, workflow artifacts, and marketplace publication rate limits/tokens.
- **Per-operation cost**: one reusable proof job, one artifact/output handoff, and two publish actions against different registries.
- **10x breakpoint**: runner queueing and artifact handoff would dominate first, so the publish job must reuse the exact verified VSIX instead of recompiling or repackaging.

## Negative Tests

- **Malformed inputs**: stale trigger comment/tag example, missing reusable workflow output, `continue-on-error`, or a caller workflow that still resolves `*.vsix` via globbing.
- **Error paths**: reusable proof failure, missing diagnostics artifact, or one-registry publish failure must each keep the overall lane red.
- **Boundary conditions**: both registries must publish the same verified VSIX file, and the publish workflow must not call `bash scripts/verify-m034-s04-extension.sh` directly if the reusable workflow owns that proof.

## Steps

1. Add `.github/workflows/extension-release-proof.yml` as a reusable workflow that checks out the repo, sets up the toolchain needed by `scripts/verify-m034-s04-extension.sh`, runs that verifier exactly once, and uploads `.tmp/m034-s04/verify/**` on failure.
2. Rewrite `.github/workflows/publish-extension.yml` so the tag lane calls the reusable proof workflow, consumes the exact verified VSIX path or artifact it emits, updates the trigger comment/example, and removes `continue-on-error` plus `ls *.vsix` selection.
3. Add `scripts/verify-m034-s04-workflows.sh` to parse both workflow files and mechanically enforce the reusable-owner pattern, exact verifier invocation, diagnostics retention, deterministic `extensionFile`, fail-closed dual-market publication, and the absence of globbing / inline proof logic.
4. Rerun the local workflow verifier plus YAML parse sweep until workflow drift is rejected mechanically before CI.

## Must-Haves

- [ ] The reusable proof workflow is the only workflow file that directly runs `bash scripts/verify-m034-s04-extension.sh`
- [ ] The tag-triggered publish workflow depends on reusable proof and publishes the exact verified VSIX to both registries
- [ ] Partial publication cannot pass green; `continue-on-error` and `ls *.vsix` are gone
- [ ] `scripts/verify-m034-s04-workflows.sh` mechanically rejects contract drift before CI

## Verification

- `bash scripts/verify-m034-s04-workflows.sh`
- `ruby -e 'require "yaml"; %w[.github/workflows/extension-release-proof.yml .github/workflows/publish-extension.yml].each { |f| YAML.load_file(f) }'`
- `! rg -n 'continue-on-error|ls \*\.vsix|bash scripts/verify-m034-s04-extension.sh' .github/workflows/publish-extension.yml`
- `rg -n './.github/workflows/extension-release-proof.yml|extensionFile:' .github/workflows/publish-extension.yml`

## Observability Impact

- Signals added/changed: reusable-workflow proof logs plus uploaded `.tmp/m034-s04/verify/**` diagnostics and a local workflow-contract verifier phase report.
- How a future agent inspects this: rerun `bash scripts/verify-m034-s04-workflows.sh`, inspect reusable-workflow artifacts, and read publish job logs for the exact `extensionFile` used.
- Failure state exposed: proof-job failure, output/artifact handoff drift, missing diagnostics, or the first registry publish failure.

## Inputs

- `.github/workflows/publish-extension.yml`
- `.github/workflows/authoritative-live-proof.yml`
- `scripts/verify-m034-s02-workflows.sh`
- `scripts/verify-m034-s04-extension.sh`

## Expected Output

- `.github/workflows/extension-release-proof.yml`
- `.github/workflows/publish-extension.yml`
- `scripts/verify-m034-s04-workflows.sh`
