# M042 S02 Research — Replica-backed admission and fail-closed durability truth

## Executive Summary

S02 is **not** a generic replication slice. The runtime continuity registry already exists, record replication already happens, and the status shape already has `mirrored`, `rejected`, and `degraded_continuing` vocabulary. The real missing boundary is stricter:

1. `mesh-rt` does **not** currently know whether a submit requires replica safety or not.
2. `mirrored` is still **manufactured by `cluster-proof`** via `Continuity.acknowledge_replica(...)`, not proven by a runtime-owned replica ack path.
3. continuity upserts are still **fire-and-forget session writes** with ignored errors, so there is no durable admission handshake.
4. node disconnect handling does **not** update continuity records, so `degraded_continuing` exists in the enum but is unused.
5. the current S01 verifier still fails on the known remote-owner crash in `compiler/mesh-rt/src/string.rs:104`, so S02 must prove its acceptance/rejection/degraded semantics **without depending on remote-owner completion**.

The clean slice boundary is:

- make submit admission policy explicit to the runtime,
- replace the proof-app’s optimistic replica ack with a runtime-owned prepare/ack or reject path,
- persist rejected records so `GET /work/:request_key` stays truthful,
- mark accepted records degraded when replica safety is later lost,
- verify all of that on **single-node cluster rejection** and **local-owner / remote-replica** two-node flows, not on the currently broken remote-owner execution path.

## Requirement Focus

This slice directly supports:

- **R050** — replica-backed continuity and two-node safety on a runtime-owned substrate
- **R053** — truthful proof surface / operator truth as the runtime boundary replaces app-owned replication logic

It also must preserve the S01 semantic seed for **R049**:

- stable `request_key` vs `attempt_id`
- duplicate dedupe vs same-key conflict rejection
- status truth via ordinary app code, not logs

## Skills Discovered

Installed during this research:

- `distributed-systems` (`yonatangross/orchestkit@distributed-systems`) via `npx skills add yonatangross/orchestkit@distributed-systems -g -y`

Directly relevant installed skills already present:

- `rust-best-practices`

Skill rules that matter here:

- **distributed-systems / Database-Backed Idempotency**: use a claim-first / first-writer-wins shape; do **not** check replica availability and then optimistically mark mirrored later. Rejected admissions need a stored record, not an ephemeral error.
- **distributed-systems / Dedup anti-pattern**: avoid check-then-act races. The durable decision should be recorded atomically before the request is considered accepted.
- **rust-best-practices Chapter 4**: use explicit `Result`/error returns for admission failure and disconnect handling; do not hide this behind panics or best-effort logs.
- **rust-best-practices Chapter 9**: be careful with pointer/FFI boundaries when adding new runtime intrinsics or message payload handling. The current remote-owner blocker is already a null-pointer dereference in string transport.

## Current Implementation Landscape

### 1. Runtime continuity registry is real, but admission truth is still incomplete

Primary file: `compiler/mesh-rt/src/dist/continuity.rs`

What is already there:

- `ContinuityRecord` with `phase`, `result`, `owner_node`, `replica_node`, `replica_status`, `execution_node`, `error`
- `SubmitOutcome::{Created, Duplicate, Conflict}` and `SubmitDecision`
- `ReplicaStatus::{Unassigned, Preparing, Mirrored, Rejected, DegradedContinuing}`
- local transitions for:
  - `submit(...)`
  - `mark_completed(...)`
  - `mirror_prepare(...)`
  - `acknowledge_replica_prepare(...)`
  - `reject_durable_request(...)`
- replication wire formats:
  - `DIST_CONTINUITY_UPSERT`
  - `DIST_CONTINUITY_SYNC`
- JSON FFI exports for Mesh-facing `Continuity.submit/status/mark_completed/acknowledge_replica`

What is missing for S02:

- `SubmitRequest` has **no durability requirement input**, so runtime submit cannot distinguish:
  - standalone `replica_node == ""` (allowed)
  - cluster replica-backed `replica_node == ""` (must reject)
