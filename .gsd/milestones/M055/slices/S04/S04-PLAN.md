# S04: `hyperpush-mono` Extraction & Two-Repo Evidence Assembly

**Goal:** Make the blessed two-repo workspace operationally real by materializing a clean `hyperpush-mono` repo from the product-owned source set, aligning mesh-lang’s compatibility and hosted-evidence rails to that sibling repo, and publishing one retained proof chain that attributes language continuity and product continuity to the correct repo/ref.
**Demo:** After this: After this, the two-repo workspace is operationally real: Hyperpush is extracted and renamed, each repo owns its own proof entrypoints, and one evidence chain can show which repo/ref proved `mesh-lang` continuity and which repo/ref proved `hyperpush-mono` continuity.

## Tasks
- [x] **T01: Locked Mesher’s toolchain/docs/tests to the nested `hyperpush-mono/mesher` workspace shape and made stale flat-sibling assumptions fail closed.** — Resolve the extracted repo-shape contradiction before any workspace handoff. The public contract already says Hyperpush lives at `hyperpush-mono/mesher/...`, but the Mesher toolchain and contract tests still assume the extracted package root sits directly next to `mesh-lang`. This task should make the nested extracted layout the only truthful product contract while preserving the current in-repo `mesh-lang/mesher` path for local development.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `mesher/scripts/lib/mesh-toolchain.sh` path resolution | Fail closed with the exact missing root/path and do not fall back to a lower-priority source tier. | N/A for local path checks. | Treat a mixed `../mesh-lang`/`../../mesh-lang` contract as resolver drift. |
| `scripts/tests/verify-m055-s02-contract.test.mjs` and `scripts/tests/verify-m055-s04-contract.test.mjs` layout simulations | Stop on the first temp-workspace contract mismatch and keep the failing fixture root. | N/A for local node:test runs. | Treat missing nested-repo markers or stale direct-sibling assumptions as real contract breaks. |

## Load Profile

- **Shared resources**: temp workspace fixtures created by the node tests and the product README/workspace contract text.
- **Per-operation cost**: local node tests and shell/path resolution only.
- **10x breakpoint**: stale path assumptions across multiple docs/tests, not CPU or memory.

## Negative Tests

- **Malformed inputs**: extracted repo rooted at `hyperpush-mono/mesher` still looking for sibling `../mesh-lang`, or repo docs still describing the wrong layout.
- **Error paths**: enclosing-source detection works, but extracted sibling-workspace detection still points at the stale path.
- **Boundary conditions**: the current in-repo `mesh-lang/mesher` flow still resolves as `enclosing-source` while the extracted nested repo resolves as the blessed sibling layout.

## Steps

1. Update `mesher/scripts/lib/mesh-toolchain.sh` so it prefers the current enclosing `mesh-lang` checkout first, then resolves the extracted nested `hyperpush-mono/mesher` layout against the sibling `mesh-lang` repo, and still keeps the installed `PATH` fallback fail-closed.
2. Rewrite `mesher/README.md` and `WORKSPACE.md` so the documented extracted shape is explicitly `mesh-lang/` beside `hyperpush-mono/`, with Mesher nested under the product repo instead of flattened at repo root.
3. Extend the existing Mesher contract test and add slice-owned S04 contract assertions so both the in-repo and extracted-nested layouts are simulated and stale `../mesh-lang` assumptions fail closed.

## Must-Haves

- [ ] The extracted product layout is explicitly `hyperpush-mono/mesher/...`, not a flattened direct-sibling package root.
- [ ] Mesher’s resolver still works from the current in-repo `mesh-lang/mesher` path and from the extracted nested product repo.
- [ ] Contract tests fail on stale direct-sibling `../mesh-lang` assumptions and missing nested-layout docs.
  - Estimate: 2h
  - Files: mesher/scripts/lib/mesh-toolchain.sh, mesher/README.md, WORKSPACE.md, scripts/tests/verify-m055-s02-contract.test.mjs, scripts/tests/verify-m055-s04-contract.test.mjs
  - Verify: node --test scripts/tests/verify-m055-s02-contract.test.mjs scripts/tests/verify-m055-s04-contract.test.mjs
