# S02 Research — Native Cluster Work Routing Proof App

## Summary

S02 is the **app-level proof slice for R047**, not another runtime/discovery slice. S01 already left the right substrate in place: `cluster-proof/` builds cleanly, the two-node convergence gate is green, and the proof app already exposes truthful membership from live runtime sessions. The shortest honest S02 is to **extend the same proof app with one work-routing endpoint** that shows `ingress_node` and `execution_node` separately while preserving `/membership` unchanged for diagnostics.

The cleanest first proof is **direct `Node.spawn` per request**, not a globally registered worker service. That matches the public distributed claim directly, avoids dragging lifecycle/global-registry concerns from later slices into S02, and makes the local proof undeniable: the Rust harness can hit node A’s HTTP port directly and still assert that node B executed the work. Front-door spread cannot fake that.

Two implementation constraints matter:

1. **Keep the response payload boring and typed.** `cluster-proof/cluster.mpl` already established that the stable Mesh HTTP payload shape is a concrete `struct ... deriving(Json)` with string/list fields. Avoid inline JSON cleverness or handler-local `Ok(n)` result binding.
2. **Do not ship local handles across nodes.** Mesher’s existing `Node.spawn` dogfood already established the right rule: pass only serializable values (strings, ids, pids), and rehydrate any local state on the execution node.

## Relevant Requirements

- **R047 — primary owner.** S02 is the slice that must prove runtime-native internal balancing by making ingress and execution visibly different on real requests.
- **R046 — supporting input only.** S02 should reuse live membership truth (`Node.self()` + `Node.list()`) when choosing or reporting execution targets, but it does not own rejoin/loss behavior.
- **Boundary only:**
  - **R048** belongs to S03. Do not pull failure/rejoin proof into S02.
  - **R052** and **R053** belong to S04. Do not grow the env/operator/doc surface in this slice unless absolutely forced.

## Skills Discovered

- **Loaded repo-local Mesh skills:**
  - `tools/skill/mesh/SKILL.md`
  - `tools/skill/mesh/skills/http/SKILL.md`
  - `tools/skill/mesh/skills/actors/SKILL.md`
- **Rules carried forward from those loaded skills:**
  - Mesh HTTP: keep handler shape as `fn handler(request) do ... HTTP.response(...) end`, wire routes with `HTTP.router()` + `HTTP.on_get(...)`, and keep routing logic explicit rather than magical.
  - Mesh actors: use the standard request/reply pattern by passing the caller PID into the spawned actor and sending the reply back explicitly; one-shot actors terminate naturally unless they loop.
- **Installed/global skill already present but not core to S02:** `flyio-cli-public` (later S04/Fly work, not this slice).
- **Skill search run:** `npx skills find "Mesh language distributed HTTP actors"`
  - Result: no relevant public Mesh skill surfaced; **no new skill was installed**.

## Current Baseline

Confirmed during research:

- `cargo run -q -p meshc -- build cluster-proof` ✅
- `cargo test -p meshc --test e2e_m039_s01 e2e_m039_s01_converges_without_manual_peers -- --nocapture` ✅

That means the planner can treat S01 as a live working baseline and extend the existing proof surface instead of spending a task revalidating discovery first.

## Existing Implementation Landscape

### `cluster-proof/` app surface

- **`cluster-proof/main.mpl`**
  - Owns startup, runtime boot, logging, and router wiring.
  - Current HTTP surface is only `GET /membership`.
  - Startup logs already expose the stable node identity, discovery provider/seed, and HTTP/cluster ports without echoing the cookie.
  - This file is the right place for **route-table changes only**. Keep startup logic stable if possible.

- **`cluster-proof/cluster.mpl`**
  - Owns the current typed membership payload.
  - Derives membership from `Node.self()` + `Node.list()` and explicitly includes self.
  - Uses a concrete `MembershipPayload` struct plus string-backed port fields, then renames `node` -> `self` in the JSON output.
  - Good place for shared membership snapshot helpers, but **not** a great place to bury actor/routing logic unless that logic stays tiny.

- **`cluster-proof/config.mpl`**
  - Pure env/identity contract for cluster vs standalone startup.
  - Already covered by pure Mesh tests under `cluster-proof/tests/config.test.mpl`.
  - S02 should avoid expanding this surface unless a new knob is truly necessary. R052 belongs later.

### Runtime seams already available

- **`compiler/mesh-rt/src/http/server.rs`**
  - Each accepted HTTP connection is dispatched onto the Mesh actor scheduler via `sched.spawn(connection_handler_entry, ...)`.
  - That means the request handler runs inside actor-backed execution with crash isolation.
  - This is the right substrate for a handler doing request/reply actor work, but the repo does **not** yet have an app-level proof that `self()` / `receive` / `Node.spawn` behave correctly inside a handler. S02 should prove that directly.