- `submit(...)` currently only returns `created/duplicate/conflict`; there is no runtime-owned admission rejection outcome
- `mirror_prepare(...)` and `reject_durable_request(...)` exist, but **only tests call them** right now; the live runtime path never uses them
- `broadcast_continuity_upsert(...)` writes to every session and ignores write failures, so a record can become `mirrored` without any acked proof
- `preferred_record(...)` only has terminal + `Mirrored` precedence; it does not establish monotonic truth for `DegradedContinuing`

### 2. Node transport only supports one-way continuity sync/upsert today

Primary file: `compiler/mesh-rt/src/dist/node.rs`

Current continuity-related wire handling:

- `DIST_CONTINUITY_UPSERT` -> `continuity_registry().merge_remote_record(...)`
- `DIST_CONTINUITY_SYNC` -> `continuity_registry().merge_snapshot(...)`
- connect-time `send_continuity_sync(...)` on both acceptor and initiator registration paths

What is missing for S02:

- no targeted continuity prepare request
- no targeted continuity ack response
- no explicit reject reason round-trip
- no continuity update inside `handle_node_disconnect(...)`

That means S01’s “replication” is currently **eventual full-record fanout**, not an admission protocol.

### 3. Compiler/runtime API seam is small and bounded, but the current submit signature is too narrow

Files:

- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-rt/src/lib.rs`

Current Mesh-facing module surface:

- `Continuity.submit(request_key, payload_hash, ingress_node, owner_node, replica_node, routed_remotely, fell_back_locally)`
- `Continuity.status(request_key)`
- `Continuity.mark_completed(request_key, attempt_id, execution_node)`
- `Continuity.acknowledge_replica(request_key, attempt_id)`

Implication:

- S02 almost certainly needs **either** an extra submit argument (`required_replica_count` or durability mode) **or** a new runtime function name.
- Only `cluster-proof` currently uses this surface, so the blast radius is contained.

### 4. `cluster-proof` still owns the “mirrored” lie on the submit path

Primary file: `cluster-proof/work.mpl`

Current hot-path shape:

- `submit_from_selection(...)` calls runtime `Continuity.submit(...)`
- `created_submit_response(...)` immediately calls `acknowledged_replica_record(...)`
- `acknowledged_replica_record(...)` unconditionally calls `Continuity.acknowledge_replica(...)` whenever `replica_node != ""`
- only then does `cluster-proof` log `replica_status=mirrored` and dispatch work

This means `mirrored` is currently proof-app-authored optimism, not runtime-owned durability truth.

Important response behavior already in place:

- `GET /work/:request_key` uses `status_response_from_record(...)` and will already surface runtime `phase/result/replica_status` truthfully
- if `record.phase == "rejected"`, `GET` returns HTTP 200 with `ok: false`

Important response behavior missing for S02:

- `POST /work` only branches on `created / duplicate / conflict`
- there is **no submit-time branch for rejected durable admission**
- `duplicate_submit_response(...)` currently assumes duplicates are `ok: true`
- `created_submit_response(...)` always dispatches work for `created`

So S02 cannot just add a runtime reject transition and hope the app surface is fine. `cluster-proof/work.mpl` must stop assuming:

- all `created` records are runnable
- all `duplicate` records are successful

### 5. Cluster config already commits the public contract to replica-backed cluster mode

Files:

- `cluster-proof/config.mpl`
- `cluster-proof/main.mpl`
- `cluster-proof/docker-entrypoint.sh`

Already true today:

- standalone defaults to `local-only`
- cluster defaults to `replica-backed`
- cluster mode explicitly rejects `local-only`
- Docker entrypoint enforces the same contract before the app starts

That is good news for S02 planning:

- the operator contract already says cluster mode claims replica-backed durability
- S02 should make the runtime and status surface honor that existing claim rather than inventing a new config rail

## Key Constraints and Implications

### 1. Runtime submit needs an explicit durability claim input

Without policy input, runtime submit cannot tell whether empty `replica_node` means:

- standalone/no replica required, or
- cluster/replica required but unavailable

The smallest likely fix is to pass `required_replica_count(current_durability_policy())` into the runtime submit surface.

### 2. Rejected admissions should be stored, not dropped

Planner should assume this is the desired model:

- initial replica-unavailable submit creates a stored rejected record
- same-key same-payload retry replays that rejected record
- same-key different-payload retry still conflicts
- `GET /work/:request_key` shows the rejected record rather than `request_key_not_found`

That matches the idempotency skill guidance: record the durable decision, then replay truth; do not make callers infer state from a transient failure.

### 3. Degraded status belongs in the runtime disconnect seam, not in app polling

`ReplicaStatus::DegradedContinuing` already exists in `continuity.rs`, but nothing sets it.

The natural seam is `compiler/mesh-rt/src/dist/node.rs::handle_node_disconnect(...)`:

- surviving owners/ingress nodes already hear about disconnects there
- S02 can downgrade records that had replica safety and lost it
- S03 can later build owner-loss recovery on top of the same seam

### 4. The current `Mirrored` precedence is insufficient for S02 truth

`preferred_record(...)` currently prefers:

- terminal phase over non-terminal
- higher `attempt_id`
- `Mirrored` over non-`Mirrored`

That is not enough once `DegradedContinuing` becomes live. Planner should explicitly audit merge precedence so stale mirrored snapshots/upserts do not silently overwrite a degraded local truth.

### 5. Do not tie S02 verification to the remote-owner blocker

Current authoritative repro:

- `bash scripts/verify-m042-s01.sh`
- runtime continuity unit tests pass
- `cluster-proof/tests` pass
- standalone continuity e2e passes
- two-node remote-owner e2e still fails with pending mirrored status and owner crash at `compiler/mesh-rt/src/string.rs:104`

So S02 must avoid the broken path rather than inheriting it as a gate.

## Recommended Slice Decomposition

### Task 1 — Runtime-owned admission truth in `mesh-rt`

Primary files:

- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/dist/node.rs`

