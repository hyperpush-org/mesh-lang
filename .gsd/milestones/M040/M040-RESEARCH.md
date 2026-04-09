# M040 Research — Replicated Continuity & In-Cluster Request Durability

## Executive Summary

M040 should start by stabilizing the **keyed submit + status contract** on the existing `cluster-proof/` rail before attempting automatic failover. The codebase already has the right narrow seams: `cluster-proof/main.mpl` is the only HTTP entrypoint, `cluster-proof/work.mpl` already owns request correlation and contains an unused local coordination service, `mesh-rt` already exposes `Global.register/whereis/unregister` and `Node.monitor`, and the M039 verifier chain already preserves the exact pre-loss/degraded/post-rejoin evidence shape that M040 should extend.

The main planning constraint is that the current distributed seam is still **raw-bytes transport**, not a trustworthy rich-state transport. `Node.spawn` packs raw arg bytes, `mesh_node_spawn` forwards them directly, and remote actor `send(...)` also forwards raw message bytes and silently drops on session loss. That means the first honest continuity design should replicate a **small app-owned continuity record** (key, owner, replica set, phase/status, attempt metadata, completion marker), not arbitrary payloads or checkpoint state.

The biggest non-obvious implementation risk is **deterministic ownership selection**. Today `cluster-proof/work.mpl` uses self-first membership plus “first non-self peer,” and `mesh_node_list()` returns `sessions.keys()` from an `FxHashMap` with no canonical ordering. That was fine for the M039 routing proof, but it is not safe to reuse for keyed owner/replica assignment. M040 needs a canonical node ordering or hash placement rule before replication is meaningful.

A second major planning tension is **Fly proof scope**. Current Fly proof is explicitly read-only (`scripts/verify-m039-s04-fly.sh`). Local Docker can prove destructive failover. Live Fly currently proves steady-state config/membership/routing only. If M040 acceptance truly requires destructive failover on Fly, that needs an explicit planning decision because the current proof surface intentionally avoids Fly mutations.

## Skills Discovered

Checked the installed skills against the directly relevant technologies for this milestone:

- `rust-best-practices` — relevant for `mesh-rt` runtime changes
- `flyio-cli-public` — relevant for Fly operator-path truth
- `multi-stage-dockerfile` exists, but Dockerfile design is not the core uncertainty for M040

No additional skills were installed. The directly relevant core technologies already have installed coverage.

## What Exists Already

### `cluster-proof` is still a very narrow proof app

`cluster-proof/main.mpl` only mounts:

- `GET /membership`
- `GET /work`

That narrowness is good news for M040. The public proof surface is localized and can evolve without creating a second showcase app.

`cluster-proof/work.mpl` is still M039-shaped:

- request identity is ingress-local `work-N`
- `/work` returns success immediately after dispatch
- there is no keyed request model
- there is no replicated continuity ledger
- there is no owner/replica status surface

Two important surprises in `work.mpl`:

1. It already contains a local `WorkResultRegistry` service, `PrepareRequest`, `TakeResult`, and `await_result(...)` flow — but the live `/work` handler does **not** use that path. That is the cleanest existing seam for introducing a request-status state machine without inventing a new proof package.
2. `result_registry_name_for_node(_node_name)` ignores its node argument and always returns the same local name. That reinforces that current coordination is node-local only.

### The operator surface is already fail-closed and env-driven

The current operator contract is implemented across:

- `cluster-proof/config.mpl`
- `cluster-proof/tests/config.test.mpl`
- `cluster-proof/docker-entrypoint.sh`
- `cluster-proof/fly.toml`

This is exactly where M040 should add any new durability-policy configuration. The existing pattern is:

- validate env up front
- fail closed on partial/invalid identity
- keep one image and one small env contract
- treat Fly as one proof environment, not the architecture

There is **no durability-policy env** yet, but the config seam is ready for one.

### Runtime primitives exist, but they are not a durability system

