# M054 Research — Load Balancing Truth & Follow-through

## Milestone fit

M054 is not starting from zero. The repo already has a real runtime-owned internal routing path for clustered work and clustered HTTP routes, a serious PostgreSQL starter, runtime-owned operator surfaces, and hosted replay of the starter failover rail. The gap is that these pieces do not yet add up to one clean public answer for evaluators asking: "If my frontend hits one public app URL, where does balancing actually happen?"

The current tree is closest to this honest model:

- **public ingress balancing is platform/proxy behavior**
- **Mesh runtime-owned placement begins after a request lands on one cluster node**
- **Mesh can remote-dispatch clustered work/clustered HTTP handlers to the chosen owner and expose owner/replica/execution truth through operator surfaces**
- **the serious starter proves that locally and in hosted CI, but not yet on a live public one-URL ingress surface**

That means M054 should start by proving or disproving the current server-side-first story on the shipped PostgreSQL starter before it changes runtime architecture.

## Skills Discovered

Relevant installed skills already present:

- `flyio-cli-public` — relevant for Fly app inspection and deployment/runbook constraints
- `github-workflows` — relevant for hosted proof-chain wiring
- `vitepress` — relevant for docs-surface placement and guardrails

Additional skill search run:

- `npx skills find "Fly Proxy"` — no directly relevant Fly ingress/load-balancing skill surfaced; returned generic proxy/security tools only

No new skills were installed.

## Executive summary

### What exists today

1. **Public claim surface is stronger than the explicit proof surface.**
   - `website/docs/index.md` still says: **"Built-in failover, load balancing, and exactly-once semantics — no orchestration layer required."**
   - `mesher/landing/app/mesh/page.tsx` still says: **"Failover, load balancing, clustering: all first-class."**
   - I found no repo-owned docs contract test or verifier that watches either of those claim strings.

2. **The serious PostgreSQL starter is intentionally narrower than that claim.**
   - `examples/todo-postgres/api/router.mpl` uses `HTTP.clustered(1, handle_list_todos)` and `HTTP.clustered(1, handle_get_todo)` only on `GET /todos` and `GET /todos/:id`.
   - `GET /health` and all mutating routes stay local.
   - `examples/todo-postgres/README.md` presents staged deploy + clustered operator inspection, but intentionally omits deeper failover wording.
   - `compiler/meshc/tests/e2e_m053_s02.rs` pins that bounded README/work contract and explicitly forbids turning the example README into a failover/operator deep dive.

3. **Runtime-owned clustered route execution is real, but the semantics are narrower than a generic "load balancer" story.**
   - `compiler/mesh-rt/src/http/server.rs` lowers clustered HTTP routes into `execute_clustered_http_route(...)`.
   - `compiler/mesh-rt/src/dist/node.rs::declared_work_placement(...)` chooses an owner by **stable hashing** `request::<request_key>` across canonical membership.
   - For clustered HTTP, `compiler/mesh-rt/src/http/server.rs::build_clustered_http_route_identity(...)` creates request keys as `http-route::<runtime>::<seq>` using a **per-process monotonic counter**.
   - This is **not load-aware**, **not sticky-session-aware**, and **not client-topology-aware**. It is deterministic runtime-owned owner selection for each generated request key.

4. **Operator truth is already good enough to explain ingress vs execution — but not yet easy enough to correlate from a real browser request.**
   - `meshc cluster continuity --json` includes `ingress_node`, `owner_node`, `replica_node`, `execution_node`, `declared_handler_runtime_name`, `routed_remotely`, and `fell_back_locally`.
   - This directly supports the existing honesty rule from `R047`: public proof must distinguish ingress-node truth from execution-node truth.
   - But clustered HTTP requests do **not** expose their continuity request key in the HTTP response today. Existing route tests discover the new request by diffing continuity lists before and after a request.