- **`compiler/mesh-rt/src/dist/node.rs`**
  - `mesh_node_spawn(...)` is the direct runtime seam behind `Node.spawn(node, fn, args...)`.
  - Important behavior:
    - target node must already be in the live session table
    - function name is sent by name, args are packed as raw i64 values
    - caller blocks until the remote node replies with the spawned PID
  - This is already the correct primitive for S02’s honest work-routing proof.

- **`website/docs/docs/distributed/index.md`**
  - Public docs already claim:
    - `Node.spawn` returns a PID valid across nodes
    - normal `send` / `receive` semantics work across the cluster
    - `Global.register` / `Global.whereis` provide cluster-wide named process lookup
  - S02 should prove the `Node.spawn` claim at the app level instead of inventing a more abstract service first.

### Dogfood prior art worth reusing conceptually

- **`mesher/ingestion/pipeline.mpl`**
  - Uses `Node.spawn(target, event_processor_worker)` for remote work.
  - Important comment/rule already encoded there: **do not send local handles across nodes**; rehydrate node-local state inside the remote worker via `Process.whereis(...)`.
  - For S02, the proof worker probably needs no rehydration beyond `Node.self()`, but the pattern is still the right one.

- **`mesher/api/helpers.mpl`**
  - Good example of a narrow cluster-aware seam: `Node.self()` switches between `Global.whereis(...)` and `Process.whereis(...)`.
  - Useful style reference if S02 needs a small helper to branch between clustered and standalone behavior.

### Existing verification harness to reuse

- **`compiler/meshc/tests/e2e_m039_s01.rs`**
  - Already owns the real two-node local cluster harness.
  - Useful helpers already exist for:
    - repo-root resolution
    - `meshc build cluster-proof`
    - dual-stack-safe cluster port selection
    - per-node process spawning with env wiring
    - durable stdout/stderr log capture
    - direct raw HTTP GETs via `TcpStream`
    - convergence waits and JSON parsing
  - This is the single biggest reuse seam for S02. Do not rebuild a second cluster harness unless isolation materially simplifies the slice.

- **`scripts/verify-m039-s01.sh`**
  - Canonical verifier pattern already exists:
    - build app
    - run named tests in order
    - record `.tmp/.../verify/phase-report.txt`
    - fail closed on zero-test filters or missing `running N test`
  - S02 should clone this structure into a **new** `scripts/verify-m039-s02.sh`, not mutate the S01 gate.

## Critical Findings

### 1. The smallest honest S02 proof is a request → remote one-shot actor → reply loop.

A long-lived globally registered worker service is possible, but it is not required to prove R047. The runtime and docs already advertise `Node.spawn`; using it directly keeps the slice narrow and avoids accidental drift into lifecycle/rejoin concerns that belong to S03.

### 2. The HTTP proof must bypass any external front door.

The harness should connect to node A’s localhost HTTP port directly and assert that the response says `ingress_node == node A` while `execution_node == node B` (or at least `!= ingress_node`) when a peer exists. That is the core honesty check for runtime-native balancing.

### 3. The response payload should stay on the same “safe” Mesh shape S01 established.

`.gsd/KNOWLEDGE.md` already records that naive helper encoding/interpolation in `cluster-proof` can miscompile into misaligned-pointer crashes or garbage port values. Keep the S02 response as a concrete derived struct with mostly string/bool/list fields.

### 4. Avoid integer result-binding shapes in the HTTP handler.

The existing repo knowledge from Mesher is directly relevant: matching `Ok(n)` on `Int ! String` inside an HTTP handler can still crash the live route even when the underlying mutation succeeds. If the handler needs a timeout/failure surface, return string/bool markers rather than integer success payloads.

### 5. Handler actor semantics are runtime-backed, but S02 is the first place the repo proves them at the app layer.

`mesh_http_serve` dispatches connections through the actor scheduler, so `self()` and `receive` are plausible inside request handling. Still, the proof does not exist yet in the repo’s app-level acceptance surface. Treat that as the main integration risk and make it the first task to retire.

## Recommendation

### App shape

- **Keep `/membership` unchanged.** S01 and S03 depend on it as the truthful membership diagnostic surface.
- **Add one new proof endpoint** (planner can name it; `/work` or `/route` are both reasonable) that returns at least:
  - `ingress_node`
  - `execution_node`
  - `target_node`
  - `routed_remotely` (bool)
  - enough membership context (`peers` or `membership`) to explain target choice
  - a stable request/proof id for log correlation

### Execution strategy

Use **direct `Node.spawn` with a one-shot actor**:

- The HTTP handler captures:
  - `ingress_node = Node.self()`
  - `membership = Node.self() + Node.list()`
  - a deterministic `target_node`
  - `reply_to = self()`
  - `request_id`
- It then either:
  - `Node.spawn(target_node, execute_work, reply_to, ingress_node, request_id)` when clustered and a peer exists, or
  - local `spawn(execute_work, reply_to, ingress_node, request_id)` / direct local fallback in standalone or single-node mode
- The one-shot actor immediately computes `execution_node = Node.self()`, logs a durable proof line, and `send`s a typed reply back to `reply_to`
- The handler waits with `receive do ... after <timeout> -> ... end` and returns a typed JSON response

