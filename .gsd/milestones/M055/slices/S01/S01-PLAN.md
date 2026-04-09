# S01: Two-Repo Boundary & GSD Authority Contract

**Goal:** Make the two-repo ownership model, local workspace layout, repo identity, and repo-local GSD authority explicit before any extraction so later moves operate through named contracts instead of folklore.
**Demo:** After this: After this, the current tree exposes one blessed sibling-repo workspace, one canonical repo-identity source, and repo-local versus cross-repo GSD rules, with drift checks that fail on stale monorepo paths or GitHub slugs.

## Tasks
- [x] **T01: Published WORKSPACE.md plus repo-root maintainer docs that define the M055 two-repo split and repo-local .gsd authority.** — Make the two-repo working shape visible from repo root before any extraction starts. This task should turn D428 and D429 into durable maintainer-facing contract text: M055 is a two-repo split only, `mesh-lang` keeps docs/installers/registry/packages/public-site surfaces, and `hyperpush-mono` is the product repo that will absorb `mesher/`.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `WORKSPACE.md` contract | fail closed on missing two-repo layout, missing ownership matrix, or missing repo-local `.gsd` rule | N/A for source assertions | treat four-repo wording or umbrella-`.gsd` language as contract drift |
| `scripts/tests/verify-m055-s01-contract.test.mjs` | keep exact required markers aligned with shipped docs | bounded local test only | treat stale allowlists or missing forbidden markers as drift |

## Negative Tests

- **Malformed inputs**: `mesh-packages` or `mesh-website` presented as sibling repos for M055, missing `hyperpush-mono` handoff, or missing repo-local `.gsd` authority wording.
- **Error paths**: README or CONTRIBUTING mention the split without linking to the durable workspace contract.
- **Boundary conditions**: packages site, registry, docs, and installers stay explicitly language-owned inside `mesh-lang` for this milestone.

## Steps

1. Create `WORKSPACE.md` with the blessed sibling layout, ownership matrix, repo-local-versus-cross-repo GSD rule, and the named coordination-layer boundary.
2. Update `README.md` and `CONTRIBUTING.md` so maintainers can discover the workspace contract from the top-level entrypoints instead of reading milestone artifacts.
3. Refresh `.gsd/PROJECT.md` so current-state text stops implying the old monorepo is the durable shape.
4. Add `scripts/tests/verify-m055-s01-contract.test.mjs` with exact marker checks for the two-repo layout, repo-local `.gsd` authority, and forbidden four-repo or umbrella wording.

## Must-Haves

- [ ] `WORKSPACE.md` names only `mesh-lang` and `hyperpush-mono` as the M055 sibling repos.
- [ ] `WORKSPACE.md` explicitly says `website/`, `packages-website/`, `registry/`, and installers remain language-owned inside `mesh-lang` for this milestone.
- [ ] Repo-local `.gsd` stays authoritative; cross-repo work is documented as a lightweight coordination layer rather than one umbrella milestone tree.
- [ ] `README.md`, `CONTRIBUTING.md`, and `.gsd/PROJECT.md` all point at the same split contract.
  - Estimate: 90m
  - Files: WORKSPACE.md, README.md, CONTRIBUTING.md, .gsd/PROJECT.md, scripts/tests/verify-m055-s01-contract.test.mjs
  - Verify: - `node --test scripts/tests/verify-m055-s01-contract.test.mjs`
