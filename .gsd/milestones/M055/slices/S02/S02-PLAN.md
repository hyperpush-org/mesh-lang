# S02: Hyperpush Toolchain Contract Outside `mesh-lang`

**Goal:** Make Mesher operationally believable outside the current `mesh-lang` repo root by giving it a product-owned toolchain contract, package-local maintainer scripts, a delegated compatibility verifier, and a language-owned public handoff that treats those product surfaces as the deeper app story.
**Demo:** After this: After this, the deeper reference app can be built, tested, migrated, and explained from the blessed sibling workspace against an explicit `mesh-lang` toolchain contract, so `hyperpush-mono` is operationally believable before extraction.

## Tasks
- [x] **T01: Added Mesher-owned meshc resolution and package-local test/migrate/build/smoke scripts with outside-package staging.** — Make the product toolchain contract real before touching docs or wrappers. This task should add a Mesher-owned script layer that treats `mesher/` as a normal Mesh package, resolves `meshc` explicitly, and stages outputs outside the tracked source tree so the same surface can move into `hyperpush-mono` later.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `mesher/scripts/lib/mesh-toolchain.sh` resolver | fail closed with one explicit message that says whether the missing contract is the enclosing source checkout, blessed sibling workspace, or `PATH` fallback | N/A for source assertions | treat an unreadable or contradictory resolved path as toolchain-contract drift |
| package-local `meshc` wrappers (`test`, `migrate`, `build`, `smoke`) | stop on the first command failure and print the exact package-local command that drifted instead of silently retrying with repo-root fallbacks | use bounded command execution and stop before leaving partial staged outputs behind | fail closed if build outputs land inside `mesher/` or if migration/smoke wrappers accept unsupported subcommands |
| staged bundle path under `.tmp/` | reject missing or source-tree output paths before build time | N/A for path validation | treat malformed output-path input as a contract error rather than normalizing it |

## Load Profile

- **Shared resources**: `target/debug/meshc`, `.tmp/m055-s02/`, and the package-local `mesher/` source tree.
- **Per-operation cost**: one toolchain probe plus package-local test/build command wrappers; smoke remains lightweight but still touches real runtime env.
- **10x breakpoint**: repeated build staging and accidental source-tree output churn, not CPU or file size.

## Negative Tests

- **Malformed inputs**: missing enclosing/sibling `mesh-lang`, missing `meshc` on `PATH`, unsupported `migrate` subcommands, and output paths that point back into `mesher/`.
- **Error paths**: a valid `meshc` exists, but the script silently falls back to repo-root `cargo run -q -p meshc -- ...` commands instead of the package-local contract.
- **Boundary conditions**: nested-in-`mesh-lang` development, blessed sibling-workspace development, and installed-CLI fallback all choose the right `meshc` source.

## Steps

1. Add `mesher/scripts/lib/mesh-toolchain.sh` to resolve `meshc` from an enclosing `mesh-lang` checkout, then the blessed sibling `../mesh-lang`, then `PATH`, and emit a fail-closed message when none exist.
2. Add package-local `mesher/scripts/test.sh`, `mesher/scripts/migrate.sh`, `mesher/scripts/build.sh`, and `mesher/scripts/smoke.sh` that run `meshc test tests`, `meshc migrate . status|up`, and `meshc build . --output ...` from the package root while keeping binaries and staged artifacts outside `mesher/`.
3. Create `scripts/tests/verify-m055-s02-contract.test.mjs` with exact assertions for the new script names, resolution-order markers, and the absence of repo-root `cargo run -q -p meshc -- ... mesher` fallbacks in the product-owned script layer.
4. Reuse the retained backend fixture patterns from `scripts/fixtures/backend/reference-backend/scripts/` instead of inventing a new staging/smoke shape.

## Must-Haves

- [ ] Mesher has one package-owned `meshc` resolver that makes the chosen source explicit.
- [ ] Package-local test/migrate/build/smoke scripts exist under `mesher/scripts/` and do not write binaries into `mesher/`.
- [ ] The new slice-owned contract test fails on stale repo-root commands or missing resolver markers.
  - Estimate: 1 context window
  - Files: mesher/scripts/lib/mesh-toolchain.sh, mesher/scripts/test.sh, mesher/scripts/migrate.sh, mesher/scripts/build.sh, mesher/scripts/smoke.sh, scripts/tests/verify-m055-s02-contract.test.mjs
  - Verify: - `node --test scripts/tests/verify-m055-s02-contract.test.mjs`
