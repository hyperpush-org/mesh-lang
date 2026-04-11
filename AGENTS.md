# Agent workspace rules

This checkout is **not** a monorepo.

## Repo layout

Blessed sibling workspace:

```text
<workspace>/
  mesh-lang/
  hyperpush-mono/
    mesher/
    mesher/client/
    mesher/landing/
```

GitHub authority is split:

- `mesh-lang` owns language/toolchain/docs/installers/registry/packages/public-site surfaces.
- `hyperpush-mono` / `hyperpush` owns `mesher/`, the dashboard package at `mesher/client/`, and the Next.js landing app at `mesher/landing/`.
- GitHub Actions secrets are repo-scoped: `mesh-lang` workflows cannot read secrets that exist only on `hyperpush`, so deploy/release/publish secrets must be present on `mesh-lang` itself or shared to it as organization secrets.

The local `mesh-lang/mesher` path is only a compatibility symlink into the sibling product repo.
If you edit `mesh-lang/mesher/...`, those changes belong to `../hyperpush-mono`, not to `mesh-lang`.

## Before commit or push

From `mesh-lang/`, run:

```bash
bash scripts/workspace-git.sh status
```

This shows both repos, their current branches, and whether the tracked split `pre-push` guards are active.

Install the tracked hooks once per clone/worktree:

```bash
bash scripts/workspace-git.sh install-hooks
```

If this is only a standalone `mesh-lang` clone with no sibling product repo, use the repo-local installer instead:

```bash
bash scripts/install-git-hooks.sh
```

## Push commands

Push the owning repo explicitly:

```bash
bash scripts/workspace-git.sh push mesh-lang
bash scripts/workspace-git.sh push hyperpush-mono
bash scripts/workspace-git.sh push both
```

The helper refuses to push a dirty target repo.
The tracked `pre-push` hooks also refuse accidental partial pushes when the sibling repo is dirty.

If a one-sided push is truly intentional, bypass the hook for that command only:

```bash
M055_ALLOW_PARTIAL_PUSH=1 git push ...
```

## Never do this

- Do not assume one repo's branch graph applies to the other repo.
- Do not commit or push product changes from `mesh-lang` just because they appeared under `mesh-lang/mesher` locally.
- Do not copy product files back into `mesh-lang` to "make the push work".
ack into `mesh-lang` to "make the push work".
iles back into `mesh-lang` to "make the push work".
