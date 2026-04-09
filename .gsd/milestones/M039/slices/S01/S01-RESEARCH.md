# S01 Research — General DNS Discovery & Membership Truth

## Summary

S01 is a **runtime/discovery slice first**, not an application-feature slice first. The repo already has the hard distributed primitives: TLS + cookie-authenticated node transport, peer-list gossip after connect, global registry sync, `Node.self`, `Node.list`, `Node.spawn`, and `Node.monitor`. What it does **not** have is a general discovery subsystem, a periodic reconcile/reconnect loop, or a narrow proof surface that reports truthful membership without manual peer lists.

**Primary recommendation:** implement DNS discovery in `mesh-rt`, not in Mesh application code. Mesh programs currently have no DNS lookup API, while the runtime already owns node transport and connection lifecycle. Keep the first provider boring: **A/AAAA lookup of one discovery name plus a fixed cluster port**, resolved on an interval. Do **not** make Fly control-plane APIs or app-specific peer lists part of the architecture.

The critical constraint is that Mesh node identity and dial target are currently coupled. Peer gossip exchanges full node names and `handle_peer_list(...)` reconnects by feeding those strings back into `mesh_node_connect(...)`. That means S01 must ensure every node advertises a **unique, directly dialable address** in its own node name. A shared hostname like `my-app.internal` is acceptable as a discovery seed, but it is **not** a safe per-node identity.

## Relevant Requirements

- **R045** — primary slice owner. Establish a general auto-discovery seam with DNS as the first canonical provider.
- **R046** — primary slice owner. Make membership truth visible and reliable on join/loss/rejoin.
- **R052** — supported here. The operator surface needs to stay one-image + small env contract.
- **R053** — supported here, but full docs reconciliation belongs later in S04.
- **R047** — boundary only. S01 should not overbuild the work-routing proof; that is S02.

## Skills Discovered

- **Loaded:** `flyio-cli-public`
  - Relevant rule carried forward for later Fly verification: start with **read-only** Fly commands (`fly status`, `fly logs`, `fly config show`) and classify failures as build vs runtime vs platform before mutating anything.
- **Available and relevant but not newly loaded:** `rust-best-practices`
- **Skill search run:** `npx skills find "dns service discovery"`
  - Result: only unrelated security/recon skills surfaced; **no relevant DNS discovery skill was installed**.

## Existing Implementation Landscape

### Runtime primitives already present

- **`compiler/mesh-rt/src/dist/node.rs`**
  - Owns node identity, listener startup, TLS handshake, cookie auth, session table, peer-list exchange, `Node.self`, `Node.list`, and outbound connect.
  - Key functions/seams:
    - `send_peer_list(...)` — sends current connected peer names to a new session.
    - `handle_peer_list(...)` — parses peer names and reconnects by calling `mesh_node_connect(...)` with those exact strings.
    - `mesh_node_start(...)` — starts the local node.
    - `mesh_node_connect(...)` — opens one TCP/TLS connection to a parsed `name@host:port` target.
    - `cleanup_session(...)` / `handle_node_disconnect(...)` — remove disconnected peers and fire local failure signals.
    - `mesh_node_self()` / `mesh_node_list()` — current membership query surface.
  - Important current behavior: on successful connect, the runtime already does **peer-list gossip** and **global registry sync**.

- **`compiler/mesh-rt/src/dist/global.rs`**
  - Fully replicated global registry with cleanup on process exit and node disconnect.
  - Useful for service lookup, but **not** a substitute for truthful membership. Membership truth should come from actual sessions (`Node.self` + `Node.list`), not registry contents.

- **`compiler/mesh-rt/src/actor/mod.rs`**
  - Owns `mesh_node_monitor(...)` and delivery of `:nodeup` / `:nodedown` events to monitoring processes.
  - This is already the right runtime surface for truthful join/loss notifications once discovery is automatic.

- **`compiler/mesh-rt/src/db/pg.rs`**
  - Already uses `ToSocketAddrs` in the Postgres path. That is useful precedent: S01 can likely reuse stdlib DNS resolution for the first provider instead of adding a new DNS dependency immediately.

### Current app-side distributed story

