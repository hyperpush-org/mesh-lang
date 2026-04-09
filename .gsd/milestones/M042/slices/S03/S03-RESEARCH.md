# S03 Research — Owner-loss recovery, same-key retry, and stale-completion safety

## Executive Summary

S03 is a runtime-state-machine slice, not a `cluster-proof` redesign.

The existing runtime already has the raw pieces the slice needs:
- replicated continuity records and sync-on-connect in `compiler/mesh-rt/src/dist/continuity.rs`
- node disconnect/reconnect hooks in `compiler/mesh-rt/src/dist/node.rs`
- an attempt-scoped completion gate (`mark_completed` rejects mismatched `attempt_id`)
- a thin `cluster-proof` submit/status/complete consumer in `cluster-proof/work.mpl`
- destructive two-node lifecycle proof patterns in `compiler/meshc/tests/e2e_m039_s03.rs`

But three gaps are still real:
1. **Same-key retry cannot recover today.** `ContinuityRegistry::submit_with_replica_prepare(...)` always returns `Duplicate` for same-key same-payload records, even when the stored owner is gone and the request should roll to a new `attempt_id`.
2. **Owner loss is not modeled today.** `handle_node_disconnect(...)` only downgrades records when the lost node was the `replica_node`; it does nothing when the lost node was the active `owner_node`.
3. **Stale completion safety will fail under replication unless merge precedence changes.** `preferred_record(...)` currently prefers any incoming terminal record over an existing non-terminal record **before** comparing attempt tokens. That means an old `attempt-0 completed` upsert could overwrite a newer `attempt-1 submitted` retry state on another node.

The smallest honest S03 path is:
- keep the Mesh-facing `Continuity.submit/status/mark_completed` API stable
- teach runtime submit to roll a new attempt for eligible same-key retries after owner loss
- fix merge precedence so newer attempts fence off older completions everywhere
- prove owner-loss status serving, retry rollover, stale completion rejection, and rejoin with a new dedicated S03 e2e/verifier rail

## Skills Discovered

Directly relevant skills:
- `rust-best-practices` (loaded): useful guidance from Chapter 1/4/5 applies here — keep the state machine explicit and testable, prefer narrow `Result`-returning transitions over panic-like control flow, and add focused unit/integration tests per behavior.
- `distributed-systems` (installed globally during this unit): the relevant rule is the fencing-token pattern. For S03, `attempt_id` is effectively the fencing token: once a retry rolls to a newer attempt, older completions must lose authority everywhere, including cross-node merge.

## Requirement Focus

This slice is directly advancing:
- **R049** — same-key retry must converge through a rolled `attempt_id` and reject stale completions instead of replaying the old pending attempt forever.
- **R050** — surviving replicated state must keep serving truthful status after owner loss, and recovery/rejoin must stay visible through the runtime-owned status rail.

Secondary support:
- the slice keeps the proof/app boundary thin, which supports the broader M042 direction without forcing a new Mesh-facing API shape.

## Implementation Landscape

### `compiler/mesh-rt/src/dist/continuity.rs`
Primary S03 runtime state machine.

What it already owns:
- `ContinuityRecord` shape: `request_key`, `payload_hash`, `attempt_id`, `phase`, `result`, `owner_node`, `replica_node`, `replica_status`, `execution_node`, `error`, routing booleans.
- `SubmitRequest` and `SubmitDecision`.
- request-key dedupe/conflict in `submit_with_replica_prepare(...)`.
- completion fencing in `mark_completed(...)` via `transition_completed_record(...)`.
- replica prepare/ack/reject transitions.
- disconnect downgrade for lost replicas in `degrade_replica_records_for_node_loss(...)`.
- cross-node merge/sync in `merge_remote_record(...)`, `merge_snapshot(...)`, `broadcast_continuity_upsert(...)`, `send_continuity_sync(...)`.
- Mesh ABI exports: `mesh_continuity_submit_with_durability`, `mesh_continuity_status`, `mesh_continuity_mark_completed`, `mesh_continuity_acknowledge_replica`.

