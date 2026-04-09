# M040/S02 Research — Replica-Backed Admission, Fail-Closed Policy, and Owner-Loss Convergence

## Executive Summary

S02 is the real continuity seam. The codebase already has a usable app-owned skeleton in `cluster-proof/work.mpl`: keyed request records, separate `request_key` vs `attempt_id`, node-specific registry discovery via `Global.whereis(...)`, and a narrow `/work` submit/status contract on the existing proof rail. The slice does **not** need a new app, a new namespace, or a generic distributed-state subsystem.

The first blocker is that the clustered keyed rail is currently red. I reproduced the exact M039 rejoin test:

- `cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_rejoins_and_routes_to_peer_again_without_manual_repair -- --nocapture`

It fails before pre-loss membership is ready. Both nodes log `Config loaded` + `Node started`, then abort at `compiler/mesh-rt/src/string.rs:171:14` (`misaligned pointer dereference`). Neither node reaches `[cluster-proof] work services ready`. That means S02 must budget an initial runtime/startup triage task before any honest two-node continuity proof can go green.

After that unblock, the implementation path should stay narrow:

- keep continuity state app-owned in `cluster-proof/work.mpl`
- make owner/replica placement deterministic across nodes
- require explicit replica acknowledgment before durable acceptance
- add a long-lived monitor actor for `Node.monitor(...)`-driven owner-loss handling
- keep "automatic continuation" as restart-by-key, not checkpoint replay

Do **not** absorb the broad operator/docs migration here. `scripts/verify-m039-s04.sh`, `scripts/verify-m039-s04-fly.sh`, and the docs truth surface are still tied to the old unkeyed `GET /work` contract; that blast radius belongs to S03. For S02, add new slice-local proof surfaces and replay S01 first.

## Requirements Focus

### Primary requirement this slice owns

- **R050** — replica-backed continuity across live nodes with two-node safety as the default proof bar.

### Requirements this slice materially supports

- **R049** — same-key retry and convergent completion after owner loss only become real once continuity survives node failure.
- **R052** — if replica safety becomes an operator-visible policy, its config must land on the existing one-image, env-driven rail (`cluster-proof/config.mpl`, `docker-entrypoint.sh`, `fly.toml`) rather than through a second deployment path.

### Requirement explicitly not owned here

- **R053** — public docs/Fly truth alignment should remain mostly in S03. S02 should create truthful new verifier evidence, not do the full docs/operator migration.

### Hard design rails that must constrain S02

- **R057** — do not turn this into a generic consensus/global-state platform.
- **R058** — reject fake durability when no surviving replica exists.
- **R059** — HTTP/front-door spread is not proof; internal node truth must remain explicit.
- **R060** — Fly is a proof environment, not the architecture.

## Skills Discovered

Directly relevant installed skills already exist:

- `rust-best-practices`
- `flyio-cli-public`

Rules from those skills that should shape S02:

- From **`rust-best-practices`**: keep Rust runtime fixes narrow around unsafe/raw-pointer boundaries, avoid introducing new `panic!/unwrap()` behavior on production paths, and use comments to explain **why** a safety workaround exists. This is directly relevant if S02 touches `mesh-rt` to retire the clustered startup crash or adjust node/global runtime seams.
- From **`flyio-cli-public`**: prefer read-only Fly verification first, and do not add mutating/destructive Fly lifecycle steps without explicit approval. This matches existing decision D144 and is a strong reason to keep destructive failover proof local-Docker authoritative in S02.

I also ran skill discovery for missing directly relevant technologies:

- `npx -y skills find "Mesh language"`
- `npx -y skills find "distributed actor runtime"`

Results were irrelevant (3D/Apify/runtime-adjacent noise), so **no new skill was installed**.

## What Exists Already

### `cluster-proof/work.mpl` already contains most of the right seams

This file is the center of gravity for S02.

Key existing sections:

- `route_selection(...)` (`cluster-proof/work.mpl:179`) — current owner selection logic.
- `submit_decision(...)` (`cluster-proof/work.mpl:438`) — keyed dedupe/conflict decision point.
- `WorkRequestRegistry` (`cluster-proof/work.mpl:813`) — app-owned continuity state service.
- `request_registry_pid_for_node(...)` (`cluster-proof/work.mpl:860`) — node-specific registry discovery seam.
- `current_target_selection()` (`cluster-proof/work.mpl:882`) — current submit-time placement input.
- `execute_work` / `dispatch_work` (`cluster-proof/work.mpl:892`, `:906`) — execution/remote-spawn seam.
- `start_work_services()` (`cluster-proof/work.mpl:921`) — startup seam for registry + monitors.
- `handle_valid_submit(...)` (`cluster-proof/work.mpl:931`) — exact durable-admission seam.
- `handle_valid_status(...)` (`cluster-proof/work.mpl:1010`) — exact status-lookup seam.

The current keyed record already exposes:

- `request_key`
- `attempt_id`
- `phase`
- `result`
- `ingress_node`
- `owner_node`
- `replica_node`
- `replica_status`
- `execution_node`

That means S02 can extend the existing JSON contract instead of inventing a new endpoint family.

### The current implementation is still ingress-local, not replica-backed

Current submit flow in `handle_valid_submit(...)` is:

1. choose owner via `current_target_selection()`
2. write the keyed record into the **local** `WorkRequestRegistry`
3. if created, immediately `dispatch_work(...)`
4. return HTTP 200

Current status flow in `handle_valid_status(...)` only queries `local_request_registry_pid()`.

Current completion flow sends `MarkCompleted` back to the registry identified by `ingress_node`.

That is enough for S01 standalone truth, but it is **not** enough for S02:

- no explicit replica acknowledgment before acceptance
- no cluster-visible status on a surviving peer
- no fail-closed admission policy when replica safety disappears
- no owner-loss monitor/continuation path

### Placement is still deterministic only by accident, not by contract

`route_selection(...)` still means “pick the first non-self peer in membership order.”

Supporting evidence:

- `cluster-proof/work.mpl:179` — `route_selection(...)`
- `cluster-proof/tests/work.test.mpl` — current test explicitly asserts “peer selection prefers the first non-self peer in membership order”
- `cluster-proof/cluster.mpl:41` — `current_membership()` is just self + `Node.list()`
- `compiler/mesh-rt/src/dist/node.rs:2528` — `mesh_node_list()` returns `sessions.keys().cloned().collect()` from an `FxHashMap`

So current placement is not canonical across nodes. S02 cannot treat that ordering as durable truth.

### Global registry is useful, but only for service discovery

Relevant runtime seams:

- `compiler/mesh-rt/src/dist/global.rs:231` — async `broadcast_global_register(...)`
- `compiler/mesh-rt/src/dist/global.rs:263` — async `broadcast_global_unregister(...)`
- `compiler/mesh-rt/src/dist/global.rs:297` — `send_global_sync(...)` on connect
- `compiler/mesh-rt/src/dist/global.rs:134` — `cleanup_node(...)`
- `compiler/mesh-rt/src/dist/node.rs:1372` — disconnect path cleans up remote node registrations

This is good prior art for finding per-node continuity services. It is **not** the continuity ledger itself:

- replication is async
- disconnect aggressively removes ownership for dead nodes
- it stores name → pid, not keyed request state

Use it to discover continuity actors/services, not to store request truth.

### Node monitoring exists, but only from actor context

- `compiler/mesh-rt/src/actor/mod.rs:1261` — `mesh_node_monitor(...)`
- It requires a current process/actor (`stack::get_current_pid()`); otherwise it fails.
- Disconnects deliver `:nodedown`/`:nodeup` messages to the monitoring process.

Implication: S02 cannot bolt owner-loss convergence onto startup code or pure HTTP handlers. It needs a long-lived actor that owns monitor registrations and reacts to node events.

