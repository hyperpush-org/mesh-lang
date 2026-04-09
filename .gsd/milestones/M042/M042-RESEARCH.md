# M042 Research — Runtime-Native Distributed Continuity Core

## Executive Summary

M042 should start by **lifting the already-working keyed continuity semantics out of `cluster-proof/work.mpl` and into a new runtime subsystem in `mesh-rt`**, not by inventing a new contract from scratch. The codebase already contains the semantic seed M042 wants:

- stable `request_key` identity separate from `attempt_id`
- deterministic owner/replica placement
- explicit owner / replica / execution / phase / error truth in status payloads
- fail-closed durability rejection when replica safety is unavailable
- owner-loss continuation that rolls attempt identity forward instead of pretending to migrate execution state exactly

The problem is ownership, not concept discovery. Today `cluster-proof` still owns:

- the continuity record schema
- replica prepare / ack / rejection choreography
- owner-loss recovery logic
- cross-node status lookup RPCs
- continuity-specific monitoring behavior
- the public status vocabulary

Meanwhile `mesh-rt` only owns the lower-level distributed substrate:

- node lifecycle, discovery, and sessions
- `:nodeup` / `:nodedown` delivery
- remote spawn / send transport
- globally replicated process names

That gap is the milestone.

The strongest planning signal is that **runtime-native continuity will require a new runtime state subsystem and compiler/runtime API plumbing**, not just a refactor of the existing global registry or a few helper moves. `Global.register` is replicated name discovery, not replicated request truth. `Node.monitor` is node-event delivery, not continuity state management. `cluster-proof/work.mpl` is currently bridging those gaps itself through Mesh services, JSON payloads, timeouts, and polling.

The first honest proof target is:

1. keep the existing keyed submit/status semantics recognizable,
2. move owner/replica continuity state and admission logic into Rust/runtime code,
3. expose that through a narrow Mesh-facing API,
4. reduce `cluster-proof` to a thin consumer and proof surface,
5. preserve the validated M039 operator rail instead of rewriting history.

## Skills Discovered

Directly relevant installed skills already present:

- `rust-best-practices` — relevant to `mesh-rt` and compiler surface changes
- `flyio-cli-public` — relevant to the existing Fly operator rail

New directly relevant skill installed during this research:

- `distributed-systems` (`yonatangross/orchestkit@distributed-systems`) — installed globally for downstream planning/execution units

Not installed:

- generic system-design skills were available, but they were broader than the actual core technologies and not necessary for this milestone

## What Exists Already

### 1. The semantic seed already exists in `cluster-proof`

`cluster-proof/work.mpl` already defines most of the user-visible continuity model M042 wants to preserve:

- `request_key` vs `attempt_id`
- `WorkRequestRecord` and `WorkStatusPayload`
- explicit `phase`, `result`, `owner_node`, `replica_node`, `replica_status`, `execution_node`, `error`, `conflict_reason`
- idempotent duplicate handling vs same-key conflict rejection
- fail-closed durability rejection (`503`) when replica safety is unavailable
- owner-loss continuation that promotes a mirrored replica to owner and rolls `attempt_id`

This is not throwaway prototype logic anymore. It is the de facto product contract.

### 2. Deterministic placement is already solved once — in Mesh code

`cluster-proof/Cluster.mpl` already retired one earlier risk: placement is no longer based on ingress-local peer order.

It now does:

- canonical membership normalization
- deterministic deduped ordering across views
- stable owner / replica selection via hash scoring (`canonical_placement`)

That logic should be **reused and moved** into the runtime boundary. Re-inventing placement rules in Rust without preserving these semantics would create needless drift.

### 3. `cluster-proof` is currently both proof app and distributed algorithm owner

`cluster-proof/main.mpl` mounts three continuity-related surfaces:

- `GET /membership` — M039 truth surface
- `GET /work` — legacy routing proof surface from M039
- `POST /work` and `GET /work/:request_key` — M040 keyed continuity surface

That means the app is currently bifurcated:

- legacy read-only remote-routing proof still exists for M039
- new keyed continuity behavior is layered on top in the same module

This is a strong reason to **avoid a big-bang contract rewrite**. M039 is validated baseline state and should stay stable while the new runtime-native continuity rail lands.

### 4. The proof app currently re-implements continuity with app-owned RPC and polling

`cluster-proof/work.mpl` still does all of this in Mesh code:

- local `WorkRequestRegistry` service state
- mirror-prepare / replica-ack / durability-reject RPCs
- record fetch RPCs (`GetRecord`, `GetStatus`)
- node-loss handling via `HandleNodeDown`
- `Job.async` + `Job.await_timeout(...)` wrappers for continuity calls
- a 250ms ticker-driven continuity monitor that polls peers and registry state

This is the main ownership smell M042 is correcting.

### 5. `mesh-rt` already provides the right low-level hooks, but not the continuity subsystem

Useful existing runtime seams:

- `compiler/mesh-rt/src/dist/node.rs`
  - node session lifecycle
  - discovery integration
  - `handle_node_disconnect(...)`
  - `:nodeup` / `:nodedown` delivery to node monitors
  - remote spawn request/reply protocol
- `compiler/mesh-rt/src/dist/global.rs`
  - replicated name registry
  - snapshot sync on connect
  - disconnect cleanup
- `compiler/mesh-rt/src/actor/mod.rs`
  - `mesh_node_monitor`
  - `mesh_global_register` / `whereis` / `unregister`
- `compiler/mesh-rt/src/actor/scheduler.rs`
  - process-exit cleanup and remote `:noconnection` propagation

What does **not** exist yet is runtime-owned continuity state:

- no request ledger
- no replicated continuity record sync
- no runtime durability-admission ack path
- no runtime owner-loss recovery logic
- no continuity-specific disconnect/rejoin reconciliation

### 6. The compiler surface for a new API is explicit and bounded

The current distributed primitives are surfaced through hard-coded module/intrinsic plumbing:

