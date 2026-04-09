# S03: Single-Cluster Failure, Safe Degrade, and Rejoin — Research

## Summary

S03 is the **continuity proof slice for R048**, with **R046** and **R047** as supporting contracts. The good news is that the repo already has the right proof surfaces:
- `GET /membership` for truthful cluster state from live sessions
- `GET /work` for ingress/target/execution truth
- a runtime-owned discovery/reconnect loop in `mesh-rt`
- stable Rust harness patterns for spawning nodes, killing one, waiting for convergence, and preserving per-node artifacts

That means S03 should start as a **harness/verifier slice**, not a new runtime or app architecture slice. The likely work is:
1. add a new `e2e_m039_s03` proof that exercises **remote route -> node loss -> truthful self-only degrade -> continued local service -> node restart -> truthful rejoin -> remote route again**
2. add a `scripts/verify-m039-s03.sh` wrapper that replays S01 and S02 first, then fail-closes on the new named filters
3. only touch `cluster-proof/` or `mesh-rt` if the live rejoin proof exposes a real bug

I also replayed the current prerequisite contract during research:
- `bash scripts/verify-m039-s02.sh` ✅

So the current S01/S02 baseline is live and reusable.

## Requirement Focus

### Primary
- **R048 — A single Mesh cluster survives node failure and clean rejoin without manual repair.**
  - S03 is the primary owner.
  - The proof has to show: safe degrade, continued service for **new** work, and clean membership restoration after restart.

### Supporting
- **R046 — Cluster membership is truthful and updates on join, loss, and rejoin.**
  - S01 proved join + loss.
  - S03 needs the **rejoin** portion on the same proof app and harness style.
- **R047 — Mesh distributes work through runtime-native internal balancing.**
  - S02 proved happy-path routing and local fallback.
  - S03 needs to prove that routing stays truthful **through topology change**, not just at steady state.

## Skills Discovered

- Installed: `distributed-systems` (`yonatangross/orchestkit@distributed-systems`)
- Relevant rule for this slice: **do not add heavyweight distributed patterns unless multiple instances actually require them**. For S03 that means:
  - no distributed lock layer
  - no circuit-breaker/idempotency subsystem
  - no new coordinator/service mesh abstraction
  - use the existing narrow proof seam unless rejoin proves a real defect

## Implementation Landscape

### `cluster-proof/` app surface

#### `cluster-proof/main.mpl`
- Starts `Node.start(...)` in cluster mode.
- Starts work services, then HTTP.
- Exposes only:
  - `GET /membership`
  - `GET /work`
- Current env surface stays small and already matches the milestone direction.
- S03 should avoid widening this contract unless a real proof gap appears.

#### `cluster-proof/cluster.mpl`
- `membership_snapshot()` derives truth from `Node.self()` + `Node.list()`.
- `membership_payload(...)` already preserves the S01 contract:
  - `self`
  - `peers`
  - `membership = [self] + peers`
- Important: `Node.list()` is still **peer-only**, so S03 must keep using this helper contract instead of reading `Node.list()` directly.

#### `cluster-proof/work.mpl`
- Keeps the current narrow routing proof:
  - picks a non-self peer when one exists
  - falls back locally when no peer exists
  - logs execution on the execution node
  - returns a typed payload immediately
- Important current limitation: `next_request_token()` returns `0`, so `request_id` is always `work-0`.
  - This was fine for S02's one-request proofs.
  - It becomes ambiguous for S03 if the same cluster lifetime performs multiple `/work` calls across loss + rejoin.
- Current `handle_work(...)` already gives the right semantic shapes for S03:
  - remote route when membership has a peer
  - truthful local fallback when membership is self-only

#### `cluster-proof/tests/work.test.mpl`
- Covers pure selection rules only.
- Relevant only if S03 changes request-id generation or route-selection helpers.

### Runtime discovery / disconnect / reconnect surface

#### `compiler/mesh-rt/src/dist/discovery.rs`
- Owns runtime discovery from env.
- Reconciles periodically from DNS seed.
- Important test seam:
  - `MESH_DISCOVERY_INTERVAL_MS`
  - default is `5000ms`
- Rejoin is already conceptually supported by the existing loop:
  - resolve candidates again
  - filter out self and already-connected targets
  - connect to new targets
- This is a strong hint that S03 may not need runtime changes unless the live restart proof exposes a duplicate-session or timing bug.

#### `compiler/mesh-rt/src/dist/node.rs`
Key relevant behavior already exists:
- `cleanup_session(remote_name)` removes disconnected sessions from `NodeState`
- `handle_node_disconnect(...)` fires local failure cleanup and `:nodedown`
- `handle_node_connect(...)` fires `:nodeup`
- `register_session(...)` has duplicate-connection tiebreaking