- `bash mesher/scripts/test.sh`
- `bash mesher/scripts/build.sh .tmp/m055-s02/mesher-build`
- [x] **T02: Moved the deeper Mesher proof rail into a package-owned verifier and reduced the repo-root M051 rail to a delegation-checked compatibility wrapper.** — Once the package-local scripts exist, make the deeper Mesher proof surface use them. This task should create the product-owned verifier under `mesher/`, refactor the mesh-lang-side Rust helper/e2e support to drive that contract, and shrink the repo-root `scripts/verify-m051-s01.sh` rail into a compatibility wrapper instead of the authoritative implementation.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `mesher/scripts/verify-maintainer-surface.sh` child phases | stop on the first failing package-local command and preserve the exact failing phase under `.tmp/m051-s01/verify/` | mark timed-out phases explicitly and stop before reusing stale bundles | fail closed if phase markers, bundle pointers, or child test-count evidence are malformed |
| `compiler/meshc/tests/support/m051_mesher.rs` delegation | fail the Rust rail if the helper still assumes `migrate mesher` / `build mesher` or repo-root `source_package_dir` semantics | treat hung command helpers as contract drift and retain the raw child logs | fail closed if helper metadata still records repo-root package assumptions instead of the package-local contract |
| repo-root compatibility wrapper | fail if the wrapper stops delegating to the package-owned verifier or if it mutates the retained `.tmp/m051-s01/verify/` shape | use the delegated verifier timeout budget and surface the wrapped phase | fail closed if the wrapper reports success without the delegated verifier’s markers |

## Load Profile

- **Shared resources**: `.tmp/m051-s01/verify/`, Docker-backed temporary Postgres from the existing e2e rail, `target/debug/meshc`, and retained Mesher startup artifacts.
- **Per-operation cost**: one full Mesher e2e replay plus one product-owned verifier replay and one compatibility-wrapper replay.
- **10x breakpoint**: verifier artifact churn and Docker/Postgres startup, not CPU.

## Negative Tests

- **Malformed inputs**: stale repo-root command strings, missing phase markers, missing retained bundle pointer, or helper metadata still naming `repo_root().join("mesher")` as the source contract.
- **Error paths**: the package-owned verifier passes, but the repo-root wrapper silently bypasses it; or the Rust rail still composes raw repo-root `meshc` commands instead of the product-owned scripts.
- **Boundary conditions**: missing `DATABASE_URL` proof stays fail-closed, runtime smoke still redacts secrets, and the retained artifact bundle remains stable across the delegation.

## Steps

1. Add `mesher/scripts/verify-maintainer-surface.sh` as the authoritative Mesher replay with phase markers, child-log retention, and package-local test/build/migrate/smoke phases.
2. Refactor `compiler/meshc/tests/support/m051_mesher.rs` so the mesh-lang-side e2e helper drives the package-owned Mesher contract, records package-root metadata, and stops hardcoding `migrate mesher` / `build mesher` / repo-root source paths.
3. Update `compiler/meshc/tests/e2e_m051_s01.rs` so its exact-string and runtime assertions pin the new product-owned verifier and compatibility-wrapper story.
4. Rewrite `scripts/verify-m051-s01.sh` into a compatibility wrapper that delegates to `bash mesher/scripts/verify-maintainer-surface.sh` while preserving the expected `.tmp/m051-s01/verify/` inspection surface.

## Must-Haves

- [ ] `bash mesher/scripts/verify-maintainer-surface.sh` is the authoritative deeper Mesher proof command.
- [ ] The mesh-lang-side Rust helper and e2e rail prove the package-owned contract rather than re-encoding repo-root Mesher commands.
- [ ] `bash scripts/verify-m051-s01.sh` survives only as a delegated compatibility wrapper with the same retained artifact surface.
  - Estimate: 1 context window
  - Files: mesher/scripts/verify-maintainer-surface.sh, scripts/verify-m051-s01.sh, compiler/meshc/tests/support/m051_mesher.rs, compiler/meshc/tests/e2e_m051_s01.rs
  - Verify: - `cargo test -p meshc --test e2e_m051_s01 -- --nocapture`
- `bash mesher/scripts/verify-maintainer-surface.sh`
- `bash scripts/verify-m051-s01.sh`
- [x] **T03: Rewrote the Mesher maintainer runbook around package-local commands and pinned it with the slice-owned contract test.** — After the scripts and verifier exist, make the maintainer runbook tell the same story. This task should rewrite `mesher/README.md` around the explicit toolchain contract and package-local commands, keep the runtime env and seed-data guidance truthful, and pin that contract with the slice-owned Node test instead of leaving it as prose.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `mesher/README.md` contract | fail closed on missing toolchain-order wording, missing package-local commands, or stale repo-root `cargo run -q -p meshc -- ... mesher` examples | N/A for source assertions | treat contradictory primary/secondary verifier wording as runbook drift |
| `scripts/tests/verify-m055-s02-contract.test.mjs` | stop on the first missing marker or forbidden legacy command | bounded local test only | treat accidental wording broadening or missing exact commands as contract drift |

