# M055 — Research

**Date:** 2026-04-05

## Summary

The repo already contains four credible product boundaries, but it does **not** yet have four credible workflow boundaries. `registry/`, `packages-website/`, `website/`, and `mesher/landing/` already look like deployable products with their own manifests and Fly/GitHub deployment seams, and `registry/` is even isolated as its own Cargo workspace. The blocker is higher up the stack: public docs, generated scaffold output, release/install surfaces, hosted evidence, and retained verifiers still assume one repo root, one GitHub slug, one `.tmp/` tree, and one chain of repo-local commands.

The strongest recommendation is to treat M055 as a **contract-splitting milestone before it becomes a file-moving milestone**. First make the current monorepo pass through split-ready seams: explicit repo ownership, one canonical source for public repo URLs/installers, repo-local verifier entrypoints, a documented sibling-repo workspace layout, and a truthful GSD rule for repo-local versus cross-repo planning. Only after that should files move. If the split starts with extraction instead of seam hardening, exact-string docs rails, scaffold README assertions, installer parity checks, and release/deploy workflows will all fail at once.

For GSD specifically, the evidence argues against “one `.gsd/` per workspace only” and also against “one giant umbrella repo continues to own everything.” Repo-local `.gsd` is already part of executable contract in places (`compiler/meshc/tests/e2e_m046_s03.rs`, `scripts/verify-m046-s03.sh`), and most verifiers derive their own repo root before writing `.tmp/...` bundles. The best fit is a **hybrid**: each repo keeps authoritative local `.gsd` state for its own work, while cross-repo work gets a lightweight coordination layer above them (documented first, automated later if it proves necessary).

## Recommendation

Use this shape:

1. **Repo-local `.gsd` stays authoritative** for repo-owned work.
   - Current tests/verifiers already treat repo-local files, `.tmp/` artifacts, and sometimes `.gsd` artifacts as part of the contract.
   - Do not try to make `mesh-lang`, `mesh-packages`, `mesh-website`, and `hyperpush-mono` all share one hidden milestone tree as their only source of truth.

2. **Add a small workspace coordination layer above the repos**, not instead of them.
   - Start as documentation + naming conventions + explicit runbooks.
   - Only automate further if real work shows the manual coordination is too costly.
   - This keeps the first M055 proof honest: the user can work “normally” across sibling repos without needing a new orchestration system on day one.

3. **Split-ready seams should land before extraction**.
   - Centralize repo identity / GitHub URL ownership.
   - Replace monorepo-only verifier assembly with repo-owned sub-verifiers plus one cross-repo evidence aggregator.
   - Decide installer ownership explicitly.
   - Decide how `hyperpush-mono` consumes local Mesh tooling in day-to-day development.

4. **Extraction order should follow runtime/workflow risk, not directory neatness**.
   - `mesh-packages` first.
   - `mesh-website` second.
   - `hyperpush-mono` last.

`mesh-packages` is the cleanest early extraction because the packages site already talks to the registry over public HTTP and the registry already has its own Cargo workspace. `mesh-website` is mechanically separable but carries more release/install/public-surface coupling, so it should wait until repo identity and installer ownership are explicit. `hyperpush-mono` is the hardest extraction because Mesher’s current maintainer loop still depends on repo-root Mesh compiler/build/test/migrate paths.

## Implementation Landscape

### Key Files

