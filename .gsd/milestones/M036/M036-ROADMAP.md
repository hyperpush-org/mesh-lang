# M036: 

## Vision
Make Mesh's editor story truthful and daily-driver credible by proving syntax parity against a real Mesh corpus, repairing the already-shipping VS Code/docs highlighting drift, and shipping a repo-owned first-class Neovim path backed by real install docs and regression checks.

## Slice Overview
| ID | Slice | Risk | Depends | Done | After this |
|----|-------|------|---------|------|------------|
| S01 | Corpus-backed syntax parity for the shared VS Code/docs surface | high — this retires the biggest unknown by proving real mesh syntax against the already-shipping shared grammar surface instead of fixing one visible regex in isolation. | — | ✅ | Open representative Mesh files in VS Code and the docs highlighter and see `#{...}` plus `${...}` handled according to compiler truth, with parity checks pinpointing any corpus sample that regresses. |
| S02 | Repo-owned first-class Neovim support pack | high — this is the new multi-editor capability and must stay bounded enough to ship a real installable path without turning into broad ecosystem work. | S01 | ✅ | Follow the repo docs to install the Mesh Neovim pack, open a `.mpl` file, and get filetype/syntax support plus `meshc lsp` through the documented first-class path. |
| S03 | Explicit support tiers and real editor proof in public docs | medium — the technical work is smaller, but this slice is where credibility is either preserved or lost because docs can still overclaim after implementation lands. | S01, S02 | ✅ | A developer can read the tooling docs, see exactly which editors are first-class versus best-effort, and follow the published VS Code and Neovim workflows with smoke proof backing the claims. |