### The operator/config seam is already the right place for durability policy

Files:

- `cluster-proof/config.mpl`
- `cluster-proof/tests/config.test.mpl`
- `cluster-proof/docker-entrypoint.sh`
- `cluster-proof/fly.toml`

This is already a fail-closed env-driven operator path. If S02 needs a durability policy input (replica count or a simple durable-mode toggle), it should land here.

### Existing verifier surfaces are split cleanly enough to preserve slice boundaries

Current proof surfaces:

- `scripts/verify-m040-s01.sh` — standalone keyed contract replay
- `compiler/meshc/tests/e2e_m040_s01.rs` — standalone keyed runtime harness
- `scripts/verify-m039-s04.sh` — local Docker operator proof, still tied to old unkeyed `GET /work`
- `scripts/verify-m039-s04-fly.sh` — read-only Fly verifier, also tied to old unkeyed `GET /work`
- `scripts/lib/m039_cluster_proof.sh` — helper assertions still expect `request_id`, `target_node`, `timed_out`, etc.

That means S02 should add **new** named proof surfaces instead of rewriting the whole operator/docs rail mid-slice.

## Findings That Should Shape Slice Ordering

### 1. First task should be the clustered startup unblock, not replica logic

Reproduced failure:

- `cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_rejoins_and_routes_to_peer_again_without_manual_repair -- --nocapture`

Observed behavior:

- both nodes print `Config loaded ...` and `Node started ...`
- neither node prints `work services ready`
- both abort with:
  - `thread '<unnamed>' panicked at compiler/mesh-rt/src/string.rs:171:14: misaligned pointer dereference`

This strongly suggests the regression is in the **new clustered work-service startup path**, not in later request handling. The most suspicious path is `start_work_services()` → `register_global_request_registry()` because that path is cluster-only and runs before the first HTTP request.

Concrete planner implication: budget a first execution task that does nothing but retire this blocker and make the two-node keyed app start reliably enough to reach pre-loss membership.

### 2. Current placement tests encode the wrong durability contract for S02

`cluster-proof/tests/work.test.mpl` currently enshrines “first non-self peer in membership order” as correct behavior. That was fine for M039/S02 routing proof, but it is not acceptable for replica-backed continuity because `Node.list()` order is not canonical.

Concrete planner implication: the deterministic placement change must include a test rewrite. If the placement helper moves to a canonical-sort or hash-by-key rule, these tests have to move with it immediately.

### 3. `handle_valid_submit(...)` is the exact durable-admission seam

Today, created requests return 200 before any remote durability confirmation. That makes this function the right place to insert:

- local prepare/state write
- remote replica prepare/ack
- fail-closed rejection if replica ack fails or required replica missing
- only then owner dispatch / HTTP 200

Do **not** spread admission logic across unrelated files. The public contract is already centered here.

### 4. Status is currently local-only, which blocks surviving-node truth

`handle_valid_status(...)` only consults the local registry, and the registry itself is currently ingress-local. Under owner loss, a surviving node can only answer truthfully if it has a real continuity copy.

Concrete planner implication: S02 needs either:

- mirrored request records on both owner and replica, or
- a more explicit cluster-visible lookup path

The minimal honest shape is mirrored minimal records on the two involved nodes.

### 5. The current record shape is missing failover-era attempt evolution

S01’s single `attempt_id` field was enough for healthy dedupe/completion. S02’s owner-loss continuation changes that.

Why:

- if the owner dies before completion, retry/continuation should not pretend it is the original execution attempt
- current `apply_completion(...)` already rejects stale completions via attempt-id mismatch, which is useful
- but there is no explicit path yet for “request survives, current attempt rolls forward”

Concrete planner implication: the request record/state machine likely needs an attempt counter or a controlled way to replace the active `attempt_id` on continuation while keeping `request_key` stable.

### 6. Node monitoring requires a dedicated actor, not startup code

`Node.monitor(...)` only works from actor context. `start_work_services()` and config/startup logic are not a monitor loop.