## Negative Tests

- **Malformed inputs**: missing sibling/enclosing/PATH toolchain explanation, stale `./mesher/mesher` run instructions, or missing product-owned verifier command.
- **Error paths**: the runbook names the compatibility wrapper as primary again, or reintroduces repo-root `migrate mesher` / `build mesher` commands from memory.
- **Boundary conditions**: startup env, seed data, and runtime inspection commands stay truthful while the command shape changes underneath them.

## Steps

1. Rewrite `mesher/README.md` so the maintainer loop starts from the package root, explains the explicit `meshc` resolution order, and uses `bash scripts/test.sh`, `bash scripts/migrate.sh status|up`, `bash scripts/build.sh <artifact-dir>`, and `bash scripts/smoke.sh` instead of repo-root cargo commands.
2. Keep the startup env, seeded default org/project/API key, and runtime inspection sections accurate; only the toolchain/runbook contract should change.
3. Make the product-owned verifier command primary in the README and frame `bash scripts/verify-m051-s01.sh` as the mesh-lang compatibility replay.
4. Extend `scripts/tests/verify-m055-s02-contract.test.mjs` so it pins the new README markers and forbids the old repo-root maintainer loop.

## Must-Haves

- [ ] `mesher/README.md` teaches the explicit toolchain contract and package-local commands.
- [ ] The product-owned Mesher verifier is the primary deeper-app proof command in the runbook.
- [ ] The slice-owned Node contract fails on stale repo-root Mesher command examples.
  - Estimate: 90m
  - Files: mesher/README.md, mesher/.env.example, scripts/tests/verify-m055-s02-contract.test.mjs
  - Verify: - `node --test scripts/tests/verify-m055-s02-contract.test.mjs`
- `bash mesher/scripts/test.sh`
- [x] **T04: Updated the Production Backend Proof page to hand deeper Mesher verification to the product-owned Mesher contract and demoted the repo-root M051 rail to compatibility-only.** — Finish the slice by updating the public-secondary handoff instead of the public first-contact path. This task should keep `production-backend-proof` scaffold/examples-first, but make it route maintainers to the product-owned Mesher runbook/verifier contract and treat the repo-root `mesh-lang` verifier as compatibility-only.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `website/docs/docs/production-backend-proof/index.md` markers | fail closed on stale repo-root Mesher commands, missing product-owned handoff markers, or lost examples-first ordering | N/A for source assertions | treat mixed public-vs-maintainer routing or stale blob links as docs drift |
| `scripts/verify-production-proof-surface.sh` | stop on the first ordering or marker regression and preserve the failing description | bounded local shell verifier only | treat missing or malformed exact-string checks as proof-surface drift |
| S01 split-boundary contract | fail if the updated docs now contradict `WORKSPACE.md` about language-owned versus product-owned surfaces | bounded local shell verifier only | treat repo-ownership wording drift as a real split-contract regression |

## Negative Tests

- **Malformed inputs**: public docs that promote repo-root Mesher commands again, missing compatibility-wrapper wording, or loss of the scaffold/examples-first intro.
- **Error paths**: the proof page points at the product-owned Mesher contract but drops the retained backend-only verifier or the failure-inspection map.
- **Boundary conditions**: the page stays public-secondary, `Production Backend Proof` remains out of the footer chain, and S01’s split-boundary verifier stays green after the handoff text changes.

## Steps

1. Update `website/docs/docs/production-backend-proof/index.md` so the deeper-app handoff points at `mesher/README.md` and `bash mesher/scripts/verify-maintainer-surface.sh`, while `bash scripts/verify-m051-s01.sh` is described as the mesh-lang compatibility wrapper.
2. Update `scripts/verify-production-proof-surface.sh` exact-marker and ordering checks to enforce that new handoff and forbid repo-root Mesher commands as the primary story.
3. Re-run the split-boundary contract through `bash scripts/verify-m055-s01.sh` so product-owned versus language-owned wording stays aligned with `WORKSPACE.md`.

## Must-Haves

- [ ] `production-backend-proof` stays examples-first and public-secondary.
- [ ] The deeper Mesher handoff now points at the product-owned runbook/verifier contract.
- [ ] The repo-root Mesher verifier is clearly compatibility-only in the public-secondary docs.
  - Estimate: 90m
  - Files: website/docs/docs/production-backend-proof/index.md, scripts/verify-production-proof-surface.sh
  - Verify: - `bash scripts/verify-production-proof-surface.sh`
- `bash scripts/verify-m055-s01.sh`
