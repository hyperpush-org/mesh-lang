---
estimated_steps: 3
estimated_files: 6
skills_used:
  - vitepress
  - test
---

# T01: Refresh first-contact docs so the starter ladder surfaces the M053 public contract

Add light evaluator-facing wording to the repo README, Getting Started, Clustered Example, and Tooling docs so readers still move scaffold → SQLite → Postgres, while the serious Postgres starter’s staged deploy/failover truth becomes visible without turning first-contact pages into a verifier maze.

## Steps

1. Update `README.md`, `website/docs/docs/getting-started/index.md`, `website/docs/docs/getting-started/clustered-example/index.md`, and `website/docs/docs/tooling/index.md` so they keep the starter/examples-first order while naming SQLite as local-only and Postgres as the serious shared/deployable starter.
2. Add only the minimum M053 language needed for evaluators: the Postgres starter owns a staged deploy + failover proof chain, packages/public-surface checks now live in the same hosted contract, and deeper proof commands stay behind the proof pages.
3. Update the first-contact contract guardrails in `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` and `scripts/verify-m050-s02.sh` only as needed to pin the new wording/order without re-promoting retained proof apps or Fly as first-contact surfaces.

## Must-Haves

- [ ] First-contact docs still lead with generated scaffold/examples instead of retained proof fixtures.
- [ ] SQLite stays explicitly local/single-node only while Postgres is named as the serious shared/deployable starter.
- [ ] The first-contact verifier remains green and guards the new M053 wording/order.

## Inputs

- `README.md` — current repo-root evaluator entrypoint copy
- `website/docs/docs/getting-started/index.md` — primary first-contact docs ladder
- `website/docs/docs/getting-started/clustered-example/index.md` — scaffold-first clustered follow-on page
- `website/docs/docs/tooling/index.md` — evaluator-facing CLI/starter routing page
- `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` — existing first-contact wording/order guardrail
- `scripts/verify-m050-s02.sh` — assembled first-contact verifier

## Expected Output

- `README.md` — updated first-contact starter ladder wording
- `website/docs/docs/getting-started/index.md` — updated evaluator-facing starter split copy
- `website/docs/docs/getting-started/clustered-example/index.md` — updated clustered follow-on handoff text
- `website/docs/docs/tooling/index.md` — updated CLI/starter contract wording
- `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` — tightened contract assertions for the M053 wording/order
- `scripts/verify-m050-s02.sh` — updated verifier expectations if the first-contact contract markers change

## Verification

- `bash scripts/verify-m050-s02.sh`
