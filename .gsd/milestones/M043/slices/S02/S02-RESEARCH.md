# M043 — Research

**Date:** 2026-03-28

## Summary

S02 owns the failover half of **R051**, supports **R052**'s same-image/small-env operator story, and is constrained by **R053** because promotion/fencing truth must stay aligned across runtime behavior, proof rails, and later docs.

The codebase already has the right raw ingredients:

- `mesh-rt` already owns keyed continuity state, sync, merge, and single-cluster owner-loss recovery.
- S01 already threaded `cluster_role`, `promotion_epoch`, and `replication_health` through the runtime record and `cluster-proof` status surfaces.
- M042 already proved the retry-rollover and same-identity rejoin pattern for newer-attempt truth.

But three current invariants make honest S02 impossible without runtime work first:

1. **Authority is process-static.** `compiler/mesh-rt/src/dist/continuity.rs` stores authority in a `OnceLock` via `current_authority_config()`. A pure env flip/restart promotion would lose the mirrored continuity registry because `continuity_registry()` is also process-local in-memory state.
2. **Inbound merges currently erase remote authority before precedence runs.** `project_remote_record(...)` overwrites the incoming record's role/epoch with local authority before `preferred_record(...)` compares anything. That was fine for S01's local standby projection, but it destroys stale-primary fencing on rejoin.
3. **`cluster-proof` reads startup env for authority truth.** `/membership` and the error/missing status payloads use `cluster-proof/config.mpl`, not runtime authority state. Dynamic promotion would immediately make those surfaces stale.

The narrowest honest S02 path is: **runtime-mutable authority state + explicit promotion action + epoch-based fencing + reuse of the existing owner-loss retry-rollover path for promoted recovery**. Do not treat process restart as the promotion primitive.

## Recommendation

1. **Do not implement promotion as restart-plus-env-change.**
   - Restarting a standby would wipe its mirrored continuity registry before failover because the registry is in-process only.
   - That fails the table-stakes R051 bar.

2. **Make promotion a runtime mutation, not a config reload.**
   - Replace the process-static `OnceLock<ContinuityAuthorityConfig>` with mutable runtime authority state.
   - Expose a narrow runtime API for:
     - explicit promotion
     - reading current authority/health state
   - This is the only clean way for `cluster-proof` to preserve mirrored state while changing authority.

3. **Treat `promotion_epoch` as the fencing token.**
   The `distributed-systems` fencing-token rule applies directly here: every mutating path has to validate authority ownership before acting. In this slice that means:
   - lower-epoch incoming upserts/snapshots must be ignored
   - lower-epoch local completions must not become authoritative
   - rejoining stale primaries must learn the newer epoch before accepting authority again

4. **Reuse the existing M042 recovery rollover instead of inventing a second failover mechanism.**
   The current recovery path already exists:
   - `compiler/mesh-rt/src/dist/node.rs::continuity_owner_loss_recovery_eligible(...)`
   - `compiler/mesh-rt/src/dist/continuity.rs::transition_retry_rollover_record(...)`
   - `cluster-proof/work_continuity.mpl::submit_required_replica_count(...)`

   The clean S02 move is to make promoted pending standby records become recovery-eligible under the new epoch, so the promoted standby can recover them through the same keyed retry flow instead of an app-authored failover state machine.

5. **Move live authority truth out of `cluster-proof/config.mpl`.**
   Config can stay the startup contract, but live truth after promotion/rejoin must come from runtime authority status.
   Otherwise `/membership` and error payloads will keep saying `primary epoch 0` on a fenced old primary.

6. **Keep the first-wave post-failover scope narrow.**
   Do not use S02 to open active-active or arbitrary post-failover ingress. The honest first-wave bar is:
   - promoted standby can recover surviving keyed work
   - old primary cannot resume authority or overwrite promoted truth
   - status surfaces make the role/epoch boundary explicit

7. **Follow the loaded Rust skill rules on errors and tests.**
   - From `rust-best-practices` chapter 4: new runtime APIs should return `Result`, not panic-driven control flow.
   - From chapter 5: keep continuity tests descriptive and single-behavior; the runtime unit suite is the right place for merge/fence edge cases, and `e2e_m043_s02` should keep one scenario per failover phase.

## Implementation Landscape

### Key Files

- `compiler/mesh-rt/src/dist/continuity.rs`
  - Owns `ContinuityAuthorityConfig`, `ContinuityRecord`, merge precedence, submit/completion transitions, sync payload encode/decode, and the exported C ABI.
  - Critical blockers live here:
    - `current_authority_config()` is immutable process state.
    - `project_remote_record(...)` overwrites incoming role/epoch before merge.
    - `ContinuityRecord::validate()` rejects `Standby + epoch > 0`.

- `compiler/mesh-rt/src/dist/node.rs`
  - Owns continuity wire handling and the recovery-eligibility hook.
  - Important seams:
    - `DIST_CONTINUITY_UPSERT` / `DIST_CONTINUITY_SYNC` receive path
    - `continuity_owner_loss_recovery_eligible(...)`
    - connect-time `send_continuity_sync(...)`
    - disconnect-time owner-loss/degraded transitions