What is missing for S03:
- no recovery transition that rewrites an existing pending record to a new `attempt_id`
- no owner-loss-specific transition or eligibility check
- no unit test covering “newer submitted attempt beats stale older completed attempt”
- no unit test covering “same-key retry after owner loss becomes a new attempt instead of duplicate”

Critical functions to inspect/change:
- `submit_with_replica_prepare(...)`
- `preferred_record(...)`
- `update_next_attempt_token(...)`
- `transition_completed_record(...)`
- likely a new helper alongside `transition_*` functions for retry rollover / owner-loss promotion

### `compiler/mesh-rt/src/dist/node.rs`
Node lifecycle and continuity wire-integration.

What it already owns:
- session registration and sync-on-connect (`register_session(...)`, `send_continuity_sync(...)` at both connect call sites)
- continuity prepare/ack wire protocol (`DIST_CONTINUITY_PREPARE`, `DIST_CONTINUITY_PREPARE_ACK`)
- disconnect hook `handle_node_disconnect(...)`
- delivery of `:nodeup` / `:nodedown`

Current S03 constraint:
- `handle_node_disconnect(...)` currently calls `continuity_registry().degrade_replica_records_for_node_loss(node_name)` and nothing else continuity-specific. That only handles “owner survives, replica died”. It does **not** handle “replica survives, owner died”.

Natural S03 seam:
- keep node-liveness knowledge in `node.rs`
- keep record rewrite logic in `continuity.rs`
- follow the existing injected-callback pattern from `submit_with_replica_prepare(...)` instead of hard-wiring node/session lookups deep into the state machine if possible

### `cluster-proof/work.mpl`
Thin consumer surface. It is already close to the right boundary.

Relevant parts:
- `continuity_submit(...)`, `continuity_status_record(...)`, `continuity_mark_completed(...)` are just runtime calls + JSON parsing.
- `submit_from_selection(...)` uses current placement and maps `created` / `duplicate` / `rejected` / `conflict` into HTTP.
- `execute_work(...)` logs start, sleeps optionally, then calls `Continuity.mark_completed(...)`; failures are logged as `[cluster-proof] keyed completion failed ... reason=...`.

Why this matters:
- if runtime reuses outcome=`created` for a same-key recovery retry, `cluster-proof` likely needs no major API change — it will dispatch the new attempt naturally.
- stale completion rejection already has a live observable surface via `log_completion_failure(...)`; if a late old attempt completes after rollover, the expected live reason is `attempt_id_mismatch`.

Likely S03 app impact:
- small or none unless the runtime introduces a new outcome/status string
- keep `cluster-proof` thin; do not reintroduce owner/replica orchestration here

### `compiler/meshc/tests/e2e_m042_s02.rs`
Best source for current continuity API harness helpers.

Useful existing helpers/patterns:
- spawn/stop process harness for `cluster-proof`
- `wait_for_membership(...)`
- `wait_for_status_condition(...)`
- `wait_for_completed_status(...)`
- `find_submit_matching_placement(...)` for forcing a desired owner/replica pair by iterating request keys
- artifact archiving of raw HTTP + parsed JSON

Important S03 implication:
- this file is the right base for a new `e2e_m042_s03.rs`
- `find_submit_matching_placement(...)` lets S03 stay on the **stable local-owner rail**: submit on node A until owner=node A and replica=node B, then kill A and continue on B. That avoids widening proof onto the unrelated remote `Node.spawn` fragility.

### `compiler/meshc/tests/e2e_m039_s03.rs`
Best existing destructive lifecycle pattern.

Useful existing pieces to copy/adapt:
- `kill_cluster_proof(...)`
- restart same node identity as a second run
- pre-loss / degraded / post-rejoin artifact sets
- fail-closed panic messages that include copied stdout/stderr and artifact paths

S03 should reuse this lifecycle structure rather than inventing a new one.

### `scripts/verify-m039-s03.sh` and `scripts/verify-m042-s02.sh`
Verification shape to copy.

What to reuse:
- replay stable prerequisites first
- run named e2e filters, not broad suites
- fail closed on missing `running N test` evidence
- copy proof artifacts into a dedicated `.tmp/.../verify/...` bundle

There is **no** `scripts/verify-m042-s03.sh` yet.