- **`mesher/main.mpl`**
  - Reads `MESHER_NODE_NAME`, `MESHER_COOKIE`, and `MESHER_PEERS`.
  - Starts distribution if name+cookie are present.
  - Only supports a **manual single peer seed** via `MESHER_PEERS`.
  - This is proof that env-driven startup works, but it is not a general discovery design.

- **`mesher/api/helpers.mpl`**
  - Good example of a narrow cluster-aware seam: `Node.self()` chooses between `Global.whereis(...)` and `Process.whereis(...)`.
  - Useful style reference for S01 proof-surface branching.

- **`mesher/ingestion/pipeline.mpl`**
  - Prior dogfood integration: `Node.list`, `Node.monitor`, `Node.spawn`, and `Global.register` are already used here.
  - Valuable as prior art for membership monitoring and globally-registered service patterns.
  - Not a good final proof surface for M039 because the milestone explicitly wants a **new narrow proof app**, not more Mesher retrofitting.

### Public claims and prior art

- **`website/docs/docs/distributed/index.md`**
  - Documents manual `Node.start(...)` + `Node.connect(...)`, `Node.self`, `Node.list`, `Global.register`, and `Node.monitor`.
  - It does **not** document general auto-discovery yet.

- **`.planning/phases/94-multi-node-clustering/94-RESEARCH.md`** and **`94-VERIFICATION.md`**
  - Strong prior art for what already worked in Mesher:
    - manual seed + runtime auto-gossip
    - global service registration
    - `Node.monitor` for peer loss
    - cluster-aware handler lookup
  - Also documents useful pitfalls: raw pointer locality, startup ordering, and the earlier race between `Node.connect` and registry sync.
  - M039 should reuse the runtime truths, but **not** reuse Mesher as the flagship proof app.

### Fly-specific operational truth already in repo

- **`benchmarks/fly/run-benchmarks-isolated.sh`**
  - Explicit repo note: `*.vm.<app>.internal` can have propagation delays inside Fly Machines; the benchmark prefers direct private IPv6 when it needs authoritative connectivity.
- **`benchmarks/fly/README.md`**
  - Shows current repo familiarity with Fly private DNS hostnames and direct private IPv6 usage.

## Critical Findings

### 1. Node identity is currently also the dial target

This is the most important design constraint in S01.

`mesh_node_connect(...)` parses a `name@host:port` string, dials `host:port`, then the handshake learns the peer’s real `remote_name`. Later, `send_peer_list(...)` and `handle_peer_list(...)` exchange and reuse those **real node names** as future connection targets.

Implication:
- discovery seeds can be temporary/synthetic connect strings
- **advertised node names cannot be synthetic**
- a node’s own `Node.start(name, cookie)` value must include a host that other peers can dial directly later

That rules out using a shared hostname like `my-app.internal` as each node’s stable identity on Fly. It is fine as a seed DNS name, but not as the runtime node name that peers will gossip.

### 2. The current runtime has no discovery or reconnect loop

Today the cluster only grows after:
- manual startup of the local node
- manual `Node.connect(...)` from the app
- peer-list gossip after at least one successful connection

There is no runtime-owned process that:
- resolves DNS periodically
- attempts connections to newly appeared candidates
- retries after node restart/rejoin
- reconciles current sessions against newly discovered addresses

S01 needs that loop or later slices will be forced to fake join/rejoin with manual peer lists.

### 3. Mesh application code cannot currently perform DNS lookup

Repo search found no Mesh stdlib DNS/host lookup surface. That makes an app-only discovery implementation a dead end unless S01 also adds a new stdlib networking API.

That is unnecessary scope. The right seam is runtime-side discovery inside `mesh-rt`, with a small env/config contract exposed to the proof app.

### 4. `Node.list()` is peer-only truth, not cluster-size truth by itself

`mesh_node_list()` returns the currently connected remote node names only. It does **not** include the local node.

So the truthful membership surface in S01 must report at least:
- `self`
- `peers`
- `membership = [self] + peers`
- optionally `connected_peer_count`

If the proof endpoint only returns `Node.list()`, it will under-report by one and confuse later slice logic.

### 5. Fly DNS is good for discovery candidates, not for stable identity