- `compiler/mesh-rt/src/lib.rs`
  - Re-exports the runtime continuity C ABI. Will need updates if a new promote/status intrinsic is added.

- `compiler/mesh-typeck/src/infer.rs`
  - Hard-codes the `Continuity` module surface. Right now it only exposes:
    - `submit`
    - `status`
    - `mark_completed`
    - `acknowledge_replica`

- `compiler/mesh-codegen/src/mir/lower.rs`
  - Maps `Continuity.*` calls to runtime intrinsics.

- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
  - Declares `mesh_continuity_*` runtime intrinsics. Any Mesh-visible promotion or authority-status API must be added here too.

- `cluster-proof/main.mpl`
  - `/membership` currently renders role/epoch/health from config/env, not runtime state.
  - Also the natural home for an explicit promotion operator surface if S02 exposes one over HTTP.

- `cluster-proof/work_continuity.mpl`
  - Thin consumer over `Continuity.*`.
  - Natural app-side seams:
    - `current_continuity_*()` currently read config/env
    - `submit_required_replica_count(...)` already knows about `owner_lost`
    - `handle_work_submit(...)` / `created_submit_response(...)` control whether work is dispatched

- `cluster-proof/config.mpl`
  - Startup topology validation only. Today it explicitly rejects `standby + epoch > 0` in cluster mode.
  - That is still fine as a startup rule, but it cannot remain the source of *live* authority truth after promotion.

- `cluster-proof/cluster.mpl`
  - Placement is pure membership hashing. It does not know about authority.
  - Important planner note: **do not try to solve stale-primary fencing in placement.** Placement will happily choose a rejoined node if membership allows it.

- `cluster-proof/tests/config.test.mpl`
  - Encodes the current startup contract, including the `standby role requires promotion epoch 0 before promotion` rule.

- `compiler/meshc/tests/e2e_m042_s03.rs`
  - Best prior-art harness for:
    - kill/restart one node
    - retry-rollover after owner loss
    - same-identity rejoin preserving newer attempt truth

- `compiler/meshc/tests/e2e_m043_s01.rs`
  - Best prior-art harness for:
    - role/epoch/replication-health assertions
    - primary/standby node config
    - retained artifacts under `.tmp/m043-s01/...`

- `scripts/lib/m043_cluster_proof.sh`
  - Reusable JSON/status assertion library for role/epoch/health-aware payloads.

- `scripts/verify-m043-s01.sh`
  - The current M043 fail-closed verifier pattern. S02 should extend this pattern, not replace it.

### Natural Seams

#### 1. Runtime authority and fencing core
Primary files:
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/dist/node.rs`

This task owns:
- mutable authority state
- promotion transition
- raw-epoch-aware merge/fence behavior
- rejoin demotion/fencing logic
- promotion of mirrored pending records into a recovery-eligible state

This is the riskiest seam and should be built first.

#### 2. Compiler/runtime API seam
Primary files:
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-rt/src/lib.rs`

This task exists **only if** promotion/authority status must be callable from Mesh code.

Based on the current code, that is the likely honest path, because `cluster-proof` needs an explicit operator action and a runtime-backed status source. There is no existing Mesh-visible API for either.

#### 3. Proof app/operator surface
Primary files:
- `cluster-proof/main.mpl`
- `cluster-proof/work_continuity.mpl`
- possibly `cluster-proof/config.mpl`
- possibly `cluster-proof/work.mpl`
- package tests in `cluster-proof/tests/*.mpl`

This task should stay thin:
- one explicit promotion action
- runtime-backed membership/status truth
- reuse existing submit/retry dispatch path where possible
- reject or avoid stale-primary authority paths rather than inventing app-side DR orchestration

#### 4. E2E and verifier rail
Primary files:
- new `compiler/meshc/tests/e2e_m043_s02.rs`
- `scripts/lib/m043_cluster_proof.sh`
- new `scripts/verify-m043-s02.sh`

This task should come last, once the runtime and proof app contract are stable.

### What to Build First

1. **Fix the authority model in `mesh-rt`.**
   - mutable authority state
   - raw-epoch-aware merge logic
   - demotion/fencing on higher remote epoch
   - promoted recovery eligibility for pending mirrored records

2. **Add the runtime API that `cluster-proof` needs.**
   Likely:
   - explicit promote call
   - authority status call

3. **Swap `cluster-proof` surfaces from config/env truth to runtime truth.**
   - `/membership`
   - error/missing status payloads
   - promotion endpoint or equivalent operator action

4. **Then add the S02 destructive proof.**
   - kill primary before work completes
   - promote standby
   - recover/complete surviving keyed work on standby
   - rejoin old primary
   - prove old primary stays fenced/deposed

### Verification Approach

Use the same layered pattern as M042/S03 and M043/S01.

#### Runtime authority
- `cargo test -p mesh-rt continuity -- --nocapture`

Add targeted unit tests for:
- higher-epoch incoming record demotes local stale authority before merge
- lower-epoch incoming record is ignored before it can overwrite promoted truth
- promoted pending mirrored record becomes recovery-eligible
- stale completion/upsert from old primary cannot beat promoted truth
- authority-status transitions are observable and non-panicking

