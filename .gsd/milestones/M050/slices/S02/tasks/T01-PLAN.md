---
estimated_steps: 12
estimated_files: 5
skills_used:
  - vitepress
  - test
---

# T01: Rewrite Getting Started and README around the explicit starter chooser

**Slice:** S02 — First-Contact Docs Rewrite
**Milestone:** M050

## Description

The highest-risk first-contact drift is still in `Getting Started`: it jumps readers toward proof surfaces too early, never gives the explicit clustered/SQLite/Postgres choice after hello-world, and still points source builds at the stale `hyperpush-org/hyperpush-mono` repo. This task fixes the repo-root and Getting Started entrypoints together and seeds the slice-owned first-contact contract on that seam.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `website/docs/docs/getting-started/index.md` + `README.md` | Fail the contract test on missing chooser markers or stale repo URLs instead of accepting partial copy drift. | N/A — source read only. | Reject reordered or missing clustered/proof next-step markers as onboarding drift. |
| `reference-backend/scripts/verify-production-proof-surface.sh` | Keep the script green or fix the page copy in the same task instead of adding a one-off verifier exception. | Stop the task until the failing verifier log is inspectable. | Treat missing `Clustered Example` / `Production Backend Proof` markers as contract drift, not as optional prose changes. |
| `compiler/mesh-pkg/src/scaffold.rs` + example READMEs | Copy from these anchors instead of inventing new starter language. | N/A — source read only. | Reject starter descriptions that contradict the generated scaffold/example truth. |

## Load Profile

- **Shared resources**: the public starter wording shared by `README.md`, `website/docs/docs/getting-started/index.md`, and the generated scaffold/example READMEs.
- **Per-operation cost**: one Markdown rewrite plus one Node source-contract pass and one proof-surface script replay.
- **10x breakpoint**: wording drift across entrypoints breaks before build cost does; stale commands and URLs are the real failure mode.

## Negative Tests

- **Malformed inputs**: unsplit `meshc init --template todo-api`, stale `hyperpush-org/hyperpush-mono` clone URL, or missing starter chooser commands.
- **Error paths**: proof-first lead-in copy survives above the chooser, or Getting Started loses the `Clustered Example` before `Production Backend Proof` ordering.
- **Boundary conditions**: hello-world remains the first executable path and proof pages stay discoverable but secondary after the rewrite.

## Steps

1. Use `compiler/mesh-pkg/src/scaffold.rs`, `examples/todo-sqlite/README.md`, `examples/todo-postgres/README.md`, and the current repo `README.md` as copy anchors; rewrite `website/docs/docs/getting-started/index.md` so install -> hello-world stays primary and the post-hello-world branch explicitly offers `meshc init --clustered`, `meshc init --template todo-api --db sqlite`, and `meshc init --template todo-api --db postgres`.
2. Tighten `README.md` starter-path wording only where it needs to match the same chooser language, and replace the stale `hyperpush-org/hyperpush-mono` source-build clone URL with the current `snowdamiz/mesh-lang` repo path.
3. Seed `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` with README + Getting Started assertions for the three starter commands, the honest local/shared-deployable split, the current repo URL, and the required `Clustered Example` before `Production Backend Proof` ordering.
4. Keep the existing clustered/proof next-step markers truthful enough that `reference-backend/scripts/verify-production-proof-surface.sh` stays green without a wording-only exception path.

## Must-Haves

- [ ] `README.md` and `website/docs/docs/getting-started/index.md` now teach the same explicit clustered/SQLite/Postgres starter choice.
- [ ] The source-build fallback uses `https://github.com/snowdamiz/mesh-lang.git` instead of the stale repo.
- [ ] The new first-contact contract fails closed on missing starter commands, stale repo URLs, or Getting Started/proof ordering drift.
- [ ] The production-proof surface verifier still recognizes the clustered-first, proof-secondary path after the rewrite.

## Verification

- `node --test scripts/tests/verify-m050-s02-first-contact-contract.test.mjs`
- `bash reference-backend/scripts/verify-production-proof-surface.sh`

## Observability Impact

- Signals added/changed: `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` names missing starter commands, stale repo URLs, and ordering drift between README and Getting Started.
- How a future agent inspects this: run `node --test scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` plus `bash reference-backend/scripts/verify-production-proof-surface.sh`.
- Failure state exposed: missing chooser command, stale clone URL, or `Clustered Example` / `Production Backend Proof` order drift.

## Inputs

- `README.md` — current repo-root starter split and verifier discoverability wording to keep aligned.
- `website/docs/docs/getting-started/index.md` — current first-contact page to rewrite.
- `compiler/mesh-pkg/src/scaffold.rs` — clustered scaffold README wording and current repo links to copy from.
- `examples/todo-sqlite/README.md` — honest local starter wording to reuse.
- `examples/todo-postgres/README.md` — shared/deployable starter wording to reuse.
- `reference-backend/scripts/verify-production-proof-surface.sh` — existing clustered/proof ordering contract that must stay green.

## Expected Output

- `README.md` — repo-root starter chooser wording aligned to the rewritten first-contact path.
- `website/docs/docs/getting-started/index.md` — hello-world page rewritten around the explicit clustered/SQLite/Postgres choice.
- `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` — first-contact source contract seeded with README + Getting Started assertions.