### Target selection policy

Use a **deterministic peer-preferred policy**, not random routing.

Recommended first rule:
- if peers exist, choose a stable peer from sorted membership that is **not self**
- otherwise fall back to self

Why this rule is good for S02:
- with two nodes it guarantees remote execution on every healthy clustered request, making the proof crisp
- with more nodes it still stays deterministic and debuggable
- it avoids overbuilding balancing heuristics that do not matter to the proof slice

### Response/logging surface

- Keep the response contract typed and string-heavy.
- Add a per-execution log line on the **execution node** (for example: `[cluster-proof] work executed request_id=... ingress=... execution=... target=...`) so the Rust harness can assert both JSON truth and log truth.
- Add explicit timeout/failure responses rather than hanging the HTTP connection if the remote reply never arrives.

## Natural Task Seams

### 1. Proof-app routing module

Primary risk-retirement task.

Files:
- `cluster-proof/main.mpl`
- `cluster-proof/cluster.mpl` (if shared snapshot helpers belong there)
- **recommended new file:** `cluster-proof/work.mpl`

Scope:
- add new route
- implement target selection helper
- implement one-shot execution actor
- implement typed response/reply structs
- add explicit timeout/error response surface

### 2. Rust e2e proof extension

Files:
- `compiler/meshc/tests/e2e_m039_s01.rs` **or**
- **recommended new file:** `compiler/meshc/tests/e2e_m039_s02.rs`

Recommendation:
- prefer a **new test file** if keeping S01 acceptance totally untouched is cheap
- otherwise reuse S01 helpers in-place if duplication would be worse than shared maintenance

Scope:
- reuse existing spawn/port/log helpers
- hit a specific node’s HTTP port directly
- assert ingress/execution truth in JSON
- assert the execution log line lands in the correct node’s stdout log
- add a standalone/local-fallback case if cheap

### 3. Canonical slice verifier

File:
- **new file:** `scripts/verify-m039-s02.sh`

Scope:
- mirror S01 verifier structure
- new artifact root: `.tmp/m039-s02/verify/`
- fail closed on missing or zero test counts
- keep S01 verifier unchanged

## Concrete File Guidance for the Planner

- **`cluster-proof/main.mpl`**
  - Safe for route wiring and very small orchestration only.
  - Do not let it accumulate the actor protocol and payload-shaping details.

- **`cluster-proof/cluster.mpl`**
  - Keep existing `/membership` contract stable.
  - If you need a shared helper like `current_membership()` or `current_peers()`, this file is a reasonable home.
  - Do not bury the whole work-routing implementation here unless it stays tiny.

- **`cluster-proof/config.mpl`**
  - Avoid touching unless absolutely necessary.
  - S02 should not grow the env contract just for debugging convenience.

- **`cluster-proof/work.mpl`** *(recommended new file)*
  - Best place for:
    - routing-proof structs
    - target selection helper
    - one-shot execution actor
    - handler-side request/reply orchestration helpers

- **`compiler/meshc/tests/e2e_m039_s01.rs`**
  - Already has the right low-level helpers.
  - Reuse if a new file would just duplicate 100+ lines of spawn/log/HTTP plumbing.

- **`compiler/meshc/tests/e2e_m039_s02.rs`** *(recommended if isolation is cheap)*
  - Better if the planner wants the slice acceptance surface to stay clearly separate from S01.

- **`scripts/verify-m039-s02.sh`**
  - Should be the new authoritative local replay wrapper for S02, built from the same fail-closed pattern as S01.

## Verification Strategy

Minimum honest proof surface:

- `cargo run -q -p meshc -- build cluster-proof`
- existing S01 named convergence proof still passes
- new named S02 e2e(s) that:
  1. start two nodes with the existing harness
  2. request node A’s proof endpoint directly
  3. assert `ingress_node == expected_a`
  4. assert `execution_node == expected_b` (or at least `!= ingress_node`) when peers exist
  5. request node B directly and assert the symmetric result
  6. inspect per-node stdout logs and confirm the execution log line appears on the execution node’s log
- `bash scripts/verify-m039-s02.sh`

Good optional isolation proof if the handler path is flaky:

- a smaller e2e/fixture that proves `HTTP handler -> self() -> Node.spawn(...) -> receive reply` outside the full `cluster-proof` response surface

## De-scope / Do Not Build Here

- Fly deployment or Fly verification
- one-image operator docs or env cleanup beyond what is strictly necessary
- node-loss/rejoin proof
- globally registered worker services unless direct `Node.spawn` turns out impossible
- any claim that front-door load spread counts as evidence

## Planner Call

Build S02 as a **narrow extension of `cluster-proof`**, not a new distributed subsystem:

1. keep `/membership` untouched
2. add one new work-routing endpoint
3. implement it with direct `Node.spawn` + request/reply actor messaging
4. prove it through the existing two-node harness with direct-port requests and per-node log assertions
5. add a new fail-closed verifier wrapper for the slice