- [x] **T02: Added scripts/lib/repo-identity.json and rewired installer/docs contract checks to validate against it while preserving source/public installer parity.** — Create the durable repo-identity source and make the language-owned installer surfaces validate against it instead of hand-copying repo slug assumptions. This task should keep the language repo identity explicit without pretending the product repo shares the same public URLs.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/lib/repo-identity.json` | fail on missing mesh-lang or hyperpush-mono identity fields before touching consumers | N/A for source assertions | treat malformed JSON or missing URL fields as a real contract break |
| Installer source/copy pair | stop on the first parity mismatch between `tools/install/` and `website/docs/public/` | N/A for source assertions | treat stale slug or diverged copy text as installer-ownership drift |
| `scripts/lib/m034_public_surface_contract.py` | keep the existing local-docs surface green while moving repo-identity data out of hand-copied constants | bounded local command only | treat fallback hardcoding or mismatched expected URLs as public-surface drift |

## Load Profile

- **Shared resources**: the installer source/copy pairs, the local-docs contract helper, and the slice-owned Node contract.
- **Per-operation cost**: one JSON contract plus a handful of source assertions and parity edits.
- **10x breakpoint**: repeated installer-copy diffs and local-docs replays dominate long before file size or CPU becomes relevant.

## Negative Tests

- **Malformed inputs**: stale `snowdamiz/mesh-lang` hardcoding where the new contract should own the value, missing `hyperpush-mono` identity fields, or diverged `tools/install` vs `website/docs/public` bytes.
- **Error paths**: the JSON contract exists, but `scripts/lib/m034_public_surface_contract.py` still embeds a second repo-identity copy and silently disagrees with it.
- **Boundary conditions**: `tools/install/install.{sh,ps1}` stay the editable source pair while `website/docs/public/install.{sh,ps1}` remain mirrored public copies with exact parity.

## Steps

1. Add `scripts/lib/repo-identity.json` with the canonical language-repo and product-repo slugs, repo URLs, issue URLs, installer roots, docs roots, and blob-base values that later slices can consume.
2. Update `scripts/lib/m034_public_surface_contract.py` so the local-docs contract reads or validates against that canonical identity data instead of owning another hand-copied repo slug table.
3. Keep `tools/install/install.sh` and `tools/install/install.ps1` as the editable installer sources, update the mirrored `website/docs/public/install.sh` and `website/docs/public/install.ps1` copies in the same task, and fail closed on parity drift.
4. Extend `scripts/tests/verify-m055-s01-contract.test.mjs` so it catches malformed repo-identity data, stale hardcoded installer slugs, and source/copy mismatches.

## Must-Haves

- [ ] `scripts/lib/repo-identity.json` is the only new canonical repo-identity data file introduced by this slice.
- [ ] `scripts/lib/m034_public_surface_contract.py` no longer owns a conflicting repo-identity copy for installer/docs metadata.
- [ ] `tools/install/install.{sh,ps1}` and `website/docs/public/install.{sh,ps1}` stay byte-parity and match the canonical identity contract.
- [ ] The slice-owned Node contract fails on missing mesh-lang or hyperpush-mono repo identity markers.
  - Estimate: 2h
  - Files: scripts/lib/repo-identity.json, scripts/lib/m034_public_surface_contract.py, tools/install/install.sh, tools/install/install.ps1, website/docs/public/install.sh, website/docs/public/install.ps1, scripts/tests/verify-m055-s01-contract.test.mjs
  - Verify: - `diff -u tools/install/install.sh website/docs/public/install.sh`
- `diff -u tools/install/install.ps1 website/docs/public/install.ps1`
- `python3 scripts/lib/m034_public_surface_contract.py local-docs --root .`
- `node --test scripts/tests/verify-m055-s01-contract.test.mjs`
- [x] **T03: Made packages, landing, and VS Code identity surfaces explicit about the M055 language-vs-product repo split.** — Use the new identity contract on the first public boundary surfaces that already drift today: language-owned packages/editor metadata versus product-owned landing links. This task should make the split visible in real user-facing surfaces, not just in contract files.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `packages-website` footer surface | fail the build on broken Svelte/template edits or missing language-repo link markers | use the normal build timeout and stop on the first error | treat a product-repo GitHub link in the packages footer as public-surface drift |
| `mesher/landing` external links | fail closed on TypeScript errors or missing product-repo markers | bounded local typecheck only | treat language-repo fallback links in product CTA surfaces as identity drift |
| VS Code package metadata | stop on missing repository or bugs URLs for the language repo | N/A for JSON edits | treat stale or product-owned URLs as extension-metadata drift |

## Load Profile

- **Shared resources**: `packages-website/node_modules`, `mesher/landing/node_modules`, and the slice-owned Node contract.
- **Per-operation cost**: one Svelte build, one landing typecheck, and a few metadata/file edits.
- **10x breakpoint**: frontend builds and typechecks dominate before the contract test does.

## Negative Tests

- **Malformed inputs**: the packages footer points at `hyperpush-mono`, landing external links point back at `mesh-lang`, or VS Code metadata mixes the two repos.
- **Error paths**: the repo-identity contract is correct but one of the user-facing surfaces still carries a stale hardcoded URL outside the contract test allowlist.
- **Boundary conditions**: language-owned public surfaces keep the language repo identity while product-owned landing surfaces keep the product repo identity.

## Steps

1. Update `packages-website/src/routes/+layout.svelte` so the footer uses the language-owned Mesh repo identity and not the product repo.
2. Update `mesher/landing/lib/external-links.ts` so the landing site keeps the product-owned `hyperpush-mono` identity explicit and does not drift back to the language repo.
3. Keep `tools/editors/vscode-mesh/package.json` on the language-repo metadata contract and extend `scripts/tests/verify-m055-s01-contract.test.mjs` so these three user-facing surfaces fail closed if they mix repo identities.
4. Rebuild the packages site and typecheck the landing app so the split-aware public identity is proven in real consumer surfaces instead of only in docs/tests.

## Must-Haves

- [ ] `packages-website/src/routes/+layout.svelte` points GitHub/docs-maintainer links at the language repo where appropriate.
- [ ] `mesher/landing/lib/external-links.ts` keeps the product landing CTA on the product repo identity.
- [ ] `tools/editors/vscode-mesh/package.json` still presents the language repo and issue tracker as the editor host source of truth.
- [ ] `scripts/tests/verify-m055-s01-contract.test.mjs` fails on any of those surfaces using the wrong repo identity.
  - Estimate: 90m
  - Files: packages-website/src/routes/+layout.svelte, mesher/landing/lib/external-links.ts, tools/editors/vscode-mesh/package.json, scripts/tests/verify-m055-s01-contract.test.mjs
  - Verify: - `npm --prefix packages-website run build`
- `./mesher/landing/node_modules/.bin/tsc --noEmit -p mesher/landing/tsconfig.json`
- `node --test scripts/tests/verify-m055-s01-contract.test.mjs`
- [x] **T04: Added the assembled M055 split-boundary verifier and documented its repo-local `.gsd` debug path.** — Finish the slice with one named verifier that proves the workspace contract, repo-identity contract, and existing repo-local `.gsd` dependency still hold together. This task should leave the next slice with one obvious stop/go command and one obvious failure-inspection path.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m055-s01.sh` child phases | stop on the first failing contract, build, or cargo phase and record it in `phase-report.txt` | mark the timed-out phase explicitly and fail closed | treat missing phase markers, missing logs, or skipped child checks as verifier drift |
| `compiler/meshc/tests/e2e_m046_s03.rs` named cargo rail | fail closed if the repo-local `.gsd` contract no longer matches what the shipped tiny-cluster test expects | use a bounded cargo invocation and preserve the raw log | treat `running 0 test` or missing `S03-PLAN.md` assertions as a real regression |
| `WORKSPACE.md` / `CONTRIBUTING.md` verifier docs | keep the named top-level verifier command discoverable from maintainer-facing docs | N/A for source assertions | treat undocumented verifier entrypoints as workflow drift |

