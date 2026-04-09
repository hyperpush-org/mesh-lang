# S04 Research — Bounded Automatic Promotion

## Scope and owning requirements

**Primary requirements:**
- **R067** — automatic promotion is auto-only, bounded, epoch/fencing-based, and fail-closed on ambiguity.
- **R068** — declared clustered handler work survives primary loss through bounded automatic promotion when the runtime can prove the transition is safe.

This slice is the real failover seam for M044. S02 already moved declared work submission into the runtime, and S03 already added read-only operator inspection. What is still missing is:
1. a runtime-owned **automatic** authority transition,
2. a truthful way to continue declared work after primary loss,
3. removal of the old **manual** proof surface from the public clustered-app story.

## Skills discovered

- **Loaded:** `debug-like-expert`
  - Relevant rule applied here: **verify, don’t assume**. I traced the actual node-disconnect, continuity, and proof-app paths instead of inferring behavior from M043 summaries.
  - Relevant rule applied here: **read before proposing fixes**. The main findings below come from the concrete runtime disconnect path, the continuity state machine, and the same-image failover verifier.
- **No additional skill install performed.** The core work is repo-native Rust/Mesh runtime/compiler code plus the existing installed debugging skill.

## Summary

S04 is not “just call promote automatically.” The runtime already does three useful things on node loss:
- marks pending primary-owned mirrored records as `owner_lost`,
- marks primary-owned records that lost only the replica as `degraded_continuing`,
- degrades standby-side mirrored truth when the replication source disappears.

What it **does not** do yet:
- automatically advance authority,
- automatically re-dispatch declared work,
- remove the manual `/promote` / `Continuity.promote()` product seam.

The most important structural gap is this:

> **The continuity record does not currently retain which declared handler should be resumed.**

`mesh_continuity_submit_declared_work(...)` eventually calls `node::submit_declared_work(runtime_name, request_key, payload_hash, required_replica_count)`, but the stored `ContinuityRecord` only contains request/placement/authority fields. After owner loss, the runtime can tell that a request is recoverable, but it does **not** currently have the declared handler runtime name needed to re-dispatch it automatically.

That means a true S04 implementation probably needs **both**:
1. a bounded auto-promotion decision in the runtime, and
2. a runtime-owned recovery/resume metadata seam for declared work.

## What exists now

### 1. Continuity state machine already has the right record vocabulary

**File:** `compiler/mesh-rt/src/dist/continuity.rs`

Important existing pieces:
- `ContinuityRecord` carries `owner_node`, `replica_node`, `replica_status`, `cluster_role`, `promotion_epoch`, `replication_health`, `execution_node`, `error`.
- `ReplicaStatus` already models the relevant failover states:
  - `Mirrored`
  - `OwnerLost`
  - `DegradedContinuing`
  - `Rejected`
- `ContinuityRegistry::promote_authority()` already exists, but it is **manual** and very weakly bounded:
  - rejects unless current authority is standby,
  - rejects unless some mirrored state exists,
  - otherwise promotes immediately.
- `project_record_for_authority_change(...)` already knows how to rewrite pending mirrored state under promotion:
  - pending mirrored records become `owner_lost` + `unavailable` on promotion to primary,
  - higher-epoch remote truth fences stale-primary rejoin back to standby.
- `continuity_owner_loss_recovery_eligible(...)` in `node.rs` plus `ContinuityRegistry::submit(...)` already support retry rollover once authority has changed and the request is submitted again.

Useful existing tests in this file:
- `continuity_promotion_rejects_standby_without_mirrored_state`
- `continuity_promotion_marks_mirrored_pending_record_owner_lost_and_reuses_retry_rollover`
- `continuity_merge_higher_epoch_truth_fences_same_identity_rejoin`
- `continuity_disconnect_marks_owner_lost_records_recoverable`
- `continuity_submit_recovery_retry_uses_owner_lost_state_on_ordinary_submit_path`

### 2. Node disconnect already drives the continuity degradation signals

**File:** `compiler/mesh-rt/src/dist/node.rs`

`handle_node_disconnect(...)` already does the continuity-side failover prep:
- `mark_owner_loss_records_for_node_loss(node_name)`
- `degrade_replica_records_for_node_loss(node_name)`
- `degrade_replication_health_for_node_loss(node_name)`

This is the natural hook for S04. It is the place where the runtime already learns “a cluster member disappeared.”

Also important in this file:
- `submit_declared_work(...)` computes declared-work placement, pulls current authority, submits continuity state, then dispatches locally or remotely.
- `declared_work_arg_payload(...)` only passes `request_key` and `attempt_id` into the declared work wrapper.

That last point matters: **automatic resume does not need the original payload body**, but it still needs the **declared runtime handler name**.

### 3. Operator diagnostics already expose the pre-promotion fault truth

**Files:**
- `compiler/mesh-rt/src/dist/operator.rs`
- `compiler/meshc/tests/e2e_m044_s03.rs`

S03 already established the read-only observability seam:
- transient operator query transport,
- `meshc cluster status|continuity|diagnostics`,
- diagnostics buffer with transitions like `owner_lost`, `degraded`, `prepare_timeout`.