Recommended scope:

- extend submit request/decision to include durability requirement and explicit accepted vs rejected truth
- implement runtime-owned replica prepare/ack or equivalent targeted admission path
- persist rejected records with clear `error` reasons
- add transition(s) that set `replica_status = degraded_continuing` when replica safety is lost after admission
- update merge precedence / unit tests so degraded/rejected truth is monotonic enough for this slice

Natural sub-seams inside `continuity.rs`:

- `SubmitRequest` / `SubmitDecision`
- submit transition logic
- reject transition logic
- new degraded transition(s)
- JSON surface returned to Mesh
- unit tests at the bottom of the file

Natural sub-seams inside `node.rs`:

- new continuity message tags / handlers (if a targeted request/ack wire path is added)
- connect-time sync still stays, but is no longer the admission proof
- disconnect hook updating continuity state

### Task 2 — Compiler/runtime API plumbing

Primary files:

- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-rt/src/lib.rs`

Recommended scope:

- update the `Continuity.submit(...)` shape if S02 adds a durability argument
- optionally add/remove runtime functions if the slice stops exposing `acknowledge_replica(...)` as a proof-app seam
- keep the API fixed-arity and data-oriented; there is no reason to widen this into a more magical distributed-work API here

### Task 3 — Thin consumer corrections in `cluster-proof`

Primary file:

- `cluster-proof/work.mpl`

Recommended scope:

- stop manufacturing mirrored state with `acknowledged_replica_record(...)` on the submit path
- pass required replica count / durability claim into runtime submit
- add submit response mapping for rejected durable admission (`503`, `ok:false`, stored record payload)
- make duplicate replay truthful when the stored record is rejected, not only when it succeeded
- leave `GET /work/:request_key` mostly alone; it already maps rejected records truthfully

Secondary files only if needed:

- `cluster-proof/tests/work.test.mpl` for any new pure helper logic
- `cluster-proof/main.mpl` only if routing or readiness logging changes

### Task 4 — Slice-specific verifier and harness

Primary files:

- `compiler/meshc/tests/e2e_m042_s02.rs` (new)
- `scripts/verify-m042-s02.sh` (new)

Recommended scope:

- copy only the stable harness pieces from `compiler/meshc/tests/e2e_m042_s01.rs`
- do **not** reuse the full S01 wrapper; it still fail-closes on the unrelated remote-owner crash
- keep S02’s gate focused on mirrored / rejected / degraded admission truth

## Verification Strategy

### Runtime unit tests

Command:

- `cargo test -p mesh-rt continuity -- --nocapture`

Add/extend tests for:

1. submit with `required_replica_count = 1` and empty `replica_node` -> rejected record persisted
2. same-key same-payload retry after rejection -> duplicate replay of rejected record
3. same-key conflicting retry after rejection -> conflict unchanged
4. successful runtime prepare/ack path -> `replica_status = mirrored`
5. failed runtime prepare/ack path -> `phase = rejected`, `result = rejected`, explicit `error`
6. disconnect downgrade -> `replica_status = degraded_continuing`
7. merge precedence keeps degraded/rejected truth stable enough for this slice

### `cluster-proof` Mesh tests

Command:

- `cargo run -q -p meshc -- test cluster-proof/tests`

Keep this as the low-cost guard that route selection, config, and JSON parsing still work after submit signature / response mapping changes.

### New slice e2e tests

Recommended new file:

- `compiler/meshc/tests/e2e_m042_s02.rs`

Recommended cases:

#### 1. Single-node cluster rejects replica-backed submit and stores truthful status

Why this is the cleanest fail-closed proof:

- no remote execution involved
- no S01 string transport blocker involved
- directly proves cluster-mode replica-backed contract

Suggested flow:

- start one `cluster-proof` node in cluster mode (`CLUSTER_PROOF_COOKIE` + `MESH_DISCOVERY_SEED`) with no peer
- `POST /work` with a keyed request
- expect `503`
- body should show:
  - `phase = rejected`
  - `result = rejected`
  - `ok = false`
  - explicit error reason (implementation-specific, but stable enough to assert)
- `GET /work/:request_key` should return the same stored rejected truth, not 404
- duplicate same-key same-payload submit should replay rejected truth

#### 2. Two-node cluster accepts with mirrored truth on a local-owner / remote-replica path

Why local-owner matters:

- it avoids the unrelated remote-owner `string.rs` crash
- it still proves runtime admission and replica truth

Suggested flow:

- start two nodes
- choose a request key whose owner is the ingress/local node and replica is the peer
- set `CLUSTER_PROOF_WORK_DELAY_MS` high enough to observe pending state before completion
- `POST /work`
- assert immediate response has `replica_status = mirrored`
- assert status is readable on ingress/owner while work is still pending

A helper analogous to S01’s `wait_for_remote_owner_submit(...)` should search for a **local-owner with non-empty replica** request key instead.

#### 3. Two-node cluster surfaces `degraded_continuing` after replica loss

Suggested flow:

- reuse the local-owner / remote-replica arrangement above
- keep work delayed long enough to avoid racing completion
- after mirrored admission, kill the replica node
- poll `GET /work/:request_key` on the surviving owner/ingress node
- expect `replica_status = degraded_continuing`
- keep `phase/result` truthful for the still-running request

This gives S02 its operator-visible degraded truth without touching owner-loss recovery.

### New slice wrapper

Recommended command shape:

```bash
#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

