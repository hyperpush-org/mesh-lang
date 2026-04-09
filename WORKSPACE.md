# Workspace Contract

This document is the maintainer-facing workspace contract for M055.

M055 is now a real two-repo split: `mesh-lang` is the language repo, and `hyperpush-mono` is the product repo that owns `mesher/`.
The extracted product package root stays nested under `hyperpush-mono/mesher/`.

## Blessed sibling workspace

```text
<workspace>/
  mesh-lang/
  hyperpush-mono/
    mesher/
    mesher/landing/
```

Only these two sibling repos are part of the blessed M055 workspace. `mesh-packages/` and `mesh-website/` are not sibling repos in this milestone.

Do not flatten the product package to `<workspace>/mesher`; the blessed extracted product surface is `hyperpush-mono/mesher/...`.

## GitHub truth vs local convenience

GitHub authority is split: `mesh-lang` owns language/toolchain/docs/installers/registry/packages/public-site surfaces, and `hyperpush-mono` owns `mesher/` plus `mesher/landing/`.

A local `mesh-lang/mesher -> ../hyperpush-mono/mesher` compatibility path is allowed only as local workspace assembly. It is not the canonical GitHub structure, and CI/public docs must not depend on it.

GitHub Actions secrets are repo-scoped too.
`mesh-lang` workflows cannot read repository secrets that exist only on `hyperpush-mono` / `hyperpush`, so any deploy/release/publish secret used by a `mesh-lang` workflow must also exist on `mesh-lang` itself or as an organization secret explicitly shared with `mesh-lang`.

## Repo ownership

`website/`, `packages-website/`, `registry/`, installers, and evaluator-facing examples remain language-owned inside `mesh-lang` for M055.

| Surface | M055 repo owner | Notes |
| --- | --- | --- |
| `compiler/`, `scripts/`, `tools/`, `.github/`, `.gsd/`, and release tooling | `mesh-lang` | Language, toolchain, release, and maintainer-proof surfaces stay here. |
| `website/` | `mesh-lang` | Public docs stay language-owned in this milestone. |
| `packages-website/` | `mesh-lang` | Packages/public-site surface stays language-owned in this milestone. |
| `registry/` | `mesh-lang` | Registry ownership stays with the language repo in this milestone. |
| `tools/install/` plus mirrored public installer files | `mesh-lang` | Installer ownership stays with the language repo in this milestone. |
| evaluator-facing generated starters and examples | `mesh-lang` | Public starter/docs continuity stays language-owned in this milestone. |
| `hyperpush-mono/mesher/` and `hyperpush-mono/mesher/landing/` | `hyperpush-mono` | `mesher/` is product-owned in `hyperpush-mono`; `mesh-lang` must not keep a tracked authoritative copy. The extracted package still lives under `hyperpush-mono/mesher/`. |

## Repo-local GSD authority

Repo-local `.gsd/` stays authoritative for repo-owned work.

Do not replace repo-local `.gsd/` with one umbrella milestone tree that pretends to own both repos. Repo-local plans, summaries, verifier entrypoints, and `.tmp/` bundles remain owned by the repo that produced them.

## Coordination layer boundary

Cross-repo work goes through a lightweight sibling-workspace coordination layer.

The coordination layer points at repo-local proofs; it does not replace repo-local plans, `.tmp/` bundles, or verifier entrypoints. Use it to say which repo owns the current slice, which sibling repo is affected, and which repo-local verifier produced the truth.

## Mesh-lang compatibility boundaries

`bash scripts/verify-m051-s01.sh` from `mesh-lang/` must resolve the sibling product repo from `M055_HYPERPUSH_ROOT` or the blessed `../hyperpush-mono` root. It must fail closed if only the in-repo `mesher/` tree exists.

`bash scripts/verify-m053-s03.sh` must derive the default language repo slug from `scripts/lib/repo-identity.json`, not from the current `origin` remote. That keeps hosted evidence tied to the language repo even when local remotes point at the sibling product repo.

## Authoritative split-boundary verifier

Run `bash scripts/verify-m055-s01.sh` before changing split-boundary ownership text, repo identity, or the repo-local `.gsd` handoff.

If it fails, start with `.tmp/m055-s01/verify/phase-report.txt` and then read the failing per-phase log in `.tmp/m055-s01/verify/`.

## Working rule

- Use `mesh-lang/` when the change is language-owned: compiler/runtime/tooling, docs/installers, registry, packages/public-site surfaces, or evaluator-facing starter/examples work.
- Use `hyperpush-mono/` when the change is product-owned: Mesher, landing, or the product runbook/proof surfaces that move under `hyperpush-mono/mesher/`.
- When one change spans both repos, keep the active slice in the owning repo and link to the sibling repo's proof or summary instead of inventing one shared umbrella plan tree.

## Git status and pushing in the split workspace

The local `mesh-lang/mesher` compatibility path can make the workspace feel monolithic, but git authority is still per repo.
If you edit `mesh-lang/mesher/...`, you are editing `../hyperpush-mono/mesher/...` and must commit/push from `hyperpush-mono`, not from `mesh-lang`.

From `mesh-lang/`, use the helper below to see both repos at once, install the split-workspace hooks, or push either side explicitly:

```bash
bash scripts/workspace-git.sh status
bash scripts/workspace-git.sh install-hooks
bash scripts/workspace-git.sh push mesh-lang
bash scripts/workspace-git.sh push hyperpush-mono
bash scripts/workspace-git.sh push both
```

The helper validates the expected `origin` remotes from `scripts/lib/repo-identity.json`, resolves the blessed sibling `../hyperpush-mono` root when it exists, and fails closed if a pushed target repo still has uncommitted changes.
It pushes the currently checked-out branch in each target repo, so keep each repo on the branch you actually intend to publish.
`install-hooks` configures `core.hooksPath=.githooks` in both repos inside the blessed sibling workspace, or just `mesh-lang` when you are in a standalone `mesh-lang` clone.
If you intentionally need a one-sided push, override the guard for that command only with `M055_ALLOW_PARTIAL_PUSH=1 git push ...`.

For a standalone `mesh-lang` clone, the simpler repo-local install path is:

```bash
bash scripts/install-git-hooks.sh
```

That tracked `pre-push` hook is safe in a standalone clone: it enforces the local `mesh-lang/mesher` compatibility-path rule, and it skips the cross-repo dirty-check when no sibling product repo is present.

Manual equivalents from the blessed sibling workspace root are still just ordinary per-repo git commands:

```bash
cd mesh-lang && git push origin main
cd ../hyperpush-mono && git push origin main
```