### Compiler API files (`compiler/mesh-typeck/src/infer.rs`, `compiler/mesh-codegen/src/mir/lower.rs`, `compiler/mesh-codegen/src/codegen/intrinsics.rs`, `compiler/mesh-rt/src/lib.rs`)
Current `Continuity` API surface is:
- `submit(String, String, String, String, String, Int, Bool, Bool) -> String ! String`
- `status(String) -> String ! String`
- `mark_completed(String, String, String) -> String ! String`
- `acknowledge_replica(String, String) -> String ! String`

Planner guidance:
- **prefer not to expand this surface for S03**
- the current API is sufficient if recovery retry remains ordinary `submit(...)` semantics with a new `attempt_id`
- only touch compiler/typeck/codegen if a genuinely new Mesh-facing operation becomes unavoidable

## Findings That Matter

### 1. Current duplicate handling blocks S03 outright
In `submit_with_replica_prepare(...)`, this branch is unconditional:
- existing same `request_key` + same `payload_hash` => `Duplicate`
- existing same `request_key` + different payload => `Conflict`

There is no exception for:
- existing record still pending
- current owner now dead
- caller now selected a different owner from current membership
- caller retrying from the surviving replica

So same-key retry after owner loss cannot currently roll forward. Without changing this branch, S03 cannot land.

### 2. Owner loss is currently invisible to the runtime state machine
`handle_node_disconnect(...)` only downgrades mirrored records where `record.replica_node == lost_node`.

For the S03 case — record mirrored on replica B, owner A dies — the surviving record on B still has:
- `owner_node = A`
- `replica_node = B`
- no owner-loss transition applied

That means status is readable from the replica copy, but the runtime has no explicit recovery state and no hook that enables retry rollover.

### 3. `attempt_id` already works as a fencing token locally
`transition_completed_record(...)` rejects mismatched `attempt_id` with `attempt_id_mismatch`.

That is already the right safety rule for stale completions. The missing part is globalizing it:
- after recovery retry rolls a new attempt, late old completions must fail locally **and** lose in replicated merges.

### 4. Merge precedence is wrong for stale-completion safety
`preferred_record(existing, incoming)` currently does:
1. terminal phase beats non-terminal phase
2. then compare parsed attempt tokens
3. then compare replica-status rank

That ordering is safe for S02 but unsafe for S03.

Failure shape:
- surviving node has `attempt-1 submitted`
- stale upsert arrives with `attempt-0 completed`
- current logic picks the stale terminal record before ever looking at attempt numbers

This is the main correctness bug to fix before any e2e proof is trustworthy.

### 5. Rejoin sync is already present
When a session registers, both connect paths call `send_continuity_sync(&session)`.

So S03 does **not** need a new rejoin transport mechanism. It needs the right record precedence and recovery transitions so rejoin cannot resurrect stale state.

### 6. The current status model is probably enough
`ContinuityRecord` already exposes:
- `attempt_id`
- `owner_node`
- `replica_node`
- `replica_status`
- `phase`
- `result`
- `execution_node`
- `error`

That is enough to make S03 observable **without** changing the Mesh-facing schema if the runtime updates those fields truthfully during retry/recovery.

Open design choice:
- the minimum change is to keep last mirrored pending truth readable after owner loss and only rewrite owner/attempt on same-key retry
- if pre-retry owner-loss must be explicit in status, S03 may need either a new `replica_status` value or an eager owner-loss rewrite; that is not present today

## Recommendation for Task Breakdown

### Task 1 — Fix the runtime state machine in `compiler/mesh-rt/src/dist/continuity.rs`
Build this first.

Minimum scope:
- add an owner-loss recovery path for same-key same-payload submit on an existing pending record
- roll `attempt_id` from `next_attempt_token`
- rewrite record fields for the new active attempt (at least owner/replica/routing/execution/error; exact field set should stay explicit and unit-tested)
- reorder `preferred_record(...)` so **newer attempt token fences older attempts before terminal/non-terminal precedence is considered**

Recommended shape:
- keep the pure record rewrite in `continuity.rs`
- inject node/liveness knowledge similarly to today’s replica-prepare callback, so the state machine stays unit-testable