Concrete planner implication: S02 should add a long-lived continuity-monitor actor spawned by `start_work_services()`. That actor should own `Node.monitor(...)` registrations and translate `:nodedown`/`:nodeup` into continuity-state transitions and continuation triggers.

### 7. Existing M039/S04 proof surfaces are deliberately out of date for keyed work

- `scripts/verify-m039-s04.sh` still probes `GET /work`
- `scripts/verify-m039-s04-fly.sh` still asserts the old routed `request_id/target_node/execution_node` response shape
- `scripts/lib/m039_cluster_proof.sh` still validates the old payload schema

Concrete planner implication: do **not** spend S02 rewriting those scripts. Add `e2e_m040_s02.rs` + `scripts/verify-m040-s02.sh` now, and leave broad operator/docs/Fly convergence to S03.

This also matches D144 plus the `flyio-cli-public` skill guidance: keep Fly verification read-only unless there is explicit approval for mutating lifecycle proof.

## Recommendation

The narrowest honest S02 design is:

1. **Retire the startup blocker first** so clustered keyed work can even run.
2. **Canonicalize placement** across nodes using a deterministic membership/placement helper.
3. **Treat continuity as an app-owned mirrored record**, not a runtime-global database.
4. **Require explicit replica ack before accepting new durable work**.
5. **Use a continuity-monitor actor** for owner/replica loss handling.
6. **Implement continuation as restart-by-key** with a new active attempt when needed.
7. **Add slice-local proof surfaces**; do not widen into S03’s docs/operator migration.

A plausible first-wave ownership model that fits the current code shape is:

- owner = deterministic chosen execution node
- replica = the other live node in the two-node proof case
- mirrored minimal record on both nodes
- submit accepted only after both nodes agree on the record
- if owner dies but replica survives, retry/continuation can promote a new owner attempt from replica-backed state
- if the cluster loses the second live continuity copy, new durable work is rejected

That design satisfies R050 without drifting into checkpoint replay or generic global state.

## Recommended Task Decomposition / Natural Seams

### Task 1 — Clustered keyed startup unblock

**Files most likely involved:**

- `cluster-proof/work.mpl`
- `compiler/mesh-rt/src/dist/node.rs`
- possibly small supporting runtime files if the fault is in the string/allocation path

**Goal:** make two-node keyed `cluster-proof` start without aborting before `work services ready`.

**Notes for planner:**

- Start from the reproduced failing test, not from speculative refactors.
- The failing path is cluster-only and happens before the first request.
- If a Rust runtime fix is needed, keep it narrow and safety-documented per `rust-best-practices`.

### Task 2 — Deterministic owner/replica placement + policy config seam

**Files most likely involved:**

- `cluster-proof/work.mpl`
- `cluster-proof/cluster.mpl`
- `cluster-proof/tests/work.test.mpl`
- possibly `cluster-proof/config.mpl` + `cluster-proof/tests/config.test.mpl`
- `cluster-proof/docker-entrypoint.sh`
- maybe `cluster-proof/fly.toml` if a default env contract must exist there

**Goal:** replace current membership-order placement with canonical placement, and introduce any durability-policy env on the existing operator rail.

**Notes for planner:**

- keep config surface small
- fail closed on invalid/partial policy env
- do not create a durability-only deployment path

### Task 3 — Replica-backed admission + mirrored continuity record

**Files most likely involved:**

- `cluster-proof/work.mpl`

**Goal:** extend the existing registry/state machine so created requests are only accepted after replica confirmation, and status shows truthful owner/replica state.

**Notes for planner:**

- this is the core S02 task
- keep the continuity record minimal: request key, payload hash (or minimal identity needed for retry safety), owner, replica, phase/result, active attempt, completion marker, rejection/error reason
- reuse `request_registry_pid_for_node(...)` / global registration for service discovery, not state storage

### Task 4 — Owner-loss monitoring and convergence