5. **The current proof chain proves serious starter runtime behavior, not live public ingress behavior.**
   - `scripts/verify-m053-s02.sh` and `compiler/meshc/tests/e2e_m053_s02.rs` prove the generated PostgreSQL starter under a two-node staged replay with real CRUD, clustered GETs, promotion/recovery, stale-primary fencing, and retained operator artifacts.
   - The retained bundle includes `route-record-primary.json`, `route-record-standby.json`, pre/post-kill status/continuity/diagnostics, and `scenario-meta.json`.
   - `authoritative-starter-failover-proof.yml` reruns that rail in GitHub Actions with runner-local Postgres.
   - This is strong freshness evidence, but it is **not** a proof that a real public one-URL ingress layer is wired to the starter.

6. **Fly remains secondary and reference-only for the older retained proof fixture, not the serious starter.**
   - `scripts/verify-m043-s04-fly.sh` is explicitly a **read-only Fly verifier for the retained `cluster-proof` reference rail**.
   - `scripts/fixtures/clustered/cluster-proof/fly.toml` belongs to the retained fixture, not `examples/todo-postgres`.
   - The serious starter intentionally contains no Fly-specific surface; scaffold tests in `compiler/mesh-pkg/src/scaffold.rs` explicitly assert that Fly guidance is absent.

7. **There is no repo implementation of Fly-specific request replay today.**
   - I found no `Fly-Replay` usage anywhere in the repository.
   - Current Fly docs (via Context7 `/superfly/docs`) say Fly Proxy distributes requests using closeness, concurrency settings, and current load, and that `Fly-Replay` is an app-emitted header used when the app wants Fly to reroute to a specific machine.
   - So Fly can plausibly serve as the public one-URL ingress layer, but Mesh does not currently emit Fly-specific reroute hints.

### Working conclusion

The current shipped story is **not** "Mesh itself is a public-edge load balancer." The honest current story is closer to:

- one public app URL can be fronted by a platform/proxy such as Fly Proxy
- that platform chooses an **ingress node**
- Mesh runtime may then choose a different **owner/execution node** for clustered work or clustered read routes
- Mesh operator surfaces can tell you which node accepted, owned, replicated, and executed the work

That is a viable platform-agnostic, server-side-first story.

What is missing is:

- a clean public explanation of those boundaries
- proof of the serious starter behind one public ingress URL
- an operator-friendly way to correlate one HTTP request to one continuity record without pre/post diff gymnastics
- guardrails on the strongest public marketing copy

## Existing code and proof surfaces that matter

### Runtime surfaces worth reusing

- `compiler/mesh-rt/src/http/server.rs`
  - clustered route identity generation
  - clustered route local/remote execution path
- `compiler/mesh-rt/src/dist/node.rs`
  - canonical membership normalization
  - stable-hash owner placement
  - declared handler registration metadata and required replica count
- `compiler/mesh-rt/src/dist/continuity.rs`
  - durable continuity records carrying ingress/owner/replica/execution truth
- `compiler/mesh-rt/src/dist/operator.rs`
  - runtime diagnostics/status/continuity snapshots
- `compiler/meshc/src/cluster.rs`
  - public operator CLI JSON surface already exposes the right fields

### Starter/public surfaces worth reusing

- `examples/todo-postgres/README.md`
- `examples/todo-postgres/main.mpl`
- `examples/todo-postgres/api/router.mpl`
- `examples/todo-postgres/api/todos.mpl`
- `examples/todo-postgres/api/health.mpl`
- `compiler/mesh-pkg/src/scaffold.rs`

### Proof rails worth reusing

- `scripts/verify-m053-s02.sh` — strongest current serious-starter runtime/failover bundle
- `.github/workflows/authoritative-starter-failover-proof.yml` — hosted freshness replay for the same rail
- `compiler/meshc/tests/e2e_m047_s07.rs` — authoritative clustered-route runtime seam (default count, record shape, unsupported counts)
- `scripts/verify-m043-s04-fly.sh` — useful as a pattern for a read-only live-environment verifier, but currently tied to retained `cluster-proof`

## Constraints imposed by the current codebase

### 1. The starter README is intentionally bounded

