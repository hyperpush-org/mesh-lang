---
estimated_steps: 4
estimated_files: 3
skills_used:
  - test
---

# T03: Rewrite the Mesher maintainer runbook around package-local commands

**Slice:** S02 — Hyperpush Toolchain Contract Outside `mesh-lang`
**Milestone:** M055

## Description

After the scripts and verifier exist, make the maintainer runbook tell the same story. This task should rewrite `mesher/README.md` around the explicit toolchain contract and package-local commands, keep the runtime env and seed-data guidance truthful, and pin that contract with the slice-owned Node test instead of leaving it as prose.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `mesher/README.md` contract | Fail closed on missing toolchain-order wording, missing package-local commands, or stale repo-root `cargo run -q -p meshc -- ... mesher` examples. | N/A for source assertions. | Treat contradictory primary/secondary verifier wording as runbook drift. |
| `scripts/tests/verify-m055-s02-contract.test.mjs` | Stop on the first missing marker or forbidden legacy command. | Bounded local test only. | Treat accidental wording broadening or missing exact commands as contract drift. |

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

## Verification

- `node --test scripts/tests/verify-m055-s02-contract.test.mjs`
- `bash mesher/scripts/test.sh`

## Inputs

- `mesher/README.md` — current repo-root maintainer runbook.
- `mesher/.env.example` — current Mesher env contract that the README must keep truthful.
- `mesher/scripts/verify-maintainer-surface.sh` — product-owned verifier from T02.
- `scripts/verify-m051-s01.sh` — compatibility wrapper from T02.
- `scripts/tests/verify-m055-s02-contract.test.mjs` — slice-owned contract test from T01.

## Expected Output

- `mesher/README.md` — package-local maintainer runbook with explicit toolchain contract.
- `mesher/.env.example` — env comments/examples aligned to the package-local runbook if wording needs adjustment.
- `scripts/tests/verify-m055-s02-contract.test.mjs` — README contract assertions aligned to the new maintainer story.
