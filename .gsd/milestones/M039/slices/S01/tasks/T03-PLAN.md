---
estimated_steps: 4
estimated_files: 3
skills_used:
  - rust-best-practices
  - test
---

# T03: Make the local verifier authoritative and prove membership loss updates fail closed

**Slice:** S01 — General DNS Discovery & Membership Truth
**Milestone:** M039

## Description

Close the slice with one replayable local proof surface instead of a bag of ad hoc commands. Extend the Rust e2e harness from the convergence task so it can kill one node, wait for the surviving endpoint to shrink membership truthfully, and leave per-node logs behind when anything stalls. Then wrap the named runtime and e2e checks in a repo-local verifier script that fail-closes on missing tests, port collisions, or missing artifacts.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Spawned proof-app child processes | Tear them down, keep stdout/stderr per node, and fail the test with the last observed membership payload. | Timeout fails the proof and preserves node logs instead of hanging until CI kills the process. | Treat crash-loop or partial startup logs as a verifier failure, not as a transient warning. |
| Local dual-stack DNS seed (`localhost`) | Fail the local proof clearly if both address families are not usable on this host, with the missing family called out in logs. | N/A | Reject a single-address-only result as insufficient for the two-node local proof. |
| Verifier wrapper script | Stop on the first failing phase, keep logs under `.tmp/m039-s01/`, and refuse to pass if a named test filter ran 0 tests. | Fail the current phase and leave the partial artifacts in place for inspection. | Treat malformed JSON/log artifacts as proof failure rather than silently skipping checks. |

## Load Profile

- **Shared resources**: local TCP ports, child-process stdout/stderr logs, and repeated HTTP polling against the proof endpoint.
- **Per-operation cost**: two proof-app processes, short polling loops, and one wrapper-script replay.
- **10x breakpoint**: port collisions or slow convergence timeouts fail before CPU becomes a problem, so the verifier must report which phase stalled.

## Negative Tests

- **Malformed inputs**: missing required env for one node, invalid port reuse, or missing output directory for logs.
- **Error paths**: one node exits before convergence, the surviving node never drops the lost peer, or the verifier wrapper sees `running 0 tests`.
- **Boundary conditions**: surviving node reports zero peers after loss but still includes itself in `membership`; loss detection must not depend on manual peer removal.

## Steps

1. Extend `compiler/meshc/tests/e2e_m039_s01.rs` with a second named proof that starts the dual-stack local cluster, kills one node, and waits for the survivor’s endpoint to report truthful membership shrinkage.
2. Capture per-node stdout/stderr from the Rust harness so timeouts and malformed membership responses include concrete logs instead of generic assertions.
3. Add `scripts/verify-m039-s01.sh` as the single local replay command that builds `cluster-proof`, runs the named `mesh-rt` and `meshc` tests in order, archives logs under `.tmp/m039-s01/`, and fails closed on 0-test filters.
4. Keep the wrapper local-only for this slice; S04 will own the canonical one-image/Fly replay and docs reconciliation.

## Must-Haves

- [ ] The local proof covers both discovery convergence and membership shrinkage after node loss with no manual peer repair.
- [ ] Failures leave per-node logs under `.tmp/m039-s01/` so future debugging starts from evidence, not from guesswork.
- [ ] `scripts/verify-m039-s01.sh` becomes the authoritative slice acceptance command from repo root.

## Verification

- `cargo test -p meshc --test e2e_m039_s01 e2e_m039_s01_membership_updates_after_node_loss -- --nocapture`
- `bash scripts/verify-m039-s01.sh`

## Observability Impact

- Signals added/changed: verifier phase logs plus per-node stdout/stderr capture for convergence and loss assertions.
- How a future agent inspects this: rerun `bash scripts/verify-m039-s01.sh` and inspect `.tmp/m039-s01/`.
- Failure state exposed: which phase stalled, what the last endpoint payload was, and what each node logged before failure.

## Inputs

- `cluster-proof/main.mpl` — proof-app entrypoint from T02.
- `cluster-proof/config.mpl` — env contract and advertised-identity builder from T02.
- `cluster-proof/cluster.mpl` — membership endpoint payload contract from T02.
- `compiler/meshc/tests/e2e_m039_s01.rs` — convergence harness to extend with loss assertions.
- `compiler/mesh-rt/src/dist/discovery.rs` — runtime discovery signals and retry behavior to reuse in failure output.
- `compiler/meshc/tests/e2e_reference_backend.rs` — reference harness patterns for spawned-process logs and polling.

## Expected Output

- `compiler/meshc/tests/e2e_m039_s01.rs` — named loss/shrinkage proof with per-node log capture.
- `scripts/verify-m039-s01.sh` — authoritative local replay command and artifact capture.