#### Compiler / proof-app smoke
- `cargo run -q -p meshc -- test cluster-proof/tests`
- `cargo run -q -p meshc -- build cluster-proof`

If new `Continuity.*` APIs are added, package tests should prove:
- promotion action result shape
- runtime-backed authority-status parsing
- old primary/deposed status truth on payload helpers

#### Destructive local authority
- `cargo test -p meshc --test e2e_m043_s02 -- --nocapture`
- `bash scripts/verify-m043-s02.sh`

The e2e harness should fail closed on named test counts the same way S01 and M042 do.

Recommended scenario split:
- promoted standby recovers pending keyed work after primary loss
- rejoined old primary reports promoted truth and does not resume authority

#### Regression replay worth keeping nearby
Because S02 touches M042 recovery semantics and S01 role/epoch projection, the slice verifier should likely replay:
- `bash scripts/verify-m043-s01.sh`

If runtime merge logic becomes invasive, also consider a targeted M042 regression filter:
- `cargo test -p meshc --test e2e_m042_s03 continuity_api_same_identity_rejoin_preserves_newer_attempt_truth -- --nocapture`

## Don't Hand-Roll

| Problem | Existing seam to reuse | Why it matters |
|---|---|---|
| Same-key recovery after authority loss | `continuity_owner_loss_recovery_eligible(...)` + `transition_retry_rollover_record(...)` + `submit_required_replica_count(...)` | S02 can likely reuse the existing retry-rollover contract instead of inventing a second failover state machine. |
| Continuity transport | `DIST_CONTINUITY_UPSERT`, `DIST_CONTINUITY_SYNC`, `send_continuity_sync(...)` | Promotion/fence truth should ride the existing runtime sync rail unless there is a very specific reason to add a new message type. |
| Role/epoch/health JSON assertions | `scripts/lib/m043_cluster_proof.sh` | The helper already knows how to assert M043 authority metadata. Extend it. |
| Kill/rejoin artifact harness | `compiler/meshc/tests/e2e_m042_s03.rs` | It already solves the hard parts of process lifecycle, retained logs, and same-identity rejoin proof. |
| Primary/standby role test config | `compiler/meshc/tests/e2e_m043_s01.rs` | It already threads explicit `cluster_role` and `promotion_epoch` into spawned nodes. |

## Constraints

- `continuity_registry()` is in-memory process state. There is no persistence seam to survive restart-based promotion.
- `project_remote_record(...)` currently rewrites incoming role/epoch to local authority before merge precedence. That behavior is the current stale-primary fencing blocker.
- `cluster-proof` membership truth is startup-config-derived today; dynamic failover cannot stay honest without a runtime authority-status source.
- `cluster-proof` placement is pure membership hashing. It will not fence old primaries by itself.
- The proof app still has to stay thin per D170. Promotion-state enumeration or recovery orchestration should not move into Mesh-side ad hoc registries.
- Public docs/runbooks are intentionally deferred to S04. S02 should update local proof rails and internal contract surfaces, but not spend time on broad public wording unless the verifier needs an internal note.

## Common Pitfalls

- **Restart-based promotion** — loses mirrored state and fails R051.
- **Keeping `project_remote_record(...)` as-is** — promoted epochs will be erased on merge.
- **Trying to solve fencing in `canonical_placement(...)`** — placement is topology, not authority.
- **Leaving `/membership` on config/env truth** — the old primary will still claim its startup role after rejoin.
- **Inventing a new Mesh-side recovery loop** — the runtime already has an owner-loss retry seam; use it.
- **Over-expanding failover scope** — rejecting or fencing stale-primary progress is enough for S02; do not slip into active-active or general state replication.
- **Forgetting fail-closed verifier behavior** — S02 should preserve the named-test-count and retained-artifact discipline from M042/M043.

## Open Risks

- **Authority-state representation is still a design choice.**
  The smallest plausible model is `primary(epoch=N)` vs `standby(epoch=N)` where `standby(epoch>0)` means deposed/follower-after-promotion. That is smaller than a new `deposed` enum, but the runtime and proof surfaces must make the meaning explicit.

- **Promotion-triggered recovery semantics need one clear contract.**
  The most codebase-aligned path is: promotion converts mirrored pending records into recovery-eligible state, then the promoted standby completes them via the existing keyed retry flow. If the slice instead wants automatic local completion without retry, the runtime/app seam changes materially because the runtime does not own payload execution.

- **Post-failover new-ingress behavior is ambiguous.**
  S02 only needs to fence stale-primary authority. Whether a deposed node rejects all new submits or forwards them is a separate operator contract question and should stay narrow in this slice.

- **Startup config vs live runtime status will diverge after promotion.**
  That is correct behavior, but later S04 docs must explain it clearly so R053 stays truthful.

## Skills Discovered

| Technology | Skill | Status |
|---|---|---|
| Rust runtime/compiler seams | `rust-best-practices` | available and used |
| Distributed failover / fencing patterns | `distributed-systems` | installed during this research (`yonatangross/orchestkit@distributed-systems`) |