cargo test -p mesh-rt continuity -- --nocapture
cargo run -q -p meshc -- test cluster-proof/tests
cargo test -p meshc --test e2e_m042_s01 continuity_api_standalone_keyed_submit_status_and_retry_contract -- --nocapture
cargo test -p meshc --test e2e_m042_s02 -- --nocapture
```

Key point: **do not call `bash scripts/verify-m042-s01.sh` from S02.** It still includes the failing two-node remote-owner completion proof.

## File-by-File Planner Notes

### `compiler/mesh-rt/src/dist/continuity.rs`

This is the primary slice file.

What it owns now:

- request record schema
- attempt id generation
- duplicate/conflict logic
- completion/reject transitions
- continuity wire format
- JSON returned to Mesh

What S02 should add here:

- durability-aware submit request
- runtime-owned accepted/rejected admission decision
- degraded transition(s)
- monotonic precedence updates for degraded/rejected
- tests for all new transitions

### `compiler/mesh-rt/src/dist/node.rs`

This is the runtime coordination seam.

What it owns now:

- node sessions
- continuity sync/upsert fanout/merge
- node disconnect callback

What S02 should add here:

- any targeted continuity prepare/ack/reject message handling
- disconnect-driven degraded state updates

### `compiler/mesh-typeck/src/infer.rs`
### `compiler/mesh-codegen/src/mir/lower.rs`
### `compiler/mesh-codegen/src/codegen/intrinsics.rs`
### `compiler/mesh-rt/src/lib.rs`

These are the bounded compiler/runtime API files.

Planner should treat them as one mechanical task if the submit signature changes.

### `cluster-proof/work.mpl`

This is the main consumer-side correction file.

Most important lines of behavior to revisit:

- `continuity_submit(...)`
- `acknowledged_replica_record(...)`
- `created_submit_response(...)`
- `duplicate_submit_response(...)`
- `submit_from_selection(...)`
- `status_response_from_record(...)`

### `compiler/meshc/tests/e2e_m042_s01.rs`

Use this as the harness reference, not as the acceptance gate.

Useful helpers to reuse:

- process spawn/stop
- membership convergence wait
- artifact directory layout
- raw HTTP helpers

But do **not** inherit the remote-owner completion proof into S02.

## Open Choices / Risks

### 1. Submit API shape

Two plausible shapes:

- extend `Continuity.submit(...)` with `required_replica_count`
- add a new `submit_with_durability(...)`-style runtime entrypoint

Given only `cluster-proof` uses this API today, either is manageable. The lower-context option is likely “extend existing submit” unless the planner wants to keep the current shape stable for S01 archaeology.

### 2. Rejected status details when no replica was selected

Current runtime code would naturally preserve:

- `replica_node = ""`
- `replica_status = unassigned`

for a reject transition on a no-replica submit.

That is mechanically easy, but planner should choose deliberately whether the user-visible truth should be:

- `replica_status = unassigned` (no replica was ever available), or
- `replica_status = rejected` (the admission as a whole was rejected)

Either can work; the important thing is consistency across POST/GET/duplicate replay.

### 3. Degraded precedence vs stale mirrored updates

If S02 makes `degraded_continuing` live, the planner should audit stale-message behavior before assuming the enum is enough. `preferred_record(...)` is currently tuned for S01’s simpler world.

### 4. Avoid accidental S03 scope creep

S02 should stop at:

- mirrored acceptance
- rejected admission
- degraded status after replica safety loss

It should **not** attempt:

- owner promotion
- rolled-attempt retry after owner loss
- stale completion suppression across failover

Those are S03 concerns.

## Current Baseline Evidence

Authoritative replay run during this research:

- `bash scripts/verify-m042-s01.sh`

Observed state:

- `cargo test -p mesh-rt continuity -- --nocapture` passes
- `cargo run -q -p meshc -- test cluster-proof/tests` passes
- standalone keyed continuity e2e passes
- two-node e2e still fails because the request remains `submitted/pending` with `replica_status=mirrored`, then the owner crashes in `compiler/mesh-rt/src/string.rs:104` on the remote execution path

That means S02 starts from a truthful but partial baseline:

- runtime-owned submit/status records are real
- mirrored state is visible across nodes
- remote-owner completion is still broken
- fail-closed durability truth and degraded status are still unimplemented