- [x] **T02: Added a fail-closed hyperpush-mono materializer and staged product-root verifier surfaces.** — Make extraction fail closed instead of ad hoc. This task should add a materializer that stages a clean `hyperpush-mono` repo from the product-owned source set, publishes the product-owned root files the extracted repo needs, and proves the staged tree excludes local state while preserving the Mesher and landing surfaces that belong there.

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
  - Estimate: 3h
  - Files: scripts/materialize-hyperpush-mono.mjs, scripts/tests/verify-m055-s04-materialize.test.mjs, scripts/fixtures/m055-s04-hyperpush-root/README.md, scripts/fixtures/m055-s04-hyperpush-root/.github/workflows/deploy-landing.yml, scripts/fixtures/m055-s04-hyperpush-root/.github/dependabot.yml, scripts/fixtures/m055-s04-hyperpush-root/scripts/verify-landing-surface.sh
  - Verify: node --test scripts/tests/verify-m055-s04-materialize.test.mjs
node scripts/materialize-hyperpush-mono.mjs --check
bash .tmp/m055-s04/workspace/hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh
bash .tmp/m055-s04/workspace/hyperpush-mono/scripts/verify-landing-surface.sh
- [x] **T03: Retargeted mesh-lang compatibility and hosted-evidence verifiers to the sibling hyperpush-mono repo and the canonical repo-identity contract.** — Once the staged product repo exists, stop treating the in-repo `mesher/` tree or the current `origin` remote as authoritative. This task should rewrite mesh-lang-side compatibility and hosted-evidence rails to find the sibling product repo explicitly and to derive the language repo slug from the canonical identity contract.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| sibling-workspace resolver/helper | Fail closed on missing or ambiguous `hyperpush-mono` roots and report the exact path source (env override vs blessed sibling). | N/A for local path checks. | Treat a fallback to the in-repo `mesher/` path as contract drift. |
| `scripts/verify-m051-s01.sh` delegation | Stop before any local Mesher replay if the sibling product repo or delegated verifier path is missing. | Use the delegated product verifier timeout budget and keep the failing repo root visible. | Treat a wrapper success without the sibling repo’s markers as false proof. |
| `scripts/verify-m053-s03.sh` hosted repo identity | Fail if the default repo slug still comes from `origin` instead of `scripts/lib/repo-identity.json`. | Preserve the existing hosted-evidence timeout budgets. | Treat malformed repo-slug resolution or mixed language/product refs as hosted-proof drift. |

## Load Profile

- **Shared resources**: `.tmp/m055-s04/workspace/`, `.tmp/m051-s01/verify/`, and hosted-evidence repo/ref metadata.
- **Per-operation cost**: one staged compatibility-wrapper replay plus local contract tests.
- **10x breakpoint**: repeated verifier delegation and repo-ref drift, not CPU.

## Negative Tests

- **Malformed inputs**: only the stale in-repo `mesher/` path exists, `origin` points at the product repo, or the wrapper is given a malformed sibling root.
- **Error paths**: the product verifier is green in the staged repo, but the mesh-lang compatibility wrapper still runs the local copy; or the hosted verifier still reads the wrong slug by default.
- **Boundary conditions**: explicit env overrides still work for debugging, but the default path and default language repo slug come from the blessed workspace contract and repo identity.

## Steps

1. Add a small workspace-resolution helper so mesh-lang compatibility rails can find sibling `hyperpush-mono` from the blessed workspace layout or an explicit env override instead of assuming in-repo `mesher/`.
2. Rewrite `scripts/verify-m051-s01.sh` to delegate to the sibling product repo’s `mesher/scripts/verify-maintainer-surface.sh` and fail closed if only the stale in-repo path exists.
3. Rewrite `scripts/verify-m053-s03.sh` so the default language repo slug comes from `scripts/lib/repo-identity.json` rather than `git remote get-url origin`, while keeping explicit override support.
4. Extend the S04 contract test to pin the sibling-wrapper and hosted-evidence behavior.

## Must-Haves

- [ ] `scripts/verify-m051-s01.sh` no longer treats the in-repo `mesher/` tree as authoritative.
- [ ] `scripts/verify-m053-s03.sh` defaults to the canonical language repo slug instead of the current `origin` remote.
- [ ] Contract tests fail on hidden local delegation or repo-slug drift.
  - Estimate: 2h
  - Files: scripts/lib/m055-workspace.sh, scripts/verify-m051-s01.sh, scripts/verify-m053-s03.sh, scripts/tests/verify-m055-s04-contract.test.mjs, WORKSPACE.md
  - Verify: node --test scripts/tests/verify-m055-s04-contract.test.mjs
