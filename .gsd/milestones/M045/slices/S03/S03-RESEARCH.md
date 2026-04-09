# S03 Research — Tiny Example Failover Truth

## Requirements in Play
- **Directly advances:** R078 — the same tiny local example must prove failover end to end.
- **Supports:** R077, R079, R080 — the primary example must stay small and runtime-owned while gaining failover truth.
- **Relies on existing runtime/operator truth:** the continuity/authority/operator surfaces already productized in M042–M044 remain the truthful source; this slice should reuse them, not replace them.

## Skills Discovered
- **Loaded:** `debug-like-expert`
  - Relevant rule applied here: **verify, don’t assume**. I treated the old failover rail as evidence to confirm, not as folklore.
- **Installed this unit:** `distributed-systems` (`yonatangross/orchestkit@distributed-systems`)
  - Discovery commands:
    - `npx skills find "distributed systems"`
    - `npx skills find "fault tolerance"`
  - Installed with:
    - `npx skills add yonatangross/orchestkit@distributed-systems -g -y`
  - Relevant rule applied here: keep idempotency/failover owned by the shared distributed surface, not by per-app retry or promotion glue.
- **Already available and relevant if runtime work becomes necessary:** `rust-best-practices`

## Summary
- The **runtime-owned auto-promotion / automatic-recovery seam already exists** and is green in this tree.
- The main missing work for S03 is **not** inventing a new failover mechanism first; it is **proving the same runtime-owned failover story on the scaffolded tiny example** rather than on `cluster-proof`.
- The current scaffold is already tiny and honest on source shape, but only **single-node completion** is proven there today. The destructive two-node failover rail still lives in `cluster-proof` tests.
- The practical risk is **timing**: the scaffold work function finishes immediately, so S03 needs a deterministic way to observe a **pending mirrored record** before killing the primary.
- `S03-PLAN.md` currently contains only the goal/demo header and **no tasks**, so the planner needs a fresh decomposition from this research.

## Live Evidence Gathered
- Ran:
  - `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture`
- Result:
  - **1 passed**
- Meaning:
  - The existing runtime-owned auto-promotion / auto-recovery rail is green before S03 starts.

## Implementation Landscape

### 1. Tiny scaffold contract
**File:** `compiler/mesh-pkg/src/scaffold.rs`

What exists now:
- Generated clustered app has only:
  - `/health`
  - `POST /work/:request_key`
  - `Node.start_from_env()` bootstrap
  - `Continuity.submit_declared_work("Work.execute_declared_work", request_key, request_key, 0)`
- Generated Mesh code intentionally omits:
  - app-owned bootstrap env parsing
  - `Node.start(...)` calls
  - `Continuity.mark_completed(...)`
  - `/promote`
  - app-owned membership/status/operator routes
- Generated README already points users at runtime truth:
  - `meshc cluster status`
  - `meshc cluster continuity`
  - `meshc cluster diagnostics`
- Generated `work.mpl` stays pure:
  - `pub fn execute_declared_work(request_key :: String, attempt_id :: String) -> Int`

Important planner implications:
- The tiny example already matches the ownership goal.
- **Request keys are path params**, not JSON body fields, so a destructive harness can vary keys cheaply via `POST /work/<request_key>`.
- The scaffold exposes **no app-owned status surface**; S03 must prove failover through the CLI/runtime surfaces.

### 2. Current scaffold guardrails
**Files:**
- `compiler/meshc/tests/tooling_e2e.rs`
- `compiler/meshc/tests/e2e_m045_s02.rs`

What they already protect:
- `tooling_e2e.rs::test_init_clustered_creates_project`
  - guards the generated source against leaked bootstrap/completion/status glue.
- `e2e_m045_s02.rs`
  - `m045_s02_scaffold_runtime_completion_contract_stays_tiny`
  - `m045_s02_scaffold_runtime_completion_reaches_completed_without_app_glue`
  - These prove the scaffold source shape and local runtime-owned completion.

What they do **not** protect yet:
- no explicit assertion that the README keeps `meshc cluster diagnostics`
- no explicit assertion that scaffolded sources omit manual failover surfaces like:
  - `Continuity.promote`
  - `/promote`