`compiler/meshc/tests/e2e_m053_s02.rs` asserts that `examples/todo-postgres/README.md` contains operator commands and route references but omits deep failover language like `automatic_promotion`, `automatic_recovery`, `fenced_rejoin`, and direct verifier references.

Implication: if M054 needs a deeper balancing explanation, it probably belongs on **Distributed Proof** (or a new tightly scoped public-secondary page) with only a bounded handoff from starter README/docs.

### 2. Public clustered proof is not supposed to depend on app-owned HTTP proof routes

`R092` explicitly says the public clustered story no longer depends on HTTP routes for proof or operator truth. The current serious starter already dogfoods clustered GET routes, but the standard operator truth is still supposed to live in runtime/CLI surfaces.

Implication: M054 should not regress into adding package-owned admin endpoints as the main explanation surface.

### 3. Route replication support is currently bounded

- `HTTP.clustered(1, ...)` => required replicas = 0
- default `HTTP.clustered(handler)` => replication count 2 in current runtime/test seam
- counts above 2 are currently rejected as `unsupported_replication_count:N`

Implication: avoid promising generalized multi-node replicated route execution or sophisticated load-aware route scheduling.

### 4. Public ingress proof is currently absent from the serious starter path

The serious starter has no Fly config, no Fly deploy guide, no Fly replay logic, and no live public URL verifier. Hosted proof is GitHub Actions plus runner-local Postgres.

Implication: if M054 wants a real one-public-URL proof, it will need **new proof wiring**, not just docs edits.

### 5. Request correlation is weak for clustered HTTP

Clustered route request keys are runtime-generated internal IDs and are not returned in the HTTP response. Existing tests discover them by diffing continuity snapshots.

Implication: the smallest real follow-through might be **operator correlation/observability**, not routing-algorithm changes.

## What should be proven first

Prove the smallest honest public story **without changing runtime placement first**:

1. Put the serious PostgreSQL starter behind **one public ingress URL** in a proving environment (Fly is acceptable) or a faithful equivalent ingress harness.
2. Exercise the shipped clustered GET route surface through that one URL.
3. Show, from retained evidence, the difference between:
   - public ingress machine/platform choice
   - Mesh runtime owner choice
   - actual execution node
4. Decide whether the result is already good enough.

If this proof is green but awkward to explain, the first follow-through is docs/observability.
If this proof is not actually achievable or not truthfully inspectable, then the milestone should escalate to runtime/platform changes.

## Recommended slice boundaries

### Slice 1 — Current-state ingress truth on the serious starter

Goal:
- prove or falsify the current one-public-URL server-side-first story on the **generated PostgreSQL starter**, not on retained `cluster-proof`

Why first:
- it decides whether M054 is mostly explanation/observability or true routing follow-through
- it prevents unnecessary architecture churn if the current runtime + proxy story is already sufficient

Likely outputs:
- one new verifier or hosted/read-only proof rail
- retained evidence showing public ingress versus owner/execution truth
- explicit boundary statement: proxy/platform ingress vs Mesh runtime placement

### Slice 2 — Correlation / operator follow-through

Goal:
- make a single clustered HTTP request understandable without node knowledge or before/after continuity diffing

Most likely smallest useful seam:
- surface the route continuity key or equivalent correlation token in a response header / log / operator-friendly emitted signal
- keep it runtime-owned rather than starter-owned

Why second:
- if Slice 1 is green but opaque, this is the smallest real product improvement that closes the operator-understanding gap

### Slice 3 — Public docs and claim reset

Goal:
- align homepage, public-secondary docs, starter guidance, and proof-page wording to the exact proven contract

Scope should include:
- `website/docs/index.md`
- `website/docs/docs/distributed-proof/index.md`
- relevant first-contact handoffs if needed
- possibly `mesher/landing/app/mesh/page.tsx` if that public page is considered in-scope for the same claim
- new docs contract coverage for any strong load-balancing claim

Why after proof:
- the wording should be driven by the proven model, not the other way around

