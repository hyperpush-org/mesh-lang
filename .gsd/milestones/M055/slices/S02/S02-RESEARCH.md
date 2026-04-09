# M055 — Research

**Date:** 2026-04-05

## Summary

S02 primarily owns **R119** (Mesher remains the maintained deeper reference app) and directly supports **R117/R118** (one clear clustered/backend handoff), **R120** (one coherent Mesh story across public and product surfaces), and **R122** (honest SQLite-vs-Postgres boundary after the split). The real blocker is **not** that Mesher only works inside the monorepo. The generic package-local Mesh CLI contract already works against `mesher/` today:

- `./target/debug/meshc test mesher/tests` passed.
- `cd mesher && ../target/debug/meshc test tests` passed.
- `cd mesher && ../target/debug/meshc build . --output ../.tmp/m055-s02/local-build/mesher` passed.

That means the extraction risk is higher-level: **the maintained runbook, authoritative verifier, Rust e2e support, and public proof handoff still assume repo-root `mesh-lang` commands and paths**.

`mesher/README.md` is still explicitly a **repo-root maintainer loop** (`cargo run -q -p meshc -- test mesher/tests`, `... migrate mesher ...`, `... build mesher`), and `scripts/verify-m051-s01.sh` fail-closes on those exact commands. `compiler/meshc/tests/support/m051_mesher.rs` compounds the coupling by deriving `meshc` through `CARGO_BIN_EXE_meshc`, anchoring `repo_root()`, and writing `source_package_dir: repo_root().join("mesher")`. So even though the package itself can already run on the standard Mesh CLI shape, the authoritative proof surface would not survive extraction.

The strongest recommendation is to make S02 a **product-owned toolchain-contract slice**:

1. create a package-owned Hyperpush/Mesher toolchain discovery seam,
2. move Mesher maintenance commands onto package-local `meshc test tests` / `meshc migrate . ...` / `meshc build . --output ...` style flows,
3. make the current repo-root verifier a compatibility wrapper over that product-owned seam,
4. keep the public proof page language-owned but point it at the new explicit product contract.

That fits the new skills loaded during this unit:

- the **using-git-worktrees** skill says to reuse existing worktree patterns and avoid inventing a new workspace manager;
- the **monorepo-management** skill warns against hidden shared-dependency assumptions and over-sharing;
- the **github-workflows** skill reinforces that CI changes should remain observable and repo-owned rather than becoming another implicit monorepo coupling.

## Recommendation

Use this slice to make `mesher/` extraction-ready **without** moving it yet.

### Recommended contract

- **Blessed local development mode:** `hyperpush-mono/` uses an explicit Mesh toolchain contract.
  - Prefer a sibling `mesh-lang/` checkout for day-to-day language/product co-development.
  - Fall back to an installed `meshc` on `PATH` when a sibling checkout is not present.
  - Fail closed with one explicit error message if neither exists.
- **Package-local Mesher commands:** the maintained app should read like every other Mesh package:
  - `meshc test tests`
  - `meshc migrate . status`
  - `meshc migrate . up`
  - `meshc build . --output <tmp-binary>`
- **Package-owned scripts:** Mesher needs its own `scripts/` layer, similar to the retained backend fixture, so its runtime proof can move with the product repo instead of remaining encoded in repo-root Rust helpers and shell verifiers.
- **Repo-root compatibility wrapper:** keep `scripts/verify-m051-s01.sh` in `mesh-lang` for now, but make it a wrapper that delegates to the product-owned verifier and retains the usual `.tmp/.../verify` markers. Do **not** leave the authoritative logic only in a language-repo script.
- **Language-owned public docs stay handoff-only:** `website/docs/docs/production-backend-proof/index.md` should remain the public-secondary route, but it should hand maintainers to the new product-owned runbook/verifier contract rather than to repo-root `mesh-lang` commands.

### What not to do in S02

- Do **not** introduce a new workspace manager or umbrella orchestration layer. `WORKSPACE.md` plus existing git worktrees are enough.
- Do **not** split `.github/workflows/deploy-services.yml` yet. Landing/registry/packages hosted evidence decomposition belongs later, after the product repo owns its own local verifier.
- Do **not** widen public docs or make Mesher a first-contact surface again. The public route must stay scaffold/examples-first.

