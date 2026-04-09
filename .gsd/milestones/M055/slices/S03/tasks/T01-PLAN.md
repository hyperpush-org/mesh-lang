---
estimated_steps: 4
estimated_files: 8
skills_used:
  - test
  - vitepress
---

# T01: Replace local-product handoffs in the scaffold and first-contact docs with a repo-boundary product handoff

Make the highest-leverage public surfaces truthful first. This task should introduce or extend the canonical product-handoff marker in `scripts/lib/repo-identity.json`, then use it to rewrite the generated clustered README and first-contact docs so `mesh-lang` stops teaching local `mesher/...` source paths or `bash scripts/verify-m051-*` commands as part of the evaluator-facing starter ladder.

## Steps

1. Extend `scripts/lib/repo-identity.json` with the product-handoff fields the public generator/docs/tests should consume, keeping S01’s language-vs-product repo split as the canonical source of truth.
2. Rewrite `compiler/mesh-pkg/src/scaffold.rs`, `README.md`, `website/docs/docs/getting-started/index.md`, `website/docs/docs/getting-started/clustered-example/index.md`, and `website/docs/docs/tooling/index.md` so the evaluator path stays scaffold/examples-first and stops teaching local product-source paths as the public follow-on step.
3. Update `scripts/tests/verify-m049-s04-onboarding-contract.test.mjs` and `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` to pin the new repo-boundary handoff and fail closed on stale local-product markers.
4. Re-run example parity rails so checked-in examples remain the authoritative public starting surface.

## Must-Haves

- [ ] Generated clustered README text and first-contact docs use one repo-boundary product handoff derived from the canonical repo identity contract.
- [ ] Public starter guidance preserves the SQLite-local vs Postgres-deployable split instead of collapsing back to one generic todo starter.
- [ ] Onboarding/first-contact mutation rails fail on stale `mesher/...` or `scripts/verify-m051-*` public handoff markers.

## Inputs

- `scripts/lib/repo-identity.json`
- `compiler/mesh-pkg/src/scaffold.rs`
- `README.md`
- `website/docs/docs/getting-started/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/tooling/index.md`
- `scripts/tests/verify-m049-s04-onboarding-contract.test.mjs`
- `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs`
- `scripts/tests/verify-m049-s03-materialize-examples.mjs`
- `compiler/meshc/tests/e2e_m049_s03.rs`

## Expected Output

- `scripts/lib/repo-identity.json`
- `compiler/mesh-pkg/src/scaffold.rs`
- `README.md`
- `website/docs/docs/getting-started/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/tooling/index.md`
- `scripts/tests/verify-m049-s04-onboarding-contract.test.mjs`
- `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs`

## Verification

node scripts/tests/verify-m049-s03-materialize-examples.mjs --check
cargo test -p meshc --test e2e_m049_s03 -- --nocapture
node --test scripts/tests/verify-m049-s04-onboarding-contract.test.mjs
node --test scripts/tests/verify-m050-s02-first-contact-contract.test.mjs