## Load Profile

- **Shared resources**: `.tmp/m055-s01/verify/`, the packages-site build cache, the landing typecheck, and the named M046 cargo target.
- **Per-operation cost**: one shell wrapper, one cargo test, one packages build, one landing typecheck, and one Node contract replay.
- **10x breakpoint**: repeated frontend builds and the cargo regression check dominate before the shell wrapper logic does.

## Negative Tests

- **Malformed inputs**: stale monorepo-path wording, missing repo-identity markers, or a wrapper that forgets to run the repo-local `.gsd` cargo regression.
- **Error paths**: the direct Node contract passes, but the assembled wrapper omits phase markers or hides which child command failed.
- **Boundary conditions**: the final S01 command proves only the slice contract and the existing `.gsd` regression seam; it should not grow into the later S02 toolchain or S03 public-surface assembly story.

## Steps

1. Add `scripts/verify-m055-s01.sh` as the authoritative S01 replay that creates `.tmp/m055-s01/verify/`, records `status.txt`, `current-phase.txt`, `phase-report.txt`, and `full-contract.log`, then runs the Node contract, installer/docs contract, packages build, landing typecheck, and the named M046 cargo regression in order.
2. Make the wrapper fail closed on missing child logs or missing test-count evidence, and keep the first failing phase obvious without requiring a broad manual grep.
3. Update `WORKSPACE.md` and `CONTRIBUTING.md` to name `bash scripts/verify-m055-s01.sh` as the top-level split-boundary verifier.
4. Record the debug entrypoint in `.gsd/KNOWLEDGE.md` so future agents know to inspect `.tmp/m055-s01/verify/phase-report.txt` first and use the named M046 cargo rail when the repo-local `.gsd` seam drifts.

## Must-Haves

- [ ] `scripts/verify-m055-s01.sh` is the named top-level verifier for this slice and writes the standard phase markers under `.tmp/m055-s01/verify/`.
- [ ] The wrapper replays the named repo-local `.gsd` regression target `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_package_contract_remains_source_first_and_route_free -- --nocapture`.
- [ ] `WORKSPACE.md` and `CONTRIBUTING.md` both point at the new verifier command.
- [ ] `.gsd/KNOWLEDGE.md` tells future agents where to start when the split-boundary verifier goes red.

## Observability Impact

- Signals added/changed: `.tmp/m055-s01/verify/status.txt`, `current-phase.txt`, `phase-report.txt`, and per-phase logs for workspace, repo-identity, packages, landing, and GSD-regression checks.
- How a future agent inspects this: run `bash scripts/verify-m055-s01.sh`, then read `.tmp/m055-s01/verify/phase-report.txt` and the failing phase log before rerunning the child command directly.
- Failure state exposed: the first failing phase, exact drifting file/marker, and the raw cargo output from the repo-local `.gsd` regression seam.
  - Estimate: 90m
  - Files: scripts/verify-m055-s01.sh, scripts/tests/verify-m055-s01-contract.test.mjs, WORKSPACE.md, CONTRIBUTING.md, .gsd/KNOWLEDGE.md
  - Verify: - `bash scripts/verify-m055-s01.sh`
