---
estimated_steps: 4
estimated_files: 1
skills_used: []
---

# T03: Add direct-port e2e proof for remote execution truth and local fallback

1. Add `compiler/meshc/tests/e2e_m039_s02.rs` by reusing the S01 harness patterns for repo-root resolution, port selection, child-process lifecycle, raw HTTP GETs, and per-node stdout/stderr capture.
2. Add a two-node proof that hits node A and node B directly, asserts `ingress_node` matches the contacted port, `target_node` / `execution_node` match the peer, `routed_remotely` is true, and the peer log contains the matching `request_id` execution line.
3. Add a single-node proof that starts one node and asserts truthful local fallback (`target_node == execution_node == ingress_node`, `routed_remotely == false`, no invented peer).
4. Fail closed on malformed JSON, missing route fields, early process exits, or zero-test filters by preserving raw bodies and log paths under `.tmp/m039-s02/`.

## Inputs

- `cluster-proof/main.mpl`
- `cluster-proof/work.mpl`
- `compiler/meshc/tests/e2e_m039_s01.rs`

## Expected Output

- `compiler/meshc/tests/e2e_m039_s02.rs with named two-node routing and single-node fallback proofs`
- `Per-node routing proof artifacts under .tmp/m039-s02/ when the tests run`

## Verification

cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_routes_work_to_peer_and_logs_execution -- --nocapture
cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_falls_back_locally_without_peers -- --nocapture

## Observability Impact

Adds direct-port proof logs that bind HTTP response truth to per-node stdout/stderr evidence, so ingress/execution drift is visible when routing breaks.