node scripts/materialize-hyperpush-mono.mjs --check
M055_HYPERPUSH_ROOT=.tmp/m055-s04/workspace/hyperpush-mono bash scripts/verify-m051-s01.sh
- [x] **T04: Added the assembled M055 S04 verifier that stages hyperpush-mono, replays both repo-local proof chains, and retains per-repo attribution metadata in one bundle.** — Close the loop with one assembled two-repo proof rail. This task should materialize or validate the staged sibling workspace, run the language-owned and product-owned proof entrypoints from their own repo roots, and retain a single S04 bundle that records which repo/ref and which proof bundle belonged to each side.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| staged workspace materializer | Stop on the first staging failure and preserve the workspace root plus manifest. | Fail within the staging budget instead of leaving partial repo trees behind. | Treat missing product-root files or leaked local state as assembly drift. |
| language-owned `bash scripts/verify-m055-s03.sh` and product-owned verifier entrypoints | Stop on the first failing delegated phase and retain each delegated `.tmp/.../verify/` pointer. | Use the delegated timeout budgets and surface the exact failing phase. | Treat missing `latest-proof-bundle.txt` pointers or malformed delegated markers as contract breaks. |
| assembled `.tmp/m055-s04/verify/` bundle | Fail if phase markers, repo/ref metadata, or retained bundle pointers are missing or contradictory. | Stop on the first bundle-shape mismatch and keep the failing assembly log. | Treat mixed language/product repo attribution as false evidence. |

## Load Profile

- **Shared resources**: `.tmp/m055-s04/workspace/`, `.tmp/m055-s04/verify/`, delegated `.tmp/m055-s03/verify/`, and delegated product verifier artifacts.
- **Per-operation cost**: one staged workspace refresh, one product-owned verifier replay, one language-owned verifier replay, and one retained bundle copy.
- **10x breakpoint**: verifier runtime and retained artifact churn, not CPU.

## Negative Tests

- **Malformed inputs**: missing sibling repo, missing delegated bundle pointer, or repo/ref metadata captured from the wrong repo root.
- **Error paths**: both delegated verifiers can pass independently, but the S04 wrapper fails because it cannot attribute repo/ref or copy the retained bundles truthfully.
- **Boundary conditions**: the wrapper may use env overrides for debugging, but the published S04 bundle must still record both repo identities and both proof-bundle pointers explicitly.

## Steps

1. Add `scripts/verify-m055-s04.sh` to refresh the staged sibling workspace, run the product-owned verifier entrypoints from `hyperpush-mono`, run the language-owned `bash scripts/verify-m055-s03.sh` with the canonical language repo slug, and stop on the first failing phase.
2. Capture product and language repo/ref metadata, delegated bundle pointers, and copied retained verifier trees into `.tmp/m055-s04/verify/`.
3. Extend the S04 contract test to pin the assembled wrapper’s phase order, repo/ref metadata fields, and retained bundle shape.
4. Replay the full S04 wrapper and inspect the retained bundle rather than raw delegated `.tmp` trees.

## Must-Haves

- [ ] One S04 bundle shows both repo identities/refs and the proof bundle pointer for each repo.
- [ ] The assembled wrapper only succeeds when both the language-owned and product-owned proof entrypoints pass from their own repo roots.
- [ ] `.tmp/m055-s04/verify/` publishes the standard `status.txt`, `current-phase.txt`, `phase-report.txt`, `full-contract.log`, and `latest-proof-bundle.txt` surfaces.

  - Estimate: 2h
  - Files: scripts/verify-m055-s04.sh, scripts/tests/verify-m055-s04-contract.test.mjs, scripts/materialize-hyperpush-mono.mjs, scripts/lib/m055-workspace.sh, scripts/verify-m055-s03.sh, scripts/verify-m053-s03.sh
  - Verify: node --test scripts/tests/verify-m055-s04-contract.test.mjs
bash scripts/verify-m055-s04.sh
