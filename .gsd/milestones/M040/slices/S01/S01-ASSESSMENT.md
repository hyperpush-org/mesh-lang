# M040/S01 wrap-up assessment — auto-fix attempt 1

Slice S01 is **not complete**. I am stopping here because the context budget warning requires a handoff.

## What changed in this attempt

- `cluster-proof/work.mpl`
  - exported `WorkStatusPayload`
  - exported `SubmitMutation`

That was enough to move the failure frontier a little, but it did **not** close the slice.

## Verified current state

### Passing
- `cargo run -q -p meshc -- build cluster-proof`

### Failing
- `cargo run -q -p meshc -- test cluster-proof/tests`
  - `cluster-proof/tests/work.test.mpl` is still red
  - the remaining failures are mostly cross-module assumptions in the test file:
    - tuple returns from `apply_submit(...)` are being treated as `Int`/opaque at the call site
    - the test still expects direct field access on imported values that the compiler is not accepting cleanly across modules
- `cargo test -p meshc --test e2e_m039_s01 -- --nocapture`
  - still fails because it is the old M039 membership proof and now replays through a repo state where `cluster-proof/tests/work.test.mpl` is red
- `bash scripts/verify-m040-s01.sh`
  - still missing; this was the original verification gate failure

## Runtime observations that matter for the next unit

1. **Standalone keyed submit is still broken at runtime**
   - Launching `cluster-proof` in standalone mode succeeds
   - `POST /work` currently returns HTTP 500
   - likely root cause: `cluster-proof/work.mpl` still derives ingress/owner selection from `current_self_name()`, which is empty in standalone mode, so keyed submit falls into `invalid_target_selection`

2. **Cluster mode currently crashes before the HTTP proof can run**
   - starting `cluster-proof` with cluster env (`CLUSTER_PROOF_COOKIE`, `MESH_DISCOVERY_SEED`, `CLUSTER_PROOF_NODE_BASENAME`, `CLUSTER_PROOF_ADVERTISE_HOST`) aborts immediately
   - crash signature:
     - `compiler/mesh-rt/src/string.rs:171:14`
     - `misaligned pointer dereference: address must be a multiple of 0x8`
   - this happened for both IPv4 and IPv6 node configs before the node became ready

## Exact resume points

1. **Fix standalone identity in `cluster-proof/work.mpl` first**
   - give the keyed work path a truthful non-empty local identity when `Node.self()` is empty
   - likely shape: synthesize a standalone node identity from `PORT` (or another stable local marker) and use that for ingress/owner/execution fields in standalone mode
   - this should make `POST /work` and `GET /work/:request_key` testable without cluster startup

2. **Decide how to make `cluster-proof/tests/work.test.mpl` honest and compiler-compatible**
   - current direct cross-module assertions are too ambitious for the compiler surface right now
   - safest next move is to stop asserting imported tuple/struct internals directly and instead:
     - add narrow exported helper functions that return scalar values the test can compare, or
     - simplify the Mesh test so it only asserts exported JSON/status helpers and request-state transitions through supported surfaces

3. **Create the missing M040 proof artifacts**
   - add `compiler/meshc/tests/e2e_m040_s01.rs`
   - add `scripts/verify-m040-s01.sh`
   - the verifier should follow the same style as the M039 repo-root scripts: phase report, explicit success/failure logging, and non-zero test-count checks

4. **If cluster-mode keyed proof is still required after standalone passes, debug the runtime crash next**
   - cluster startup is currently blocked by the `mesh-rt` string panic above
   - do not trust any keyed two-node proof until that crash is resolved or explicitly scoped out

## Recommended next unit order

1. repair standalone keyed submit/status contract
2. make `cluster-proof/tests/work.test.mpl` green
3. add `e2e_m040_s01.rs`
4. add `scripts/verify-m040-s01.sh`
5. only then revisit two-node cluster proof if the `mesh-rt` crash still blocks it
