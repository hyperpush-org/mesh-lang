---
estimated_steps: 4
estimated_files: 6
skills_used:
  - bash-scripting
  - test
  - github-workflows
---

# T02: Materialize a clean `hyperpush-mono` repo tree with product-owned root surfaces

Make extraction fail closed instead of ad hoc. This task should add a materializer that stages a clean `hyperpush-mono` repo from the product-owned source set, publishes the product-owned root files the extracted repo needs, and proves the staged tree excludes local state while preserving the Mesher and landing surfaces that belong there.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| extraction materializer allowlist/exclude rules | Stop on the first missing required product path and leave the staged workspace root for inspection. | Fail within the local staging budget instead of leaving a half-written repo tree behind. | Treat leaked `.git`, `node_modules`, `.next`, `.env.local`, or build outputs as extraction drift. |
| product-root templates (`README`, landing verifier/workflow, dependabot) | Fail if the staged repo omits a required root surface or keeps mesh-lang-local wording. | N/A for source-only files. | Treat malformed root files as a real product-repo contract break. |
| staged product verifier entrypoints | Fail if the extracted repo cannot run `mesher/scripts/verify-maintainer-surface.sh` or the new landing surface verifier from the product root. | Use the delegated verifier timeout budget and keep the failing staged workspace. | Treat missing root script paths or malformed bundle pointers as product-surface drift. |

## Load Profile

- **Shared resources**: `.tmp/m055-s04/workspace/`, the staged product tree, and delegated `.tmp/m051-s01/verify/` artifacts.
- **Per-operation cost**: one materialization replay plus one staged product verifier replay.
- **10x breakpoint**: file churn and Docker-backed runtime verification, not CPU.

## Negative Tests

- **Malformed inputs**: nested `.git`, `node_modules`, `.next`, `.env.local`, `mesher/mesher`, or `mesher/mesher.ll` leaking into the staged repo.
- **Error paths**: the staged repo exists, but required root files or verifier entrypoints are missing or point back into mesh-lang.
- **Boundary conditions**: the staged tree still contains `mesher/README.md`, `mesher/scripts/verify-maintainer-surface.sh`, and `mesher/landing/`, but no local-state debris.

## Steps

1. Add tracked product-root templates for the extracted repo’s root README, landing verifier, landing deploy workflow, and dependabot config.
2. Implement `scripts/materialize-hyperpush-mono.mjs` with fail-closed `--check`/`--write` staging semantics, an explicit allowlist/exclude set, and a standard output root at `.tmp/m055-s04/workspace/hyperpush-mono`.
3. Add `scripts/tests/verify-m055-s04-materialize.test.mjs` to fail on missing required product surfaces, leaked local state, or malformed staged bundle metadata.
4. Prove the staged product repo can run the product-owned Mesher verifier and the new landing surface verifier from the extracted repo root.

## Must-Haves

- [ ] Extraction is explicit and fail-closed; it does not recursively copy local state or nested repos.
- [ ] The staged `hyperpush-mono` tree includes the product-owned root surfaces it needs: root README, landing verifier/workflow, dependabot metadata, and the existing Mesher maintainer verifier under `mesher/`.
- [ ] `node scripts/materialize-hyperpush-mono.mjs --check` refreshes a trustworthy staged product repo under `.tmp/m055-s04/workspace/hyperpush-mono`.

## Inputs

- `mesher/README.md`
- `mesher/scripts/verify-maintainer-surface.sh`
- `mesher/landing/lib/external-links.ts`
- `.github/dependabot.yml`
- `WORKSPACE.md`
- `scripts/tests/verify-m055-s04-contract.test.mjs`

## Expected Output

- `scripts/materialize-hyperpush-mono.mjs`
- `scripts/tests/verify-m055-s04-materialize.test.mjs`
- `scripts/fixtures/m055-s04-hyperpush-root/README.md`
- `scripts/fixtures/m055-s04-hyperpush-root/.github/workflows/deploy-landing.yml`
- `scripts/fixtures/m055-s04-hyperpush-root/.github/dependabot.yml`
- `scripts/fixtures/m055-s04-hyperpush-root/scripts/verify-landing-surface.sh`

## Verification

node --test scripts/tests/verify-m055-s04-materialize.test.mjs
node scripts/materialize-hyperpush-mono.mjs --check
bash .tmp/m055-s04/workspace/hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh
bash .tmp/m055-s04/workspace/hyperpush-mono/scripts/verify-landing-surface.sh

## Observability Impact

- Signals added/changed: staged-tree manifest/output root, excluded-path failures, and retained staged product verifier logs.
- How a future agent inspects this: rerun the materializer in `--check` mode, then open `.tmp/m055-s04/workspace/hyperpush-mono` and the retained verifier logs under that staged root.
- Failure state exposed: leaked local-state paths, missing product-root templates, malformed staged repo shape, or product verifier boot drift.