`m044_s03_operator_continuity_and_diagnostics_report_runtime_truth()` already proves that killing the right node yields a visible `owner_lost` or `degraded` diagnostic entry without mutating membership truth.

That makes operator diagnostics the obvious observability surface for S04. If promotion becomes automatic, add/verify new runtime diagnostic transitions there instead of inventing proof-only logging.

### 4. The public proof surface is still explicitly manual

**Files:**
- `cluster-proof/main.mpl`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/README.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `README.md`

Current truth:
- `cluster-proof/main.mpl` still registers `POST /promote`.
- `cluster-proof/work_continuity.mpl` still exposes `handle_promote()` and calls `Continuity.promote()`.
- `cluster-proof/README.md` explicitly says:
  - `POST /promote` is the authority boundary,
  - no automatic promotion is claimed or verified.
- `website/docs/docs/distributed-proof/index.md` still teaches the explicit `/promote` boundary.

So even if the runtime gains automatic promotion, the public proof/app/docs surface will still be lying until those files change.

### 5. `Continuity.promote()` is still a real public language/runtime surface

**Files:**
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-typeck/src/builtins.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/lib.rs`
- `compiler/meshc/tests/e2e_m044_s01.rs`

This is not just a proof-app helper. `Continuity.promote()` is wired through:
- typechecker module surface,
- builtin mapping,
- MIR lowering,
- intrinsic declaration,
- runtime export `mesh_continuity_promote()`.

If S04 means **strictly auto-only**, the planner must decide whether to:
- **A.** remove/deprecate the public API itself, or
- **B.** keep it as a low-level/internal seam but remove it from the public clustered-app/product story.

Choice A is broader but cleaner against R067.
Choice B is narrower but leaves a product-contract leak.

## Critical gap: automatic resumption metadata

### The gap

`submit_declared_work(...)` needs `runtime_name` to look up the declared handler entry and dispatch it.

But `ContinuityRecord` stores:
- request key
- attempt id
- owner/replica/authority truth
- error / status

It does **not** store:
- declared runtime handler name,
- executable symbol,
- any recovery-dispatch metadata.

### Why it matters

With the current code, automatic promotion alone only gets you to:
- standby authority becomes primary,
- old request becomes `owner_lost`,
- stale primary can be fenced later.

It does **not** automatically execute the work.

Today, work only continues because the M043 proof explicitly does a same-key retry after manual promotion, and the retry path re-enters `submit_declared_work(...)` with the handler runtime name still available at the call site.

### Likely implementation consequence

S04 probably needs one new runtime-owned recovery seam, for example:
- extend the stored continuity record/recovery entry with the declared runtime name, or
- keep a side registry keyed by request key / attempt id that records the declared runtime target for recoverable requests.

Because `declared_work_arg_payload(...)` only reconstructs `(request_key, attempt_id)`, no original request payload body appears necessary for the cluster-proof style declared work.

## Natural seams for planning

### Seam 1 — bounded auto-promotion decision

**Primary files:**
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/dist/continuity.rs`

Likely work:
- define the runtime’s bounded safety rule,
- trigger evaluation from the existing disconnect path,
- advance authority only when safety is provable,
- emit operator diagnostics/logging for both promotion and fail-closed refusal.

Important constraint:
- the safety rule should live at the runtime boundary that knows about **node membership/session state**, not only inside the continuity registry. The registry knows record truth; `node.rs` knows live peers.

### Seam 2 — automatic declared-work recovery/resume

**Primary files:**
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/dist/continuity.rs`
- possibly `compiler/mesh-codegen/src/declared.rs` or registration plumbing if new metadata must be recorded

Likely work:
- store the runtime dispatch identity needed to resume declared work,
- on safe promotion, re-dispatch eligible owner-lost declared work without an external `/promote` or client retry,
- preserve attempt fencing / same-key idempotency semantics.

This is the riskiest implementation seam.

### Seam 3 — public/manual failover surface cleanup

**Primary files:**
- `cluster-proof/main.mpl`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/README.md`
- `README.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`

Likely work:
- remove or stop advertising `POST /promote`,
- rewrite proof-app wording from manual boundary to bounded auto-promotion,
- keep status/membership/operator inspection truthful.

### Seam 4 — compiler/public API cleanup if strict auto-only is required