**Files most likely involved:**

- `cluster-proof/work.mpl`
- maybe small runtime touch only if monitor semantics are insufficient

**Goal:** add a dedicated actor that monitors continuity peers, updates state on owner loss, and triggers continuation/restart-by-key when truthful.

**Notes for planner:**

- do not try to bolt `Node.monitor(...)` into startup/config code
- this is where fail-closed policy for new durable work vs continuation for already-replicated work should become explicit

### Task 5 — New S02 proof surfaces

**Files to add:**

- `compiler/meshc/tests/e2e_m040_s02.rs`
- `scripts/verify-m040-s02.sh`

**Existing files to replay but not rewrite yet:**

- `scripts/verify-m040-s01.sh`
- `compiler/meshc/tests/e2e_m040_s01.rs`

**Do not pull into this slice unless strictly necessary:**

- `scripts/lib/m039_cluster_proof.sh`
- `scripts/verify-m039-s04.sh`
- `scripts/verify-m039-s04-fly.sh`
- docs/README truth surfaces

## Verification Strategy

### Repro / unblock loop

Use this as the first red→green gate while fixing the startup blocker:

- `cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_rejoins_and_routes_to_peer_again_without_manual_repair -- --nocapture`

Current failure is useful because it proves the app never reaches healthy two-node keyed startup.

### Fast inner-loop checks

- `cargo run -q -p meshc -- test cluster-proof/tests`
- `cargo run -q -p meshc -- build cluster-proof`

If runtime files are touched, also run targeted `mesh-rt` tests covering the exact touched node/global-runtime functions before relying only on app-level e2e.

### Slice acceptance commands

Recommended S02 acceptance chain:

1. `bash scripts/verify-m040-s01.sh`
2. `cargo test -p meshc --test e2e_m040_s02 -- --nocapture`
3. `bash scripts/verify-m040-s02.sh`

### What the new S02 proof surfaces should preserve

The new verifier/harness should archive artifacts that mirror the milestone acceptance, not just report pass/fail:

- pre-loss keyed submit JSON
- pre-loss keyed status JSON showing owner/replica truth
- proof that acceptance only happened after replica-backed durability was confirmed
- degraded-phase JSON showing **new** durable work rejected after replica safety disappears
- surviving-node status JSON for the existing request after owner loss
- same-key retry/continuation JSON proving convergent completion without duplicate completion leakage
- per-node stdout/stderr logs showing owner/replica/continuation transitions

This should follow the existing M039 artifact discipline: pre-loss, degraded, post-failure continuation, copied logs, and a phase report.

## Risks and Unknowns

### Clustered startup root cause is not fully isolated yet

The exact crash is reproduced, but the precise culprit is still a hypothesis. Budget a root-cause task first; do not paper over it with broader refactors.

### Cross-node string transport remains a real risk after startup is fixed

Even after the startup blocker, S02 will still pressure the same distributed seam:

- `dispatch_work(...)` currently remote-spawns with string arguments (`request_key`, `attempt_id`, `ingress_node`)
- replica prepare/ack will likely also need cross-node string traffic unless carefully minimized

So after the startup fix, immediately prove a real two-node keyed submit before adding owner-loss logic.

### The current state model does not yet make continuation history explicit

S01’s single-attempt record was enough for standalone dedupe. S02 needs an explicit stance on whether status shows the original attempt, the active attempt, or both. My recommendation is: keep `request_key` stable, expose the **current/latest** attempt id, and avoid adding a deep execution-history surface in this slice.

### Durability policy should stay tiny

The config seam is ready, but the policy contract is not decided yet. Keep it to one small concept (for example a replica count or simple durability mode). Do not create a large matrix of mode flags.

### Automatic continuation should stay restart-by-key only

Do not expand into checkpoint replay. The milestone context and M040 research already set the honest bar: automatic continuation when possible should mean “surviving continuity state can launch a fresh attempt by key,” not generic process migration or arbitrary in-memory recovery.
