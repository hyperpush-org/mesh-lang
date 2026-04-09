# M055: Multi-Repo Split & GSD Workflow Continuity — Context Draft

**Gathered:** 2026-04-03
**Status:** Draft — needs dedicated discussion before planning

## Project Description

M055 splits the current monorepo into a set of focused repos without breaking the day-to-day GSD workflow or the ability to develop Mesh as one system. The initial target repo set is:

- `mesh-lang` — the language/toolchain repo for the compiler, runtime, CLI, LSP, formatter, package tooling, language-owned examples, and the proof rails that still belong with the language itself.
- `mesh-packages` — the package registry plus the packages website.
- `mesh-website` — the public website/docs surface currently rooted at `website/`.
- `hyperpush-mono` — the repo renamed from `mesher/`.

The local checkout structure may change somewhat, but GSD must be updated and documented so the user can continue working across the new project layout normally instead of having to remember old monorepo assumptions or invent new cross-repo habits ad hoc.

## Why This Milestone

The current repository now contains several distinct products and operational surfaces with different ownership boundaries, deployment concerns, and public roles. Keeping everything in one repo is starting to blur those boundaries. But a repo split that breaks planning, verification, local development, or GSD continuity would be worse than the current monorepo. The split therefore has to preserve the user's working rhythm: one understandable local structure, explicit repo ownership, and GSD guidance that still makes everyday work feel normal.

## User-Visible Outcome

### When this milestone is complete, the user can:

- clone or arrange the documented sibling repos locally and keep using GSD in a normal way instead of rebuilding a workflow from scratch.
- tell which repo owns which code and public surface.
- follow documented local workspace structure and cross-repo commands without guessing.
- continue planning, implementing, and verifying work across the new repo boundaries without hidden dependence on the old monorepo layout.

### Entry point / environment

- Entry point: a documented local multi-repo workspace layout covering `mesh-lang`, `mesh-packages`, `mesh-website`, and `hyperpush-mono`
- Environment: local git checkouts, local builds/tests/docs/dev servers, repo-owned verification scripts, and GSD artifacts/documentation
- Live dependencies involved: multiple local repos with cross-repo references, current CI/release/deploy assumptions, and any retained verification rails that still depend on repo-relative paths

## Completion Class

- Contract complete means: repo boundaries are explicit, the intended local multi-repo layout is documented, and GSD has a truthful documented way to operate against that layout.
- Integration complete means: code, docs, scripts, and cross-repo references have moved to their new homes without leaving the old monorepo path assumptions as hidden blockers.
- Operational complete means: a user can open the documented workspace structure and continue working normally with GSD, including understanding which repo to use for which task and how cross-repo work is coordinated.

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- the target repos exist with clear ownership boundaries: `mesh-lang`, `mesh-packages`, `mesh-website`, and `hyperpush-mono`.
- the local workspace structure is documented clearly enough that the user can keep working normally after the split.
- GSD documentation and/or project structure points to the new layout instead of silently depending on the old monorepo.
- cross-repo references, verification flows, and handoff surfaces are updated so the split is operationally real rather than a directory shuffle.

## Risks and Unknowns

- GSD may currently depend on monorepo-relative paths, shared `.gsd/` assumptions, or repo-root verifier scripts in ways that are easy to break during extraction.
- CI, release, and deploy flows may currently assume one repo even where product boundaries are already separate.
- It is not yet decided whether the correct GSD shape is one umbrella coordination layer over several repos, one `.gsd/` per repo, or a hybrid.
- Repo boundaries can easily become muddled if docs/examples/proof rails move without a clear rule for what stays language-owned versus app/site-owned.
- The rename from `mesher` to `hyperpush-mono` will affect docs, scripts, and historical references beyond a simple directory move.

## Existing Codebase / Prior Art

- `compiler/`, `scripts/`, `.github/`, and the current `.gsd/` tree as the language/tooling core
- `registry/` and `packages-website/` as the current packages surface
- `website/` as the public docs/site surface
- `mesher/` as the broader app surface that would become `hyperpush-mono`
- current docs, verifiers, and workflows that still assume a single repo root

> See `.gsd/DECISIONS.md` for all architectural and pattern decisions — it is an append-only register; read it during planning, append to it during execution.

## Scope

### In Scope

- defining the repo boundaries and extraction plan for `mesh-lang`, `mesh-packages`, `mesh-website`, and `hyperpush-mono`
- documenting the supported local multi-repo structure
- updating GSD-facing documentation and workflow guidance so the new structure is usable in normal day-to-day work
- renaming `mesher` to `hyperpush-mono` as part of the new structure
- reconciling cross-repo references where the repo split changes ownership or path assumptions

### Out of Scope / Non-Goals

- major product rewrites unrelated to the repo split itself
- weakening proof rigor just to make the split easier
- broad public-site messaging rewrites beyond what is required to reflect the new repo ownership boundaries
- pretending the right GSD shape is already known before the dedicated planning discussion happens

## Open Questions

- Should one umbrella workspace own the queue/roadmap and point to child repos, or should each repo have its own local GSD with one coordinating layer above them?
- What is the blessed local sibling-repo layout for day-to-day development?
- Which docs/examples/proof rails stay with `mesh-lang`, and which move to `mesh-website` or `hyperpush-mono`?
- How should cross-repo CI/release/deploy dependencies be represented once the monorepo disappears?
- What is the minimum documentation needed so the user can keep using GSD “normally” after the split instead of learning a second workflow?