## Implementation Landscape

### Key Files

- `WORKSPACE.md` — S01’s durable split contract. It already says the blessed workspace is `mesh-lang/` plus `hyperpush-mono/`, repo-local `.gsd` stays authoritative, and cross-repo work uses a lightweight coordination layer.
- `scripts/lib/repo-identity.json` — canonical language-vs-product repo identity. Important because local `origin` currently points at `https://github.com/hyperpush-org/hyperpush-mono.git`, so repo identity cannot be inferred from Git remotes.
- `mesher/mesh.toml` — proves Mesher is already a normal Mesh package root and can use the generic package-local CLI contract.
- `mesher/README.md` — current canonical maintainer runbook. Still explicitly repo-root and language-repo-owned.
- `mesher/.env.example` — current Mesher runtime env contract; useful input for a future package-owned smoke/start script.
- `mesher/config.mpl` — app-local env key/default/error surface.
- `mesher/main.mpl` — app-local startup contract. It already does the right app-level thing: validate config, open Postgres pool, then `Node.start_from_env()`. The problem is not here.
- `mesher/tests/config.test.mpl`
- `mesher/tests/fingerprint.test.mpl`
- `mesher/tests/validation.test.mpl` — current package tests. They prove pure package behavior, but they are not a full runtime/deploy proof.
- `compiler/meshc/tests/support/m051_mesher.rs` — current authoritative Mesher runtime helper. This is the deepest hidden coupling seam: `meshc` comes from `CARGO_BIN_EXE_meshc`, paths come from `repo_root()`, migrations/build still hardcode `mesher`, and artifacts record `source_package_dir: repo_root().join("mesher")`.
- `compiler/meshc/tests/e2e_m051_s01.rs` — exact-string contract test for `mesher/README.md` and `scripts/verify-m051-s01.sh`. Any runbook/verifier change must update this file in the same task.
- `scripts/verify-m051-s01.sh` — current authoritative Mesher maintainer rail. It still owns the logic instead of delegating to a Mesher-owned surface.
- `scripts/fixtures/backend/reference-backend/README.md` — positive model for a maintainer-only, package-local, source-clean proof surface.
- `scripts/fixtures/backend/reference-backend/scripts/stage-deploy.sh`
- `scripts/fixtures/backend/reference-backend/scripts/smoke.sh` — best existing pattern for package-owned scripts that call `meshc` but keep binaries out of the source tree and fail closed on missing env/artifacts.
- `compiler/meshc/tests/support/m046_route_free.rs` — generic package helper layer. Good base pattern for package-relative `meshc test <path>` / `meshc build <path> --output ...` helpers.
- `compiler/meshc/tests/tooling_e2e.rs` — proves the generic public Mesh package contract already uses `meshc test .`, `meshc migrate . up`, and `meshc build .`.
- `website/docs/docs/tooling/index.md` — public docs already teach the generic package-local CLI contract and starter flows.
- `website/docs/docs/production-backend-proof/index.md` — public-secondary backend handoff. Still points to [`mesher/README.md`](https://github.com/snowdamiz/mesh-lang/blob/main/mesher/README.md), `bash scripts/verify-m051-s01.sh`, and `bash scripts/verify-m051-s02.sh`.
- `scripts/verify-production-proof-surface.sh` — exact-string proof for the production-backend-proof page and its generic-guide handoffs.
- `mesher/landing/package.json` — landing is already a normal standalone Node app (`npm run build`).
- `mesher/landing/fly.toml` — landing is already a deployable product-owned sub-surface.
- `.github/workflows/deploy-services.yml` — still deploys `mesher/landing` from the language repo checkout; important deferred coupling.

### What exists vs what is missing

**Already true**

- Mesher is a package root (`mesher/mesh.toml`).
- Package-local Mesh commands already work when pointed at a real `meshc` binary.
- Landing is already separately buildable/deployable.
- S01 already documented the sibling workspace and repo-local `.gsd` rule.

**Still missing**

- No `mesher/scripts/` directory exists at all.
- No package-owned Mesher verifier exists.
- No explicit toolchain discovery helper exists for a sibling `mesh-lang` checkout vs installed `meshc`.
- The public-secondary docs still hand off to language-repo paths/commands.
- The authoritative runtime proof still lives in `compiler/meshc/tests/support/m051_mesher.rs` + `scripts/verify-m051-s01.sh`.

### Natural seams

#### 1. Toolchain discovery seam

Create one product-owned helper that answers:

- where `meshc` comes from,
- whether sibling `mesh-lang/` is available,
- whether PATH-installed `meshc` is acceptable,
- and how failure is reported.

This should be the **first task** because every later runbook, script, verifier, and Rust helper depends on it.

#### 2. Package-owned Mesher scripts seam

Add a `mesher/scripts/` layer modeled on the retained backend fixture. Minimum likely surfaces:

- test/package proof
- migration status/up
- build-to-temp-output
- runtime smoke / start wrapper
- possibly a separate landing build wrapper if one product-owned verifier wants to cover both Mesher and landing

These scripts should operate from the Mesher package root and preserve a source-only tree.

#### 3. Maintainer runbook seam

Rework `mesher/README.md` around the package-owned scripts and the explicit toolchain contract. The content should read as if `mesher/` already lives in `hyperpush-mono/`, not as if it only makes sense from the monorepo root.

#### 4. Compatibility wrapper seam

Refactor `scripts/verify-m051-s01.sh` into a language-repo compatibility wrapper over the product-owned verifier, keeping the same `.tmp/m051-s01/verify/` observability markers. This lets the later extraction move the product-owned verifier without making the current repo forget how to replay historical proof.

#### 5. Public-secondary docs seam

Update the language-owned proof page and `scripts/verify-production-proof-surface.sh` only after the product-owned contract exists. That preserves **R117/R118** while letting the language repo hand maintainers to a truthful product-owned surface.

### Build Order

1. **Define the product toolchain discovery contract**
   - sibling `mesh-lang` debug build vs installed `meshc`
   - fail-closed behavior
   - one reusable helper/script

2. **Add Mesher package-owned scripts**
   - reuse the retained backend pattern: source-only tree, staged binaries outside the package, fail-closed env checks

3. **Rewrite the Mesher maintainer runbook**
   - package-local commands only
   - explicit sibling workspace contract from `WORKSPACE.md`
   - clear statement of when the language repo is required and when installed tools are enough

4. **Refactor the authoritative verifier and Rust support**
   - `scripts/verify-m051-s01.sh`
   - `compiler/meshc/tests/support/m051_mesher.rs`
   - `compiler/meshc/tests/e2e_m051_s01.rs`

5. **Update the language-owned proof handoff**
   - `website/docs/docs/production-backend-proof/index.md`
   - `scripts/verify-production-proof-surface.sh`

6. **Only then consider hosted/workflow follow-through**
   - landing deploy and cross-repo hosted evidence remain later-slice work

### Verification Approach

#### Current baseline evidence gathered in this research

These already work today and are strong proof that S02 is a contract problem, not a compiler blocker:

```bash
./target/debug/meshc test mesher/tests
cd mesher && ../target/debug/meshc test tests
mkdir -p .tmp/m055-s02/local-build && cd mesher && ../target/debug/meshc build . --output ../.tmp/m055-s02/local-build/mesher
```

All three passed in this unit.

#### Target verification for S02

The slice should end with one product-owned verifier that proves:

- toolchain discovery succeeds in the blessed sibling workspace
- package tests run package-locally
- migrations run package-locally
- build emits a temp output outside the tracked source tree
- runtime startup/smoke still proves the maintained deeper app contract
- landing, if included, is verified as a separate product-owned phase rather than as an incidental language-repo build

#### Rails that must stay green after the refactor

- `bash scripts/verify-m051-s01.sh` — repo-root compatibility rail
- `bash scripts/verify-production-proof-surface.sh` — language-owned public-secondary docs truth
- `bash scripts/verify-m055-s01.sh` — split-boundary contract from S01

If the slice rewrites the Mesher contract through the existing exact-string tests, also expect updates to:

- `compiler/meshc/tests/e2e_m051_s01.rs`
- any docs/page strings guarded by `scripts/verify-production-proof-surface.sh`

## Don't Hand-Roll

- **Package-local script model:** reuse `scripts/fixtures/backend/reference-backend/scripts/stage-deploy.sh` and `scripts/fixtures/backend/reference-backend/scripts/smoke.sh` instead of inventing a new ad hoc staging/smoke pattern.
- **Generic Mesh package CLI contract:** reuse the already-proven `meshc test .`, `meshc migrate . up`, and `meshc build .` pattern from `website/docs/docs/tooling/index.md` and `compiler/meshc/tests/tooling_e2e.rs`.
- **Workspace coordination:** reuse `WORKSPACE.md` and existing git worktree behavior instead of introducing a workspace manager. This matches both S01 and the loaded **using-git-worktrees** skill.
- **Repo identity:** reuse `scripts/lib/repo-identity.json` for repo ownership/source URLs instead of copying more slug constants into new docs or scripts.

## Constraints

- `mesher/README.md` still requires repo-root commands: `cargo run -q -p meshc -- test mesher/tests`, `... migrate mesher ...`, `... build mesher`.
- `scripts/verify-m051-s01.sh` fail-closes on those same exact commands and repo-root file locations.
- `compiler/meshc/tests/support/m051_mesher.rs` still derives `meshc` from `CARGO_BIN_EXE_meshc`, anchors all paths to the language repo root, and records `source_package_dir: repo_root().join("mesher")`.
- `website/docs/docs/production-backend-proof/index.md` and `scripts/verify-production-proof-surface.sh` still hand maintainers to `mesh-lang` blob links and repo-root verifier commands.
- `find mesher -maxdepth 3 -type d -name scripts` returned nothing: there is currently no package-owned script layer under `mesher/`.
- `mesher/landing` is already separately buildable and deployable, but `.github/workflows/deploy-services.yml` still deploys it from the language repo checkout (`working-directory: mesher/landing`).
- Local Git remote state is misleading for repo identity: `origin` points at `https://github.com/hyperpush-org/hyperpush-mono.git`, while S01’s canonical language repo identity remains `snowdamiz/mesh-lang` in `scripts/lib/repo-identity.json`.
- Repo-local `.gsd` remains authoritative, and `git worktree list` already shows active external worktrees. Per the loaded **using-git-worktrees** skill, this argues for reuse of existing isolation patterns, not a new workspace/orchestration system.

## Common Pitfalls

- **Changing docs first:** `compiler/meshc/tests/e2e_m051_s01.rs` and `scripts/verify-production-proof-surface.sh` both pin exact strings. Docs/runbook/verifier must move together.
- **Treating package tests as the full proof:** `mesher/tests/` only covers pure package logic. Runtime/migration/startup proof still needs a product-owned verifier.
- **Leaving toolchain discovery implicit:** “just use whatever `meshc` you have” is not a truthful cross-repo contract.
- **Re-encoding the authoritative proof only in `mesh-lang`:** that would make `hyperpush-mono` look extracted while still depending on hidden language-repo rails.
- **Trying to solve hosted evidence now:** workflow decomposition is real, but S02 should not spend itself on CI graph surgery before the local product-owned verifier exists.
- **Inventing a workspace manager:** the loaded **monorepo-management** and **using-git-worktrees** skills both point the other direction here — make dependencies explicit, avoid hidden sharing, reuse existing isolation.

## Open Risks

- The toolchain default still needs an explicit choice: sibling `mesh-lang` first, PATH-installed `meshc` first, or strict sibling-only. The evidence supports sibling-first with PATH fallback, but the repo must state that plainly.
- `meshc migrate .` for Mesher still needs a truthful DB/bootstrap story in product-owned scripts; unlike the retained backend fixture, Mesher currently has no package-local migration/smoke wrapper.
- The public proof page is language-owned and exact-string-guarded. If the product handoff wording drifts without a corresponding verifier update, S02 can fail on docs truth instead of on toolchain truth.
- Landing is already a product surface, but hosted deploy proof still lives in the language repo. S02 can prepare the split but not finish the hosted evidence chain.

## Skills Discovered

| Technology | Skill | Status |
|------------|-------|--------|
| Repo split / multi-repo boundary management | `monorepo-management` | installed (`wshobson/agents@monorepo-management`) |
| Git worktrees / sibling-workspace isolation | `using-git-worktrees` | installed (`obra/superpowers@using-git-worktrees`) |
| GitHub Actions workflow proof / debugging | `github-workflows` | available |
| Fly.io deploy surfaces | `flyio-cli-public` | available |