**Primary files:**
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-typeck/src/builtins.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/lib.rs`
- `compiler/meshc/tests/e2e_m044_s01.rs`

Only needed if the slice removes `Continuity.promote()` from the public surface instead of merely ceasing to use it.

## Recommended build order

1. **Decide the bounded safety rule first.**
   - Without that, the rest is guesswork.
   - The planner should force an explicit rule for when auto-promotion is allowed and when it fail-closes.

2. **Close the recovery metadata gap second.**
   - If the runtime cannot identify which declared handler to resume, true automatic continuation cannot happen.

3. **Implement runtime auto-promotion + auto-resume together.**
   - Promotion without resume is not enough for R068.
   - Resume without strict promotion rules violates R067.

4. **Only then rewrite cluster-proof and docs/verifiers.**
   - The old same-image proof is a good base, but it currently proves manual promotion.

## Verification landscape

### Best existing rail to reuse

**Primary source:** `compiler/meshc/tests/e2e_m043_s03.rs`

The existing same-image Docker test already proves:
- two-node startup,
- pending mirrored record before failure,
- primary kill,
- standby degraded pre-promotion truth,
- promoted authority truth,
- recovery rollover to a new attempt,
- completion on new primary,
- fenced old-primary rejoin,
- stale-primary same-key guard.

### What must change for S04

The S04 authoritative proof should replace the manual parts:
- no `POST /promote`,
- no proof step that manually calls `Continuity.promote()`,
- no docs/verifier claim that promotion is operator-triggered.

A good S04 proof shape is:
1. start primary + standby,
2. submit declared work that becomes mirrored,
3. kill primary while request is pending,
4. standby exposes degraded/owner-lost truth first,
5. runtime auto-promotes when safe,
6. declared work is resumed/completed on promoted standby without a manual promotion call,
7. old primary rejoins fenced as standby with newer epoch,
8. stale-primary same-key request does not resume execution locally.

### Best observability surfaces

Reuse these rather than inventing new ones:
- operator diagnostics (`meshc cluster diagnostics --json`)
- continuity status (`meshc cluster continuity ... --json` and `/work/:request_key` if still kept)
- runtime stderr transitions in `mesh-rt continuity`
- retained artifact bundles under `.tmp/m044-s04/verify/`

### Likely new rails

- `compiler/meshc/tests/e2e_m044_s04.rs`
- `scripts/verify-m044-s04.sh`

Recommended verifier pattern:
- replay required prerequisites (at least S03; likely S02/S03 depending on chosen proof bundle),
- require named test filters to run real tests (`running N test` guard),
- retain copied same-image artifact bundle and assert exact JSON/log truth,
- fail closed on missing diagnostic/promotion/resume evidence.

## Risks and decisions the planner must surface explicitly

### 1. What exactly counts as “safe” auto-promotion?

The current runtime only knows:
- its own authority role/epoch,
- local continuity records,
- live peer sessions.

It does **not** currently expose an explicit quorum/consensus layer. The bounded rule therefore needs to stay narrow and probably tied to the supported one-primary/one-standby topology.

### 2. Is a same-key client retry still acceptable after auto-promotion?

Current code supports recovery rollover on retry.

Requirement text and roadmap wording lean toward **no**: the work should survive primary loss, not merely become retryable after automatic authority change.

If the planner tries to satisfy S04 with “auto-promote, then let the user retry,” it should treat that as likely insufficient for R068 unless the requirement is explicitly re-scoped.

### 3. Does S04 remove `Continuity.promote()` or only stop using it publicly?

This is a real scope fork. The codebase currently treats it as a first-class public API, and the public docs still teach it.

## Concrete file map for the planner

### Runtime
- `compiler/mesh-rt/src/dist/continuity.rs` — authority transitions, owner-loss/degraded transitions, runtime intrinsics, diagnostics/logging
- `compiler/mesh-rt/src/dist/node.rs` — disconnect hook, declared-handler registry, declared-work placement/dispatch, recovery eligibility
- `compiler/mesh-rt/src/dist/operator.rs` — diagnostics ring buffer and transient query surfaces
- `compiler/mesh-rt/src/lib.rs` — public runtime export surface

### Compiler/public API (only if manual API removal is in scope)
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-typeck/src/builtins.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`

### Proof app / docs
- `cluster-proof/main.mpl`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/README.md`
- `README.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`

### Verification
- `compiler/meshc/tests/e2e_m043_s03.rs` — strongest reuse candidate
- `compiler/meshc/tests/e2e_m044_s03.rs` — operator diagnostics under live fault
- `scripts/verify-m043-s03.sh` — existing same-image artifact verifier pattern
- `scripts/verify-m044-s03.sh` — current M044 assembled verifier pattern

## Recommendation

Plan S04 as **three executable tasks**, in this order:

1. **Runtime bounded auto-promotion decision + diagnostics**
   - add the safety rule where node-loss and continuity truth meet,
   - emit explicit diagnostic transitions for promote vs fail-closed refusal.

2. **Runtime automatic declared-work recovery**
   - add the missing recovery metadata seam for declared handlers,
   - prove resumed execution without manual `/promote`.

3. **Proof-surface rewrite and assembled verifier**
   - remove manual promotion from cluster-proof’s public route/docs story,
   - add `e2e_m044_s04` + `verify-m044-s04.sh`,
   - preserve retained artifact bundles and exact stale-primary fencing checks.

If the planner tries to combine all of this into one task, the risk is high. The runtime decision logic and the recovery metadata seam are separable and should be isolated first.

## Resume notes

- The key unanswered implementation decision is whether S04 removes the public `Continuity.promote()` API or only stops using/documenting it.
- The hardest technical seam is automatic resumption metadata. Promotion alone is already mostly present; automatic continuation is not.
- The best proof baseline is still the M043 same-image test/script pair. Reuse their artifact assertions and replace only the manual-promotion-specific expectations.