Current Fly docs confirm:
- `FLY_PRIVATE_IP` exists in the machine runtime environment
- `FLY_MACHINE_ID` exists
- `my-app.internal` AAAA returns the 6PN addresses of **started** Machines only
- `.internal` DNS is already the right general discovery surface on Fly

This strongly suggests the right Fly mapping is:
- **advertised node name:** something like `<machine-id>.<region>@[<FLY_PRIVATE_IP>]:<cluster-port>`
- **discovery seed name:** `${FLY_APP_NAME}.internal` (or `global.${FLY_APP_NAME}.internal`)

That keeps the architecture DNS-based and general while avoiding fragile app-internal hostnames as identity.

### 6. S01 probably does not need SRV/TXT discovery to be honest

A fixed cluster port plus A/AAAA lookup is enough for the first provider.

Why:
- the milestone only requires DNS as first provider, not SRV sophistication
- Fly already returns all started machine IPv6 addresses via AAAA
- region and machine identity can come from env and be embedded into the node name
- the operator surface stays smaller if port is explicit config instead of DNS-derived metadata

TXT/SRV support may still be useful later, but it looks like overbuild for S01.

## Recommendation

### Recommended architecture for S01

1. **Add a runtime discovery seam in `mesh-rt`**
   - Prefer a new focused module under `compiler/mesh-rt/src/dist/` rather than growing all logic inside `node.rs`.
   - Keep ownership with the runtime, because the runtime already owns sessions, peer gossip, cleanup, and monitor delivery.

2. **Make the provider return socket addresses, not node names**
   - The discovery provider should resolve candidate dial addresses.
   - Outbound connect attempts can synthesize a temporary target string for `mesh_node_connect(...)`; the handshake will replace it with the peer’s real advertised node name.
   - This avoids coupling discovery to remote identity conventions.

3. **Use DNS A/AAAA lookup + fixed port as the first canonical provider**
   - Reuse host/system resolver behavior rather than adding a DNS stack unless the implementation proves inadequate.
   - The missing behavior is not “can Rust do DNS,” it is “collect all addresses and reconcile them periodically.”

4. **Separate bind/advertise identity conceptually, even if the first API remains env-driven**
   - The runtime name must advertise a unique dialable host.
   - On Fly, default to `FLY_PRIVATE_IP` for the advertised/bind host and `FLY_MACHINE_ID` (+ `FLY_REGION` if desired) for the name part.
   - For IPv6 literals, keep bracketed form in the node name string.

5. **Build a narrow membership endpoint in the new proof app, but stop there**
   - S01’s proof surface should answer “who am I and who do I think is in the cluster?”
   - Do not pull S02’s ingress-vs-execution routing proof into this slice.

## Natural Seams For Planning

### Seam A — runtime discovery core

**Likely files:**
- `compiler/mesh-rt/src/dist/node.rs`
- new `compiler/mesh-rt/src/dist/discovery.rs` (recommended)
- `compiler/mesh-rt/src/dist/mod.rs`
- possibly `compiler/mesh-rt/src/lib.rs` if new externs/config helpers are exposed

**What belongs here:**
- discovery config struct / env-driven bootstrap inputs
- DNS resolution of all candidate addresses
- dedupe/self-filter against current node + current sessions
- interval-based reconcile loop
- connect-attempt policy and logging

**Why first:** this is the real blocker. Until runtime-owned discovery exists, the proof app can only fake automatic clustering.

### Seam B — node identity / operator contract

**Likely files:**
- new proof app bootstrap (new directory not yet present)
- possibly `mesher/main.mpl` as a reference pattern only
- maybe small runtime helpers if env defaults are standardized

**What belongs here:**
- cluster port env
- cookie env
- discovery mode/provider env
- discovery DNS name env
- advertise host / node id defaults for Fly

**Key design rule:** keep the contract small and image-friendly. S01 should not turn “run one image” into “assemble a bespoke cluster manifest.”

### Seam C — truthful membership surface

**Likely files:**
- new proof app HTTP entrypoint/handler files

**What belongs here:**
- endpoint that returns `self`, `peers`, combined membership, and maybe last discovery time / discovery source
- no routing proof yet

**Why separate from runtime:** runtime should own transport truth; the proof app should only present it.

