---
estimated_steps: 4
estimated_files: 6
skills_used:
  - vitepress
  - test
---

# T04: Split the public README and website docs between SQLite-local and Postgres-clustered starter guidance

**Slice:** S02 — SQLite local starter contract
**Milestone:** M049

## Description

Update the repo-facing docs so they stop describing the SQLite Todo starter as part of the canonical clustered contract. Keep the current proof-app references bounded until S04, but make the starter split explicit now: SQLite is the local starter, Postgres is the serious clustered/deployable starter, and `meshc init --clustered` stays the minimal clustered scaffold.

## Steps

1. Rewrite `README.md` and the M047-facing website pages so `meshc init --template todo-api` no longer implies clustered durability when `--db sqlite` is in play.
2. Point serious shared/deployable guidance at `meshc init --template todo-api --db postgres` and keep `meshc init --clustered` as the canonical minimal clustered surface.
3. Preserve the boundary that top-level proof apps are only being re-framed here, not retired yet; do not overclaim S04 in this slice.
4. Update `compiler/meshc/tests/e2e_m047_s06.rs` so stale clustered-SQLite wording becomes a named docs-contract failure.

## Must-Haves

- [ ] Public README and website docs clearly split SQLite-local, Postgres-clustered, and minimal clustered-scaffold guidance.
- [ ] The docs stop calling the SQLite Todo starter a canonical clustered/operator proof surface.
- [ ] `compiler/meshc/tests/e2e_m047_s06.rs` fails on stale SQLite-clustered wording while preserving the bounded proof-app references deferred to S04.

## Verification

- `cargo test -p meshc --test e2e_m047_s06 -- --nocapture`
- `npm --prefix website run build`

## Inputs

- `README.md` — current repo landing-page starter guidance that still references the old clustered SQLite story.
- `website/docs/docs/tooling/index.md` — CLI/scaffold docs that must stop teaching the SQLite starter as part of the clustered contract.
- `website/docs/docs/getting-started/clustered-example/index.md` — route-free clustered walkthrough that needs a bounded starter split.
- `website/docs/docs/distributed/index.md` — distributed guide that currently layers the SQLite Todo starter into the clustered proof story.
- `website/docs/docs/distributed-proof/index.md` — proof map that needs the new local-vs-clustered split without overclaiming S04.
- `compiler/meshc/tests/e2e_m047_s06.rs` — docs-contract rail that must fail closed on stale starter wording.

## Expected Output

- `README.md` — repo landing page aligned to the honest SQLite-local vs Postgres-clustered starter split.
- `website/docs/docs/tooling/index.md` — tooling docs that present the same split and bounded proof-app story.
- `website/docs/docs/getting-started/clustered-example/index.md` — clustered example doc that keeps `meshc init --clustered` primary and SQLite bounded.
- `website/docs/docs/distributed/index.md` — distributed guide aligned to the same source-first clustered story.
- `website/docs/docs/distributed-proof/index.md` — proof map that stops using the SQLite starter as a clustered proof surface.
- `compiler/meshc/tests/e2e_m047_s06.rs` — docs-contract assertions updated to the new split.
