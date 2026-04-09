# M055:

## Vision
Split the current monorepo into two working repos—`mesh-lang` for language, tooling, docs, installers, registry, and packages/public-site surfaces, and `hyperpush-mono` for the product—without breaking the user's normal GSD workflow, evaluator-facing starter path, or the truthful proof chain between them.

## Slice Overview
| ID | Slice | Risk | Depends | Done | After this |
|----|-------|------|---------|------|------------|
| S01 | Two-Repo Boundary & GSD Authority Contract | high | — | ✅ | After this, the current tree exposes one blessed sibling-repo workspace, one canonical repo-identity source, and repo-local versus cross-repo GSD rules, with drift checks that fail on stale monorepo paths or GitHub slugs. |
| S02 | Hyperpush Toolchain Contract Outside `mesh-lang` | high | S01 | ✅ | After this, the deeper reference app can be built, tested, migrated, and explained from the blessed sibling workspace against an explicit `mesh-lang` toolchain contract, so `hyperpush-mono` is operationally believable before extraction. |
| S03 | `mesh-lang` Public Surface & Starter Contract Consolidation | medium | S01 | ✅ | After this, `mesh-lang` stands on its own for evaluator-facing generated examples, scaffolded starter docs, the public docs/install surface, and the packages deploy contract, with repo-local proof rails that do not require product-repo source paths. |
| S04 | `hyperpush-mono` Extraction & Two-Repo Evidence Assembly | medium | S01, S02, S03 | ✅ | After this, the two-repo workspace is operationally real: Hyperpush is extracted and renamed, each repo owns its own proof entrypoints, and one evidence chain can show which repo/ref proved `mesh-lang` continuity and which repo/ref proved `hyperpush-mono` continuity. |
| S05 | Validation Remediation: Contract Truth & Two-Repo Evidence Closure | high | S01, S03, S04 | ✅ | After this, `bash scripts/verify-m055-s01.sh`, `bash scripts/verify-m055-s03.sh`, and `bash scripts/verify-m055-s04.sh` all pass from a clean repo and publish the retained bundle plus per-repo attribution files promised by M055. |