Relevant `mesh-rt` surfaces already exist:

- `Node.self()` / `Node.list()`
- `Node.monitor(node_name)`
- `Global.register(name, pid)` / `Global.whereis(name)` / `Global.unregister(name)`
- remote `Node.spawn(...)`
- cross-node actor send via remote PIDs

The useful prior art is `compiler/mesh-rt/src/dist/global.rs`:

- async broadcast of global registrations
- sync snapshot on connect (`DIST_GLOBAL_SYNC`)
- cleanup on disconnect (`cleanup_node`)
- first-writer-wins merge on sync

This makes the global registry useful for **service discovery** and maybe owner visibility, but not sufficient as the continuity ledger itself:

- registration replication is async
- disconnect cleanup aggressively removes names
- it stores name → PID ownership, not request continuity state

### The M039 verifier chain is already the right evidence shape

The proof chain already composes cleanly:

- `compiler/meshc/tests/e2e_m039_s03.rs`
- `scripts/verify-m039-s03.sh`
- `scripts/verify-m039-s04.sh`
- `scripts/verify-m039-s04-fly.sh`
- `cluster-proof/README.md`
- `website/docs/docs/distributed-proof/index.md`

The authoritative artifact pattern is already correct for M040:

- pre-loss evidence
- degraded evidence
- post-rejoin evidence
- JSON snapshots plus copied stdout/stderr logs
- later verifiers replay earlier ones and fail closed

M040 should extend this chain, not fork it.

## Findings That Should Shape Slice Ordering

### 1. Ownership/replica placement must become canonical before replication work starts

Current routing selection in `cluster-proof/work.mpl` is ingress-centric:

- membership is built as `[self] ++ peers`
- `find_first_peer_index(...)` picks the first non-self peer
- `current_sorted_membership()` is misnamed; it just returns `current_membership()` with no sorting

At runtime, `mesh_node_list()` returns `sessions.keys()` from an `FxHashMap`, so peer order is not canonical.

That means any M040 design that assigns primary/replica using current membership order will produce different answers on different nodes. This is the first thing that needs to be fixed conceptually. For keyed continuity, owner/replica selection must come from a deterministic rule shared across nodes, not from each node’s local self-first view.

### 2. Stable external request key and internal attempt identity should be separate

Today `request_id` means “local monotonically increasing probe token.” M040 needs a stable **caller-supplied request key** for retry/idempotence, but it will still need an internal attempt/execution identifier for logs and transitions.

Planning implication: do not overload `request_id` into the stable user contract. Keep a stable request key and a separate volatile attempt/execution identity.

### 3. Durable acceptance must require explicit replica confirmation

Two runtime facts matter here:

- `mesh_node_spawn(...)` forwards raw arg bytes
- remote actor `send(...)` goes through `dist_send(...)`, which silently drops on unknown session/write failure

So a design that “accepts durable work, then best-effort broadcasts replica state” would be dishonest. M040’s durable admission path needs an explicit replica-confirmed step before it claims durable acceptance.

`Global.register(...)` is not enough for this because it is async replicated discovery, not a write-ack durability primitive.

### 4. Keep the first-wave replica record minimal and app-owned

The M039 research and decisions already showed that rich cross-node payload movement through restart/rejoin is not a seam to trust casually. The safest first-wave continuity record is small and explicit, e.g.:

- request key
- owner node
- replica node(s)
- lifecycle phase/status
- attempt counter / execution token
- completion marker
- rejection reason / durability-health reason

That supports restart-by-key semantics without pretending to do checkpoint replay or generic replicated app-state storage.

### 5. `Global.register` and `Node.monitor` are reuse candidates, not the whole design

The best reuse path is likely:

- use `Global.register` for discovering continuity service actors on each node
- use `Node.monitor` for owner/replica failure observation
- keep the actual request ledger in app-owned continuity services

That reuses proven runtime seams without pretending that the global name registry itself is the durable truth.