### Seam D — verification harness

**Likely files:**
- new shell verifier(s) under `scripts/`
- new runtime or compiler e2e tests if a mesh proof fixture is added

**What belongs here:**
- local multi-node proof without manual peer lists
- deterministic wait-for-convergence logic
- explicit membership assertions

## Verification Strategy

### What existing tests do cover

Current `mesh-rt` tests cover:
- handshake success/failure
- full connect lifecycle in-memory / localhost style
- peer-list wire format parsing
- `mesh_node_self()` / `mesh_node_list()` basic validity

This is useful baseline protection, but it is **not** authoritative S01 proof.

### What S01 needs as authoritative proof

1. **Runtime-level test coverage**
   - candidate resolution returns multiple addresses
   - self-address is filtered
   - already-connected peers are filtered
   - reconnect/reconcile loop ignores duplicates cleanly
   - bracketed IPv6 advertised names parse and round-trip correctly

2. **Local multi-node proof**
   - start 3 identical nodes with the same discovery DNS name and cookie
   - no manual peer list anywhere
   - assert each node converges to truthful membership (`self + 2 peers`)
   - assert membership truth updates after a node disappears

3. **Fly proof preparation**
   - later slices can deploy, but S01 should shape the contract so Fly verification is straightforward:
     - inside a Machine, DNS lookup of `${FLY_APP_NAME}.internal` yields started node candidates
     - node names advertise `FLY_PRIVATE_IP`
     - membership endpoint shows `FLY_MACHINE_ID`-derived identity

### Fly verification guidance from the loaded Fly skill

When the milestone reaches real Fly runs, follow the skill’s rule:
- start with **read-only** checks (`fly status`, `fly logs`, `fly config show`)
- then use `fly ssh console -C "..."` for in-machine truth gathering
- do not jump straight to deploy/scale mutations while debugging discovery

## Risks / Planner Watchouts

- **Identity bug risk:** if nodes advertise a shared hostname instead of a unique dialable address, peer gossip and rejoin will be dishonest or flaky.
- **Scope creep risk:** adding SRV/TXT/generalized DNS metadata support in S01 is likely overbuild if fixed-port A/AAAA discovery is sufficient.
- **False-proof risk:** using Mesher again as the flagship proof surface will bury the slice under unrelated product behavior.
- **Membership lie risk:** using `Global.whereis(...)` or discovery candidates as “cluster membership” is softer truth than actual live sessions.
- **Fly hostname risk:** repo evidence already shows `*.vm.<app>.internal` can lag enough to be a bad authoritative connectivity surface inside Machines.

## Don’t Hand-Roll

- **Do not** make Fly’s API/control plane the architecture. Use DNS as the provider seam, consistent with D125.
- **Do not** use a shared multi-address hostname as a node’s own advertised runtime identity.
- **Do not** overbuild the first provider with SRV/TXT if A/AAAA + fixed port satisfies the slice.
- **Do not** count front-door spread or load balancer behavior as membership or clustering proof.
- **Do not** extend Mesher again as the primary M039 proof app; the milestone context explicitly wants a narrow separate proof surface.

## Sources

### Codebase
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/dist/global.rs`
- `compiler/mesh-rt/src/actor/mod.rs`
- `compiler/mesh-rt/src/db/pg.rs`
- `mesher/main.mpl`
- `mesher/api/helpers.mpl`
- `mesher/ingestion/pipeline.mpl`
- `website/docs/docs/distributed/index.md`
- `.planning/phases/94-multi-node-clustering/94-RESEARCH.md`
- `.planning/phases/94-multi-node-clustering/94-VERIFICATION.md`
- `benchmarks/fly/run-benchmarks-isolated.sh`
- `benchmarks/fly/README.md`

### External docs
- Fly private networking / `.internal` DNS: https://fly.io/docs/networking/private-networking/
- Fly machine runtime environment (`FLY_PRIVATE_IP`, `FLY_MACHINE_ID`): https://fly.io/docs/machines/runtime-environment/
- Fly app service / 6PN routing notes: https://fly.io/docs/networking/app-services/
- Hickory resolver docs (only if richer DNS records become necessary): https://docs.rs/hickory-resolver/latest/hickory_resolver/