- `Cargo.toml` — Root Rust workspace for the language/tooling repo. Important because `registry/` is explicitly excluded already, which makes it the cleanest Rust-side split candidate.
- `registry/Cargo.toml` — Separate Cargo workspace for the registry service. Strong evidence that `registry/` is already structurally closer to an independent repo than to a workspace member.
- `.github/workflows/deploy.yml` — Standalone deploy lane for `website/` to GitHub Pages. Suggests `mesh-website` can become its own hosted unit once verifier ownership is separated.
- `.github/workflows/deploy-services.yml` — One workflow currently deploys `registry/`, `packages-website/`, and `mesher/landing/` from the same checkout/ref, then runs shared health checks. This is one of the biggest monorepo couplings.
- `.github/workflows/authoritative-verification.yml` — Main verification lane. Current language repo verification depends on the starter failover proof and shared hosted lanes.
- `.github/workflows/release.yml` — Tag release lane. Current language release is blocked on `authoritative-live-proof`, `authoritative-starter-failover-proof`, and release smoke; it is not yet split-aware.
- `scripts/verify-m034-s05.sh` — Current assembled monorepo “public release proof” command. This is the highest-leverage split seam because it fans out into docs, packages, installers, workflows, and hosted evidence.
- `scripts/lib/m034_public_surface_contract.py` — Canonical public-surface checker. It currently binds together README/docs, installers, VS Code metadata, packages site, and registry API from one repo root.
- `scripts/verify-m034-s05-workflows.sh` — Encodes exact workflow/job/step expectations for `deploy.yml` and `deploy-services.yml`. This will need decomposition, not casual editing.
- `website/docs/public/install.sh`
- `website/docs/public/install.ps1`
- `tools/install/install.sh`
- `tools/install/install.ps1` — Installer scripts are duplicated and currently verified against each other. Split ownership must be explicit before `mesh-website` moves.
- `compiler/mesh-pkg/src/scaffold.rs` — Generated clustered README text bakes in monorepo examples, Mesher runbook links, and repo-root verifier commands. Split work must update generator output, not just checked-in docs.
- `scripts/tests/verify-m049-s03-materialize-examples.mjs` — Existing mechanical parity seam for generated examples. Strong reason to keep evaluator-facing generated examples in `mesh-lang` rather than inventing a new cross-repo sync story.
- `compiler/meshc/tests/e2e_m049_s03.rs` — Proves the checked-in examples match scaffold output and stay runnable. Useful anchor for “language-owned examples stay with mesh-lang.”
- `mesher/README.md` — Canonical maintainer runbook for the deeper app. It still assumes repo-root `meshc` build/test/migrate flows.
- `scripts/verify-m051-s01.sh` — Enforces the current Mesher maintainer contract. `hyperpush-mono` extraction is not done until this seam is replaced truthfully.
- `compiler/meshc/tests/e2e_m046_s03.rs`
- `scripts/verify-m046-s03.sh` — Concrete evidence that repo-local `.gsd` is not purely optional metadata today; it is read by tests/verifiers.
- `packages-website/src/routes/+layout.svelte` — Packages site footer still links to `https://github.com/snowdamiz/mesh-lang`.
- `mesher/landing/lib/external-links.ts` — Landing app already points to `https://github.com/hyperpush-org/hyperpush-mono`. This is a clean example of public repo-identity drift across surfaces.
- `registry/src/config.rs` and `packages-website/src/routes/**/*.server.js` — Positive signal: the registry/packages boundary is already URL/API based (`FRONTEND_URL`, `https://api.packages.meshlang.dev/...`), not filesystem coupled.

### Build Order

1. **Define the split-ready contracts while still in the monorepo.**
   - Write the ownership matrix: what stays in `mesh-lang`, what moves to `mesh-packages`, `mesh-website`, and `hyperpush-mono`.
   - Define the blessed local sibling layout.
   - Decide the GSD shape: repo-local `.gsd` + workspace coordination layer.
   - Centralize canonical repo identity / GitHub URL ownership.
   - Decide installer ownership.

   This should happen first because exact-string contract tests currently hardcode repo URLs, runbook links, and verifier commands all over the tree.

2. **Break the monorepo-only verification/release graph into repo-owned seams.**
   - Decompose the current `verify-m034-s05.sh` / `m034_public_surface_contract.py` shape.
   - Turn it into repo-local verifiers plus one cross-repo aggregator contract.
   - Keep the monorepo green through the new seams before moving directories.

   This is the real risk retirement step. Without it, extraction just means each repo inherits a broken half of the old proof chain.

3. **Extract `mesh-packages` first.**
   - Move `registry/` and `packages-website/` together.
   - Preserve their public HTTP/API contract and shared deploy proof.
   - Update the language repo to consume packages via published/API contract, not sibling source paths.

   This is the best first “real split” because `registry/` is already its own Cargo workspace and the packages website already consumes the registry over HTTP.

4. **Extract `mesh-website` second.**
   - Move `website/` after installer ownership and repo identity are settled.
   - Re-home built-docs verification and source-of-truth installer copies.
   - Make `mesh-lang` consume the website as a public/built surface rather than as a source-tree sibling.

   Website is structurally separable, but it is currently coupled to install scripts, starter links, packages URLs, and public proof pages. It should move only after those boundaries are explicit.

5. **Extract/rename `hyperpush-mono` last.**
   - Move the current `mesher/` tree, including `mesher/landing/`, only after the deeper-app toolchain contract is clear.
   - Replace the repo-root `cargo run -q -p meshc -- build mesher` maintainer story with one truthful local-development contract: either sibling `mesh-lang` checkout usage, or installed/published Mesh tooling, but documented and verified.

   This is the highest-risk split because Mesher is still source-coupled to the language repo’s local compiler workflow.

### Verification Approach

Treat M055 verification as a layered contract, not one mega-script on day one.