### 6. Introduce keyed endpoints before removing `/work`

The docs, shell assertions, local Docker verifier, Fly verifier, and proof-surface verifier all currently speak in terms of `/work`.

For planning, the lowest-risk contract migration is:

1. add keyed submit/status endpoints on the same `cluster-proof` app
2. switch verifiers/docs to the new surfaces
3. only then retire or demote `/work` if desired

That keeps the proof surface honest while reducing blast radius from a single contract cutover.

### 7. Fly proof scope needs an explicit decision early

Current `scripts/verify-m039-s04-fly.sh` is deliberately read-only. It proves:

- app/config drift
- two-machine running-state expectation
- live `/membership`
- live `/work`
- matching logs

It does **not** kill a machine or force replica loss.

If M040’s final acceptance means “prove destructive node-loss continuity on Fly,” that is a planning decision, not just implementation work. Without that decision, the safest assumption is:

- destructive failover proof = local Docker authoritative surface
- Fly proof = steady-state/read-only operator truth

## What Should Be Proven First

Recommended proof order:

1. **Healthy-cluster keyed contract**
   - submit keyed work
   - status exposes owner/replica/phase
   - same-key retry converges idempotently on a healthy two-node cluster
2. **Fail-closed durability admission**
   - when required replica safety is missing, new durable submissions are visibly rejected
3. **Replica-backed node-loss continuity**
   - submit keyed work, lose owner, observe surviving continuity state, retry same key against survivor, converge correctly
4. **Automatic continuation when possible**
   - only after retry-safe baseline is proven
   - keep the first honest continuation mode narrow: restart-by-key using replicated continuity metadata
5. **Operator/docs/Fly alignment**
   - local Docker verifier
   - Fly verifier surface
   - README/docs/proof-page truth

That sequence matches the real risk curve: contract and truth surface first, failover second, polish last.

## Recommended Slice Boundaries

### Slice 1 — Keyed contract + local continuity state machine

Primary goal: retire unkeyed `/work` as the proof center without yet claiming replicated continuity.

Suggested content:

- extend `cluster-proof/main.mpl` with keyed submit/status routes
- refactor `cluster-proof/work.mpl` into a keyed continuity state machine or sibling module
- separate stable request key from attempt/execution identity
- reuse the existing service/state-map pattern (`WorkResultRegistry` style) for status tracking
- prove same-key idempotent convergence on a healthy cluster
- keep `/work` temporarily for transition if needed

Why first: this creates the public contract that every later slice has to prove.

### Slice 2 — Replica-backed continuity + failover policy

Primary goal: make the keyed contract honest under node loss.

Suggested content:

- add canonical owner/replica placement
- register continuity services globally for discovery
- add explicit replica-ack durability admission
- use node monitors / disconnect truth to drive failover policy
- prove owner loss + same-key retry convergence
- prove rejection of new durable work when replica policy is not met

Why second: this is the highest-risk runtime/app seam and should not be mixed with docs/operator churn.

### Slice 3 — Operator-path verification and docs truth

Primary goal: move the whole proof surface from `/work` to keyed continuity without drift.

Suggested content:

- extend Rust e2e harness artifact set for keyed submit/status
- extend shell assertion helpers in `scripts/lib/m039_cluster_proof.sh`
- update `scripts/verify-m039-s04.sh` local Docker proof
- update `scripts/verify-m039-s04-fly.sh` read-only live proof to keyed surfaces
- update `cluster-proof/README.md`, public docs, and proof-surface verifier

Why third: the blast radius is large, but mostly mechanical once the contract is stable.

### Optional late slice if required — Mutating Fly lifecycle proof

If roadmap planning insists on destructive Fly failover proof, reserve it as its own late slice. It changes operational risk and likely requires explicit approval because the current Fly proof is intentionally read-only.

## Requirement Analysis

### Table stakes