Important implication for rejoin:
- if a restarted node reconnects before the old dead session is fully cleaned up, the duplicate-connection tiebreaker can transiently reject one side
- the proof should therefore assert **eventual reconvergence**, not “instant reconnect on first attempt”

### Existing Rust harnesses

#### `compiler/meshc/tests/e2e_m039_s01.rs`
Provides the reusable failure-path substrate already needed for S03:
- `ClusterProofConfig`
- `spawn_cluster_proof(...)`
- `stop_cluster_proof(...)`
- `kill_cluster_proof(...)`
- `wait_for_membership(...)`
- per-node stdout/stderr preservation
- node-loss proof already waits for truthful self-only membership after peer death

#### `compiler/meshc/tests/e2e_m039_s02.rs`
Provides the reusable routing substrate already needed for S03:
- `wait_for_work_response(...)`
- `parse_work_snapshot(...)`
- exact assertions for remote routing and local fallback
- durable `node-*-work.json` artifacts
- execution-node log assertion

Important S03 planning detail:
- S02 is intentionally **startup-order-sensitive** and proves opposite ingress directions in separate cluster lifetimes.
- S03 does **not** need symmetric two-direction proof to close R048. One ingress direction is enough if it proves:
  - remote route before failure
  - local fallback after failure
  - remote route again after rejoin

### Existing verifier patterns

#### `scripts/verify-m039-s01.sh`
- builds `cluster-proof`
- runs named filters
- fail-closes on missing `running N test`
- records `.tmp/m039-s01/verify/phase-report.txt`

#### `scripts/verify-m039-s02.sh`
- replays S01 first
- then runs named S02 filters
- copies timestamped test artifacts into stable `verify/` subdirs
- this is the right model for S03

## Natural Seams / Task Boundaries

### 1. New Rust proof surface: `compiler/meshc/tests/e2e_m039_s03.rs`
This should be the first substantive task.

Recommended scope:
- reuse S01/S02 helpers and shapes
- create a new artifact root `.tmp/m039-s03/`
- prove the continuity path in named tests

Recommended named tests:
1. `e2e_m039_s03_degrades_safely_and_serves_locally_after_peer_loss`
   - boot node A + node B
   - wait for `/membership` convergence on both
   - hit `/work` on node A and confirm remote execution on node B
   - kill node B
   - wait until node A reports self-only membership
   - hit `/work` on node A again and confirm truthful local fallback
2. `e2e_m039_s03_rejoins_and_routes_to_peer_again_without_manual_repair`
   - boot node A + node B
   - converge
   - kill node B
   - wait for node A self-only membership
   - restart node B with the **same identity**
   - wait for both nodes to report two-node membership again
   - hit `/work` on node A and confirm remote execution on node B again

That split keeps failure diagnostics narrower than one huge end-to-end test while still covering the full S03 contract.

### 2. Possible `cluster-proof/` observability tweak
Only do this if the S03 harness proves the current signals are too ambiguous.

Most likely candidate:
- `cluster-proof/work.mpl`
- `cluster-proof/tests/work.test.mpl`

Reason:
- `request_id` is always `work-0`
- S03 wants multiple `/work` calls in one cluster lifetime
- repeated `work-0` makes postmortem correlation weaker than S02

Smallest acceptable improvement if needed:
- make `request_token` unique per request using a scalar-safe mechanism
- keep the response typed and string/bool-heavy
- do **not** reintroduce the failed S02 coordinator/result-registry design unless absolutely necessary

### 3. New verifier wrapper: `scripts/verify-m039-s03.sh`
This should be the last task.

Recommended shape:
1. run `cargo run -q -p meshc -- test cluster-proof/tests`
2. run `cargo run -q -p meshc -- build cluster-proof`
3. replay `bash scripts/verify-m039-s01.sh`
4. replay `bash scripts/verify-m039-s02.sh`
5. run the named S03 tests with non-zero test-count checks
6. snapshot and copy new `.tmp/m039-s03/...` artifacts into stable `verify/` dirs
7. write `.tmp/m039-s03/verify/phase-report.txt`

## Findings and Constraints

### 1. Rejoin needs the same stable node identity
The restarted node must come back with the same:
- basename
- advertised host
- cluster port

Otherwise the proof is node replacement, not clean rejoin.

### 2. Current harness log naming will clobber restarted-node evidence
In S01/S02, log files are keyed by `node_basename` in one log dir.

That breaks on restart because a second `node-b` process in the same test directory will overwrite/truncate the first `node-b.stdout.log` and `node-b.stderr.log`.

S03 needs one of:
- incarnation-specific log filenames (`node-b-run1.stdout.log`, `node-b-run2.stdout.log`)
- or separate phase/incarnation subdirectories within one test root

This is a real planner concern. Without it, the rejoin proof will erase the pre-crash evidence.