### Slice 4 — Platform/runtime escalation only if Slice 1 proves current behavior insufficient

Only open this slice if the current server-side story fails materially.

Possible escalation targets:
- runtime-owned HTTP correlation improvements beyond a simple header
- platform adapter support such as Fly-specific replay hints
- stronger route-placement behavior

This should be conditional, not assumed.

## Requirement analysis

### Already-covered table stakes

These are already in place and should be reused, not re-litigated:

- `R047` — Mesh must show ingress-node truth separately from execution-node truth
- `R065` — operator truth is runtime API first, CLI second, HTTP optional
- `R066` — `meshc init --clustered` is a real public clustered app path
- `R122` — serious Postgres starter already has real clustered deploy/failover proof
- `R060` — Fly is not the architecture
- `R124` — frontend-aware adapters stay deferred unless the deep dive proves they are needed

### Missing but likely needed as explicit M054 requirement detail

These feel like candidate requirements rather than silent scope creep:

1. **Public request correlation requirement**
   - A clustered HTTP request exercised through a serious public starter surface should be traceable to one continuity record without requiring before/after continuity list diffing.
   - Rationale: this is the missing piece for "understand what happened" at one public URL.

2. **Guarded public claim requirement**
   - Any public "load balancing" claim on homepage/docs/landing must be covered by a repo-owned docs contract test, not left as unguarded marketing copy.
   - Rationale: `website/docs/index.md` is currently outside the verified docs surface.

3. **One-public-URL proof requirement**
   - The serious clustered starter should have one retained proof surface demonstrating success through one public ingress URL while preserving Mesh-owned owner/execution evidence.
   - Rationale: current M053 proof is strong but node-direct/local-hosted, not public-ingress-facing.

### Things that should remain advisory or explicitly out of scope unless proof forces them in

- client-side node selection or frontend topology awareness
- sticky sessions/session affinity as a default product story
- least-loaded routing claims
- active-active writes
- generalized >2 replica route replication
- a second live proving environment in this milestone

## Risks that should drive slice ordering

1. **Docs can go green while the real public claim stays too strong.**
   - Homepage and landing claims are currently unguarded.

2. **A live Fly proof could accidentally become the architecture story.**
   - Current repo direction is explicit: Fly is evidence, not definition.

3. **The operator story may be technically true but too opaque for evaluators.**
   - Current clustered route request correlation is harness-friendly, not operator-friendly.

4. **The starter’s current clustered read surface is intentionally narrow.**
   - Only two GET routes are clustered, and they use replication count 1.
   - That is fine if docs say so; it is not fine if the public claim sounds like broad built-in request balancing across the whole app.

## Resume notes for the next unit

If the planner or next researcher needs to resume deeper work, the most valuable next inspections are:

1. inspect the actual VitePress nav/sidebar config to decide where an M054 explanation page or proof-page expansion should live
2. inspect `scripts/verify-m050-s02.sh` and `scripts/verify-m050-s03.sh` to see the cheapest place to add homepage/public-claim coverage without widening unrelated docs rails
3. inspect whether the smallest correlation follow-through should be a response header, a CLI helper, or a diagnostics/log addition by tracing from `clustered_route_response_from_request(...)` in `compiler/mesh-rt/src/http/server.rs`
4. if live Fly proof becomes the first slice, start from the existing retained pattern in `scripts/verify-m043-s04-fly.sh` but keep it scoped to the serious starter rather than the retained `cluster-proof` fixture

## Bottom line for roadmap planning

The likely M054 shape is:

- **prove the current one-public-URL server-side-first story on the serious starter first**
- **improve request-to-continuity observability if that story is too opaque**
- **then tighten public copy and add docs-contract guardrails**
- **only escalate to Fly-specific replay headers or deeper routing changes if the existing runtime-owned remote-dispatch model turns out not to be sufficient**

That ordering matches the codebase as it exists today and avoids inventing frontend-aware or Fly-specific architecture before the current server-side story has actually been measured.