- **R049** is core M040 scope. The repo has no keyed work model today.
- **R050** is core M040 scope. There is no live-node replicated continuity ledger yet.
- **R052** remains binding even though it was born in M039. Any durability feature must fit the same one-image, env-driven operator path.
- **R053** remains binding. The current docs truth surface is already mechanically checked, so M040 must update docs/verifiers in lockstep.

### Important under-specification in the active requirements

The active set does **not** say clearly enough that new durable work must be rejected when replica safety is unavailable.

The milestone context says that explicitly, and the final acceptance depends on it, but the active requirements only imply it indirectly through R050 and R052 notes.

This gap matters because “survives node loss” and “refuses to accept fake-durable work when under-replicated” are different behaviors.

### Already-recorded constraints that should actively shape M040

These are currently out-of-scope/constraint items, but they are operationally active for this milestone:

- **R057** — do not drift into generic consensus-backed global app state
- **R058** — do not claim durability when no surviving replica exists
- **R059** — front-door spreading is not proof
- **R060** — Fly is not the architecture

In practice, M040 should treat these as hard design rails, not background notes.

### Launchability expectation that needs explicit handling

`cluster-proof/fly.toml` currently has `min_machines_running = 1`, while the Fly verifier requires two running machines. That was acceptable for M039’s read-only proof, but M040’s “default two-node safety” means the operator story must say more clearly:

- one machine may still run the app
- but durable mode cannot honestly accept new durable work without the configured replica safety

That behavior should be visible in the app contract, not just implied in docs.

## Candidate Requirements

These are research recommendations, not auto-binding scope changes.

### Candidate Requirement A — Fail-closed durable admission should be active, not just implied

Either promote **R058** into the active M040 set or add a new active requirement that says:

> when the configured replica safety is unavailable, the system visibly rejects new durable work instead of silently degrading into non-durable acceptance.

This is already in milestone acceptance and should be traceable as a first-class requirement.

### Candidate Requirement B — Keyed status must expose enough state to prove continuity mechanically

Current active requirements do not explicitly require the status surface to expose owner/replica/admission truth. Add an active requirement if the planner wants the proof contract to stay crisp.

Minimum likely fields:

- request key
- current phase/status
- owner node
- replica node(s)
- attempt/execution identifier or count
- durability/admission state or rejection reason

Without this, the proof will drift toward log-scraping.

### Candidate Requirement C — Do **not** create a checkpoint/replay obligation in M040

This should remain advisory unless the user explicitly wants it. The honest first-wave continuation model is still restart-by-key with replicated continuity metadata.

## Key Files For Follow-Up Planning

- `cluster-proof/main.mpl` — current route surface; submit/status lands here
- `cluster-proof/work.mpl` — narrowest existing seam for keyed continuity
- `cluster-proof/tests/config.test.mpl` — fail-closed env-contract pattern to extend for durability policy
- `cluster-proof/docker-entrypoint.sh` — container identity/operator contract
- `cluster-proof/fly.toml` — Fly contract that must stay honest
- `compiler/mesh-rt/src/dist/node.rs` — session lifecycle, `Node.list()`, remote spawn, disconnect behavior
- `compiler/mesh-rt/src/dist/global.rs` — replicated global discovery primitive, not a continuity ledger
- `compiler/mesh-rt/src/actor/mod.rs` — remote send path and `Node.monitor` ABI
- `mesher/ingestion/pipeline.mpl` — real in-repo use of `Node.monitor` and `Global.register`
- `compiler/meshc/tests/e2e_m039_s03.rs` — artifact and lifecycle proof shape to extend
- `scripts/lib/m039_cluster_proof.sh` — shared JSON assertion helpers that will need keyed-contract updates
- `scripts/verify-m039-s04.sh` — authoritative local Docker proof surface
- `scripts/verify-m039-s04-fly.sh` — current read-only live proof surface
- `cluster-proof/README.md` and `website/docs/docs/distributed-proof/index.md` — public truth surfaces that still describe `/work`

Milestone M040 researched.