**A. Contract-prep verification inside the current repo**
- `bash scripts/verify-m034-s05.sh` — current assembled monorepo proof. Use this as the “what currently couples everything together” baseline.
- `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check` — proves language-owned examples stay mechanically aligned.
- `bash scripts/verify-m051-s01.sh` — proves the current Mesher maintainer contract before trying to replace it.
- `bash scripts/verify-m046-s03.sh` — proves the current `.gsd` contract is still intact where tests/verifiers depend on it.

**B. Repo-local verification after decomposition**
- `mesh-lang` should own the language/toolchain/example proofs and should not require `website/`, `registry/`, or `packages-website/` source trees to be present locally except through documented cross-repo checks.
- `mesh-packages` should own registry + packages-site deploy/public-surface proof.
- `mesh-website` should own docs build + public install/docs proof.
- `hyperpush-mono` should own Mesher + landing proof, with a documented dependency on either sibling `mesh-lang` or installed Mesh binaries.

**C. Workspace-level verification**
- One documented sibling layout should be enough for a human to:
  - edit language/tooling in `mesh-lang`,
  - run Mesher from `hyperpush-mono`,
  - update docs in `mesh-website`,
  - and verify package flows through `mesh-packages`,
  without silently relying on the old monorepo root.

The milestone is done when that workspace-level flow is documented and the repo-local verifiers are truthful, not when everything is hidden behind a new orchestration wrapper.

## Don't Hand-Roll

| Problem | Existing Solution | Why Use It |
|---------|------------------|------------|
| Keeping evaluator-facing examples aligned with scaffold output | `scripts/tests/verify-m049-s03-materialize-examples.mjs` + `compiler/meshc/tests/e2e_m049_s03.rs` | This already proves `examples/todo-sqlite` and `examples/todo-postgres` mechanically match generated output. Keep examples language-owned instead of inventing cross-repo example mirroring. |
| Packages site ↔ registry runtime boundary | Existing public HTTP/API contract (`packages-website` fetches `https://api.packages.meshlang.dev/...`, `registry/src/config.rs` uses `FRONTEND_URL`) | The runtime boundary is already URL-based. Preserve that contract and move the repos around it rather than re-coupling them by source tree. |
| Isolated development across multiple branches/units | Existing git worktrees and current GSD-managed external worktrees (`git worktree list` already shows `~/.gsd/projects/.../worktrees/...`) | This avoids inventing a bespoke checkout/workspace manager. The split should reuse existing git/GSD isolation patterns. |
| Public surface drift detection | `scripts/lib/m034_public_surface_contract.py` | It needs decomposition, not replacement. It already captures the classes of drift that matter (installers, docs markers, packages site, registry API, extension metadata). |

## Constraints

- **Most verifier scripts are repo-root based.** There are **71** `scripts/verify-*.sh` files, and **64** of them derive `ROOT_DIR` from their own location before `cd`-ing into the repo root.
- **Repo identity is inconsistent right now.** `git remote -v` resolves to `https://github.com/hyperpush-org/hyperpush-mono.git`, but there are still **148** hardcoded `snowdamiz/mesh-lang` references across README, docs, installers, scripts, tests, extension metadata, and packages UI.
- **Mesher is not independently buildable under its current contract.** `mesher/README.md` and `scripts/verify-m051-s01.sh` require repo-root `cargo run -q -p meshc -- test mesher/tests`, `migrate mesher`, and `build mesher` flows.
- **Repo-local `.gsd` is part of executable contract in at least one shipped rail.** `compiler/meshc/tests/e2e_m046_s03.rs` and `scripts/verify-m046-s03.sh` read `.gsd/milestones/M046/slices/S03/S03-PLAN.md` directly.
- **Current GSD split policy is not explicitly configured in-repo.** `.gsd/PREFERENCES.md` is absent here, so the future multi-repo policy must be documented deliberately rather than inferred from local habit.
- **Installer ownership is ambiguous today.** `tools/install/install.{sh,ps1}` and `website/docs/public/install.{sh,ps1}` are duplicated and verified for parity. A repo split must choose one source of truth.
- **Packages and website already carry independent app/deploy identities.** `registry/fly.toml`, `packages-website/fly.toml`, and `mesher/landing/fly.toml` are real deploy surfaces, but hosted verification still binds them together.
- **Public/docs/runbook exact-string tests are common.** Path and slug changes are not just markdown edits; they are API migrations for tests, verifiers, and scaffold output.
- **Auto-mode worktrees already live outside the repo root in practice.** `git worktree list` shows GSD-managed worktrees under `~/.gsd/projects/.../worktrees/...`, which is a useful positive for multi-repo continuity.

