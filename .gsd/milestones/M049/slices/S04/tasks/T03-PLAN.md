---
estimated_steps: 19
estimated_files: 8
skills_used: []
---

# T03: Rewrite public clustered onboarding to scaffold plus generated `/examples`

Replace the old equal-surface proof-app story with the new scaffold/examples-first story across README, site docs, generated clustered README text, and the Mesh clustering skill. Public surfaces must point readers to `meshc init --clustered`, `examples/todo-postgres`, and `examples/todo-sqlite`, keep `reference-backend` as the deeper backend proof, and preserve the explicit SQLite-local vs Postgres-clustered split.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Public docs and README copy | Fail the new onboarding contract test before stale proof-app links ship. | N/A — local content checks only. | Treat missing example-first replacements or wrong SQLite/Postgres split text as contract drift. |
| `compiler/mesh-pkg/src/scaffold.rs` generated README text | Fail clustered scaffold wording tests before a stale generated README lands. | N/A — local unit/tooling tests only. | Reject README text that still frames `tiny-cluster` / `cluster-proof` as the public first-contact story. |
| `tools/skill/mesh/skills/clustering/SKILL.md` plus M048 guardrails | Fail the skill/docs contract rails before stale onboarding reaches the shipped skill bundle. | N/A — Node test execution only. | Reject unsplit `todo-api` guidance or any text that projects clustered claims onto the SQLite starter. |

## Load Profile

- **Shared resources**: static Markdown content, the clustering skill file, the scaffold template source, and Node-based contract tests.
- **Per-operation cost**: one docs build plus a small set of deterministic content-contract tests.
- **10x breakpoint**: review noise and content drift appear before runtime cost; the task should keep one clear public starter story instead of adding a second transitional layer.

## Negative Tests

- **Malformed inputs**: stale `tiny-cluster/README.md` or `cluster-proof/README.md` onboarding links, missing `examples/todo-sqlite` / `examples/todo-postgres` references, or generic `meshc init --template todo-api` wording.
- **Error paths**: SQLite described as clustered/operator-capable, Postgres no longer described as the serious shared starter, or `reference-backend` promoted back to a coequal starter.
- **Boundary conditions**: public docs may still mention retained verifier commands and deeper proof surfaces, but they must stop teaching the proof fixtures as first-contact onboarding.

## Steps

1. Rewrite `README.md`, `compiler/mesh-pkg/src/scaffold.rs`, and `tools/skill/mesh/skills/clustering/SKILL.md` so the public clustered story points at scaffold plus generated `/examples` while keeping `reference-backend` later/deeper.
2. Rewrite the clustered onboarding pages in `website/docs/docs/getting-started/clustered-example/index.md`, `website/docs/docs/distributed/index.md`, `website/docs/docs/distributed-proof/index.md`, and `website/docs/docs/tooling/index.md` to retire proof-app-first language.
3. Add `scripts/tests/verify-m049-s04-onboarding-contract.test.mjs` so stale proof-app onboarding links, missing example-first replacements, or broken SQLite/Postgres split wording fail closed.

## Inputs

- ``README.md` — current public onboarding copy that still names the proof-app runbooks.`
- ``compiler/mesh-pkg/src/scaffold.rs` — generated clustered README text that still aligns with `tiny-cluster` / `cluster-proof`.`
- ``website/docs/docs/getting-started/clustered-example/index.md` — scaffold-first public walkthrough that still teaches three equal canonical surfaces.`
- ``website/docs/docs/distributed/index.md` — distributed guide that still points beginners at the proof-app runbooks.`
- ``website/docs/docs/distributed-proof/index.md` — public verifier map that still names the proof-app readmes as canonical surfaces.`
- ``website/docs/docs/tooling/index.md` — tooling guide that still frames `tiny-cluster` / `cluster-proof` as public package surfaces.`
- ``tools/skill/mesh/skills/clustering/SKILL.md` — shipped clustering guidance that still encodes the old equal-surface story.`
- ``examples/todo-sqlite/README.md` — generator-owned local example README that should become a public target.`
- ``examples/todo-postgres/README.md` — generator-owned serious clustered example README that should become a public target.`
- ``scripts/tests/verify-m048-s04-skill-contract.test.mjs` — existing M048 skill guardrail to keep green.`
- ``scripts/tests/verify-m048-s05-contract.test.mjs` — existing M048 docs/update guardrail to keep green.`
- ``scripts/tests/verify-m049-s03-materialize-examples.test.mjs` — existing example-parity guardrail that public copy must not contradict.`

## Expected Output

- ``README.md` — example-first public onboarding copy with the correct SQLite/Postgres split.`
- ``compiler/mesh-pkg/src/scaffold.rs` — generated clustered README text repointed away from the proof-app runbooks.`
- ``website/docs/docs/getting-started/clustered-example/index.md` — scaffold-first page rewritten around examples rather than proof-app onboarding.`
- ``website/docs/docs/distributed/index.md` — distributed guide updated to stop teaching proof fixtures as first-contact surfaces.`
- ``website/docs/docs/distributed-proof/index.md` — verifier map rewritten around scaffold/examples-first public entrypoints and lower-level retained rails.`
- ``website/docs/docs/tooling/index.md` — tooling docs updated to align with the new onboarding story.`
- ``tools/skill/mesh/skills/clustering/SKILL.md` — clustering skill rewritten to match the public example-first contract.`
- ``scripts/tests/verify-m049-s04-onboarding-contract.test.mjs` — slice-owned fail-closed contract test for the new onboarding story.`

## Verification

- `node --test scripts/tests/verify-m049-s04-onboarding-contract.test.mjs`
- `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check`
- `node --test scripts/tests/verify-m048-s04-skill-contract.test.mjs`
- `node --test scripts/tests/verify-m048-s05-contract.test.mjs`
- `npm --prefix website run build`

## Observability Impact

- Signals added/changed: the new onboarding contract test should name the exact stale file/marker when public copy regresses.
- How a future agent inspects this: rerun the new Node test together with the M048 contract tests and `npm --prefix website run build`.
- Failure state exposed: stale proof-app onboarding links, missing example references, or wrong SQLite/Postgres split text fail as precise content-contract errors instead of as vague docs drift.
