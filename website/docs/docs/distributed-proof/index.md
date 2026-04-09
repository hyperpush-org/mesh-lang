---
title: Distributed Proof
description: Canonical proof map for one public app URL in front of multiple nodes, server-side runtime placement, direct request-key lookup, and the serious PostgreSQL starter’s staged deploy/failover chain.
prev: false
next: false
---

# Distributed Proof

This is the only public-secondary docs page that carries the named clustered verifier rails.

Use [Distributed Actors](/docs/distributed/) for the language/runtime primitives, [Clustered Example](/docs/getting-started/clustered-example/) for the scaffold-first walkthrough, and [Production Backend Proof](/docs/production-backend-proof/) when the work becomes backend-specific. The clustered proof story now centers the generated PostgreSQL starter's M053 chain: `bash scripts/verify-m053-s01.sh` owns staged deploy truth, `bash scripts/verify-m053-s02.sh` owns failover truth, and `bash scripts/verify-m053-s03.sh` keeps packages/public-surface proof in the same hosted contract.

Keep the public starter split honest here too: [`examples/todo-sqlite/README.md`](https://github.com/hyperpush-org/mesh-lang/blob/main/examples/todo-sqlite/README.md) is the honest local single-node starter with no `work.mpl`, `HTTP.clustered(...)`, or `meshc cluster` story, while [`examples/todo-postgres/README.md`](https://github.com/hyperpush-org/mesh-lang/blob/main/examples/todo-postgres/README.md) is the serious shared/deployable starter that keeps source-first `@cluster` work and only dogfoods explicit-count `HTTP.clustered(1, ...)` on `GET /todos` and `GET /todos/:id`.

Once the public proof map reaches the maintained-app boundary, hand off into the [Hyperpush product repo](https://github.com/hyperpush-org/hyperpush-mono) and its [`mesher/README.md`](https://github.com/hyperpush-org/hyperpush-mono/blob/main/mesher/README.md) maintainer runbook. The local `verify-m051*` rails stay retained compatibility wrappers, not the public clustered story.

A proxy/platform ingress may expose one public app URL in front of multiple nodes, but that is where the public routing story ends. Mesh runtime placement starts after ingress forwards the request: the runtime records ingress, owner, replica, and execution truth, and operators inspect that on `meshc cluster` instead of through sticky sessions, frontend-aware routing, or client-visible topology.

## Public surfaces and verifier rails

This page is the canonical clustered proof map. The other public-secondary pages should hand readers here instead of repeating the named clustered verifier ledger.

- [Clustered Example](/docs/getting-started/clustered-example/) — first stop for the public scaffold surface
- [`examples/todo-postgres/README.md`](https://github.com/hyperpush-org/mesh-lang/blob/main/examples/todo-postgres/README.md) — the serious shared/deployable starter that owns the shipped clustered contract
- [`examples/todo-sqlite/README.md`](https://github.com/hyperpush-org/mesh-lang/blob/main/examples/todo-sqlite/README.md) — the honest local single-node SQLite starter, not a clustered/operator proof surface
- `bash scripts/verify-m053-s01.sh` — starter-owned staged deploy proof that retains the generated PostgreSQL bundle plus bundled artifacts
- `bash scripts/verify-m053-s02.sh` — starter-owned failover proof that replays S01, exercises the staged PostgreSQL starter under failover, and retains the failover proof bundle
- `bash scripts/verify-m053-s03.sh` — hosted packages/public-surface contract that checks the same starter proof remains visible in the public hosted story
- [Production Backend Proof](/docs/production-backend-proof/) — the compact backend proof handoff before any maintainer-only surface
- [Hyperpush product repo](https://github.com/hyperpush-org/hyperpush-mono) — repo-boundary maintained-app/backend handoff
- [`mesher/README.md`](https://github.com/hyperpush-org/hyperpush-mono/blob/main/mesher/README.md) — deeper maintained app runbook after the repo-boundary handoff

## One public URL, server-side placement, and request correlation

Keep the clustered HTTP story bounded:

- a proxy/platform ingress may expose one public app URL in front of multiple nodes, but it only chooses the HTTP ingress node
- Mesh runtime placement begins after that ingress hop and records ingress, owner, replica, and execution truth on the runtime side
- clustered `GET /todos` and `GET /todos/:id` responses include `X-Mesh-Continuity-Request-Key`; when you have that header, jump straight to the same request with `meshc cluster continuity <node-name@host:port> <request-key> --json`
- continuity-list discovery stays for startup records and manual inspection when you do not already have a request key
- `meshc cluster status` and `meshc cluster diagnostics` stay the outer operator/debug rails around either path

## What the public clustered contract proves

The public clustered story is intentionally smaller than the repo's full retained proof inventory:

- start with `meshc init --clustered`, then branch to the generated Postgres or SQLite example that matches the contract you actually want
- keep `meshc init --template todo-api --db postgres` as the fuller shared/deployable starter without changing the source-first `@cluster` contract
- keep `meshc init --template todo-api --db sqlite` on its honest local single-node contract instead of projecting clustered/operator claims onto it
- let one public app URL sit in front of multiple nodes while Mesh runtime placement stays server-side and operator truth stays on `meshc cluster`
- let `bash scripts/verify-m053-s01.sh` own staged deploy proof for the generated PostgreSQL starter
- let `bash scripts/verify-m053-s02.sh` own failover proof for that same staged PostgreSQL starter
- let `bash scripts/verify-m053-s03.sh` prove the hosted packages/public-surface contract still tells the same story
- keep clustered declaration state in source instead of the manifest
- rename legacy helper-shaped names to ordinary verbs instead of preserving runtime-plumbing-shaped public APIs
- let the runtime own startup, placement, continuity, promotion, recovery, and diagnostics
- use the same operator surfaces everywhere: status, direct continuity-record lookup when a clustered HTTP response already gave you a request key, continuity-list discovery for startup/manual inspection, and diagnostics
- keep the PostgreSQL Todo starter's clustered-route adoption narrow: `work.mpl` stays route-free, `GET /todos` and `GET /todos/:id` use explicit-count `HTTP.clustered(1, ...)`, and `GET /health` plus mutating routes stay local application routes
- keep Fly as a retained read-only reference/proof lane for already-deployed environments instead of treating it as a coequal public starter surface
- keep the deeper backend handoff on Production Backend Proof, the Hyperpush product repo, and the retained backend-only compatibility wrappers instead of promoting any mesh-lang-local product-source path as a coequal first-contact clustered starter

## Retained reference rails

Older repo-owned rails still exist for compatibility, history, and deeper operator proof, but they are secondary to the M053 starter-owned chain above:

- `bash scripts/verify-m051-s01.sh` — mesh-lang compatibility wrapper that confirms the public handoff still points at the product-owned Mesher verifier
- `bash scripts/verify-m051-s02.sh` — retained backend-only verifier replay kept behind the repo-boundary handoff
- `bash scripts/verify-m047-s04.sh` — authoritative M047 cutover rail for the source-first route-free clustered contract
- `bash scripts/verify-m047-s05.sh` — retained historical clustered Todo subrail kept behind fixture-backed rails instead of the public starter contract
- `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` — repo S07 rail for default-count and two-node `HTTP.clustered(...)` behavior beyond the PostgreSQL starter's explicit-count read routes
- `bash scripts/verify-m047-s06.sh` — docs and retained-proof closeout rail that wraps S05, rebuilds docs truth, and owns the assembled `.tmp/m047-s06/verify` bundle
- `bash scripts/verify-m046-s06.sh`, `bash scripts/verify-m046-s05.sh`, `bash scripts/verify-m046-s04.sh`, `bash scripts/verify-m045-s05.sh`, `bash scripts/verify-m045-s04.sh`, and `bash scripts/verify-m045-s03.sh` — historical compatibility aliases and subrails retained for replay, not for first-contact starter teaching
- `cargo run -q -p meshc -- build scripts/fixtures/clustered/tiny-cluster` plus `cargo run -q -p meshc -- test scripts/fixtures/clustered/tiny-cluster/tests` — lower-level retained tiny-cluster fixture contract
- `cargo run -q -p meshc -- build scripts/fixtures/clustered/cluster-proof` plus `cargo run -q -p meshc -- test scripts/fixtures/clustered/cluster-proof/tests` — retained fixture contract for the bounded Fly/reference package
- `bash scripts/verify-m043-s04-fly.sh --help` and the live mode behind `CLUSTER_PROOF_FLY_APP` / `CLUSTER_PROOF_BASE_URL` — retained read-only Fly sanity/config/log/probe verifier for an already-deployed reference environment

## Named proof commands

These are the repo-level commands behind the current distributed proof story:

```bash
# public starter-owned clustered proof chain in mesh-lang
bash scripts/verify-m053-s01.sh
bash scripts/verify-m053-s02.sh
bash scripts/verify-m053-s03.sh

# after the repo-boundary handoff in the Hyperpush product repo
bash mesher/scripts/verify-maintainer-surface.sh

# retained mesh-lang compatibility and history rails
bash scripts/verify-m051-s01.sh
bash scripts/verify-m051-s02.sh
bash scripts/verify-m047-s04.sh
bash scripts/verify-m047-s05.sh
cargo test -p meshc --test e2e_m047_s07 -- --nocapture
bash scripts/verify-m047-s06.sh
bash scripts/verify-m046-s06.sh
bash scripts/verify-m046-s05.sh
bash scripts/verify-m046-s04.sh
bash scripts/verify-m045-s05.sh
bash scripts/verify-m045-s04.sh
bash scripts/verify-m045-s03.sh
cargo run -q -p meshc -- build scripts/fixtures/clustered/tiny-cluster
cargo run -q -p meshc -- test scripts/fixtures/clustered/tiny-cluster/tests
cargo run -q -p meshc -- build scripts/fixtures/clustered/cluster-proof
cargo run -q -p meshc -- test scripts/fixtures/clustered/cluster-proof/tests
npm --prefix website run build
bash scripts/verify-m043-s04-fly.sh --help
CLUSTER_PROOF_FLY_APP=mesh-cluster-proof \
CLUSTER_PROOF_BASE_URL=https://mesh-cluster-proof.fly.dev \
  bash scripts/verify-m043-s04-fly.sh
```

> **Note:** The Fly verifier is intentionally read-only and intentionally secondary. Use `bash scripts/verify-m043-s04-fly.sh --help` when you only want the non-live syntax/help path. Live mode inspects an already-deployed reference app and optionally reads an existing continuity key with `CLUSTER_PROOF_REQUEST_KEY`; it does not create new work, does not mutate authority, and does not replace the portable staged PostgreSQL starter contract.

## Operator workflow across the public clustered surfaces

Whichever public surface you start from, keep the operator flow bounded:

1. `meshc cluster status <node-name@host:port> --json`
2. If a clustered HTTP response returned `X-Mesh-Continuity-Request-Key`, run `meshc cluster continuity <node-name@host:port> <request-key> --json` directly for that same public request.
3. If you are inspecting startup work or doing manual discovery without a request key yet, run `meshc cluster continuity <node-name@host:port> --json` first and then drill into a single record.
4. `meshc cluster diagnostics <node-name@host:port> --json`

The response header is an operator/debug seam, not a client routing signal. The runtime still owns ingress, owner, replica, and execution truth.

## Supported topology and non-goals

Supported topology and operator seam:

- one primary plus one standby using the same image and the same repo packaging path
- small env surface: cookie, discovery seed, explicit identity injection, continuity role, and promotion epoch
- same-image local proof for destructive failover and rejoin truth through the staged PostgreSQL starter chain
- read-only Fly inspection for already-deployed reference apps

Non-goals for this public rail:

- active-active writes or active-active intake
- multi-standby quorum or consensus claims
- sticky sessions, frontend-aware routing, or client-visible topology claims
- package-owned operator surfaces that compete with the runtime CLI
- presenting retained internal fixtures as the public onboarding story
- projecting clustered/operator claims onto the SQLite starter
- implying Fly is a required deploy target for the public clustered contract
- implying the retained `cluster-proof` fixture is a coequal public starter surface

## When to use this page vs the generic distributed guide

Use the generic [Distributed Actors](/docs/distributed/) guide when you want the language/runtime primitives.

Use this page when you want the named proof surfaces behind the scaffold/examples-first clustered story, the PostgreSQL starter's staged deploy + failover + hosted-contract chain, the SQLite-local boundary, the Production Backend Proof handoff into the Hyperpush product repo, and the retained Fly reference rail.

## Failure inspection map

If a proof fails, rerun the named command for the exact surface you care about:

- **Starter staged deploy proof:** `bash scripts/verify-m053-s01.sh`
- **Starter failover proof:** `bash scripts/verify-m053-s02.sh`
- **Hosted starter/packages/public-surface proof:** `bash scripts/verify-m053-s03.sh`
- **Repo-boundary maintained-app handoff:** [Production Backend Proof](/docs/production-backend-proof/), [Hyperpush product repo](https://github.com/hyperpush-org/hyperpush-mono), and [`mesher/README.md`](https://github.com/hyperpush-org/hyperpush-mono/blob/main/mesher/README.md)
- **Product-owned Mesher verifier:** `bash mesher/scripts/verify-maintainer-surface.sh`
- **mesh-lang compatibility wrapper:** `bash scripts/verify-m051-s01.sh`
- **Retained backend-only replay:** `bash scripts/verify-m051-s02.sh`
- **Authoritative M047 cutover rail:** `bash scripts/verify-m047-s04.sh`
- **Historical clustered Todo subrail:** `bash scripts/verify-m047-s05.sh`
- **Repo S07 clustered-route rail:** `cargo test -p meshc --test e2e_m047_s07 -- --nocapture`
- **Docs + retained-proof closeout rail:** `bash scripts/verify-m047-s06.sh`
- **Historical M046 closeout alias:** `bash scripts/verify-m046-s06.sh`
- **Historical M046 equal-surface alias:** `bash scripts/verify-m046-s05.sh`
- **Historical M046 package/startup alias:** `bash scripts/verify-m046-s04.sh`
- **Historical M045 closeout alias:** `bash scripts/verify-m045-s05.sh`
- **Historical M045 assembled alias:** `bash scripts/verify-m045-s04.sh`
- **Historical failover-only subrail:** `bash scripts/verify-m045-s03.sh`
- **Lower-level retained tiny-cluster fixture contract:** `cargo run -q -p meshc -- build scripts/fixtures/clustered/tiny-cluster && cargo run -q -p meshc -- test scripts/fixtures/clustered/tiny-cluster/tests`
- **Lower-level retained cluster-proof fixture contract:** `cargo run -q -p meshc -- build scripts/fixtures/clustered/cluster-proof && cargo run -q -p meshc -- test scripts/fixtures/clustered/cluster-proof/tests`
- **Public docs build:** `npm --prefix website run build`
- **Read-only Fly reference check:** `bash scripts/verify-m043-s04-fly.sh --help` for syntax, or `CLUSTER_PROOF_FLY_APP=mesh-cluster-proof CLUSTER_PROOF_BASE_URL=https://mesh-cluster-proof.fly.dev bash scripts/verify-m043-s04-fly.sh` for live inspection