## Requirements Pressure

### Table stakes from the current requirement contract

These are not “new M055 ideas”; they are already part of the project’s visible contract and the split must preserve them:

- **R116** — evaluator-facing generated examples should remain the public starting surface. That argues for keeping `examples/` language-owned in `mesh-lang`.
- **R117 / R118** — docs must stay evaluator-facing and keep one clear clustered-app path. Splitting docs must not reintroduce a proof maze or blur low-level versus clustered guidance.
- **R119** — Mesher remains the maintained deeper reference app. Moving it to `hyperpush-mono` must preserve a truthful maintainer runbook and verifier path.
- **R120** — landing, docs, and packages must still tell one coherent Mesh story. Splitting them raises the importance of this requirement rather than reducing it.
- **R121** — the packages site remains part of the normal deploy/release contract. It can move repos, but it cannot become an optional side lane again.
- **R122** — the SQLite-local vs Postgres-deployable starter boundary must stay honest across examples/docs/verifiers after the split.

### Likely omissions / candidate requirements for M055

These should be considered explicitly by the roadmap planner instead of staying implicit:

- **Blessed sibling-repo workspace layout.** There should be one documented local layout for `mesh-lang`, `mesh-packages`, `mesh-website`, and `hyperpush-mono`.
- **Repo-local GSD authority plus cross-repo coordination rule.** Each repo should have an authoritative local GSD path, and cross-repo work should have a documented coordination layer instead of silent monorepo assumptions.
- **No hidden monorepo path assumptions in generated or public surfaces.** Scaffolded README text, docs links, installer URLs, and verifier instructions should point to the right repo or public surface explicitly.
- **Cross-repo hosted evidence chain.** After the split, it should remain possible to tell which repo/version proved language release, packages deploy, website deploy, and Hyperpush deploy, without one repo pretending to own all evidence locally.
- **Truthful local Mesher/Hyperpush toolchain contract.** `hyperpush-mono` should document and verify whether it consumes a sibling `mesh-lang` checkout or installed Mesh binaries for day-to-day development.
- **One canonical public repo-identity source.** GitHub URLs, issue links, installer repo slug, and scaffold/doc blob links should come from one maintained contract, not hand-copied constants.

### What looks optional or overbuilt for M055

- A fully automated umbrella queue/roadmap/orchestration system across repos is **not** required to make day-to-day work normal again.
- A new workspace manager or monorepo replacement tool is **not** the main need here.
- Extracting `hyperpush-mono` first is likely overaggressive; it has the least independent toolchain story today.
- Rewriting public messaging broadly is out of scope; only ownership, routing, and continuity surfaces need to change.

## Common Pitfalls

- **Moving directories before splitting contracts** — exact-string tests, scaffold assertions, installer parity, and hosted workflow checks will all fail at once. Normalize boundaries first.
- **Trying to solve M055 with one giant umbrella `.gsd`** — current repo-local tests/verifiers already consume repo-root files and `.tmp`/`.gsd` artifacts. Repo-local authority needs to remain real.
- **Extracting Mesher before replacing the repo-root toolchain contract** — `hyperpush-mono` will look split on paper but still require `mesh-lang` internals by undocumented convention.
- **Splitting `mesh-website` without choosing installer ownership** — the public site, release smoke, and toolchain update path can silently diverge.
- **Keeping one monolithic hosted verifier after the split** — every repo will end up depending on missing checkout paths from the old monorepo.
- **Treating GitHub slug cleanup as cosmetic** — it is a contract surface today across docs, installers, packages UI, extension metadata, and tests.

## Open Risks

- The final cross-repo hosted evidence model is not obvious yet: one umbrella aggregator is probably needed, but it should aggregate repo-owned proofs rather than recreate monorepo-local assumptions.
- `hyperpush-mono` still needs an explicit dev-toolchain contract. This is the most likely place for the roadmap to discover a plan-invalidating blocker.
- Installer ownership may require a small generated/copy step or a dedicated shared source file if `mesh-lang` keeps the release logic while `mesh-website` hosts the public files.
- Because many proof pages and tests still pin exact repo/blob URLs, repo identity cleanup can easily become noisier than expected unless centralized early.

## Skills Discovered

| Technology | Skill | Status |
|------------|-------|--------|
| GitHub Actions workflows | `github-workflows` | available |
| Fly.io deploy/debug flows | `flyio-cli-public` | available |
| Repo split / boundary decomposition | `wshobson/agents@monorepo-management` | installed |
| Git worktrees / isolated workspace flow | `obra/superpowers@using-git-worktrees` | installed |
