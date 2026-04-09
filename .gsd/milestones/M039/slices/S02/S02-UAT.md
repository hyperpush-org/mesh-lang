# S02: Native Cluster Work Routing Proof App — UAT

**Milestone:** M039
**Written:** 2026-03-28

## UAT Type

- UAT mode: live-runtime
- Why this mode is sufficient: S02’s contract is not just compilation or static routing logic; it requires a real multi-process cluster where one node accepts HTTP work and another node proves execution.

## Preconditions

- Run from the repo root with Cargo/Rust available.
- Do not override the cluster with manual peer lists; S02 depends on the S01 DNS-discovery contract.
- Let the verifier choose ephemeral ports. If you run the named tests manually outside the wrapper, avoid reusing stale `cluster-proof` processes from an older run.

## Smoke Test

1. Run `bash scripts/verify-m039-s02.sh`.
2. **Expected:** The command exits 0.
3. **Expected:** `.tmp/m039-s02/verify/phase-report.txt` records `passed` for `cluster-proof-tests`, `build-cluster-proof`, `s01-contract`, `s02-remote-route`, and `s02-local-fallback`.

## Test Cases

### 1. The proof app builds and its pure routing helpers stay honest

1. Run `cargo run -q -p meshc -- test cluster-proof/tests`.
2. Run `cargo run -q -p meshc -- build cluster-proof`.
3. **Expected:** Both commands exit 0.
4. **Expected:** The helper tests prove deterministic peer-preferred routing and truthful local fallback before the live runtime harness runs.

### 2. Hitting node A proves Mesh routed the work to node B

1. Run `cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_routes_work_to_peer_and_logs_execution -- --nocapture`.
2. Open the newest `.tmp/m039-s02/e2e-m039-s02-ingress-a-*/node-a-work.json` file.
3. **Expected:** The JSON body has `ok=true`, `timed_out=false`, `error=""`, `ingress_node="node-a@..."`, `target_node="node-b@..."`, `execution_node="node-b@..."`, and `routed_remotely=true`.
4. Open the matching `node-b.stdout.log` file from the same artifact directory.
5. **Expected:** It contains `[cluster-proof] work executed request_id=work-0 execution=node-b@...`.

### 3. Hitting node B proves the opposite ingress direction in its own cluster lifetime

1. Reuse the same command output from the named test above; it should also create a newest `.tmp/m039-s02/e2e-m039-s02-ingress-b-*/node-b-work.json` bundle.
2. Open that `node-b-work.json` file.
3. **Expected:** The JSON body has `ingress_node="node-b@..."`, `target_node="node-a@..."`, `execution_node="node-a@..."`, and `routed_remotely=true`.
4. Open the matching `node-a.stdout.log` file from the same artifact directory.
5. **Expected:** It contains `[cluster-proof] work executed request_id=work-0 execution=node-a@...`.

### 4. A single-node cluster stays truthful and falls back locally

1. Run `cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_falls_back_locally_without_peers -- --nocapture`.
2. Open the newest `.tmp/m039-s02/e2e-m039-s02-solo-*/node-solo-work.json` file.
3. **Expected:** The JSON body has `ok=true`, `routed_remotely=false`, `fell_back_locally=true`, and `ingress_node == target_node == execution_node == node-solo@...`.
4. Open the matching `node-solo.stdout.log` file.
5. **Expected:** It contains `[cluster-proof] work executed request_id=work-0 execution=node-solo@...` and does not imply any invented peer.

### 5. The assembled verifier preserves stable postmortem artifacts

1. Run `bash scripts/verify-m039-s02.sh` again.
2. Inspect `.tmp/m039-s02/verify/04-s02-remote-route-artifacts.txt` and `.tmp/m039-s02/verify/05-s02-local-fallback-artifacts.txt`.
3. **Expected:** The remote manifest lists one ingress-a artifact directory and one ingress-b artifact directory, each with `node-*-work.json` plus both nodes’ stdout/stderr logs.
4. **Expected:** The local manifest lists one solo artifact directory with `node-solo-work.json` plus stdout/stderr logs.

## Edge Cases

### `/membership` stays the separate truth surface while `/work` proves routing

1. Run the named remote-routing test: `cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_routes_work_to_peer_and_logs_execution -- --nocapture`.
2. **Expected:** The test only passes after both nodes first converge on `/membership` length 2 with truthful `self` and `peers`, then `/work` is exercised.
3. **Expected:** A routing pass without prior membership convergence is treated as invalid proof.

### Named filter drift must fail closed

1. Run `bash scripts/verify-m039-s02.sh`.
2. Open `.tmp/m039-s02/verify/04-s02-remote-route.test-count.log` and `05-s02-local-fallback.test-count.log`.
3. **Expected:** Each file records a non-zero `running N test` count. If a filter ever runs 0 tests, the wrapper must fail instead of claiming success.

## Failure Signals

- `bash scripts/verify-m039-s02.sh` exits non-zero or `phase-report.txt` is missing one of the five `passed` lines.
- A `node-*-work.json` body is malformed JSON or is missing `request_id`, `ingress_node`, `target_node`, `execution_node`, `routed_remotely`, or `fell_back_locally`.
- The two-node proof reports `ingress_node == execution_node` or `routed_remotely=false` when a peer exists.
- The single-node proof reports an invented peer, `routed_remotely=true`, or mismatched ingress/target/execution values.
- The matching execution-node stdout log is missing the `[cluster-proof] work executed request_id=... execution=...` line for the returned `request_id`.

## Requirements Proved By This UAT

- R047 — proves runtime-native internal balancing by distinguishing ingress-node acceptance from execution-node work on a live two-node cluster, while also proving truthful local fallback on a one-node cluster.

## Not Proven By This UAT

- R048 clean degrade and rejoin after node loss.
- The one-image/Fly operator path and public distributed-runtime docs truth planned for S04.
- Per-request unique cross-node correlation; the current stable proof token is still `work-0`.

## Notes for Tester

- Start with `bash scripts/verify-m039-s01.sh` if routing proof regresses unexpectedly; S02 is only honest on top of a healthy S01 discovery/membership contract.
- The two remote-route directions are intentionally proven in separate cluster lifecycles because the current harness is startup-order-sensitive.
- The fastest truthful debug path is: `phase-report.txt` -> `node-*-work.json` -> matching execution-node stdout log.