- no destructive failover proof on the scaffolded binary itself

### 3. Existing runtime failover path
**Files:**
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-codegen/src/codegen/expr.rs`

#### `compiler/mesh-rt/src/dist/node.rs`
Key seams:
- `handle_node_disconnect(node_name, node_id)`
  - marks owner-loss / degraded continuity state
  - calls `maybe_automatic_promote_and_resume(node_name)` when conditions justify it
- `automatic_promotion_reason(...)`
  - promotion is allowed only when:
    - local authority is `standby`
    - remaining peer count is `0`
    - **all pending records** are standby-mirrored records where:
      - `owner_node == disconnected_node`
      - `replica_node == local_node`
      - `replica_status ∈ {preparing, mirrored}`
  - otherwise runtime logs `automatic_promotion_rejected:*`
- `automatic_recovery_candidates(...)`
  - only resumes records that are now:
    - `phase=submitted`
    - `result=pending`
    - `cluster_role=primary`
    - `replica_status=owner_lost`
    - `owner_node == disconnected_node`
- `submit_declared_work(...)`
  - stores `declared_handler_runtime_name` in the continuity request
  - dispatches local/remote execution through the declared handler registry
- `complete_declared_work(...)`
  - records completion with the runtime-chosen execution node

#### `compiler/mesh-rt/src/dist/continuity.rs`
Key seams:
- `promote_authority()`
  - flips standby -> primary, bumps epoch, reprojects records, broadcasts updated truth
- `mark_owner_loss_records_for_node_loss(...)`
- `degrade_replica_records_for_node_loss(...)`
- `degrade_replication_health_for_node_loss(...)`
- `mark_completed(...)`
  - authoritative completion surface
- operator diagnostics already record:
  - `owner_lost`
  - `degraded`
  - `promote`
  - `fenced_rejoin`
  - other continuity transitions

#### `compiler/mesh-codegen/src/codegen/expr.rs`
Key seam:
- generated `__declared_work_*` wrappers automatically call `mesh_continuity_complete_declared_work` after the body function returns
- this is why the scaffolded `work.mpl` can stay pure and should remain pure in S03

Planner implication:
- If S03 needs code changes below the harness level, the cleanest seam is still **runtime/codegen**, not a scaffold-specific failover helper.

### 4. Existing failover proof rail to borrow from
**File:** `compiler/meshc/tests/e2e_m044_s04.rs`

What it already has:
- authoritative destructive failover harness
- reusable patterns for:
  - deterministic placement search:
    - `request_key_matches_placement(...)`
    - `find_submit_matching_placement(...)`
  - rejoin/fencing truth expectations
  - retained `scenario-meta.json` and per-node logs

Important caveats:
- This harness is still **cluster-proof-shaped**:
  - submit response includes owner/replica/status fields
  - work path has configurable delay
- The scaffold does **not** have either of those luxuries.

Planner implication:
- Reuse the **truth pattern**, not the proof-app assumptions.
- The new S03 rail should borrow the placement helper idea, but confirm actual placement using the runtime CLI instead of trusting a prefilter alone.

### 5. Runtime-owned truth surfaces available to the tiny example
**File:** `compiler/meshc/src/cluster.rs`

The public truth surfaces are already enough for S03:
- `meshc cluster status <node> --json`
  - membership + authority
- `meshc cluster continuity <node> <request_key> --json`
  - full continuity record
- `meshc cluster diagnostics <node> --json`
  - failover/recovery/operator transitions

Planner implication:
- The tiny example does **not** need new HTTP routes for membership, status, or failover.
- S03 can stay honest by proving everything through these CLI/runtime surfaces.

### 6. Existing verifier patterns
**Files:**
- `scripts/verify-m045-s02.sh`
- `scripts/verify-m044-s04.sh`

Useful established pattern:
- flattened replay of prerequisites
- fail-closed zero-test detection
- retained bundle pointer + bundle-shape checks

Important local rule from S02:
- avoid nested verifier wrappers when a flattened replay is possible; the older wrapper stack can hang on this host

## Key Constraints and Risks

### 1. Failover only works on the “primary-owned, standby-mirrored, still-pending” shape
S02’s happy-path remote-owner rail deliberately searched for `owner=standby, replica=primary`. That shape does **not** prove owner-loss promotion when the primary dies; it only proves replica loss.

For S03, the harness must deliberately reach and confirm a record with:
- `owner_node = primary`
- `replica_node = standby`
- `phase = submitted`
- `result = pending`
- `replica_status = mirrored` (or `preparing` before ack finishes)

before killing the primary.

### 2. The scaffold has no built-in work delay
The generated `work.mpl` returns immediately. That is the slice’s main practical risk.

`cluster-proof` keeps failover deterministic with `CLUSTER_PROOF_WORK_DELAY_MS`; the scaffold has no equivalent. Without a pending window, a destructive failover test can race completion and stop proving anything.

### 3. Hash prefilters are not authoritative
Project knowledge already warns that local placement prediction can drift from actual runtime truth.

For the scaffold this matters even more because submit responses only contain:
- `request_key`
- `attempt_id`
- `outcome`

They do **not** echo owner/replica placement.

Planner implication:
- any prefilter is only a candidate-selection heuristic
- the harness must confirm actual placement using `meshc cluster continuity ... --json` before destructive steps

### 4. The scaffold exposes only CLI truth by design
No `/membership`, no `/work/:request_key` status route, no `/promote`.

That is correct for R079, but it means the e2e harness must poll CLI truth instead of leaning on app-owned JSON surfaces.

### 5. Do not reintroduce app-owned failover choreography
From D209/D215 plus the distributed-systems skill:
- do **not** add manual promotion endpoints
- do **not** add local retry helpers in the scaffold
- do **not** add app-owned status/operator routes just to make testing easier

If deterministic timing becomes the blocker, prefer a lower runtime/codegen seam over app-level proof glue.

### 6. Exclusive ownership of the cluster during verification
The assembled clustered verifiers already assume they own the processes and ports. Do not plan concurrent replays on the same surface.

## Recommendation
Treat S03 as primarily a **scaffold-first proof/harness slice**, not as a new runtime feature wave.

The runtime failover machinery is already present and green. The honest missing work is moving the destructive proof onto the generated tiny example without leaking failover/status logic back into app code.

### Recommended execution order

#### 1. Build the scaffold destructive failover rail first
Create a new test target, likely:
- `compiler/meshc/tests/e2e_m045_s03.rs`

Use:
- temp-project init/build/spawn patterns from `e2e_m045_s02.rs`
- placement-search / rejoin-truth patterns from `e2e_m044_s04.rs`
- CLI truth only (`cluster status`, `cluster continuity`, `cluster diagnostics`)

Do **not** add new app HTTP routes.

#### 2. Prefer a harness-only timing strategy before touching product code
First attempt should preserve the tiny example unchanged.

Best path to try first:
- choose deterministic candidate keys biased toward `owner=primary, replica=standby`
- submit to the primary scaffold node
- immediately confirm actual pending mirrored truth via `meshc cluster continuity`
- if single submits are too racy, try **batching multiple matching keys** before the kill so at least one mirrored record remains pending

Why batch-first is worth trying:
- it preserves the tiny example surface
- automatic promotion already supports more than one promotable pending record, as long as all pending records match the allowed mirrored-owner-loss shape

Only if this proves flaky should the slice fall back to a lower-layer timing seam.

#### 3. Tighten scaffold source-contract tests after the failover rail is clear
Likely files:
- `compiler/meshc/tests/tooling_e2e.rs`
- optionally the scaffold contract test inside `compiler/meshc/tests/e2e_m045_s02.rs`

Useful new assertions:
- README contains `meshc cluster diagnostics`
- scaffolded sources omit `Continuity.promote`
- scaffolded sources omit `/promote`
- scaffolded sources still omit app-owned status/placement/helper logic

#### 4. Add a flattened assembled verifier
Likely new file:
- `scripts/verify-m045-s03.sh`

Pattern to copy:
- phase/status files and retained bundle checks from `scripts/verify-m045-s02.sh`
- direct prerequisite replay, not nested wrapper calls

## Suggested Implementation Shape

### New scaffold failover test target
**Likely file:** `compiler/meshc/tests/e2e_m045_s03.rs`

Natural structure:
1. scaffold temp project
2. `meshc build` temp project
3. spawn **two copies of the same scaffold binary** with:
   - shared cookie
   - shared cluster port
   - different HTTP ports / node names
   - primary + standby roles at epoch 0
4. wait for `meshc cluster status` convergence on both nodes
5. drive one or more submit attempts against the primary until CLI truth shows a pending mirrored record with actual:
   - `owner=primary`
   - `replica=standby`
6. kill the primary process
7. on standby, wait for:
   - `cluster status` -> `cluster_role=primary`, `promotion_epoch=1`
   - `cluster diagnostics` -> `automatic_promotion` and `automatic_recovery`
   - `cluster continuity` -> new attempt, completed, `owner=standby`, `execution_node=standby`
8. restart the primary with the same node identity
9. wait for fenced rejoin truth on both nodes:
   - old primary becomes `standby` at epoch 1
   - continuity record remains on the failover attempt
   - diagnostics/logs show fenced rejoin

### Important harness helpers to add/adapt
Borrow/adapt locally rather than broad refactor unless duplication gets out of hand:
- from `e2e_m045_s02.rs`
  - temp scaffold init/build helpers
  - process spawn/stop helpers
  - HTTP submit helper
  - `run_meshc_cluster(...)`
  - cluster status / continuity polling helpers
- from `e2e_m044_s04.rs`
  - deterministic placement helper idea
  - scenario artifact retention pattern
  - rejoin/fencing expectations

### If timing still blocks the scaffold rail
Fallback direction should be:
- **runtime-owned timing seam** at the declared-work wrapper/runtime boundary
- not scaffold-owned failover/status logic

The cleanest fallback seam is likely around:
- declared-work wrapper completion in `compiler/mesh-codegen/src/codegen/expr.rs`
- or a lower runtime-owned hook adjacent to `complete_declared_work(...)`

Avoid as long as possible:
- generated `WORK_DELAY_MS`-style app env knobs
- manual promotion routes
- scaffold-specific status/operator endpoints

## Verifier Plan
Minimum assembled acceptance stack:
1. `cargo test -p mesh-rt automatic_promotion_ -- --nocapture`
2. `cargo test -p mesh-rt automatic_recovery_ -- --nocapture`
3. `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture`
4. `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`
5. `cargo test -p meshc --test e2e_m045_s02 m045_s02_scaffold_runtime_completion_ -- --nocapture`
6. `cargo test -p meshc --test e2e_m045_s03 m045_s03_ -- --nocapture`
7. `bash scripts/verify-m045-s03.sh`

Verifier should fail closed on:
- missing `running N test` evidence
- 0-test filters
- missing fresh scaffold failover artifact directory
- malformed retained bundle pointer
- retained bundle missing key evidence files

## Recommended Retained Bundle Contents
For the new scaffold failover bundle, keep at least:
- scaffold contract artifacts:
  - `init.log`
  - `build.log`
  - `main.mpl`
  - `work.mpl`
  - `README.md`
- scenario metadata:
  - request key(s)
  - chosen candidate(s)
  - node names / ports
- HTTP evidence:
  - submit request/response artifacts
- runtime truth artifacts:
  - pre-kill cluster status JSON
  - pre-kill continuity JSON showing pending mirrored truth
  - standby post-promotion status JSON
  - standby post-recovery continuity JSON
  - standby diagnostics JSON
  - post-rejoin status/continuity JSON on both nodes
- node logs:
  - primary stdout/stderr before kill
  - standby stdout/stderr
  - primary stdout/stderr after rejoin

## Open Question to Resolve Early
**Can the scaffold failover rail stay deterministic with harness-only timing, or does it need a lower-layer timing seam?**

Recommended decision order:
1. try harness-only with deterministic primary-owned keys + immediate CLI confirmation + optional batch submit
2. if still flaky, add the smallest runtime-owned timing seam possible
3. do not fall back to app-owned failover/status/promotion helpers in the scaffold