### 3. Multi-request artifacts need per-phase naming
Same issue for response bodies:
- S02 writes a single `node-a-work.json`
- S03 will likely need at least:
  - pre-loss work body
  - degraded work body
  - post-rejoin work body

Use explicit phase names instead of reusing one filename.

### 4. `work-0` is a real correlation weakness for S03
Because `request_id` is fixed today, S03 should not rely on `request_id` alone if multiple `/work` calls happen in one node lifetime.

Acceptable options:
- fix request-id generation in app code
- or key assertions to explicit artifact files plus log append order

The first is cleaner if it can be done without widening scope.

### 5. Reconnect timing is discovery-loop-bound, not instant
The rejoin path depends on the runtime discovery loop. Default interval is `5000ms`.

Implications:
- use eventual-convergence assertions
- give restart proofs a longer timeout than the S02 happy path
- optionally lower `MESH_DISCOVERY_INTERVAL_MS` in the Rust harness to tighten local proof runtime without changing the public operator contract

### 6. Node monitors exist, but the slice does not obviously need them
`mesh_node_monitor(...)` and `:nodeup` / `:nodedown` delivery already exist.

But for S03 acceptance, the existing external surfaces are probably enough:
- `/membership`
- `/work`
- stdout/stderr logs

Recommendation: do **not** add extra Mesh-side monitor actors unless polling `/membership` proves dishonest or insufficient.

### 7. Do not broaden S03 into in-flight durability or worker supervision
S03 only needs to prove:
- safe degrade after a node dies
- continued service for **new** work
- clean rejoin

It does **not** need:
- in-flight request completion across failure
- durable replay
- global worker registries
- distributed locking
- cross-cluster anything

## Recommendation

Build S03 as a **new proof harness on top of the existing proof app**.

### What to prove first

1. **Degraded continuity before rejoin**
   - A routes to B successfully
   - B dies
   - A membership becomes self-only
   - A still serves `/work` locally and truthfully

2. **Then clean rejoin**
   - restart B with the same identity
   - A and B membership reconverges to two nodes
   - A routes to B again without any manual repair step

This order matters. If degrade is not stable first, rejoin noise will hide the real bug.

### When to touch app/runtime code

- If the S03 e2e passes with only harness/verifier work, stop there.
- If it fails because the evidence is ambiguous, make the smallest app-side observability fix (`request_id`, per-phase logs).
- If it fails because the cluster does not actually reform, then debug `mesh-rt` reconnect/session cleanup.

## Verification Plan

### Baseline prerequisites
- `cargo run -q -p meshc -- test cluster-proof/tests`
- `cargo run -q -p meshc -- build cluster-proof`
- `bash scripts/verify-m039-s01.sh`
- `bash scripts/verify-m039-s02.sh`

### Slice-specific commands
- `cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_degrades_safely_and_serves_locally_after_peer_loss -- --nocapture`
- `cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_rejoins_and_routes_to_peer_again_without_manual_repair -- --nocapture`
- `bash scripts/verify-m039-s03.sh`

### Verifier artifact expectations
Recommended stable artifact root:
- `.tmp/m039-s03/verify/`

Recommended copied evidence:
- pre-loss remote-route JSON body
- post-loss local-fallback JSON body
- post-rejoin remote-route JSON body
- per-incarnation node stdout/stderr logs
- phase manifest / phase report
- non-zero test-count logs for each named filter

## Files Most Likely To Change

- `compiler/meshc/tests/e2e_m039_s03.rs` *(new; primary slice implementation)*
- `scripts/verify-m039-s03.sh` *(new; canonical local acceptance wrapper)*
- `cluster-proof/work.mpl` *(only if unique request correlation or small observability repair is needed)*
- `cluster-proof/tests/work.test.mpl` *(only if work helper behavior changes)*

## Evidence Checked

- `cluster-proof/main.mpl`
- `cluster-proof/cluster.mpl`
- `cluster-proof/work.mpl`
- `cluster-proof/tests/work.test.mpl`
- `compiler/mesh-rt/src/dist/discovery.rs`
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/actor/mod.rs` (`mesh_node_monitor`)
- `compiler/meshc/tests/e2e_m039_s01.rs`
- `compiler/meshc/tests/e2e_m039_s02.rs`
- `scripts/verify-m039-s01.sh`
- `scripts/verify-m039-s02.sh`
- `.gsd/REQUIREMENTS.md`
- `.gsd/PROJECT.md`
- `.gsd/milestones/M039/slices/S01/S01-RESEARCH.md`
- `.gsd/milestones/M039/slices/S01/S01-SUMMARY.md`
- `.gsd/milestones/M039/slices/S02/S02-SUMMARY.md`
- `.gsd/milestones/M039/slices/S02/S02-UAT.md`

### Research run results
- `bash scripts/verify-m039-s02.sh` ✅