- `compiler/mesh-typeck/src/infer.rs` defines `Node.*` and `Global.*`
- `compiler/mesh-codegen/src/mir/lower.rs` maps intrinsic names to runtime symbols
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` declares the runtime externs
- `compiler/mesh-rt/src/lib.rs` exports them

That means M042 can add a first-class API **without parser work**, as long as the API is expressed as ordinary module functions.

This is an important planning constraint: if the first API wave stays fixed-arity and data-oriented, the compiler work stays straightforward. If the API demands a new variadic “remote continuity spawn” surface, the typechecker/codegen complexity rises immediately because `Node.spawn` is already special-cased for that reason.

### 7. The operator and proof rails already exist and should be reused

Current authoritative proof surfaces:

- `scripts/verify-m039-s04.sh` — local Docker destructive lifecycle proof
- `scripts/verify-m039-s04-fly.sh` — read-only live Fly proof
- `scripts/verify-m039-s04-proof-surface.sh` — docs truth gate
- `scripts/verify-m040-s01.sh` — standalone keyed submit/status contract proof
- `compiler/meshc/tests/e2e_m039_s03.rs` — degrade/rejoin harness pattern
- `compiler/meshc/tests/e2e_m040_s01.rs` — keyed contract harness

This is good news: M042 does not need a new proof ecosystem, only a new implementation boundary behind the existing one-image/operator story.

## Key Constraints From the Current Codebase

### 1. `Global.register` is not the continuity database

`compiler/mesh-rt/src/dist/global.rs` is intentionally simple:

- name → pid + owning node
- async broadcast register/unregister
- snapshot sync on connect
- disconnect cleanup by node
- process-exit cleanup by pid

That is a good pattern for discovery and rendezvous. It is not sufficient as continuity truth because:

- it only stores process names
- it is first-writer-wins, not request-state aware
- it eagerly cleans names on disconnect, exactly when continuity truth is most important
- it has no admission, ack, replica, or phase model

Planning implication: **do not extend `global.rs` until it becomes a fake general distributed KV store.** A separate continuity subsystem is cleaner and aligns with R057.

### 2. Current remote send still silently drops on failure

`compiler/mesh-rt/src/actor/mod.rs::dist_send(...)` still:

- silently drops if node not started
- silently drops if node id is unknown
- silently drops if the session is gone
- silently drops on write error

That is acceptable for generic async messaging with `:nodedown` semantics layered around it. It is **not** an honest durability-admission path.

Planning implication: runtime-native continuity needs an explicit acked protocol, not best-effort app-level sends.

### 3. Current continuity RPC is timeout-based and JSON-shaped in Mesh code

`cluster-proof/work.mpl` currently uses `Job.async(...)` + `Job.await_timeout(...)` around continuity operations like:

- submit to owner registry
- fetch record from owner/replica
- mirror prepare
- acknowledge replica
- reject durability

Those are truthful enough for a proof app, but they are exactly the sort of brittle app-authored orchestration M042 is meant to eliminate.

### 4. Node-loss continuity today is polling-driven, not event-native

Although the runtime already emits `:nodeup` / `:nodedown`, the proof app’s continuity monitor still combines:

- `Node.monitor(...)`
- a 250ms ticker
- peer-list polling
- local registry calls

That is a strong signal that the app is compensating for a missing runtime facility. M042 should move continuity reaction closer to `handle_node_disconnect(...)` / session lifecycle instead of keeping a polling actor as the real owner.

### 5. The M039 baseline must stay intact

M042 context explicitly says M039 stays validated baseline and should not be rewritten. The code reinforces that:

- M039 verifiers and docs still depend on `GET /membership` and legacy `GET /work`
- M040 standalone proof depends on keyed `POST /work` and `GET /work/:request_key`
- `cluster-proof` currently serves both contracts

Planning implication: do **not** make “replace the legacy `/work` proof rail” the first risky slice. Land runtime-native keyed continuity first, keep M039 baseline working, then reconcile surfaces deliberately.

## What Should Be Proven First

### First proof target: runtime-owned keyed contract on a healthy cluster

Before failover, prove that the runtime can own the keyed contract without semantic drift:

- submit keyed work through the new API
- report truthful status through the same field vocabulary
- dedupe same-key same-payload retries
- reject same-key conflicting retries
- keep `request_key` stable and `attempt_id` attempt-scoped

This should be the first proof because it validates the new ownership boundary while keeping failure complexity low.

### Second proof target: fail-closed durable admission

Before automatic continuation, prove the runtime refuses to lie:

- if configured replica safety is unavailable, new durable work is rejected
- the response surface is explicit and machine-checkable
- status reads remain truthful instead of silently degrading into local-only acceptance

This is the real honesty contract for M042 and is more important than clever failover.

### Third proof target: owner-loss recovery on the runtime-owned substrate

Once healthy-cluster semantics and admission truth hold, prove:

- owner loss is detected by runtime-owned machinery
- surviving replica can present truthful status
- same-key retry converges through rolled `attempt_id`
- stale completions do not overwrite the active attempt

This should preserve the existing restart-by-key model, not overreach into exactly-once or process migration.

### Fourth proof target: thin proof app + operator/docs truth

Only after the substrate is real should `cluster-proof` be simplified and the operator/docs rails updated.

## Existing Patterns To Reuse

### Reuse 1: Current status vocabulary

The current keyed contract already exposes a useful truth model:

- `phase`: `submitted`, `completed`, `rejected`, `missing`, etc.
- `result`: `pending`, `succeeded`, `rejected`, `unknown`
- `replica_status`: `unassigned`, `preparing`, `mirrored`, `rejected`, `degraded_continuing`
- `owner_node`, `replica_node`, `execution_node`
- `conflict_reason`, `error`

This is already good enough for verifiers and operators. Reuse it.

### Reuse 2: Deterministic placement algorithm

Lift the semantics of `cluster-proof/Cluster.mpl::canonical_placement(...)` into runtime-owned code instead of designing a new placement rule.

### Reuse 3: Runtime session/disconnect hooks

The new subsystem should hook into the same lifecycle that already exists:

- session registration
- connect sync path
- disconnect cleanup path
- node monitor events

The likely shape is a new runtime module alongside `dist/global.rs`, not a further expansion of `cluster-proof` service logic.

### Reuse 4: One-image operator rail

Keep using:

- `cluster-proof/fly.toml`
- repo-root Docker build
- local Docker verifier as destructive authority
- Fly verifier as read-only live authority unless explicitly expanded later

This matches D144 and avoids unplanned live-environment blast radius.

### Reuse 5: Compiler intrinsic plumbing

The current `Node` / `Global` path is the cleanest way to expose a first-class continuity API in the first wave.

## Boundary Contracts That Matter

### 1. `request_key` vs `attempt_id`

This split is non-negotiable now. It is the core reason duplicates, conflicts, stale completions, and failover can be expressed honestly.

### 2. Admission truth must be explicit

The API must expose whether work was accepted under full durability, rejected, or being served from degraded surviving state. This cannot be reduced to log interpretation.

### 3. Owner / replica truth must be inspectable from ordinary app code

M042 context explicitly wants observable owner/replica/durability state. That means status reads should stay app-visible instead of burying continuity truth in opaque runtime internals.

### 4. Recovery model should stay “restart-by-key”, not “resume arbitrary process state”

Nothing in the codebase supports honest checkpoint migration today, and the requirement set explicitly resists turning this into a generic distributed-state system. The current rolled-attempt model is the right semantic floor.

### 5. `cluster-proof` should consume the capability, not define it

The app should still be able to shape:

- HTTP routes
- response mapping
- payload schema for its proof/demo

But it should not continue to own:

- replica placement
- admission protocol
- replicated record sync
- owner-loss handoff logic
- durability-state transitions

## Known Failure Modes That Should Shape Slice Ordering

### 1. Silent remote-send failure

If the first slice relies on app-level send/receive semantics for durable replication, it will recreate the same honesty gap M042 is meant to close.

### 2. Global-name cleanup on disconnect

If continuity truth depends on only being able to resolve a globally registered owner service after disconnect, status lookup will collapse exactly when it is needed most.

### 3. Current app-level timeout seams

The existing 250ms timeout-based continuity RPC layer is a proof-app convenience, not a durable product seam. It should not survive as the real implementation under a runtime-native claim.

### 4. Contract churn across validated rails

M039’s `/membership` + legacy `/work` rail is validated baseline. Conflating “runtime-native continuity” with “retire the old proof contract” will raise risk without proving the new substrate sooner.

### 5. Variadic API ambition

If the first API design tries to be too magical — e.g. a generalized distributed work-spawn abstraction with function refs and variadic args — the compiler surface widens immediately. A smaller fixed-arity submit/status API is the safer first wave.

## Suggested Slice Boundaries

### Slice 1 — Runtime continuity substrate + narrow Mesh API on the healthy path

Primary goal: create the new ownership boundary without changing semantics.

Suggested scope:

- introduce a dedicated runtime continuity subsystem in `mesh-rt` (separate from `dist/global.rs`)
- add narrow Mesh-facing continuity API plumbing through typeck/lowering/intrinsics/runtime exports
- keep the API fixed-arity and data-oriented in the first wave
- port current keyed submit/status semantics onto the runtime-owned substrate on the healthy-cluster path
- prove standalone + healthy two-node keyed contract

Why first:

- highest architecture risk
- lowest failure-mode complexity
- establishes whether the boundary change is real

### Slice 2 — Replica-backed admission and truthful degraded refusal

Primary goal: make durability claims honest.

Suggested scope:

- runtime-owned owner/replica record sync / ack path
- fail-closed admission when replica safety is unavailable
- status truth for mirrored vs rejected vs degraded states
- verifier coverage for missing-replica rejection and still-readable status

Why second:

- this is the milestone’s honesty bar
- it can be proven before automatic continuation logic

### Slice 3 — Owner-loss recovery and stale-completion safety

Primary goal: prove the runtime can continue keyed work after node loss on surviving replicated state.

Suggested scope:

- runtime-driven owner-loss promotion from mirrored replica
- rolled `attempt_id` semantics preserved
- stale completion rejection remains truthful
- local Docker destructive proof for owner loss, degraded serve, and rejoin

Why third:

- highest runtime failure-semantics risk
- should build on already-stable admission truth and status model

### Slice 4 — Thin `cluster-proof` consumer + operator/docs/Fly reconciliation

Primary goal: reduce the app to proof surface and align public truth.

Suggested scope:

- delete or shrink app-authored continuity orchestration from `cluster-proof/work.mpl`
- keep M039 validated surfaces intact or deliberately bridged
- update local Docker verifier, Fly verifier, README, docs page, and proof-surface checks
- keep Fly as read-only steady-state verification unless the roadmap explicitly expands it

Why fourth:

- mostly integration/cleanup once the substrate is trustworthy
- avoids turning docs/operator drift into the first blocker

If the roadmap planner wants fewer slices, S2 and S3 can merge — but that raises risk because it combines “admission honesty” and “owner-loss recovery” in one step.

## Requirement Analysis

### Table stakes

Still core for M042:

- **R049** — keyed at-least-once idempotent continuity
- **R050** — replica-backed continuity with two-node default safety bar
- **R052** — one-image, env-driven operator path still holds
- **R053** — public proof/docs/verifier surfaces stay truthful

### Important observation: R053 is already validated, but it still constrains this milestone

Even though R053 is validated from M039, M042 will break truth if it changes the contract without updating the proof rail. Treat it as active guardrail in planning even if the file marks it validated.

### Under-specified gaps for M042

The current active requirement set does not make three things explicit enough:

1. **runtime ownership boundary** — the milestone context says the algorithm belongs in `mesh-rt`, but the requirement contract does not state that directly
2. **status-surface obligations** — owner/replica/admission truth is expected, but not fully contracted as an explicit requirement
3. **fail-closed admission** — R058 captures the honesty boundary, but it is still out-of-scope/constraint instead of being promoted into active milestone coverage

### Constraints that should actively shape planning

Treat these as live rails, not background notes:

- **R057** — do not turn this into generic consensus-backed app state
- **R058** — do not claim durability when no surviving replica exists
- **R059** — front-door spread is not proof
- **R060** — Fly is a proof environment, not the architecture

## Candidate Requirements

These are advisory recommendations for the roadmap planner, not automatic scope expansion.

### Candidate Requirement A — Runtime-native ownership should be explicit

Add or promote a requirement that says the continuity algorithm for keyed owner/replica/admission/recovery lives in `mesh-rt`, while Mesh apps consume it through a narrow API instead of implementing it themselves.

Why:

- this is the core milestone boundary
- without it, closeout can go green while too much logic still lives in `cluster-proof`

### Candidate Requirement B — Status must expose operator-useful continuity truth

Add or promote a requirement that status surfaces must expose enough machine-checkable state to prove continuity without log scraping.

Minimum likely fields:

- `request_key`
- `attempt_id`
- `phase`
- `result`
- `owner_node`
- `replica_node`
- `replica_status`
- `execution_node`
- `error` / `conflict_reason`

### Candidate Requirement C — Fail-closed durable admission should be active, not just implied

Promote the spirit of R058 into explicit active milestone coverage for M042.

Why:

- it is part of milestone acceptance already
- it is too important to leave as background constraint text

### Candidate Requirement D — Preserve validated M039 proof surfaces while landing M042

Consider an explicit launchability requirement that M039’s validated `/membership` and internal-routing proof rail remain truthful while the runtime-native keyed continuity path is introduced.

Why:

- M039 is validated baseline
- this prevents “new milestone broke old truth” regressions from being treated as acceptable churn

## Key Files For Follow-Up Planning

### Runtime / compiler boundary

- `compiler/mesh-rt/src/dist/node.rs` — node lifecycle, disconnect handling, node monitor events, remote spawn protocol
- `compiler/mesh-rt/src/dist/global.rs` — replicated-name pattern to reuse structurally, but not as the continuity store
- `compiler/mesh-rt/src/actor/mod.rs` — exported distributed ABI functions and current silent-drop behavior
- `compiler/mesh-rt/src/actor/scheduler.rs` — exit cleanup and remote `:noconnection` propagation
- `compiler/mesh-typeck/src/infer.rs` — hard-coded module typing for `Node` / `Global`
- `compiler/mesh-codegen/src/mir/lower.rs` — intrinsic-name mapping
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — runtime extern declarations

### Current semantic seed in Mesh code

- `cluster-proof/Cluster.mpl` — canonical membership + deterministic owner/replica placement
- `cluster-proof/work.mpl` — current keyed continuity model and the logic M042 is meant to absorb into runtime
- `cluster-proof/main.mpl` — public proof routes and current split between legacy `/work` and keyed submit/status
- `cluster-proof/tests/work.test.mpl` — placement and keyed-contract expectations
- `cluster-proof/tests/config.test.mpl` — durability-policy config behavior

### Proof and operator rails

- `compiler/meshc/tests/e2e_m039_s03.rs` — degrade/rejoin proof harness shape
- `compiler/meshc/tests/e2e_m040_s01.rs` — standalone keyed submit/status harness
- `scripts/verify-m039-s04.sh` — authoritative local destructive operator proof
- `scripts/verify-m039-s04-fly.sh` — read-only live Fly proof
- `scripts/verify-m039-s04-proof-surface.sh` — docs truth gate
- `scripts/verify-m040-s01.sh` — standalone keyed contract verifier
- `cluster-proof/README.md` — operator runbook contract
- `website/docs/docs/distributed-proof/index.md` — public distributed proof page

## Bottom Line For The Roadmap Planner

The safest strategic direction is:

- **do not redesign the semantics** — promote the current keyed contract
- **do not keep the algorithm in the app** — move it into a dedicated runtime subsystem
- **do not widen into generic replicated state** — keep the record minimal and keyed
- **do not start by rewriting the M039 proof rail** — preserve the validated baseline while the new runtime-native path lands
- **do prove honesty before cleverness** — healthy keyed contract first, fail-closed admission second, owner-loss recovery third, docs/operator cleanup last
