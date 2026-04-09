---
estimated_steps: 3
estimated_files: 6
skills_used: []
---

# T03: Recover truthful Docker clustered-route proof for the Todo scaffold and close the S05 blocker

Why: S08 is blocked until the generated Todo scaffold can prove clustered read-route truth inside Docker instead of stopping at the published-cluster-port handshake EOF.
Do: Use the retained T02 container artifacts to root-cause the `meshc cluster status` EOF, then fix the generated scaffold/runtime or the Docker proof harness so the container proof uses an authoritative operator seam without weakening fail-closed assertions. Rebaseline the S05 helper/test/script rails so wrapped `GET /todos` succeeds natively and in Docker with continuity metadata for the shipped wrapper.
Done when: `m047_s05_todo_scaffold_clustered_list_route_truth_is_real_natively_and_in_container` passes, the Docker proof captures successful cluster status/continuity evidence for `Api.Todos.handle_list_todos` with `replication_count=1`, `replication_health=local_only`, and `fell_back_locally=true`, and the retained bundle only appears on real regressions.

## Inputs

- `.tmp/m047-s05/todo-scaffold-clustered-route-truth-1775096433580666000/container/`
- `compiler/meshc/tests/support/m047_todo_scaffold.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `scripts/verify-m047-s05.sh`

## Expected Output

- `Passing native + Docker clustered-route truth proof for the generated Todo scaffold`
- `Updated S05 verification rails that retain Docker/operator diagnostics only on genuine failures`

## Verification

cargo test -p meshc --test e2e_m047_s05 m047_s05_todo_scaffold_clustered_list_route_truth_is_real_natively_and_in_container -- --nocapture && cargo test -p meshc --test e2e_m047_s05 -- --nocapture && cargo test -p meshc --test e2e_m047_s07 -- --nocapture && cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture && bash scripts/verify-m047-s05.sh

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| README/VitePress docs authority strings | fail exact contract assertions and do not leave mixed route-free/wrapper wording in public surfaces | N/A | contradictory wording is a red contract failure, not optional polish |
| assembled `scripts/verify-m047-s06.sh` replay and retained-bundle handoff | stop at the first red prerequisite and keep copied logs/pointers under `.tmp/m047-s06/verify` | bound every replayed command and fail with the captured phase log | malformed status files or bundle pointers fail the closeout rail instead of being repaired implicitly |
| website/docs build and S06 contract test | fail the named docs/contract rails until every public surface agrees on the new story | use existing bounded test/build timeouts | malformed docs output or zero-test drift is a hard failure, not a warning |

## Load Profile

- **Shared resources**: README/VitePress pages, `scripts/verify-m047-s06.sh` phase files, retained `.tmp/m047-s05/verify` proof trees, and the docs build output.
- **Per-operation cost**: file-content assertions, one docs build, and one assembled verifier replay.
- **10x breakpoint**: stale authority strings and malformed bundle handoff fail before throughput matters.

## Negative Tests

- **Malformed inputs**: stale "not shipped" markers, missing `HTTP.clustered(1, ...)` starter wording, missing S07 wrapper authority references, or broken retained-bundle/status files.
- **Error paths**: the S06 wrapper replays stale S05 proof, the docs build fails, or the contract test still allows mixed route-free/wrapper claims.
- **Boundary conditions**: route-free canonical surfaces remain first, the starter uses explicit-count-1 wrapper adoption, and `scripts/verify-m047-s06.sh` stays the final closeout rail.