Required new unit tests:
- same-key retry after owner loss returns a new `attempt_id` instead of `Duplicate`
- stale older `Completed` record does not overwrite newer pending retry record in `merge_remote_record(...)`
- late `mark_completed(old_attempt_id)` fails after retry rollover
- rejoin snapshot/upsert with older attempt does not regress the active attempt

### Task 2 — Add owner-loss detection / recovery plumbing in `compiler/mesh-rt/src/dist/node.rs`
Second, once Task 1 is shaped.

Likely responsibilities:
- decide when an existing record is eligible for recovery retry (old owner unavailable, surviving node is the mirrored replica / current selected owner)
- optionally add an owner-loss transition on disconnect if the slice wants status to become explicit before retry
- preserve current sync-on-connect behavior and ensure rejoin cannot clobber newer attempts

Planner note:
- avoid smearing node/session checks throughout the registry; keep node-specific liveness reasoning here

### Task 3 — Add the slice proof rail (`compiler/meshc/tests/e2e_m042_s03.rs` + `scripts/verify-m042-s03.sh`)
Third, after the runtime semantics exist.

Best harness composition:
- copy S02 continuity HTTP/membership/status helpers from `e2e_m042_s02.rs`
- copy kill/restart/rejoin helpers/patterns from `e2e_m039_s03.rs`

Recommended live scenarios:
1. **owner-loss status serving**
   - submit on node A with local owner A and replica B
   - confirm pending mirrored state on both
   - kill A
   - confirm B still serves truthful status from replicated state
2. **same-key retry rollover**
   - retry same key/same payload against B after A is gone
   - expect a **new** `attempt_id`
   - expect owner fields to move to the surviving execution path
   - expect old attempt not to stay active
3. **rejoin truth**
   - restart A with same identity
   - confirm membership converges
   - confirm latest attempt remains authoritative after sync/rejoin
4. **stale-completion safety**
   - prove with a runtime unit test at minimum
   - if live proof is practical, the cheapest observable surface is `cluster-proof`’s existing completion-failure log with `reason=attempt_id_mismatch`

### Task 4 — Only touch `cluster-proof/work.mpl` if runtime semantics force it
Probably last, probably small.

Likely needs:
- no API changes if runtime keeps using outcome=`created` for recovery retry
- maybe additional logging/assertion surfaces if the planner wants explicit stale-completion observability in the e2e logs

Avoid:
- putting owner-loss orchestration back in Mesh code
- adding app-authored replica/owner repair logic

## Verification Plan

Authoritative command stack for the finished slice should be:

1. Runtime unit tests
   - `cargo test -p mesh-rt continuity -- --nocapture`

2. Thin consumer helper tests
   - `cargo run -q -p meshc -- test cluster-proof/tests`

3. Slice e2e tests
   - `cargo test -p meshc --test e2e_m042_s03 -- --nocapture`

4. Slice verifier
   - `bash scripts/verify-m042-s03.sh`

Verifier expectations:
- replay S02 prerequisite proof first rather than assuming a clean tree
- fail closed if named test filters run `0 tests`
- archive copied proof bundles for pre-loss, owner-loss, retry, and rejoin phases
- preserve stdout/stderr for both nodes so owner-loss and stale-completion diagnostics are inspectable after failures

Useful evidence signals to assert explicitly:
- status JSON shows the rolled `attempt_id` after retry
- status JSON from the surviving node stays readable after owner loss
- after rejoin, status still reports the latest attempt/owner truth
- logs contain either an explicit recovery transition or, at minimum, stale completion rejection via `attempt_id_mismatch`

## Planning Notes

- **Do not start with compiler/typeck/codegen changes.** The current `Continuity` API can probably absorb S03 semantically without widening the language surface.
- **Do not start with docs/operator rails.** S03 is still runtime + proof-harness work.
- **Keep the local-owner rail.** S02 already established that this is the stable continuity path; S03 can prove owner loss by killing the local owner and continuing on the replica without reopening unrelated remote-spawn issues.
- **Treat `attempt_id` as the fencing token.** That is the cleanest way to make stale completion rejection honest and aligns with the distributed-systems skill guidance.

