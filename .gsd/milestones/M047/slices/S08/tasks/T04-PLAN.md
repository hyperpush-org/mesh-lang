---
estimated_steps: 3
estimated_files: 7
skills_used: []
---

# T04: Rebase public docs and the assembled S06 closeout rail onto recovered wrapper adoption

Why: After the Docker proof blocker is closed, the public authority surfaces still need to present `HTTP.clustered(1, ...)` adoption truthfully without displacing the canonical route-free `@cluster` story.
Do: Update README, VitePress docs, and the S06 closeout verifier to describe the Todo starter's explicit-count clustered read routes, keep route-free surfaces first, point default-count/two-node wrapper behavior at S07, and replay the recovered S05 proof as part of final closeout.
Done when: stale "not shipped" / non-goal language is removed, the docs fail closed on mixed route-free versus wrapper authority, and the S06 verifier replays the updated starter proof plus docs build successfully.

## Inputs

- `README.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `compiler/meshc/tests/e2e_m047_s06.rs`
- `scripts/verify-m047-s06.sh`

## Expected Output

- `Updated public clustered-route adoption docs with route-free `@cluster` surfaces still canonical`
- `Passing S06 closeout rail that replays recovered S05 proof and the website docs build`

## Verification

cargo test -p meshc --test e2e_m047_s06 m047_s06_ -- --nocapture && bash scripts/verify-m047-s06.sh && npm --prefix website run build
